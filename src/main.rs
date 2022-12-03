use std::{f64::consts::PI, io::Write};

fn main() {
	let mut args = std::env::args().collect::<Vec<String>>();
	let _args_clone = args.clone();
	let starting_vals_names = ["A", "B", "T"];
	let mut starting_vals = [0.0, 0.0, 0.0];
	let mut i = 0;
	let mut ask_for_starting_vals = true;
	while !args.is_empty() {
		let val = args.remove(0);
		if val.trim() == "|" {
			ask_for_starting_vals = false;
			break;
		}
		if let Ok(x) = val.parse::<f64>() {
			starting_vals[i] = x;
			println!("Using {} = {} from arguments", starting_vals_names[i], x);
			i += 1;
			if i == starting_vals.len() {
				break;
			}
		}
	}
	while i < starting_vals.len() && ask_for_starting_vals {
		print!("What should the starting value of {} be? ", starting_vals_names[i]);
		if let Err(_) = std::io::stdout().flush() {
			println!("");
		};
		let line = read_input_line();
		match line.parse::<f64>() {
			Ok(x) => {
				starting_vals[i] = x;
				println!("Using {} = {}", starting_vals_names[i], x);
			}
			Err(err) => println!("Failed to parse the input value: {}", err),
		}
		i += 1;
	}
	let mut points: Vec<[f64; 2]> = vec![];
	let mut ask_for_points = true;
	while !args.is_empty() {
		let val = args.remove(0);
		if val.trim() == "|" {
			ask_for_points = false;
			break;
		}
		if let Ok(x) = val.parse::<f64>() {
			if !args.is_empty() {
				let val = args.remove(0);
				if val.trim() == "|" {
					ask_for_points = false;
					break;
				}
				if let Ok(y) = val.parse() {
					points.push([x, y]);
				}
			}
		}
	}
	if ask_for_points {
		println!("Input [x, y] pairs to include to the points one by one, separated with new lines ('s' to stop): ");
	}
	while ask_for_points {
		if let Err(_) = std::io::stdout().flush() {
			println!("");
		};
		let line = read_input_line();
		if line.to_lowercase() == *"s" {
			break;
		}
		let vals = line.split(' ').filter_map(|x| x.trim().parse::<f64>().ok()).collect::<Vec<f64>>();
		if vals.len() < 2 {
			continue;
		}
		points.push([vals[0], vals[1]]);
	}
	let points = points;
	println!();
	println!("--------------------");
	for (name, value) in starting_vals_names.iter().zip(starting_vals) {
		println!("Using {} = {}", name, value);
	}
	println!("----------");
	println!("Using the following points:");
	for [x, y] in points {
		println!("[{x}, {y}]");
	}
	println!("--------------------");
}

fn cost(a: f64, b: f64, t: f64, values: &[[f64; 2]]) -> f64 {
	let mut sum = 0.0;
	for &[x, y] in values {
		sum += (a * (2.0 * PI * x / t + b).sin() - y).powi(2);
	}
	sum
}

fn read_input_line() -> String {
	let mut buffer = String::new();
	std::io::stdin().read_line(&mut buffer).expect("Failed to read input line :(");
	buffer.trim().to_string()
}
