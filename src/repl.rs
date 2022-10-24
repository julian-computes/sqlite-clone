use std::io::{BufRead, Write};
use crate::meta_command::{do_meta_command, MetaCommandType};
use crate::statement::{execute_statement, Statement};
use crate::{DBContext};

/// Print a DB REPL prompt to stdout
fn print_prompt(out_stream: &mut impl Write) {
    // print!("db > ");
    write!(out_stream, "db > ").expect("Fatal error: Failed to write to out stream");
    out_stream.flush().unwrap_or_default();
}

/// Read a line from in_stream and return it without any trailing newline characters
fn read_input(
    in_stream: &mut impl BufRead
) -> String {
    let mut buf = String::new();
    in_stream.read_line(&mut buf).expect("Fatal error: failed to read input stream");
    // Strip newline
    if buf.ends_with("\n") {
        buf.truncate(buf.len() - 1);
        if buf.ends_with("\r") {
            buf.truncate(buf.len() - 1);
        }
    }
    buf
}

/// Read input from in_stream, execute it as a meta-command or as SQL, write the result to out_stream,
/// repeat.
pub fn do_repl(
    db: &mut DBContext,
    in_stream: &mut impl BufRead,
    out_stream: &mut impl Write,
) {
    loop {
        print_prompt(out_stream);
        let input_line = read_input(in_stream);
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
                execute_statement(db, out_stream, &statement);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, BufWriter};
    use super::*;

    /// Send a newline separated String of commands to the REPL and returns the output as a String
    fn send_commands(commands: String) -> String {
        let mut db = DBContext::new();
        let mut in_stream = BufReader::new(commands.as_bytes());
        let mut out_stream = BufWriter::<&mut [u8]>::new(&mut []);

        do_repl(&mut db, &mut in_stream, &mut out_stream);

        String::from_utf8_lossy(out_stream.buffer()).to_string()
    }

    #[test]
    fn test_repl() {
        // Test that a row can be inserted and retrieved
        let commands = vec![
            "insert 1 user1 user1@mail.com",
            "select",
            ".exit",
        ].join("\n");
        let output = send_commands(commands);
        assert!(output.contains("1 user1 user1@mail.com"));

        // Test that a row can be silently inserted
        let commands = vec![
            "insert 2 user2 user2@mail.com",
            ".exit",
        ].join("\n");
        let output = send_commands(commands);
        assert!(!output.contains("1 user user@mail.com"));
    }
}
