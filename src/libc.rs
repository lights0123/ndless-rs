pub use cty::*;
pub use ndless_sys::nuc_readdir as readdir;
pub use ndless_sys::NU_Open as open;
pub use ndless_sys::*;
pub use ndless_sys::{
	nuc_closedir as closedir, nuc_opendir as opendir, stat as lstat, NU_Set_Current_Dir as chdir,
	NU_Truncate as ftruncate,
};

extern "C" {
	pub fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
	pub fn memrchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
	pub fn fcntl(fd: c_int, cmd: c_int, _: ...) -> c_int;
}

#[allow(non_camel_case_types)]
pub type mode_t = c_uint;

pub const FD_CLOEXEC: c_int = 0x1;

pub const EWOULDBLOCK: c_int = EAGAIN;

pub const O_NONBLOCK: c_int = 2048;

pub const S_IFIFO: mode_t = 4096;
pub const S_IFCHR: mode_t = 8192;
pub const S_IFBLK: mode_t = 24576;
pub const S_IFDIR: mode_t = 16384;
pub const S_IFREG: mode_t = 32768;
pub const S_IFSOCK: mode_t = 49152;
pub const S_IFMT: mode_t = 61440;

pub const _SC_PAGESIZE: c_int = 30;
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const EINTR: c_int = 4;
pub const EAGAIN: c_int = 11;
pub const EACCES: c_int = 13;
pub const EEXIST: c_int = 17;
pub const EINVAL: c_int = 22;
pub const EPIPE: c_int = 32;
pub const ERANGE: c_int = 34;
pub const EADDRINUSE: c_int = 98;
pub const EADDRNOTAVAIL: c_int = 99;
pub const ECONNABORTED: c_int = 103;
pub const ECONNRESET: c_int = 104;
pub const ENOTCONN: c_int = 107;
pub const ETIMEDOUT: c_int = 110;
pub const ECONNREFUSED: c_int = 111;

pub const F_DUPFD: c_int = 0;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;

pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;

pub const STDOUT_FILENO: c_int = 1;

#[repr(C)]
pub struct iovec {
	pub iov_base: *mut c_void,
	pub iov_len: size_t,
}
