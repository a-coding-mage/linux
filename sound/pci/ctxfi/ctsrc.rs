// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	ctsrc.c
 *
 * @Brief
 * This file contains the implementation of the Sample Rate Convertor
 * resource management object.
 *
 * @Author	Liu Chun
 * @Date 	May 13 2008
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

type u32 = u32;

const SRC_RESOURCE_NUM: u32 = 256;
const SRCIMP_RESOURCE_NUM: u32 = 256;
const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;

const MEMRD: usize = 0;
const MEMWR: usize = 1;
const ARCRW: usize = 2;
const NUM_SRCMODES: u32 = 3;
const SRC: u32 = 0;
const SRCIMP: u32 = 1;
const SRC_STATE_OFF: u32 = 0;
const SRC_SF_S16: u32 = 0;
const SRC_SF_F32: u32 = 1;

static mut conj_mask: u32 = 0;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device;

#[repr(C)]
pub struct card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct hw {
    pub card: *mut card,
    pub src_set_state: unsafe extern "C" fn(*mut c_void, u32),
    pub src_set_bm: unsafe extern "C" fn(*mut c_void, u32),
    pub src_set_sf: unsafe extern "C" fn(*mut c_void, u32),
    pub src_set_pm: unsafe extern "C" fn(*mut c_void, u32),
    pub src_set_rom: unsafe extern "C" fn(*mut c_void, u32),
    pub src_set_vo: unsafe extern "C" fn(*mut c_void, u32),
    pub src_set_st: unsafe extern "C" fn(*mut c_void, u32),
    pub src_set_bp: unsafe extern "C" fn(*mut c_void, u32),
    pub src_set_cisz: unsafe extern "C" fn(*mut c_void, u32),
    pub src_set_ca: unsafe extern "C" fn(*mut c_void, u32),
    pub src_set_sa: unsafe extern "C" fn(*mut c_void, u32),
    pub src_set_la: unsafe extern "C" fn(*mut c_void, u32),
    pub src_set_pitch: unsafe extern "C" fn(*mut c_void, u32),
    pub src_set_clear_zbufs: unsafe extern "C" fn(*mut c_void, u32),
    pub src_get_dirty: unsafe extern "C" fn(*mut c_void) -> u32,
    pub src_set_dirty: unsafe extern "C" fn(*mut c_void, u32),
    pub src_commit_write: unsafe extern "C" fn(*mut hw, i32, *mut c_void),
    pub src_get_ca: unsafe extern "C" fn(*mut hw, i32, *mut c_void) -> i32,
    pub src_set_rsr: unsafe extern "C" fn(*mut c_void, u32),
    pub src_set_wr: unsafe extern "C" fn(*mut c_void, u32),
    pub src_set_ilsz: unsafe extern "C" fn(*mut c_void, u32),
    pub src_mgr_enbs_src: unsafe extern "C" fn(*mut c_void, i32),
    pub src_mgr_enb_src: unsafe extern "C" fn(*mut c_void, i32),
    pub src_mgr_dsb_src: unsafe extern "C" fn(*mut c_void, i32),
    pub src_mgr_commit_write: unsafe extern "C" fn(*mut hw, *mut c_void),
    pub src_dirty_conj_mask: unsafe extern "C" fn() -> u32,
    pub srcimp_mgr_set_imaparc: unsafe extern "C" fn(*mut c_void, u32),
    pub srcimp_mgr_set_imapuser: unsafe extern "C" fn(*mut c_void, u32),
    pub srcimp_mgr_set_imapnxt: unsafe extern "C" fn(*mut c_void, u32),
    pub srcimp_mgr_set_imapaddr: unsafe extern "C" fn(*mut c_void, u32),
    pub srcimp_mgr_commit_write: unsafe extern "C" fn(*mut hw, *mut c_void),
}

#[repr(C)]
pub struct rsc_ops {
    pub master: unsafe extern "C" fn(*mut rsc),
    pub next_conj: unsafe extern "C" fn(*mut rsc),
    pub index: unsafe extern "C" fn(*const rsc) -> i32,
    pub output_slot: Option<unsafe extern "C" fn(*mut rsc) -> u32>,
}

#[repr(C)]
pub struct rsc {
    pub hw: *mut hw,
    pub ctrl_blk: *mut c_void,
    pub ops: *const rsc_ops,
    pub msr: i32,
    pub conj: u32,
    pub idx: u32,
}

#[repr(C)]
pub struct rsc_mgr {
    pub hw: *mut hw,
    pub ctrl_blk: *mut c_void,
}

#[repr(C)]
pub struct src_desc {
    pub mode: u32,
    pub multi: i32,
    pub msr: i32,
}

