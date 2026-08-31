// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C includes:
// <vmlinux.h>
// <bpf/bpf_helpers.h>
// <bpf/bpf_tracing.h>
// "../test_kmods/bpf_testmod.h"

unsafe extern "C" {
    type bpf_testmod_ops;
    type bpf_testmod_ops2;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

#[unsafe(link_section = "struct_ops/test_1")]
#[unsafe(no_mangle)]
pub extern "C" fn test_1() -> ::core::ffi::c_int {
    0
}

#[unsafe(link_section = "struct_ops/test_2")]
#[unsafe(no_mangle)]
pub extern "C" fn test_2() -> ::core::ffi::c_int {
    0
}

#[repr(C)]
pub struct bpf_testmod_ops_init {
    pub test_1: *mut ::core::ffi::c_void,
    pub test_2: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct bpf_testmod_ops2_init {
    pub test_1: *mut ::core::ffi::c_void,
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut testmod_1: bpf_testmod_ops_init = bpf_testmod_ops_init {
    test_1: test_1 as *mut ::core::ffi::c_void,
    test_2: test_2 as *mut ::core::ffi::c_void,
};

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut testmod_2: bpf_testmod_ops2_init = bpf_testmod_ops2_init {
    test_1: test_1 as *mut ::core::ffi::c_void,
};
