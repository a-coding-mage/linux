// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 ByteDance */

// C dependencies: "vmlinux.h" and <bpf/bpf_helpers.h>.
// PAGE_SIZE defaults to __PAGE_SIZE when PAGE_SIZE is not already defined.
pub const PAGE_SIZE: u64 = __PAGE_SIZE as u64;
pub const BPF_SKB_MAX_LEN: u64 = PAGE_SIZE << 2;

#[repr(C)]
pub struct SockMapRx {
    // __uint(type, BPF_MAP_TYPE_SOCKMAP);
    // __uint(max_entries, 1);
    // __type(key, int);
    // __type(value, int);
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut sock_map_rx: SockMapRx = SockMapRx {};

#[no_mangle]
pub static mut change_tail_ret: i64 = 1;

extern "C" {
    pub static __PAGE_SIZE: u64;

    pub fn bpf_skb_pull_data(skb: *mut __sk_buff, len: u32) -> i64;
    pub fn bpf_skb_change_tail(skb: *mut __sk_buff, len: u32, flags: u64) -> i64;
}

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
    pub data: u32,
    pub data_end: u32,
}

pub const SK_PASS: i32 = 1;

#[link_section = "sk_skb"]
#[no_mangle]
pub unsafe extern "C" fn prog_skb_verdict(skb: *mut __sk_buff) -> i32 {
    let mut data: *mut i8;
    let mut data_end: *mut i8;

    bpf_skb_pull_data(skb, 1);
    data = (*skb).data as u64 as *mut i8;
    data_end = (*skb).data_end as u64 as *mut i8;

    if data.add(1) > data_end {
        return SK_PASS;
    }

    if *data.add(0) == b'T' as i8 {
        /* Trim the packet */
        change_tail_ret = bpf_skb_change_tail(skb, (*skb).len.wrapping_sub(1), 0);
        return SK_PASS;
    } else if *data.add(0) == b'G' as i8 {
        /* Grow the packet */
        change_tail_ret = bpf_skb_change_tail(skb, (*skb).len.wrapping_add(1), 0);
        return SK_PASS;
    } else if *data.add(0) == b'E' as i8 {
        /* Error */
        change_tail_ret = bpf_skb_change_tail(skb, BPF_SKB_MAX_LEN as u32, 0);
        return SK_PASS;
    }
    SK_PASS
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
