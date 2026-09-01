// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2019 Facebook */
/* C source included: sched.h, sys/prctl.h, test_progs.h */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::{size_of, size_of_val};

type __u64 = u64;

const MAX_CNT: c_int = 100000;
const CLOCK_MONOTONIC: c_int = 1;
const O_WRONLY: c_int = 1;
const O_TRUNC: c_int = 0o1000;
const PR_GET_NAME: c_int = 16;
const PR_SET_NAME: c_int = 15;

#[repr(C)]
struct timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

#[repr(C)]
struct cpu_set_t {
    __bits: [u64; 16],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

unsafe extern "C" {
    static errno: c_int;

    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn sched_setaffinity(pid: c_int, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;

    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn bpf_program__attach_kprobe(
        prog: *mut bpf_program,
        retprobe: bool,
        func_name: *const c_char,
    ) -> *mut bpf_link;
    fn bpf_program__attach_raw_tracepoint(
        prog: *mut bpf_program,
        tp_name: *const c_char,
    ) -> *mut bpf_link;
    fn bpf_program__attach_trace(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
}

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    for bit in (*set).__bits.iter_mut() {
        *bit = 0;
    }
}

unsafe fn CPU_SET(cpu: c_int, set: *mut cpu_set_t) {
    let cpu = cpu as usize;
    (*set).__bits[cpu / 64] |= 1u64 << (cpu % 64);
}

unsafe fn time_get_ns() -> __u64 {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    clock_gettime(CLOCK_MONOTONIC, &mut ts);
    (ts.tv_sec as __u64)
        .wrapping_mul(1000000000u64)
        .wrapping_add(ts.tv_nsec as __u64)
}

unsafe fn test_task_rename(prog: *const c_char) -> c_int {
    let mut i: c_int;
    let fd: c_int;
    let mut duration: c_int = 0;
    let mut err: isize;
    let buf = *b"test_overhead\0";
    let start_time: __u64;

    fd = open(
        b"/proc/self/comm\0".as_ptr() as *const c_char,
        O_WRONLY | O_TRUNC,
    );
    if CHECK!(
        fd < 0,
        b"open /proc\0".as_ptr() as *const c_char,
        b"err %d\0".as_ptr() as *const c_char,
        errno
    ) {
        return -1;
    }
    start_time = time_get_ns();
    i = 0;
    while i < MAX_CNT {
        err = write(fd, buf.as_ptr() as *const c_void, size_of_val(&buf));
        if err < 0 {
            CHECK!(
                err < 0,
                b"task rename\0".as_ptr() as *const c_char,
                b"err %d\0".as_ptr() as *const c_char,
                errno
            );
            close(fd);
            return -1;
        }
        i += 1;
    }
    printf(
        b"task_rename %s\t%lluK events per sec\n\0".as_ptr() as *const c_char,
        prog,
        (MAX_CNT as i64).wrapping_mul(1000000i64) / ((time_get_ns() - start_time) as i64),
    );
    close(fd);
    0
}

unsafe fn test_run(prog: *const c_char) {
    test_task_rename(prog);
}

unsafe fn setaffinity() {
    let mut cpuset = cpu_set_t { __bits: [0; 16] };
    let cpu: c_int = 0;

    CPU_ZERO(&mut cpuset);
    CPU_SET(cpu, &mut cpuset);
    sched_setaffinity(0, size_of_val(&cpuset), &cpuset);
}

#[no_mangle]
pub unsafe extern "C" fn test_test_overhead() {
    let kprobe_name: *const c_char = b"prog1\0".as_ptr() as *const c_char;
    let kretprobe_name: *const c_char = b"prog2\0".as_ptr() as *const c_char;
    let raw_tp_name: *const c_char = b"prog3\0".as_ptr() as *const c_char;
    let fentry_name: *const c_char = b"prog4\0".as_ptr() as *const c_char;
    let fexit_name: *const c_char = b"prog5\0".as_ptr() as *const c_char;
    let kprobe_func: *const c_char = b"__set_task_comm\0".as_ptr() as *const c_char;
    let mut kprobe_prog: *mut bpf_program;
    let mut kretprobe_prog: *mut bpf_program;
    let mut raw_tp_prog: *mut bpf_program;
    let mut fentry_prog: *mut bpf_program;
    let mut fexit_prog: *mut bpf_program;
    let obj: *mut bpf_object;
    let mut link: *mut bpf_link;
    let mut err: c_int;
    let mut duration: c_int = 0;
    let mut comm: [c_char; 16] = [0; 16];

    if CHECK_FAIL!(prctl(
        PR_GET_NAME,
        comm.as_mut_ptr(),
        0 as c_long,
        0 as c_long,
        0 as c_long
    )) {
        return;
    }

    obj = bpf_object__open_file(
        b"./test_overhead.bpf.o\0".as_ptr() as *const c_char,
        core::ptr::null(),
    );
    if !ASSERT_OK_PTR!(obj, b"obj_open_file\0".as_ptr() as *const c_char) {
        return;
    }

    'cleanup: loop {
        kprobe_prog = bpf_object__find_program_by_name(obj, kprobe_name);
        if CHECK!(
            kprobe_prog.is_null(),
            b"find_probe\0".as_ptr() as *const c_char,
            b"prog '%s' not found\n\0".as_ptr() as *const c_char,
            kprobe_name
        ) {
            break 'cleanup;
        }
        kretprobe_prog = bpf_object__find_program_by_name(obj, kretprobe_name);
        if CHECK!(
            kretprobe_prog.is_null(),
            b"find_probe\0".as_ptr() as *const c_char,
            b"prog '%s' not found\n\0".as_ptr() as *const c_char,
            kretprobe_name
        ) {
            break 'cleanup;
        }
        raw_tp_prog = bpf_object__find_program_by_name(obj, raw_tp_name);
        if CHECK!(
            raw_tp_prog.is_null(),
            b"find_probe\0".as_ptr() as *const c_char,
            b"prog '%s' not found\n\0".as_ptr() as *const c_char,
            raw_tp_name
        ) {
            break 'cleanup;
        }
        fentry_prog = bpf_object__find_program_by_name(obj, fentry_name);
        if CHECK!(
            fentry_prog.is_null(),
            b"find_probe\0".as_ptr() as *const c_char,
            b"prog '%s' not found\n\0".as_ptr() as *const c_char,
            fentry_name
        ) {
            break 'cleanup;
        }
        fexit_prog = bpf_object__find_program_by_name(obj, fexit_name);
        if CHECK!(
            fexit_prog.is_null(),
            b"find_probe\0".as_ptr() as *const c_char,
            b"prog '%s' not found\n\0".as_ptr() as *const c_char,
            fexit_name
        ) {
            break 'cleanup;
        }
        err = bpf_object__load(obj);
        if CHECK!(
            err != 0,
            b"obj_load\0".as_ptr() as *const c_char,
            b"err %d\n\0".as_ptr() as *const c_char,
            err
        ) {
            break 'cleanup;
        }

        setaffinity();

        /* base line run */
        test_run(b"base\0".as_ptr() as *const c_char);

        /* attach kprobe */
        link = bpf_program__attach_kprobe(kprobe_prog, false /* retprobe */, kprobe_func);
        if !ASSERT_OK_PTR!(link, b"attach_kprobe\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }
        test_run(b"kprobe\0".as_ptr() as *const c_char);
        bpf_link__destroy(link);

        /* attach kretprobe */
        link = bpf_program__attach_kprobe(kretprobe_prog, true /* retprobe */, kprobe_func);
        if !ASSERT_OK_PTR!(link, b"attach_kretprobe\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }
        test_run(b"kretprobe\0".as_ptr() as *const c_char);
        bpf_link__destroy(link);

        /* attach raw_tp */
        link = bpf_program__attach_raw_tracepoint(
            raw_tp_prog,
            b"task_rename\0".as_ptr() as *const c_char,
        );
        if !ASSERT_OK_PTR!(link, b"attach_raw_tp\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }
        test_run(b"raw_tp\0".as_ptr() as *const c_char);
        bpf_link__destroy(link);

        /* attach fentry */
        link = bpf_program__attach_trace(fentry_prog);
        if !ASSERT_OK_PTR!(link, b"attach_fentry\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }
        test_run(b"fentry\0".as_ptr() as *const c_char);
        bpf_link__destroy(link);

        /* attach fexit */
        link = bpf_program__attach_trace(fexit_prog);
        if !ASSERT_OK_PTR!(link, b"attach_fexit\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }
        test_run(b"fexit\0".as_ptr() as *const c_char);
        bpf_link__destroy(link);

        break 'cleanup;
    }

    prctl(
        PR_SET_NAME,
        comm.as_ptr(),
        0 as c_long,
        0 as c_long,
        0 as c_long,
    );
    bpf_object__close(obj);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
