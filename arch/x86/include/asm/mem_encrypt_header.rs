/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AMD Memory Encryption Support
 *
 * Copyright (C) 2016 Advanced Micro Devices, Inc.
 *
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 */

// C header dependencies: linux/init.h, linux/cc_platform.h, asm/asm.h.
// Build-time CONFIG_X86_MEM_ENCRYPT and CONFIG_AMD_MEM_ENCRYPT conditions are
// preserved below as Rust cfg conditions.

#[cfg(CONFIG_X86_MEM_ENCRYPT)]
extern "C" {
    pub fn mem_encrypt_init();
    pub fn mem_encrypt_setup_arch();
}

#[cfg(not(CONFIG_X86_MEM_ENCRYPT))]
#[inline]
pub unsafe fn mem_encrypt_init() {}

#[cfg(not(CONFIG_X86_MEM_ENCRYPT))]
#[inline]
pub unsafe fn mem_encrypt_setup_arch() {}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
extern "C" {
    pub static mut sme_me_mask: u64;
    pub static mut sev_status: u64;

    pub fn sme_encrypt_execute(
        encrypted_kernel_vaddr: ::core::ffi::c_ulong,
        decrypted_kernel_vaddr: ::core::ffi::c_ulong,
        kernel_len: ::core::ffi::c_ulong,
        encryption_wa: ::core::ffi::c_ulong,
        encryption_pgd: ::core::ffi::c_ulong,
    );

    pub fn sme_early_encrypt(paddr: resource_size_t, size: ::core::ffi::c_ulong);
    pub fn sme_early_decrypt(paddr: resource_size_t, size: ::core::ffi::c_ulong);

    pub fn sme_map_bootdata(real_mode_data: *mut ::core::ffi::c_char);
    pub fn sme_unmap_bootdata(real_mode_data: *mut ::core::ffi::c_char);

    pub fn sme_early_init();

    pub fn sme_encrypt_kernel(bp: *mut boot_params);
    pub fn sme_enable(bp: *mut boot_params);

    pub fn early_set_memory_decrypted(
        vaddr: ::core::ffi::c_ulong,
        size: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn early_set_memory_encrypted(
        vaddr: ::core::ffi::c_ulong,
        size: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn early_set_mem_enc_dec_hypercall(
        vaddr: ::core::ffi::c_ulong,
        size: ::core::ffi::c_ulong,
        enc: bool,
    );

    pub fn mem_encrypt_free_decrypted_mem();
    pub fn sev_es_init_vc_handling();
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
#[inline]
pub unsafe fn sme_get_me_mask() -> u64 {
    sme_me_mask
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
#[macro_export]
macro_rules! __bss_decrypted {
    () => { #[link_section = ".bss..decrypted"] };
}

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
pub const sme_me_mask: u64 = 0;

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
pub const sev_status: u64 = 0;

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
#[inline]
pub unsafe fn sme_early_encrypt(_paddr: resource_size_t, _size: ::core::ffi::c_ulong) {}

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
#[inline]
pub unsafe fn sme_early_decrypt(_paddr: resource_size_t, _size: ::core::ffi::c_ulong) {}

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
#[inline]
pub unsafe fn sme_map_bootdata(_real_mode_data: *mut ::core::ffi::c_char) {}

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
#[inline]
pub unsafe fn sme_unmap_bootdata(_real_mode_data: *mut ::core::ffi::c_char) {}

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
#[inline]
pub unsafe fn sme_early_init() {}

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
#[inline]
pub unsafe fn sme_encrypt_kernel(_bp: *mut boot_params) {}

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
#[inline]
pub unsafe fn sme_enable(_bp: *mut boot_params) {}

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
#[inline]
pub unsafe fn sev_es_init_vc_handling() {}

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
#[inline]
pub unsafe fn early_set_memory_decrypted(
    _vaddr: ::core::ffi::c_ulong,
    _size: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int { 0 }

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
#[inline]
pub unsafe fn early_set_memory_encrypted(
    _vaddr: ::core::ffi::c_ulong,
    _size: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int { 0 }

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
#[inline]
pub unsafe fn early_set_mem_enc_dec_hypercall(
    _vaddr: ::core::ffi::c_ulong,
    _size: ::core::ffi::c_ulong,
    _enc: bool,
) {}

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
#[inline]
pub unsafe fn mem_encrypt_free_decrypted_mem() {}

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
#[inline]
pub unsafe fn sme_get_me_mask() -> u64 { 0 }

extern "C" {
    pub fn add_encrypt_protection_map();

    pub static mut __start_bss_decrypted: [::core::ffi::c_char; 0];
    pub static mut __end_bss_decrypted: [::core::ffi::c_char; 0];
    pub static mut __start_bss_decrypted_unused: [::core::ffi::c_char; 0];
}

// The C macros use external __pa/__pa_nodebug dependencies and preserve the
// encryption mask operation for values written to or compared with CR3.
#[inline]
pub unsafe fn __sme_pa<T>(x: T) -> u64 {
    __pa(x) | sme_me_mask
}

#[inline]
pub unsafe fn __sme_pa_nodebug<T>(x: T) -> u64 {
    __pa_nodebug(x) | sme_me_mask
}

extern "C" {
    pub fn __pa<T>(x: T) -> u64;
    pub fn __pa_nodebug<T>(x: T) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
