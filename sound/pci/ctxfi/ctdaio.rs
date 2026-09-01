// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	ctdaio.c
 *
 * @Brief
 * This file contains the implementation of Digital Audio Input Output
 * resource management object.
 *
 * @Author	Liu Chun
 * @Date 	May 23 2008
 */

use core::ffi::c_void;
use core::mem::zeroed;
use core::ptr::{addr_of_mut, null_mut};

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const ENOENT: i32 = 2;
const GFP_KERNEL: u32 = 0;

const ATC20K1: i32 = 0;
const ATC20K2: i32 = 1;
const DAIO: i32 = 0;
const NUM_DAIOTYP: usize = 9;

const LINEO1: DAIOTYP = 0;
const LINEO2: DAIOTYP = 1;
const LINEO3: DAIOTYP = 2;
const LINEO4: DAIOTYP = 3;
const LINEIM: DAIOTYP = 4;
const SPDIFOO: DAIOTYP = 5;
const SPDIFIO: DAIOTYP = 6;
const SPDIFI_BAY: DAIOTYP = 7;
const MIC: DAIOTYP = 8;
const RCA: DAIOTYP = 8;

type DAIOTYP = usize;

#[repr(C)]
struct daio_usage {
    data: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct daio_rsc_idx {
    left: u16,
    right: u16,
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct rsc {
    idx: i32,
    conj: i32,
    msr: u32,
    ops: *const rsc_ops,
}

#[repr(C)]
pub struct rsc_ops {
    master: Option<unsafe extern "C" fn(*mut rsc)>,
    next_conj: Option<unsafe extern "C" fn(*mut rsc)>,
    index: Option<unsafe extern "C" fn(*const rsc) -> i32>,
    output_slot: Option<unsafe extern "C" fn(*const rsc) -> i32>,
}

#[repr(C)]
pub struct rsc_mgr {
    hw: *mut hw,
    ctrl_blk: *mut c_void,
    rscs: *mut c_void,
}

#[repr(C)]
pub struct imapper {
    list: list_head,
    slot: i32,
    addr: i32,
    next: i32,
    user: i32,
}

#[repr(C)]
pub struct daio {
    rscl: rsc,
    rscr: rsc,
    typ: DAIOTYP,
    output: u32,
}

#[repr(C)]
pub struct daio_desc {
    typ: DAIOTYP,
    msr: u32,
    passthru: u32,
    output: u32,
}

#[repr(C)]
pub struct dao_desc {
    msr: u32,
    passthru: u32,
}

#[repr(C)]
pub struct dao_rsc_ops {
    set_spos: Option<unsafe extern "C" fn(*mut dao, u32) -> i32>,
    commit_write: Option<unsafe extern "C" fn(*mut dao) -> i32>,
    get_spos: Option<unsafe extern "C" fn(*mut dao, *mut u32) -> i32>,
    reinit: Option<unsafe extern "C" fn(*mut dao, *const dao_desc) -> i32>,
    set_left_input: Option<unsafe extern "C" fn(*mut dao, *mut rsc) -> i32>,
    set_right_input: Option<unsafe extern "C" fn(*mut dao, *mut rsc) -> i32>,
    clear_left_input: Option<unsafe extern "C" fn(*mut dao) -> i32>,
    clear_right_input: Option<unsafe extern "C" fn(*mut dao) -> i32>,
}

#[repr(C)]
pub struct dai_rsc_ops {
    set_srt_srcl: Option<unsafe extern "C" fn(*mut dai, *mut rsc) -> i32>,
    set_srt_srcr: Option<unsafe extern "C" fn(*mut dai, *mut rsc) -> i32>,
    set_srt_msr: Option<unsafe extern "C" fn(*mut dai, u32) -> i32>,
    set_enb_src: Option<unsafe extern "C" fn(*mut dai, u32) -> i32>,
    set_enb_srt: Option<unsafe extern "C" fn(*mut dai, u32) -> i32>,
    commit_write: Option<unsafe extern "C" fn(*mut dai) -> i32>,
}

#[repr(C)]
pub struct dao {
    daio: daio,
    ops: *const dao_rsc_ops,
    mgr: *mut daio_mgr,
    hw: *mut hw,
    ctrl_blk: *mut c_void,
    imappers: *mut *mut imapper,
}

#[repr(C)]
pub struct dai {
    daio: daio,
    ops: *const dai_rsc_ops,
    hw: *mut hw,
    ctrl_blk: *mut c_void,
}

#[repr(C)]
pub struct card {
    dev: *mut c_void,
}

#[repr(C)]
pub struct hw {
    chip_type: i32,
    card: *mut card,
    dao_get_spos: unsafe extern "C" fn(*mut c_void, *mut u32),
    dao_set_spos: unsafe extern "C" fn(*mut c_void, u32),
    dao_commit_write: unsafe extern "C" fn(*mut hw, i32, *mut c_void),
    dao_get_ctrl_blk: unsafe extern "C" fn(*mut *mut c_void) -> i32,
    dao_put_ctrl_blk: unsafe extern "C" fn(*mut c_void),
    dai_srt_set_srcm: unsafe extern "C" fn(*mut c_void, i32),
    dai_srt_set_srco: unsafe extern "C" fn(*mut c_void, i32),
    dai_srt_set_rsr: unsafe extern "C" fn(*mut c_void, u32),
    dai_srt_set_ec: unsafe extern "C" fn(*mut c_void, u32),
    dai_srt_set_et: unsafe extern "C" fn(*mut c_void, u32),
    dai_commit_write: unsafe extern "C" fn(*mut hw, i32, *mut c_void),
    dai_get_ctrl_blk: unsafe extern "C" fn(*mut *mut c_void) -> i32,
    dai_put_ctrl_blk: unsafe extern "C" fn(*mut c_void),
    dai_srt_set_drat: unsafe extern "C" fn(*mut c_void, u32),
    daio_mgr_dsb_dao: unsafe extern "C" fn(*mut c_void, i32),
    daio_mgr_commit_write: unsafe extern "C" fn(*mut hw, *mut c_void),
    daio_mgr_dao_init: unsafe extern "C" fn(*mut hw, *mut c_void, i32, u32),
    daio_mgr_enb_dao: unsafe extern "C" fn(*mut c_void, i32),
    daio_mgr_dsb_dai: unsafe extern "C" fn(*mut c_void, i32),
    daio_mgr_enb_dai: unsafe extern "C" fn(*mut c_void, i32),
    daio_mgr_set_imaparc: unsafe extern "C" fn(*mut c_void, i32),
    daio_mgr_set_imapnxt: unsafe extern "C" fn(*mut c_void, i32),
    daio_mgr_set_imapaddr: unsafe extern "C" fn(*mut c_void, i32),
}

#[repr(C)]
pub struct daio_mgr {
    mgr: rsc_mgr,
    mgr_lock: spinlock_t,
    imap_lock: spinlock_t,
    imappers: list_head,
    init_imap: *mut imapper,
    init_imap_added: i32,
    get_daio: Option<unsafe extern "C" fn(*mut daio_mgr, *const daio_desc, *mut *mut daio) -> i32>,
    put_daio: Option<unsafe extern "C" fn(*mut daio_mgr, *mut daio) -> i32>,
    daio_enable: Option<unsafe extern "C" fn(*mut daio_mgr, *mut daio) -> i32>,
    daio_disable: Option<unsafe extern "C" fn(*mut daio_mgr, *mut daio) -> i32>,
    imap_add: Option<unsafe extern "C" fn(*mut daio_mgr, *mut imapper) -> i32>,
    imap_delete: Option<unsafe extern "C" fn(*mut daio_mgr, *mut imapper) -> i32>,
    commit_write: Option<unsafe extern "C" fn(*mut daio_mgr) -> i32>,
    card: *mut card,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn rsc_init(rsc: *mut rsc, idx: u32, typ: i32, msr: u32, hw: *mut hw) -> i32;
    fn rsc_uninit(rsc: *mut rsc) -> i32;
    fn rsc_mgr_init(mgr: *mut rsc_mgr, typ: i32, amount: usize, hw: *mut hw) -> i32;
    fn rsc_mgr_uninit(mgr: *mut rsc_mgr) -> i32;
    fn input_mapper_add(list: *mut list_head, entry: *mut imapper,
                        op: unsafe extern "C" fn(*mut c_void, *mut imapper) -> i32,
                        data: *mut c_void) -> i32;
    fn input_mapper_delete(list: *mut list_head, entry: *mut imapper,
                           op: unsafe extern "C" fn(*mut c_void, *mut imapper) -> i32,
                           data: *mut c_void) -> i32;
    fn free_input_mapper_list(list: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_empty(head: *const list_head) -> i32;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
}

const IDX_ZERO: daio_rsc_idx = daio_rsc_idx { left: 0, right: 0 };

static idx_20k1: [daio_rsc_idx; NUM_DAIOTYP] = {
    let mut a = [IDX_ZERO; NUM_DAIOTYP];
    a[LINEO1] = daio_rsc_idx { left: 0x00, right: 0x01 };
    a[LINEO2] = daio_rsc_idx { left: 0x18, right: 0x19 };
    a[LINEO3] = daio_rsc_idx { left: 0x08, right: 0x09 };
    a[LINEO4] = daio_rsc_idx { left: 0x10, right: 0x11 };
    a[LINEIM] = daio_rsc_idx { left: 0x1b5, right: 0x1bd };
    a[SPDIFOO] = daio_rsc_idx { left: 0x20, right: 0x21 };
    a[SPDIFIO] = daio_rsc_idx { left: 0x15, right: 0x1d };
    a[SPDIFI_BAY] = daio_rsc_idx { left: 0x95, right: 0x9d };
    a
};

static idx_20k2: [daio_rsc_idx; NUM_DAIOTYP] = {
    let mut a = [IDX_ZERO; NUM_DAIOTYP];
    a[LINEO1] = daio_rsc_idx { left: 0x40, right: 0x41 };
    a[LINEO2] = daio_rsc_idx { left: 0x60, right: 0x61 };
    a[LINEO3] = daio_rsc_idx { left: 0x50, right: 0x51 };
    a[LINEO4] = daio_rsc_idx { left: 0x70, right: 0x71 };
    a[LINEIM] = daio_rsc_idx { left: 0x45, right: 0xc5 };
    a[MIC] = daio_rsc_idx { left: 0x55, right: 0xd5 };
    a[RCA] = daio_rsc_idx { left: 0x30, right: 0x31 };
    a[SPDIFOO] = daio_rsc_idx { left: 0x00, right: 0x01 };
    a[SPDIFIO] = daio_rsc_idx { left: 0x05, right: 0x85 };
    a
};

unsafe extern "C" fn daio_master(rsc: *mut rsc) {
    /* Actually, this is not the resource index of DAIO.
     * For DAO, it is the input mapper index. And, for DAI,
     * it is the output time-slot index. */
    unsafe { (*rsc).conj = (*rsc).idx };
}

unsafe extern "C" fn daio_index(rsc: *const rsc) -> i32 {
    unsafe { (*rsc).conj }
}

unsafe extern "C" fn daio_out_next_conj(rsc: *mut rsc) {
    unsafe { (*rsc).conj += 2 };
}

unsafe extern "C" fn daio_in_next_conj_20k1(rsc: *mut rsc) {
    unsafe { (*rsc).conj += 0x200 };
}

unsafe extern "C" fn daio_in_next_conj_20k2(rsc: *mut rsc) {
    unsafe { (*rsc).conj += 0x100 };
}

static daio_out_rsc_ops: rsc_ops = rsc_ops {
    master: Some(daio_master),
    next_conj: Some(daio_out_next_conj),
    index: Some(daio_index),
    output_slot: None,
};

static daio_in_rsc_ops_20k1: rsc_ops = rsc_ops {
    master: Some(daio_master),
    next_conj: Some(daio_in_next_conj_20k1),
    index: None,
    output_slot: Some(daio_index),
};

static daio_in_rsc_ops_20k2: rsc_ops = rsc_ops {
    master: Some(daio_master),
    next_conj: Some(daio_in_next_conj_20k2),
    index: None,
    output_slot: Some(daio_index),
};

unsafe fn daio_device_index(typ: DAIOTYP, hw: *mut hw) -> i32 {
    unsafe {
        match (*hw).chip_type {
            ATC20K1 => match typ {
                SPDIFOO => 0,
                SPDIFIO => 0,
                SPDIFI_BAY => 1,
                LINEO1 => 4,
                LINEO2 => 7,
                LINEO3 => 5,
                LINEO4 => 6,
                LINEIM => 7,
                _ => -EINVAL,
            },
            ATC20K2 => match typ {
                SPDIFOO => 0,
                SPDIFIO => 0,
                LINEO1 => 4,
                LINEO2 => 7,
                LINEO3 => 5,
                LINEO4 => 6,
                LINEIM => 4,
                MIC => 5,
                RCA => 3,
                _ => -EINVAL,
            },
            _ => -EINVAL,
        }
    }
}

unsafe extern "C" fn dao_spdif_get_spos(dao: *mut dao, spos: *mut u32) -> i32 {
    unsafe { ((*(*dao).hw).dao_get_spos)((*dao).ctrl_blk, spos) };
    0
}

unsafe extern "C" fn dao_spdif_set_spos(dao: *mut dao, spos: u32) -> i32 {
    unsafe { ((*(*dao).hw).dao_set_spos)((*dao).ctrl_blk, spos) };
    0
}

unsafe extern "C" fn dao_commit_write(dao: *mut dao) -> i32 {
    let idx = unsafe { daio_device_index((*dao).daio.typ, (*dao).hw) };
    if idx < 0 {
        return idx;
    }
    unsafe { ((*(*dao).hw).dao_commit_write)((*dao).hw, idx, (*dao).ctrl_blk) };
    0
}

unsafe extern "C" fn dao_set_left_input(dao: *mut dao, input: *mut rsc) -> i32 {
    unsafe {
        let daio = addr_of_mut!((*dao).daio);
        let mut entry = kzalloc(core::mem::size_of::<imapper>() * (*daio).rscl.msr as usize, GFP_KERNEL) as *mut imapper;
        if entry.is_null() {
            return -ENOMEM;
        }

        ((*(*dao).ops).clear_left_input.unwrap())(dao);
        /* Program master and conjugate resources */
        ((*(*input).ops).master.unwrap())(input);
        ((*(*daio).rscl.ops).master.unwrap())(addr_of_mut!((*daio).rscl));
        for i in 0..(*daio).rscl.msr as isize {
            (*entry).slot = ((*(*input).ops).output_slot.unwrap())(input);
            (*entry).addr = ((*(*daio).rscl.ops).index.unwrap())(addr_of_mut!((*daio).rscl));
            (*entry).user = (*entry).addr;
            ((*(*dao).mgr).imap_add.unwrap())((*dao).mgr, entry);
            *(*dao).imappers.offset(i) = entry;

            ((*(*input).ops).next_conj.unwrap())(input);
            ((*(*daio).rscl.ops).next_conj.unwrap())(addr_of_mut!((*daio).rscl));
            entry = entry.offset(1);
        }
        ((*(*input).ops).master.unwrap())(input);
        ((*(*daio).rscl.ops).master.unwrap())(addr_of_mut!((*daio).rscl));
    }

    0
}

unsafe extern "C" fn dao_set_right_input(dao: *mut dao, input: *mut rsc) -> i32 {
    unsafe {
        let daio = addr_of_mut!((*dao).daio);
        let mut entry = kzalloc(core::mem::size_of::<imapper>() * (*daio).rscr.msr as usize, GFP_KERNEL) as *mut imapper;
        if entry.is_null() {
            return -ENOMEM;
        }

        ((*(*dao).ops).clear_right_input.unwrap())(dao);
        /* Program master and conjugate resources */
        ((*(*input).ops).master.unwrap())(input);
        ((*(*daio).rscr.ops).master.unwrap())(addr_of_mut!((*daio).rscr));
        for i in 0..(*daio).rscr.msr as isize {
            (*entry).slot = ((*(*input).ops).output_slot.unwrap())(input);
            (*entry).addr = ((*(*daio).rscr.ops).index.unwrap())(addr_of_mut!((*daio).rscr));
            (*entry).user = (*entry).addr;
            ((*(*dao).mgr).imap_add.unwrap())((*dao).mgr, entry);
            *(*dao).imappers.offset((*daio).rscl.msr as isize + i) = entry;

            ((*(*input).ops).next_conj.unwrap())(input);
            ((*(*daio).rscr.ops).next_conj.unwrap())(addr_of_mut!((*daio).rscr));
            entry = entry.offset(1);
        }
        ((*(*input).ops).master.unwrap())(input);
        ((*(*daio).rscr.ops).master.unwrap())(addr_of_mut!((*daio).rscr));
    }

    0
}

unsafe fn dao_clear_input(dao: *mut dao, start: u32, end: u32) -> i32 {
    unsafe {
        if (*(*dao).imappers.add(start as usize)).is_null() {
            return 0;
        }
        for i in start..end {
            ((*(*dao).mgr).imap_delete.unwrap())((*dao).mgr, *(*dao).imappers.add(i as usize));
            *(*dao).imappers.add(i as usize) = null_mut();
        }
    }

    0
}

unsafe extern "C" fn dao_clear_left_input(dao: *mut dao) -> i32 {
    unsafe { dao_clear_input(dao, 0, (*dao).daio.rscl.msr) }
}

unsafe extern "C" fn dao_clear_right_input(dao: *mut dao) -> i32 {
    unsafe { dao_clear_input(dao, (*dao).daio.rscl.msr, (*dao).daio.rscl.msr + (*dao).daio.rscr.msr) }
}

static dao_ops: dao_rsc_ops = dao_rsc_ops {
    set_spos: Some(dao_spdif_set_spos),
    commit_write: Some(dao_commit_write),
    get_spos: Some(dao_spdif_get_spos),
    reinit: Some(dao_rsc_reinit),
    set_left_input: Some(dao_set_left_input),
    set_right_input: Some(dao_set_right_input),
    clear_left_input: Some(dao_clear_left_input),
    clear_right_input: Some(dao_clear_right_input),
};

unsafe extern "C" fn dai_set_srt_srcl(dai: *mut dai, src: *mut rsc) -> i32 {
    unsafe {
        ((*(*src).ops).master.unwrap())(src);
        ((*(*dai).hw).dai_srt_set_srcm)((*dai).ctrl_blk, ((*(*src).ops).index.unwrap())(src));
    }
    0
}

unsafe extern "C" fn dai_set_srt_srcr(dai: *mut dai, src: *mut rsc) -> i32 {
    unsafe {
        ((*(*src).ops).master.unwrap())(src);
        ((*(*dai).hw).dai_srt_set_srco)((*dai).ctrl_blk, ((*(*src).ops).index.unwrap())(src));
    }
    0
}

unsafe extern "C" fn dai_set_srt_msr(dai: *mut dai, mut msr: u32) -> i32 {
    let mut rsr = 0;
    while msr > 1 {
        msr >>= 1;
        rsr += 1;
    }

    unsafe { ((*(*dai).hw).dai_srt_set_rsr)((*dai).ctrl_blk, rsr) };
    0
}

unsafe extern "C" fn dai_set_enb_src(dai: *mut dai, enb: u32) -> i32 {
    unsafe { ((*(*dai).hw).dai_srt_set_ec)((*dai).ctrl_blk, enb) };
    0
}

unsafe extern "C" fn dai_set_enb_srt(dai: *mut dai, enb: u32) -> i32 {
    unsafe { ((*(*dai).hw).dai_srt_set_et)((*dai).ctrl_blk, enb) };
    0
}

unsafe extern "C" fn dai_commit_write(dai: *mut dai) -> i32 {
    let idx = unsafe { daio_device_index((*dai).daio.typ, (*dai).hw) };
    if idx < 0 {
        return idx;
    }
    unsafe { ((*(*dai).hw).dai_commit_write)((*dai).hw, idx, (*dai).ctrl_blk) };
    0
}

static dai_ops: dai_rsc_ops = dai_rsc_ops {
    set_srt_srcl: Some(dai_set_srt_srcl),
    set_srt_srcr: Some(dai_set_srt_srcr),
    set_srt_msr: Some(dai_set_srt_msr),
    set_enb_src: Some(dai_set_enb_src),
    set_enb_srt: Some(dai_set_enb_srt),
    commit_write: Some(dai_commit_write),
};

unsafe fn daio_rsc_init(daio: *mut daio, desc: *const daio_desc, hw: *mut hw) -> i32 {
    let (idx_l, idx_r) = unsafe {
        match (*hw).chip_type {
            ATC20K1 => (idx_20k1[(*desc).typ].left as u32, idx_20k1[(*desc).typ].right as u32),
            ATC20K2 => (idx_20k2[(*desc).typ].left as u32, idx_20k2[(*desc).typ].right as u32),
            _ => return -EINVAL,
        }
    };
    let mut err = unsafe { rsc_init(addr_of_mut!((*daio).rscl), idx_l, DAIO, (*desc).msr, hw) };
    if err != 0 {
        return err;
    }

    err = unsafe { rsc_init(addr_of_mut!((*daio).rscr), idx_r, DAIO, (*desc).msr, hw) };
    if err != 0 {
        unsafe { rsc_uninit(addr_of_mut!((*daio).rscl)) };
        return err;
    }

    /* Set daio->rscl/r->ops to daio specific ones */
    unsafe {
        if (*desc).output != 0 {
            (*daio).rscl.ops = &daio_out_rsc_ops;
            (*daio).rscr.ops = &daio_out_rsc_ops;
        } else {
            match (*hw).chip_type {
                ATC20K1 => {
                    (*daio).rscl.ops = &daio_in_rsc_ops_20k1;
                    (*daio).rscr.ops = &daio_in_rsc_ops_20k1;
                }
                ATC20K2 => {
                    (*daio).rscl.ops = &daio_in_rsc_ops_20k2;
                    (*daio).rscr.ops = &daio_in_rsc_ops_20k2;
                }
                _ => {}
            }
        }
        (*daio).typ = (*desc).typ;
        (*daio).output = (*desc).output;
    }

    0
}

unsafe fn daio_rsc_uninit(daio: *mut daio) -> i32 {
    unsafe {
        rsc_uninit(addr_of_mut!((*daio).rscl));
        rsc_uninit(addr_of_mut!((*daio).rscr));
    }

    0
}

unsafe fn dao_rsc_init(dao: *mut dao, desc: *const daio_desc, mgr: *mut daio_mgr) -> i32 {
    unsafe {
        let hw = (*mgr).mgr.hw;
        let mut err = daio_rsc_init(addr_of_mut!((*dao).daio), desc, (*mgr).mgr.hw);
        if err != 0 {
            return err;
        }

        (*dao).imappers = kzalloc(core::mem::size_of::<*mut c_void>() * (*desc).msr as usize * 2, GFP_KERNEL) as *mut *mut imapper;
        if (*dao).imappers.is_null() {
            err = -ENOMEM;
            daio_rsc_uninit(addr_of_mut!((*dao).daio));
            return err;
        }
        (*dao).ops = &dao_ops;
        (*dao).mgr = mgr;
        (*dao).hw = hw;
        err = ((*hw).dao_get_ctrl_blk)(addr_of_mut!((*dao).ctrl_blk));
        if err != 0 {
            kfree((*dao).imappers as *mut c_void);
            (*dao).imappers = null_mut();
            daio_rsc_uninit(addr_of_mut!((*dao).daio));
            return err;
        }

        let idx = daio_device_index((*dao).daio.typ, hw);
        if idx < 0 {
            err = idx;
            kfree((*dao).imappers as *mut c_void);
            (*dao).imappers = null_mut();
            daio_rsc_uninit(addr_of_mut!((*dao).daio));
            return err;
        }

        ((*hw).daio_mgr_dsb_dao)((*mgr).mgr.ctrl_blk, idx);
        ((*hw).daio_mgr_commit_write)(hw, (*mgr).mgr.ctrl_blk);

        let conf = ((*desc).msr & 0x7) | ((*desc).passthru << 3);
        ((*hw).daio_mgr_dao_init)(hw, (*mgr).mgr.ctrl_blk, idx, conf);
        ((*hw).daio_mgr_enb_dao)((*mgr).mgr.ctrl_blk, idx);
        ((*hw).daio_mgr_commit_write)(hw, (*mgr).mgr.ctrl_blk);
    }

    0
}

unsafe fn dao_rsc_uninit(dao: *mut dao) -> i32 {
    unsafe {
        if !(*dao).imappers.is_null() {
            if !(*(*dao).imappers).is_null() {
                dao_clear_left_input(dao);
            }

            if !(*(*dao).imappers.add((*dao).daio.rscl.msr as usize)).is_null() {
                dao_clear_right_input(dao);
            }

            kfree((*dao).imappers as *mut c_void);
            (*dao).imappers = null_mut();
        }
        ((*(*dao).hw).dao_put_ctrl_blk)((*dao).ctrl_blk);
        (*dao).hw = null_mut();
        (*dao).ctrl_blk = null_mut();
        daio_rsc_uninit(addr_of_mut!((*dao).daio));
    }

    0
}

unsafe extern "C" fn dao_rsc_reinit(dao: *mut dao, desc: *const dao_desc) -> i32 {
    unsafe {
        let mgr = (*dao).mgr;
        let mut dsc: daio_desc = zeroed();

        dsc.typ = (*dao).daio.typ;
        dsc.msr = (*desc).msr;
        dsc.passthru = (*desc).passthru;
        dsc.output = (*dao).daio.output;
        dao_rsc_uninit(dao);
        dao_rsc_init(dao, &dsc, mgr)
    }
}

unsafe fn dai_rsc_init(dai: *mut dai, desc: *const daio_desc, mgr: *mut daio_mgr) -> i32 {
    unsafe {
        let hw = (*mgr).mgr.hw;
        let mut err = daio_rsc_init(addr_of_mut!((*dai).daio), desc, (*mgr).mgr.hw);
        if err != 0 {
            return err;
        }

        (*dai).ops = &dai_ops;
        (*dai).hw = (*mgr).mgr.hw;
        err = ((*hw).dai_get_ctrl_blk)(addr_of_mut!((*dai).ctrl_blk));
        if err != 0 {
            daio_rsc_uninit(addr_of_mut!((*dai).daio));
            return err;
        }

        let idx = daio_device_index((*dai).daio.typ, (*dai).hw);
        if idx < 0 {
            err = idx;
            daio_rsc_uninit(addr_of_mut!((*dai).daio));
            return err;
        }

        let mut rsr = 0;
        let mut msr = (*desc).msr;
        while msr > 1 {
            msr >>= 1;
            rsr += 1;
        }

        ((*hw).dai_srt_set_rsr)((*dai).ctrl_blk, rsr);
        ((*hw).dai_srt_set_drat)((*dai).ctrl_blk, 0);
        /* default to disabling control of a SRC */
        ((*hw).dai_srt_set_ec)((*dai).ctrl_blk, 0);
        ((*hw).dai_srt_set_et)((*dai).ctrl_blk, 0); /* default to disabling SRT */
        ((*hw).dai_commit_write)(hw, idx, (*dai).ctrl_blk);
    }

    0
}

unsafe fn dai_rsc_uninit(dai: *mut dai) -> i32 {
    unsafe {
        ((*(*dai).hw).dai_put_ctrl_blk)((*dai).ctrl_blk);
        (*dai).hw = null_mut();
        (*dai).ctrl_blk = null_mut();
        daio_rsc_uninit(addr_of_mut!((*dai).daio));
    }
    0
}

unsafe fn daio_mgr_get_rsc(mgr: *mut rsc_mgr, typ: DAIOTYP) -> i32 {
    unsafe {
        if ((*(*mgr).rscs.cast::<daio_usage>()).data & ((0x1u16) << typ)) != 0 {
            return -ENOENT;
        }

        (*(*mgr).rscs.cast::<daio_usage>()).data |= (0x1u16) << typ;
    }

    0
}

unsafe fn daio_mgr_put_rsc(mgr: *mut rsc_mgr, typ: DAIOTYP) -> i32 {
    unsafe {
        (*(*mgr).rscs.cast::<daio_usage>()).data &= !((0x1u16) << typ);
    }

    0
}

unsafe extern "C" fn get_daio_rsc(mgr: *mut daio_mgr, desc: *const daio_desc, rdaio: *mut *mut daio) -> i32 {
    unsafe {
        *rdaio = null_mut();

        /* Check whether there are sufficient daio resources to meet request. */
        let mut flags = 0usize;
        spin_lock_irqsave(addr_of_mut!((*mgr).mgr_lock), &mut flags);
        let mut err = daio_mgr_get_rsc(addr_of_mut!((*mgr).mgr), (*desc).typ);
        spin_unlock_irqrestore(addr_of_mut!((*mgr).mgr_lock), flags);
        if err != 0 {
            return err;
        }

        err = -ENOMEM;
        /* Allocate mem for daio resource */
        if (*desc).output != 0 {
            let dao = kzalloc(core::mem::size_of::<dao>(), GFP_KERNEL) as *mut dao;
            if dao.is_null() {
                goto_error(mgr, desc);
                return err;
            }

            err = dao_rsc_init(dao, desc, mgr);
            if err != 0 {
                kfree(dao as *mut c_void);
                goto_error(mgr, desc);
                return err;
            }

            *rdaio = addr_of_mut!((*dao).daio);
        } else {
            let dai = kzalloc(core::mem::size_of::<dai>(), GFP_KERNEL) as *mut dai;
            if dai.is_null() {
                goto_error(mgr, desc);
                return err;
            }

            err = dai_rsc_init(dai, desc, mgr);
            if err != 0 {
                kfree(dai as *mut c_void);
                goto_error(mgr, desc);
                return err;
            }

            *rdaio = addr_of_mut!((*dai).daio);
        }

        ((*mgr).daio_enable.unwrap())(mgr, *rdaio);
        ((*mgr).commit_write.unwrap())(mgr);
    }

    0
}

unsafe fn goto_error(mgr: *mut daio_mgr, desc: *const daio_desc) {
    unsafe {
        let mut flags = 0usize;
        spin_lock_irqsave(addr_of_mut!((*mgr).mgr_lock), &mut flags);
        daio_mgr_put_rsc(addr_of_mut!((*mgr).mgr), (*desc).typ);
        spin_unlock_irqrestore(addr_of_mut!((*mgr).mgr_lock), flags);
    }
}

unsafe extern "C" fn put_daio_rsc(mgr: *mut daio_mgr, daio: *mut daio) -> i32 {
    unsafe {
        ((*mgr).daio_disable.unwrap())(mgr, daio);
        ((*mgr).commit_write.unwrap())(mgr);

        let mut flags = 0usize;
        spin_lock_irqsave(addr_of_mut!((*mgr).mgr_lock), &mut flags);
        daio_mgr_put_rsc(addr_of_mut!((*mgr).mgr), (*daio).typ);
        spin_unlock_irqrestore(addr_of_mut!((*mgr).mgr_lock), flags);

        if (*daio).output != 0 {
            let dao = daio as *mut dao;
            dao_rsc_uninit(dao);
            kfree(dao as *mut c_void);
        } else {
            let dai = daio as *mut dai;
            dai_rsc_uninit(dai);
            kfree(dai as *mut c_void);
        }
    }

    0
}

unsafe extern "C" fn daio_mgr_enb_daio(mgr: *mut daio_mgr, daio: *mut daio) -> i32 {
    unsafe {
        let hw = (*mgr).mgr.hw;
        let idx = daio_device_index((*daio).typ, hw);

        if idx < 0 {
            return idx;
        }
        if (*daio).output != 0 {
            ((*hw).daio_mgr_enb_dao)((*mgr).mgr.ctrl_blk, idx);
        } else {
            ((*hw).daio_mgr_enb_dai)((*mgr).mgr.ctrl_blk, idx);
        }
    }
    0
}

unsafe extern "C" fn daio_mgr_dsb_daio(mgr: *mut daio_mgr, daio: *mut daio) -> i32 {
    unsafe {
        let hw = (*mgr).mgr.hw;
        let idx = daio_device_index((*daio).typ, hw);

        if idx < 0 {
            return idx;
        }
        if (*daio).output != 0 {
            ((*hw).daio_mgr_dsb_dao)((*mgr).mgr.ctrl_blk, idx);
        } else {
            ((*hw).daio_mgr_dsb_dai)((*mgr).mgr.ctrl_blk, idx);
        }
    }
    0
}

unsafe extern "C" fn daio_map_op(data: *mut c_void, entry: *mut imapper) -> i32 {
    unsafe {
        let mgr = addr_of_mut!((*(data as *mut daio_mgr)).mgr);
        let hw = (*mgr).hw;

        ((*hw).daio_mgr_set_imaparc)((*mgr).ctrl_blk, (*entry).slot);
        ((*hw).daio_mgr_set_imapnxt)((*mgr).ctrl_blk, (*entry).next);
        ((*hw).daio_mgr_set_imapaddr)((*mgr).ctrl_blk, (*entry).addr);
        ((*hw).daio_mgr_commit_write)((*mgr).hw, (*mgr).ctrl_blk);
    }

    0
}

unsafe extern "C" fn daio_imap_add(mgr: *mut daio_mgr, entry: *mut imapper) -> i32 {
    unsafe {
        let mut flags = 0usize;
        spin_lock_irqsave(addr_of_mut!((*mgr).imap_lock), &mut flags);
        if (*entry).addr == 0 && (*mgr).init_imap_added != 0 {
            input_mapper_delete(addr_of_mut!((*mgr).imappers), (*mgr).init_imap, daio_map_op, mgr as *mut c_void);
            (*mgr).init_imap_added = 0;
        }
        let ret = input_mapper_add(addr_of_mut!((*mgr).imappers), entry, daio_map_op, mgr as *mut c_void);
        spin_unlock_irqrestore(addr_of_mut!((*mgr).imap_lock), flags);
        ret
    }
}

unsafe extern "C" fn daio_imap_delete(mgr: *mut daio_mgr, entry: *mut imapper) -> i32 {
    unsafe {
        let mut flags = 0usize;
        spin_lock_irqsave(addr_of_mut!((*mgr).imap_lock), &mut flags);
        let err = input_mapper_delete(addr_of_mut!((*mgr).imappers), entry, daio_map_op, mgr as *mut c_void);
        if list_empty(addr_of_mut!((*mgr).imappers)) != 0 {
            input_mapper_add(addr_of_mut!((*mgr).imappers), (*mgr).init_imap, daio_map_op, mgr as *mut c_void);
            (*mgr).init_imap_added = 1;
        }
        spin_unlock_irqrestore(addr_of_mut!((*mgr).imap_lock), flags);

        err
    }
}

unsafe extern "C" fn daio_mgr_commit_write(mgr: *mut daio_mgr) -> i32 {
    unsafe {
        let hw = (*mgr).mgr.hw;

        ((*hw).daio_mgr_commit_write)(hw, (*mgr).mgr.ctrl_blk);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn daio_mgr_create(hw: *mut hw, rdaio_mgr: *mut *mut c_void) -> i32 {
    unsafe {
        *rdaio_mgr = null_mut();
        let daio_mgr = kzalloc(core::mem::size_of::<daio_mgr>(), GFP_KERNEL) as *mut daio_mgr;
        if daio_mgr.is_null() {
            return -ENOMEM;
        }

        let mut err = rsc_mgr_init(addr_of_mut!((*daio_mgr).mgr), DAIO, NUM_DAIOTYP, hw);
        if err != 0 {
            kfree(daio_mgr as *mut c_void);
            return err;
        }

        spin_lock_init(addr_of_mut!((*daio_mgr).mgr_lock));
        spin_lock_init(addr_of_mut!((*daio_mgr).imap_lock));
        INIT_LIST_HEAD(addr_of_mut!((*daio_mgr).imappers));
        let entry = kzalloc(core::mem::size_of::<imapper>(), GFP_KERNEL) as *mut imapper;
        if entry.is_null() {
            err = -ENOMEM;
            rsc_mgr_uninit(addr_of_mut!((*daio_mgr).mgr));
            kfree(daio_mgr as *mut c_void);
            return err;
        }
        (*entry).slot = 0;
        (*entry).addr = 0;
        (*entry).next = 0;
        (*entry).user = 0;
        list_add(addr_of_mut!((*entry).list), addr_of_mut!((*daio_mgr).imappers));
        (*daio_mgr).init_imap = entry;
        (*daio_mgr).init_imap_added = 1;

        (*daio_mgr).get_daio = Some(get_daio_rsc);
        (*daio_mgr).put_daio = Some(put_daio_rsc);
        (*daio_mgr).daio_enable = Some(daio_mgr_enb_daio);
        (*daio_mgr).daio_disable = Some(daio_mgr_dsb_daio);
        (*daio_mgr).imap_add = Some(daio_imap_add);
        (*daio_mgr).imap_delete = Some(daio_imap_delete);
        (*daio_mgr).commit_write = Some(daio_mgr_commit_write);
        (*daio_mgr).card = (*hw).card;

        for i in 0..8 {
            ((*hw).daio_mgr_dsb_dao)((*daio_mgr).mgr.ctrl_blk, i);
            ((*hw).daio_mgr_dsb_dai)((*daio_mgr).mgr.ctrl_blk, i);
        }
        ((*hw).daio_mgr_commit_write)(hw, (*daio_mgr).mgr.ctrl_blk);

        *rdaio_mgr = daio_mgr as *mut c_void;
    }

    0
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

#[no_mangle]
pub unsafe extern "C" fn daio_mgr_destroy(ptr: *mut c_void) -> i32 {
    unsafe {
        let daio_mgr = ptr as *mut daio_mgr;

        /* free daio input mapper list */
        let mut flags = 0usize;
        spin_lock_irqsave(addr_of_mut!((*daio_mgr).imap_lock), &mut flags);
        free_input_mapper_list(addr_of_mut!((*daio_mgr).imappers));
        spin_unlock_irqrestore(addr_of_mut!((*daio_mgr).imap_lock), flags);

        rsc_mgr_uninit(addr_of_mut!((*daio_mgr).mgr));
        kfree(daio_mgr as *mut c_void);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
