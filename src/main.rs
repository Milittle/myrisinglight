use risinglight::Database;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    env_logger::init();
    let db = Database::new();
    let mut rl = Editor::<()>::new();
    loop {
        match rl.readline("> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let ret = db.r(&line);
                match ret {
                    Ok(chunks) => {
                        for chunk in chunks {
                            println!("{}", chunk);
                        }
                    },
                    Err(err) => println!("{}", err),
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted");
            },
            Err(ReadlineError::Eof) => {
                println!("Eof");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}