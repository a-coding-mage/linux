// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Performance event support - powerpc architecture code
 *
 * Copyright 2008-2009 Paul Mackerras, IBM Corporation.
 */
// Kernel and architecture headers from the C implementation are external dependencies.

const BHRB_MAX_ENTRIES: usize = 32;
const BHRB_TARGET: u64 = 0x0000000000000002;
const BHRB_PREDICTION: u64 = 0x0000000000000001;
const BHRB_EA: u64 = 0xFFFFFFFFFFFFFFFC;

#[repr(C)]
pub struct CpuHwEvents {
    n_events: i32,
    n_percpu: i32,
    disabled: i32,
    n_added: i32,
    n_limited: i32,
    pmcs_enabled: u8,
    event: [*mut perf_event; MAX_HWEVENTS],
    events: [u64; MAX_HWEVENTS],
    flags: [u32; MAX_HWEVENTS],
    mmcr: mmcr_regs,
    limited_counter: [*mut perf_event; MAX_LIMITED_HWCOUNTERS],
    limited_hwidx: [u8; MAX_LIMITED_HWCOUNTERS],
    alternatives: [[u64; MAX_EVENT_ALTERNATIVES]; MAX_HWEVENTS],
    amasks: [[usize; MAX_EVENT_ALTERNATIVES]; MAX_HWEVENTS],
    avalues: [[usize; MAX_EVENT_ALTERNATIVES]; MAX_HWEVENTS],
    txn_flags: u32,
    n_txn_start: i32,
    bhrb_filter: u64,
    bhrb_users: u32,
    bhrb_context: *mut core::ffi::c_void,
    bhrb_stack: perf_branch_stack,
    bhrb_entries: [perf_branch_entry; BHRB_MAX_ENTRIES],
    ic_init: u64,
    pmcs: [usize; MAX_HWEVENTS],
}

static mut CPU_HW_EVENTS: CpuHwEvents = unsafe { core::mem::zeroed() };
static mut ppmu: *mut power_pmu = core::ptr::null_mut();
static mut freeze_events_kernel: u32 = MMCR0_FCS;

#[cfg(feature = "CONFIG_PPC32")]
const MMCR0_FCHV: u32 = 0;

#[cfg(feature = "CONFIG_PPC32")]
#[inline]
unsafe fn perf_ip_adjust(_regs: *mut pt_regs) -> usize { 0 }
#[cfg(feature = "CONFIG_PPC32")]
#[inline]
unsafe fn perf_get_data_addr(_event: *mut perf_event, _regs: *mut pt_regs, _addrp: *mut u64) {}
#[cfg(feature = "CONFIG_PPC32")]
#[inline]
unsafe fn perf_get_misc_flags(_regs: *mut pt_regs) -> u32 { 0 }
#[cfg(feature = "CONFIG_PPC32")]
#[inline]
unsafe fn perf_read_regs(regs: *mut pt_regs) { (*regs).result = 0; }
#[cfg(feature = "CONFIG_PPC32")]
#[inline]
unsafe fn siar_valid(_regs: *mut pt_regs) -> i32 { 1 }
#[cfg(feature = "CONFIG_PPC32")]
unsafe fn is_ebb_event(_event: *mut perf_event) -> bool { false }
#[cfg(feature = "CONFIG_PPC32")]
unsafe fn ebb_event_check(_event: *mut perf_event) -> i32 { 0 }
#[cfg(feature = "CONFIG_PPC32")]
unsafe fn ebb_event_add(_event: *mut perf_event) {}
#[cfg(feature = "CONFIG_PPC32")]
unsafe fn ebb_switch_out(_mmcr0: usize) {}
#[cfg(feature = "CONFIG_PPC32")]
unsafe fn ebb_switch_in(_ebb: bool, cpuhw: *mut CpuHwEvents) -> usize { (*cpuhw).mmcr.mmcr0 }
#[cfg(feature = "CONFIG_PPC32")]
unsafe fn power_pmu_bhrb_enable(_event: *mut perf_event) {}
#[cfg(feature = "CONFIG_PPC32")]
unsafe fn power_pmu_bhrb_disable(_event: *mut perf_event) {}
#[cfg(feature = "CONFIG_PPC32")]
unsafe fn power_pmu_sched_task(_pmu_ctx: *mut perf_event_pmu_context, _task: *mut task_struct, _sched_in: bool) {}
#[cfg(feature = "CONFIG_PPC32")]
unsafe fn power_pmu_bhrb_read(_event: *mut perf_event, _cpuhw: *mut CpuHwEvents) {}
#[cfg(feature = "CONFIG_PPC32")]
unsafe fn pmao_restore_workaround(_ebb: bool) {}

