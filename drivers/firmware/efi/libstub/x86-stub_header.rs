/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency supplied by the surrounding EFI code.

unsafe extern "C" {
    pub fn trampoline_32bit_src(arg0: *mut core::ffi::c_void, arg1: bool);
    pub static trampoline_ljmp_imm_offset: u16;

    pub fn efi_adjust_memory_range_protection(
        start: core::ffi::c_ulong,
        size: core::ffi::c_ulong,
    ) -> efi_status_t;
}

// CONFIG_X86_64 selects the native five-level paging implementation.
#[cfg(CONFIG_X86_64)]
unsafe extern "C" {
    pub fn efi_setup_5level_paging() -> efi_status_t;
    pub fn efi_5level_switch();
}

// Non-x86_64 builds use the C header's inline fallback implementations.
#[cfg(not(CONFIG_X86_64))]
#[inline]
pub unsafe fn efi_setup_5level_paging() -> efi_status_t {
    EFI_SUCCESS
}

#[cfg(not(CONFIG_X86_64))]
#[inline]
pub unsafe fn efi_5level_switch() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
