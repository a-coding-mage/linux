// SPDX-License-Identifier: GPL-2.0

// C dependencies translated as external Rust dependencies:
// <vmlinux.h>
// <bpf/bpf_tracing.h>
// <bpf/bpf_helpers.h>
// "bpf_misc.h"
// "../test_kmods/bpf_testmod_kfunc.h"

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

unsafe extern "C" {
    fn bpf_kfunc_nested_acquire_nonzero_offset_test(arg: *mut sk_buff_head) -> *mut sk_buff;
    fn bpf_kfunc_nested_acquire_zero_offset_test(arg: *mut sock_common) -> *mut sk_buff;
    fn bpf_kfunc_nested_release_test(arg: *mut sk_buff);
}

#[no_mangle]
#[link_section = "tp_btf/tcp_probe"]
// __success
pub unsafe extern "C" fn test_nested_acquire_nonzero(
    sk: *mut sock,
    skb: *mut sk_buff,
) -> ::core::ffi::c_int {
    let mut ptr: *mut sk_buff;

    ptr = unsafe {
        bpf_kfunc_nested_acquire_nonzero_offset_test(::core::ptr::addr_of_mut!(
            (*sk).sk_write_queue
        ))
    };

    unsafe {
        bpf_kfunc_nested_release_test(ptr);
    }
    0
}

#[no_mangle]
#[link_section = "tp_btf/tcp_probe"]
// __success
pub unsafe extern "C" fn test_nested_acquire_zero(
    sk: *mut sock,
    skb: *mut sk_buff,
) -> ::core::ffi::c_int {
    let mut ptr: *mut sk_buff;

    ptr = unsafe {
        bpf_kfunc_nested_acquire_zero_offset_test(::core::ptr::addr_of_mut!(
            (*sk).__sk_common
        ))
    };

    unsafe {
        bpf_kfunc_nested_release_test(ptr);
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
