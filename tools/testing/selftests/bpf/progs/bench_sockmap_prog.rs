// SPDX-License-Identifier: GPL-2.0
// Translated from C eBPF source. Original dependencies:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type c_int = i32;
type c_long = i64;
type __u32 = u32;

extern "C" {
    static BPF_MAP_TYPE_SOCKMAP: __u32;
    static SK_DROP: c_int;
    static SK_PASS: c_int;

    fn bpf_sk_redirect_map(
        skb: *mut __sk_buff,
        map: *mut bpf_map_def,
        key: c_int,
        flags: c_int,
    ) -> c_int;

    fn bpf_msg_redirect_map(
        msg: *mut sk_msg_md,
        map: *mut bpf_map_def,
        key: c_int,
        flags: c_int,
    ) -> c_int;
}

#[repr(C)]
pub struct __sk_buff {
    // Field supplied by linux/bpf.h; only the member used by this program is
    // represented here for source-level translation.
    pub len: __u32,
}

#[repr(C)]
pub struct sk_msg_md {
    // Field supplied by linux/bpf.h; only the member used by this program is
    // represented here for source-level translation.
    pub size: __u32,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[no_mangle]
pub static mut process_byte: c_long = 0;
#[no_mangle]
pub static mut verdict_dir: c_int = 0;
#[no_mangle]
pub static mut dropped: c_int = 0;
#[no_mangle]
pub static mut pkt_size: c_int = 0;

#[no_mangle]
#[link_section = ".maps"]
pub static mut sock_map_rx: bpf_map_def = bpf_map_def {
    type_: 15, // BPF_MAP_TYPE_SOCKMAP
    max_entries: 20,
    key_size: core::mem::size_of::<c_int>() as __u32,
    value_size: core::mem::size_of::<c_int>() as __u32,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut sock_map_tx: bpf_map_def = bpf_map_def {
    type_: 15, // BPF_MAP_TYPE_SOCKMAP
    max_entries: 20,
    key_size: core::mem::size_of::<c_int>() as __u32,
    value_size: core::mem::size_of::<c_int>() as __u32,
};

unsafe fn __sync_fetch_and_add_long(ptr: *mut c_long, val: __u32) -> c_long {
    core::intrinsics::atomic_xadd::<c_long>(ptr, val as c_long)
}

#[no_mangle]
#[link_section = "sk_skb/stream_parser"]
pub unsafe extern "C" fn prog_skb_parser(_skb: *mut __sk_buff) -> c_int {
    return pkt_size;
}

#[no_mangle]
#[link_section = "sk_skb/stream_verdict"]
pub unsafe extern "C" fn prog_skb_verdict(skb: *mut __sk_buff) -> c_int {
    let one: c_int = 1;
    let ret: c_int = bpf_sk_redirect_map(
        skb,
        core::ptr::addr_of_mut!(sock_map_rx),
        one,
        verdict_dir,
    );

    if ret == SK_DROP {
        dropped += 1;
    }
    __sync_fetch_and_add_long(core::ptr::addr_of_mut!(process_byte), (*skb).len);
    return ret;
}

#[no_mangle]
#[link_section = "sk_skb/stream_verdict"]
pub unsafe extern "C" fn prog_skb_pass(skb: *mut __sk_buff) -> c_int {
    __sync_fetch_and_add_long(core::ptr::addr_of_mut!(process_byte), (*skb).len);
    return SK_PASS;
}

#[no_mangle]
#[link_section = "sk_msg"]
pub unsafe extern "C" fn prog_skmsg_verdict(msg: *mut sk_msg_md) -> c_int {
    let one: c_int = 1;

    __sync_fetch_and_add_long(core::ptr::addr_of_mut!(process_byte), (*msg).size);
    return bpf_msg_redirect_map(
        msg,
        core::ptr::addr_of_mut!(sock_map_tx),
        one,
        verdict_dir,
    );
}

#[no_mangle]
#[link_section = "sk_msg"]
pub unsafe extern "C" fn prog_skmsg_pass(msg: *mut sk_msg_md) -> c_int {
    __sync_fetch_and_add_long(core::ptr::addr_of_mut!(process_byte), (*msg).size);
    return SK_PASS;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
