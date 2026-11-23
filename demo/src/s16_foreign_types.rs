//! 16: Another crate's types (bollard's Docker models).

#[mlua_ir::module("vnd:docker_models")]
pub mod docker_models {
    use std::collections::HashMap;

    use bollard_stubs::models::{ContainerSummary, ContainerSummaryStateEnum};
    use mlua_ir::Foreign;

    pub(crate) fn sample_containers() -> Vec<ContainerSummary> {
        let make = |id: &str, name: &str, state| ContainerSummary {
            id: Some(id.to_owned()),
            names: Some(vec![format!("/{name}")]),
            image: Some("nginx:1.27".to_owned()),
            state: Some(state),
            labels: Some(HashMap::from([("app".to_owned(), name.to_owned())])),
            ..Default::default()
        };
        vec![
            make("4f2a", "web", ContainerSummaryStateEnum::RUNNING),
            make("9c1e", "worker", ContainerSummaryStateEnum::EXITED),
        ]
    }

    /// Two containers as Docker reports them: bollard's own `ContainerSummary`, typed field by field.
    ///
    /// ```luau
    /// local containers = docker_models.sample()
    /// assert(containers[1].State == "running")
    /// ```
    pub fn sample() -> Foreign<Vec<ContainerSummary>> {
        Foreign(sample_containers())
    }

    /// One line for a container: its first name and its state. Takes bollard's type as an argument too.
    pub fn line(container: Foreign<ContainerSummary>) -> String {
        let c = container.0;
        let name = c.names.and_then(|n| n.into_iter().next()).unwrap_or_default();
        let state = c.state.map(|s| s.to_string()).unwrap_or_default();
        format!("{name} {state}")
    }
}
