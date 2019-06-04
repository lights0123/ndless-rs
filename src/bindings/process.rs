use core::fmt;

/// ## WARNING
///
/// This **will** leak memory without careful planning, as it does not run any destructors!
/// You need to make sure that all scopes end before calling this!
/// You can use
///
/// ```rust
/// fn main() {
///     {
///         // Main code
///         let a = vec![5];
///     }
///     ndless::process::abort();
/// }
/// ```
/// to ensure that no memory leaks.
pub fn abort() -> ! {
	unsafe { ndless_sys::abort() }
}

/// ## WARNING
///
/// This **will** leak memory without careful planning, as it does not run any destructors!
/// You need to make sure that all scopes end before calling this!
/// You can either use
///
/// ```rust
/// fn main() {
///     {
///         // Main code
///         let a = vec![5];
///     }
///     ndless::process::exit(1);
/// }
/// ```
/// or
/// ```rust
/// fn main() {
///     ndless::process::exit({
///         // Main code
///	        let a = vec![5];
///         0
///     });
/// }
/// ```
/// to ensure that no memory leaks.
pub fn exit(code: i32) -> ! {
	unsafe { ndless_sys::exit(code) }
}

/// A trait for implementing arbitrary return types in the `main` function.
///
/// The c-main function only supports to return integers as return type.
/// So, every type implementing the `Termination` trait has to be converted
/// to an integer.
///
/// The default implementations are returning 0 to indicate
/// a successful execution. In case of a failure, 1 is returned.
pub trait Termination {
	/// Is called to get the representation of the value as status code.
	/// This status code is returned to the operating system.
	fn report(self) -> i32;
}

impl Termination for () {
	#[inline]
	fn report(self) -> i32 {
		0
	}
}

impl Termination for i32 {
	#[inline]
	fn report(self) -> i32 {
		self
	}
}

impl<E: fmt::Debug> Termination for Result<(), E> {
	fn report(self) -> i32 {
		match self {
			Ok(()) => ().report(),
			Err(err) => Err::<!, _>(err).report(),
		}
	}
}

impl Termination for ! {
	fn report(self) -> i32 {
		self
	}
}

impl<E: fmt::Debug> Termination for Result<!, E> {
	fn report(self) -> i32 {
		let err = match self {
			Err(err) => err,
			_ => unreachable!(),
		};
		crate::msg::msg("Error", &alloc::format!("Error: {:?}", err));
		1
	}
}
