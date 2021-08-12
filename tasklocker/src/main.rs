use std::io::{stdin,stdout,Write};

mod command_processor;
use command_processor::{process_command, Command, CommandExecutionError};

fn do_nothing() -> Result<bool, CommandExecutionError> {
    println!("Doing nothing");
    Ok(true)
}

fn main() {
    // Initialize the necessary variables
    let mut can_finish: bool = false;

    let valid_commands: Vec<Command> = vec![
    Command {
        name: String::from("exit"),
        execute: do_nothing,
        },
    Command {
        name: String::from("quit"),
        execute: do_nothing,
        },   
    ];

    let mut s=String::new();
    let mut current_command: Command = Command {
        name: String::from("none"),
        execute: do_nothing,
        };
    while !can_finish {
        print!("\x1B[2J\x1B[1;1H");
        println!("Available commands: ");
        println!("1) New");
        println!("2) List");
        println!("3) Modify");
        println!("4) Remove");
        println!("5) Exit/Quit");

        print!("Please enter a command: ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        match process_command(&s, &valid_commands) {
            Ok(comm) => current_command = comm,
            Err(_) => println!("Please enter a valid command")
        }
        s.clear();
        if current_command.name.eq("exit") || current_command.name.eq("quit") {
            println!("Exiting");
            can_finish = true;
        }
    }
}