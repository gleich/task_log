use std::fmt::Display;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

mod util;

use chrono::Utc;
use colorful::core::color_string::CString;
use colorful::Colorful;
use lazy_static::lazy_static;

lazy_static! {
	static ref CONF: Arc<Mutex<ConfigBuilder>> = Arc::new(Mutex::new(ConfigBuilder::default()));
}

/// Configure task_log using this interactive configuration struct.
///
/// This is done by calling methods an instance of the struct to configure it and then calling `.apply()` to apply that configuration. Behind the scenes this updates the static mutex holding the configuration.
///
/// ## Example:
/// ```
/// use task_log::ConfigBuilder;
///
/// ConfigBuilder::new()
/// 	.color(false)
/// 	.duration(false)
/// 	.apply()
/// 	.expect("Failed to configure logger");
/// ```
pub struct ConfigBuilder {
	/// If the prefix (e.g. "DONE" or "RUNNING") should be in color.
	pub color: bool,
	/// If the duration that the task took should be included in the prefix (e.g. "DONE") at the end of the task.
	pub duration: bool,
}

impl ConfigBuilder {
	/// Create a new ConfigBuilder off of the default struct values.
	pub fn new() -> Self { Self::default() }

	/// Set the color value
	pub fn color(mut self, enabled: bool) -> Self {
		self.color = enabled;
		self
	}

	/// Set the duration value
	pub fn duration(mut self, enabled: bool) -> Self {
		self.duration = enabled;
		self
	}

	/// Apply the configuration
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

/// Automatic logging for function execution.
///
/// This function does the following:
/// 1. Output that the task is now running.
/// 2. Run the task.
/// 3. Clear running output and output that the task is now done.
///
/// Whatever the runner returns is returned by this function so you can still use `Result` and `?` if you're using a closure.
///
/// Logging this way actually makes for some very clean code. You can clearly see what is happening in a specific section of your code. It is almost like normal comments but with logging added on!
///
/// # Examples
///
/// To see more examples that you can even run locally please check out the [examples directory](https://github.com/gleich/task_log/tree/main/examples).
///
/// ## Basic Example
///
/// ```
/// use task_log::task;
///
/// let sum = task("Adding 1 and 2", || -> u32 { 1 + 2 });
/// println!("Sum of 1 and 2 is {}", sum);
/// ```
///
/// ## Error Example
///
/// ```
/// use std::fs;
/// use std::io::Result;
///
/// use task_log::task;
///
/// task("Creating and removing file", || -> Result<()> {
/// 	let filename = "hello.txt";
/// 	fs::write(filename, "foo bar")?;
/// 	fs::remove_file(filename)?;
/// 	Ok(())
/// })
/// .expect("Failed to create and remove the file");
/// ```
pub fn task<M, F, R>(msg: M, mut runner: F) -> R
where
	F: FnMut() -> R,
	M: Display,
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
		msg
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
		msg
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
