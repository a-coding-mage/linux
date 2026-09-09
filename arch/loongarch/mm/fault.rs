// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 *
 * Derived from MIPS:
 * Copyright (C) 1995 - 2000 by Ralf Baechle
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

pub static mut show_unhandled_signals: ::core::ffi::c_int = 1;

unsafe fn spurious_fault(write: libc::c_ulong, address: libc::c_ulong) -> libc::c_int {
    let mut pgd: *mut pgd_t;
    let mut p4d: *mut p4d_t;
    let mut pud: *mut pud_t;
    let mut pmd: *mut pmd_t;
    let mut pte: *mut pte_t;

    if address & __UA_LIMIT == 0 { return 0; }
    pgd = pgd_offset_k(address);
    if !pgd_present(pgdp_get(pgd)) { return 0; }
    p4d = p4d_offset(pgd, address);
    if !p4d_present(p4dp_get(p4d)) { return 0; }
    pud = pud_offset(p4d, address);
    if !pud_present(pudp_get(pud)) { return 0; }
    pmd = pmd_offset(pud, address);
    if !pmd_present(pmdp_get(pmd)) { return 0; }

    if pmd_leaf(*pmd) {
        if write != 0 { pmd_write(pmdp_get(pmd)) } else { 1 }
    } else {
        pte = pte_offset_kernel(pmd, address);
        if !pte_present(ptep_get(pte)) { return 0; }
        if write != 0 { pte_write(ptep_get(pte)) } else { 1 }
    }
}

unsafe fn no_context(regs: *mut pt_regs, write: libc::c_ulong, address: libc::c_ulong) {
    let field: libc::c_int = (core::mem::size_of::<libc::c_ulong>() * 2) as libc::c_int;
    if spurious_fault(write, address) != 0 { return; }
    if fixup_exception(regs) != 0 { return; }
    if kfence_handle_page_fault(address, write, regs) != 0 { return; }
    bust_spinlocks(1);
    pr_alert!("CPU %d Unable to handle kernel paging request at virtual address %0*lx, era == %0*lx, ra == %0*lx\n", raw_smp_processor_id(), field, address, field, (*regs).csr_era, field, (*regs).regs[1]);
    die(c"Oops", regs);
}

unsafe fn do_out_of_memory(regs: *mut pt_regs, write: libc::c_ulong, address: libc::c_ulong) {
    if user_mode(regs) == 0 { no_context(regs, write, address); return; }
    pagefault_out_of_memory();
}

unsafe fn do_sigbus(regs: *mut pt_regs, write: libc::c_ulong, address: libc::c_ulong, si_code: libc::c_int) {
    if user_mode(regs) == 0 { no_context(regs, write, address); return; }
    (*current).thread.csr_badvaddr = address;
    (*current).thread.trap_nr = read_csr_excode();
    force_sig_fault(SIGBUS, BUS_ADRERR, address as *mut core::ffi::c_void);
}

unsafe fn do_sigsegv(regs: *mut pt_regs, write: libc::c_ulong, address: libc::c_ulong, si_code: libc::c_int) {
    let field: libc::c_int = (core::mem::size_of::<libc::c_ulong>() * 2) as libc::c_int;
    static mut ratelimit_state: ratelimit_state = DEFINE_RATELIMIT_STATE!("ratelimit_state", 5 * HZ, 10);
    if user_mode(regs) == 0 { no_context(regs, write, address); return; }
    (*current).thread.csr_badvaddr = address;
    (*current).thread.error_code = if write == 0 { 1 } else { 2 };
    (*current).thread.trap_nr = read_csr_excode();
    if show_unhandled_signals != 0 && unhandled_signal(current, SIGSEGV) != 0 && __ratelimit(&mut ratelimit_state) != 0 {
        pr_info!("do_page_fault(): sending SIGSEGV to %s for invalid %s %0*lx\n", (*current).comm, if write != 0 { "write access to" } else { "read access from" }, field, address);
        pr_info!("era = %0*lx in", field, (*regs).csr_era as libc::c_ulong);
        print_vma_addr(KERN_CONT " ", (*regs).csr_era);
        pr_cont!("\n");
        pr_info!("ra  = %0*lx in", field, (*regs).regs[1] as libc::c_ulong);
        print_vma_addr(KERN_CONT " ", (*regs).regs[1]);
        pr_cont!("\n");
    }
    force_sig_fault(SIGSEGV, si_code, address as *mut core::ffi::c_void);
}

