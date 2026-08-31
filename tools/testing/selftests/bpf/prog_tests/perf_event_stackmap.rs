// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook
// C source used _GNU_SOURCE and included pthread.h, sched.h, test_progs.h,
// and "perf_event_stackmap.skel.h"; those dependencies are expected to be
// supplied by the surrounding translated test harness.

use core::ffi::{c_int, c_long, c_uint, c_void};

extern "C" {
    fn usleep(usec: c_uint) -> c_int;
    fn read_perf_max_sample_freq() -> u64;
    fn perf_event_stackmap__open() -> *mut perf_event_stackmap;
    fn perf_event_stackmap__load(skel: *mut perf_event_stackmap) -> c_int;
    fn perf_event_stackmap__destroy(skel: *mut perf_event_stackmap);
    fn pthread_self() -> pthread_t;
    fn pthread_setaffinity_np(thread: pthread_t, cpusetsize: usize, cpuset: *const cpu_set_t) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn printf(format: *const i8, ...) -> c_int;
    fn test__skip();
    fn bpf_program__attach_perf_event(prog: *mut bpf_program, pfd: c_int) -> *mut bpf_link;
    fn close(fd: c_int) -> c_int;
    fn __errno_location() -> *mut c_int;
}

extern "Rust" {
    fn CHECK(condition: bool, tag: *const i8, format: *const i8, ...) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut bpf_link, name: *const i8) -> bool;
}

type pthread_t = usize;

#[repr(C)]
pub struct cpu_set_t {
    __bits: [usize; 16],
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period_or_freq: u64,
    pub sample_type: u64,
    pub read_format: u64,
    pub flags: u64,
    pub wakeup_events_or_watermark: u32,
    pub bp_type: u32,
    pub bp_addr_or_config1: u64,
    pub bp_len_or_config2: u64,
    pub branch_sample_type: u64,
    pub sample_regs_user: u64,
    pub sample_stack_user: u32,
    pub clockid: i32,
    pub sample_regs_intr: u64,
    pub aux_watermark: u32,
    pub sample_max_stack: u16,
    pub __reserved_2: u16,
    pub aux_sample_size: u32,
    pub __reserved_3: u32,
    pub sig_data: u64,
    pub config3: u64,
}

#[repr(C)]
pub struct perf_event_stackmap {
    pub links: perf_event_stackmap_links,
    pub progs: perf_event_stackmap_progs,
    pub data: *mut perf_event_stackmap_data,
}

#[repr(C)]
pub struct perf_event_stackmap_links {
    pub oncpu: *mut bpf_link,
}

#[repr(C)]
pub struct perf_event_stackmap_progs {
    pub oncpu: *mut bpf_program,
}

#[repr(C)]
pub struct perf_event_stackmap_data {
    pub stackid_kernel: i32,
    pub stackid_user: i32,
    pub stack_kernel: i32,
    pub stack_user: i32,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
const PERF_SAMPLE_IP: u64 = 1 << 0;
const PERF_SAMPLE_CALLCHAIN: u64 = 1 << 5;
const PERF_SAMPLE_BRANCH_STACK: u64 = 1 << 11;
const PERF_SAMPLE_BRANCH_USER: u64 = 1 << 0;
const PERF_SAMPLE_BRANCH_NO_FLAGS: u64 = 1 << 3;
const PERF_SAMPLE_BRANCH_NO_CYCLES: u64 = 1 << 4;
const PERF_SAMPLE_BRANCH_CALL_STACK: u64 = 1 << 5;
const PERF_ATTR_FLAG_FREQ: u64 = 1 << 10;
const PERF_ATTR_FLAG_PRECISE_IP_SHIFT: u64 = 15;
const __NR_perf_event_open: c_long = 298;
const CPU_SETSIZE: usize = 1024;
const NCPUBITS: usize = 8 * core::mem::size_of::<usize>();

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    (*set).__bits = [0; 16];
}

unsafe fn CPU_SET(cpu: usize, set: *mut cpu_set_t) {
    if cpu < CPU_SETSIZE {
        (*set).__bits[cpu / NCPUBITS] |= 1usize << (cpu % NCPUBITS);
    }
}

#[inline(never)]
pub unsafe extern "C" fn func_1() -> c_int {
    static mut VAL: c_int = 1;

    VAL += 1;

    usleep(100);
    VAL
}

#[inline(never)]
pub unsafe extern "C" fn func_2() -> c_int {
    func_1()
}

#[inline(never)]
pub unsafe extern "C" fn func_3() -> c_int {
    func_2()
}

#[inline(never)]
pub unsafe extern "C" fn func_4() -> c_int {
    func_3()
}

#[inline(never)]
pub unsafe extern "C" fn func_5() -> c_int {
    func_4()
}

