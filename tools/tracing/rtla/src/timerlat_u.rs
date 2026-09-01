// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2023 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
 */

// C dependencies: sched.h, fcntl.h, stdlib.h, unistd.h, stdio.h, errno.h,
// string.h, tracefs.h, pthread.h, sys/wait.h, sys/prctl.h, common.h,
// timerlat_u.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const MAX_PATH: usize = 1024;
const SCHED_FIFO: c_int = 1;
const O_RDONLY: c_int = 0;
const SIGKILL: c_int = 9;
const WNOHANG: c_int = 1;
const PR_SET_NAME: c_int = 15;

#[repr(C)]
pub struct sched_param {
    pub sched_priority: c_int,
}

#[repr(C)]
pub struct cpu_set_t {
    __bits: [c_ulong; 16],
}

#[repr(C)]
pub struct timerlat_u_params {
    pub sched_param: *mut c_void,
    pub cgroup_name: *const c_char,
    pub set: *mut cpu_set_t,
    pub should_run: c_int,
    pub stopped_running: c_int,
}

type pid_t = c_int;
type pthread_t = c_ulong;

unsafe extern "C" {
    static mut nr_cpus: c_int;
    static mut errno: c_int;

    fn gettid() -> pid_t;
    fn getpid() -> pid_t;
    fn sched_setaffinity(pid: pid_t, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;
    fn sched_setscheduler(pid: pid_t, policy: c_int, param: *const sched_param) -> c_int;
    fn __set_sched_attr(pid: pid_t, sched_param: *mut c_void) -> c_int;
    fn set_pid_cgroup(pid: pid_t, cgroup_name: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn tracefs_instance_file_open(instance: *mut c_void, file: *const c_char, flags: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn pthread_exit(retval: *mut c_void) -> !;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn fork() -> pid_t;
    fn pthread_self() -> pthread_t;
    fn pthread_setname_np(thread: pthread_t, name: *const c_char) -> c_int;
    fn prctl(option: c_int, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn sleep(seconds: c_uint) -> c_uint;
    fn strerror(errnum: c_int) -> *mut c_char;

    fn debug_msg(format: *const c_char, ...);
    fn err_msg(format: *const c_char, ...);
    fn fatal(format: *const c_char, ...) -> !;
}

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    unsafe {
        let bits = (*set).__bits.as_mut_ptr();
        let mut i = 0usize;
        while i < (*set).__bits.len() {
            *bits.add(i) = 0;
            i += 1;
        }
    }
}

unsafe fn CPU_SET(cpu: c_int, set: *mut cpu_set_t) {
    unsafe {
        let bits_per_word = 8 * core::mem::size_of::<c_ulong>();
        let cpu = cpu as usize;
        (*set).__bits[cpu / bits_per_word] |= (1 as c_ulong) << (cpu % bits_per_word);
    }
}

unsafe fn CPU_ISSET(cpu: c_int, set: *const cpu_set_t) -> bool {
    unsafe {
        let bits_per_word = 8 * core::mem::size_of::<c_ulong>();
        let cpu = cpu as usize;
        (((*set).__bits[cpu / bits_per_word] >> (cpu % bits_per_word)) & 1) != 0
    }
}

/*
 * This is the user-space main for the tool timerlatu/ threads.
 *
 * It is as simple as this:
 *  - set affinity
 *  - set priority
 *  - open tracer fd
 *  - spin
 *  - close
 */
unsafe fn timerlat_u_main(cpu: c_int, params: *mut timerlat_u_params) -> c_int {
    unsafe {
        let mut sp = sched_param { sched_priority: 95 };
        let mut buffer = [0 as c_char; MAX_PATH];
        let timerlat_fd: c_int;
        let mut set = cpu_set_t { __bits: [0; 16] };
        let mut retval: c_int;

        /*
         * This all is only setting up the tool.
         */
        CPU_ZERO(&mut set);
        CPU_SET(cpu, &mut set);

        retval = sched_setaffinity(gettid(), core::mem::size_of_val(&set), &set);
        if retval == -1 {
            debug_msg(
                c"Error setting user thread affinity %d, is the CPU online?\n".as_ptr(),
                cpu,
            );
            exit(1);
        }

        if (*params).sched_param.is_null() {
            retval = sched_setscheduler(0, SCHED_FIFO, &sp);
            if retval < 0 {
                fatal(
                    c"Error setting timerlat u default priority: %s".as_ptr(),
                    strerror(errno),
                );
            }
        } else {
            retval = __set_sched_attr(getpid(), (*params).sched_param);
            if retval != 0 {
                /* __set_sched_attr prints an error message, so */
                exit(0);
            }
        }

        if !(*params).cgroup_name.is_null() {
            retval = set_pid_cgroup(gettid(), (*params).cgroup_name);
            if retval == 0 {
                err_msg(c"Error setting timerlat u cgroup pid\n".as_ptr());
                pthread_exit((&mut retval as *mut c_int).cast::<c_void>());
            }
        }

        /*
         * This is the tool's loop. If you want to use as base for your own tool...
         * go ahead.
         */
        snprintf(
            buffer.as_mut_ptr(),
            buffer.len(),
            c"osnoise/per_cpu/cpu%d/timerlat_fd".as_ptr(),
            cpu,
        );

        timerlat_fd = tracefs_instance_file_open(core::ptr::null_mut(), buffer.as_ptr(), O_RDONLY);
        if timerlat_fd < 0 {
            fatal(c"Error opening %s:%s".as_ptr(), buffer.as_ptr(), strerror(errno));
        }

        debug_msg(c"User-space timerlat pid %d on cpu %d\n".as_ptr(), gettid(), cpu);

        /* add should continue with a signal handler */
        loop {
            retval = read(timerlat_fd, buffer.as_mut_ptr().cast::<c_void>(), buffer.len()) as c_int;
            if retval < 0 {
                break;
            }
        }

        close(timerlat_fd);

        debug_msg(c"Leaving timerlat pid %d on cpu %d\n".as_ptr(), gettid(), cpu);
        exit(0);
    }
}

/*
 * timerlat_u_send_kill - send a kill signal for all processes
 *
 * Return the number of processes that received the kill.
 */
unsafe fn timerlat_u_send_kill(procs: *mut pid_t) -> c_int {
    unsafe {
        let mut killed = 0;
        let mut retval: c_int;
        let mut i = 0;

        while i < nr_cpus {
            if *procs.add(i as usize) == 0 {
                i += 1;
                continue;
            }
            retval = kill(*procs.add(i as usize), SIGKILL);
            if retval == 0 {
                killed += 1;
            } else {
                err_msg(c"Error killing child process %d\n".as_ptr(), *procs.add(i as usize));
            }
            i += 1;
        }

        killed
    }
}

/**
 * timerlat_u_dispatcher - dispatch one timerlatu/ process per monitored CPU
 *
 * This is a thread main that will fork one new process for each monitored
 * CPU. It will wait for:
 *
 *  - rtla to tell to kill the child processes
 *  - some child process to die, and the cleanup all the processes
 *
 * whichever comes first.
 *
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn timerlat_u_dispatcher(data: *mut c_void) -> *mut c_void {
    unsafe {
        let params = data.cast::<timerlat_u_params>();
        let mut proc_name = [0 as c_char; 128];
        let mut procs_count = 0;
        let mut retval = 1;
        let procs: *mut pid_t;
        let mut wstatus: c_int = 0;
        let mut pid: pid_t;
        let mut i: c_int;

        debug_msg(c"Dispatching timerlat u procs\n".as_ptr());

        procs = calloc(nr_cpus as usize, core::mem::size_of::<pid_t>()).cast::<pid_t>();
        if procs.is_null() {
            pthread_exit((&mut retval as *mut c_int).cast::<c_void>());
        }

        i = 0;
        while i < nr_cpus {
            if !(*params).set.is_null() && !CPU_ISSET(i, (*params).set) {
                i += 1;
                continue;
            }

            pid = fork();

            /* child */
            if pid == 0 {
                /*
                 * rename the process
                 */
                snprintf(
                    proc_name.as_mut_ptr(),
                    proc_name.len(),
                    c"timerlatu/%d".as_ptr(),
                    i,
                );
                pthread_setname_np(pthread_self(), proc_name.as_ptr());
                prctl(PR_SET_NAME, proc_name.as_ptr() as c_ulong, 0, 0, 0);

                timerlat_u_main(i, params);
                /* timerlat_u_main should exit()! Anyways... */
                pthread_exit((&mut retval as *mut c_int).cast::<c_void>());
            }

            /* parent */
            if pid == -1 {
                timerlat_u_send_kill(procs);
                debug_msg(c"Failed to create child processes".as_ptr());
                pthread_exit((&mut retval as *mut c_int).cast::<c_void>());
            }

            procs_count += 1;
            *procs.add(i as usize) = pid;
            i += 1;
        }

        while (*params).should_run != 0 {
            /* check if processes died */
            pid = waitpid(-1, &mut wstatus, WNOHANG);
            if pid != 0 {
                i = 0;
                while i < nr_cpus {
                    if *procs.add(i as usize) == pid {
                        *procs.add(i as usize) = 0;
                        procs_count -= 1;
                    }
                    i += 1;
                }

                if procs_count == 0 {
                    break;
                }
            }

            sleep(1);
        }

        timerlat_u_send_kill(procs);

        while procs_count != 0 {
            pid = waitpid(-1, &mut wstatus, 0);
            if pid == -1 {
                err_msg(c"Failed to monitor child processes".as_ptr());
                pthread_exit((&mut retval as *mut c_int).cast::<c_void>());
            }
            i = 0;
            while i < nr_cpus {
                if *procs.add(i as usize) == pid {
                    *procs.add(i as usize) = 0;
                    procs_count -= 1;
                }
                i += 1;
            }
        }

        (*params).stopped_running = 1;

        free(procs.cast::<c_void>());
        retval = 0;
        pthread_exit((&mut retval as *mut c_int).cast::<c_void>());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
