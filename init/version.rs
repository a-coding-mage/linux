// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 1992 Theodore Ts'o */
//! Preliminary version owner and the early hostname parameter.
//!
//! The build supplies bindings generated with its temporary UTS_VERSION. Final
//! version-timestamp.o replaces the two weak data symbols during the last link.

#[allow(
    clippy::all, dead_code, missing_docs, non_camel_case_types, non_snake_case,
    non_upper_case_globals, improper_ctypes, unsafe_op_in_unsafe_fn, unreachable_pub
)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/init_version_generated.rs"));
}

mod version_data;
#[path = "main_setup.rs"]
mod main_setup;
#[path = "main_printk.rs"]
mod main_printk;
#[path = "../rust/ffi_export.rs"]
mod ffi_export;

use core::ptr;
use kernel::ffi::{c_char, c_int};

#[link_section = ".init.text"]
unsafe extern "C" fn early_hostname(arg: *mut c_char) -> c_int {
    let bufsize = bindings::RUST_VERSION_NODENAME_SIZE as usize;
    // SAFETY: early option parsing supplies a live C string and serializes
    // mutation of the canonical namespace. Its nodename array has bufsize
    // bytes. The fortified wrapper knows this destination bound but not the
    // source object's size. Its bounded scan reduces the copy count, preserving
    // the bytes after a short string's terminator just as the C callback does.
    unsafe {
        let destination = ptr::addr_of_mut!(init_uts_ns.name.nodename).cast::<c_char>();
        let count = if bindings::RUST_VERSION_FORTIFY != 0 {
            let length = bindings::strnlen(arg, bufsize);
            if length == bufsize { bufsize } else { length.wrapping_add(1) }
        } else {
            bufsize
        };
        if bindings::sized_strscpy(destination, arg, count) < 0 {
            main_printk::main_printk!(
                "early_hostname",
                b"\x014hostname parameter exceeds %zd characters and will be truncated\0",
                bufsize - 1,
            );
        }
    }
    0
}

main_setup::setup_param!("hostname", early_hostname_parameter, Some(early_hostname), 1);

/// Build identity template consumed by /proc/version.
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static linux_proc_banner: [u8; bindings::RUST_VERSION_PROC_BANNER.len()] =
    *bindings::RUST_VERSION_PROC_BANNER;

/// Preliminary initial UTS namespace; the final timestamp object overrides it.
#[allow(non_upper_case_globals)]
#[no_mangle]
#[linkage = "weak"]
pub static mut init_uts_ns: bindings::uts_namespace =
    // SAFETY: the pointer names this static for the self-linked namespace lists.
    unsafe { version_data::namespace(ptr::addr_of_mut!(init_uts_ns)) };
ffi_export::export_symbol!(init_uts_ns, init_uts_ns, "GPL", "");

/// Preliminary boot banner, replaced by the final timestamp object.
#[allow(non_upper_case_globals)]
#[no_mangle]
#[linkage = "weak"]
pub static linux_banner: [u8; bindings::RUST_VERSION_BANNER.len()] =
    *bindings::RUST_VERSION_BANNER;

/// ELF notes use native-endian words and independent four-byte padding.
#[repr(C, align(4))]
struct LinuxNote<const D: usize> {
    namesz: u32,
    descsz: u32,
    kind: u32,
    name: [u8; 8],
    desc: [u8; D],
}

impl<const D: usize> LinuxNote<D> {
    const fn new(kind: u32, value: &[u8]) -> Self {
        assert!(D % 4 == 0 && value.len() <= D && value.len() <= u32::MAX as usize);
        let mut desc = [0; D];
        let mut index = 0;
        while index < value.len() {
            desc[index] = value[index];
            index += 1;
        }
        Self { namesz: 6, descsz: value.len() as u32, kind, name: *b"Linux\0\0\0", desc }
    }
}

#[used]
#[link_section = ".note.Linux"]
static BUILD_SALT: LinuxNote<{ (bindings::RUST_VERSION_BUILD_SALT.len() + 3) & !3 }> =
    LinuxNote::new(0x100, bindings::RUST_VERSION_BUILD_SALT);

#[used]
#[link_section = ".note.Linux"]
static BUILD_LTO_INFO: LinuxNote<4> = LinuxNote::new(0x101, &(cfg!(CONFIG_LTO) as i32).to_ne_bytes());

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
