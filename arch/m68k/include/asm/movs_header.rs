/* SPDX-License-Identifier: GPL-2.0 */

/*
** movs.h
**
** Inline assembly macros to generate movs & related instructions
*/

/* Set DFC register value */
#[macro_export]
macro_rules! SET_DFC {
    ($x:expr) => {
        unsafe { core::arch::asm!("movec {0},dfc", in(reg) $x) }
    };
}

/* Get DFC register value */
#[macro_export]
macro_rules! GET_DFC {
    ($x:expr) => {
        unsafe { core::arch::asm!("movec dfc, {0}", out(reg) $x) }
    };
}

/* Set SFC register value */
#[macro_export]
macro_rules! SET_SFC {
    ($x:expr) => {
        unsafe { core::arch::asm!("movec {0},sfc", in(reg) $x) }
    };
}

/* Get SFC register value */
#[macro_export]
macro_rules! GET_SFC {
    ($x:expr) => {
        unsafe { core::arch::asm!("movec sfc, {0}", out(reg) $x) }
    };
}

#[macro_export]
macro_rules! SET_VBR {
    ($x:expr) => {
        unsafe { core::arch::asm!("movec {0},vbr", in(reg) $x) }
    };
}

#[macro_export]
macro_rules! GET_VBR {
    ($x:expr) => {
        unsafe { core::arch::asm!("movec vbr, {0}", out(reg) $x) }
    };
}

/* Set a byte using the "movs" instruction */
#[macro_export]
macro_rules! SET_CONTROL_BYTE {
    ($addr:expr, $value:expr) => {
        unsafe { core::arch::asm!("movsb {0}, {1}@", in(reg) $value, in(reg) $addr) }
    };
}

/* Get a byte using the "movs" instruction */
#[macro_export]
macro_rules! GET_CONTROL_BYTE {
    ($addr:expr, $value:expr) => {
        unsafe { core::arch::asm!("movsb {1}@, {0}", out(reg) $value, in(reg) $addr) }
    };
}

/* Set a (long)word using the "movs" instruction */
#[macro_export]
macro_rules! SET_CONTROL_WORD {
    ($addr:expr, $value:expr) => {
        unsafe { core::arch::asm!("movsl {0}, {1}@", in(reg) $value, in(reg) $addr) }
    };
}

/* Get a (long)word using the "movs" instruction */
#[macro_export]
macro_rules! GET_CONTROL_WORD {
    ($addr:expr, $value:expr) => {
        unsafe { core::arch::asm!("movsl {1}@, {0}", out(reg) $value, in(reg) $addr) }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