unsafe fn __do_page_fault(regs: *mut pt_regs, write: libc::c_ulong, address: libc::c_ulong) {
    let mut si_code = SEGV_MAPERR;
    let mut flags = FAULT_FLAG_DEFAULT;
    let tsk = current;
    let mm = (*tsk).mm;
    let mut vma: *mut vm_area_struct = core::ptr::null_mut();
    let mut fault: vm_fault_t;

    if kprobe_page_fault(regs, (*tsk).thread.trap_nr) != 0 { return; }
    if address & __UA_LIMIT != 0 { if user_mode(regs) == 0 { no_context(regs, write, address); } else { do_sigsegv(regs, write, address, si_code); } return; }
    if faulthandler_disabled() != 0 || mm.is_null() { do_sigsegv(regs, write, address, si_code); return; }
    if user_mode(regs) != 0 { flags |= FAULT_FLAG_USER; }
    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, address);
    if flags & FAULT_FLAG_USER == 0 { goto_lock_mmap!(lock_mmap); }
    vma = lock_vma_under_rcu(mm, address);
    if vma.is_null() { goto_lock_mmap!(lock_mmap); }
    if write != 0 { flags |= FAULT_FLAG_WRITE; if (*vma).vm_flags & VM_WRITE == 0 { vma_end_read(vma); si_code = SEGV_ACCERR; count_vm_vma_lock_event(VMA_LOCK_SUCCESS); goto_bad_area_nosemaphore!(); } }
    else if ((*vma).vm_flags & VM_EXEC == 0) && address == exception_era(regs) { vma_end_read(vma); si_code = SEGV_ACCERR; count_vm_vma_lock_event(VMA_LOCK_SUCCESS); goto_bad_area_nosemaphore!(); }
    else if (*vma).vm_flags & (VM_READ | VM_WRITE) == 0 && address != exception_era(regs) { vma_end_read(vma); si_code = SEGV_ACCERR; count_vm_vma_lock_event(VMA_LOCK_SUCCESS); goto_bad_area_nosemaphore!(); }
    fault = handle_mm_fault(vma, address, flags | FAULT_FLAG_VMA_LOCK, regs);
    if fault & (VM_FAULT_RETRY | VM_FAULT_COMPLETED) == 0 { vma_end_read(vma); }
    if fault & VM_FAULT_RETRY == 0 { count_vm_vma_lock_event(VMA_LOCK_SUCCESS); goto_done!(fault); }
    count_vm_vma_lock_event(VMA_LOCK_RETRY);
    if fault & VM_FAULT_MAJOR != 0 { flags |= FAULT_FLAG_TRIED; }
    if fault_signal_pending(fault, regs) != 0 { if user_mode(regs) == 0 { no_context(regs, write, address); } return; }

    'lock_mmap: loop {
        vma = lock_mm_and_find_vma(mm, address, regs);
        if vma.is_null() { do_sigsegv(regs, write, address, si_code); return; }
        break;
    }
    si_code = SEGV_ACCERR;
    if write != 0 { flags |= FAULT_FLAG_WRITE; if (*vma).vm_flags & VM_WRITE == 0 { mmap_read_unlock(mm); do_sigsegv(regs, write, address, si_code); return; } }
    else if (*vma).vm_flags & VM_EXEC == 0 && address == exception_era(regs) || (*vma).vm_flags & (VM_READ | VM_WRITE) == 0 && address != exception_era(regs) { mmap_read_unlock(mm); do_sigsegv(regs, write, address, si_code); return; }
    fault = handle_mm_fault(vma, address, flags, regs);
    if fault_signal_pending(fault, regs) != 0 { if user_mode(regs) == 0 { no_context(regs, write, address); } return; }
    if fault & VM_FAULT_COMPLETED != 0 { return; }
    if fault & VM_FAULT_RETRY != 0 { flags |= FAULT_FLAG_TRIED; goto_retry!(fault); }
    mmap_read_unlock(mm);
    if fault & VM_FAULT_ERROR != 0 { if fault & VM_FAULT_OOM != 0 { do_out_of_memory(regs, write, address); return; } else if fault & VM_FAULT_SIGSEGV != 0 { do_sigsegv(regs, write, address, si_code); return; } else if fault & (VM_FAULT_SIGBUS | VM_FAULT_HWPOISON | VM_FAULT_HWPOISON_LARGE) != 0 { do_sigbus(regs, write, address, si_code); return; } BUG!(); }
}

pub unsafe fn do_page_fault(regs: *mut pt_regs, write: libc::c_ulong, address: libc::c_ulong) {
    let state = irqentry_enter(regs);
    if (*regs).csr_prmd & CSR_PRMD_PIE != 0 { local_irq_enable(); }
    __do_page_fault(regs, write, address);
    local_irq_disable();
    irqentry_exit(regs, state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
