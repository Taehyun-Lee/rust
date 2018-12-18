// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Android-specific raw type definitions

#![stable(feature = "raw_ext", since = "1.1.0")]
#![rustc_deprecated(since = "1.8.0",
                    reason = "these type aliases are no longer supported by \
                              the standard library, the `libc` crate on \
                              crates.io should be used instead for the correct \
                              definitions")]
#![allow(deprecated)]

use os::raw::{c_long, c_ulong};

#[stable(feature = "pthread_t", since = "1.8.0")]
pub type pthread_t = c_ulong;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type dev_t = c_ulong;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type mode_t = i32;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type blkcnt_t = c_long;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type blksize_t = c_long;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type ino_t = c_ulong;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type nlink_t = c_ulong;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type off_t = i64;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type time_t = c_long;

#[repr(C)]
#[derive(Clone)]
#[stable(feature = "metadata_ext2", since = "1.8.0")]
pub struct stat {
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    pub st_dev: c_ulong,
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    pub st_ino: c_ulong,
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    pub st_mode: i32,
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    pub st_nlink: c_ulong,
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    pub st_uid: u16,
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    pub st_gid: u16,
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    pub st_rdev: c_ulong,
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    pub st_size: i64,
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    pub st_atime: c_long,
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    pub st_mtime: c_long,
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    pub st_ctime: c_long,
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    pub st_blksize: c_long,
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    pub st_blocks: c_long,
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    pub st_attrib: u8,
    st_reserved1: i32,
    st_reserved2: i32,
    st_reserved3: i32,
    st_reserved4: i32,
}
