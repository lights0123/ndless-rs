pub mod freetype {
	use ndless::prelude::*;
	use unicode_segmentation::UnicodeSegmentation;

	use crate::{
		video::Color,
		video::Surface,
		video::SurfaceFlag::SWSurface
	};
	use crate::video::Color::{RGB, RGBA};

	#[derive(Debug, PartialEq, Clone)]
	pub struct Text<'a> {
		text: String,
		matrix: ndless_freetype::Matrix,
		surface: Option<Surface>,
		up_to_date: bool,
		height: isize,
		font: &'a ndless_freetype::Face,
		color: Color,
	}

	impl<'a> Text<'a> {
		pub fn new(font: &'a ndless_freetype::Face) -> Self {
			Self {
				text: "".to_string(),
				matrix: Self::radians_to_matrix(0.),
				surface: None,
				up_to_date: false,
				height: 40,
				font,
				color: RGB(0, 0, 0),
			}
		}
		pub fn text(&mut self, str: impl Into<String>) -> &mut Self {
			let str = str.into();
			if str != self.text {
				self.text = str;
				self.up_to_date = false;
			}
			self
		}
		pub fn font(&mut self, font: &'a ndless_freetype::Face) -> &mut Self {
			if font != self.font {
				self.font = font;
				self.up_to_date = false;
			}
			self
		}
		pub fn color(&mut self, color: Color) -> &mut Self {
			if color != self.color {
				self.color = color;
				self.up_to_date = false;
			}
			self
		}
		pub fn height(&mut self, height: isize) -> &mut Self {
			if height != self.height {
				self.height = height;
				self.up_to_date = false;
			}
			self
		}
		fn radians_to_matrix(angle: f64) -> ndless_freetype::Matrix {
			ndless_freetype::Matrix {
				xx: (angle.cos() * f64::from(0x10000)) as ndless_freetype::FT_Fixed,
				xy: (-angle.sin() * f64::from(0x10000)) as ndless_freetype::FT_Fixed,
				yx: (angle.sin() * f64::from(0x10000)) as ndless_freetype::FT_Fixed,
				yy: (angle.cos() * f64::from(0x10000)) as ndless_freetype::FT_Fixed,
			}
		}
		// Removed for now because rotation can cause integer overflow
		/*/// Angle in degrees
		pub fn rotate(&mut self, angle: f64) -> &mut Self {
			let angle = angle.to_radians();
			let matrix = Self::radians_to_matrix(angle);
			if matrix != self.matrix {
				self.matrix = matrix;
				self.up_to_date = false;
			}
			self
		}*/
		/// Use when the size of text
		pub fn reallocate(&mut self) {
			self.surface = None;
		}
		#[allow(clippy::many_single_char_names)]
		pub fn render(&mut self) -> &Surface {
			if self.up_to_date && self.surface.as_ref().is_some() {
				return self.surface.as_ref().unwrap()
			}
			self.font.set_char_size(self.height * 64, 0, 50, 0).unwrap();
			let chars = self.text.graphemes(true);
			let mut pen = ndless_freetype::Vector { x: 0, y: 0 };
			let mut max_height = 0;
			let mut baseline_height = 0;
			let mut max_width = 0;
			let mut min_height = 0;
			for letter in chars.clone() {
				self.font.set_transform(&mut self.matrix, &mut pen);
				self.font
				    .load_char(
					    letter.chars().next().unwrap() as usize,
					    ndless_freetype::face::LoadFlag::RENDER,
				    )
				    .unwrap();
				let glyph = self.font.glyph();
				let cbox = glyph.get_glyph().unwrap().get_cbox(0);
				if cbox.xMax / 64 > max_width { max_width = cbox.xMax / 64 }
				if cbox.yMin / 64 < min_height { min_height = cbox.yMin / 64 }
				if cbox.yMax / 64 > max_height { max_height = cbox.yMax / 64 }
				let y = glyph.bitmap_top() as usize;
				pen.x += glyph.advance().x;
				pen.y += glyph.advance().y;
				if y > baseline_height {
					baseline_height = y;
				}
			}
			let max_height = -min_height + max_height;
			let reassign = match &self.surface {
				Some(surface) => {
					surface.get_width() < max_width as u16 || surface.get_height() < max_height as u16
				}
				None => true
			};
			if reassign {
				self.surface = Some(Surface::new(
					&[SWSurface],
					max_width as isize,
					max_height as isize,
					32,
					0xFF00_0000,
					0x00FF_0000,
					0x0000_FF00,
					0x0000_00FF,
				).unwrap());
			} else {
				self.surface.as_ref().unwrap().fill(RGBA(0, 0, 0, 0));
			}
			let scr = self.surface.as_ref().unwrap();
			let mut pen = ndless_freetype::Vector { x: 0, y: 0 };
			let max_width = scr.get_width();
			let max_height = scr.get_height();
			for letter in chars {
				self.font.set_transform(&mut self.matrix, &mut pen);
				self.font
				    .load_char(
					    letter.chars().next().unwrap() as usize,
					    ndless_freetype::face::LoadFlag::RENDER,
				    )
				    .unwrap();
				let glyph = self.font.glyph();
				let bitmap = glyph.bitmap();
				let x = glyph.bitmap_left() as usize;
				let y = baseline_height - glyph.bitmap_top() as usize;
				let mut col = 0;
				let width = bitmap.width() as usize;
				let x_max = x + width;
				let y_max = y + bitmap.rows() as usize;

				for (row, x_scaled) in (x..x_max).enumerate() {
					for y_scaled in y..y_max {
						if x_scaled < max_width as usize && y_scaled < max_height as usize {
							let alpha = bitmap.buffer()[col * width + row];
							scr.fill_rect(
								Some(crate::Rect {
									x: x_scaled as i16,
									y: y_scaled as i16,
									w: 1,
									h: 1,
								}),
								match self.color {
									RGB(r, g, b) => {
										RGBA(r, g, b, alpha)
									},
									RGBA(r, g, b, _) => {
										RGBA(r, g, b, alpha)
									},
								},
							);
							col += 1;
						}
					}
					col = 0;
				}
				pen.x += glyph.advance().x;
				pen.y += glyph.advance().y;
			}
			scr
		}
	}
}
