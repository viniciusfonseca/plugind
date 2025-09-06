use clap::{Parser, Subcommand};
use reqwest::blocking::multipart::Part;
use serde::Deserialize;

/// CLI for managing plugins
#[derive(Parser, Debug)]
#[command(name = "pluginctl")]
struct Cli {
    #[command(subcommand)]
    command: PluginSubcommand
}

#[derive(Debug, Clone, Subcommand)]
enum PluginSubcommand {
    Create {
        name: String
    },
    Deploy {
        plugin_conf: Option<String>
    }
}

#[derive(Deserialize)]
struct PluginConf {
    name: String,
    path: String
}

fn main() -> anyhow::Result<()> {

    let plugind_url = std::env::var("PLUGIND_URL").unwrap_or("http://localhost:8080".to_string());

    match Cli::parse().command {
        PluginSubcommand::Create { name } => {

            std::process::Command::new("git")
                .arg("clone")
                .arg("https://github.com/viniciusfonseca/plugin-template.git")
                .arg(&name)
                .output()?;
            
            println!("Successfully created plugin: {}", &name);
                
            Ok(())
        }
        PluginSubcommand::Deploy { plugin_conf } => {
            let cwd = std::env::current_dir()?;
            let plugin_conf = match plugin_conf {
                Some(f) => std::fs::read(f)?,
                None => std::fs::read(cwd.join("plugin.toml"))?
            };
            let plugin_conf = toml::from_slice::<PluginConf>(&plugin_conf)?;

            let plugin_bytes = std::fs::read(cwd.join(&plugin_conf.path))?;

            let mut form = reqwest::blocking::multipart::Form::new();
            form = form.part("name", Part::text(plugin_conf.name.clone()));
            form = form.part("file", Part::bytes(plugin_bytes));

            let client = reqwest::blocking::Client::new();

            let res = client.post(format!("{}/plugin", plugind_url))
                .multipart(form)
                .send()?;

            if !res.status().is_success() {
                return Err(anyhow::anyhow!("Failed to deploy plugin: {}, {}", res.status(), res.text()?));
            }

            println!("Successfully deployed plugin: {}", &plugin_conf.name.clone());

            Ok(())
        }
    }
}
