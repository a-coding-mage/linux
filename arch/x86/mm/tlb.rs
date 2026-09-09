// SPDX-License-Identifier: GPL-2.0-only
// Translated from tlb.c. Kernel includes and externally supplied symbols are
// intentionally represented by their native Rust names and are not defined here.

const LAST_USER_MM_IBPB: usize = 0x1;
const LAST_USER_MM_L1D_FLUSH: usize = 0x2;
const LAST_USER_MM_SPEC_MASK: usize = LAST_USER_MM_IBPB | LAST_USER_MM_L1D_FLUSH;
const LAST_USER_MM_INIT: usize = LAST_USER_MM_IBPB;

#[repr(C)]
struct NewAsid { asid: u16, need_flush: bool }

#[inline]
unsafe fn kern_pcid(asid: u16) -> u16 {
    VM_WARN_ON_ONCE(asid as usize > MAX_ASID_AVAILABLE);
    asid.wrapping_add(1)
}

#[inline]
unsafe fn user_pcid(asid: u16) -> u16 {
    let mut ret = kern_pcid(asid);
    #[cfg(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION)]
    { ret |= 1 << X86_CR3_PTI_PCID_USER_BIT; }
    ret
}

#[inline]
unsafe fn build_cr3(pgd: *mut pgd_t, asid: u16, lam: usize) -> usize {
    let mut cr3 = __sme_pa(pgd) | lam;
    if cpu_feature_enabled(X86_FEATURE_PCID) { cr3 |= kern_pcid(asid) as usize; }
    else { VM_WARN_ON_ONCE(asid != 0); }
    cr3
}

#[inline]
unsafe fn build_cr3_noflush(pgd: *mut pgd_t, asid: u16, lam: usize) -> usize {
    VM_WARN_ON_ONCE(!boot_cpu_has(X86_FEATURE_PCID));
    build_cr3(pgd, asid, lam) | CR3_NOFLUSH
}

unsafe fn clear_asid_other() {
    if !cpu_feature_enabled(X86_FEATURE_PTI) { WARN_ON_ONCE(1); return; }
    for asid in 0..TLB_NR_DYN_ASIDS {
        if asid == this_cpu_read(cpu_tlbstate.loaded_mm_asid) { continue; }
        this_cpu_write(cpu_tlbstate.ctxs[asid].ctx_id, 0);
    }
    this_cpu_write(cpu_tlbstate.invalidate_other, false);
}

static mut last_mm_ctx_id: atomic64_t = ATOMIC64_INIT(1);

unsafe fn choose_new_asid(next: *mut mm_struct, next_tlb_gen: u64) -> NewAsid {
    let mut ns = NewAsid { asid: 0, need_flush: true };
    if !cpu_feature_enabled(X86_FEATURE_PCID) { return ns; }
    if cpu_feature_enabled(X86_FEATURE_INVLPGB) {
        let global_asid = mm_global_asid(next);
        if global_asid != 0 { return NewAsid { asid: global_asid, need_flush: false }; }
    }
    if this_cpu_read(cpu_tlbstate.invalidate_other) { clear_asid_other(); }
    for asid in 0..TLB_NR_DYN_ASIDS {
        if this_cpu_read(cpu_tlbstate.ctxs[asid].ctx_id) != (*next).context.ctx_id { continue; }
        return NewAsid { asid: asid as u16,
            need_flush: this_cpu_read(cpu_tlbstate.ctxs[asid].tlb_gen) < next_tlb_gen };
    }
    let mut asid = this_cpu_add_return(cpu_tlbstate.next_asid, 1) - 1;
    if asid >= TLB_NR_DYN_ASIDS { asid = 0; this_cpu_write(cpu_tlbstate.next_asid, 1); }
    ns.asid = asid as u16; ns
}

static mut global_asid_lock: raw_spinlock_t = DEFINE_RAW_SPINLOCK();
static mut last_global_asid: u16 = MAX_ASID_AVAILABLE;
static mut global_asid_available: i32 = MAX_ASID_AVAILABLE - TLB_NR_DYN_ASIDS - 1;

