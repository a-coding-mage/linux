// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	cthw20k2.c
 *
 * @Brief
 * This file contains the implementation of hardware access method for 20k2.
 *
 * @Author	Liu Chun
 * @Date 	May 14 2008
 */

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

/* Dependencies from linux/types.h, linux/slab.h, linux/pci.h, linux/io.h,
 * linux/string.h, linux/kernel.h, linux/interrupt.h, linux/delay.h,
 * cthw20k2.h, and ct20k2reg.h are expected to be supplied by the surrounding
 * translation unit/module.
 */
use crate::*;

#[repr(C)]
pub struct hw20k2 {
    pub hw: hw,
    /* for i2c */
    pub dev_id: u8,
    pub addr_size: u8,
    pub data_size: u8,

    pub mic_source: i32,
}

unsafe fn hw_read_20kx(hw: *mut hw, reg: u32) -> u32;
unsafe fn hw_write_20kx(hw: *mut hw, reg: u32, data: u32);

/*
 * Type definition block.
 * The layout of control structures can be directly applied on 20k2 chip.
 */

/*
 * SRC control block definitions.
 */

/* SRC resource control block */
const SRCCTL_STATE: u32 = 0x00000007;
const SRCCTL_BM: u32 = 0x00000008;
const SRCCTL_RSR: u32 = 0x00000030;
const SRCCTL_SF: u32 = 0x000001C0;
const SRCCTL_WR: u32 = 0x00000200;
const SRCCTL_PM: u32 = 0x00000400;
const SRCCTL_ROM: u32 = 0x00001800;
const SRCCTL_VO: u32 = 0x00002000;
const SRCCTL_ST: u32 = 0x00004000;
const SRCCTL_IE: u32 = 0x00008000;
const SRCCTL_ILSZ: u32 = 0x000F0000;
const SRCCTL_BP: u32 = 0x00100000;

const SRCCCR_CISZ: u32 = 0x000007FF;
const SRCCCR_CWA: u32 = 0x001FF800;
const SRCCCR_D: u32 = 0x00200000;
const SRCCCR_RS: u32 = 0x01C00000;
const SRCCCR_NAL: u32 = 0x3E000000;
const SRCCCR_RA: u32 = 0xC0000000;

const SRCCA_CA: u32 = 0x0FFFFFFF;
const SRCCA_RS: u32 = 0xE0000000;

const SRCSA_SA: u32 = 0x0FFFFFFF;

const SRCLA_LA: u32 = 0x0FFFFFFF;

/* Mixer Parameter Ring ram Low and Hight register.
 * Fixed-point value in 8.24 format for parameter channel */
const MPRLH_PITCH: u32 = 0xFFFFFFFF;

/* SRC resource register dirty flags */
#[repr(C)]
pub union src_dirty {
    pub data: u16,
}

const SRC_DIRTY_CTL: u16 = 1 << 0;
const SRC_DIRTY_CCR: u16 = 1 << 1;
const SRC_DIRTY_SA: u16 = 1 << 2;
const SRC_DIRTY_LA: u16 = 1 << 3;
const SRC_DIRTY_CA: u16 = 1 << 4;
const SRC_DIRTY_MPR: u16 = 1 << 5;
const SRC_DIRTY_CZBFS: u16 = 1 << 6;

#[repr(C)]
pub struct src_rsc_ctrl_blk {
    pub ctl: u32,
    pub ccr: u32,
    pub ca: u32,
    pub sa: u32,
    pub la: u32,
    pub mpr: u32,
    pub dirty: src_dirty,
}

/* SRC manager control block */
#[repr(C)]
pub union src_mgr_dirty {
    pub data: u16,
}

const SRC_MGR_DIRTY_ENBSA: u16 = 1 << 8;

#[repr(C)]
pub struct src_mgr_ctrl_blk {
    pub enbsa: u32,
    pub enb: [u32; 8],
    pub dirty: src_mgr_dirty,
}

/* SRCIMP manager control block */
const SRCAIM_ARC: u32 = 0x00000FFF;
const SRCAIM_NXT: u32 = 0x00FF0000;
const SRCAIM_SRC: u32 = 0xFF000000;

#[repr(C)]
pub struct srcimap {
    pub srcaim: u32,
    pub idx: u32,
}

/* SRCIMP manager register dirty flags */
#[repr(C)]
pub union srcimp_mgr_dirty {
    pub data: u16,
}

const SRCIMP_MGR_DIRTY_SRCIMAP: u16 = 1 << 0;

#[repr(C)]
pub struct srcimp_mgr_ctrl_blk {
    pub srcimap: srcimap,
    pub dirty: srcimp_mgr_dirty,
}

unsafe fn dirty16_get(dirty: *const u16, mask: u16) -> bool {
    (*dirty & mask) != 0
}

unsafe fn dirty16_set(dirty: *mut u16, mask: u16, val: bool) {
    if val {
        *dirty |= mask;
    } else {
        *dirty &= !mask;
    }
}

unsafe fn dirty32_get(dirty: *const u32, mask: u32) -> bool {
    (*dirty & mask) != 0
}

unsafe fn dirty32_set(dirty: *mut u32, mask: u32, val: bool) {
    if val {
        *dirty |= mask;
    } else {
        *dirty &= !mask;
    }
}

/*
 * Function implementation block.
 */

unsafe fn src_get_rsc_ctrl_blk(rblk: *mut *mut c_void) -> i32 {
    let blk: *mut src_rsc_ctrl_blk;

    *rblk = ptr::null_mut();
    blk = kzalloc_obj::<src_rsc_ctrl_blk>() as *mut src_rsc_ctrl_blk;
    if blk.is_null() {
        return -ENOMEM;
    }

    *rblk = blk as *mut c_void;

    0
}

unsafe fn src_put_rsc_ctrl_blk(blk: *mut c_void) -> i32 {
    kfree(blk);

    0
}

unsafe fn src_set_state(blk: *mut c_void, state: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).ctl, SRCCTL_STATE, state);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CTL, true);
    0
}

unsafe fn src_set_bm(blk: *mut c_void, bm: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).ctl, SRCCTL_BM, bm);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CTL, true);
    0
}

unsafe fn src_set_rsr(blk: *mut c_void, rsr: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).ctl, SRCCTL_RSR, rsr);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CTL, true);
    0
}

unsafe fn src_set_sf(blk: *mut c_void, sf: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).ctl, SRCCTL_SF, sf);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CTL, true);
    0
}

unsafe fn src_set_wr(blk: *mut c_void, wr: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).ctl, SRCCTL_WR, wr);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CTL, true);
    0
}

unsafe fn src_set_pm(blk: *mut c_void, pm: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).ctl, SRCCTL_PM, pm);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CTL, true);
    0
}

unsafe fn src_set_rom(blk: *mut c_void, rom: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).ctl, SRCCTL_ROM, rom);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CTL, true);
    0
}

unsafe fn src_set_vo(blk: *mut c_void, vo: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).ctl, SRCCTL_VO, vo);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CTL, true);
    0
}

unsafe fn src_set_st(blk: *mut c_void, st: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).ctl, SRCCTL_ST, st);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CTL, true);
    0
}

unsafe fn src_set_ie(blk: *mut c_void, ie: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).ctl, SRCCTL_IE, ie);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CTL, true);
    0
}

unsafe fn src_set_ilsz(blk: *mut c_void, ilsz: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).ctl, SRCCTL_ILSZ, ilsz);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CTL, true);
    0
}

unsafe fn src_set_bp(blk: *mut c_void, bp: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).ctl, SRCCTL_BP, bp);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CTL, true);
    0
}

unsafe fn src_set_cisz(blk: *mut c_void, cisz: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).ccr, SRCCCR_CISZ, cisz);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CCR, true);
    0
}

unsafe fn src_set_ca(blk: *mut c_void, ca: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).ca, SRCCA_CA, ca);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CA, true);
    0
}

unsafe fn src_set_sa(blk: *mut c_void, sa: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).sa, SRCSA_SA, sa);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_SA, true);
    0
}

unsafe fn src_set_la(blk: *mut c_void, la: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).la, SRCLA_LA, la);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_LA, true);
    0
}

unsafe fn src_set_pitch(blk: *mut c_void, pitch: u32) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    set_field(&mut (*ctl).mpr, MPRLH_PITCH, pitch);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_MPR, true);
    0
}

unsafe fn src_set_clear_zbufs(blk: *mut c_void, clear: u32) -> i32 {
    dirty16_set(&mut (*(blk as *mut src_rsc_ctrl_blk)).dirty.data, SRC_DIRTY_CZBFS, clear != 0);
    0
}

unsafe fn src_set_dirty(blk: *mut c_void, flags: u32) -> i32 {
    (*(blk as *mut src_rsc_ctrl_blk)).dirty.data = (flags & 0xffff) as u16;
    0
}

unsafe fn src_set_dirty_all(blk: *mut c_void) -> i32 {
    (*(blk as *mut src_rsc_ctrl_blk)).dirty.data = !(0x0u16);
    0
}

const AR_SLOT_SIZE: u32 = 4096;
const AR_SLOT_BLOCK_SIZE: u32 = 16;
const AR_PTS_PITCH: u32 = 6;
const AR_PARAM_SRC_OFFSET: u32 = 0x60;

fn src_param_pitch_mixer(src_idx: u32) -> u32 {
    ((src_idx << 4) + AR_PTS_PITCH + AR_SLOT_SIZE - AR_PARAM_SRC_OFFSET) % AR_SLOT_SIZE
}

unsafe fn src_commit_write(hw: *mut hw, idx: u32, blk: *mut c_void) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;
    let mut i: i32;

    if dirty16_get(&(*ctl).dirty.data, SRC_DIRTY_CZBFS) {
        /* Clear Z-Buffer registers */
        i = 0;
        while i < 8 {
            hw_write_20kx(hw, SRC_UPZ + idx * 0x100 + i as u32 * 0x4, 0);
            i += 1;
        }

        i = 0;
        while i < 4 {
            hw_write_20kx(hw, SRC_DN0Z + idx * 0x100 + i as u32 * 0x4, 0);
            i += 1;
        }

        i = 0;
        while i < 8 {
            hw_write_20kx(hw, SRC_DN1Z + idx * 0x100 + i as u32 * 0x4, 0);
            i += 1;
        }

        dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CZBFS, false);
    }
    if dirty16_get(&(*ctl).dirty.data, SRC_DIRTY_MPR) {
        /* Take the parameter mixer resource in the same group as that
         * the idx src is in for simplicity. Unlike src, all conjugate
         * parameter mixer resources must be programmed for
         * corresponding conjugate src resources. */
        let pm_idx = src_param_pitch_mixer(idx);
        hw_write_20kx(hw, MIXER_PRING_LO_HI + 4 * pm_idx, (*ctl).mpr);
        hw_write_20kx(hw, MIXER_PMOPLO + 8 * pm_idx, 0x3);
        hw_write_20kx(hw, MIXER_PMOPHI + 8 * pm_idx, 0x0);
        dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_MPR, false);
    }
    if dirty16_get(&(*ctl).dirty.data, SRC_DIRTY_SA) {
        hw_write_20kx(hw, SRC_SA + idx * 0x100, (*ctl).sa);
        dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_SA, false);
    }
    if dirty16_get(&(*ctl).dirty.data, SRC_DIRTY_LA) {
        hw_write_20kx(hw, SRC_LA + idx * 0x100, (*ctl).la);
        dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_LA, false);
    }
    if dirty16_get(&(*ctl).dirty.data, SRC_DIRTY_CA) {
        hw_write_20kx(hw, SRC_CA + idx * 0x100, (*ctl).ca);
        dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CA, false);
    }

    /* Write srccf register */
    hw_write_20kx(hw, SRC_CF + idx * 0x100, 0x0);

    if dirty16_get(&(*ctl).dirty.data, SRC_DIRTY_CCR) {
        hw_write_20kx(hw, SRC_CCR + idx * 0x100, (*ctl).ccr);
        dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CCR, false);
    }
    if dirty16_get(&(*ctl).dirty.data, SRC_DIRTY_CTL) {
        hw_write_20kx(hw, SRC_CTL + idx * 0x100, (*ctl).ctl);
        dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CTL, false);
    }

    0
}

