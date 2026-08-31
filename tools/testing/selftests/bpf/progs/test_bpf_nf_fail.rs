// SPDX-License-Identifier: GPL-2.0
// #define BPF_NO_KFUNC_PROTOTYPES
// C dependencies removed from executable Rust:
// <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_core_read.h>, "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type s32 = i32;
type u8 = u8;
type u32 = u32;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xdp_md {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_sock_tuple_ipv4 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_sock_tuple {
    pub ipv4: bpf_sock_tuple_ipv4,
}

#[repr(C)]
pub struct nf_conn {
    pub status: u32,
}

#[repr(C)]
pub struct bpf_ct_opts___local {
    pub netns_id: s32,
    pub error: s32,
    pub l4proto: u8,
    pub reserved: [u8; 3],
}

extern "C" {
    pub fn bpf_skb_ct_alloc(
        arg1: *mut __sk_buff,
        arg2: *mut bpf_sock_tuple,
        arg3: u32,
        arg4: *mut bpf_ct_opts___local,
        arg5: u32,
    ) -> *mut nf_conn;
    pub fn bpf_skb_ct_lookup(
        arg1: *mut __sk_buff,
        arg2: *mut bpf_sock_tuple,
        arg3: u32,
        arg4: *mut bpf_ct_opts___local,
        arg5: u32,
    ) -> *mut nf_conn;
    pub fn bpf_xdp_ct_alloc(
        arg1: *mut xdp_md,
        arg2: *mut bpf_sock_tuple,
        arg3: u32,
        arg4: *mut bpf_ct_opts___local,
        arg5: u32,
    ) -> *mut nf_conn;
    pub fn bpf_xdp_ct_lookup(
        arg1: *mut xdp_md,
        arg2: *mut bpf_sock_tuple,
        arg3: u32,
        arg4: *mut bpf_ct_opts___local,
        arg5: u32,
    ) -> *mut nf_conn;
    pub fn bpf_ct_insert_entry(arg1: *mut nf_conn) -> *mut nf_conn;
    pub fn bpf_ct_release(arg1: *mut nf_conn);
    pub fn bpf_ct_set_timeout(arg1: *mut nf_conn, arg2: u32);
    pub fn bpf_ct_change_timeout(arg1: *mut nf_conn, arg2: u32) -> i32;
    pub fn bpf_ct_set_status(arg1: *mut nf_conn, arg2: u32) -> i32;
    pub fn bpf_ct_change_status(arg1: *mut nf_conn, arg2: u32) -> i32;
}

#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn alloc_release(ctx: *mut __sk_buff) -> i32 {
    let mut opts: bpf_ct_opts___local = core::mem::zeroed();
    let mut tup: bpf_sock_tuple = core::mem::zeroed();
    let mut ct: *mut nf_conn;

    ct = bpf_skb_ct_alloc(
        ctx,
        &mut tup,
        core::mem::size_of_val(&tup.ipv4) as u32,
        &mut opts,
        core::mem::size_of_val(&opts) as u32,
    );
    if ct.is_null() {
        return 0;
    }
    bpf_ct_release(ct);
    return 0;
}

#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn insert_insert(ctx: *mut __sk_buff) -> i32 {
    let mut opts: bpf_ct_opts___local = core::mem::zeroed();
    let mut tup: bpf_sock_tuple = core::mem::zeroed();
    let mut ct: *mut nf_conn;

    ct = bpf_skb_ct_alloc(
        ctx,
        &mut tup,
        core::mem::size_of_val(&tup.ipv4) as u32,
        &mut opts,
        core::mem::size_of_val(&opts) as u32,
    );
    if ct.is_null() {
        return 0;
    }
    ct = bpf_ct_insert_entry(ct);
    if ct.is_null() {
        return 0;
    }
    ct = bpf_ct_insert_entry(ct);
    return 0;
}

#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn lookup_insert(ctx: *mut __sk_buff) -> i32 {
    let mut opts: bpf_ct_opts___local = core::mem::zeroed();
    let mut tup: bpf_sock_tuple = core::mem::zeroed();
    let mut ct: *mut nf_conn;

    ct = bpf_skb_ct_lookup(
        ctx,
        &mut tup,
        core::mem::size_of_val(&tup.ipv4) as u32,
        &mut opts,
        core::mem::size_of_val(&opts) as u32,
    );
    if ct.is_null() {
        return 0;
    }
    bpf_ct_insert_entry(ct);
    return 0;
}

#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn write_not_allowlisted_field(ctx: *mut __sk_buff) -> i32 {
    let mut opts: bpf_ct_opts___local = core::mem::zeroed();
    let mut tup: bpf_sock_tuple = core::mem::zeroed();
    let mut ct: *mut nf_conn;

    ct = bpf_skb_ct_lookup(
        ctx,
        &mut tup,
        core::mem::size_of_val(&tup.ipv4) as u32,
        &mut opts,
        core::mem::size_of_val(&opts) as u32,
    );
    if ct.is_null() {
        return 0;
    }
    (*ct).status = 0xF00;
    return 0;
}

