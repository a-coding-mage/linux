/* Copyright (c) 2016 Thomas Graf <tgraf@tgraf.ch>
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of the version 2 of the GNU General
 * Public License as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 */

// C dependencies: vmlinux.h, net_shared.h, bpf/bpf_helpers.h, and string.h.

const CB_MAGIC: i32 = 1234;
const IS_PSEUDO: u64 = 0x10;

// External types, constants, globals, and helpers are supplied by the BPF environment.
extern "C" {
    fn bpf_trace_printk(fmt: *mut u8, size: u32, ...) -> i64;
    fn bpf_skb_load_bytes(skb: *mut __sk_buff, offset: u32, to: *mut u8, len: u32) -> i32;
    fn bpf_l4_csum_replace(skb: *mut __sk_buff, offset: u32, from: u32, to: u32, flags: u64) -> i32;
    fn bpf_l3_csum_replace(skb: *mut __sk_buff, offset: u32, from: u32, to: u32, size: u64) -> i32;
    fn bpf_skb_store_bytes(skb: *mut __sk_buff, offset: u32, from: *const u8, len: u32, flags: u64) -> i32;
    fn bpf_skb_change_head(skb: *mut __sk_buff, len: u32, flags: u64) -> i32;
    fn bpf_redirect(ifindex: i32, flags: u64) -> i32;
    fn bpf_htons(value: u16) -> u16;
}

// Supplied by included headers.
#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
    pub hash: u32,
    pub protocol: u32,
    pub cb: [u32; 5],
    pub ingress_ifindex: u32,
    pub ifindex: u32,
    pub data: u32,
    pub data_end: u32,
}
#[repr(C)] pub struct iphdr { pub saddr: u32, pub daddr: u32, pub check: u16, pub protocol: u8 }
#[repr(C)] pub struct tcphdr { pub check: u16 }
#[repr(C)] pub struct udphdr { pub check: u16 }
#[repr(C)] pub struct icmp6hdr { pub icmp6_cksum: u16 }
#[repr(C)] pub struct ethhdr { pub h_proto: u16, pub h_source: [u8; 6], pub h_dest: [u8; 6] }

// Build-time symbols supplied by net_shared.h.
extern "C" {
    static SRC_MAC: u64;
    static DST_MAC: u64;
    static DST_IFINDEX: i32;
}

const BPF_OK: i32 = 0;
const BPF_DROP: i32 = 2;
const BPF_F_MARK_MANGLED_0: u64 =  const_bpf_f_mark_mangled_0();
const IPPROTO_TCP: u8 = 6;
const IPPROTO_UDP: u8 = 17;
const IPPROTO_ICMPV6: u8 = 58;

const fn const_bpf_f_mark_mangled_0() -> u64 { 1 << 5 }

macro_rules! printk {
    ($fmt:expr $(, $arg:expr)*) => {{
        let mut ____fmt = concat!($fmt, "\\0").as_bytes().to_vec();
        unsafe { bpf_trace_printk(____fmt.as_mut_ptr(), ____fmt.len() as u32 $(, $arg)*) }
    }};
}

#[no_mangle]
pub unsafe extern "C" fn do_nop(_skb: *mut __sk_buff) -> i32 { BPF_OK }

#[no_mangle]
pub unsafe extern "C" fn do_test_ctx(skb: *mut __sk_buff) -> i32 {
    (*skb).cb[0] = CB_MAGIC as u32;
    printk!("len %d hash %d protocol %d", (*skb).len, (*skb).hash, (*skb).protocol);
    printk!("cb %d ingress_ifindex %d ifindex %d", (*skb).cb[0], (*skb).ingress_ifindex, (*skb).ifindex);
    BPF_OK
}

#[no_mangle]
pub unsafe extern "C" fn do_test_cb(skb: *mut __sk_buff) -> i32 {
    printk!("cb0: %x cb1: %x cb2: %x", (*skb).cb[0], (*skb).cb[1], (*skb).cb[2]);
    printk!("cb3: %x cb4: %x", (*skb).cb[3], (*skb).cb[4]);
    BPF_OK
}

#[no_mangle]
pub unsafe extern "C" fn do_test_data(skb: *mut __sk_buff) -> i32 {
    let data = (*skb).data as usize as *mut u8;
    let data_end = (*skb).data_end as usize as *mut u8;
    let iph = data as *mut iphdr;
    if (data as usize + core::mem::size_of::<iphdr>()) > data_end as usize {
        printk!("packet truncated");
        return BPF_DROP;
    }
    printk!("src: %x dst: %x", (*iph).saddr, (*iph).daddr);
    BPF_OK
}

const IP_CSUM_OFF: u32 = 10;
const IP_DST_OFF: u32 = 16;
const IP_SRC_OFF: u32 = 12;
const IP_PROTO_OFF: u32 = 9;
const TCP_CSUM_OFF: u32 = 16;
const UDP_CSUM_OFF: u32 = 6;

