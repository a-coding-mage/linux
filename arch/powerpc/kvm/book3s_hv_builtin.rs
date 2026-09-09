// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2011 Paul Mackerras, IBM Corp. <paulus@au1.ibm.com>
 */

// Linux and architecture dependencies are supplied by the surrounding kernel translation.

const KVM_CMA_CHUNK_ORDER: c_int = 18;
const HPT_ALIGN_PAGES: usize = (1usize << 18) >> PAGE_SHIFT;

static mut kvm_cma_resv_ratio: c_ulong = 5;
static mut kvm_cma: *mut cma = core::ptr::null_mut();

unsafe extern "C" fn early_parse_kvm_cma_resv(p: *mut c_char) -> c_int {
    pr_debug!("{}({})\n", "early_parse_kvm_cma_resv", p);
    if p.is_null() {
        return -EINVAL;
    }
    kstrtoul(p, 0, &mut kvm_cma_resv_ratio)
}

// early_param("kvm_cma_resv_ratio", early_parse_kvm_cma_resv);

#[no_mangle]
pub unsafe extern "C" fn kvm_alloc_hpt_cma(nr_pages: c_ulong) -> *mut page {
    VM_BUG_ON(order_base_2(nr_pages) < KVM_CMA_CHUNK_ORDER - PAGE_SHIFT as c_int);
    cma_alloc(kvm_cma, nr_pages, order_base_2(HPT_ALIGN_PAGES as c_ulong), false)
}

#[no_mangle]
pub unsafe extern "C" fn kvm_free_hpt_cma(page: *mut page, nr_pages: c_ulong) {
    cma_release(kvm_cma, page, nr_pages);
}

/* kvm_cma_reserve() - reserve area for kvm hash pagetable */
#[no_mangle]
pub unsafe extern "C" fn kvm_cma_reserve() {
    let mut align_size: c_ulong;
    let selected_size: phys_addr_t;

    if !cpu_has_feature(CPU_FTR_HVMODE) {
        return;
    }

    selected_size = PAGE_ALIGN(memblock_phys_mem_size() * kvm_cma_resv_ratio / 100);
    if selected_size != 0 {
        pr_info!("{}: reserving {} MiB for global area\n", "kvm_cma_reserve", selected_size as c_ulong / SZ_1M);
        align_size = (HPT_ALIGN_PAGES as c_ulong) << PAGE_SHIFT;
        cma_declare_contiguous(
            0, selected_size, 0, align_size,
            KVM_CMA_CHUNK_ORDER - PAGE_SHIFT as c_int, false,
            b"kvm_cma\0".as_ptr() as *const c_char, &mut kvm_cma,
        );
    }
}

pub unsafe extern "C" fn kvmppc_rm_h_confer(
    vcpu: *mut kvm_vcpu, target: c_int, yield_count: c_uint,
) -> c_long {
    let vc = (*local_paca).kvm_hstate.kvm_vcore;
    let ptid = (*local_paca).kvm_hstate.ptid;
    let mut threads_running: c_int;
    let mut threads_ceded: c_int;
    let mut threads_conferring: c_int;
    let stop = get_tb() + 10 * tb_ticks_per_usec;
    let mut rv = H_SUCCESS;

    set_bit(ptid, &mut (*vc).conferring_threads);
    while get_tb() < stop && !VCORE_IS_EXITING(vc) {
        threads_running = VCORE_ENTRY_MAP(vc);
        threads_ceded = (*vc).napping_threads;
        threads_conferring = (*vc).conferring_threads;
        if (threads_ceded | threads_conferring) == threads_running {
            rv = H_TOO_HARD;
            break;
        }
    }
    clear_bit(ptid, &mut (*vc).conferring_threads);
    rv
}

static mut hv_vm_count: atomic_t = atomic_t { counter: 0 };

pub unsafe extern "C" fn kvm_hv_vm_activated() {
    cpus_read_lock();
    atomic_inc(&mut hv_vm_count);
    cpus_read_unlock();
}

pub unsafe extern "C" fn kvm_hv_vm_deactivated() {
    cpus_read_lock();
    atomic_dec(&mut hv_vm_count);
    cpus_read_unlock();
}

