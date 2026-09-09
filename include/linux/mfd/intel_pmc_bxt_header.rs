/* SPDX-License-Identifier: GPL-2.0 */

/* GCR reg offsets from GCR base */
pub const PMC_GCR_PMC_CFG_REG: u32 = 0x08;
pub const PMC_GCR_TELEM_DEEP_S0IX_REG: u32 = 0x78;
pub const PMC_GCR_TELEM_SHLW_S0IX_REG: u32 = 0x80;

/* PMC_CFG_REG bit masks */
pub const PMC_CFG_NO_REBOOT_EN: u32 = 1u32 << 4;

/**
 * struct intel_pmc_dev - Intel PMC device structure
 * @dev: Pointer to the parent PMC device
 * @scu: Pointer to the SCU IPC device data structure
 * @gcr_mem_base: Virtual base address of GCR (Global Configuration Registers)
 * @gcr_lock: Lock used to serialize access to GCR registers
 * @telem_base: Pointer to telemetry SSRAM base resource or %NULL if not
 *              available
 */
#[repr(C)]
pub struct intel_pmc_dev {
    pub dev: *mut device,
    pub scu: *mut intel_scu_ipc_dev,
    pub gcr_mem_base: *mut core::ffi::c_void,
    pub gcr_lock: spinlock_t,
    pub telem_base: *mut resource,
}

/* CONFIG_MFD_INTEL_PMC_BXT is a build-time kernel configuration option. */
#[cfg(feature = "CONFIG_MFD_INTEL_PMC_BXT")]
unsafe extern "C" {
    pub fn intel_pmc_gcr_read64(
        pmc: *mut intel_pmc_dev,
        offset: u32,
        data: *mut u64,
    ) -> i32;
    pub fn intel_pmc_gcr_update(
        pmc: *mut intel_pmc_dev,
        offset: u32,
        mask: u32,
        val: u32,
    ) -> i32;
    pub fn intel_pmc_s0ix_counter_read(pmc: *mut intel_pmc_dev, data: *mut u64) -> i32;
}

#[cfg(not(feature = "CONFIG_MFD_INTEL_PMC_BXT"))]
#[inline]
pub unsafe fn intel_pmc_gcr_read64(
    _pmc: *mut intel_pmc_dev,
    _offset: u32,
    _data: *mut u64,
) -> i32 {
    -ENOTSUPP
}

#[cfg(not(feature = "CONFIG_MFD_INTEL_PMC_BXT"))]
#[inline]
pub unsafe fn intel_pmc_gcr_update(
    _pmc: *mut intel_pmc_dev,
    _offset: u32,
    _mask: u32,
    _val: u32,
) -> i32 {
    -ENOTSUPP
}

#[cfg(not(feature = "CONFIG_MFD_INTEL_PMC_BXT"))]
#[inline]
pub unsafe fn intel_pmc_s0ix_counter_read(_pmc: *mut intel_pmc_dev, _data: *mut u64) -> i32 {
    -ENOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
