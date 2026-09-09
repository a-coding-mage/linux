// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000, 2002 Jeff Dike (jdike@karaya.com)
 */

// C dependencies supplied by the surrounding kernel/UML sources:
// linux/fs.h, linux/tty.h, linux/tty_driver.h, linux/major.h, linux/mm.h,
// linux/init.h, linux/console.h, asm/termbits.h, asm/irq.h, chan.h, init.h,
// irq_user.h, and mconsole_kern.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong};

static ssl_version: c_int = 1;

const NR_PORTS: usize = 64;

unsafe extern "C" {
    fn printk(format: *const c_char, ...);
    fn line_config(lines: *mut line, count: usize, string: *mut c_char,
                   opts: *mut chan_opts, error_out: *mut *mut c_char) -> c_int;
    fn line_get_config(dev: *mut c_char, lines: *mut line, count: usize,
                       string: *mut c_char, size: c_int,
                       error_out: *mut *mut c_char) -> c_int;
    fn line_remove(lines: *mut line, count: usize, n: c_int,
                   error_out: *mut *mut c_char) -> c_int;
    fn line_install(driver: *mut tty_driver, tty: *mut tty_struct,
                    line: *mut line) -> c_int;
    fn line_open(tty: *mut tty_struct, file: *mut file) -> c_int;
    fn line_close(tty: *mut tty_struct, file: *mut file);
    fn line_write(tty: *mut tty_struct, buf: *const u8, count: c_int) -> c_int;
    fn line_write_room(tty: *mut tty_struct) -> c_int;
    fn line_chars_in_buffer(tty: *mut tty_struct) -> c_int;
    fn line_flush_buffer(tty: *mut tty_struct);
    fn line_flush_chars(tty: *mut tty_struct);
    fn line_throttle(tty: *mut tty_struct);
    fn line_unthrottle(tty: *mut tty_struct);
    fn line_hangup(tty: *mut tty_struct);
    fn console_write_chan(chan_out: *mut c_void, string: *const c_char, len: c_uint);
    fn console_open_chan(line: *mut line, console: *mut console) -> c_int;
    fn register_lines(driver: *mut line_driver, ops: *const tty_operations,
                      lines: *mut line, count: usize) -> c_int;
    fn add_xterm_umid(title: *mut c_char) -> *mut c_char;
    fn setup_one_line(lines: *mut line, index: c_int, config: *mut c_char,
                      opts: *mut chan_opts, error: *mut *mut c_char) -> c_int;
    fn register_console(console: *mut console);
    fn close_lines(lines: *mut line, count: usize);
    fn line_setup(conf: *mut *mut c_char, count: usize,
                  def_conf: *mut *mut c_char, string: *mut c_char,
                  name: *const c_char);
    fn line_id(_: *mut c_void) -> c_int;
}

use core::ffi::c_void;

unsafe fn ssl_announce(dev_name: *mut c_char, dev: c_int) {
    printk(b"Serial line %d assigned device '%s'\n\0".as_ptr() as *const c_char,
           dev, dev_name);
}

// Almost const, except that xterm_title may be changed in an initcall.
static mut opts: chan_opts = chan_opts {
    announce: Some(ssl_announce),
    xterm_title: b"Serial Line #%d\0".as_ptr() as *mut c_char,
    raw: 1,
};

// Const, except for .mc.list.
static mut driver: line_driver = line_driver {
    name: b"UML serial line\0".as_ptr() as *mut c_char,
    device_name: b"ttyS\0".as_ptr() as *mut c_char,
    major: TTY_MAJOR,
    minor_start: 64,
    type_: TTY_DRIVER_TYPE_SERIAL,
    subtype: 0,
    read_irq_name: b"ssl\0".as_ptr() as *mut c_char,
    write_irq_name: b"ssl-write\0".as_ptr() as *mut c_char,
    mc: mconsole_driver {
        list: LIST_HEAD_INIT,
        name: b"ssl\0".as_ptr() as *mut c_char,
        config: Some(ssl_config),
        get_config: Some(ssl_get_config),
        id: Some(line_id),
        remove: Some(ssl_remove),
    },
};

// The array is initialized by line_init, at initcall time. The elements are
// locked individually as needed.
static mut conf: [*mut c_char; NR_PORTS] = [core::ptr::null_mut(); NR_PORTS];
static mut def_conf: *mut c_char = CONFIG_SSL_CHAN;
static mut serial_lines: [line; NR_PORTS] = [line::ZERO; NR_PORTS];

