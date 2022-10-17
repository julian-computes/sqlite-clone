use std::io::Write;
use crate::meta_command::{do_meta_command, MetaCommandType};
use crate::statement::{execute_statement, Statement};

/// Print a DB REPL prompt to stdout
fn print_prompt() {
    print!("db > ");
    std::io::stdout().flush().expect("Fatal error: Failed to flush stdout");
}

/// Read a line from stdin and return it without any trailing newline characters
fn read_input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Fatal error: failed to read from stdin");
    // Strip newline
    if buf.ends_with("\n") {
        buf.truncate(buf.len() - 1);
        if buf.ends_with("\r") {
            buf.truncate(buf.len() - 1);
        }
    }
    buf
}

/// Read input from the stdin, execute it as a meta-command or as SQL, print the result,
/// repeat.
pub fn do_repl() {

    loop {
        print_prompt();
        let input_line = read_input();
        let input_line = input_line.as_str();

        if input_line.starts_with(".") {
            match do_meta_command(&input_line) {
                Ok(command_type) => {
                    match command_type {
                        MetaCommandType::Exit => {
                            break;
                        }
                    }
                }
                Err(msg) => {
                    eprintln!("{}", msg);
                    continue;
                }
            }
        }

        match Statement::try_from(&input_line) {
            Ok(statement) => {
                execute_statement(&statement);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

