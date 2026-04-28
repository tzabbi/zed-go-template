use zed_extension_api as zed;

struct GoTemplateExtension;

impl zed::Extension for GoTemplateExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let path = worktree
            .which("gopls")
            .ok_or_else(|| "The LSP for go-template 'gopls' is not installed".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        if language_server_id.as_ref() == "gopls" {
            return Ok(Some(zed::serde_json::json!({
                "templateExtensions": ["tmpl", "gotmpl", "gohtml", "html"]
            })));
        }

        Ok(None)
    }
}

zed::register_extension!(GoTemplateExtension);
