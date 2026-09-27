// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2001 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// C dependencies supplied by the surrounding UML tree are intentionally left
// as external Rust symbols.

#[repr(C)]
pub struct XtermChan {
    pub pid: kernel::ffi::c_int,
    pub helper_pid: kernel::ffi::c_int,
    pub chan_fd: kernel::ffi::c_int,
    pub title: *mut kernel::ffi::c_char,
    pub device: kernel::ffi::c_int,
    pub raw: kernel::ffi::c_int,
    pub tt: libc::termios,
}

#[repr(C)]
pub struct ChanOpts {
    pub xterm_title: *mut kernel::ffi::c_char,
    pub raw: kernel::ffi::c_int,
}

extern "C" {
    static mut CONFIG_XTERM_CHAN_DEFAULT_EMULATOR: *mut kernel::ffi::c_char;
    fn uml_kmalloc(size: usize, flags: kernel::ffi::c_int) -> *mut kernel::ffi::c_void;
    fn strchr(s: *mut kernel::ffi::c_char, c: kernel::ffi::c_int) -> *mut kernel::ffi::c_char;
    fn access(path: *const kernel::ffi::c_char, mode: kernel::ffi::c_int) -> kernel::ffi::c_int;
    fn getenv(name: *const kernel::ffi::c_char) -> *mut kernel::ffi::c_char;
    fn mkstemp(template: *mut kernel::ffi::c_char) -> kernel::ffi::c_int;
    fn unlink(path: *const kernel::ffi::c_char) -> kernel::ffi::c_int;
    fn close(fd: kernel::ffi::c_int) -> kernel::ffi::c_int;
    fn sprintf(dst: *mut kernel::ffi::c_char, fmt: *const kernel::ffi::c_char, ...) -> kernel::ffi::c_int;
    fn printk(fmt: *const kernel::ffi::c_char, ...);
    fn os_create_unix_socket(path: *const kernel::ffi::c_char, len: kernel::ffi::c_int, close_on_exec: kernel::ffi::c_int) -> kernel::ffi::c_int;
    fn run_helper(a: *mut kernel::ffi::c_void, b: *mut kernel::ffi::c_void, argv: *mut *mut kernel::ffi::c_char) -> kernel::ffi::c_int;
    fn os_set_fd_block(fd: kernel::ffi::c_int, block: kernel::ffi::c_int) -> kernel::ffi::c_int;
    fn xterm_fd(fd: kernel::ffi::c_int, helper_pid: *mut kernel::ffi::c_int) -> kernel::ffi::c_int;
    fn tcgetattr(fd: kernel::ffi::c_int, termios_p: *mut libc::termios) -> kernel::ffi::c_int;
    fn raw(fd: kernel::ffi::c_int) -> kernel::ffi::c_int;
    fn os_kill_process(pid: kernel::ffi::c_int, reap: kernel::ffi::c_int);
    fn os_close_file(fd: kernel::ffi::c_int);
    fn generic_read(fd: kernel::ffi::c_int, data: *mut kernel::ffi::c_void, len: usize) -> isize;
    fn generic_write(fd: kernel::ffi::c_int, data: *const kernel::ffi::c_void, len: usize) -> isize;
    fn generic_console_write(fd: kernel::ffi::c_int, data: *const kernel::ffi::c_void, len: usize);
    fn generic_window_size(fd: kernel::ffi::c_int, data: *mut kernel::ffi::c_void);
    fn generic_free(data: *mut kernel::ffi::c_void);

    static mut errno: kernel::ffi::c_int;
}

static mut TERMINAL_EMULATOR: *mut kernel::ffi::c_char = core::ptr::null_mut();
static mut TITLE_SWITCH: *mut kernel::ffi::c_char = b"-T\0".as_ptr() as *mut kernel::ffi::c_char;
static mut EXEC_SWITCH: *mut kernel::ffi::c_char = b"-e\0".as_ptr() as *mut kernel::ffi::c_char;

pub unsafe extern "C" fn xterm_init(
    _str: *mut kernel::ffi::c_char,
    device: kernel::ffi::c_int,
    opts: *const ChanOpts,
) -> *mut kernel::ffi::c_void {
    let data = uml_kmalloc(core::mem::size_of::<XtermChan>(), UM_GFP_KERNEL) as *mut XtermChan;
    if data.is_null() { return core::ptr::null_mut(); }
    (*data) = XtermChan { pid: -1, helper_pid: -1, chan_fd: -1,
        title: (*opts).xterm_title, device, raw: (*opts).raw, tt: core::mem::zeroed() };
    data as *mut kernel::ffi::c_void
}

pub unsafe extern "C" fn xterm_setup(mut line: *mut kernel::ffi::c_char, add: *mut kernel::ffi::c_int) -> kernel::ffi::c_int {
    *add = 0;
    TERMINAL_EMULATOR = line;
    line = strchr(line, b',' as kernel::ffi::c_int);
    if line.is_null() { return 0; }
    *line = 0; line = line.add(1);
    if *line != 0 { TITLE_SWITCH = line; }
    line = strchr(line, b',' as kernel::ffi::c_int);
    if line.is_null() { return 0; }
    *line = 0; line = line.add(1);
    if *line != 0 { EXEC_SWITCH = line; }
    0
}

