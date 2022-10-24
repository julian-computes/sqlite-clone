mod statement;
mod meta_command;
mod repl;
mod db_ctx;

use repl::do_repl;
use crate::db_ctx::DBContext;

fn main() {
    let in_stream = std::io::stdin();
    let out_stream = std::io::stdout();
    let mut db = DBContext::new();

    do_repl(&mut db, &mut in_stream.lock(), &mut out_stream.lock());
}
