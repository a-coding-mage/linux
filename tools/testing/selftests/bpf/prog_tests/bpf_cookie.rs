// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
/* Translated from C source. Original includes:
 * pthread.h, sched.h, sys/syscall.h, sys/mman.h, unistd.h,
 * linux/compiler.h, test_progs.h, network_helpers.h, bpf/btf.h,
 * test_bpf_cookie.skel.h, kprobe_multi.skel.h, uprobe_multi.skel.h
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_ulonglong, c_void};
use core::mem;
use core::ptr;

type __u64 = u64;
type u64 = u64;
type u32 = u32;
type ssize_t = isize;
type pid_t = c_int;

const SYS_NANOSLEEP_KPROBE_NAME: *const c_char = c"sys_nanosleep".as_ptr();
const BPF_TRACE_KPROBE_MULTI: c_int = 0;
const BPF_F_KPROBE_MULTI_RETURN: u32 = 1;
const BPF_TRACE_FENTRY: c_int = 0;
const BPF_TRACE_FEXIT: c_int = 0;
const BPF_MODIFY_RETURN: c_int = 0;
const BPF_LSM_MAC: c_int = 0;
const BPF_TRACE_RAW_TP: c_int = 0;
const BPF_LINK_TYPE_TRACING: u32 = 0;
const BPF_LINK_TYPE_RAW_TRACEPOINT: u32 = 0;
const PERF_TYPE_SOFTWARE: u32 = 1;
const PERF_COUNT_SW_CPU_CLOCK: u64 = 0;
const PERF_FLAG_FD_CLOEXEC: c_ulong = 8;
const __NR_perf_event_open: c_long = 298;
const EPERM: c_int = 1;

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_kprobe_opts {
    bpf_cookie: __u64,
    retprobe: bool,
}

#[repr(C)]
struct bpf_kprobe_multi_opts {
    syms: *const *const c_char,
    cnt: usize,
    cookies: *const __u64,
    retprobe: bool,
}

#[repr(C)]
struct bpf_uprobe_opts {
    bpf_cookie: __u64,
    retprobe: bool,
}

#[repr(C)]
struct bpf_uprobe_multi_opts {
    syms: *const *const c_char,
    cnt: usize,
    cookies: *const __u64,
    retprobe: bool,
}

#[repr(C)]
struct bpf_tracepoint_opts {
    bpf_cookie: __u64,
}

#[repr(C)]
struct bpf_perf_event_opts {
    bpf_cookie: __u64,
}

#[repr(C)]
struct bpf_test_run_opts {
    retval: u32,
}

#[repr(C)]
struct bpf_link_create_opts {
    kprobe_multi: bpf_link_create_kprobe_multi_opts,
    tracing: bpf_link_create_tracing_opts,
}

#[repr(C)]
struct bpf_link_create_kprobe_multi_opts {
    addrs: *const c_ulong,
    cnt: usize,
    cookies: *const __u64,
    flags: u32,
}

#[repr(C)]
struct bpf_link_create_tracing_opts {
    cookie: __u64,
}

#[repr(C)]
struct bpf_raw_tp_opts {
    tp_name: *const c_char,
    cookie: __u64,
}

#[repr(C)]
struct bpf_trace_opts {
    cookie: __u64,
}

#[repr(C)]
struct bpf_raw_tracepoint_opts {
    cookie: __u64,
}

#[repr(C)]
struct perf_event_attr {
    size: u32,
    type_: u32,
    config: u64,
    sample_period: u64,
}

#[repr(C)]
struct bpf_link_info {
    type_: u32,
    tracing: bpf_link_info_tracing,
    raw_tracepoint: bpf_link_info_raw_tracepoint,
}

#[repr(C)]
struct bpf_link_info_tracing {
    cookie: __u64,
}

#[repr(C)]
struct bpf_link_info_raw_tracepoint {
    cookie: __u64,
}

#[repr(C)]
struct test_bpf_cookie {
    progs: test_bpf_cookie_progs,
    bss: *mut test_bpf_cookie_bss,
}

#[repr(C)]
struct test_bpf_cookie_progs {
    handle_kprobe: *mut bpf_program,
    handle_kretprobe: *mut bpf_program,
    handle_uprobe: *mut bpf_program,
    handle_uretprobe: *mut bpf_program,
    handle_tp1: *mut bpf_program,
    handle_tp2: *mut bpf_program,
    handle_tp3: *mut bpf_program,
    handle_pe: *mut bpf_program,
    fentry_test1: *mut bpf_program,
    fexit_test1: *mut bpf_program,
    fmod_ret_test: *mut bpf_program,
    test_int_hook: *mut bpf_program,
    handle_tp_btf: *mut bpf_program,
    handle_raw_tp: *mut bpf_program,
}

#[repr(C)]
struct test_bpf_cookie_bss {
    my_tid: c_int,
    kprobe_res: __u64,
    kretprobe_res: __u64,
    uprobe_res: __u64,
    uretprobe_res: __u64,
    tp_res: __u64,
    pe_res: __u64,
    fentry_res: __u64,
    fexit_res: __u64,
    fmod_ret_res: __u64,
    lsm_res: __u64,
    tp_btf_res: __u64,
    raw_tp_res: __u64,
}

#[repr(C)]
struct kprobe_multi {
    progs: kprobe_multi_progs,
    bss: *mut kprobe_multi_bss,
}

#[repr(C)]
struct kprobe_multi_progs {
    trigger: *mut bpf_program,
    test_kprobe: *mut bpf_program,
    test_kretprobe: *mut bpf_program,
}

#[repr(C)]
struct kprobe_multi_bss {
    pid: c_int,
    test_cookie: bool,
    kprobe_test1_result: c_int,
    kprobe_test2_result: c_int,
    kprobe_test3_result: c_int,
    kprobe_test4_result: c_int,
    kprobe_test5_result: c_int,
    kprobe_test6_result: c_int,
    kprobe_test7_result: c_int,
    kprobe_test8_result: c_int,
    kretprobe_test1_result: c_int,
    kretprobe_test2_result: c_int,
    kretprobe_test3_result: c_int,
    kretprobe_test4_result: c_int,
    kretprobe_test5_result: c_int,
    kretprobe_test6_result: c_int,
    kretprobe_test7_result: c_int,
    kretprobe_test8_result: c_int,
}

#[repr(C)]
struct uprobe_multi {
    progs: uprobe_multi_progs,
    bss: *mut uprobe_multi_bss,
}

#[repr(C)]
struct uprobe_multi_progs {
    uprobe: *mut bpf_program,
    uretprobe: *mut bpf_program,
}

#[repr(C)]
struct uprobe_multi_bss {
    uprobe_multi_func_1_addr: __u64,
    uprobe_multi_func_2_addr: __u64,
    uprobe_multi_func_3_addr: __u64,
    pid: c_int,
    test_cookie: bool,
    uprobe_multi_func_1_result: c_int,
    uprobe_multi_func_2_result: c_int,
    uprobe_multi_func_3_result: c_int,
    uretprobe_multi_func_1_result: c_int,
    uretprobe_multi_func_2_result: c_int,
    uretprobe_multi_func_3_result: c_int,
}

#[repr(C)]
struct test_env {
    has_testmod: bool,
}

unsafe extern "C" {
    static env: test_env;
    static mut errno: c_int;

    fn bpf_program__attach_kprobe_opts(
        prog: *mut bpf_program,
        func_name: *const c_char,
        opts: *const bpf_kprobe_opts,
    ) -> *mut bpf_link;
    fn bpf_program__attach_kprobe_multi_opts(
        prog: *mut bpf_program,
        pattern: *const c_char,
        opts: *const bpf_kprobe_multi_opts,
    ) -> *mut bpf_link;
    fn bpf_program__attach_uprobe_opts(
        prog: *mut bpf_program,
        pid: pid_t,
        binary_path: *const c_char,
        func_offset: usize,
        opts: *const bpf_uprobe_opts,
    ) -> *mut bpf_link;
    fn bpf_program__attach_uprobe_multi(
        prog: *mut bpf_program,
        pid: pid_t,
        binary_path: *const c_char,
        func_pattern: *const c_char,
        opts: *const bpf_uprobe_multi_opts,
    ) -> *mut bpf_link;
    fn bpf_program__attach_tracepoint_opts(
        prog: *mut bpf_program,
        tp_category: *const c_char,
        tp_name: *const c_char,
        opts: *const bpf_tracepoint_opts,
    ) -> *mut bpf_link;
    fn bpf_program__attach_perf_event_opts(
        prog: *mut bpf_program,
        pfd: c_int,
        opts: *const bpf_perf_event_opts,
    ) -> *mut bpf_link;
    fn bpf_program__attach_trace_opts(
        prog: *mut bpf_program,
        opts: *const bpf_trace_opts,
    ) -> *mut bpf_link;
    fn bpf_program__attach_raw_tracepoint_opts(
        prog: *mut bpf_program,
        tp_name: *const c_char,
        opts: *const bpf_raw_tracepoint_opts,
    ) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_link__disconnect(link: *mut bpf_link);
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_link_create(prog_fd: c_int, target_fd: c_int, attach_type: c_int, opts: *const bpf_link_create_opts) -> c_int;
    fn bpf_raw_tracepoint_open_opts(prog_fd: c_int, opts: *const bpf_raw_tp_opts) -> c_int;
    fn bpf_link_get_info_by_fd(fd: c_int, info: *mut bpf_link_info, info_len: *mut u32) -> c_int;

    fn test_bpf_cookie__open_and_load() -> *mut test_bpf_cookie;
    fn test_bpf_cookie__destroy(skel: *mut test_bpf_cookie);
    fn kprobe_multi__open_and_load() -> *mut kprobe_multi;
    fn kprobe_multi__destroy(skel: *mut kprobe_multi);
    fn uprobe_multi__open_and_load() -> *mut uprobe_multi;
    fn uprobe_multi__destroy(skel: *mut uprobe_multi);

    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: u64, expected: u64, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: u64, expected: u64, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_long, expected: c_long, name: *const c_char) -> bool;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;
    fn load_kallsyms() -> c_int;
    fn ksym_get_addr(sym: *const c_char) -> c_ulonglong;
    fn get_uprobe_offset(func: *const c_void) -> ssize_t;
    fn kern_sync_rcu();
    fn stack_mprotect() -> c_int;
    fn sys_gettid() -> c_int;
    fn getpid() -> c_int;
    fn usleep(usec: u32) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn pthread_self() -> c_ulong;
    fn pthread_setaffinity_np(thread: c_ulong, cpusetsize: usize, cpuset: *const cpu_set_t) -> c_int;
}

#[repr(C)]
struct cpu_set_t {
    bits: [c_ulong; 16],
}

unsafe fn ASSERT_OK_PTR_LINK(ptr: *mut bpf_link, name: *const c_char) -> bool {
    ASSERT_OK_PTR(ptr as *mut c_void, name)
}

unsafe fn ASSERT_OK_PTR_TEST_BPF_COOKIE(ptr: *mut test_bpf_cookie, name: *const c_char) -> bool {
    ASSERT_OK_PTR(ptr as *mut c_void, name)
}

unsafe fn ASSERT_OK_PTR_KPROBE_MULTI(ptr: *mut kprobe_multi, name: *const c_char) -> bool {
    ASSERT_OK_PTR(ptr as *mut c_void, name)
}

unsafe fn ASSERT_OK_PTR_UPROBE_MULTI(ptr: *mut uprobe_multi, name: *const c_char) -> bool {
    ASSERT_OK_PTR(ptr as *mut c_void, name)
}

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    (*set).bits = [0; 16];
}

unsafe fn CPU_SET(cpu: usize, set: *mut cpu_set_t) {
    let bits_per_word = 8 * mem::size_of::<c_ulong>();
    (*set).bits[cpu / bits_per_word] |= 1 as c_ulong << (cpu % bits_per_word);
}

fn barrier() {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

/* uprobe attach point */
#[inline(never)]
unsafe fn trigger_func() {
    core::arch::asm!("", options(nomem, nostack, preserves_flags));
}

