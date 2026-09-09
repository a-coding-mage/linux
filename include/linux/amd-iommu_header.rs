/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2007-2010 Advanced Micro Devices, Inc.
 * Author: Joerg Roedel <joerg.roedel@amd.com>
 *         Leo Duran <leo.duran@amd.com>
 */

// Translated from the C header. The original include supplies Linux integer
// types; Rust primitive integer types are used directly here.

#[repr(C)]
pub struct amd_iommu {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_AMD_IOMMU")]
extern "C" {
    pub fn amd_iommu_detect();
}

#[cfg(not(feature = "CONFIG_AMD_IOMMU"))]
#[inline]
pub fn amd_iommu_detect() {}

// IOMMU AVIC Function. The original declarations are enabled when both
// CONFIG_AMD_IOMMU and CONFIG_IRQ_REMAP are enabled.
#[cfg(all(feature = "CONFIG_AMD_IOMMU", feature = "CONFIG_IRQ_REMAP"))]
extern "C" {
    pub fn amd_iommu_register_ga_log_notifier(
        notifier: Option<extern "C" fn(u32) -> i32>,
    ) -> i32;
    pub fn amd_iommu_update_ga(data: *mut core::ffi::c_void, cpu: i32, ga_log_intr: bool) -> i32;
    pub fn amd_iommu_activate_guest_mode(
        data: *mut core::ffi::c_void,
        cpu: i32,
        ga_log_intr: bool,
    ) -> i32;
    pub fn amd_iommu_deactivate_guest_mode(data: *mut core::ffi::c_void) -> i32;
}

#[cfg(not(all(feature = "CONFIG_AMD_IOMMU", feature = "CONFIG_IRQ_REMAP")))]
#[inline]
pub fn amd_iommu_register_ga_log_notifier(
    _notifier: Option<extern "C" fn(u32) -> i32>,
) -> i32 {
    0
}

#[cfg(not(all(feature = "CONFIG_AMD_IOMMU", feature = "CONFIG_IRQ_REMAP")))]
#[inline]
pub fn amd_iommu_update_ga(
    _data: *mut core::ffi::c_void,
    _cpu: i32,
    _ga_log_intr: bool,
) -> i32 {
    0
}

#[cfg(not(all(feature = "CONFIG_AMD_IOMMU", feature = "CONFIG_IRQ_REMAP")))]
#[inline]
pub fn amd_iommu_activate_guest_mode(
    _data: *mut core::ffi::c_void,
    _cpu: i32,
    _ga_log_intr: bool,
) -> i32 {
    0
}

#[cfg(not(all(feature = "CONFIG_AMD_IOMMU", feature = "CONFIG_IRQ_REMAP")))]
#[inline]
pub fn amd_iommu_deactivate_guest_mode(_data: *mut core::ffi::c_void) -> i32 {
    0
}

extern "C" {
    pub fn amd_iommu_get_num_iommus() -> i32;
    pub fn amd_iommu_pc_supported() -> bool;
    pub fn amd_iommu_pc_get_max_banks(idx: u32) -> u8;
    pub fn amd_iommu_pc_get_max_counters(idx: u32) -> u8;
    pub fn amd_iommu_pc_set_reg(
        iommu: *mut amd_iommu,
        bank: u8,
        cntr: u8,
        fxn: u8,
        value: *mut u64,
    ) -> i32;
    pub fn amd_iommu_pc_get_reg(
        iommu: *mut amd_iommu,
        bank: u8,
        cntr: u8,
        fxn: u8,
        value: *mut u64,
    ) -> i32;
    pub fn get_amd_iommu(idx: u32) -> *mut amd_iommu;
}

#[cfg(feature = "CONFIG_KVM_AMD_SEV")]
extern "C" {
    pub fn amd_iommu_snp_disable() -> i32;
    pub fn amd_iommu_sev_tio_supported() -> bool;
}

#[cfg(not(feature = "CONFIG_KVM_AMD_SEV"))]
#[inline]
pub fn amd_iommu_snp_disable() -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_KVM_AMD_SEV"))]
#[inline]
pub fn amd_iommu_sev_tio_supported() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
