#[derive(Debug)]
pub struct Point {
	pub x: f64,
	pub y: f64,
	pub error: f64,
}

impl Point {
	pub fn from_vec(data: Vec<f64>) -> Option<Self> {
		if data.len() < 3 {
			return None;
		}
		Some(Self {
			x: data[0],
			y: data[1],
			error: data[2],
		})
	}
}

#[derive(Debug)]
pub struct Wave {
	pub a: f64,
	pub b: f64,
	pub f: f64,
	pub cost: f64,
}

impl Wave {
	pub fn from_vec(data: Vec<f64>) -> Option<Self> {
		if data.len() < 4 {
			return None;
		}
		Some(Self {
			a: data[0],
			b: data[1],
			f: data[2],
			cost: data[2],
		})
	}
}
