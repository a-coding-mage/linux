// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>

// Original C macro expansion helpers:
// #define X_0(x)
// #define X_1(x) x X_0(x)
// #define X_2(x) x X_1(x)
// #define X_3(x) x X_2(x)
// #define X_4(x) x X_3(x)
// #define X_5(x) x X_4(x)
// #define X_6(x) x X_5(x)
// #define X_7(x) x X_6(x)
// #define X_8(x) x X_7(x)
// #define X_9(x) x X_8(x)
// #define X_10(x) x X_9(x)
// #define REPEAT_256(Y) X_2(X_10(X_10(Y))) X_5(X_10(Y)) X_6(Y)

extern "C" {
    // extern const int bpf_testmod_ksym_percpu __ksym;
    static bpf_testmod_ksym_percpu: ::core::ffi::c_int;

    // extern void bpf_testmod_test_mod_kfunc(int i) __ksym;
    fn bpf_testmod_test_mod_kfunc(i: ::core::ffi::c_int);

    // extern void bpf_testmod_invalid_mod_kfunc(void) __ksym __weak;
    fn bpf_testmod_invalid_mod_kfunc();

    fn bpf_this_cpu_ptr(ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[no_mangle]
pub static mut out_bpf_testmod_ksym: ::core::ffi::c_int = 0;

#[no_mangle]
pub static x: ::core::ffi::c_int = 0;

// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn load(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = skb;

    /* This will be kept by clang, but removed by verifier. Since it is
     * marked as __weak, libbpf and gen_loader don't error out if BTF ID
     * is not found for it, instead imm and off is set to 0 for it.
     */
    if ::core::ptr::read_volatile(&x) != 0 {
        bpf_testmod_invalid_mod_kfunc();
    }
    bpf_testmod_test_mod_kfunc(42);
    out_bpf_testmod_ksym =
        *(bpf_this_cpu_ptr(&bpf_testmod_ksym_percpu as *const _ as *const ::core::ffi::c_void)
            as *mut ::core::ffi::c_int);
    return 0;
}

// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn load_256(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = skb;

    /* this will fail if kfunc doesn't reuse its own btf fd index */
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);
    bpf_testmod_test_mod_kfunc(42);

    bpf_testmod_test_mod_kfunc(42);
    return 0;
}

// char LICENSE[] SEC("license") = "GPL";
#[no_mangle]
pub static LICENSE: [u8; 4] = *b"GPL\0";
