// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	ctamixer.c
 *
 * @Brief
 * This file contains the implementation of the Audio Mixer
 * resource management object.
 *
 * @Author	Liu Chun
 * @Date 	May 21 2008
 */

/* Original C dependencies: "ctamixer.h", "cthardware.h", <linux/slab.h> */

use core::mem::size_of;
use core::ptr;

use crate::*;

const AMIXER_RESOURCE_NUM: u32 = 256;
const SUM_RESOURCE_NUM: u32 = 256;

const AMIXER_Y_IMMEDIATE: u32 = 1;

const BLANK_SLOT: u32 = 4094;

unsafe extern "C" fn amixer_master(rsc: *mut rsc) {
    unsafe {
        (*rsc).conj = 0;
        (*rsc).idx = (*container_of!(rsc, amixer, rsc)).idx[0];
    }
}

unsafe extern "C" fn amixer_next_conj(rsc: *mut rsc) {
    unsafe {
        (*rsc).conj += 1;
    }
}

unsafe extern "C" fn amixer_index(rsc: *const rsc) -> i32 {
    unsafe { (*container_of!(rsc, amixer, rsc)).idx[(*rsc).conj as usize] as i32 }
}

unsafe extern "C" fn amixer_output_slot(rsc: *const rsc) -> i32 {
    unsafe { (amixer_index(rsc) << 4) + 0x4 }
}

static amixer_basic_rsc_ops: rsc_ops = rsc_ops {
    master: Some(amixer_master),
    next_conj: Some(amixer_next_conj),
    index: Some(amixer_index),
    output_slot: Some(amixer_output_slot),
};

unsafe extern "C" fn amixer_set_input(amixer: *mut amixer, rsc: *mut rsc) -> i32 {
    let hw: *mut hw;

    unsafe {
        hw = (*amixer).rsc.hw;
        ((*hw).amixer_set_mode).unwrap()((*amixer).rsc.ctrl_blk, AMIXER_Y_IMMEDIATE);
        (*amixer).input = rsc;
        if rsc.is_null() {
            ((*hw).amixer_set_x).unwrap()((*amixer).rsc.ctrl_blk, BLANK_SLOT);
        } else {
            ((*hw).amixer_set_x).unwrap()(
                (*amixer).rsc.ctrl_blk,
                ((*(*rsc).ops).output_slot).unwrap()(rsc as *const rsc) as u32,
            );
        }
    }

    0
}

/* y is a 14-bit immediate constant */
unsafe extern "C" fn amixer_set_y(amixer: *mut amixer, y: u32) -> i32 {
    let hw: *mut hw;

    unsafe {
        hw = (*amixer).rsc.hw;
        ((*hw).amixer_set_y).unwrap()((*amixer).rsc.ctrl_blk, y);
    }

    0
}

unsafe extern "C" fn amixer_set_invalid_squash(amixer: *mut amixer, iv: u32) -> i32 {
    let hw: *mut hw;

    unsafe {
        hw = (*amixer).rsc.hw;
        ((*hw).amixer_set_iv).unwrap()((*amixer).rsc.ctrl_blk, iv);
    }

    0
}

unsafe extern "C" fn amixer_set_sum(amixer: *mut amixer, sum: *mut sum) -> i32 {
    let hw: *mut hw;

    unsafe {
        hw = (*amixer).rsc.hw;
        (*amixer).sum = sum;
        if sum.is_null() {
            ((*hw).amixer_set_se).unwrap()((*amixer).rsc.ctrl_blk, 0);
        } else {
            ((*hw).amixer_set_se).unwrap()((*amixer).rsc.ctrl_blk, 1);
            ((*hw).amixer_set_sadr).unwrap()(
                (*amixer).rsc.ctrl_blk,
                ((*(*(*sum).rsc.ops).index).unwrap())(&(*sum).rsc as *const rsc) as u32,
            );
        }
    }

    0
}