unsafe fn kprobe_subtest(skel: *mut test_bpf_cookie) {
    let mut opts: bpf_kprobe_opts = mem::zeroed();
    let mut link1: *mut bpf_link = ptr::null_mut();
    let mut link2: *mut bpf_link = ptr::null_mut();
    let mut retlink1: *mut bpf_link = ptr::null_mut();
    let mut retlink2: *mut bpf_link = ptr::null_mut();

    /* attach two kprobes */
    opts.bpf_cookie = 0x1;
    opts.retprobe = false;
    link1 = bpf_program__attach_kprobe_opts((*skel).progs.handle_kprobe, SYS_NANOSLEEP_KPROBE_NAME, &opts);
    if !ASSERT_OK_PTR_LINK(link1, c"link1".as_ptr()) {
        goto_kprobe_cleanup(link1, link2, retlink1, retlink2);
        return;
    }

    opts.bpf_cookie = 0x2;
    opts.retprobe = false;
    link2 = bpf_program__attach_kprobe_opts((*skel).progs.handle_kprobe, SYS_NANOSLEEP_KPROBE_NAME, &opts);
    if !ASSERT_OK_PTR_LINK(link2, c"link2".as_ptr()) {
        goto_kprobe_cleanup(link1, link2, retlink1, retlink2);
        return;
    }

    /* attach two kretprobes */
    opts.bpf_cookie = 0x10;
    opts.retprobe = true;
    retlink1 = bpf_program__attach_kprobe_opts((*skel).progs.handle_kretprobe, SYS_NANOSLEEP_KPROBE_NAME, &opts);
    if !ASSERT_OK_PTR_LINK(retlink1, c"retlink1".as_ptr()) {
        goto_kprobe_cleanup(link1, link2, retlink1, retlink2);
        return;
    }

    opts.bpf_cookie = 0x20;
    opts.retprobe = true;
    retlink2 = bpf_program__attach_kprobe_opts((*skel).progs.handle_kretprobe, SYS_NANOSLEEP_KPROBE_NAME, &opts);
    if !ASSERT_OK_PTR_LINK(retlink2, c"retlink2".as_ptr()) {
        goto_kprobe_cleanup(link1, link2, retlink1, retlink2);
        return;
    }

    /* trigger kprobe && kretprobe */
    usleep(1);

    ASSERT_EQ((*(*skel).bss).kprobe_res, 0x1 | 0x2, c"kprobe_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).kretprobe_res, 0x10 | 0x20, c"kretprobe_res".as_ptr());

    goto_kprobe_cleanup(link1, link2, retlink1, retlink2);
}

unsafe fn goto_kprobe_cleanup(link1: *mut bpf_link, link2: *mut bpf_link, retlink1: *mut bpf_link, retlink2: *mut bpf_link) {
    bpf_link__destroy(link1);
    bpf_link__destroy(link2);
    bpf_link__destroy(retlink1);
    bpf_link__destroy(retlink2);
}

unsafe fn kprobe_multi_test_run(skel: *mut kprobe_multi) {
    let mut topts: bpf_test_run_opts = mem::zeroed();
    let mut err: c_int;
    let mut prog_fd: c_int;

    prog_fd = bpf_program__fd((*skel).progs.trigger);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_EQ(topts.retval as u64, 0, c"test_run".as_ptr());

    ASSERT_EQ((*(*skel).bss).kprobe_test1_result as u64, 1, c"kprobe_test1_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).kprobe_test2_result as u64, 1, c"kprobe_test2_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).kprobe_test3_result as u64, 1, c"kprobe_test3_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).kprobe_test4_result as u64, 1, c"kprobe_test4_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).kprobe_test5_result as u64, 1, c"kprobe_test5_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).kprobe_test6_result as u64, 1, c"kprobe_test6_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).kprobe_test7_result as u64, 1, c"kprobe_test7_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).kprobe_test8_result as u64, 1, c"kprobe_test8_result".as_ptr());

    ASSERT_EQ((*(*skel).bss).kretprobe_test1_result as u64, 1, c"kretprobe_test1_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).kretprobe_test2_result as u64, 1, c"kretprobe_test2_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).kretprobe_test3_result as u64, 1, c"kretprobe_test3_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).kretprobe_test4_result as u64, 1, c"kretprobe_test4_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).kretprobe_test5_result as u64, 1, c"kretprobe_test5_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).kretprobe_test6_result as u64, 1, c"kretprobe_test6_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).kretprobe_test7_result as u64, 1, c"kretprobe_test7_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).kretprobe_test8_result as u64, 1, c"kretprobe_test8_result".as_ptr());
}

