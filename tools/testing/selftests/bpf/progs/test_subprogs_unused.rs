// C dependencies from the original source:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_core_read.h>

#[unsafe(link_section = "license")]
pub static LICENSE: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

#[allow(dead_code)]
#[inline(never)]
pub extern "C" fn unused1(x: ::core::ffi::c_int) -> ::core::ffi::c_int {
    x + 1
}

#[allow(dead_code)]
#[inline(never)]
extern "C" fn unused2(x: ::core::ffi::c_int) -> ::core::ffi::c_int {
    x + 2
}

#[unsafe(link_section = "raw_tp/sys_enter")]
pub extern "C" fn main_prog(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let _ = ctx;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
