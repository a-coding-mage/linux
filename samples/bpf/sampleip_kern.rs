/* Copyright 2016 Netflix, Inc.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// Linux/BPF headers supplied by the surrounding build.

const MAX_IPS: u32 = 8192;

// The following map metadata corresponds to the C map declaration:
// type: BPF_MAP_TYPE_HASH, key: u64, value: u32,
// max_entries: MAX_IPS, placed in the ".maps" section.
#[repr(C)]
pub struct IpMap {
    pub _map_type: u32,
    pub _key_size: u32,
    pub _value_size: u32,
    pub _max_entries: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut ip_map: IpMap = IpMap {
    _map_type: 1,
    _key_size: core::mem::size_of::<u64>() as u32,
    _value_size: core::mem::size_of::<u32>() as u32,
    _max_entries: MAX_IPS,
};

#[repr(C)]
pub struct BpfPerfEventData {
    pub regs: BpfPtRegs,
}

#[repr(C)]
pub struct BpfPtRegs {
    pub _opaque: [u64; 0],
}

extern "C" {
    fn bpf_map_lookup_elem(map: *const IpMap, key: *const u64) -> *mut u32;
    fn bpf_map_update_elem(
        map: *const IpMap,
        key: *const u64,
        value: *const u32,
        flags: u64,
    ) -> i64;
}

const BPF_NOEXIST: u64 = 1;

// PT_REGS_IP(&ctx->regs), supplied by the BPF tracing headers.
unsafe fn pt_regs_ip(regs: *const BpfPtRegs) -> u64 {
    *(regs as *const u64)
}

#[link_section = "perf_event"]
#[no_mangle]
pub unsafe extern "C" fn do_sample(ctx: *mut BpfPerfEventData) -> i32 {
    let ip: u64;
    let value: *mut u32;
    let init_val: u32 = 1;

    ip = pt_regs_ip(&(*ctx).regs);
    value = bpf_map_lookup_elem(&ip_map, &ip);
    if !value.is_null() {
        *value = (*value).wrapping_add(1);
    } else {
        /* E2BIG not tested for this example only */
        bpf_map_update_elem(&ip_map, &ip, &init_val, BPF_NOEXIST);
    }

    0
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
