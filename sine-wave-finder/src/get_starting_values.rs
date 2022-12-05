use std::io::Write;

use crate::{
	structs::{Parameter, Point},
	OUTPUT_DIVIDER,
};

pub fn get_starting_values(mut args: Vec<String>) -> (Vec<Point>, [Parameter; 3], bool) {
	let mut starting_vals = [
		Parameter::default_with_name("A".to_string()),
		Parameter::default_with_name("B".to_string()),
		Parameter::default_with_name("F".to_string()),
	];
	let raw_output = args.len() > 1 && args[1].to_lowercase() == "--raw-output";
	let mut i = 0;
	let mut ask_for_starting_vals = true;
	'outer: while !args.is_empty() {
		let mut vals = vec![];
		for _ in 0..4 {
			if args.is_empty() {
				break 'outer;
			}
			let val = args.remove(0);
			if val.trim() == "|" {
				ask_for_starting_vals = false;
				break 'outer;
			}
			match val.parse::<f64>() {
				Ok(v) => vals.push(v),
				Err(_) => {
					continue 'outer;
				}
			}
		}
		starting_vals[i].value = vals[0];
		starting_vals[i].min = vals[1];
		starting_vals[i].max = vals[2];
		starting_vals[i].steps = vals[3];
		if !raw_output {
			println!(
				"Using {} = {}; {} <= {} <= {}; steps = {} from arguments",
				starting_vals[i].name, starting_vals[i].value, starting_vals[i].min, starting_vals[i].name, starting_vals[i].max, starting_vals[i].steps
			);
		}
		i += 1;
		if i == starting_vals.len() {
			break;
		}
	}
	while i < starting_vals.len() && ask_for_starting_vals {
		if !raw_output {
			print!("What should the starting value of {} be? ", starting_vals[i].name);
			if std::io::stdout().flush().is_err() {
				println!();
			};
		}
		let line = read_input_line().trim().split(' ').filter_map(|x| x.parse().ok()).collect::<Vec<f64>>();
		if line.len() < 4 {
			continue;
		}
		starting_vals[i].value = line[0];
		starting_vals[i].min = line[1];
		starting_vals[i].max = line[2];
		starting_vals[i].steps = line[3];
		i += 1;
	}
	let mut points: Vec<Point> = vec![];
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
					if !args.is_empty() {
						let val = args.remove(0);
						if val.trim() == "|" {
							ask_for_points = false;
							break;
						}
						if let Ok(error) = val.parse() {
							points.push(Point::from_array([x, y, error]));
						}
					}
				}
			}
		}
	}
	if ask_for_points && !raw_output {
		println!("Input [x, y, error] pairs to include to the points one by one, separated with new lines ('s' to stop): ");
		if std::io::stdout().flush().is_err() {
			println!();
		};
	}
	if ask_for_points {
		loop {
			let line = read_input_line();
			if line.to_lowercase() == *"s" {
				break;
			}
			let vals = line.split(' ').filter_map(|x| x.trim().parse::<f64>().ok()).collect::<Vec<f64>>();
			if vals.len() < 3 {
				continue;
			}
			points.push(Point::from_array([vals[0], vals[1], vals[2]]));
		}
	}
	if !raw_output {
		println!();
		println!("--------------------");
		for parameter in &starting_vals {
			println!(
				"Using {} = {}; {} <= {} <= {}; steps = {}",
				parameter.name, parameter.value, parameter.min, parameter.name, parameter.max, parameter.steps
			);
		}
		println!("----------");
		println!("Using the following points:");
		for point in &points {
			println!("[{}, {}]; error = {}", point.x, point.y, point.error);
		}
		println!("--------------------");
	} else {
		for point in &points {
			println!("{} {} {}", point.x, point.y, point.error);
		}
		println!("{OUTPUT_DIVIDER}");
	}
	(points, starting_vals, raw_output)
}

fn read_input_line() -> String {
	let mut buffer = String::new();
	std::io::stdin().read_line(&mut buffer).expect("Failed to read input line :(");
	buffer.trim().to_string()
}
