// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) ST-Ericsson SA 2012
//
// Author: Ola Lilja <ola.o.lilja@stericsson.com>,
//         for ST-Ericsson.

// Requires: <linux/platform_device.h>

use core::ffi::c_void;

pub const MSP_INPUT_FREQ_APB: u32 = 48000000;

// Stereo mode. Used for APB data accesses as 16 bits accesses (mono),
// 32 bits accesses (stereo).
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_stereo_mode {
    MSP_MONO = 0,
    MSP_STEREO = 1,
}

// Direction (Transmit/Receive mode)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_direction {
    MSP_TX = 1,
    MSP_RX = 2,
}

// Transmit and receive configuration register
pub const MSP_BIG_ENDIAN: u32 = 0x00000000;
pub const MSP_LITTLE_ENDIAN: u32 = 0x00001000;
pub const MSP_UNEXPECTED_FS_ABORT: u32 = 0x00000000;
pub const MSP_UNEXPECTED_FS_IGNORE: u32 = 0x00008000;
pub const MSP_NON_MODE_BIT_MASK: u32 = 0x00009000;

// Global configuration register
pub const RX_ENABLE: u32 = 0x00000001;
pub const RX_FIFO_ENABLE: u32 = 0x00000002;
pub const RX_SYNC_SRG: u32 = 0x00000010;
pub const RX_CLK_POL_RISING: u32 = 0x00000020;
pub const RX_CLK_SEL_SRG: u32 = 0x00000040;
pub const TX_ENABLE: u32 = 0x00000100;
pub const TX_FIFO_ENABLE: u32 = 0x00000200;
pub const TX_SYNC_SRG_PROG: u32 = 0x00001800;
pub const TX_SYNC_SRG_AUTO: u32 = 0x00001000;
pub const TX_CLK_POL_RISING: u32 = 0x00002000;
pub const TX_CLK_SEL_SRG: u32 = 0x00004000;
pub const TX_EXTRA_DELAY_ENABLE: u32 = 0x00008000;
pub const SRG_ENABLE: u32 = 0x00010000;
pub const FRAME_GEN_ENABLE: u32 = 0x00100000;
pub const SRG_CLK_SEL_APB: u32 = 0x00000000;
pub const RX_FIFO_SYNC_HI: u32 = 0x00000000;
pub const TX_FIFO_SYNC_HI: u32 = 0x00000000;
pub const SPI_CLK_MODE_NORMAL: u32 = 0x00000000;

pub const MSP_FRAME_SIZE_AUTO: i32 = -1;

pub const MSP_DR: u32 = 0x00;
pub const MSP_GCR: u32 = 0x04;
pub const MSP_TCF: u32 = 0x08;
pub const MSP_RCF: u32 = 0x0c;
pub const MSP_SRG: u32 = 0x10;
pub const MSP_FLR: u32 = 0x14;
pub const MSP_DMACR: u32 = 0x18;

pub const MSP_IMSC: u32 = 0x20;
pub const MSP_RIS: u32 = 0x24;
pub const MSP_MIS: u32 = 0x28;
pub const MSP_ICR: u32 = 0x2c;
pub const MSP_MCR: u32 = 0x30;
pub const MSP_RCV: u32 = 0x34;
pub const MSP_RCM: u32 = 0x38;

pub const MSP_TCE0: u32 = 0x40;
pub const MSP_TCE1: u32 = 0x44;
pub const MSP_TCE2: u32 = 0x48;
pub const MSP_TCE3: u32 = 0x4c;

pub const MSP_RCE0: u32 = 0x60;
pub const MSP_RCE1: u32 = 0x64;
pub const MSP_RCE2: u32 = 0x68;
pub const MSP_RCE3: u32 = 0x6c;
pub const MSP_IODLY: u32 = 0x70;

pub const MSP_ITCR: u32 = 0x80;
pub const MSP_ITIP: u32 = 0x84;
pub const MSP_ITOP: u32 = 0x88;
pub const MSP_TSTDR: u32 = 0x8c;

pub const MSP_PID0: u32 = 0xfe0;
pub const MSP_PID1: u32 = 0xfe4;
pub const MSP_PID2: u32 = 0xfe8;
pub const MSP_PID3: u32 = 0xfec;

pub const MSP_CID0: u32 = 0xff0;
pub const MSP_CID1: u32 = 0xff4;
pub const MSP_CID2: u32 = 0xff8;
pub const MSP_CID3: u32 = 0xffc;

