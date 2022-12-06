use eframe::egui;

mod application;
mod get_input;
mod structs;

pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const COLOURS: [egui::Color32; 5] = [egui::Color32::RED, egui::Color32::GREEN, egui::Color32::BLUE, egui::Color32::YELLOW, egui::Color32::WHITE];

fn main() {
	dotenv::dotenv().ok();
	let args = std::env::args().collect::<Vec<String>>();
	let input = get_input::get_input();
	println!("{} {}", REPOSITORY, VERSION);
	println!("{input:?}");
	let vsync_args = args.len() > 1 && args[1].to_lowercase() == *"--vsync-on";
	let native_options = eframe::NativeOptions {
		maximized: true,
		resizable: true,
		vsync: (vsync_args
			|| match std::env::var("VSYNC") {
				Ok(val) => val.to_lowercase() == "on",
				Err(_) => false,
			}),
		..Default::default()
	};

	eframe::run_native(
		"Sine wave visualiser",
		native_options,
		Box::new(|cc| Box::new(application::Application::new(cc, input, VERSION.to_string()))),
	);
}

impl eframe::App for application::Application {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		self.render(ctx);
		if self.step < self.max_step {
			self.step += 1;
		}
		ctx.request_repaint();
	}
}
