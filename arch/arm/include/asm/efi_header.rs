/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015 Linaro Ltd <ard.biesheuvel@linaro.org>
 */

// C dependencies supplied by other translation units/headers are intentionally
// referenced here rather than reimplemented.

#[cfg(feature = "CONFIG_EFI")]
extern "C" {
    pub fn efi_init();
    pub fn arm_efi_init();

    pub fn efi_create_mapping(mm: *mut mm_struct, md: *mut efi_memory_desc_t) -> ::core::ffi::c_int;
    pub fn efi_set_mapping_permissions(
        mm: *mut mm_struct,
        md: *mut efi_memory_desc_t,
        value: bool,
    ) -> ::core::ffi::c_int;

    pub fn efi_virtmap_load();
    pub fn efi_virtmap_unload();
}

#[cfg(feature = "CONFIG_EFI")]
#[inline]
pub unsafe fn arch_efi_call_virt_setup() {
    efi_virtmap_load();
}

#[cfg(feature = "CONFIG_EFI")]
#[inline]
pub unsafe fn arch_efi_call_virt_teardown() {
    efi_virtmap_unload();
}

#[cfg(feature = "CONFIG_EFI")]
#[inline]
pub unsafe fn efi_set_pgd(mm: *mut mm_struct) {
    check_and_switch_context(mm, core::ptr::null_mut());
}

#[cfg(feature = "CONFIG_CPU_TTBR0_PAN")]
#[macro_export]
macro_rules! arch_efi_call_virt {
    ($p:expr, $f:ident $(, $args:expr)*) => {{
        let flags: u32 = uaccess_save_and_enable();
        // C _Generic selects the EFI status expression or substitutes
        // EFI_ABORTED when the expression has another type.
        let res: efi_status_t = $p.$f($($args),*);
        uaccess_restore(flags);
        res
    }};
}

#[cfg(not(feature = "CONFIG_EFI"))]
#[macro_export]
macro_rules! arm_efi_init {
    () => {};
}

#[cfg(feature = "CONFIG_EFI")]
pub const ARCH_EFI_IRQ_FLAGS_MASK: u32 =
    PSR_J_BIT | PSR_E_BIT | PSR_A_BIT | PSR_I_BIT | PSR_F_BIT | PSR_T_BIT | MODE_MASK;

/*
 * A reasonable upper bound for the uncompressed kernel size is 32 MBytes,
 * so we will reserve that amount of memory. We have no easy way to tell what
 * the actuall size of code + data the uncompressed kernel will use.
 * If this is insufficient, the decompressor will relocate itself out of the
 * way before performing the decompression.
 */
pub const MAX_UNCOMP_KERNEL_SIZE: usize = SZ_32M;

/*
 * phys-to-virt patching requires that the physical to virtual offset is a
 * multiple of 2 MiB. However, using an alignment smaller than TEXT_OFFSET
 * here throws off the memory allocation logic, so let's use the lowest power
 * of two greater than 2 MiB and greater than TEXT_OFFSET.
 */
pub const EFI_PHYS_ALIGN: usize = if SZ_2M > roundup_pow_of_two(TEXT_OFFSET) {
    SZ_2M
} else {
    roundup_pow_of_two(TEXT_OFFSET)
};

/* on ARM, the initrd should be loaded in a lowmem region */
#[inline]
pub fn efi_get_max_initrd_addr(image_addr: usize) -> usize {
    (image_addr / SZ_4M) * SZ_4M + SZ_512M
}

#[repr(C)]
pub struct efi_arm_entry_state {
    pub cpsr_before_ebs: u32,
    pub sctlr_before_ebs: u32,
    pub cpsr_after_ebs: u32,
    pub sctlr_after_ebs: u32,
}

#[inline]
pub unsafe fn efi_capsule_flush_cache_range(addr: *mut core::ffi::c_void, size: ::core::ffi::c_int) {
    __cpuc_flush_dcache_area(addr, size);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
