use std::f64::consts::PI;

use crate::structs;
use eframe::egui;

pub struct Application {
	pub max_step: usize,
	pub step: usize,
	pub input: Vec<structs::InputSection>,
	pub version: String,
}

impl Application {
	pub fn new(cc: &eframe::CreationContext<'_>, input: Vec<structs::InputSection>, version: String) -> Self {
		cc.egui_ctx.set_visuals(egui::Visuals::dark());
		Self {
			max_step: input.iter().map(|input_section| input_section.waves.len()).max().unwrap_or(1) - 1,
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
			let mut all_waves_lines = vec![];
			for (index, data_set) in self.input.iter().enumerate() {
				let colour = crate::COLOURS[index % crate::COLOURS.len()];
				let structs::InputSection {
					points_min_x,
					points_max_x,
					points,
					waves,
				} = data_set;
				let graph_points = egui::plot::Points::new(points.iter().map(|point| [point.x, point.y]).collect::<Vec<[f64; 2]>>())
					.color(colour)
					.highlight(true);
				let error_lines = points
					.iter()
					.map(|point| {
						let line_points: egui::plot::PlotPoints = (0..=1)
							.map(|i| {
								let y = egui::remap(i as f64, 0.0..=1.0 as f64, (point.y - point.error)..=(point.y + point.error));
								[point.x, y]
							})
							.collect();
						egui::plot::Line::new(line_points).color(colour)
					})
					.collect::<Vec<egui::plot::Line>>();

				all_points.push(graph_points);
				all_points_error_lines.push(error_lines);

				if waves.is_empty() {
					continue;
				}
				let wave = &waves[self.step.min(waves.len() - 1)];
				let wave_line = {
					let n = ((*points_max_x - *points_min_x + 1.0) * 5.0).abs().ceil() as usize;
					let line_points: egui::plot::PlotPoints = (0..=n)
						.map(|i| {
							let x = egui::remap(i as f64, 0.0..=n as f64, *points_min_x..=*points_max_x);
							[x, wave.a * (2.0 * PI * x * wave.f + wave.b).sin()]
						})
						.collect();
					egui::plot::Line::new(line_points).color(colour)
				};
				all_waves_lines.push(wave_line);
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
				for wave_line in all_waves_lines {
					plot_ui.line(wave_line);
				}
			})
			.response
		});
	}
}
