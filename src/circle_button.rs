//This file was copied from image_button.rs originally and then modified to suit my needs for circular buttons in RustPort

use egui::{
	widgets, Color32, CornerRadius, Image, TextureOptions, Rect, Pos2, Response, Sense, Ui, Vec2, Widget, WidgetInfo,
	WidgetType, Painter, TextStyle
};
use egui::emath::{ Align };
use egui::epaint::{ Mesh, RectShape, Shape, StrokeKind };
use egui::epaint::text::{ LayoutJob, TextFormat, TextWrapping };
use egui::load::{ SizedTexture, TextureLoadResult, TexturePoll };
use egui::widgets::{ ImageOptions, ImageSource, Spinner };
use egui::style::{ WidgetVisuals };

/// A clickable image within a frame.
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
#[derive(Clone, Debug)]
pub struct CircleButton<'a> {
	pub(crate) iconImage: Image<'a>,
	pub(crate) backgroundImage: Image<'a>,
	pub(crate) highlightImage: Image<'a>,
	sense: Sense,
	frame: bool,
	selected: bool,
	alt_text: Option<String>,
}

impl<'a> CircleButton<'a> {
	pub fn new(
		iconImage: impl Into<Image<'a>>,
		backgroundImage: impl Into<Image<'a>>,
		highlightImage: impl Into<Image<'a>>) -> Self {
		Self {
			iconImage: iconImage.into().texture_options(TextureOptions::LINEAR),
			backgroundImage: backgroundImage.into().texture_options(TextureOptions::LINEAR),
			highlightImage: highlightImage.into().texture_options(TextureOptions::LINEAR),
			sense: Sense::click(),
			frame: true,
			selected: false,
			alt_text: None,
		}
	}

	/// Select UV range. Default is (0,0) in top-left, (1,1) bottom right.
	#[inline]
	pub fn uv(mut self, uv: impl Into<Rect>) -> Self {
		let rectangle = uv.into();
		self.iconImage = self.iconImage.uv(rectangle.clone());
		self.backgroundImage = self.backgroundImage.uv(rectangle.clone());
		self.highlightImage = self.highlightImage.uv(rectangle.clone());
		self
	}

	/// Multiply image color with this. Default is WHITE (no tint).
	#[inline]
	pub fn tint(mut self, tint: impl Into<Color32>) -> Self {
		let tintColor = tint.into();
		self.iconImage = self.iconImage.tint(tintColor.clone());
		self.backgroundImage = self.backgroundImage.tint(tintColor.clone());
		self.highlightImage = self.highlightImage.tint(tintColor.clone());
		self
	}

	/// If `true`, mark this button as "selected".
	#[inline]
	pub fn selected(mut self, selected: bool) -> Self {
		self.selected = selected;
		self
	}

	/// Turn off the frame
	#[inline]
	pub fn frame(mut self, frame: bool) -> Self {
		self.frame = frame;
		self
	}

	/// By default, buttons senses clicks.
	/// Change this to a drag-button with `Sense::drag()`.
	#[inline]
	pub fn sense(mut self, sense: Sense) -> Self {
		self.sense = sense;
		self
	}

	/// Set rounding for the `CircleButton`.
	///
	/// If the underlying image already has rounding, this
	/// will override that value.
	#[inline]
	pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
		let radius = corner_radius.into();
		self.iconImage = self.iconImage.corner_radius(radius.clone());
		self.backgroundImage = self.backgroundImage.corner_radius(radius.clone());
		self.highlightImage = self.highlightImage.corner_radius(radius.clone());
		self
	}

	/// Set rounding for the `CircleButton`.
	///
	/// If the underlying image already has rounding, this
	/// will override that value.
	#[inline]
	#[deprecated = "Renamed to `corner_radius`"]
	pub fn rounding(self, corner_radius: impl Into<CornerRadius>) -> Self {
		self.corner_radius(corner_radius)
	}
}

fn my_paint_texture_load_result(
	ui: &Ui,
	tlr: &TextureLoadResult,
	rect: Rect,
	show_loading_spinner: Option<bool>,
	options: &ImageOptions,
	alt: Option<&str>,
) {
	match tlr {
		Ok(TexturePoll::Ready { texture }) => {
			my_paint_texture_at(ui.painter(), rect, options, texture);
		}
		Ok(TexturePoll::Pending { .. }) => {
			let show_loading_spinner =
				show_loading_spinner.unwrap_or(ui.visuals().image_loading_spinners);
			if show_loading_spinner {
				Spinner::new().paint_at(ui, rect);
			}
		}
		Err(_) => {
			let font_id = TextStyle::Body.resolve(ui.style());
			let mut job = LayoutJob {
				wrap: TextWrapping::truncate_at_width(rect.width()),
				halign: Align::Center,
				..Default::default()
			};
			job.append(
				"⚠",
				0.0,
				TextFormat::simple(font_id.clone(), ui.visuals().error_fg_color),
			);
			if let Some(alt) = alt {
				job.append(
					alt,
					ui.spacing().item_spacing.x,
					TextFormat::simple(font_id, ui.visuals().text_color()),
				);
			}
			let galley = ui.painter().layout_job(job);
			ui.painter().galley(
				rect.center() - Vec2::Y * galley.size().y * 0.5,
				galley,
				ui.visuals().text_color(),
			);
		}
	}
}

