// SPDX-License-Identifier: GPL-2.0

// Translated from C eBPF source. Original include dependencies:
// <stddef.h>, <linux/bpf.h>, <linux/in.h>, <linux/if_ether.h>,
// <linux/ip.h>, <linux/ipv6.h>, <linux/udp.h>,
// <bpf/bpf_endian.h>, <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type __u8 = u8;
type __u16 = u16;
type __s16 = i16;
type __u32 = u32;
type __s32 = i32;
type __u64 = u64;
type __be32 = u32;

const MAX_ADJST_OFFSET: __u32 = 256;
const MAX_PAYLOAD_LEN: __u32 = 5000;
const MAX_HDR_LEN: usize = 64;

const ETH_ALEN: usize = 6;
const ETH_P_IP: __u16 = 0x0800;
const ETH_P_IPV6: __u16 = 0x86DD;
const IPPROTO_UDP: __u8 = 17;

const XDP_ABORTED: i32 = 0;
const XDP_DROP: i32 = 1;
const XDP_PASS: i32 = 2;
const XDP_TX: i32 = 3;

const BPF_MAP_TYPE_ARRAY: __u32 = 2;

#[repr(C)]
pub struct xdp_md {
    pub data: __u32,
    pub data_end: __u32,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [__u8; ETH_ALEN],
    pub h_source: [__u8; ETH_ALEN],
    pub h_proto: __u16,
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: __u8,
    pub tos: __u8,
    pub tot_len: __u16,
    pub id: __u16,
    pub frag_off: __u16,
    pub ttl: __u8,
    pub protocol: __u8,
    pub check: __u16,
    pub saddr: __be32,
    pub daddr: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [__u8; 16],
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: __u8,
    pub flow_lbl: [__u8; 3],
    pub payload_len: __u16,
    pub nexthdr: __u8,
    pub hop_limit: __u8,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

#[repr(C)]
pub struct udphdr {
    pub source: __u16,
    pub dest: __u16,
    pub len: __u16,
    pub check: __u16,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

extern "C" {
    // Original declaration used __ksym __weak.
    fn bpf_xdp_pull_data(xdp: *mut xdp_md, len: __u32) -> i32;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    fn bpf_csum_diff(
        from: *const __be32,
        from_size: __u32,
        to: *const __be32,
        to_size: __u32,
        seed: __u32,
    ) -> __u32;
    fn bpf_xdp_get_buff_len(ctx: *mut xdp_md) -> __u32;
    fn bpf_xdp_load_bytes(
        ctx: *mut xdp_md,
        offset: __u32,
        buf: *mut core::ffi::c_void,
        len: __u32,
    ) -> i32;
    fn bpf_xdp_store_bytes(
        ctx: *mut xdp_md,
        offset: __u32,
        buf: *const core::ffi::c_void,
        len: __u32,
    ) -> i32;
    fn bpf_xdp_adjust_tail(ctx: *mut xdp_md, delta: i32) -> i32;
    fn bpf_xdp_adjust_head(ctx: *mut xdp_md, delta: i32) -> i32;
    fn bpf_printk(fmt: *const u8, ...) -> i32;
}

const XDP_MODE: __u32 = 0;
const XDP_PORT: __u32 = 1;
const XDP_ADJST_OFFSET: __u32 = 2;
const XDP_ADJST_TAG: __u32 = 3;

const XDP_MODE_PASS: __s32 = 0;
const XDP_MODE_DROP: __s32 = 1;
const XDP_MODE_TX: __s32 = 2;
const XDP_MODE_TAIL_ADJST: __s32 = 3;
const XDP_MODE_HEAD_ADJST: __s32 = 4;

const STATS_RX: __u32 = 0;
const STATS_PASS: __u32 = 1;
const STATS_DROP: __u32 = 2;
const STATS_TX: __u32 = 3;
const STATS_ABORT: __u32 = 4;

// SEC(".maps")
#[no_mangle]
pub static mut map_xdp_setup: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 5,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__s32>() as __u32,
};

// SEC(".maps")
#[no_mangle]
pub static mut map_xdp_stats: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 5,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

#[inline]
unsafe fn bpf_htons(x: __u16) -> __u16 {
    x.to_be()
}

#[inline]
unsafe fn bpf_ntohs(x: __u16) -> __u16 {
    __u16::from_be(x)
}

unsafe fn min(a: __u32, b: __u32) -> __u32 {
    if a < b { a } else { b }
}

unsafe fn record_stats(_ctx: *mut xdp_md, stat_type: __u32) {
    let count: *mut __u64 = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(map_xdp_stats).cast(),
        core::ptr::addr_of!(stat_type).cast(),
    )
    .cast();

