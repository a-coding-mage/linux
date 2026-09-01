// SPDX-License-Identifier: GPL-2.0-only
/*
 * Guest agent for virtio-trace
 *
 * Copyright (C) 2012 Hitachi, Ltd.
 * Created by Yoshihiro Yunomae <yoshihiro.yunomae.ez@hitachi.com>
 *            Masami Hiramatsu <masami.hiramatsu.pt@hitachi.com>
 */

/*
 * C dependencies removed from executable Rust:
 * _GNU_SOURCE, <limits.h>, <stdio.h>, <stdlib.h>, <unistd.h>,
 * and "trace-agent.h".
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

const PIPE_DEF_BUFS: c_ulong = 16;
const PIPE_MAX_SIZE: c_ulong = 1024 * 1024;
const TRACEFS: &[u8] = b"/sys/kernel/tracing\0";
const DEBUGFS: &[u8] = b"/sys/kernel/debug/tracing\0";
const READ_PATH_FMT: &[u8] = b"%s/per_cpu/cpu%d/trace_pipe_raw\0";
const WRITE_PATH_FMT: &[u8] = b"/dev/virtio-ports/trace-path-cpu%d\0";
const CTL_PATH: &[u8] = b"/dev/virtio-ports/agent-ctl-path\0";

const EXIT_FAILURE: c_int = 1;
const F_OK: c_int = 0;
const PATH_MAX: usize = 4096;
const _SC_PAGE_SIZE: c_int = 30;
const _SC_NPROCESSORS_CONF: c_int = 83;

/* Provided by trace-agent.h in the original C source. */
const MAX_CPUS: usize = 0;
const PIPE_INIT: c_ulong = 0;

#[repr(C)]
pub struct pthread_mutex_t {
    _private: [usize; 0],
}

#[repr(C)]
pub struct pthread_cond_t {
    _private: [usize; 0],
}

pub type pthread_t = c_ulong;

#[repr(C)]
pub struct rw_thread_info {
    pub in_fd: c_int,
    pub out_fd: c_int,
    pub read_pipe: c_int,
    pub write_pipe: c_int,
}

#[repr(C)]
pub struct agent_info {
    pub pipe_size: c_ulong,
    pub use_stdout: bool,
    pub cpus: c_int,
    pub ctl_fd: c_int,
    pub rw_ti: [*mut rw_thread_info; MAX_CPUS],
}

unsafe extern "C" {
    static mut optarg: *mut c_char;

    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn free(ptr: *mut c_void);
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn sysconf(name: c_int) -> isize;

    fn zalloc(size: usize) -> *mut c_void;
    fn rw_thread_info_new() -> *mut rw_thread_info;
    fn rw_thread_init(
        cpu: c_int,
        in_path: *const c_char,
        out_path: *const c_char,
        use_stdout: bool,
        pipe_size: c_ulong,
        ti: *mut rw_thread_info,
    );
    fn rw_ctl_init(path: *const c_char) -> c_int;
    fn rw_thread_run(ti: *mut rw_thread_info) -> pthread_t;
    fn rw_ctl_loop(ctl_fd: c_int);

    fn pr_err(format: *const c_char, ...);
    fn pr_debug(format: *const c_char, ...);
}

/*
 * Original C used PTHREAD_MUTEX_INITIALIZER and PTHREAD_COND_INITIALIZER.
 * The concrete initializers are platform/header supplied.
 */
#[no_mangle]
pub static mut mutex_notify: pthread_mutex_t = pthread_mutex_t { _private: [] };
#[no_mangle]
pub static mut cond_wakeup: pthread_cond_t = pthread_cond_t { _private: [] };

unsafe fn page_size() -> c_ulong {
    sysconf(_SC_PAGE_SIZE) as c_ulong
}

unsafe fn pipe_min_size() -> c_ulong {
    page_size().wrapping_mul(PIPE_DEF_BUFS)
}

