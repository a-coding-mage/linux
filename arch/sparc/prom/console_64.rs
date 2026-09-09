// SPDX-License-Identifier: GPL-2.0
/* console.c: Routines that deal with sending and receiving IO
 *            to/from the current console device using the PROM.
 *
 * Copyright (C) 1995 David S. Miller (davem@davemloft.net)
 * Copyright (C) 1996,1997 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_ulong};

extern "C" {
    static mut prom_stdout: c_ulong;
    fn p1275_cmd_direct(args: *mut c_ulong);
}

unsafe fn __prom_console_write_buf(buf: *const c_char, len: c_int) -> c_int {
    let mut args: [c_ulong; 7] = [0; 7];
    let ret: c_int;

    args[0] = b"write\0".as_ptr() as c_ulong;
    args[1] = 3;
    args[2] = 1;
    args[3] = prom_stdout as c_ulong;
    args[4] = buf as c_ulong;
    args[5] = len as u32 as c_ulong;
    args[6] = (-1i32) as c_ulong;

    p1275_cmd_direct(args.as_mut_ptr());

    ret = args[6] as c_int;
    if ret < 0 {
        return -1;
    }
    ret
}

pub unsafe fn prom_console_write_buf(mut buf: *const c_char, mut len: c_int) {
    while len != 0 {
        let n: c_int = __prom_console_write_buf(buf, len);
        if n < 0 {
            continue;
        }
        len -= n;
        buf = buf.add(len as usize);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
