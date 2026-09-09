// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP4 PRM instance functions
 *
 * Copyright (C) 2009 Nokia Corporation
 * Copyright (C) 2011 Texas Instruments, Inc.
 * Paul Walmsley
 */

// Linux and OMAP header dependencies are supplied by the surrounding crate.

use core::ptr;

#[repr(C)]
pub struct omap_domain_base {
    pub va: *mut u8,
}

extern "C" {
    static mut prm_base: omap_domain_base;
    static mut prcm_mpu_base: omap_domain_base;
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn writel_relaxed(val: u32, addr: *mut u8);
    fn bug_on(condition: bool);
    fn omap_test_timeout(condition: u32, timeout: i32, counter: *mut i32);
}

const OMAP4_MAX_PRCM_PARTITIONS: usize = 2;
const OMAP4430_INVALID_PRCM_PARTITION: u8 = 0xff;
const PRM_INSTANCE_UNKNOWN: i32 = -1;
const OMAP4430_PRM_PARTITION: usize = 0;
const OMAP4430_PRCM_MPU_PARTITION: usize = 1;
const OMAP4_PRM_RSTCTRL_OFFSET: u16 = 0;
const OMAP4430_RST_GLOBAL_WARM_SW_MASK: u32 = 1;
const MAX_MODULE_HARDRESET_WAIT: i32 = 10000;
const EEXIST: i32 = 17;
const EBUSY: i32 = 16;

static mut _prm_bases: [omap_domain_base; OMAP4_MAX_PRCM_PARTITIONS] = [
    omap_domain_base { va: ptr::null_mut() },
    omap_domain_base { va: ptr::null_mut() },
];

static mut prm_dev_inst: i32 = PRM_INSTANCE_UNKNOWN;

/**
 * omap_prm_base_init - Populates the prm partitions
 *
 * Populates the base addresses of the _prm_bases
 * array used for read/write of prm module registers.
 */
pub unsafe fn omap_prm_base_init() {
    _prm_bases[OMAP4430_PRM_PARTITION] = prm_base;
    _prm_bases[OMAP4430_PRCM_MPU_PARTITION] = prcm_mpu_base;
}

pub unsafe fn omap4_prmst_get_prm_dev_inst() -> i32 {
    prm_dev_inst
}

pub unsafe fn omap4_prminst_set_prm_dev_inst(dev_inst: i32) {
    prm_dev_inst = dev_inst;
}

/* Read a register in a PRM instance */
pub unsafe fn omap4_prminst_read_inst_reg(part: u8, inst: i16, idx: u16) -> u32 {
    bug_on(part as usize >= OMAP4_MAX_PRCM_PARTITIONS
        || part == OMAP4430_INVALID_PRCM_PARTITION
        || _prm_bases[part as usize].va.is_null());
    readl_relaxed(_prm_bases[part as usize].va.offset(inst as isize + idx as isize))
}

/* Write into a register in a PRM instance */
pub unsafe fn omap4_prminst_write_inst_reg(val: u32, part: u8, inst: i16, idx: u16) {
    bug_on(part as usize >= OMAP4_MAX_PRCM_PARTITIONS
        || part == OMAP4430_INVALID_PRCM_PARTITION
        || _prm_bases[part as usize].va.is_null());
    writel_relaxed(val, _prm_bases[part as usize].va.offset(inst as isize + idx as isize));
}

/* Read-modify-write a register in PRM. Caller must lock */
pub unsafe fn omap4_prminst_rmw_inst_reg_bits(
    mask: u32, bits: u32, part: u8, inst: i16, idx: u16,
) -> u32 {
    let mut v = omap4_prminst_read_inst_reg(part, inst, idx);
    v &= !mask;
    v |= bits;
    omap4_prminst_write_inst_reg(v, part, inst, idx);
    v
}

/** Read the HW reset line state of a submodule. */
pub unsafe fn omap4_prminst_is_hardreset_asserted(
    shift: u8, part: u8, inst: i16, rstctrl_offs: u16,
) -> i32 {
    let mut v = omap4_prminst_read_inst_reg(part, inst, rstctrl_offs);
    v &= 1u32 << shift;
    v >>= shift;
    v as i32
}

/** Assert the HW reset line of a submodule. */
pub unsafe fn omap4_prminst_assert_hardreset(
    shift: u8, part: u8, inst: i16, rstctrl_offs: u16,
) -> i32 {
    let mask = 1u32 << shift;
    omap4_prminst_rmw_inst_reg_bits(mask, mask, part, inst, rstctrl_offs);
    0
}

/** Deassert a submodule hardreset line and wait. */
pub unsafe fn omap4_prminst_deassert_hardreset(
    shift: u8, st_shift: u8, part: u8, inst: i16,
    rstctrl_offs: u16, rstst_offs: u16,
) -> i32 {
    let mut c: i32 = 0;
    let mask = 1u32 << shift;
    let st_mask = 1u32 << st_shift;

    /* Check the current status to avoid de-asserting the line twice */
    if omap4_prminst_is_hardreset_asserted(shift, part, inst, rstctrl_offs) == 0 {
        return -EEXIST;
    }

    /* Clear the reset status by writing 1 to the status bit */
    omap4_prminst_rmw_inst_reg_bits(u32::MAX, st_mask, part, inst, rstst_offs);
    /* de-assert the reset control line */
    omap4_prminst_rmw_inst_reg_bits(mask, 0, part, inst, rstctrl_offs);
    /* wait the status to be set */
    omap_test_timeout(
        omap4_prminst_is_hardreset_asserted(st_shift, part, inst, rstst_offs) as u32,
        MAX_MODULE_HARDRESET_WAIT,
        &mut c,
    );

    if c == MAX_MODULE_HARDRESET_WAIT { -EBUSY } else { 0 }
}

pub unsafe fn omap4_prminst_global_warm_sw_reset() {
    let inst = omap4_prmst_get_prm_dev_inst();
    if inst == PRM_INSTANCE_UNKNOWN {
        return;
    }

    let mut v = omap4_prminst_read_inst_reg(
        OMAP4430_PRM_PARTITION as u8, inst as i16, OMAP4_PRM_RSTCTRL_OFFSET,
    );
    v |= OMAP4430_RST_GLOBAL_WARM_SW_MASK;
    omap4_prminst_write_inst_reg(
        v, OMAP4430_PRM_PARTITION as u8, inst as i16, OMAP4_PRM_RSTCTRL_OFFSET,
    );

    /* OCP barrier */
    let _ = omap4_prminst_read_inst_reg(
        OMAP4430_PRM_PARTITION as u8, inst as i16, OMAP4_PRM_RSTCTRL_OFFSET,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