pub unsafe extern "C" fn kvm_hv_mode_active() -> bool {
    atomic_read(&hv_vm_count) != 0
}

unsafe extern "C" {
    static mut hcall_real_table: [c_int; 0];
    static mut hcall_real_table_end: [c_int; 0];
    static mut kvmppc_host_rm_ops_hv: *mut kvmppc_host_rm_ops;
}

pub unsafe extern "C" fn kvmppc_hcall_impl_hv_realmode(mut cmd: c_ulong) -> c_int {
    cmd /= 4;
    if cmd < hcall_real_table_end.as_ptr().offset_from(hcall_real_table.as_ptr()) as c_ulong
        && *hcall_real_table.as_ptr().add(cmd as usize) != 0
    {
        return 1;
    }
    0
}

pub unsafe extern "C" fn kvmppc_hwrng_present() -> c_int {
    if (*ppc_md).get_random_seed.is_some() { 1 } else { 0 }
}

pub unsafe extern "C" fn kvmppc_rm_h_random(vcpu: *mut kvm_vcpu) -> c_long {
    let mut rand: c_ulong = 0;
    if let Some(get_random_seed) = (*ppc_md).get_random_seed {
        if get_random_seed(&mut rand) {
            kvmppc_set_gpr(vcpu, 4, rand);
            return H_SUCCESS;
        }
    }
    H_HARDWARE
}

pub unsafe extern "C" fn kvmhv_rm_send_ipi(cpu: c_int) {
    let mut xics_phys: *mut core::ffi::c_void;
    let mut msg = PPC_DBELL_TYPE(PPC_DBELL_SERVER);
    if cpu_has_feature(CPU_FTR_ARCH_300) {
        msg |= get_hard_smp_processor_id(cpu);
        core::arch::asm!(PPC_MSGSND, in("r0") msg);
        return;
    }
    if cpu_has_feature(CPU_FTR_ARCH_207S)
        && cpu_first_thread_sibling(cpu) == cpu_first_thread_sibling(raw_smp_processor_id())
    {
        msg |= cpu_thread_in_core(cpu);
        core::arch::asm!(PPC_MSGSND, in("r0") msg);
        return;
    }
    if WARN_ON_ONCE(xics_on_xive()) { return; }
    xics_phys = (*paca_ptrs.add(cpu as usize)).kvm_hstate.xics_phys;
    if !xics_phys.is_null() {
        __raw_rm_writeb(IPI_PRIORITY, xics_phys.add(XICS_MFRR as usize));
    } else {
        opal_int_set_mfrr(get_hard_smp_processor_id(cpu), IPI_PRIORITY);
    }
}

unsafe fn kvmhv_interrupt_vcore(vc: *mut kvmppc_vcore, active: c_int) {
    let mut cpu = (*vc).pcpu;
    smp_mb();
    let mut bits = active;
    while bits != 0 {
        if bits & 1 != 0 { kvmhv_rm_send_ipi(cpu); }
        bits >>= 1;
        cpu += 1;
    }
}

pub unsafe extern "C" fn kvmhv_commence_exit(trap: c_int) {
    let mut vc = (*local_paca).kvm_hstate.kvm_vcore;
    let ptid = (*local_paca).kvm_hstate.ptid;
    let sip = (*local_paca).kvm_hstate.kvm_split_mode;
    let me = 0x100 << ptid;
    let mut ee;
    loop { ee = (*vc).entry_exit_map; if cmpxchg(&mut (*vc).entry_exit_map, ee, ee | me) == ee { break; } }
    if (ee >> 8) != 0 { return; }
    if trap != BOOK3S_INTERRUPT_HV_DECREMENTER { kvmhv_interrupt_vcore(vc, ee & !(1 << ptid)); }
    if sip.is_null() { return; }
    for i in 0..MAX_SUBCORES {
        vc = (*sip).vc[i as usize];
        if vc.is_null() { break; }
        loop {
            ee = (*vc).entry_exit_map;
            if (ee >> 8) != 0 { break; }
            if cmpxchg(&mut (*vc).entry_exit_map, ee, ee | VCORE_EXIT_REQ) == ee { break; }
        }
        if (ee >> 8) == 0 { kvmhv_interrupt_vcore(vc, ee); }
    }
}

