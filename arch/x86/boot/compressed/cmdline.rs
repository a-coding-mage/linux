// SPDX-License-Identifier: GPL-2.0-only
//! Flat-address form of the original early x86 command-line state machines.
use crate::bindings;
use core::ffi::{c_char, c_int, c_ulong};

#[no_mangle]
pub(crate) unsafe extern "C" fn get_cmd_line_ptr() -> c_ulong {
    // SAFETY: the decompressor sets the native boot-parameter pointer first.
    unsafe {
        let bp = bindings::boot_params_ptr;
        (*bp).hdr.cmd_line_ptr as c_ulong | ((*bp).ext_cmd_line_ptr as c_ulong) << 32
    }
}

#[derive(Clone, Copy)]
enum State {
    WordStart,
    WordCompare,
    WordSkip,
    BufferCopy,
}

#[no_mangle]
pub(crate) unsafe extern "C" fn __cmdline_find_option(
    address: c_ulong,
    option: *const c_char,
    buffer: *mut c_char,
    bufsize: c_int,
) -> c_int {
    // SAFETY: the C callers supply a readable command line and option, and
    // writable buffer storage exactly as required by the original parser.
    unsafe {
        if address == 0 {
            return -1;
        }
        let base = address & !0xf;
        let mut cursor = address & 0xf;
        let mut state = State::WordStart;
        let mut len: c_int = -1;
        let mut option_cursor = option;
        let mut output = buffer;
        while cursor < 0x10000 {
            let c = *((base + cursor) as *const c_char);
            cursor += 1;
            if c == 0 {
                break;
            }
            let space = (c as u8) <= b' ';
            if let State::WordStart = state {
                if space {
                    continue;
                }
                state = State::WordCompare;
                option_cursor = option;
            }
            match state {
                State::WordCompare => {
                    if c == b'=' as c_char && *option_cursor == 0 {
                        len = 0;
                        output = buffer;
                        state = State::BufferCopy;
                    } else if space {
                        state = State::WordStart;
                    } else {
                        let expected = *option_cursor;
                        option_cursor = option_cursor.add(1);
                        if c != expected {
                            state = State::WordSkip;
                        }
                    }
                }
                State::WordSkip => {
                    if space {
                        state = State::WordStart;
                    }
                }
                State::BufferCopy => {
                    if space {
                        state = State::WordStart;
                    } else {
                        if len < bufsize.wrapping_sub(1) {
                            *output = c;
                            output = output.add(1);
                        }
                        len = len.wrapping_add(1);
                    }
                }
                State::WordStart => {}
            }
        }
        if bufsize != 0 {
            *output = 0;
        }
        len
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn __cmdline_find_option_bool(
    address: c_ulong,
    option: *const c_char,
) -> c_int {
    // SAFETY: native boot command line and option are readable C strings.
    unsafe {
        if address == 0 {
            return -1;
        }
        let base = address & !0xf;
        let mut cursor = address & 0xf;
        let mut state = State::WordStart;
        let mut position: c_int = 0;
        let mut word_start: c_int = 0;
        let mut option_cursor = option;
        while cursor < 0x10000 {
            let c = *((base + cursor) as *const c_char);
            cursor += 1;
            position += 1;
            let space = (c as u8) <= b' ';
            if let State::WordStart = state {
                if c == 0 {
                    return 0;
                }
                if space {
                    continue;
                }
                state = State::WordCompare;
                option_cursor = option;
                word_start = position;
            }
            match state {
                State::WordCompare => {
                    if *option_cursor == 0 {
                        if c == 0 || space {
                            return word_start;
                        }
                        state = State::WordSkip;
                    } else if c == 0 {
                        return 0;
                    } else {
                        let expected = *option_cursor;
                        option_cursor = option_cursor.add(1);
                        if c != expected {
                            state = State::WordSkip;
                        }
                    }
                }
                State::WordSkip => {
                    if c == 0 {
                        return 0;
                    }
                    if space {
                        state = State::WordStart;
                    }
                }
                State::WordStart | State::BufferCopy => {}
            }
        }
        0
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn cmdline_find_option(
    option: *const c_char,
    buffer: *mut c_char,
    bufsize: c_int,
) -> c_int {
    // SAFETY: the wrapper preserves all original C parser arguments.
    unsafe { __cmdline_find_option(get_cmd_line_ptr(), option, buffer, bufsize) }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn cmdline_find_option_bool(option: *const c_char) -> c_int {
    // SAFETY: the wrapper preserves all original C parser arguments.
    unsafe { __cmdline_find_option_bool(get_cmd_line_ptr(), option) }
}
