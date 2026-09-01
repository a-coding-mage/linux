// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	ctresource.c
 *
 * @Brief
 * This file contains the implementation of some generic helper functions.
 *
 * @Author	Liu Chun
 * @Date 	May 15 2008
 */

// C dependencies: "ctresource.h", "cthardware.h", <linux/err.h>, <linux/slab.h>

pub type u8 = core::ffi::c_uchar;
pub type u32 = core::ffi::c_uint;

pub const ENOENT: core::ffi::c_int = 2;
pub const EINVAL: core::ffi::c_int = 22;
pub const ENOMEM: core::ffi::c_int = 12;
pub const GFP_KERNEL: core::ffi::c_uint = 0;

const AUDIO_SLOT_BLOCK_NUM: core::ffi::c_uint = 256;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct card {
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RSCTYP {
    SRC = 0,
    SRCIMP = 1,
    AMIXER = 2,
    SUM = 3,
    DAIO = 4,
    NUM_RSCTYP = 5,
}

use RSCTYP::{AMIXER, DAIO, NUM_RSCTYP, SRC, SRCIMP, SUM};

#[repr(C)]
pub struct hw {
    pub card: *mut card,
    pub src_rsc_get_ctrl_blk:
        unsafe extern "C" fn(ctrl_blk: *mut *mut core::ffi::c_void) -> core::ffi::c_int,
    pub amixer_rsc_get_ctrl_blk:
        unsafe extern "C" fn(ctrl_blk: *mut *mut core::ffi::c_void) -> core::ffi::c_int,
    pub src_rsc_put_ctrl_blk: unsafe extern "C" fn(ctrl_blk: *mut core::ffi::c_void),
    pub amixer_rsc_put_ctrl_blk: unsafe extern "C" fn(ctrl_blk: *mut core::ffi::c_void),
    pub src_mgr_get_ctrl_blk:
        unsafe extern "C" fn(ctrl_blk: *mut *mut core::ffi::c_void) -> core::ffi::c_int,
    pub srcimp_mgr_get_ctrl_blk:
        unsafe extern "C" fn(ctrl_blk: *mut *mut core::ffi::c_void) -> core::ffi::c_int,
    pub amixer_mgr_get_ctrl_blk:
        unsafe extern "C" fn(ctrl_blk: *mut *mut core::ffi::c_void) -> core::ffi::c_int,
    pub daio_mgr_get_ctrl_blk:
        unsafe extern "C" fn(hw: *mut hw, ctrl_blk: *mut *mut core::ffi::c_void) -> core::ffi::c_int,
    pub src_mgr_put_ctrl_blk: unsafe extern "C" fn(ctrl_blk: *mut core::ffi::c_void),
    pub srcimp_mgr_put_ctrl_blk: unsafe extern "C" fn(ctrl_blk: *mut core::ffi::c_void),
    pub amixer_mgr_put_ctrl_blk: unsafe extern "C" fn(ctrl_blk: *mut core::ffi::c_void),
    pub daio_mgr_put_ctrl_blk: unsafe extern "C" fn(ctrl_blk: *mut core::ffi::c_void),
}

#[repr(C)]
pub struct rsc_ops {
    pub index: unsafe extern "C" fn(rsc: *const rsc) -> core::ffi::c_int,
    pub output_slot: unsafe extern "C" fn(rsc: *const rsc) -> core::ffi::c_int,
    pub master: unsafe extern "C" fn(rsc: *mut rsc),
    pub next_conj: unsafe extern "C" fn(rsc: *mut rsc),
}

#[repr(C)]
pub struct rsc {
    pub idx: u32,
    pub conj: u32,
    pub type_: RSCTYP,
    pub msr: u32,
    pub hw: *mut hw,
    pub ops: *const rsc_ops,
    pub ctrl_blk: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct rsc_mgr {
    pub type_: RSCTYP,
    pub rscs: *mut u8,
    pub ctrl_blk: *mut core::ffi::c_void,
    pub avail: core::ffi::c_uint,
    pub amount: core::ffi::c_uint,
    pub hw: *mut hw,
}

unsafe extern "C" {
    fn kzalloc(size: usize, flags: core::ffi::c_uint) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

const fn div_round_up(n: core::ffi::c_uint, d: core::ffi::c_uint) -> core::ffi::c_uint {
    (n + d - 1) / d
}

/* Resource allocation based on bit-map management mechanism */
unsafe fn get_resource(
    rscs: *mut u8,
    amount: core::ffi::c_uint,
    multi: core::ffi::c_uint,
    ridx: *mut core::ffi::c_uint,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int;
    let mut j: core::ffi::c_int;
    let mut k: core::ffi::c_int;
    let mut n: core::ffi::c_int;

    /* Check whether there are sufficient resources to meet request. */
    i = 0;
    n = multi as core::ffi::c_int;
    while i < amount as core::ffi::c_int {
        j = i / 8;
        k = i % 8;
        if *rscs.add(j as usize) & ((1 as u8) << k) != 0 {
            n = multi as core::ffi::c_int;
            i += 1;
            continue;
        }
        n -= 1;
        if n == 0 {
            break; /* found sufficient contiguous resources */
        }
        i += 1;
    }

    if i >= amount as core::ffi::c_int {
        /* Can not find sufficient contiguous resources */
        return -ENOENT;
    }

    /* Mark the contiguous bits in resource bit-map as used */
    n = multi as core::ffi::c_int;
    while n > 0 {
        j = i / 8;
        k = i % 8;
        *rscs.add(j as usize) |= (1 as u8) << k;
        i -= 1;
        n -= 1;
    }

    *ridx = (i + 1) as core::ffi::c_uint;

    0
}

unsafe fn put_resource(
    rscs: *mut u8,
    multi: core::ffi::c_uint,
    idx: core::ffi::c_uint,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_uint;
    let mut j: core::ffi::c_uint;
    let mut k: core::ffi::c_uint;
    let mut n: core::ffi::c_uint;

    /* Mark the contiguous bits in resource bit-map as used */
    n = multi;
    i = idx;
    while n > 0 {
        j = i / 8;
        k = i % 8;
        *rscs.add(j as usize) &= !((1 as u8) << k);
        i = i.wrapping_add(1);
        n -= 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mgr_get_resource(
    mgr: *mut rsc_mgr,
    n: core::ffi::c_uint,
    ridx: *mut core::ffi::c_uint,
) -> core::ffi::c_int {
    let err: core::ffi::c_int;

    if n > (*mgr).avail {
        return -ENOENT;
    }

    err = get_resource((*mgr).rscs, (*mgr).amount, n, ridx);
    if err == 0 {
        (*mgr).avail -= n;
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn mgr_put_resource(
    mgr: *mut rsc_mgr,
    n: core::ffi::c_uint,
    idx: core::ffi::c_uint,
) -> core::ffi::c_int {
    put_resource((*mgr).rscs, n, idx);
    (*mgr).avail += n;

    0
}

static offset_in_audio_slot_block: [core::ffi::c_uchar; NUM_RSCTYP as usize] = {
    let mut a = [0; NUM_RSCTYP as usize];
    /* SRC channel is at Audio Ring slot 1 every 16 slots. */
    a[SRC as usize] = 0x1;
    a[AMIXER as usize] = 0x4;
    a[SUM as usize] = 0xc;
    a
};

unsafe extern "C" fn rsc_index(rsc: *const rsc) -> core::ffi::c_int {
    (*rsc).conj as core::ffi::c_int
}

unsafe extern "C" fn audio_ring_slot(rsc: *const rsc) -> core::ffi::c_int {
    (((*rsc).conj << 4) + offset_in_audio_slot_block[(*rsc).type_ as usize] as u32)
        as core::ffi::c_int
}

unsafe extern "C" fn rsc_next_conj(rsc: *mut rsc) {
    let mut i: core::ffi::c_uint;
    i = 0;
    while i < 8 && ((*rsc).msr & (0x1 << i)) == 0 {
        i += 1;
    }
    (*rsc).conj += AUDIO_SLOT_BLOCK_NUM >> i;
}

unsafe extern "C" fn rsc_master(rsc: *mut rsc) {
    (*rsc).conj = (*rsc).idx;
}

static rsc_generic_ops: rsc_ops = rsc_ops {
    index: rsc_index,
    output_slot: audio_ring_slot,
    master: rsc_master,
    next_conj: rsc_next_conj,
};

#[no_mangle]
pub unsafe extern "C" fn rsc_init(
    rsc: *mut rsc,
    idx: u32,
    type_: RSCTYP,
    msr: u32,
    hw: *mut hw,
) -> core::ffi::c_int {
    let mut err: core::ffi::c_int = 0;

    (*rsc).idx = idx;
    (*rsc).conj = idx;
    (*rsc).type_ = type_;
    (*rsc).msr = msr;
    (*rsc).hw = hw;
    (*rsc).ops = &rsc_generic_ops;
    if hw.is_null() {
        (*rsc).ctrl_blk = core::ptr::null_mut();
        return 0;
    }

    match type_ {
        SRC => {
            err = ((*hw).src_rsc_get_ctrl_blk)(&mut (*rsc).ctrl_blk);
        }
        AMIXER => {
            err = ((*hw).amixer_rsc_get_ctrl_blk)(&mut (*rsc).ctrl_blk);
        }
        SRCIMP | SUM | DAIO => {}
        _ => {
            dev_err(
                (*(*hw).card).dev,
                b"Invalid resource type value %d!\n\0".as_ptr() as *const core::ffi::c_char,
                type_ as core::ffi::c_int,
            );
            return -EINVAL;
        }
    }

    if err != 0 {
        dev_err(
            (*(*hw).card).dev,
            b"Failed to get resource control block!\n\0".as_ptr() as *const core::ffi::c_char,
        );
        return err;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn rsc_uninit(rsc: *mut rsc) -> core::ffi::c_int {
    if !(*rsc).hw.is_null() && !(*rsc).ctrl_blk.is_null() {
        match (*rsc).type_ {
            SRC => {
                ((*(*rsc).hw).src_rsc_put_ctrl_blk)((*rsc).ctrl_blk);
            }
            AMIXER => {
                ((*(*rsc).hw).amixer_rsc_put_ctrl_blk)((*rsc).ctrl_blk);
            }
            SUM | DAIO => {}
            _ => {
                dev_err(
                    (*(*(*rsc).hw).card).dev,
                    b"Invalid resource type value %d!\n\0".as_ptr() as *const core::ffi::c_char,
                    (*rsc).type_ as core::ffi::c_int,
                );
            }
        }

        (*rsc).ctrl_blk = core::ptr::null_mut();
        (*rsc).hw = (*rsc).ctrl_blk as *mut hw;
    }

    (*rsc).conj = 0;
    (*rsc).idx = (*rsc).conj;
    (*rsc).type_ = NUM_RSCTYP;
    (*rsc).msr = 0;

    0
}

#[no_mangle]
pub unsafe extern "C" fn rsc_mgr_init(
    mgr: *mut rsc_mgr,
    type_: RSCTYP,
    amount: core::ffi::c_uint,
    hw: *mut hw,
) -> core::ffi::c_int {
    let mut err: core::ffi::c_int = 0;

    (*mgr).type_ = NUM_RSCTYP;

    (*mgr).rscs = kzalloc(div_round_up(amount, 8) as usize, GFP_KERNEL) as *mut u8;
    if (*mgr).rscs.is_null() {
        return -ENOMEM;
    }

    match type_ {
        SRC => {
            err = ((*hw).src_mgr_get_ctrl_blk)(&mut (*mgr).ctrl_blk);
        }
        SRCIMP => {
            err = ((*hw).srcimp_mgr_get_ctrl_blk)(&mut (*mgr).ctrl_blk);
        }
        AMIXER => {
            err = ((*hw).amixer_mgr_get_ctrl_blk)(&mut (*mgr).ctrl_blk);
        }
        DAIO => {
            err = ((*hw).daio_mgr_get_ctrl_blk)(hw, &mut (*mgr).ctrl_blk);
        }
        SUM => {}
        _ => {
            dev_err(
                (*(*hw).card).dev,
                b"Invalid resource type value %d!\n\0".as_ptr() as *const core::ffi::c_char,
                type_ as core::ffi::c_int,
            );
            err = -EINVAL;
            kfree((*mgr).rscs as *mut core::ffi::c_void);
            return err;
        }
    }

    if err != 0 {
        dev_err(
            (*(*hw).card).dev,
            b"Failed to get manager control block!\n\0".as_ptr() as *const core::ffi::c_char,
        );
        kfree((*mgr).rscs as *mut core::ffi::c_void);
        return err;
    }

    (*mgr).type_ = type_;
    (*mgr).amount = amount;
    (*mgr).avail = (*mgr).amount;
    (*mgr).hw = hw;

    0
}

#[no_mangle]
pub unsafe extern "C" fn rsc_mgr_uninit(mgr: *mut rsc_mgr) -> core::ffi::c_int {
    kfree((*mgr).rscs as *mut core::ffi::c_void);
    (*mgr).rscs = core::ptr::null_mut();

    if !(*mgr).hw.is_null() && !(*mgr).ctrl_blk.is_null() {
        match (*mgr).type_ {
            SRC => {
                ((*(*mgr).hw).src_mgr_put_ctrl_blk)((*mgr).ctrl_blk);
            }
            SRCIMP => {
                ((*(*mgr).hw).srcimp_mgr_put_ctrl_blk)((*mgr).ctrl_blk);
            }
            AMIXER => {
                ((*(*mgr).hw).amixer_mgr_put_ctrl_blk)((*mgr).ctrl_blk);
            }
            DAIO => {
                ((*(*mgr).hw).daio_mgr_put_ctrl_blk)((*mgr).ctrl_blk);
            }
            SUM => {}
            _ => {
                dev_err(
                    (*(*(*mgr).hw).card).dev,
                    b"Invalid resource type value %d!\n\0".as_ptr() as *const core::ffi::c_char,
                    (*mgr).type_ as core::ffi::c_int,
                );
            }
        }

        (*mgr).ctrl_blk = core::ptr::null_mut();
        (*mgr).hw = (*mgr).ctrl_blk as *mut hw;
    }

    (*mgr).type_ = NUM_RSCTYP;
    (*mgr).amount = 0;
    (*mgr).avail = (*mgr).amount;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
