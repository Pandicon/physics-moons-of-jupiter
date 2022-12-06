use crate::structs::{InputSection, Point, Wave};

pub fn get_input() -> Vec<InputSection> {
	let mut data = vec![];
	let mut loop_outer = true;
	while loop_outer {
		let mut points = vec![];
		let mut waves = vec![];
		loop {
			let line = read_input_line();
			if line == *"---" {
				break;
			} else if line == *"-----" {
				loop_outer = false;
				break;
			}
			let split = line.split(' ').filter_map(|x| x.parse::<f64>().ok()).collect::<Vec<f64>>();
			if let Some(point) = Point::from_vec(split) {
				if point.x.is_nan() || point.x.is_infinite() || point.y.is_nan() || point.y.is_infinite() {
					continue;
				}
				points.push(point)
			}
		}
		if loop_outer {
			loop {
				let line = read_input_line();
				if line == *"---" {
					break;
				} else if line == *"-----" {
					loop_outer = false;
					break;
				}
				let split = line.split(' ').filter_map(|x| x.parse::<f64>().ok()).collect::<Vec<f64>>();
				if let Some(wave) = Wave::from_vec(split) {
					waves.push(wave)
				}
			}
		}
		let points_x = points.iter().map(|point| point.x);
		let mut points_min_x = None;
		let mut points_max_x = None;
		for x in points_x {
			if let Some(min_x) = points_min_x {
				if x < min_x {
					points_min_x = Some(x)
				}
			} else {
				points_min_x = Some(x)
			}
			if let Some(max_x) = points_max_x {
				if x > max_x {
					points_max_x = Some(x)
				}
			} else {
				points_max_x = Some(x)
			}
		}
		data.push(InputSection {
			points_min_x: points_min_x.unwrap_or(0.0) - 10.0,
			points_max_x: points_max_x.unwrap_or(0.0) + 10.0,
			points,
			waves,
		});
	}
	data
}

fn read_input_line() -> String {
	let mut buffer = String::new();
	std::io::stdin().read_line(&mut buffer).expect("Failed to read input line D:");
	buffer.trim().to_string()
}
