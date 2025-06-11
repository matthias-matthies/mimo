use std::{
    io::{self, stdin, stdout},
    path::PathBuf,
    str::FromStr,
};

use clap::{
    Parser,
    Subcommand
};



pub struct UserInterfaceInput {

}

pub struct UserInterfaceOutput {

}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn verbose information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8
}

#[derive(Debug)]
enum Command {
    Exit,
    Help,
    Args,
    Unknown(String)
}

impl FromStr for Command {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ' ').collect();

        match parts[0].to_lowercase().as_str() {
            "exit" => Ok(Command::Exit),
            "help" => Ok(Command::Help),
            "args" => Ok(Command::Args),
            _      => Ok(Command::Unknown(s.to_string()))
        }
    }
}

pub fn run_input_handle() -> std::io::Result<()> {
    let args = Arguments::parse();
    let mut input_buffer = String::new();

    println!("Type 'exit' to quit.");

    loop {
        input_buffer.clear();

        print!("> ");

        stdin().read_line(&mut input_buffer)?;

        let trimmed_input = input_buffer.trim();

        let input_command = Command::from_str(trimmed_input)?;

        match input_command {
            Command::Exit => {
                break
            }
            Command::Help => {
                println!("Type 'exit' to quit.");
            }
            Command::Args => {
                println!("The current args are: {:#?}", args);
            }
            Command::Unknown(string) => {
                println!("Unknown Command: {}", string);
            }
        }
    }

    Ok(())
}

/*pub fn run_output_handle() -> std::io::Result<()> {
    run_cli_output()
}*/

/*fn run_cli_input() {

}*/

/*fn run_cli_output() -> std::io::Result<()> {

    Ok(())
}*/
