// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

/* Translated from C. Original dependencies:
 * <vmlinux.h>
 * <bpf/bpf_helpers.h>
 * <bpf/bpf_tracing.h>
 * "bpf_misc.h"
 * "bpf_experimental.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type c_void = core::ffi::c_void;
pub type c_int = i32;
pub type u32 = u32;

#[repr(C)]
pub struct bpf_timer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_wq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_task_work {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_copy_from_user(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> c_int;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_timer_init(timer: *mut bpf_timer, map: *mut c_void, flags: u64) -> c_int;
    fn bpf_timer_set_callback(
        timer: *mut bpf_timer,
        callback: unsafe extern "C" fn(*mut c_void, *mut c_int, *mut bpf_timer) -> c_int,
    ) -> c_int;
    fn bpf_wq_init(wq: *mut bpf_wq, map: *mut c_void, flags: u64) -> c_int;
    fn bpf_wq_set_callback(
        wq: *mut bpf_wq,
        callback: unsafe extern "C" fn(*mut c_void, *mut c_int, *mut c_void) -> c_int,
        flags: u64,
    ) -> c_int;
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_task_work_schedule_resume(
        task: *mut task_struct,
        tw: *mut bpf_task_work,
        map: *mut c_void,
        callback: unsafe extern "C" fn(*mut bpf_map, *mut c_void, *mut c_void) -> c_int,
    ) -> c_int;
}

/* char _license[] SEC("license") = "GPL"; */
#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* Timer tests */

#[repr(C)]
pub struct timer_elem {
    pub t: bpf_timer,
}

#[repr(C)]
pub struct timer_map_def {
    _private: [u8; 0],
}

/* C map definition:
 * struct {
 *     __uint(type, BPF_MAP_TYPE_ARRAY);
 *     __uint(max_entries, 1);
 *     __type(key, int);
 *     __type(value, struct timer_elem);
 * } timer_map SEC(".maps");
 */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut timer_map: timer_map_def = timer_map_def { _private: [] };

unsafe extern "C" fn timer_cb(
    _map: *mut c_void,
    _key: *mut c_int,
    _timer: *mut bpf_timer,
) -> c_int {
    let mut data: u32 = 0;
    /* Timer callbacks are never sleepable, even from non-sleepable programs */
    unsafe {
        bpf_copy_from_user(
            &mut data as *mut u32 as *mut c_void,
            core::mem::size_of_val(&data) as u32,
            core::ptr::null(),
        );
    }
    0
}

/* SEC("fentry/bpf_fentry_test1")
 * __failure __msg("sleepable helper bpf_copy_from_user#{{[0-9]+}} in non-sleepable prog")
 */
#[unsafe(no_mangle)]
#[unsafe(link_section = "fentry/bpf_fentry_test1")]
pub unsafe extern "C" fn timer_non_sleepable_prog(_ctx: *mut c_void) -> c_int {
    let mut val: *mut timer_elem;
    let key: c_int = 0;

    unsafe {
        val = bpf_map_lookup_elem(
            &raw mut timer_map as *mut timer_map_def as *mut c_void,
            &key as *const c_int as *const c_void,
        ) as *mut timer_elem;
        if val.is_null() {
            return 0;
        }

        bpf_timer_init(
            &mut (*val).t as *mut bpf_timer,
            &raw mut timer_map as *mut timer_map_def as *mut c_void,
            0,
        );
        bpf_timer_set_callback(&mut (*val).t as *mut bpf_timer, timer_cb);
    }
    0
}

/* SEC("lsm.s/file_open")
 * __failure __msg("sleepable helper bpf_copy_from_user#{{[0-9]+}} in non-sleepable prog")
 */
#[unsafe(no_mangle)]
#[unsafe(link_section = "lsm.s/file_open")]
pub unsafe extern "C" fn timer_sleepable_prog(_ctx: *mut c_void) -> c_int {
    let mut val: *mut timer_elem;
    let key: c_int = 0;

    unsafe {
        val = bpf_map_lookup_elem(
            &raw mut timer_map as *mut timer_map_def as *mut c_void,
            &key as *const c_int as *const c_void,
        ) as *mut timer_elem;
        if val.is_null() {
            return 0;
        }

        bpf_timer_init(
            &mut (*val).t as *mut bpf_timer,
            &raw mut timer_map as *mut timer_map_def as *mut c_void,
            0,
        );
        bpf_timer_set_callback(&mut (*val).t as *mut bpf_timer, timer_cb);
    }
    0
}

/* Workqueue tests */

#[repr(C)]
pub struct wq_elem {
    pub w: bpf_wq,
}

#[repr(C)]
pub struct wq_map_def {
    _private: [u8; 0],
}

/* C map definition:
 * struct {
 *     __uint(type, BPF_MAP_TYPE_ARRAY);
 *     __uint(max_entries, 1);
 *     __type(key, int);
 *     __type(value, struct wq_elem);
 * } wq_map SEC(".maps");
 */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut wq_map: wq_map_def = wq_map_def { _private: [] };

