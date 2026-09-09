/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * SharpSL NAND support
 *
 * Copyright (C) 2008 Dmitry Baryshkov
 */

/* Dependencies supplied by the Linux MTD headers. */

#[repr(C)]
pub struct sharpsl_nand_platform_data {
    pub badblock_pattern: *mut nand_bbt_descr,
    pub ecc_layout: *const mtd_ooblayout_ops,
    pub partitions: *mut mtd_partition,
    pub nr_partitions: core::ffi::c_uint,
    pub part_parsers: *const *const core::ffi::c_char,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
