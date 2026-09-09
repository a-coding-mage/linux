// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000, 2001 Jeff Dike (jdike@karaya.com)
 */

// Linux/UML dependencies supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const MAX_TTYS: usize = 16;

extern "C" {
    fn printk(fmt: *const c_char, ...);
    fn line_config(vts: *mut line, count: usize, str_: *mut c_char,
                   opts: *mut chan_opts, error_out: *mut *mut c_char) -> c_int;
    fn line_get_config(dev: *mut c_char, vts: *mut line, count: usize,
                       str_: *mut c_char, size: c_int,
                       error_out: *mut *mut c_char) -> c_int;
    fn line_remove(vts: *mut line, count: usize, n: c_int,
                   error_out: *mut *mut c_char) -> c_int;
    fn line_install(driver: *mut tty_driver, tty: *mut tty_struct,
                    line: *mut line) -> c_int;
    fn console_write_chan(chan_out: *mut c_void, string: *const c_char, len: c_uint);
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: c_ulong);
    fn console_open_chan(line: *mut line, console: *mut console) -> c_int;
    fn register_lines(driver: *mut line_driver, ops: *const tty_operations,
                      vts: *mut line, count: usize) -> c_int;
    fn add_xterm_umid(title: *mut c_char) -> *mut c_char;
    fn setup_one_line(vts: *mut line, index: c_int, str_: *mut c_char,
                      opts: *mut chan_opts, error: *mut *mut c_char) -> c_int;
    fn register_console(console: *mut console);
    fn close_lines(vts: *mut line, count: usize);
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn line_setup(vt_conf: *mut *mut c_char, count: usize,
                  def_conf: *mut *mut c_char, str_: *mut c_char,
                  name: *const c_char);
    fn line_id() -> c_int;
    fn line_open(); fn line_close(); fn line_write(); fn line_write_room(); fn line_chars_in_buffer(); fn line_flush_buffer(); fn line_flush_chars(); fn line_throttle(); fn line_unthrottle(); fn line_hangup();

    static CONFIG_CON_CHAN: *mut c_char;
    static CONFIG_CON_ZERO_CHAN: *mut c_char;
}

#[repr(C)] pub struct chan_opts {
    pub announce: Option<unsafe extern "C" fn(*mut c_char, c_int)>,
    pub xterm_title: *mut c_char,
    pub raw: c_int,
}
#[repr(C)] pub struct line_driver { pub driver: *mut tty_driver, pub mc: mc_data, pub name: *mut c_char, pub device_name: *mut c_char, pub major: c_int, pub minor_start: c_int, pub type_: c_int, pub subtype: c_int, pub read_irq_name: *mut c_char, pub write_irq_name: *mut c_char }
#[repr(C)] pub struct mc_data { pub list: list_head, pub name: *mut c_char, pub config: Option<unsafe extern "C" fn(*mut c_char, *mut *mut c_char) -> c_int>, pub get_config: Option<unsafe extern "C" fn(*mut c_char, *mut c_char, c_int, *mut *mut c_char) -> c_int>, pub id: Option<unsafe extern "C" fn() -> c_int>, pub remove: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int> }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct line { pub lock: c_void, pub chan_out: *mut c_void }
#[repr(C)] pub struct tty_driver { pub _private: c_void }
#[repr(C)] pub struct tty_struct { pub index: usize }
#[repr(C)] pub struct tty_operations { pub open: Option<unsafe extern "C" fn()>, pub install: Option<unsafe extern "C" fn(*mut tty_driver, *mut tty_struct) -> c_int>, pub close: Option<unsafe extern "C" fn()>, pub write: Option<unsafe extern "C" fn()>, pub write_room: Option<unsafe extern "C" fn()>, pub chars_in_buffer: Option<unsafe extern "C" fn()>, pub flush_buffer: Option<unsafe extern "C" fn()>, pub flush_chars: Option<unsafe extern "C" fn()>, pub throttle: Option<unsafe extern "C" fn()>, pub unthrottle: Option<unsafe extern "C" fn()>, pub hangup: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct console { pub name: *mut c_char, pub write: Option<unsafe extern "C" fn(*mut console, *const c_char, c_uint)>, pub device: Option<unsafe extern "C" fn(*mut console, *mut c_int) -> *mut tty_driver>, pub setup: Option<unsafe extern "C" fn(*mut console, *mut c_char) -> c_int>, pub flags: c_int, pub index: c_int }

unsafe extern "C" fn stdio_announce(dev_name: *mut c_char, dev: c_int) {
    printk(b"Virtual console %d assigned device '%s'\n\0".as_ptr() as *const c_char, dev, dev_name);
}