#[repr(C)]
pub struct src_rsc_ops {
    pub set_state: unsafe extern "C" fn(*mut src, u32) -> i32,
    pub set_bm: unsafe extern "C" fn(*mut src, u32) -> i32,
    pub set_sf: unsafe extern "C" fn(*mut src, u32) -> i32,
    pub set_pm: unsafe extern "C" fn(*mut src, u32) -> i32,
    pub set_rom: unsafe extern "C" fn(*mut src, u32) -> i32,
    pub set_vo: unsafe extern "C" fn(*mut src, u32) -> i32,
    pub set_st: unsafe extern "C" fn(*mut src, u32) -> i32,
    pub set_bp: unsafe extern "C" fn(*mut src, u32) -> i32,
    pub set_cisz: unsafe extern "C" fn(*mut src, u32) -> i32,
    pub set_ca: unsafe extern "C" fn(*mut src, u32) -> i32,
    pub set_sa: unsafe extern "C" fn(*mut src, u32) -> i32,
    pub set_la: unsafe extern "C" fn(*mut src, u32) -> i32,
    pub set_pitch: unsafe extern "C" fn(*mut src, u32) -> i32,
    pub set_clr_zbufs: unsafe extern "C" fn(*mut src) -> i32,
    pub commit_write: unsafe extern "C" fn(*mut src) -> i32,
    pub get_ca: unsafe extern "C" fn(*mut src) -> i32,
    pub init: unsafe extern "C" fn(*mut src) -> i32,
    pub next_interleave: unsafe extern "C" fn(*mut src) -> *mut src,
}

#[repr(C)]
pub struct src {
    pub rsc: rsc,
    pub ops: *const src_rsc_ops,
    pub multi: i32,
    pub mode: u32,
    pub intlv: *mut src,
}

#[repr(C)]
pub struct src_mgr {
    pub mgr: rsc_mgr,
    pub mgr_lock: spinlock_t,
    pub get_src: unsafe extern "C" fn(*mut src_mgr, *const src_desc, *mut *mut src) -> i32,
    pub put_src: unsafe extern "C" fn(*mut src_mgr, *mut src) -> i32,
    pub src_enable_s: unsafe extern "C" fn(*mut src_mgr, *mut src) -> i32,
    pub src_enable: unsafe extern "C" fn(*mut src_mgr, *mut src) -> i32,
    pub src_disable: unsafe extern "C" fn(*mut src_mgr, *mut src) -> i32,
    pub commit_write: unsafe extern "C" fn(*mut src_mgr) -> i32,
    pub card: *mut card,
}

#[repr(C)]
pub struct imapper {
    pub list: list_head,
    pub slot: u32,
    pub user: u32,
    pub addr: u32,
    pub next: u32,
}

#[repr(C)]
pub struct srcimp_desc {
    pub msr: i32,
}

#[repr(C)]
pub struct srcimp_rsc_ops {
    pub map: unsafe extern "C" fn(*mut srcimp, *mut src, *mut rsc) -> i32,
    pub unmap: unsafe extern "C" fn(*mut srcimp) -> i32,
}

#[repr(C)]
pub struct srcimp {
    pub rsc: rsc,
    pub ops: *const srcimp_rsc_ops,
    pub mgr: *mut srcimp_mgr,
    pub mapped: u32,
    pub idx: [u32; 0],
    pub imappers: [imapper; 0],
}

#[repr(C)]
pub struct srcimp_mgr {
    pub mgr: rsc_mgr,
    pub mgr_lock: spinlock_t,
    pub imap_lock: spinlock_t,
    pub imappers: list_head,
    pub init_imap: *mut imapper,
    pub init_imap_added: i32,
    pub get_srcimp: unsafe extern "C" fn(*mut srcimp_mgr, *const srcimp_desc, *mut *mut srcimp) -> i32,
    pub put_srcimp: unsafe extern "C" fn(*mut srcimp_mgr, *mut srcimp) -> i32,
    pub imap_add: unsafe extern "C" fn(*mut srcimp_mgr, *mut imapper) -> i32,
    pub imap_delete: unsafe extern "C" fn(*mut srcimp_mgr, *mut imapper) -> i32,
    pub card: *mut card,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn rsc_init(rsc: *mut rsc, idx: u32, ty: u32, msr: i32, hw: *mut hw) -> i32;
    fn rsc_uninit(rsc: *mut rsc);
    fn rsc_mgr_init(mgr: *mut rsc_mgr, ty: u32, amount: u32, hw: *mut hw) -> i32;
    fn rsc_mgr_uninit(mgr: *mut rsc_mgr);
    fn mgr_get_resource(mgr: *mut rsc_mgr, amount: i32, idx: *mut u32) -> i32;
    fn mgr_put_resource(mgr: *mut rsc_mgr, amount: i32, idx: i32);
    fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kcalloc(n: i32, size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn input_mapper_add(
        head: *mut list_head,
        entry: *mut imapper,
        op: unsafe extern "C" fn(*mut c_void, *mut imapper) -> i32,
        data: *mut c_void,
    ) -> i32;
    fn input_mapper_delete(
        head: *mut list_head,
        entry: *mut imapper,
        op: unsafe extern "C" fn(*mut c_void, *mut imapper) -> i32,
        data: *mut c_void,
    ) -> i32;
    fn free_input_mapper_list(head: *mut list_head);
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe fn list_add(new: *mut list_head, head: *mut list_head) {
    (*new).next = (*head).next;
    (*new).prev = head;
    (*(*head).next).prev = new;
    (*head).next = new;
}

unsafe fn list_empty(head: *const list_head) -> bool {
    (*head).next == head as *mut list_head
}

unsafe fn srcimp_idx(srcimp: *mut srcimp, i: i32) -> *mut u32 {
    (srcimp as *mut u8)
        .add(size_of::<srcimp>())
        .add((i as usize) * size_of::<u32>()) as *mut u32
}

unsafe fn srcimp_imapper(srcimp: *mut srcimp, i: i32, msr: i32) -> *mut imapper {
    (srcimp as *mut u8)
        .add(size_of::<srcimp>())
        .add((msr as usize) * size_of::<u32>())
        .add((i as usize) * size_of::<imapper>()) as *mut imapper
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    kzalloc(size_of::<T>(), GFP_KERNEL) as *mut T
}

unsafe fn kzalloc_flex_srcimp(msr: i32) -> *mut srcimp {
    kzalloc(
        size_of::<srcimp>()
            + (msr as usize) * size_of::<u32>()
            + (msr as usize) * size_of::<imapper>(),
        GFP_KERNEL,
    ) as *mut srcimp
}

unsafe extern "C" fn src_default_config_memrd(src: *mut src) -> i32 {
    let hw = (*src).rsc.hw;
    let mut rsr: u32;
    let mut msr: i32;

    ((*hw).src_set_state)((*src).rsc.ctrl_blk, SRC_STATE_OFF);
    ((*hw).src_set_bm)((*src).rsc.ctrl_blk, 1);
    rsr = 0;
    msr = (*src).rsc.msr;
    while msr > 1 {
        rsr += 1;
        msr >>= 1;
    }

    ((*hw).src_set_rsr)((*src).rsc.ctrl_blk, rsr);
    ((*hw).src_set_sf)((*src).rsc.ctrl_blk, SRC_SF_S16);
    ((*hw).src_set_wr)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_pm)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_rom)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_vo)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_st)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_ilsz)((*src).rsc.ctrl_blk, ((*src).multi - 1) as u32);
    ((*hw).src_set_cisz)((*src).rsc.ctrl_blk, 0x80);
    ((*hw).src_set_sa)((*src).rsc.ctrl_blk, 0x0);
    ((*hw).src_set_la)((*src).rsc.ctrl_blk, 0x1000);
    ((*hw).src_set_ca)((*src).rsc.ctrl_blk, 0x80);
    ((*hw).src_set_pitch)((*src).rsc.ctrl_blk, 0x1000000);
    ((*hw).src_set_clear_zbufs)((*src).rsc.ctrl_blk, 1);

    ((*(*src).rsc.ops).master)(&mut (*src).rsc);
    ((*hw).src_commit_write)(hw, ((*(*src).rsc.ops).index)(&(*src).rsc), (*src).rsc.ctrl_blk);

    msr = 1;
    while msr < (*src).rsc.msr {
        ((*(*src).rsc.ops).next_conj)(&mut (*src).rsc);
        ((*hw).src_set_pitch)((*src).rsc.ctrl_blk, 0x1000000);
        ((*hw).src_commit_write)(hw, ((*(*src).rsc.ops).index)(&(*src).rsc), (*src).rsc.ctrl_blk);
        msr += 1;
    }
    ((*(*src).rsc.ops).master)(&mut (*src).rsc);

    0
}

