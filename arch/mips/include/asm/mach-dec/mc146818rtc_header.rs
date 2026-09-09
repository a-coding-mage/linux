/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * RTC definitions for DECstation style attached Dallas DS1287 chip.
 *
 * Copyright (C) 1998, 2001 by Ralf Baechle
 * Copyright (C) 1998 by Harald Koerfgen
 * Copyright (C) 2002, 2005  Maciej W. Rozycki
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h, asm/addrspace.h, and asm/dec/system.h.

unsafe extern "C" {
    pub static mut dec_rtc_base: *mut u8;
    pub static mut dec_kn_slot_size: usize;
}

pub const ARCH_RTC_LOCATION: () = ();

// CPHYSADDR is supplied by asm/addrspace.h.
#[macro_export]
macro_rules! RTC_PORT {
    ($x:expr) => {
        CPHYSADDR(dec_rtc_base as isize)
    };
}

#[macro_export]
macro_rules! RTC_IO_EXTENT {
    () => {
        dec_kn_slot_size
    };
}

pub const RTC_IOMAPPED: i32 = 0;
// #undef RTC_IRQ: RTC_IRQ is intentionally not defined for this platform.

#[inline]
pub unsafe fn CMOS_READ(addr: u64) -> u8 {
    core::ptr::read_volatile(dec_rtc_base.add(addr as usize * 4))
}

#[inline]
pub unsafe fn CMOS_WRITE(data: u8, addr: u64) {
    core::ptr::write_volatile(dec_rtc_base.add(addr as usize * 4), data);
}

pub const RTC_ALWAYS_BCD: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