#[inline(never)]
pub unsafe extern "C" fn func_6() -> c_int {
    let mut i: c_int;
    let mut val: c_int = 1;

    i = 0;
    while i < 100 {
        val += func_5();
        i += 1;
    }

    val
}

pub unsafe extern "C" fn test_perf_event_stackmap() {
    let mut attr = perf_event_attr {
        // .type = PERF_TYPE_SOFTWARE,
        type_: PERF_TYPE_HARDWARE,
        config: PERF_COUNT_HW_CPU_CYCLES,
        flags: (2 << PERF_ATTR_FLAG_PRECISE_IP_SHIFT) | PERF_ATTR_FLAG_FREQ,
        sample_type: PERF_SAMPLE_IP | PERF_SAMPLE_BRANCH_STACK | PERF_SAMPLE_CALLCHAIN,
        branch_sample_type: PERF_SAMPLE_BRANCH_USER
            | PERF_SAMPLE_BRANCH_NO_FLAGS
            | PERF_SAMPLE_BRANCH_NO_CYCLES
            | PERF_SAMPLE_BRANCH_CALL_STACK,
        sample_period_or_freq: read_perf_max_sample_freq(),
        size: core::mem::size_of::<perf_event_attr>() as u32,
        read_format: 0,
        wakeup_events_or_watermark: 0,
        bp_type: 0,
        bp_addr_or_config1: 0,
        bp_len_or_config2: 0,
        sample_regs_user: 0,
        sample_stack_user: 0,
        clockid: 0,
        sample_regs_intr: 0,
        aux_watermark: 0,
        sample_max_stack: 0,
        __reserved_2: 0,
        aux_sample_size: 0,
        __reserved_3: 0,
        sig_data: 0,
        config3: 0,
    };
    let mut skel: *mut perf_event_stackmap;
    let duration: u32 = 0;
    let mut cpu_set = cpu_set_t { __bits: [0; 16] };
    let mut pmu_fd: c_int;
    let mut err: c_int;

    let _ = duration;
    let _ = &mut attr;

    skel = perf_event_stackmap__open();

    if CHECK(
        skel.is_null(),
        b"skel_open\0".as_ptr() as *const i8,
        b"skeleton open failed\n\0".as_ptr() as *const i8,
    ) {
        return;
    }

    err = perf_event_stackmap__load(skel);
    if CHECK(
        err != 0,
        b"skel_load\0".as_ptr() as *const i8,
        b"skeleton load failed: %d\n\0".as_ptr() as *const i8,
        err,
    ) {
        perf_event_stackmap__destroy(skel);
        return;
    }

    CPU_ZERO(&mut cpu_set);
    CPU_SET(0, &mut cpu_set);
    err = pthread_setaffinity_np(
        pthread_self(),
        core::mem::size_of_val(&cpu_set),
        &cpu_set,
    );
    if CHECK(
        err != 0,
        b"set_affinity\0".as_ptr() as *const i8,
        b"err %d, errno %d\n\0".as_ptr() as *const i8,
        err,
        *__errno_location(),
    ) {
        perf_event_stackmap__destroy(skel);
        return;
    }

    pmu_fd = syscall(
        __NR_perf_event_open,
        &mut attr as *mut perf_event_attr,
        -1 as c_int, // pid
        0 as c_int,  // cpu 0
        -1 as c_int, // group id
        0 as c_int,  // flags
    ) as c_int;
    if pmu_fd < 0 {
        printf(
            b"%s:SKIP:cpu doesn't support the event\n\0".as_ptr() as *const i8,
            b"test_perf_event_stackmap\0".as_ptr() as *const i8,
        );
        test__skip();
        perf_event_stackmap__destroy(skel);
        return;
    }

    (*skel).links.oncpu = bpf_program__attach_perf_event((*skel).progs.oncpu, pmu_fd);
    if !ASSERT_OK_PTR(
        (*skel).links.oncpu,
        b"attach_perf_event\0".as_ptr() as *const i8,
    ) {
        close(pmu_fd);
        perf_event_stackmap__destroy(skel);
        return;
    }

    // create kernel and user stack traces for testing
    func_6();

    CHECK(
        (*(*skel).data).stackid_kernel != 2,
        b"get_stackid_kernel\0".as_ptr() as *const i8,
        b"failed\n\0".as_ptr() as *const i8,
    );
    CHECK(
        (*(*skel).data).stackid_user != 2,
        b"get_stackid_user\0".as_ptr() as *const i8,
        b"failed\n\0".as_ptr() as *const i8,
    );
    CHECK(
        (*(*skel).data).stack_kernel != 2,
        b"get_stack_kernel\0".as_ptr() as *const i8,
        b"failed\n\0".as_ptr() as *const i8,
    );
    CHECK(
        (*(*skel).data).stack_user != 2,
        b"get_stack_user\0".as_ptr() as *const i8,
        b"failed\n\0".as_ptr() as *const i8,
    );

    perf_event_stackmap__destroy(skel);
}
