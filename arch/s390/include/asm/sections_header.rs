/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <asm-generic/sections.h> are supplied
// by the surrounding translation unit.

/*
 * .boot.data section contains variables "shared" between the decompressor and
 * the decompressed kernel. The decompressor will store values in them, and
 * copy over to the decompressed image before starting it.
 *
 * Each variable end up in its own intermediate section .boot.data.<var name>,
 * those sections are later sorted by alignment + name and merged together into
 * final .boot.data section, which should be identical in the decompressor and
 * the decompressed kernel (that is checked during the build).
 */
macro_rules! __bootdata {
    ($var:item) => {
        #[link_section = concat!(".boot.data.", stringify!($var))]
        $var
    };
}

/*
 * .boot.preserved.data is similar to .boot.data, but it is not part of the
 * .init section and thus will be preserved for later use in the decompressed
 * kernel.
 */
macro_rules! __bootdata_preserved {
    ($var:item) => {
        #[link_section = concat!(".boot.preserved.data.", stringify!($var))]
        $var
    };
}

extern "C" {
    pub static mut __samode31: *mut core::ffi::c_char;
    pub static mut __eamode31: *mut core::ffi::c_char;
    pub static mut __stext_amode31: *mut core::ffi::c_char;
    pub static mut __etext_amode31: *mut core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
