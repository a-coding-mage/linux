/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2007-2008 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Author: Tony Li <tony.li@freescale.com>
 *         Jason Jin <Jason.jin@freescale.com>
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

pub const NR_MSI_REG_MSIIR: u32 = 8; // MSIIR can index 8 MSI registers
pub const NR_MSI_REG_MSIIR1: u32 = 16; // MSIIR1 can index 16 MSI registers
pub const NR_MSI_REG_MAX: u32 = NR_MSI_REG_MSIIR1;
pub const IRQS_PER_MSI_REG: u32 = 32;
pub const NR_MSI_IRQS_MAX: u32 = NR_MSI_REG_MAX * IRQS_PER_MSI_REG;

pub const FSL_PIC_IP_MASK: u32 = 0x0000000F;
pub const FSL_PIC_IP_MPIC: u32 = 0x00000001;
pub const FSL_PIC_IP_IPIC: u32 = 0x00000002;
pub const FSL_PIC_IP_VMPIC: u32 = 0x00000003;

pub const MSI_HW_ERRATA_ENDIAN: u32 = 0x00000010;

pub enum fsl_msi_cascade_data {}

#[repr(C)]
pub struct fsl_msi {
    pub irqhost: *mut irq_domain,

    pub cascade_irq: usize,

    // Offset of MSIIR, relative to start of CCSR
    pub msiir_offset: u32,
    // Shift of interrupt bit select
    pub ibs_shift: u32,
    // Shift of the shared interrupt register select
    pub srs_shift: u32,
    pub msi_regs: *mut c_void,
    pub feature: u32,
    pub cascade_array: [*mut fsl_msi_cascade_data; NR_MSI_REG_MAX as usize],

    pub bitmap: msi_bitmap,

    // support multiple MSI banks
    pub list: list_head,

    pub phandle: phandle,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
