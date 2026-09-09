/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2006 Micron Technology Inc.
 */

// Dependency intent: declarations from <linux/mtd/partitions.h> are supplied
// by other translated dependencies.

pub const GPMC_BCH_NUM_REMAINDER: usize = 8;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum nand_io {
    NAND_OMAP_PREFETCH_POLLED = 0, // prefetch polled mode, default
    NAND_OMAP_POLLED,              // polled mode, without prefetch
    NAND_OMAP_PREFETCH_DMA,        // prefetch enabled sDMA mode
    NAND_OMAP_PREFETCH_IRQ,        // prefetch enabled irq mode
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum omap_ecc {
    /*
     * 1-bit ECC: calculation and correction by SW
     * ECC stored at end of spare area
     */
    OMAP_ECC_HAM1_CODE_SW = 0,

    /*
     * 1-bit ECC: calculation by GPMC, Error detection by Software
     * ECC layout compatible with ROM code layout
     */
    OMAP_ECC_HAM1_CODE_HW,
    /* 4-bit  ECC calculation by GPMC, Error detection by Software */
    OMAP_ECC_BCH4_CODE_HW_DETECTION_SW,
    /* 4-bit  ECC calculation by GPMC, Error detection by ELM */
    OMAP_ECC_BCH4_CODE_HW,
    /* 8-bit  ECC calculation by GPMC, Error detection by Software */
    OMAP_ECC_BCH8_CODE_HW_DETECTION_SW,
    /* 8-bit  ECC calculation by GPMC, Error detection by ELM */
    OMAP_ECC_BCH8_CODE_HW,
    /* 16-bit ECC calculation by GPMC, Error detection by ELM */
    OMAP_ECC_BCH16_CODE_HW,
}

#[repr(C)]
pub struct gpmc_nand_regs {
    pub gpmc_nand_command: *mut core::ffi::c_void,
    pub gpmc_nand_address: *mut core::ffi::c_void,
    pub gpmc_nand_data: *mut core::ffi::c_void,
    pub gpmc_prefetch_config1: *mut core::ffi::c_void,
    pub gpmc_prefetch_config2: *mut core::ffi::c_void,
    pub gpmc_prefetch_control: *mut core::ffi::c_void,
    pub gpmc_prefetch_status: *mut core::ffi::c_void,
    pub gpmc_ecc_config: *mut core::ffi::c_void,
    pub gpmc_ecc_control: *mut core::ffi::c_void,
    pub gpmc_ecc_size_config: *mut core::ffi::c_void,
    pub gpmc_ecc1_result: *mut core::ffi::c_void,
    pub gpmc_bch_result0: [*mut core::ffi::c_void; GPMC_BCH_NUM_REMAINDER],
    pub gpmc_bch_result1: [*mut core::ffi::c_void; GPMC_BCH_NUM_REMAINDER],
    pub gpmc_bch_result2: [*mut core::ffi::c_void; GPMC_BCH_NUM_REMAINDER],
    pub gpmc_bch_result3: [*mut core::ffi::c_void; GPMC_BCH_NUM_REMAINDER],
    pub gpmc_bch_result4: [*mut core::ffi::c_void; GPMC_BCH_NUM_REMAINDER],
    pub gpmc_bch_result5: [*mut core::ffi::c_void; GPMC_BCH_NUM_REMAINDER],
    pub gpmc_bch_result6: [*mut core::ffi::c_void; GPMC_BCH_NUM_REMAINDER],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
