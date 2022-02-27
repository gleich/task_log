use task_log::task;

fn main() {
	let result = task("Adding two numbers", || -> u32 { 1 + 2 });
	println!("{}", result);
}