pub unsafe fn is_sier_available() -> bool {
    !ppmu.is_null() && ((*ppmu).flags & PPMU_HAS_SIER) != 0
}

pub unsafe fn get_pmcs_ext_regs(idx: i32) -> usize {
    CPU_HW_EVENTS.pmcs[idx as usize]
}

unsafe fn regs_use_siar(regs: *mut pt_regs) -> bool {
    TRAP(regs) == INTERRUPT_PERFMON && (*regs).result != 0
}

#[cfg(feature = "CONFIG_PPC64")]
#[inline]
unsafe fn perf_ip_adjust(regs: *mut pt_regs) -> usize {
    let mmcra = (*regs).dsisr;
    if ((*ppmu).flags & PPMU_HAS_SSLOT) != 0 && (mmcra & MMCRA_SAMPLE_ENABLE) != 0 {
        let slot = (mmcra & MMCRA_SLOT) >> MMCRA_SLOT_SHIFT;
        if slot > 1 { return 4 * (slot - 1) as usize; }
    }
    0
}

#[cfg(feature = "CONFIG_PPC64")]
unsafe fn perf_get_data_addr(event: *mut perf_event, regs: *mut pt_regs, addrp: *mut u64) {
    let mmcra = (*regs).dsisr;
    let sdar_valid = if ((*ppmu).flags & PPMU_HAS_SIER) != 0 { ((*regs).dar & SIER_SDAR_VALID) != 0 } else {
        let sdsync = if ((*ppmu).flags & PPMU_SIAR_VALID) != 0 { POWER7P_MMCRA_SDAR_VALID }
            else if ((*ppmu).flags & PPMU_ALT_SIPR) != 0 { POWER6_MMCRA_SDSYNC }
            else if ((*ppmu).flags & PPMU_NO_SIAR) != 0 { MMCRA_SAMPLE_ENABLE } else { MMCRA_SDSYNC };
        (mmcra & sdsync) != 0
    };
    if (mmcra & MMCRA_SAMPLE_ENABLE) == 0 || sdar_valid { *addrp = mfspr(SPRN_SDAR); }
    if is_kernel_addr(mfspr(SPRN_SDAR)) && (*event).attr.exclude_kernel { *addrp = 0; }
}

#[cfg(feature = "CONFIG_PPC64")]
unsafe fn regs_sihv(regs: *mut pt_regs) -> bool {
    let sihv = if ((*ppmu).flags & PPMU_ALT_SIPR) != 0 { POWER6_MMCRA_SIHV } else { MMCRA_SIHV };
    if ((*ppmu).flags & PPMU_HAS_SIER) != 0 { ((*regs).dar & SIER_SIHV) != 0 } else { ((*regs).dsisr & sihv) != 0 }
}

#[cfg(feature = "CONFIG_PPC64")]
unsafe fn regs_sipr(regs: *mut pt_regs) -> bool {
    let sipr = if ((*ppmu).flags & PPMU_ALT_SIPR) != 0 { POWER6_MMCRA_SIPR } else { MMCRA_SIPR };
    if ((*ppmu).flags & PPMU_HAS_SIER) != 0 { ((*regs).dar & SIER_SIPR) != 0 } else { ((*regs).dsisr & sipr) != 0 }
}

