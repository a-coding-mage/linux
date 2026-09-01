// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

/* C dependencies removed from executable Rust:
 * <vmlinux.h>, <string.h>, <stdbool.h>, <bpf/bpf_tracing.h>,
 * "bpf_misc.h", "errno.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type c_void = core::ffi::c_void;
type c_int = i32;
type u32 = u32;
type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_ARRAY: u32 = 2;
const EFAULT: c_int = 14;

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
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

#[repr(C)]
pub struct bpf_task_work {
    _private: [u8; 0],
}

#[repr(C)]
pub struct elem {
    pub file: *mut file,
    pub tw: bpf_task_work,
}

/* Translation of the BPF map declaration:
 * struct {
 *     __uint(type, BPF_MAP_TYPE_ARRAY);
 *     __uint(max_entries, 1);
 *     __type(key, int);
 *     __type(value, struct elem);
 * } arrmap SEC(".maps");
 */
#[repr(C)]
pub struct arrmap_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key: c_int,
    pub value: elem,
}

#[link_section = ".maps"]
pub static mut arrmap: arrmap_def = arrmap_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key: 0,
    value: elem {
        file: core::ptr::null_mut(),
        tw: bpf_task_work { _private: [] },
    },
};

#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

pub static mut user_buf: [i8; 256000] = [0; 256000];
pub static mut tmp_buf: [i8; 256000] = [0; 256000];

pub static mut pid: c_int = 0;
pub static mut err: c_int = 0;
pub static mut run_success: c_int = 0;

type task_work_callback_fn =
    unsafe extern "C" fn(map: *mut bpf_map, key: *mut c_void, value: *mut c_void) -> c_int;

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_get_task_exe_file(task: *mut task_struct) -> *mut file;
    fn bpf_dynptr_from_file(file: *mut file, flags: u64, ptr: *mut bpf_dynptr) -> c_int;
    fn bpf_dynptr_read(
        dst: *mut c_void,
        len: __u32,
        src: *mut bpf_dynptr,
        offset: __u32,
        flags: u64,
    ) -> c_int;
    fn bpf_dynptr_file_discard(ptr: *mut bpf_dynptr);
    fn bpf_put_file(file: *mut file);
    fn bpf_map_lookup_elem(map: *mut arrmap_def, key: *const c_void) -> *mut c_void;
    fn bpf_task_work_schedule_signal(
        task: *mut task_struct,
        work: *mut bpf_task_work,
        map: *mut arrmap_def,
        callback: task_work_callback_fn,
    ) -> c_int;
    fn bpf_dynptr_adjust(ptr: *mut bpf_dynptr, start: __u32, end: __u32) -> c_int;
}

unsafe fn validate_file_read(file: *mut file) -> c_int {
    let mut dynptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let mut loc_err: c_int = 1;
    let mut off: c_int;
    let user_buf_sz: __u32 = core::mem::size_of_val(&user_buf) as __u32;

    if bpf_dynptr_from_file(file, 0, dynptr.as_mut_ptr()) != 0 {
        bpf_dynptr_file_discard(dynptr.as_mut_ptr());
        return loc_err;
    }

    loc_err = verify_dynptr_read(
        dynptr.as_mut_ptr(),
        0,
        user_buf.as_mut_ptr(),
        user_buf_sz,
    );
    off = 1;
    if loc_err == 0 {
        loc_err = verify_dynptr_read(
            dynptr.as_mut_ptr(),
            off as __u32,
            user_buf.as_mut_ptr().offset(off as isize),
            user_buf_sz - off as __u32,
        );
    }
    off = user_buf_sz as c_int - 1;
    if loc_err == 0 {
        loc_err = verify_dynptr_read(
            dynptr.as_mut_ptr(),
            off as __u32,
            user_buf.as_mut_ptr().offset(off as isize),
            user_buf_sz - off as __u32,
        );
    }
    /* Read file with random offset and length */
    off = 4097;
    if loc_err == 0 {
        loc_err = verify_dynptr_read(
            dynptr.as_mut_ptr(),
            off as __u32,
            user_buf.as_mut_ptr().offset(off as isize),
            100,
        );
    }

    /* Adjust dynptr, verify read */
    if loc_err == 0 {
        loc_err = bpf_dynptr_adjust(dynptr.as_mut_ptr(), off as __u32, (off + 1) as __u32);
    }
    if loc_err == 0 {
        loc_err = verify_dynptr_read(
            dynptr.as_mut_ptr(),
            0,
            user_buf.as_mut_ptr().offset(off as isize),
            1,
        );
    }
    /* Can't read more than 1 byte */
    if loc_err == 0 {
        loc_err = (verify_dynptr_read(
            dynptr.as_mut_ptr(),
            0,
            user_buf.as_mut_ptr().offset(off as isize),
            2,
        ) == 0) as c_int;
    }
    /* Can't read with far offset */
    if loc_err == 0 {
        loc_err = (verify_dynptr_read(
            dynptr.as_mut_ptr(),
            1,
            user_buf.as_mut_ptr().offset(off as isize),
            1,
        ) == 0) as c_int;
    }

    bpf_dynptr_file_discard(dynptr.as_mut_ptr());
    loc_err
}

