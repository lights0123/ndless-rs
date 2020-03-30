//! Inspection and manipulation of the process's environment.
//!
//! This module contains functions to inspect various aspects such as
//! process arguments, the current directory, and various
//! other important directories.

use alloc::vec::IntoIter;

use cstr_core::CStr;

use crate::io;
use crate::io::ErrorKind;
use crate::libc;
use crate::path::Path;
use crate::path::PathBuf;
use crate::prelude::*;

pub type Args = IntoIter<String>;

/// Returns the arguments which this program was started with.
///
/// The first element is traditionally the path of the executable, but it can be
/// set to arbitrary text, and may not even exist. This means this property should
/// not be relied upon for security purposes.
///
/// On Unix systems shell usually expands unquoted arguments with glob patterns
/// (such as `*` and `?`).
///
/// # Panics
///
/// The returned iterator will panic during iteration if any argument to the
/// process is not valid unicode.
///
/// # Examples
///
/// ```
/// use ndless::env;
///
/// // Prints each argument on a separate line
/// for argument in env::args() {
///     println!("{}", argument);
/// }
/// ```
pub fn args() -> Args {
	unsafe { &crate::ARGUMENTS }
		.map(|args| {
			args.iter()
				.map(|arg| {
					unsafe { CStr::from_ptr(*arg) }
						.to_str()
						.unwrap()
						.to_string()
				})
				.collect::<Vec<_>>()
		})
		.unwrap_or_default()
		.into_iter()
}

/// Returns the current working directory as a [`PathBuf`].
///
/// # Errors
///
/// Returns an [`Err`] if the current working directory value is invalid.
/// Possible cases:
///
/// * Current directory does not exist.
/// * There are insufficient permissions to access the current directory.
///
/// [`PathBuf`]: ../../std/path/struct.PathBuf.html
/// [`Err`]: ../../std/result/enum.Result.html#method.err
///
/// # Examples
///
/// ```
/// use ndless::env;
///
/// fn main() -> std::io::Result<()> {
///     let path = env::current_dir()?;
///     println!("The current directory is {}", path.display());
///     Ok(())
/// }
/// ```
pub fn current_dir() -> io::Result<PathBuf> {
	crate::file_io::sys::os::getcwd()
}

/// Changes the current working directory to the specified path.
///
/// Returns an [`Err`] if the operation fails.
///
/// [`Err`]: ../../std/result/enum.Result.html#method.err
///
/// # Examples
///
/// ```
/// use ndless::env;
/// use ndless::path::Path;
///
/// let root = Path::new("/");
/// assert!(env::set_current_dir(&root).is_ok());
/// println!("Successfully changed working directory to {}!", root.display());
/// ```
pub fn set_current_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
	crate::file_io::sys::os::chdir(path.as_ref())
}

pub fn get_documents_dir() -> io::Result<PathBuf> {
	unsafe {
		let ptr = libc::get_documents_dir();
		if ptr.is_null() {
			Err(ErrorKind::NotFound.into())
		} else {
			Ok(CStr::from_ptr(ptr).to_string_lossy().as_ref().into())
		}
	}
}
