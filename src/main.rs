#![allow(dead_code)]
#![allow(unused_imports)]

use eframe::egui;
use eframe::egui::{Vec2, vec2};

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
	name: String,
	age: u32,
	lines: Vec<String>,
}

impl Default for MyApp
{
	fn default() -> MyApp
	{
		return MyApp {
			name: "Taylor".to_owned(),
			age: 39,
			lines: vec![],
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
					
					if ui.button("Open").clicked()
					{
						ui.set_width(50.0);
						println!("Open button was clicked!");
					}
					
					if ui.button("Settings").clicked()
					{
						println!("Settings button was clicked!");
					}
					if ui.button("Info").clicked()
					{
						println!("Info button was clicked!");
					}
					let mut is_checked: bool = false;
					ui.checkbox(&mut is_checked, "Test");
				});
			});
			
			ui.horizontal(|ui|
			{
				let name_label = ui.label("Your name: ");
				ui.text_edit_singleline(&mut self.name)
					.labelled_by(name_label.id);
			});
			
			egui::ScrollArea::vertical().show(ui, |ui|
			{
				ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
				
				if ui.button("Increment").clicked() { self.age += 1; }
				
				ui.image(egui::include_image!("F:\\test.png"));
				
				ui.label(format!("Hello {}, age {}", self.name, self.age));
			});
		});
	}
}