unsafe fn kprobe_multi_link_api_subtest() {
    let mut prog_fd: c_int;
    let mut link1_fd: c_int = -1;
    let mut link2_fd: c_int = -1;
    let mut skel: *mut kprobe_multi = ptr::null_mut();
    let mut opts: bpf_link_create_opts = mem::zeroed();
    let mut addrs: [c_ulonglong; 8] = [0; 8];
    let mut cookies: [__u64; 8] = [0; 8];

    if !env.has_testmod {
        test__skip();
        return;
    }

    if !ASSERT_OK(load_kallsyms(), c"load_kallsyms".as_ptr()) {
        close(link1_fd);
        close(link2_fd);
        kprobe_multi__destroy(skel);
        return;
    }

    skel = kprobe_multi__open_and_load();
    if !ASSERT_OK_PTR_KPROBE_MULTI(skel, c"fentry_raw_skel_load".as_ptr()) {
        close(link1_fd);
        close(link2_fd);
        kprobe_multi__destroy(skel);
        return;
    }

    (*(*skel).bss).pid = getpid();
    (*(*skel).bss).test_cookie = true;

    macro_rules! get_addr {
        ($sym:literal, $addr:expr) => {{
            $addr = ksym_get_addr(c_str!($sym).as_ptr());
            if !ASSERT_NEQ($addr as u64, 0, c"ksym_get_addr".as_ptr()) {
                close(link1_fd);
                close(link2_fd);
                kprobe_multi__destroy(skel);
                return;
            }
        }};
    }

    get_addr!("bpf_fentry_test1", addrs[0]);
    get_addr!("bpf_fentry_test3", addrs[1]);
    get_addr!("bpf_fentry_test4", addrs[2]);
    get_addr!("bpf_fentry_test5", addrs[3]);
    get_addr!("bpf_fentry_test6", addrs[4]);
    get_addr!("bpf_fentry_test7", addrs[5]);
    get_addr!("bpf_fentry_test2", addrs[6]);
    get_addr!("bpf_fentry_test8", addrs[7]);

    cookies[0] = 1; /* bpf_fentry_test1 */
    cookies[1] = 2; /* bpf_fentry_test3 */
    cookies[2] = 3; /* bpf_fentry_test4 */
    cookies[3] = 4; /* bpf_fentry_test5 */
    cookies[4] = 5; /* bpf_fentry_test6 */
    cookies[5] = 6; /* bpf_fentry_test7 */
    cookies[6] = 7; /* bpf_fentry_test2 */
    cookies[7] = 8; /* bpf_fentry_test8 */

    opts.kprobe_multi.addrs = addrs.as_ptr() as *const c_ulong;
    opts.kprobe_multi.cnt = addrs.len();
    opts.kprobe_multi.cookies = cookies.as_ptr();
    prog_fd = bpf_program__fd((*skel).progs.test_kprobe);

    link1_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_KPROBE_MULTI, &opts);
    if !ASSERT_GE(link1_fd as c_long, 0, c"link1_fd".as_ptr()) {
        close(link1_fd);
        close(link2_fd);
        kprobe_multi__destroy(skel);
        return;
    }

    cookies[0] = 8; /* bpf_fentry_test1 */
    cookies[1] = 7; /* bpf_fentry_test3 */
    cookies[2] = 6; /* bpf_fentry_test4 */
    cookies[3] = 5; /* bpf_fentry_test5 */
    cookies[4] = 4; /* bpf_fentry_test6 */
    cookies[5] = 3; /* bpf_fentry_test7 */
    cookies[6] = 2; /* bpf_fentry_test2 */
    cookies[7] = 1; /* bpf_fentry_test8 */

    opts.kprobe_multi.flags = BPF_F_KPROBE_MULTI_RETURN;
    prog_fd = bpf_program__fd((*skel).progs.test_kretprobe);

    link2_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_KPROBE_MULTI, &opts);
    if !ASSERT_GE(link2_fd as c_long, 0, c"link2_fd".as_ptr()) {
        close(link1_fd);
        close(link2_fd);
        kprobe_multi__destroy(skel);
        return;
    }

    kprobe_multi_test_run(skel);

    close(link1_fd);
    close(link2_fd);
    kprobe_multi__destroy(skel);
}

