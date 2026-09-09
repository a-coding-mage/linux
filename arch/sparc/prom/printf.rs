// SPDX-License-Identifier: GPL-2.0
/*
 * printf.c:  Internal prom library printf facility.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 * Copyright (C) 1997 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 * Copyright (c) 2002 Pete Zaitcev (zaitcev@yahoo.com)
 *
 * We used to warn all over the code: DO NOT USE prom_printf(),
 * and yet people do. Anton's banking code was outputting banks
 * with prom_printf for most of the 2.4 lifetime. Since an effective
 * stick is not available, we deployed a carrot: an early printk
 * through PROM by means of -p boot option. This ought to fix it.
 * USE printk; if you need, deploy -p.
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_ulong, c_uint, VaList};

const CONSOLE_WRITE_BUF_SIZE: usize = 1024;

static mut PPBUF: [c_char; 1024] = [0; 1024];
static mut CONSOLE_WRITE_BUF: [c_char; CONSOLE_WRITE_BUF_SIZE] = [0; CONSOLE_WRITE_BUF_SIZE];

extern "C" {
    fn prom_console_write_buf(buf: *const c_char, len: c_uint);
    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: *mut c_ulong);
    fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: c_ulong);
    fn vscnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, args: VaList<'_>) -> c_int;
}

#[repr(C)]
pub struct raw_spinlock_t {
    _opaque: [u8; 0],
}

static mut console_write_lock: raw_spinlock_t = raw_spinlock_t { _opaque: [] };

pub unsafe extern "C" fn prom_write(buf: *const c_char, mut n: c_uint) {
    let mut dest_len: c_uint;
    let mut flags: c_ulong = 0;
    let mut dest: *mut c_char;

    dest = CONSOLE_WRITE_BUF.as_mut_ptr();
    raw_spin_lock_irqsave(&mut console_write_lock, &mut flags);

    dest_len = 0;
    while n != 0 {
        n -= 1;
        let ch = *buf;
        buf = buf.add(1);
        if ch == b'\n' as c_char {
            *dest = b'\r' as c_char;
            dest = dest.add(1);
            dest_len += 1;
        }
        *dest = ch;
        dest = dest.add(1);
        dest_len += 1;
        if dest_len >= (CONSOLE_WRITE_BUF_SIZE - 1) as c_uint {
            prom_console_write_buf(CONSOLE_WRITE_BUF.as_ptr(), dest_len);
            dest = CONSOLE_WRITE_BUF.as_mut_ptr();
            dest_len = 0;
        }
    }
    if dest_len != 0 {
        prom_console_write_buf(CONSOLE_WRITE_BUF.as_ptr(), dest_len);
    }

    raw_spin_unlock_irqrestore(&mut console_write_lock, flags);
}

pub unsafe extern "C" fn prom_printf(fmt: *const c_char, mut args: ...) {
    let i: c_int;

    i = vscnprintf(PPBUF.as_mut_ptr(), PPBUF.len(), fmt, args);
    prom_write(PPBUF.as_ptr(), i as c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