#[cfg(feature = "CONFIG_PPC64")]
unsafe fn perf_flags_from_msr(regs: *mut pt_regs) -> u32 {
    if user_mode(regs) { return PERF_RECORD_MISC_USER; }
    if ((*regs).msr & MSR_HV) != 0 && freeze_events_kernel != MMCR0_FCHV { return PERF_RECORD_MISC_HYPERVISOR; }
    PERF_RECORD_MISC_KERNEL
}

#[cfg(feature = "CONFIG_PPC64")]
unsafe fn perf_get_misc_flags(regs: *mut pt_regs) -> u32 {
    if !regs_use_siar(regs) { return perf_flags_from_msr(regs); }
    if ((*ppmu).flags & PPMU_NO_SIPR) != 0 {
        return if is_kernel_addr(mfspr(SPRN_SIAR)) { PERF_RECORD_MISC_KERNEL } else { PERF_RECORD_MISC_USER };
    }
    if regs_sipr(regs) && ((*ppmu).flags & PPMU_P10) == 0 { return PERF_RECORD_MISC_USER; }
    if regs_sihv(regs) && freeze_events_kernel != MMCR0_FCHV { return PERF_RECORD_MISC_HYPERVISOR; }
    if ((*ppmu).flags & PPMU_P10) != 0 {
        let siar = mfspr(SPRN_SIAR); let addr = if siar != 0 { siar } else { (*regs).nip };
        if !is_kernel_addr(addr) { return PERF_RECORD_MISC_USER; }
    }
    PERF_RECORD_MISC_KERNEL
}

#[cfg(feature = "CONFIG_PPC64")]
unsafe fn perf_read_regs(regs: *mut pt_regs) {
    let mmcra = mfspr(SPRN_MMCRA); let marked = mmcra & MMCRA_SAMPLE_ENABLE;
    (*regs).dsisr = mmcra;
    if ((*ppmu).flags & PPMU_HAS_SIER) != 0 { (*regs).dar = mfspr(SPRN_SIER); }
    let use_siar = if TRAP(regs) != INTERRUPT_PERFMON || ((*ppmu).flags & PPMU_NO_SIAR) != 0 { 0 }
        else if marked != 0 { 1 } else if ((*ppmu).flags & PPMU_NO_CONT_SAMPLING) != 0 { 0 }
        else if !user_mode(regs) { 1 } else if ((*ppmu).flags & PPMU_NO_SIPR) == 0 && regs_sipr(regs) { 0 } else { 1 };
    (*regs).result = use_siar;
}

#[cfg(feature = "CONFIG_PPC64")]
unsafe fn siar_valid(regs: *mut pt_regs) -> i32 {
    let mmcra = (*regs).dsisr; if (mmcra & MMCRA_SAMPLE_ENABLE) != 0 {
        if ((*ppmu).flags & PPMU_P10_DD1) != 0 { return 1; }
        if ((*ppmu).flags & PPMU_HAS_SIER) != 0 { return if ((*regs).dar & SIER_SIAR_VALID) != 0 { 1 } else { 0 }; }
        if ((*ppmu).flags & PPMU_SIAR_VALID) != 0 { return if (mmcra & POWER7P_MMCRA_SIAR_VALID) != 0 { 1 } else { 0 }; }
    } 1
}

#[cfg(feature = "CONFIG_PPC64")]
unsafe fn power_pmu_bhrb_reset() { core::arch::asm!("" /* PPC_CLRBHRB */); }

#[cfg(feature = "CONFIG_PPC64")]
unsafe fn power_pmu_bhrb_enable(event: *mut perf_event) {
    let cpuhw = &mut CPU_HW_EVENTS;
    if (*ppmu).bhrb_nr == 0 { return; }
    if !(*event).ctx.task.is_null() && cpuhw.bhrb_context != (*event).ctx as *mut _ { power_pmu_bhrb_reset(); cpuhw.bhrb_context = (*event).ctx as *mut _; }
    cpuhw.bhrb_users += 1; perf_sched_cb_inc((*event).pmu);
}

