//! Unix-specific extensions to primitives in the `std::fs` module.

use crate::file_io::sys::platform::fs::MetadataExt as UnixMetadataExt;
use crate::file_io::sys_common::{AsInner, AsInnerMut, FromInner};
use crate::fs::{self, OpenOptions, Permissions};
use crate::libc;
/// Unix-specific extensions to [`fs::Permissions`].
///
/// [`fs::Permissions`]: ../../../../std/fs/struct.Permissions.html

pub trait PermissionsExt {
	/// Returns the underlying raw `st_mode` bits that contain the standard
	/// Unix permissions for this file.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs::File;
	/// use std::os::unix::fs::PermissionsExt;
	///
	/// fn main() -> std::io::Result<()> {
	///     let f = File::create("foo.txt")?;
	///     let metadata = f.metadata()?;
	///     let permissions = metadata.permissions();
	///
	///     println!("permissions: {}", permissions.mode());
	///     Ok(()) }
	/// ```

	fn mode(&self) -> u32;

	/// Sets the underlying raw bits for this set of permissions.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs::File;
	/// use std::os::unix::fs::PermissionsExt;
	///
	/// fn main() -> std::io::Result<()> {
	///     let f = File::create("foo.txt")?;
	///     let metadata = f.metadata()?;
	///     let mut permissions = metadata.permissions();
	///
	///     permissions.set_mode(0o644); // Read/write for owner and read for others.
	///     assert_eq!(permissions.mode(), 0o644);
	///     Ok(()) }
	/// ```

	fn set_mode(&mut self, mode: u32);

	/// Creates a new instance of `Permissions` from the given set of Unix
	/// permission bits.
	///
	/// # Examples
	///
	/// ```
	/// use std::fs::Permissions;
	/// use std::os::unix::fs::PermissionsExt;
	///
	/// // Read/write for owner and read for others.
	/// let permissions = Permissions::from_mode(0o644);
	/// assert_eq!(permissions.mode(), 0o644);
	/// ```

	fn from_mode(mode: u32) -> Self;
}

impl PermissionsExt for Permissions {
	fn mode(&self) -> u32 {
		self.as_inner().mode()
	}

	fn set_mode(&mut self, mode: u32) {
		*self = Permissions::from_inner(FromInner::from_inner(mode));
	}

	fn from_mode(mode: u32) -> Permissions {
		Permissions::from_inner(FromInner::from_inner(mode))
	}
}

/// Unix-specific extensions to [`fs::OpenOptions`].
///
/// [`fs::OpenOptions`]: ../../../../std/fs/struct.OpenOptions.html

pub trait OpenOptionsExt {
	/// Sets the mode bits that a new file will be created with.
	///
	/// If a new file is created as part of a `File::open_opts` call then this
	/// specified `mode` will be used as the permission bits for the new file.
	/// If no `mode` is set, the default of `0o666` will be used.
	/// The operating system masks out bits with the systems `umask`, to produce
	/// the final permissions.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs::OpenOptions;
	/// use std::os::unix::fs::OpenOptionsExt;
	///
	/// # fn main() {
	/// let mut options = OpenOptions::new();
	/// options.mode(0o644); // Give read/write for owner and read for others.
	/// let file = options.open("foo.txt");
	/// # }
	/// ```

	fn mode(&mut self, mode: u32) -> &mut Self;

	/// Pass custom flags to the `flags` argument of `open`.
	///
	/// The bits that define the access mode are masked out with `O_ACCMODE`, to
	/// ensure they do not interfere with the access mode set by Rusts options.
	///
	/// Custom flags can only set flags, not remove flags set by Rusts options.
	/// This options overwrites any previously set custom flags.
	///
	/// # Examples
	///
	/// ```no_run
	/// # #![feature(rustc_private)]
	/// use std::fs::OpenOptions;
	/// use std::os::unix::fs::OpenOptionsExt;
	///
	/// # fn main() {
	/// let mut options = OpenOptions::new();
	/// options.write(true);
	/// if cfg!(unix) {
	///     options.custom_flags(libc::O_NOFOLLOW);
	/// }
	/// let file = options.open("foo.txt");
	/// # }
	/// ```

