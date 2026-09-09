/* Copyright (c) 2015 PLUMgrid, http://plumgrid.com
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// The C source includes vmlinux.h, linux/version.h, bpf_helpers.h, and
// bpf_tracing.h.  Their supplied types, constants, macros, and helpers are
// intentionally referenced here rather than reimplemented.

#[repr(C)]
pub struct pair {
    pub val: u64,
    pub ip: u64,
}

// BPF_MAP_TYPE_HASH, key long, value struct pair, max_entries 1000000.
// The concrete map representation is supplied by the BPF bindings.
#[repr(C)]
pub struct BpfMap {
    _opaque: [u8; 0],
}

#[link_section = ".maps"]
#[used]
pub static mut my_map: BpfMap = BpfMap { _opaque: [] };

// kprobe is NOT a stable ABI. If kernel internals change this bpf+kprobe
// example will no longer be meaningful
#[link_section = "kprobe/kmem_cache_free"]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut pt_regs) -> i32 {
    let ptr: i64 = PT_REGS_PARM2(ctx) as i64;

    bpf_map_delete_elem(
        &raw mut my_map as *mut BpfMap,
        &ptr as *const i64 as *const core::ffi::c_void,
    );
    0
}

#[link_section = "kretprobe/kmem_cache_alloc_node_noprof"]
pub unsafe extern "C" fn bpf_prog2(ctx: *mut pt_regs) -> i32 {
    let ptr: i64 = PT_REGS_RC(ctx) as i64;
    let mut ip: i64 = 0;

    /* get ip address of kmem_cache_alloc_node_noprof() caller */
    BPF_KRETPROBE_READ_RET_IP(&mut ip, ctx);

    let v = pair {
        val: bpf_ktime_get_ns(),
        ip: ip as u64,
    };

    bpf_map_update_elem(
        &raw mut my_map as *mut BpfMap,
        &ptr as *const i64 as *const core::ffi::c_void,
        &v as *const pair as *const core::ffi::c_void,
        BPF_ANY,
    );
    0
}

#[link_section = "license"]
#[used]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[link_section = "version"]
#[used]
pub static mut _version: u32 = LINUX_VERSION_CODE;

// Supplied by vmlinux.h and the BPF helper/tracing headers.
extern "C" {
    pub type pt_regs;

    fn PT_REGS_PARM2(ctx: *mut pt_regs) -> i64;
    fn PT_REGS_RC(ctx: *mut pt_regs) -> i64;
    fn BPF_KRETPROBE_READ_RET_IP(ip: *mut i64, ctx: *mut pt_regs);
    fn bpf_map_delete_elem(map: *mut BpfMap, key: *const core::ffi::c_void) -> i64;
    fn bpf_map_update_elem(
        map: *mut BpfMap,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
    fn bpf_ktime_get_ns() -> u64;
}

// Supplied by bpf_helpers.h.
const BPF_ANY: u64 = 0;
// Supplied by linux/version.h.
const LINUX_VERSION_CODE: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
