use eframe::egui;
use egui::{Visuals};
use std::thread;
use std::sync::Arc;
use std::time;

mod core;

mod ui;

const MAX_FPS: f32 = 30.0;

//🎥 🎧 🎨
fn main() -> Result<(), eframe::Error> {
	let icon_data = core::load_icon().unwrap_or_default();

	let options = eframe::NativeOptions {
   		viewport: egui::ViewportBuilder::default()
   			.with_inner_size([1200.0, 800.0])
   			.with_icon(Arc::new(icon_data)),
   		..Default::default()
   	};

   	eframe::run_native(
		"MaBO",
		options,
		Box::new(move |_cc| Box::from(MaBO::default())),
	)
}


struct MaBO {
	frame_time: time::Instant,
	equation: core::Equation,
}

impl Default for MaBO {
	fn default() -> Self {
		Self {
			frame_time: time::Instant::now(),
			equation: core::Equation::new()
		}
	}
}

impl eframe::App for MaBO {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		thread::sleep(time::Duration::from_secs_f32(
			((1.0 / MAX_FPS) - self.frame_time.elapsed().as_secs_f32()).max(0.0),
		));
		self.frame_time = time::Instant::now();
		
		ctx.set_visuals(Visuals::light());
		
		self.show_credits_panel(ctx);
		
		self.show_info_panel(ctx);

		self.show_main_panel(ctx);
	}
}

