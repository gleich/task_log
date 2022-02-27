use std::thread;
use std::time::Duration;

use task_log::{task, ConfigBuilder};

fn main() {
	ConfigBuilder::new()
		.duration(false)
		.apply()
		.expect("Failed to setup configuration");

	let sum = task("Adding 1 and 2", || -> u32 {
		let result = 1 + 2;
		thread::sleep(Duration::from_secs(2));
		result
	});

	println!("Sum is {}", sum)
}