// Protocol dependant parameters list
pub const RX_ENABLE_MASK: u32 = 1 << 0;
pub const RX_FIFO_ENABLE_MASK: u32 = 1 << 1;
pub const RX_FSYNC_MASK: u32 = 1 << 2;
pub const DIRECT_COMPANDING_MASK: u32 = 1 << 3;
pub const RX_SYNC_SEL_MASK: u32 = 1 << 4;
pub const RX_CLK_POL_MASK: u32 = 1 << 5;
pub const RX_CLK_SEL_MASK: u32 = 1 << 6;
pub const LOOPBACK_MASK: u32 = 1 << 7;
pub const TX_ENABLE_MASK: u32 = 1 << 8;
pub const TX_FIFO_ENABLE_MASK: u32 = 1 << 9;
pub const TX_FSYNC_MASK: u32 = 1 << 10;
pub const TX_MSP_TDR_TSR: u32 = 1 << 11;
pub const TX_SYNC_SEL_MASK: u32 = (1 << 12) | (1 << 11);
pub const TX_CLK_POL_MASK: u32 = 1 << 13;
pub const TX_CLK_SEL_MASK: u32 = 1 << 14;
pub const TX_EXTRA_DELAY_MASK: u32 = 1 << 15;
pub const SRG_ENABLE_MASK: u32 = 1 << 16;
pub const SRG_CLK_POL_MASK: u32 = 1 << 17;
pub const SRG_CLK_SEL_MASK: u32 = (1 << 19) | (1 << 18);
pub const FRAME_GEN_EN_MASK: u32 = 1 << 20;
pub const SPI_CLK_MODE_MASK: u32 = (1 << 22) | (1 << 21);
pub const SPI_BURST_MODE_MASK: u32 = 1 << 23;

pub const RXEN_SHIFT: u32 = 0;
pub const RFFEN_SHIFT: u32 = 1;
pub const RFSPOL_SHIFT: u32 = 2;
pub const DCM_SHIFT: u32 = 3;
pub const RFSSEL_SHIFT: u32 = 4;
pub const RCKPOL_SHIFT: u32 = 5;
pub const RCKSEL_SHIFT: u32 = 6;
pub const LBM_SHIFT: u32 = 7;
pub const TXEN_SHIFT: u32 = 8;
pub const TFFEN_SHIFT: u32 = 9;
pub const TFSPOL_SHIFT: u32 = 10;
pub const TFSSEL_SHIFT: u32 = 11;
pub const TCKPOL_SHIFT: u32 = 13;
pub const TCKSEL_SHIFT: u32 = 14;
pub const TXDDL_SHIFT: u32 = 15;
pub const SGEN_SHIFT: u32 = 16;
pub const SCKPOL_SHIFT: u32 = 17;
pub const SCKSEL_SHIFT: u32 = 18;
pub const FGEN_SHIFT: u32 = 20;
pub const SPICKM_SHIFT: u32 = 21;
pub const TBSWAP_SHIFT: u32 = 28;

pub const RCKPOL_MASK: u32 = 1 << 0;
pub const TCKPOL_MASK: u32 = 1 << 0;
pub const SPICKM_MASK: u32 = (1 << 1) | (1 << 0);

pub const fn msp_rx_clkpol_bit(n: u32) -> u32 {
    (n & RCKPOL_MASK) << RCKPOL_SHIFT
}

pub const fn msp_tx_clkpol_bit(n: u32) -> u32 {
    (n & TCKPOL_MASK) << TCKPOL_SHIFT
}

pub const P1ELEN_SHIFT: u32 = 0;
pub const P1FLEN_SHIFT: u32 = 3;
pub const DTYP_SHIFT: u32 = 10;
pub const ENDN_SHIFT: u32 = 12;
pub const DDLY_SHIFT: u32 = 13;
pub const FSIG_SHIFT: u32 = 15;
pub const P2ELEN_SHIFT: u32 = 16;
pub const P2FLEN_SHIFT: u32 = 19;
pub const P2SM_SHIFT: u32 = 26;
pub const P2EN_SHIFT: u32 = 27;
pub const FSYNC_SHIFT: u32 = 15;

pub const P1ELEN_MASK: u32 = 0x00000007;
pub const P2ELEN_MASK: u32 = 0x00070000;
pub const P1FLEN_MASK: u32 = 0x00000378;
pub const P2FLEN_MASK: u32 = 0x03780000;
pub const DDLY_MASK: u32 = 0x00003000;
pub const DTYP_MASK: u32 = 0x00000600;
pub const P2SM_MASK: u32 = 0x04000000;
pub const P2EN_MASK: u32 = 0x08000000;
pub const ENDN_MASK: u32 = 0x00001000;
pub const TFSPOL_MASK: u32 = 0x00000400;
pub const TBSWAP_MASK: u32 = 0x30000000;
pub const COMPANDING_MODE_MASK: u32 = 0x00000c00;
pub const FSYNC_MASK: u32 = 0x00008000;

