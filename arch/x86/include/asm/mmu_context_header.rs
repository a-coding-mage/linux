/* SPDX-License-Identifier: GPL-2.0 */

// Translated from x86/include/asm/mmu_context.h.

extern "C" {
    static mut last_mm_ctx_id: atomic64_t;
}

#[cfg(CONFIG_PERF_EVENTS)]
extern "C" {
    static mut rdpmc_never_available_key: static_key_false;
    static mut rdpmc_always_available_key: static_key_false;
    fn cr4_update_pce(ignored: *mut core::ffi::c_void);
}

#[cfg(CONFIG_MODIFY_LDT_SYSCALL)]
#[repr(C)]
pub struct ldt_struct {
    pub entries: *mut desc_struct,
    pub nr_entries: core::ffi::c_uint,
    pub slot: core::ffi::c_int,
}

#[cfg(CONFIG_MODIFY_LDT_SYSCALL)]
#[inline]
pub unsafe fn init_new_context_ldt(mm: *mut mm_struct) {
    (*mm).context.ldt = core::ptr::null_mut();
    init_rwsem(&mut (*mm).context.ldt_usr_sem);
}

#[cfg(not(CONFIG_MODIFY_LDT_SYSCALL))]
#[inline]
pub unsafe fn init_new_context_ldt(_mm: *mut mm_struct) {}

extern "C" {
    #[cfg(CONFIG_MODIFY_LDT_SYSCALL)]
    fn ldt_dup_context(oldmm: *mut mm_struct, mm: *mut mm_struct) -> core::ffi::c_int;
    #[cfg(CONFIG_MODIFY_LDT_SYSCALL)]
    fn destroy_context_ldt(mm: *mut mm_struct);
    #[cfg(CONFIG_MODIFY_LDT_SYSCALL)]
    fn ldt_arch_exit_mmap(mm: *mut mm_struct);
}

#[cfg(not(CONFIG_MODIFY_LDT_SYSCALL))]
#[inline]
pub unsafe fn ldt_dup_context(_oldmm: *mut mm_struct, _mm: *mut mm_struct) -> core::ffi::c_int { 0 }
#[cfg(not(CONFIG_MODIFY_LDT_SYSCALL))]
#[inline]
pub unsafe fn destroy_context_ldt(_mm: *mut mm_struct) {}
#[cfg(not(CONFIG_MODIFY_LDT_SYSCALL))]
#[inline]
pub unsafe fn ldt_arch_exit_mmap(_mm: *mut mm_struct) {}

#[cfg(CONFIG_MODIFY_LDT_SYSCALL)]
extern "C" {
    fn load_mm_ldt(mm: *mut mm_struct);
    fn switch_ldt(prev: *mut mm_struct, next: *mut mm_struct);
}

#[cfg(not(CONFIG_MODIFY_LDT_SYSCALL))]
#[inline]
pub unsafe fn load_mm_ldt(_mm: *mut mm_struct) { clear_LDT(); }
#[cfg(not(CONFIG_MODIFY_LDT_SYSCALL))]
#[inline]
pub unsafe fn switch_ldt(_prev: *mut mm_struct, _next: *mut mm_struct) {
    DEBUG_LOCKS_WARN_ON(preemptible());
}

#[cfg(CONFIG_ADDRESS_MASKING)]
#[inline]
pub unsafe fn mm_lam_cr3_mask(mm: *mut mm_struct) -> core::ffi::c_ulong {
    READ_ONCE((*mm).context.lam_cr3_mask)
}
#[cfg(CONFIG_ADDRESS_MASKING)]
#[inline]
pub unsafe fn dup_lam(oldmm: *mut mm_struct, mm: *mut mm_struct) {
    (*mm).context.lam_cr3_mask = (*oldmm).context.lam_cr3_mask;
    (*mm).context.untag_mask = (*oldmm).context.untag_mask;
}
#[cfg(CONFIG_ADDRESS_MASKING)]
#[inline]
pub unsafe fn mm_untag_mask(mm: *mut mm_struct) -> core::ffi::c_ulong { (*mm).context.untag_mask }
#[cfg(CONFIG_ADDRESS_MASKING)]
#[inline]
pub unsafe fn mm_reset_untag_mask(mm: *mut mm_struct) { (*mm).context.untag_mask = !0; }
#[cfg(CONFIG_ADDRESS_MASKING)]
#[inline]
pub unsafe fn arch_pgtable_dma_compat(mm: *mut mm_struct) -> bool {
    !mm_lam_cr3_mask(mm) || test_bit(MM_CONTEXT_FORCE_TAGGED_SVA, &(*mm).context.flags)
}