unsafe extern "C" fn src_default_config_memwr(src: *mut src) -> i32 {
    let hw = (*src).rsc.hw;

    ((*hw).src_set_state)((*src).rsc.ctrl_blk, SRC_STATE_OFF);
    ((*hw).src_set_bm)((*src).rsc.ctrl_blk, 1);
    ((*hw).src_set_rsr)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_sf)((*src).rsc.ctrl_blk, SRC_SF_S16);
    ((*hw).src_set_wr)((*src).rsc.ctrl_blk, 1);
    ((*hw).src_set_pm)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_rom)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_vo)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_st)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_ilsz)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_cisz)((*src).rsc.ctrl_blk, 0x80);
    ((*hw).src_set_sa)((*src).rsc.ctrl_blk, 0x0);
    ((*hw).src_set_la)((*src).rsc.ctrl_blk, 0x1000);
    ((*hw).src_set_ca)((*src).rsc.ctrl_blk, 0x80);
    ((*hw).src_set_pitch)((*src).rsc.ctrl_blk, 0x1000000);
    ((*hw).src_set_clear_zbufs)((*src).rsc.ctrl_blk, 1);

    ((*(*src).rsc.ops).master)(&mut (*src).rsc);
    ((*hw).src_commit_write)(hw, ((*(*src).rsc.ops).index)(&(*src).rsc), (*src).rsc.ctrl_blk);

    0
}

unsafe extern "C" fn src_default_config_arcrw(src: *mut src) -> i32 {
    let hw = (*src).rsc.hw;
    let mut rsr: u32;
    let mut msr: i32;
    let dirty: u32;

    ((*hw).src_set_state)((*src).rsc.ctrl_blk, SRC_STATE_OFF);
    ((*hw).src_set_bm)((*src).rsc.ctrl_blk, 0);
    rsr = 0;
    msr = (*src).rsc.msr;
    while msr > 1 {
        rsr += 1;
        msr >>= 1;
    }

    ((*hw).src_set_rsr)((*src).rsc.ctrl_blk, rsr);
    ((*hw).src_set_sf)((*src).rsc.ctrl_blk, SRC_SF_F32);
    ((*hw).src_set_wr)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_pm)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_rom)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_vo)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_st)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_ilsz)((*src).rsc.ctrl_blk, 0);
    ((*hw).src_set_cisz)((*src).rsc.ctrl_blk, 0x80);
    ((*hw).src_set_sa)((*src).rsc.ctrl_blk, 0x0);
    /*hw->src_set_sa(src->rsc.ctrl_blk, 0x100);*/
    ((*hw).src_set_la)((*src).rsc.ctrl_blk, 0x1000);
    /*hw->src_set_la(src->rsc.ctrl_blk, 0x03ffffe0);*/
    ((*hw).src_set_ca)((*src).rsc.ctrl_blk, 0x80);
    ((*hw).src_set_pitch)((*src).rsc.ctrl_blk, 0x1000000);
    ((*hw).src_set_clear_zbufs)((*src).rsc.ctrl_blk, 1);

    dirty = ((*hw).src_get_dirty)((*src).rsc.ctrl_blk);
    ((*(*src).rsc.ops).master)(&mut (*src).rsc);
    msr = 0;
    while msr < (*src).rsc.msr {
        ((*hw).src_set_dirty)((*src).rsc.ctrl_blk, dirty);
        ((*hw).src_commit_write)(hw, ((*(*src).rsc.ops).index)(&(*src).rsc), (*src).rsc.ctrl_blk);
        ((*(*src).rsc.ops).next_conj)(&mut (*src).rsc);
        msr += 1;
    }
    ((*(*src).rsc.ops).master)(&mut (*src).rsc);

    0
}

