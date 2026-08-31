// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

extern "C" {
    #[link_name = "bpf_test_modorder_retx"]
    fn bpf_test_modorder_retx() -> ::core::ffi::c_int;
    #[link_name = "bpf_test_modorder_rety"]
    fn bpf_test_modorder_rety() -> ::core::ffi::c_int;
}

#[no_mangle]
#[link_section = "classifier"]
pub unsafe extern "C" fn call_kfunc_xy(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let ret1: ::core::ffi::c_int;
    let ret2: ::core::ffi::c_int;

    ret1 = bpf_test_modorder_retx();
    ret2 = bpf_test_modorder_rety();

    if ret1 == b'x' as ::core::ffi::c_int && ret2 == b'y' as ::core::ffi::c_int {
        0
    } else {
        -1
    }
}

#[no_mangle]
#[link_section = "classifier"]
pub unsafe extern "C" fn call_kfunc_yx(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let ret1: ::core::ffi::c_int;
    let ret2: ::core::ffi::c_int;

    ret1 = bpf_test_modorder_rety();
    ret2 = bpf_test_modorder_retx();

    if ret1 == b'y' as ::core::ffi::c_int && ret2 == b'x' as ::core::ffi::c_int {
        0
    } else {
        -1
    }
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];
