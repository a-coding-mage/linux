/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Internal header file for QE TDM mode routines.
 *
 * Copyright (C) 2016 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Authors: Zhao Qiang <qiang.zhao@nxp.com>
 */

// Dependencies supplied by the corresponding kernel/QE headers are intentionally
// left as external types and declarations.

/* SI RAM entries */
pub const SIR_LAST: u32 = 0x0001;
pub const SIR_BYTE: u32 = 0x0002;
#[inline]
pub const fn SIR_CNT(x: u32) -> u32 { x << 2 }
#[inline]
pub const fn SIR_CSEL(x: u32) -> u32 { x << 5 }
pub const SIR_SGS: u32 = 0x0200;
pub const SIR_SWTR: u32 = 0x4000;
pub const SIR_MCC: u32 = 0x8000;
pub const SIR_IDLE: u32 = 0;

/* SIxMR fields */
#[inline]
pub const fn SIMR_SAD(x: u32) -> u32 { x << 12 }
pub const SIMR_SDM_NORMAL: u32 = 0x0000;
pub const SIMR_SDM_INTERNAL_LOOPBACK: u32 = 0x0800;
pub const SIMR_SDM_MASK: u32 = 0x0c00;
pub const SIMR_CRT: u32 = 0x0040;
pub const SIMR_SL: u32 = 0x0020;
pub const SIMR_CE: u32 = 0x0010;
pub const SIMR_FE: u32 = 0x0008;
pub const SIMR_GM: u32 = 0x0004;
#[inline]
pub const fn SIMR_TFSD(n: u32) -> u32 { n }
#[inline]
pub const fn SIMR_RFSD(n: u32) -> u32 { n << 8 }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tdm_ts_t {
    TDM_TX_TS,
    TDM_RX_TS,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tdm_framer_t {
    TDM_FRAMER_T1,
    TDM_FRAMER_E1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tdm_mode_t {
    TDM_INTERNAL_LOOPBACK,
    TDM_NORMAL,
}

#[repr(C)]
pub struct si_mode_info {
    pub simr_rfsd: u8,
    pub simr_tfsd: u8,
    pub simr_crt: u8,
    pub simr_sl: u8,
    pub simr_ce: u8,
    pub simr_fe: u8,
    pub simr_gm: u8,
}

#[repr(C)]
pub struct ucc_tdm_info {
    pub uf_info: ucc_fast_info,
    pub si_info: si_mode_info,
}

#[repr(C)]
pub struct ucc_tdm {
    pub tdm_port: u16, /* port for this tdm:TDMA,TDMB */
    pub siram_entry_id: u32,
    pub siram: *mut u16,
    pub si_regs: *mut si1,
    pub tdm_framer_type: tdm_framer_t,
    pub tdm_mode: tdm_mode_t,
    pub num_of_ts: u8, /* the number of timeslots in this tdm frame */
    pub tx_ts_mask: u32, /* tx time slot mask */
    pub rx_ts_mask: u32, /* rx time slot mask */
}

extern "C" {
    pub fn ucc_of_parse_tdm(
        np: *mut device_node,
        utdm: *mut ucc_tdm,
        ut_info: *mut ucc_tdm_info,
    ) -> i32;
    pub fn ucc_tdm_init(utdm: *mut ucc_tdm, ut_info: *mut ucc_tdm_info);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
