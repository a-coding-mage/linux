/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const HW_BREAKPOINT_LEN_1: u32 = 1;
pub const HW_BREAKPOINT_LEN_2: u32 = 2;
pub const HW_BREAKPOINT_LEN_3: u32 = 3;
pub const HW_BREAKPOINT_LEN_4: u32 = 4;
pub const HW_BREAKPOINT_LEN_5: u32 = 5;
pub const HW_BREAKPOINT_LEN_6: u32 = 6;
pub const HW_BREAKPOINT_LEN_7: u32 = 7;
pub const HW_BREAKPOINT_LEN_8: u32 = 8;

pub const HW_BREAKPOINT_EMPTY: u32 = 0;
pub const HW_BREAKPOINT_R: u32 = 1;
pub const HW_BREAKPOINT_W: u32 = 2;
pub const HW_BREAKPOINT_RW: u32 = HW_BREAKPOINT_R | HW_BREAKPOINT_W;
pub const HW_BREAKPOINT_X: u32 = 4;
pub const HW_BREAKPOINT_INVALID: u32 = HW_BREAKPOINT_RW | HW_BREAKPOINT_X;
