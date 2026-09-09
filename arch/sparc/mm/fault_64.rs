// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sparc64/mm/fault.c: Page fault handlers for the 64-bit Sparc.
 *
 * Copyright (C) 1996, 2008 David S. Miller (davem@davemloft.net)
 * Copyright (C) 1997, 1999 Jakub Jelinek (jj@ultra.linux.cz)
 */

// C header dependencies are supplied by the surrounding kernel translation.

pub static mut show_unhandled_signals: i32 = 1;

unsafe fn unhandled_fault(address: c_ulong, tsk: *mut task_struct, regs: *mut pt_regs) {
    if address < PAGE_SIZE {
        printk(KERN_ALERT "Unable to handle kernel NULL pointer dereference\n");
    } else {
        printk(KERN_ALERT "Unable to handle kernel paging request at virtual address %016lx\n", address);
    }
    printk(KERN_ALERT "tsk->{mm,active_mm}->context = %016lx\n",
           if !(*tsk).mm.is_null() { CTX_HWBITS((*(*tsk).mm).context) } else { CTX_HWBITS((*(*tsk).active_mm).context) });
    printk(KERN_ALERT "tsk->{mm,active_mm}->pgd = %016lx\n",
           if !(*tsk).mm.is_null() { (*(*tsk).mm).pgd as c_ulong } else { (*(*tsk).active_mm).pgd as c_ulong });
    die_if_kernel("Oops", regs);
}

unsafe fn bad_kernel_pc(regs: *mut pt_regs, vaddr: c_ulong) {
    printk(KERN_CRIT "OOPS: Bogus kernel PC [%016lx] in fault handler\n", (*regs).tpc);
    printk(KERN_CRIT "OOPS: RPC [%016lx]\n", (*regs).u_regs[15]);
    printk("OOPS: RPC <%pS>\n", (*regs).u_regs[15] as *mut c_void);
    printk(KERN_CRIT "OOPS: Fault was to vaddr[%lx]\n", vaddr);
    dump_stack();
    unhandled_fault((*regs).tpc, current, regs);
}

/* We now make sure that mmap_lock is held in all paths that call this. */
unsafe fn get_user_insn(tpc: c_ulong) -> u32 {
    let pgdp = pgd_offset((*current).mm, tpc);
    let mut insn: u32 = 0;
    if pgd_none(*pgdp) || unlikely(pgd_bad(*pgdp)) { return insn; }
    let p4dp = p4d_offset(pgdp, tpc);
    if p4d_none(*p4dp) || unlikely(p4d_bad(*p4dp)) { return insn; }
    let pudp = pud_offset(p4dp, tpc);
    if pud_none(*pudp) || unlikely(pud_bad(*pudp)) { return insn; }
    local_irq_disable();
    let pmdp = pmd_offset(pudp, tpc);
    'again: loop {
        if pmd_none(*pmdp) || unlikely(pmd_bad(*pmdp)) { break; }
        #[cfg(any(CONFIG_HUGETLB_PAGE, CONFIG_TRANSPARENT_HUGEPAGE))]
        if is_hugetlb_pmd(*pmdp) {
            let pa = (pmd_pfn(*pmdp) << PAGE_SHIFT) + (tpc & !HPAGE_MASK);
            core::arch::asm!("lduwa [{1}] {2}, {0}", out(reg) insn, in(reg) pa, const ASI_PHYS_USE_EC);
        } else
        {
            let ptep = pte_offset_map(pmdp, tpc);
            if ptep.is_null() { continue 'again; }
            let pte = *ptep;
            if pte_present(pte) {
                let pa = (pte_pfn(pte) << PAGE_SHIFT) + (tpc & !PAGE_MASK);
                core::arch::asm!("lduwa [{1}] {2}, {0}", out(reg) insn, in(reg) pa, const ASI_PHYS_USE_EC);
            }
            pte_unmap(ptep);
        }
        break;
    }
    local_irq_enable();
    insn
}

unsafe fn show_signal_msg(regs: *mut pt_regs, sig: i32, code: i32, address: c_ulong, tsk: *mut task_struct) {
    if !unhandled_signal(tsk, sig) || !printk_ratelimit() { return; }
    printk("%s%s[%d]: segfault at %lx ip %px (rpc %px) sp %px error %x",
           if task_pid_nr(tsk) > 1 { KERN_INFO } else { KERN_EMERG }, (*tsk).comm,
           task_pid_nr(tsk), address, (*regs).tpc as *mut c_void,
           (*regs).u_regs[UREG_I7] as *mut c_void, (*regs).u_regs[UREG_FP] as *mut c_void, code);
    print_vma_addr(KERN_CONT " in ", (*regs).tpc);
    printk(KERN_CONT "\n");
}