macro_rules! c_str {
    ($lit:literal) => {
        concat!($lit, "\0")
    };
}

unsafe fn kprobe_multi_attach_api_subtest() {
    let mut link1: *mut bpf_link = ptr::null_mut();
    let mut link2: *mut bpf_link = ptr::null_mut();
    let mut opts: bpf_kprobe_multi_opts = mem::zeroed();
    let mut _topts: bpf_test_run_opts = mem::zeroed();
    let mut skel: *mut kprobe_multi = ptr::null_mut();
    let syms: [*const c_char; 8] = [
        c"bpf_fentry_test1".as_ptr(),
        c"bpf_fentry_test3".as_ptr(),
        c"bpf_fentry_test4".as_ptr(),
        c"bpf_fentry_test5".as_ptr(),
        c"bpf_fentry_test6".as_ptr(),
        c"bpf_fentry_test7".as_ptr(),
        c"bpf_fentry_test2".as_ptr(),
        c"bpf_fentry_test8".as_ptr(),
    ];
    let mut cookies: [__u64; 8] = [0; 8];

    if !env.has_testmod {
        test__skip();
        return;
    }

    skel = kprobe_multi__open_and_load();
    if !ASSERT_OK_PTR_KPROBE_MULTI(skel, c"fentry_raw_skel_load".as_ptr()) {
        bpf_link__destroy(link2);
        bpf_link__destroy(link1);
        kprobe_multi__destroy(skel);
        return;
    }

    (*(*skel).bss).pid = getpid();
    (*(*skel).bss).test_cookie = true;

    cookies[0] = 1; cookies[1] = 2; cookies[2] = 3; cookies[3] = 4;
    cookies[4] = 5; cookies[5] = 6; cookies[6] = 7; cookies[7] = 8;

    opts.syms = syms.as_ptr();
    opts.cnt = syms.len();
    opts.cookies = cookies.as_ptr();

    link1 = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kprobe, ptr::null(), &opts);
    if !ASSERT_OK_PTR_LINK(link1, c"bpf_program__attach_kprobe_multi_opts".as_ptr()) {
        bpf_link__destroy(link2);
        bpf_link__destroy(link1);
        kprobe_multi__destroy(skel);
        return;
    }

    cookies[0] = 8; cookies[1] = 7; cookies[2] = 6; cookies[3] = 5;
    cookies[4] = 4; cookies[5] = 3; cookies[6] = 2; cookies[7] = 1;

    opts.retprobe = true;
    link2 = bpf_program__attach_kprobe_multi_opts((*skel).progs.test_kretprobe, ptr::null(), &opts);
    if !ASSERT_OK_PTR_LINK(link2, c"bpf_program__attach_kprobe_multi_opts".as_ptr()) {
        bpf_link__destroy(link2);
        bpf_link__destroy(link1);
        kprobe_multi__destroy(skel);
        return;
    }

    kprobe_multi_test_run(skel);

    bpf_link__destroy(link2);
    bpf_link__destroy(link1);
    kprobe_multi__destroy(skel);
}

/*
 * Weak uprobe target stubs. noinline is required because
 * uprobe_multi_test_run() takes their addresses to configure the BPF
 * program's attachment points; an inlined function has no stable
 * address in the binary to probe. The strong definitions in
 * uprobe_multi_test.c take precedence when that translation unit is
 * linked.
 */
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn uprobe_multi_func_1() { core::arch::asm!("", options(nomem, nostack, preserves_flags)); }
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn uprobe_multi_func_2() { core::arch::asm!("", options(nomem, nostack, preserves_flags)); }
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn uprobe_multi_func_3() { core::arch::asm!("", options(nomem, nostack, preserves_flags)); }

unsafe fn uprobe_multi_test_run(skel: *mut uprobe_multi) {
    (*(*skel).bss).uprobe_multi_func_1_addr = uprobe_multi_func_1 as usize as __u64;
    (*(*skel).bss).uprobe_multi_func_2_addr = uprobe_multi_func_2 as usize as __u64;
    (*(*skel).bss).uprobe_multi_func_3_addr = uprobe_multi_func_3 as usize as __u64;

    (*(*skel).bss).pid = getpid();
    (*(*skel).bss).test_cookie = true;

    uprobe_multi_func_1();
    uprobe_multi_func_2();
    uprobe_multi_func_3();

    ASSERT_EQ((*(*skel).bss).uprobe_multi_func_1_result as u64, 1, c"uprobe_multi_func_1_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).uprobe_multi_func_2_result as u64, 1, c"uprobe_multi_func_2_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).uprobe_multi_func_3_result as u64, 1, c"uprobe_multi_func_3_result".as_ptr());

    ASSERT_EQ((*(*skel).bss).uretprobe_multi_func_1_result as u64, 1, c"uretprobe_multi_func_1_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).uretprobe_multi_func_2_result as u64, 1, c"uretprobe_multi_func_2_result".as_ptr());
    ASSERT_EQ((*(*skel).bss).uretprobe_multi_func_3_result as u64, 1, c"uretprobe_multi_func_3_result".as_ptr());
}

