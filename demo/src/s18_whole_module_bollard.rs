//! 18: A whole module end to end: `vnd:bollard`, against an in-memory daemon (no Docker needed).

#[mlua_ir::module("vnd:bollard")]
pub mod bollard {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    use bollard_stubs::models::{ContainerCreateBody, ContainerSummary, ContainerSummaryStateEnum};
    use mlua_ir::{Foreign, LuaError, Object, Options, Shape, Stream};
    use serde::{Deserialize, Serialize};

    use crate::s15_validated_text::names::ContainerName;

    /// How to reach a daemon.
    #[derive(Debug, Default, Serialize, Deserialize, Options)]
    pub struct ConnectOptions {
        /// `unix:///var/run/docker.sock` or `tcp://host:2375`; the local socket when not given.
        pub host: Option<String>,
        /// Seconds a call waits for the daemon.
        pub timeout: Option<u32>,
        /// Client certificates for a TLS daemon.
        pub tls: Option<Tls>,
    }

    /// The three files of a TLS client: all three or none, which the type says (not a runtime check).
    #[derive(Debug, Clone, Serialize, Deserialize, Shape)]
    pub struct Tls {
        pub ca: String,
        pub cert: String,
        pub key: String,
    }

    /// Why a Docker call failed.
    #[derive(Debug, Serialize, thiserror::Error, LuaError)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    pub enum DockerError {
        /// No daemon answered at `host`.
        #[error("no Docker daemon answers at {host}")]
        Connect { host: String },
        /// There is no such container.
        #[error("no such container: {name}")]
        NotFound { name: String },
        /// The container's state does not allow it (a name in use, a running container).
        #[error("{reason}")]
        Conflict { reason: String },
    }

    /// Options of `list_containers`.
    #[derive(Debug, Default, Serialize, Deserialize, Options)]
    pub struct ListContainers {
        /// Stopped containers too.
        pub all: bool,
        /// At most this many.
        pub limit: Option<u32>,
    }

    /// Options of `logs`.
    #[derive(Debug, Default, Serialize, Deserialize, Options)]
    pub struct Logs {
        /// Start this many lines from the end.
        pub tail: Option<u32>,
    }

    /// Which output a piece came from.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, Shape)]
    #[serde(rename_all = "snake_case")]
    pub enum Output {
        Stdout,
        Stderr,
    }

    /// A piece of a container's output.
    #[derive(Debug, Serialize, Deserialize, Shape)]
    pub struct LogPiece {
        /// Which output it came from.
        pub stream: Output,
        /// What was written.
        pub data: String,
    }

    /// Connect to a daemon. Sends nothing: the first call does.
    ///
    /// ```luau
    /// local docker = assert(bollard.connect())
    /// for _, c in assert(docker:list_containers({ all = true })) do
    ///     assert(c.Names ~= nil)
    /// end
    /// ```
    pub fn connect(opts: ConnectOptions) -> Result<Client, DockerError> {
        let host = opts.host.unwrap_or_else(|| "unix:///var/run/docker.sock".to_owned());
        if host.starts_with("tcp://nowhere") {
            return Err(DockerError::Connect { host });
        }
        let containers = crate::s16_foreign_types::docker_models::sample_containers();
        Ok(Client { host, containers: Rc::new(RefCell::new(containers)) })
    }

    /// A client for one daemon.
    #[derive(Clone, Object)]
    pub struct Client {
        host: String,
        containers: Rc<RefCell<Vec<ContainerSummary>>>,
    }

    fn named(c: &ContainerSummary, name: &str) -> bool {
        c.names.iter().flatten().any(|n| n.trim_start_matches('/') == name) || c.id.as_deref() == Some(name)
    }

    #[mlua_ir::object]
    impl Client {
        /// The daemon this client talks to.
        #[get]
        pub fn host(&self) -> String {
            self.host.clone()
        }

        /// The containers, as Docker's summaries. Only running ones unless `all`.
        pub async fn list_containers(&self, opts: ListContainers) -> Result<Foreign<Vec<ContainerSummary>>, DockerError> {
            let running = |c: &&ContainerSummary| opts.all || c.state == Some(ContainerSummaryStateEnum::RUNNING);
            let limit = opts.limit.map_or(usize::MAX, |l| l as usize);
            Ok(Foreign(self.containers.borrow().iter().filter(running).take(limit).cloned().collect()))
        }

        /// One container's summary, or `nil` when there is none: absence is not an error.
        pub async fn find_container(&self, name: ContainerName) -> Result<Option<Foreign<ContainerSummary>>, DockerError> {
            Ok(self.containers.borrow().iter().find(|c| named(c, &name.0)).cloned().map(Foreign))
        }

        /// Create a container from Docker's own create body (`Image`, `Labels`, `Env`, `HostConfig`, ...): its id.
        ///
        /// ```luau
        /// local docker = assert(bollard.connect())
        /// local id = assert(docker:create_container("api", { Image = "nginx:1.27", Labels = { app = "api" } }))
        /// assert(docker:find_container("api") ~= nil and #id > 0)
        /// ```
        pub async fn create_container(
            &self,
            name: ContainerName,
            body: Foreign<ContainerCreateBody>,
        ) -> Result<String, DockerError> {
            if self.containers.borrow().iter().any(|c| named(c, &name.0)) {
                return Err(DockerError::Conflict { reason: format!("the name `{}` is in use", name.0) });
            }
            let id = format!("{:04x}", self.containers.borrow().len() * 4099 + 7);
            self.containers.borrow_mut().push(ContainerSummary {
                id: Some(id.clone()),
                names: Some(vec![format!("/{}", name.0)]),
                image: body.0.image,
                labels: body.0.labels,
                state: Some(ContainerSummaryStateEnum::CREATED),
                ..Default::default()
            });
            Ok(id)
        }

        /// A container's output as a stream; a container that does not exist is the stream's error.
        pub fn logs(&self, name: ContainerName, opts: Logs) -> Stream<LogPiece, DockerError> {
            let exists = self.containers.borrow().iter().any(|c| named(c, &name.0));
            let lines = ["starting", "listening on :80", "GET / 200"];
            let skip = opts.tail.map_or(0, |t| lines.len().saturating_sub(t as usize));
            let items: Vec<Result<LogPiece, DockerError>> = if exists {
                lines.iter().skip(skip).map(|l| Ok(LogPiece { stream: Output::Stdout, data: (*l).to_owned() })).collect()
            } else {
                vec![Err(DockerError::NotFound { name: name.0 })]
            };
            Stream::new(futures::stream::iter(items))
        }
    }

    #[allow(dead_code)]
    fn labels(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| ((*k).to_owned(), (*v).to_owned())).collect()
    }
}