unsafe fn src_get_ca(hw: *mut hw, idx: u32, blk: *mut c_void) -> i32 {
    let ctl = blk as *mut src_rsc_ctrl_blk;

    (*ctl).ca = hw_read_20kx(hw, SRC_CA + idx * 0x100);
    dirty16_set(&mut (*ctl).dirty.data, SRC_DIRTY_CA, false);

    get_field((*ctl).ca, SRCCA_CA) as i32
}

unsafe fn src_get_dirty(blk: *mut c_void) -> u32 {
    (*(blk as *mut src_rsc_ctrl_blk)).dirty.data as u32
}

fn src_dirty_conj_mask() -> u32 {
    0x20
}

unsafe fn src_mgr_enbs_src(blk: *mut c_void, idx: u32) -> i32 {
    (*(blk as *mut src_mgr_ctrl_blk)).enbsa |= 0x1 << ((idx % 128) / 4);
    dirty16_set(&mut (*(blk as *mut src_mgr_ctrl_blk)).dirty.data, SRC_MGR_DIRTY_ENBSA, true);
    (*(blk as *mut src_mgr_ctrl_blk)).enb[(idx / 32) as usize] |= 0x1 << (idx % 32);
    0
}

unsafe fn src_mgr_enb_src(blk: *mut c_void, idx: u32) -> i32 {
    (*(blk as *mut src_mgr_ctrl_blk)).enb[(idx / 32) as usize] |= 0x1 << (idx % 32);
    (*(blk as *mut src_mgr_ctrl_blk)).dirty.data |= (0x1 << (idx / 32)) as u16;
    0
}

unsafe fn src_mgr_dsb_src(blk: *mut c_void, idx: u32) -> i32 {
    (*(blk as *mut src_mgr_ctrl_blk)).enb[(idx / 32) as usize] &= !(0x1 << (idx % 32));
    (*(blk as *mut src_mgr_ctrl_blk)).dirty.data |= (0x1 << (idx / 32)) as u16;
    0
}

unsafe fn src_mgr_commit_write(hw: *mut hw, blk: *mut c_void) -> i32 {
    let ctl = blk as *mut src_mgr_ctrl_blk;
    let mut i: i32;
    let mut ret: u32;

    if dirty16_get(&(*ctl).dirty.data, SRC_MGR_DIRTY_ENBSA) {
        loop {
            ret = hw_read_20kx(hw, SRC_ENBSTAT);
            if (ret & 0x1) == 0 {
                break;
            }
        }
        hw_write_20kx(hw, SRC_ENBSA, (*ctl).enbsa);
        dirty16_set(&mut (*ctl).dirty.data, SRC_MGR_DIRTY_ENBSA, false);
    }
    i = 0;
    while i < 8 {
        if ((*ctl).dirty.data & ((0x1 << i) as u16)) != 0 {
            hw_write_20kx(hw, SRC_ENB + (i as u32 * 0x100), (*ctl).enb[i as usize]);
            (*ctl).dirty.data &= !((0x1 << i) as u16);
        }
        i += 1;
    }

    0
}

unsafe fn src_mgr_get_ctrl_blk(rblk: *mut *mut c_void) -> i32 {
    let blk: *mut src_mgr_ctrl_blk;

    *rblk = ptr::null_mut();
    blk = kzalloc_obj::<src_mgr_ctrl_blk>() as *mut src_mgr_ctrl_blk;
    if blk.is_null() {
        return -ENOMEM;
    }

    *rblk = blk as *mut c_void;

    0
}

unsafe fn src_mgr_put_ctrl_blk(blk: *mut c_void) -> i32 {
    kfree(blk);

    0
}

unsafe fn srcimp_mgr_get_ctrl_blk(rblk: *mut *mut c_void) -> i32 {
    let blk: *mut srcimp_mgr_ctrl_blk;

    *rblk = ptr::null_mut();
    blk = kzalloc_obj::<srcimp_mgr_ctrl_blk>() as *mut srcimp_mgr_ctrl_blk;
    if blk.is_null() {
        return -ENOMEM;
    }

    *rblk = blk as *mut c_void;

    0
}

unsafe fn srcimp_mgr_put_ctrl_blk(blk: *mut c_void) -> i32 {
    kfree(blk);

    0
}

unsafe fn srcimp_mgr_set_imaparc(blk: *mut c_void, slot: u32) -> i32 {
    let ctl = blk as *mut srcimp_mgr_ctrl_blk;

    set_field(&mut (*ctl).srcimap.srcaim, SRCAIM_ARC, slot);
    dirty16_set(&mut (*ctl).dirty.data, SRCIMP_MGR_DIRTY_SRCIMAP, true);
    0
}

unsafe fn srcimp_mgr_set_imapuser(blk: *mut c_void, user: u32) -> i32 {
    let ctl = blk as *mut srcimp_mgr_ctrl_blk;

    set_field(&mut (*ctl).srcimap.srcaim, SRCAIM_SRC, user);
    dirty16_set(&mut (*ctl).dirty.data, SRCIMP_MGR_DIRTY_SRCIMAP, true);
    0
}

unsafe fn srcimp_mgr_set_imapnxt(blk: *mut c_void, next: u32) -> i32 {
    let ctl = blk as *mut srcimp_mgr_ctrl_blk;

    set_field(&mut (*ctl).srcimap.srcaim, SRCAIM_NXT, next);
    dirty16_set(&mut (*ctl).dirty.data, SRCIMP_MGR_DIRTY_SRCIMAP, true);
    0
}

unsafe fn srcimp_mgr_set_imapaddr(blk: *mut c_void, addr: u32) -> i32 {
    (*(blk as *mut srcimp_mgr_ctrl_blk)).srcimap.idx = addr;
    dirty16_set(&mut (*(blk as *mut srcimp_mgr_ctrl_blk)).dirty.data, SRCIMP_MGR_DIRTY_SRCIMAP, true);
    0
}

unsafe fn srcimp_mgr_commit_write(hw: *mut hw, blk: *mut c_void) -> i32 {
    let ctl = blk as *mut srcimp_mgr_ctrl_blk;

    if dirty16_get(&(*ctl).dirty.data, SRCIMP_MGR_DIRTY_SRCIMAP) {
        hw_write_20kx(hw, SRC_IMAP + (*ctl).srcimap.idx * 0x100, (*ctl).srcimap.srcaim);
        dirty16_set(&mut (*ctl).dirty.data, SRCIMP_MGR_DIRTY_SRCIMAP, false);
    }

    0
}

/*
 * AMIXER control block definitions.
 */

const AMOPLO_M: u32 = 0x00000003;
const AMOPLO_IV: u32 = 0x00000004;
const AMOPLO_X: u32 = 0x0003FFF0;
const AMOPLO_Y: u32 = 0xFFFC0000;

const AMOPHI_SADR: u32 = 0x000000FF;
const AMOPHI_SE: u32 = 0x80000000;

/* AMIXER resource register dirty flags */
#[repr(C)]
pub union amixer_dirty {
    pub data: u16,
}

const AMIXER_DIRTY_AMOPLO: u16 = 1 << 0;
const AMIXER_DIRTY_AMOPHI: u16 = 1 << 1;

/* AMIXER resource control block */
#[repr(C)]
pub struct amixer_rsc_ctrl_blk {
    pub amoplo: u32,
    pub amophi: u32,
    pub dirty: amixer_dirty,
}

unsafe fn amixer_set_mode(blk: *mut c_void, mode: u32) -> i32 {
    let ctl = blk as *mut amixer_rsc_ctrl_blk;

    set_field(&mut (*ctl).amoplo, AMOPLO_M, mode);
    dirty16_set(&mut (*ctl).dirty.data, AMIXER_DIRTY_AMOPLO, true);
    0
}

unsafe fn amixer_set_iv(blk: *mut c_void, iv: u32) -> i32 {
    let ctl = blk as *mut amixer_rsc_ctrl_blk;

    set_field(&mut (*ctl).amoplo, AMOPLO_IV, iv);
    dirty16_set(&mut (*ctl).dirty.data, AMIXER_DIRTY_AMOPLO, true);
    0
}

unsafe fn amixer_set_x(blk: *mut c_void, x: u32) -> i32 {
    let ctl = blk as *mut amixer_rsc_ctrl_blk;

    set_field(&mut (*ctl).amoplo, AMOPLO_X, x);
    dirty16_set(&mut (*ctl).dirty.data, AMIXER_DIRTY_AMOPLO, true);
    0
}

unsafe fn amixer_set_y(blk: *mut c_void, y: u32) -> i32 {
    let ctl = blk as *mut amixer_rsc_ctrl_blk;

    set_field(&mut (*ctl).amoplo, AMOPLO_Y, y);
    dirty16_set(&mut (*ctl).dirty.data, AMIXER_DIRTY_AMOPLO, true);
    0
}

unsafe fn amixer_set_sadr(blk: *mut c_void, sadr: u32) -> i32 {
    let ctl = blk as *mut amixer_rsc_ctrl_blk;

    set_field(&mut (*ctl).amophi, AMOPHI_SADR, sadr);
    dirty16_set(&mut (*ctl).dirty.data, AMIXER_DIRTY_AMOPHI, true);
    0
}

unsafe fn amixer_set_se(blk: *mut c_void, se: u32) -> i32 {
    let ctl = blk as *mut amixer_rsc_ctrl_blk;

    set_field(&mut (*ctl).amophi, AMOPHI_SE, se);
    dirty16_set(&mut (*ctl).dirty.data, AMIXER_DIRTY_AMOPHI, true);
    0
}

unsafe fn amixer_set_dirty(blk: *mut c_void, flags: u32) -> i32 {
    (*(blk as *mut amixer_rsc_ctrl_blk)).dirty.data = (flags & 0xffff) as u16;
    0
}

unsafe fn amixer_set_dirty_all(blk: *mut c_void) -> i32 {
    (*(blk as *mut amixer_rsc_ctrl_blk)).dirty.data = !(0x0u16);
    0
}

unsafe fn amixer_commit_write(hw: *mut hw, idx: u32, blk: *mut c_void) -> i32 {
    let ctl = blk as *mut amixer_rsc_ctrl_blk;

    if dirty16_get(&(*ctl).dirty.data, AMIXER_DIRTY_AMOPLO)
        || dirty16_get(&(*ctl).dirty.data, AMIXER_DIRTY_AMOPHI)
    {
        hw_write_20kx(hw, MIXER_AMOPLO + idx * 8, (*ctl).amoplo);
        dirty16_set(&mut (*ctl).dirty.data, AMIXER_DIRTY_AMOPLO, false);
        hw_write_20kx(hw, MIXER_AMOPHI + idx * 8, (*ctl).amophi);
        dirty16_set(&mut (*ctl).dirty.data, AMIXER_DIRTY_AMOPHI, false);
    }

    0
}