#[cfg(feature = "CONFIG_PPC64")]
unsafe fn power_pmu_bhrb_disable(event: *mut perf_event) {
    let cpuhw = &mut CPU_HW_EVENTS; if (*ppmu).bhrb_nr == 0 { return; }
    WARN_ON_ONCE(cpuhw.bhrb_users == 0); cpuhw.bhrb_users -= 1; perf_sched_cb_dec((*event).pmu);
    if cpuhw.disabled == 0 && cpuhw.bhrb_users == 0 { cpuhw.bhrb_context = core::ptr::null_mut(); }
}

#[cfg(feature = "CONFIG_PPC64")]
unsafe fn power_pmu_sched_task(_pmu_ctx: *mut perf_event_pmu_context, _task: *mut task_struct, sched_in: bool) {
    if (*ppmu).bhrb_nr != 0 && sched_in { power_pmu_bhrb_reset(); }
}

#[cfg(feature = "CONFIG_PPC64")]
unsafe fn power_pmu_bhrb_to(addr: u64) -> u64 {
    let mut instr: u32 = 0;
    if is_kernel_addr(addr) { if copy_from_kernel_nofault(&mut instr, addr as *mut _, core::mem::size_of::<u32>()) != 0 { return 0; } }
    else if copy_from_user_nofault(&mut instr, addr as *const _, core::mem::size_of::<u32>()) != 0 { return 0; }
    let target = branch_target(&instr); if target == 0 || (instr & BRANCH_ABSOLUTE) != 0 { target } else { target.wrapping_sub((&instr as *const _ as usize) as u64).wrapping_add(addr) }
}

#[cfg(feature = "CONFIG_PPC64")]
unsafe fn power_pmu_bhrb_read(_event: *mut perf_event, cpuhw: *mut CpuHwEvents) {
    let mut r_index = 0; let mut u_index = 0;
    while r_index < (*ppmu).bhrb_nr {
        let val = read_bhrb(r_index); r_index += 1; if val == 0 { break; }
        let mut addr = val & BHRB_EA; let pred = val & BHRB_PREDICTION; if addr == 0 { continue; }
        if ((*ppmu).flags & PPMU_ARCH_31) == 0 && is_kernel_addr(addr) && (*_event).attr.exclude_kernel { continue; }
        if (val & BHRB_TARGET) != 0 {
            (*cpuhw).bhrb_entries[u_index].to = addr; (*cpuhw).bhrb_entries[u_index].mispred = pred; (*cpuhw).bhrb_entries[u_index].predicted = !pred;
            let val2 = read_bhrb(r_index); r_index += 1; addr = val2 & BHRB_EA;
            if (val2 & BHRB_TARGET) != 0 { r_index -= 1; addr = 0; }
            (*cpuhw).bhrb_entries[u_index].from = addr;
        } else {
            (*cpuhw).bhrb_entries[u_index].from = addr; (*cpuhw).bhrb_entries[u_index].to = power_pmu_bhrb_to(addr);
            (*cpuhw).bhrb_entries[u_index].mispred = pred; (*cpuhw).bhrb_entries[u_index].predicted = !pred;
        }
        u_index += 1;
    }
    (*cpuhw).bhrb_stack.nr = u_index; (*cpuhw).bhrb_stack.hw_idx = u64::MAX;
}

unsafe fn is_ebb_event(event: *mut perf_event) -> bool { ((*ppmu).flags & PPMU_ARCH_207S) != 0 && (((*event).attr.config >> PERF_EVENT_CONFIG_EBB_SHIFT) & 1) != 0 }
unsafe fn ebb_event_check(event: *mut perf_event) -> i32 {
    let leader = (*event).group_leader; if is_ebb_event(leader) != is_ebb_event(event) { return -EINVAL; }
    if is_ebb_event(event) { if ((*event).attach_state & PERF_ATTACH_TASK) == 0 || !(*leader).attr.pinned || !(*leader).attr.exclusive { return -EINVAL; }
        if (*event).attr.freq || (*event).attr.inherit || (*event).attr.sample_type != 0 || (*event).attr.sample_period != 0 || (*event).attr.enable_on_exec { return -EINVAL; } }
    0
}

