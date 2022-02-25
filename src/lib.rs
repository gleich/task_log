use std::fmt::Display;

use colorful::Colorful;

pub fn task<S, F>(name: S, runner: F)
where
    F: FnOnce(),
    S: Display,
{
    println!("  {}  | {}", "RUNNING".yellow(), name);
    runner();
    println!("\x1b[A\x1b[A");
    println!("  {}     | {}", "DONE".green(), name);
}