unsafe fn amixer_get_y(blk: *mut c_void) -> i32 {
    let ctl = blk as *mut amixer_rsc_ctrl_blk;

    get_field((*ctl).amoplo, AMOPLO_Y) as i32
}

unsafe fn amixer_get_dirty(blk: *mut c_void) -> u32 {
    (*(blk as *mut amixer_rsc_ctrl_blk)).dirty.data as u32
}

unsafe fn amixer_rsc_get_ctrl_blk(rblk: *mut *mut c_void) -> i32 {
    let blk: *mut amixer_rsc_ctrl_blk;

    *rblk = ptr::null_mut();
    blk = kzalloc_obj::<amixer_rsc_ctrl_blk>() as *mut amixer_rsc_ctrl_blk;
    if blk.is_null() {
        return -ENOMEM;
    }

    *rblk = blk as *mut c_void;

    0
}

unsafe fn amixer_rsc_put_ctrl_blk(blk: *mut c_void) -> i32 {
    kfree(blk);

    0
}

unsafe fn amixer_mgr_get_ctrl_blk(rblk: *mut *mut c_void) -> i32 {
    *rblk = ptr::null_mut();

    0
}

unsafe fn amixer_mgr_put_ctrl_blk(_blk: *mut c_void) -> i32 {
    0
}

/*
 * DAIO control block definitions.
 */

/* Receiver Sample Rate Tracker Control register */
const SRTCTL_SRCO: u32 = 0x000000FF;
const SRTCTL_SRCM: u32 = 0x0000FF00;
const SRTCTL_RSR: u32 = 0x00030000;
const SRTCTL_DRAT: u32 = 0x00300000;
const SRTCTL_EC: u32 = 0x01000000;
const SRTCTL_ET: u32 = 0x10000000;

/* DAIO Receiver register dirty flags */
#[repr(C)]
pub union dai_dirty {
    pub data: u16,
}

const DAI_DIRTY_SRT: u16 = 1 << 0;

/* DAIO Receiver control block */
#[repr(C)]
pub struct dai_ctrl_blk {
    pub srt: u32,
    pub dirty: dai_dirty,
}

/* Audio Input Mapper RAM */
const AIM_ARC: u32 = 0x00000FFF;
const AIM_NXT: u32 = 0x007F0000;

#[repr(C)]
pub struct daoimap {
    pub aim: u32,
    pub idx: u32,
}

/* Audio Transmitter Control and Status register */
const ATXCTL_EN: u32 = 0x00000001;
const ATXCTL_MODE: u32 = 0x00000010;
const ATXCTL_CD: u32 = 0x00000020;
const ATXCTL_RAW: u32 = 0x00000100;
const ATXCTL_MT: u32 = 0x00000200;
const ATXCTL_NUC: u32 = 0x00003000;
const ATXCTL_BEN: u32 = 0x00010000;
const ATXCTL_BMUX: u32 = 0x00700000;
const ATXCTL_B24: u32 = 0x01000000;
const ATXCTL_CPF: u32 = 0x02000000;
const ATXCTL_RIV: u32 = 0x10000000;
const ATXCTL_LIV: u32 = 0x20000000;
const ATXCTL_RSAT: u32 = 0x40000000;
const ATXCTL_LSAT: u32 = 0x80000000;

/* XDIF Transmitter register dirty flags */
#[repr(C)]
pub union dao_dirty {
    pub data: u16,
}

const DAO_DIRTY_ATXCSL: u16 = 1 << 0;

/* XDIF Transmitter control block */
#[repr(C)]
pub struct dao_ctrl_blk {
    /* XDIF Transmitter Channel Status Low Register */
    pub atxcsl: u32,
    pub dirty: dao_dirty,
}

/* Audio Receiver Control register */
const ARXCTL_EN: u32 = 0x00000001;

/* DAIO manager register dirty flags */
#[repr(C)]
pub union daio_mgr_dirty {
    pub data: u32,
}

const DAIO_MGR_DIRTY_ATXCTL_SHIFT: u32 = 0;
const DAIO_MGR_DIRTY_ARXCTL_SHIFT: u32 = 8;
const DAIO_MGR_DIRTY_DAOIMAP: u32 = 1 << 16;

/* DAIO manager control block */
#[repr(C)]
pub struct daio_mgr_ctrl_blk {
    pub daoimap: daoimap,
    pub txctl: [u32; 8],
    pub rxctl: [u32; 8],
    pub dirty: daio_mgr_dirty,
}

unsafe fn dai_srt_set_srco(blk: *mut c_void, src: u32) -> i32 {
    let ctl = blk as *mut dai_ctrl_blk;

    set_field(&mut (*ctl).srt, SRTCTL_SRCO, src);
    dirty16_set(&mut (*ctl).dirty.data, DAI_DIRTY_SRT, true);
    0
}

unsafe fn dai_srt_set_srcm(blk: *mut c_void, src: u32) -> i32 {
    let ctl = blk as *mut dai_ctrl_blk;

    set_field(&mut (*ctl).srt, SRTCTL_SRCM, src);
    dirty16_set(&mut (*ctl).dirty.data, DAI_DIRTY_SRT, true);
    0
}

unsafe fn dai_srt_set_rsr(blk: *mut c_void, rsr: u32) -> i32 {
    let ctl = blk as *mut dai_ctrl_blk;

    set_field(&mut (*ctl).srt, SRTCTL_RSR, rsr);
    dirty16_set(&mut (*ctl).dirty.data, DAI_DIRTY_SRT, true);
    0
}

unsafe fn dai_srt_set_drat(blk: *mut c_void, drat: u32) -> i32 {
    let ctl = blk as *mut dai_ctrl_blk;

    set_field(&mut (*ctl).srt, SRTCTL_DRAT, drat);
    dirty16_set(&mut (*ctl).dirty.data, DAI_DIRTY_SRT, true);
    0
}

unsafe fn dai_srt_set_ec(blk: *mut c_void, ec: u32) -> i32 {
    let ctl = blk as *mut dai_ctrl_blk;

    set_field(&mut (*ctl).srt, SRTCTL_EC, if ec != 0 { 1 } else { 0 });
    dirty16_set(&mut (*ctl).dirty.data, DAI_DIRTY_SRT, true);
    0
}

unsafe fn dai_srt_set_et(blk: *mut c_void, et: u32) -> i32 {
    let ctl = blk as *mut dai_ctrl_blk;

    set_field(&mut (*ctl).srt, SRTCTL_ET, if et != 0 { 1 } else { 0 });
    dirty16_set(&mut (*ctl).dirty.data, DAI_DIRTY_SRT, true);
    0
}

unsafe fn dai_commit_write(hw: *mut hw, idx: u32, blk: *mut c_void) -> i32 {
    let ctl = blk as *mut dai_ctrl_blk;

    if dirty16_get(&(*ctl).dirty.data, DAI_DIRTY_SRT) {
        hw_write_20kx(hw, AUDIO_IO_RX_SRT_CTL + 0x40 * idx, (*ctl).srt);
        dirty16_set(&mut (*ctl).dirty.data, DAI_DIRTY_SRT, false);
    }

    0
}

unsafe fn dai_get_ctrl_blk(rblk: *mut *mut c_void) -> i32 {
    let blk: *mut dai_ctrl_blk;

    *rblk = ptr::null_mut();
    blk = kzalloc_obj::<dai_ctrl_blk>() as *mut dai_ctrl_blk;
    if blk.is_null() {
        return -ENOMEM;
    }

    *rblk = blk as *mut c_void;

    0
}

unsafe fn dai_put_ctrl_blk(blk: *mut c_void) -> i32 {
    kfree(blk);

    0
}

unsafe fn dao_set_spos(blk: *mut c_void, spos: u32) -> i32 {
    (*(blk as *mut dao_ctrl_blk)).atxcsl = spos;
    dirty16_set(&mut (*(blk as *mut dao_ctrl_blk)).dirty.data, DAO_DIRTY_ATXCSL, true);
    0
}

unsafe fn dao_commit_write(hw: *mut hw, idx: u32, blk: *mut c_void) -> i32 {
    let ctl = blk as *mut dao_ctrl_blk;

    if dirty16_get(&(*ctl).dirty.data, DAO_DIRTY_ATXCSL) {
        if (idx < 4) && ((*hw).model != CTOK0010 || idx < 3) {
            /* S/PDIF SPOSx */
            hw_write_20kx(hw, AUDIO_IO_TX_CSTAT_L + 0x40 * idx, (*ctl).atxcsl);
        }
        dirty16_set(&mut (*ctl).dirty.data, DAO_DIRTY_ATXCSL, false);
    }

    0
}

unsafe fn dao_get_spos(blk: *mut c_void, spos: *mut u32) -> i32 {
    *spos = (*(blk as *mut dao_ctrl_blk)).atxcsl;
    0
}

unsafe fn dao_get_ctrl_blk(rblk: *mut *mut c_void) -> i32 {
    let blk: *mut dao_ctrl_blk;

    *rblk = ptr::null_mut();
    blk = kzalloc_obj::<dao_ctrl_blk>() as *mut dao_ctrl_blk;
    if blk.is_null() {
        return -ENOMEM;
    }

    *rblk = blk as *mut c_void;

    0
}

unsafe fn dao_put_ctrl_blk(blk: *mut c_void) -> i32 {
    kfree(blk);

    0
}

unsafe fn daio_mgr_enb_dai(blk: *mut c_void, idx: u32) -> i32 {
    let ctl = blk as *mut daio_mgr_ctrl_blk;

    set_field(&mut (*ctl).rxctl[idx as usize], ARXCTL_EN, 1);
    (*ctl).dirty.data |= 0x1 << (DAIO_MGR_DIRTY_ARXCTL_SHIFT + idx);
    0
}

unsafe fn daio_mgr_dsb_dai(blk: *mut c_void, idx: u32) -> i32 {
    let ctl = blk as *mut daio_mgr_ctrl_blk;

    set_field(&mut (*ctl).rxctl[idx as usize], ARXCTL_EN, 0);

    (*ctl).dirty.data |= 0x1 << (DAIO_MGR_DIRTY_ARXCTL_SHIFT + idx);
    0
}

unsafe fn daio_mgr_enb_dao(blk: *mut c_void, idx: u32) -> i32 {
    let ctl = blk as *mut daio_mgr_ctrl_blk;

    set_field(&mut (*ctl).txctl[idx as usize], ATXCTL_EN, 1);
    (*ctl).dirty.data |= 0x1 << (DAIO_MGR_DIRTY_ATXCTL_SHIFT + idx);
    0
}

unsafe fn daio_mgr_dsb_dao(blk: *mut c_void, idx: u32) -> i32 {
    let ctl = blk as *mut daio_mgr_ctrl_blk;

    set_field(&mut (*ctl).txctl[idx as usize], ATXCTL_EN, 0);
    (*ctl).dirty.data |= 0x1 << (DAIO_MGR_DIRTY_ATXCTL_SHIFT + idx);
    0
}

