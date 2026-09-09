// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007 rPath, Inc. - All Rights Reserved
 *
 * ----------------------------------------------------------------------- */

/*
 * Simple command-line parser for early boot.
 */

// Supplied by the surrounding boot environment.
extern "C" {
    fn set_fs(fs: addr_t);
    fn rdfs8(addr: addr_t) -> u8;
}

type addr_t = usize;

#[inline]
unsafe fn myisspace(c: u8) -> bool {
    c <= b' ' /* Close enough approximation */
}

/*
 * Find a non-boolean option, that is, "option=argument".  In accordance
 * with standard Linux practice, if this option is repeated, this returns the
 * last instance on the command line.
 *
 * Returns the length of the argument (regardless of if it was
 * truncated to fit in the buffer), or -1 on not found.
 */
pub unsafe fn __cmdline_find_option(
    cmdline_ptr: libc::c_ulong,
    option: *const libc::c_char,
    buffer: *mut libc::c_char,
    bufsize: libc::c_int,
) -> libc::c_int {
    let mut cptr: addr_t;
    let mut len: libc::c_int = -1;
    let mut opptr: *const libc::c_char = core::ptr::null();
    let mut bufptr: *mut libc::c_char = buffer;
    const ST_WORDSTART: u8 = 0;
    const ST_WORDCMP: u8 = 1;
    const ST_WORDSKIP: u8 = 2;
    const ST_BUFCOPY: u8 = 3;
    let mut state = ST_WORDSTART;

    if cmdline_ptr == 0 {
        return -1; /* No command line */
    }

    cptr = (cmdline_ptr as addr_t) & 0xf;
    set_fs((cmdline_ptr as addr_t) >> 4);

    while cptr < 0x10000 {
        let c = rdfs8(cptr);
        cptr += 1;
        if c == 0 {
            break;
        }

        match state {
            ST_WORDSTART => {
                if myisspace(c) {
                    continue;
                }
                state = ST_WORDCMP;
                opptr = option;
                // C fallthrough
                if c == b'=' && *opptr == 0 {
                    len = 0;
                    bufptr = buffer;
                    state = ST_BUFCOPY;
                } else if myisspace(c) {
                    state = ST_WORDSTART;
                } else if c != *opptr {
                    opptr = opptr.add(1);
                    state = ST_WORDSKIP;
                } else {
                    opptr = opptr.add(1);
                }
            }
            ST_WORDCMP => {
                if c == b'=' && *opptr == 0 {
                    len = 0;
                    bufptr = buffer;
                    state = ST_BUFCOPY;
                } else if myisspace(c) {
                    state = ST_WORDSTART;
                } else if c != *opptr {
                    opptr = opptr.add(1);
                    state = ST_WORDSKIP;
                } else {
                    opptr = opptr.add(1);
                }
            }
            ST_WORDSKIP => {
                if myisspace(c) {
                    state = ST_WORDSTART;
                }
            }
            ST_BUFCOPY => {
                if myisspace(c) {
                    state = ST_WORDSTART;
                } else {
                    if len < bufsize - 1 {
                        *bufptr = c as libc::c_char;
                        bufptr = bufptr.add(1);
                    }
                    len += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    if bufsize != 0 {
        *bufptr = 0;
    }

    len
}

/*
 * Find a boolean option (like quiet,noapic,nosmp....)
 *
 * Returns the position of that option (starts counting with 1)
 * or 0 on not found
 */
pub unsafe fn __cmdline_find_option_bool(
    cmdline_ptr: libc::c_ulong,
    option: *const libc::c_char,
) -> libc::c_int {
    let mut cptr: addr_t;
    let mut pos: libc::c_int = 0;
    let mut wstart: libc::c_int = 0;
    let mut opptr: *const libc::c_char = core::ptr::null();
    const ST_WORDSTART: u8 = 0;
    const ST_WORDCMP: u8 = 1;
    const ST_WORDSKIP: u8 = 2;
    let mut state = ST_WORDSTART;

    if cmdline_ptr == 0 {
        return -1; /* No command line */
    }

    cptr = (cmdline_ptr as addr_t) & 0xf;
    set_fs((cmdline_ptr as addr_t) >> 4);

    while cptr < 0x10000 {
        let c = rdfs8(cptr);
        cptr += 1;
        pos += 1;

        match state {
            ST_WORDSTART => {
                if c == 0 {
                    return 0;
                } else if myisspace(c) {
                    continue;
                }
                state = ST_WORDCMP;
                opptr = option;
                wstart = pos;
                // C fallthrough
                if *opptr == 0 {
                    if c == 0 || myisspace(c) {
                        return wstart;
                    }
                    state = ST_WORDSKIP;
                } else if c == 0 {
                    return 0;
                } else if c != *opptr {
                    opptr = opptr.add(1);
                    state = ST_WORDSKIP;
                } else {
                    opptr = opptr.add(1);
                }
            }
            ST_WORDCMP => {
                if *opptr == 0 {
                    if c == 0 || myisspace(c) {
                        return wstart;
                    }
                    state = ST_WORDSKIP;
                } else if c == 0 {
                    return 0;
                } else if c != *opptr {
                    opptr = opptr.add(1);
                    state = ST_WORDSKIP;
                } else {
                    opptr = opptr.add(1);
                }
            }
            ST_WORDSKIP => {
                if c == 0 {
                    return 0;
                } else if myisspace(c) {
                    state = ST_WORDSTART;
                }
            }
            _ => unreachable!(),
        }
    }

    0 /* Buffer overrun */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