#[cfg(not(CONFIG_ADDRESS_MASKING))]
#[inline]
pub unsafe fn mm_lam_cr3_mask(_mm: *mut mm_struct) -> core::ffi::c_ulong { 0 }
#[cfg(not(CONFIG_ADDRESS_MASKING))]
#[inline]
pub unsafe fn dup_lam(_oldmm: *mut mm_struct, _mm: *mut mm_struct) {}
#[cfg(not(CONFIG_ADDRESS_MASKING))]
#[inline]
pub unsafe fn mm_reset_untag_mask(_mm: *mut mm_struct) {}

extern "C" {
    fn mm_init_global_asid(mm: *mut mm_struct);
    fn mm_free_global_asid(mm: *mut mm_struct);
}

#[inline]
pub unsafe fn init_new_context(_tsk: *mut task_struct, mm: *mut mm_struct) -> core::ffi::c_int {
    mutex_init(&mut (*mm).context.lock);
    (*mm).context.ctx_id = atomic64_inc_return(&mut last_mm_ctx_id);
    atomic64_set(&mut (*mm).context.tlb_gen, 0);
    (*mm).context.next_trim_cpumask = jiffies + HZ;
    #[cfg(CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS)]
    if cpu_feature_enabled(X86_FEATURE_OSPKE) {
        (*mm).context.pkey_allocation_map = 0x1;
        (*mm).context.execute_only_pkey = -1;
    }
    mm_init_global_asid(mm);
    mm_reset_untag_mask(mm);
    init_new_context_ldt(mm);
    0
}

#[inline]
pub unsafe fn destroy_context(mm: *mut mm_struct) {
    destroy_context_ldt(mm);
    mm_free_global_asid(mm);
}

extern "C" {
    fn switch_mm(prev: *mut mm_struct, next: *mut mm_struct, tsk: *mut task_struct);
    fn switch_mm_irqs_off(prev: *mut mm_struct, next: *mut mm_struct, tsk: *mut task_struct);
}

#[inline]
pub unsafe fn activate_mm(prev: *mut mm_struct, next: *mut mm_struct) {
    paravirt_enter_mmap(next);
    switch_mm_irqs_off(prev, next, core::ptr::null_mut());
}

#[cfg(CONFIG_X86_32)]
#[inline]
pub unsafe fn deactivate_mm(_tsk: *mut task_struct, _mm: *mut mm_struct) { loadsegment(gs, 0); }
#[cfg(not(CONFIG_X86_32))]
#[inline]
pub unsafe fn deactivate_mm(tsk: *mut task_struct, _mm: *mut mm_struct) {
    shstk_free(tsk);
    load_gs_index(0);
    loadsegment(fs, 0);
}

#[inline]
pub unsafe fn arch_dup_pkeys(oldmm: *mut mm_struct, mm: *mut mm_struct) {
    #[cfg(CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS)]
    if cpu_feature_enabled(X86_FEATURE_OSPKE) {
        (*mm).context.pkey_allocation_map = (*oldmm).context.pkey_allocation_map;
        (*mm).context.execute_only_pkey = (*oldmm).context.execute_only_pkey;
    }
}

#[inline]
pub unsafe fn arch_dup_mmap(oldmm: *mut mm_struct, mm: *mut mm_struct) -> core::ffi::c_int {
    arch_dup_pkeys(oldmm, mm);
    paravirt_enter_mmap(mm);
    dup_lam(oldmm, mm);
    ldt_dup_context(oldmm, mm)
}

#[inline]
pub unsafe fn arch_exit_mmap(mm: *mut mm_struct) {
    paravirt_arch_exit_mmap(mm);
    ldt_arch_exit_mmap(mm);
}

#[cfg(CONFIG_X86_64)]
#[inline]
pub unsafe fn is_64bit_mm(mm: *mut mm_struct) -> bool {
    !IS_ENABLED(CONFIG_IA32_EMULATION) || !test_bit(MM_CONTEXT_UPROBE_IA32, &(*mm).context.flags)
}
#[cfg(not(CONFIG_X86_64))]
#[inline]
pub unsafe fn is_64bit_mm(_mm: *mut mm_struct) -> bool { false }

#[inline]
pub unsafe fn is_notrack_mm(mm: *mut mm_struct) -> bool {
    test_bit(MM_CONTEXT_NOTRACK, &(*mm).context.flags)
}
#[inline]
pub unsafe fn set_notrack_mm(mm: *mut mm_struct) {
    set_bit(MM_CONTEXT_NOTRACK, &mut (*mm).context.flags);
}

#[inline]
pub unsafe fn arch_vma_access_permitted(vma: *mut vm_area_struct, write: bool, execute: bool, foreign: bool) -> bool {
    if execute { return true; }
    if foreign || vma_is_foreign(vma) { return true; }
    __pkru_allows_pkey(vma_pkey(vma), write)
}

extern "C" {
    fn __get_current_cr3_fast() -> core::ffi::c_ulong;
    fn use_temporary_mm(temp_mm: *mut mm_struct) -> *mut mm_struct;
    fn unuse_temporary_mm(prev_mm: *mut mm_struct);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