unsafe fn daio_mgr_dao_init(hw: *mut hw, blk: *mut c_void, idx: u32, conf: u32) -> i32 {
    let ctl = blk as *mut daio_mgr_ctrl_blk;

    /* Port 3 is dedicated to RCA on SE-300PCIE */
    if (idx < 4) && ((*hw).model != CTOK0010 || idx < 3) {
        /* S/PDIF output */
        match conf & 0xf {
            1 => set_field(&mut (*ctl).txctl[idx as usize], ATXCTL_NUC, 0),
            2 => set_field(&mut (*ctl).txctl[idx as usize], ATXCTL_NUC, 1),
            4 => set_field(&mut (*ctl).txctl[idx as usize], ATXCTL_NUC, 2),
            8 => set_field(&mut (*ctl).txctl[idx as usize], ATXCTL_NUC, 3),
            _ => {}
        }
        /* CDIF */
        set_field(&mut (*ctl).txctl[idx as usize], ATXCTL_CD, if (conf & 0x7) == 0 { 1 } else { 0 });
        /* Non-audio */
        set_field(&mut (*ctl).txctl[idx as usize], ATXCTL_LIV, (conf >> 4) & 0x1);
        /* Non-audio */
        set_field(&mut (*ctl).txctl[idx as usize], ATXCTL_RIV, (conf >> 4) & 0x1);
        set_field(&mut (*ctl).txctl[idx as usize], ATXCTL_RAW, if ((conf >> 3) & 0x1) != 0 { 0 } else { 0 });
        (*ctl).dirty.data |= 0x1 << (DAIO_MGR_DIRTY_ATXCTL_SHIFT + idx);
    } else {
        /* I2S output */
        /*idx %= 4; */
    }
    0
}

unsafe fn daio_mgr_set_imaparc(blk: *mut c_void, slot: u32) -> i32 {
    let ctl = blk as *mut daio_mgr_ctrl_blk;

    set_field(&mut (*ctl).daoimap.aim, AIM_ARC, slot);
    dirty32_set(&mut (*ctl).dirty.data, DAIO_MGR_DIRTY_DAOIMAP, true);
    0
}

unsafe fn daio_mgr_set_imapnxt(blk: *mut c_void, next: u32) -> i32 {
    let ctl = blk as *mut daio_mgr_ctrl_blk;

    set_field(&mut (*ctl).daoimap.aim, AIM_NXT, next);
    dirty32_set(&mut (*ctl).dirty.data, DAIO_MGR_DIRTY_DAOIMAP, true);
    0
}

unsafe fn daio_mgr_set_imapaddr(blk: *mut c_void, addr: u32) -> i32 {
    (*(blk as *mut daio_mgr_ctrl_blk)).daoimap.idx = addr;
    dirty32_set(&mut (*(blk as *mut daio_mgr_ctrl_blk)).dirty.data, DAIO_MGR_DIRTY_DAOIMAP, true);
    0
}

unsafe fn daio_mgr_commit_write(hw: *mut hw, blk: *mut c_void) -> i32 {
    let ctl = blk as *mut daio_mgr_ctrl_blk;
    let mut data: u32;
    let mut i: i32;

    i = 0;
    while i < 8 {
        if ((*ctl).dirty.data & (0x1 << (DAIO_MGR_DIRTY_ATXCTL_SHIFT + i as u32))) != 0 {
            data = (*ctl).txctl[i as usize];
            hw_write_20kx(hw, AUDIO_IO_TX_CTL + (0x40 * i as u32), data);
            (*ctl).dirty.data &= !(0x1 << (DAIO_MGR_DIRTY_ATXCTL_SHIFT + i as u32));
            mdelay(1);
        }
        if ((*ctl).dirty.data & (0x1 << (DAIO_MGR_DIRTY_ARXCTL_SHIFT + i as u32))) != 0 {
            data = (*ctl).rxctl[i as usize];
            hw_write_20kx(hw, AUDIO_IO_RX_CTL + (0x40 * i as u32), data);
            (*ctl).dirty.data &= !(0x1 << (DAIO_MGR_DIRTY_ARXCTL_SHIFT + i as u32));
            mdelay(1);
        }
        i += 1;
    }
    if dirty32_get(&(*ctl).dirty.data, DAIO_MGR_DIRTY_DAOIMAP) {
        hw_write_20kx(hw, AUDIO_IO_AIM + (*ctl).daoimap.idx * 4, (*ctl).daoimap.aim);
        dirty32_set(&mut (*ctl).dirty.data, DAIO_MGR_DIRTY_DAOIMAP, false);
    }

    0
}

unsafe fn daio_mgr_get_ctrl_blk(hw: *mut hw, rblk: *mut *mut c_void) -> i32 {
    let blk: *mut daio_mgr_ctrl_blk;
    let mut i: i32;

    *rblk = ptr::null_mut();
    blk = kzalloc_obj::<daio_mgr_ctrl_blk>() as *mut daio_mgr_ctrl_blk;
    if blk.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < 8 {
        (*blk).txctl[i as usize] = hw_read_20kx(hw, AUDIO_IO_TX_CTL + (0x40 * i as u32));
        (*blk).rxctl[i as usize] = hw_read_20kx(hw, AUDIO_IO_RX_CTL + (0x40 * i as u32));
        i += 1;
    }

    *rblk = blk as *mut c_void;

    0
}

unsafe fn daio_mgr_put_ctrl_blk(blk: *mut c_void) -> i32 {
    kfree(blk);

    0
}

/* Timer interrupt */
unsafe fn set_timer_irq(hw: *mut hw, enable: i32) -> i32 {
    hw_write_20kx(hw, GIE, if enable != 0 { IT_INT } else { 0 });
    0
}

unsafe fn set_timer_tick(hw: *mut hw, mut ticks: u32) -> i32 {
    if ticks != 0 {
        ticks |= TIMR_IE | TIMR_IP;
    }
    hw_write_20kx(hw, TIMR, ticks);
    0
}

unsafe fn get_wc(hw: *mut hw) -> u32 {
    hw_read_20kx(hw, WC)
}

/* Card hardware initialization block */
#[repr(C)]
pub struct dac_conf {
    pub msr: u32, /* master sample rate in rsrs */
}

#[repr(C)]
pub struct adc_conf {
    pub msr: u32,     /* master sample rate in rsrs */
    pub input: u8,    /* the input source of ADC */
    pub mic20db: u8,  /* boost mic by 20db if input is microphone */
}

#[repr(C)]
pub struct daio_conf {
    pub msr: u32, /* master sample rate in rsrs */
}

#[repr(C)]
pub struct trn_conf {
    pub vm_pgt_phys: usize,
}

unsafe fn hw_daio_init(hw: *mut hw, info: *const daio_conf) -> i32 {
    let mut data: u32;
    let mut i: i32;

    /* Program I2S with proper sample rate and enable the correct I2S
     * channel. ED(0/8/16/24): Enable all I2S/I2X master clock output */
    if 1 == (*info).msr {
        hw_write_20kx(hw, AUDIO_IO_MCLK, 0x01010101);
        hw_write_20kx(hw, AUDIO_IO_TX_BLRCLK, 0x01010101);
        hw_write_20kx(hw, AUDIO_IO_RX_BLRCLK, 0);
    } else if 2 == (*info).msr {
        if (*hw).model != CTSB1270 {
            hw_write_20kx(hw, AUDIO_IO_MCLK, 0x11111111);
        } else {
            /* PCM4220 on Titanium HD is different. */
            hw_write_20kx(hw, AUDIO_IO_MCLK, 0x11011111);
        }
        /* Specify all playing 96khz
         * EA [0]	- Enabled
         * RTA [4:5]	- 96kHz
         * EB [8]	- Enabled
         * RTB [12:13]	- 96kHz
         * EC [16]	- Enabled
         * RTC [20:21]	- 96kHz
         * ED [24]	- Enabled
         * RTD [28:29]	- 96kHz */
        hw_write_20kx(hw, AUDIO_IO_TX_BLRCLK, 0x11111111);
        hw_write_20kx(hw, AUDIO_IO_RX_BLRCLK, 0);
    } else if (4 == (*info).msr) && ((*hw).model == CTSB1270) {
        hw_write_20kx(hw, AUDIO_IO_MCLK, 0x21011111);
        hw_write_20kx(hw, AUDIO_IO_TX_BLRCLK, 0x21212121);
        hw_write_20kx(hw, AUDIO_IO_RX_BLRCLK, 0);
    } else if (4 == (*info).msr) && ((*hw).model == CTOK0010) {
        hw_write_20kx(hw, AUDIO_IO_MCLK, 0x21212121);
        hw_write_20kx(hw, AUDIO_IO_TX_BLRCLK, 0x21212121);
        hw_write_20kx(hw, AUDIO_IO_RX_BLRCLK, 0);
    } else {
        dev_alert((*(*hw).card).dev, c_str!("ERROR!!! Invalid sampling rate!!!\n"));
        return -EINVAL;
    }

    i = 0;
    while i < 8 {
        /* Port 3 is configured as I2S on SE-300PCIE */
        if (i < 4) && ((*hw).model != CTOK0010 || i < 3) {
            /* This comment looks wrong since loop is over 4  */
            /* channels and emu20k2 supports 4 spdif IOs.     */
            /* 1st 3 channels are SPDIFs (SB0960) */
            if i == 3 {
                data = 0x1001001;
            } else {
                data = 0x1000001;
            }

            hw_write_20kx(hw, AUDIO_IO_TX_CTL + (0x40 * i as u32), data);
            hw_write_20kx(hw, AUDIO_IO_RX_CTL + (0x40 * i as u32), data);

            /* Initialize the SPDIF Out Channel status registers.
             * The value specified here is based on the typical
             * values provided in the specification, namely: Clock
             * Accuracy of 1000ppm, Sample Rate of 48KHz,
             * unspecified source number, Generation status = 1,
             * Category code = 0x12 (Digital Signal Mixer),
             * Mode = 0, Emph = 0, Copy Permitted, AN = 0
             * (indicating that we're transmitting digital audio,
             * and the Professional Use bit is 0. */

            hw_write_20kx(hw, AUDIO_IO_TX_CSTAT_L + (0x40 * i as u32), 0x02109204); /* Default to 48kHz */

            hw_write_20kx(hw, AUDIO_IO_TX_CSTAT_H + (0x40 * i as u32), 0x0B);
        } else {
            /* Again, loop is over 4 channels not 5. */
            /* Next 5 channels are I2S (SB0960) */
            data = 0x11;
            hw_write_20kx(hw, AUDIO_IO_RX_CTL + (0x40 * i as u32), data);
            if 2 == (*info).msr {
                /* Four channels per sample period */
                data |= 0x1000;
            } else if 4 == (*info).msr {
                /* FIXME: check this against the chip spec */
                data |= 0x2000;
            }
            hw_write_20kx(hw, AUDIO_IO_TX_CTL + (0x40 * i as u32), data);
        }
        i += 1;
    }

    0
}