unsafe extern "C" fn amixer_commit_write(amixer: *mut amixer) -> i32 {
    let hw: *mut hw;
    let mut index: u32;
    let mut i: i32;
    let input: *mut rsc;
    let sum: *mut sum;

    unsafe {
        hw = (*amixer).rsc.hw;
        input = (*amixer).input;
        sum = (*amixer).sum;

        /* Program master and conjugate resources */
        ((*(*amixer).rsc.ops).master).unwrap()(&mut (*amixer).rsc as *mut rsc);
        if !input.is_null() {
            ((*(*input).ops).master).unwrap()(input);
        }

        if !sum.is_null() {
            ((*(*sum).rsc.ops).master).unwrap()(&mut (*sum).rsc as *mut rsc);
        }

        i = 0;
        while i < (*amixer).rsc.msr {
            ((*hw).amixer_set_dirty_all).unwrap()((*amixer).rsc.ctrl_blk);
            if !input.is_null() {
                ((*hw).amixer_set_x).unwrap()(
                    (*amixer).rsc.ctrl_blk,
                    ((*(*input).ops).output_slot).unwrap()(input as *const rsc) as u32,
                );
                ((*(*input).ops).next_conj).unwrap()(input);
            }
            if !sum.is_null() {
                ((*hw).amixer_set_sadr).unwrap()(
                    (*amixer).rsc.ctrl_blk,
                    ((*(*(*sum).rsc.ops).index).unwrap())(&(*sum).rsc as *const rsc) as u32,
                );
                ((*(*sum).rsc.ops).next_conj).unwrap()(&mut (*sum).rsc as *mut rsc);
            }
            index = ((*(*amixer).rsc.ops).output_slot).unwrap()(&(*amixer).rsc as *const rsc) as u32;
            ((*hw).amixer_commit_write).unwrap()(hw, index, (*amixer).rsc.ctrl_blk);
            ((*(*amixer).rsc.ops).next_conj).unwrap()(&mut (*amixer).rsc as *mut rsc);
            i += 1;
        }
        ((*(*amixer).rsc.ops).master).unwrap()(&mut (*amixer).rsc as *mut rsc);
        if !input.is_null() {
            ((*(*input).ops).master).unwrap()(input);
        }

        if !sum.is_null() {
            ((*(*sum).rsc.ops).master).unwrap()(&mut (*sum).rsc as *mut rsc);
        }
    }

    0
}

unsafe extern "C" fn amixer_commit_raw_write(amixer: *mut amixer) -> i32 {
    let hw: *mut hw;
    let index: u32;

    unsafe {
        hw = (*amixer).rsc.hw;
        index = ((*(*amixer).rsc.ops).output_slot).unwrap()(&(*amixer).rsc as *const rsc) as u32;
        ((*hw).amixer_commit_write).unwrap()(hw, index, (*amixer).rsc.ctrl_blk);
    }

    0
}

unsafe extern "C" fn amixer_get_y(amixer: *mut amixer) -> i32 {
    let hw: *mut hw;

    unsafe {
        hw = (*amixer).rsc.hw;
        ((*hw).amixer_get_y).unwrap()((*amixer).rsc.ctrl_blk)
    }
}

unsafe extern "C" fn amixer_setup(
    amixer: *mut amixer,
    input: *mut rsc,
    scale: u32,
    sum: *mut sum,
) -> i32 {
    unsafe {
        amixer_set_input(amixer, input);
        amixer_set_y(amixer, scale);
        amixer_set_sum(amixer, sum);
        amixer_commit_write(amixer);
    }
    0
}

static amixer_ops: amixer_rsc_ops = amixer_rsc_ops {
    set_input: Some(amixer_set_input),
    set_invalid_squash: Some(amixer_set_invalid_squash),
    set_scale: Some(amixer_set_y),
    set_sum: Some(amixer_set_sum),
    commit_write: Some(amixer_commit_write),
    commit_raw_write: Some(amixer_commit_raw_write),
    setup: Some(amixer_setup),
    get_scale: Some(amixer_get_y),
};

unsafe extern "C" fn amixer_rsc_init(
    amixer: *mut amixer,
    desc: *const amixer_desc,
    mgr: *mut amixer_mgr,
) -> i32 {
    let mut err: i32;

    unsafe {
        err = rsc_init(
            &mut (*amixer).rsc as *mut rsc,
            (*amixer).idx[0],
            AMIXER,
            (*desc).msr,
            (*mgr).mgr.hw,
        );
        if err != 0 {
            return err;
        }

        /* Set amixer specific operations */
        (*amixer).rsc.ops = &amixer_basic_rsc_ops as *const rsc_ops;
        (*amixer).rsc.conj = 0;
        (*amixer).ops = &amixer_ops as *const amixer_rsc_ops;
        (*amixer).input = ptr::null_mut();
        (*amixer).sum = ptr::null_mut();

        amixer_setup(amixer, ptr::null_mut(), 0, ptr::null_mut());
    }

    0
}

unsafe extern "C" fn amixer_rsc_uninit(amixer: *mut amixer) -> i32 {
    unsafe {
        amixer_setup(amixer, ptr::null_mut(), 0, ptr::null_mut());
        rsc_uninit(&mut (*amixer).rsc as *mut rsc);
        (*amixer).ops = ptr::null();
        (*amixer).input = ptr::null_mut();
        (*amixer).sum = ptr::null_mut();
    }
    0
}

