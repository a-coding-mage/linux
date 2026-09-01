// SPDX-License-Identifier: GPL-2.0
// Rust translation of testing/selftests/bpf/prog_tests/missed.c.
// External test, libbpf, and skeleton definitions are supplied by the surrounding tree.

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;
type __u64 = u64;

const BPF_LINK_TYPE_PERF_EVENT: __u32 = 7;
const BPF_PERF_EVENT_KPROBE: __u32 = 1;

#[repr(C)]
pub struct bpf_test_run_opts {
    pub retval: __u32,
}

#[repr(C)]
pub struct bpf_link_info {
    pub type_: __u32,
    pub perf_event: bpf_link_info_perf_event,
}

#[repr(C)]
pub struct bpf_link_info_perf_event {
    pub type_: __u32,
    pub kprobe: bpf_link_info_perf_event_kprobe,
}

#[repr(C)]
pub struct bpf_link_info_perf_event_kprobe {
    pub missed: __u64,
}

#[repr(C)]
pub struct bpf_prog_info {
    pub recursion_misses: __u64,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct missed_kprobe {
    pub progs: missed_kprobe_progs,
    pub links: missed_kprobe_links,
}

#[repr(C)]
pub struct missed_kprobe_progs {
    pub trigger: *mut bpf_program,
}

#[repr(C)]
pub struct missed_kprobe_links {
    pub test2: *mut bpf_link,
}

#[repr(C)]
pub struct missed_kprobe_recursion {
    pub progs: missed_kprobe_recursion_progs,
}

#[repr(C)]
pub struct missed_kprobe_recursion_progs {
    pub trigger: *mut bpf_program,
    pub test1: *mut bpf_program,
    pub test2: *mut bpf_program,
    pub test3: *mut bpf_program,
    pub test4: *mut bpf_program,
    pub test5: *mut bpf_program,
    pub test6: *mut bpf_program,
}

#[repr(C)]
pub struct missed_tp_recursion {
    pub progs: missed_tp_recursion_progs,
}

#[repr(C)]
pub struct missed_tp_recursion_progs {
    pub trigger: *mut bpf_program,
    pub test1: *mut bpf_program,
    pub test2: *mut bpf_program,
    pub test3: *mut bpf_program,
    pub test4: *mut bpf_program,
}

unsafe extern "C" {
    fn missed_kprobe__open_and_load() -> *mut missed_kprobe;
    fn missed_kprobe__attach(skel: *mut missed_kprobe) -> c_int;
    fn missed_kprobe__destroy(skel: *mut missed_kprobe);

    fn missed_kprobe_recursion__open_and_load() -> *mut missed_kprobe_recursion;
    fn missed_kprobe_recursion__attach(skel: *mut missed_kprobe_recursion) -> c_int;
    fn missed_kprobe_recursion__destroy(skel: *mut missed_kprobe_recursion);

    fn missed_tp_recursion__open_and_load() -> *mut missed_tp_recursion;
    fn missed_tp_recursion__attach(skel: *mut missed_tp_recursion) -> c_int;
    fn missed_tp_recursion__destroy(skel: *mut missed_tp_recursion);

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_link_get_info_by_fd(fd: c_int, info: *mut bpf_link_info, len: *mut __u32) -> c_int;
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, len: *mut __u32) -> c_int;

    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_GE<T>(actual: T, expected: T, name: *const c_char) -> bool;
}

/*
 * Putting kprobe on bpf_fentry_test1 that calls bpf_kfunc_common_test
 * kfunc, which has also kprobe on. The latter won't get triggered due
 * to kprobe recursion check and kprobe missed counter is incremented.
 */
unsafe fn test_missed_perf_kprobe() {
    let mut topts: bpf_test_run_opts = core::mem::zeroed();
    let mut info: bpf_link_info = core::mem::zeroed();
    let mut skel: *mut missed_kprobe;
    let mut len: __u32 = size_of::<bpf_link_info>() as __u32;
    let mut err: c_int;
    let prog_fd: c_int;

    skel = missed_kprobe__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"missed_kprobe__open_and_load".as_ptr()) {
        goto_cleanup_missed_perf_kprobe(skel);
        return;
    }

    err = missed_kprobe__attach(skel);
    if !ASSERT_OK(err, c"missed_kprobe__attach".as_ptr()) {
        goto_cleanup_missed_perf_kprobe(skel);
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.trigger);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(topts.retval, 0, c"test_run".as_ptr());

    err = bpf_link_get_info_by_fd(bpf_link__fd((*skel).links.test2), &mut info, &mut len);
    if !ASSERT_OK(err, c"bpf_link_get_info_by_fd".as_ptr()) {
        goto_cleanup_missed_perf_kprobe(skel);
        return;
    }

    ASSERT_EQ(info.type_, BPF_LINK_TYPE_PERF_EVENT, c"info.type".as_ptr());
    ASSERT_EQ(info.perf_event.type_, BPF_PERF_EVENT_KPROBE, c"info.perf_event.type".as_ptr());
    ASSERT_EQ(info.perf_event.kprobe.missed, 1, c"info.perf_event.kprobe.missed".as_ptr());

    goto_cleanup_missed_perf_kprobe(skel);
}

