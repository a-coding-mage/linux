/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Xilinx SD-FEC
 *
 * Copyright (C) 2019 Xilinx, Inc.
 *
 * Description:
 * This driver is developed for SDFEC16 IP. It provides a char device
 * in sysfs and supports file operations like open(), close() and ioctl().
 */

// Shared LDPC Tables
pub const XSDFEC_LDPC_SC_TABLE_ADDR_BASE: u32 = 0x10000;
pub const XSDFEC_LDPC_SC_TABLE_ADDR_HIGH: u32 = 0x10400;
pub const XSDFEC_LDPC_LA_TABLE_ADDR_BASE: u32 = 0x18000;
pub const XSDFEC_LDPC_LA_TABLE_ADDR_HIGH: u32 = 0x19000;
pub const XSDFEC_LDPC_QC_TABLE_ADDR_BASE: u32 = 0x20000;
pub const XSDFEC_LDPC_QC_TABLE_ADDR_HIGH: u32 = 0x28000;

// LDPC tables depth
pub const XSDFEC_SC_TABLE_DEPTH: u32 =
    XSDFEC_LDPC_SC_TABLE_ADDR_HIGH - XSDFEC_LDPC_SC_TABLE_ADDR_BASE;
pub const XSDFEC_LA_TABLE_DEPTH: u32 =
    XSDFEC_LDPC_LA_TABLE_ADDR_HIGH - XSDFEC_LDPC_LA_TABLE_ADDR_BASE;
pub const XSDFEC_QC_TABLE_DEPTH: u32 =
    XSDFEC_LDPC_QC_TABLE_ADDR_HIGH - XSDFEC_LDPC_QC_TABLE_ADDR_BASE;

#[repr(i32)]
pub enum xsdfec_code { XSDFEC_TURBO_CODE = 0, XSDFEC_LDPC_CODE }

#[repr(i32)]
pub enum xsdfec_order { XSDFEC_MAINTAIN_ORDER = 0, XSDFEC_OUT_OF_ORDER }

#[repr(i32)]
pub enum xsdfec_turbo_alg {
    XSDFEC_MAX_SCALE = 0,
    XSDFEC_MAX_STAR,
    XSDFEC_TURBO_ALG_MAX,
}

#[repr(i32)]
pub enum xsdfec_state {
    XSDFEC_INIT = 0,
    XSDFEC_STARTED,
    XSDFEC_STOPPED,
    XSDFEC_NEEDS_RESET,
    XSDFEC_PL_RECONFIGURE,
}

#[repr(i32)]
pub enum xsdfec_axis_width {
    XSDFEC_1x128b = 1,
    XSDFEC_2x128b = 2,
    XSDFEC_4x128b = 4,
}

#[repr(i32)]
pub enum xsdfec_axis_word_include {
    XSDFEC_FIXED_VALUE = 0,
    XSDFEC_IN_BLOCK,
    XSDFEC_PER_AXI_TRANSACTION,
    XSDFEC_AXIS_WORDS_INCLUDE_MAX,
}

#[repr(C)]
pub struct xsdfec_turbo { pub alg: u32, pub scale: u8 }

#[repr(C)]
pub struct xsdfec_ldpc_params {
    pub n: u32, pub k: u32, pub psize: u32, pub nlayers: u32,
    pub nqc: u32, pub nmqc: u32, pub nm: u32, pub norm_type: u32,
    pub no_packing: u32, pub special_qc: u32, pub no_final_parity: u32,
    pub max_schedule: u32, pub sc_off: u32, pub la_off: u32, pub qc_off: u32,
    pub sc_table: *mut u32, pub la_table: *mut u32, pub qc_table: *mut u32,
    pub code_id: u16,
}

#[repr(C)]
pub struct xsdfec_status { pub state: u32, pub activity: i8 }

#[repr(C)]
pub struct xsdfec_irq { pub enable_isr: i8, pub enable_ecc_isr: i8 }

#[repr(C)]
pub struct xsdfec_config {
    pub code: u32, pub order: u32, pub din_width: u32,
    pub din_word_include: u32, pub dout_width: u32,
    pub dout_word_include: u32, pub irq: xsdfec_irq,
    pub bypass: i8, pub code_wr_protect: i8,
}

#[repr(C)]
pub struct xsdfec_stats { pub isr_err_count: u32, pub cecc_count: u32, pub uecc_count: u32 }

#[repr(C)]
pub struct xsdfec_ldpc_param_table_sizes { pub sc_size: u32, pub la_size: u32, pub qc_size: u32 }

// XSDFEC IOCTL List
pub const XSDFEC_MAGIC: u8 = b'f';

// These ioctl values depend on the platform's Linux ioctl encoding macros.
macro_rules! XSDFEC_START_DEV { () => { _IO(XSDFEC_MAGIC, 0) }; }
macro_rules! XSDFEC_STOP_DEV { () => { _IO(XSDFEC_MAGIC, 1) }; }
macro_rules! XSDFEC_GET_STATUS { () => { _IOR(XSDFEC_MAGIC, 2, xsdfec_status) }; }
macro_rules! XSDFEC_SET_IRQ { () => { _IOW(XSDFEC_MAGIC, 3, xsdfec_irq) }; }
macro_rules! XSDFEC_SET_TURBO { () => { _IOW(XSDFEC_MAGIC, 4, xsdfec_turbo) }; }
macro_rules! XSDFEC_ADD_LDPC_CODE_PARAMS { () => { _IOW(XSDFEC_MAGIC, 5, xsdfec_ldpc_params) }; }
macro_rules! XSDFEC_GET_CONFIG { () => { _IOR(XSDFEC_MAGIC, 6, xsdfec_config) }; }
macro_rules! XSDFEC_GET_TURBO { () => { _IOR(XSDFEC_MAGIC, 7, xsdfec_turbo) }; }
macro_rules! XSDFEC_SET_ORDER { () => { _IOW(XSDFEC_MAGIC, 8, ::core::ffi::c_ulong) }; }
macro_rules! XSDFEC_SET_BYPASS { () => { _IOW(XSDFEC_MAGIC, 9, bool) }; }
macro_rules! XSDFEC_IS_ACTIVE { () => { _IOR(XSDFEC_MAGIC, 10, bool) }; }
macro_rules! XSDFEC_CLEAR_STATS { () => { _IO(XSDFEC_MAGIC, 11) }; }
macro_rules! XSDFEC_GET_STATS { () => { _IOR(XSDFEC_MAGIC, 12, xsdfec_stats) }; }
macro_rules! XSDFEC_SET_DEFAULT_CONFIG { () => { _IO(XSDFEC_MAGIC, 13) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
