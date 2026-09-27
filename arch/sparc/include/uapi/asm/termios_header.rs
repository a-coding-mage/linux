/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding asm/ioctls.h and asm/termbits.h
// headers are intentionally not reimplemented here.

// C condition: defined(__KERNEL__) || defined(__DEFINE_BSD_TERMIOS)
#[cfg(any(feature = "kernel", feature = "define_bsd_termios"))]
#[repr(C)]
pub struct Sgttyb {
    pub sg_ispeed: kernel::ffi::c_char,
    pub sg_ospeed: kernel::ffi::c_char,
    pub sg_erase: kernel::ffi::c_char,
    pub sg_kill: kernel::ffi::c_char,
    pub sg_flags: i16,
}

#[cfg(any(feature = "kernel", feature = "define_bsd_termios"))]
#[repr(C)]
pub struct Tchars {
    pub t_intrc: kernel::ffi::c_char,
    pub t_quitc: kernel::ffi::c_char,
    pub t_startc: kernel::ffi::c_char,
    pub t_stopc: kernel::ffi::c_char,
    pub t_eofc: kernel::ffi::c_char,
    pub t_brkc: kernel::ffi::c_char,
}

#[cfg(any(feature = "kernel", feature = "define_bsd_termios"))]
#[repr(C)]
pub struct Ltchars {
    pub t_suspc: kernel::ffi::c_char,
    pub t_dsuspc: kernel::ffi::c_char,
    pub t_rprntc: kernel::ffi::c_char,
    pub t_flushc: kernel::ffi::c_char,
    pub t_werasc: kernel::ffi::c_char,
    pub t_lnextc: kernel::ffi::c_char,
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
