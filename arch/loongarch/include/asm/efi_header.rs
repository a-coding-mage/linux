/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependency corresponding to: #include <linux/efi.h>

use core::ffi::{c_char, c_void};

// The definition is supplied by the corresponding dependency.
pub struct screen_info {
    _private: [u8; 0],
}

// The C __init annotation is a linker/build-time attribute and has no direct
// Rust syntax here.
extern "C" {
    pub fn efi_init();
    pub fn efi_runtime_init();
    pub fn efi_fdt_pointer() -> *mut c_void;
    pub fn efifb_setup_from_dmi(si: *mut screen_info, opt: *const c_char);
}

pub const ARCH_EFI_IRQ_FLAGS_MASK: u32 = 0x0000_0004; // Bit 2: CSR.CRMD.IE

// arch_efi_call_virt_setup() and arch_efi_call_virt_teardown() are empty
// macros in the source header.

pub const EFI_ALLOC_ALIGN: usize = 64 * 1024;
pub const EFI_RT_VIRTUAL_OFFSET: usize = CSR_DMW0_BASE;

pub const fn efi_get_max_initrd_addr(_image_addr: usize) -> usize {
    usize::MAX
}

pub const fn efi_get_kimg_min_align() -> usize {
    2 * 1024 * 1024
}

extern "C" {
    pub fn efi_get_kimg_kaslr_address() -> usize;
}

// C macro equivalent: EFI_KIMG_PREFERRED_ADDRESS efi_get_kimg_kaslr_address()
#[inline]
pub unsafe fn EFI_KIMG_PREFERRED_ADDRESS() -> usize {
    efi_get_kimg_kaslr_address()
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
