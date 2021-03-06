pub mod error;
pub mod fs;
pub mod io;
mod os;
pub mod path;
pub(crate) mod sys;
mod sys_common;
pub mod time;
mod memchr {
	#[inline]
	/// A safe interface to `memchr`.
	///
	/// Returns the index corresponding to the first occurrence of `needle` in
	/// `haystack`, or `None` if one is not found.
	///
	/// memchr reduces to super-optimized machine code at around an order of
	/// magnitude faster than `haystack.iter().position(|&b| b == needle)`.
	/// (See benchmarks.)
	///
	/// # Examples
	///
	/// This shows how to find the first position of a byte in a byte string.
	///
	/// ```ignore (cannot-doctest-private-modules)
	/// use memchr::memchr;
	///
	/// let haystack = b"the quick brown fox";
	/// assert_eq!(memchr(b'k', haystack), Some(8));
	/// ```
	pub fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
		crate::file_io::sys::memchr::memchr(needle, haystack)
	}
	/// A safe interface to `memrchr`.
	///
	/// Returns the index corresponding to the last occurrence of `needle` in
	/// `haystack`, or `None` if one is not found.
	///
	/// # Examples
	///
	/// This shows how to find the last position of a byte in a byte string.
	///
	/// ```ignore (cannot-doctest-private-modules)
	/// use memchr::memrchr;
	///
	/// let haystack = b"the quick brown fox";
	/// assert_eq!(memrchr(b'o', haystack), Some(17));
	/// ```
	#[inline]
	pub fn memrchr(needle: u8, haystack: &[u8]) -> Option<usize> {
		crate::file_io::sys::memchr::memrchr(needle, haystack)
	}
}
