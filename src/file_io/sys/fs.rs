use alloc::sync::Arc;

use core::fmt;
use core::mem;
use core::ptr;

use embedded_ffi::{CStr, CString, OsStr, OsString};
use ndless_sys::fileno;

use libc::{c_int, c_long, mode_t};
use libc::{lseek as lseek64, nuc_stat, readdir as readdir64};

use crate::file_io::os::unix::prelude::*;
use crate::file_io::sys::fd::FileDesc;
use crate::file_io::sys::time::SystemTime;
use crate::file_io::sys::{cvt, cvt_r};
pub use crate::file_io::sys_common::fs::remove_dir_all;
use crate::file_io::sys_common::{AsInner, FromInner};
use crate::io::{self, Error, ErrorKind, SeekFrom};
use crate::libc;
use crate::libc::{fopen, ftruncate};
use crate::path::{Path, PathBuf};
use alloc::borrow::ToOwned;

pub struct File(FileDesc);

#[derive(Clone)]
pub struct FileAttr {
	stat: nuc_stat,
}

// all DirEntry's will have a reference to this struct
struct InnerReadDir {
	dirp: Dir,
	root: PathBuf,
}

#[derive(Clone)]
pub struct ReadDir {
	inner: Arc<InnerReadDir>,
	end_of_stream: bool,
}

struct Dir(*mut libc::c_void);

unsafe impl Send for Dir {}

unsafe impl Sync for Dir {}

pub struct DirEntry {
	name: CString,
	dir: ReadDir,
}

#[derive(Clone, Debug)]
pub struct OpenOptions {
	// generic
	read: bool,
	write: bool,
	append: bool,
	truncate: bool,
	create: bool,
	create_new: bool,
	// system-specific
	custom_flags: i32,
	mode: mode_t,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FilePermissions {
	mode: mode_t,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileType {
	mode: mode_t,
}

#[derive(Debug)]
pub struct DirBuilder {
	mode: mode_t,
}

impl FileAttr {
	pub fn size(&self) -> u64 {
		self.stat.st_size as u64
	}
	pub fn perm(&self) -> FilePermissions {
		FilePermissions {
			mode: (self.stat.st_mode as mode_t),
		}
	}

	pub fn file_type(&self) -> FileType {
		FileType {
			mode: self.stat.st_mode as mode_t,
		}
	}
}

impl FileAttr {
	pub fn modified(&self) -> io::Result<SystemTime> {
		Ok(SystemTime::from(self.stat.st_mtime))
	}

	pub fn accessed(&self) -> io::Result<SystemTime> {
		Ok(SystemTime::from(self.stat.st_atime))
	}

	pub fn created(&self) -> io::Result<SystemTime> {
		Ok(SystemTime::from(self.stat.st_ctime))
	}
}

impl AsInner<nuc_stat> for FileAttr {
	fn as_inner(&self) -> &nuc_stat {
		&self.stat
	}
}

impl FilePermissions {
	pub fn readonly(&self) -> bool {
		// check if any class (owner, group, others) has write permission
		self.mode & 0o222 == 0
	}

	pub fn set_readonly(&mut self, readonly: bool) {
		if readonly {
			// remove write permission for all classes; equivalent to `chmod a-w <file>`
			self.mode &= !0o222;
		} else {
			// add write permission for all classes; equivalent to `chmod a+w <file>`
			self.mode |= 0o222;
		}
	}
	pub fn mode(&self) -> u32 {
		self.mode as u32
	}
}

impl FileType {
	pub fn is_dir(self) -> bool {
		self.is(libc::S_IFDIR)
	}
	pub fn is_file(self) -> bool {
		self.is(libc::S_IFREG)
	}

	pub fn is(self, mode: mode_t) -> bool {
		self.mode & libc::S_IFMT == mode
	}
}

impl FromInner<u32> for FilePermissions {
	fn from_inner(mode: u32) -> FilePermissions {
		FilePermissions {
			mode: mode as mode_t,
		}
	}
}

impl fmt::Debug for ReadDir {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// This will only be called from std::fs::ReadDir, which will add a "ReadDir()" frame.
		// Thus the result will be e g 'ReadDir("/home")'
		fmt::Debug::fmt(&*self.inner.root, f)
	}
}

impl Iterator for ReadDir {
	type Item = io::Result<DirEntry>;

