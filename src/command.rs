use std::{fs::File, io::Error, os::unix::fs::PermissionsExt, path::PathBuf};

pub const BUILTINS_COMMAND: [&str; 3] = ["echo", "type", "exit"];
pub enum Command {
    Exit,
    Echo { display_string: String },
    Type { cmd: String },
    NotFound,
}

impl Command {
    pub fn from_string(input: &str) -> Result<Self, Error> {
        if input == "exit" {
            return Ok(self::Command::Exit);
        }

        if let Some(0) = input.find("echo ") {
            return Ok(self::Command::Echo {
                display_string: input["echo ".len()..].to_string(),
            });
        }

        if let Some(0) = input.find("type ") {
            let splited_str: Vec<&str> = input.split_whitespace().collect();
            return Ok(self::Command::Type {
                cmd: splited_str[1].trim().to_string(),
            });
        }

        Ok(self::Command::NotFound)
    }

    pub fn echo_command(display: String) {
        println!("{display}");
    }

    pub fn type_command(paths: Vec<PathBuf>, cmd: String) {
        if BUILTINS_COMMAND.contains(&cmd.as_str()) {
            println!("{cmd} is a shell builtin");
            return;
        }

        for path in &paths {
            let cmd_path = path.join(&cmd);
            if cmd_path.exists() {
                let permission = File::open(cmd_path.as_path())
                    .expect("file")
                    .metadata()
                    .expect("metadata")
                    .permissions();

                if permission.mode() & 0o111 != 0 {
                    println!("{} is {}", cmd, cmd_path.display());
                    return;
                }
            }
        }

        println!("{}: command not found", cmd.trim())
    }

    pub fn other_command(paths: Vec<PathBuf>, input: String) {
        let splited_inpt: Vec<&str> = input.split_whitespace().collect();
        if splited_inpt.is_empty() {
            return;
        }

        let cmd = splited_inpt[0];
        let args = &splited_inpt[1..];

        for path in &paths {
            let cmd_path = path.join(cmd);
            if cmd_path.exists() {
                let mut process = std::process::Command::new(cmd).args(args).spawn().unwrap();

                process.wait().unwrap();
                return;
            }
        }

        println!("{}: command not found", cmd.trim())
    }
}
