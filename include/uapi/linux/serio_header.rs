/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 1999-2002 Vojtech Pavlik
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation.
 */

// Dependency supplied by the corresponding Linux ioctl definitions.
pub const SPIOCSTYPE: core::ffi::c_ulong =
    _IOW(b'q' as core::ffi::c_ulong, 0x01, core::mem::size_of::<core::ffi::c_ulong>());

/*
 * bit masks for use in "interrupt" flags (3rd argument)
 */
pub const SERIO_TIMEOUT: core::ffi::c_ulong = 1 as core::ffi::c_ulong;
pub const SERIO_PARITY: core::ffi::c_ulong = (1 as core::ffi::c_ulong) << 1;
pub const SERIO_FRAME: core::ffi::c_ulong = (1 as core::ffi::c_ulong) << 2;
pub const SERIO_OOB_DATA: core::ffi::c_ulong = (1 as core::ffi::c_ulong) << 3;

/*
 * Serio types
 */
pub const SERIO_XT: u32 = 0x00;
pub const SERIO_8042: u32 = 0x01;
pub const SERIO_RS232: u32 = 0x02;
pub const SERIO_HIL_MLC: u32 = 0x03;
pub const SERIO_PS_PSTHRU: u32 = 0x05;
pub const SERIO_8042_XL: u32 = 0x06;

/*
 * Serio protocols
 */
pub const SERIO_UNKNOWN: u32 = 0x00;
pub const SERIO_MSC: u32 = 0x01;
pub const SERIO_SUN: u32 = 0x02;
pub const SERIO_MS: u32 = 0x03;
pub const SERIO_MP: u32 = 0x04;
pub const SERIO_MZ: u32 = 0x05;
pub const SERIO_MZP: u32 = 0x06;
pub const SERIO_MZPP: u32 = 0x07;
pub const SERIO_VSXXXAA: u32 = 0x08;
pub const SERIO_SUNKBD: u32 = 0x10;
pub const SERIO_WARRIOR: u32 = 0x18;
pub const SERIO_SPACEORB: u32 = 0x19;
pub const SERIO_MAGELLAN: u32 = 0x1a;
pub const SERIO_SPACEBALL: u32 = 0x1b;
pub const SERIO_GUNZE: u32 = 0x1c;
pub const SERIO_IFORCE: u32 = 0x1d;
pub const SERIO_STINGER: u32 = 0x1e;
pub const SERIO_NEWTON: u32 = 0x1f;
pub const SERIO_STOWAWAY: u32 = 0x20;
pub const SERIO_H3600: u32 = 0x21;
pub const SERIO_PS2SER: u32 = 0x22;
pub const SERIO_TWIDKBD: u32 = 0x23;
pub const SERIO_TWIDJOY: u32 = 0x24;
pub const SERIO_HIL: u32 = 0x25;
pub const SERIO_SNES232: u32 = 0x26;
pub const SERIO_SEMTECH: u32 = 0x27;
pub const SERIO_LKKBD: u32 = 0x28;
pub const SERIO_ELO: u32 = 0x29;
pub const SERIO_MICROTOUCH: u32 = 0x30;
pub const SERIO_PENMOUNT: u32 = 0x31;
pub const SERIO_TOUCHRIGHT: u32 = 0x32;
pub const SERIO_TOUCHWIN: u32 = 0x33;
pub const SERIO_TAOSEVM: u32 = 0x34;
pub const SERIO_FUJITSU: u32 = 0x35;
pub const SERIO_ZHENHUA: u32 = 0x36;
pub const SERIO_INEXIO: u32 = 0x37;
pub const SERIO_TOUCHIT213: u32 = 0x38;
pub const SERIO_W8001: u32 = 0x39;
pub const SERIO_DYNAPRO: u32 = 0x3a;
pub const SERIO_HAMPSHIRE: u32 = 0x3b;
pub const SERIO_PS2MULT: u32 = 0x3c;
pub const SERIO_TSC40: u32 = 0x3d;
pub const SERIO_WACOM_IV: u32 = 0x3e;
pub const SERIO_EGALAX: u32 = 0x3f;
pub const SERIO_PULSE8_CEC: u32 = 0x40;
pub const SERIO_RAINSHADOW_CEC: u32 = 0x41;
pub const SERIO_FSIA6B: u32 = 0x42;
pub const SERIO_EXTRON_DA_HD_4K_PLUS: u32 = 0x43;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