static mut opts: chan_opts = chan_opts { announce: Some(stdio_announce), xterm_title: b"Virtual Console #%d\0".as_ptr() as *mut c_char, raw: 1 };
static mut vt_conf: [*mut c_char; MAX_TTYS] = [core::ptr::null_mut(); MAX_TTYS];
static mut def_conf: *mut c_char = core::ptr::null_mut();
static mut vts: [line; MAX_TTYS] = unsafe { core::mem::zeroed() };
static mut con_init_done: c_int = 0;
static mut driver: line_driver = line_driver { driver: core::ptr::null_mut(), mc: mc_data { list: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() }, name: b"con\0".as_ptr() as *mut c_char, config: Some(con_config), get_config: Some(con_get_config), id: Some(line_id), remove: Some(con_remove) }, name: b"UML console\0".as_ptr() as *mut c_char, device_name: b"tty\0".as_ptr() as *mut c_char, major: 0, minor_start: 0, type_: 0, subtype: 0, read_irq_name: b"console\0".as_ptr() as *mut c_char, write_irq_name: b"console-write\0".as_ptr() as *mut c_char };
static console_ops: tty_operations = tty_operations { open: Some(line_open), install: Some(con_install), close: Some(line_close), write: Some(line_write), write_room: Some(line_write_room), chars_in_buffer: Some(line_chars_in_buffer), flush_buffer: Some(line_flush_buffer), flush_chars: Some(line_flush_chars), throttle: Some(line_throttle), unthrottle: Some(line_unthrottle), hangup: Some(line_hangup) };
static mut stdiocons: console = console { name: b"tty\0".as_ptr() as *mut c_char, write: Some(uml_console_write), device: Some(uml_console_device), setup: Some(uml_console_setup), flags: 0, index: -1 };

unsafe extern "C" fn con_config(str_: *mut c_char, error_out: *mut *mut c_char) -> c_int { line_config(vts.as_mut_ptr(), MAX_TTYS, str_, &mut opts, error_out) }
unsafe extern "C" fn con_get_config(dev: *mut c_char, str_: *mut c_char, size: c_int, error_out: *mut *mut c_char) -> c_int { line_get_config(dev, vts.as_mut_ptr(), MAX_TTYS, str_, size, error_out) }
unsafe extern "C" fn con_remove(n: c_int, error_out: *mut *mut c_char) -> c_int { line_remove(vts.as_mut_ptr(), MAX_TTYS, n, error_out) }
unsafe extern "C" fn con_install(driver: *mut tty_driver, tty: *mut tty_struct) -> c_int { line_install(driver, tty, &mut vts[(*tty).index]) }

unsafe extern "C" fn uml_console_write(console: *mut console, string: *const c_char, len: c_uint) {
    let line = &mut vts[(*console).index as usize]; let mut flags = 0;
    spin_lock_irqsave(&mut line.lock, &mut flags); console_write_chan(line.chan_out, string, len); spin_unlock_irqrestore(&mut line.lock, flags);
}
unsafe extern "C" fn uml_console_device(c: *mut console, index: *mut c_int) -> *mut tty_driver { *index = (*c).index; core::ptr::null_mut() }
unsafe extern "C" fn uml_console_setup(co: *mut console, _options: *mut c_char) -> c_int { console_open_chan(&mut vts[(*co).index as usize], co) }

unsafe extern "C" fn stdio_init() -> c_int {
    let err = register_lines(&mut driver, &console_ops, vts.as_mut_ptr(), MAX_TTYS); if err != 0 { return err; }
    printk(b"Initialized stdio console driver\n\0".as_ptr() as *const c_char);
    let new_title = add_xterm_umid(opts.xterm_title); if !new_title.is_null() { opts.xterm_title = new_title; }
    for i in 0..MAX_TTYS { let mut error = core::ptr::null_mut(); let mut s = vt_conf[i]; if s.is_null() { s = def_conf; } if s.is_null() { s = if i != 0 { CONFIG_CON_CHAN } else { CONFIG_CON_ZERO_CHAN }; } if setup_one_line(vts.as_mut_ptr(), i as c_int, s, &mut opts, &mut error) != 0 { printk(b"setup_one_line failed for device %d : %s\n\0".as_ptr() as *const c_char, i as c_int, error); } }
    con_init_done = 1; register_console(&mut stdiocons); 0
}
// late_initcall(stdio_init)
unsafe extern "C" fn console_exit() { if con_init_done == 0 { return; } close_lines(vts.as_mut_ptr(), MAX_TTYS); }
// __uml_exitcall(console_exit)
unsafe extern "C" fn console_chan_setup(str_: *mut c_char) -> c_int { if strncmp(str_, b"sole=\0".as_ptr() as *const c_char, 5) == 0 { return 0; } line_setup(vt_conf.as_mut_ptr(), MAX_TTYS, &mut def_conf, str_, b"console\0".as_ptr() as *const c_char); 1 }
// __setup("con", console_chan_setup); __channel_help(console_chan_setup, "con")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
