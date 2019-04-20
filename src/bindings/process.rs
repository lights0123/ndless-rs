/// ## WARNING
///
/// This **will** leak memory without careful planning, as it does not run any destructors!
/// You need to make sure that all scopes end before calling this!
/// You can use
///
/// ```rust
/// fn main() {
/// 	{
/// 		// Main code
/// 		let a = vec![5];
///  	}
/// 	ndless::process::abort();
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
/// 	{
/// 		// Main code
/// 		let a = vec![5];
/// 	}
/// 	ndless::process::exit(1);
/// }
/// ```
/// or
/// ```rust
/// fn main() {
/// 	ndless::process::exit({
/// 		// Main code
///			let a = vec![5];
/// 		0
/// 	});
/// }
/// ```
/// to ensure that no memory leaks.
pub fn exit(code: i32) -> ! {
    unsafe { ndless_sys::exit(code) }
}
