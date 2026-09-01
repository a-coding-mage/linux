// SPDX-License-Identifier: GPL-2.0-only
// C dependencies:
// - "vmlinux.h"
// - <bpf/bpf_helpers.h>
// - <bpf/bpf_endian.h>
// - "bpf_tracing_net.h"

const NF_DROP: i32 = 0;
const NF_ACCEPT: i32 = 1;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86DD;
const IP_MF: i32 = 0x2000;
const IP_OFFSET: i32 = 0x1FFF;
const NEXTHDR_FRAGMENT: u8 = 44;

#[no_mangle]
pub static mut shootdowns: i32 = 0;

extern "C" {
    fn bpf_ntohs(x: u16) -> u16;
    fn bpf_dynptr_from_skb(skb: *mut __sk_buff, flags: u64, ptr: *mut bpf_dynptr) -> i32;
    fn bpf_dynptr_slice(
        ptr: *const bpf_dynptr,
        offset: u32,
        buffer: *mut core::ffi::c_void,
        buffer__sz: u32,
    ) -> *mut core::ffi::c_void;
}

unsafe fn is_frag_v4(iph: *mut iphdr) -> bool {
    let mut offset: i32;
    let flags: i32;

    offset = bpf_ntohs((*iph).frag_off) as i32;
    flags = offset & !IP_OFFSET;
    offset &= IP_OFFSET;
    offset <<= 3;

    (flags & IP_MF) != 0 || offset != 0
}

unsafe fn is_frag_v6(ip6h: *mut ipv6hdr) -> bool {
    /* Simplifying assumption that there are no extension headers
     * between fixed header and fragmentation header. This assumption
     * is only valid in this test case. It saves us the hassle of
     * searching all potential extension headers.
     */
    (*ip6h).nexthdr == NEXTHDR_FRAGMENT
}

unsafe fn shootdowns_inc() {
    let current = core::ptr::read_volatile(core::ptr::addr_of!(shootdowns));
    core::ptr::write_volatile(core::ptr::addr_of_mut!(shootdowns), current.wrapping_add(1));
}

unsafe fn handle_v4(skb: *mut __sk_buff) -> i32 {
    let mut ptr: bpf_dynptr = core::mem::zeroed();
    let mut iph_buf: [u8; 20] = [0; 20];
    let mut iph: *mut iphdr;

    if bpf_dynptr_from_skb(skb, 0, core::ptr::addr_of_mut!(ptr)) != 0 {
        return NF_DROP;
    }

    iph = bpf_dynptr_slice(
        core::ptr::addr_of!(ptr),
        0,
        iph_buf.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&iph_buf) as u32,
    ) as *mut iphdr;
    if iph.is_null() {
        return NF_DROP;
    }

    /* Shootdown any frags */
    if is_frag_v4(iph) {
        shootdowns_inc();
        return NF_DROP;
    }

    NF_ACCEPT
}

unsafe fn handle_v6(skb: *mut __sk_buff) -> i32 {
    let mut ptr: bpf_dynptr = core::mem::zeroed();
    let mut ip6h: *mut ipv6hdr;
    let mut ip6h_buf: [u8; 40] = [0; 40];

    if bpf_dynptr_from_skb(skb, 0, core::ptr::addr_of_mut!(ptr)) != 0 {
        return NF_DROP;
    }

    ip6h = bpf_dynptr_slice(
        core::ptr::addr_of!(ptr),
        0,
        ip6h_buf.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&ip6h_buf) as u32,
    ) as *mut ipv6hdr;
    if ip6h.is_null() {
        return NF_DROP;
    }

    /* Shootdown any frags */
    if is_frag_v6(ip6h) {
        shootdowns_inc();
        return NF_DROP;
    }

    NF_ACCEPT
}

#[no_mangle]
#[link_section = "netfilter"]
pub unsafe extern "C" fn defrag(ctx: *mut bpf_nf_ctx) -> i32 {
    let skb: *mut __sk_buff = (*ctx).skb as *mut __sk_buff;

    match bpf_ntohs((*(*ctx).skb).protocol) {
        ETH_P_IP => handle_v4(skb),
        ETH_P_IPV6 => handle_v6(skb),
        _ => NF_ACCEPT,
    }
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
