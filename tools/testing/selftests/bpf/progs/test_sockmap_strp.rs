// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_endian.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

extern "C" {
    fn bpf_sk_redirect_map(
        skb: *mut __sk_buff,
        map: *mut SockMapDef,
        key: __u32,
        flags: __u64,
    ) -> i32;
    fn bpf_skb_change_tail(skb: *mut __sk_buff, len: __u32, flags: __u64) -> i32;
}

pub type __u32 = u32;
pub type __u64 = u64;

pub const BPF_MAP_TYPE_SOCKMAP: u32 = 15;
pub const SK_PASS: i32 = 1;

#[repr(C)]
pub struct __sk_buff {
    pub len: __u32,
}

#[repr(C)]
pub struct SockMapDef {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[no_mangle]
pub static mut verdict_max_size: i32 = 10000;

#[no_mangle]
#[link_section = ".maps"]
pub static mut sock_map: SockMapDef = SockMapDef {
    type_: BPF_MAP_TYPE_SOCKMAP,
    max_entries: 20,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

#[no_mangle]
#[link_section = "sk_skb/stream_verdict"]
pub unsafe extern "C" fn prog_skb_verdict(skb: *mut __sk_buff) -> i32 {
    let one: __u32 = 1;

    if (*skb).len > verdict_max_size as __u32 {
        return SK_PASS;
    }

    bpf_sk_redirect_map(skb, &mut sock_map, one, 0)
}

#[no_mangle]
#[link_section = "sk_skb/stream_verdict"]
pub unsafe extern "C" fn prog_skb_verdict_pass(_skb: *mut __sk_buff) -> i32 {
    SK_PASS
}

#[no_mangle]
#[link_section = "sk_skb/stream_parser"]
pub unsafe extern "C" fn prog_skb_parser(skb: *mut __sk_buff) -> i32 {
    (*skb).len as i32
}

#[no_mangle]
#[link_section = "sk_skb/stream_parser"]
pub unsafe extern "C" fn prog_skb_parser_partial(skb: *mut __sk_buff) -> i32 {
    /* agreement with the test program on a 4-byte size header
     * and 6-byte body.
     */
    if (*skb).len < 4 {
        /* need more header to determine full length */
        return 0;
    }
    /* return full length decoded from header.
     * the return value may be larger than skb->len which
     * means framework must wait body coming.
     */
    10
}

#[no_mangle]
#[link_section = "sk_skb/stream_parser"]
pub unsafe extern "C" fn prog_skb_parser_resize(skb: *mut __sk_buff) -> i32 {
    bpf_skb_change_tail(skb, (*skb).len, 0);
    (*skb).len as i32
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
