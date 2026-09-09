/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2021 Intel Corporation */

/* Translated from linux/peci-cpu.h. */

/* Copied from x86 <asm/processor.h> */
pub const X86_VENDOR_INTEL: u32 = 0;

/* Copied from x86 <asm/cpu_device_id.h> */
pub const VFM_MODEL_BIT: u32 = 0;
pub const VFM_FAMILY_BIT: u32 = 8;
pub const VFM_VENDOR_BIT: u32 = 16;
pub const VFM_RSVD_BIT: u32 = 24;

pub const VFM_MODEL_MASK: u32 = ((1u32 << (VFM_FAMILY_BIT - VFM_MODEL_BIT)) - 1) << VFM_MODEL_BIT;
pub const VFM_FAMILY_MASK: u32 = ((1u32 << (VFM_VENDOR_BIT - VFM_FAMILY_BIT)) - 1) << VFM_FAMILY_BIT;
pub const VFM_VENDOR_MASK: u32 = ((1u32 << (VFM_RSVD_BIT - VFM_VENDOR_BIT)) - 1) << VFM_VENDOR_BIT;

#[inline]
pub const fn vfm_model(vfm: u32) -> u32 {
    (vfm & VFM_MODEL_MASK) >> VFM_MODEL_BIT
}

#[inline]
pub const fn vfm_family(vfm: u32) -> u32 {
    (vfm & VFM_FAMILY_MASK) >> VFM_FAMILY_BIT
}

#[inline]
pub const fn vfm_vendor(vfm: u32) -> u32 {
    (vfm & VFM_VENDOR_MASK) >> VFM_VENDOR_BIT
}

#[inline]
pub const fn vfm_make(vendor: u32, family: u32, model: u32) -> u32 {
    (model << VFM_MODEL_BIT) | (family << VFM_FAMILY_BIT) | (vendor << VFM_VENDOR_BIT)
}
/* End of copied code */

/* Dependency supplied by ../../arch/x86/include/asm/intel-family.h. */

pub const PECI_PCS_PKG_ID: u8 = 0;
pub const PECI_PKG_ID_CPU_ID: u16 = 0x0000;
pub const PECI_PKG_ID_PLATFORM_ID: u16 = 0x0001;
pub const PECI_PKG_ID_DEVICE_ID: u16 = 0x0002;
pub const PECI_PKG_ID_MAX_THREAD_ID: u16 = 0x0003;
pub const PECI_PKG_ID_MICROCODE_REV: u16 = 0x0004;
pub const PECI_PKG_ID_MCA_ERROR_LOG: u16 = 0x0005;
pub const PECI_PCS_MODULE_TEMP: u8 = 9;
pub const PECI_PCS_THERMAL_MARGIN: u8 = 10;
pub const PECI_PCS_DDR_DIMM_TEMP: u8 = 14;
pub const PECI_PCS_TEMP_TARGET: u8 = 16;
pub const PECI_PCS_TDP_UNITS: u8 = 30;

#[repr(C)]
pub struct peci_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn peci_temp_read(device: *mut peci_device, temp_raw: *mut i16) -> i32;
    pub fn peci_pcs_read(device: *mut peci_device, index: u8, param: u16, data: *mut u32) -> i32;
    pub fn peci_pci_local_read(
        device: *mut peci_device,
        bus: u8,
        dev: u8,
        func: u8,
        reg: u16,
        data: *mut u32,
    ) -> i32;
    pub fn peci_ep_pci_local_read(
        device: *mut peci_device,
        seg: u8,
        bus: u8,
        dev: u8,
        func: u8,
        reg: u16,
        data: *mut u32,
    ) -> i32;
    pub fn peci_mmio_read(
        device: *mut peci_device,
        bar: u8,
        seg: u8,
        bus: u8,
        dev: u8,
        func: u8,
        address: u64,
        data: *mut u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