	fn next(&mut self) -> Option<io::Result<DirEntry>> {
		if self.end_of_stream {
			return None;
		}

		unsafe {
			let mut ret = DirEntry {
				name: Default::default(),
				dir: self.clone(),
			};
			loop {
				crate::file_io::sys::os::set_errno(0);
				let dirent = readdir64(self.inner.dirp.0);
				if dirent.is_null() {
					self.end_of_stream = true;
					return match crate::file_io::sys::os::errno() {
						0 => None,
						err => Some(Err(Error::from_raw_os_error(err))),
					};
				}
				ret.name = CStr::from_ptr((*dirent).d_name.as_mut_ptr()).to_owned();
				if ret.name_bytes() != b"." && ret.name_bytes() != b".." {
					return Some(Ok(ret));
				}
			}
		}
	}
}

impl Drop for Dir {
	fn drop(&mut self) {
		let r = unsafe { libc::closedir(self.0) };
		debug_assert_eq!(r, 0);
	}
}

impl DirEntry {
	pub fn path(&self) -> PathBuf {
		self.dir
			.inner
			.root
			.join(OsStr::from_bytes(self.name_bytes()))
	}

	pub fn file_name(&self) -> OsString {
		OsStr::from_bytes(self.name_bytes()).to_os_string()
	}

	pub fn metadata(&self) -> io::Result<FileAttr> {
		stat(&self.path())
	}

	pub fn file_type(&self) -> io::Result<FileType> {
		stat(&self.path()).map(|m| m.file_type())
	}

	fn name_bytes(&self) -> &[u8] {
		self.name.as_bytes()
	}
}
macro_rules! c_str {
	($lit:expr) => {
		concat!($lit, "\0").as_ptr() as *const libc::c_char
	};
}

impl OpenOptions {
	pub fn new() -> OpenOptions {
		OpenOptions {
			// generic
			read: false,
			write: false,
			append: false,
			truncate: false,
			create: false,
			create_new: false,
			// system-specific
			custom_flags: 0,
			mode: 0,
		}
	}

	pub fn read(&mut self, read: bool) {
		self.read = read;
	}
	pub fn write(&mut self, write: bool) {
		self.write = write;
	}
	pub fn append(&mut self, append: bool) {
		self.append = append;
	}
	pub fn truncate(&mut self, truncate: bool) {
		self.truncate = truncate;
	}
	pub fn create(&mut self, create: bool) {
		self.create = create;
	}
	pub fn create_new(&mut self, create_new: bool) {
		self.create_new = create_new;
	}

	pub fn custom_flags(&mut self, flags: i32) {
		self.custom_flags = flags;
	}
	pub fn mode(&mut self, mode: u32) {
		self.mode = mode as mode_t;
	}

	fn c_mode(&self) -> *const libc::c_char {
		match (self.append, self.write, self.read, self.create) {
			(true, _, true, _) => c_str!("a+"),
			(true, _, _, _) => c_str!("a"),
			(_, true, true, true) => c_str!("w+"),
			(_, true, true, _) => c_str!("r+"),
			(_, true, _, _) => c_str!("w"),
			_ => c_str!("r"),
		}
	}
}

impl File {
	pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
		let path = cstr(path)?;
		File::open_c(&path, opts)
	}

