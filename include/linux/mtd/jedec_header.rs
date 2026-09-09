/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright © 2000-2010 David Woodhouse <dwmw2@infradead.org>
 *              Steven J. Hill <sjhill@realitydiluted.com>
 *              Thomas Gleixner <tglx@kernel.org>
 *
 * Contains all JEDEC related definitions
 */

#[repr(C, packed)]
pub struct jedec_ecc_info {
    pub ecc_bits: u8,
    pub codeword_size: u8,
    pub bb_per_lun: u16,
    pub block_endurance: u16,
    pub reserved: [u8; 2],
}

/* JEDEC features */
pub const JEDEC_FEATURE_16_BIT_BUS: u32 = 1 << 0;

/* JEDEC Optional Commands */
pub const JEDEC_OPT_CMD_READ_CACHE: u32 = 1 << 1;

#[repr(C, packed)]
pub struct nand_jedec_params {
    /* rev info and features block */
    /* 'J' 'E' 'S' 'D'  */
    pub sig: [u8; 4],
    pub revision: u16,
    pub features: u16,
    pub opt_cmd: [u8; 3],
    pub sec_cmd: u16,
    pub num_of_param_pages: u8,
    pub reserved0: [u8; 18],

    /* manufacturer information block */
    pub manufacturer: [u8; 12],
    pub model: [u8; 20],
    pub jedec_id: [u8; 6],
    pub reserved1: [u8; 10],

    /* memory organization block */
    pub byte_per_page: u32,
    pub spare_bytes_per_page: u16,
    pub reserved2: [u8; 6],
    pub pages_per_block: u32,
    pub blocks_per_lun: u32,
    pub lun_count: u8,
    pub addr_cycles: u8,
    pub bits_per_cell: u8,
    pub programs_per_page: u8,
    pub multi_plane_addr: u8,
    pub multi_plane_op_attr: u8,
    pub reserved3: [u8; 38],

    /* electrical parameter block */
    pub async_sdr_speed_grade: u16,
    pub toggle_ddr_speed_grade: u16,
    pub sync_ddr_speed_grade: u16,
    pub async_sdr_features: u8,
    pub toggle_ddr_features: u8,
    pub sync_ddr_features: u8,
    pub t_prog: u16,
    pub t_bers: u16,
    pub t_r: u16,
    pub t_r_multi_plane: u16,
    pub t_ccs: u16,
    pub io_pin_capacitance_typ: u16,
    pub input_pin_capacitance_typ: u16,
    pub clk_pin_capacitance_typ: u16,
    pub driver_strength_support: u8,
    pub t_adl: u16,
    pub reserved4: [u8; 36],

    /* ECC and endurance block */
    pub guaranteed_good_blocks: u8,
    pub guaranteed_block_endurance: u16,
    pub ecc_info: [jedec_ecc_info; 4],
    pub reserved5: [u8; 29],

    /* reserved */
    pub reserved6: [u8; 148],

    /* vendor */
    pub vendor_rev_num: u16,
    pub reserved7: [u8; 88],

    /* CRC for Parameter Page */
    pub crc: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