unsafe fn uprobe_multi_attach_api_subtest() {
    let mut link1: *mut bpf_link = ptr::null_mut();
    let mut link2: *mut bpf_link = ptr::null_mut();
    let mut skel: *mut uprobe_multi = ptr::null_mut();
    let mut opts: bpf_uprobe_multi_opts = mem::zeroed();
    let syms: [*const c_char; 3] = [
        c"uprobe_multi_func_1".as_ptr(),
        c"uprobe_multi_func_2".as_ptr(),
        c"uprobe_multi_func_3".as_ptr(),
    ];
    let mut cookies: [__u64; 3] = [0; 3];

    cookies[0] = 3; /* uprobe_multi_func_1 */
    cookies[1] = 1; /* uprobe_multi_func_2 */
    cookies[2] = 2; /* uprobe_multi_func_3 */

    opts.syms = syms.as_ptr();
    opts.cnt = syms.len();
    opts.cookies = cookies.as_ptr();

    skel = uprobe_multi__open_and_load();
    if !ASSERT_OK_PTR_UPROBE_MULTI(skel, c"uprobe_multi".as_ptr()) {
        bpf_link__destroy(link2);
        bpf_link__destroy(link1);
        uprobe_multi__destroy(skel);
        return;
    }

    link1 = bpf_program__attach_uprobe_multi((*skel).progs.uprobe, -1, c"/proc/self/exe".as_ptr(), ptr::null(), &opts);
    if !ASSERT_OK_PTR_LINK(link1, c"bpf_program__attach_uprobe_multi".as_ptr()) {
        bpf_link__destroy(link2);
        bpf_link__destroy(link1);
        uprobe_multi__destroy(skel);
        return;
    }

    cookies[0] = 2; /* uprobe_multi_func_1 */
    cookies[1] = 3; /* uprobe_multi_func_2 */
    cookies[2] = 1; /* uprobe_multi_func_3 */

    opts.retprobe = true;
    link2 = bpf_program__attach_uprobe_multi((*skel).progs.uretprobe, -1, c"/proc/self/exe".as_ptr(), ptr::null(), &opts);
    if !ASSERT_OK_PTR_LINK(link2, c"bpf_program__attach_uprobe_multi_retprobe".as_ptr()) {
        bpf_link__destroy(link2);
        bpf_link__destroy(link1);
        uprobe_multi__destroy(skel);
        return;
    }

    uprobe_multi_test_run(skel);

    bpf_link__destroy(link2);
    bpf_link__destroy(link1);
    uprobe_multi__destroy(skel);
}

