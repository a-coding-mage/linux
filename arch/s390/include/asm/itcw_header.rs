/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Functions for incremental construction of fcx enabled I/O control blocks.
 *
 *    Copyright IBM Corp. 2008
 *    Author(s): Peter Oberparleiter <peter.oberparleiter@de.ibm.com>
 */

// Dependencies supplied by the surrounding translation unit:
// linux/types.h and asm/fcx.h

pub const ITCW_OP_READ: i32 = 0;
pub const ITCW_OP_WRITE: i32 = 1;

#[repr(C)]
pub struct itcw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dcw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tidaw {
    _private: [u8; 0],
}

extern "C" {
    pub fn itcw_get_tcw(itcw: *mut itcw) -> *mut tcw;
    pub fn itcw_calc_size(
        intrg: i32,
        max_tidaws: i32,
        intrg_max_tidaws: i32,
    ) -> usize;
    pub fn itcw_init(
        buffer: *mut core::ffi::c_void,
        size: usize,
        op: i32,
        intrg: i32,
        max_tidaws: i32,
        intrg_max_tidaws: i32,
    ) -> *mut itcw;
    pub fn itcw_add_dcw(
        itcw: *mut itcw,
        cmd: u8,
        flags: u8,
        cd: *mut core::ffi::c_void,
        cd_count: u8,
        count: u32,
    ) -> *mut dcw;
    pub fn itcw_add_tidaw(
        itcw: *mut itcw,
        flags: u8,
        addr: *mut core::ffi::c_void,
        count: u32,
    ) -> *mut tidaw;
    pub fn itcw_set_data(itcw: *mut itcw, addr: *mut core::ffi::c_void, use_tidal: i32);
    pub fn itcw_finalize(itcw: *mut itcw);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
