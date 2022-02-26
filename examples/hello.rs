use task_log::task;

fn main() {
    let mut result = 0;
    task("Adding 1 and 2", || {
        result = 1 + 2;
    });
    println!("Answer is {}", result);
}
