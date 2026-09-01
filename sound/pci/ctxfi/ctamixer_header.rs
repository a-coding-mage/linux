/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	ctamixer.h
 *
 * @Brief
 * This file contains the definition of the Audio Mixer
 * resource management object.
 *
 * @Author	Liu Chun
 * @Date 	May 21 2008
 */

use core::ffi::{c_int, c_uchar, c_uint, c_void};

/* Dependencies from ctresource.h, linux/spinlock.h, and sound/core.h. */

/* Define the descriptor of a summation node resource */
#[repr(C)]
pub struct sum {
    pub rsc: rsc, /* Basic resource info */
    pub idx: [c_uchar; 8],
}

/* Define sum resource request description info */
#[repr(C)]
pub struct sum_desc {
    pub msr: c_uint,
}

#[repr(C)]
pub struct sum_mgr {
    pub mgr: rsc_mgr,        /* Basic resource manager info */
    pub card: *mut snd_card, /* pointer to this card */
    pub mgr_lock: spinlock_t,

    /* request one sum resource */
    pub get_sum: Option<
        unsafe extern "C" fn(
            mgr: *mut sum_mgr,
            desc: *const sum_desc,
            rsum: *mut *mut sum,
        ) -> c_int,
    >,
    /* return one sum resource */
    pub put_sum: Option<unsafe extern "C" fn(mgr: *mut sum_mgr, sum: *mut sum) -> c_int>,
}

unsafe extern "C" {
    /* Constructor and destructor of daio resource manager */
    pub fn sum_mgr_create(hw: *mut hw, ptr: *mut *mut c_void) -> c_int;
    pub fn sum_mgr_destroy(ptr: *mut c_void) -> c_int;
}

/* Define the descriptor of a amixer resource */
#[repr(C)]
pub struct amixer_rsc_ops {
    pub set_input:
        Option<unsafe extern "C" fn(amixer: *mut amixer, rsc: *mut rsc) -> c_int>,
    pub set_scale:
        Option<unsafe extern "C" fn(amixer: *mut amixer, scale: c_uint) -> c_int>,
    pub set_invalid_squash:
        Option<unsafe extern "C" fn(amixer: *mut amixer, iv: c_uint) -> c_int>,
    pub set_sum:
        Option<unsafe extern "C" fn(amixer: *mut amixer, sum: *mut sum) -> c_int>,
    pub commit_write: Option<unsafe extern "C" fn(amixer: *mut amixer) -> c_int>,
    /* Only for interleaved recording */
    pub commit_raw_write: Option<unsafe extern "C" fn(amixer: *mut amixer) -> c_int>,
    pub setup: Option<
        unsafe extern "C" fn(
            amixer: *mut amixer,
            input: *mut rsc,
            scale: c_uint,
            sum: *mut sum,
        ) -> c_int,
    >,
    pub get_scale: Option<unsafe extern "C" fn(amixer: *mut amixer) -> c_int>,
}

#[repr(C)]
pub struct amixer {
    pub rsc: rsc,                    /* Basic resource info */
    pub idx: [c_uchar; 8],
    pub input: *mut rsc,             /* pointer to a resource acting as source */
    pub sum: *mut sum,               /* Put amixer output to this summation node */
    pub ops: *const amixer_rsc_ops,  /* AMixer specific operations */
}

/* Define amixer resource request description info */
#[repr(C)]
pub struct amixer_desc {
    pub msr: c_uint,
}

#[repr(C)]
pub struct amixer_mgr {
    pub mgr: rsc_mgr,        /* Basic resource manager info */
    pub card: *mut snd_card, /* pointer to this card */
    pub mgr_lock: spinlock_t,

    /* request one amixer resource */
    pub get_amixer: Option<
        unsafe extern "C" fn(
            mgr: *mut amixer_mgr,
            desc: *const amixer_desc,
            ramixer: *mut *mut amixer,
        ) -> c_int,
    >,
    /* return one amixer resource */
    pub put_amixer:
        Option<unsafe extern "C" fn(mgr: *mut amixer_mgr, amixer: *mut amixer) -> c_int>,
}

unsafe extern "C" {
    /* Constructor and destructor of amixer resource manager */
    pub fn amixer_mgr_create(hw: *mut hw, ramixer_mgr: *mut *mut c_void) -> c_int;
    pub fn amixer_mgr_destroy(amixer_mgr: *mut c_void) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
