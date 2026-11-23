//! 15: Validated newtypes.

#[mlua_ir::module("std:names")]
pub mod names {
    use mlua_ir::Text;

    /// The name or id of a container: letters, digits and `_.-`, starting with a letter or digit.
    #[derive(Debug, Clone, Text)]
    #[text(validate = "container_name")]
    pub struct ContainerName(pub String);

    fn container_name(text: &str) -> Result<(), String> {
        let mut chars = text.chars();
        let first_ok = chars.next().is_some_and(|c| c.is_ascii_alphanumeric());
        if first_ok && chars.all(|c| c.is_ascii_alphanumeric() || "_.-".contains(c)) {
            Ok(())
        } else {
            Err(format!("`{text}` is not a container name: letters, digits and `_.-`, starting with a letter or digit"))
        }
    }

    /// How a container is shown.
    ///
    /// ```luau
    /// assert(names.describe("web-1") == "container web-1")
    /// ```
    pub fn describe(name: ContainerName) -> String {
        format!("container {}", name.0)
    }
}
