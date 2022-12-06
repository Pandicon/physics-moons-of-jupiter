use crate::structs::{Point, Wave};

pub fn get_input() -> Vec<(Vec<Point>, Vec<Wave>)> {
	let mut data = vec![];
	'outer: loop {
		let mut points = vec![];
		let mut waves = vec![];
		loop {
			let line = read_input_line();
			if line == *"---" {
				break;
			} else if line == *"-----" {
				break 'outer;
			}
			let split = line.split(' ').filter_map(|x| x.parse::<f64>().ok()).collect::<Vec<f64>>();
			if let Some(point) = Point::from_vec(split) {
				points.push(point)
			}
		}
		loop {
			let line = read_input_line();
			if line == *"---" {
				break;
			} else if line == *"-----" {
				break 'outer;
			}
			let split = line.split(' ').filter_map(|x| x.parse::<f64>().ok()).collect::<Vec<f64>>();
			if let Some(wave) = Wave::from_vec(split) {
				waves.push(wave)
			}
		}
		data.push((points, waves));
	}
	data
}

fn read_input_line() -> String {
	let mut buffer = String::new();
	std::io::stdin().read_line(&mut buffer).expect("Failed to read input line D:");
	buffer.trim().to_string()
}
