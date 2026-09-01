// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// Depends on vmlinux.h and bpf/bpf_helpers.h for BPF types and SEC section
// placement in the original C source.

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [::core::ffi::c_char; 4] = [b'G' as ::core::ffi::c_char, b'P' as ::core::ffi::c_char, b'L' as ::core::ffi::c_char, 0];

#[unsafe(no_mangle)]
pub static mut value_sum: __u32 = 0;

#[unsafe(link_section = "iter/bpf_map_elem")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_bpf_hash_map(ctx: *mut bpf_iter__bpf_map_elem) -> ::core::ffi::c_int {
    let value: *mut ::core::ffi::c_void = unsafe { (*ctx).value };

    if value == 0 as *mut ::core::ffi::c_void {
        return 0;
    }

    /* negative offset, verifier failure. */
    unsafe {
        value_sum = value_sum.wrapping_add(*((value as *mut u8).offset(-4) as *mut __u32));
    }
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
