// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc */

// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, "bpf_misc.h",
// "bpf_kfuncs.h", "../test_kmods/bpf_testmod_kfunc.h"

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _unused: [u8; 0],
}

unsafe extern "C" {
    fn bpf_dynptr_from_skb(
        skb: *mut __sk_buff,
        flags: u64,
        ptr: *mut bpf_dynptr,
    ) -> ::core::ffi::c_long;
    fn bpf_kfunc_dynptr_test(ptr1: *mut bpf_dynptr, ptr2: *mut bpf_dynptr);
}

// SEC("tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_dynptr_nullable_test1(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut data = ::core::mem::MaybeUninit::<bpf_dynptr>::uninit();

    unsafe {
        bpf_dynptr_from_skb(skb, 0, data.as_mut_ptr());
        bpf_kfunc_dynptr_test(data.as_mut_ptr(), ::core::ptr::null_mut());
    }

    0
}

// SEC("tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_dynptr_nullable_test2(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut data = ::core::mem::MaybeUninit::<bpf_dynptr>::uninit();

    unsafe {
        bpf_dynptr_from_skb(skb, 0, data.as_mut_ptr());
        bpf_kfunc_dynptr_test(data.as_mut_ptr(), data.as_mut_ptr());
    }

    0
}

// SEC("tc")
// __failure __msg("Possibly NULL pointer passed to trusted R1")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_dynptr_nullable_test3(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut data = ::core::mem::MaybeUninit::<bpf_dynptr>::uninit();

    unsafe {
        bpf_dynptr_from_skb(skb, 0, data.as_mut_ptr());
        bpf_kfunc_dynptr_test(::core::ptr::null_mut(), data.as_mut_ptr());
    }

    0
}

// char _license[] SEC("license") = "GPL";
#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [::core::ffi::c_char; 4] = [b'G' as ::core::ffi::c_char, b'P' as ::core::ffi::c_char, b'L' as ::core::ffi::c_char, 0];