static src_default_config: [unsafe extern "C" fn(*mut src) -> i32; 3] = [
    src_default_config_memrd,
    src_default_config_memwr,
    src_default_config_arcrw,
];

unsafe extern "C" fn src_set_state(src: *mut src, state: u32) -> i32 {
    let hw = (*src).rsc.hw;
    ((*hw).src_set_state)((*src).rsc.ctrl_blk, state);
    0
}

unsafe extern "C" fn src_set_bm(src: *mut src, bm: u32) -> i32 {
    let hw = (*src).rsc.hw;
    ((*hw).src_set_bm)((*src).rsc.ctrl_blk, bm);
    0
}

unsafe extern "C" fn src_set_sf(src: *mut src, sf: u32) -> i32 {
    let hw = (*src).rsc.hw;
    ((*hw).src_set_sf)((*src).rsc.ctrl_blk, sf);
    0
}

unsafe extern "C" fn src_set_pm(src: *mut src, pm: u32) -> i32 {
    let hw = (*src).rsc.hw;
    ((*hw).src_set_pm)((*src).rsc.ctrl_blk, pm);
    0
}

unsafe extern "C" fn src_set_rom(src: *mut src, rom: u32) -> i32 {
    let hw = (*src).rsc.hw;
    ((*hw).src_set_rom)((*src).rsc.ctrl_blk, rom);
    0
}

unsafe extern "C" fn src_set_vo(src: *mut src, vo: u32) -> i32 {
    let hw = (*src).rsc.hw;
    ((*hw).src_set_vo)((*src).rsc.ctrl_blk, vo);
    0
}

unsafe extern "C" fn src_set_st(src: *mut src, st: u32) -> i32 {
    let hw = (*src).rsc.hw;
    ((*hw).src_set_st)((*src).rsc.ctrl_blk, st);
    0
}

unsafe extern "C" fn src_set_bp(src: *mut src, bp: u32) -> i32 {
    let hw = (*src).rsc.hw;
    ((*hw).src_set_bp)((*src).rsc.ctrl_blk, bp);
    0
}

unsafe extern "C" fn src_set_cisz(src: *mut src, cisz: u32) -> i32 {
    let hw = (*src).rsc.hw;
    ((*hw).src_set_cisz)((*src).rsc.ctrl_blk, cisz);
    0
}

unsafe extern "C" fn src_set_ca(src: *mut src, ca: u32) -> i32 {
    let hw = (*src).rsc.hw;
    ((*hw).src_set_ca)((*src).rsc.ctrl_blk, ca);
    0
}

unsafe extern "C" fn src_set_sa(src: *mut src, sa: u32) -> i32 {
    let hw = (*src).rsc.hw;
    ((*hw).src_set_sa)((*src).rsc.ctrl_blk, sa);
    0
}

unsafe extern "C" fn src_set_la(src: *mut src, la: u32) -> i32 {
    let hw = (*src).rsc.hw;
    ((*hw).src_set_la)((*src).rsc.ctrl_blk, la);
    0
}

unsafe extern "C" fn src_set_pitch(src: *mut src, pitch: u32) -> i32 {
    let hw = (*src).rsc.hw;
    ((*hw).src_set_pitch)((*src).rsc.ctrl_blk, pitch);
    0
}

unsafe extern "C" fn src_set_clear_zbufs(src: *mut src) -> i32 {
    let hw = (*src).rsc.hw;
    ((*hw).src_set_clear_zbufs)((*src).rsc.ctrl_blk, 1);
    0
}

unsafe extern "C" fn src_commit_write(src: *mut src) -> i32 {
    let hw = (*src).rsc.hw;
    let mut i: i32;
    let mut dirty: u32 = 0;

    ((*(*src).rsc.ops).master)(&mut (*src).rsc);
    if (*src).rsc.msr > 1 {
        /* Save dirty flags for conjugate resource programming */
        dirty = ((*hw).src_get_dirty)((*src).rsc.ctrl_blk) & conj_mask;
    }
    ((*hw).src_commit_write)(hw, ((*(*src).rsc.ops).index)(&(*src).rsc), (*src).rsc.ctrl_blk);

    /* Program conjugate parameter mixer resources */
    if MEMWR as u32 == (*src).mode {
        return 0;
    }

    i = 1;
    while i < (*src).rsc.msr {
        ((*(*src).rsc.ops).next_conj)(&mut (*src).rsc);
        ((*hw).src_set_dirty)((*src).rsc.ctrl_blk, dirty);
        ((*hw).src_commit_write)(hw, ((*(*src).rsc.ops).index)(&(*src).rsc), (*src).rsc.ctrl_blk);
        i += 1;
    }
    ((*(*src).rsc.ops).master)(&mut (*src).rsc);

    0
}