unsafe extern "C" fn get_amixer_rsc(
    mgr: *mut amixer_mgr,
    desc: *const amixer_desc,
    ramixer: *mut *mut amixer,
) -> i32 {
    let mut err: i32;
    let mut i: i32;
    let mut idx: u32 = 0;
    let amixer: *mut amixer;

    unsafe {
        *ramixer = ptr::null_mut();

        /* Allocate mem for amixer resource */
        amixer = kzalloc(size_of::<amixer>(), GFP_KERNEL) as *mut amixer;
        if amixer.is_null() {
            return -ENOMEM;
        }

        /* Check whether there are sufficient
         * amixer resources to meet request. */
        err = 0;
        scoped_guard!(spinlock_irqsave, &mut (*mgr).mgr_lock, {
            i = 0;
            while i < (*desc).msr {
                err = mgr_get_resource(&mut (*mgr).mgr as *mut rsc_mgr, 1, &mut idx as *mut u32);
                if err != 0 {
                    break;
                }

                (*amixer).idx[i as usize] = idx;
                i += 1;
            }
        });
        if err != 0 {
            dev_err((*(*mgr).card).dev, c_str!("Can't meet AMIXER resource request!\n"));
            goto_error_amixer(mgr, amixer, i);
            return err;
        }

        err = amixer_rsc_init(amixer, desc, mgr);
        if err != 0 {
            goto_error_amixer(mgr, amixer, i);
            return err;
        }

        *ramixer = amixer;
    }

    0
}

unsafe fn goto_error_amixer(mgr: *mut amixer_mgr, amixer: *mut amixer, mut i: i32) {
    unsafe {
        scoped_guard!(spinlock_irqsave, &mut (*mgr).mgr_lock, {
            i -= 1;
            while i >= 0 {
                mgr_put_resource(&mut (*mgr).mgr as *mut rsc_mgr, 1, (*amixer).idx[i as usize]);
                i -= 1;
            }
        });

        kfree(amixer as *const core::ffi::c_void);
    }
}