unsafe fn ebb_event_add(event: *mut perf_event) {
    if !is_ebb_event(event) || current().thread.used_ebb { return; }
    current_mut().thread.used_ebb = true; current_mut().thread.mmcr0 |= MMCR0_PMXE;
}
unsafe fn ebb_switch_out(mmcr0: usize) {
    if (mmcr0 & MMCR0_EBE) == 0 { return; }
    let t = &mut current_mut().thread; t.siar = mfspr(SPRN_SIAR); t.sier = mfspr(SPRN_SIER); t.sdar = mfspr(SPRN_SDAR); t.mmcr0 = mmcr0 & MMCR0_USER_MASK; t.mmcr2 = mfspr(SPRN_MMCR2) & MMCR2_USER_MASK;
    if ((*ppmu).flags & PPMU_ARCH_31) != 0 { t.mmcr3 = mfspr(SPRN_MMCR3); t.sier2 = mfspr(SPRN_SIER2); t.sier3 = mfspr(SPRN_SIER3); }
}
unsafe fn ebb_switch_in(ebb: bool, cpuhw: *mut CpuHwEvents) -> usize {
    let mut mmcr0 = (*cpuhw).mmcr.mmcr0; if !ebb { return mmcr0; }
    mmcr0 |= MMCR0_EBE | MMCR0_BHRBA | MMCR0_PMCC_U6; mmcr0 |= current().thread.mmcr0; if (current().thread.mmcr0 & MMCR0_PMXE) == 0 { mmcr0 &= !MMCR0_PMXE; }
    mtspr(SPRN_SIAR, current().thread.siar); mtspr(SPRN_SIER, current().thread.sier); mtspr(SPRN_SDAR, current().thread.sdar); mtspr(SPRN_MMCR2, (*cpuhw).mmcr.mmcr2 | current().thread.mmcr2);
    if ((*ppmu).flags & PPMU_ARCH_31) != 0 { mtspr(SPRN_MMCR3, current().thread.mmcr3); mtspr(SPRN_SIER2, current().thread.sier2); mtspr(SPRN_SIER3, current().thread.sier3); } mmcr0
}

unsafe fn pmao_restore_workaround(ebb: bool) {
    if !cpu_has_feature(CPU_FTR_PMAO_BUG) || (current().thread.mmcr0 & (MMCR0_PMAO | MMCR0_PMAO_SYNC)) != MMCR0_PMAO || (ebb && (current().thread.bescr & BESCR_GE) == 0) { return; }
    hard_irq_disable(); let pmcs = [mfspr(SPRN_PMC1), mfspr(SPRN_PMC2), mfspr(SPRN_PMC3), mfspr(SPRN_PMC4), mfspr(SPRN_PMC5), mfspr(SPRN_PMC6)];
    mtspr(SPRN_MMCR2, 0); mtspr(SPRN_PMC6, 0x7ffffffe); mtspr(SPRN_MMCR0, MMCR0_PMXE | MMCR0_PMCjCE | MMCR0_PMAO); mtspr(SPRN_MMCR0, MMCR0_FC | MMCR0_PMAO);
    mtspr(SPRN_PMC1, pmcs[0]); mtspr(SPRN_PMC2, pmcs[1]); mtspr(SPRN_PMC3, pmcs[2]); mtspr(SPRN_PMC4, pmcs[3]); mtspr(SPRN_PMC5, pmcs[4]); mtspr(SPRN_PMC6, pmcs[5]);
}

pub unsafe fn power_pmu_wants_prompt_pmi() -> bool { !ppmu.is_null() && CPU_HW_EVENTS.n_events != 0 }

