/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2006 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Authors:  Shlomi Gridish <gridish@freescale.com>
 *           Li Yang <leoli@freescale.com>
 *
 * Description:
 * Internal header file for UCC SLOW unit routines.
 */

// C dependencies supplied by other translation units:
// linux/types.h, immap_qe.h, qe.h, and ucc.h

/* transmit BD's status */
pub const T_R: u32 = 0x80000000;
pub const T_PAD: u32 = 0x40000000;
pub const T_W: u32 = 0x20000000;
pub const T_I: u32 = 0x10000000;
pub const T_L: u32 = 0x08000000;
pub const T_A: u32 = 0x04000000;
pub const T_TC: u32 = 0x04000000;
pub const T_CM: u32 = 0x02000000;
pub const T_DEF: u32 = 0x02000000;
pub const T_P: u32 = 0x01000000;
pub const T_HB: u32 = 0x01000000;
pub const T_NS: u32 = 0x00800000;
pub const T_LC: u32 = 0x00800000;
pub const T_RL: u32 = 0x00400000;
pub const T_UN: u32 = 0x00020000;
pub const T_CT: u32 = 0x00010000;
pub const T_CSL: u32 = 0x00010000;
pub const T_RC: u32 = 0x003c0000;

/* Receive BD's status */
pub const R_E: u32 = 0x80000000;
pub const R_W: u32 = 0x20000000;
pub const R_I: u32 = 0x10000000;
pub const R_L: u32 = 0x08000000;
pub const R_C: u32 = 0x08000000;
pub const R_F: u32 = 0x04000000;
pub const R_A: u32 = 0x04000000;
pub const R_CM: u32 = 0x02000000;
pub const R_ID: u32 = 0x01000000;
pub const R_M: u32 = 0x01000000;
pub const R_AM: u32 = 0x00800000;
pub const R_DE: u32 = 0x00800000;
pub const R_LG: u32 = 0x00200000;
pub const R_BR: u32 = 0x00200000;
pub const R_NO: u32 = 0x00100000;
pub const R_FR: u32 = 0x00100000;
pub const R_PR: u32 = 0x00080000;
pub const R_AB: u32 = 0x00080000;
pub const R_SH: u32 = 0x00080000;
pub const R_CR: u32 = 0x00040000;
pub const R_OV: u32 = 0x00020000;
pub const R_CD: u32 = 0x00010000;
pub const R_CL: u32 = 0x00010000;

pub const UCC_SLOW_RX_ALIGN: usize = 4;
pub const UCC_SLOW_MRBLR_ALIGNMENT: usize = 4;
pub const UCC_SLOW_PRAM_SIZE: usize = 0x100;
pub const ALIGNMENT_OF_UCC_SLOW_PRAM: usize = 64;

#[repr(u32)]
pub enum ucc_slow_channel_protocol_mode {
    UCC_SLOW_CHANNEL_PROTOCOL_MODE_QMC = 0x00000002,
    UCC_SLOW_CHANNEL_PROTOCOL_MODE_UART = 0x00000004,
    UCC_SLOW_CHANNEL_PROTOCOL_MODE_BISYNC = 0x00000008,
}

#[repr(u32)]
pub enum ucc_slow_transparent_tcrc {
    UCC_SLOW_TRANSPARENT_TCRC_CCITT_CRC16 = 0x00000000,
    UCC_SLOW_TRANSPARENT_TCRC_CRC16 = 0x00004000,
    UCC_SLOW_TRANSPARENT_TCRC_CCITT_CRC32 = 0x00008000,
}

#[repr(u32)]
pub enum ucc_slow_tx_oversampling_rate {
    UCC_SLOW_OVERSAMPLING_RATE_TX_TDCR_1 = 0x00000000,
    UCC_SLOW_OVERSAMPLING_RATE_TX_TDCR_8 = 0x00010000,
    UCC_SLOW_OVERSAMPLING_RATE_TX_TDCR_16 = 0x00020000,
    UCC_SLOW_OVERSAMPLING_RATE_TX_TDCR_32 = 0x00030000,
}

#[repr(u32)]
pub enum ucc_slow_rx_oversampling_rate {
    UCC_SLOW_OVERSAMPLING_RATE_RX_RDCR_1 = 0x00000000,
    UCC_SLOW_OVERSAMPLING_RATE_RX_RDCR_8 = 0x00004000,
    UCC_SLOW_OVERSAMPLING_RATE_RX_RDCR_16 = 0x00008000,
    UCC_SLOW_OVERSAMPLING_RATE_RX_RDCR_32 = 0x0000c000,
}

