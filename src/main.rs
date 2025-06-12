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

struct MyApp<'a>
{
	lines: Vec<String>,
	portIcon: egui::ImageSource<'a>,
	settingsIcon: egui::ImageSource<'a>,
	infoIcon: egui::ImageSource<'a>,
}

impl<'a> Default for MyApp<'a>
{
	fn default() -> MyApp<'a>
	{
		let mut lines: Vec<String> = vec![];
		for lIndex in 0..100
		{
			lines.push(format!("Line {}", lIndex+1).to_string());
		}
		return MyApp {
			lines: lines,
			portIcon:     egui::include_image!("..\\data\\buttonIcon4.png").to_owned(),
			settingsIcon: egui::include_image!("..\\data\\buttonIcon5.png").to_owned(),
			infoIcon:     egui::include_image!("..\\data\\buttonIcon2.png").to_owned(),
		};
	}
}

impl<'a> eframe::App for MyApp<'a>
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
						egui::ImageButton::new(self.portIcon.clone()))
						.clicked())
					{
						println!("You clicked the COM button!");
						EnumerateAvailableComPorts();
					}
					
					// +==============================+
					// |       Settings Button        |
					// +==============================+
					if (ui.add_sized([40.0, 40.0],
						egui::ImageButton::new(self.settingsIcon.clone()))
						.clicked())
					{
						println!("You clicked the Settings button!");
					}
					
					// +==============================+
					// |         Info Button          |
					// +==============================+
					if (ui.add_sized([40.0, 40.0],
						egui::ImageButton::new(self.infoIcon.clone()))
						.clicked())
					{
						println!("You clicked the Info button!");
						self.lines.push(format!("New Line {}", self.lines.len()));
					}
				});
				ui.add_space(10.0);
			});
			
			egui::TopBottomPanel::bottom("bottombar")
			.show_inside(ui, |ui| {
				ui.horizontal(|ui| {
					ui.label(format!("Status: {} Line{}", self.lines.len(), if self.lines.len() == 1 {""} else {"s"}));
				});
			});
			
			egui::ScrollArea::both()
			.auto_shrink(false)
			.show(ui, |ui|
			{
				for line in &self.lines
				{
					ui.label(line);
				}
			});
		});
	}
}
