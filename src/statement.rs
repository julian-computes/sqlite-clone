use anyhow::anyhow;

/// The type of a SQL statement
#[derive(Debug, PartialEq)]
pub enum StatementType {
    Insert,
    Select,
}

/// A SQL statement
#[derive(Debug, PartialEq)]
pub struct Statement {
    statement_type: StatementType,
}

impl TryFrom<&&str> for Statement {
    type Error = anyhow::Error;

    fn try_from(input_line: &&str) -> Result<Self, Self::Error> {
        let mut tokens = input_line.split(' ');
        let statement_type = match tokens.next().ok_or(anyhow!("empty statement"))? {
            "select" => StatementType::Select,
            "insert" => StatementType::Insert,
            unrecognized_command =>
                return Err(anyhow!("unrecognized statement type: '{}'", unrecognized_command)),
        };

        Ok(Self {
            statement_type,
        })
    }
}

pub fn execute_statement(statement: &Statement) {
    match statement.statement_type {
        StatementType::Insert => {
            println!("do insert here");
        }
        StatementType::Select => {
            println!("do select here");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_statement() {
        let statement_type_pairs = vec![
            ("select", StatementType::Select),
            ("insert", StatementType::Insert),
        ];

        for (statement_str, statement_type) in statement_type_pairs {
            assert_eq!(statement_type,
                       Statement::try_from(&statement_str).unwrap().statement_type)
        }

        Statement::try_from(&"").expect_err("Unexpected successful conversion");
    }
}
