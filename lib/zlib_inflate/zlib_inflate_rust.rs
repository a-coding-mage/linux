// SPDX-License-Identifier: GPL-2.0-only
//! Native kernel zlib inflate owner, including original exports and metadata.

#[cfg(any(PREBOOT, ASMINF, INFLATE_STRICT, PKZIP_BUG_WORKAROUND))]
compile_error!("The kernel inflate owner requires the ordinary software zlib row");

#[path = "kernel_bindings.rs"]
mod bindings;
#[path = "../../rust/ffi_export.rs"]
mod ffi_export;

#[path = "inffast.rs"]
mod zlib_inffast;
#[path = "inflate.rs"]
mod zlib_inflate;
#[path = "inftrees.rs"]
mod zlib_inftrees;
#[path = "infutil.rs"]
mod zlib_infutil;

ffi_export::export_symbol!(
    zlib_inflate_workspacesize,
    zlib_inflate::zlib_inflate_workspacesize,
    "",
    ""
);
ffi_export::export_symbol!(zlib_inflate, zlib_inflate::zlib_inflate, "", "");
ffi_export::export_symbol!(zlib_inflateInit2, zlib_inflate::zlib_inflateInit2, "", "");
ffi_export::export_symbol!(zlib_inflateEnd, zlib_inflate::zlib_inflateEnd, "", "");
ffi_export::export_symbol!(zlib_inflateReset, zlib_inflate::zlib_inflateReset, "", "");
ffi_export::export_symbol!(zlib_inflateIncomp, zlib_inflate::zlib_inflateIncomp, "", "");
ffi_export::export_symbol!(zlib_inflate_blob, zlib_infutil::zlib_inflate_blob, "", "");

// Like inflate_syms.c, this stateless library has no init or exit entry point.
#[cfg(MODULE)]
const MODINFO: &str = concat!(
    "description=Data decompression using the deflation algorithm\0",
    "license=GPL\0",
);
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "zlib_inflate.description=Data decompression using the deflation algorithm\0",
    "zlib_inflate.license=GPL\0zlib_inflate.file=",
    env!("RUST_MODFILE"),
    "\0",
);

#[used]
#[link_section = ".modinfo"]
static MODULE_INFO: [u8; MODINFO.len()] = {
    let mut bytes = [0; MODINFO.len()];
    let mut index = 0;
    while index < bytes.len() {
        bytes[index] = MODINFO.as_bytes()[index];
        index += 1;
    }
    bytes
};

#[cfg(MODULE)]
#[used]
static __IS_RUST_MODULE: () = ();