unsafe fn uprobe_subtest(skel: *mut test_bpf_cookie) {
    let mut opts: bpf_uprobe_opts = mem::zeroed();
    let mut link1: *mut bpf_link = ptr::null_mut();
    let mut link2: *mut bpf_link = ptr::null_mut();
    let mut retlink1: *mut bpf_link = ptr::null_mut();
    let mut retlink2: *mut bpf_link = ptr::null_mut();
    let uprobe_offset: ssize_t;

    uprobe_offset = get_uprobe_offset(trigger_func as *const c_void);
    if !ASSERT_GE(uprobe_offset as c_long, 0, c"uprobe_offset".as_ptr()) {
        bpf_link__destroy(link1); bpf_link__destroy(link2); bpf_link__destroy(retlink1); bpf_link__destroy(retlink2);
        return;
    }

    /* attach two uprobes */
    opts.bpf_cookie = 0x100;
    opts.retprobe = false;
    link1 = bpf_program__attach_uprobe_opts((*skel).progs.handle_uprobe, 0 /* self pid */, c"/proc/self/exe".as_ptr(), uprobe_offset as usize, &opts);
    if !ASSERT_OK_PTR_LINK(link1, c"link1".as_ptr()) {
        bpf_link__destroy(link1); bpf_link__destroy(link2); bpf_link__destroy(retlink1); bpf_link__destroy(retlink2);
        return;
    }

    opts.bpf_cookie = 0x200;
    opts.retprobe = false;
    link2 = bpf_program__attach_uprobe_opts((*skel).progs.handle_uprobe, -1 /* any pid */, c"/proc/self/exe".as_ptr(), uprobe_offset as usize, &opts);
    if !ASSERT_OK_PTR_LINK(link2, c"link2".as_ptr()) {
        bpf_link__destroy(link1); bpf_link__destroy(link2); bpf_link__destroy(retlink1); bpf_link__destroy(retlink2);
        return;
    }

    /* attach two uretprobes */
    opts.bpf_cookie = 0x1000;
    opts.retprobe = true;
    retlink1 = bpf_program__attach_uprobe_opts((*skel).progs.handle_uretprobe, -1 /* any pid */, c"/proc/self/exe".as_ptr(), uprobe_offset as usize, &opts);
    if !ASSERT_OK_PTR_LINK(retlink1, c"retlink1".as_ptr()) {
        bpf_link__destroy(link1); bpf_link__destroy(link2); bpf_link__destroy(retlink1); bpf_link__destroy(retlink2);
        return;
    }

    opts.bpf_cookie = 0x2000;
    opts.retprobe = true;
    retlink2 = bpf_program__attach_uprobe_opts((*skel).progs.handle_uretprobe, 0 /* self pid */, c"/proc/self/exe".as_ptr(), uprobe_offset as usize, &opts);
    if !ASSERT_OK_PTR_LINK(retlink2, c"retlink2".as_ptr()) {
        bpf_link__destroy(link1); bpf_link__destroy(link2); bpf_link__destroy(retlink1); bpf_link__destroy(retlink2);
        return;
    }

    /* trigger uprobe && uretprobe */
    trigger_func();

    ASSERT_EQ((*(*skel).bss).uprobe_res, 0x100 | 0x200, c"uprobe_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).uretprobe_res, 0x1000 | 0x2000, c"uretprobe_res".as_ptr());

    bpf_link__destroy(link1);
    bpf_link__destroy(link2);
    bpf_link__destroy(retlink1);
    bpf_link__destroy(retlink2);
}

unsafe fn tp_subtest(skel: *mut test_bpf_cookie) {
    let mut opts: bpf_tracepoint_opts = mem::zeroed();
    let mut link1: *mut bpf_link = ptr::null_mut();
    let mut link2: *mut bpf_link = ptr::null_mut();
    let mut link3: *mut bpf_link = ptr::null_mut();

    /* attach first tp prog */
    opts.bpf_cookie = 0x10000;
    link1 = bpf_program__attach_tracepoint_opts((*skel).progs.handle_tp1, c"syscalls".as_ptr(), c"sys_enter_nanosleep".as_ptr(), &opts);
    if !ASSERT_OK_PTR_LINK(link1, c"link1".as_ptr()) {
        bpf_link__destroy(link1); bpf_link__destroy(link2); bpf_link__destroy(link3);
        return;
    }

    /* attach second tp prog */
    opts.bpf_cookie = 0x20000;
    link2 = bpf_program__attach_tracepoint_opts((*skel).progs.handle_tp2, c"syscalls".as_ptr(), c"sys_enter_nanosleep".as_ptr(), &opts);
    if !ASSERT_OK_PTR_LINK(link2, c"link2".as_ptr()) {
        bpf_link__destroy(link1); bpf_link__destroy(link2); bpf_link__destroy(link3);
        return;
    }

    /* trigger tracepoints */
    usleep(1);

    ASSERT_EQ((*(*skel).bss).tp_res, 0x10000 | 0x20000, c"tp_res1".as_ptr());

    /* now we detach first prog and will attach third one, which causes
     * two internal calls to bpf_prog_array_copy(), shuffling
     * bpf_prog_array_items around. We test here that we don't lose track
     * of associated bpf_cookies.
     */
    bpf_link__destroy(link1);
    link1 = ptr::null_mut();
    kern_sync_rcu();
    (*(*skel).bss).tp_res = 0;

    /* attach third tp prog */
    opts.bpf_cookie = 0x40000;
    link3 = bpf_program__attach_tracepoint_opts((*skel).progs.handle_tp3, c"syscalls".as_ptr(), c"sys_enter_nanosleep".as_ptr(), &opts);
    if !ASSERT_OK_PTR_LINK(link3, c"link3".as_ptr()) {
        bpf_link__destroy(link1); bpf_link__destroy(link2); bpf_link__destroy(link3);
        return;
    }

    /* trigger tracepoints */
    usleep(1);

    ASSERT_EQ((*(*skel).bss).tp_res, 0x20000 | 0x40000, c"tp_res2".as_ptr());

    bpf_link__destroy(link1);
    bpf_link__destroy(link2);
    bpf_link__destroy(link3);
}

unsafe fn burn_cpu(loops: c_long) {
    let mut j: c_long = 0;
    let mut cpu_set: cpu_set_t = mem::zeroed();
    let mut i: c_long;
    let err: c_int;

    /* generate some branches on cpu 0 */
    CPU_ZERO(&mut cpu_set);
    CPU_SET(0, &mut cpu_set);
    err = pthread_setaffinity_np(pthread_self(), mem::size_of::<cpu_set_t>(), &cpu_set);
    ASSERT_OK(err, c"set_thread_affinity".as_ptr());

    i = 0;
    while i < loops {
        j += 1;
        barrier();
        i += 1;
    }
    let _ = j;
}

unsafe fn pe_subtest(skel: *mut test_bpf_cookie) {
    let mut opts: bpf_perf_event_opts = mem::zeroed();
    let mut link: *mut bpf_link = ptr::null_mut();
    let mut attr: perf_event_attr = mem::zeroed();
    let mut pfd: c_int = -1;

    /* create perf event */
    attr.size = mem::size_of::<perf_event_attr>() as u32;
    attr.type_ = PERF_TYPE_SOFTWARE;
    attr.config = PERF_COUNT_SW_CPU_CLOCK;
    attr.sample_period = 100000;
    pfd = syscall(__NR_perf_event_open, &attr as *const perf_event_attr, 0, -1, -1, PERF_FLAG_FD_CLOEXEC) as c_int;
    if !ASSERT_GE(pfd as c_long, 0, c"perf_fd".as_ptr()) {
        close(pfd); bpf_link__destroy(link);
        return;
    }

    opts.bpf_cookie = 0x100000;
    link = bpf_program__attach_perf_event_opts((*skel).progs.handle_pe, pfd, &opts);
    if !ASSERT_OK_PTR_LINK(link, c"link1".as_ptr()) {
        close(pfd); bpf_link__destroy(link);
        return;
    }

    burn_cpu(100000000); /* trigger BPF prog */

    ASSERT_EQ((*(*skel).bss).pe_res, 0x100000, c"pe_res1".as_ptr());

    /* prevent bpf_link__destroy() closing pfd itself */
    bpf_link__disconnect(link);
    /* close BPF link's FD explicitly */
    close(bpf_link__fd(link));
    /* free up memory used by struct bpf_link */
    bpf_link__destroy(link);
    link = ptr::null_mut();
    kern_sync_rcu();
    (*(*skel).bss).pe_res = 0;

    opts.bpf_cookie = 0x200000;
    link = bpf_program__attach_perf_event_opts((*skel).progs.handle_pe, pfd, &opts);
    if !ASSERT_OK_PTR_LINK(link, c"link2".as_ptr()) {
        close(pfd); bpf_link__destroy(link);
        return;
    }

    burn_cpu(100000000); /* trigger BPF prog */

    ASSERT_EQ((*(*skel).bss).pe_res, 0x200000, c"pe_res2".as_ptr());

    close(pfd);
    bpf_link__destroy(link);
}

unsafe fn verify_tracing_link_info(fd: c_int, cookie: u64) -> c_int {
    let mut info: bpf_link_info = mem::zeroed();
    let mut err: c_int;
    let mut len: u32 = mem::size_of::<bpf_link_info>() as u32;

    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    if !ASSERT_OK(err, c"get_link_info".as_ptr()) {
        return -1;
    }

    if !ASSERT_EQ(info.type_ as u64, BPF_LINK_TYPE_TRACING as u64, c"link_type".as_ptr()) {
        return -1;
    }

    ASSERT_EQ(info.tracing.cookie, cookie, c"tracing_cookie".as_ptr());

    0
}

unsafe fn tracing_subtest(skel: *mut test_bpf_cookie) {
    let mut cookie: __u64;
    let mut prog_fd: c_int;
    let mut err: c_int;
    let mut fentry_fd: c_int = -1;
    let mut fexit_fd: c_int = -1;
    let mut fmod_ret_fd: c_int = -1;
    let mut opts: bpf_test_run_opts = mem::zeroed();
    let mut link_opts: bpf_link_create_opts = mem::zeroed();

    (*(*skel).bss).fentry_res = 0;
    (*(*skel).bss).fexit_res = 0;

    cookie = 0x10000000000000;
    prog_fd = bpf_program__fd((*skel).progs.fentry_test1);
    link_opts.tracing.cookie = cookie;
    fentry_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_FENTRY, &link_opts);
    if !ASSERT_GE(fentry_fd as c_long, 0, c"fentry.link_create".as_ptr()) {
        if fentry_fd >= 0 { close(fentry_fd); }
        if fexit_fd >= 0 { close(fexit_fd); }
        if fmod_ret_fd >= 0 { close(fmod_ret_fd); }
        return;
    }

    err = verify_tracing_link_info(fentry_fd, cookie);
    if !ASSERT_OK(err, c"verify_tracing_link_info".as_ptr()) {
        if fentry_fd >= 0 { close(fentry_fd); }
        if fexit_fd >= 0 { close(fexit_fd); }
        if fmod_ret_fd >= 0 { close(fmod_ret_fd); }
        return;
    }

    cookie = 0x20000000000000;
    prog_fd = bpf_program__fd((*skel).progs.fexit_test1);
    link_opts.tracing.cookie = cookie;
    fexit_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_FEXIT, &link_opts);
    if !ASSERT_GE(fexit_fd as c_long, 0, c"fexit.link_create".as_ptr()) {
        if fentry_fd >= 0 { close(fentry_fd); }
        if fexit_fd >= 0 { close(fexit_fd); }
        if fmod_ret_fd >= 0 { close(fmod_ret_fd); }
        return;
    }

    cookie = 0x30000000000000;
    prog_fd = bpf_program__fd((*skel).progs.fmod_ret_test);
    link_opts.tracing.cookie = cookie;
    fmod_ret_fd = bpf_link_create(prog_fd, 0, BPF_MODIFY_RETURN, &link_opts);
    if !ASSERT_GE(fmod_ret_fd as c_long, 0, c"fmod_ret.link_create".as_ptr()) {
        if fentry_fd >= 0 { close(fentry_fd); }
        if fexit_fd >= 0 { close(fexit_fd); }
        if fmod_ret_fd >= 0 { close(fmod_ret_fd); }
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.fentry_test1);
    bpf_prog_test_run_opts(prog_fd, &mut opts);

    prog_fd = bpf_program__fd((*skel).progs.fmod_ret_test);
    bpf_prog_test_run_opts(prog_fd, &mut opts);

    ASSERT_EQ((*(*skel).bss).fentry_res, 0x10000000000000, c"fentry_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).fexit_res, 0x20000000000000, c"fexit_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).fmod_ret_res, 0x30000000000000, c"fmod_ret_res".as_ptr());

    if fentry_fd >= 0 { close(fentry_fd); }
    if fexit_fd >= 0 { close(fexit_fd); }
    if fmod_ret_fd >= 0 { close(fmod_ret_fd); }
}

