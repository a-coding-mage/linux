// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// C dependencies translated by intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

const BPF_MAP_TYPE_RINGBUF: u32 = 27;
const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_ANY: u64 = 0;

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct sample {
    pub pid: i32,
    pub seq: i32,
    pub value: i64,
    pub comm: [i8; 16],
}

#[repr(C)]
pub struct ringbuf_map_def {
    pub type_: u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut ringbuf: ringbuf_map_def = ringbuf_map_def {
    type_: BPF_MAP_TYPE_RINGBUF,
};

#[repr(C)]
pub struct hash_map_def {
    pub type_: u32,
    pub max_entries: u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut hash_map: hash_map_def = hash_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1000,
};

/* inputs */
#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;

/* inner state */
#[unsafe(no_mangle)]
pub static mut seq: i64 = 0;

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_ringbuf_reserve(ringbuf: *mut ringbuf_map_def, size: u64, flags: u64) -> *mut core::ffi::c_void;
    fn bpf_get_current_comm(buf: *mut core::ffi::c_void, size_of_buf: u32) -> i64;
    fn bpf_map_lookup_elem(map: *mut hash_map_def, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut hash_map_def,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
    fn bpf_ringbuf_submit(data: *mut core::ffi::c_void, flags: u64);
    fn __sink(arg: *mut i32);
}

// SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_ringbuf_mem_map_key(ctx: *mut core::ffi::c_void) -> i32 {
    let cur_pid: i32 = (unsafe { bpf_get_current_pid_tgid() } >> 32) as i32;
    let sample: *mut sample;
    let lookup_val: *mut i32;

    let _ = ctx;

    if cur_pid != unsafe { pid } {
        return 0;
    }

    sample = unsafe {
        bpf_ringbuf_reserve(
            &raw mut ringbuf,
            core::mem::size_of::<sample>() as u64,
            0,
        ) as *mut sample
    };
    if sample.is_null() {
        return 0;
    }

    unsafe {
        (*sample).pid = pid;
        bpf_get_current_comm(
            (*sample).comm.as_mut_ptr() as *mut core::ffi::c_void,
            core::mem::size_of_val(&(*sample).comm) as u32,
        );
        seq += 1;
        (*sample).seq = seq as i32;
        (*sample).value = 42;
    }

    /* test using 'sample' (PTR_TO_MEM | MEM_ALLOC) as map key arg
     */
    lookup_val = unsafe {
        bpf_map_lookup_elem(
            &raw mut hash_map,
            sample as *const core::ffi::c_void,
        ) as *mut i32
    };
    unsafe {
        __sink(lookup_val);
    }

    /*
     * Since bpf_map_lookup_elem above uses 'sample' as key, test using
     * sample field as value below
     */
    unsafe {
        bpf_map_update_elem(
            &raw mut hash_map,
            sample as *const core::ffi::c_void,
            &raw const (*sample).seq as *const core::ffi::c_void,
            BPF_ANY,
        );
    }

    unsafe {
        bpf_ringbuf_submit(sample as *mut core::ffi::c_void, 0);
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