unsafe extern "C" fn src_get_ca(src: *mut src) -> i32 {
    let hw = (*src).rsc.hw;
    ((*hw).src_get_ca)(hw, ((*(*src).rsc.ops).index)(&(*src).rsc), (*src).rsc.ctrl_blk)
}

unsafe extern "C" fn src_init(src: *mut src) -> i32 {
    src_default_config[(*src).mode as usize](src);
    0
}

unsafe extern "C" fn src_next_interleave(src: *mut src) -> *mut src {
    (*src).intlv
}

static src_rsc_ops: src_rsc_ops = src_rsc_ops {
    set_state: src_set_state,
    set_bm: src_set_bm,
    set_sf: src_set_sf,
    set_pm: src_set_pm,
    set_rom: src_set_rom,
    set_vo: src_set_vo,
    set_st: src_set_st,
    set_bp: src_set_bp,
    set_cisz: src_set_cisz,
    set_ca: src_set_ca,
    set_sa: src_set_sa,
    set_la: src_set_la,
    set_pitch: src_set_pitch,
    set_clr_zbufs: src_set_clear_zbufs,
    commit_write: src_commit_write,
    get_ca: src_get_ca,
    init: src_init,
    next_interleave: src_next_interleave,
};

unsafe fn src_rsc_init(src: *mut src, idx: u32, desc: *const src_desc, mgr: *mut src_mgr) -> i32 {
    let mut err: i32;
    let mut i: i32;
    let n: i32;
    let mut p: *mut src;

    n = if MEMRD as u32 == (*desc).mode { (*desc).multi } else { 1 };
    i = 0;
    p = src;
    while i < n {
        err = rsc_init(&mut (*p).rsc, idx + i as u32, SRC, (*desc).msr, (*mgr).mgr.hw);
        if err != 0 {
            goto_error1(src, mgr, i, p, err);
            return err;
        }

        /* Initialize src specific rsc operations */
        (*p).ops = &src_rsc_ops;
        (*p).multi = if 0 == i { (*desc).multi } else { 1 };
        (*p).mode = (*desc).mode;
        src_default_config[(*desc).mode as usize](p);
        ((*mgr).src_enable)(mgr, p);
        (*p).intlv = p.add(1);
        i += 1;
        p = p.add(1);
    }
    p = p.sub(1);
    (*p).intlv = ptr::null_mut(); /* Set @intlv of the last SRC to NULL */

    ((*mgr).commit_write)(mgr);

    0
}

unsafe fn goto_error1(src: *mut src, mgr: *mut src_mgr, mut i: i32, mut p: *mut src, err: i32) -> i32 {
    i -= 1;
    p = p.sub(1);
    while i >= 0 {
        ((*mgr).src_disable)(mgr, p);
        rsc_uninit(&mut (*p).rsc);
        i -= 1;
        p = p.sub(1);
    }
    ((*mgr).commit_write)(mgr);
    err
}

unsafe fn src_rsc_uninit(src: *mut src, mgr: *mut src_mgr) -> i32 {
    let mut i: i32;
    let n: i32;
    let mut p: *mut src;

    n = if MEMRD as u32 == (*src).mode { (*src).multi } else { 1 };
    i = 0;
    p = src;
    while i < n {
        ((*mgr).src_disable)(mgr, p);
        rsc_uninit(&mut (*p).rsc);
        (*p).multi = 0;
        (*p).ops = ptr::null();
        (*p).mode = NUM_SRCMODES;
        (*p).intlv = ptr::null_mut();
        i += 1;
        p = p.add(1);
    }
    ((*mgr).commit_write)(mgr);

    0
}

unsafe extern "C" fn get_src_rsc(mgr: *mut src_mgr, desc: *const src_desc, rsrc: *mut *mut src) -> i32 {
    let mut idx: u32 = SRC_RESOURCE_NUM;
    let mut err: i32;
    let src: *mut src;

    *rsrc = ptr::null_mut();

    /* Check whether there are sufficient src resources to meet request. */
    /* scoped_guard(spinlock_irqsave, &mgr->mgr_lock) */
    if MEMRD as u32 == (*desc).mode {
        err = mgr_get_resource(&mut (*mgr).mgr, (*desc).multi, &mut idx);
    } else {
        err = mgr_get_resource(&mut (*mgr).mgr, 1, &mut idx);
    }
    if err != 0 {
        dev_err((*(*mgr).card).dev, b"Can't meet SRC resource request!\n\0".as_ptr());
        return err;
    }

    /* Allocate mem for master src resource */
    if MEMRD as u32 == (*desc).mode {
        src = kcalloc((*desc).multi, size_of::<src>(), GFP_KERNEL) as *mut src;
    } else {
        src = kzalloc(size_of::<src>(), GFP_KERNEL) as *mut src;
    }

    if src.is_null() {
        err = -ENOMEM;
        if MEMRD as u32 == (*desc).mode {
            mgr_put_resource(&mut (*mgr).mgr, (*desc).multi, idx as i32);
        } else {
            mgr_put_resource(&mut (*mgr).mgr, 1, idx as i32);
        }
        return err;
    }

    err = src_rsc_init(src, idx, desc, mgr);
    if err != 0 {
        kfree(src as *mut c_void);
        /* scoped_guard(spinlock_irqsave, &mgr->mgr_lock) */
        if MEMRD as u32 == (*desc).mode {
            mgr_put_resource(&mut (*mgr).mgr, (*desc).multi, idx as i32);
        } else {
            mgr_put_resource(&mut (*mgr).mgr, 1, idx as i32);
        }
        return err;
    }

    *rsrc = src;

    0
}

