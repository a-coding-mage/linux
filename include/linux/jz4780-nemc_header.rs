/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * JZ4780 NAND/external memory controller (NEMC)
 *
 * Copyright (c) 2015 Imagination Technologies
 * Author: Alex Smith <alex@alex-smith.me.uk>
 */

// Dependency equivalent of: #include <linux/types.h>

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/*
 * Number of NEMC banks. Note that there are actually 6, but they are numbered
 * from 1.
 */
pub const JZ4780_NEMC_NUM_BANKS: u32 = 7;

/**
 * enum jz4780_nemc_bank_type - device types which can be connected to a bank
 * @JZ4780_NEMC_BANK_SRAM: SRAM
 * @JZ4780_NEMC_BANK_NAND: NAND
 */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum jz4780_nemc_bank_type {
    JZ4780_NEMC_BANK_SRAM = 0,
    JZ4780_NEMC_BANK_NAND = 1,
}

extern "C" {
    pub fn jz4780_nemc_num_banks(dev: *mut device) -> ::core::ffi::c_uint;

    pub fn jz4780_nemc_set_type(
        dev: *mut device,
        bank: ::core::ffi::c_uint,
        type_: jz4780_nemc_bank_type,
    );
    pub fn jz4780_nemc_assert(
        dev: *mut device,
        bank: ::core::ffi::c_uint,
        assert_: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
