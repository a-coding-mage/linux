/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2001, 2002 Jeff Dike (jdike@karaya.com)
 */

// Dependencies supplied by the surrounding kernel/UML translation.

#[repr(C)]
pub struct line_driver {
    pub name: *const core::ffi::c_char,
    pub device_name: *const core::ffi::c_char,
    pub major: i16,
    pub minor_start: i16,
    pub type_: i16,
    pub subtype: i16,
    pub read_irq_name: *const core::ffi::c_char,
    pub write_irq_name: *const core::ffi::c_char,
    pub mc: mc_device,
    pub driver: *mut tty_driver,
}

#[repr(C)]
pub struct line {
    pub port: tty_port,
    pub valid: i32,
    pub read_irq: i32,
    pub write_irq: i32,
    pub init_str: *mut core::ffi::c_char,
    pub chan_list: list_head,
    pub chan_in: *mut chan,
    pub chan_out: *mut chan,
    pub lock: spinlock_t,
    pub throttled: i32,
    pub buffer: *mut u8,
    pub head: *mut u8,
    pub tail: *mut u8,
    pub sigio: i32,
    pub task: delayed_work,
    pub driver: *const line_driver,
}

extern "C" {
    pub fn line_close(tty: *mut tty_struct, filp: *mut file);
    pub fn line_open(tty: *mut tty_struct, filp: *mut file) -> i32;
    pub fn line_install(
        driver: *mut tty_driver,
        tty: *mut tty_struct,
        line: *mut line,
    ) -> i32;
    pub fn line_cleanup(tty: *mut tty_struct);
    pub fn line_hangup(tty: *mut tty_struct);
    pub fn line_setup(
        conf: *mut *mut core::ffi::c_char,
        nlines: u32,
        def: *mut *mut core::ffi::c_char,
        init: *mut core::ffi::c_char,
        name: *mut core::ffi::c_char,
    ) -> i32;
    pub fn line_write(
        tty: *mut tty_struct,
        buf: *const u8,
        len: usize,
    ) -> isize;
    pub fn line_chars_in_buffer(tty: *mut tty_struct) -> u32;
    pub fn line_flush_buffer(tty: *mut tty_struct);
    pub fn line_flush_chars(tty: *mut tty_struct);
    pub fn line_write_room(tty: *mut tty_struct) -> u32;
    pub fn line_throttle(tty: *mut tty_struct);
    pub fn line_unthrottle(tty: *mut tty_struct);

    pub fn add_xterm_umid(base: *mut core::ffi::c_char) -> *mut core::ffi::c_char;
    pub fn line_setup_irq(
        fd: i32,
        input: i32,
        output: i32,
        line: *mut line,
        data: *mut core::ffi::c_void,
    ) -> i32;
    pub fn line_close_chan(line: *mut line);
    pub fn register_lines(
        line_driver: *mut line_driver,
        driver: *const tty_operations,
        lines: *mut line,
        nlines: i32,
    ) -> i32;
    pub fn setup_one_line(
        lines: *mut line,
        n: i32,
        init: *mut core::ffi::c_char,
        opts: *const chan_opts,
        error_out: *mut *mut core::ffi::c_char,
    ) -> i32;
    pub fn close_lines(lines: *mut line, nlines: i32);
    pub fn line_config(
        lines: *mut line,
        sizeof_lines: u32,
        str_: *mut core::ffi::c_char,
        opts: *const chan_opts,
        error_out: *mut *mut core::ffi::c_char,
    ) -> i32;
    pub fn line_id(
        str_: *mut *mut core::ffi::c_char,
        start_out: *mut i32,
        end_out: *mut i32,
    ) -> i32;
    pub fn line_remove(
        lines: *mut line,
        sizeof_lines: u32,
        n: i32,
        error_out: *mut *mut core::ffi::c_char,
    ) -> i32;
    pub fn line_get_config(
        dev: *mut core::ffi::c_char,
        lines: *mut line,
        sizeof_lines: u32,
        str_: *mut core::ffi::c_char,
        size: i32,
        error_out: *mut *mut core::ffi::c_char,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
