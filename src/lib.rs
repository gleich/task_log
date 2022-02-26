use std::fmt::Display;

use colorful::Colorful;

pub fn task<S, F, R>(name: S, mut runner: F) -> R
where
    F: FnMut() -> R,
    S: Display,
{
    println!("  {}  | {}", "RUNNING".yellow(), name);
    let result = runner();
    println!("\x1b[A\x1b[A");
    println!("  {}     | {}", "DONE".green(), name);
    result
}
