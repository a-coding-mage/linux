/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020 Western Digital Corporation or its affiliates.
 */

// C dependencies: asm/csr.h, asm/io.h, asm/mmu_context.h, asm/ptrace.h,
// asm/tlbflush.h, and asm/pgalloc.h.

// CONFIG_EFI controls whether the firmware initialization function is
// provided by the build.
#[cfg(feature = "CONFIG_EFI")]
extern "C" {
    pub fn efi_init();
}

#[cfg(not(feature = "CONFIG_EFI"))]
#[inline(always)]
pub fn efi_init() {}

extern "C" {
    pub fn efi_create_mapping(mm: *mut mm_struct, md: *mut efi_memory_desc_t) -> ::core::ffi::c_int;
    pub fn efi_set_mapping_permissions(
        mm: *mut mm_struct,
        md: *mut efi_memory_desc_t,
        executable: bool,
    ) -> ::core::ffi::c_int;

    pub fn arch_efi_call_virt_setup();
    pub fn arch_efi_call_virt_teardown();

    pub fn stext_offset() -> ::core::ffi::c_ulong;

    pub fn efi_icache_sync(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
}

pub const ARCH_EFI_IRQ_FLAGS_MASK: _ = SR_IE | SR_SPIE;

/* Load initrd anywhere in system RAM */
#[inline]
pub unsafe fn efi_get_max_initrd_addr(_image_addr: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    ::core::ffi::c_ulong::MAX
}

#[inline]
pub fn efi_get_kimg_min_align() -> ::core::ffi::c_ulong {
    /*
     * RISC-V requires the kernel image to placed 2 MB aligned base for 64
     * bit and 4MB for 32 bit.
     */
    if cfg!(target_pointer_width = "64") {
        SZ_2M
    } else {
        SZ_4M
    }
}

pub const EFI_KIMG_PREFERRED_ADDRESS: ::core::ffi::c_ulong = efi_get_kimg_min_align();


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