    if !count.is_null() {
        core::sync::atomic::AtomicU64::from_ptr(count).fetch_add(
            1,
            core::sync::atomic::Ordering::SeqCst,
        );
    }
}

unsafe fn filter_udphdr(ctx: *mut xdp_md, port: __u16) -> *mut udphdr {
    let mut udph: *mut udphdr = core::ptr::null_mut();
    let mut data: *mut u8;
    let mut data_end: *mut u8;
    let eth: *mut ethhdr;
    let mut err: i32;

    err = bpf_xdp_pull_data(ctx, core::mem::size_of::<ethhdr>() as __u32);
    if err != 0 {
        return core::ptr::null_mut();
    }

    data_end = (*ctx).data_end as usize as *mut u8;
    data = (*ctx).data as usize as *mut u8;
    eth = data.cast();

    if data.add(core::mem::size_of::<ethhdr>()) > data_end {
        return core::ptr::null_mut();
    }

    if (*eth).h_proto == bpf_htons(ETH_P_IP) {
        let iph: *mut iphdr;

        err = bpf_xdp_pull_data(
            ctx,
            (core::mem::size_of::<ethhdr>()
                + core::mem::size_of::<iphdr>()
                + core::mem::size_of::<udphdr>()) as __u32,
        );
        if err != 0 {
            return core::ptr::null_mut();
        }

        data_end = (*ctx).data_end as usize as *mut u8;
        data = (*ctx).data as usize as *mut u8;

        iph = data.add(core::mem::size_of::<ethhdr>()).cast();

        if iph.add(1) > data_end.cast::<iphdr>() || (*iph).protocol != IPPROTO_UDP {
            return core::ptr::null_mut();
        }

        udph = data
            .add(core::mem::size_of::<iphdr>() + core::mem::size_of::<ethhdr>())
            .cast();
    } else if (*eth).h_proto == bpf_htons(ETH_P_IPV6) {
        let ipv6h: *mut ipv6hdr;

        err = bpf_xdp_pull_data(
            ctx,
            (core::mem::size_of::<ethhdr>()
                + core::mem::size_of::<ipv6hdr>()
                + core::mem::size_of::<udphdr>()) as __u32,
        );
        if err != 0 {
            return core::ptr::null_mut();
        }

        data_end = (*ctx).data_end as usize as *mut u8;
        data = (*ctx).data as usize as *mut u8;

        ipv6h = data.add(core::mem::size_of::<ethhdr>()).cast();

        if ipv6h.add(1) > data_end.cast::<ipv6hdr>() || (*ipv6h).nexthdr != IPPROTO_UDP {
            return core::ptr::null_mut();
        }

        udph = data
            .add(core::mem::size_of::<ipv6hdr>() + core::mem::size_of::<ethhdr>())
            .cast();
    } else {
        return core::ptr::null_mut();
    }

    if udph.add(1) > data_end.cast::<udphdr>() {
        return core::ptr::null_mut();
    }

    if (*udph).dest != bpf_htons(port) {
        return core::ptr::null_mut();
    }

    record_stats(ctx, STATS_RX);

    udph
}

