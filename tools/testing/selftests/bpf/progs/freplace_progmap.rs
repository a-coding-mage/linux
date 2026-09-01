// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

#[repr(C)]
pub struct bpf_cpumap_val {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct xdp_md {
    _unused: [u8; 0],
}

pub const BPF_MAP_TYPE_CPUMAP: u32 = 16;
pub const XDP_PASS: u32 = 2;
pub const XDP_DROP: i32 = 1;

#[repr(C)]
pub struct cpu_map_def {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

unsafe extern "C" {
    pub fn bpf_redirect_map(map: *const cpu_map_def, key: u64, flags: u64) -> i32;
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static cpu_map: cpu_map_def = cpu_map_def {
    type_: BPF_MAP_TYPE_CPUMAP,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<bpf_cpumap_val>() as u32,
    max_entries: 1,
};

#[unsafe(link_section = "xdp/cpumap")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_drop_prog(ctx: *mut xdp_md) -> i32 {
    let _ = ctx;
    XDP_DROP
}

#[unsafe(link_section = "freplace")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_cpumap_prog(ctx: *mut xdp_md) -> i32 {
    let _ = ctx;
    unsafe { bpf_redirect_map(&cpu_map as *const cpu_map_def, 0, XDP_PASS as u64) }
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
