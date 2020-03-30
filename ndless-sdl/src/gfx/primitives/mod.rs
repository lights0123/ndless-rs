use ndless::prelude::*;

pub use crate::video::{Color, Surface};

pub mod ll;

pub type Point = (i16, i16);

pub trait Graphics {
	fn draw_pixel(&self, point: Point, color: Color);

	fn draw_horiz_line(&self, x1: i16, x2: i16, y: i16, color: Color);
	fn draw_vert_line(&self, x: i16, y1: i16, y2: i16, color: Color);
	fn draw_line(&self, point1: Point, point2: Point, color: Color);
	fn draw_antialiased_line(&self, point1: Point, point2: Point, color: Color);
	fn draw_thick_line(&self, point1: Point, point2: Point, width: u8, color: Color);

	fn draw_rectangle(&self, point1: Point, point2: Point, color: Color);
	fn draw_rounded_rectangle(&self, point1: Point, point2: Point, radius: i16, color: Color);
	fn draw_filled_rectangle(&self, point1: Point, point2: Point, color: Color);
	fn draw_rounded_filled_rectangle(
		&self,
		point1: Point,
		point2: Point,
		radius: i16,
		color: Color,
	);

	fn draw_circle(&self, center: Point, radius: i16, color: Color);
	fn draw_filled_circle(&self, center: Point, radius: i16, color: Color);
	fn draw_antialiased_circle(&self, center: Point, radius: i16, color: Color);

	fn draw_arc(&self, center: Point, radius: i16, start: i16, end: i16, color: Color);

	fn draw_ellipse(&self, center: Point, x_radius: i16, y_radius: i16, color: Color);
	fn draw_filled_ellipse(&self, center: Point, x_radius: i16, y_radius: i16, color: Color);
	fn draw_antialiased_ellipse(
		&self,
		center: Point,
		x_radius: i16,
		y_radius: i16,
		color: Color,
	);

	fn draw_pie(&self, center: Point, radius: i16, start: i16, end: i16, color: Color);
	fn draw_filled_pie(&self, center: Point, radius: i16, start: i16, end: i16, color: Color);

	fn draw_triangle(&self, points: [Point; 3], color: Color);
	fn draw_filled_triangle(&self, points: [Point; 3], color: Color);
	fn draw_antialiased_triangle(&self, points: [Point; 3], color: Color);

	fn draw_polygon(&self, points: &[Point], color: Color) {
		let (x, y): (Vec<_>, Vec<_>) = points.iter().cloned().unzip();
		self.draw_polygon_list(&x[..], &y[..], color)
	}

	fn draw_filled_polygon(&self, points: &[Point], color: Color) {
		let (x, y): (Vec<_>, Vec<_>) = points.iter().cloned().unzip();
		self.draw_filled_polygon_list(&x[..], &y[..], color)
	}

	fn draw_antialiased_polygon(&self, points: &[Point], color: Color) {
		let (x, y): (Vec<_>, Vec<_>) = points.iter().cloned().unzip();
		self.draw_antialiased_polygon_list(&x[..], &y[..], color)
	}

	fn draw_textured_polygon(
		&self,
		points: &[Point],
		texture: &Surface,
		texture_offset: Point,
	) {
		let (x, y): (Vec<_>, Vec<_>) = points.iter().cloned().unzip();
		self.draw_textured_polygon_list(&x[..], &y[..], texture, texture_offset)
	}

	fn draw_polygon_list(&self, x_points: &[i16], y_points: &[i16], color: Color);
	fn draw_filled_polygon_list(&self, x_points: &[i16], y_points: &[i16], color: Color);
	fn draw_antialiased_polygon_list(&self, x_points: &[i16], y_points: &[i16], color: Color);
	fn draw_textured_polygon_list(
		&self,
		x_points: &[i16],
		y_points: &[i16],
		texture: &Surface,
		texture_offset: Point,
	);

	fn draw_bezier(&self, points: &[Point], interpolation: i32, color: Color) {
		let (x, y): (Vec<_>, Vec<_>) = points.iter().cloned().unzip();
		self.draw_bezier_list(&x[..], &y[..], interpolation, color)
	}
	fn draw_bezier_list(
		&self,
		x_points: &[i16],
		y_points: &[i16],
		interpolation: i32,
		color: Color,
	);
}

