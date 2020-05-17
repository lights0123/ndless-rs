pub mod ll {
	/// Structure holding the state and timing information of the framerate
	/// controller.
	#[repr(C)]
	#[derive(Debug, Copy, Clone, Default)]
	pub struct FPSmanager {
		pub framecount: u32,
		pub rateticks: f32,
		pub lastticks: u32,
		pub rate: u32,
	}

	extern "C" {
		pub fn SDL_initFramerate(manager: *mut FPSmanager);
		pub fn SDL_setFramerate(manager: *mut FPSmanager, rate: u32) -> cty::c_int;
		pub fn SDL_getFramerate(manager: *mut FPSmanager) -> cty::c_int;
		pub fn SDL_getFramecount(manager: *mut FPSmanager) -> cty::c_int;
		pub fn SDL_framerateDelay(manager: *mut FPSmanager);
	}
}

/// Example:
/// ```
/// let manager = FPS::new();
/// manager.framerate(60); // 60 FPS
/// loop {
///     println!("loop");
///     manager.delay();
/// }
/// ```
#[derive(Debug)]
pub struct FPS {
	manager: ll::FPSmanager,
}

impl Default for FPS {
	fn default() -> Self {
		let mut manager: ll::FPSmanager = Default::default();
		unsafe { ll::SDL_initFramerate(&mut manager) };
		Self { manager }
	}
}

impl FPS {
	pub fn new() -> Self {
		Default::default()
	}
	pub fn delay(&mut self) {
		unsafe { ll::SDL_framerateDelay(&mut self.manager) }
	}
	pub fn framerate(&mut self, rate: u32) {
		unsafe {
			ll::SDL_setFramerate(&mut self.manager, rate);
		}
	}
	pub fn get_framerate(&self) -> u32 {
		self.manager.rate
	}
	pub fn frame_count(&self) -> u32 {
		self.manager.framecount
	}
}
