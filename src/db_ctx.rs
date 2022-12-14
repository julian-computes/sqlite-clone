use std::io::Write;
use crate::statement::Row;

/// A handle on the DB
pub struct DBContext {
    rows: Vec<Row>,
}

impl DBContext {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
        }
    }

    /// Insert a row into the DB's single table
    pub fn insert_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    /// Print all rows to an output stream
    pub fn list_rows(&self, out_stream: &mut impl Write) {
        for row in &self.rows {
            writeln!(out_stream, "{}", row).expect("Write failure");
        }
    }
}