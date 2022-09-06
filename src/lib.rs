//DB instance.
pub mod db;

// Stage 1: Parse the SQL string into an Abstract Syntax Tree (AST).
pub mod parser;

// Stage 2: Execute the statement to the str.
pub mod executor;

// Enable macros for logging.
#[macro_use]
extern crate log;

#[cfg(test)]
mod test;

pub use self::db::{Database, Error};