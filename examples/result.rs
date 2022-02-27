use std::io::Result;
use std::time::Duration;
use std::{fs, thread};

use task_log::task;

fn main() {
	task("Creating and removing file", || -> Result<()> {
		let filename = "hello.txt";
		fs::write(filename, "foo bar")?;
		thread::sleep(Duration::from_secs(2));
		fs::remove_file(filename)?;
		Ok(())
	})
	.expect("Failed to create and delete the file");
}
