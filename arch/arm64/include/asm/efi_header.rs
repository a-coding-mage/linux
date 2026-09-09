/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: asm/boot.h, asm/cpufeature.h, asm/fpsimd.h,
// asm/io.h, asm/memory.h, asm/mmu_context.h, asm/neon.h, asm/ptrace.h,
// and asm/tlbflush.h.

#[cfg(feature = "CONFIG_EFI")]
extern "C" {
    pub fn efi_init();
    pub fn efi_runtime_fixup_exception(regs: *mut pt_regs, msg: *const ::core::ffi::c_char) -> bool;
}

#[cfg(not(feature = "CONFIG_EFI"))]
#[inline]
pub fn efi_init() {}

#[cfg(not(feature = "CONFIG_EFI"))]
#[inline]
pub unsafe fn efi_runtime_fixup_exception(
    _regs: *mut pt_regs,
    _msg: *const ::core::ffi::c_char,
) -> bool {
    false
}

extern "C" {
    pub fn efi_create_mapping(mm: *mut mm_struct, md: *mut efi_memory_desc_t) -> ::core::ffi::c_int;
    pub fn efi_set_mapping_permissions(
        mm: *mut mm_struct,
        md: *mut efi_memory_desc_t,
        has_bti: bool,
    ) -> ::core::ffi::c_int;

    pub static mut efi_rt_stack_top: *mut u64;
    pub fn __efi_rt_asm_wrapper(
        f: *mut ::core::ffi::c_void,
        name: *const ::core::ffi::c_char,
        ...,
    ) -> efi_status_t;

    pub fn arch_efi_call_virt_setup();
    pub fn arch_efi_call_virt_teardown();
    pub fn primary_entry_offset() -> ::core::primitive::usize;
    pub static mut efi_nokaslr: bool;
    pub fn efi_virtmap_load();
    pub fn efi_virtmap_unload();
    pub fn efi_handle_corrupted_x18(
        s: efi_status_t,
        f: *const ::core::ffi::c_char,
    ) -> efi_status_t;
    pub fn efi_icache_sync(start: ::core::primitive::usize, end: ::core::primitive::usize);
}

#[macro_export]
macro_rules! arch_efi_call_virt {
    ($p:expr, $f:ident $(, $args:expr)*) => {
        unsafe { $crate::__efi_rt_asm_wrapper(($p).$f, stringify!($f), $($args),*) }
    };
}

/* efi_rt_stack_top[-1] contains the value the stack pointer had before
 * switching to the EFI runtime stack. */
#[inline]
pub unsafe fn current_in_efi() -> bool {
    efi_rt_stack_top != ::core::ptr::null_mut()
        && on_task_stack(current, READ_ONCE((*efi_rt_stack_top.offset(-1))), 1)
}

pub const ARCH_EFI_IRQ_FLAGS_MASK: u64 = PSR_D_BIT | PSR_A_BIT | PSR_I_BIT | PSR_F_BIT;

/* Even when Linux uses IRQ priorities for IRQ disabling, EFI does not.
 * And EFI shouldn't really play around with priority masking as it is not aware
 * which priorities the OS has assigned to its interrupts. */
#[inline]
pub unsafe fn arch_efi_save_flags(state_flags: &mut u64) {
    *state_flags = read_sysreg(daif);
}

#[inline]
pub unsafe fn arch_efi_restore_flags(state_flags: u64) {
    write_sysreg(state_flags, daif);
}

/* In some configurations (e.g. VMAP_STACK && 64K pages), stacks built into the
 * kernel need greater alignment than we require the segments to be padded to. */
pub const EFI_KIMG_ALIGN: usize = if SEGMENT_ALIGN > THREAD_ALIGN {
    SEGMENT_ALIGN
} else {
    THREAD_ALIGN
};

#[inline]
pub const fn efi_get_max_initrd_addr(image_addr: usize) -> usize {
    (image_addr & !(SZ_1G - 1usize)) + (1usize << (VA_BITS_MIN - 1))
}

#[inline]
pub unsafe fn efi_get_kimg_min_align() -> usize {
    if efi_nokaslr { MIN_KIMG_ALIGN } else { EFI_KIMG_ALIGN }
}

pub const EFI_ALLOC_ALIGN: usize = SZ_64K;
pub const EFI_ALLOC_LIMIT: usize = (1usize << 48) - 1;

#[inline]
pub unsafe fn efi_set_pgd(mm: *mut mm_struct) {
    __switch_mm(mm);

    if system_uses_ttbr0_pan() {
        if (*mm != (*current).active_mm) {
            update_saved_ttbr0(current, mm);
        } else {
            update_saved_ttbr0(current, (*current).active_mm);
        }
    }
}

#[inline]
pub unsafe fn efi_capsule_flush_cache_range(addr: *mut ::core::ffi::c_void, size: ::core::ffi::c_int) {
    dcache_clean_inval_poc(addr as usize, addr as usize + size as usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
