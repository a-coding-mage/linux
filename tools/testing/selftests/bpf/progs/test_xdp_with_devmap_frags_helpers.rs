// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>

#[repr(C)]
pub struct xdp_md {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_devmap_val {
    _unused: [u8; 0],
}

pub const BPF_MAP_TYPE_DEVMAP: u32 = 14;
pub const XDP_PASS: i32 = 2;

#[repr(C)]
pub struct dm_ports_def {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static dm_ports: dm_ports_def = dm_ports_def {
    type_: BPF_MAP_TYPE_DEVMAP,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<bpf_devmap_val>() as u32,
    max_entries: 4,
};

/* valid program on DEVMAP entry via SEC name;
 * has access to egress and ingress ifindex
 */
#[unsafe(link_section = "xdp/devmap")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_dummy_dm(ctx: *mut xdp_md) -> i32 {
    let _ = ctx;
    XDP_PASS
}

#[unsafe(link_section = "xdp.frags/devmap")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_dummy_dm_frags(ctx: *mut xdp_md) -> i32 {
    let _ = ctx;
    XDP_PASS
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
