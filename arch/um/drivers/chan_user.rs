// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{linux.intel,addtoit}.com)
 */

// Dependencies supplied by the surrounding UML implementation.

use core::ffi::c_void;

#[repr(C)]
pub struct tty_port {
    _private: [u8; 0],
}

#[repr(C)]
struct winsize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}

#[repr(C)]
struct termios {
    _private: [u8; 0],
}

extern "C" {
    fn close(fd: i32) -> i32;
    fn read(fd: i32, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: i32, buf: *const c_void, count: usize) -> isize;
    fn ioctl(fd: i32, request: usize, ...) -> i32;
    fn isatty(fd: i32) -> i32;
    fn tcgetattr(fd: i32, termios_p: *mut termios) -> i32;
    fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *const termios) -> i32;
    fn tcgetpgrp(fd: i32) -> i32;
    fn sigemptyset(set: *mut c_void) -> i32;
    fn sigaddset(set: *mut c_void, signum: i32) -> i32;
    fn sigprocmask(how: i32, set: *const c_void, oldset: *mut c_void) -> i32;
    fn signal(signum: i32, handler: extern "C" fn(i32));
    fn sigfillset(set: *mut c_void) -> i32;
    fn sigdelset(set: *mut c_void, signum: i32) -> i32;
    fn sigsuspend(mask: *const c_void) -> i32;
    fn setsid() -> i32;
    fn tcsetpgrp(fd: i32, pgrp: i32) -> i32;
    fn pause() -> i32;
    fn kfree(data: *mut c_void);
    fn os_set_pdeathsig();
    fn os_info(fmt: *const u8, ...);
    fn os_getpid() -> i32;
    fn os_pipe(fds: *mut i32, close_on_exec: i32, nonblock: i32) -> i32;
    fn run_helper_thread(
        thread: unsafe extern "C" fn(*mut c_void) -> !,
        arg: *mut c_void,
        flags: i32,
        stack_out: *mut usize,
    ) -> i32;
    fn os_set_fd_block(fd: i32, block: i32) -> i32;
    fn is_skas_winch(pid: i32, fd: i32, port: *mut tty_port) -> i32;
    fn register_winch_irq(thread_fd: i32, fd: i32, thread: i32, port: *mut tty_port, stack: usize);
    fn printk(fmt: *const u8, ...);
}

const EIO: i32 = 5;
const EAGAIN: i32 = 11;
const EINTR: i32 = 4;
const EINVAL: i32 = 22;
const SIGIO: i32 = 29;
const SIGWINCH: i32 = 28;
const SIG_BLOCK: i32 = 0;
const SIG_SETMASK: i32 = 2;
const TCSAFLUSH: i32 = 2;
const OPOST: u32 = 1;
const TIOCGWINSZ: usize = 0x5413;
const TIOCSCTTY: usize = 0x540e;
const CLONE_FILES: i32 = 0x00000400;

static mut ERRNO: i32 = 0;

pub unsafe extern "C" fn generic_close(fd: i32, _unused: *mut c_void) {
    close(fd);
}

pub unsafe extern "C" fn generic_read(fd: i32, c_out: *mut u8, _unused: *mut c_void) -> i32 {
    let n = read(fd, c_out as *mut c_void, core::mem::size_of::<u8>()) as i32;
    if n > 0 { n } else if n == 0 { -EIO } else if ERRNO == EAGAIN { 0 } else { -ERRNO }
}

pub unsafe extern "C" fn generic_write(fd: i32, buf: *const u8, n: usize, _unused: *mut c_void) -> i32 {
    let mut written: usize = 0;
    let mut err: i32;
    loop {
        ERRNO = 0;
        err = write(fd, buf.add(written) as *const c_void, n - written) as i32;
        if err > 0 { written += err as usize; continue; }
        if !(err < 0 && ERRNO == EINTR) { break; }
    }
    if written > 0 { written as i32 } else if ERRNO == EAGAIN { 0 } else if err == 0 { -EIO } else { -ERRNO }
}

pub unsafe extern "C" fn generic_window_size(fd: i32, _unused: *mut c_void, rows_out: *mut u16, cols_out: *mut u16) -> i32 {
    let mut size = winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };
    if ioctl(fd, TIOCGWINSZ, &mut size) < 0 { return -ERRNO; }
    let ret = ((*rows_out != size.ws_row) || (*cols_out != size.ws_col)) as i32;
    *rows_out = size.ws_row;
    *cols_out = size.ws_col;
    ret
}

