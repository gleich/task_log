use std::{
    fmt::Display,
    sync::{Arc, Mutex, MutexGuard, PoisonError},
};

mod util;

use chrono::Utc;
use colorful::{core::color_string::CString, Colorful};
use lazy_static::lazy_static;

lazy_static! {
    static ref CONF: Arc<Mutex<Conf>> = Arc::new(Mutex::new(Conf::default()));
}

pub struct Conf {
    pub color: bool,
}

impl Conf {
    pub fn apply<'a>(self) -> Result<(), PoisonError<MutexGuard<'a, Conf>>> {
        let mut changer = CONF.lock()?;
        *changer = self;
        Ok(())
    }
}

impl Default for Conf {
    fn default() -> Conf {
        Conf { color: true }
    }
}

pub fn task<S, F, R>(name: S, mut runner: F) -> R
where
    F: FnMut() -> R,
    S: Display,
{
    let arc_ref = Arc::clone(&CONF);
    let config = arc_ref.lock().unwrap();

    // START
    let start_time = Utc::now();
    let running_msg = "  RUNNING       ";
    println!(
        "{}| {}",
        if config.color {
            running_msg.yellow()
        } else {
            CString::new(running_msg)
        },
        name
    );

    let result = runner();

    // DONE
    println!("\x1b[A\x1b[A");
    let done_msg = format!(
        "  DONE in {}  ",
        util::format_duration(Utc::now() - start_time)
    );
    println!(
        "{} | {}",
        if config.color {
            done_msg.green()
        } else {
            CString::new(done_msg)
        },
        name
    );
    result
}

#[cfg(test)]
mod tests {
    use crate::task;

    #[test]
    fn basic_run() {
        let name = "basic run";
        assert_eq!(3, task(name, || -> u32 { 1 + 2 }));
        assert_eq!(true, task(name, || -> bool { true }));
    }
}