impl Graphics for Surface {
	fn draw_pixel(&self, point: Point, color: Color) {
		unsafe {
			ll::pixelColor(
				self.raw,
				point.0,
				point.1,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_horiz_line(&self, x1: i16, x2: i16, y: i16, color: Color) {
		unsafe {
			ll::hlineColor(self.raw, x1, x2, y, color.to_mapped((*self.raw).format));
		}
	}

	fn draw_vert_line(&self, x: i16, y1: i16, y2: i16, color: Color) {
		unsafe {
			ll::vlineColor(self.raw, x, y1, y2, color.to_mapped((*self.raw).format));
		}
	}

	fn draw_line(&self, point1: Point, point2: Point, color: Color) {
		unsafe {
			ll::lineColor(
				self.raw,
				point1.0,
				point1.1,
				point2.0,
				point2.1,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_antialiased_line(&self, point1: Point, point2: Point, color: Color) {
		unsafe {
			ll::aalineColor(
				self.raw,
				point1.0,
				point1.1,
				point2.0,
				point2.1,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_thick_line(&self, point1: Point, point2: Point, width: u8, color: Color) {
		unsafe {
			ll::thickLineColor(
				self.raw,
				point1.0,
				point1.1,
				point2.0,
				point2.1,
				width,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_rectangle(&self, point1: Point, point2: Point, color: Color) {
		unsafe {
			ll::rectangleColor(
				self.raw,
				point1.0,
				point1.1,
				point2.0,
				point2.1,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_rounded_rectangle(&self, point1: Point, point2: Point, radius: i16, color: Color) {
		unsafe {
			ll::roundedRectangleColor(
				self.raw,
				point1.0,
				point1.1,
				point2.0,
				point2.1,
				radius,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_filled_rectangle(&self, point1: Point, point2: Point, color: Color) {
		unsafe {
			ll::boxColor(
				self.raw,
				point1.0,
				point1.1,
				point2.0,
				point2.1,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_rounded_filled_rectangle(
		&self,
		point1: Point,
		point2: Point,
		radius: i16,
		color: Color,
	) {
		unsafe {
			ll::roundedBoxColor(
				self.raw,
				point1.0,
				point1.1,
				point2.0,
				point2.1,
				radius,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_circle(&self, center: Point, radius: i16, color: Color) {
		unsafe {
			ll::circleColor(
				self.raw,
				center.0,
				center.1,
				radius,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_filled_circle(&self, center: Point, radius: i16, color: Color) {
		unsafe {
			ll::filledCircleColor(
				self.raw,
				center.0,
				center.1,
				radius,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_antialiased_circle(&self, center: Point, radius: i16, color: Color) {
		unsafe {
			ll::aacircleColor(
				self.raw,
				center.0,
				center.1,
				radius,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_arc(&self, center: Point, radius: i16, start: i16, end: i16, color: Color) {
		unsafe {
			ll::arcColor(
				self.raw,
				center.0,
				center.1,
				radius,
				start,
				end,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_ellipse(&self, center: Point, x_radius: i16, y_radius: i16, color: Color) {
		unsafe {
			ll::ellipseColor(
				self.raw,
				center.0,
				center.1,
				x_radius,
				y_radius,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_filled_ellipse(&self, center: Point, x_radius: i16, y_radius: i16, color: Color) {
		unsafe {
			ll::filledEllipseColor(
				self.raw,
				center.0,
				center.1,
				x_radius,
				y_radius,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_antialiased_ellipse(
		&self,
		center: Point,
		x_radius: i16,
		y_radius: i16,
		color: Color,
	) {
		unsafe {
			ll::aaellipseColor(
				self.raw,
				center.0,
				center.1,
				x_radius,
				y_radius,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_pie(&self, center: Point, radius: i16, start: i16, end: i16, color: Color) {
		unsafe {
			ll::pieColor(
				self.raw,
				center.0,
				center.1,
				radius,
				start,
				end,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_filled_pie(&self, center: Point, radius: i16, start: i16, end: i16, color: Color) {
		unsafe {
			ll::filledPieColor(
				self.raw,
				center.0,
				center.1,
				radius,
				start,
				end,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_triangle(&self, points: [Point; 3], color: Color) {
		unsafe {
			ll::trigonColor(
				self.raw,
				points[0].0,
				points[0].1,
				points[1].0,
				points[1].1,
				points[2].0,
				points[2].1,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_filled_triangle(&self, points: [Point; 3], color: Color) {
		unsafe {
			ll::filledTrigonColor(
				self.raw,
				points[0].0,
				points[0].1,
				points[1].0,
				points[1].1,
				points[2].0,
				points[2].1,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_antialiased_triangle(&self, points: [Point; 3], color: Color) {
		unsafe {
			ll::aatrigonColor(
				self.raw,
				points[0].0,
				points[0].1,
				points[1].0,
				points[1].1,
				points[2].0,
				points[2].1,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_polygon_list(&self, x_points: &[i16], y_points: &[i16], color: Color) {
		let x_len = x_points.len();
		let y_len = y_points.len();
		unsafe {
			ll::polygonColor(
				self.raw,
				x_points.as_ptr(),
				y_points.as_ptr(),
				x_len.min(y_len) as i32,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_filled_polygon_list(&self, x_points: &[i16], y_points: &[i16], color: Color) {
		let x_len = x_points.len();
		let y_len = y_points.len();
		unsafe {
			ll::filledPolygonColor(
				self.raw,
				x_points.as_ptr(),
				y_points.as_ptr(),
				x_len.min(y_len) as i32,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_antialiased_polygon_list(&self, x_points: &[i16], y_points: &[i16], color: Color) {
		let x_len = x_points.len();
		let y_len = y_points.len();
		unsafe {
			ll::aapolygonColor(
				self.raw,
				x_points.as_ptr(),
				y_points.as_ptr(),
				x_len.min(y_len) as i32,
				color.to_mapped((*self.raw).format),
			);
		}
	}

	fn draw_textured_polygon_list(
		&self,
		x_points: &[i16],
		y_points: &[i16],
		texture: &Surface,
		texture_offset: Point,
	) {
		let x_len = x_points.len();
		let y_len = y_points.len();
		unsafe {
			ll::texturedPolygon(
				self.raw,
				x_points.as_ptr(),
				y_points.as_ptr(),
				x_len.min(y_len) as i32,
				texture.raw,
				i32::from(texture_offset.0),
				i32::from(texture_offset.1),
			);
		}
	}

	fn draw_bezier_list(
		&self,
		x_points: &[i16],
		y_points: &[i16],
		interpolation: i32,
		color: Color,
	) {
		let x_len = x_points.len();
		let y_len = y_points.len();
		unsafe {
			ll::bezierColor(
				self.raw,
				x_points.as_ptr(),
				y_points.as_ptr(),
				x_len.min(y_len) as i32,
				interpolation,
				color.to_mapped((*self.raw).format),
			);
		}
	}
}
