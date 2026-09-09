// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

// Dependencies supplied by the surrounding driver translation unit:
// cc_driver.h, cc_sram_mgr.h

use crate::{
    cc_drvdata, cc_hw_desc, device, cc_ioread, dev_err, dev_dbg, drvdata_to_dev,
    hw_desc_init, set_din_const, set_dout_sram, set_flow_mode, BYPASS,
    CC_CC_SRAM_SIZE, CC_HW_REV_712, CC_REG, HOST_SEP_SRAM_THRESHOLD,
    NULL_SRAM_ADDR,
};

/**
 * cc_sram_mgr_init() - Initializes SRAM pool.
 *      The pool starts right at the beginning of SRAM.
 *      Returns zero for success, negative value otherwise.
 *
 * @drvdata: Associated device driver context
 *
 * Return:
 * 0 for success, negative error code for failure.
 */
pub unsafe fn cc_sram_mgr_init(drvdata: *mut cc_drvdata) -> i32 {
    let mut start: u32 = 0;
    let dev: *mut device = drvdata_to_dev(drvdata);

    if (*drvdata).hw_rev < CC_HW_REV_712 {
        /* Pool starts after ROM bytes */
        start = cc_ioread(drvdata, CC_REG(HOST_SEP_SRAM_THRESHOLD));
        if (start & 0x3) != 0 {
            dev_err(dev, "Invalid SRAM offset 0x%x\n", start);
            return -22; // -EINVAL
        }
    }

    (*drvdata).sram_free_offset = start;
    0
}

/**
 * cc_sram_alloc() - Allocate buffer from SRAM pool.
 *
 * @drvdata: Associated device driver context
 * @size: The requested numer of bytes to allocate
 *
 * Return:
 * Address offset in SRAM or NULL_SRAM_ADDR for failure.
 */
pub unsafe fn cc_sram_alloc(drvdata: *mut cc_drvdata, size: u32) -> u32 {
    let dev: *mut device = drvdata_to_dev(drvdata);
    let mut p: u32;

    if (size & 0x3) != 0 {
        dev_err(
            dev,
            "Requested buffer size (%u) is not multiple of 4",
            size,
        );
        return NULL_SRAM_ADDR;
    }
    if size > (CC_CC_SRAM_SIZE - (*drvdata).sram_free_offset) {
        dev_err(
            dev,
            "Not enough space to allocate %u B (at offset %u)\n",
            size,
            (*drvdata).sram_free_offset,
        );
        return NULL_SRAM_ADDR;
    }

    p = (*drvdata).sram_free_offset;
    (*drvdata).sram_free_offset = (*drvdata).sram_free_offset.wrapping_add(size);
    dev_dbg(dev, "Allocated %u B @ %u\n", size, p);
    p
}

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
pub unsafe fn cc_set_sram_desc(
    src: *const u32,
    dst: u32,
    nelement: u32,
    seq: *mut cc_hw_desc,
    seq_len: *mut u32,
) {
    let mut i: u32;
    let mut idx: u32 = *seq_len;

    i = 0;
    while i < nelement {
        hw_desc_init(seq.add(idx as usize));
        set_din_const(seq.add(idx as usize), *src.add(i as usize), core::mem::size_of::<u32>());
        set_dout_sram(
            seq.add(idx as usize),
            dst.wrapping_add(i.wrapping_mul(core::mem::size_of::<u32>() as u32)),
            core::mem::size_of::<u32>(),
        );
        set_flow_mode(seq.add(idx as usize), BYPASS);
        i = i.wrapping_add(1);
        idx = idx.wrapping_add(1);
    }

    *seq_len = idx;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
