/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type size_t = usize;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum binary_printer_ops {
    BINARY_PRINT_DATA_BEGIN,
    BINARY_PRINT_LINE_BEGIN,
    BINARY_PRINT_ADDR,
    BINARY_PRINT_NUM_DATA,
    BINARY_PRINT_NUM_PAD,
    BINARY_PRINT_SEP,
    BINARY_PRINT_CHAR_DATA,
    BINARY_PRINT_CHAR_PAD,
    BINARY_PRINT_LINE_END,
    BINARY_PRINT_DATA_END,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

pub type binary__fprintf_t = Option<
    unsafe extern "C" fn(
        op: binary_printer_ops,
        val: c_uint,
        extra: *mut c_void,
        fp: *mut FILE,
    ) -> c_int,
>;

unsafe extern "C" {
    pub static mut stdout: *mut FILE;

    pub fn binary__fprintf(
        data: *mut u8,
        len: size_t,
        bytes_per_line: size_t,
        printer: binary__fprintf_t,
        extra: *mut c_void,
        fp: *mut FILE,
    ) -> c_int;

    pub fn is_printable_array(p: *mut c_char, len: c_uint) -> c_int;
}

pub unsafe fn print_binary(
    data: *mut u8,
    len: size_t,
    bytes_per_line: size_t,
    printer: binary__fprintf_t,
    extra: *mut c_void,
) {
    unsafe {
        binary__fprintf(data, len, bytes_per_line, printer, extra, stdout);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