#[cfg(CONFIG_KVM_XICS)]
unsafe fn get_irqmap(pimap: *mut kvmppc_passthru_irqmap, xisr: u32) -> *mut kvmppc_irq_map {
    for i in 0..(*pimap).n_mapped {
        if xisr == (*pimap).mapped.add(i as usize).as_ref().unwrap().r_hwirq {
            smp_rmb();
            return (*pimap).mapped.add(i as usize);
        }
    }
    core::ptr::null_mut()
}

#[cfg(CONFIG_KVM_XICS)]
unsafe fn kvmppc_check_passthru(xisr: u32, xirr: __be32, again: *mut bool) -> c_int {
    let vcpu = (*local_paca).kvm_hstate.kvm_vcpu;
    if vcpu.is_null() { return 1; }
    let pimap = kvmppc_get_passthru_irqmap((*vcpu).kvm);
    if pimap.is_null() { return 1; }
    let irq_map = get_irqmap(pimap, xisr);
    if irq_map.is_null() { return 1; }
    (*local_paca).kvm_hstate.saved_xirr = 0;
    kvmppc_deliver_irq_passthru(vcpu, xirr, irq_map, pimap, again)
}

#[cfg(not(CONFIG_KVM_XICS))]
unsafe fn kvmppc_check_passthru(_xisr: u32, _xirr: __be32, _again: *mut bool) -> c_int { 1 }

unsafe fn kvmppc_read_one_intr(again: *mut bool) -> c_long;

pub unsafe extern "C" fn kvmppc_read_intr() -> c_long {
    let mut ret = 0;
    let mut rc;
    let mut again;
    if xive_enabled() { return 1; }
    loop {
        again = false;
        rc = kvmppc_read_one_intr(&mut again);
        if rc != 0 && (ret == 0 || rc > ret) { ret = rc; }
        if !again { break; }
    }
    ret
}

unsafe fn kvmppc_read_one_intr(again: *mut bool) -> c_long {
    let mut xics_phys: *mut core::ffi::c_void;
    let mut h_xirr: u32;
    let mut xirr: __be32 = 0;
    let mut xisr: u32;
    let mut host_ipi: u8;
    let mut rc: i64;
    if xive_enabled() { return 1; }
    host_ipi = READ_ONCE((*local_paca).kvm_hstate.host_ipi);
    if host_ipi != 0 { return 1; }
    xics_phys = (*local_paca).kvm_hstate.xics_phys;
    rc = 0;
    if xics_phys.is_null() { rc = opal_int_get_xirr(&mut xirr, false); }
    else { xirr = __raw_rm_readl(xics_phys.add(XICS_XIRR as usize)); }
    if rc < 0 { return 1; }
    h_xirr = be32_to_cpu(xirr);
    (*local_paca).kvm_hstate.saved_xirr = h_xirr;
    xisr = h_xirr & 0xffffff;
    smp_mb();
    if xisr == 0 { return 0; }
    if xisr == XICS_IPI {
        rc = 0;
        if !xics_phys.is_null() {
            __raw_rm_writeb(0xff, xics_phys.add(XICS_MFRR as usize));
            __raw_rm_writel(xirr, xics_phys.add(XICS_XIRR as usize));
        } else {
            opal_int_set_mfrr(hard_smp_processor_id(), 0xff);
            rc = opal_int_eoi(h_xirr);
        }
        *again = rc > 0;
        smp_mb();
        host_ipi = READ_ONCE((*local_paca).kvm_hstate.host_ipi);
        if host_ipi != 0 {
            if !xics_phys.is_null() { __raw_rm_writeb(IPI_PRIORITY, xics_phys.add(XICS_MFRR as usize)); }
            else { opal_int_set_mfrr(hard_smp_processor_id(), IPI_PRIORITY); }
            smp_mb();
            return 1;
        }
        (*local_paca).kvm_hstate.saved_xirr = 0;
        return -1;
    }
    kvmppc_check_passthru(xisr, xirr, again) as c_long
}

