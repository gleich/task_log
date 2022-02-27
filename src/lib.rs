use std::{
    fmt::Display,
    sync::{Arc, Mutex, MutexGuard, PoisonError},
};

mod util;

use chrono::Utc;
use colorful::{core::color_string::CString, Colorful};
use lazy_static::lazy_static;

lazy_static! {
    static ref CONF: Arc<Mutex<ConfigBuilder>> = Arc::new(Mutex::new(ConfigBuilder::default()));
}

pub struct ConfigBuilder {
    pub color: bool,
    pub duration: bool,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self, enabled: bool) -> Self {
        self.color = enabled;
        self
    }

    pub fn duration(mut self, enabled: bool) -> Self {
        self.duration = enabled;
        self
    }

    pub fn apply<'a>(self) -> Result<(), PoisonError<MutexGuard<'a, Self>>> {
        let mut changer = CONF.lock()?;
        *changer = self;
        Ok(())
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        ConfigBuilder {
            color: true,
            duration: true,
        }
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
    let running_msg = if config.duration {
        "  RUNNING       "
    } else {
        "  RUNNING  "
    };
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
    let done_msg = if config.duration {
        format!(
            "  DONE in {}  ",
            util::format_duration(Utc::now() - start_time)
        )
    } else {
        String::from("  DONE    ")
    };
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
