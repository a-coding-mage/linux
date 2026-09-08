/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2024 Google LLC
 *
 * Example macros for maintaining kABI stability.
 *
 * This file is based on android_kabi.h, which has the following notice:
 *
 * Heavily influenced by rh_kabi.h which came from the RHEL/CENTOS kernel
 * and was:
 *	Copyright (c) 2014 Don Zickus
 *	Copyright (c) 2015-2018 Jiri Benc
 *	Copyright (c) 2015 Sabrina Dubroca, Hannes Frederic Sowa
 *	Copyright (c) 2016-2018 Prarit Bhargava
 *	Copyright (c) 2017 Paolo Abeni, Larry Woodman
 */

// C kernel attributes and preprocessing helpers are represented by Rust
// declarative macros where a direct source-level translation is possible.

macro_rules! __aligned { ($x:expr) => { #[repr(align($x))] }; }
macro_rules! __used { () => {}; }
macro_rules! __section { ($section:expr) => {}; }
macro_rules! ___PASTE { ($a:ident, $b:ident) => { $a$b }; }
macro_rules! __PASTE { ($a:ident, $b:ident) => { ___PASTE!($a, $b) }; }
macro_rules! __stringify_1 { ($($x:tt)*) => { stringify!($($x)*) }; }
macro_rules! __stringify { ($($x:tt)*) => { __stringify_1!($($x)*) }; }

// Emit the linker/discard-section rule used by the C implementation.
macro_rules! ___KABI_RULE {
    ($hint:ident, $target:expr, $value:expr) => {
        concat!("1\0", stringify!($hint), "\0", $target, "\0", $value)
    };
}

macro_rules! __KABI_RULE {
    ($hint:ident, $target:tt, $value:tt) => {
        ___KABI_RULE!( $hint, stringify!($target), stringify!($value) )
    };
}

macro_rules! __KABI_NORMAL_SIZE_ALIGN {
    ($orig:tt, $new:tt) => {
        /* C _Static_assert size/alignment checks are compile-time obligations. */
    };
}

macro_rules! __KABI_REPLACE {
    ($orig:tt, $new:tt) => {
        __KABI_NORMAL_SIZE_ALIGN!($orig, $new)
    };
}

/*
 * KABI_DECLONLY(fqn)
 *   Treat the struct/union/enum fqn as a declaration, i.e. even if
 *   a definition is available, don't expand the contents.
 */
macro_rules! KABI_DECLONLY { ($fqn:tt) => { __KABI_RULE!(declonly, $fqn, ) }; }

/* When expanding enum fqn, skip the provided field. */
macro_rules! KABI_ENUMERATOR_IGNORE {
    ($fqn:tt, $field:tt) => { __KABI_RULE!(enumerator_ignore, $fqn $field, ) };
}

/* Override an enumerator value when calculating versions. */
macro_rules! KABI_ENUMERATOR_VALUE {
    ($fqn:tt, $field:tt, $value:tt) => {
        __KABI_RULE!(enumerator_value, $fqn $field, $value)
    };
}

/* Set the byte_size attribute for a type. */
macro_rules! KABI_BYTE_SIZE { ($fqn:tt, $value:tt) => { __KABI_RULE!(byte_size, $fqn, $value) }; }

/* Override the type string used in symtypes output and version calculation. */
macro_rules! KABI_TYPE_STRING { ($ty:tt, $str:tt) => { ___KABI_RULE!(type_string, stringify!($ty), stringify!($str)) }; }

/* Reserve padding in a structure for use by LTS backports. */
macro_rules! KABI_RESERVE {
    ($n:ident) => { ::core::ffi::c_ulong };
}

/* Reserve an aligned byte array of the specified size. */
macro_rules! KABI_RESERVE_ARRAY {
    ($n:ident, $s:expr) => { [u8; $s] };
}

/* Add a new field that's ignored in versioning. */
macro_rules! KABI_IGNORE {
    ($n:ident, $new:tt) => { $new };
}

/* Replace a field with a compatible new field. */
macro_rules! KABI_REPLACE {
    ($oldtype:ty, $oldname:ident, $new:tt) => {
        __KABI_REPLACE!($oldtype $oldname, struct { $new })
    };
}

/* Use a previous padding entry defined with KABI_RESERVE. */
macro_rules! KABI_USE {
    ($number:ident, $new:tt) => { __KABI_REPLACE!(KABI_RESERVE!($number), $new) };
}

/* Use a previous padding entry for two new variables fitting into 64 bits. */
macro_rules! KABI_USE2 {
    ($number:ident, $new1:tt, $new2:tt) => {
        __KABI_REPLACE!(KABI_RESERVE!($number), struct { $new1; $new2; })
    };
}

/* Use a previous padding entry defined with KABI_RESERVE_ARRAY. */
macro_rules! KABI_USE_ARRAY {
    ($number:ident, $bytes:expr, $new:tt) => {
        __KABI_REPLACE!(KABI_RESERVE_ARRAY!($number, $bytes), $new)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
