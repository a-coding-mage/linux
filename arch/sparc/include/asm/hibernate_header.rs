/* SPDX-License-Identifier: GPL-2.0 */
/*
 * hibernate.h:  Hibernaton support specific for sparc64.
 *
 * Copyright (C) 2013 Kirill V Tkhai (tkhai@yandex.ru)
 */

#[repr(C)]
pub struct saved_context {
    pub fp: ::core::ffi::c_ulong,
    pub cwp: ::core::ffi::c_ulong,
    pub wstate: ::core::ffi::c_ulong,

    pub tick: ::core::ffi::c_ulong,
    pub pstate: ::core::ffi::c_ulong,

    pub g4: ::core::ffi::c_ulong,
    pub g5: ::core::ffi::c_ulong,
    pub g6: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