#[repr(u32)]
pub enum ucc_slow_tx_encoding_method {
    UCC_SLOW_TRANSMITTER_ENCODING_METHOD_TENC_NRZ = 0x00000000,
    UCC_SLOW_TRANSMITTER_ENCODING_METHOD_TENC_NRZI = 0x00000100,
}

#[repr(u32)]
pub enum ucc_slow_rx_decoding_method {
    UCC_SLOW_RECEIVER_DECODING_METHOD_RENC_NRZ = 0x00000000,
    UCC_SLOW_RECEIVER_DECODING_METHOD_RENC_NRZI = 0x00000800,
}

#[repr(u32)]
pub enum ucc_slow_diag_mode {
    UCC_SLOW_DIAG_MODE_NORMAL = 0x00000000,
    UCC_SLOW_DIAG_MODE_LOOPBACK = 0x00000040,
    UCC_SLOW_DIAG_MODE_ECHO = 0x00000080,
    UCC_SLOW_DIAG_MODE_LOOPBACK_ECHO = 0x000000c0,
}

#[repr(C)]
pub struct ucc_slow_info {
    pub ucc_num: core::ffi::c_int,
    pub protocol: core::ffi::c_int,
    pub rx_clock: qe_clock,
    pub tx_clock: qe_clock,
    pub regs: phys_addr_t,
    pub irq: core::ffi::c_int,
    pub uccm_mask: u16,
    pub data_mem_part: core::ffi::c_int,
    pub init_tx: core::ffi::c_int,
    pub init_rx: core::ffi::c_int,
    pub tx_bd_ring_len: u32,
    pub rx_bd_ring_len: u32,
    pub rx_interrupts: core::ffi::c_int,
    pub brkpt_support: core::ffi::c_int,
    pub grant_support: core::ffi::c_int,
    pub tsa: core::ffi::c_int,
    pub cdp: core::ffi::c_int,
    pub cds: core::ffi::c_int,
    pub ctsp: core::ffi::c_int,
    pub ctss: core::ffi::c_int,
    pub rinv: core::ffi::c_int,
    pub tinv: core::ffi::c_int,
    pub rtsm: core::ffi::c_int,
    pub rfw: core::ffi::c_int,
    pub tci: core::ffi::c_int,
    pub tend: core::ffi::c_int,
    pub tfl: core::ffi::c_int,
    pub txsy: core::ffi::c_int,
    pub max_rx_buf_length: u16,
    pub tcrc: ucc_slow_transparent_tcrc,
    pub mode: ucc_slow_channel_protocol_mode,
    pub diag: ucc_slow_diag_mode,
    pub tdcr: ucc_slow_tx_oversampling_rate,
    pub rdcr: ucc_slow_rx_oversampling_rate,
    pub tenc: ucc_slow_tx_encoding_method,
    pub renc: ucc_slow_rx_decoding_method,
}

#[repr(C)]
pub struct ucc_slow_private {
    pub us_info: *mut ucc_slow_info,
    pub us_regs: *mut ucc_slow,
    pub us_pram: *mut ucc_slow_pram,
    pub us_pram_offset: i32,
    pub enabled_tx: core::ffi::c_int,
    pub enabled_rx: core::ffi::c_int,
    pub stopped_tx: core::ffi::c_int,
    pub stopped_rx: core::ffi::c_int,
    pub confQ: list_head,
    pub first_tx_bd_mask: u32,
    pub tx_base_offset: i32,
    pub rx_base_offset: i32,
    pub confBd: *mut qe_bd,
    pub tx_bd: *mut qe_bd,
    pub rx_bd: *mut qe_bd,
    pub p_rx_frame: *mut core::ffi::c_void,
    pub p_ucce: *mut u16,
    pub p_uccm: *mut u16,
    pub saved_uccm: u16,
    #[cfg(feature = "STATISTICS")]
    pub tx_frames: u32,
    #[cfg(feature = "STATISTICS")]
    pub rx_frames: u32,
    #[cfg(feature = "STATISTICS")]
    pub rx_discarded: u32,
}

extern "C" {
    pub fn ucc_slow_init(us_info: *mut ucc_slow_info, uccs_ret: *mut *mut ucc_slow_private) -> core::ffi::c_int;
    pub fn ucc_slow_free(uccs: *mut ucc_slow_private);
    pub fn ucc_slow_enable(uccs: *mut ucc_slow_private, mode: comm_dir);
    pub fn ucc_slow_disable(uccs: *mut ucc_slow_private, mode: comm_dir);
    pub fn ucc_slow_graceful_stop_tx(uccs: *mut ucc_slow_private);
    pub fn ucc_slow_stop_tx(uccs: *mut ucc_slow_private);
    pub fn ucc_slow_restart_tx(uccs: *mut ucc_slow_private);
    pub fn ucc_slow_get_qe_cr_subblock(uccs_num: core::ffi::c_int) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
