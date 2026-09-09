/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <asm/errno.h>, <linux/uuid.h>, and <linux/cper.h>.

#[cfg(feature = "CONFIG_DEBUG_FS")]
extern "C" {
    pub fn ras_userspace_consumers() -> ::core::ffi::c_int;
    pub fn ras_debugfs_init();
    pub fn ras_add_daemon_trace() -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub fn ras_userspace_consumers() -> ::core::ffi::c_int { 0 }

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub fn ras_debugfs_init() {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub fn ras_add_daemon_trace() -> ::core::ffi::c_int { 0 }

#[cfg(feature = "CONFIG_RAS_CEC")]
extern "C" {
    // C declaration carries the kernel __init attribute.
    pub fn parse_cec_param(str_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}

#[cfg(feature = "CONFIG_RAS")]
extern "C" {
    pub fn log_non_standard_event(
        sec_type: *const guid_t,
        fru_id: *const guid_t,
        fru_text: *const ::core::ffi::c_char,
        sev: u8,
        err: *const u8,
        len: u32,
    );
    pub fn log_arm_hw_error(err: *mut cper_sec_proc_arm, sev: u8);
}

#[cfg(not(feature = "CONFIG_RAS"))]
#[inline]
pub unsafe fn log_non_standard_event(
    _sec_type: *const guid_t,
    _fru_id: *const guid_t,
    _fru_text: *const ::core::ffi::c_char,
    _sev: u8,
    _err: *const u8,
    _len: u32,
) {}

#[cfg(not(feature = "CONFIG_RAS"))]
#[inline]
pub unsafe fn log_arm_hw_error(_err: *mut cper_sec_proc_arm, _sev: u8) {}

#[repr(C)]
pub struct atl_err {
    pub addr: u64,
    pub ipid: u64,
    pub cpu: u32,
}

#[cfg(feature = "CONFIG_AMD_ATL")]
extern "C" {
    pub fn amd_atl_register_decoder(
        decoder: Option<unsafe extern "C" fn(*mut atl_err) -> ::core::ffi::c_ulong>,
    );
    pub fn amd_atl_unregister_decoder();
    pub fn amd_retire_dram_row(err: *mut atl_err);
    pub fn amd_convert_umc_mca_addr_to_sys_addr(err: *mut atl_err) -> ::core::ffi::c_ulong;
}

#[cfg(not(feature = "CONFIG_AMD_ATL"))]
#[inline]
pub unsafe fn amd_retire_dram_row(_err: *mut atl_err) {}

#[cfg(not(feature = "CONFIG_AMD_ATL"))]
#[inline]
pub unsafe fn amd_convert_umc_mca_addr_to_sys_addr(_err: *mut atl_err) -> ::core::ffi::c_ulong {
    0usize.wrapping_sub(EINVAL as usize) as ::core::ffi::c_ulong
}

// CONFIG_ARM || CONFIG_ARM64 supplies <asm/smp_plat.h> and get_logical_index.
#[cfg(any(feature = "CONFIG_ARM", feature = "CONFIG_ARM64"))]
#[macro_export]
macro_rules! GET_LOGICAL_INDEX {
    ($mpidr:expr) => {
        get_logical_index(($mpidr) & MPIDR_HWID_BITMASK)
    };
}

#[cfg(not(any(feature = "CONFIG_ARM", feature = "CONFIG_ARM64")))]
#[macro_export]
macro_rules! GET_LOGICAL_INDEX {
    ($mpidr:expr) => {
        -EINVAL
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