unsafe fn rewrite(skb: *mut __sk_buff, old_ip: u32, new_ip: u32, rw_daddr: i32) -> i32 {
    let mut off = 0u32;
    let mut flags = IS_PSEUDO;
    let mut proto = 0u8;
    let mut ret = bpf_skb_load_bytes(skb, IP_PROTO_OFF, &mut proto, 1);
    if ret < 0 { printk!("bpf_l4_csum_replace failed: %d", ret); return BPF_DROP; }
    match proto { IPPROTO_TCP => off = TCP_CSUM_OFF, IPPROTO_UDP => { off = UDP_CSUM_OFF; flags |= BPF_F_MARK_MANGLED_0; }, IPPROTO_ICMPV6 => off = 2, _ => {} }
    if off != 0 {
        ret = bpf_l4_csum_replace(skb, off, old_ip, new_ip, flags | core::mem::size_of::<u32>() as u64);
        if ret < 0 { printk!("bpf_l4_csum_replace failed: %d"); return BPF_DROP; }
    }
    ret = bpf_l3_csum_replace(skb, IP_CSUM_OFF, old_ip, new_ip, core::mem::size_of::<u32>() as u64);
    if ret < 0 { printk!("bpf_l3_csum_replace failed: %d", ret); return BPF_DROP; }
    ret = if rw_daddr != 0 { bpf_skb_store_bytes(skb, IP_DST_OFF, &new_ip as *const u32 as *const u8, 4, 0) } else { bpf_skb_store_bytes(skb, IP_SRC_OFF, &new_ip as *const u32 as *const u8, 4, 0) };
    if ret < 0 { printk!("bpf_skb_store_bytes() failed: %d", ret); return BPF_DROP; }
    BPF_OK
}

#[no_mangle]
pub unsafe extern "C" fn do_test_rewrite(skb: *mut __sk_buff) -> i32 {
    let mut old_ip = 0u32;
    let new_ip = 0x3fea8c0u32;
    let ret = bpf_skb_load_bytes(skb, IP_DST_OFF, &mut old_ip, 4);
    if ret < 0 { printk!("bpf_skb_load_bytes failed: %d", ret); return BPF_DROP; }
    if old_ip == 0x2fea8c0 { printk!("out: rewriting from %x to %x", old_ip, new_ip); return rewrite(skb, old_ip, new_ip, 1); }
    BPF_OK
}

unsafe fn __do_push_ll_and_redirect(skb: *mut __sk_buff) -> i32 {
    let smac = SRC_MAC; let dmac = DST_MAC; let ifindex = DST_IFINDEX;
    let ret = bpf_skb_change_head(skb, 14, 0);
    if ret < 0 { printk!("skb_change_head() failed: %d", ret); }
    let mut ehdr = ethhdr { h_proto: bpf_htons(0x0800), h_source: [0; 6], h_dest: [0; 6] };
    core::ptr::copy_nonoverlapping(&smac as *const u64 as *const u8, ehdr.h_source.as_mut_ptr(), 6);
    core::ptr::copy_nonoverlapping(&dmac as *const u64 as *const u8, ehdr.h_dest.as_mut_ptr(), 6);
    let ret = bpf_skb_store_bytes(skb, 0, &ehdr as *const ethhdr as *const u8, core::mem::size_of::<ethhdr>() as u32, 0);
    if ret < 0 { printk!("skb_store_bytes() failed: %d", ret); return BPF_DROP; }
    bpf_redirect(ifindex, 0)
}

#[no_mangle] pub unsafe extern "C" fn do_push_ll_and_redirect_silent(skb: *mut __sk_buff) -> i32 { __do_push_ll_and_redirect(skb) }
#[no_mangle] pub unsafe extern "C" fn do_push_ll_and_redirect(skb: *mut __sk_buff) -> i32 { let ifindex = DST_IFINDEX; let ret = __do_push_ll_and_redirect(skb); if ret >= 0 { printk!("redirected to %d", ifindex); } ret }

unsafe fn __fill_garbage(skb: *mut __sk_buff) { let f = 0xffffffffffffffffu64; for off in [0u32,8,16,24,32,40,48,56,64,72,80,88] { bpf_skb_store_bytes(skb, off, &f as *const u64 as *const u8, 8, 0); } }

#[no_mangle] pub unsafe extern "C" fn do_fill_garbage(skb: *mut __sk_buff) -> i32 { __fill_garbage(skb); printk!("Set initial 96 bytes of header to FF"); BPF_OK }
#[no_mangle] pub unsafe extern "C" fn do_fill_garbage_and_redirect(skb: *mut __sk_buff) -> i32 { let ifindex = DST_IFINDEX; __fill_garbage(skb); printk!("redirected to %d", ifindex); bpf_redirect(ifindex, 0) }
#[no_mangle] pub unsafe extern "C" fn do_drop_all(_skb: *mut __sk_buff) -> i32 { printk!("dropping with: %d", BPF_DROP); BPF_DROP }

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
