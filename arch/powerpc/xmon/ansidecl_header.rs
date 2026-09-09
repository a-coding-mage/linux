/* SPDX-License-Identifier: GPL-2.0-or-later */
/* ANSI and traditional C compatibility macros, translated from ansidecl.h. */

/* The original header selects these definitions for ANSI C, AIX, SVR4 MIPS,
 * or WIN32 builds. Rust has no equivalent traditional-C compilation mode;
 * the ANSI branch is therefore the direct Rust representation. */

use core::ffi::c_void;

pub type Ptr = *mut c_void;
pub type PtrConst = *mut c_void;
pub type LongDouble = f64;

pub const ANSI_PROTOTYPES: i32 = 1;

/* Token-like compatibility macros retained for source-level translation. */
#[macro_export]
macro_rules! PTR {
    () => { *mut c_void };
}

#[macro_export]
macro_rules! PTRCONST {
    () => { *mut c_void };
}

#[macro_export]
macro_rules! LONG_DOUBLE {
    () => { f64 };
}

#[macro_export]
macro_rules! AND {
    () => { , };
}

#[macro_export]
macro_rules! NOARGS {
    () => { () };
}

#[macro_export]
macro_rules! CONST {
    ($value:expr) => { $value };
}

#[macro_export]
macro_rules! VOLATILE {
    ($value:expr) => { $value };
}

#[macro_export]
macro_rules! SIGNED {
    ($value:ty) => { $value };
}

/* C's variadic token has no direct standalone Rust equivalent. */
#[macro_export]
macro_rules! DOTS {
    () => { /* C variadic marker */ };
}

/* These declaration-building macros preserve the original names and argument
 * ordering; Rust declarations should normally be written explicitly. */
#[macro_export]
macro_rules! EXFUN {
    ($name:ident, $proto:tt) => { $name };
}

#[macro_export]
macro_rules! DEFUN {
    ($name:ident, $arglist:tt, $args:tt) => { $name };
}

#[macro_export]
macro_rules! DEFUN_VOID {
    ($name:ident) => { $name };
}

#[macro_export]
macro_rules! PROTO {
    ($type:ty, $name:ident, $arglist:tt) => { $name };
}

#[macro_export]
macro_rules! PARAMS {
    (($($params:tt)*)) => { ($($params)*) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
