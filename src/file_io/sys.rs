use crate::io::ErrorKind;
use crate::libc;

pub mod ext;
pub mod fd;
pub mod fs;
pub mod io;
pub mod memchr;
pub mod os;
pub mod path;
pub mod platform;
pub mod stdio;
pub mod time;
pub fn decode_error_kind(errno: i32) -> ErrorKind {
	match errno as libc::c_int {
		libc::ECONNREFUSED => ErrorKind::ConnectionRefused,
		libc::ECONNRESET => ErrorKind::ConnectionReset,
		libc::EPERM | libc::EACCES => ErrorKind::PermissionDenied,
		libc::EPIPE => ErrorKind::BrokenPipe,
		libc::ENOTCONN => ErrorKind::NotConnected,
		libc::ECONNABORTED => ErrorKind::ConnectionAborted,
		libc::EADDRNOTAVAIL => ErrorKind::AddrNotAvailable,
		libc::EADDRINUSE => ErrorKind::AddrInUse,
		libc::ENOENT => ErrorKind::NotFound,
		libc::EINTR => ErrorKind::Interrupted,
		libc::EINVAL => ErrorKind::InvalidInput,
		libc::ETIMEDOUT => ErrorKind::TimedOut,
		libc::EEXIST => ErrorKind::AlreadyExists,

		// These two constants can have the same value on some systems,
		// but different values on others, so we can't use a match
		// clause
		x if x == libc::EAGAIN || x == libc::EWOULDBLOCK => ErrorKind::WouldBlock,

		_ => ErrorKind::Other,
	}
}
#[doc(hidden)]
pub trait IsMinusOne {
	fn is_minus_one(&self) -> bool;
}

macro_rules! impl_is_minus_one {
    ($($t:ident)*) => ($(impl IsMinusOne for $t {
        fn is_minus_one(&self) -> bool {
            *self == -1
        }
    })*)
}

impl_is_minus_one! { i8 i16 i32 i64 isize }
pub fn cvt<T: IsMinusOne>(t: T) -> crate::io::Result<T> {
	if t.is_minus_one() {
		Err(crate::io::Error::last_os_error())
	} else {
		Ok(t)
	}
}

pub fn cvt_r<T, F>(mut f: F) -> crate::io::Result<T>
where
	T: IsMinusOne,
	F: FnMut() -> T,
{
	loop {
		match cvt(f()) {
			Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
			other => return other,
		}
	}
}
