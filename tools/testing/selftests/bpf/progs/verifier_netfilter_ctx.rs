// SPDX-License-Identifier: GPL-2.0

// C dependencies from the original source:
// vmlinux.h
// bpf_misc.h
// bpf/bpf_endian.h
// bpf/bpf_tracing.h
// bpf/bpf_helpers.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u64 = u64;
type uint8_t = u8;

const NF_DROP: i32 = 0;
const NF_ACCEPT: i32 = 1;

extern "C" {
    static __bpf_nf_ctx_state: i32;
    static __bpf_nf_ctx_skb: i32;
    static __bpf_nf_ctx_size: i32;

    fn bpf_dynptr_from_skb(skb: *mut __sk_buff, flags: u64, ptr: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_slice(
        ptr: *mut bpf_dynptr,
        offset: u32,
        buffer: *mut c_void,
        buffer__sz: u32,
    ) -> *mut c_void;
    fn bpf_htons(x: u16) -> u16;
}

#[repr(C)]
pub struct bpf_nf_ctx {
    pub skb: *mut __sk_buff,
    pub state: *mut nf_hook_state,
}

#[repr(C)]
pub struct nf_hook_state {
    pub sk: *mut c_void,
    pub pf: u8,
}

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: u8,
}

impl iphdr {
    unsafe fn ihl(&self) -> uint8_t {
        self.ihl_version & 0x0f
    }
}

#[repr(C)]
pub struct tcphdr {
    pub source: u16,
    pub dest: u16,
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u64; 2],
}

// SEC("netfilter")
// __description("netfilter invalid context access, size too short")
// __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn with_invalid_ctx_access_test1() {
    asm!(
        "r2 = *(u8*)(r1 + {__bpf_nf_ctx_state})",
        "r0 = 0",
        "exit",
        __bpf_nf_ctx_state = const 8,
        options(noreturn)
    );
}

// SEC("netfilter")
// __description("netfilter invalid context access, size too short")
// __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn with_invalid_ctx_access_test2() {
    asm!(
        "r2 = *(u16*)(r1 + {__bpf_nf_ctx_skb})",
        "r0 = 0",
        "exit",
        __bpf_nf_ctx_skb = const 0,
        options(noreturn)
    );
}

// SEC("netfilter")
// __description("netfilter invalid context access, past end of ctx")
// __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn with_invalid_ctx_access_test3() {
    asm!(
        "r2 = *(u64*)(r1 + {__bpf_nf_ctx_size})",
        "r0 = 0",
        "exit",
        __bpf_nf_ctx_size = const size_of::<bpf_nf_ctx>(),
        options(noreturn)
    );
}

// SEC("netfilter")
// __description("netfilter invalid context, write")
// __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn with_invalid_ctx_access_test4() {
    asm!(
        "r2 = r1",
        "*(u64*)(r2 + 0) = r1",
        "r0 = 1",
        "exit",
        options(noreturn)
    );
}

// SEC("netfilter")
// __description("netfilter valid context read and invalid write")
// __failure __msg("only read is supported")
#[no_mangle]
pub unsafe extern "C" fn with_invalid_ctx_access_test5(ctx: *mut bpf_nf_ctx) -> i32 {
    let state: *mut nf_hook_state = (*ctx).state as *mut c_void as *mut nf_hook_state;

    (*state).sk = ptr::null_mut();
    NF_ACCEPT
}

// SEC("netfilter")
// __description("netfilter test prog with skb and state read access")
// __success __failure_unpriv
// __retval(0)
#[no_mangle]
pub unsafe extern "C" fn with_valid_ctx_access_test6(ctx: *mut bpf_nf_ctx) -> i32 {
    let skb: *mut __sk_buff = (*ctx).skb as *mut __sk_buff;
    let state: *const nf_hook_state = (*ctx).state;
    let mut iph: *const iphdr;
    let mut th: *const tcphdr;
    let mut buffer_iph: [u8; 20] = [0; 20];
    let mut buffer_th: [u8; 40] = [0; 40];
    let mut ptr: bpf_dynptr = core::mem::zeroed();
    let ihl: uint8_t;

    if (*(*ctx).skb).len <= 20 || bpf_dynptr_from_skb(skb, 0, &mut ptr) != 0 {
        return NF_ACCEPT;
    }

    iph = bpf_dynptr_slice(
        &mut ptr,
        0,
        buffer_iph.as_mut_ptr() as *mut c_void,
        size_of::<[u8; 20]>() as u32,
    ) as *const iphdr;
    if iph.is_null() {
        return NF_ACCEPT;
    }

    if (*state).pf != 2 {
        return NF_ACCEPT;
    }

    ihl = (*iph).ihl() << 2;

    th = bpf_dynptr_slice(
        &mut ptr,
        ihl as u32,
        buffer_th.as_mut_ptr() as *mut c_void,
        size_of::<[u8; 40]>() as u32,
    ) as *const tcphdr;
    if th.is_null() {
        return NF_ACCEPT;
    }

    if (*th).dest == bpf_htons(22) {
        NF_ACCEPT
    } else {
        NF_DROP
    }
}

// char _license[] SEC("license") = "GPL";
#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
