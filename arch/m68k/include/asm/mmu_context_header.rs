/* SPDX-License-Identifier: GPL-2.0 */
// Translation of m68k/include/asm/mmu_context.h.

#[cfg(CONFIG_MMU)]
#[cfg(CONFIG_COLDFIRE)]
pub const NO_CONTEXT: u32 = 256;
#[cfg(CONFIG_MMU)]
#[cfg(CONFIG_COLDFIRE)]
pub const LAST_CONTEXT: u32 = 255;
#[cfg(CONFIG_MMU)]
#[cfg(CONFIG_COLDFIRE)]
pub const FIRST_CONTEXT: u32 = 1;

#[cfg(all(CONFIG_MMU, CONFIG_COLDFIRE))]
extern "C" {
    pub static mut context_map: [core::ffi::c_ulong; 0];
    pub static mut next_mmu_context: mm_context_t;
    pub static mut nr_free_contexts: atomic_t;
    pub static mut context_mm: [*mut mm_struct; (LAST_CONTEXT + 1) as usize];
    pub fn steal_context();
}

#[cfg(all(CONFIG_MMU, CONFIG_COLDFIRE))]
pub unsafe fn get_mmu_context(mm: *mut mm_struct) {
    let mut ctx: mm_context_t;
    if (*mm).context != NO_CONTEXT {
        return;
    }
    while arch_atomic_dec_and_test_lt(&mut nr_free_contexts) {
        atomic_inc(&mut nr_free_contexts);
        steal_context();
    }
    ctx = next_mmu_context;
    while test_and_set_bit(ctx, context_map.as_mut_ptr()) {
        ctx = find_next_zero_bit(context_map.as_ptr(), LAST_CONTEXT + 1, ctx);
        if ctx > LAST_CONTEXT { ctx = 0; }
    }
    next_mmu_context = (ctx + 1) & LAST_CONTEXT;
    (*mm).context = ctx;
    context_mm[ctx as usize] = mm;
}

#[cfg(all(CONFIG_MMU, CONFIG_COLDFIRE))]
pub unsafe fn init_new_context(_tsk: *mut task_struct, mm: *mut mm_struct) -> i32 {
    (*mm).context = NO_CONTEXT;
    0
}

#[cfg(all(CONFIG_MMU, CONFIG_COLDFIRE))]
pub unsafe fn destroy_context(mm: *mut mm_struct) {
    if (*mm).context != NO_CONTEXT {
        clear_bit((*mm).context, context_map.as_mut_ptr());
        (*mm).context = NO_CONTEXT;
        atomic_inc(&mut nr_free_contexts);
    }
}

#[cfg(all(CONFIG_MMU, CONFIG_COLDFIRE))]
pub unsafe fn set_context(context: mm_context_t, _pgd: *mut pgd_t) {
    core::arch::asm!("movec {0},%asid", in("d") context);
}

#[cfg(all(CONFIG_MMU, CONFIG_COLDFIRE))]
pub unsafe fn switch_mm(_prev: *mut mm_struct, next: *mut mm_struct, tsk: *mut task_struct) {
    get_mmu_context((*tsk).mm);
    set_context((*(*tsk).mm).context, (*next).pgd);
}

#[cfg(all(CONFIG_MMU, CONFIG_COLDFIRE))]
pub unsafe fn activate_mm(_active_mm: *mut mm_struct, mm: *mut mm_struct) {
    get_mmu_context(mm);
    set_context((*mm).context, (*mm).pgd);
}

#[cfg(all(CONFIG_MMU, CONFIG_COLDFIRE))]
pub unsafe fn prepare_arch_switch(next: *mut task_struct) { load_ksp_mmu(next); }

#[cfg(all(CONFIG_MMU, CONFIG_COLDFIRE))]
pub unsafe fn load_ksp_mmu(task: *mut task_struct) {
    let mut flags: core::ffi::c_ulong = 0;
    let mut mm: *mut mm_struct;
    let mut asid: i32;
    let mut pte: *mut pte_t = core::ptr::null_mut();
    let mmuar = (*task).thread.ksp;
    local_irq_save(&mut flags);
    mmu_write(MMUAR, mmuar);
    mmu_write(MMUOR, MMUOR_STLB | MMUOR_ADR);
    if mmu_read(MMUSR) & MMUSR_HIT != 0 { local_irq_restore(flags); return; }
    if mmuar >= PAGE_OFFSET { mm = &mut init_mm; }
    else { pr_info!("load_ksp_mmu: non-kernel mm found: 0x%p\n", (*task).mm); mm = (*task).mm; }
    if mm.is_null() { pr_info!("ksp load failed: mm=0x%p ksp=0x08%lx\n", mm, mmuar); local_irq_restore(flags); return; }
    let pgd = pgd_offset(mm, mmuar); if pgd_none(*pgd) { local_irq_restore(flags); return; }
    let p4d = p4d_offset(pgd, mmuar); if p4d_none(*p4d) { local_irq_restore(flags); return; }
    let pud = pud_offset(p4d, mmuar); if pud_none(*pud) { local_irq_restore(flags); return; }
    let pmd = pmd_offset(pud, mmuar); if pmd_none(*pmd) { local_irq_restore(flags); return; }
    pte = if mmuar >= PAGE_OFFSET { pte_offset_kernel(pmd, mmuar) } else { pte_offset_map(pmd, mmuar) };
    if pte.is_null() || pte_none(*pte) || !pte_present(*pte) { if !pte.is_null() && mmuar < PAGE_OFFSET { pte_unmap(pte); } local_irq_restore(flags); return; }
    set_pte(pte, pte_mkyoung(*pte));
    asid = ((*mm).context & 0xff) as i32;
    if !pte_dirty(*pte) && mmuar <= PAGE_OFFSET { set_pte(pte, pte_wrprotect(*pte)); }
    mmu_write(MMUTR, (mmuar & PAGE_MASK) | ((asid as core::ffi::c_ulong) << MMUTR_IDN) | ((((*pte).pte as i32) & CF_PAGE_MMUTR_MASK) >> CF_PAGE_MMUTR_SHIFT) as core::ffi::c_ulong | MMUTR_V);
    mmu_write(MMUDR, (pte_val(*pte) & PAGE_MASK) | ((*pte).pte & CF_PAGE_MMUDR_MASK) | MMUDR_SZ_8KB | MMUDR_X);
    mmu_write(MMUOR, MMUOR_ACC | MMUOR_UAA);
    if mmuar < PAGE_OFFSET { pte_unmap(pte); }
    local_irq_restore(flags);
}

