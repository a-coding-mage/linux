// SPDX-License-Identifier: GPL-2.0
//! Ordinary kernel/initramfs gunzip owner using the Rust zlib library.

#[cfg(any(PREBOOT, MODULE, CONFIG_ZLIB_DFLTCC))]
compile_error!("The ordinary Rust gunzip owner requires built-in, non-DFLTCC zlib");

// Nullable callback layout is preserved, but Rust Option<fn> needs a native
// signature-specific CFI encoding before indirect calls through the original
// C decompress_fn table can be accepted on CFI kernels.
#[cfg(CONFIG_CFI)]
compile_error!("Rust gunzip requires a verified native nullable-callback CFI adapter");

#[path = "zlib_inflate/kernel_bindings.rs"]
mod bindings;

// This crate imports the library ABI; it must not define the codec a second time.
mod zlib_inflate {
    pub(crate) use crate::bindings::{
        zlib_inflate, zlib_inflateEnd, zlib_inflateInit2, zlib_inflate_workspacesize,
    };
}

#[path = "decompress_inflate.rs"]
mod implementation;