fn my_paint_texture_at(
	painter: &egui::Painter,
	rect: Rect,
	options: &ImageOptions,
	texture: &SizedTexture,
) {
	if options.bg_fill != Default::default() {
		painter.add(RectShape::filled(
			rect,
			options.corner_radius,
			options.bg_fill,
		));
	}

	match options.rotation {
		Some((rot, origin)) => {
			// TODO(emilk): implement this using `PathShape` (add texture support to it).
			// This will also give us anti-aliasing of rotated images.
			debug_assert!(
				options.corner_radius == CornerRadius::ZERO,
				"Image had both rounding and rotation. Please pick only one"
			);

			let mut mesh = Mesh::with_texture(texture.id);
			mesh.add_rect_with_uv(rect, options.uv, options.tint);
			mesh.rotate(rot, rect.min + origin * rect.size());
			painter.add(Shape::mesh(mesh));
		}
		None => {
			painter.add(
				RectShape::filled(rect, options.corner_radius, options.tint)
					.with_texture(texture.id, options.uv),
			);
		}
	}
}

/// Attach tooltips like "Loading…" or "Failed loading: …".
fn my_texture_load_result_response(
	source: &ImageSource<'_>,
	tlr: &TextureLoadResult,
	response: Response,
) -> Response {
	match tlr {
		Ok(TexturePoll::Ready { .. }) => response,
		Ok(TexturePoll::Pending { .. }) => {
			let uri = source.uri().unwrap_or("image");
			response.on_hover_text(format!("Loading {uri}…"))
		}
		Err(err) => {
			let uri = source.uri().unwrap_or("image");
			response.on_hover_text(format!("Failed loading {uri}: {err}"))
		}
	}
}

impl Widget for CircleButton<'_> {
	fn ui(self, ui: &mut Ui) -> Response {
		let padding = Vec2::ZERO;
		// let padding = if self.frame {
		// 	// so we can see that it is a button:
		// 	Vec2::splat(ui.spacing().button_padding.x)
		// } else {
		// 	Vec2::ZERO
		// };

		let available_size_for_image = ui.available_size() - 2.0 * padding;
		let iconTlr = self.iconImage.load_for_size(ui.ctx(), available_size_for_image);
		let backgroundTlr = self.backgroundImage.load_for_size(ui.ctx(), available_size_for_image);
		let highlightTlr = self.highlightImage.load_for_size(ui.ctx(), available_size_for_image);
		let original_image_size = backgroundTlr.as_ref().ok().and_then(|t| t.size());
		let image_size = self
			.backgroundImage
			.calc_size(available_size_for_image, original_image_size);

		let padded_size = image_size + 2.0 * padding;
		let (rect, response) = ui.allocate_exact_size(padded_size, self.sense);
		response.widget_info(|| {
			let mut info = WidgetInfo::new(WidgetType::Button);
			info.label = self.alt_text.clone();
			info
		});

		if ui.is_rect_visible(rect) {
			let (expansion, rounding, fill, stroke) = if self.selected {
				let selection = ui.visuals().selection;
				(
					Vec2::ZERO,
					self.backgroundImage.image_options().corner_radius,
					selection.bg_fill,
					selection.stroke,
				)
			} else if self.frame {
				let visuals: &WidgetVisuals = ui.style().interact(&response);
				let expansion = Vec2::splat(visuals.expansion);
				(
					expansion,
					self.backgroundImage.image_options().corner_radius,
					visuals.weak_bg_fill,
					visuals.bg_stroke,
				)
			} else {
				Default::default()
			};

			// Draw frame background (for transparent images):
			let rectangle: Rect = rect.expand2(expansion);
			let center: Vec2 = (rectangle.min.to_vec2() + rectangle.max.to_vec2())/2.0;
			let radius = (rectangle.max.x - rectangle.min.x)/2.0;
			ui.painter()
				.circle_filled(center.to_pos2(), radius, fill);

			let image_rect = ui
				.layout()
				.align_size_within_rect(image_size, rect.shrink2(padding));
			// let image_rect = image_rect.expand2(expansion); // can make it blurry, so let's not
			let image_options = self.backgroundImage.image_options().clone();

			my_paint_texture_load_result(
				ui,
				&backgroundTlr,
				image_rect,
				None,
				&image_options,
				self.alt_text.as_deref(),
			);

			// if (stroke.width > 0.0)
			// {
			// 	my_paint_texture_load_result(
			// 		ui,
			// 		&highlightTlr,
			// 		image_rect,
			// 		None,
			// 		&image_options,
			// 		self.alt_text.as_deref(),
			// 	);
			// }

			my_paint_texture_load_result(
				ui,
				&iconTlr,
				image_rect,
				None,
				&image_options,
				self.alt_text.as_deref(),
			);

			// Draw frame outline:
			ui.painter().circle_stroke(
				center.to_pos2(),
				radius,
				stroke,
			);
		}

		my_texture_load_result_response(&self.iconImage.source(ui.ctx()), &iconTlr, response)
	}
}