// __uml_setup("xterm=", xterm_setup, <the source's help text>).

pub unsafe extern "C" fn xterm_open(input: kernel::ffi::c_int, output: kernel::ffi::c_int, primary: kernel::ffi::c_int,
                                     d: *mut kernel::ffi::c_void, dev_out: *mut *mut kernel::ffi::c_char) -> kernel::ffi::c_int {
    let data = d as *mut XtermChan;
    let mut title = [0i8; 256];
    let mut file = *b"/tmp/xterm-pipeXXXXXX\0";
    let mut helper = b"/usr/lib/uml/port-helper\0".as_ptr() as *mut kernel::ffi::c_char;
    let mut argv = [TERMINAL_EMULATOR, TITLE_SWITCH, title.as_mut_ptr(), EXEC_SWITCH,
        helper, b"-uml-socket\0".as_ptr() as *mut kernel::ffi::c_char, file.as_mut_ptr(), core::ptr::null_mut()];
    if access(argv[4], X_OK) < 0 { argv[4] = b"port-helper\0".as_ptr() as *mut kernel::ffi::c_char; }
    if getenv(b"DISPLAY\0".as_ptr() as *const kernel::ffi::c_char).is_null() && getenv(b"WAYLAND_DISPLAY\0".as_ptr() as *const kernel::ffi::c_char).is_null() { return -ENODEV; }
    let mut fd = mkstemp(file.as_mut_ptr());
    if fd < 0 { return -errno; }
    if unlink(file.as_ptr()) != 0 { let e = -errno; close(fd); return e; }
    close(fd);
    fd = os_create_unix_socket(file.as_ptr(), file.len() as kernel::ffi::c_int, 1);
    if fd < 0 { return fd; }
    sprintf(title.as_mut_ptr(), (*data).title, (*data).device);
    let pid = run_helper(core::ptr::null_mut(), core::ptr::null_mut(), argv.as_mut_ptr());
    if pid < 0 { close(fd); return pid; }
    let mut err = os_set_fd_block(fd, 0);
    if err < 0 { os_kill_process(pid, 1); close(fd); return err; }
    (*data).chan_fd = fd;
    let new = xterm_fd(fd, &mut (*data).helper_pid);
    if new < 0 { os_kill_process(pid, 1); close(fd); return new; }
    err = os_set_fd_block(new, 0);
    if err != 0 { close(new); os_kill_process(pid, 1); close(fd); return err; }
    err = tcgetattr(new, &mut (*data).tt);
    if err != 0 { close(new); os_kill_process(pid, 1); close(fd); return err; }
    if (*data).raw != 0 { err = raw(new); if err != 0 { close(new); os_kill_process(pid, 1); close(fd); return err; } }
    unlink(file.as_ptr()); (*data).pid = pid; *dev_out = core::ptr::null_mut(); new
}

pub unsafe extern "C" fn xterm_close(fd: kernel::ffi::c_int, d: *mut kernel::ffi::c_void) {
    let data = d as *mut XtermChan;
    if (*data).pid != -1 { os_kill_process((*data).pid, 1); } (*data).pid = -1;
    if (*data).helper_pid != -1 { os_kill_process((*data).helper_pid, 0); } (*data).helper_pid = -1;
    if (*data).chan_fd != -1 { os_close_file((*data).chan_fd); }
    os_close_file(fd);
}

#[repr(C)]
pub struct ChanOps {
    pub type_: *const kernel::ffi::c_char,
    pub init: Option<unsafe extern "C" fn(*mut kernel::ffi::c_char, kernel::ffi::c_int, *const ChanOpts) -> *mut kernel::ffi::c_void>,
    pub open: Option<unsafe extern "C" fn(kernel::ffi::c_int, kernel::ffi::c_int, kernel::ffi::c_int, *mut kernel::ffi::c_void, *mut *mut kernel::ffi::c_char) -> kernel::ffi::c_int>,
    pub close: Option<unsafe extern "C" fn(kernel::ffi::c_int, *mut kernel::ffi::c_void)>,
    pub read: Option<unsafe extern "C" fn()>,
    pub write: Option<unsafe extern "C" fn()>,
    pub console_write: Option<unsafe extern "C" fn()>,
    pub window_size: Option<unsafe extern "C" fn()>,
    pub free: Option<unsafe extern "C" fn()>,
    pub winch: kernel::ffi::c_int,
}

pub static mut XTERM_OPS: ChanOps = ChanOps {
    type_: b"xterm\0".as_ptr() as *const kernel::ffi::c_char,
    init: Some(xterm_init), open: Some(xterm_open), close: Some(xterm_close),
    read: None, write: None, console_write: None, window_size: None, free: None, winch: 1,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