unsafe fn lsm_subtest(skel: *mut test_bpf_cookie) {
    let mut cookie: __u64;
    let mut prog_fd: c_int;
    let mut lsm_fd: c_int = -1;
    let mut link_opts: bpf_link_create_opts = mem::zeroed();
    let mut err: c_int;

    (*(*skel).bss).lsm_res = 0;

    cookie = 0x90000000000090;
    prog_fd = bpf_program__fd((*skel).progs.test_int_hook);
    link_opts.tracing.cookie = cookie;
    lsm_fd = bpf_link_create(prog_fd, 0, BPF_LSM_MAC, &link_opts);
    if !ASSERT_GE(lsm_fd as c_long, 0, c"lsm.link_create".as_ptr()) {
        if lsm_fd >= 0 { close(lsm_fd); }
        return;
    }

    err = stack_mprotect();
    if !ASSERT_EQ(err as u64, (-1i32) as u64, c"stack_mprotect".as_ptr()) ||
       !ASSERT_EQ(errno as u64, EPERM as u64, c"stack_mprotect".as_ptr()) {
        if lsm_fd >= 0 { close(lsm_fd); }
        return;
    }

    usleep(1);

    ASSERT_EQ((*(*skel).bss).lsm_res, 0x90000000000090, c"fentry_res".as_ptr());

    if lsm_fd >= 0 { close(lsm_fd); }
}

unsafe fn tp_btf_subtest(skel: *mut test_bpf_cookie) {
    let mut cookie: __u64;
    let mut prog_fd: c_int;
    let mut link_fd: c_int = -1;
    let mut link: *mut bpf_link = ptr::null_mut();
    let mut link_opts: bpf_link_create_opts = mem::zeroed();
    let mut raw_tp_opts: bpf_raw_tp_opts = mem::zeroed();
    let mut trace_opts: bpf_trace_opts = mem::zeroed();

    /* There are three different ways to attach tp_btf (BTF-aware raw
     * tracepoint) programs. Let's test all of them.
     */
    prog_fd = bpf_program__fd((*skel).progs.handle_tp_btf);

    /* low-level BPF_RAW_TRACEPOINT_OPEN command wrapper */
    (*(*skel).bss).tp_btf_res = 0;

    cookie = 0x11000000000000;
    raw_tp_opts.cookie = cookie;
    link_fd = bpf_raw_tracepoint_open_opts(prog_fd, &raw_tp_opts);
    if !ASSERT_GE(link_fd as c_long, 0, c"bpf_raw_tracepoint_open_opts".as_ptr()) {
        if link_fd >= 0 { close(link_fd); }
        bpf_link__destroy(link);
        return;
    }

    usleep(1); /* trigger */
    close(link_fd); /* detach */
    link_fd = -1;

    ASSERT_EQ((*(*skel).bss).tp_btf_res, cookie, c"raw_tp_open_res".as_ptr());

    /* low-level generic bpf_link_create() API */
    (*(*skel).bss).tp_btf_res = 0;

    cookie = 0x22000000000000;
    link_opts.tracing.cookie = cookie;
    link_fd = bpf_link_create(prog_fd, 0, BPF_TRACE_RAW_TP, &link_opts);
    if !ASSERT_GE(link_fd as c_long, 0, c"bpf_link_create".as_ptr()) {
        if link_fd >= 0 { close(link_fd); }
        bpf_link__destroy(link);
        return;
    }

    usleep(1); /* trigger */
    close(link_fd); /* detach */
    link_fd = -1;

    ASSERT_EQ((*(*skel).bss).tp_btf_res, cookie, c"link_create_res".as_ptr());

    /* high-level bpf_link-based bpf_program__attach_trace_opts() API */
    (*(*skel).bss).tp_btf_res = 0;

    cookie = 0x33000000000000;
    trace_opts.cookie = cookie;
    link = bpf_program__attach_trace_opts((*skel).progs.handle_tp_btf, &trace_opts);
    if !ASSERT_OK_PTR_LINK(link, c"attach_trace_opts".as_ptr()) {
        if link_fd >= 0 { close(link_fd); }
        bpf_link__destroy(link);
        return;
    }

    usleep(1); /* trigger */
    bpf_link__destroy(link); /* detach */
    link = ptr::null_mut();

    ASSERT_EQ((*(*skel).bss).tp_btf_res, cookie, c"attach_trace_opts_res".as_ptr());

    if link_fd >= 0 { close(link_fd); }
    bpf_link__destroy(link);
}

