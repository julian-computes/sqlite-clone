use anyhow::anyhow;

/// The type of a meta command (.exit, etc.)
#[derive(Debug, PartialEq)]
pub enum MetaCommandType {
    Exit,
}

pub fn do_meta_command(input_line: &&str) -> anyhow::Result<MetaCommandType> {
    match *input_line {
        ".exit" => Ok(MetaCommandType::Exit),
        _ => Err(anyhow!("Error: unrecognized command '{}'", input_line)),
    }
}

#[cfg(test)]
mod tests {
    use crate::meta_command::{do_meta_command, MetaCommandType};

    #[test]
    fn test_do_meta_command() {
        let command_type_pairs = vec![
            (".exit", MetaCommandType::Exit),
        ];

        for (command, result) in command_type_pairs {
            assert_eq!(result, do_meta_command(&command).unwrap());
        }

        do_meta_command(&"")
            .expect_err("Failed to fail to execute unrecognized meta-command");
    }
}