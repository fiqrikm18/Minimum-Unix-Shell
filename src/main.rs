mod command;
mod path;

use crate::command::Command;
use crate::path::retrieve_os_path;
use std::io;
use std::io::Write;

fn main() {
    let paths = retrieve_os_path();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let command = Command::from_string(input.trim());
        match command {
            Ok(cmd) => match cmd {
                Command::Exit => break,
                Command::Echo { display_string } => Command::echo_command(display_string),
                Command::Type { cmd } => Command::type_command(paths.clone(), cmd),
                _ => Command::other_command(paths.clone(), input.trim().to_string()),
            },
            Err(_) => {
                println!("Command not found")
            }
        }
    }
}
