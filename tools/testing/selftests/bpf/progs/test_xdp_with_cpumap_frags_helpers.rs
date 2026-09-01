// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

pub const IFINDEX_LO: u32 = 1;

#[repr(C)]
pub struct CpuMapDef {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

extern "C" {
    pub type xdp_md;
    pub type bpf_cpumap_val;
}

pub const BPF_MAP_TYPE_CPUMAP: u32 = 16;
pub const XDP_PASS: i32 = 2;

#[link_section = ".maps"]
#[no_mangle]
pub static cpu_map: CpuMapDef = CpuMapDef {
    type_: BPF_MAP_TYPE_CPUMAP,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<bpf_cpumap_val>() as u32,
    max_entries: 4,
};

// SEC("xdp/cpumap")
#[no_mangle]
pub unsafe extern "C" fn xdp_dummy_cm(ctx: *mut xdp_md) -> i32 {
    let _ = ctx;
    XDP_PASS
}

// SEC("xdp.frags/cpumap")
#[no_mangle]
pub unsafe extern "C" fn xdp_dummy_cm_frags(ctx: *mut xdp_md) -> i32 {
    let _ = ctx;
    XDP_PASS
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
