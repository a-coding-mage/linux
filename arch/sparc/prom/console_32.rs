// SPDX-License-Identifier: GPL-2.0
/*
 * console.c: Routines that deal with sending and receiving IO
 *            to/from the current console device using the PROM.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 * Copyright (C) 1998 Pete Zaitcev <zaitcev@yahoo.com>
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

/* Declarations supplied by the surrounding kernel/PROM implementation. */
#[repr(C)]
pub struct SpinLock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PromV2BootArgs {
    pub fd_stdout: *const c_int,
}

#[repr(C)]
pub struct PromV2DevOps {
    pub v2_dev_write:
        Option<unsafe extern "C" fn(fd: c_int, buf: *const c_char, len: c_int) -> c_int>,
}

#[repr(C)]
pub struct PromVec {
    pub pv_nbputchar: Option<unsafe extern "C" fn(ch: c_char) -> c_int>,
    pub pv_v2devops: PromV2DevOps,
    pub pv_v2bootargs: PromV2BootArgs,
}

extern "C" {
    pub fn restore_current();
    pub fn spin_lock_irqsave(lock: *mut SpinLock, flags: *mut c_ulong);
    pub fn spin_unlock_irqrestore(lock: *mut SpinLock, flags: c_ulong);
    pub static mut prom_lock: SpinLock;
    pub static mut prom_vers: c_int;
    pub static mut romvec: *mut PromVec;
}

pub const PROM_V0: c_int = 0;
pub const PROM_V2: c_int = 2;
pub const PROM_V3: c_int = 3;

/* Non blocking put character to console device, returns -1 if
 * unsuccessful.
 */
unsafe fn prom_nbputchar(buf: *const c_char) -> c_int {
    let mut flags: c_ulong = 0;
    let mut i: c_int = -1;

    spin_lock_irqsave(&raw mut prom_lock, &mut flags);
    match prom_vers {
        PROM_V0 => {
            if ((*romvec).pv_nbputchar.expect("pv_nbputchar"))(*buf) != 0 {
                i = 1;
            }
        }
        PROM_V2 | PROM_V3 => {
            if ((*romvec)
                .pv_v2devops
                .v2_dev_write
                .expect("v2_dev_write"))(
                *(*romvec).pv_v2bootargs.fd_stdout,
                buf,
                0x1,
            ) == 1
            {
                i = 1;
            }
        }
        _ => {}
    }
    restore_current();
    spin_unlock_irqrestore(&raw mut prom_lock, flags);
    i /* Ugh, we could spin forever on unsupported proms ;( */
}

pub unsafe fn prom_console_write_buf(mut buf: *const c_char, mut len: c_int) {
    while len != 0 {
        let n = prom_nbputchar(buf);
        if n < 0 {
            continue;
        }
        len -= 1;
        buf = buf.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