unsafe fn xdp_mode_pass(ctx: *mut xdp_md, port: __u16) -> i32 {
    let mut udph: *mut udphdr = core::ptr::null_mut();

    udph = filter_udphdr(ctx, port);
    if udph.is_null() {
        return XDP_PASS;
    }

    record_stats(ctx, STATS_PASS);

    XDP_PASS
}

unsafe fn xdp_mode_drop_handler(ctx: *mut xdp_md, port: __u16) -> i32 {
    let mut udph: *mut udphdr = core::ptr::null_mut();

    udph = filter_udphdr(ctx, port);
    if udph.is_null() {
        return XDP_PASS;
    }

    record_stats(ctx, STATS_DROP);

    XDP_DROP
}

unsafe fn swap_machdr(data: *mut core::ffi::c_void) {
    let eth: *mut ethhdr = data.cast();
    let mut tmp_mac: [__u8; ETH_ALEN] = [0; ETH_ALEN];

    core::ptr::copy_nonoverlapping((*eth).h_source.as_ptr(), tmp_mac.as_mut_ptr(), ETH_ALEN);
    core::ptr::copy_nonoverlapping((*eth).h_dest.as_ptr(), (*eth).h_source.as_mut_ptr(), ETH_ALEN);
    core::ptr::copy_nonoverlapping(tmp_mac.as_ptr(), (*eth).h_dest.as_mut_ptr(), ETH_ALEN);
}

unsafe fn xdp_mode_tx_handler(ctx: *mut xdp_md, port: __u16) -> i32 {
    let mut udph: *mut udphdr = core::ptr::null_mut();
    let mut data: *mut u8;
    let mut data_end: *mut u8;
    let mut eth: *mut ethhdr;
    let mut err: i32;

    err = bpf_xdp_pull_data(ctx, core::mem::size_of::<ethhdr>() as __u32);
    if err != 0 {
        return XDP_PASS;
    }

    data_end = (*ctx).data_end as usize as *mut u8;
    data = (*ctx).data as usize as *mut u8;
    eth = data.cast();

    if data.add(core::mem::size_of::<ethhdr>()) > data_end {
        return XDP_PASS;
    }

    if (*eth).h_proto == bpf_htons(ETH_P_IP) {
        let iph: *mut iphdr;
        let tmp_ip: __be32;

        err = bpf_xdp_pull_data(
            ctx,
            (core::mem::size_of::<ethhdr>()
                + core::mem::size_of::<iphdr>()
                + core::mem::size_of::<udphdr>()) as __u32,
        );
        if err != 0 {
            return XDP_PASS;
        }

        data_end = (*ctx).data_end as usize as *mut u8;
        data = (*ctx).data as usize as *mut u8;

        iph = data.add(core::mem::size_of::<ethhdr>()).cast();

        if iph.add(1) > data_end.cast::<iphdr>() || (*iph).protocol != IPPROTO_UDP {
            return XDP_PASS;
        }

        udph = data
            .add(core::mem::size_of::<iphdr>() + core::mem::size_of::<ethhdr>())
            .cast();

        if udph.add(1) > data_end.cast::<udphdr>() {
            return XDP_PASS;
        }
        if (*udph).dest != bpf_htons(port) {
            return XDP_PASS;
        }

        record_stats(ctx, STATS_RX);
        eth = data.cast();
        swap_machdr(eth.cast());

        tmp_ip = (*iph).saddr;
        (*iph).saddr = (*iph).daddr;
        (*iph).daddr = tmp_ip;

        record_stats(ctx, STATS_TX);

        return XDP_TX;
    } else if (*eth).h_proto == bpf_htons(ETH_P_IPV6) {
        let mut tmp_ipv6: in6_addr;
        let ipv6h: *mut ipv6hdr;

        err = bpf_xdp_pull_data(
            ctx,
            (core::mem::size_of::<ethhdr>()
                + core::mem::size_of::<ipv6hdr>()
                + core::mem::size_of::<udphdr>()) as __u32,
        );
        if err != 0 {
            return XDP_PASS;
        }

        data_end = (*ctx).data_end as usize as *mut u8;
        data = (*ctx).data as usize as *mut u8;

        ipv6h = data.add(core::mem::size_of::<ethhdr>()).cast();

        if ipv6h.add(1) > data_end.cast::<ipv6hdr>() || (*ipv6h).nexthdr != IPPROTO_UDP {
            return XDP_PASS;
        }

        udph = data
            .add(core::mem::size_of::<ipv6hdr>() + core::mem::size_of::<ethhdr>())
            .cast();

        if udph.add(1) > data_end.cast::<udphdr>() {
            return XDP_PASS;
        }
        if (*udph).dest != bpf_htons(port) {
            return XDP_PASS;
        }

        record_stats(ctx, STATS_RX);
        eth = data.cast();
        swap_machdr(eth.cast());

        tmp_ipv6 = (*ipv6h).saddr;
        (*ipv6h).saddr = (*ipv6h).daddr;
        (*ipv6h).daddr = tmp_ipv6;

        record_stats(ctx, STATS_TX);

        return XDP_TX;
    }

    XDP_PASS
}

