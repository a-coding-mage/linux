// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u32 = u32;

extern "C" {
    static BPF_MAP_TYPE_DEVMAP: u32;
    static XDP_PASS: i32;
}

#[repr(C)]
pub struct xdp_md {
    pub data: __u32,
    pub data_end: __u32,
    pub ingress_ifindex: __u32,
    pub egress_ifindex: __u32,
}

#[repr(C)]
pub struct bpf_devmap_val {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_redirect_map(map: *const dm_ports_map, key: u64, flags: u64) -> i32;
    fn bpf_trace_printk(fmt: *const core::ffi::c_char, fmt_size: i32, ...) -> i32;
}

#[repr(C)]
pub struct dm_ports_map {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

// SEC(".maps")
#[no_mangle]
pub static dm_ports: dm_ports_map = dm_ports_map {
    type_: unsafe { BPF_MAP_TYPE_DEVMAP },
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<bpf_devmap_val>() as u32,
    max_entries: 4,
};

// SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_redir_prog(ctx: *mut xdp_md) -> i32 {
    bpf_redirect_map(&dm_ports, 0, 0)
}

/* invalid program on DEVMAP entry;
 * SEC name means expected attach type not set
 */
// SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_dummy_prog(ctx: *mut xdp_md) -> i32 {
    XDP_PASS
}

/* valid program on DEVMAP entry via SEC name;
 * has access to egress and ingress ifindex
 */
// SEC("xdp/devmap")
#[no_mangle]
pub unsafe extern "C" fn xdp_dummy_dm(ctx: *mut xdp_md) -> i32 {
    let fmt = b"devmap redirect: dev %u -> dev %u len %u\n\0";
    let data_end = (*ctx).data_end as usize as *mut core::ffi::c_void;
    let data = (*ctx).data as usize as *mut core::ffi::c_void;
    let len = data_end.offset_from(data) as u32;

    bpf_trace_printk(
        fmt.as_ptr() as *const core::ffi::c_char,
        core::mem::size_of_val(fmt) as i32,
        (*ctx).ingress_ifindex,
        (*ctx).egress_ifindex,
        len,
    );

    XDP_PASS
}

// SEC("xdp.frags/devmap")
#[no_mangle]
pub unsafe extern "C" fn xdp_dummy_dm_frags(ctx: *mut xdp_md) -> i32 {
    XDP_PASS
}

// SEC("license")
#[no_mangle]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
