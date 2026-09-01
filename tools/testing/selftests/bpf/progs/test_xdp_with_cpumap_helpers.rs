// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C includes:
// <linux/bpf.h>
// <bpf/bpf_helpers.h>

pub const IFINDEX_LO: u32 = 1;

pub const BPF_MAP_TYPE_CPUMAP: u32 = 16;
pub const XDP_PASS: i32 = 2;
pub const XDP_DROP: i32 = 1;

#[repr(C)]
pub struct xdp_md {
    pub data: u32,
    pub data_end: u32,
    pub data_meta: u32,
    pub ingress_ifindex: u32,
    pub rx_queue_index: u32,
    pub egress_ifindex: u32,
}

#[repr(C)]
pub struct bpf_cpumap_val {
    pub qsize: u32,
    pub bpf_prog: bpf_cpumap_val__bindgen_ty_1,
}

#[repr(C)]
pub union bpf_cpumap_val__bindgen_ty_1 {
    pub fd: i32,
    pub id: u32,
}

#[repr(C)]
pub struct cpu_map_def {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static cpu_map: cpu_map_def = cpu_map_def {
    type_: BPF_MAP_TYPE_CPUMAP,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<bpf_cpumap_val>() as u32,
    max_entries: 4,
};

#[no_mangle]
pub static mut redirect_count: u32 = 0;

extern "C" {
    pub fn bpf_redirect_map(map: *const cpu_map_def, key: u64, flags: u64) -> i32;
    pub fn bpf_get_smp_processor_id() -> u32;
}

#[link_section = "xdp"]
#[no_mangle]
pub unsafe extern "C" fn xdp_redir_prog(ctx: *mut xdp_md) -> i32 {
    let _ = ctx;
    bpf_redirect_map(&cpu_map as *const cpu_map_def, 0, 0)
}

#[link_section = "xdp"]
#[no_mangle]
pub unsafe extern "C" fn xdp_dummy_prog(ctx: *mut xdp_md) -> i32 {
    let _ = ctx;
    XDP_PASS
}

#[link_section = "xdp/cpumap"]
#[no_mangle]
pub unsafe extern "C" fn xdp_dummy_cm(ctx: *mut xdp_md) -> i32 {
    if bpf_get_smp_processor_id() == 0 {
        redirect_count = redirect_count.wrapping_add(1);
    }

    if (*ctx).ingress_ifindex == IFINDEX_LO {
        return XDP_DROP;
    }

    XDP_PASS
}

#[link_section = "xdp.frags/cpumap"]
#[no_mangle]
pub unsafe extern "C" fn xdp_dummy_cm_frags(ctx: *mut xdp_md) -> i32 {
    let _ = ctx;
    XDP_PASS
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
