// SPDX-License-Identifier: GPL-2.0
// Depends on test_progs.h and stacktrace_map_skip.skel.h definitions.

use core::ffi::{c_char, c_int, c_uint, c_void};

const TEST_STACK_DEPTH: c_int = 2;

#[repr(C)]
pub struct stacktrace_map_skip {
    pub maps: stacktrace_map_skip_maps,
    pub bss: *mut stacktrace_map_skip_bss,
}

#[repr(C)]
pub struct stacktrace_map_skip_maps {
    pub stackid_hmap: *mut bpf_map,
    pub stackmap: *mut bpf_map,
    pub stack_amap: *mut bpf_map,
}

#[repr(C)]
pub struct stacktrace_map_skip_bss {
    pub pid: c_int,
    pub control: c_int,
    pub failed: c_int,
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn stacktrace_map_skip__open_and_load() -> *mut stacktrace_map_skip;
    fn stacktrace_map_skip__attach(skel: *mut stacktrace_map_skip) -> c_int;
    fn stacktrace_map_skip__destroy(skel: *mut stacktrace_map_skip);

    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn compare_map_keys(map1_fd: c_int, map2_fd: c_int) -> c_int;
    fn compare_stack_ips(stackmap_fd: c_int, stack_amap_fd: c_int, stack_trace_len: c_int) -> c_int;
    fn getpid() -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

pub unsafe fn test_stacktrace_map_skip() {
    let skel: *mut stacktrace_map_skip;
    let stackid_hmap_fd: c_int;
    let stackmap_fd: c_int;
    let stack_amap_fd: c_int;
    let mut err: c_int;
    let stack_trace_len: c_int;

    skel = stacktrace_map_skip__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open_and_load".as_ptr()) {
        return;
    }

    /* find map fds */
    stackid_hmap_fd = bpf_map__fd((*skel).maps.stackid_hmap);
    if !ASSERT_GE(stackid_hmap_fd, 0, c"stackid_hmap fd".as_ptr()) {
        stacktrace_map_skip__destroy(skel);
        return;
    }

    stackmap_fd = bpf_map__fd((*skel).maps.stackmap);
    if !ASSERT_GE(stackmap_fd, 0, c"stackmap fd".as_ptr()) {
        stacktrace_map_skip__destroy(skel);
        return;
    }

    stack_amap_fd = bpf_map__fd((*skel).maps.stack_amap);
    if !ASSERT_GE(stack_amap_fd, 0, c"stack_amap fd".as_ptr()) {
        stacktrace_map_skip__destroy(skel);
        return;
    }

    (*(*skel).bss).pid = getpid();

    err = stacktrace_map_skip__attach(skel);
    if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
        stacktrace_map_skip__destroy(skel);
        return;
    }

    /* give some time for bpf program run */
    sleep(1);

    /* disable stack trace collection */
    (*(*skel).bss).control = 1;

    /* for every element in stackid_hmap, we can find a corresponding one
     * in stackmap, and vice versa.
     */
    err = compare_map_keys(stackid_hmap_fd, stackmap_fd);
    if !ASSERT_OK(err, c"compare_map_keys stackid_hmap vs. stackmap".as_ptr()) {
        stacktrace_map_skip__destroy(skel);
        return;
    }

    err = compare_map_keys(stackmap_fd, stackid_hmap_fd);
    if !ASSERT_OK(err, c"compare_map_keys stackmap vs. stackid_hmap".as_ptr()) {
        stacktrace_map_skip__destroy(skel);
        return;
    }

    stack_trace_len = TEST_STACK_DEPTH * core::mem::size_of::<u64>() as c_int;
    err = compare_stack_ips(stackmap_fd, stack_amap_fd, stack_trace_len);
    if !ASSERT_OK(err, c"compare_stack_ips stackmap vs. stack_amap".as_ptr()) {
        stacktrace_map_skip__destroy(skel);
        return;
    }

    if !ASSERT_EQ((*(*skel).bss).failed, 0, c"skip_failed".as_ptr()) {
        stacktrace_map_skip__destroy(skel);
        return;
    }

    stacktrace_map_skip__destroy(skel);
}