	fn custom_flags(&mut self, flags: i32) -> &mut Self;
}

impl OpenOptionsExt for OpenOptions {
	fn mode(&mut self, mode: u32) -> &mut OpenOptions {
		self.as_inner_mut().mode(mode);
		self
	}

	fn custom_flags(&mut self, flags: i32) -> &mut OpenOptions {
		self.as_inner_mut().custom_flags(flags);
		self
	}
}

/// Unix-specific extensions to [`fs::Metadata`].
///
/// [`fs::Metadata`]: ../../../../std/fs/struct.Metadata.html

pub trait MetadataExt {
	/// Returns the ID of the device containing the file.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::io;
	/// use std::fs;
	/// use std::os::unix::fs::MetadataExt;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("some_file")?;
	///     let dev_id = meta.dev();
	///     Ok(())
	/// }
	/// ```

	fn dev(&self) -> u64;
	/// Returns the inode number.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::MetadataExt;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("some_file")?;
	///     let inode = meta.ino();
	///     Ok(())
	/// }
	/// ```

	fn ino(&self) -> u64;
	/// Returns the rights applied to this file.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::MetadataExt;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("some_file")?;
	///     let mode = meta.mode();
	///     let user_has_write_access      = mode & 0o200;
	///     let user_has_read_write_access = mode & 0o600;
	///     let group_has_read_access      = mode & 0o040;
	///     let others_have_exec_access    = mode & 0o001;
	///     Ok(())
	/// }
	/// ```

	fn mode(&self) -> u32;
	/// Returns the number of hard links pointing to this file.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::MetadataExt;
	///  use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("some_file")?;
	///     let nb_hard_links = meta.nlink();
	///     Ok(())
	/// }
	/// ```

	fn nlink(&self) -> u64;
	/// Returns the user ID of the owner of this file.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::MetadataExt;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("some_file")?;
	///     let user_id = meta.uid();
	///     Ok(())
	/// }
	/// ```

	fn uid(&self) -> u32;
	/// Returns the group ID of the owner of this file.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::MetadataExt;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("some_file")?;
	///     let group_id = meta.gid();
	///     Ok(())
	/// }
	/// ```

	fn gid(&self) -> u32;
	/// Returns the device ID of this file (if it is a special one).
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::MetadataExt;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("some_file")?;
	///     let device_id = meta.rdev();
	///     Ok(())
	/// }
	/// ```

	fn rdev(&self) -> u64;
	/// Returns the total size of this file in bytes.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::MetadataExt;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("some_file")?;
	///     let file_size = meta.size();
	///     Ok(())
	/// }
	/// ```

	fn size(&self) -> u64;
	/// Returns the last access time of the file, in seconds since Unix Epoch.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::MetadataExt;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("some_file")?;
	///     let last_access_time = meta.atime();
	///     Ok(())
	/// }
	/// ```

	fn atime(&self) -> i64;
	/// Returns the last access time of the file, in nanoseconds since [`atime`].
	///
	/// [`atime`]: #tymethod.atime
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::MetadataExt;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("some_file")?;
	///     let nano_last_access_time = meta.atime_nsec();
	///     Ok(())
	/// }
	/// ```

	fn atime_nsec(&self) -> i64;
	/// Returns the last modification time of the file, in seconds since Unix Epoch.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::MetadataExt;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("some_file")?;
	///     let last_modification_time = meta.mtime();
	///     Ok(())
	/// }
	/// ```

	fn mtime(&self) -> i64;
	/// Returns the last modification time of the file, in nanoseconds since [`mtime`].
	///
	/// [`mtime`]: #tymethod.mtime
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::MetadataExt;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("some_file")?;
	///     let nano_last_modification_time = meta.mtime_nsec();
	///     Ok(())
	/// }
	/// ```

	fn mtime_nsec(&self) -> i64;
	/// Returns the last status change time of the file, in seconds since Unix Epoch.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::MetadataExt;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("some_file")?;
	///     let last_status_change_time = meta.ctime();
	///     Ok(())
	/// }
	/// ```

