// SPDX-License-Identifier: GPL-2.0
/*  GPLv2, Copyright(c) 2017 Jesper Dangaard Brouer, Red Hat, Inc. */
// Dependencies supplied by xdp_sample.bpf.h and the BPF helper headers remain external.

#[repr(C)]
pub struct datarec {
    pub processed: u64,
    pub dropped: u64,
    pub issue: u64,
    pub xdp_pass: u64,
    pub xdp_drop: u64,
    pub xdp_redirect: u64,
    pub info: u64,
}

#[repr(C)]
pub struct net_device {
    pub ifindex: i32,
}

#[repr(C)]
pub struct bpf_prog;
#[repr(C)]
pub struct bpf_map;
#[repr(C)]
pub struct xdp_cpumap_stats {
    pub pass: u64,
    pub drop: u64,
    pub redirect: u64,
}

extern "C" {
    static mut rx_cnt: array_map;
    static mut redir_err_cnt: array_map;
    static mut cpumap_enqueue_cnt: array_map;
    static mut cpumap_kthread_cnt: array_map;
    static mut exception_cnt: array_map;
    static mut devmap_xmit_cnt: array_map;
    static mut devmap_xmit_cnt_multi: array_map;
    fn bpf_get_smp_processor_id() -> u32;
    fn bpf_map_lookup_elem(map: *const array_map, key: *const u32) -> *mut datarec;
    fn bpf_map_update_elem(map: *const array_map, key: *const u64, value: *const datarec, flags: u64) -> i64;
}

#[repr(C)]
pub struct array_map;

pub const nr_cpus: i32 = 0;
pub static mut from_match: [i32; 32] = [0; 32];
pub static mut to_match: [i32; 32] = [0; 32];
pub static mut cpumap_map_id: i32 = 0;

const EINVAL: i32 = 22;
const ENETDOWN: i32 = 100;
const EMSGSIZE: i32 = 90;
const EOPNOTSUPP: i32 = 95;
const ENOSPC: i32 = 28;
const XDP_REDIRECT_ERROR: u32 = 1;
const XDP_REDIRECT: u32 = _redirect_value();
const BPF_NOEXIST: u64 = 1;

const fn _redirect_value() -> u32 { 4 }

#[inline(always)]
unsafe fn in_set(a: &[i32; 32], b: i32) -> bool {
    let mut result = a[0] == 0;
    let mut i = 0;
    while i < a.len() && a[i] != 0 {
        result = a[i] == b;
        if result { break; }
        i += 1;
    }
    result
}

#[inline(always)]
unsafe fn xdp_get_err_key(err: i32) -> u32 {
    match err {
        0 => 0,
        -EINVAL => 2,
        -ENETDOWN => 3,
        -EMSGSIZE => 4,
        -EOPNOTSUPP => 5,
        -ENOSPC => 6,
        _ => 1,
    }
}

#[inline(always)]
unsafe fn xdp_redirect_collect_stat(from: i32, err: i32) -> i32 {
    let cpu = bpf_get_smp_processor_id();
    if !in_set(&from_match, from) { return 0; }
    let key = xdp_get_err_key(err);
    let idx = key * nr_cpus as u32 + cpu;
    let rec = bpf_map_lookup_elem(&redir_err_cnt, &idx);
    if rec.is_null() { return 0; }
    if key != 0 { (*rec).dropped = (*rec).dropped.wrapping_add(1); }
    else { (*rec).processed = (*rec).processed.wrapping_add(1); }
    0 /* Indicate event was filtered (no further processing) */
}

// SEC("tp_btf/xdp_redirect_err")
pub unsafe fn tp_xdp_redirect_err(dev: *const net_device, _xdp: *const bpf_prog, _tgt: *const core::ffi::c_void, err: i32, _map: *const bpf_map, _index: u32) -> i32 { xdp_redirect_collect_stat((*dev).ifindex, err) }
// SEC("tp_btf/xdp_redirect_map_err")
pub unsafe fn tp_xdp_redirect_map_err(dev: *const net_device, _xdp: *const bpf_prog, _tgt: *const core::ffi::c_void, err: i32, _map: *const bpf_map, _index: u32) -> i32 { xdp_redirect_collect_stat((*dev).ifindex, err) }
// SEC("tp_btf/xdp_redirect")
pub unsafe fn tp_xdp_redirect(dev: *const net_device, _xdp: *const bpf_prog, _tgt: *const core::ffi::c_void, err: i32, _map: *const bpf_map, _index: u32) -> i32 { xdp_redirect_collect_stat((*dev).ifindex, err) }
// SEC("tp_btf/xdp_redirect_map")
pub unsafe fn tp_xdp_redirect_map(dev: *const net_device, _xdp: *const bpf_prog, _tgt: *const core::ffi::c_void, err: i32, _map: *const bpf_map, _index: u32) -> i32 { xdp_redirect_collect_stat((*dev).ifindex, err) }