#[inline(always)]
unsafe fn csum_fold_helper(mut csum: __u32) -> __u16 {
    csum = (csum & 0xffff).wrapping_add(csum >> 16);
    !((csum & 0xffff).wrapping_add(csum >> 16) as __u16)
}

#[inline(always)]
unsafe fn csum_fold_udp_helper(csum: __u32) -> __u16 {
    let folded = csum_fold_helper(csum);
    if folded != 0 { folded } else { 0xffff }
}

unsafe fn update_pkt(ctx: *mut xdp_md, offset: __s16, udp_csum: *mut __u32) -> *mut core::ffi::c_void {
    let data_end: *mut u8 = (*ctx).data_end as usize as *mut u8;
    let data: *mut u8 = (*ctx).data as usize as *mut u8;
    let mut udph: *mut udphdr = core::ptr::null_mut();
    let eth: *mut ethhdr = data.cast();
    let mut len: __u32;
    let len_new: __u32;

    if data.add(core::mem::size_of::<ethhdr>()) > data_end {
        return core::ptr::null_mut();
    }

    if (*eth).h_proto == bpf_htons(ETH_P_IP) {
        let iph: *mut iphdr = data.add(core::mem::size_of::<ethhdr>()).cast();

        if iph.add(1) > data_end.cast::<iphdr>() {
            return core::ptr::null_mut();
        }

        udph = (eth.cast::<u8>())
            .add(core::mem::size_of::<iphdr>() + core::mem::size_of::<ethhdr>())
            .cast();
        if udph.is_null() || udph.add(1) > data_end.cast::<udphdr>() {
            return core::ptr::null_mut();
        }

        len = (*iph).tot_len as __u32;
        len_new = bpf_htons((bpf_ntohs(len as __u16) as __s16).wrapping_add(offset) as __u16) as __u32;
        (*iph).tot_len = len_new as __u16;
        (*iph).check = csum_fold_helper(bpf_csum_diff(
            core::ptr::addr_of!(len).cast(),
            core::mem::size_of_val(&len) as __u32,
            core::ptr::addr_of!(len_new).cast(),
            core::mem::size_of_val(&len_new) as __u32,
            !((*iph).check as __u32),
        ));
    } else if (*eth).h_proto == bpf_htons(ETH_P_IPV6) {
        let ipv6h: *mut ipv6hdr = data.add(core::mem::size_of::<ethhdr>()).cast();

        if ipv6h.add(1) > data_end.cast::<ipv6hdr>() {
            return core::ptr::null_mut();
        }

        udph = (eth.cast::<u8>())
            .add(core::mem::size_of::<ipv6hdr>() + core::mem::size_of::<ethhdr>())
            .cast();
        if udph.is_null() || udph.add(1) > data_end.cast::<udphdr>() {
            return core::ptr::null_mut();
        }

        len = (*ipv6h).payload_len as __u32;
        len_new = bpf_htons((bpf_ntohs(len as __u16) as __s16).wrapping_add(offset) as __u16) as __u32;
        (*ipv6h).payload_len = len_new as __u16;
    } else {
        return core::ptr::null_mut();
    }

    len = (*udph).len as __u32;
    len_new = bpf_htons((bpf_ntohs(len as __u16) as __s16).wrapping_add(offset) as __u16) as __u32;

    *udp_csum = !((*udph).check as __u32);
    *udp_csum = bpf_csum_diff(
        core::ptr::addr_of!(len).cast(),
        core::mem::size_of_val(&len) as __u32,
        core::ptr::addr_of!(len_new).cast(),
        core::mem::size_of_val(&len_new) as __u32,
        *udp_csum,
    );
    *udp_csum = bpf_csum_diff(
        core::ptr::addr_of!(len).cast(),
        core::mem::size_of_val(&len) as __u32,
        core::ptr::addr_of!(len_new).cast(),
        core::mem::size_of_val(&len_new) as __u32,
        *udp_csum,
    );

    (*udph).len = len_new as __u16;

    udph.cast()
}