unsafe fn reset_global_asid_space() {
    lockdep_assert_held(&global_asid_lock);
    invlpgb_flush_all_nonglobals();
    bitmap_andnot(global_asid_used, global_asid_used, global_asid_freed, MAX_ASID_AVAILABLE);
    bitmap_clear(global_asid_freed, 0, MAX_ASID_AVAILABLE);
    last_global_asid = TLB_NR_DYN_ASIDS;
}

unsafe fn allocate_global_asid() -> u16 {
    lockdep_assert_held(&global_asid_lock);
    if last_global_asid >= MAX_ASID_AVAILABLE - 1 { reset_global_asid_space(); }
    let asid = find_next_zero_bit(global_asid_used, MAX_ASID_AVAILABLE, last_global_asid);
    if asid >= MAX_ASID_AVAILABLE && global_asid_available == 0 {
        VM_WARN_ONCE(1, "Unable to allocate global ASID despite available\n"); return 0;
    }
    __set_bit(asid, global_asid_used); last_global_asid = asid; global_asid_available -= 1; asid as u16
}

unsafe fn mm_active_cpus_exceeds(mm: *mut mm_struct, threshold: i32) -> bool {
    let mut count = 0;
    if cpumask_weight(mm_cpumask(mm)) <= threshold { return false; }
    for_each_cpu!(cpu, mm_cpumask(mm)) {
        if per_cpu(cpu_tlbstate.loaded_mm, cpu) != mm || per_cpu(cpu_tlbstate_shared.is_lazy, cpu) { continue; }
        count += 1; if count > threshold { return true; }
    }
    false
}

unsafe fn use_global_asid(mm: *mut mm_struct) {
    guard!(raw_spinlock_irqsave, &global_asid_lock);
    if mm_global_asid(mm) != 0 || global_asid_available == 0 { return; }
    let asid = allocate_global_asid(); if asid == 0 { return; }
    mm_assign_global_asid(mm, asid);
}

unsafe fn mm_needs_global_asid(mm: *mut mm_struct, asid: u16) -> bool {
    if !cpu_feature_enabled(X86_FEATURE_INVLPGB) { return false; }
    let global_asid = mm_global_asid(mm); global_asid != 0 && asid != global_asid
}

unsafe fn consider_global_asid(mm: *mut mm_struct) {
    if !cpu_feature_enabled(X86_FEATURE_INVLPGB) || (current->pid & 0x1f) != (jiffies & 0x1f) { return; }
    if mm_active_cpus_exceeds(mm, 3) { use_global_asid(mm); }
}

unsafe fn finish_asid_transition(info: *mut flush_tlb_info) {
    let mm = (*info).mm; let bc_asid = mm_global_asid(mm);
    if !mm_in_asid_transition(mm) { return; }
    for_each_cpu!(cpu, mm_cpumask(mm)) {
        while READ_ONCE(per_cpu(cpu_tlbstate.loaded_mm, cpu)) == LOADED_MM_SWITCHING { cpu_relax(); }
        if READ_ONCE(per_cpu(cpu_tlbstate.loaded_mm, cpu)) != mm { continue; }
        if READ_ONCE(per_cpu(cpu_tlbstate.loaded_mm_asid, cpu)) != bc_asid {
            flush_tlb_multi(mm_cpumask((*info).mm), info); return;
        }
    }
    mm_clear_asid_transition(mm);
}

unsafe fn broadcast_tlb_flush(info: *mut flush_tlb_info) {
    let pmd = (*info).stride_shift == PMD_SHIFT; let asid = mm_global_asid((*info).mm);
    let mut addr = (*info).start;
    if (*info).end == TLB_FLUSH_ALL {
        invlpgb_flush_single_pcid_nosync(kern_pcid(asid));
        if cpu_feature_enabled(X86_FEATURE_PTI) { invlpgb_flush_single_pcid_nosync(user_pcid(asid)); }
    } else { while addr < (*info).end {
        let mut nr = 1; if (*info).stride_shift <= PMD_SHIFT { nr = clamp_val(((*info).end-addr)>>(*info).stride_shift, 1, invlpgb_count_max); }
        invlpgb_flush_user_nr_nosync(kern_pcid(asid), addr, nr, pmd);
        if cpu_feature_enabled(X86_FEATURE_PTI) { invlpgb_flush_user_nr_nosync(user_pcid(asid), addr, nr, pmd); }
        addr += nr << (*info).stride_shift;
    }}
    finish_asid_transition(info); __tlbsync();
}

