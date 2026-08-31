// SPDX-License-Identifier: GPL-2.0

// Dependencies in the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut test_1_result: i32 = 0;

#[no_mangle]
#[link_section = "struct_ops/test_1"]
pub unsafe extern "C" fn test_1() -> i32 {
    unsafe {
        test_1_result = 42;
    }
    0
}

#[no_mangle]
#[link_section = "struct_ops/test_1"]
pub unsafe extern "C" fn test_2() -> i32 {
    0
}

#[repr(C)]
pub struct bpf_testmod_ops___v1 {
    pub test_1: Option<unsafe extern "C" fn() -> i32>,
}

#[repr(C)]
pub struct bpf_testmod_ops___v2 {
    pub test_1: Option<unsafe extern "C" fn() -> i32>,
    pub does_not_exist: Option<unsafe extern "C" fn() -> i32>,
}

#[no_mangle]
#[link_section = ".struct_ops.link"]
pub static mut testmod_1: bpf_testmod_ops___v1 = bpf_testmod_ops___v1 {
    test_1: Some(test_1),
};

#[no_mangle]
#[link_section = ".struct_ops.link"]
pub static mut testmod_2: bpf_testmod_ops___v2 = bpf_testmod_ops___v2 {
    test_1: Some(test_1),
    does_not_exist: Some(test_2),
};

#[no_mangle]
#[link_section = "?.struct_ops"]
pub static mut optional_map: bpf_testmod_ops___v1 = bpf_testmod_ops___v1 {
    test_1: Some(test_1),
};

#[no_mangle]
#[link_section = "?.struct_ops.link"]
pub static mut optional_map2: bpf_testmod_ops___v1 = bpf_testmod_ops___v1 {
    test_1: Some(test_1),
};
