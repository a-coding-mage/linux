// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// C includes translated as external dependencies:
// <vmlinux.h>, <string.h>, <stdbool.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_tracing.h>, "bpf_misc.h", "errno.h"

use core::ffi::c_void;

pub type c_char = i8;
pub type c_int = i32;

pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_MAP_TYPE_ARRAY: u32 = 2;
pub const BPF_MAP_TYPE_LRU_HASH: u32 = 9;
pub const BPF_F_NO_PREALLOC: u64 = 1;
pub const BPF_NOEXIST: u64 = 1;

#[repr(C)]
pub struct bpf_map {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_task_work {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct elem {
    pub data: [c_char; 128],
    pub tw: bpf_task_work,
}

#[repr(C)]
pub struct bpf_map_def_hmap {
    pub type_: u32,
    pub map_flags: u64,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[repr(C)]
pub struct bpf_map_def_arrmap {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[repr(C)]
pub struct bpf_map_def_lrumap {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

unsafe extern "C" {
    fn bpf_copy_from_user_str(
        dst: *mut c_void,
        size: u32,
        unsafe_ptr: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_map_update_elem(
        map: *mut c_void,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_task_work_schedule_resume(
        task: *mut task_struct,
        tw: *mut bpf_task_work,
        map: *mut c_void,
        callback: unsafe extern "C" fn(*mut bpf_map, *mut c_void, *mut c_void) -> c_int,
    ) -> c_int;
    fn bpf_task_work_schedule_signal(
        task: *mut task_struct,
        tw: *mut bpf_task_work,
        map: *mut c_void,
        callback: unsafe extern "C" fn(*mut bpf_map, *mut c_void, *mut c_void) -> c_int,
    ) -> c_int;
}

// SEC("license")
#[unsafe(no_mangle)]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

#[unsafe(no_mangle)]
pub static mut user_ptr: *const c_void = core::ptr::null();

// SEC(".maps")
#[unsafe(no_mangle)]
pub static mut hmap: bpf_map_def_hmap = bpf_map_def_hmap {
    type_: BPF_MAP_TYPE_HASH,
    map_flags: BPF_F_NO_PREALLOC,
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<elem>() as u32,
};

// SEC(".maps")
#[unsafe(no_mangle)]
pub static mut arrmap: bpf_map_def_arrmap = bpf_map_def_arrmap {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<elem>() as u32,
};

// SEC(".maps")
#[unsafe(no_mangle)]
pub static mut lrumap: bpf_map_def_lrumap = bpf_map_def_lrumap {
    type_: BPF_MAP_TYPE_LRU_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<elem>() as u32,
};

unsafe extern "C" fn process_work(
    _map: *mut bpf_map,
    _key: *mut c_void,
    value: *mut c_void,
) -> c_int {
    let work: *mut elem = value as *mut elem;

    unsafe {
        bpf_copy_from_user_str(
            (*work).data.as_mut_ptr() as *mut c_void,
            core::mem::size_of_val(&(*work).data) as u32,
            user_ptr as *const c_void,
            0,
        );
    }
    0
}

#[unsafe(no_mangle)]
pub static mut key: c_int = 0;

// SEC("perf_event")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn oncpu_hash_map(_args: *mut pt_regs) -> c_int {
    let empty_work: elem = elem {
        data: [0; 128],
        tw: bpf_task_work { _unused: [] },
    };
    let mut work: *mut elem;
    let task: *mut task_struct;
    let err: c_int;

    unsafe {
        task = bpf_get_current_task_btf();
        err = bpf_map_update_elem(
            (&raw mut hmap) as *mut c_void,
            (&raw const key) as *const c_void,
            (&empty_work as *const elem) as *const c_void,
            BPF_NOEXIST,
        );
        if err != 0 {
            return 0;
        }
        work = bpf_map_lookup_elem(
            (&raw mut hmap) as *mut c_void,
            (&raw const key) as *const c_void,
        ) as *mut elem;
        if work.is_null() {
            return 0;
        }
        bpf_task_work_schedule_resume(
            task,
            &mut (*work).tw,
            (&raw mut hmap) as *mut c_void,
            process_work,
        );
    }
    0
}

// SEC("perf_event")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn oncpu_array_map(_args: *mut pt_regs) -> c_int {
    let work: *mut elem;
    let task: *mut task_struct;

    unsafe {
        task = bpf_get_current_task_btf();
        work = bpf_map_lookup_elem(
            (&raw mut arrmap) as *mut c_void,
            (&raw const key) as *const c_void,
        ) as *mut elem;
        if work.is_null() {
            return 0;
        }
        bpf_task_work_schedule_signal(
            task,
            &mut (*work).tw,
            (&raw mut arrmap) as *mut c_void,
            process_work,
        );
    }
    0
}

// SEC("perf_event")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn oncpu_lru_map(_args: *mut pt_regs) -> c_int {
    let empty_work: elem = elem {
        data: [0; 128],
        tw: bpf_task_work { _unused: [] },
    };
    let mut work: *mut elem;
    let task: *mut task_struct;
    let err: c_int;

    unsafe {
        task = bpf_get_current_task_btf();
        work = bpf_map_lookup_elem(
            (&raw mut lrumap) as *mut c_void,
            (&raw const key) as *const c_void,
        ) as *mut elem;
        if !work.is_null() {
            return 0;
        }
        err = bpf_map_update_elem(
            (&raw mut lrumap) as *mut c_void,
            (&raw const key) as *const c_void,
            (&empty_work as *const elem) as *const c_void,
            BPF_NOEXIST,
        );
        if err != 0 {
            return 0;
        }
        work = bpf_map_lookup_elem(
            (&raw mut lrumap) as *mut c_void,
            (&raw const key) as *const c_void,
        ) as *mut elem;
        if work.is_null() || (*work).data[0] != 0 {
            return 0;
        }
        bpf_task_work_schedule_resume(
            task,
            &mut (*work).tw,
            (&raw mut lrumap) as *mut c_void,
            process_work,
        );
    }
    0
}