#[inline] unsafe fn invalidate_user_asid(asid: u16) {
    if !IS_ENABLED(CONFIG_MITIGATION_PAGE_TABLE_ISOLATION) || !cpu_feature_enabled(X86_FEATURE_PCID) || !cpu_feature_enabled(X86_FEATURE_PTI) { return; }
    __set_bit(kern_pcid(asid), this_cpu_ptr(&mut cpu_tlbstate.user_pcid_flush_mask) as *mut unsigned_long);
}

unsafe fn load_new_mm_cr3(pgdir: *mut pgd_t, asid: u16, lam: usize, need_flush: bool) {
    let cr3 = if need_flush { invalidate_user_asid(asid); build_cr3(pgdir, asid, lam) } else { build_cr3_noflush(pgdir, asid, lam) };
    write_cr3(cr3);
}

pub unsafe fn leave_mm() { let loaded_mm = this_cpu_read(cpu_tlbstate.loaded_mm); if loaded_mm == &mut init_mm { return; } WARN_ON(!this_cpu_read(cpu_tlbstate_shared.is_lazy)); switch_mm(core::ptr::null_mut(), &mut init_mm, core::ptr::null_mut()); }

pub unsafe fn switch_mm(_prev: *mut mm_struct, next: *mut mm_struct, tsk: *mut task_struct) { let mut flags=0; local_irq_save(&mut flags); switch_mm_irqs_off(core::ptr::null_mut(), next, tsk); local_irq_restore(flags); }

unsafe fn l1d_flush_force_sigbus(_ch: *mut callback_head) { force_sig(SIGBUS); }
unsafe fn l1d_flush_evaluate(prev_mm: usize, next_mm: usize, next: *mut task_struct) { if prev_mm & LAST_USER_MM_L1D_FLUSH != 0 { wrmsrq(MSR_IA32_FLUSH_CMD, L1D_FLUSH); } if next_mm & LAST_USER_MM_L1D_FLUSH == 0 { return; } if this_cpu_read(cpu_info.smt_active) { clear_ti_thread_flag(&mut (*next).thread_info, TIF_SPEC_L1D_FLUSH); (*next).l1d_flush_kill.func=Some(l1d_flush_force_sigbus); task_work_add(next, &mut (*next).l1d_flush_kill, TWA_RESUME); } }
unsafe fn mm_mangle_tif_spec_bits(next: *mut task_struct) -> usize { let flags=read_task_thread_flags(next); let bits=(flags>>TIF_SPEC_IB)&LAST_USER_MM_SPEC_MASK; (*next).mm as usize | bits }
unsafe fn cond_mitigation(next: *mut task_struct) { if next.is_null() || (*next).mm.is_null() { return; } let next_mm=mm_mangle_tif_spec_bits(next); let prev_mm=this_cpu_read(cpu_tlbstate.last_user_mm_spec); if static_branch_likely(&switch_mm_cond_ibpb) && next_mm!=prev_mm && (next_mm|prev_mm)&LAST_USER_MM_IBPB!=0 { indirect_branch_prediction_barrier(); } if static_branch_unlikely(&switch_mm_always_ibpb) && (prev_mm&!LAST_USER_MM_SPEC_MASK)!=(*next).mm as usize { indirect_branch_prediction_barrier(); } if static_branch_unlikely(&switch_mm_cond_l1d_flush) && (prev_mm|next_mm)&LAST_USER_MM_L1D_FLUSH!=0 { l1d_flush_evaluate(prev_mm,next_mm,next); } this_cpu_write(cpu_tlbstate.last_user_mm_spec,next_mm); }