pub unsafe extern "C" fn generic_free(data: *mut c_void) { kfree(data); }

pub unsafe extern "C" fn generic_console_write(fd: i32, buf: *const i8, n: i32) -> i32 {
    let mut save = termios { _private: [] };
    let mut new = termios { _private: [] };
    if isatty(fd) != 0 {
        // Terminal signal-mask and termios manipulation are supplied by libc/UML.
        if sigprocmask(SIG_BLOCK, core::ptr::null(), core::ptr::null()) != 0 { return -ERRNO; }
        if tcgetattr(fd, &mut save) != 0 { return -ERRNO; }
        new = save;
        let _ = OPOST;
        if tcsetattr(fd, TCSAFLUSH, &new) != 0 { return -ERRNO; }
    }
    let err = generic_write(fd, buf as *const u8, n as usize, core::ptr::null_mut());
    if isatty(fd) != 0 { let _ = tcsetattr(fd, TCSAFLUSH, &save); }
    err
}

extern "C" fn winch_handler(_sig: i32) {}

#[repr(C)]
struct winch_data { pty_fd: i32, pipe_fd: i32 }

unsafe extern "C" fn winch_thread(arg: *mut c_void) -> ! {
    let data = &*(arg as *mut winch_data);
    let pty_fd = data.pty_fd;
    let pipe_fd = data.pipe_fd;
    let mut c: u8 = 1;
    os_set_pdeathsig();
    let count = write(pipe_fd, &c as *const u8 as *const c_void, 1);
    if count != 1 { os_info(b"winch_thread : failed to write synchronization byte, err = %d\n\0".as_ptr(), -count as i32); }
    signal(SIGWINCH, winch_handler);
    let mut sigs = [0u8; 128];
    sigfillset(sigs.as_mut_ptr() as *mut c_void);
    if sigprocmask(SIG_SETMASK, sigs.as_ptr() as *const c_void, core::ptr::null_mut()) < 0 { goto_wait_kill(); }
    sigdelset(sigs.as_mut_ptr() as *mut c_void, SIGWINCH);
    if setsid() < 0 || ioctl(pty_fd, TIOCSCTTY, 0) < 0 || tcsetpgrp(pty_fd, os_getpid()) < 0 { goto_wait_kill(); }
    let _ = read(pipe_fd, &mut c as *mut u8 as *mut c_void, 1);
    loop { sigsuspend(sigs.as_ptr() as *const c_void); let _ = write(pipe_fd, &c as *const u8 as *const c_void, 1); }
}

unsafe fn goto_wait_kill() -> ! { let c: u8 = 2; loop { pause(); let _ = c; } }

unsafe fn winch_tramp(fd: i32, _port: *mut tty_port, fd_out: *mut i32, stack_out: *mut usize) -> i32 {
    let mut fds = [0i32; 2];
    let mut data = winch_data { pty_fd: fd, pipe_fd: 0 };
    let mut c: u8 = 0;
    let mut err = os_pipe(fds.as_mut_ptr(), 1, 1);
    if err < 0 { return err; }
    data.pipe_fd = fds[1];
    let pid = run_helper_thread(winch_thread, &mut data as *mut _ as *mut c_void, CLONE_FILES, stack_out);
    if pid < 0 { err = pid; close(fds[1]); close(fds[0]); return err; }
    *fd_out = fds[0];
    let n = read(fds[0], &mut c as *mut u8 as *mut c_void, 1);
    if n != 1 { close(fds[1]); close(fds[0]); return -EINVAL; }
    err = os_set_fd_block(*fd_out, 0);
    if err != 0 { close(fds[1]); close(fds[0]); return err; }
    pid
}

pub unsafe extern "C" fn register_winch(fd: i32, port: *mut tty_port) {
    if isatty(fd) == 0 { return; }
    let pid = tcgetpgrp(fd);
    if is_skas_winch(pid, fd, port) != 0 { register_winch_irq(-1, fd, -1, port, 0); }
    else if pid == -1 {
        let mut stack = 0usize;
        let mut thread_fd = -1i32;
        let thread = winch_tramp(fd, port, &mut thread_fd, &mut stack);
        if thread >= 0 {
            register_winch_irq(thread_fd, fd, thread, port, stack);
            let c: u8 = 1;
            let _ = write(thread_fd, &c as *const u8 as *const c_void, 1);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
