// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_endian.h>

pub type __u64 = u64;

#[no_mangle]
pub static mut test_get_constant: __u64 = 0;

#[no_mangle]
#[link_section = "freplace/get_constant"]
pub unsafe extern "C" fn security_new_get_constant(val: core::ffi::c_long) -> core::ffi::c_int {
    if val != 123 {
        return 0;
    }
    core::ptr::write_volatile(core::ptr::addr_of_mut!(test_get_constant), 1);
    return core::ptr::read_volatile(core::ptr::addr_of!(test_get_constant)) as core::ffi::c_int;
    /* original get_constant() returns val - 122 */
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [core::ffi::c_char; 4] = [b'G' as core::ffi::c_char, b'P' as core::ffi::c_char, b'L' as core::ffi::c_char, 0];