pub const fn msp_p1_elem_len_bits(n: u32) -> u32 {
    n & P1ELEN_MASK
}

pub const fn msp_p2_elem_len_bits(n: u32) -> u32 {
    ((n) << P2ELEN_SHIFT) & P2ELEN_MASK
}

pub const fn msp_p1_frame_len_bits(n: u32) -> u32 {
    ((n) << P1FLEN_SHIFT) & P1FLEN_MASK
}

pub const fn msp_p2_frame_len_bits(n: u32) -> u32 {
    ((n) << P2FLEN_SHIFT) & P2FLEN_MASK
}

pub const fn msp_data_delay_bits(n: u32) -> u32 {
    ((n) << DDLY_SHIFT) & DDLY_MASK
}

pub const fn msp_data_type_bits(n: u32) -> u32 {
    ((n) << DTYP_SHIFT) & DTYP_MASK
}

pub const fn msp_p2_start_mode_bit(n: u32) -> u32 {
    (n << P2SM_SHIFT) & P2SM_MASK
}

pub const fn msp_p2_enable_bit(n: u32) -> u32 {
    (n << P2EN_SHIFT) & P2EN_MASK
}

pub const fn msp_set_endiannes_bit(n: u32) -> u32 {
    (n << ENDN_SHIFT) & ENDN_MASK
}

pub const fn msp_fsync_pol(n: u32) -> u32 {
    (n << TFSPOL_SHIFT) & TFSPOL_MASK
}

pub const fn msp_data_word_swap(n: u32) -> u32 {
    (n << TBSWAP_SHIFT) & TBSWAP_MASK
}

pub const fn msp_set_companding_mode(n: u32) -> u32 {
    (n << DTYP_SHIFT) & COMPANDING_MODE_MASK
}

pub const fn msp_set_fsync_ignore(n: u32) -> u32 {
    (n << FSYNC_SHIFT) & FSYNC_MASK
}

// Flag register
pub const RX_BUSY: u32 = 1 << 0;
pub const RX_FIFO_EMPTY: u32 = 1 << 1;
pub const RX_FIFO_FULL: u32 = 1 << 2;
pub const TX_BUSY: u32 = 1 << 3;
pub const TX_FIFO_EMPTY: u32 = 1 << 4;
pub const TX_FIFO_FULL: u32 = 1 << 5;

pub const RBUSY_SHIFT: u32 = 0;
pub const RFE_SHIFT: u32 = 1;
pub const RFU_SHIFT: u32 = 2;
pub const TBUSY_SHIFT: u32 = 3;
pub const TFE_SHIFT: u32 = 4;
pub const TFU_SHIFT: u32 = 5;

// Multichannel control register
pub const RMCEN_SHIFT: u32 = 0;
pub const RMCSF_SHIFT: u32 = 1;
pub const RCMPM_SHIFT: u32 = 3;
pub const TMCEN_SHIFT: u32 = 5;
pub const TNCSF_SHIFT: u32 = 6;

// Sample rate generator register
pub const SCKDIV_SHIFT: u32 = 0;
pub const FRWID_SHIFT: u32 = 10;
pub const FRPER_SHIFT: u32 = 16;

pub const SCK_DIV_MASK: u32 = 0x0000003FF;

pub const fn frame_width_bits(n: u32) -> u32 {
    ((n) << FRWID_SHIFT) & 0x0000FC00
}

pub const fn frame_period_bits(n: u32) -> u32 {
    ((n) << FRPER_SHIFT) & 0x1FFF0000
}

// DMA controller register
pub const RX_DMA_ENABLE: u32 = 1 << 0;
pub const TX_DMA_ENABLE: u32 = 1 << 1;

pub const RDMAE_SHIFT: u32 = 0;
pub const TDMAE_SHIFT: u32 = 1;

// Interrupt Register
pub const RX_SERVICE_INT: u32 = 1 << 0;
pub const RX_OVERRUN_ERROR_INT: u32 = 1 << 1;
pub const RX_FSYNC_ERR_INT: u32 = 1 << 2;
pub const RX_FSYNC_INT: u32 = 1 << 3;
pub const TX_SERVICE_INT: u32 = 1 << 4;
pub const TX_UNDERRUN_ERR_INT: u32 = 1 << 5;
pub const TX_FSYNC_ERR_INT: u32 = 1 << 6;
pub const TX_FSYNC_INT: u32 = 1 << 7;
pub const ALL_INT: u32 = 0x000000ff;

