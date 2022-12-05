#[derive(Debug)]
pub struct Parameter {
	pub name: String,
	pub value: f64,
	pub min: f64,
	pub max: f64,
	pub steps: f64,
}

impl Parameter {
	pub fn default_with_name(name: String) -> Self {
		Self {
			name,
			value: 0.0,
			min: 0.0,
			max: 0.0,
			steps: 0.0,
		}
	}
}

#[derive(Debug)]
pub struct Point {
	pub x: f64,
	pub y: f64,
	pub error: f64,
}

impl Point {
	pub fn from_array(data: [f64; 3]) -> Self {
		Self {
			x: data[0],
			y: data[1],
			error: data[2],
		}
	}
}
