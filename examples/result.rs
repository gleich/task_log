use std::{fs, thread, time::Duration};

use anyhow::Result;
use task_log::task;

fn main() {
    let file_name = "testing_testing.txt";

    task(
        format!(
            "Creating {fname}, waiting 2 seconds, and then deleting {fname}",
            fname = file_name
        ),
        || -> Result<()> {
            fs::write(file_name, "Hello World")?;
            thread::sleep(Duration::from_secs(2));
            fs::remove_file("test")?;
            Ok(())
        },
    )
    .expect("Failed to work with the file");
}
