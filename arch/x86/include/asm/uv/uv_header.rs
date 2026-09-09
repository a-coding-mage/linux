/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uv_system_type {
    UV_NONE,
    UV_LEGACY_APIC,
    UV_X2APIC,
}

// CONFIG_X86_UV selects the implementation below; otherwise the fallback
// declarations at the end of this file are used.
#[cfg(feature = "CONFIG_X86_UV")]
pub const UV_PROC_NODE: &str = "sgi_uv";

#[cfg(feature = "CONFIG_X86_UV")]
#[inline]
pub fn uv(uvtype: i32) -> i32 {
    /* uv(0) is "any" */
    if uvtype >= 0 && uvtype <= 30 {
        1i32 << (uvtype as u32)
    } else {
        1
    }
}

#[cfg(feature = "CONFIG_X86_UV")]
extern "C" {
    pub static mut uv_systab_phys: ::core::ffi::c_ulong;

    pub fn get_uv_system_type() -> uv_system_type;
    pub fn is_uv_system() -> ::core::ffi::c_int;
    pub fn is_uv_hubbed(uvtype: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn uv_cpu_init();
    pub fn uv_nmi_init();
    pub fn uv_system_init();
}

#[cfg(feature = "CONFIG_X86_UV")]
#[inline]
pub unsafe fn is_early_uv_system() -> bool {
    uv_systab_phys != 0 && uv_systab_phys != efi::EFI_INVALID_TABLE_ADDR
}

// !CONFIG_X86_UV
#[cfg(not(feature = "CONFIG_X86_UV"))]
#[inline]
pub fn get_uv_system_type() -> uv_system_type {
    uv_system_type::UV_NONE
}

#[cfg(not(feature = "CONFIG_X86_UV"))]
#[inline]
pub fn is_early_uv_system() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_X86_UV"))]
#[inline]
pub fn is_uv_system() -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_X86_UV"))]
#[inline]
pub fn is_uv_hubbed(_uv: i32) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_X86_UV"))]
#[inline]
pub fn uv_cpu_init() {}

#[cfg(not(feature = "CONFIG_X86_UV"))]
#[inline]
pub fn uv_system_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
