/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	ctresource.h
 *
 * @Brief
 * This file contains the definition of generic hardware resources for
 * resource management.
 *
 * @Author	Liu Chun
 * @Date 	May 13 2008
 */

/* C dependency intent: #include <linux/types.h> */

use core::ffi::{c_int, c_void};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSCTYP {
    SRC,
    SRCIMP,
    AMIXER,
    SUM,
    DAIO,
    NUM_RSCTYP, /* This must be the last one and less than 16 */
}

#[repr(C)]
pub struct hw {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct rsc_ops {
    pub master: Option<unsafe extern "C" fn(rsc: *mut rsc)>, /* Move to master resource */
    pub next_conj: Option<unsafe extern "C" fn(rsc: *mut rsc)>, /* Move to next conjugate resource */
    pub index: Option<unsafe extern "C" fn(rsc: *const rsc) -> c_int>, /* Return the index of resource */
    /* Return the output slot number */
    pub output_slot: Option<unsafe extern "C" fn(rsc: *const rsc) -> c_int>,
}

#[repr(C)]
pub struct rsc {
    /*
     * C bitfields packed into this word:
     * u32 idx:12;  The index of a resource
     * u32 type:4;  The type (RSCTYP) of a resource
     * u32 conj:12; Current conjugate index
     * u32 msr:4;   The Master Sample Rate a resource working on
     */
    pub bitfields: u32,
    pub ctrl_blk: *mut c_void, /* Chip specific control info block for a resource */
    pub hw: *mut hw, /* Chip specific object for hardware access means */
    pub ops: *const rsc_ops, /* Generic resource operations */
}

impl rsc {
    #[inline]
    pub unsafe fn idx(&self) -> u32 {
        self.bitfields & 0x0fff
    }

    #[inline]
    pub unsafe fn set_idx(&mut self, val: u32) {
        self.bitfields = (self.bitfields & !0x0fff) | (val & 0x0fff);
    }

    #[inline]
    pub unsafe fn type_(&self) -> u32 {
        (self.bitfields >> 12) & 0x000f
    }

    #[inline]
    pub unsafe fn set_type(&mut self, val: u32) {
        self.bitfields = (self.bitfields & !(0x000f << 12)) | ((val & 0x000f) << 12);
    }

    #[inline]
    pub unsafe fn conj(&self) -> u32 {
        (self.bitfields >> 16) & 0x0fff
    }

    #[inline]
    pub unsafe fn set_conj(&mut self, val: u32) {
        self.bitfields = (self.bitfields & !(0x0fff << 16)) | ((val & 0x0fff) << 16);
    }

    #[inline]
    pub unsafe fn msr(&self) -> u32 {
        (self.bitfields >> 28) & 0x000f
    }

    #[inline]
    pub unsafe fn set_msr(&mut self, val: u32) {
        self.bitfields = (self.bitfields & !(0x000f << 28)) | ((val & 0x000f) << 28);
    }
}

unsafe extern "C" {
    pub fn rsc_init(rsc: *mut rsc, idx: u32, type_: RSCTYP, msr: u32, hw: *mut hw) -> c_int;
    pub fn rsc_uninit(rsc: *mut rsc) -> c_int;
}

#[repr(C)]
pub struct rsc_mgr {
    pub type_: RSCTYP, /* The type (RSCTYP) of resource to manage */
    pub amount: c_uint, /* The total amount of a kind of resource */
    pub avail: c_uint, /* The amount of currently available resources */
    pub rscs: *mut c_uchar, /* The bit-map for resource allocation */
    pub ctrl_blk: *mut c_void, /* Chip specific control info block */
    pub hw: *mut hw, /* Chip specific object for hardware access */
}

pub type c_uint = ::core::ffi::c_uint;
pub type c_uchar = ::core::ffi::c_uchar;

/* Resource management is based on bit-map mechanism */
unsafe extern "C" {
    pub fn rsc_mgr_init(
        mgr: *mut rsc_mgr,
        type_: RSCTYP,
        amount: c_uint,
        hw: *mut hw,
    ) -> c_int;
    pub fn rsc_mgr_uninit(mgr: *mut rsc_mgr) -> c_int;
    pub fn mgr_get_resource(mgr: *mut rsc_mgr, n: c_uint, ridx: *mut c_uint) -> c_int;
    pub fn mgr_put_resource(mgr: *mut rsc_mgr, n: c_uint, idx: c_uint) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
