#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::video::ll::SDL_Surface;
extern "C" {
	pub fn pixelColor(dst: *mut SDL_Surface, x: i16, y: i16, color: u32) -> cty::c_int;

	pub fn hlineColor(dst: *mut SDL_Surface, x1: i16, x2: i16, y: i16, color: u32) -> cty::c_int;

	pub fn vlineColor(dst: *mut SDL_Surface, x: i16, y1: i16, y2: i16, color: u32) -> cty::c_int;

	pub fn rectangleColor(
		dst: *mut SDL_Surface,
		x1: i16,
		y1: i16,
		x2: i16,
		y2: i16,
		color: u32,
	) -> cty::c_int;

	pub fn roundedRectangleColor(
		dst: *mut SDL_Surface,
		x1: i16,
		y1: i16,
		x2: i16,
		y2: i16,
		rad: i16,
		color: u32,
	) -> cty::c_int;

	pub fn boxColor(
		dst: *mut SDL_Surface,
		x1: i16,
		y1: i16,
		x2: i16,
		y2: i16,
		color: u32,
	) -> cty::c_int;

	pub fn roundedBoxColor(
		dst: *mut SDL_Surface,
		x1: i16,
		y1: i16,
		x2: i16,
		y2: i16,
		rad: i16,
		color: u32,
	) -> cty::c_int;

	pub fn lineColor(
		dst: *mut SDL_Surface,
		x1: i16,
		y1: i16,
		x2: i16,
		y2: i16,
		color: u32,
	) -> cty::c_int;

	pub fn aalineColor(
		dst: *mut SDL_Surface,
		x1: i16,
		y1: i16,
		x2: i16,
		y2: i16,
		color: u32,
	) -> cty::c_int;

	pub fn thickLineColor(
		dst: *mut SDL_Surface,
		x1: i16,
		y1: i16,
		x2: i16,
		y2: i16,
		width: u8,
		color: u32,
	) -> cty::c_int;

	pub fn circleColor(dst: *mut SDL_Surface, x: i16, y: i16, rad: i16, color: u32) -> cty::c_int;

	pub fn arcColor(
		dst: *mut SDL_Surface,
		x: i16,
		y: i16,
		rad: i16,
		start: i16,
		end: i16,
		color: u32,
	) -> cty::c_int;

	pub fn aacircleColor(dst: *mut SDL_Surface, x: i16, y: i16, rad: i16, color: u32)
		-> cty::c_int;

	pub fn filledCircleColor(
		dst: *mut SDL_Surface,
		x: i16,
		y: i16,
		r: i16,
		color: u32,
	) -> cty::c_int;

	pub fn ellipseColor(
		dst: *mut SDL_Surface,
		x: i16,
		y: i16,
		rx: i16,
		ry: i16,
		color: u32,
	) -> cty::c_int;

	pub fn aaellipseColor(
		dst: *mut SDL_Surface,
		x: i16,
		y: i16,
		rx: i16,
		ry: i16,
		color: u32,
	) -> cty::c_int;

	pub fn filledEllipseColor(
		dst: *mut SDL_Surface,
		x: i16,
		y: i16,
		rx: i16,
		ry: i16,
		color: u32,
	) -> cty::c_int;

	pub fn pieColor(
		dst: *mut SDL_Surface,
		x: i16,
		y: i16,
		rad: i16,
		start: i16,
		end: i16,
		color: u32,
	) -> cty::c_int;

	pub fn filledPieColor(
		dst: *mut SDL_Surface,
		x: i16,
		y: i16,
		rad: i16,
		start: i16,
		end: i16,
		color: u32,
	) -> cty::c_int;

	pub fn trigonColor(
		dst: *mut SDL_Surface,
		x1: i16,
		y1: i16,
		x2: i16,
		y2: i16,
		x3: i16,
		y3: i16,
		color: u32,
	) -> cty::c_int;

	pub fn aatrigonColor(
		dst: *mut SDL_Surface,
		x1: i16,
		y1: i16,
		x2: i16,
		y2: i16,
		x3: i16,
		y3: i16,
		color: u32,
	) -> cty::c_int;

	pub fn filledTrigonColor(
		dst: *mut SDL_Surface,
		x1: i16,
		y1: i16,
		x2: i16,
		y2: i16,
		x3: i16,
		y3: i16,
		color: u32,
	) -> cty::c_int;

	pub fn polygonColor(
		dst: *mut SDL_Surface,
		vx: *const i16,
		vy: *const i16,
		n: cty::c_int,
		color: u32,
	) -> cty::c_int;

	pub fn aapolygonColor(
		dst: *mut SDL_Surface,
		vx: *const i16,
		vy: *const i16,
		n: cty::c_int,
		color: u32,
	) -> cty::c_int;

	pub fn filledPolygonColor(
		dst: *mut SDL_Surface,
		vx: *const i16,
		vy: *const i16,
		n: cty::c_int,
		color: u32,
	) -> cty::c_int;

	pub fn texturedPolygon(
		dst: *mut SDL_Surface,
		vx: *const i16,
		vy: *const i16,
		n: cty::c_int,
		texture: *mut SDL_Surface,
		texture_dx: cty::c_int,
		texture_dy: cty::c_int,
	) -> cty::c_int;

	pub fn bezierColor(
		dst: *mut SDL_Surface,
		vx: *const i16,
		vy: *const i16,
		n: cty::c_int,
		s: cty::c_int,
		color: u32,
	) -> cty::c_int;

	pub fn gfxPrimitivesSetFont(fontdata: *const cty::c_void, cw: u32, ch: u32);

	pub fn gfxPrimitivesSetFontRotation(rotation: u32);

	pub fn characterColor(
		dst: *mut SDL_Surface,
		x: i16,
		y: i16,
		c: cty::c_char,
		color: u32,
	) -> cty::c_int;

	pub fn stringColor(
		dst: *mut SDL_Surface,
		x: i16,
		y: i16,
		s: *const cty::c_char,
		color: u32,
	) -> cty::c_int;
}