unsafe fn get_total_cpus() -> c_int {
    let nr_cpus: c_int = sysconf(_SC_NPROCESSORS_CONF) as c_int;

    if nr_cpus <= 0 {
        pr_err(c"Could not read cpus\n".as_ptr());
        exit(EXIT_FAILURE);
    } else if nr_cpus > MAX_CPUS as c_int {
        pr_err(c"Exceed max cpus(%d)\n".as_ptr(), MAX_CPUS as c_int);
        exit(EXIT_FAILURE);
    }

    nr_cpus
}

unsafe fn agent_info_new() -> *mut c_void {
    let s: *mut agent_info = zalloc(core::mem::size_of::<agent_info>()) as *mut agent_info;
    let mut i: c_int;

    if s.is_null() {
        pr_err(c"agent_info zalloc error\n".as_ptr());
        exit(EXIT_FAILURE);
    }

    (*s).pipe_size = PIPE_INIT;
    (*s).use_stdout = false;
    (*s).cpus = get_total_cpus();
    (*s).ctl_fd = -1;

    /* read/write threads init */
    i = 0;
    while i < (*s).cpus {
        (*s).rw_ti[i as usize] = rw_thread_info_new();
        i += 1;
    }

    s as *mut c_void
}

unsafe fn parse_size(arg: *const c_char) -> c_ulong {
    let mut value: c_ulong;
    let round: c_ulong;
    let mut ptr: *mut c_char = core::ptr::null_mut();

    value = strtoul(arg, &mut ptr, 10);
    match *ptr {
        b'K' | b'k' => {
            value <<= 10;
        }
        b'M' | b'm' => {
            value <<= 20;
        }
        _ => {}
    }

    if value > PIPE_MAX_SIZE {
        pr_err(c"Pipe size must be less than 1MB\n".as_ptr());
        return 0;
    } else if value < pipe_min_size() {
        pr_err(c"Pipe size must be over 64KB\n".as_ptr());
        return 0;
    }

    /* Align buffer size with page unit */
    round = value & (page_size() - 1);
    value = value - round;

    value
}

unsafe fn usage(prg: *const c_char) {
    pr_err(c"usage: %s [-h] [-o] [-s <size of pipe>]\n".as_ptr(), prg);
}

unsafe fn make_path(cpu_num: c_int, this_is_write_path: bool) -> *const c_char {
    let ret: c_int;
    let buf: *mut c_char;

    buf = zalloc(PATH_MAX) as *mut c_char;
    if buf.is_null() {
        pr_err(c"Could not allocate buffer\n".as_ptr());
        return core::ptr::null();
    }

    if this_is_write_path {
        /* write(output) path */
        ret = snprintf(buf, PATH_MAX, WRITE_PATH_FMT.as_ptr() as *const c_char, cpu_num);
    } else {
        let mut tmp_ret: c_int;

        /* read(input) path */
        tmp_ret = snprintf(
            buf,
            PATH_MAX,
            READ_PATH_FMT.as_ptr() as *const c_char,
            TRACEFS.as_ptr() as *const c_char,
            cpu_num,
        );
        if tmp_ret > 0 && access(buf, F_OK) != 0 {
            tmp_ret = snprintf(
                buf,
                PATH_MAX,
                READ_PATH_FMT.as_ptr() as *const c_char,
                DEBUGFS.as_ptr() as *const c_char,
                cpu_num,
            );
        }
        ret = tmp_ret;
    }

    if ret <= 0 {
        pr_err(
            c"Failed to generate %s path(CPU#%d):%d\n".as_ptr(),
            if this_is_write_path {
                c"read".as_ptr()
            } else {
                c"write".as_ptr()
            },
            cpu_num,
            ret,
        );
        free(buf as *mut c_void);
        return core::ptr::null();
    }

    buf
}

unsafe fn make_input_path(cpu_num: c_int) -> *const c_char {
    make_path(cpu_num, false)
}

unsafe fn make_output_path(cpu_num: c_int) -> *const c_char {
    make_path(cpu_num, true)
}

