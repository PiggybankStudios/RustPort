#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::ffi::CString;
use std::ptr;
use std::mem::{size_of, zeroed};

use winapi::um::{winnt, winreg};
use winapi::shared::{winerror, minwindef::FILETIME, minwindef::DWORD};

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

struct MyApp<'a>
{
	name: String,
	age: u32,
	lines: Vec<String>,
	icon4: egui::ImageSource<'a>,
}

impl<'a> Default for MyApp<'a>
{
	fn default() -> MyApp<'a>
	{
		return MyApp {
			name: "Taylor".to_owned(),
			age: 39,
			lines: vec![],
			icon4: egui::include_image!("..\\data\\buttonIcon4.png").to_owned(),
		};
	}
}

fn EnumerateAvailableComPorts()
{
	unsafe
	{
		let keyCStr = CString::new("HARDWARE\\DEVICEMAP\\SERIALCOMM").unwrap(); //You'll find this under HKEY_LOCAL_MACHINE in regedit
		let mut regHandle = ptr::null_mut();
		
		let openResult = winreg::RegOpenKeyExA(
			winreg::HKEY_LOCAL_MACHINE,
			keyCStr.as_ptr(),
			0,
			winnt::KEY_READ,
			&mut regHandle
		);
		
		if (openResult == winerror::ERROR_SUCCESS as i32)
		{
			let mut classNameBuffer = vec![0u8;256];
			let mut classNameLength: DWORD = classNameBuffer.len() as DWORD;
			let mut numSubkeys: DWORD = 0;
			let mut numSubkeysLength: DWORD = 0;
			let mut maxClassLength: DWORD = 0;
			let mut numValues: DWORD = 0;
			let mut maxValueNameLength: DWORD = 0;
			let mut maxValuesLength: DWORD = 0;
			let mut securityDescriptor: DWORD = 0;
			let mut lastWriteTime: FILETIME = zeroed();
			
			let queryResult = winreg::RegQueryInfoKeyA(
				regHandle,
				classNameBuffer.as_mut_ptr() as *mut winnt::CHAR,
				&mut classNameLength as *mut DWORD,
				ptr::null_mut(),
				&mut numSubkeys as *mut DWORD,
				&mut numSubkeysLength as *mut DWORD,
				&mut maxClassLength as *mut DWORD,
				&mut numValues as *mut DWORD,
				&mut maxValueNameLength as *mut DWORD,
				&mut maxValuesLength as *mut DWORD,
				&mut securityDescriptor as *mut DWORD,
				&mut lastWriteTime as *mut FILETIME
			);
			
			if (queryResult == winerror::ERROR_SUCCESS as i32)
			{
				//numSubkeys = 0 numSubkeysLength = 0 maxClassLength = 0 numValues = 2 maxValueNameLength = 17 maxValuesLength = 10 securityDescriptor = 220
				// println!("numSubkeys = {} numSubkeysLength = {} maxClassLength = {} numValues = {} maxValueNameLength = {} maxValuesLength = {} securityDescriptor = {}",
				// 	numSubkeys,
				// 	numSubkeysLength,
				// 	maxClassLength,
				// 	numValues,
				// 	maxValueNameLength,
				// 	maxValuesLength,
				// 	securityDescriptor);
				
				let mut valueBuffer = vec![0u8;256];
				let mut valueBufferLength = valueBuffer.len() as DWORD;
				let mut dataBuffer = vec![0u8;256];
				let mut dataBufferLength = dataBuffer.len() as DWORD;
				
				for vIndex in 0..numValues
				{
					println!("Checking value[{}]", vIndex);
				}
			}
			else
			{
				println!("QueryResult: {}", queryResult);
			}
		}
		else { println!("Failed to open registry key: {}", openResult); }
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
					
					if ui.button("Open").clicked()
					{
						ui.set_width(50.0);
						println!("Open button was clicked!");
					}
					
					if ui.button("Settings").clicked()
					{
						println!("Settings button was clicked!");
						EnumerateAvailableComPorts();
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
				
				
				ui.image(self.icon4.clone());
				
				ui.label(format!("Hello {}, age {}", self.name, self.age));
			});
		});
	}
}