unsafe extern "C" fn put_src_rsc(mgr: *mut src_mgr, src: *mut src) -> i32 {
    /* scoped_guard(spinlock_irqsave, &mgr->mgr_lock) */
    ((*(*src).rsc.ops).master)(&mut (*src).rsc);
    if MEMRD as u32 == (*src).mode {
        mgr_put_resource(&mut (*mgr).mgr, (*src).multi, ((*(*src).rsc.ops).index)(&(*src).rsc));
    } else {
        mgr_put_resource(&mut (*mgr).mgr, 1, ((*(*src).rsc.ops).index)(&(*src).rsc));
    }
    src_rsc_uninit(src, mgr);
    kfree(src as *mut c_void);

    0
}

unsafe extern "C" fn src_enable_s(mgr: *mut src_mgr, src: *mut src) -> i32 {
    let hw = (*mgr).mgr.hw;
    let mut i: i32;

    ((*(*src).rsc.ops).master)(&mut (*src).rsc);
    i = 0;
    while i < (*src).rsc.msr {
        ((*hw).src_mgr_enbs_src)((*mgr).mgr.ctrl_blk, ((*(*src).rsc.ops).index)(&(*src).rsc));
        ((*(*src).rsc.ops).next_conj)(&mut (*src).rsc);
        i += 1;
    }
    ((*(*src).rsc.ops).master)(&mut (*src).rsc);

    0
}

unsafe extern "C" fn src_enable(mgr: *mut src_mgr, src: *mut src) -> i32 {
    let hw = (*mgr).mgr.hw;
    let mut i: i32;

    ((*(*src).rsc.ops).master)(&mut (*src).rsc);
    i = 0;
    while i < (*src).rsc.msr {
        ((*hw).src_mgr_enb_src)((*mgr).mgr.ctrl_blk, ((*(*src).rsc.ops).index)(&(*src).rsc));
        ((*(*src).rsc.ops).next_conj)(&mut (*src).rsc);
        i += 1;
    }
    ((*(*src).rsc.ops).master)(&mut (*src).rsc);

    0
}

unsafe extern "C" fn src_disable(mgr: *mut src_mgr, src: *mut src) -> i32 {
    let hw = (*mgr).mgr.hw;
    let mut i: i32;

    ((*(*src).rsc.ops).master)(&mut (*src).rsc);
    i = 0;
    while i < (*src).rsc.msr {
        ((*hw).src_mgr_dsb_src)((*mgr).mgr.ctrl_blk, ((*(*src).rsc.ops).index)(&(*src).rsc));
        ((*(*src).rsc.ops).next_conj)(&mut (*src).rsc);
        i += 1;
    }
    ((*(*src).rsc.ops).master)(&mut (*src).rsc);

    0
}

unsafe extern "C" fn src_mgr_commit_write(mgr: *mut src_mgr) -> i32 {
    let hw = (*mgr).mgr.hw;
    ((*hw).src_mgr_commit_write)(hw, (*mgr).mgr.ctrl_blk);
    0
}

#[no_mangle]
pub unsafe extern "C" fn src_mgr_create(hw: *mut hw, rsrc_mgr: *mut *mut c_void) -> i32 {
    let mut err: i32;
    let mut i: i32;
    let src_mgr: *mut src_mgr;

    *rsrc_mgr = ptr::null_mut();
    src_mgr = kzalloc_obj::<src_mgr>();
    if src_mgr.is_null() {
        return -ENOMEM;
    }

    err = rsc_mgr_init(&mut (*src_mgr).mgr, SRC, SRC_RESOURCE_NUM, hw);
    if err != 0 {
        kfree(src_mgr as *mut c_void);
        return err;
    }

    spin_lock_init(&mut (*src_mgr).mgr_lock);
    conj_mask = ((*hw).src_dirty_conj_mask)();

    (*src_mgr).get_src = get_src_rsc;
    (*src_mgr).put_src = put_src_rsc;
    (*src_mgr).src_enable_s = src_enable_s;
    (*src_mgr).src_enable = src_enable;
    (*src_mgr).src_disable = src_disable;
    (*src_mgr).commit_write = src_mgr_commit_write;
    (*src_mgr).card = (*hw).card;

    /* Disable all SRC resources. */
    i = 0;
    while i < 256 {
        ((*hw).src_mgr_dsb_src)((*src_mgr).mgr.ctrl_blk, i);
        i += 1;
    }

    ((*hw).src_mgr_commit_write)(hw, (*src_mgr).mgr.ctrl_blk);

    *rsrc_mgr = src_mgr as *mut c_void;

    0
}

#[no_mangle]
pub unsafe extern "C" fn src_mgr_destroy(ptr: *mut c_void) -> i32 {
    let src_mgr = ptr as *mut src_mgr;
    rsc_mgr_uninit(&mut (*src_mgr).mgr);
    kfree(src_mgr as *mut c_void);

    0
}

/* SRCIMP resource manager operations */

