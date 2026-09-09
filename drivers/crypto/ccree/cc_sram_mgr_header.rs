/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

/* C header guard: __CC_SRAM_MGR_H__ */

/* Build-time default preserved from CC_CC_SRAM_SIZE. */
#[cfg(not(CC_CC_SRAM_SIZE))]
pub const CC_CC_SRAM_SIZE: u32 = 4096;

#[repr(C)]
pub struct cc_drvdata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cc_hw_desc {
    _private: [u8; 0],
}

pub const NULL_SRAM_ADDR: u32 = u32::MAX;

/**
 * cc_sram_mgr_init() - Initializes SRAM pool.
 * The first X bytes of SRAM are reserved for ROM usage, hence, pool
 * starts right after X bytes.
 *
 * @drvdata: Associated device driver context
 *
 * Return:
 * Zero for success, negative value otherwise.
 */
extern "C" {
    pub fn cc_sram_mgr_init(drvdata: *mut cc_drvdata) -> ::core::ffi::c_int;

    /**
     * cc_sram_alloc() - Allocate buffer from SRAM pool.
     *
     * @drvdata: Associated device driver context
     * @size: The requested bytes to allocate
     *
     * Return:
     * Address offset in SRAM or NULL_SRAM_ADDR for failure.
     */
    pub fn cc_sram_alloc(drvdata: *mut cc_drvdata, size: u32) -> u32;

    /**
     * cc_set_sram_desc() - Create const descriptors sequence to
     *\tset values in given array into SRAM.
     * Note: each const value can't exceed word size.
     *
     * @src:      A pointer to array of words to set as consts.
     * @dst:      The target SRAM buffer to set into
     * @nelement:      The number of words in "src" array
     * @seq:      A pointer to the given IN/OUT descriptor sequence
     * @seq_len:      A pointer to the given IN/OUT sequence length
     */
    pub fn cc_set_sram_desc(
        src: *const u32,
        dst: u32,
        nelement: ::core::ffi::c_uint,
        seq: *mut cc_hw_desc,
        seq_len: *mut ::core::ffi::c_uint,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
