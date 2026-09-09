/*
 * Copyright 2016 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 */

pub const PRIORITY_MARK_MASK: u32 = 0x7fff;
pub const PRIORITY_OFF: u32 = 1 << 16;
pub const PRIORITY_ALWAYS_ON: u32 = 1 << 20;

pub const GFX_POWER_STATUS: u32 = 1 << 1;
pub const GFX_CLOCK_STATUS: u32 = 1 << 2;
pub const GFX_LS_STATUS: u32 = 1 << 3;

pub const RLC_BUSY_STATUS: u32 = 1 << 0;

#[macro_export]
macro_rules! RLC_PUD {
    ($x:expr) => {
        ($x) << 0
    };
}

pub const RLC_PUD_MASK: u32 = 0xff << 0;

#[macro_export]
macro_rules! RLC_PDD {
    ($x:expr) => {
        ($x) << 8
    };
}

pub const RLC_PDD_MASK: u32 = 0xff << 8;

#[macro_export]
macro_rules! RLC_TTPD {
    ($x:expr) => {
        ($x) << 16
    };
}

pub const RLC_TTPD_MASK: u32 = 0xff << 16;

#[macro_export]
macro_rules! RLC_MSD {
    ($x:expr) => {
        ($x) << 24
    };
}

pub const RLC_MSD_MASK: u32 = 0xff << 24;

pub const RLC_SAVE_AND_RESTORE_STARTING_OFFSET: u32 = 0x90;
pub const RLC_CLEAR_STATE_DESCRIPTOR_OFFSET: u32 = 0x3D;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
