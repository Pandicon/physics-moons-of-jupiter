mod get_starting_values;

use std::f64::consts::PI;

fn main() {
	let args = std::env::args().collect::<Vec<String>>();
	let starting_vals_names = ["A", "B", "T"];
	let (starting_values, points) = get_starting_values::get_starting_values(args, &starting_vals_names);
	println!("{:?}", points);
	println!("{:?}", starting_values);
}

fn cost(a: f64, b: f64, t: f64, values: &[[f64; 2]]) -> f64 {
	let mut sum = 0.0;
	for &[x, y] in values {
		sum += (a * (2.0 * PI * x / t + b).sin() - y).powi(2);
	}
	sum
}
