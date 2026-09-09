/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (C) 2003-2018 Cavium, Inc.
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2, or (at your option) any
 * later version.
 *
 * This file is distributed in the hope that it will be useful, but
 * AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
 * NONINFRINGEMENT. See the GNU General Public License for more details.
 ***********************license end**************************************/

// The C header depends on OCTEON family constants and CVMX_ADD_IO_SEG,
// supplied by other translated headers.

#[inline]
fn cvmx_pcsx_reg(offset: usize, block_id: usize, address: u64) -> u64 {
    let stride = match cvmx_get_octeon_family() {
        OCTEON_CN68XX & OCTEON_FAMILY_MASK => 0x4000u64,
        _ => 0x20000u64,
    };
    CVMX_ADD_IO_SEG(address) + ((offset as u64) + (block_id as u64) * stride) * 1024
}

#[inline]
pub fn CVMX_PCSX_ANX_ADV_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001010u64) }
#[inline]
pub fn CVMX_PCSX_ANX_EXT_ST_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001028u64) }
#[inline]
pub fn CVMX_PCSX_ANX_LP_ABIL_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001018u64) }
#[inline]
pub fn CVMX_PCSX_ANX_RESULTS_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001020u64) }
#[inline]
pub fn CVMX_PCSX_INTX_EN_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001088u64) }
#[inline]
pub fn CVMX_PCSX_INTX_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001080u64) }
#[inline]
pub fn CVMX_PCSX_LINKX_TIMER_COUNT_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001040u64) }
#[inline]
pub fn CVMX_PCSX_LOG_ANLX_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001090u64) }
#[inline]
pub fn CVMX_PCSX_MISCX_CTL_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001078u64) }
#[inline]
pub fn CVMX_PCSX_MRX_CONTROL_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001000u64) }
#[inline]
pub fn CVMX_PCSX_MRX_STATUS_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001008u64) }
#[inline]
pub fn CVMX_PCSX_RXX_STATES_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001058u64) }
#[inline]
pub fn CVMX_PCSX_RXX_SYNC_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001050u64) }
#[inline]
pub fn CVMX_PCSX_SGMX_AN_ADV_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001068u64) }
#[inline]
pub fn CVMX_PCSX_SGMX_LP_ADV_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001070u64) }
#[inline]
pub fn CVMX_PCSX_TXX_STATES_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001060u64) }
#[inline]
pub fn CVMX_PCSX_TX_RXX_POLARITY_REG(offset: usize, block_id: usize) -> u64 { cvmx_pcsx_reg(offset, block_id, 0x00011800B0001048u64) }

extern "C" {
    pub fn __cvmx_interrupt_pcsx_intx_en_reg_enable(index: i32, block: i32);
}

// C bit-fields are represented as their native 64-bit register words. The
// field ordering is conditional on __BIG_ENDIAN_BITFIELD in the source.
macro_rules! pcsx_reg_union {
    ($union:ident, $s:ident) => {
        #[repr(C)]
        pub union $union { pub u64: u64, pub s: $s }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct $s { pub bits: u64 }
    };
}

pcsx_reg_union!(cvmx_pcsx_anx_adv_reg, cvmx_pcsx_anx_adv_reg_s);
pcsx_reg_union!(cvmx_pcsx_anx_ext_st_reg, cvmx_pcsx_anx_ext_st_reg_s);
pcsx_reg_union!(cvmx_pcsx_anx_lp_abil_reg, cvmx_pcsx_anx_lp_abil_reg_s);
pcsx_reg_union!(cvmx_pcsx_anx_results_reg, cvmx_pcsx_anx_results_reg_s);
pcsx_reg_union!(cvmx_pcsx_intx_en_reg, cvmx_pcsx_intx_en_reg_s);
pcsx_reg_union!(cvmx_pcsx_intx_reg, cvmx_pcsx_intx_reg_s);
pcsx_reg_union!(cvmx_pcsx_linkx_timer_count_reg, cvmx_pcsx_linkx_timer_count_reg_s);
pcsx_reg_union!(cvmx_pcsx_log_anlx_reg, cvmx_pcsx_log_anlx_reg_s);
pcsx_reg_union!(cvmx_pcsx_miscx_ctl_reg, cvmx_pcsx_miscx_ctl_reg_s);
pcsx_reg_union!(cvmx_pcsx_mrx_control_reg, cvmx_pcsx_mrx_control_reg_s);
pcsx_reg_union!(cvmx_pcsx_mrx_status_reg, cvmx_pcsx_mrx_status_reg_s);
pcsx_reg_union!(cvmx_pcsx_rxx_states_reg, cvmx_pcsx_rxx_states_reg_s);
pcsx_reg_union!(cvmx_pcsx_rxx_sync_reg, cvmx_pcsx_rxx_sync_reg_s);
pcsx_reg_union!(cvmx_pcsx_sgmx_an_adv_reg, cvmx_pcsx_sgmx_an_adv_reg_s);
pcsx_reg_union!(cvmx_pcsx_sgmx_lp_adv_reg, cvmx_pcsx_sgmx_lp_adv_reg_s);
pcsx_reg_union!(cvmx_pcsx_txx_states_reg, cvmx_pcsx_txx_states_reg_s);
pcsx_reg_union!(cvmx_pcsx_tx_rxx_polarity_reg, cvmx_pcsx_tx_rxx_polarity_reg_s);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
