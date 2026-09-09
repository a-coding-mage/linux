/* SPDX-License-Identifier: GPL-2.0 */

// Build-time condition preserved from CONFIG_SMP.
#[cfg(feature = "CONFIG_SMP")]
unsafe extern "C" {
    pub fn cpu_map_rebuild();
    pub fn map_to_cpu(index: core::ffi::c_uint) -> core::ffi::c_int;
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn cpu_map_init() {
    cpu_map_rebuild();
}

#[cfg(not(feature = "CONFIG_SMP"))]
unsafe extern "C" {
    pub fn raw_smp_processor_id() -> core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn cpu_map_init() {}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn map_to_cpu(_index: core::ffi::c_uint) -> core::ffi::c_int {
    raw_smp_processor_id()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
