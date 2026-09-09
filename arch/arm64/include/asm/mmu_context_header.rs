// SPDX-License-Identifier: GPL-2.0-only
// Based on arch/arm/include/asm/mmu_context.h
// Copyright (C) 1996 Russell King.
// Copyright (C) 2012 ARM Ltd.

// C includes and header guards omitted; referenced symbols are supplied by dependencies.

extern "C" {
    pub static mut rodata_full: bool;
    pub fn cpu_do_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
    pub fn __cpu_replace_ttbr1(pgdp: *mut pgd_t, cnp: bool);
    pub fn check_and_switch_context(mm: *mut mm_struct);
    pub fn task_cpu_fallback_mask(p: *mut task_struct) -> *const cpumask;
    pub fn verify_cpu_asid_bits();
    pub fn post_ttbr_update_workaround();
    pub fn arm64_mm_context_get(mm: *mut mm_struct) -> c_ulong;
    pub fn arm64_mm_context_put(mm: *mut mm_struct);
}

#[inline]
pub unsafe fn contextidr_thread_switch(next: *mut task_struct) {
    if !IS_ENABLED(CONFIG_PID_IN_CONTEXTIDR) { return; }
    write_sysreg(task_pid_nr(next), contextidr_el1);
    isb();
}

#[inline]
pub unsafe fn cpu_set_reserved_ttbr0_nosync() {
    let ttbr: c_ulong = phys_to_ttbr(__pa_symbol(reserved_pg_dir));
    write_sysreg(ttbr, ttbr0_el1);
}

#[inline]
pub unsafe fn cpu_set_reserved_ttbr0() {
    cpu_set_reserved_ttbr0_nosync();
    isb();
}

#[inline]
pub unsafe fn cpu_switch_mm(pgd: *mut pgd_t, mm: *mut mm_struct) {
    BUG_ON(pgd == swapper_pg_dir);
    cpu_do_switch_mm(virt_to_phys(pgd), mm);
}

#[inline]
pub unsafe fn __cpu_set_tcr_t0sz(t0sz: c_ulong) {
    let mut tcr: c_ulong = read_sysreg(tcr_el1);
    if (tcr & TCR_EL1_T0SZ_MASK) == t0sz { return; }
    tcr &= !TCR_EL1_T0SZ_MASK;
    tcr |= t0sz;
    write_sysreg(tcr, tcr_el1);
    isb();
}

#[inline]
pub unsafe fn cpu_uninstall_idmap() {
    let mm = (*current).active_mm;
    cpu_set_reserved_ttbr0();
    local_flush_tlb_all();
    __cpu_set_tcr_t0sz(TCR_T0SZ(vabits_actual));
    if mm != &raw mut init_mm && !system_uses_ttbr0_pan() {
        cpu_switch_mm((*mm).pgd, mm);
    }
}

#[inline]
pub unsafe fn cpu_install_idmap() {
    cpu_set_reserved_ttbr0();
    local_flush_tlb_all();
    __cpu_set_tcr_t0sz(TCR_T0SZ(IDMAP_VA_BITS));
    cpu_switch_mm(lm_alias(idmap_pg_dir), &raw mut init_mm);
}

#[inline]
pub unsafe fn cpu_install_ttbr0(ttbr0: phys_addr_t, t0sz: c_ulong) {
    cpu_set_reserved_ttbr0();
    local_flush_tlb_all();
    __cpu_set_tcr_t0sz(t0sz);
    // avoid cpu_switch_mm() and its SW-PAN and CNP interactions
    write_sysreg(ttbr0, ttbr0_el1);
    isb();
}

#[inline]
pub unsafe fn cpu_enable_swapper_cnp() {
    __cpu_replace_ttbr1(lm_alias(swapper_pg_dir), true);
}

#[inline]
pub unsafe fn cpu_replace_ttbr1(pgdp: *mut pgd_t) {
    // Only for early TTBR1 replacement before cpucaps are finalized and before CNP selection.
    WARN_ON(system_capabilities_finalized());
    __cpu_replace_ttbr1(pgdp, false);
}

