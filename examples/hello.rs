use task_log::task;

fn main() {
    let sum = task("Adding 1 and 2", || -> u32 { 1 + 2 });
    println!("Answer is {}", sum);
}
