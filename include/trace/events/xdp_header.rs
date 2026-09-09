/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/xdp.h.  Kernel tracepoint registration and
// formatting are represented by the corresponding C ABI declarations below.

use core::ffi::{c_char, c_int, c_void};

pub type __u32 = u32;

#[repr(C)]
pub struct net_device {
    pub ifindex: c_int,
}

#[repr(C)]
pub struct bpf_prog {
    pub aux: *const bpf_prog_aux,
}

#[repr(C)]
pub struct bpf_prog_aux {
    pub id: c_int,
}

#[repr(C)]
pub struct bpf_map;
#[repr(C)]
pub struct xdp_cpumap_stats {
    pub pass: u32,
    pub drop: u32,
    pub redirect: u32,
}
#[repr(C)]
pub struct xdp_mem_allocator {
    pub mem: xdp_mem_info,
    pub allocator: *const c_void,
}
#[repr(C)]
pub struct xdp_mem_info {
    pub id: u32,
    pub type_: u32,
}
#[repr(C)]
pub struct xdp_rxq_info {
    pub dev: *const net_device,
}

pub const XDP_ABORTED: u32 = 0;
pub const XDP_DROP: u32 = 1;
pub const XDP_PASS: u32 = 2;
pub const XDP_TX: u32 = 3;
pub const XDP_REDIRECT: u32 = 4;

pub const BPF_MAP_TYPE_UNSPEC: c_int = 0;
pub const BPF_MAP_TYPE_DEVMAP: c_int = 14;
pub const BPF_MAP_TYPE_DEVMAP_HASH: c_int = 25;
pub const INT_MAX: c_int = c_int::MAX;

pub const MEM_TYPE_PAGE_SHARED: u32 = 0;
pub const MEM_TYPE_PAGE_ORDER0: u32 = 1;
pub const MEM_TYPE_PAGE_POOL: u32 = 2;
pub const MEM_TYPE_XSK_BUFF_POOL: u32 = 3;

#[repr(C)]
pub struct _bpf_dtab_netdev {
    pub dev: *mut net_device,
}

extern "C" {
    pub fn trace_xdp_exception(dev: *const net_device, xdp: *const bpf_prog, act: u32);
    pub fn trace_xdp_bulk_tx(dev: *const net_device, sent: c_int, drops: c_int, err: c_int);
    pub fn trace_xdp_redirect(
        dev: *const net_device, xdp: *const bpf_prog, tgt: *const c_void,
        err: c_int, map_type: c_int, map_id: u32, index: u32,
    );
    pub fn trace_xdp_redirect_err(
        dev: *const net_device, xdp: *const bpf_prog, tgt: *const c_void,
        err: c_int, map_type: c_int, map_id: u32, index: u32,
    );
    pub fn trace_xdp_cpumap_kthread(
        map_id: c_int, processed: u32, drops: u32, sched: c_int,
        xdp_stats: *mut xdp_cpumap_stats,
    );
    pub fn trace_xdp_cpumap_enqueue(map_id: c_int, processed: u32, drops: u32, to_cpu: c_int);
    pub fn trace_xdp_devmap_xmit(
        from_dev: *const net_device, to_dev: *const net_device,
        sent: c_int, drops: c_int, err: c_int,
    );
    pub fn trace_mem_disconnect(xa: *const xdp_mem_allocator);
    pub fn trace_mem_connect(xa: *const xdp_mem_allocator, rxq: *const xdp_rxq_info);
    pub fn trace_bpf_xdp_link_attach_failed(msg: *const c_char);
}

#[inline]
pub unsafe fn _trace_xdp_redirect(dev: *const net_device, xdp: *const bpf_prog, to: u32) {
    trace_xdp_redirect(dev, xdp, core::ptr::null(), 0, BPF_MAP_TYPE_UNSPEC, INT_MAX as u32, to);
}

#[inline]
pub unsafe fn _trace_xdp_redirect_err(
    dev: *const net_device, xdp: *const bpf_prog, to: u32, err: c_int,
) {
    trace_xdp_redirect_err(dev, xdp, core::ptr::null(), err, BPF_MAP_TYPE_UNSPEC, INT_MAX as u32, to);
}

#[inline]
pub unsafe fn _trace_xdp_redirect_map(
    dev: *const net_device, xdp: *const bpf_prog, to: *const c_void,
    map_type: c_int, map_id: u32, index: u32,
) {
    trace_xdp_redirect(dev, xdp, to, 0, map_type, map_id, index);
}

#[inline]
pub unsafe fn _trace_xdp_redirect_map_err(
    dev: *const net_device, xdp: *const bpf_prog, to: *const c_void,
    map_type: c_int, map_id: u32, index: u32, err: c_int,
) {
    trace_xdp_redirect_err(dev, xdp, to, err, map_type, map_id, index);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
