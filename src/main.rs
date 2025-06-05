
use eframe::egui;

fn main()
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
	);
}

struct MyApp
{
	name: String,
	age: u32,
}

impl Default for MyApp
{
	fn default() -> MyApp
	{
		return MyApp {
			name: "Taylor".to_owned(),
			age: 39,
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
			ui.heading("Rustport");
			
			ui.horizontal(|ui| {
				let name_label = ui.label("Your name: ");
				ui.text_edit_singleline(&mut self.name)
					.labelled_by(name_label.id);
			});
			
			ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
			
			if ui.button("Increment").clicked() { self.age += 1; }
			
			ui.label(format!("Hello {}, age {}", self.name, self.age));
			
			ui.image(egui::include_image!("F:\\test.png"));
		});
	}
}