unsafe fn xdp_adjst_tail_shrnk_data(ctx: *mut xdp_md, mut offset: __u16, hdr_len: usize) -> i32 {
    let mut tmp_buff: [i8; MAX_ADJST_OFFSET as usize] = [0; MAX_ADJST_OFFSET as usize];
    let mut buff_pos: __u32;
    let mut udp_csum: __u32 = 0;
    let mut udph: *mut udphdr = core::ptr::null_mut();
    let buff_len: __u32;

    udph = update_pkt(ctx, (0i16).wrapping_sub(offset as __s16), core::ptr::addr_of_mut!(udp_csum)).cast();
    if udph.is_null() {
        return -1;
    }

    buff_len = bpf_xdp_get_buff_len(ctx);

    offset = if ((offset as __u32) & 0x1ff) >= MAX_ADJST_OFFSET {
        MAX_ADJST_OFFSET as __u16
    } else {
        offset & 0xff
    };
    if offset == 0 {
        return -1;
    }

    /* Make sure we have enough data to avoid eating the header */
    if buff_len.wrapping_sub(offset as __u32) < hdr_len as __u32 {
        return -1;
    }

    buff_pos = buff_len.wrapping_sub(offset as __u32);
    if bpf_xdp_load_bytes(ctx, buff_pos, tmp_buff.as_mut_ptr().cast(), offset as __u32) < 0 {
        return -1;
    }

    udp_csum = bpf_csum_diff(tmp_buff.as_ptr().cast(), offset as __u32, core::ptr::null(), 0, udp_csum);
    (*udph).check = csum_fold_udp_helper(udp_csum) as __u16;

    if bpf_xdp_adjust_tail(ctx, (0i32).wrapping_sub(offset as i32)) < 0 {
        return -1;
    }

    0
}