	fn ctime(&self) -> i64;
	/// Returns the last status change time of the file, in nanoseconds since [`ctime`].
	///
	/// [`ctime`]: #tymethod.ctime
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::MetadataExt;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("some_file")?;
	///     let nano_last_status_change_time = meta.ctime_nsec();
	///     Ok(())
	/// }
	/// ```

	fn ctime_nsec(&self) -> i64;
}

impl MetadataExt for fs::Metadata {
	fn dev(&self) -> u64 {
		self.st_dev()
	}
	fn ino(&self) -> u64 {
		self.st_ino()
	}
	fn mode(&self) -> u32 {
		self.st_mode()
	}
	fn nlink(&self) -> u64 {
		self.st_nlink()
	}
	fn uid(&self) -> u32 {
		self.st_uid()
	}
	fn gid(&self) -> u32 {
		self.st_gid()
	}
	fn rdev(&self) -> u64 {
		self.st_rdev()
	}
	fn size(&self) -> u64 {
		self.st_size()
	}
	fn atime(&self) -> i64 {
		self.st_atime()
	}
	fn atime_nsec(&self) -> i64 {
		self.st_atime_nsec()
	}
	fn mtime(&self) -> i64 {
		self.st_mtime()
	}
	fn mtime_nsec(&self) -> i64 {
		self.st_mtime_nsec()
	}
	fn ctime(&self) -> i64 {
		self.st_ctime()
	}
	fn ctime_nsec(&self) -> i64 {
		self.st_ctime_nsec()
	}
}

/// Unix-specific extensions for [`FileType`].
///
/// Adds support for special Unix file types such as block/character devices,
/// pipes, and sockets.
///
/// [`FileType`]: ../../../../std/fs/struct.FileType.html

pub trait FileTypeExt {
	/// Returns `true` if this file type is a block device.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::FileTypeExt;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("block_device_file")?;
	///     let file_type = meta.file_type();
	///     assert!(file_type.is_block_device());
	///     Ok(())
	/// }
	/// ```

	fn is_block_device(&self) -> bool;
	/// Returns `true` if this file type is a char device.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::FileTypeExt;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("char_device_file")?;
	///     let file_type = meta.file_type();
	///     assert!(file_type.is_char_device());
	///     Ok(())
	/// }
	/// ```

	fn is_char_device(&self) -> bool;
	/// Returns `true` if this file type is a fifo.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::FileTypeExt;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("fifo_file")?;
	///     let file_type = meta.file_type();
	///     assert!(file_type.is_fifo());
	///     Ok(())
	/// }
	/// ```

	fn is_fifo(&self) -> bool;
	/// Returns `true` if this file type is a socket.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs;
	/// use std::os::unix::fs::FileTypeExt;
	/// use std::io;
	///
	/// fn main() -> io::Result<()> {
	///     let meta = fs::metadata("unix.socket")?;
	///     let file_type = meta.file_type();
	///     assert!(file_type.is_socket());
	///     Ok(())
	/// }
	/// ```

	fn is_socket(&self) -> bool;
}

impl FileTypeExt for fs::FileType {
	fn is_block_device(&self) -> bool {
		self.as_inner().is(libc::S_IFBLK)
	}
	fn is_char_device(&self) -> bool {
		self.as_inner().is(libc::S_IFCHR)
	}
	fn is_fifo(&self) -> bool {
		self.as_inner().is(libc::S_IFIFO)
	}
	fn is_socket(&self) -> bool {
		self.as_inner().is(libc::S_IFSOCK)
	}
}

/// Unix-specific extensions to [`fs::DirBuilder`].
///
/// [`fs::DirBuilder`]: ../../../../std/fs/struct.DirBuilder.html

pub trait DirBuilderExt {
	/// Sets the mode to create new directories with. This option defaults to
	/// 0o777.
	///
	/// # Examples
	///
	/// ```no_run
	/// use std::fs::DirBuilder;
	/// use std::os::unix::fs::DirBuilderExt;
	///
	/// let mut builder = DirBuilder::new();
	/// builder.mode(0o755);
	/// ```

	fn mode(&mut self, mode: u32) -> &mut Self;
}

impl DirBuilderExt for fs::DirBuilder {
	fn mode(&mut self, mode: u32) -> &mut fs::DirBuilder {
		self.as_inner_mut().set_mode(mode);
		self
	}
}
