// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{linux.intel,addtoit}.com)
 */

// Dependencies supplied by the surrounding kernel/UML sources.

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

extern "C" {
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
}

// These are called from os-Linux/sigio.c to protect its pollfds arrays.
// Corresponds to the C DEFINE_MUTEX(sigio_mutex) declaration.
static mut sigio_mutex: mutex = mutex { _private: [] };

pub unsafe fn sigio_lock() {
    mutex_lock(&raw mut sigio_mutex);
}

pub unsafe fn sigio_unlock() {
    mutex_unlock(&raw mut sigio_mutex);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