unsafe fn xdp_adjst_tail_grow_data(ctx: *mut xdp_md, mut offset: __u16) -> i32 {
    let mut tmp_buff: [i8; MAX_ADJST_OFFSET as usize] = [0; MAX_ADJST_OFFSET as usize];
    let mut udp_csum: __u32 = 0;
    let buff_len: __u32;
    let mut key: __u32;
    let mut udph: *mut udphdr;
    let val: *mut __s32;
    let tag: __u8;

    /* Proceed to update the packet headers before attempting to adjuste
     * the tail. Once the tail is adjusted we lose access to the offset
     * amount of data at the end of the packet which is crucial to update
     * the checksum.
     * Since any failure beyond this would abort the packet, we should
     * not worry about passing a packet up the stack with wrong headers
     */
    udph = update_pkt(ctx, offset as __s16, core::ptr::addr_of_mut!(udp_csum)).cast();
    if udph.is_null() {
        return -1;
    }

    key = XDP_ADJST_TAG;
    val = bpf_map_lookup_elem(core::ptr::addr_of_mut!(map_xdp_setup).cast(), core::ptr::addr_of!(key).cast()).cast();
    if val.is_null() {
        return -1;
    }

    tag = *val as __u8;

    for i in 0..MAX_ADJST_OFFSET as usize {
        core::ptr::copy_nonoverlapping(core::ptr::addr_of!(tag), tmp_buff.as_mut_ptr().add(i).cast(), 1);
    }

    offset = if ((offset as __u32) & 0x1ff) >= MAX_ADJST_OFFSET {
        MAX_ADJST_OFFSET as __u16
    } else {
        offset & 0xff
    };
    if offset == 0 {
        return -1;
    }

    udp_csum = bpf_csum_diff(core::ptr::null(), 0, tmp_buff.as_ptr().cast(), offset as __u32, udp_csum);
    (*udph).check = csum_fold_udp_helper(udp_csum) as __u16;

    buff_len = bpf_xdp_get_buff_len(ctx);

    if bpf_xdp_adjust_tail(ctx, offset as i32) < 0 {
        bpf_printk(c"Failed to adjust tail\n".as_ptr().cast());
        return -1;
    }

    if bpf_xdp_store_bytes(ctx, buff_len, tmp_buff.as_ptr().cast(), offset as __u32) < 0 {
        return -1;
    }

    0
}

unsafe fn xdp_adjst_tail(ctx: *mut xdp_md, port: __u16) -> i32 {
    let mut udph: *mut udphdr = core::ptr::null_mut();
    let adjust_offset: *mut __s32;
    let hdr_len: usize;
    let mut key: __u32;
    let ret: i32;

    udph = filter_udphdr(ctx, port);
    if udph.is_null() {
        return XDP_PASS;
    }

    hdr_len = (udph.cast::<u8>() as usize)
        .wrapping_sub((*ctx).data as usize)
        .wrapping_add(core::mem::size_of::<udphdr>());
    key = XDP_ADJST_OFFSET;
    adjust_offset =
        bpf_map_lookup_elem(core::ptr::addr_of_mut!(map_xdp_setup).cast(), core::ptr::addr_of!(key).cast()).cast();
    if adjust_offset.is_null() {
        return XDP_PASS;
    }

    if *adjust_offset < 0 {
        ret = xdp_adjst_tail_shrnk_data(ctx, (0i32).wrapping_sub(*adjust_offset) as __u16, hdr_len);
    } else {
        ret = xdp_adjst_tail_grow_data(ctx, *adjust_offset as __u16);
    }
    if ret != 0 {
        record_stats(ctx, STATS_ABORT);
        return XDP_ABORTED;
    }

    record_stats(ctx, STATS_PASS);
    XDP_PASS
}

