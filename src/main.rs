mod get_starting_values;
mod structs;

use std::f64::consts::PI;

use structs::Point;

fn main() {
	let args = std::env::args().collect::<Vec<String>>();
	let (points, starting_values) = get_starting_values::get_starting_values(args);
	let [mut amplitude, mut offset, mut frequency] = starting_values;
	let mut min_cost = cost(amplitude.value, offset.value, frequency.value, &points);
	println!("Starting cost: {min_cost}");
	for i in -1..(frequency.steps as i32) {
		if i >= 0 {
			frequency.value += (frequency.max - frequency.min) / frequency.steps;
		}
		for j in -1..(offset.steps as i32) {
			if j >= 0 {
				offset.value += (offset.max - offset.min) / offset.steps;
			}
			for i in 0..(amplitude.steps as i32) {
				if i >= 0 {
					amplitude.value += (amplitude.max - amplitude.min) / amplitude.steps;
				}
				let cost = cost(amplitude.value, offset.value, frequency.value, &points);
				if cost < min_cost {
					min_cost = cost;
					println!(
						"New minimal cost: {} = {}; {} = {}; {} = {}; cost = {}",
						amplitude.name, amplitude.value, offset.name, offset.value, frequency.name, frequency.value, cost
					)
				}
			}
			amplitude.value = amplitude.min;
		}
		offset.value = offset.min;
	}
}

fn cost(a: f64, b: f64, f: f64, values: &[Point]) -> f64 {
	let mut sum = 0.0;
	for point in values {
		sum += ((a * (2.0 * PI * point.x * f + b).sin() - point.y) / point.error.max(10e-6)).abs().powi(2);
	}
	sum
}
