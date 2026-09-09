/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Liberally adapted from alpha/termios.h.  In particular, the c_cc[]
 * fields have been reordered so that termio & termios share the
 * common subset in the same order (for brain dead programs that don't
 * know or care about the differences).
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version
 * 2 of the License, or (at your option) any later version.
 */

// Dependencies supplied by the corresponding asm/ioctls.h and asm/termbits.h.

#[repr(C)]
pub struct sgttyb {
    pub sg_ispeed: i8,
    pub sg_ospeed: i8,
    pub sg_erase: i8,
    pub sg_kill: i8,
    pub sg_flags: i16,
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
    pub t_suspc: i8,
    pub t_dsuspc: i8,
    pub t_rprntc: i8,
    pub t_flushc: i8,
    pub t_werasc: i8,
    pub t_lnextc: i8,
}

#[repr(C)]
pub struct winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

pub const NCC: usize = 10;

#[repr(C)]
pub struct termio {
    pub c_iflag: u16, // input mode flags
    pub c_oflag: u16, // output mode flags
    pub c_cflag: u16, // control mode flags
    pub c_lflag: u16, // local mode flags
    pub c_line: u8, // line discipline
    pub c_cc: [u8; NCC], // control characters
}

/* c_cc characters */
pub const _VINTR: usize = 0;
pub const _VQUIT: usize = 1;
pub const _VERASE: usize = 2;
pub const _VKILL: usize = 3;
pub const _VEOF: usize = 4;
pub const _VMIN: usize = 5;
pub const _VEOL: usize = 6;
pub const _VTIME: usize = 7;
pub const _VEOL2: usize = 8;
pub const _VSWTC: usize = 9;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
