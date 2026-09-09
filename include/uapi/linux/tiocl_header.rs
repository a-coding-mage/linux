/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const TIOCL_SETSEL: u32 = 2; // set a selection
pub const TIOCL_SELCHAR: u32 = 0; // select characters
pub const TIOCL_SELWORD: u32 = 1; // select whole words
pub const TIOCL_SELLINE: u32 = 2; // select whole lines
pub const TIOCL_SELPOINTER: u32 = 3; // show the pointer
pub const TIOCL_SELCLEAR: u32 = 4; // clear visibility of selection
pub const TIOCL_SELMOUSEREPORT: u32 = 16; // report beginning of selection
pub const TIOCL_SELBUTTONMASK: u32 = 15; // button mask for report

// selection extent
#[repr(C)]
pub struct tiocl_selection {
    pub xs: u16, // X start
    pub ys: u16, // Y start
    pub xe: u16, // X end
    pub ye: u16, // Y end
    pub sel_mode: u16, // selection mode
}

pub const TIOCL_PASTESEL: u32 = 3; // paste previous selection
pub const TIOCL_UNBLANKSCREEN: u32 = 4; // unblank screen

pub const TIOCL_SELLOADLUT: u32 = 5;
// set characters to be considered alphabetic when selecting
// u32[8] bit array, 4 bytes-aligned with type

// These two don't return a value: they write it back in the type.
pub const TIOCL_GETSHIFTSTATE: u32 = 6; // write shift state
pub const TIOCL_GETMOUSEREPORTING: u32 = 7; // write whether mouse event are reported
pub const TIOCL_SETVESABLANK: u32 = 10; // set vesa blanking mode
pub const TIOCL_SETKMSGREDIRECT: u32 = 11; // restrict kernel messages to a vt
pub const TIOCL_GETFGCONSOLE: u32 = 12; // get foreground vt
pub const TIOCL_SCROLLCONSOLE: u32 = 13; // scroll console
pub const TIOCL_BLANKSCREEN: u32 = 14; // keep screen blank even if a key is pressed
pub const TIOCL_BLANKEDSCREEN: u32 = 15; // return which vt was blanked
pub const TIOCL_GETKMSGREDIRECT: u32 = 17; // get the vt the kernel messages are restricted to
pub const TIOCL_GETBRACKETEDPASTE: u32 = 18; // get whether paste may be bracketed

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