unsafe fn do_fault_siginfo(code: i32, sig: i32, regs: *mut pt_regs, fault_addr: c_ulong, insn: u32, fault_code: i32) {
    let addr = if fault_code & FAULT_CODE_ITLB != 0 { (*regs).tpc } else if insn != 0 { compute_effective_address(regs, insn, 0) } else { fault_addr };
    if unlikely(show_unhandled_signals != 0) { show_signal_msg(regs, sig, code, addr, current); }
    force_sig_fault(sig, code, addr as *mut c_void);
}

unsafe fn get_fault_insn(regs: *mut pt_regs, mut insn: u32) -> u32 {
    if insn == 0 {
        if (*regs).tpc == 0 || (*regs).tpc & 3 != 0 { return 0; }
        insn = if (*regs).tstate & TSTATE_PRIV != 0 { *((*regs).tpc as *const u32) } else { get_user_insn((*regs).tpc) };
    }
    insn
}

unsafe fn do_kernel_fault(regs: *mut pt_regs, si_code: i32, fault_code: i32, insn: u32, address: c_ulong) {
    let mut asi: u8 = ASI_P;
    if insn == 0 && (*regs).tstate & TSTATE_PRIV != 0 { unhandled_fault(address, current, regs); return; }
    if fault_code & (FAULT_CODE_WRITE | FAULT_CODE_ITLB) == 0 && insn & 0xc0800000 == 0xc0800000 {
        asi = if insn & 0x2000 != 0 { ((*regs).tstate >> 24) as u8 } else { (insn >> 5) as u8 };
        if asi & 0xf2 == 0x82 {
            if insn & 0x1000000 != 0 { handle_ldf_stq(insn, regs); } else { handle_ld_nf(insn, regs); }
            return;
        }
    }
    if (*regs).tstate & TSTATE_PRIV != 0 {
        let entry = search_exception_tables((*regs).tpc);
        if !entry.is_null() { (*regs).tpc = (*entry).fixup; (*regs).tnpc = (*regs).tpc + 4; return; }
    } else {
        do_fault_siginfo(si_code, SIGSEGV, regs, address, insn, fault_code);
        return;
    }
    unhandled_fault(address, current, regs);
}

unsafe fn bogus_32bit_fault_tpc(regs: *mut pt_regs) {
    static mut times: i32 = 0;
    if times < 10 { times += 1; printk(KERN_ERR "FAULT[%s:%d]: 32-bit process reports 64-bit TPC [%lx]\n", (*current).comm, (*current).pid, (*regs).tpc); }
    show_regs(regs);
}