unsafe fn kvmppc_end_cede(vcpu: *mut kvm_vcpu) {
    (*vcpu).arch.ceded = 0;
    if (*vcpu).arch.timer_running {
        hrtimer_try_to_cancel(&mut (*vcpu).arch.dec_timer);
        (*vcpu).arch.timer_running = 0;
    }
}

pub unsafe extern "C" fn kvmppc_set_msr_hv(vcpu: *mut kvm_vcpu, mut msr: u64) {
    msr = (msr | MSR_ME) & !MSR_HV;
    if msr & MSR_TS_MASK == MSR_TS_MASK { msr &= !MSR_TS_MASK; }
    __kvmppc_set_msr_hv(vcpu, msr);
    kvmppc_end_cede(vcpu);
}

unsafe fn inject_interrupt(vcpu: *mut kvm_vcpu, vec: c_int, srr1_flags: u64) {
    let msr = kvmppc_get_msr(vcpu);
    let pc = kvmppc_get_pc(vcpu);
    let mut new_msr = (*vcpu).arch.intr_msr;
    let mut new_pc = vec as u64;
    if MSR_TM_TRANSACTIONAL(msr) { new_msr |= MSR_TS_S; } else { new_msr |= msr & MSR_TS_MASK; }
    if vec != BOOK3S_INTERRUPT_SYSTEM_RESET && vec != BOOK3S_INTERRUPT_MACHINE_CHECK
        && (*(*vcpu).arch.vcore).lpcr & LPCR_AIL == LPCR_AIL_3
        && msr & (MSR_IR | MSR_DR) == (MSR_IR | MSR_DR)
    { new_msr |= MSR_IR | MSR_DR; new_pc = new_pc.wrapping_add(0xC000000000004000); }
    kvmppc_set_srr0(vcpu, pc);
    kvmppc_set_srr1(vcpu, (msr & SRR1_MSR_BITS) | srr1_flags);
    kvmppc_set_pc(vcpu, new_pc);
    __kvmppc_set_msr_hv(vcpu, new_msr);
}

pub unsafe extern "C" fn kvmppc_inject_interrupt_hv(vcpu: *mut kvm_vcpu, vec: c_int, srr1_flags: u64) {
    inject_interrupt(vcpu, vec, srr1_flags);
    kvmppc_end_cede(vcpu);
}

pub unsafe extern "C" fn kvmppc_guest_entry_inject_int(vcpu: *mut kvm_vcpu) {
    let ext = ((*vcpu).arch.pending_exceptions >> BOOK3S_IRQPRIO_EXTERNAL) & 1;
    let mut lpcr = mfspr(SPRN_LPCR);
    WARN_ON_ONCE(cpu_has_feature(CPU_FTR_ARCH_300));
    lpcr |= ext << LPCR_MER_SH;
    mtspr(SPRN_LPCR, lpcr); isync();
    if (*vcpu).arch.shregs.msr & MSR_EE != 0 {
        if ext != 0 { inject_interrupt(vcpu, BOOK3S_INTERRUPT_EXTERNAL, 0); }
        else { let mut dec = mfspr(SPRN_DEC); if lpcr & LPCR_LD == 0 { dec = dec as i32 as u64; } if (dec as i64) < 0 { inject_interrupt(vcpu, BOOK3S_INTERRUPT_DECREMENTER, 0); } }
    }
    if (*vcpu).arch.doorbell_request != 0 { mtspr(SPRN_DPDES, 1); (*(*vcpu).arch.vcore).dpdes = 1; smp_wmb(); (*vcpu).arch.doorbell_request = 0; }
}

unsafe fn flush_guest_tlb(kvm: *mut kvm) {
    let mut rb = PPC_BIT(52);
    for _set in 0..(*kvm).arch.tlb_sets {
        core::arch::asm!(PPC_TLBIEL, in("r") rb, options(nostack));
        rb += PPC_BIT(51);
    }
    core::arch::asm!("ptesync", options(nostack));
}

pub unsafe extern "C" fn kvmppc_check_need_tlb_flush(kvm: *mut kvm, pcpu: c_int) {
    if cpumask_test_cpu(pcpu, &mut (*kvm).arch.need_tlb_flush) {
        flush_guest_tlb(kvm);
        cpumask_clear_cpu(pcpu, &mut (*kvm).arch.need_tlb_flush);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
