/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2000, 2001 Jeff Dike (jdike@karaya.com)
 */

/* Declarations supplied by the Linux/UML headers and local headers. */

#[repr(C)]
pub struct ListHead {
    pub next: *mut ListHead,
    pub prev: *mut ListHead,
}

#[repr(C)]
pub struct Line {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ChanOps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ChanOpts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct TtyPort {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Console {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Chan {
    pub list: ListHead,
    pub free_list: ListHead,
    pub line: *mut Line,
    pub dev: *mut i8,
    pub primary: u32,
    pub input: u32,
    pub output: u32,
    pub opened: u32,
    pub enabled: u32,
    pub fd_in: i32,
    /* only different to fd_in if blocking output is needed */
    pub fd_out: i32,
    pub ops: *const ChanOps,
    pub data: *mut core::ffi::c_void,
}

extern "C" {
    pub fn chan_interrupt(line: *mut Line, irq: i32);
    pub fn parse_chan_pair(
        str_: *mut i8,
        line: *mut Line,
        device: i32,
        opts: *const ChanOpts,
        error_out: *mut *mut i8,
    ) -> i32;
    pub fn write_chan(chan: *mut Chan, buf: *const u8, len: usize, write_irq: i32) -> i32;
    pub fn console_write_chan(chan: *mut Chan, buf: *const i8, len: i32) -> i32;
    pub fn console_open_chan(line: *mut Line, co: *mut Console) -> i32;
    pub fn deactivate_chan(chan: *mut Chan, irq: i32);
    pub fn chan_enable_winch(chan: *mut Chan, port: *mut TtyPort);
    pub fn enable_chan(line: *mut Line) -> i32;
    pub fn close_chan(line: *mut Line);
    pub fn chan_window_size(
        line: *mut Line,
        rows_out: *mut u16,
        cols_out: *mut u16,
    ) -> i32;
    pub fn chan_config_string(
        line: *mut Line,
        str_: *mut i8,
        size: i32,
        error_out: *mut *mut i8,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
