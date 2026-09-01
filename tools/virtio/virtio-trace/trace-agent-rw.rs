// SPDX-License-Identifier: GPL-2.0-only
/*
 * Read/write thread of a guest agent for virtio-trace
 *
 * Copyright (C) 2012 Hitachi, Ltd.
 * Created by Yoshihiro Yunomae <yoshihiro.yunomae.ez@hitachi.com>
 *            Masami Hiramatsu <masami.hiramatsu.pt@hitachi.com>
 */

/* C source included: fcntl.h, stdio.h, stdlib.h, unistd.h, sys/syscall.h,
 * and "trace-agent.h".
 */

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong};
use std::ptr;

const READ_WAIT_USEC: c_uint = 100000;

#[repr(C)]
pub struct rw_thread_info {
    pub cpu_num: c_int,
    pub in_fd: c_int,
    pub out_fd: c_int,
    pub read_pipe: c_int,
    pub write_pipe: c_int,
    pub pipe_size: c_ulong,
}

type ssize_t = isize;
type pthread_t = c_ulong;
type pthread_mutex_t = c_void;
type pthread_cond_t = c_void;

const EXIT_FAILURE: c_int = 1;
const STDOUT_FILENO: c_int = 1;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_NONBLOCK: c_int = 0o4000;
const F_SETPIPE_SZ: c_int = 1031;
const SPLICE_F_MOVE: c_uint = 1;
const SPLICE_F_MORE: c_uint = 4;
const PIPE_INIT: c_ulong = 65536;

unsafe extern "C" {
    static mut global_run_operation: bool;
    static mut global_sig_receive: bool;
    static mut mutex_notify: pthread_mutex_t;
    static mut cond_wakeup: pthread_cond_t;

    fn zalloc(size: usize) -> *mut c_void;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn exit(status: c_int) -> !;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn pipe2(pipefd: *mut c_int, flags: c_int) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn splice(
        fd_in: c_int,
        off_in: *mut c_long,
        fd_out: c_int,
        off_out: *mut c_long,
        len: usize,
        flags: c_uint,
    ) -> ssize_t;
    fn usleep(usec: c_uint) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;

    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg: *mut c_void,
    ) -> c_int;

    fn sched_setaffinity(pid: c_int, cpusetsize: usize, mask: *const libc::cpu_set_t) -> c_int;
}

pub unsafe extern "C" fn rw_thread_info_new() -> *mut c_void {
    let rw_ti: *mut rw_thread_info;

    rw_ti = unsafe { zalloc(std::mem::size_of::<rw_thread_info>()) as *mut rw_thread_info };
    if rw_ti.is_null() {
        unsafe { pr_err(c"rw_thread_info zalloc error\n".as_ptr()) };
        unsafe { exit(EXIT_FAILURE) };
    }

    unsafe {
        (*rw_ti).cpu_num = -1;
        (*rw_ti).in_fd = -1;
        (*rw_ti).out_fd = -1;
        (*rw_ti).read_pipe = -1;
        (*rw_ti).write_pipe = -1;
        (*rw_ti).pipe_size = PIPE_INIT;
    }

    rw_ti as *mut c_void
}

pub unsafe extern "C" fn rw_thread_init(
    cpu: c_int,
    in_path: *const c_char,
    out_path: *const c_char,
    stdout_flag: bool,
    pipe_size: c_ulong,
    rw_ti: *mut rw_thread_info,
) -> *mut c_void {
    let mut data_pipe: [c_int; 2] = [0; 2];

    unsafe {
        (*rw_ti).cpu_num = cpu;
    }

    /* set read(input) fd */
    unsafe {
        (*rw_ti).in_fd = open(in_path, O_RDONLY);
    }
    if unsafe { (*rw_ti).in_fd } == -1 {
        unsafe { pr_err(c"Could not open in_fd (CPU:%d)\n".as_ptr(), cpu) };
        unsafe { exit(EXIT_FAILURE) };
    }

    /* set write(output) fd */
    if !stdout_flag {
        /* virtio-serial output mode */
        unsafe {
            (*rw_ti).out_fd = open(out_path, O_WRONLY);
        }
        if unsafe { (*rw_ti).out_fd } == -1 {
            unsafe { pr_err(c"Could not open out_fd (CPU:%d)\n".as_ptr(), cpu) };
            unsafe { exit(EXIT_FAILURE) };
        }
    } else {
        /* stdout mode */
        unsafe {
            (*rw_ti).out_fd = STDOUT_FILENO;
        }
    }

    if unsafe { pipe2(data_pipe.as_mut_ptr(), O_NONBLOCK) } < 0 {
        unsafe { pr_err(c"Could not create pipe in rw-thread(%d)\n".as_ptr(), cpu) };
        unsafe { exit(EXIT_FAILURE) };
    }

    /*
     * Size of pipe is 64kB in default based on fs/pipe.c.
     * To read/write trace data speedy, pipe size is changed.
     */
    if unsafe { fcntl(data_pipe[0], F_SETPIPE_SZ, pipe_size) } < 0 {
        unsafe { pr_err(c"Could not change pipe size in rw-thread(%d)\n".as_ptr(), cpu) };
        unsafe { exit(EXIT_FAILURE) };
    }

    unsafe {
        (*rw_ti).read_pipe = data_pipe[1];
        (*rw_ti).write_pipe = data_pipe[0];
        (*rw_ti).pipe_size = pipe_size;
    }

    ptr::null_mut()
}

