use anyhow::{ensure, Result};
use log::info;
use os_str_bytes::OsStrBytes;
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::path::Path;

pub fn send_file(port: u16, dest_dir: &Path, file: &Path) -> Result<()> {
	info!("Sending file {}", file.display());
	let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);
	let mut stream = TcpStream::connect(addr)?;
	let dest_dir_bytes = dest_dir.as_os_str().to_bytes();
	let file_bytes = file.as_os_str().to_bytes();
	ensure!(
		!dest_dir_bytes.contains(&b'\n'),
		"The destination directory, {}, must not contain a newline",
		dest_dir.display()
	);
	ensure!(
		!file_bytes.contains(&b'\n'),
		"The sent binary, {}, must not contain a newline",
		file.display()
	);
	stream.write_all(b"ln st ")?;
	stream.write_all(&dest_dir_bytes)?;
	stream.write_all(b"\nln s ")?;
	stream.write_all(&file_bytes)?;
	stream.write_all(b"\n")?;
	Ok(())
}
