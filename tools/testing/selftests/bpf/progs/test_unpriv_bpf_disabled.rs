// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022, Oracle and/or its affiliates. */

/* Translated from C BPF source. Original includes:
 * "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, "bpf_misc.h"
 */

type __u32 = u32;

const BPF_MAP_TYPE_ARRAY: __u32 = 2;
const BPF_MAP_TYPE_PERCPU_ARRAY: __u32 = 6;
const BPF_MAP_TYPE_HASH: __u32 = 1;
const BPF_MAP_TYPE_PERCPU_HASH: __u32 = 5;
const BPF_MAP_TYPE_PERF_EVENT_ARRAY: __u32 = 4;
const BPF_MAP_TYPE_RINGBUF: __u32 = 27;
const BPF_MAP_TYPE_PROG_ARRAY: __u32 = 3;
const BPF_F_CURRENT_CPU: u64 = 0xffffffff;

/* SYS_PREFIX is supplied by bpf_misc.h in the original source. */

#[repr(C)]
pub struct BpfMapArray {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[repr(C)]
pub struct BpfMapNoMaxEntries {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[repr(C)]
pub struct BpfMapRingbuf {
    pub type_: __u32,
    pub max_entries: __u32,
}

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_perf_event_output(
        ctx: *mut core::ffi::c_void,
        map: *mut BpfMapNoMaxEntries,
        flags: u64,
        data: *mut core::ffi::c_void,
        size: u64,
    ) -> i64;
    fn bpf_ringbuf_output(
        ringbuf: *mut BpfMapRingbuf,
        data: *mut core::ffi::c_void,
        size: u64,
        flags: u64,
    ) -> i64;
}

#[no_mangle]
pub static mut perfbuf_val: __u32 = 0;
#[no_mangle]
pub static mut ringbuf_val: __u32 = 0;

#[no_mangle]
pub static mut test_pid: i32 = 0;

#[no_mangle]
#[link_section = ".maps"]
pub static mut array: BpfMapArray = BpfMapArray {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut percpu_array: BpfMapArray = BpfMapArray {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut hash: BpfMapArray = BpfMapArray {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut percpu_hash: BpfMapArray = BpfMapArray {
    type_: BPF_MAP_TYPE_PERCPU_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut perfbuf: BpfMapNoMaxEntries = BpfMapNoMaxEntries {
    type_: BPF_MAP_TYPE_PERF_EVENT_ARRAY,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut ringbuf: BpfMapRingbuf = BpfMapRingbuf {
    type_: BPF_MAP_TYPE_RINGBUF,
    max_entries: 1 << 12,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut prog_array: BpfMapArray = BpfMapArray {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

/* Original section: SEC("fentry/" SYS_PREFIX "sys_nanosleep") */
#[no_mangle]
#[link_section = "fentry/sys_nanosleep"]
pub unsafe extern "C" fn sys_nanosleep_enter(ctx: *mut core::ffi::c_void) -> i32 {
    let cur_pid: i32;

    cur_pid = (bpf_get_current_pid_tgid() >> 32) as i32;

    if cur_pid != test_pid {
        return 0;
    }

    bpf_perf_event_output(
        ctx,
        &mut perfbuf,
        BPF_F_CURRENT_CPU,
        &mut perfbuf_val as *mut __u32 as *mut core::ffi::c_void,
        core::mem::size_of_val(&perfbuf_val) as u64,
    );
    bpf_ringbuf_output(
        &mut ringbuf,
        &mut ringbuf_val as *mut __u32 as *mut core::ffi::c_void,
        core::mem::size_of_val(&ringbuf_val) as u64,
        0,
    );

    0
}

#[no_mangle]
#[link_section = "perf_event"]
pub extern "C" fn handle_perf_event(_ctx: *mut core::ffi::c_void) -> i32 {
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