/* Bind a thread to a cpu */
unsafe fn bind_cpu(cpu_num: c_int) {
    let mut mask: libc::cpu_set_t = unsafe { std::mem::zeroed() };

    unsafe {
        libc::CPU_ZERO(&mut mask);
        libc::CPU_SET(cpu_num as usize, &mut mask);
    }

    /* bind my thread to cpu_num by assigning zero to the first argument */
    if unsafe { sched_setaffinity(0, std::mem::size_of_val(&mask), &mask) } == -1 {
        unsafe { pr_err(c"Could not set CPU#%d affinity\n".as_ptr(), cpu_num as c_int) };
    }
}

unsafe extern "C" fn rw_thread_main(thread_info: *mut c_void) -> *mut c_void {
    let mut rlen: ssize_t;
    let mut wlen: ssize_t;
    let mut ret: ssize_t;
    let ts: *mut rw_thread_info = thread_info as *mut rw_thread_info;

    unsafe { bind_cpu((*ts).cpu_num) };

    loop {
        /* Wait for a read order of trace data by Host OS */
        if unsafe { !global_run_operation } {
            unsafe {
                pthread_mutex_lock(&raw mut mutex_notify);
                pthread_cond_wait(&raw mut cond_wakeup, &raw mut mutex_notify);
                pthread_mutex_unlock(&raw mut mutex_notify);
            }
        }

        if unsafe { global_sig_receive } {
            break;
        }

        /*
         * Each thread read trace_pipe_raw of each cpu bounding the
         * thread, so contention of multi-threads does not occur.
         */
        rlen = unsafe {
            splice(
                (*ts).in_fd,
                ptr::null_mut(),
                (*ts).read_pipe,
                ptr::null_mut(),
                (*ts).pipe_size as usize,
                SPLICE_F_MOVE | SPLICE_F_MORE,
            )
        };

        if rlen < 0 {
            unsafe { pr_err(c"Splice_read in rw-thread(%d)\n".as_ptr(), (*ts).cpu_num) };
            unsafe { exit(EXIT_FAILURE) };
        } else if rlen == 0 {
            /*
             * If trace data do not exist or are unreadable not
             * for exceeding the page size, splice_read returns
             * NULL. Then, this waits for being filled the data in a
             * ring-buffer.
             */
            unsafe {
                usleep(READ_WAIT_USEC);
                pr_debug(c"Read retry(cpu:%d)\n".as_ptr(), (*ts).cpu_num);
            }
            continue;
        }

        wlen = 0;

        loop {
            ret = unsafe {
                splice(
                    (*ts).write_pipe,
                    ptr::null_mut(),
                    (*ts).out_fd,
                    ptr::null_mut(),
                    (rlen - wlen) as usize,
                    SPLICE_F_MOVE | SPLICE_F_MORE,
                )
            };

            if ret < 0 {
                unsafe { pr_err(c"Splice_write in rw-thread(%d)\n".as_ptr(), (*ts).cpu_num) };
                unsafe { exit(EXIT_FAILURE) };
            } else if ret == 0 {
                /*
                 * When host reader is not in time for reading
                 * trace data, guest will be stopped. This is
                 * because char dev in QEMU is not supported
                 * non-blocking mode. Then, writer might be
                 * sleep in that case.
                 * This sleep will be removed by supporting
                 * non-blocking mode.
                 */
                unsafe {
                    sleep(1);
                }
            }
            wlen += ret;

            if !(wlen < rlen) {
                break;
            }
        }
    }

    ptr::null_mut()
}

pub unsafe extern "C" fn rw_thread_run(rw_ti: *mut rw_thread_info) -> pthread_t {
    let ret: c_int;
    let mut rw_thread_per_cpu: pthread_t = 0;

    ret = unsafe {
        pthread_create(
            &mut rw_thread_per_cpu,
            ptr::null(),
            Some(rw_thread_main),
            rw_ti as *mut c_void,
        )
    };
    if ret != 0 {
        unsafe { pr_err(c"Could not create a rw thread(%d)\n".as_ptr(), (*rw_ti).cpu_num) };
        unsafe { exit(EXIT_FAILURE) };
    }

    rw_thread_per_cpu
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