unsafe fn goto_cleanup_missed_perf_kprobe(skel: *mut missed_kprobe) {
    missed_kprobe__destroy(skel);
}

unsafe fn get_missed_count(fd: c_int) -> __u64 {
    let mut info: bpf_prog_info = core::mem::zeroed();
    let mut len: __u32 = size_of::<bpf_prog_info>() as __u32;
    let err: c_int;

    err = bpf_prog_get_info_by_fd(fd, &mut info, &mut len);
    if !ASSERT_OK(err, c"bpf_prog_get_info_by_fd".as_ptr()) {
        return -1i64 as __u64;
    }
    info.recursion_misses
}

/*
 * Putting kprobe.multi on bpf_fentry_test1 that calls bpf_kfunc_common_test
 * kfunc which has 3 perf event kprobes and 1 kprobe.multi attached.
 *
 * Because fprobe (kprobe.multi attach layear) does not have strict recursion
 * check the kprobe's bpf_prog_active check is hit for test2-5.
 */
unsafe fn test_missed_kprobe_recursion() {
    let mut topts: bpf_test_run_opts = core::mem::zeroed();
    let mut skel: *mut missed_kprobe_recursion;
    let mut err: c_int;
    let prog_fd: c_int;

    skel = missed_kprobe_recursion__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"missed_kprobe_recursion__open_and_load".as_ptr()) {
        goto_cleanup_missed_kprobe_recursion(skel);
        return;
    }

    err = missed_kprobe_recursion__attach(skel);
    if !ASSERT_OK(err, c"missed_kprobe_recursion__attach".as_ptr()) {
        goto_cleanup_missed_kprobe_recursion(skel);
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.trigger);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(topts.retval, 0, c"test_run".as_ptr());

    ASSERT_EQ(get_missed_count(bpf_program__fd((*skel).progs.test1)), 0, c"test1_recursion_misses".as_ptr());
    ASSERT_GE(get_missed_count(bpf_program__fd((*skel).progs.test2)), 1, c"test2_recursion_misses".as_ptr());
    ASSERT_GE(get_missed_count(bpf_program__fd((*skel).progs.test3)), 1, c"test3_recursion_misses".as_ptr());
    ASSERT_GE(get_missed_count(bpf_program__fd((*skel).progs.test4)), 1, c"test4_recursion_misses".as_ptr());
    ASSERT_GE(get_missed_count(bpf_program__fd((*skel).progs.test5)), 1, c"test5_recursion_misses".as_ptr());
    ASSERT_EQ(get_missed_count(bpf_program__fd((*skel).progs.test6)), 1, c"test6_recursion_misses".as_ptr());

    goto_cleanup_missed_kprobe_recursion(skel);
}

unsafe fn goto_cleanup_missed_kprobe_recursion(skel: *mut missed_kprobe_recursion) {
    missed_kprobe_recursion__destroy(skel);
}

/*
 * Putting kprobe on bpf_fentry_test1 that calls bpf_printk and invokes
 * bpf_trace_printk tracepoint. The bpf_trace_printk tracepoint has test[234]
 * programs attached to it.
 *
 * Because kprobe execution goes through bpf_prog_active check, programs
 * attached to the tracepoint will fail the recursion check and increment
 * the recursion_misses stats.
 */
unsafe fn test_missed_tp_recursion() {
    let mut topts: bpf_test_run_opts = core::mem::zeroed();
    let mut skel: *mut missed_tp_recursion;
    let mut err: c_int;
    let prog_fd: c_int;

    skel = missed_tp_recursion__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"missed_tp_recursion__open_and_load".as_ptr()) {
        goto_cleanup_missed_tp_recursion(skel);
        return;
    }

    err = missed_tp_recursion__attach(skel);
    if !ASSERT_OK(err, c"missed_tp_recursion__attach".as_ptr()) {
        goto_cleanup_missed_tp_recursion(skel);
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.trigger);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(topts.retval, 0, c"test_run".as_ptr());

    ASSERT_EQ(get_missed_count(bpf_program__fd((*skel).progs.test1)), 0, c"test1_recursion_misses".as_ptr());
    ASSERT_EQ(get_missed_count(bpf_program__fd((*skel).progs.test2)), 1, c"test2_recursion_misses".as_ptr());
    ASSERT_EQ(get_missed_count(bpf_program__fd((*skel).progs.test3)), 1, c"test3_recursion_misses".as_ptr());
    ASSERT_EQ(get_missed_count(bpf_program__fd((*skel).progs.test4)), 1, c"test4_recursion_misses".as_ptr());

    goto_cleanup_missed_tp_recursion(skel);
}

unsafe fn goto_cleanup_missed_tp_recursion(skel: *mut missed_tp_recursion) {
    missed_tp_recursion__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_missed() {
    if test__start_subtest(c"perf_kprobe".as_ptr()) {
        test_missed_perf_kprobe();
    }
    if test__start_subtest(c"kprobe_recursion".as_ptr()) {
        test_missed_kprobe_recursion();
    }
    if test__start_subtest(c"tp_recursion".as_ptr()) {
        test_missed_tp_recursion();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
