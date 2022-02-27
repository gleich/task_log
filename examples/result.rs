use std::io::Result;
use std::time::Duration;
use std::{fs, thread};

use task_log::task;

fn main() {
	let filename = "testing_testing.txt";

	task(
		format!(
			"Creating {fname}, waiting 2 seconds, and then deleting {fname}",
			fname = filename
		),
		|| -> Result<()> {
			fs::write(filename, "Hello World")?;
			thread::sleep(Duration::from_secs(2));
			// Purposely giving the wrong filename to make the function fail.
			fs::remove_file("test")?;
			Ok(())
		},
	)
	.expect("Failed to create and delete the file");
}
