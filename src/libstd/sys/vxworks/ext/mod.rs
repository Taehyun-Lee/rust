// Uhhh
#![stable(feature = "rust1", since = "1.0.0")]
#![allow(missing_docs)]

#[stable(feature = "rust1", since = "1.0.0")]
pub mod ffi;
pub mod fs;
pub mod raw;

#[stable(feature = "rust1", since = "1.0.0")]
pub mod prelude {
    #[doc(no_inline)] #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::ffi::{OsStrExt, OsStringExt};
    #[doc(no_inline)] #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::fs::{PermissionsExt, OpenOptionsExt, MetadataExt, FileTypeExt};
}
