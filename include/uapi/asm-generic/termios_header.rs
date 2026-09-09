/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Most architectures have straight copies of the x86 code, with
 * varying levels of bug fixes on top. Usually it's a good idea
 * to use this generic version instead, but be careful to avoid
 * ABI changes.
 * New architectures should not provide their own version.
 *
 * Dependencies supplied by the original headers:
 *   <asm/termbits.h>
 *   <asm/ioctls.h>
 */

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
    pub c_iflag: u16, /* input mode flags */
    pub c_oflag: u16, /* output mode flags */
    pub c_cflag: u16, /* control mode flags */
    pub c_lflag: u16, /* local mode flags */
    pub c_line: u8, /* line discipline */
    pub c_cc: [u8; NCC], /* control characters */
}

/* modem lines */
pub const TIOCM_LE: u32 = 0x001;
pub const TIOCM_DTR: u32 = 0x002;
pub const TIOCM_RTS: u32 = 0x004;
pub const TIOCM_ST: u32 = 0x008;
pub const TIOCM_SR: u32 = 0x010;
pub const TIOCM_CTS: u32 = 0x020;
pub const TIOCM_CAR: u32 = 0x040;
pub const TIOCM_RNG: u32 = 0x080;
pub const TIOCM_DSR: u32 = 0x100;
pub const TIOCM_CD: u32 = TIOCM_CAR;
pub const TIOCM_RI: u32 = TIOCM_RNG;
pub const TIOCM_OUT1: u32 = 0x2000;
pub const TIOCM_OUT2: u32 = 0x4000;
pub const TIOCM_LOOP: u32 = 0x8000;

/* ioctl (fd, TIOCSERGETLSR, &result) where result may be as below */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
