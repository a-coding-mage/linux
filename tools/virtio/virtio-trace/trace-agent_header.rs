/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies from the original header:
// #include <pthread.h>
// #include <stdbool.h>

pub const MAX_CPUS: usize = 256;
pub const PIPE_INIT: usize = 1024 * 1024;

/*
 * agent_info - structure managing total information of guest agent
 * @pipe_size:	size of pipe (default 1MB)
 * @use_stdout:	set to true when o option is added (default false)
 * @cpus:	total number of CPUs
 * @ctl_fd:	fd of control path, /dev/virtio-ports/agent-ctl-path
 * @rw_ti:	structure managing information of read/write threads
 */
#[repr(C)]
pub struct agent_info {
    pub pipe_size: libc::c_ulong,
    pub use_stdout: bool,
    pub cpus: libc::c_int,
    pub ctl_fd: libc::c_int,
    pub rw_ti: [*mut rw_thread_info; MAX_CPUS],
}

/*
 * rw_thread_info - structure managing a read/write thread a cpu
 * @cpu_num:	cpu number operating this read/write thread
 * @in_fd:	fd of reading trace data path in cpu_num
 * @out_fd:	fd of writing trace data path in cpu_num
 * @read_pipe:	fd of read pipe
 * @write_pipe:	fd of write pipe
 * @pipe_size:	size of pipe (default 1MB)
 */
#[repr(C)]
pub struct rw_thread_info {
    pub cpu_num: libc::c_int,
    pub in_fd: libc::c_int,
    pub out_fd: libc::c_int,
    pub read_pipe: libc::c_int,
    pub write_pipe: libc::c_int,
    pub pipe_size: libc::c_ulong,
}

unsafe extern "C" {
    /* use for stopping rw threads */
    pub static mut global_sig_receive: bool;

    /* use for notification */
    pub static mut global_run_operation: bool;
    pub static mut mutex_notify: libc::pthread_mutex_t;
    pub static mut cond_wakeup: libc::pthread_cond_t;

    /* for controller of read/write threads */
    pub fn rw_ctl_init(ctl_path: *const libc::c_char) -> libc::c_int;
    pub fn rw_ctl_loop(ctl_fd: libc::c_int) -> *mut libc::c_void;

    /* for trace read/write thread */
    pub fn rw_thread_info_new() -> *mut libc::c_void;
    pub fn rw_thread_init(
        cpu: libc::c_int,
        in_path: *const libc::c_char,
        out_path: *const libc::c_char,
        stdout_flag: bool,
        pipe_size: libc::c_ulong,
        rw_ti: *mut rw_thread_info,
    ) -> *mut libc::c_void;
    pub fn rw_thread_run(rw_ti: *mut rw_thread_info) -> libc::pthread_t;
}

#[inline]
pub unsafe fn zalloc(size: libc::size_t) -> *mut libc::c_void {
    unsafe { libc::calloc(1, size) }
}

#[macro_export]
macro_rules! pr_err {
    ($($arg:tt)*) => {
        eprint!($($arg)*)
    };
}

#[macro_export]
macro_rules! pr_info {
    ($($arg:tt)*) => {
        print!($($arg)*)
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! pr_debug {
    ($($arg:tt)*) => {
        eprint!($($arg)*)
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! pr_debug {
    ($($arg:tt)*) => {};
}