unsafe extern "C" fn srcimp_master(rsc: *mut rsc) {
    let srcimp = rsc as *mut srcimp;
    (*rsc).conj = 0;
    (*rsc).idx = *srcimp_idx(srcimp, 0);
}

unsafe extern "C" fn srcimp_next_conj(rsc: *mut rsc) {
    (*rsc).conj += 1;
}

unsafe extern "C" fn srcimp_index(rsc: *const rsc) -> i32 {
    let srcimp = rsc as *mut srcimp;
    *srcimp_idx(srcimp, (*rsc).conj as i32) as i32
}

static srcimp_basic_rsc_ops: rsc_ops = rsc_ops {
    master: srcimp_master,
    next_conj: srcimp_next_conj,
    index: srcimp_index,
    output_slot: None,
};

unsafe extern "C" fn srcimp_map(srcimp: *mut srcimp, src: *mut src, input: *mut rsc) -> i32 {
    let mut entry: *mut imapper;
    let mut i: i32;

    ((*(*srcimp).rsc.ops).master)(&mut (*srcimp).rsc);
    ((*(*src).rsc.ops).master)(&mut (*src).rsc);
    ((*(*input).ops).master)(input);

    /* Program master and conjugate resources */
    i = 0;
    while i < (*srcimp).rsc.msr {
        entry = srcimp_imapper(srcimp, i, (*srcimp).rsc.msr);
        (*entry).slot = ((*(*input).ops).output_slot.unwrap())(input);
        (*entry).user = ((*(*src).rsc.ops).index)(&(*src).rsc) as u32;
        (*entry).addr = ((*(*srcimp).rsc.ops).index)(&(*srcimp).rsc) as u32;
        ((*(*srcimp).mgr).imap_add)((*srcimp).mgr, entry);
        (*srcimp).mapped |= 0x1u32 << i;

        ((*(*srcimp).rsc.ops).next_conj)(&mut (*srcimp).rsc);
        ((*(*input).ops).next_conj)(input);
        i += 1;
    }

    ((*(*srcimp).rsc.ops).master)(&mut (*srcimp).rsc);
    ((*(*input).ops).master)(input);

    0
}

unsafe extern "C" fn srcimp_unmap(srcimp: *mut srcimp) -> i32 {
    let mut i: i32;

    /* Program master and conjugate resources */
    i = 0;
    while i < (*srcimp).rsc.msr {
        if ((*srcimp).mapped & (0x1u32 << i)) != 0 {
            ((*(*srcimp).mgr).imap_delete)((*srcimp).mgr, srcimp_imapper(srcimp, i, (*srcimp).rsc.msr));
            (*srcimp).mapped &= !(0x1u32 << i);
        }
        i += 1;
    }

    0
}

static srcimp_ops: srcimp_rsc_ops = srcimp_rsc_ops {
    map: srcimp_map,
    unmap: srcimp_unmap,
};

unsafe fn srcimp_rsc_init(srcimp: *mut srcimp, desc: *const srcimp_desc, mgr: *mut srcimp_mgr) -> i32 {
    let err: i32;

    err = rsc_init(&mut (*srcimp).rsc, *srcimp_idx(srcimp, 0), SRCIMP, (*desc).msr, (*mgr).mgr.hw);
    if err != 0 {
        return err;
    }

    /* Set srcimp specific operations */
    (*srcimp).rsc.ops = &srcimp_basic_rsc_ops;
    (*srcimp).ops = &srcimp_ops;
    (*srcimp).mgr = mgr;

    ((*(*srcimp).rsc.ops).master)(&mut (*srcimp).rsc);

    0
}

unsafe fn srcimp_rsc_uninit(srcimp: *mut srcimp) -> i32 {
    (*srcimp).ops = ptr::null();
    (*srcimp).mgr = ptr::null_mut();
    rsc_uninit(&mut (*srcimp).rsc);

    0
}

unsafe extern "C" fn get_srcimp_rsc(
    mgr: *mut srcimp_mgr,
    desc: *const srcimp_desc,
    rsrcimp: *mut *mut srcimp,
) -> i32 {
    let mut err: i32;
    let mut i: i32;
    let mut idx: u32 = 0;
    let srcimp: *mut srcimp;

    *rsrcimp = ptr::null_mut();

    /* Allocate mem for SRCIMP resource */
    srcimp = kzalloc_flex_srcimp((*desc).msr);
    if srcimp.is_null() {
        return -ENOMEM;
    }

    /* Check whether there are sufficient SRCIMP resources. */
    err = 0;
    /* scoped_guard(spinlock_irqsave, &mgr->mgr_lock) */
    i = 0;
    while i < (*desc).msr {
        err = mgr_get_resource(&mut (*mgr).mgr, 1, &mut idx);
        if err != 0 {
            break;
        }

        *srcimp_idx(srcimp, i) = idx;
        i += 1;
    }
    if err != 0 {
        dev_err((*(*mgr).card).dev, b"Can't meet SRCIMP resource request!\n\0".as_ptr());
        /* error1 */
        i -= 1;
        while i >= 0 {
            mgr_put_resource(&mut (*mgr).mgr, 1, *srcimp_idx(srcimp, i) as i32);
            i -= 1;
        }
        kfree(srcimp as *mut c_void);
        return err;
    }

    err = srcimp_rsc_init(srcimp, desc, mgr);
    if err != 0 {
        /* error1 */
        i -= 1;
        while i >= 0 {
            mgr_put_resource(&mut (*mgr).mgr, 1, *srcimp_idx(srcimp, i) as i32);
            i -= 1;
        }
        kfree(srcimp as *mut c_void);
        return err;
    }

    *rsrcimp = srcimp;

    0
}