	pub fn open_c(path: &CStr, opts: &OpenOptions) -> io::Result<File> {
		// TODO: fix fopen
		let file_ptr = unsafe { fopen(path.as_ptr(), opts.c_mode()) };
		if file_ptr.is_null() {
			return Err(crate::io::Error::last_os_error());
		}
		let fd = cvt_r(|| unsafe { fileno(file_ptr as _) })?;
		let fd = FileDesc::new(fd);

		Ok(File(fd))
	}

	/*pub fn file_attr(&self) -> io::Result<FileAttr> {
		let mut stat: stat64 = unsafe { mem::zeroed() };
		cvt(unsafe {
			fstat64(self.0.raw(), &mut stat)
		})?;
		Ok(FileAttr { stat: nuc_stat {
			st_dev: stat.st_dev as u16,
			st_ino: stat.st_ino as libc::c_uint,
			st_mode: stat.st_mode,
			st_nlink: stat.st_nlink as u16,
			st_uid: stat.st_uid as u16,
			st_gid: stat.st_gid as u16,
			st_rdev: stat.st_rdev as u16,
			st_size: stat.st_size as u32,
			st_atime: stat.st_atim.tv_sec as u32,
			st_mtime: stat.st_mtim.tv_sec as u32,
			st_ctime: stat.st_ctim.tv_sec as libc::uint,
		} })
	}*/

	pub fn fsync(&self) -> io::Result<()> {
		Ok(())
	}

	pub fn datasync(&self) -> io::Result<()> {
		Ok(())
	}

	pub fn truncate(&self, size: u64) -> io::Result<()> {
		#[cfg(target_os = "android")]
		return crate::sys::android::ftruncate64(self.0.raw(), size);

		#[cfg(not(target_os = "android"))]
		return cvt_r(|| unsafe { ftruncate(self.0.raw(), size as c_long) }).map(|_| ());
	}

	pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
		self.0.read(buf)
	}

	pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
		self.0.write(buf)
	}

	pub fn flush(&self) -> io::Result<()> {
		Ok(())
	}

	pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
		let (whence, pos) = match pos {
			// Casting to `i64` is fine, too large values will end up as
			// negative which will cause an error in `lseek64`.
			SeekFrom::Start(off) => (libc::SEEK_SET, off as i64),
			SeekFrom::End(off) => (libc::SEEK_END, off),
			SeekFrom::Current(off) => (libc::SEEK_CUR, off),
		};
		#[cfg(target_os = "emscripten")]
		let pos = pos as i32;
		let n = cvt(unsafe { lseek64(self.0.raw(), pos as libc::c_long, whence) })?;
		Ok(n as u64)
	}

	pub fn duplicate(&self) -> io::Result<File> {
		self.0.duplicate().map(File)
	}

	pub fn fd(&self) -> &FileDesc {
		&self.0
	}

	pub fn into_fd(self) -> FileDesc {
		self.0
	}
}

impl DirBuilder {
	pub fn new() -> DirBuilder {
		DirBuilder { mode: 0o777 }
	}

	pub fn mkdir(&self, p: &Path) -> io::Result<()> {
		let p = cstr(p)?;
		cvt(unsafe { libc::mkdir(p.as_ptr(), self.mode) })?;
		Ok(())
	}

	pub fn set_mode(&mut self, mode: u32) {
		self.mode = mode as mode_t;
	}
}

fn cstr(path: &Path) -> io::Result<CString> {
	Ok(CString::new(path.as_os_str().as_bytes())?)
}

impl FromInner<c_int> for File {
	fn from_inner(fd: c_int) -> File {
		File(FileDesc::new(fd))
	}
}

impl fmt::Debug for File {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fn get_path(_fd: c_int) -> Option<PathBuf> {
			// FIXME(#24570): implement this for other Unix platforms
			None
		}

		fn get_mode(_fd: c_int) -> Option<(bool, bool)> {
			// FIXME(#24570): implement this for other Unix platforms
			None
		}