/* TRANSPORT operations */
unsafe fn hw_trn_init(hw: *mut hw, info: *const trn_conf) -> i32 {
    let mut vmctl: u32;
    let mut data: u32;
    let ptp_phys_low: u32;
    let ptp_phys_high: u32;
    let mut i: i32;

    /* Set up device page table */
    if !0usize == (*info).vm_pgt_phys {
        dev_alert((*(*hw).card).dev, c_str!("Wrong device page table page address!!!\n"));
        return -1;
    }

    vmctl = 0x80000C0F;  /* 32-bit, 4k-size page */
    ptp_phys_low = (*info).vm_pgt_phys as u32;
    ptp_phys_high = upper_32_bits((*info).vm_pgt_phys);
    if size_of::<*mut c_void>() == 8 {
        /* 64bit address */
        vmctl |= 3 << 8;
    }
    /* Write page table physical address to all PTPAL registers */
    i = 0;
    while i < 64 {
        hw_write_20kx(hw, VMEM_PTPAL + (16 * i as u32), ptp_phys_low);
        hw_write_20kx(hw, VMEM_PTPAH + (16 * i as u32), ptp_phys_high);
        i += 1;
    }
    /* Enable virtual memory transfer */
    hw_write_20kx(hw, VMEM_CTL, vmctl);
    /* Enable transport bus master and queueing of request */
    hw_write_20kx(hw, TRANSPORT_CTL, 0x03);
    hw_write_20kx(hw, TRANSPORT_INT, 0x200c01);
    /* Enable transport ring */
    data = hw_read_20kx(hw, TRANSPORT_ENB);
    hw_write_20kx(hw, TRANSPORT_ENB, data | 0x03);

    0
}

/* Card initialization */
const GCTL_AIE: u32 = 0x00000001;
const GCTL_UAA: u32 = 0x00000002;
const GCTL_DPC: u32 = 0x00000004;
const GCTL_DBP: u32 = 0x00000008;
const GCTL_ABP: u32 = 0x00000010;
const GCTL_TBP: u32 = 0x00000020;
const GCTL_SBP: u32 = 0x00000040;
const GCTL_FBP: u32 = 0x00000080;
const GCTL_ME: u32 = 0x00000100;
const GCTL_AID: u32 = 0x00001000;

const PLLCTL_SRC: u32 = 0x00000007;
const PLLCTL_SPE: u32 = 0x00000008;
const PLLCTL_RD: u32 = 0x000000F0;
const PLLCTL_FD: u32 = 0x0001FF00;
const PLLCTL_OD: u32 = 0x00060000;
const PLLCTL_B: u32 = 0x00080000;
const PLLCTL_AS: u32 = 0x00100000;
const PLLCTL_LF: u32 = 0x03E00000;
const PLLCTL_SPS: u32 = 0x1C000000;
const PLLCTL_AD: u32 = 0x60000000;

const PLLSTAT_CCS: u32 = 0x00000007;
const PLLSTAT_SPL: u32 = 0x00000008;
const PLLSTAT_CRD: u32 = 0x000000F0;
const PLLSTAT_CFD: u32 = 0x0001FF00;
const PLLSTAT_SL: u32 = 0x00020000;
const PLLSTAT_FAS: u32 = 0x00040000;
const PLLSTAT_B: u32 = 0x00080000;
const PLLSTAT_PD: u32 = 0x00100000;
const PLLSTAT_OCA: u32 = 0x00200000;
const PLLSTAT_NCA: u32 = 0x00400000;

unsafe fn hw_pll_init(hw: *mut hw, rsr: u32) -> i32 {
    let mut pllenb: u32;
    let mut pllctl: u32;
    let mut pllstat: u32;
    let mut i: i32;

    pllenb = 0xB;
    hw_write_20kx(hw, PLL_ENB, pllenb);
    pllctl = 0x20C00000;
    set_field(&mut pllctl, PLLCTL_B, 0);
    set_field(&mut pllctl, PLLCTL_FD, if 48000 == rsr { 16 - 4 } else { 147 - 4 });
    set_field(&mut pllctl, PLLCTL_RD, if 48000 == rsr { 1 - 1 } else { 10 - 1 });
    hw_write_20kx(hw, PLL_CTL, pllctl);
    msleep(40);

    pllctl = hw_read_20kx(hw, PLL_CTL);
    set_field(&mut pllctl, PLLCTL_FD, if 48000 == rsr { 16 - 2 } else { 147 - 2 });
    hw_write_20kx(hw, PLL_CTL, pllctl);
    msleep(40);

    i = 0;
    while i < 1000 {
        pllstat = hw_read_20kx(hw, PLL_STAT);
        if get_field(pllstat, PLLSTAT_PD) != 0 {
            i += 1;
            continue;
        }

        if get_field(pllstat, PLLSTAT_B) != get_field(pllctl, PLLCTL_B) {
            i += 1;
            continue;
        }

        if get_field(pllstat, PLLSTAT_CCS) != get_field(pllctl, PLLCTL_SRC) {
            i += 1;
            continue;
        }

        if get_field(pllstat, PLLSTAT_CRD) != get_field(pllctl, PLLCTL_RD) {
            i += 1;
            continue;
        }

        if get_field(pllstat, PLLSTAT_CFD) != get_field(pllctl, PLLCTL_FD) {
            i += 1;
            continue;
        }

        break;
    }
    if i >= 1000 {
        dev_alert((*(*hw).card).dev, c_str!("PLL initialization failed!!!\n"));
        return -EBUSY;
    }

    0
}

unsafe fn hw_auto_init(hw: *mut hw) -> i32 {
    let mut gctl: u32;
    let mut i: i32;

    gctl = hw_read_20kx(hw, GLOBAL_CNTL_GCTL);
    set_field(&mut gctl, GCTL_AIE, 0);
    hw_write_20kx(hw, GLOBAL_CNTL_GCTL, gctl);
    set_field(&mut gctl, GCTL_AIE, 1);
    hw_write_20kx(hw, GLOBAL_CNTL_GCTL, gctl);
    mdelay(10);
    i = 0;
    while i < 400000 {
        gctl = hw_read_20kx(hw, GLOBAL_CNTL_GCTL);
        if get_field(gctl, GCTL_AID) != 0 {
            break;
        }
        i += 1;
    }
    if get_field(gctl, GCTL_AID) == 0 {
        dev_alert((*(*hw).card).dev, c_str!("Card Auto-init failed!!!\n"));
        return -EBUSY;
    }

    0
}

/* DAC operations */

const CS4382_MC1: u16 = 0x1;
const CS4382_MC2: u16 = 0x2;
const CS4382_MC3: u16 = 0x3;
const CS4382_FC: u16 = 0x4;
const CS4382_IC: u16 = 0x5;
const CS4382_XC1: u16 = 0x6;
const CS4382_VCA1: u16 = 0x7;
const CS4382_VCB1: u16 = 0x8;
const CS4382_XC2: u16 = 0x9;
const CS4382_VCA2: u16 = 0xA;
const CS4382_VCB2: u16 = 0xB;
const CS4382_XC3: u16 = 0xC;
const CS4382_VCA3: u16 = 0xD;
const CS4382_VCB3: u16 = 0xE;
const CS4382_XC4: u16 = 0xF;
const CS4382_VCA4: u16 = 0x10;
const CS4382_VCB4: u16 = 0x11;
const CS4382_CREV: u16 = 0x12;

/* I2C status */
const STATE_LOCKED: u32 = 0x00;
const STATE_UNLOCKED: u32 = 0xAA;
const DATA_READY: u32 = 0x800000;    /* Used with I2C_IF_STATUS */
const DATA_ABORT: u32 = 0x10000;     /* Used with I2C_IF_STATUS */

const I2C_STATUS_DCM: u32 = 0x00000001;
const I2C_STATUS_BC: u32 = 0x00000006;
const I2C_STATUS_APD: u32 = 0x00000008;
const I2C_STATUS_AB: u32 = 0x00010000;
const I2C_STATUS_DR: u32 = 0x00800000;

const I2C_ADDRESS_PTAD: u32 = 0x0000FFFF;
const I2C_ADDRESS_SLAD: u32 = 0x007F0000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regs_cs4382 {
    pub mode_control_1: u32,
    pub mode_control_2: u32,
    pub mode_control_3: u32,

    pub filter_control: u32,
    pub invert_control: u32,

    pub mix_control_P1: u32,
    pub vol_control_A1: u32,
    pub vol_control_B1: u32,

    pub mix_control_P2: u32,
    pub vol_control_A2: u32,
    pub vol_control_B2: u32,

    pub mix_control_P3: u32,
    pub vol_control_A3: u32,
    pub vol_control_B3: u32,

    pub mix_control_P4: u32,
    pub vol_control_A4: u32,
    pub vol_control_B4: u32,
}

unsafe fn hw20k2_i2c_unlock_full_access(hw: *mut hw) -> i32 {
    let UnlockKeySequence_FLASH_FULLACCESS_MODE: [u8; 2] = [0xB3, 0xD4];

    /* Send keys for forced BIOS mode */
    hw_write_20kx(hw, I2C_IF_WLOCK, UnlockKeySequence_FLASH_FULLACCESS_MODE[0] as u32);
    hw_write_20kx(hw, I2C_IF_WLOCK, UnlockKeySequence_FLASH_FULLACCESS_MODE[1] as u32);
    /* Check whether the chip is unlocked */
    if hw_read_20kx(hw, I2C_IF_WLOCK) == STATE_UNLOCKED {
        return 0;
    }

    -1
}

unsafe fn hw20k2_i2c_lock_chip(hw: *mut hw) -> i32 {
    /* Write twice */
    hw_write_20kx(hw, I2C_IF_WLOCK, STATE_LOCKED);
    hw_write_20kx(hw, I2C_IF_WLOCK, STATE_LOCKED);
    if hw_read_20kx(hw, I2C_IF_WLOCK) == STATE_LOCKED {
        return 0;
    }

    -1
}

unsafe fn hw20k2_i2c_init(hw: *mut hw, dev_id: u8, addr_size: u8, data_size: u8) -> i32 {
    let hw20k2 = hw as *mut hw20k2;
    let mut err: i32;
    let mut i2c_status: u32;
    let mut i2c_addr: u32;

    err = hw20k2_i2c_unlock_full_access(hw);
    if err < 0 {
        return err;
    }

    (*hw20k2).addr_size = addr_size;
    (*hw20k2).data_size = data_size;
    (*hw20k2).dev_id = dev_id;

    i2c_addr = 0;
    set_field(&mut i2c_addr, I2C_ADDRESS_SLAD, dev_id as u32);

    hw_write_20kx(hw, I2C_IF_ADDRESS, i2c_addr);

    i2c_status = hw_read_20kx(hw, I2C_IF_STATUS);

    set_field(&mut i2c_status, I2C_STATUS_DCM, 1); /* Direct control mode */

    hw_write_20kx(hw, I2C_IF_STATUS, i2c_status);

    0
}

unsafe fn hw20k2_i2c_uninit(hw: *mut hw) -> i32 {
    let mut i2c_status: u32;
    let mut i2c_addr: u32;

    i2c_addr = 0;
    set_field(&mut i2c_addr, I2C_ADDRESS_SLAD, 0x57); /* I2C id */

    hw_write_20kx(hw, I2C_IF_ADDRESS, i2c_addr);

    i2c_status = hw_read_20kx(hw, I2C_IF_STATUS);

    set_field(&mut i2c_status, I2C_STATUS_DCM, 0); /* I2C mode */

    hw_write_20kx(hw, I2C_IF_STATUS, i2c_status);

    hw20k2_i2c_lock_chip(hw)
}

unsafe fn hw20k2_i2c_wait_data_ready(hw: *mut hw) -> i32 {
    let mut i: i32 = 0x400000;
    let mut ret: u32;

    loop {
        ret = hw_read_20kx(hw, I2C_IF_STATUS);
        if (ret & DATA_READY) != 0 {
            break;
        }
        i -= 1;
        if i == 0 {
            break;
        }
    }

    i
}