pub unsafe fn do_sparc64_fault(regs: *mut pt_regs) {
    let prev_state = exception_enter();
    let mm = (*current).mm;
    let mut vma: *mut vm_area_struct;
    let mut insn: u32 = 0;
    let mut si_code: i32;
    let mut fault_code: i32;
    let mut fault: vm_fault_t;
    let mut address: c_ulong;
    let mut mm_rss: c_ulong;
    let mut flags: u32 = FAULT_FLAG_DEFAULT;
    fault_code = get_thread_fault_code();
    if kprobe_page_fault(regs, 0) != 0 { exception_exit(prev_state); return; }
    si_code = SEGV_MAPERR;
    address = current_thread_info().fault_address;
    if fault_code & FAULT_CODE_ITLB != 0 && fault_code & FAULT_CODE_DTLB != 0 { BUG(); }
    if test_thread_flag(TIF_32BIT) != 0 {
        if (*regs).tstate & TSTATE_PRIV == 0 && unlikely((*regs).tpc >> 32 != 0) { bogus_32bit_fault_tpc(regs); goto intr_or_no_mm; }
        if unlikely(address >> 32 != 0) { goto intr_or_no_mm; }
    }
    if (*regs).tstate & TSTATE_PRIV != 0 {
        let tpc = (*regs).tpc;
        if !((tpc >= KERNBASE && tpc < __init_end as c_ulong) || (tpc >= MODULES_VADDR && tpc < MODULES_END)) { bad_kernel_pc(regs, address); exception_exit(prev_state); return; }
    } else { flags |= FAULT_FLAG_USER; }
    if faulthandler_disabled() || mm.is_null() { goto intr_or_no_mm; }
    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, address);
    if !mmap_read_trylock(mm) {
        if (*regs).tstate & TSTATE_PRIV != 0 && search_exception_tables((*regs).tpc).is_null() { insn = get_fault_insn(regs, insn); goto handle_kernel_fault; }
        'retry: loop { mmap_read_lock(mm); break 'retry; }
    }
    if fault_code & FAULT_CODE_BAD_RA != 0 { goto do_sigbus; }
    vma = find_vma(mm, address);
    if vma.is_null() { goto bad_area; }
    if (fault_code & (FAULT_CODE_DTLB | FAULT_CODE_WRITE | FAULT_CODE_WINFIXUP)) == FAULT_CODE_DTLB && (*vma).vm_flags & VM_WRITE != 0 {
        insn = get_fault_insn(regs, 0); if insn == 0 { goto continue_fault; }
        if insn & 0xc0200000 == 0xc0200000 && insn & 0x01780000 != 0x01680000 { fault_code |= FAULT_CODE_WRITE; }
    }
    'continue_fault: {
        if (*vma).vm_start > address {
            if (*vma).vm_flags & VM_GROWSDOWN == 0 { goto bad_area; }
            if fault_code & FAULT_CODE_WRITE == 0 {
                insn = get_fault_insn(regs, insn);
                if insn & 0xc0800000 == 0xc0800000 { let asi = if insn & 0x2000 != 0 { ((*regs).tstate >> 24) as u8 } else { (insn >> 5) as u8 }; if asi & 0xf2 == 0x82 { goto bad_area; } }
            }
            vma = expand_stack(mm, address); if vma.is_null() { goto bad_area_nosemaphore; }
        }
    }
    si_code = SEGV_ACCERR;
    if fault_code & FAULT_CODE_ITLB != 0 && (*vma).vm_flags & VM_EXEC == 0 { WARN(address != (*regs).tpc, "address (%lx) != regs->tpc (%lx)\n", address, (*regs).tpc); WARN_ON((*regs).tstate & TSTATE_PRIV); goto bad_area; }
    if fault_code & FAULT_CODE_WRITE != 0 { if (*vma).vm_flags & VM_WRITE == 0 { goto bad_area; } if tlb_type == spitfire && (*vma).vm_flags & VM_EXEC != 0 && !(*vma).vm_file.is_null() { set_thread_fault_code(fault_code | FAULT_CODE_BLKCOMMIT); } flags |= FAULT_FLAG_WRITE; } else if (*vma).vm_flags & (VM_READ | VM_EXEC) == 0 { goto bad_area; }
    fault = handle_mm_fault(vma, address, flags, regs);
    if fault_signal_pending(fault, regs) { if (*regs).tstate & TSTATE_PRIV != 0 { insn = get_fault_insn(regs, insn); goto handle_kernel_fault; } exception_exit(prev_state); return; }
    if fault & VM_FAULT_COMPLETED != 0 { goto lock_released; }
    if unlikely(fault & VM_FAULT_ERROR != 0) { if fault & VM_FAULT_OOM != 0 { goto out_of_memory; } else if fault & VM_FAULT_SIGSEGV != 0 { goto bad_area; } else if fault & VM_FAULT_SIGBUS != 0 { goto do_sigbus; } BUG(); }
    if fault & VM_FAULT_RETRY != 0 { flags |= FAULT_FLAG_TRIED; goto retry; }
    mmap_read_unlock(mm);
    'lock_released: {
        mm_rss = get_mm_rss(mm);
        #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)] { mm_rss -= (*mm).context.thp_pte_count * (HPAGE_SIZE / PAGE_SIZE); }
        if unlikely(mm_rss > (*mm).context.tsb_block[MM_TSB_BASE].tsb_rss_limit) { tsb_grow(mm, MM_TSB_BASE, mm_rss); }
        #[cfg(any(CONFIG_HUGETLB_PAGE, CONFIG_TRANSPARENT_HUGEPAGE))] { mm_rss = (*mm).context.hugetlb_pte_count + (*mm).context.thp_pte_count; mm_rss *= REAL_HPAGE_PER_HPAGE; if unlikely(mm_rss > (*mm).context.tsb_block[MM_TSB_HUGE].tsb_rss_limit) { if !(*mm).context.tsb_block[MM_TSB_HUGE].tsb.is_null() { tsb_grow(mm, MM_TSB_HUGE, mm_rss); } else { hugetlb_setup(regs); } } }
    }
    exception_exit(prev_state); return;

    bad_area: mmap_read_unlock(mm);
    bad_area_nosemaphore: insn = get_fault_insn(regs, insn);
    handle_kernel_fault: do_kernel_fault(regs, si_code, fault_code, insn, address); exception_exit(prev_state); return;
    out_of_memory: insn = get_fault_insn(regs, insn); mmap_read_unlock(mm); if (*regs).tstate & TSTATE_PRIV == 0 { pagefault_out_of_memory(); exception_exit(prev_state); return; } goto handle_kernel_fault;
    intr_or_no_mm: insn = get_fault_insn(regs, 0); goto handle_kernel_fault;
    do_sigbus: insn = get_fault_insn(regs, insn); mmap_read_unlock(mm); do_fault_siginfo(BUS_ADRERR, SIGBUS, regs, address, insn, fault_code); if (*regs).tstate & TSTATE_PRIV != 0 { goto handle_kernel_fault; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
