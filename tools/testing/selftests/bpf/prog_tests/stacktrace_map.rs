// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, "stacktrace_map.skel.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::c_void;
use core::mem::size_of;

type c_char = i8;
type c_int = i32;
type __u32 = u32;
type __u64 = u64;

const PERF_MAX_STACK_DEPTH: usize = 127;
const ENOENT: c_int = 2;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stacktrace_map_maps {
    pub control_map: *mut bpf_map,
    pub stackid_hmap: *mut bpf_map,
    pub stackmap: *mut bpf_map,
    pub stack_amap: *mut bpf_map,
}

#[repr(C)]
pub struct stacktrace_map_bss {
    pub stack_id: __u32,
}

#[repr(C)]
pub struct stacktrace_map {
    pub maps: stacktrace_map_maps,
    pub bss: *mut stacktrace_map_bss,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn stacktrace_map__open_and_load() -> *mut stacktrace_map;
    fn stacktrace_map__attach(skel: *mut stacktrace_map) -> c_int;
    fn stacktrace_map__destroy(skel: *mut stacktrace_map);

    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: __u64,
    ) -> c_int;
    fn bpf_map_lookup_and_delete_elem(
        fd: c_int,
        key: *const c_void,
        value: *mut c_void,
    ) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;

    fn compare_map_keys(map1_fd: c_int, map2_fd: c_int) -> c_int;
    fn compare_stack_ips(stackmap_fd: c_int, stack_amap_fd: c_int, stack_trace_len: c_int)
        -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn CHECK(condition: c_int, name: *const c_char, format: *const c_char, ...) -> bool;

    fn sleep(seconds: c_uint) -> c_uint;
}

type c_uint = u32;

pub unsafe extern "C" fn test_stacktrace_map() {
    let skel: *mut stacktrace_map;
    let control_map_fd: c_int;
    let stackid_hmap_fd: c_int;
    let stackmap_fd: c_int;
    let stack_amap_fd: c_int;
    let mut err: c_int;
    let stack_trace_len: c_int;
    let mut key: __u32;
    let mut val: __u32;
    let mut stack_id: __u32;
    let _duration: __u32 = 0;
    let mut stack: [__u64; PERF_MAX_STACK_DEPTH] = [0; PERF_MAX_STACK_DEPTH];

    skel = stacktrace_map__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        b"skel_open_and_load\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    control_map_fd = bpf_map__fd((*skel).maps.control_map);
    stackid_hmap_fd = bpf_map__fd((*skel).maps.stackid_hmap);
    stackmap_fd = bpf_map__fd((*skel).maps.stackmap);
    stack_amap_fd = bpf_map__fd((*skel).maps.stack_amap);

    'out: {
        err = stacktrace_map__attach(skel);
        if !ASSERT_OK(err, b"skel_attach\0".as_ptr() as *const c_char) {
            break 'out;
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
        if CHECK(
            err,
            b"compare_map_keys stackid_hmap vs. stackmap\0".as_ptr() as *const c_char,
            b"err %d errno %d\n\0".as_ptr() as *const c_char,
            err,
            errno,
        ) {
            break 'out;
        }

        err = compare_map_keys(stackmap_fd, stackid_hmap_fd);
        if CHECK(
            err,
            b"compare_map_keys stackmap vs. stackid_hmap\0".as_ptr() as *const c_char,
            b"err %d errno %d\n\0".as_ptr() as *const c_char,
            err,
            errno,
        ) {
            break 'out;
        }

        stack_trace_len = (PERF_MAX_STACK_DEPTH * size_of::<__u64>()) as c_int;
        err = compare_stack_ips(stackmap_fd, stack_amap_fd, stack_trace_len);
        if CHECK(
            err,
            b"compare_stack_ips stackmap vs. stack_amap\0".as_ptr() as *const c_char,
            b"err %d errno %d\n\0".as_ptr() as *const c_char,
            err,
            errno,
        ) {
            break 'out;
        }

        stack_id = (*(*skel).bss).stack_id;
        err = bpf_map_lookup_and_delete_elem(
            stackmap_fd,
            &stack_id as *const __u32 as *const c_void,
            stack.as_mut_ptr() as *mut c_void,
        );
        if !ASSERT_OK(
            err,
            b"lookup and delete target stack_id\0".as_ptr() as *const c_char,
        ) {
            break 'out;
        }

        err = bpf_map_lookup_elem(
            stackmap_fd,
            &stack_id as *const __u32 as *const c_void,
            stack.as_mut_ptr() as *mut c_void,
        );
        if !ASSERT_EQ(
            err,
            -ENOENT,
            b"lookup deleted stack_id\0".as_ptr() as *const c_char,
        ) {
            break 'out;
        }
    }
    stacktrace_map__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