unsafe fn hw20k2_i2c_read(hw: *mut hw, addr: u16, datap: *mut u32) -> i32 {
    let hw20k2 = hw as *mut hw20k2;
    let mut i2c_status: u32;

    i2c_status = hw_read_20kx(hw, I2C_IF_STATUS);
    set_field(
        &mut i2c_status,
        I2C_STATUS_BC,
        if 4 == (*hw20k2).addr_size { 0 } else { (*hw20k2).addr_size as u32 },
    );
    hw_write_20kx(hw, I2C_IF_STATUS, i2c_status);
    if hw20k2_i2c_wait_data_ready(hw) == 0 {
        return -1;
    }

    hw_write_20kx(hw, I2C_IF_WDATA, addr as u32);
    if hw20k2_i2c_wait_data_ready(hw) == 0 {
        return -1;
    }

    /* Force a read operation */
    hw_write_20kx(hw, I2C_IF_RDATA, 0);
    if hw20k2_i2c_wait_data_ready(hw) == 0 {
        return -1;
    }

    *datap = hw_read_20kx(hw, I2C_IF_RDATA);

    0
}

unsafe fn hw20k2_i2c_write(hw: *mut hw, addr: u16, data: u32) -> i32 {
    let hw20k2 = hw as *mut hw20k2;
    let i2c_data: u32 = (data << ((*hw20k2).addr_size as u32 * 8)) | addr as u32;
    let mut i2c_status: u32;

    i2c_status = hw_read_20kx(hw, I2C_IF_STATUS);

    set_field(
        &mut i2c_status,
        I2C_STATUS_BC,
        if 4 == ((*hw20k2).addr_size + (*hw20k2).data_size) {
            0
        } else {
            ((*hw20k2).addr_size + (*hw20k2).data_size) as u32
        },
    );

    hw_write_20kx(hw, I2C_IF_STATUS, i2c_status);
    hw20k2_i2c_wait_data_ready(hw);
    /* Dummy write to trigger the write operation */
    hw_write_20kx(hw, I2C_IF_WDATA, 0);
    hw20k2_i2c_wait_data_ready(hw);

    /* This is the real data */
    hw_write_20kx(hw, I2C_IF_WDATA, i2c_data);
    hw20k2_i2c_wait_data_ready(hw);

    0
}

unsafe fn hw_dac_stop(hw: *mut hw) {
    let mut data: u32;
    data = hw_read_20kx(hw, GPIO_DATA);
    data &= 0xFFFFFFFD;
    hw_write_20kx(hw, GPIO_DATA, data);
    usleep_range(10000, 11000);
}

unsafe fn hw_dac_start(hw: *mut hw) {
    let mut data: u32;
    data = hw_read_20kx(hw, GPIO_DATA);
    data |= 0x2;
    hw_write_20kx(hw, GPIO_DATA, data);
    msleep(50);
}

unsafe fn hw_dac_reset(hw: *mut hw) {
    hw_dac_stop(hw);
    hw_dac_start(hw);
}

unsafe fn hw_dac_init(hw: *mut hw, info: *const dac_conf) -> i32 {
    let mut err: i32;
    let mut data: u32;
    let mut i: i32;
    let mut cs_read: regs_cs4382 = core::mem::zeroed();
    let cs_def = regs_cs4382 {
        mode_control_1: 0x00000001, /* Mode Control 1 */
        mode_control_2: 0x00000000, /* Mode Control 2 */
        mode_control_3: 0x00000084, /* Mode Control 3 */
        filter_control: 0x00000000, /* Filter Control */
        invert_control: 0x00000000, /* Invert Control */
        mix_control_P1: 0x00000024, /* Mixing Control Pair 1 */
        vol_control_A1: 0x00000000, /* Vol Control A1 */
        vol_control_B1: 0x00000000, /* Vol Control B1 */
        mix_control_P2: 0x00000024, /* Mixing Control Pair 2 */
        vol_control_A2: 0x00000000, /* Vol Control A2 */
        vol_control_B2: 0x00000000, /* Vol Control B2 */
        mix_control_P3: 0x00000024, /* Mixing Control Pair 3 */
        vol_control_A3: 0x00000000, /* Vol Control A3 */
        vol_control_B3: 0x00000000, /* Vol Control B3 */
        mix_control_P4: 0x00000024, /* Mixing Control Pair 4 */
        vol_control_A4: 0x00000000, /* Vol Control A4 */
        vol_control_B4: 0x00000000, /* Vol Control B4 */
    };

    if (*hw).model == CTSB1270 {
        hw_dac_stop(hw);
        data = hw_read_20kx(hw, GPIO_DATA);
        data &= !0x0600;
        if 1 == (*info).msr {
            data |= 0x0000; /* Single Speed Mode 0-50kHz */
        } else if 2 == (*info).msr {
            data |= 0x0200; /* Double Speed Mode 50-100kHz */
        } else {
            data |= 0x0600; /* Quad Speed Mode 100-200kHz */
        }
        hw_write_20kx(hw, GPIO_DATA, data);
        hw_dac_start(hw);
        return 0;
    } else if (*hw).model == CTOK0010 {
        hw_dac_stop(hw);
        data = hw_read_20kx(hw, GPIO_DATA);
        data |= 0x1000;
        hw_write_20kx(hw, GPIO_DATA, data);
        hw_dac_start(hw);
        return 0;
    }

    /* Set DAC reset bit as output */
    data = hw_read_20kx(hw, GPIO_CTRL);
    data |= 0x02;
    hw_write_20kx(hw, GPIO_CTRL, data);

    err = hw20k2_i2c_init(hw, 0x18, 1, 1);
    if err < 0 {
        hw20k2_i2c_uninit(hw);
        return -1;
    }

    i = 0;
    while i < 2 {
        /* Reset DAC twice just in-case the chip
         * didn't initialized properly */
        hw_dac_reset(hw);
        hw_dac_reset(hw);

        if hw20k2_i2c_read(hw, CS4382_MC1, &mut cs_read.mode_control_1) != 0 {
            i += 1;
            continue;
        }
        if hw20k2_i2c_read(hw, CS4382_MC2, &mut cs_read.mode_control_2) != 0 {
            i += 1;
            continue;
        }
        if hw20k2_i2c_read(hw, CS4382_MC3, &mut cs_read.mode_control_3) != 0 {
            i += 1;
            continue;
        }
        if hw20k2_i2c_read(hw, CS4382_FC, &mut cs_read.filter_control) != 0 {
            i += 1;
            continue;
        }
        if hw20k2_i2c_read(hw, CS4382_IC, &mut cs_read.invert_control) != 0 {
            i += 1;
            continue;
        }
        if hw20k2_i2c_read(hw, CS4382_XC1, &mut cs_read.mix_control_P1) != 0 {
            i += 1;
            continue;
        }
        if hw20k2_i2c_read(hw, CS4382_VCA1, &mut cs_read.vol_control_A1) != 0 {
            i += 1;
            continue;
        }
        if hw20k2_i2c_read(hw, CS4382_VCB1, &mut cs_read.vol_control_B1) != 0 {
            i += 1;
            continue;
        }
        if hw20k2_i2c_read(hw, CS4382_XC2, &mut cs_read.mix_control_P2) != 0 {
            i += 1;
            continue;
        }
        if hw20k2_i2c_read(hw, CS4382_VCA2, &mut cs_read.vol_control_A2) != 0 {
            i += 1;
            continue;
        }
        if hw20k2_i2c_read(hw, CS4382_VCB2, &mut cs_read.vol_control_B2) != 0 {
            i += 1;
            continue;
        }
        if hw20k2_i2c_read(hw, CS4382_XC3, &mut cs_read.mix_control_P3) != 0 {
            i += 1;
            continue;
        }
        if hw20k2_i2c_read(hw, CS4382_VCA3, &mut cs_read.vol_control_A3) != 0 {
            i += 1;
            continue;
        }
        if hw20k2_i2c_read(hw, CS4382_VCB3, &mut cs_read.vol_control_B3) != 0 {
            i += 1;
            continue;
        }
        if hw20k2_i2c_read(hw, CS4382_XC4, &mut cs_read.mix_control_P4) != 0 {
            i += 1;
            continue;
        }
        if hw20k2_i2c_read(hw, CS4382_VCA4, &mut cs_read.vol_control_A4) != 0 {
            i += 1;
            continue;
        }
        if hw20k2_i2c_read(hw, CS4382_VCB4, &mut cs_read.vol_control_B4) != 0 {
            i += 1;
            continue;
        }

        if memcmp(
            &cs_read as *const _ as *const c_void,
            &cs_def as *const _ as *const c_void,
            size_of::<regs_cs4382>(),
        ) != 0
        {
            i += 1;
            continue;
        } else {
            break;
        }
    }

    if i >= 2 {
        hw20k2_i2c_uninit(hw);
        return -1;
    }

    /* Note: Every I2C write must have some delay.
     * This is not a requirement but the delay works here... */
    hw20k2_i2c_write(hw, CS4382_MC1, 0x80);
    hw20k2_i2c_write(hw, CS4382_MC2, 0x10);
    if 1 == (*info).msr {
        hw20k2_i2c_write(hw, CS4382_XC1, 0x24);
        hw20k2_i2c_write(hw, CS4382_XC2, 0x24);
        hw20k2_i2c_write(hw, CS4382_XC3, 0x24);
        hw20k2_i2c_write(hw, CS4382_XC4, 0x24);
    } else if 2 == (*info).msr {
        hw20k2_i2c_write(hw, CS4382_XC1, 0x25);
        hw20k2_i2c_write(hw, CS4382_XC2, 0x25);
        hw20k2_i2c_write(hw, CS4382_XC3, 0x25);
        hw20k2_i2c_write(hw, CS4382_XC4, 0x25);
    } else {
        hw20k2_i2c_write(hw, CS4382_XC1, 0x26);
        hw20k2_i2c_write(hw, CS4382_XC2, 0x26);
        hw20k2_i2c_write(hw, CS4382_XC3, 0x26);
        hw20k2_i2c_write(hw, CS4382_XC4, 0x26);
    }

    0
}

/* ADC operations */
fn MAKE_WM8775_ADDR(addr: u32, data: u32) -> u32 {
    ((addr << 1) & 0xFE) | ((data >> 8) & 0x1)
}

fn MAKE_WM8775_DATA(data: u32) -> u32 {
    data & 0xFF
}

const WM8775_IC: u32 = 0x0B;
const WM8775_MMC: u32 = 0x0C;
const WM8775_AADCL: u32 = 0x0E;
const WM8775_AADCR: u32 = 0x0F;
const WM8775_ADCMC: u32 = 0x15;
const WM8775_RESET: u32 = 0x17;

unsafe fn hw_is_adc_input_selected(hw: *mut hw, type_: ADCSRC) -> i32 {
    let mut data: u32;
    if ((*hw).model == CTSB1270) || ((*hw).model == CTOK0010) {
        /* Titanium HD has two ADC chips, one for line in and one */
        /* for MIC. Also, SE-300PCIE has a single ADC chip that */
        /* simultaneously supports 4-channel input. We don't need */
        /* to switch the ADC input. */
        return 1;
    }
    data = hw_read_20kx(hw, GPIO_DATA);
    match type_ {
        ADCSRC::ADC_MICIN => {
            data = if (data & (0x1 << 14)) != 0 { 1 } else { 0 };
        }
        ADCSRC::ADC_LINEIN => {
            data = if (data & (0x1 << 14)) != 0 { 0 } else { 1 };
        }
        _ => {
            data = 0;
        }
    }
    data as i32
}

