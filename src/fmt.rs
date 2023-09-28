pub use std::fmt::Display;

use crate::*;

/// Prints to stdout.
/// Returns value passed in.
pub fn println<T: Display>(item: Immutable<T>) -> Immutable<T> {
    println!("{}", item.0);
    item
}

/// Prints to stdout.
/// Returns value passed in.
pub fn print<T: Display>(item: Immutable<T>) -> Immutable<T> {
    print!("{}", item.0);
    item
}