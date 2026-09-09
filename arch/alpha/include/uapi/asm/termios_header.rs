/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent from the original header:
// #include <asm/ioctls.h>
// #include <asm/termbits.h>

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

pub const NCC: usize = 8;

#[repr(C)]
pub struct termio {
    pub c_iflag: u16, // input mode flags
    pub c_oflag: u16, // output mode flags
    pub c_cflag: u16, // control mode flags
    pub c_lflag: u16, // local mode flags
    pub c_line: u8,   // line discipline
    pub c_cc: [u8; NCC], // control characters
}

/*
 * c_cc characters in the termio structure.  Oh, how I love being
 * backwardly compatible.  Notice that character 4 and 5 are
 * interpreted differently depending on whether ICANON is set in
 * c_lflag.  If it's set, they are used as _VEOF and _VEOL, otherwise
 * as _VMIN and V_TIME.  This is for compatibility with OSF/1 (which
 * is compatible with sysV)...
 */
pub const _VINTR: u32 = 0;
pub const _VQUIT: u32 = 1;
pub const _VERASE: u32 = 2;
pub const _VKILL: u32 = 3;
pub const _VEOF: u32 = 4;
pub const _VMIN: u32 = 4;
pub const _VEOL: u32 = 5;
pub const _VTIME: u32 = 5;
pub const _VEOL2: u32 = 6;
pub const _VSWTC: u32 = 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
