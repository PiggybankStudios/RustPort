#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use eframe::egui;
use eframe::egui::{Vec2, vec2};

mod platform;
use platform::com_port::EnumerateAvailableComPorts;
mod resources;

const DEBUG_BUILD: bool = true;

fn main() -> eframe::Result
{
	env_logger::init(); //log to stderr (if you run with RUST_LOG=debug)
	
	let options = eframe::NativeOptions
	{
		viewport: egui::ViewportBuilder::default().with_inner_size([320.0,420.0]),
		..Default::default()
	};
	
	#[allow(unused_must_use)]
	eframe::run_native(
		"Rustport",
		options,
		Box::new(|cc| {
			egui_extras::install_image_loaders(&cc.egui_ctx); //gives us image support
			Ok(Box::<MyApp>::default())
		})
	)
}

struct MyApp
{
	lines: Vec<String>,
	portIcon: egui::ColorImage,
	settingsIcon: egui::ColorImage,
	infoIcon: egui::ColorImage,
}

impl Default for MyApp
{
	fn default() -> MyApp
	{
		return MyApp {
			lines: vec![],
			portIcon:     egui_extras::image::load_image_bytes(resources::ButtonIcon1).expect("Failed to load portIcon").to_owned(),
			settingsIcon: egui_extras::image::load_image_bytes(resources::ButtonIcon2).expect("Failed to load settingsIcon").to_owned(),
			infoIcon:     egui_extras::image::load_image_bytes(resources::ButtonIcon3).expect("Failed to load infoIcon").to_owned(),
		};
	}
}

impl eframe::App for MyApp
{
	#[allow(unused_variables)]
	fn update(&mut self, context: &egui::Context, frame: &mut eframe::Frame)
	{
		egui::CentralPanel::default().show(context, |ui|
		{
			// +==============================+
			// |        Render Topbar         |
			// +==============================+
			egui::TopBottomPanel::top("topbar")
			.show_inside(ui, |ui| {
				ui.horizontal(|ui| {
					
					// let (rect, _response) = ui.allocate_at_least(vec2(32.0, 32.0), egui::Sense::hover());
					
					// +==============================+
					// |       Open COM Button        |
					// +==============================+
					if (ui.add_sized([40.0, 40.0],
						egui::ImageButton::new(egui::SizedTexture::from(self.portIcon)))
						.clicked())
					{
						println!("You clicked the COM button!");
						EnumerateAvailableComPorts();
					}
					
					// +==============================+
					// |       Settings Button        |
					// +==============================+
					if (ui.add_sized([40.0, 40.0],
						egui::ImageButton::new(egui::SizedTexture::from(self.settingsIcon)))
						.clicked())
					{
						println!("You clicked the Settings button!");
					}
					
					// +==============================+
					// |         Info Button          |
					// +==============================+
					if (ui.add_sized([40.0, 40.0],
						egui::ImageButton::new(egui::SizedTexture::from(self.infoIcon)))
						.clicked())
					{
						println!("You clicked the Info button!");
					}
				});
				ui.add_space(10.0);
			});
			
			egui::TopBottomPanel::bottom("bottombar")
			.show_inside(ui, |ui| {
				ui.horizontal(|ui| {
					ui.label("Status: Loading...");
					ui.label("Status: Loading...");
					ui.label("Status: Loading...");
					ui.label("Status: Loading...");
					ui.label("Status: Loading...");
				});
			});
			
			egui::ScrollArea::both()
			.auto_shrink(false)
			.show(ui, |ui|
			{
				for lIndex in 0..10
				{
					ui.label(format!("Line {}", lIndex));
				}
			});
		});
	}
}
