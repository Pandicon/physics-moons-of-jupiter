use crate::structs;
use eframe::egui;

pub struct Application {
	pub max_step: usize,
	pub step: usize,
	pub input: Vec<(Vec<structs::Point>, Vec<structs::Wave>)>,
	pub version: String,
}

impl Application {
	pub fn new(cc: &eframe::CreationContext<'_>, input: Vec<(Vec<structs::Point>, Vec<structs::Wave>)>, version: String) -> Self {
		cc.egui_ctx.set_visuals(egui::Visuals::dark());
		Self {
			max_step: input.iter().map(|(_points, waves)| waves.len()).max().unwrap_or(1) - 1,
			step: 0,
			input,
			version,
		}
	}

	pub fn render(&mut self, ctx: &egui::Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			let plot = egui::plot::Plot::new("Data");
			let mut all_points = vec![];
			let mut all_points_error_lines = vec![];
			for (index, data_set) in self.input.iter().enumerate() {
				let colour = crate::COLOURS[index % crate::COLOURS.len()];
				let (points, _waves) = data_set;
				let graph_points = egui::plot::Points::new(points.iter().map(|point| [point.x, point.y]).collect::<Vec<[f64; 2]>>());
				let graph_points = graph_points.color(colour);
				let graph_points = graph_points.highlight(true);
				let lines = points
					.iter()
					.map(|point| {
						let n = (point.error * 10.0 + 1.0) as u64;
						let line_points: egui::plot::PlotPoints = (0..=n)
							.map(|i| {
								let y = egui::remap(i as f64, 0.0..=n as f64, (point.y - point.error)..=(point.y + point.error));
								[point.x, y]
							})
							.collect();
						egui::plot::Line::new(line_points).color(colour)
					})
					.collect::<Vec<egui::plot::Line>>();

				all_points.push(graph_points);
				all_points_error_lines.push(lines);
			}
			plot.show(ui, |plot_ui| {
				for points_group_error_lines in all_points_error_lines {
					for error_line in points_group_error_lines {
						plot_ui.line(error_line);
					}
				}
				for points in all_points {
					plot_ui.points(points);
				}
			})
			.response
		});
	}
}