// The remaining BPF_PROG tracepoint declarations are preserved as Rust entry points.
pub unsafe fn tp_xdp_cpumap_enqueue(map_id: i32, processed: u32, drops: u32, to_cpu: i32) -> i32 {
    if cpumap_map_id != 0 && cpumap_map_id != map_id { return 0; }
    let cpu = bpf_get_smp_processor_id(); let idx = to_cpu as u32 * nr_cpus as u32 + cpu;
    let rec = bpf_map_lookup_elem(&cpumap_enqueue_cnt, &idx); if rec.is_null() { return 0; }
    (*rec).processed = (*rec).processed.wrapping_add(processed as u64); (*rec).dropped = (*rec).dropped.wrapping_add(drops as u64);
    if processed > 0 { (*rec).issue = (*rec).issue.wrapping_add(1); } 0
}
pub unsafe fn tp_xdp_cpumap_kthread(map_id: i32, processed: u32, drops: u32, sched: i32, xdp_stats: *mut xdp_cpumap_stats) -> i32 {
    if cpumap_map_id != 0 && cpumap_map_id != map_id { return 0; }
    let cpu = bpf_get_smp_processor_id(); let rec = bpf_map_lookup_elem(&cpumap_kthread_cnt, &cpu); if rec.is_null() { return 0; }
    (*rec).processed = (*rec).processed.wrapping_add(processed as u64); (*rec).dropped = (*rec).dropped.wrapping_add(drops as u64);
    (*rec).xdp_pass = (*rec).xdp_pass.wrapping_add((*xdp_stats).pass); (*rec).xdp_drop = (*rec).xdp_drop.wrapping_add((*xdp_stats).drop); (*rec).xdp_redirect = (*rec).xdp_redirect.wrapping_add((*xdp_stats).redirect);
    if sched != 0 { (*rec).issue = (*rec).issue.wrapping_add(1); } 0
}
pub unsafe fn tp_xdp_exception(dev: *const net_device, _xdp: *const bpf_prog, act: u32) -> i32 {
    if !in_set(&from_match, (*dev).ifindex) || !in_set(&to_match, (*dev).ifindex) { return 0; }
    let key = if act > XDP_REDIRECT { XDP_REDIRECT + 1 } else { act }; let cpu = bpf_get_smp_processor_id(); let idx = key * nr_cpus as u32 + cpu;
    let rec = bpf_map_lookup_elem(&exception_cnt, &idx); if !rec.is_null() { (*rec).dropped = (*rec).dropped.wrapping_add(1); } 0
}
pub unsafe fn tp_xdp_devmap_xmit(from_dev: *const net_device, to_dev: *const net_device, sent: i32, drops: i32, err: i32) -> i32 {
    let a = (*from_dev).ifindex; let b = (*to_dev).ifindex; if !in_set(&from_match, a) || !in_set(&to_match, b) { return 0; }
    let cpu = bpf_get_smp_processor_id(); let rec = bpf_map_lookup_elem(&devmap_xmit_cnt, &cpu); if rec.is_null() { return 0; }
    (*rec).processed = (*rec).processed.wrapping_add(sent as u64); (*rec).dropped = (*rec).dropped.wrapping_add(drops as u64); (*rec).info = (*rec).info.wrapping_add(1); if err != 0 || drops < 0 { (*rec).issue = (*rec).issue.wrapping_add(1); } 0
}
pub unsafe fn tp_xdp_devmap_xmit_multi(from_dev: *const net_device, to_dev: *const net_device, sent: i32, drops: i32, err: i32) -> i32 {
    let a = (*from_dev).ifindex; let b = (*to_dev).ifindex; if !in_set(&from_match, a) || !in_set(&to_match, b) { return 0; }
    let idx = ((a as u64) << 32) | b as u32 as u64; let empty = datarec { processed: 0, dropped: 0, issue: 0, xdp_pass: 0, xdp_drop: 0, xdp_redirect: 0, info: 0 };
    bpf_map_update_elem(&devmap_xmit_cnt_multi, &idx, &empty, BPF_NOEXIST); let rec = bpf_map_lookup_elem(&devmap_xmit_cnt_multi, &(idx as u32)); if rec.is_null() { return 0; }
    (*rec).processed = (*rec).processed.wrapping_add(sent as u64); (*rec).dropped = (*rec).dropped.wrapping_add(drops as u64); (*rec).info = (*rec).info.wrapping_add(1); if err != 0 || drops < 0 { (*rec).issue = (*rec).issue.wrapping_add(1); } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
