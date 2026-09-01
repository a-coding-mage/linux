// SPDX-License-Identifier: GPL-2.0
// KBUILD_MODNAME was defined as "foo" in the C source.
// C dependencies removed from executable Rust:
// <string.h>, <linux/in.h>, <linux/if_ether.h>, <linux/if_packet.h>,
// <linux/ip.h>, <linux/ipv6.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_endian.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __be64 = u64;

pub const BPF_MAP_TYPE_DEVMAP: __u32 = 14;
pub const BPF_MAP_TYPE_DEVMAP_HASH: __u32 = 25;
pub const BPF_MAP_TYPE_HASH: __u32 = 1;
pub const BPF_F_BROADCAST: __u64 = 1 << 3;
pub const BPF_F_EXCLUDE_INGRESS: __u64 = 1 << 4;
pub const ETH_ALEN: usize = 6;
pub const ETH_P_IP: __u16 = 0x0800;
pub const ETH_P_IPV6: __u16 = 0x86DD;
pub const XDP_DROP: i32 = 1;
pub const XDP_PASS: i32 = 2;

#[repr(C)]
pub struct xdp_md {
    pub data: __u32,
    pub data_end: __u32,
    pub data_meta: __u32,
    pub ingress_ifindex: __u32,
    pub rx_queue_index: __u32,
    pub egress_ifindex: __u32,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; ETH_ALEN],
    pub h_source: [u8; ETH_ALEN],
    pub h_proto: __u16,
}

#[repr(C)]
pub struct bpf_devmap_val {
    pub ifindex: __u32,
    pub bpf_prog: bpf_devmap_prog,
}

#[repr(C)]
pub struct bpf_devmap_prog {
    pub fd: __u32,
    pub id: __u32,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
}

unsafe extern "C" {
    pub fn bpf_map_lookup_elem(map: *const bpf_map_def, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    pub fn bpf_redirect_map(map: *const bpf_map_def, key: __u64, flags: __u64) -> i32;
}

#[inline(always)]
pub const fn bpf_htons(x: __u16) -> __u16 {
    x.to_be()
}

/* One map use devmap, another one use devmap_hash for testing */
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static map_all: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_DEVMAP,
    key_size: core::mem::size_of::<i32>() as __u32,
    value_size: core::mem::size_of::<i32>() as __u32,
    max_entries: 1024,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static map_egress: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_DEVMAP_HASH,
    key_size: core::mem::size_of::<i32>() as __u32,
    value_size: core::mem::size_of::<bpf_devmap_val>() as __u32,
    max_entries: 128,
};

/* map to store egress interfaces mac addresses */
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mac_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__be64>() as __u32,
    max_entries: 128,
};

/* map to store redirect flags for each protocol*/
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static redirect_flags: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<__u16>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
    max_entries: 16,
};

#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_redirect_map_multi_prog(ctx: *mut xdp_md) -> i32 {
    let data_end: *mut core::ffi::c_void = (*ctx).data_end as usize as *mut core::ffi::c_void;
    let data: *mut core::ffi::c_void = (*ctx).data as usize as *mut core::ffi::c_void;
    let if_index: i32 = (*ctx).ingress_ifindex as i32;
    let eth: *mut ethhdr = data as *mut ethhdr;
    let flags_from_map: *mut __u64;
    let h_proto: __u16;
    let nh_off: __u64;
    let flags: __u64;

    nh_off = core::mem::size_of_val(&*eth) as __u64;
    if (data as usize).wrapping_add(nh_off as usize) > data_end as usize {
        return XDP_DROP;
    }

    h_proto = bpf_htons((*eth).h_proto);

    flags_from_map = bpf_map_lookup_elem(
        &redirect_flags,
        &h_proto as *const __u16 as *const core::ffi::c_void,
    ) as *mut __u64;

    /* Default flags for IPv4 : (BPF_F_BROADCAST | BPF_F_EXCLUDE_INGRESS) */
    if h_proto == ETH_P_IP {
        flags = if !flags_from_map.is_null() {
            *flags_from_map
        } else {
            BPF_F_BROADCAST | BPF_F_EXCLUDE_INGRESS
        };
        return bpf_redirect_map(&map_all, 0, flags);
    }
    /* Default flags for IPv6 : 0 */
    if h_proto == ETH_P_IPV6 {
        flags = if !flags_from_map.is_null() {
            *flags_from_map
        } else {
            0
        };
        return bpf_redirect_map(&map_all, if_index as __u64, flags);
    }
    /* Default flags for others BPF_F_BROADCAST : 0 */
    else {
        flags = if !flags_from_map.is_null() {
            *flags_from_map
        } else {
            BPF_F_BROADCAST
        };
        return bpf_redirect_map(&map_all, 0, flags);
    }
}

/* The following 2 progs are for 2nd devmap prog testing */
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_redirect_map_all_prog(_ctx: *mut xdp_md) -> i32 {
    bpf_redirect_map(
        &map_egress,
        0,
        BPF_F_BROADCAST | BPF_F_EXCLUDE_INGRESS,
    )
}

#[unsafe(link_section = "xdp/devmap")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_devmap_prog(ctx: *mut xdp_md) -> i32 {
    let data_end: *mut core::ffi::c_void = (*ctx).data_end as usize as *mut core::ffi::c_void;
    let data: *mut core::ffi::c_void = (*ctx).data as usize as *mut core::ffi::c_void;
    let key: __u32 = (*ctx).egress_ifindex;
    let eth: *mut ethhdr = data as *mut ethhdr;
    let nh_off: __u64;
    let mac: *mut __be64;

    nh_off = core::mem::size_of_val(&*eth) as __u64;
    if (data as usize).wrapping_add(nh_off as usize) > data_end as usize {
        return XDP_DROP;
    }

    mac = bpf_map_lookup_elem(
        &mac_map,
        &key as *const __u32 as *const core::ffi::c_void,
    ) as *mut __be64;
    if !mac.is_null() {
        core::ptr::copy_nonoverlapping(mac as *const u8, (*eth).h_source.as_mut_ptr(), ETH_ALEN);
    }

    XDP_PASS
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