const MIC_BOOST_0DB: i32 = 0xCF;
const MIC_BOOST_STEPS_PER_DB: i32 = 2;

unsafe fn hw_wm8775_input_select(hw: *mut hw, mut input: u8, mut gain_in_db: i8) {
    let mut adcmc: u32;
    let mut gain: u32;

    if input > 3 {
        input = 3;
    }

    adcmc = ((1u32) << input) | 0x100; /* Link L+R gain... */

    hw20k2_i2c_write(
        hw,
        MAKE_WM8775_ADDR(WM8775_ADCMC, adcmc) as u16,
        MAKE_WM8775_DATA(adcmc),
    );

    if gain_in_db < -103 {
        gain_in_db = -103;
    }
    if gain_in_db > 24 {
        gain_in_db = 24;
    }

    gain = (gain_in_db as i32 * MIC_BOOST_STEPS_PER_DB + MIC_BOOST_0DB) as u32;

    hw20k2_i2c_write(
        hw,
        MAKE_WM8775_ADDR(WM8775_AADCL, gain) as u16,
        MAKE_WM8775_DATA(gain),
    );
    /* ...so there should be no need for the following. */
    hw20k2_i2c_write(
        hw,
        MAKE_WM8775_ADDR(WM8775_AADCR, gain) as u16,
        MAKE_WM8775_DATA(gain),
    );
}

unsafe fn hw_adc_input_select(hw: *mut hw, type_: ADCSRC) -> i32 {
    let mut data: u32;
    data = hw_read_20kx(hw, GPIO_DATA);
    match type_ {
        ADCSRC::ADC_MICIN => {
            data |= 0x1 << 14;
            hw_write_20kx(hw, GPIO_DATA, data);
            hw_wm8775_input_select(hw, 0, 20); /* Mic, 20dB */
        }
        ADCSRC::ADC_LINEIN => {
            data &= !(0x1 << 14);
            hw_write_20kx(hw, GPIO_DATA, data);
            hw_wm8775_input_select(hw, 1, 0); /* Line-in, 0dB */
        }
        _ => {}
    }

    0
}

unsafe fn hw_adc_stop(hw: *mut hw) {
    let mut data: u32;
    /* Reset the ADC (reset is active low). */
    data = hw_read_20kx(hw, GPIO_DATA);
    data &= !(0x1 << 15);
    hw_write_20kx(hw, GPIO_DATA, data);
    usleep_range(10000, 11000);
}

unsafe fn hw_adc_start(hw: *mut hw) {
    let mut data: u32;
    /* Return the ADC to normal operation. */
    data = hw_read_20kx(hw, GPIO_DATA);
    data |= 0x1 << 15;
    hw_write_20kx(hw, GPIO_DATA, data);
    msleep(50);
}

unsafe fn hw_adc_reset(hw: *mut hw) {
    hw_adc_stop(hw);
    hw_adc_start(hw);
}

unsafe fn hw_adc_init(hw: *mut hw, info: *const adc_conf) -> i32 {
    let mut err: i32;
    let mut data: u32;
    let mut ctl: u32;

    /*  Set ADC reset bit as output */
    data = hw_read_20kx(hw, GPIO_CTRL);
    data |= 0x1 << 15;
    hw_write_20kx(hw, GPIO_CTRL, data);

    if (*hw).model == CTOK0010 {
        /* Manual ADC setup for SE-300PCIE is not needed. */
        hw_adc_reset(hw);
        return 0;
    }

    /* Initialize I2C */
    err = hw20k2_i2c_init(hw, 0x1A, 1, 1);
    if err < 0 {
        dev_alert((*(*hw).card).dev, c_str!("Failure to acquire I2C!!!\n"));
        hw20k2_i2c_uninit(hw);
        return err;
    }

    hw_adc_stop(hw);

    if (*hw).model == CTSB1270 {
        /* Set up the PCM4220 ADC on Titanium HD */
        data &= !0x0C;
        if 1 == (*info).msr {
            data |= 0x00; /* Single Speed Mode 32-50kHz */
        } else if 2 == (*info).msr {
            data |= 0x08; /* Double Speed Mode 50-108kHz */
        } else {
            data |= 0x04; /* Quad Speed Mode 108kHz-216kHz */
        }
        hw_write_20kx(hw, GPIO_DATA, data);
    }

    hw_adc_start(hw);

    /* I2C write to register offset 0x0B to set ADC LRCLK polarity */
    /* invert bit, interface format to I2S, word length to 24-bit, */
    /* enable ADC high pass filter. Fixes bug 5323?		*/
    hw20k2_i2c_write(hw, MAKE_WM8775_ADDR(WM8775_IC, 0x26) as u16, MAKE_WM8775_DATA(0x26));

    /* Set the master mode (256fs) */
    if 1 == (*info).msr {
        /* slave mode, 128x oversampling 256fs */
        hw20k2_i2c_write(hw, MAKE_WM8775_ADDR(WM8775_MMC, 0x02) as u16, MAKE_WM8775_DATA(0x02));
    } else if (2 == (*info).msr) || (4 == (*info).msr) {
        /* slave mode, 64x oversampling, 256fs */
        hw20k2_i2c_write(hw, MAKE_WM8775_ADDR(WM8775_MMC, 0x0A) as u16, MAKE_WM8775_DATA(0x0A));
    } else {
        dev_alert((*(*hw).card).dev, c_str!("Invalid master sampling rate (msr %d)!!!\n"), (*info).msr);
        err = -EINVAL;
        hw20k2_i2c_uninit(hw);
        return err;
    }

    if (*hw).model != CTSB1270 {
        /* Configure GPIO bit 14 change to line-in/mic-in */
        ctl = hw_read_20kx(hw, GPIO_CTRL);
        ctl |= 0x1 << 14;
        hw_write_20kx(hw, GPIO_CTRL, ctl);
        hw_adc_input_select(hw, ADCSRC::ADC_LINEIN);
    } else {
        hw_wm8775_input_select(hw, 0, 0);
    }

    0
}

unsafe fn hw_capabilities(hw: *mut hw) -> capabilities {
    let mut cap: capabilities = core::mem::zeroed();

    cap.digit_io_switch = 0;
    cap.dedicated_mic = (((*hw).model == CTSB1270) || ((*hw).model == CTOK0010)) as _;
    cap.dedicated_rca = ((*hw).model == CTOK0010) as _;
    cap.output_switch = ((*hw).model == CTSB1270) as _;
    cap.mic_source_switch = ((*hw).model == CTSB1270) as _;

    cap
}

unsafe fn hw_output_switch_get(hw: *mut hw) -> i32 {
    let data: u32 = hw_read_20kx(hw, GPIO_EXT_DATA);

    match data & 0x30 {
        0x00 => 0,
        0x10 => 1,
        0x20 => 2,
        _ => 3,
    }
}

unsafe fn hw_output_switch_put(hw: *mut hw, position: i32) -> i32 {
    let mut data: u32;

    if position == hw_output_switch_get(hw) {
        return 0;
    }

    /* Mute line and headphones (intended for anti-pop). */
    data = hw_read_20kx(hw, GPIO_DATA);
    data |= 0x03 << 11;
    hw_write_20kx(hw, GPIO_DATA, data);

    data = hw_read_20kx(hw, GPIO_EXT_DATA) & !0x30;
    match position {
        0 => {}
        1 => {
            data |= 0x10;
        }
        _ => {
            data |= 0x20;
        }
    }
    hw_write_20kx(hw, GPIO_EXT_DATA, data);

    /* Unmute line and headphones. */
    data = hw_read_20kx(hw, GPIO_DATA);
    data &= !(0x03 << 11);
    hw_write_20kx(hw, GPIO_DATA, data);

    1
}

unsafe fn hw_mic_source_switch_get(hw: *mut hw) -> i32 {
    let hw20k2 = hw as *mut hw20k2;

    (*hw20k2).mic_source
}

unsafe fn hw_mic_source_switch_put(hw: *mut hw, position: i32) -> i32 {
    let hw20k2 = hw as *mut hw20k2;

    if position == (*hw20k2).mic_source {
        return 0;
    }

    match position {
        0 => {
            hw_wm8775_input_select(hw, 0, 0); /* Mic, 0dB */
        }
        1 => {
            hw_wm8775_input_select(hw, 1, 0); /* FP Mic, 0dB */
        }
        2 => {
            hw_wm8775_input_select(hw, 3, 0); /* Aux Ext, 0dB */
        }
        _ => {
            return 0;
        }
    }

    (*hw20k2).mic_source = position;

    1
}

unsafe extern "C" fn ct_20k2_interrupt(_irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    let hw = dev_id as *mut hw;
    let mut status: u32;

    status = hw_read_20kx(hw, GIP);
    if status == 0 {
        return IRQ_NONE;
    }

    if (*hw).irq_callback.is_some() {
        ((*hw).irq_callback.unwrap())((*hw).irq_callback_data, status);
    }

    hw_write_20kx(hw, GIP, status);
    IRQ_HANDLED
}

unsafe fn hw_card_start(hw: *mut hw) -> i32 {
    let mut err: i32 = 0;
    let pci = (*hw).pci;
    let mut gctl: u32;
    let dma_bits: u32 = BITS_PER_LONG;

    err = pci_enable_device(pci);
    if err < 0 {
        return err;
    }

    /* Set DMA transfer mask */
    if dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK(dma_bits)) != 0 {
        dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK(32));
    }

    if (*hw).io_base == 0 {
        err = pci_request_regions(pci, c_str!("XFi"));
        if err < 0 {
            pci_disable_device(pci);
            return err;
        }

        (*hw).io_base = pci_resource_start((*hw).pci, 2);
        (*hw).mem_base = ioremap((*hw).io_base, pci_resource_len((*hw).pci, 2));
        if (*hw).mem_base.is_null() {
            err = -ENOENT;
            pci_release_regions(pci);
            (*hw).io_base = 0;
            pci_disable_device(pci);
            return err;
        }
    }

    /* Switch to 20k2 mode from UAA mode. */
    gctl = hw_read_20kx(hw, GLOBAL_CNTL_GCTL);
    set_field(&mut gctl, GCTL_UAA, 0);
    hw_write_20kx(hw, GLOBAL_CNTL_GCTL, gctl);

    if (*hw).irq < 0 {
        err = request_irq((*pci).irq, Some(ct_20k2_interrupt), IRQF_SHARED, KBUILD_MODNAME, hw as *mut c_void);
        if err < 0 {
            dev_err((*(*hw).card).dev, c_str!("XFi: Cannot get irq %d\n"), (*pci).irq);
            pci_release_regions(pci);
            (*hw).io_base = 0;
            pci_disable_device(pci);
            return err;
        }
        (*hw).irq = (*pci).irq;
        (*(*hw).card).sync_irq = (*hw).irq;
    }

    pci_set_master(pci);

    0
}

unsafe fn hw_card_stop(hw: *mut hw) -> i32 {
    let mut data: u32;

    /* disable transport bus master and queueing of request */
    hw_write_20kx(hw, TRANSPORT_CTL, 0x00);

    /* disable pll */
    data = hw_read_20kx(hw, PLL_ENB);
    hw_write_20kx(hw, PLL_ENB, data & !0x07);

    /* TODO: Disable interrupt and so on... */
    0
}

