/* SPDX-License-Identifier: GPL-2.0 */
/* Derived heavily from Linus's Alpha/AXP ASN code... */

// C header dependencies are supplied by the surrounding translation unit.

extern "C" {
    static mut ctx_alloc_lock: spinlock_t;
    static mut tlb_context_cache: c_ulong;
    static mut mmu_context_bmap: *mut c_ulong;
    static mut per_cpu_secondary_mm: *mut mm_struct;

    fn get_new_mmu_context(mm: *mut mm_struct);
    fn init_new_context(tsk: *mut task_struct, mm: *mut mm_struct) -> c_int;
    fn destroy_context(mm: *mut mm_struct);
    fn __tsb_context_switch(
        pgd_pa: c_ulong,
        tsb_base: *mut tsb_config,
        tsb_huge: *mut tsb_config,
        tsb_descr_pa: c_ulong,
        secondary_ctx: c_ulong,
    );
    fn tsb_grow(mm: *mut mm_struct, tsb_index: c_ulong, mm_rss: c_ulong);
    fn __flush_tlb_mm(ctx: c_ulong, secondary_context: c_ulong);
    fn adi_capable() -> bool;
    fn set_tsk_thread_flag(task: *mut task_struct, flag: c_int);
    fn clear_tsk_thread_flag(task: *mut task_struct, flag: c_int);
    fn test_thread_flag(flag: c_int) -> c_ulong;
    fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
}

#[cfg(feature = "smp")]
extern "C" { fn smp_tsb_sync(mm: *mut mm_struct); }

#[inline]
unsafe fn tsb_context_switch_ctx(mm: *mut mm_struct, ctx: c_ulong) {
    let huge = {
        #[cfg(any(feature = "hugetlb_page", feature = "transparent_hugepage"))]
        { if (*mm).context.tsb_block[MM_TSB_HUGE].tsb != core::ptr::null_mut() {
            &mut (*mm).context.tsb_block[MM_TSB_HUGE] as *mut tsb_config
        } else { core::ptr::null_mut() } }
        #[cfg(not(any(feature = "hugetlb_page", feature = "transparent_hugepage")))]
        { core::ptr::null_mut() }
    };
    __tsb_context_switch(
        __pa((*mm).pgd),
        &mut (*mm).context.tsb_block[MM_TSB_BASE],
        huge,
        __pa(&(*mm).context.tsb_descr[MM_TSB_BASE]),
        ctx,
    );
}

#[inline]
unsafe fn tsb_context_switch(mm: *mut mm_struct) { tsb_context_switch_ctx(mm, 0); }

#[cfg(not(feature = "smp"))]
#[inline]
unsafe fn smp_tsb_sync(_mm: *mut mm_struct) {}

// Set MMU context in the actual hardware. The original SPARC inline assembly
// is retained as the required low-level operation for a SPARC backend.
#[inline]
unsafe fn load_secondary_context(mm: *mut mm_struct) {
    core::arch::asm!(
        "stxa {ctx}, [{secondary}] {dmmu}",
        "flush %g6",
        ctx = in(reg) CTX_HWBITS((*mm).context),
        secondary = in(reg) SECONDARY_CONTEXT,
        dmmu = const ASI_DMMU,
        options(nostack)
    );
}

// Switch the current MM context.
#[inline]
unsafe fn switch_mm(old_mm: *mut mm_struct, mm: *mut mm_struct, tsk: *mut task_struct) {
    let _ = old_mm;
    let _ = tsk;
    let mut ctx_valid: c_ulong;
    let mut flags: c_ulong = 0;
    let cpu: c_int = smp_processor_id();

    per_cpu(per_cpu_secondary_mm, cpu) = mm;
    if unlikely(mm == &mut init_mm as *mut mm_struct) { return; }

    spin_lock_irqsave(&mut (*mm).context.lock, &mut flags);
    ctx_valid = CTX_VALID((*mm).context);
    if ctx_valid == 0 { get_new_mmu_context(mm); }

    tsb_context_switch_ctx(mm, CTX_HWBITS((*mm).context));
    if ctx_valid == 0 || !cpumask_test_cpu(cpu, mm_cpumask(mm)) {
        cpumask_set_cpu(cpu, mm_cpumask(mm));
        __flush_tlb_mm(CTX_HWBITS((*mm).context), SECONDARY_CONTEXT);
    }
    spin_unlock_irqrestore(&mut (*mm).context.lock, flags);
}

#[inline]
unsafe fn activate_mm(active_mm: *mut mm_struct, mm: *mut mm_struct) {
    switch_mm(active_mm, mm, core::ptr::null_mut());
}

// __HAVE_ARCH_START_CONTEXT_SWITCH
#[inline]
unsafe fn arch_start_context_switch(prev: *mut task_struct) {
    if adi_capable() {
        let tmp_mcdper: c_ulong;
        core::arch::asm!(".word 0x83438000", "mov %g1, {out}", out = lateout(reg) tmp_mcdper);
        if tmp_mcdper != 0 { set_tsk_thread_flag(prev, TIF_MCDPER); }
        else { clear_tsk_thread_flag(prev, TIF_MCDPER); }
    }
}

#[inline]
unsafe fn finish_arch_post_lock_switch() {
    if adi_capable() {
        let tmp_mcdper = test_thread_flag(TIF_MCDPER);
        core::arch::asm!("mov {input}, %g1", ".word 0x9d800001", ".word 0xaf902001", input = in(reg) tmp_mcdper);
        if !current.is_null() && !(*current).mm.is_null() && (*(*current).mm).context.adi {
            let regs = task_pt_regs(current);
            (*regs).tstate |= TSTATE_MCDE;
        }
    }
}

#[inline]
unsafe fn mm_untag_mask(_mm: *mut mm_struct) -> c_ulong { (!0 as c_ulong) >> adi_nbits() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