pub unsafe fn switch_mm_irqs_off(_unused: *mut mm_struct, next: *mut mm_struct, tsk: *mut task_struct) {
    let prev=this_cpu_read(cpu_tlbstate.loaded_mm); let prev_asid=this_cpu_read(cpu_tlbstate.loaded_mm_asid); let was_lazy=this_cpu_read(cpu_tlbstate_shared.is_lazy); let cpu=smp_processor_id();
    if prev==next { if is_global_asid(prev_asid) { return; } if !was_lazy { return; } smp_mb(); let gen=atomic64_read(&(*next).context.tlb_gen); if this_cpu_read(cpu_tlbstate.ctxs[prev_asid].tlb_gen)==gen { return; } let ns=NewAsid{asid:prev_asid,need_flush:true}; reload_tlb(next,prev,ns,gen,cpu); return; }
    cond_mitigation(tsk); this_cpu_write(cpu_tlbstate.loaded_mm,LOADED_MM_SWITCHING); if next!=&mut init_mm && !cpumask_test_cpu(cpu,mm_cpumask(next)) { cpumask_set_cpu(cpu,mm_cpumask(next)); } else { smp_mb(); } let gen=atomic64_read(&(*next).context.tlb_gen); let ns=choose_new_asid(next,gen); reload_tlb(next,prev,ns,gen,cpu);
}

unsafe fn reload_tlb(next:*mut mm_struct, prev:*mut mm_struct, ns:NewAsid, gen:u64, _cpu:unsigned) { let lam=mm_lam_cr3_mask(next); if ns.need_flush { this_cpu_write(cpu_tlbstate.ctxs[ns.asid].ctx_id,(*next).context.ctx_id); this_cpu_write(cpu_tlbstate.ctxs[ns.asid].tlb_gen,gen); load_new_mm_cr3((*next).pgd,ns.asid,lam,true); trace_tlb_flush(TLB_FLUSH_ON_TASK_SWITCH,TLB_FLUSH_ALL); } else { load_new_mm_cr3((*next).pgd,ns.asid,lam,false); trace_tlb_flush(TLB_FLUSH_ON_TASK_SWITCH,0); } barrier(); this_cpu_write(cpu_tlbstate.loaded_mm,next); this_cpu_write(cpu_tlbstate.loaded_mm_asid,ns.asid); cpu_tlbstate_update_lam(lam,mm_untag_mask(next)); if next!=prev { cr4_update_pce_mm(next); switch_ldt(prev,next); } }

#[cfg(CONFIG_PERF_EVENTS)] unsafe fn cr4_update_pce_mm(mm:*mut mm_struct) { if static_branch_unlikely(&rdpmc_always_available_key) || (!static_branch_unlikely(&rdpmc_never_available_key) && atomic_read(&(*mm).context.perf_rdpmc_allowed)!=0) { perf_clear_dirty_counters(); cr4_set_bits_irqsoff(X86_CR4_PCE); } else { cr4_clear_bits_irqsoff(X86_CR4_PCE); } }
#[cfg(CONFIG_PERF_EVENTS)] pub unsafe fn cr4_update_pce(_ignored:*mut core::ffi::c_void) { cr4_update_pce_mm(this_cpu_read(cpu_tlbstate.loaded_mm)); }

pub static mut tlb_single_page_flush_ceiling: usize = 33;
unsafe fn init_flush_tlb_info(info:*mut flush_tlb_info,mm:*mut mm_struct,mut start:usize,mut end:usize,stride_shift:unsigned,freed_tables:bool,new_tlb_gen:u64) { if (end-start)>>stride_shift > tlb_single_page_flush_ceiling { start=0; end=TLB_FLUSH_ALL; } (*info).start=start;(*info).end=end;(*info).mm=mm;(*info).stride_shift=stride_shift;(*info).freed_tables=freed_tables;(*info).new_tlb_gen=new_tlb_gen;(*info).initiating_cpu=smp_processor_id();(*info).trim_cpumask=0; }