// MSP test control register
pub const MSP_ITCR_ITEN: u32 = 1 << 0;
pub const MSP_ITCR_TESTFIFO: u32 = 1 << 1;

pub const RMCEN_BIT: u32 = 0;
pub const RMCSF_BIT: u32 = 1;
pub const RCMPM_BIT: u32 = 3;
pub const TMCEN_BIT: u32 = 5;
pub const TNCSF_BIT: u32 = 6;

// Single or dual phase mode
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_phase_mode {
    MSP_SINGLE_PHASE = 0,
    MSP_DUAL_PHASE = 1,
}

// Frame length
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_frame_length {
    MSP_FRAME_LEN_1 = 0,
    MSP_FRAME_LEN_2 = 1,
    MSP_FRAME_LEN_4 = 3,
    MSP_FRAME_LEN_8 = 7,
    MSP_FRAME_LEN_12 = 11,
    MSP_FRAME_LEN_16 = 15,
    MSP_FRAME_LEN_20 = 19,
    MSP_FRAME_LEN_32 = 31,
    MSP_FRAME_LEN_48 = 47,
    MSP_FRAME_LEN_64 = 63,
}

// Element length
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_elem_length {
    MSP_ELEM_LEN_8 = 0,
    MSP_ELEM_LEN_10 = 1,
    MSP_ELEM_LEN_12 = 2,
    MSP_ELEM_LEN_14 = 3,
    MSP_ELEM_LEN_16 = 4,
    MSP_ELEM_LEN_20 = 5,
    MSP_ELEM_LEN_24 = 6,
    MSP_ELEM_LEN_32 = 7,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_data_xfer_width {
    MSP_DATA_TRANSFER_WIDTH_BYTE = 0,
    MSP_DATA_TRANSFER_WIDTH_HALFWORD = 1,
    MSP_DATA_TRANSFER_WIDTH_WORD = 2,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_frame_sync {
    MSP_FSYNC_UNIGNORE = 0,
    MSP_FSYNC_IGNORE = 1,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_phase2_start_mode {
    MSP_PHASE2_START_MODE_IMEDIATE = 0,
    MSP_PHASE2_START_MODE_FSYNC = 1,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_btf {
    MSP_BTF_MS_BIT_FIRST = 0,
    MSP_BTF_LS_BIT_FIRST = 1,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_fsync_pol {
    MSP_FSYNC_POL_ACT_HI = 0,
    MSP_FSYNC_POL_ACT_LO = 1,
}

// Data delay (in bit clock cycles)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_delay {
    MSP_DELAY_0 = 0,
    MSP_DELAY_1 = 1,
    MSP_DELAY_2 = 2,
    MSP_DELAY_3 = 3,
}

// Configurations of clocks (transmit, receive or sample rate generator)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_edge {
    MSP_FALLING_EDGE = 0,
    MSP_RISING_EDGE = 1,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_hws {
    MSP_SWAP_NONE = 0,
    MSP_SWAP_BYTE_PER_WORD = 1,
    MSP_SWAP_BYTE_PER_HALF_WORD = 2,
    MSP_SWAP_HALF_WORD_PER_WORD = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_compress_mode {
    MSP_COMPRESS_MODE_LINEAR = 0,
    MSP_COMPRESS_MODE_MU_LAW = 2,
    MSP_COMPRESS_MODE_A_LAW = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_expand_mode {
    MSP_EXPAND_MODE_LINEAR = 0,
    MSP_EXPAND_MODE_LINEAR_SIGNED = 1,
    MSP_EXPAND_MODE_MU_LAW = 2,
    MSP_EXPAND_MODE_A_LAW = 3,
}

pub const MSP_FRAME_PERIOD_IN_MONO_MODE: u32 = 256;
pub const MSP_FRAME_PERIOD_IN_STEREO_MODE: u32 = 32;
pub const MSP_FRAME_WIDTH_IN_STEREO_MODE: u32 = 16;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_protocol {
    MSP_I2S_PROTOCOL = 0,
    MSP_PCM_PROTOCOL = 1,
    MSP_PCM_COMPAND_PROTOCOL = 2,
    MSP_INVALID_PROTOCOL = 3,
}

// No of registers to backup during suspend resume
pub const MAX_MSP_BACKUP_REGS: usize = 36;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum i2s_direction_t {
    MSP_DIR_TX = 0x01,
    MSP_DIR_RX = 0x02,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_data_size {
    MSP_DATA_BITS_DEFAULT = -1,
    MSP_DATA_BITS_8 = 0x00,
    MSP_DATA_BITS_10 = 0x01,
    MSP_DATA_BITS_12 = 0x02,
    MSP_DATA_BITS_14 = 0x03,
    MSP_DATA_BITS_16 = 0x04,
    MSP_DATA_BITS_20 = 0x05,
    MSP_DATA_BITS_24 = 0x06,
    MSP_DATA_BITS_32 = 0x07,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_state {
    MSP_STATE_IDLE = 0,
    MSP_STATE_CONFIGURED = 1,
    MSP_STATE_RUNNING = 2,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum msp_rx_comparison_enable_mode {
    MSP_COMPARISON_DISABLED = 0,
    MSP_COMPARISON_NONEQUAL_ENABLED = 2,
    MSP_COMPARISON_EQUAL_ENABLED = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct msp_multichannel_config {
    pub rx_multichannel_enable: bool,
    pub tx_multichannel_enable: bool,
    pub rx_comparison_enable_mode: msp_rx_comparison_enable_mode,
    pub padding: u8,
    pub comparison_value: u32,
    pub comparison_mask: u32,
    pub rx_channel_0_enable: u32,
    pub rx_channel_1_enable: u32,
    pub rx_channel_2_enable: u32,
    pub rx_channel_3_enable: u32,
    pub tx_channel_0_enable: u32,
    pub tx_channel_1_enable: u32,
    pub tx_channel_2_enable: u32,
    pub tx_channel_3_enable: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct msp_protdesc {
    pub rx_phase_mode: u32,
    pub tx_phase_mode: u32,
    pub rx_phase2_start_mode: u32,
    pub tx_phase2_start_mode: u32,
    pub rx_byte_order: u32,
    pub tx_byte_order: u32,
    pub rx_frame_len_1: u32,
    pub rx_frame_len_2: u32,
    pub tx_frame_len_1: u32,
    pub tx_frame_len_2: u32,
    pub rx_elem_len_1: u32,
    pub rx_elem_len_2: u32,
    pub tx_elem_len_1: u32,
    pub tx_elem_len_2: u32,
    pub rx_data_delay: u32,
    pub tx_data_delay: u32,
    pub rx_clk_pol: u32,
    pub tx_clk_pol: u32,
    pub rx_fsync_pol: u32,
    pub tx_fsync_pol: u32,
    pub rx_half_word_swap: u32,
    pub tx_half_word_swap: u32,
    pub compression_mode: u32,
    pub expansion_mode: u32,
    pub frame_sync_ignore: u32,
    pub frame_period: u32,
    pub frame_width: u32,
    pub clocks_per_frame: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ux500_msp_config {
    pub f_inputclk: u32,
    pub rx_clk_sel: u32,
    pub tx_clk_sel: u32,
    pub srg_clk_sel: u32,
    pub rx_fsync_pol: u32,
    pub tx_fsync_pol: u32,
    pub rx_fsync_sel: u32,
    pub tx_fsync_sel: u32,
    pub rx_fifo_config: u32,
    pub tx_fifo_config: u32,
    pub loopback_enable: u32,
    pub tx_data_enable: u32,
    pub default_protdesc: u32,
    pub protdesc: msp_protdesc,
    pub multichannel_configured: i32,
    pub multichannel_config: msp_multichannel_config,
    pub direction: u32,
    pub protocol: u32,
    pub frame_freq: u32,
    pub data_size: msp_data_size,
    pub def_elem_len: u32,
    pub iodelay: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct ux500_msp {
    pub id: i32,
    pub registers: *mut c_void,
    pub dev: *mut c_void,
    pub tx_rx_addr: u64,
    pub msp_state: msp_state,
    pub def_elem_len: i32,
    pub dir_busy: u32,
    pub loopback_enable: i32,
    pub f_bitclk: u32,
}

extern "C" {
    pub fn ux500_msp_i2s_init_msp(pdev: *mut c_void, msp_p: *mut *mut ux500_msp) -> i32;
    pub fn ux500_msp_i2s_cleanup_msp(pdev: *mut c_void, msp: *mut ux500_msp);
    pub fn ux500_msp_i2s_open(msp: *mut ux500_msp, config: *mut ux500_msp_config) -> i32;
    pub fn ux500_msp_i2s_close(msp: *mut ux500_msp, dir: u32) -> i32;
    pub fn ux500_msp_i2s_trigger(msp: *mut ux500_msp, cmd: i32, direction: i32) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
