// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP4 PRCM_MPU module functions
 *
 * Copyright (C) 2009 Nokia Corporation
 * Paul Walmsley
 */

// Dependencies supplied by the surrounding translation unit:
// linux kernel types, I/O helpers, iomap/common definitions, PRCM_MPU
// declarations, and CM register-bit definitions.

/*
 * prcm_mpu_base: the virtual address of the start of the PRCM_MPU IP
 *   block registers
 */
pub static mut prcm_mpu_base: omap_domain_base = omap_domain_base {
    va: core::ptr::null_mut(),
};

/* PRCM_MPU low-level functions */

pub unsafe fn omap4_prcm_mpu_read_inst_reg(inst: i16, reg: u16) -> u32 {
    readl_relaxed(OMAP44XX_PRCM_MPU_REGADDR(inst, reg))
}

pub unsafe fn omap4_prcm_mpu_write_inst_reg(val: u32, inst: i16, reg: u16) {
    writel_relaxed(val, OMAP44XX_PRCM_MPU_REGADDR(inst, reg));
}

/**
 * omap2_set_globals_prcm_mpu - set the MPU PRCM base address (for early use)
 * @prcm_mpu: PRCM_MPU base virtual address
 *
 * XXX Will be replaced when the PRM/CM drivers are completed.
 */
pub unsafe fn omap2_set_globals_prcm_mpu(prcm_mpu: *mut core::ffi::c_void) {
    prcm_mpu_base.va = prcm_mpu;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
