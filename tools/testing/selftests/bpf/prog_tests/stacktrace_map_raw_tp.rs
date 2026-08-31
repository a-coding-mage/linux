// SPDX-License-Identifier: GPL-2.0
// C dependency intent: #include <test_progs.h>

use core::ffi::{c_char, c_void};

type __u32 = u32;

const BPF_PROG_TYPE_RAW_TRACEPOINT: u32 = 17;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: i32;

    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: u32,
        obj: *mut *mut bpf_object,
        prog_fd: *mut i32,
    ) -> i32;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__attach_raw_tracepoint(
        prog: *mut bpf_program,
        tp_name: *const c_char,
    ) -> *mut bpf_link;
    fn bpf_find_map(
        test_name: *const c_char,
        obj: *mut bpf_object,
        map_name: *const c_char,
    ) -> i32;
    fn sleep(seconds: u32) -> u32;
    fn bpf_map_update_elem(fd: i32, key: *const c_void, value: *const c_void, flags: u64) -> i32;
    fn compare_map_keys(map1_fd: i32, map2_fd: i32) -> i32;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__close(obj: *mut bpf_object);
}

pub unsafe fn test_stacktrace_map_raw_tp() {
    let prog_name: *const c_char = c"oncpu".as_ptr();
    let control_map_fd: i32;
    let stackid_hmap_fd: i32;
    let stackmap_fd: i32;
    let file: *const c_char = c"./stacktrace_map.bpf.o".as_ptr();
    let mut key: __u32;
    let mut val: __u32;
    let _duration: __u32 = 0;
    let mut err: i32;
    let mut prog_fd: i32 = 0;
    let mut prog: *mut bpf_program;
    let mut obj: *mut bpf_object = core::ptr::null_mut();
    let mut link: *mut bpf_link = core::ptr::null_mut();

    err = bpf_prog_test_load(
        file,
        BPF_PROG_TYPE_RAW_TRACEPOINT,
        &mut obj,
        &mut prog_fd,
    );
    if CHECK!(
        err,
        c"prog_load raw tp".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        return;
    }

    prog = bpf_object__find_program_by_name(obj, prog_name);
    if CHECK!(
        prog.is_null(),
        c"find_prog".as_ptr(),
        c"prog '%s' not found\n".as_ptr(),
        prog_name
    ) {
        bpf_link__destroy(link);
        bpf_object__close(obj);
        return;
    }

    link = bpf_program__attach_raw_tracepoint(prog, c"sched_switch".as_ptr());
    if !ASSERT_OK_PTR!(link, c"attach_raw_tp".as_ptr()) {
        bpf_link__destroy(link);
        bpf_object__close(obj);
        return;
    }

    /* find map fds */
    control_map_fd = bpf_find_map(c"test_stacktrace_map_raw_tp".as_ptr(), obj, c"control_map".as_ptr());
    if CHECK_FAIL!(control_map_fd < 0) {
        bpf_link__destroy(link);
        bpf_object__close(obj);
        return;
    }

    stackid_hmap_fd =
        bpf_find_map(c"test_stacktrace_map_raw_tp".as_ptr(), obj, c"stackid_hmap".as_ptr());
    if CHECK_FAIL!(stackid_hmap_fd < 0) {
        bpf_link__destroy(link);
        bpf_object__close(obj);
        return;
    }

    stackmap_fd = bpf_find_map(c"test_stacktrace_map_raw_tp".as_ptr(), obj, c"stackmap".as_ptr());
    if CHECK_FAIL!(stackmap_fd < 0) {
        bpf_link__destroy(link);
        bpf_object__close(obj);
        return;
    }

    /* give some time for bpf program run */
    sleep(1);

    /* disable stack trace collection */
    key = 0;
    val = 1;
    bpf_map_update_elem(
        control_map_fd,
        &key as *const __u32 as *const c_void,
        &val as *const __u32 as *const c_void,
        0,
    );

    /* for every element in stackid_hmap, we can find a corresponding one
     * in stackmap, and vice versa.
     */
    err = compare_map_keys(stackid_hmap_fd, stackmap_fd);
    if CHECK!(
        err,
        c"compare_map_keys stackid_hmap vs. stackmap".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        bpf_link__destroy(link);
        bpf_object__close(obj);
        return;
    }

    err = compare_map_keys(stackmap_fd, stackid_hmap_fd);
    if CHECK!(
        err,
        c"compare_map_keys stackmap vs. stackid_hmap".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        bpf_link__destroy(link);
        bpf_object__close(obj);
        return;
    }

    bpf_link__destroy(link);
    bpf_object__close(obj);
}
