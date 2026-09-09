/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 1996, 2000, 2001 by Ralf Baechle
 * Copyright (C) 2000, 2001 Silicon Graphics, Inc.
 */

// Dependencies supplied by the corresponding Linux UAPI headers:
// <linux/errno.h>, <asm/termbits.h>, and <asm/ioctls.h>

#[repr(C)]
pub struct sgttyb {
    pub sg_ispeed: i8,
    pub sg_ospeed: i8,
    pub sg_erase: i8,
    pub sg_kill: i8,
    // SGI special - int, not short
    pub sg_flags: i32,
}

#[repr(C)]
pub struct tchars {
    pub t_intrc: i8,
    pub t_quitc: i8,
    pub t_startc: i8,
    pub t_stopc: i8,
    pub t_eofc: i8,
    pub t_brkc: i8,
}

#[repr(C)]
pub struct ltchars {
    // stop process signal
    pub t_suspc: i8,
    // delayed stop process signal
    pub t_dsuspc: i8,
    // reprint line
    pub t_rprntc: i8,
    // flush output (toggles)
    pub t_flushc: i8,
    // word erase
    pub t_werasc: i8,
    // literal next character
    pub t_lnextc: i8,
}

// TIOCGSIZE, TIOCSSIZE not defined yet.  Only needed for SunOS source
// compatibility anyway ...

#[repr(C)]
pub struct winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

pub const NCC: usize = 8;

#[repr(C)]
pub struct termio {
    // input mode flags
    pub c_iflag: u16,
    // output mode flags
    pub c_oflag: u16,
    // control mode flags
    pub c_cflag: u16,
    // local mode flags
    pub c_lflag: u16,
    // line discipline
    pub c_line: i8,
    // control characters
    pub c_cc: [u8; NCCS],
}

/* modem lines */
pub const TIOCM_LE: u32 = 0x001;
pub const TIOCM_DTR: u32 = 0x002;
pub const TIOCM_RTS: u32 = 0x004;
pub const TIOCM_ST: u32 = 0x010;
pub const TIOCM_SR: u32 = 0x020;
pub const TIOCM_CTS: u32 = 0x040;
pub const TIOCM_CAR: u32 = 0x100;
pub const TIOCM_CD: u32 = TIOCM_CAR;
pub const TIOCM_RNG: u32 = 0x200;
pub const TIOCM_RI: u32 = TIOCM_RNG;
pub const TIOCM_DSR: u32 = 0x400;
pub const TIOCM_OUT1: u32 = 0x2000;
pub const TIOCM_OUT2: u32 = 0x4000;
pub const TIOCM_LOOP: u32 = 0x8000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
