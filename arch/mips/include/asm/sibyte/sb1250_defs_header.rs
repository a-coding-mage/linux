/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SB1250 Board Support Package: global constants and macros. */

/* The original header requires ANSI C89 concatenation and 64-bit integers. */

pub const SIBYTE_HDR_FMASK_1250_ALL: u32 = 0x000000ff;
pub const SIBYTE_HDR_FMASK_1250_PASS1: u32 = 0x00000001;
pub const SIBYTE_HDR_FMASK_1250_PASS2: u32 = 0x00000002;
pub const SIBYTE_HDR_FMASK_1250_PASS3: u32 = 0x00000004;

pub const SIBYTE_HDR_FMASK_112x_ALL: u32 = 0x00000f00;
pub const SIBYTE_HDR_FMASK_112x_PASS1: u32 = 0x00000100;

pub const SIBYTE_HDR_FMASK_1480_ALL: u32 = 0x0000f000;
pub const SIBYTE_HDR_FMASK_1480_PASS1: u32 = 0x00001000;
pub const SIBYTE_HDR_FMASK_1480_PASS2: u32 = 0x00002000;

#[macro_export]
macro_rules! SIBYTE_HDR_FMASK {
    (1250, PASS1) => { $crate::SIBYTE_HDR_FMASK_1250_PASS1 };
    (1250, PASS2) => { $crate::SIBYTE_HDR_FMASK_1250_PASS2 };
    (1250, PASS3) => { $crate::SIBYTE_HDR_FMASK_1250_PASS3 };
    (112x, PASS1) => { $crate::SIBYTE_HDR_FMASK_112x_PASS1 };
    (1480, PASS1) => { $crate::SIBYTE_HDR_FMASK_1480_PASS1 };
    (1480, PASS2) => { $crate::SIBYTE_HDR_FMASK_1480_PASS2 };
}

#[macro_export]
macro_rules! SIBYTE_HDR_FMASK_ALLREVS {
    (1250) => { $crate::SIBYTE_HDR_FMASK_1250_ALL };
    (112x) => { $crate::SIBYTE_HDR_FMASK_112x_ALL };
    (1480) => { $crate::SIBYTE_HDR_FMASK_1480_ALL };
}

pub const SIBYTE_HDR_FMASK_ALL: u32 =
    SIBYTE_HDR_FMASK_1250_ALL | SIBYTE_HDR_FMASK_112x_ALL | SIBYTE_HDR_FMASK_1480_ALL;
pub const SIBYTE_HDR_FMASK_1250_112x_ALL: u32 =
    SIBYTE_HDR_FMASK_1250_ALL | SIBYTE_HDR_FMASK_112x_ALL;
pub const SIBYTE_HDR_FMASK_1250_112x: u32 = SIBYTE_HDR_FMASK_1250_112x_ALL;

/* Build-time SIBYTE_HDR_FEATURES defaults to all chips and revisions. */
pub const SIBYTE_HDR_FEATURES: u32 = SIBYTE_HDR_FMASK_ALL;

#[macro_export]
macro_rules! SIBYTE_HDR_FMASK_BEFORE {
    ($chip:tt, $pass:tt) => {
        ($crate::SIBYTE_HDR_FMASK!($chip, $pass).wrapping_sub(1)
            & $crate::SIBYTE_HDR_FMASK_ALLREVS!($chip))
    };
}

#[macro_export]
macro_rules! SIBYTE_HDR_FMASK_AFTER {
    ($chip:tt, $pass:tt) => {
        (!($crate::SIBYTE_HDR_FMASK!($chip, $pass)
            | $crate::SIBYTE_HDR_FMASK!($chip, $pass).wrapping_sub(1)))
            & $crate::SIBYTE_HDR_FMASK_ALLREVS!($chip)
    };
}

#[macro_export]
macro_rules! SIBYTE_HDR_FEATURE_CHIP {
    ($chip:tt) => {
        (($crate::SIBYTE_HDR_FMASK_ALLREVS!($chip) & $crate::SIBYTE_HDR_FEATURES) != 0)
    };
}

pub const SIBYTE_HDR_FEATURE_1250_112x: bool =
    SIBYTE_HDR_FMASK_1250_ALL & SIBYTE_HDR_FEATURES != 0
        || SIBYTE_HDR_FMASK_112x_ALL & SIBYTE_HDR_FEATURES != 0;

#[macro_export]
macro_rules! SIBYTE_HDR_FEATURE {
    ($chip:tt, $pass:tt) => {
        (($crate::SIBYTE_HDR_FMASK!($chip, $pass)
            | $crate::SIBYTE_HDR_FMASK_AFTER!($chip, $pass))
            & $crate::SIBYTE_HDR_FEATURES) != 0
    };
}

#[macro_export]
macro_rules! SIBYTE_HDR_FEATURE_EXACT {
    ($chip:tt, $pass:tt) => {
        ($crate::SIBYTE_HDR_FMASK!($chip, $pass) & $crate::SIBYTE_HDR_FEATURES) != 0
    };
}

#[macro_export]
macro_rules! SIBYTE_HDR_FEATURE_UP_TO {
    ($chip:tt, $pass:tt) => {
        (($crate::SIBYTE_HDR_FMASK!($chip, $pass)
            | $crate::SIBYTE_HDR_FMASK_BEFORE!($chip, $pass))
            & $crate::SIBYTE_HDR_FEATURES) != 0
    };
}

#[macro_export]
macro_rules! _SB_MAKE64 { ($x:expr) => { $x as u64 }; }
#[macro_export]
macro_rules! _SB_MAKE32 { ($x:expr) => { $x as u32 }; }
#[macro_export]
macro_rules! _SB_MAKEMASK1 { ($n:expr) => { (1u64) << ($n as u64) }; }
#[macro_export]
macro_rules! _SB_MAKEMASK1_32 { ($n:expr) => { (1u32) << ($n as u32) }; }
#[macro_export]
macro_rules! _SB_MAKEMASK {
    ($v:expr, $n:expr) => { (((1u64 << ($v as u64)).wrapping_sub(1)) << ($n as u64)) };
}
#[macro_export]
macro_rules! _SB_MAKEMASK_32 {
    ($v:expr, $n:expr) => { (((1u32 << ($v as u32)).wrapping_sub(1)) << ($n as u32)) };
}
#[macro_export]
macro_rules! _SB_MAKEVALUE { ($v:expr, $n:expr) => { ($v as u64) << ($n as u64) }; }
#[macro_export]
macro_rules! _SB_MAKEVALUE_32 { ($v:expr, $n:expr) => { ($v as u32) << ($n as u32) }; }
#[macro_export]
macro_rules! _SB_GETVALUE { ($v:expr, $n:expr, $m:expr) => { (($v as u64 & $m as u64) >> $n as u64) }; }
#[macro_export]
macro_rules! _SB_GETVALUE_32 { ($v:expr, $n:expr, $m:expr) => { (($v as u32 & $m as u32) >> $n as u32) }; }

/* __mips64, non-assembler builds: CSR access uses volatile physical K1 addresses. */
#[macro_export]
macro_rules! SBWRITECSR {
    ($csr:expr, $val:expr) => {{
        unsafe { core::ptr::write_volatile(PHYS_TO_K1!($csr) as *mut u64, $val as u64); }
    }};
}
#[macro_export]
macro_rules! SBREADCSR {
    ($csr:expr) => {{ unsafe { core::ptr::read_volatile(PHYS_TO_K1!($csr) as *const u64) } }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
