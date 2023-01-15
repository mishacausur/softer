fn print_vec(xs: &Vec<i32>) {
	for x in xs {
		println!("{}", x);
	}
}

fn fn main() {
	let xs = Vec![1,2,3];
	print_vec(&xs);
	print_vec(&xs);
}