unsafe fn verify_raw_tp_link_info(fd: c_int, cookie: u64) -> c_int {
    let mut info: bpf_link_info = mem::zeroed();
    let mut err: c_int;
    let mut len: u32 = mem::size_of::<bpf_link_info>() as u32;

    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    if !ASSERT_OK(err, c"get_link_info".as_ptr()) {
        return -1;
    }

    if !ASSERT_EQ(info.type_ as u64, BPF_LINK_TYPE_RAW_TRACEPOINT as u64, c"link_type".as_ptr()) {
        return -1;
    }

    ASSERT_EQ(info.raw_tracepoint.cookie, cookie, c"raw_tp_cookie".as_ptr());

    0
}

unsafe fn raw_tp_subtest(skel: *mut test_bpf_cookie) {
    let mut cookie: __u64;
    let mut err: c_int;
    let mut prog_fd: c_int;
    let mut link_fd: c_int = -1;
    let mut link: *mut bpf_link = ptr::null_mut();
    let mut raw_tp_opts: bpf_raw_tp_opts = mem::zeroed();
    let mut opts: bpf_raw_tracepoint_opts = mem::zeroed();

    /* There are two different ways to attach raw_tp programs */
    prog_fd = bpf_program__fd((*skel).progs.handle_raw_tp);

    /* low-level BPF_RAW_TRACEPOINT_OPEN command wrapper */
    (*(*skel).bss).raw_tp_res = 0;

    raw_tp_opts.tp_name = c"sys_enter".as_ptr();
    cookie = 0x55000000000000;
    raw_tp_opts.cookie = cookie;
    link_fd = bpf_raw_tracepoint_open_opts(prog_fd, &raw_tp_opts);
    if !ASSERT_GE(link_fd as c_long, 0, c"bpf_raw_tracepoint_open_opts".as_ptr()) {
        if link_fd >= 0 { close(link_fd); }
        bpf_link__destroy(link);
        return;
    }

    usleep(1); /* trigger */

    err = verify_raw_tp_link_info(link_fd, cookie);
    if !ASSERT_OK(err, c"verify_raw_tp_link_info".as_ptr()) {
        if link_fd >= 0 { close(link_fd); }
        bpf_link__destroy(link);
        return;
    }

    close(link_fd); /* detach */
    link_fd = -1;

    ASSERT_EQ((*(*skel).bss).raw_tp_res, cookie, c"raw_tp_open_res".as_ptr());

    /* high-level bpf_link-based bpf_program__attach_raw_tracepoint_opts() API */
    (*(*skel).bss).raw_tp_res = 0;

    cookie = 0x66000000000000;
    opts.cookie = cookie;
    link = bpf_program__attach_raw_tracepoint_opts((*skel).progs.handle_raw_tp, c"sys_enter".as_ptr(), &opts);
    if !ASSERT_OK_PTR_LINK(link, c"attach_raw_tp_opts".as_ptr()) {
        if link_fd >= 0 { close(link_fd); }
        bpf_link__destroy(link);
        return;
    }

    usleep(1); /* trigger */
    bpf_link__destroy(link); /* detach */
    link = ptr::null_mut();

    ASSERT_EQ((*(*skel).bss).raw_tp_res, cookie, c"attach_raw_tp_opts_res".as_ptr());

    if link_fd >= 0 { close(link_fd); }
    bpf_link__destroy(link);
}

#[no_mangle]
pub unsafe extern "C" fn test_bpf_cookie() {
    let mut skel: *mut test_bpf_cookie;

    skel = test_bpf_cookie__open_and_load();
    if !ASSERT_OK_PTR_TEST_BPF_COOKIE(skel, c"skel_open".as_ptr()) {
        return;
    }

    (*(*skel).bss).my_tid = sys_gettid();

    if test__start_subtest(c"kprobe".as_ptr()) {
        kprobe_subtest(skel);
    }
    if test__start_subtest(c"multi_kprobe_link_api".as_ptr()) {
        kprobe_multi_link_api_subtest();
    }
    if test__start_subtest(c"multi_kprobe_attach_api".as_ptr()) {
        kprobe_multi_attach_api_subtest();
    }
    if test__start_subtest(c"uprobe".as_ptr()) {
        uprobe_subtest(skel);
    }
    if test__start_subtest(c"multi_uprobe_attach_api".as_ptr()) {
        uprobe_multi_attach_api_subtest();
    }
    if test__start_subtest(c"tracepoint".as_ptr()) {
        tp_subtest(skel);
    }
    if test__start_subtest(c"perf_event".as_ptr()) {
        pe_subtest(skel);
    }
    if test__start_subtest(c"trampoline".as_ptr()) {
        tracing_subtest(skel);
    }
    if test__start_subtest(c"lsm".as_ptr()) {
        lsm_subtest(skel);
    }
    if test__start_subtest(c"tp_btf".as_ptr()) {
        tp_btf_subtest(skel);
    }
    if test__start_subtest(c"raw_tp".as_ptr()) {
        raw_tp_subtest(skel);
    }
    test_bpf_cookie__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