unsafe extern "C" fn put_amixer_rsc(mgr: *mut amixer_mgr, amixer: *mut amixer) -> i32 {
    let mut i: i32;

    unsafe {
        scoped_guard!(spinlock_irqsave, &mut (*mgr).mgr_lock, {
            i = 0;
            while i < (*amixer).rsc.msr {
                mgr_put_resource(&mut (*mgr).mgr as *mut rsc_mgr, 1, (*amixer).idx[i as usize]);
                i += 1;
            }
        });
        amixer_rsc_uninit(amixer);
        kfree(amixer as *const core::ffi::c_void);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn amixer_mgr_create(hw: *mut hw, ramixer_mgr: *mut *mut core::ffi::c_void) -> i32 {
    let mut err: i32;
    let amixer_mgr: *mut amixer_mgr;

    unsafe {
        *ramixer_mgr = ptr::null_mut();
        amixer_mgr = kzalloc(size_of::<amixer_mgr>(), GFP_KERNEL) as *mut amixer_mgr;
        if amixer_mgr.is_null() {
            return -ENOMEM;
        }

        err = rsc_mgr_init(&mut (*amixer_mgr).mgr as *mut rsc_mgr, AMIXER, AMIXER_RESOURCE_NUM, hw);
        if err != 0 {
            kfree(amixer_mgr as *const core::ffi::c_void);
            return err;
        }

        spin_lock_init(&mut (*amixer_mgr).mgr_lock);

        (*amixer_mgr).get_amixer = Some(get_amixer_rsc);
        (*amixer_mgr).put_amixer = Some(put_amixer_rsc);
        (*amixer_mgr).card = (*hw).card;

        *ramixer_mgr = amixer_mgr as *mut core::ffi::c_void;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn amixer_mgr_destroy(ptr: *mut core::ffi::c_void) -> i32 {
    let amixer_mgr: *mut amixer_mgr = ptr as *mut amixer_mgr;
    unsafe {
        rsc_mgr_uninit(&mut (*amixer_mgr).mgr as *mut rsc_mgr);
        kfree(amixer_mgr as *const core::ffi::c_void);
    }
    0
}

/* SUM resource management */

unsafe extern "C" fn sum_master(rsc: *mut rsc) {
    unsafe {
        (*rsc).conj = 0;
        (*rsc).idx = (*container_of!(rsc, sum, rsc)).idx[0];
    }
}

unsafe extern "C" fn sum_next_conj(rsc: *mut rsc) {
    unsafe {
        (*rsc).conj += 1;
    }
}

unsafe extern "C" fn sum_index(rsc: *const rsc) -> i32 {
    unsafe { (*container_of!(rsc, sum, rsc)).idx[(*rsc).conj as usize] as i32 }
}

unsafe extern "C" fn sum_output_slot(rsc: *const rsc) -> i32 {
    unsafe { (sum_index(rsc) << 4) + 0xc }
}

static sum_basic_rsc_ops: rsc_ops = rsc_ops {
    master: Some(sum_master),
    next_conj: Some(sum_next_conj),
    index: Some(sum_index),
    output_slot: Some(sum_output_slot),
};

unsafe extern "C" fn sum_rsc_init(sum: *mut sum, desc: *const sum_desc, mgr: *mut sum_mgr) -> i32 {
    let mut err: i32;

    unsafe {
        err = rsc_init(&mut (*sum).rsc as *mut rsc, (*sum).idx[0], SUM, (*desc).msr, (*mgr).mgr.hw);
        if err != 0 {
            return err;
        }

        (*sum).rsc.ops = &sum_basic_rsc_ops as *const rsc_ops;
        (*sum).rsc.conj = 0;
    }

    0
}

unsafe extern "C" fn sum_rsc_uninit(sum: *mut sum) -> i32 {
    unsafe {
        rsc_uninit(&mut (*sum).rsc as *mut rsc);
    }
    0
}

unsafe extern "C" fn get_sum_rsc(
    mgr: *mut sum_mgr,
    desc: *const sum_desc,
    rsum: *mut *mut sum,
) -> i32 {
    let mut err: i32;
    let mut i: i32;
    let mut idx: u32 = 0;
    let sum: *mut sum;

    unsafe {
        *rsum = ptr::null_mut();

        /* Allocate mem for sum resource */
        sum = kzalloc(size_of::<sum>(), GFP_KERNEL) as *mut sum;
        if sum.is_null() {
            return -ENOMEM;
        }

        /* Check whether there are sufficient sum resources to meet request. */
        err = 0;
        scoped_guard!(spinlock_irqsave, &mut (*mgr).mgr_lock, {
            i = 0;
            while i < (*desc).msr {
                err = mgr_get_resource(&mut (*mgr).mgr as *mut rsc_mgr, 1, &mut idx as *mut u32);
                if err != 0 {
                    break;
                }

                (*sum).idx[i as usize] = idx;
                i += 1;
            }
        });
        if err != 0 {
            dev_err((*(*mgr).card).dev, c_str!("Can't meet SUM resource request!\n"));
            goto_error_sum(mgr, sum, i);
            return err;
        }

        err = sum_rsc_init(sum, desc, mgr);
        if err != 0 {
            goto_error_sum(mgr, sum, i);
            return err;
        }

        *rsum = sum;
    }

    0
}

unsafe fn goto_error_sum(mgr: *mut sum_mgr, sum: *mut sum, mut i: i32) {
    unsafe {
        scoped_guard!(spinlock_irqsave, &mut (*mgr).mgr_lock, {
            i -= 1;
            while i >= 0 {
                mgr_put_resource(&mut (*mgr).mgr as *mut rsc_mgr, 1, (*sum).idx[i as usize]);
                i -= 1;
            }
        });
        kfree(sum as *const core::ffi::c_void);
    }
}

unsafe extern "C" fn put_sum_rsc(mgr: *mut sum_mgr, sum: *mut sum) -> i32 {
    let mut i: i32;

    unsafe {
        scoped_guard!(spinlock_irqsave, &mut (*mgr).mgr_lock, {
            i = 0;
            while i < (*sum).rsc.msr {
                mgr_put_resource(&mut (*mgr).mgr as *mut rsc_mgr, 1, (*sum).idx[i as usize]);
                i += 1;
            }
        });
        sum_rsc_uninit(sum);
        kfree(sum as *const core::ffi::c_void);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn sum_mgr_create(hw: *mut hw, rsum_mgr: *mut *mut core::ffi::c_void) -> i32 {
    let mut err: i32;
    let sum_mgr: *mut sum_mgr;

    unsafe {
        *rsum_mgr = ptr::null_mut();
        sum_mgr = kzalloc(size_of::<sum_mgr>(), GFP_KERNEL) as *mut sum_mgr;
        if sum_mgr.is_null() {
            return -ENOMEM;
        }

        err = rsc_mgr_init(&mut (*sum_mgr).mgr as *mut rsc_mgr, SUM, SUM_RESOURCE_NUM, hw);
        if err != 0 {
            kfree(sum_mgr as *const core::ffi::c_void);
            return err;
        }

        spin_lock_init(&mut (*sum_mgr).mgr_lock);

        (*sum_mgr).get_sum = Some(get_sum_rsc);
        (*sum_mgr).put_sum = Some(put_sum_rsc);
        (*sum_mgr).card = (*hw).card;

        *rsum_mgr = sum_mgr as *mut core::ffi::c_void;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn sum_mgr_destroy(ptr: *mut core::ffi::c_void) -> i32 {
    let sum_mgr: *mut sum_mgr = ptr as *mut sum_mgr;
    unsafe {
        rsc_mgr_uninit(&mut (*sum_mgr).mgr as *mut rsc_mgr);
        kfree(sum_mgr as *const core::ffi::c_void);
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