unsafe fn hw_card_shutdown(hw: *mut hw) -> i32 {
    if (*hw).irq >= 0 {
        free_irq((*hw).irq, hw as *mut c_void);
    }

    (*hw).irq = -1;
    iounmap((*hw).mem_base);
    (*hw).mem_base = ptr::null_mut();

    if (*hw).io_base != 0 {
        pci_release_regions((*hw).pci);
    }

    (*hw).io_base = 0;

    pci_disable_device((*hw).pci);

    0
}

unsafe fn hw_card_init(hw: *mut hw, info: *mut card_conf) -> i32 {
    let mut err: i32;
    let mut gctl: u32;
    let mut data: u32 = 0;
    let mut dac_info: dac_conf = core::mem::zeroed();
    let mut adc_info: adc_conf = core::mem::zeroed();
    let mut daio_info: daio_conf = core::mem::zeroed();
    let mut trn_info: trn_conf = core::mem::zeroed();

    /* Get PCI io port/memory base address and
     * do 20kx core switch if needed. */
    err = hw_card_start(hw);
    if err != 0 {
        return err;
    }

    /* PLL init */
    err = hw_pll_init(hw, (*info).rsr);
    if err < 0 {
        return err;
    }

    /* kick off auto-init */
    err = hw_auto_init(hw);
    if err < 0 {
        return err;
    }

    gctl = hw_read_20kx(hw, GLOBAL_CNTL_GCTL);
    set_field(&mut gctl, GCTL_DBP, 1);
    set_field(&mut gctl, GCTL_TBP, 1);
    set_field(&mut gctl, GCTL_FBP, 1);
    set_field(&mut gctl, GCTL_DPC, 0);
    hw_write_20kx(hw, GLOBAL_CNTL_GCTL, gctl);

    /* Reset all global pending interrupts */
    hw_write_20kx(hw, GIE, 0);
    /* Reset all SRC pending interrupts */
    hw_write_20kx(hw, SRC_IP, 0);

    if (*hw).model == CTSB1270 {
        hw_write_20kx(hw, GPIO_CTRL, 0x9E5F);
    } else if (*hw).model == CTOK0010 {
        hw_write_20kx(hw, GPIO_CTRL, 0x9902);
    } else {
        /* TODO: detect the card ID and configure GPIO accordingly. */
        /* Configures GPIO (0xD802 0x98028) */
        /*hw_write_20kx(hw, GPIO_CTRL, 0x7F07);*/
        /* Configures GPIO (SB0880) */
        /*hw_write_20kx(hw, GPIO_CTRL, 0xFF07);*/
        hw_write_20kx(hw, GPIO_CTRL, 0xD802);
    }
    /* Enable audio ring */
    hw_write_20kx(hw, MIXER_AR_ENABLE, 0x01);

    trn_info.vm_pgt_phys = (*info).vm_pgt_phys;
    err = hw_trn_init(hw, &trn_info);
    if err < 0 {
        return err;
    }

    daio_info.msr = (*info).msr;
    err = hw_daio_init(hw, &daio_info);
    if err < 0 {
        return err;
    }

    dac_info.msr = (*info).msr;
    err = hw_dac_init(hw, &dac_info);
    if err < 0 {
        return err;
    }

    adc_info.msr = (*info).msr;
    adc_info.input = ADCSRC::ADC_LINEIN as u8;
    adc_info.mic20db = 0;
    err = hw_adc_init(hw, &adc_info);
    if err < 0 {
        return err;
    }

    data = hw_read_20kx(hw, SRC_MCTL);
    data |= 0x1; /* Enables input from the audio ring */
    hw_write_20kx(hw, SRC_MCTL, data);

    0
}

/* CONFIG_PM_SLEEP conditional code from the C source. */
#[cfg(CONFIG_PM_SLEEP)]
unsafe fn hw_suspend(hw: *mut hw) -> i32 {
    hw_card_stop(hw);
    0
}

#[cfg(CONFIG_PM_SLEEP)]
unsafe fn hw_resume(hw: *mut hw, info: *mut card_conf) -> i32 {
    /* Re-initialize card hardware. */
    hw_card_init(hw, info)
}

unsafe fn hw_read_20kx(hw: *mut hw, reg: u32) -> u32 {
    readl((*hw).mem_base.add(reg as usize))
}

unsafe fn hw_write_20kx(hw: *mut hw, reg: u32, data: u32) {
    writel(data, (*hw).mem_base.add(reg as usize));
}

static ct20k2_preset: hw = hw {
    irq: -1,

    card_init: Some(hw_card_init),
    card_stop: Some(hw_card_stop),
    pll_init: Some(hw_pll_init),
    is_adc_source_selected: Some(hw_is_adc_input_selected),
    select_adc_source: Some(hw_adc_input_select),
    capabilities: Some(hw_capabilities),
    output_switch_get: Some(hw_output_switch_get),
    output_switch_put: Some(hw_output_switch_put),
    mic_source_switch_get: Some(hw_mic_source_switch_get),
    mic_source_switch_put: Some(hw_mic_source_switch_put),
    #[cfg(CONFIG_PM_SLEEP)]
    suspend: Some(hw_suspend),
    #[cfg(CONFIG_PM_SLEEP)]
    resume: Some(hw_resume),

    src_rsc_get_ctrl_blk: Some(src_get_rsc_ctrl_blk),
    src_rsc_put_ctrl_blk: Some(src_put_rsc_ctrl_blk),
    src_mgr_get_ctrl_blk: Some(src_mgr_get_ctrl_blk),
    src_mgr_put_ctrl_blk: Some(src_mgr_put_ctrl_blk),
    src_set_state: Some(src_set_state),
    src_set_bm: Some(src_set_bm),
    src_set_rsr: Some(src_set_rsr),
    src_set_sf: Some(src_set_sf),
    src_set_wr: Some(src_set_wr),
    src_set_pm: Some(src_set_pm),
    src_set_rom: Some(src_set_rom),
    src_set_vo: Some(src_set_vo),
    src_set_st: Some(src_set_st),
    src_set_ie: Some(src_set_ie),
    src_set_ilsz: Some(src_set_ilsz),
    src_set_bp: Some(src_set_bp),
    src_set_cisz: Some(src_set_cisz),
    src_set_ca: Some(src_set_ca),
    src_set_sa: Some(src_set_sa),
    src_set_la: Some(src_set_la),
    src_set_pitch: Some(src_set_pitch),
    src_set_dirty: Some(src_set_dirty),
    src_set_clear_zbufs: Some(src_set_clear_zbufs),
    src_set_dirty_all: Some(src_set_dirty_all),
    src_commit_write: Some(src_commit_write),
    src_get_ca: Some(src_get_ca),
    src_get_dirty: Some(src_get_dirty),
    src_dirty_conj_mask: Some(src_dirty_conj_mask),
    src_mgr_enbs_src: Some(src_mgr_enbs_src),
    src_mgr_enb_src: Some(src_mgr_enb_src),
    src_mgr_dsb_src: Some(src_mgr_dsb_src),
    src_mgr_commit_write: Some(src_mgr_commit_write),

    srcimp_mgr_get_ctrl_blk: Some(srcimp_mgr_get_ctrl_blk),
    srcimp_mgr_put_ctrl_blk: Some(srcimp_mgr_put_ctrl_blk),
    srcimp_mgr_set_imaparc: Some(srcimp_mgr_set_imaparc),
    srcimp_mgr_set_imapuser: Some(srcimp_mgr_set_imapuser),
    srcimp_mgr_set_imapnxt: Some(srcimp_mgr_set_imapnxt),
    srcimp_mgr_set_imapaddr: Some(srcimp_mgr_set_imapaddr),
    srcimp_mgr_commit_write: Some(srcimp_mgr_commit_write),

    amixer_rsc_get_ctrl_blk: Some(amixer_rsc_get_ctrl_blk),
    amixer_rsc_put_ctrl_blk: Some(amixer_rsc_put_ctrl_blk),
    amixer_mgr_get_ctrl_blk: Some(amixer_mgr_get_ctrl_blk),
    amixer_mgr_put_ctrl_blk: Some(amixer_mgr_put_ctrl_blk),
    amixer_set_mode: Some(amixer_set_mode),
    amixer_set_iv: Some(amixer_set_iv),
    amixer_set_x: Some(amixer_set_x),
    amixer_set_y: Some(amixer_set_y),
    amixer_set_sadr: Some(amixer_set_sadr),
    amixer_set_se: Some(amixer_set_se),
    amixer_set_dirty: Some(amixer_set_dirty),
    amixer_set_dirty_all: Some(amixer_set_dirty_all),
    amixer_commit_write: Some(amixer_commit_write),
    amixer_get_y: Some(amixer_get_y),
    amixer_get_dirty: Some(amixer_get_dirty),

    dai_get_ctrl_blk: Some(dai_get_ctrl_blk),
    dai_put_ctrl_blk: Some(dai_put_ctrl_blk),
    dai_srt_set_srco: Some(dai_srt_set_srco),
    dai_srt_set_srcm: Some(dai_srt_set_srcm),
    dai_srt_set_rsr: Some(dai_srt_set_rsr),
    dai_srt_set_drat: Some(dai_srt_set_drat),
    dai_srt_set_ec: Some(dai_srt_set_ec),
    dai_srt_set_et: Some(dai_srt_set_et),
    dai_commit_write: Some(dai_commit_write),

    dao_get_ctrl_blk: Some(dao_get_ctrl_blk),
    dao_put_ctrl_blk: Some(dao_put_ctrl_blk),
    dao_set_spos: Some(dao_set_spos),
    dao_commit_write: Some(dao_commit_write),
    dao_get_spos: Some(dao_get_spos),

    daio_mgr_get_ctrl_blk: Some(daio_mgr_get_ctrl_blk),
    daio_mgr_put_ctrl_blk: Some(daio_mgr_put_ctrl_blk),
    daio_mgr_enb_dai: Some(daio_mgr_enb_dai),
    daio_mgr_dsb_dai: Some(daio_mgr_dsb_dai),
    daio_mgr_enb_dao: Some(daio_mgr_enb_dao),
    daio_mgr_dsb_dao: Some(daio_mgr_dsb_dao),
    daio_mgr_dao_init: Some(daio_mgr_dao_init),
    daio_mgr_set_imaparc: Some(daio_mgr_set_imaparc),
    daio_mgr_set_imapnxt: Some(daio_mgr_set_imapnxt),
    daio_mgr_set_imapaddr: Some(daio_mgr_set_imapaddr),
    daio_mgr_commit_write: Some(daio_mgr_commit_write),

    set_timer_irq: Some(set_timer_irq),
    set_timer_tick: Some(set_timer_tick),
    get_wc: Some(get_wc),
};

#[no_mangle]
pub unsafe extern "C" fn create_20k2_hw_obj(rhw: *mut *mut hw) -> i32 {
    let hw20k2: *mut hw20k2;

    *rhw = ptr::null_mut();
    hw20k2 = kzalloc_obj::<hw20k2>() as *mut hw20k2;
    if hw20k2.is_null() {
        return -ENOMEM;
    }

    (*hw20k2).hw = ct20k2_preset;
    *rhw = &mut (*hw20k2).hw;

    0
}

#[no_mangle]
pub unsafe extern "C" fn destroy_20k2_hw_obj(hw: *mut hw) -> i32 {
    if (*hw).io_base != 0 {
        hw_card_shutdown(hw);
    }

    kfree(hw as *mut c_void);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
