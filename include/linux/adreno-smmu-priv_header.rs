// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 Google, Inc
 */

use core::ffi::c_void;

// Dependency supplied by the Linux io-pgtable interface.
// The corresponding Rust declaration is expected from the surrounding build.
// use ...::io_pgtable_cfg;
// type phys_addr_t = ...;

/**
 * struct adreno_smmu_fault_info - container for key fault information
 *
 * @far: The faulting IOVA from ARM_SMMU_CB_FAR
 * @ttbr0: The current TTBR0 pagetable from ARM_SMMU_CB_TTBR0
 * @contextidr: The value of ARM_SMMU_CB_CONTEXTIDR
 * @fsr: The fault status from ARM_SMMU_CB_FSR
 * @fsynr0: The value of FSYNR0 from ARM_SMMU_CB_FSYNR0
 * @fsynr1: The value of FSYNR1 from ARM_SMMU_CB_FSYNR0
 * @cbfrsynra: The value of CBFRSYNRA from ARM_SMMU_GR1_CBFRSYNRA(idx)
 *
 * This struct passes back key page fault information to the GPU driver
 * through the get_fault_info function pointer.
 * The GPU driver can use this information to print informative
 * log messages and provide deeper GPU specific insight into the fault.
 */
#[repr(C)]
pub struct adreno_smmu_fault_info {
    pub far: u64,
    pub ttbr0: u64,
    pub contextidr: u32,
    pub fsr: u32,
    pub fsynr0: u32,
    pub fsynr1: u32,
    pub cbfrsynra: u32,
}

/**
 * struct adreno_smmu_priv - private interface between adreno-smmu and GPU
 *
 * @cookie:        An opque token provided by adreno-smmu and passed
 *                 back into the callbacks
 * @get_ttbr1_cfg: Get the TTBR1 config for the GPUs context-bank
 * @set_ttbr0_cfg: Set the TTBR0 config for the GPUs context bank.  A
 *                 NULL config disables TTBR0 translation, otherwise
 *                 TTBR0 translation is enabled with the specified cfg
 * @get_fault_info: Called by the GPU fault handler to get information about
 *                  the fault
 * @set_stall:     Configure whether stall on fault (CFCFG) is enabled. If
 *                 stalling on fault is enabled, the GPU driver must call
 *                 resume_translation()
 * @resume_translation: Resume translation after a fault
 *
 * @set_prr_bit:   [optional] Configure the GPU's Partially Resident
 *                 Region (PRR) bit in the ACTLR register.
 * @set_prr_addr:  [optional] Configure the PRR_CFG_*ADDR register with
 *                 the physical address of PRR page passed from GPU
 *                 driver.
 *
 * The GPU driver (drm/msm) and adreno-smmu work together for controlling
 * the GPU's SMMU instance.  This is by necessity, as the GPU is directly
 * updating the SMMU for context switches, while on the other hand we do
 * not want to duplicate all of the initial setup logic from arm-smmu.
 *
 * This private interface is used for the two drivers to coordinate.  The
 * cookie and callback functions are populated when the GPU driver attaches
 * it's domain.
 */
#[repr(C)]
pub struct adreno_smmu_priv {
    pub cookie: *const c_void,
    pub get_ttbr1_cfg: Option<unsafe extern "C" fn(cookie: *const c_void) -> *const io_pgtable_cfg>,
    pub set_ttbr0_cfg: Option<unsafe extern "C" fn(cookie: *const c_void, cfg: *const io_pgtable_cfg) -> i32>,
    pub get_fault_info: Option<unsafe extern "C" fn(cookie: *const c_void, info: *mut adreno_smmu_fault_info)>,
    pub set_stall: Option<unsafe extern "C" fn(cookie: *const c_void, enabled: bool)>,
    pub resume_translation: Option<unsafe extern "C" fn(cookie: *const c_void, terminate: bool)>,
    pub set_prr_bit: Option<unsafe extern "C" fn(cookie: *const c_void, set: bool)>,
    pub set_prr_addr: Option<unsafe extern "C" fn(cookie: *const c_void, page_addr: phys_addr_t)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
