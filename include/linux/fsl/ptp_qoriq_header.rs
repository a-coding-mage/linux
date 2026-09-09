// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2010 OMICRON electronics GmbH
 * Copyright 2018 NXP
 */

/* Linux dependencies are supplied by the surrounding translation unit. */

#[repr(C)]
pub struct ctrl_regs {
    pub tmr_ctrl: u32, pub tmr_tevent: u32, pub tmr_temask: u32,
    pub tmr_pevent: u32, pub tmr_pemask: u32, pub tmr_stat: u32,
    pub tmr_cnt_h: u32, pub tmr_cnt_l: u32, pub tmr_add: u32,
    pub tmr_acc: u32, pub tmr_prsc: u32, pub res1: [u8; 4],
    pub tmroff_h: u32, pub tmroff_l: u32,
}

#[repr(C)]
pub struct alarm_regs { pub tmr_alarm1_h: u32, pub tmr_alarm1_l: u32, pub tmr_alarm2_h: u32, pub tmr_alarm2_l: u32 }
#[repr(C)]
pub struct fiper_regs { pub tmr_fiper1: u32, pub tmr_fiper2: u32, pub tmr_fiper3: u32 }
#[repr(C)]
pub struct etts_regs { pub tmr_etts1_h: u32, pub tmr_etts1_l: u32, pub tmr_etts2_h: u32, pub tmr_etts2_l: u32 }

#[repr(C)]
pub struct ptp_qoriq_registers {
    pub ctrl_regs: *mut ctrl_regs,
    pub alarm_regs: *mut alarm_regs,
    pub fiper_regs: *mut fiper_regs,
    pub etts_regs: *mut etts_regs,
}

pub const ETSEC_CTRL_REGS_OFFSET: u32 = 0x0;
pub const ETSEC_ALARM_REGS_OFFSET: u32 = 0x40;
pub const ETSEC_FIPER_REGS_OFFSET: u32 = 0x80;
pub const ETSEC_ETTS_REGS_OFFSET: u32 = 0xa0;
pub const CTRL_REGS_OFFSET: u32 = 0x80;
pub const ALARM_REGS_OFFSET: u32 = 0xb8;
pub const FIPER_REGS_OFFSET: u32 = 0xd0;
pub const ETTS_REGS_OFFSET: u32 = 0xe0;

pub const ALM1P: u32 = 1 << 31; pub const ALM2P: u32 = 1 << 30; pub const FIPERST: u32 = 1 << 28;
pub const PP1L: u32 = 1 << 27; pub const PP2L: u32 = 1 << 26; pub const TCLK_PERIOD_SHIFT: u32 = 16;
pub const TCLK_PERIOD_MASK: u32 = 0x3ff; pub const RTPE: u32 = 1 << 15; pub const FRD: u32 = 1 << 14;
pub const ESFDP: u32 = 1 << 11; pub const ESFDE: u32 = 1 << 10; pub const ETEP2: u32 = 1 << 9;
pub const ETEP1: u32 = 1 << 8; pub const COPH: u32 = 1 << 7; pub const CIPH: u32 = 1 << 6;
pub const TMSR: u32 = 1 << 5; pub const BYP: u32 = 1 << 3; pub const TE: u32 = 1 << 2;
pub const CKSEL_SHIFT: u32 = 0; pub const CKSEL_MASK: u32 = 0x3;
pub const ETS2: u32 = 1 << 25; pub const ETS1: u32 = 1 << 24; pub const ALM2: u32 = 1 << 17;
pub const ALM1: u32 = 1 << 16; pub const PP1: u32 = 1 << 7; pub const PP2: u32 = 1 << 6; pub const PP3: u32 = 1 << 5;
pub const ETS2EN: u32 = 1 << 25; pub const ETS1EN: u32 = 1 << 24; pub const ALM2EN: u32 = 1 << 17;
pub const ALM1EN: u32 = 1 << 16; pub const PP1EN: u32 = 1 << 7; pub const PP2EN: u32 = 1 << 6;
pub const TXP2: u32 = 1 << 9; pub const TXP1: u32 = 1 << 8; pub const RXP: u32 = 1;
pub const TXP2EN: u32 = 1 << 9; pub const TXP1EN: u32 = 1 << 8; pub const RXPEN: u32 = 1;
pub const STAT_VEC_SHIFT: u32 = 0; pub const STAT_VEC_MASK: u32 = 0x3f; pub const ETS1_VLD: u32 = 1 << 24; pub const ETS2_VLD: u32 = 1 << 25;
pub const PRSC_OCK_SHIFT: u32 = 0; pub const PRSC_OCK_MASK: u32 = 0xffff;

pub const DRIVER: &str = "ptp_qoriq";
pub const N_EXT_TS: i32 = 2;
pub const DEFAULT_CKSEL: u32 = 1;
pub const DEFAULT_TMR_PRSC: u32 = 2;
pub const DEFAULT_FIPER1_PERIOD: u32 = 1_000_000_000;
pub const DEFAULT_FIPER2_PERIOD: u32 = 1_000_000_000;
pub const DEFAULT_FIPER3_PERIOD: u32 = 1_000_000_000;

#[repr(C)]
pub struct ptp_qoriq {
    pub base: *mut core::ffi::c_void,
    pub regs: ptp_qoriq_registers,
    pub lock: spinlock_t,
    pub clock: *mut ptp_clock,
    pub caps: ptp_clock_info,
    pub rsrc: *mut resource,
    pub dev: *mut device,
    pub extts_fifo_support: bool, pub fiper3_support: bool, pub etsec: bool,
    pub irq: i32, pub phc_index: i32, pub tclk_period: u32, pub tmr_prsc: u32,
    pub tmr_add: u32, pub cksel: u32, pub tmr_fiper1: u32, pub tmr_fiper2: u32, pub tmr_fiper3: u32,
    pub read: Option<unsafe extern "C" fn(*mut u32) -> u32>,
    pub write: Option<unsafe extern "C" fn(*mut u32, u32)>,
}

#[inline]
pub unsafe fn qoriq_read_be(addr: *mut u32) -> u32 { ioread32be(addr) }
#[inline]
pub unsafe fn qoriq_write_be(addr: *mut u32, val: u32) { iowrite32be(val, addr); }
#[inline]
pub unsafe fn qoriq_read_le(addr: *mut u32) -> u32 { ioread32(addr) }
#[inline]
pub unsafe fn qoriq_write_le(addr: *mut u32, val: u32) { iowrite32(val, addr); }

extern "C" {
    pub fn ptp_qoriq_isr(irq: i32, priv_: *mut core::ffi::c_void) -> irqreturn_t;
    pub fn ptp_qoriq_init(ptp_qoriq: *mut ptp_qoriq, base: *mut core::ffi::c_void, caps: *const ptp_clock_info) -> i32;
    pub fn ptp_qoriq_free(ptp_qoriq: *mut ptp_qoriq);
    pub fn ptp_qoriq_adjfine(ptp: *mut ptp_clock_info, scaled_ppm: i64) -> i32;
    pub fn ptp_qoriq_adjtime(ptp: *mut ptp_clock_info, delta: i64) -> i32;
    pub fn ptp_qoriq_gettime(ptp: *mut ptp_clock_info, ts: *mut timespec64) -> i32;
    pub fn ptp_qoriq_settime(ptp: *mut ptp_clock_info, ts: *const timespec64) -> i32;
    pub fn ptp_qoriq_enable(ptp: *mut ptp_clock_info, rq: *mut ptp_clock_request, on: i32) -> i32;
    pub fn extts_clean_up(ptp_qoriq: *mut ptp_qoriq, index: i32, update_event: bool) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