pub unsafe fn flush_tlb_mm_range(mm:*mut mm_struct,start:usize,end:usize,stride_shift:unsigned,freed_tables:bool) { let mut info=core::mem::zeroed::<flush_tlb_info>(); let cpu=get_cpu(); let gen=inc_mm_tlb_gen(mm); init_flush_tlb_info(&mut info,mm,start,end,stride_shift,freed_tables,gen); if mm_global_asid(mm)!=0 { broadcast_tlb_flush(&mut info); } else if cpumask_any_but(mm_cpumask(mm),cpu)<nr_cpu_ids { info.trim_cpumask=should_trim_cpumask(mm); flush_tlb_multi(mm_cpumask(mm),&info); } else if mm==this_cpu_read(cpu_tlbstate.loaded_mm) { local_irq_disable(); flush_tlb_func(&mut info); local_irq_enable(); } put_cpu(); mmu_notifier_arch_invalidate_secondary_tlbs(mm,start,end); }

pub unsafe fn use_temporary_mm(temp_mm:*mut mm_struct)->*mut mm_struct { lockdep_assert_preemption_disabled(); guard!(irqsave); if this_cpu_read(cpu_tlbstate_shared.is_lazy) { leave_mm(); } let prev=this_cpu_read(cpu_tlbstate.loaded_mm); switch_mm_irqs_off(core::ptr::null_mut(),temp_mm,current); if hw_breakpoint_active() { hw_breakpoint_disable(); } prev }
pub unsafe fn unuse_temporary_mm(prev_mm:*mut mm_struct) { lockdep_assert_preemption_disabled(); guard!(irqsave); cpumask_clear_cpu(smp_processor_id(),mm_cpumask(this_cpu_read(cpu_tlbstate.loaded_mm))); switch_mm_irqs_off(core::ptr::null_mut(),prev_mm,current); if hw_breakpoint_active() { hw_breakpoint_restore(); } }

pub unsafe fn initialize_tlbstate_and_flush() { let mm=this_cpu_read(cpu_tlbstate.loaded_mm); let gen=atomic64_read(&init_mm.context.tlb_gen); let lam=mm_lam_cr3_mask(mm); WARN_ON((__read_cr3()&CR3_ADDR_MASK)!=__pa((*mm).pgd)); WARN_ON(__read_cr3()&(X86_CR3_LAM_U48|X86_CR3_LAM_U57)); WARN_ON(lam); write_cr3(build_cr3((*mm).pgd,0,0)); this_cpu_write(cpu_tlbstate.last_user_mm_spec,LAST_USER_MM_INIT); this_cpu_write(cpu_tlbstate.loaded_mm_asid,0); this_cpu_write(cpu_tlbstate.next_asid,1); this_cpu_write(cpu_tlbstate.ctxs[0].ctx_id,(*mm).context.ctx_id); this_cpu_write(cpu_tlbstate.ctxs[0].tlb_gen,gen); cpu_tlbstate_update_lam(lam,mm_untag_mask(mm)); for i in 1..TLB_NR_DYN_ASIDS { this_cpu_write(cpu_tlbstate.ctxs[i].ctx_id,0); } }

unsafe fn should_trim_cpumask(mm:*mut mm_struct)->bool { if time_after(jiffies,READ_ONCE((*mm).context.next_trim_cpumask)) { WRITE_ONCE((*mm).context.next_trim_cpumask,jiffies+HZ); return true; } false }
unsafe fn should_flush_tlb(cpu:i32,data:*mut core::ffi::c_void)->bool { let loaded=per_cpu(cpu_tlbstate.loaded_mm,cpu); let info=data as *mut flush_tlb_info; smp_rmb(); if per_cpu(cpu_tlbstate_shared.is_lazy,cpu) { return false; } if (*info).mm.is_null() || loaded==LOADED_MM_SWITCHING || loaded==(*info).mm || (*info).trim_cpumask { return true; } false }

pub unsafe fn flush_tlb_multi(mask:*const cpumask,info:*const flush_tlb_info) { __flush_tlb_multi(mask,info); }

// Remaining TLB shootdown entry points are supplied by the surrounding kernel translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
