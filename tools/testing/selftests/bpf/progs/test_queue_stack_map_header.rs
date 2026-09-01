/* SPDX-License-Identifier: GPL-2.0 */
// Copyright (c) 2018 Politecnico di Torino
// Translated from C header test_queue_stack_map.h.
// Original includes: <stddef.h>, <string.h>, <linux/bpf.h>,
// <linux/if_ether.h>, <linux/ip.h>, <linux/pkt_cls.h>, <bpf/bpf_helpers.h>.

#[repr(C)]
pub struct MapDef {
    pub type_: u32,
    pub max_entries: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

unsafe extern "C" {
    static MAP_TYPE: u32;

    fn bpf_map_pop_elem(map: *mut MapDef, value: *mut u32) -> i32;
    fn bpf_map_push_elem(map: *mut MapDef, value: *const u32, flags: u64) -> i32;
}

// SEC(".maps")
#[unsafe(no_mangle)]
pub static mut map_in: MapDef = MapDef {
    type_: unsafe { MAP_TYPE },
    max_entries: 32,
    map_flags: 0,
    key_size: 0,
    value_size: core::mem::size_of::<u32>() as u32,
};

// SEC(".maps")
#[unsafe(no_mangle)]
pub static mut map_out: MapDef = MapDef {
    type_: unsafe { MAP_TYPE },
    max_entries: 32,
    map_flags: 0,
    key_size: 0,
    value_size: core::mem::size_of::<u32>() as u32,
};

// External dependency from <linux/bpf.h>.
#[repr(C)]
pub struct __sk_buff {
    pub data: u32,
    pub data_end: u32,
}

// External dependency from <linux/if_ether.h>.
#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: u16,
}

// External dependency from <linux/ip.h>.
#[repr(C)]
pub struct iphdr {
    pub ihl_version: u8,
    pub tos: u8,
    pub tot_len: u16,
    pub id: u16,
    pub frag_off: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub check: u16,
    pub saddr: u32,
    pub daddr: u32,
}

// External dependencies from <linux/pkt_cls.h>.
pub const TC_ACT_OK: i32 = 0;
pub const TC_ACT_SHOT: i32 = 2;

// SEC("tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _test(skb: *mut __sk_buff) -> i32 {
    let data_end: *mut core::ffi::c_void = (*skb).data_end as usize as *mut core::ffi::c_void;
    let data: *mut core::ffi::c_void = (*skb).data as usize as *mut core::ffi::c_void;
    let eth: *mut ethhdr = data as *mut ethhdr;
    let mut value: u32 = 0;
    let mut err: i32;

    if eth.add(1) as *mut core::ffi::c_void > data_end {
        return TC_ACT_SHOT;
    }

    let iph: *mut iphdr = eth.add(1) as *mut iphdr;

    if iph.add(1) as *mut core::ffi::c_void > data_end {
        return TC_ACT_SHOT;
    }

    err = bpf_map_pop_elem(core::ptr::addr_of_mut!(map_in), &mut value);
    if err != 0 {
        return TC_ACT_SHOT;
    }

    (*iph).daddr = value;

    err = bpf_map_push_elem(core::ptr::addr_of_mut!(map_out), core::ptr::addr_of!((*iph).saddr), 0);
    if err != 0 {
        return TC_ACT_SHOT;
    }

    TC_ACT_OK
}

// SEC("license")
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
