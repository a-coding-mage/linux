/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * NAND family Bad Block Management (BBM) header file
 *   - Bad Block Table (BBT) implementation
 *
 * Copyright © 2005 Samsung Electronics
 * Kyungmin Park <kyungmin.park@samsung.com>
 *
 * Copyright © 2000-2005
 * Thomas Gleixner <tglx@linuxtronix.de>
 */

/* The maximum number of NAND chips in an array */
pub const NAND_MAX_CHIPS: usize = 8;

/**
 * struct nand_bbt_descr - bad block table descriptor
 * @options: options for this descriptor
 * @pages: the page(s) where we find the bbt, used with option BBT_ABSPAGE
 * @offs: offset of the pattern in the oob area of the page
 * @veroffs: offset of the bbt version counter in the oob area of the page
 * @version: version read from the bbt page during scan
 * @len: length of the pattern, if 0 no pattern check is performed
 * @maxblocks: maximum number of blocks to search for a bbt
 * @reserved_block_code: reserved block marker in the stored bbt
 * @pattern: pattern to identify the bad block table or factory marked blocks
 */
#[repr(C)]
pub struct nand_bbt_descr {
    pub options: i32,
    pub pages: [i32; NAND_MAX_CHIPS],
    pub offs: i32,
    pub veroffs: i32,
    pub version: [u8; NAND_MAX_CHIPS],
    pub len: i32,
    pub maxblocks: i32,
    pub reserved_block_code: i32,
    pub pattern: *mut u8,
}

/* Options for the bad block table descriptors */
pub const NAND_BBT_NRBITS_MSK: i32 = 0x0000000F;
pub const NAND_BBT_1BIT: i32 = 0x00000001;
pub const NAND_BBT_2BIT: i32 = 0x00000002;
pub const NAND_BBT_4BIT: i32 = 0x00000004;
pub const NAND_BBT_8BIT: i32 = 0x00000008;
pub const NAND_BBT_LASTBLOCK: i32 = 0x00000010;
pub const NAND_BBT_ABSPAGE: i32 = 0x00000020;
pub const NAND_BBT_PERCHIP: i32 = 0x00000080;
pub const NAND_BBT_VERSION: i32 = 0x00000100;
pub const NAND_BBT_CREATE: i32 = 0x00000200;
pub const NAND_BBT_CREATE_EMPTY: i32 = 0x00000400;
pub const NAND_BBT_WRITE: i32 = 0x00002000;
pub const NAND_BBT_SAVECONTENT: i32 = 0x00004000;
pub const NAND_BBT_USE_FLASH: i32 = 0x00020000;
pub const NAND_BBT_NO_OOB: i32 = 0x00040000;
pub const NAND_BBT_NO_OOB_BBM: i32 = 0x00080000;
pub const NAND_BBT_DYNAMICSTRUCT: u32 = 0x80000000;
pub const NAND_BBT_SCAN_MAXBLOCKS: i32 = 4;

/* Bad block scanning errors */
pub const ONENAND_BBT_READ_ERROR: i32 = 1;
pub const ONENAND_BBT_READ_ECC_ERROR: i32 = 2;
pub const ONENAND_BBT_READ_FATAL_ERROR: i32 = 4;

/** struct bbm_info - [GENERIC] Bad Block Table data structure */
#[repr(C)]
pub struct bbm_info {
    pub bbt_erase_shift: i32,
    pub options: i32,
    pub bbt: *mut u8,
    pub isbad_bbt: Option<unsafe extern "C" fn(mtd: *mut mtd_info, ofs: i64, allowbbt: i32) -> i32>,
    pub badblock_pattern: *mut nand_bbt_descr,
    pub priv_: *mut core::ffi::c_void,
}

/* External dependency supplied by the surrounding translation unit. */
extern "C" {
    pub fn onenand_default_bbt(mtd: *mut mtd_info) -> i32;
}

/* External dependency type supplied by the surrounding translation unit. */
#[repr(C)]
pub struct mtd_info {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
