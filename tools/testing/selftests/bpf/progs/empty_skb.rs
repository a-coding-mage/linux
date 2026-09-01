// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>

extern "C" {
    fn bpf_clone_redirect(skb: *mut __sk_buff, ifindex: i32, flags: u64) -> i64;
    fn bpf_skb_adjust_room(skb: *mut __sk_buff, len_diff: i32, mode: u32, flags: u64) -> i64;
}

extern "C" {
    type __sk_buff;
}

extern "C" {
    static BPF_F_INGRESS: u64;
    static BPF_ADJ_ROOM_NET: u32;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut ifindex: i32 = 0;

#[no_mangle]
pub static mut ret: i32 = 0;

#[no_mangle]
#[link_section = "lwt_xmit"]
pub unsafe extern "C" fn redirect_ingress(skb: *mut __sk_buff) -> i32 {
    ret = bpf_clone_redirect(skb, ifindex, BPF_F_INGRESS) as i32;
    0
}

#[no_mangle]
#[link_section = "lwt_xmit"]
pub unsafe extern "C" fn redirect_egress(skb: *mut __sk_buff) -> i32 {
    ret = bpf_clone_redirect(skb, ifindex, 0) as i32;
    0
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_redirect_ingress(skb: *mut __sk_buff) -> i32 {
    ret = bpf_clone_redirect(skb, ifindex, BPF_F_INGRESS) as i32;
    0
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_redirect_egress(skb: *mut __sk_buff) -> i32 {
    ret = bpf_clone_redirect(skb, ifindex, 0) as i32;
    0
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn tc_adjust_room(skb: *mut __sk_buff) -> i32 {
    ret = bpf_skb_adjust_room(skb, 4, BPF_ADJ_ROOM_NET, 0) as i32;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