unsafe extern "C" fn put_srcimp_rsc(mgr: *mut srcimp_mgr, srcimp: *mut srcimp) -> i32 {
    let mut i: i32;

    /* scoped_guard(spinlock_irqsave, &mgr->mgr_lock) */
    i = 0;
    while i < (*srcimp).rsc.msr {
        mgr_put_resource(&mut (*mgr).mgr, 1, *srcimp_idx(srcimp, i) as i32);
        i += 1;
    }
    srcimp_rsc_uninit(srcimp);
    kfree(srcimp as *mut c_void);

    0
}

unsafe extern "C" fn srcimp_map_op(data: *mut c_void, entry: *mut imapper) -> i32 {
    let mgr = &mut (*(data as *mut srcimp_mgr)).mgr as *mut rsc_mgr;
    let hw = (*mgr).hw;

    ((*hw).srcimp_mgr_set_imaparc)((*mgr).ctrl_blk, (*entry).slot);
    ((*hw).srcimp_mgr_set_imapuser)((*mgr).ctrl_blk, (*entry).user);
    ((*hw).srcimp_mgr_set_imapnxt)((*mgr).ctrl_blk, (*entry).next);
    ((*hw).srcimp_mgr_set_imapaddr)((*mgr).ctrl_blk, (*entry).addr);
    ((*hw).srcimp_mgr_commit_write)((*mgr).hw, (*mgr).ctrl_blk);

    0
}

unsafe extern "C" fn srcimp_imap_add(mgr: *mut srcimp_mgr, entry: *mut imapper) -> i32 {
    /* guard(spinlock_irqsave)(&mgr->imap_lock); */
    if 0 == (*entry).addr && (*mgr).init_imap_added != 0 {
        input_mapper_delete(&mut (*mgr).imappers, (*mgr).init_imap, srcimp_map_op, mgr as *mut c_void);
        (*mgr).init_imap_added = 0;
    }
    input_mapper_add(&mut (*mgr).imappers, entry, srcimp_map_op, mgr as *mut c_void)
}

unsafe extern "C" fn srcimp_imap_delete(mgr: *mut srcimp_mgr, entry: *mut imapper) -> i32 {
    let err: i32;

    /* guard(spinlock_irqsave)(&mgr->imap_lock); */
    err = input_mapper_delete(&mut (*mgr).imappers, entry, srcimp_map_op, mgr as *mut c_void);
    if list_empty(&(*mgr).imappers) {
        input_mapper_add(&mut (*mgr).imappers, (*mgr).init_imap, srcimp_map_op, mgr as *mut c_void);
        (*mgr).init_imap_added = 1;
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn srcimp_mgr_create(hw: *mut hw, rsrcimp_mgr: *mut *mut c_void) -> i32 {
    let mut err: i32;
    let srcimp_mgr: *mut srcimp_mgr;
    let entry: *mut imapper;

    *rsrcimp_mgr = ptr::null_mut();
    srcimp_mgr = kzalloc_obj::<srcimp_mgr>();
    if srcimp_mgr.is_null() {
        return -ENOMEM;
    }

    err = rsc_mgr_init(&mut (*srcimp_mgr).mgr, SRCIMP, SRCIMP_RESOURCE_NUM, hw);
    if err != 0 {
        kfree(srcimp_mgr as *mut c_void);
        return err;
    }

    spin_lock_init(&mut (*srcimp_mgr).mgr_lock);
    spin_lock_init(&mut (*srcimp_mgr).imap_lock);
    INIT_LIST_HEAD(&mut (*srcimp_mgr).imappers);
    entry = kzalloc_obj::<imapper>();
    if entry.is_null() {
        err = -ENOMEM;
        rsc_mgr_uninit(&mut (*srcimp_mgr).mgr);
        kfree(srcimp_mgr as *mut c_void);
        return err;
    }
    (*entry).slot = 0;
    (*entry).addr = 0;
    (*entry).next = 0;
    (*entry).user = 0;
    list_add(&mut (*entry).list, &mut (*srcimp_mgr).imappers);
    (*srcimp_mgr).init_imap = entry;
    (*srcimp_mgr).init_imap_added = 1;

    (*srcimp_mgr).get_srcimp = get_srcimp_rsc;
    (*srcimp_mgr).put_srcimp = put_srcimp_rsc;
    (*srcimp_mgr).imap_add = srcimp_imap_add;
    (*srcimp_mgr).imap_delete = srcimp_imap_delete;
    (*srcimp_mgr).card = (*hw).card;

    *rsrcimp_mgr = srcimp_mgr as *mut c_void;

    0
}

#[no_mangle]
pub unsafe extern "C" fn srcimp_mgr_destroy(ptr: *mut c_void) -> i32 {
    let srcimp_mgr = ptr as *mut srcimp_mgr;

    /* free src input mapper list */
    /* scoped_guard(spinlock_irqsave, &srcimp_mgr->imap_lock) */
    free_input_mapper_list(&mut (*srcimp_mgr).imappers);

    rsc_mgr_uninit(&mut (*srcimp_mgr).mgr);
    kfree(srcimp_mgr as *mut c_void);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
