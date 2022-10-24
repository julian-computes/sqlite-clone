use std::fmt::{Display, Formatter};
use std::io::Write;
use anyhow::anyhow;
use crate::{DBContext};

/// The type of a SQL statement
#[derive(Debug, PartialEq)]
pub enum StatementType {
    Insert(Row),
    Select,
}

/// A row in the database's table
#[derive(Debug, PartialEq, Clone)]
pub struct Row {
    id: u32,
    name: String,
    email: String,
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.id, self.name, self.email)
    }
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
            "insert" => {
                let id = tokens.next().ok_or(anyhow!("Missing ID"))?;
                let name = tokens.next().ok_or(anyhow!("Missing name"))?;
                let email = tokens.next().ok_or(anyhow!("Missing email"))?;

                StatementType::Insert(
                    Row {
                        id: id.parse().ok().ok_or(anyhow!("ID must be positive integer"))?,
                        name: name.to_string(),
                        email: email.to_string(),
                    }
                )
            },
            unrecognized_command =>
                return Err(anyhow!("unrecognized statement type: '{}'", unrecognized_command)),
        };

        Ok(Self {
            statement_type,
        })
    }
}

/// Execute a SQL statement
pub fn execute_statement(db: &mut DBContext, out_stream: &mut impl Write, statement: &Statement) {
    match &statement.statement_type {
        StatementType::Insert(row) => {
            execute_insert(db, row);
        }
        StatementType::Select => {
            execute_select(db, out_stream);
        }
    }
}

/// Execute an insert statement given a row to insert
fn execute_insert(db: &mut DBContext, row: &Row) {
    db.insert_row(row.clone());
}

/// Execute a select statement by writing all rows to an output stream
fn execute_select(db: &mut DBContext, out_stream: &mut impl Write) {
    db.list_rows(out_stream);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_statement() {
        let statement_type_pairs = vec![
            ("select", StatementType::Select),
            ("insert 1 user user@mail.com", StatementType::Insert(Row {
                id: 1,
                name: "user".to_string(),
                email: "user@mail.com".to_string()
            })),
        ];

        for (statement_str, statement_type) in statement_type_pairs {
            assert_eq!(statement_type,
                       Statement::try_from(&statement_str).unwrap().statement_type)
        }

        Statement::try_from(&"").expect_err("Unexpected successful conversion");
    }
}