#[link_section = "lsm/file_open"]
pub unsafe extern "C" fn on_open_expect_fault(c: *mut c_void) -> c_int {
    let mut dynptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let mut file: *mut file;
    let mut local_err: c_int = 1;
    let user_buf_sz: __u32 = core::mem::size_of_val(&user_buf) as __u32;

    if (bpf_get_current_pid_tgid() >> 32) as c_int != pid {
        return 0;
    }

    file = bpf_get_task_exe_file(bpf_get_current_task_btf());
    if file.is_null() {
        return 0;
    }

    if bpf_dynptr_from_file(file, 0, dynptr.as_mut_ptr()) != 0 {
        bpf_dynptr_file_discard(dynptr.as_mut_ptr());
        if local_err != 0 {
            err = local_err;
        }
        bpf_put_file(file);
        return 0;
    }

    local_err = bpf_dynptr_read(
        tmp_buf.as_mut_ptr() as *mut c_void,
        user_buf_sz,
        dynptr.as_mut_ptr(),
        user_buf_sz,
        0,
    );
    if local_err == -EFAULT || local_err == 0 {
        /* Expect page fault or success */
        local_err = 0;
        run_success = 1;
    }

    bpf_dynptr_file_discard(dynptr.as_mut_ptr());
    if local_err != 0 {
        err = local_err;
    }
    bpf_put_file(file);
    0
}

#[link_section = "lsm/file_open"]
pub unsafe extern "C" fn on_open_validate_file_read(c: *mut c_void) -> c_int {
    let task: *mut task_struct = bpf_get_current_task_btf();
    let mut work: *mut elem;
    let mut key: c_int = 0;

    if (bpf_get_current_pid_tgid() >> 32) as c_int != pid {
        return 0;
    }

    work = bpf_map_lookup_elem(&mut arrmap, &mut key as *mut c_int as *mut c_void) as *mut elem;
    if work.is_null() {
        err = 1;
        return 0;
    }
    bpf_task_work_schedule_signal(task, &mut (*work).tw, &mut arrmap, task_work_callback);
    0
}

/* Called in a sleepable context, read 256K bytes, cross check with user space read data */
unsafe extern "C" fn task_work_callback(
    map: *mut bpf_map,
    key: *mut c_void,
    value: *mut c_void,
) -> c_int {
    let task: *mut task_struct = bpf_get_current_task_btf();
    let file: *mut file = bpf_get_task_exe_file(task);

    if file.is_null() {
        return 0;
    }

    err = validate_file_read(file);
    if err == 0 {
        run_success = 1;
    }
    bpf_put_file(file);
    0
}

unsafe fn verify_dynptr_read(
    ptr: *mut bpf_dynptr,
    off: u32,
    user_buf: *mut i8,
    len: u32,
) -> c_int {
    let mut i: c_int;

    if bpf_dynptr_read(tmp_buf.as_mut_ptr() as *mut c_void, len, ptr, off, 0) != 0 {
        return 1;
    }

    /* Verify file contents read from BPF is the same as the one read from userspace */
    i = 0;
    while i < len as c_int {
        if tmp_buf[i as usize] != *user_buf.offset(i as isize) {
            return 1;
        }
        i += 1;
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