unsafe fn ssl_config(string: *mut c_char, error_out: *mut *mut c_char) -> c_int {
    line_config(serial_lines.as_mut_ptr(), serial_lines.len(), string,
                &raw mut opts, error_out)
}

unsafe fn ssl_get_config(dev: *mut c_char, string: *mut c_char, size: c_int,
                         error_out: *mut *mut c_char) -> c_int {
    line_get_config(dev, serial_lines.as_mut_ptr(), serial_lines.len(), string,
                    size, error_out)
}

unsafe fn ssl_remove(n: c_int, error_out: *mut *mut c_char) -> c_int {
    line_remove(serial_lines.as_mut_ptr(), serial_lines.len(), n, error_out)
}

unsafe fn ssl_install(driver_: *mut tty_driver, tty: *mut tty_struct) -> c_int {
    line_install(driver_, tty, &mut serial_lines[(*tty).index as usize])
}

static ssl_ops: tty_operations = tty_operations {
    open: Some(line_open), close: Some(line_close), write: Some(line_write),
    write_room: Some(line_write_room), chars_in_buffer: Some(line_chars_in_buffer),
    flush_buffer: Some(line_flush_buffer), flush_chars: Some(line_flush_chars),
    throttle: Some(line_throttle), unthrottle: Some(line_unthrottle),
    install: Some(ssl_install), hangup: Some(line_hangup),
};

// Changed by ssl_init and referenced by ssl_exit; both are serialized by
// being an initcall and exitcall, respectively.
static mut ssl_init_done: c_int = 0;

unsafe fn ssl_console_write(c: *mut console, string: *const c_char, len: c_uint) {
    let line = &mut serial_lines[(*c).index as usize];
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut line.lock, &mut flags);
    console_write_chan(line.chan_out, string, len);
    spin_unlock_irqrestore(&mut line.lock, flags);
}

unsafe fn ssl_console_device(c: *mut console, index: *mut c_int) -> *mut tty_driver {
    *index = (*c).index;
    driver.driver
}

unsafe fn ssl_console_setup(co: *mut console, _options: *mut c_char) -> c_int {
    console_open_chan(&mut serial_lines[(*co).index as usize], co)
}

// No locking for register_console call; relies on single-threaded initcalls.
static mut ssl_cons: console = console {
    name: b"ttyS\0".as_ptr() as *mut c_char,
    write: Some(ssl_console_write), device: Some(ssl_console_device),
    setup: Some(ssl_console_setup), flags: CON_PRINTBUFFER | CON_ANYTIME,
    index: -1,
};

unsafe fn ssl_init() -> c_int {
    printk(b"Initializing software serial port version %d\n\0".as_ptr() as *const c_char,
           ssl_version);
    let err = register_lines(&mut driver, &ssl_ops, serial_lines.as_mut_ptr(), serial_lines.len());
    if err != 0 { return err; }
    let new_title = add_xterm_umid(opts.xterm_title);
    if !new_title.is_null() { opts.xterm_title = new_title; }
    for i in 0..NR_PORTS {
        let mut error: *mut c_char = core::ptr::null_mut();
        let mut s = conf[i];
        if s.is_null() { s = def_conf; }
        if setup_one_line(serial_lines.as_mut_ptr(), i as c_int, s, &mut opts, &mut error) != 0 {
            printk(b"setup_one_line failed for device %d : %s\n\0".as_ptr() as *const c_char,
                   i as c_int, error);
        }
    }
    ssl_init_done = 1;
    register_console(&mut ssl_cons);
    0
}

unsafe fn ssl_exit() {
    if ssl_init_done == 0 { return; }
    close_lines(serial_lines.as_mut_ptr(), serial_lines.len());
}

unsafe fn ssl_chan_setup(string: *mut c_char) -> c_int {
    line_setup(conf.as_mut_ptr(), NR_PORTS, &mut def_conf, string,
               b"serial line\0".as_ptr() as *const c_char);
    1
}

unsafe fn ssl_non_raw_setup(_string: *mut c_char) -> c_int {
    opts.raw = 0;
    1
}

// late_initcall(ssl_init);
// __uml_exitcall(ssl_exit);
// __setup("ssl", ssl_chan_setup);
// __channel_help(ssl_chan_setup, "ssl");
// __setup("ssl-non-raw", ssl_non_raw_setup);
// __uml_help(ssl_non_raw_setup,
// "ssl-non-raw\n"
// "    Set serial lines to non-raw mode.\n\n");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