unsafe fn xdp_adjst_head_shrnk_data(ctx: *mut xdp_md, mut hdr_len: __u64, mut offset: __u32) -> i32 {
    let mut tmp_buff: [i8; MAX_ADJST_OFFSET as usize] = [0; MAX_ADJST_OFFSET as usize];
    let mut udph: *mut udphdr;
    let mut udp_csum: __u32 = 0;

    /* Update the length information in the IP and UDP headers before
     * adjusting the headroom. This simplifies accessing the relevant
     * fields in the IP and UDP headers for fragmented packets. Any
     * failure beyond this point will result in the packet being aborted,
     * so we don't need to worry about incorrect length information for
     * passed packets.
     */
    udph = update_pkt(ctx, (0i16).wrapping_sub(offset as __s16), core::ptr::addr_of_mut!(udp_csum)).cast();
    if udph.is_null() {
        return -1;
    }

    offset = if (offset & 0x1ff) >= MAX_ADJST_OFFSET {
        MAX_ADJST_OFFSET
    } else {
        offset & 0xff
    };
    if offset == 0 {
        return -1;
    }

    if bpf_xdp_load_bytes(ctx, hdr_len as __u32, tmp_buff.as_mut_ptr().cast(), offset) < 0 {
        return -1;
    }

    udp_csum = bpf_csum_diff(tmp_buff.as_ptr().cast(), offset, core::ptr::null(), 0, udp_csum);
    (*udph).check = csum_fold_udp_helper(udp_csum) as __u16;

    if bpf_xdp_load_bytes(ctx, 0, tmp_buff.as_mut_ptr().cast(), MAX_ADJST_OFFSET) < 0 {
        return -1;
    }

    if bpf_xdp_adjust_head(ctx, offset as i32) < 0 {
        return -1;
    }

    if offset > MAX_ADJST_OFFSET {
        return -1;
    }

    if hdr_len > MAX_ADJST_OFFSET as __u64 || hdr_len == 0 {
        return -1;
    }

    /* Added here to handle clang complain about negative value */
    hdr_len = hdr_len & 0xff;

    if hdr_len == 0 {
        return -1;
    }

    if bpf_xdp_store_bytes(ctx, 0, tmp_buff.as_ptr().cast(), hdr_len as __u32) < 0 {
        return -1;
    }

    0
}

unsafe fn xdp_adjst_head_grow_data(ctx: *mut xdp_md, mut hdr_len: __u64, mut offset: __u32) -> i32 {
    let mut hdr_buff: [i8; MAX_HDR_LEN] = [0; MAX_HDR_LEN];
    let mut data_buff: [i8; MAX_ADJST_OFFSET as usize] = [0; MAX_ADJST_OFFSET as usize];
    let val: *mut __s32;
    let mut key: __u32;
    let tag: __u8;
    let mut udp_csum: __u32 = 0;
    let mut udph: *mut udphdr;

    udph = update_pkt(ctx, offset as __s16, core::ptr::addr_of_mut!(udp_csum)).cast();
    if udph.is_null() {
        return -1;
    }

    key = XDP_ADJST_TAG;
    val = bpf_map_lookup_elem(core::ptr::addr_of_mut!(map_xdp_setup).cast(), core::ptr::addr_of!(key).cast()).cast();
    if val.is_null() {
        return -1;
    }

    tag = *val as __u8;
    for i in 0..MAX_ADJST_OFFSET as usize {
        core::ptr::copy_nonoverlapping(core::ptr::addr_of!(tag), data_buff.as_mut_ptr().add(i).cast(), 1);
    }

    offset = if (offset & 0x1ff) >= MAX_ADJST_OFFSET {
        MAX_ADJST_OFFSET
    } else {
        offset & 0xff
    };
    if offset == 0 {
        return -1;
    }

    udp_csum = bpf_csum_diff(core::ptr::null(), 0, data_buff.as_ptr().cast(), offset, udp_csum);
    (*udph).check = csum_fold_udp_helper(udp_csum) as __u16;

    if hdr_len > MAX_ADJST_OFFSET as __u64 || hdr_len == 0 {
        return -1;
    }

    /* Added here to handle clang complain about negative value */
    hdr_len = hdr_len & 0xff;

    if hdr_len == 0 {
        return -1;
    }

    if bpf_xdp_load_bytes(ctx, 0, hdr_buff.as_mut_ptr().cast(), hdr_len as __u32) < 0 {
        return -1;
    }

    if offset > MAX_ADJST_OFFSET {
        return -1;
    }

    if bpf_xdp_adjust_head(ctx, (0i32).wrapping_sub(offset as i32)) < 0 {
        return -1;
    }

    if bpf_xdp_store_bytes(ctx, 0, hdr_buff.as_ptr().cast(), hdr_len as __u32) < 0 {
        return -1;
    }

    if bpf_xdp_store_bytes(ctx, hdr_len as __u32, data_buff.as_ptr().cast(), offset) < 0 {
        return -1;
    }

    0
}