		let fd = self.0.raw();
		let mut b = f.debug_struct("File");
		b.field("fd", &fd);
		if let Some(path) = get_path(fd) {
			b.field("path", &path);
		}
		if let Some((read, write)) = get_mode(fd) {
			b.field("read", &read).field("write", &write);
		}
		b.finish()
	}
}

pub fn readdir(p: &Path) -> io::Result<ReadDir> {
	let root = p.to_path_buf();
	let p = cstr(p)?;
	unsafe {
		let ptr = libc::opendir(p.as_ptr());
		if ptr.is_null() {
			Err(Error::last_os_error())
		} else {
			let inner = InnerReadDir {
				dirp: Dir(ptr),
				root,
			};
			Ok(ReadDir {
				inner: Arc::new(inner),
				end_of_stream: false,
			})
		}
	}
}

pub fn unlink(p: &Path) -> io::Result<()> {
	let p = cstr(p)?;
	cvt(unsafe { libc::unlink(p.as_ptr()) })?;
	Ok(())
}

pub fn rename(old: &Path, new: &Path) -> io::Result<()> {
	let old = cstr(old)?;
	let new = cstr(new)?;
	cvt(unsafe { libc::rename(old.as_ptr(), new.as_ptr()) })?;
	Ok(())
}

pub fn rmdir(p: &Path) -> io::Result<()> {
	let p = cstr(p)?;
	cvt(unsafe { libc::rmdir(p.as_ptr()) })?;
	Ok(())
}

pub fn link(src: &Path, dst: &Path) -> io::Result<()> {
	let src = cstr(src)?;
	let dst = cstr(dst)?;
	cvt(unsafe { libc::link(src.as_ptr(), dst.as_ptr()) })?;
	Ok(())
}

pub fn stat(p: &Path) -> io::Result<FileAttr> {
	let p = cstr(p)?;
	let mut stat: nuc_stat = unsafe { mem::zeroed() };
	cvt(unsafe { nuc_stat(p.as_ptr(), &mut stat) })?;
	let msg = alloc::format!("stat: {:#?}\n", stat);
	unsafe { libc::write(1, msg.as_ptr() as _, msg.len()) };
	Ok(FileAttr { stat })
}

pub fn canonicalize(p: &Path) -> io::Result<PathBuf> {
	let path = CString::new(p.as_os_str().as_bytes())?;
	let buf;
	unsafe {
		let r = libc::realpath(path.as_ptr(), ptr::null_mut());
		if r.is_null() {
			return Err(io::Error::last_os_error());
		}
		buf = CStr::from_ptr(r).to_bytes().to_vec();
		libc::free(r as *mut _);
	}
	Ok(PathBuf::from(OsString::from_vec(buf)))
}

fn open_from(from: &Path) -> io::Result<(crate::fs::File, crate::fs::Metadata)> {
	use crate::fs::File;

	let reader = File::open(from)?;
	let metadata = from.metadata()?;
	if !metadata.is_file() {
		return Err(Error::new(
			ErrorKind::InvalidInput,
			"the source path is not an existing regular file",
		));
	}
	Ok((reader, metadata))
}

fn open_to_and_set_permissions(
	to: &Path,
	reader_metadata: crate::fs::Metadata,
) -> io::Result<(crate::fs::File, crate::fs::Metadata)> {
	use crate::fs::OpenOptions;

	let perm = reader_metadata.permissions();
	let writer = OpenOptions::new()
		// create the file with the correct mode right away
		.mode(perm.mode())
		.write(true)
		.create(true)
		.truncate(true)
		.open(to)?;
	let writer_metadata = to.metadata()?;
	Ok((writer, writer_metadata))
}

pub fn copy(from: &Path, to: &Path) -> io::Result<u64> {
	let (mut reader, reader_metadata) = open_from(from)?;
	let (mut writer, _) = open_to_and_set_permissions(to, reader_metadata)?;

	io::copy(&mut reader, &mut writer)
}
