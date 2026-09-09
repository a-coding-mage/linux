/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * i8k.h -- Linux driver for accessing the SMM BIOS on Dell laptops
 *
 * Copyright (C) 2001  Massimo Dal Zotto <dz@debian.org>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2, or (at your option) any
 * later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 */

pub const I8K_PROC: &str = "/proc/i8k";
pub const I8K_PROC_FMT: &str = "1.0";

// These constants correspond to the Linux _IOR/_IOWR ioctl macros.
// The ioctl encoding parameters are supplied by the target architecture.
#[allow(unused_macros)]
macro_rules! _IOC {
    ($dir:expr, $ty:expr, $nr:expr, $size:expr) => {
        (($dir << 30) | ($size << 16) | ($ty << 8) | $nr)
    };
}
#[allow(unused_macros)]
macro_rules! _IOR {
    ($ty:expr, $nr:expr, $size:ty) => {
        _IOC!(2u32, $ty as u32, $nr as u32, core::mem::size_of::<$size>() as u32)
    };
}
#[allow(unused_macros)]
macro_rules! _IOWR {
    ($ty:expr, $nr:expr, $size:ty) => {
        _IOC!(3u32, $ty as u32, $nr as u32, core::mem::size_of::<$size>() as u32)
    };
}

pub const I8K_BIOS_VERSION: u32 = _IOR!('i', 0x80, i32); // broken: meant 4 bytes
pub const I8K_MACHINE_ID: u32 = _IOR!('i', 0x81, i32); // broken: meant 16 bytes
pub const I8K_POWER_STATUS: u32 = _IOR!('i', 0x82, usize);
pub const I8K_FN_STATUS: u32 = _IOR!('i', 0x83, usize);
pub const I8K_GET_TEMP: u32 = _IOR!('i', 0x84, usize);
pub const I8K_GET_SPEED: u32 = _IOWR!('i', 0x85, usize);
pub const I8K_GET_FAN: u32 = _IOWR!('i', 0x86, usize);
pub const I8K_SET_FAN: u32 = _IOWR!('i', 0x87, usize);

pub const I8K_FAN_LEFT: i32 = 1;
pub const I8K_FAN_RIGHT: i32 = 0;
pub const I8K_FAN_OFF: i32 = 0;
pub const I8K_FAN_LOW: i32 = 1;
pub const I8K_FAN_HIGH: i32 = 2;
pub const I8K_FAN_TURBO: i32 = 3;
/* Many machines treat this mode as some sort of automatic mode */
pub const I8K_FAN_AUTO: i32 = 3;
pub const I8K_FAN_MAX: i32 = I8K_FAN_TURBO;

pub const I8K_VOL_UP: i32 = 1;
pub const I8K_VOL_DOWN: i32 = 2;
pub const I8K_VOL_MUTE: i32 = 4;

pub const I8K_AC: i32 = 1;
pub const I8K_BATTERY: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