unsafe fn perf_event_interrupt(_regs: *mut pt_regs);
unsafe fn read_pmc(idx: i32) -> usize { match idx { 1 => mfspr(SPRN_PMC1), 2 => mfspr(SPRN_PMC2), 3 => mfspr(SPRN_PMC3), 4 => mfspr(SPRN_PMC4), 5 => mfspr(SPRN_PMC5), 6 => mfspr(SPRN_PMC6), #[cfg(feature="CONFIG_PPC64")] 7 => mfspr(SPRN_PMC7), #[cfg(feature="CONFIG_PPC64")] 8 => mfspr(SPRN_PMC8), _ => { printk!("oops trying to read PMC%d\n", idx); 0 } } }
unsafe fn write_pmc(idx: i32, val: usize) { match idx { 1 => mtspr(SPRN_PMC1,val), 2 => mtspr(SPRN_PMC2,val), 3 => mtspr(SPRN_PMC3,val), 4 => mtspr(SPRN_PMC4,val), 5 => mtspr(SPRN_PMC5,val), 6 => mtspr(SPRN_PMC6,val), #[cfg(feature="CONFIG_PPC64")] 7 => mtspr(SPRN_PMC7,val), #[cfg(feature="CONFIG_PPC64")] 8 => mtspr(SPRN_PMC8,val), _ => printk!("oops trying to write PMC%d\n", idx) } }
unsafe fn any_pmc_overflown(cpuhw: *mut CpuHwEvents) -> i32 { for i in 0..(*cpuhw).n_events { let idx = (*(*cpuhw).event[i as usize]).hw.idx; if idx != 0 && (read_pmc(idx) as isize) < 0 { return idx; } } 0 }

pub unsafe fn perf_event_print_debug() {
    if ppmu.is_null() { pr_info!("Performance monitor hardware not registered.\n"); return; }
    if (*ppmu).n_counter == 0 { return; }
    let mut flags = 0usize; local_irq_save(&mut flags); pr_info!("CPU: %d PMU registers, ppmu = %s n_counters = %d", smp_processor_id(), (*ppmu).name, (*ppmu).n_counter);
    let mut pmcs = [0u32; MAX_HWEVENTS]; let mut i = 0; while i < (*ppmu).n_counter { pmcs[i as usize] = read_pmc(i + 1) as u32; i += 1; } while i < MAX_HWEVENTS as i32 { pmcs[i as usize] = 0xdeadbeef; i += 1; }
    pr_info!("PMC1:  %08x PMC2: %08x PMC3: %08x PMC4: %08x\n", pmcs[0],pmcs[1],pmcs[2],pmcs[3]); if (*ppmu).n_counter > 4 { pr_info!("PMC5:  %08x PMC6: %08x PMC7: %08x PMC8: %08x\n",pmcs[4],pmcs[5],pmcs[6],pmcs[7]); }
    pr_info!("MMCR0: %016lx MMCR1: %016lx MMCRA: %016lx\n",mfspr(SPRN_MMCR0),mfspr(SPRN_MMCR1),mfspr(SPRN_MMCRA));
    let mut sdar = 0; let mut sier = 0;
    #[cfg(feature="CONFIG_PPC64")] { sdar=mfspr(SPRN_SDAR); if ((*ppmu).flags & PPMU_HAS_SIER)!=0 { sier=mfspr(SPRN_SIER); } if ((*ppmu).flags & PPMU_ARCH_207S)!=0 { pr_info!("MMCR2: %016lx EBBHR: %016lx\n",mfspr(SPRN_MMCR2),mfspr(SPRN_EBBHR)); pr_info!("EBBRR: %016lx BESCR: %016lx\n",mfspr(SPRN_EBBRR),mfspr(SPRN_BESCR)); } if ((*ppmu).flags & PPMU_ARCH_31)!=0 { pr_info!("MMCR3: %016lx SIER2: %016lx SIER3: %016lx\n",mfspr(SPRN_MMCR3),mfspr(SPRN_SIER2),mfspr(SPRN_SIER3)); } }
    pr_info!("SIAR:  %016lx SDAR:  %016lx SIER:  %016lx\n",mfspr(SPRN_SIAR),sdar,sier); local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
