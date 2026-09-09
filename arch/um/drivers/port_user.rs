// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2001 - 2007 Jeff Dike (jdike@{linux.intel,addtoit}.com)
 */

// C dependencies supplied by the surrounding UML sources and libc are intentionally
// left as external declarations.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct termios {
    _private: [u8; 0],
}

#[repr(C)]
pub struct chan_opts {
    pub raw: c_int,
}

#[repr(C)]
pub struct chan_ops {
    pub type_: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut c_char, c_int, *const chan_opts) -> *mut c_void>,
    pub open: Option<unsafe extern "C" fn(c_int, c_int, c_int, *mut c_void, *mut *mut c_char) -> c_int>,
    pub close: Option<unsafe extern "C" fn(c_int, *mut c_void)>,
    pub read: Option<unsafe extern "C" fn(c_int, *mut c_void, *mut c_void, c_int) -> c_int>,
    pub write: Option<unsafe extern "C" fn(c_int, *mut c_void, *const c_void, c_int) -> c_int>,
    pub console_write: Option<unsafe extern "C" fn(c_int, *mut c_void, *const c_void, c_int) -> c_int>,
    pub window_size: Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
    pub winch: c_int,
}

extern "C" {
    fn port_data(port: c_int) -> *mut c_void;
    fn port_kern_free(data: *mut c_void);
    fn port_wait(data: *mut c_void) -> c_int;
    fn port_remove_dev(data: *mut c_void);
    fn uml_kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn raw(fd: c_int) -> c_int;
    fn tcgetattr(fd: c_int, termios_p: *mut termios) -> c_int;
    fn os_close_file(fd: c_int);
    fn generic_read(fd: c_int, data: *mut c_void, buf: *mut c_void, len: c_int) -> c_int;
    fn generic_write(fd: c_int, data: *mut c_void, buf: *const c_void, len: c_int) -> c_int;
    fn generic_console_write(fd: c_int, data: *mut c_void, buf: *const c_void, len: c_int) -> c_int;
    fn generic_window_size(fd: c_int, data: *mut c_void) -> c_int;
    fn os_set_fd_block(fd: c_int, block: c_int) -> c_int;
    fn os_access(path: *const c_char, mode: c_int) -> c_int;
    fn os_pipe(fds: *mut c_int, close_on_exec: c_int, nonblock: c_int) -> c_int;
    fn run_helper(pre_exec: unsafe extern "C" fn(*mut c_void), arg: *mut c_void, argv: *mut *mut c_char) -> c_int;
}

#[repr(C)]
struct port_chan {
    raw: c_int,
    tt: termios,
    kernel_data: *mut c_void,
    dev: [c_char; 7],
}

unsafe extern "C" fn port_init(str_: *mut c_char, _device: c_int, opts: *const chan_opts) -> *mut c_void {
    if *str_ != b':' as c_char { return core::ptr::null_mut(); }
    let str_ = str_.add(1);
    let port = libc_strtoul(str_, core::ptr::null_mut(), 0) as c_int;
    let kern_data = port_data(port);
    if kern_data.is_null() { return core::ptr::null_mut(); }
    let data = uml_kmalloc(core::mem::size_of::<port_chan>(), 0) as *mut port_chan;
    if data.is_null() { port_kern_free(kern_data); return core::ptr::null_mut(); }
    (*data).raw = (*opts).raw;
    (*data).tt = core::mem::zeroed();
    (*data).kernel_data = kern_data;
    (*data).dev = [0; 7];
    libc_snprintf((*data).dev.as_mut_ptr(), 7, port);
    data as *mut c_void
}

unsafe extern "C" fn port_free(d: *mut c_void) {
    let data = d as *mut port_chan;
    port_kern_free((*data).kernel_data);
    kfree(data as *mut c_void);
}

unsafe extern "C" fn port_open(_input: c_int, _output: c_int, _primary: c_int, d: *mut c_void, dev_out: *mut *mut c_char) -> c_int {
    let data = d as *mut port_chan;
    let fd = port_wait((*data).kernel_data);
    if fd >= 0 && (*data).raw != 0 {
        let err = tcgetattr(fd, &mut (*data).tt);
        if err != 0 { return err; }
        let err = raw(fd);
        if err != 0 { return err; }
    }
    *dev_out = (*data).dev.as_mut_ptr();
    fd
}