#[inline]
pub unsafe fn init_new_context(_tsk: *mut task_struct, mm: *mut mm_struct) -> c_int {
    atomic64_set(&raw mut (*mm).context.id, 0);
    refcount_set(&raw mut (*mm).context.pinned, 0);
    // pkey 0 is the default, so always reserve it.
    (*mm).context.pkey_allocation_map = BIT(0);
    0
}

#[inline]
pub unsafe fn arch_dup_pkeys(oldmm: *mut mm_struct, mm: *mut mm_struct) {
    (*mm).context.pkey_allocation_map = (*oldmm).context.pkey_allocation_map;
}

#[inline]
pub unsafe fn arch_dup_mmap(oldmm: *mut mm_struct, mm: *mut mm_struct) -> c_int {
    arch_dup_pkeys(oldmm, mm);
    0
}

#[inline]
pub unsafe fn arch_exit_mmap(_mm: *mut mm_struct) {}

#[inline]
pub unsafe fn arch_unmap(_mm: *mut mm_struct, _start: c_ulong, _end: c_ulong) {}

#[cfg(CONFIG_ARM64_SW_TTBR0_PAN)]
#[inline]
pub unsafe fn update_saved_ttbr0(tsk: *mut task_struct, mm: *mut mm_struct) {
    let ttbr: u64;
    if !system_uses_ttbr0_pan() { return; }
    if mm == &raw mut init_mm {
        ttbr = phys_to_ttbr(__pa_symbol(reserved_pg_dir));
    } else {
        ttbr = phys_to_ttbr(virt_to_phys((*mm).pgd)) |
            FIELD_PREP(TTBRx_EL1_ASID_MASK, ASID(mm));
    }
    WRITE_ONCE((*task_thread_info(tsk)).ttbr0, ttbr);
}

#[cfg(not(CONFIG_ARM64_SW_TTBR0_PAN))]
#[inline]
pub unsafe fn update_saved_ttbr0(_tsk: *mut task_struct, _mm: *mut mm_struct) {}

#[inline]
pub unsafe fn enter_lazy_tlb(mm: *mut mm_struct, tsk: *mut task_struct) {
    // We don't actually care about the ttbr0 mapping, so point it at the zero page.
    update_saved_ttbr0(tsk, &raw mut init_mm);
}

#[inline]
pub unsafe fn __switch_mm(next: *mut mm_struct) {
    // init_mm.pgd has no user mappings and is always active for kernel addresses in TTBR1.
    if next == &raw mut init_mm {
        cpu_set_reserved_ttbr0();
        return;
    }
    check_and_switch_context(next);
}

#[inline]
pub unsafe fn switch_mm(prev: *mut mm_struct, next: *mut mm_struct, tsk: *mut task_struct) {
    if prev != next { __switch_mm(next); }
    update_saved_ttbr0(tsk, next);
}

#[inline]
pub unsafe fn __task_cpu_possible_mask(p: *mut task_struct, mask: *const cpumask) -> *const cpumask {
    if !static_branch_unlikely(&raw mut arm64_mismatched_32bit_el0) { return mask; }
    if !is_compat_thread(task_thread_info(p)) { return mask; }
    system_32bit_el0_cpumask()
}

#[inline]
pub unsafe fn task_cpu_possible_mask(p: *mut task_struct) -> *const cpumask {
    __task_cpu_possible_mask(p, cpu_possible_mask)
}

#[inline]
pub unsafe fn mm_untag_mask(_mm: *mut mm_struct) -> c_ulong { !0 as c_ulong >> 8 }

#[inline]
pub unsafe fn arch_vma_access_permitted(vma: *mut vm_area_struct, write: bool, execute: bool, foreign: bool) -> bool {
    if !system_supports_poe() { return true; }
    if foreign || vma_is_foreign(vma) { return true; }
    por_el0_allows_pkey(vma_pkey(vma), write, execute)
}

#[inline]
pub unsafe fn deactivate_mm(tsk: *mut task_struct, _mm: *mut mm_struct) {
    gcs_free(tsk);
}

// The generic MMU context declarations are supplied by asm-generic/mmu_context.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
