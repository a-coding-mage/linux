/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2016 Cavium, Inc.
 */

// Dependency intent: symbols from cpt_common.h are supplied by other files.

pub const CSR_DELAY: u32 = 30;
pub const CPT_MAX_CORE_GROUPS: usize = 8;
pub const CPT_MAX_SE_CORES: usize = 10;
pub const CPT_MAX_AE_CORES: usize = 6;
pub const CPT_MAX_TOTAL_CORES: usize = CPT_MAX_SE_CORES + CPT_MAX_AE_CORES;
pub const CPT_MAX_VF_NUM: usize = 16;
pub const CPT_PF_MSIX_VECTORS: usize = 3;

#[inline]
pub const fn CPT_PF_INT_VEC_E_MBOXX(a: u32) -> u32 {
    0x02 + a
}

pub const CPT_UCODE_VERSION_SZ: usize = 32;

pub struct cpt_device;

#[repr(C)]
pub struct microcode {
    pub is_mc_valid: u8,
    pub is_ae: u8,
    pub group: u8,
    pub num_cores: u8,
    pub code_size: u32,
    pub core_mask: u64,
    pub version: [u8; CPT_UCODE_VERSION_SZ],
    /* Base info */
    pub phys_base: dma_addr_t,
    pub code: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct cpt_vf_info {
    pub state: u8,
    pub priority: u8,
    pub id: u8,
    pub qlen: u32,
}

/**
 * cpt device structure
 */
#[repr(C)]
pub struct cpt_device {
    pub flags: u16, /* Flags to hold device status bits */
    pub num_vf_en: u8, /* Number of VFs enabled (0...CPT_MAX_VF_NUM) */
    pub vfinfo: [cpt_vf_info; CPT_MAX_VF_NUM], /* Per VF info */

    pub reg_base: *mut core::ffi::c_void, /* Register start address */
    pub pdev: *mut pci_dev, /* pci device handle */

    pub mcode: [microcode; CPT_MAX_CORE_GROUPS],
    pub next_mc_idx: u8, /* next microcode index */
    pub next_group: u8,
    pub max_se_cores: u8,
    pub max_ae_cores: u8,
}

extern "C" {
    pub fn cpt_mbox_intr_handler(cpt: *mut cpt_device, mbx: core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