unsafe fn xdp_head_adjst(ctx: *mut xdp_md, port: __u16) -> i32 {
    let mut udph_ptr: *mut udphdr = core::ptr::null_mut();
    let mut key: __u32;
    let size: __u32;
    let hdr_len: __u32;
    let val: *mut __s32;
    let res: i32;

    /* Filter packets based on UDP port */
    udph_ptr = filter_udphdr(ctx, port);
    if udph_ptr.is_null() {
        return XDP_PASS;
    }

    hdr_len = (udph_ptr.cast::<u8>() as usize)
        .wrapping_sub((*ctx).data as usize)
        .wrapping_add(core::mem::size_of::<udphdr>()) as __u32;

    key = XDP_ADJST_OFFSET;
    val = bpf_map_lookup_elem(core::ptr::addr_of_mut!(map_xdp_setup).cast(), core::ptr::addr_of!(key).cast()).cast();
    if val.is_null() {
        return XDP_PASS;
    }

    match *val {
        -16 | 16 => {
            size = 16;
        }
        -32 | 32 => {
            size = 32;
        }
        -64 | 64 => {
            size = 64;
        }
        -128 | 128 => {
            size = 128;
        }
        -256 | 256 => {
            size = 256;
        }
        _ => {
            bpf_printk(c"Invalid adjustment offset: %d\n".as_ptr().cast(), *val);
            record_stats(ctx, STATS_ABORT);
            return XDP_ABORTED;
        }
    }

    if *val < 0 {
        res = xdp_adjst_head_grow_data(ctx, hdr_len as __u64, size);
    } else {
        res = xdp_adjst_head_shrnk_data(ctx, hdr_len as __u64, size);
    }

    if res != 0 {
        record_stats(ctx, STATS_ABORT);
        return XDP_ABORTED;
    }

    record_stats(ctx, STATS_PASS);
    XDP_PASS
}

unsafe fn xdp_prog_common(ctx: *mut xdp_md) -> i32 {
    let mut key: __u32;
    let port: *mut __u32;
    let mode: *mut __s32;

    key = XDP_MODE;
    mode = bpf_map_lookup_elem(core::ptr::addr_of_mut!(map_xdp_setup).cast(), core::ptr::addr_of!(key).cast()).cast();
    if mode.is_null() {
        return XDP_PASS;
    }

    key = XDP_PORT;
    port = bpf_map_lookup_elem(core::ptr::addr_of_mut!(map_xdp_setup).cast(), core::ptr::addr_of!(key).cast()).cast();
    if port.is_null() {
        return XDP_PASS;
    }

    match *mode {
        XDP_MODE_PASS => return xdp_mode_pass(ctx, *port as __u16),
        XDP_MODE_DROP => return xdp_mode_drop_handler(ctx, *port as __u16),
        XDP_MODE_TX => return xdp_mode_tx_handler(ctx, *port as __u16),
        XDP_MODE_TAIL_ADJST => return xdp_adjst_tail(ctx, *port as __u16),
        XDP_MODE_HEAD_ADJST => return xdp_head_adjst(ctx, *port as __u16),
        _ => {}
    }

    /* Default action is to simple pass */
    XDP_PASS
}

// SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_prog(ctx: *mut xdp_md) -> i32 {
    xdp_prog_common(ctx)
}

// SEC("xdp.frags")
#[no_mangle]
pub unsafe extern "C" fn xdp_prog_frags(ctx: *mut xdp_md) -> i32 {
    xdp_prog_common(ctx)
}

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
