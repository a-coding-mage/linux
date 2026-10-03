// SPDX-License-Identifier: GPL-2.0-only
//! Configured native declarations shared by the library and gunzip owner.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, missing_docs, unreachable_pub)]

use core::ffi;

type __kernel_size_t = usize;
type __kernel_ssize_t = isize;
type __kernel_ptrdiff_t = isize;

include!(concat!(
    env!("OBJTREE"),
    "/rust/bindings/zlib_inflate_generated.rs"
));

pub(crate) const GFP_KERNEL: gfp_t = RUST_ZLIB_GFP_KERNEL;
