use std::{fs, thread, time::Duration};

use task_log::task;

fn main() {
    let file_name = "testing_testing.txt";
    task(
        format!(
            "Creating {fname}, waiting 2 seconds, and then deleting {fname}",
            fname = file_name
        ),
        || {
            fs::write(file_name, "Hello World").expect("Failed to write to file");
            thread::sleep(Duration::from_secs(2));
            fs::remove_file(file_name).expect("Failed to remove file");
        },
    )
}
