// SPDX-License-Identifier: GPL-2.0

// C dependencies: linux/compiler.h, stdio.h, stdlib.h, sys/prctl.h,
// sys/wait.h, unistd.h, and ../tests.h.

use core::ffi::{c_char, c_int, c_void};

type PidT = c_int;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn atoi(nptr: *const c_char) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> PidT;
    fn close(fd: c_int) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn kill(pid: PidT, sig: c_int) -> c_int;
    fn waitpid(pid: PidT, wstatus: *mut c_int, options: c_int) -> PidT;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
}

const PR_SET_NAME: c_int = 15;
const SIGKILL: c_int = 9;

static mut loops: c_int = 100;
static mut buf: c_char = 0;

#[no_mangle]
pub static mut context_switch_loop_work: c_int = 1234;

unsafe fn write_block(fd: c_int) -> c_int {
    if unsafe { write(fd, core::ptr::addr_of!(buf) as *const c_void, 1) } <= 0 {
        return 1;
    }
    0
}

unsafe fn read_block(fd: c_int) -> c_int {
    if unsafe { read(fd, core::ptr::addr_of_mut!(buf) as *mut c_void, 1) } <= 0 {
        return 1;
    }
    0
}

/* Not static to avoid LTO clobbering the function name */
#[no_mangle]
pub unsafe extern "C" fn context_switch_loop_proc1(in_fd: c_int, out_fd: c_int) -> c_int {
    let mut i: c_int = 0;
    while i < unsafe { loops } {
        if unsafe { read_block(in_fd) } != 0 {
            return 1;
        }
        unsafe {
            context_switch_loop_work += i * 3;
        }
        if unsafe { write_block(out_fd) } != 0 {
            return 1;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn context_switch_loop_proc2(in_fd: c_int, out_fd: c_int) -> c_int {
    let mut i: c_int = 0;
    while i < unsafe { loops } {
        if unsafe { write_block(out_fd) } != 0 {
            return 1;
        }
        unsafe {
            context_switch_loop_work += i * 7;
        }
        if unsafe { read_block(in_fd) } != 0 {
            return 1;
        }
        i += 1;
    }
    0
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

/*
 * Launches two processes that take turns to execute a multiplication N times
 */
unsafe extern "C" fn context_switch_loop(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut a_to_b: [c_int; 2] = [0; 2];
    let mut b_to_a: [c_int; 2] = [0; 2];
    let proc1_pid: PidT;
    let mut status: c_int = 0;
    let mut ret: c_int;

    if argc > 0 {
        unsafe {
            loops = atoi(*argv);
            if loops < 0 {
                fprintf(
                    stderr,
                    b"Invalid number of loops: %s\n\0".as_ptr() as *const c_char,
                    *argv,
                );
                return 1;
            }
        }
    }

    if unsafe { pipe(a_to_b.as_mut_ptr()) } != 0 || unsafe { pipe(b_to_a.as_mut_ptr()) } != 0 {
        unsafe {
            perror(b"Pipe error\0".as_ptr() as *const c_char);
        }
        return 1;
    }

    proc1_pid = unsafe { fork() };
    if proc1_pid < 0 {
        unsafe {
            perror(b"Fork error\0".as_ptr() as *const c_char);
        }
        return 1;
    }

    if proc1_pid == 0 {
        unsafe {
            close(a_to_b[0]);
            close(b_to_a[1]);
            prctl(
                PR_SET_NAME,
                b"proc1\0".as_ptr() as *const c_char,
                0,
                0,
                0,
            );
            ret = context_switch_loop_proc1(b_to_a[0], a_to_b[1]);
            close(a_to_b[1]);
            close(b_to_a[0]);
            exit(ret);
        }
    }

    unsafe {
        close(a_to_b[1]);
        close(b_to_a[0]);
        prctl(
            PR_SET_NAME,
            b"proc2\0".as_ptr() as *const c_char,
            0,
            0,
            0,
        );
        ret = context_switch_loop_proc2(a_to_b[0], b_to_a[1]);
        close(a_to_b[0]);
        close(b_to_a[1]);
    }

    if ret != 0 {
        unsafe {
            kill(proc1_pid, SIGKILL);
        }
        return ret;
    }

    if unsafe { waitpid(proc1_pid, &mut status, 0) } != proc1_pid
        || !WIFEXITED(status)
        || WEXITSTATUS(status) != 0
    {
        return 1;
    }

    0
}

DEFINE_WORKLOAD!(context_switch_loop);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
