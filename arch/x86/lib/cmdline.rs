// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Misc librarized functions for cmdline poking.
 */

use core::ffi::c_char;

// Supplied by the corresponding kernel headers/build configuration.
const COMMAND_LINE_SIZE: i32 = 2048;

extern "C" {
    static builtin_cmdline: *const c_char;
    static builtin_cmdline_added: bool;
}

#[inline]
unsafe fn myisspace(c: u8) -> bool {
    c <= b' '
}

/*
 * Find a boolean option (like quiet,noapic,nosmp....)
 *
 * @cmdline: the cmdline string
 * @max_cmdline_size: the maximum size of cmdline
 * @option: option string to look for
 *
 * Returns the position of that @option (starts counting with 1)
 * or 0 on not found.  @option will only be found if it is found
 * as an entire word in @cmdline.  For instance, if @option="car"
 * then a cmdline which contains "cart" will not match.
 */
unsafe fn __cmdline_find_option_bool(
    mut cmdline: *const c_char,
    max_cmdline_size: i32,
    option: *const c_char,
) -> i32 {
    if cmdline.is_null() {
        return -1;
    }

    let mut pos: i32 = 0;
    let mut wstart: i32 = 0;
    let mut opptr = core::ptr::null();
    let mut state = 0i32; // st_wordstart

    while pos < max_cmdline_size {
        let c = *(cmdline as *const u8);
        cmdline = cmdline.add(1);
        pos += 1;

        match state {
            0 => {
                if c == 0 {
                    return 0;
                } else if myisspace(c) {
                    continue;
                }
                state = 1;
                opptr = option;
                wstart = pos;
            }
            _ => {}
        }

        if state == 1 {
            if *opptr == 0 {
                if c == 0 || myisspace(c) {
                    return wstart;
                }
            } else if c == 0 {
                return 0;
            } else if c == *opptr as u8 {
                opptr = opptr.add(1);
                continue;
            }
            state = 2;
        }

        if state == 2 {
            if c == 0 {
                return 0;
            } else if myisspace(c) {
                state = 0;
            }
        }
    }

    0
}

/*
 * Find a non-boolean option (i.e. option=argument). In accordance with
 * standard Linux practice, if this option is repeated, this returns the
 * last instance on the command line.
 *
 * @cmdline: the cmdline string
 * @max_cmdline_size: the maximum size of cmdline
 * @option: option string to look for
 * @buffer: memory buffer to return the option argument
 * @bufsize: size of the supplied memory buffer
 *
 * Returns the length of the argument (regardless of if it was
 * truncated to fit in the buffer), or -1 on not found.
 */
unsafe fn __cmdline_find_option(
    mut cmdline: *const c_char,
    max_cmdline_size: i32,
    option: *const c_char,
    buffer: *mut c_char,
    bufsize: i32,
) -> i32 {
    if cmdline.is_null() {
        return -1;
    }

    let mut pos: i32 = 0;
    let mut len: i32 = -1;
    let mut opptr = core::ptr::null();
    let mut bufptr = buffer;
    let mut state = 0i32; // st_wordstart

    while {
        pos += 1;
        pos <= max_cmdline_size
    } {
        let c = *(cmdline as *const u8);
        cmdline = cmdline.add(1);
        if c == 0 {
            break;
        }

        match state {
            0 => {
                if myisspace(c) {
                    continue;
                }
                state = 1;
                opptr = option;
            }
            _ => {}
        }

        match state {
            1 => {
                if c == b'=' && *opptr == 0 {
                    len = 0;
                    bufptr = buffer;
                    state = 3;
                } else if c == *opptr as u8 {
                    opptr = opptr.add(1);
                    continue;
                } else {
                    state = 2;
                }
            }
            2 => {
                if myisspace(c) {
                    state = 0;
                }
                continue;
            }
            3 => {}
            _ => unreachable!(),
        }

        if state == 3 {
            if myisspace(c) {
                state = 0;
            } else {
                len += 1;
                if len < bufsize {
                    *bufptr = c as c_char;
                    bufptr = bufptr.add(1);
                }
            }
        }
    }

    if bufsize != 0 {
        *bufptr = 0;
    }
    len
}

pub unsafe fn cmdline_find_option_bool(cmdline: *const c_char, option: *const c_char) -> i32 {
    let ret = __cmdline_find_option_bool(cmdline, COMMAND_LINE_SIZE, option);
    if ret > 0 {
        return ret;
    }

    #[cfg(CONFIG_CMDLINE_BOOL)]
    if !builtin_cmdline_added {
        return __cmdline_find_option_bool(builtin_cmdline, COMMAND_LINE_SIZE, option);
    }

    ret
}

pub unsafe fn cmdline_find_option(
    cmdline: *const c_char,
    option: *const c_char,
    buffer: *mut c_char,
    bufsize: i32,
) -> i32 {
    let ret = __cmdline_find_option(cmdline, COMMAND_LINE_SIZE, option, buffer, bufsize);
    if ret > 0 {
        return ret;
    }

    #[cfg(CONFIG_CMDLINE_BOOL)]
    if !builtin_cmdline_added {
        return __cmdline_find_option(builtin_cmdline, COMMAND_LINE_SIZE, option, buffer, bufsize);
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
