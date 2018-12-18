#![allow(dead_code, missing_docs, bad_style)]


use io as IO; // Hackerman!
//use os::raw::c_char;
use ptr;
use sys::os_str::Buf;
use ffi::{OsString, OsStr};
use time::Duration;
pub use libc::strlen;

#[cfg(all(not(dox), target_os = "vxworks"))]   pub use os::vxworks as platform;

pub use self::rand::hashmap_random_keys;

pub mod args;
pub mod cmath;
pub mod condvar;
pub mod env;
pub mod ext; // this is likely to be a problem
pub mod fd;
pub mod ffi;
pub mod ffifix;
pub mod fs;
pub mod io;
pub mod memchr;
pub mod mutex;
pub mod net;
pub mod os;
pub mod os_str;
pub mod path;
pub mod pipe;
pub mod process;
pub mod rand;
pub mod rwlock;
pub mod stack_overflow;
pub mod stdio;
pub mod thread;
pub mod thread_local;
pub mod time;
pub use libc::signal;
/* use sys_common::{FromInner, IntoInner, AsInner}; Apparently ever used */


#[cfg(not(test))]
pub fn init() {
	unsafe {
		reset_sigpipe();
	}
	unsafe fn reset_sigpipe() {
		assert!(signal(::libc::SIGPIPE, ::libc::SIG_IGN as usize) != ::libc::SIG_ERR as usize);
	}
}

pub fn decode_error_kind(errno: i32) -> IO::ErrorKind {
	match errno as ::libc::c_int {
		::libc::ECONNREFUSED => IO::ErrorKind::ConnectionRefused,
		::libc::ECONNRESET => IO::ErrorKind::ConnectionReset,
		::libc::EPERM | ::libc::EACCES | ::libc::S_nfsLib_NFSERR_PERM | ::libc::S_nfsLib_NFSERR_ACCESS => IO::ErrorKind::PermissionDenied,
		::libc::EPIPE => IO::ErrorKind::BrokenPipe,
		::libc::ENOTCONN => IO::ErrorKind::NotConnected,
		::libc::ECONNABORTED => IO::ErrorKind::ConnectionAborted,
		::libc::EADDRNOTAVAIL => IO::ErrorKind::AddrNotAvailable,
		::libc::EADDRINUSE => IO::ErrorKind::AddrInUse,
		::libc::ENOENT | ::libc::S_nfsLib_NFSERR_NOENT =>IO::ErrorKind::NotFound,
		::libc::EINTR => IO::ErrorKind::Interrupted,
		::libc::EINVAL | ::libc::S_nfsLib_NFSERR_INVAL => IO::ErrorKind::InvalidInput,
		::libc::ETIMEDOUT => IO::ErrorKind::TimedOut,
		::libc::EEXIST | ::libc::S_nfsLib_NFSERR_EXIST => IO::ErrorKind::AlreadyExists,

		// These two constants can have the same value on some systems,
		// but different values on others, so we can't use a match
		// clause
		x if x == ::libc::EAGAIN || x == ::libc::EWOULDBLOCK =>
			IO::ErrorKind::WouldBlock,

		_ => IO::ErrorKind::Other,
	}
}

// This might be a baaad idea
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

pub fn cvt<T: IsMinusOne>(t: T) -> IO::Result<T> {
	if t.is_minus_one() {
		Err(IO::Error::last_os_error())
	} else {
		Ok(t)
	}
}

pub fn cvt_r<T, F>(mut f: F) -> IO::Result<T>
	where T: IsMinusOne,
		  F: FnMut() -> T
{
	loop {
		match cvt(f()) {
			Err(ref e) if e.kind() == IO::ErrorKind::Interrupted => {}
			other => return other,
		}
	}
}

//Probably useful somewhere
pub unsafe fn abort_internal() -> ! {
	::libc::abort()
}

//This is for all stuff that isn't actually supported yet
pub fn unsupported<T>() -> IO::Result<T> {
	Err(unsupported_err())
}

pub fn unsupported_err() -> IO::Error {
	IO::Error::new(IO::ErrorKind::Other,
		"operation not supported on VxWorks yet")
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Void {}
