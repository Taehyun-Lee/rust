// from Unix

use fs;
use os::raw;
use sys;
use io;
use sys_common::{AsInner, FromInner, IntoInner};
use libc;

pub type RawFd = raw::c_int;

pub trait AsRawFd { // some speculate this might even work
	fn as_raw_fd(&self) -> RawFd;
}

pub trait FromRawFd {
	unsafe fn from_raw_fd(fd: RawFd) -> Self;
}

pub trait IntoRawFd {
	fn into_raw_fd(self) -> RawFd;
}

impl AsRawFd for fs::File {
	fn as_raw_fd(&self) -> RawFd {
		self.as_inner().fd().raw()
	}
}

impl FromRawFd for fs::File {
	unsafe fn from_raw_fd(fd: RawFd) -> fs::File {
		fs::File::from_inner(sys::fs::File::from_inner(fd))
	}
}

impl IntoRawFd for fs::File {
	fn into_raw_fd(self) -> RawFd {
		self.into_inner().into_fd().into_raw()
	}
}

impl AsRawFd for io::Stdin {
	fn as_raw_fd(&self) -> RawFd { libc::STDIN_FILENO }
}

impl AsRawFd for io::Stdout {
	fn as_raw_fd(&self) -> RawFd { libc::STDOUT_FILENO }
}

impl AsRawFd for io::Stderr {
	fn as_raw_fd(&self) -> RawFd { libc::STDERR_FILENO }
}



