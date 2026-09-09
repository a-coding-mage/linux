/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Machine dependent access functions for RTC registers.
 */

/* Dependency equivalent of <asm/io.h>. */
extern "C" {
    pub fn outb_p(value: u8, port: u16);
    pub fn inb_p(port: u16) -> u8;
}

/*
 * The C header defines these only when RTC_PORT has not already been
 * provided by the build configuration. Rust conditional configuration for
 * an externally supplied RTC_PORT is preserved by this local default.
 */
#[macro_export]
macro_rules! RTC_PORT {
    ($x:expr) => {
        0x70u16.wrapping_add(($x) as u16)
    };
}

pub const RTC_ALWAYS_BCD: i32 = 1; /* RTC operates in binary mode */

/*
 * The yet supported machines all access the RTC index register via
 * an ISA port access but the way to access the date register differs ...
 */
#[macro_export]
macro_rules! CMOS_READ {
    ($addr:expr) => {{
        unsafe {
            $crate::outb_p(($addr) as u8, $crate::RTC_PORT!(0));
            $crate::inb_p($crate::RTC_PORT!(1))
        }
    }};
}

#[macro_export]
macro_rules! CMOS_WRITE {
    ($val:expr, $addr:expr) => {{
        unsafe {
            $crate::outb_p(($addr) as u8, $crate::RTC_PORT!(0));
            $crate::outb_p(($val) as u8, $crate::RTC_PORT!(1));
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