unsafe extern "C" fn wq_cb(
    _map: *mut c_void,
    _key: *mut c_int,
    _value: *mut c_void,
) -> c_int {
    let mut data: u32 = 0;
    /* Workqueue callbacks are always sleepable, even from non-sleepable programs */
    unsafe {
        bpf_copy_from_user(
            &mut data as *mut u32 as *mut c_void,
            core::mem::size_of_val(&data) as u32,
            core::ptr::null(),
        );
    }
    0
}

/* SEC("fentry/bpf_fentry_test1")
 * __success
 */
#[unsafe(no_mangle)]
#[unsafe(link_section = "fentry/bpf_fentry_test1")]
pub unsafe extern "C" fn wq_non_sleepable_prog(_ctx: *mut c_void) -> c_int {
    let mut val: *mut wq_elem;
    let key: c_int = 0;

    unsafe {
        val = bpf_map_lookup_elem(
            &raw mut wq_map as *mut wq_map_def as *mut c_void,
            &key as *const c_int as *const c_void,
        ) as *mut wq_elem;
        if val.is_null() {
            return 0;
        }

        if bpf_wq_init(
            &mut (*val).w as *mut bpf_wq,
            &raw mut wq_map as *mut wq_map_def as *mut c_void,
            0,
        ) != 0
        {
            return 0;
        }
        if bpf_wq_set_callback(&mut (*val).w as *mut bpf_wq, wq_cb, 0) != 0 {
            return 0;
        }
    }
    0
}

/* SEC("lsm.s/file_open")
 * __success
 */
#[unsafe(no_mangle)]
#[unsafe(link_section = "lsm.s/file_open")]
pub unsafe extern "C" fn wq_sleepable_prog(_ctx: *mut c_void) -> c_int {
    let mut val: *mut wq_elem;
    let key: c_int = 0;

    unsafe {
        val = bpf_map_lookup_elem(
            &raw mut wq_map as *mut wq_map_def as *mut c_void,
            &key as *const c_int as *const c_void,
        ) as *mut wq_elem;
        if val.is_null() {
            return 0;
        }

        if bpf_wq_init(
            &mut (*val).w as *mut bpf_wq,
            &raw mut wq_map as *mut wq_map_def as *mut c_void,
            0,
        ) != 0
        {
            return 0;
        }
        if bpf_wq_set_callback(&mut (*val).w as *mut bpf_wq, wq_cb, 0) != 0 {
            return 0;
        }
    }
    0
}

/* Task work tests */

#[repr(C)]
pub struct task_work_elem {
    pub tw: bpf_task_work,
}

#[repr(C)]
pub struct task_work_map_def {
    _private: [u8; 0],
}

/* C map definition:
 * struct {
 *     __uint(type, BPF_MAP_TYPE_ARRAY);
 *     __uint(max_entries, 1);
 *     __type(key, int);
 *     __type(value, struct task_work_elem);
 * } task_work_map SEC(".maps");
 */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut task_work_map: task_work_map_def = task_work_map_def { _private: [] };

unsafe extern "C" fn task_work_cb(
    _map: *mut bpf_map,
    _key: *mut c_void,
    _value: *mut c_void,
) -> c_int {
    let mut data: u32 = 0;
    /* Task work callbacks are always sleepable, even from non-sleepable programs */
    unsafe {
        bpf_copy_from_user(
            &mut data as *mut u32 as *mut c_void,
            core::mem::size_of_val(&data) as u32,
            core::ptr::null(),
        );
    }
    0
}

/* SEC("fentry/bpf_fentry_test1")
 * __success
 */
#[unsafe(no_mangle)]
#[unsafe(link_section = "fentry/bpf_fentry_test1")]
pub unsafe extern "C" fn task_work_non_sleepable_prog(_ctx: *mut c_void) -> c_int {
    let mut val: *mut task_work_elem;
    let mut task: *mut task_struct;
    let key: c_int = 0;

    unsafe {
        val = bpf_map_lookup_elem(
            &raw mut task_work_map as *mut task_work_map_def as *mut c_void,
            &key as *const c_int as *const c_void,
        ) as *mut task_work_elem;
        if val.is_null() {
            return 0;
        }

        task = bpf_get_current_task_btf();
        if task.is_null() {
            return 0;
        }

        bpf_task_work_schedule_resume(
            task,
            &mut (*val).tw as *mut bpf_task_work,
            &raw mut task_work_map as *mut task_work_map_def as *mut c_void,
            task_work_cb,
        );
    }
    0
}

/* SEC("lsm.s/file_open")
 * __success
 */
#[unsafe(no_mangle)]
#[unsafe(link_section = "lsm.s/file_open")]
pub unsafe extern "C" fn task_work_sleepable_prog(_ctx: *mut c_void) -> c_int {
    let mut val: *mut task_work_elem;
    let mut task: *mut task_struct;
    let key: c_int = 0;

    unsafe {
        val = bpf_map_lookup_elem(
            &raw mut task_work_map as *mut task_work_map_def as *mut c_void,
            &key as *const c_int as *const c_void,
        ) as *mut task_work_elem;
        if val.is_null() {
            return 0;
        }

        task = bpf_get_current_task_btf();
        if task.is_null() {
            return 0;
        }

        bpf_task_work_schedule_resume(
            task,
            &mut (*val).tw as *mut bpf_task_work,
            &raw mut task_work_map as *mut task_work_map_def as *mut c_void,
            task_work_cb,
        );
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
