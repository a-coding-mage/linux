// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// Dependency intent from C source:
// #include "../test_kmods/bpf_testmod_kfunc.h"

#[repr(C)]
pub struct __sk_buff {
    pub sk: *mut bpf_sock,
}

#[repr(C)]
pub struct bpf_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock_common {
    pub skc_state: u8,
}

#[repr(C)]
pub struct sock {
    pub __sk_common: sock_common,
}

unsafe extern "C" {
    #[link_name = "bpf_prog_active"]
    pub static bpf_prog_active: ::core::ffi::c_int;

    pub fn bpf_sk_fullsock(sk: *mut bpf_sock) -> *mut bpf_sock;
    pub fn bpf_per_cpu_ptr(
        ptr: *const ::core::ffi::c_void,
        cpu: ::core::ffi::c_uint,
    ) -> *mut ::core::ffi::c_void;
    pub fn bpf_get_smp_processor_id() -> ::core::ffi::c_uint;
    pub fn bpf_kfunc_call_test1(
        sk: *mut sock,
        a: ::core::ffi::c_int,
        b: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
        d: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uint;
    pub fn bpf_kfunc_call_test3(sk: *mut sock) -> *mut sock;
}

#[unsafe(no_mangle)]
pub static mut active_res: ::core::ffi::c_int = -1;
#[unsafe(no_mangle)]
pub static mut sk_state_res: ::core::ffi::c_int = -1;

#[inline(never)]
pub unsafe extern "C" fn f1(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut sk: *mut bpf_sock = unsafe { (*skb).sk };
    let active: *mut ::core::ffi::c_int;

    if sk.is_null() {
        return -1;
    }

    sk = unsafe { bpf_sk_fullsock(sk) };
    if sk.is_null() {
        return -1;
    }

    active = unsafe {
        bpf_per_cpu_ptr(
            &raw const bpf_prog_active as *const ::core::ffi::c_void,
            bpf_get_smp_processor_id(),
        ) as *mut ::core::ffi::c_int
    };
    if !active.is_null() {
        unsafe {
            active_res = *active;
        }
    }

    unsafe {
        sk_state_res =
            (*bpf_kfunc_call_test3(sk as *mut sock)).__sk_common.skc_state as ::core::ffi::c_int;
    }

    unsafe { bpf_kfunc_call_test1(sk as *mut sock, 1, 2, 3, 4) as u32 as ::core::ffi::c_int }
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test1(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    unsafe { f1(skb) }
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [::core::ffi::c_char; 4] = [b'G' as _, b'P' as _, b'L' as _, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
