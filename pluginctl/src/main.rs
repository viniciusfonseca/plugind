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
    List,
    Create {
        name: String
    },
    Deploy {
        plugin_conf: Option<String>,
        token: Option<String>
    },
    Rpc {
        lib_name: String,
        input: Option<String>
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
        PluginSubcommand::List => {

            let list = reqwest::blocking::get(format!("{}/plugins", plugind_url))?.text()?;

            println!("{}", list);
            Ok(())
        }
        PluginSubcommand::Create { name } => {

            std::process::Command::new("git")
                .arg("clone")
                .arg("https://github.com/viniciusfonseca/plugin-template.git")
                .arg(&name)
                .output()?;
            
            println!("Successfully created plugin: {}", &name);
                
            Ok(())
        }
        PluginSubcommand::Deploy { plugin_conf, token } => {
            let cwd = std::env::current_dir()?;
            let plugin_conf_path = cwd.join("plugind.toml");
            if let Err(e) = std::fs::metadata(&plugin_conf_path) {
                eprintln!("Failed to deploy plugin: {}", e);
                return Err(e.into());
            }
            let plugin_conf = match plugin_conf {
                Some(f) => std::fs::read(f)?,
                None => std::fs::read(plugin_conf_path)?
            };
            let plugin_conf = toml::from_slice::<PluginConf>(&plugin_conf)?;

            let plugin_bytes = std::fs::read(cwd.join(&plugin_conf.path))?;

            let mut form = reqwest::blocking::multipart::Form::new();
            form = form.part("name", Part::text(plugin_conf.name.clone()));
            form = form.part("file", Part::bytes(plugin_bytes));

            let client = reqwest::blocking::Client::new();

            let mut req = client.post(format!("{}/plugin", plugind_url))
                .multipart(form);

            if let Some(token) = token {
                req = req.bearer_auth(token);
            }

            let res = req.send()?;

            if !res.status().is_success() {
                return Err(anyhow::anyhow!("Failed to deploy plugin: {}, {}", res.status(), res.text()?));
            }

            println!("Successfully deployed plugin: {}", &plugin_conf.name.clone());

            Ok(())
        },
        PluginSubcommand::Rpc { lib_name, input } => {
            let input = input.unwrap_or("".to_string()).as_bytes().to_vec();
            let output = reqwest::blocking::Client::new()
                .post(format!("{}/plugins/{}/rpc", plugind_url, lib_name))
                .body(input)
                .send()?;
            println!("{}", String::from_utf8_lossy(&output.bytes()?));
            Ok(())
        }
    }
}
