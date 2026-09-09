/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright © 2000-2010 David Woodhouse <dwmw2@infradead.org>
 *                         Steven J. Hill <sjhill@realitydiluted.com>
 *                         Thomas Gleixner <tglx@kernel.org>
 *
 * Contains all ONFI related definitions
 */

/* ONFI version bits */
pub const ONFI_VERSION_1_0: u32 = 1 << 1;
pub const ONFI_VERSION_2_0: u32 = 1 << 2;
pub const ONFI_VERSION_2_1: u32 = 1 << 3;
pub const ONFI_VERSION_2_2: u32 = 1 << 4;
pub const ONFI_VERSION_2_3: u32 = 1 << 5;
pub const ONFI_VERSION_3_0: u32 = 1 << 6;
pub const ONFI_VERSION_3_1: u32 = 1 << 7;
pub const ONFI_VERSION_3_2: u32 = 1 << 8;
pub const ONFI_VERSION_4_0: u32 = 1 << 9;

/* ONFI features */
pub const ONFI_FEATURE_16_BIT_BUS: u32 = 1 << 0;
pub const ONFI_FEATURE_NV_DDR: u32 = 1 << 5;
pub const ONFI_FEATURE_EXT_PARAM_PAGE: u32 = 1 << 7;

/* ONFI timing mode, used in both asynchronous and synchronous mode */
pub const ONFI_DATA_INTERFACE_SDR: u32 = 0;
pub const ONFI_DATA_INTERFACE_NVDDR: u32 = 1 << 4;
pub const ONFI_DATA_INTERFACE_NVDDR2: u32 = 1 << 5;
pub const ONFI_TIMING_MODE_0: u32 = 1 << 0;
pub const ONFI_TIMING_MODE_1: u32 = 1 << 1;
pub const ONFI_TIMING_MODE_2: u32 = 1 << 2;
pub const ONFI_TIMING_MODE_3: u32 = 1 << 3;
pub const ONFI_TIMING_MODE_4: u32 = 1 << 4;
pub const ONFI_TIMING_MODE_5: u32 = 1 << 5;
pub const ONFI_TIMING_MODE_UNKNOWN: u32 = 1 << 6;
macro_rules! ONFI_TIMING_MODE_PARAM { ($x:expr) => { (($x) & 0xF) }; }

/* ONFI feature number/address */
pub const ONFI_FEATURE_NUMBER: u32 = 256;
pub const ONFI_FEATURE_ADDR_TIMING_MODE: u32 = 0x1;

/* Vendor-specific feature address (Micron) */
pub const ONFI_FEATURE_ADDR_READ_RETRY: u32 = 0x89;
pub const ONFI_FEATURE_ON_DIE_ECC: u32 = 0x90;
pub const ONFI_FEATURE_ON_DIE_ECC_EN: u32 = 1 << 3;

/* ONFI subfeature parameters length */
pub const ONFI_SUBFEATURE_PARAM_LEN: u32 = 4;

/* ONFI optional commands SET/GET FEATURES supported? */
pub const ONFI_OPT_CMD_READ_CACHE: u32 = 1 << 1;
pub const ONFI_OPT_CMD_SET_GET_FEATURES: u32 = 1 << 2;

#[repr(C, packed)]
pub struct nand_onfi_params {
    pub sig: [u8; 4], pub revision: u16, pub features: u16, pub opt_cmd: u16,
    pub reserved0: [u8; 2], pub ext_param_page_length: u16, pub num_of_param_pages: u8,
    pub reserved1: [u8; 17], pub manufacturer: [i8; 12], pub model: [i8; 20],
    pub jedec_id: u8, pub date_code: u16, pub reserved2: [u8; 13],
    pub byte_per_page: u32, pub spare_bytes_per_page: u16, pub data_bytes_per_ppage: u32,
    pub spare_bytes_per_ppage: u16, pub pages_per_block: u32, pub blocks_per_lun: u32,
    pub lun_count: u8, pub addr_cycles: u8, pub bits_per_cell: u8, pub bb_per_lun: u16,
    pub block_endurance: u16, pub guaranteed_good_blocks: u8, pub guaranteed_block_endurance: u16,
    pub programs_per_page: u8, pub ppage_attr: u8, pub ecc_bits: u8, pub interleaved_bits: u8,
    pub interleaved_ops: u8, pub reserved3: [u8; 13], pub io_pin_capacitance_max: u8,
    pub sdr_timing_modes: u16, pub program_cache_timing_mode: u16, pub t_prog: u16,
    pub t_bers: u16, pub t_r: u16, pub t_ccs: u16, pub nvddr_timing_modes: u8,
    pub nvddr2_timing_modes: u8, pub nvddr_nvddr2_features: u8, pub clk_pin_capacitance_typ: u16,
    pub io_pin_capacitance_typ: u16, pub input_pin_capacitance_typ: u16,
    pub input_pin_capacitance_max: u8, pub driver_strength_support: u8, pub t_int_r: u16,
    pub t_adl: u16, pub reserved4: [u8; 8], pub vendor_revision: u16, pub vendor: [u8; 88],
    pub crc: u16,
}

pub const ONFI_CRC_BASE: u16 = 0x4F4E;

#[repr(C, packed)]
pub struct onfi_ext_ecc_info { pub ecc_bits: u8, pub codeword_size: u8, pub bb_per_lun: u16, pub block_endurance: u16, pub reserved: [u8; 2] }
pub const ONFI_SECTION_TYPE_0: u8 = 0;
pub const ONFI_SECTION_TYPE_1: u8 = 1;
pub const ONFI_SECTION_TYPE_2: u8 = 2;
#[repr(C, packed)] pub struct onfi_ext_section { pub type_: u8, pub length: u8 }
pub const ONFI_EXT_SECTION_MAX: usize = 8;

#[repr(C, packed)]
pub struct onfi_ext_param_page { pub crc: u16, pub sig: [u8; 4], pub reserved0: [u8; 10], pub sections: [onfi_ext_section; ONFI_EXT_SECTION_MAX] }

#[repr(C)]
pub struct onfi_params {
    pub version: i32, pub tPROG: u16, pub tBERS: u16, pub tR: u16, pub tCCS: u16,
    pub fast_tCAD: bool, pub sdr_timing_modes: u16, pub nvddr_timing_modes: u16,
    pub vendor_revision: u16, pub vendor: [u8; 88],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
