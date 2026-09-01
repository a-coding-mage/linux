// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C source:
// - "vmlinux.h"
// - <bpf/bpf_helpers.h>
// - <bpf/bpf_tracing.h>

use core::ffi::{c_int, c_uint, c_void};

type u32 = u32;
type __u32 = u32;

#[repr(C)]
pub struct path {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct kstat {
    _unused: [u8; 0],
}

extern "C" {
    #[link_name = "bpf_prog_active"]
    static bpf_prog_active: c_int;

    fn bpf_get_smp_processor_id() -> u32;
    fn bpf_per_cpu_ptr(ptr: *const c_void, cpu: u32) -> *mut c_void;
    fn bpf_ringbuf_submit(data: *mut c_void, flags: u64);
}

// Original BPF map declaration:
// struct {
//      __uint(type, BPF_MAP_TYPE_RINGBUF);
//      __uint(max_entries, 1 << 12);
// } ringbuf SEC(".maps");
#[repr(C)]
pub struct ringbuf {
    _unused: [u8; 0],
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut ringbuf: ringbuf = ringbuf { _unused: [] };

#[link_section = "fentry/security_inode_getattr"]
#[no_mangle]
pub unsafe extern "C" fn d_path_check_rdonly_mem(
    path: *mut path,
    stat: *mut kstat,
    request_mask: __u32,
    query_flags: c_uint,
) -> c_int {
    let active: *mut c_void;
    let cpu: u32;

    let _ = path;
    let _ = stat;
    let _ = request_mask;
    let _ = query_flags;

    cpu = bpf_get_smp_processor_id();
    active = bpf_per_cpu_ptr(&bpf_prog_active as *const _ as *const c_void, cpu);
    if !active.is_null() {
        /* FAIL here! 'active' points to 'regular' memory. It
         * cannot be submitted to ring buffer.
         */
        bpf_ringbuf_submit(active, 0);
    }
    return 0;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
