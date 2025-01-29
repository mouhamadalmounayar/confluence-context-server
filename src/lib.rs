struct ConfluenceContextServer;
use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{self as zed, serde_json, Command, ContextServerId, Project, Result};

#[derive(Debug, Deserialize)]
struct ConfluenceContextServerSettings {
    api_token: String,
    email: String,
    domain_name: String,
}

const SERVER_PATH: &str = "node_modules/mcp-confluence/dist/index.js";
impl zed::Extension for ConfluenceContextServer {
    fn new() -> Self {
        Self
    }
    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        let version = zed::npm_package_installed_version("mcp-confluence")?;
        if version.as_deref() != Some("1.1.0") {
            zed::npm_install_package("mcp-confluence", "1.1.0")?;
        }
        let settings = ContextServerSettings::for_project("confluence-context-server", _project)?;
        let Some(settings) = settings.settings else {
            return Err("missing settings".into());
        };
        let settings: ConfluenceContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;
        Ok(Command {
            command: "node".to_string(),
            args: vec![env::current_dir()
                .unwrap()
                .join(SERVER_PATH)
                .to_string_lossy()
                .to_string()],
            env: vec![
                ("API_TOKEN".into(), settings.api_token),
                ("EMAIL".into(), settings.email),
                ("DOMAIN_NAME".into(), settings.domain_name),
            ],
        })
    }
}

zed::register_extension!(ConfluenceContextServer);