unsafe extern "C" fn port_close(fd: c_int, d: *mut c_void) {
    let data = d as *mut port_chan;
    port_remove_dev((*data).kernel_data);
    os_close_file(fd);
}

pub static port_ops: chan_ops = chan_ops {
    type_: b"port\0".as_ptr() as *const c_char,
    init: Some(port_init), open: Some(port_open), close: Some(port_close),
    read: Some(generic_read), write: Some(generic_write),
    console_write: Some(generic_console_write), window_size: Some(generic_window_size),
    free: Some(port_free), winch: 1,
};

extern "C" { fn libc_strtoul(s: *mut c_char, end: *mut *mut c_char, base: c_int) -> c_uint; fn libc_snprintf(buf: *mut c_char, size: usize, value: c_int); }

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: u32,
    sin_zero: [u8; 8],
}

extern "C" {
    fn socket(domain: c_int, kind: c_int, protocol: c_int) -> c_int;
    fn setsockopt(fd: c_int, level: c_int, name: c_int, value: *const c_void, len: u32) -> c_int;
    fn htons(value: u16) -> u16;
    fn htonl(value: u32) -> u32;
    fn bind(fd: c_int, addr: *const c_void, len: u32) -> c_int;
    fn listen(fd: c_int, backlog: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn accept(fd: c_int, addr: *mut c_void, len: *mut u32) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn shutdown(fd: c_int, how: c_int) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
}

pub unsafe extern "C" fn port_listen_fd(port: c_int) -> c_int {
    let fd = socket(2, 1, 0);
    if fd == -1 { return -1; }
    let arg: c_int = 1;
    if setsockopt(fd, 1, 2, &arg as *const _ as *const c_void, 4) < 0 {
        let err = -1; close(fd); return err;
    }
    let addr = sockaddr_in { sin_family: 2, sin_port: htons(port as u16), sin_addr: htonl(0), sin_zero: [0; 8] };
    if bind(fd, &addr as *const _ as *const c_void, core::mem::size_of::<sockaddr_in>() as u32) < 0 {
        let err = -1; close(fd); return err;
    }
    if listen(fd, 1) < 0 { let err = -1; close(fd); return err; }
    let err = os_set_fd_block(fd, 0);
    if err < 0 { close(fd); return err; }
    fd
}

#[repr(C)]
struct port_pre_exec_data { sock_fd: c_int, pipe_fd: c_int }

unsafe extern "C" fn port_pre_exec(arg: *mut c_void) {
    let data = arg as *mut port_pre_exec_data;
    dup2((*data).sock_fd, 0); dup2((*data).sock_fd, 1); dup2((*data).sock_fd, 2);
    close((*data).sock_fd); dup2((*data).pipe_fd, 3); shutdown(3, 0); close((*data).pipe_fd);
}

pub unsafe extern "C" fn port_connection(fd: c_int, socket_: *mut c_int, pid_out: *mut c_int) -> c_int {
    let mut argv: [*mut c_char; 4] = [b"in.telnetd\0".as_ptr() as *mut c_char, b"-L\0".as_ptr() as *mut c_char, b"/uml/port-helper\0".as_ptr() as *mut c_char, core::ptr::null_mut()];
    let env = getenv(b"UML_PORT_HELPER\0".as_ptr() as *const c_char);
    if !env.is_null() { argv[2] = env; }
    let new = accept(fd, core::ptr::null_mut(), core::ptr::null_mut());
    if new < 0 { return -1; }
    if os_access(argv[2], 1) < 0 { close(new); return -1; }
    if os_pipe(socket_, 0, 0) < 0 { close(new); return -1; }
    let mut data = port_pre_exec_data { sock_fd: new, pipe_fd: *socket_.add(1) };
    let err = run_helper(port_pre_exec, &mut data as *mut _ as *mut c_void, argv.as_mut_ptr());
    if err < 0 { shutdown(*socket_, 2); close(*socket_); shutdown(*socket_.add(1), 2); close(*socket_.add(1)); close(new); return err; }
    *pid_out = err; new
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