#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn set_timeout_after_insert(ctx: *mut __sk_buff) -> i32 {
    let mut opts: bpf_ct_opts___local = core::mem::zeroed();
    let mut tup: bpf_sock_tuple = core::mem::zeroed();
    let mut ct: *mut nf_conn;

    ct = bpf_skb_ct_alloc(
        ctx,
        &mut tup,
        core::mem::size_of_val(&tup.ipv4) as u32,
        &mut opts,
        core::mem::size_of_val(&opts) as u32,
    );
    if ct.is_null() {
        return 0;
    }
    ct = bpf_ct_insert_entry(ct);
    if ct.is_null() {
        return 0;
    }
    bpf_ct_set_timeout(ct, 0);
    return 0;
}

#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn set_status_after_insert(ctx: *mut __sk_buff) -> i32 {
    let mut opts: bpf_ct_opts___local = core::mem::zeroed();
    let mut tup: bpf_sock_tuple = core::mem::zeroed();
    let mut ct: *mut nf_conn;

    ct = bpf_skb_ct_alloc(
        ctx,
        &mut tup,
        core::mem::size_of_val(&tup.ipv4) as u32,
        &mut opts,
        core::mem::size_of_val(&opts) as u32,
    );
    if ct.is_null() {
        return 0;
    }
    ct = bpf_ct_insert_entry(ct);
    if ct.is_null() {
        return 0;
    }
    bpf_ct_set_status(ct, 0);
    return 0;
}

#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn change_timeout_after_alloc(ctx: *mut __sk_buff) -> i32 {
    let mut opts: bpf_ct_opts___local = core::mem::zeroed();
    let mut tup: bpf_sock_tuple = core::mem::zeroed();
    let mut ct: *mut nf_conn;

    ct = bpf_skb_ct_alloc(
        ctx,
        &mut tup,
        core::mem::size_of_val(&tup.ipv4) as u32,
        &mut opts,
        core::mem::size_of_val(&opts) as u32,
    );
    if ct.is_null() {
        return 0;
    }
    bpf_ct_change_timeout(ct, 0);
    return 0;
}

#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn change_status_after_alloc(ctx: *mut __sk_buff) -> i32 {
    let mut opts: bpf_ct_opts___local = core::mem::zeroed();
    let mut tup: bpf_sock_tuple = core::mem::zeroed();
    let mut ct: *mut nf_conn;

    ct = bpf_skb_ct_alloc(
        ctx,
        &mut tup,
        core::mem::size_of_val(&tup.ipv4) as u32,
        &mut opts,
        core::mem::size_of_val(&opts) as u32,
    );
    if ct.is_null() {
        return 0;
    }
    bpf_ct_change_status(ct, 0);
    return 0;
}

// __failure __msg("Possibly NULL pointer passed to trusted R2")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn lookup_null_bpf_tuple(ctx: *mut __sk_buff) -> i32 {
    let mut opts: bpf_ct_opts___local = core::mem::zeroed();
    let mut ct: *mut nf_conn;

    ct = bpf_skb_ct_lookup(
        ctx,
        core::ptr::null_mut(),
        0,
        &mut opts,
        core::mem::size_of_val(&opts) as u32,
    );
    if !ct.is_null() {
        bpf_ct_release(ct);
    }
    return 0;
}

// __failure __msg("Possibly NULL pointer passed to trusted R4")
#[no_mangle]
#[link_section = "?tc"]
pub unsafe extern "C" fn lookup_null_bpf_opts(ctx: *mut __sk_buff) -> i32 {
    let mut tup: bpf_sock_tuple = core::mem::zeroed();
    let mut ct: *mut nf_conn;

    ct = bpf_skb_ct_lookup(
        ctx,
        &mut tup,
        core::mem::size_of_val(&tup.ipv4) as u32,
        core::ptr::null_mut(),
        core::mem::size_of::<bpf_ct_opts___local>() as u32,
    );
    if !ct.is_null() {
        bpf_ct_release(ct);
    }
    return 0;
}

// __failure __msg("Possibly NULL pointer passed to trusted R2")
#[no_mangle]
#[link_section = "?xdp"]
pub unsafe extern "C" fn xdp_lookup_null_bpf_tuple(ctx: *mut xdp_md) -> i32 {
    let mut opts: bpf_ct_opts___local = core::mem::zeroed();
    let mut ct: *mut nf_conn;

    ct = bpf_xdp_ct_lookup(
        ctx,
        core::ptr::null_mut(),
        0,
        &mut opts,
        core::mem::size_of_val(&opts) as u32,
    );
    if !ct.is_null() {
        bpf_ct_release(ct);
    }
    return 0;
}

// __failure __msg("Possibly NULL pointer passed to trusted R4")
#[no_mangle]
#[link_section = "?xdp"]
pub unsafe extern "C" fn xdp_lookup_null_bpf_opts(ctx: *mut xdp_md) -> i32 {
    let mut tup: bpf_sock_tuple = core::mem::zeroed();
    let mut ct: *mut nf_conn;

    ct = bpf_xdp_ct_lookup(
        ctx,
        &mut tup,
        core::mem::size_of_val(&tup.ipv4) as u32,
        core::ptr::null_mut(),
        core::mem::size_of::<bpf_ct_opts___local>() as u32,
    );
    if !ct.is_null() {
        bpf_ct_release(ct);
    }
    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
