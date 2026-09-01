/* SPDX-License-Identifier: GPL-2.0 */

// C dependency/header guard intent:
// #ifndef _MUTEX_H
// #define _MUTEX_H

macro_rules! DEFINE_MUTEX {
    ($name:ident) => {
        static mut $name: core::ffi::c_int = 0;
    };
}

pub(crate) use DEFINE_MUTEX;

#[inline]
pub unsafe extern "C" fn dummy_mutex_guard(_name: *mut core::ffi::c_int) {}

// C macro intent:
// #define guard(mutex) dummy_##mutex##_guard
//
// Rust macro_rules! cannot concatenate identifiers without an external helper.
// The only file-local expansion present here is guard(mutex) -> dummy_mutex_guard.
macro_rules! guard {
    (mutex) => {
        dummy_mutex_guard
    };
}

pub(crate) use guard;

// #endif /* _MUTEX_H */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
