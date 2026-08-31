// SPDX-License-Identifier: GPL-2.0
// Dependencies in the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

#[no_mangle]
pub static mut test_fmod_ret: u64 = 0;

#[no_mangle]
#[link_section = "fmod_ret/security_new_get_constant"]
pub unsafe extern "C" fn fmod_ret_test(val: core::ffi::c_long, ret: core::ffi::c_int) -> core::ffi::c_int {
    core::ptr::write_volatile(core::ptr::addr_of_mut!(test_fmod_ret), 1);
    120
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
