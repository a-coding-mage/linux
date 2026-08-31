// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C includes:
// linux/bpf.h, linux/netdev.h, bpf/bpf_helpers.h, bpf/bpf_endian.h,
// bpf/bpf_tracing.h, linux/if_ether.h, linux/ip.h, linux/ipv6.h, linux/in.h,
// linux/in6.h, linux/udp.h, asm-generic/errno-base.h, and "xdp_features.h".

use core::ffi::c_void;

extern "C" {
    static stats: c_void;
    static dut_stats: c_void;
    static cpu_map: c_void;
    static dev_map: c_void;

    fn bpf_map_lookup_elem(map: *const c_void, key: *const c_void) -> *mut __u32;
    fn bpf_redirect_map(map: *const c_void, key: __u32, flags: __u64) -> i32;
}

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __be16 = u16;
pub type __be32 = u32;

pub const ETH_ALEN: usize = 6;
pub const ETH_P_IP: __u16 = 0x0800;
pub const ETH_P_IPV6: __u16 = 0x86DD;
pub const IPPROTO_UDP: __u8 = 17;
pub const EINVAL: i32 = 22;

// Supplied by "xdp_features.h" in the original repository.
extern "C" {
    static DUT_ECHO_PORT: __u16;
    static CMD_ECHO: __be16;
}