#[cfg(all(CONFIG_MMU, CONFIG_SUN3))]
extern "C" { pub fn get_free_context(mm: *mut mm_struct) -> core::ffi::c_ulong; pub fn clear_context(context: core::ffi::c_ulong); }
#[cfg(all(CONFIG_MMU, CONFIG_SUN3))]
pub unsafe fn init_new_context(_tsk: *mut task_struct, mm: *mut mm_struct) -> i32 { (*mm).context = SUN3_INVALID_CONTEXT; 0 }
#[cfg(all(CONFIG_MMU, CONFIG_SUN3))]
pub unsafe fn get_mmu_context(mm: *mut mm_struct) { if (*mm).context == SUN3_INVALID_CONTEXT { (*mm).context = get_free_context(mm); } }
#[cfg(all(CONFIG_MMU, CONFIG_SUN3))]
pub unsafe fn destroy_context(mm: *mut mm_struct) { if (*mm).context != SUN3_INVALID_CONTEXT { clear_context((*mm).context); } }
#[cfg(all(CONFIG_MMU, CONFIG_SUN3))]
pub unsafe fn activate_context(mm: *mut mm_struct) { get_mmu_context(mm); sun3_put_context((*mm).context); }
#[cfg(all(CONFIG_MMU, CONFIG_SUN3))]
pub unsafe fn switch_mm(_prev: *mut mm_struct, _next: *mut mm_struct, tsk: *mut task_struct) { activate_context((*tsk).mm); }
#[cfg(all(CONFIG_MMU, CONFIG_SUN3))]
pub unsafe fn activate_mm(_prev_mm: *mut mm_struct, next_mm: *mut mm_struct) { activate_context(next_mm); }

// The remaining CONFIG_MMU m68k 020/030/040/060 implementation consists of
// processor-specific cache/TLB assembly and uses the external kernel symbols.
// Preserve its declarations and dispatch shape here.
#[cfg(all(CONFIG_MMU, not(any(CONFIG_COLDFIRE, CONFIG_SUN3))))]
pub unsafe fn init_new_context(_tsk: *mut task_struct, mm: *mut mm_struct) -> i32 { (*mm).context = virt_to_phys((*mm).pgd); 0 }
#[cfg(all(CONFIG_MMU, not(any(CONFIG_COLDFIRE, CONFIG_SUN3))))]
pub unsafe fn switch_mm(prev: *mut mm_struct, next: *mut mm_struct, _tsk: *mut task_struct) {
    if prev != next { if CPU_IS_020_OR_030 { switch_mm_0230(next); } else { switch_mm_0460(next); } }
}
#[cfg(all(CONFIG_MMU, not(any(CONFIG_COLDFIRE, CONFIG_SUN3))))]
pub unsafe fn switch_mm_0230(_mm: *mut mm_struct) { core::arch::asm!(".chip 68030", ".chip 68k"); }
#[cfg(all(CONFIG_MMU, not(any(CONFIG_COLDFIRE, CONFIG_SUN3))))]
pub unsafe fn switch_mm_0460(_mm: *mut mm_struct) { core::arch::asm!(".chip 68040", "pflushan", ".chip 68k"); }
#[cfg(all(CONFIG_MMU, not(any(CONFIG_COLDFIRE, CONFIG_SUN3))))]
pub unsafe fn activate_mm(_prev_mm: *mut mm_struct, next_mm: *mut mm_struct) { (*next_mm).context = virt_to_phys((*next_mm).pgd); if CPU_IS_020_OR_030 { switch_mm_0230(next_mm); } else { switch_mm_0460(next_mm); } }

// #include <asm-generic/mmu_context.h> and nommu_context.h provide the
// remaining generic declarations under their respective build conditions.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
