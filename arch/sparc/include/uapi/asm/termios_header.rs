/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding asm/ioctls.h and asm/termbits.h
// headers are intentionally not reimplemented here.

// C condition: defined(__KERNEL__) || defined(__DEFINE_BSD_TERMIOS)
#[cfg(any(feature = "kernel", feature = "define_bsd_termios"))]
#[repr(C)]
pub struct Sgttyb {
    pub sg_ispeed: core::ffi::c_char,
    pub sg_ospeed: core::ffi::c_char,
    pub sg_erase: core::ffi::c_char,
    pub sg_kill: core::ffi::c_char,
    pub sg_flags: i16,
}

#[cfg(any(feature = "kernel", feature = "define_bsd_termios"))]
#[repr(C)]
pub struct Tchars {
    pub t_intrc: core::ffi::c_char,
    pub t_quitc: core::ffi::c_char,
    pub t_startc: core::ffi::c_char,
    pub t_stopc: core::ffi::c_char,
    pub t_eofc: core::ffi::c_char,
    pub t_brkc: core::ffi::c_char,
}

#[cfg(any(feature = "kernel", feature = "define_bsd_termios"))]
#[repr(C)]
pub struct Ltchars {
    pub t_suspc: core::ffi::c_char,
    pub t_dsuspc: core::ffi::c_char,
    pub t_rprntc: core::ffi::c_char,
    pub t_flushc: core::ffi::c_char,
    pub t_werasc: core::ffi::c_char,
    pub t_lnextc: core::ffi::c_char,
}

#[repr(C)]
pub struct Winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

pub const NCC: usize = 8;

#[repr(C)]
pub struct Termio {
    pub c_iflag: u16, // input mode flags
    pub c_oflag: u16, // output mode flags
    pub c_cflag: u16, // control mode flags
    pub c_lflag: u16, // local mode flags
    pub c_line: u8,   // line discipline
    pub c_cc: [u8; NCC], // control characters
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
