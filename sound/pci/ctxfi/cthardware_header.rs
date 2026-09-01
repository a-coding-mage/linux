/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
 *
 * @File	cthardware.h
 *
 * @Brief
 * This file contains the definition of hardware access methord.
 *
 * @Author	Liu Chun
 * @Date 	May 13 2008
 */

/* C header dependencies: linux/types.h, linux/pci.h, sound/core.h */

use core::ffi::c_void;

pub enum pci_dev {}
pub enum snd_card {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CHIPTYP {
    ATC20K1 = 0,
    ATC20K2 = 1,
    ATCNONE = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CTCARDS {
    /* 20k1 models */
    CTSB046X = 0,
    CT20K1_MODEL_FIRST = 0,
    CTSB055X = 1,
    CTSB073X = 2,
    CTUAA = 3,
    CT20K1_UNKNOWN = 4,
    /* 20k2 models */
    CTSB0760 = 5,
    CT20K2_MODEL_FIRST = 5,
    CTHENDRIX = 6,
    CTSB0880 = 7,
    CTSB1270 = 8,
    CTOK0010 = 9,
    CT20K2_UNKNOWN = 10,
    NUM_CTCARDS = 11, /* This should always be the last */
}

/* Type of input source for ADC */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ADCSRC {
    ADC_MICIN = 0,
    ADC_LINEIN = 1,
    ADC_VIDEO = 2,
    ADC_AUX = 3,
    ADC_NONE = 4, /* Switch to digital input */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct card_conf {
    /*
     * device virtual mem page table page physical addr
     * (supporting one page table page now)
     */
    pub vm_pgt_phys: usize,
    pub rsr: u32, /* reference sample rate in Hzs*/
    pub msr: u32, /* master sample rate in rsrs */
}

/*
 * C bitfields:
 * unsigned int digit_io_switch:1;
 * unsigned int dedicated_mic:1;
 * unsigned int dedicated_rca:1;
 * unsigned int output_switch:1;
 * unsigned int mic_source_switch:1;
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct capabilities {
    pub bits: u32,
}

pub const CAPABILITIES_DIGIT_IO_SWITCH: u32 = 1 << 0;
pub const CAPABILITIES_DEDICATED_MIC: u32 = 1 << 1;
pub const CAPABILITIES_DEDICATED_RCA: u32 = 1 << 2;
pub const CAPABILITIES_OUTPUT_SWITCH: u32 = 1 << 3;
pub const CAPABILITIES_MIC_SOURCE_SWITCH: u32 = 1 << 4;

#[repr(C)]
pub struct hw {
    pub card_init: Option<unsafe extern "C" fn(hw: *mut hw, info: *mut card_conf) -> i32>,
    pub card_stop: Option<unsafe extern "C" fn(hw: *mut hw) -> i32>,
    pub pll_init: Option<unsafe extern "C" fn(hw: *mut hw, rsr: u32) -> i32>,
    /*
     * Present in C only under CONFIG_PM_SLEEP:
     * int (*suspend)(struct hw *hw);
     * int (*resume)(struct hw *hw, struct card_conf *info);
     */
    pub suspend: Option<unsafe extern "C" fn(hw: *mut hw) -> i32>,
    pub resume: Option<unsafe extern "C" fn(hw: *mut hw, info: *mut card_conf) -> i32>,
    pub is_adc_source_selected: Option<unsafe extern "C" fn(hw: *mut hw, source: ADCSRC) -> i32>,
    pub select_adc_source: Option<unsafe extern "C" fn(hw: *mut hw, source: ADCSRC) -> i32>,
    pub capabilities: Option<unsafe extern "C" fn(hw: *mut hw) -> capabilities>,
    pub output_switch_get: Option<unsafe extern "C" fn(hw: *mut hw) -> i32>,
    pub output_switch_put: Option<unsafe extern "C" fn(hw: *mut hw, position: i32) -> i32>,
    pub mic_source_switch_get: Option<unsafe extern "C" fn(hw: *mut hw) -> i32>,
    pub mic_source_switch_put: Option<unsafe extern "C" fn(hw: *mut hw, position: i32) -> i32>,

    /* SRC operations */
    pub src_rsc_get_ctrl_blk: Option<unsafe extern "C" fn(rblk: *mut *mut c_void) -> i32>,
    pub src_rsc_put_ctrl_blk: Option<unsafe extern "C" fn(blk: *mut c_void) -> i32>,
    pub src_set_state: Option<unsafe extern "C" fn(blk: *mut c_void, state: u32) -> i32>,
    pub src_set_bm: Option<unsafe extern "C" fn(blk: *mut c_void, bm: u32) -> i32>,
    pub src_set_rsr: Option<unsafe extern "C" fn(blk: *mut c_void, rsr: u32) -> i32>,
    pub src_set_sf: Option<unsafe extern "C" fn(blk: *mut c_void, sf: u32) -> i32>,
    pub src_set_wr: Option<unsafe extern "C" fn(blk: *mut c_void, wr: u32) -> i32>,
    pub src_set_pm: Option<unsafe extern "C" fn(blk: *mut c_void, pm: u32) -> i32>,
    pub src_set_rom: Option<unsafe extern "C" fn(blk: *mut c_void, rom: u32) -> i32>,
    pub src_set_vo: Option<unsafe extern "C" fn(blk: *mut c_void, vo: u32) -> i32>,
    pub src_set_st: Option<unsafe extern "C" fn(blk: *mut c_void, st: u32) -> i32>,
    pub src_set_ie: Option<unsafe extern "C" fn(blk: *mut c_void, ie: u32) -> i32>,
    pub src_set_ilsz: Option<unsafe extern "C" fn(blk: *mut c_void, ilsz: u32) -> i32>,
    pub src_set_bp: Option<unsafe extern "C" fn(blk: *mut c_void, bp: u32) -> i32>,
    pub src_set_cisz: Option<unsafe extern "C" fn(blk: *mut c_void, cisz: u32) -> i32>,
    pub src_set_ca: Option<unsafe extern "C" fn(blk: *mut c_void, ca: u32) -> i32>,
    pub src_set_sa: Option<unsafe extern "C" fn(blk: *mut c_void, sa: u32) -> i32>,
    pub src_set_la: Option<unsafe extern "C" fn(blk: *mut c_void, la: u32) -> i32>,
    pub src_set_pitch: Option<unsafe extern "C" fn(blk: *mut c_void, pitch: u32) -> i32>,
    pub src_set_clear_zbufs: Option<unsafe extern "C" fn(blk: *mut c_void, clear: u32) -> i32>,
    pub src_set_dirty: Option<unsafe extern "C" fn(blk: *mut c_void, flags: u32) -> i32>,
    pub src_set_dirty_all: Option<unsafe extern "C" fn(blk: *mut c_void) -> i32>,
    pub src_commit_write: Option<unsafe extern "C" fn(hw: *mut hw, idx: u32, blk: *mut c_void) -> i32>,
    pub src_get_ca: Option<unsafe extern "C" fn(hw: *mut hw, idx: u32, blk: *mut c_void) -> i32>,
    pub src_get_dirty: Option<unsafe extern "C" fn(blk: *mut c_void) -> u32>,
    pub src_dirty_conj_mask: Option<unsafe extern "C" fn() -> u32>,
    pub src_mgr_get_ctrl_blk: Option<unsafe extern "C" fn(rblk: *mut *mut c_void) -> i32>,
    pub src_mgr_put_ctrl_blk: Option<unsafe extern "C" fn(blk: *mut c_void) -> i32>,
    /* syncly enable src @idx */
    pub src_mgr_enbs_src: Option<unsafe extern "C" fn(blk: *mut c_void, idx: u32) -> i32>,
    /* enable src @idx */
    pub src_mgr_enb_src: Option<unsafe extern "C" fn(blk: *mut c_void, idx: u32) -> i32>,
    /* disable src @idx */
    pub src_mgr_dsb_src: Option<unsafe extern "C" fn(blk: *mut c_void, idx: u32) -> i32>,
    pub src_mgr_commit_write: Option<unsafe extern "C" fn(hw: *mut hw, blk: *mut c_void) -> i32>,

    /* SRC Input Mapper operations */
    pub srcimp_mgr_get_ctrl_blk: Option<unsafe extern "C" fn(rblk: *mut *mut c_void) -> i32>,
    pub srcimp_mgr_put_ctrl_blk: Option<unsafe extern "C" fn(blk: *mut c_void) -> i32>,
    pub srcimp_mgr_set_imaparc: Option<unsafe extern "C" fn(blk: *mut c_void, slot: u32) -> i32>,
    pub srcimp_mgr_set_imapuser: Option<unsafe extern "C" fn(blk: *mut c_void, user: u32) -> i32>,
    pub srcimp_mgr_set_imapnxt: Option<unsafe extern "C" fn(blk: *mut c_void, next: u32) -> i32>,
    pub srcimp_mgr_set_imapaddr: Option<unsafe extern "C" fn(blk: *mut c_void, addr: u32) -> i32>,
    pub srcimp_mgr_commit_write: Option<unsafe extern "C" fn(hw: *mut hw, blk: *mut c_void) -> i32>,

    /* AMIXER operations */
    pub amixer_rsc_get_ctrl_blk: Option<unsafe extern "C" fn(rblk: *mut *mut c_void) -> i32>,
    pub amixer_rsc_put_ctrl_blk: Option<unsafe extern "C" fn(blk: *mut c_void) -> i32>,
    pub amixer_mgr_get_ctrl_blk: Option<unsafe extern "C" fn(rblk: *mut *mut c_void) -> i32>,
    pub amixer_mgr_put_ctrl_blk: Option<unsafe extern "C" fn(blk: *mut c_void) -> i32>,
    pub amixer_set_mode: Option<unsafe extern "C" fn(blk: *mut c_void, mode: u32) -> i32>,
    pub amixer_set_iv: Option<unsafe extern "C" fn(blk: *mut c_void, iv: u32) -> i32>,
    pub amixer_set_x: Option<unsafe extern "C" fn(blk: *mut c_void, x: u32) -> i32>,
    pub amixer_set_y: Option<unsafe extern "C" fn(blk: *mut c_void, y: u32) -> i32>,
    pub amixer_set_sadr: Option<unsafe extern "C" fn(blk: *mut c_void, sadr: u32) -> i32>,
    pub amixer_set_se: Option<unsafe extern "C" fn(blk: *mut c_void, se: u32) -> i32>,
    pub amixer_set_dirty: Option<unsafe extern "C" fn(blk: *mut c_void, flags: u32) -> i32>,
    pub amixer_set_dirty_all: Option<unsafe extern "C" fn(blk: *mut c_void) -> i32>,
    pub amixer_commit_write: Option<unsafe extern "C" fn(hw: *mut hw, idx: u32, blk: *mut c_void) -> i32>,
    pub amixer_get_y: Option<unsafe extern "C" fn(blk: *mut c_void) -> i32>,
    pub amixer_get_dirty: Option<unsafe extern "C" fn(blk: *mut c_void) -> u32>,

    /* DAIO operations */
    pub dai_get_ctrl_blk: Option<unsafe extern "C" fn(rblk: *mut *mut c_void) -> i32>,
    pub dai_put_ctrl_blk: Option<unsafe extern "C" fn(blk: *mut c_void) -> i32>,
    pub dai_srt_set_srco: Option<unsafe extern "C" fn(blk: *mut c_void, src: u32) -> i32>,
    pub dai_srt_set_srcm: Option<unsafe extern "C" fn(blk: *mut c_void, src: u32) -> i32>,
    pub dai_srt_set_rsr: Option<unsafe extern "C" fn(blk: *mut c_void, rsr: u32) -> i32>,
    pub dai_srt_set_drat: Option<unsafe extern "C" fn(blk: *mut c_void, drat: u32) -> i32>,
    pub dai_srt_set_ec: Option<unsafe extern "C" fn(blk: *mut c_void, ec: u32) -> i32>,
    pub dai_srt_set_et: Option<unsafe extern "C" fn(blk: *mut c_void, et: u32) -> i32>,
    pub dai_commit_write: Option<unsafe extern "C" fn(hw: *mut hw, idx: u32, blk: *mut c_void) -> i32>,
    pub dao_get_ctrl_blk: Option<unsafe extern "C" fn(rblk: *mut *mut c_void) -> i32>,
    pub dao_put_ctrl_blk: Option<unsafe extern "C" fn(blk: *mut c_void) -> i32>,
    pub dao_set_spos: Option<unsafe extern "C" fn(blk: *mut c_void, spos: u32) -> i32>,
    pub dao_commit_write: Option<unsafe extern "C" fn(hw: *mut hw, idx: u32, blk: *mut c_void) -> i32>,
    pub dao_get_spos: Option<unsafe extern "C" fn(blk: *mut c_void, spos: *mut u32) -> i32>,

    pub daio_mgr_get_ctrl_blk: Option<unsafe extern "C" fn(hw: *mut hw, rblk: *mut *mut c_void) -> i32>,
    pub daio_mgr_put_ctrl_blk: Option<unsafe extern "C" fn(blk: *mut c_void) -> i32>,
    pub daio_mgr_enb_dai: Option<unsafe extern "C" fn(blk: *mut c_void, idx: u32) -> i32>,
    pub daio_mgr_dsb_dai: Option<unsafe extern "C" fn(blk: *mut c_void, idx: u32) -> i32>,
    pub daio_mgr_enb_dao: Option<unsafe extern "C" fn(blk: *mut c_void, idx: u32) -> i32>,
    pub daio_mgr_dsb_dao: Option<unsafe extern "C" fn(blk: *mut c_void, idx: u32) -> i32>,
    pub daio_mgr_dao_init:
        Option<unsafe extern "C" fn(hw: *mut hw, blk: *mut c_void, idx: u32, conf: u32) -> i32>,
    pub daio_mgr_set_imaparc: Option<unsafe extern "C" fn(blk: *mut c_void, slot: u32) -> i32>,
    pub daio_mgr_set_imapnxt: Option<unsafe extern "C" fn(blk: *mut c_void, next: u32) -> i32>,
    pub daio_mgr_set_imapaddr: Option<unsafe extern "C" fn(blk: *mut c_void, addr: u32) -> i32>,
    pub daio_mgr_commit_write: Option<unsafe extern "C" fn(hw: *mut hw, blk: *mut c_void) -> i32>,

    pub set_timer_irq: Option<unsafe extern "C" fn(hw: *mut hw, enable: i32) -> i32>,
    pub set_timer_tick: Option<unsafe extern "C" fn(hw: *mut hw, tick: u32) -> i32>,
    pub get_wc: Option<unsafe extern "C" fn(hw: *mut hw) -> u32>,

    pub irq_callback: Option<unsafe extern "C" fn(data: *mut c_void, bit: u32)>,
    pub irq_callback_data: *mut c_void,

    pub pci: *mut pci_dev,      /* the pci kernel structure of this card */
    pub card: *mut snd_card,    /* pointer to this card */
    pub irq: i32,
    pub io_base: usize,
    pub mem_base: *mut c_void,

    pub chip_type: CHIPTYP,
    pub model: CTCARDS,
}

unsafe extern "C" {
    pub fn create_hw_obj(
        pci: *mut pci_dev,
        chip_type: CHIPTYP,
        model: CTCARDS,
        rhw: *mut *mut hw,
    ) -> i32;
    pub fn destroy_hw_obj(hw: *mut hw) -> i32;

    pub fn get_field(data: u32, field: u32) -> u32;
    pub fn set_field(data: *mut u32, field: u32, value: u32);
}

/* IRQ bits */
pub const PLL_INT: u32 = 1 << 10; /* PLL input-clock out-of-range */
pub const FI_INT: u32 = 1 << 9; /* forced interrupt */
pub const IT_INT: u32 = 1 << 8; /* timer interrupt */
pub const PCI_INT: u32 = 1 << 7; /* PCI bus error pending */
pub const URT_INT: u32 = 1 << 6; /* UART Tx/Rx */
pub const GPI_INT: u32 = 1 << 5; /* GPI pin */
pub const MIX_INT: u32 = 1 << 4; /* mixer parameter segment FIFO channels */
pub const DAI_INT: u32 = 1 << 3; /* DAI (SR-tracker or SPDIF-receiver) */
pub const TP_INT: u32 = 1 << 2; /* transport priority queue */
pub const DSP_INT: u32 = 1 << 1; /* DSP */
pub const SRC_INT: u32 = 1 << 0; /* SRC channels */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
