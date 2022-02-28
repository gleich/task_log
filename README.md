# task_log

[![build](https://github.com/gleich/task_log/actions/workflows/build.yml/badge.svg)](https://github.com/gleich/task_log/actions/workflows/build.yml)
[![test](https://github.com/gleich/task_log/actions/workflows/test.yml/badge.svg)](https://github.com/gleich/task_log/actions/workflows/test.yml)
[![lint](https://github.com/gleich/task_log/actions/workflows/lint.yml/badge.svg)](https://github.com/gleich/task_log/actions/workflows/lint.yml)
[![docs.rs](https://img.shields.io/docsrs/task_log)](https://docs.rs/task_log/)
[![Crates.io](https://img.shields.io/crates/v/task_log)](https://crates.io/crates/task_log/)

task_log is a task-based logger.

## Installing

Just add `task_log = 0.1.4` to your Cargo.toml's `dependency` section.

## Example

Let's get right to the chase. What does using this logger look like?

```rust
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

```

As you can see we provide a task to run and a description of what that task is doing. When we run this code we get the following output.

![demo](https://raw.githubusercontent.com/gleich/task_log/main/demo.gif)

To see more examples see the [examples folder](./examples/).

## Configuration

You can configure task_log's `task` function using a struct called `ConfigBuilder`. Here is an example of using `ConfigBuilder`:

```rust
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
```

To learn more about `ConfigBuilder` please reference the [docs.rs documentation](https://docs.rs/task_log).

## Future Plans

Here are some features I'm hoping to implement in the future:

- Run time: Output the run time so far in the `RUNNING` prefix.
- Spinners: Show a loading symbol for long-running tasks.
- File output: Allow writing of logs to a file.
- Double log lines: Option to output log to a second line instead of replacing the first one.