unsafe fn agent_info_init(s: *mut agent_info) -> *mut c_void {
    let mut cpu: c_int;
    let mut in_path: *const c_char;
    let mut out_path: *const c_char;

    /* init read/write threads */
    cpu = 0;
    while cpu < (*s).cpus {
        /* set read(input) path per read/write thread */
        in_path = make_input_path(cpu);
        if in_path.is_null() {
            exit(EXIT_FAILURE);
        }

        /* set write(output) path per read/write thread*/
        if !(*s).use_stdout {
            out_path = make_output_path(cpu);
            if out_path.is_null() {
                exit(EXIT_FAILURE);
            }
        } else {
            out_path = core::ptr::null();
            /* stdout mode */
            pr_debug(c"stdout mode\n".as_ptr());
        }

        rw_thread_init(
            cpu,
            in_path,
            out_path,
            (*s).use_stdout,
            (*s).pipe_size,
            (*s).rw_ti[cpu as usize],
        );

        cpu += 1;
    }

    /* init controller of read/write threads */
    (*s).ctl_fd = rw_ctl_init(CTL_PATH.as_ptr() as *const c_char);

    core::ptr::null_mut()
}

unsafe fn parse_args(argc: c_int, argv: *mut *mut c_char, s: *mut agent_info) -> *mut c_void {
    let mut cmd: c_int;
    let size: c_ulong;

    loop {
        cmd = getopt(argc, argv as *const *mut c_char, c"hos:".as_ptr());
        if cmd == -1 {
            break;
        }

        match cmd {
            /* stdout mode */
            x if x == b'o' as c_int => {
                (*s).use_stdout = true;
            }
            /* size of pipe */
            x if x == b's' as c_int => {
                size = parse_size(optarg);
                if size == 0 {
                    exit(EXIT_FAILURE);
                }
                (*s).pipe_size = size;
            }
            x if x == b'h' as c_int => {
                usage(*argv.add(0) as *const c_char);
                exit(EXIT_FAILURE);
            }
            _ => {
                usage(*argv.add(0) as *const c_char);
                exit(EXIT_FAILURE);
            }
        }
    }

    agent_info_init(s);

    core::ptr::null_mut()
}

unsafe fn agent_main_loop(s: *mut agent_info) {
    let mut cpu: c_int;
    let mut rw_thread_per_cpu: [pthread_t; MAX_CPUS] = [0; MAX_CPUS];

    /* Start all read/write threads */
    cpu = 0;
    while cpu < (*s).cpus {
        rw_thread_per_cpu[cpu as usize] = rw_thread_run((*s).rw_ti[cpu as usize]);
        cpu += 1;
    }

    rw_ctl_loop((*s).ctl_fd);

    /* Finish all read/write threads */
    cpu = 0;
    while cpu < (*s).cpus {
        let ret: c_int;

        ret = pthread_join(rw_thread_per_cpu[cpu as usize], core::ptr::null_mut());
        if ret != 0 {
            pr_err(c"pthread_join() error:%d (cpu %d)\n".as_ptr(), ret, cpu);
            exit(EXIT_FAILURE);
        }

        cpu += 1;
    }
}

unsafe fn agent_info_free(s: *mut agent_info) {
    let mut i: c_int;

    close((*s).ctl_fd);
    i = 0;
    while i < (*s).cpus {
        close((*(*s).rw_ti[i as usize]).in_fd);
        close((*(*s).rw_ti[i as usize]).out_fd);
        close((*(*s).rw_ti[i as usize]).read_pipe);
        close((*(*s).rw_ti[i as usize]).write_pipe);
        free((*s).rw_ti[i as usize] as *mut c_void);
        i += 1;
    }
    free(s as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut s: *mut agent_info = core::ptr::null_mut();

    s = agent_info_new() as *mut agent_info;
    parse_args(argc, argv, s);

    agent_main_loop(s);

    agent_info_free(s);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
