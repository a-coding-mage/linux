// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// C dependencies in the original source:
// signal.h, stdio.h, stdlib.h, stdbool.h, sys/types.h, sys/wait.h, unistd.h
// and "ebb.h".

use core::ffi::{c_char, c_int, c_uint};

type pid_t = c_int;

const BIND_CPU_ANY: c_int = -1;

#[repr(C)]
pub struct perf_event_attr {
    pub exclude_kernel: c_uint,
    pub exclude_hv: c_uint,
    pub exclude_idle: c_uint,
}

#[repr(C)]
pub struct event_result {
    pub enabled: u64,
    pub running: u64,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
    pub result: event_result,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pipe {
    pub fds: [c_int; 2],
}

extern "C" {
    fn event_init_named(event: *mut event, config: u64, name: *const c_char);
    fn require_paranoia_below(paranoia: c_int) -> c_int;
    fn event_open_with_cpu(event: *mut event, cpu: c_int) -> c_int;
    fn event_enable(event: *mut event) -> c_int;
    fn ebb_is_supported() -> c_int;
    fn bind_to_cpu(cpu: c_int) -> c_int;
    fn pipe(fds: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn ebb_child(read_pipe: pipe, write_pipe: pipe) -> c_int;
    fn sync_with_child(read_pipe: pipe, write_pipe: pipe) -> c_int;
    fn kill_child_and_wait(pid: pid_t);
    fn wait_for_child(pid: pid_t) -> c_int;
    fn event_disable(event: *mut event) -> c_int;
    fn event_read(event: *mut event) -> c_int;
    fn event_report(event: *mut event);
    fn test_harness(test: extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
            return 4;
        }
    };
}

/*
 * Tests an EBB vs a cpu event - in that order. The EBB should force the cpu
 * event off the PMU.
 */

unsafe fn setup_cpu_event(event: *mut event, cpu: c_int) -> c_int {
    event_init_named(event, 0x400FA, b"PM_RUN_INST_CMPL\0".as_ptr() as *const c_char);

    (*event).attr.exclude_kernel = 1;
    (*event).attr.exclude_hv = 1;
    (*event).attr.exclude_idle = 1;

    SKIP_IF!(require_paranoia_below(1) != 0);
    FAIL_IF!(event_open_with_cpu(event, cpu) != 0);
    FAIL_IF!(event_enable(event) != 0);

    0
}

#[no_mangle]
pub unsafe extern "C" fn ebb_vs_cpu_event() -> c_int {
    let mut read_pipe: pipe = pipe { fds: [0; 2] };
    let mut write_pipe: pipe = pipe { fds: [0; 2] };
    let mut event: event = core::mem::zeroed();
    let cpu: c_int;
    let rc: c_int;
    let pid: pid_t;

    SKIP_IF!(ebb_is_supported() == 0);

    cpu = bind_to_cpu(BIND_CPU_ANY);
    FAIL_IF!(cpu < 0);

    FAIL_IF!(pipe(read_pipe.fds.as_mut_ptr()) == -1);
    FAIL_IF!(pipe(write_pipe.fds.as_mut_ptr()) == -1);

    pid = fork();
    if pid == 0 {
        /* NB order of pipes looks reversed */
        exit(ebb_child(write_pipe, read_pipe));
    }

    /* Signal the child to install its EBB event and wait */
    FAIL_IF!(sync_with_child(read_pipe, write_pipe) != 0);

    /* Now try to install our CPU event */
    rc = setup_cpu_event(&mut event, cpu);
    if rc != 0 {
        kill_child_and_wait(pid);
        return rc;
    }

    /* Signal the child to run */
    FAIL_IF!(sync_with_child(read_pipe, write_pipe) != 0);

    /* .. and wait for it to complete */
    FAIL_IF!(wait_for_child(pid) != 0);
    FAIL_IF!(event_disable(&mut event) != 0);
    FAIL_IF!(event_read(&mut event) != 0);

    event_report(&mut event);

    /* The cpu event may have run, but we don't expect 100% */
    FAIL_IF!(event.result.enabled >= event.result.running);

    0
}

fn main() {
    unsafe {
        test_harness(ebb_vs_cpu_event, b"ebb_vs_cpu_event\0".as_ptr() as *const c_char);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