// XDP action constants are supplied by linux/bpf.h in the original source.
extern "C" {
    static XDP_ABORTED: i32;
    static XDP_DROP: i32;
    static XDP_PASS: i32;
    static XDP_TX: i32;
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_prog {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xdp_md {
    pub data: __u32,
    pub data_end: __u32,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [__u8; ETH_ALEN],
    pub h_source: [__u8; ETH_ALEN],
    pub h_proto: __be16,
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: __u8,
    pub tos: __u8,
    pub tot_len: __be16,
    pub id: __be16,
    pub frag_off: __be16,
    pub ttl: __u8,
    pub protocol: __u8,
    pub check: __be16,
    pub saddr: __be32,
    pub daddr: __be32,
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: __u8,
    pub flow_lbl: [__u8; 3],
    pub payload_len: __be16,
    pub nexthdr: __u8,
    pub hop_limit: __u8,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}

#[repr(C)]
pub struct udphdr {
    pub source: __be16,
    pub dest: __be16,
    pub len: __be16,
    pub check: __be16,
}

#[repr(C)]
pub struct tlv_hdr {
    pub type_: __be16,
}

#[repr(C)]
pub struct in6_addr {
    pub s6_addr32: [__be32; 4],
}

#[repr(C)]
pub struct xdp_cpumap_stats {
    pub redirect: u32,
    pub pass: u32,
    pub drop: u32,
}

// Original BPF map declarations:
// struct { __uint(type, BPF_MAP_TYPE_ARRAY); __type(key, __u32);
//          __type(value, __u32); __uint(max_entries, 1); } stats SEC(".maps");
// struct { __uint(type, BPF_MAP_TYPE_ARRAY); __type(key, __u32);
//          __type(value, __u32); __uint(max_entries, 1); } dut_stats SEC(".maps");
// struct { __uint(type, BPF_MAP_TYPE_CPUMAP); __uint(key_size, sizeof(__u32));
//          __uint(value_size, sizeof(struct bpf_cpumap_val));
//          __uint(max_entries, 1); } cpu_map SEC(".maps");
// struct { __uint(type, BPF_MAP_TYPE_DEVMAP); __uint(key_size, sizeof(__u32));
//          __uint(value_size, sizeof(struct bpf_devmap_val));
//          __uint(max_entries, 1); } dev_map SEC(".maps");

#[no_mangle]
pub static mut tester_addr: in6_addr = in6_addr { s6_addr32: [0; 4] };

#[no_mangle]
pub static mut dut_addr: in6_addr = in6_addr { s6_addr32: [0; 4] };

#[inline(always)]
fn bpf_htons(x: __u16) -> __be16 {
    x.to_be()
}

#[inline(always)]
unsafe fn ipv6_addr_equal(a: in6_addr, b: in6_addr) -> bool {
    a.s6_addr32[0] == b.s6_addr32[0]
        && a.s6_addr32[1] == b.s6_addr32[1]
        && a.s6_addr32[2] == b.s6_addr32[2]
        && a.s6_addr32[3] == b.s6_addr32[3]
}

#[inline(always)]
unsafe fn xdp_process_echo_packet(xdp: *mut xdp_md, dut: bool) -> i32 {
    let data_end = (*xdp).data_end as usize as *mut c_void;
    let data = (*xdp).data as usize as *mut c_void;
    let eh = data as *mut ethhdr;
    let tlv: *mut tlv_hdr;
    let uh: *mut udphdr;
    let port: __be16;

    if eh.add(1) > data_end as *mut ethhdr {
        return -EINVAL;
    }

    if (*eh).h_proto == bpf_htons(ETH_P_IP) {
        let ih = eh.add(1) as *mut iphdr;
        let saddr: __be32 = if dut {
            tester_addr.s6_addr32[3]
        } else {
            dut_addr.s6_addr32[3]
        };
        let daddr: __be32 = if dut {
            dut_addr.s6_addr32[3]
        } else {
            tester_addr.s6_addr32[3]
        };

        if ih.add(1) > data_end as *mut iphdr {
            return -EINVAL;
        }

        if saddr != (*ih).saddr {
            return -EINVAL;
        }

        if daddr != (*ih).daddr {
            return -EINVAL;
        }

        if (*ih).protocol != IPPROTO_UDP {
            return -EINVAL;
        }

        uh = ih.add(1) as *mut udphdr;
    } else if (*eh).h_proto == bpf_htons(ETH_P_IPV6) {
        let saddr: in6_addr = if dut { tester_addr } else { dut_addr };
        let daddr: in6_addr = if dut { dut_addr } else { tester_addr };
        let ih6 = eh.add(1) as *mut ipv6hdr;

        if ih6.add(1) > data_end as *mut ipv6hdr {
            return -EINVAL;
        }

        if !ipv6_addr_equal(saddr, (*ih6).saddr) {
            return -EINVAL;
        }

        if !ipv6_addr_equal(daddr, (*ih6).daddr) {
            return -EINVAL;
        }

        if (*ih6).nexthdr != IPPROTO_UDP {
            return -EINVAL;
        }

        uh = ih6.add(1) as *mut udphdr;
    } else {
        return -EINVAL;
    }

    if uh.add(1) > data_end as *mut udphdr {
        return -EINVAL;
    }

    port = if dut { (*uh).dest } else { (*uh).source };
    if port != bpf_htons(DUT_ECHO_PORT) {
        return -EINVAL;
    }

    tlv = uh.add(1) as *mut tlv_hdr;
    if tlv.add(1) as *mut c_void > data_end {
        return -EINVAL;
    }

    if bpf_htons((*tlv).type_) == CMD_ECHO {
        0
    } else {
        -EINVAL
    }
}

#[inline(always)]
unsafe fn xdp_update_stats(xdp: *mut xdp_md, tx: bool, dut: bool) -> i32 {
    let mut key: __u32 = 0;
    let val: *mut __u32;

    if xdp_process_echo_packet(xdp, tx) != 0 {
        return -EINVAL;
    }

    if dut {
        val = bpf_map_lookup_elem(&dut_stats as *const _ as *const c_void, &mut key as *mut _ as *const c_void);
    } else {
        val = bpf_map_lookup_elem(&stats as *const _ as *const c_void, &mut key as *mut _ as *const c_void);
    }

    if !val.is_null() {
        (*val) = (*val).wrapping_add(1);
    }

    0
}

/* Tester */

#[no_mangle]
pub unsafe extern "C" fn xdp_tester_check_tx(xdp: *mut xdp_md) -> i32 {
    xdp_update_stats(xdp, true, false);

    XDP_PASS
}

#[no_mangle]
pub unsafe extern "C" fn xdp_tester_check_rx(xdp: *mut xdp_md) -> i32 {
    xdp_update_stats(xdp, false, false);

    XDP_PASS
}

/* DUT */

#[no_mangle]
pub unsafe extern "C" fn xdp_do_pass(xdp: *mut xdp_md) -> i32 {
    xdp_update_stats(xdp, true, true);

    XDP_PASS
}

#[no_mangle]
pub unsafe extern "C" fn xdp_do_drop(xdp: *mut xdp_md) -> i32 {
    if xdp_update_stats(xdp, true, true) != 0 {
        return XDP_PASS;
    }

    XDP_DROP
}

#[no_mangle]
pub unsafe extern "C" fn xdp_do_aborted(xdp: *mut xdp_md) -> i32 {
    if xdp_process_echo_packet(xdp, true) != 0 {
        return XDP_PASS;
    }

    XDP_ABORTED
}

#[no_mangle]
pub unsafe extern "C" fn xdp_do_tx(xdp: *mut xdp_md) -> i32 {
    let data = (*xdp).data as usize as *mut c_void;
    let eh = data as *mut ethhdr;
    let mut tmp_mac: [__u8; ETH_ALEN] = [0; ETH_ALEN];

    if xdp_update_stats(xdp, true, true) != 0 {
        return XDP_PASS;
    }

    core::ptr::copy_nonoverlapping((*eh).h_source.as_ptr(), tmp_mac.as_mut_ptr(), ETH_ALEN);
    core::ptr::copy_nonoverlapping((*eh).h_dest.as_ptr(), (*eh).h_source.as_mut_ptr(), ETH_ALEN);
    core::ptr::copy_nonoverlapping(tmp_mac.as_ptr(), (*eh).h_dest.as_mut_ptr(), ETH_ALEN);

    XDP_TX
}

#[no_mangle]
pub unsafe extern "C" fn xdp_do_redirect(xdp: *mut xdp_md) -> i32 {
    if xdp_process_echo_packet(xdp, true) != 0 {
        return XDP_PASS;
    }

    bpf_redirect_map(&cpu_map as *const _ as *const c_void, 0, 0)
}

#[no_mangle]
pub unsafe extern "C" fn xdp_exception(
    _dev: *const net_device,
    _xdp: *const bpf_prog,
    _act: __u32,
) -> i32 {
    let mut key: __u32 = 0;
    let val: *mut __u32;

    val = bpf_map_lookup_elem(&dut_stats as *const _ as *const c_void, &mut key as *mut _ as *const c_void);
    if !val.is_null() {
        (*val) = (*val).wrapping_add(1);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn tp_xdp_cpumap_kthread(
    _map_id: i32,
    _processed: u32,
    _drops: u32,
    _sched: i32,
    _xdp_stats: *mut xdp_cpumap_stats,
) -> i32 {
    let mut key: __u32 = 0;
    let val: *mut __u32;

    val = bpf_map_lookup_elem(&dut_stats as *const _ as *const c_void, &mut key as *mut _ as *const c_void);
    if !val.is_null() {
        (*val) = (*val).wrapping_add(1);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn xdp_do_redirect_cpumap(xdp: *mut xdp_md) -> i32 {
    let data = (*xdp).data as usize as *mut c_void;
    let eh = data as *mut ethhdr;
    let mut tmp_mac: [__u8; ETH_ALEN] = [0; ETH_ALEN];

    if xdp_process_echo_packet(xdp, true) != 0 {
        return XDP_PASS;
    }

    core::ptr::copy_nonoverlapping((*eh).h_source.as_ptr(), tmp_mac.as_mut_ptr(), ETH_ALEN);
    core::ptr::copy_nonoverlapping((*eh).h_dest.as_ptr(), (*eh).h_source.as_mut_ptr(), ETH_ALEN);
    core::ptr::copy_nonoverlapping(tmp_mac.as_ptr(), (*eh).h_dest.as_mut_ptr(), ETH_ALEN);

    bpf_redirect_map(&dev_map as *const _ as *const c_void, 0, 0)
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
