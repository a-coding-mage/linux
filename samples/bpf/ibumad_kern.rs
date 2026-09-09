// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB

/*
 * ibumad BPF sample kernel side
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 *
 * Copyright(c) 2018 Ira Weiny, Intel Corporation
 */

// KBUILD_MODNAME = "ibumad_count_pkts_by_class"
// Dependencies supplied by the kernel/BPF environment:
// uapi/linux/bpf.h and bpf/bpf_helpers.h

use core::ffi::c_void;

// The __uint/__type map declarations are BPF helper macros.  This preserves
// their represented map layout and section placement for the translated item.
#[repr(C)]
pub struct BpfArrayMap {
    pub map_type: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

#[unsafe(link_section = ".maps")]
pub static mut read_count: BpfArrayMap = BpfArrayMap {
    map_type: BPF_MAP_TYPE_ARRAY,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u64>() as u32,
    max_entries: 256,
};

#[unsafe(link_section = ".maps")]
pub static mut write_count: BpfArrayMap = BpfArrayMap {
    map_type: BPF_MAP_TYPE_ARRAY,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u64>() as u32,
    max_entries: 256,
};

// Supplied by <uapi/linux/bpf.h>.
extern "C" {
    static BPF_MAP_TYPE_ARRAY: u32;
    static BPF_NOEXIST: u64;
}

extern "C" {
    fn bpf_map_lookup_elem(map: *const c_void, key: *const c_void) -> *mut u64;
    fn bpf_map_update_elem(
        map: *const c_void,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> i64;
}

// DEBUG is undefined in the source, so bpf_printk expands to nothing.

#[repr(C)]
pub struct ib_umad_rw_args {
    pub pad: u64,
    pub port_num: u8,
    pub sl: u8,
    pub path_bits: u8,
    pub grh_present: u8,
    pub id: u32,
    pub status: u32,
    pub timeout_ms: u32,
    pub retires: u32,
    pub length: u32,
    pub qpn: u32,
    pub qkey: u32,
    pub gid_index: u8,
    pub hop_limit: u8,
    pub lid: u16,
    pub attr_id: u16,
    pub pkey_index: u16,
    pub base_version: u8,
    pub mgmt_class: u8,
    pub class_version: u8,
    pub method: u8,
    pub flow_label: u32,
    pub mad_status: u16,
    pub class_specific: u16,
    pub attr_mod: u32,
    pub tid: u64,
    pub gid: [u8; 16],
    pub dev_index: u32,
    pub traffic_class: u8,
}

#[unsafe(link_section = "tracepoint/ib_umad/ib_umad_read_recv")]
pub unsafe extern "C" fn on_ib_umad_read_recv(ctx: *mut ib_umad_rw_args) -> i32 {
    let zero: u64 = 0;
    let class: u8 = (*ctx).mgmt_class;
    let mut val: *mut u64;

    val = bpf_map_lookup_elem(&raw const read_count as *const _ as *const c_void, &class as *const _ as *const c_void);
    if val.is_null() {
        bpf_map_update_elem(&raw const read_count as *const _ as *const c_void, &class as *const _ as *const c_void, &zero as *const _ as *const c_void, BPF_NOEXIST as u64);
        val = bpf_map_lookup_elem(&raw const read_count as *const _ as *const c_void, &class as *const _ as *const c_void);
        if val.is_null() { return 0; }
    }
    *val += 1;
    0
}

#[unsafe(link_section = "tracepoint/ib_umad/ib_umad_read_send")]
pub unsafe extern "C" fn on_ib_umad_read_send(ctx: *mut ib_umad_rw_args) -> i32 {
    let zero: u64 = 0;
    let class: u8 = (*ctx).mgmt_class;
    let mut val = bpf_map_lookup_elem(&raw const read_count as *const _ as *const c_void, &class as *const _ as *const c_void);
    if val.is_null() {
        bpf_map_update_elem(&raw const read_count as *const _ as *const c_void, &class as *const _ as *const c_void, &zero as *const _ as *const c_void, BPF_NOEXIST as u64);
        val = bpf_map_lookup_elem(&raw const read_count as *const _ as *const c_void, &class as *const _ as *const c_void);
        if val.is_null() { return 0; }
    }
    *val += 1;
    0
}

#[unsafe(link_section = "tracepoint/ib_umad/ib_umad_write")]
pub unsafe extern "C" fn on_ib_umad_write(ctx: *mut ib_umad_rw_args) -> i32 {
    let zero: u64 = 0;
    let class: u8 = (*ctx).mgmt_class;
    let mut val = bpf_map_lookup_elem(&raw const write_count as *const _ as *const c_void, &class as *const _ as *const c_void);
    if val.is_null() {
        bpf_map_update_elem(&raw const write_count as *const _ as *const c_void, &class as *const _ as *const c_void, &zero as *const _ as *const c_void, BPF_NOEXIST as u64);
        val = bpf_map_lookup_elem(&raw const write_count as *const _ as *const c_void, &class as *const _ as *const c_void);
        if val.is_null() { return 0; }
    }
    *val += 1;
    0
}

#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
