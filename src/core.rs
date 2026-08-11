use std::collections::HashMap;
use std::path::PathBuf;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::fs;

#[derive(Debug, Clone)]
pub struct Aliases(HashMap<String, String>);

impl Aliases {
    fn new() -> Aliases {
        return Aliases {
            0: HashMap::new()
        }
    }
    pub fn load_from_file(path: PathBuf) -> Result<Aliases, String> {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Aliases::new()),
            Err(e) => return Err(e.to_string())
        };

        return Ok(Aliases {
            0: toml::from_str(&content)
                .map_err(|e| e.to_string())?
        })
    }
    fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }
}

fn expand_alias(command: &str, aliases: &Aliases) -> String {
    let mut parts = command.split_whitespace();
    if let Some(head) = parts.next() {
        if let Some(expanded_head) = aliases.get(head){
            return format!("{}{}", expanded_head, &command[head.len()..])
        }
    }
    command.to_string()
}

pub fn launch(command: &str, aliases: &Aliases) -> std::io::Result<()> {
    let expanded_command = expand_alias(command, aliases);
    Command::new("sh")
        .arg("-c")
        .arg(expanded_command)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .process_group(0)
        .spawn()?;
    Ok(())
}
