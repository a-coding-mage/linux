// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2009 Sunplus Core Technology Co., Ltd.
 *  Lennox Wu <lennox.wu@sunplusct.com>
 *  Chen Liqin <liqin.chen@sunplusct.com>
 * Copyright (C) 2012 Regents of the University of California
 */

// Linux and architecture dependencies are supplied by the surrounding crate.

unsafe fn show_pte(addr: c_ulong) {
    let mut pgdp: *mut pgd_t;
    let mut pgd: pgd_t;
    let mut p4dp: *mut p4d_t;
    let mut p4d: p4d_t;
    let mut pudp: *mut pud_t;
    let mut pud: pud_t;
    let mut pmdp: *mut pmd_t;
    let mut pmd: pmd_t;
    let mut ptep: *mut pte_t;
    let mut pte: pte_t;
    let mut mm: *mut mm_struct = (*current).mm;

    if mm.is_null() { mm = &raw mut init_mm; }
    pr_alert!("Current %s pgtable: %luK pagesize, %d-bit VAs, pgdp=0x%016llx\n",
        (*current).comm, PAGE_SIZE / SZ_1K, VA_BITS,
        if mm == &raw mut init_mm { __pa_symbol((*mm).pgd) as u64 } else { virt_to_phys((*mm).pgd) });

    pgdp = pgd_offset(mm, addr); pgd = pgdp_get(pgdp);
    pr_alert!("[%016lx] pgd=%016lx", addr, pgd_val(pgd));
    if pgd_none(pgd) || pgd_bad(pgd) || pgd_leaf(pgd) { goto_out!(); }
    p4dp = p4d_offset(pgdp, addr); p4d = p4dp_get(p4dp);
    pr_cont!(', p4d=%016lx', p4d_val(p4d));
    if p4d_none(p4d) || p4d_bad(p4d) || p4d_leaf(p4d) { goto_out!(); }
    pudp = pud_offset(p4dp, addr); pud = pudp_get(pudp);
    pr_cont!(', pud=%016lx', pud_val(pud));
    if pud_none(pud) || pud_bad(pud) || pud_leaf(pud) { goto_out!(); }
    pmdp = pmd_offset(pudp, addr); pmd = pmdp_get(pmdp);
    pr_cont!(', pmd=%016lx', pmd_val(pmd));
    if pmd_none(pmd) || pmd_bad(pmd) || pmd_leaf(pmd) { goto_out!(); }
    ptep = pte_offset_map(pmdp, addr);
    if !ptep.is_null() { pte = ptep_get(ptep); pr_cont!(', pte=%016lx', pte_val(pte)); pte_unmap(ptep); }
    goto_out!();
}

unsafe fn die_kernel_fault(msg: *const c_char, addr: c_ulong, regs: *mut pt_regs) {
    bust_spinlocks(1);
    pr_alert!("Unable to handle kernel %s at virtual address " REG_FMT "\n", msg, addr);
    bust_spinlocks(0); show_pte(addr); die(regs, c"Oops".as_ptr()); make_task_dead(SIGKILL);
}

unsafe fn no_context(regs: *mut pt_regs, addr: c_ulong) {
    if fixup_exception(regs) { return; }
    if addr < PAGE_SIZE { die_kernel_fault(c"NULL pointer dereference".as_ptr(), addr, regs); }
    else { if kfence_handle_page_fault(addr, (*regs).cause == EXC_STORE_PAGE_FAULT, regs) { return; }
        die_kernel_fault(c"paging request".as_ptr(), addr, regs); }
}

unsafe fn mm_fault_error(regs: *mut pt_regs, addr: c_ulong, fault: vm_fault_t) {
    if !user_mode(regs) { no_context(regs, addr); return; }
    if fault & VM_FAULT_OOM != 0 { pagefault_out_of_memory(); }
    else if fault & (VM_FAULT_SIGBUS | VM_FAULT_HWPOISON | VM_FAULT_HWPOISON_LARGE) != 0 { do_trap(regs, SIGBUS, BUS_ADRERR, addr); }
    else if fault & VM_FAULT_SIGSEGV != 0 { do_trap(regs, SIGSEGV, SEGV_MAPERR, addr); }
    else { BUG!(); }
}

unsafe fn bad_area_nosemaphore(regs: *mut pt_regs, code: c_int, addr: c_ulong) {
    if user_mode(regs) { do_trap(regs, SIGSEGV, code, addr); return; }
    no_context(regs, addr);
}

unsafe fn bad_area(regs: *mut pt_regs, mm: *mut mm_struct, code: c_int, addr: c_ulong) {
    mmap_read_unlock(mm); bad_area_nosemaphore(regs, code, addr);
}

unsafe fn vmalloc_fault(regs: *mut pt_regs, code: c_int, addr: c_ulong) {
    if user_mode(regs) { do_trap(regs, SIGSEGV, code, addr); return; }
    let index = pgd_index(addr); let pfn = csr_read(CSR_SATP) & SATP_PPN;
    let pgd = (pfn_to_virt(pfn) as *mut pgd_t).add(index); let pgd_k = (*(&raw mut init_mm)).pgd.add(index);
    if !pgd_present(pgdp_get(pgd_k)) { no_context(regs, addr); return; } set_pgd(pgd, pgdp_get(pgd_k));
    let p4d_k = p4d_offset(pgd_k, addr); if !p4d_present(p4dp_get(p4d_k)) { no_context(regs, addr); return; }
    let pud_k = pud_offset(p4d_k, addr); if !pud_present(pudp_get(pud_k)) { no_context(regs, addr); return; }
    if pud_leaf(pudp_get(pud_k)) { local_flush_tlb_page(addr); return; }
    let pmd_k = pmd_offset(pud_k, addr); if !pmd_present(pmdp_get(pmd_k)) { no_context(regs, addr); return; }
    if pmd_leaf(pmdp_get(pmd_k)) { local_flush_tlb_page(addr); return; }
    let pte_k = pte_offset_kernel(pmd_k, addr); if !pte_present(ptep_get(pte_k)) { no_context(regs, addr); return; }
    local_flush_tlb_page(addr);
}

unsafe fn access_error(cause: c_ulong, vma: *mut vm_area_struct) -> bool {
    match cause {
        EXC_INST_PAGE_FAULT => (*vma).vm_flags & VM_EXEC == 0,
        EXC_LOAD_PAGE_FAULT => (*vma).vm_flags & (VM_READ | VM_WRITE) == 0,
        EXC_STORE_PAGE_FAULT => (*vma).vm_flags & VM_WRITE == 0,
        _ => { panic!("%s: unhandled cause %lu", c"access_error".as_ptr(), cause); false }
    }
}

pub unsafe fn handle_page_fault(regs: *mut pt_regs) {
    let mut flags = FAULT_FLAG_DEFAULT; let mut code = SEGV_MAPERR; let cause = (*regs).cause; let addr = (*regs).badaddr;
    let tsk = current; let mm = (*tsk).mm;
    if kprobe_page_fault(regs, cause) { return; }
    if user_mode(regs) { trace_page_fault_user(addr, regs, cause); } else { trace_page_fault_kernel(addr, regs, cause); }
    if (!IS_ENABLED(CONFIG_MMU) || !IS_ENABLED(CONFIG_64BIT)) && unlikely(addr >= VMALLOC_START && addr < VMALLOC_END) { vmalloc_fault(regs, code, addr); return; }
    if !regs_irqs_disabled(regs) { local_irq_enable(); }
    if unlikely(faulthandler_disabled() || mm.is_null()) { (*tsk).thread.bad_cause = cause; no_context(regs, addr); return; }
    if user_mode(regs) { flags |= FAULT_FLAG_USER; }
    if !user_mode(regs) && addr < TASK_SIZE && unlikely((*regs).status & SR_SUM == 0) { if fixup_exception(regs) { return; } die_kernel_fault(c"access to user memory without uaccess routines".as_ptr(), addr, regs); }
    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, addr);
    if cause == EXC_STORE_PAGE_FAULT { flags |= FAULT_FLAG_WRITE; } else if cause == EXC_INST_PAGE_FAULT { flags |= FAULT_FLAG_INSTRUCTION; }
    if flags & FAULT_FLAG_USER == 0 { goto_lock_mmap!(); }
    let mut vma = lock_vma_under_rcu(mm, addr); if vma.is_null() { goto_lock_mmap!(); }
    if unlikely(access_error(cause, vma)) { vma_end_read(vma); count_vm_vma_lock_event(VMA_LOCK_SUCCESS); (*tsk).thread.bad_cause = cause; bad_area_nosemaphore(regs, SEGV_ACCERR, addr); return; }
    let mut fault = handle_mm_fault(vma, addr, flags | FAULT_FLAG_VMA_LOCK, regs); if fault & (VM_FAULT_RETRY | VM_FAULT_COMPLETED) == 0 { vma_end_read(vma); }
    if fault & VM_FAULT_RETRY == 0 { count_vm_vma_lock_event(VMA_LOCK_SUCCESS); goto_done!(); }
    count_vm_vma_lock_event(VMA_LOCK_RETRY); if fault & VM_FAULT_MAJOR != 0 { flags |= FAULT_FLAG_TRIED; }
    if fault_signal_pending(fault, regs) { if !user_mode(regs) { no_context(regs, addr); } return; }
    vma = lock_mm_and_find_vma(mm, addr, regs); if vma.is_null() { (*tsk).thread.bad_cause = cause; bad_area_nosemaphore(regs, code, addr); return; }
    code = SEGV_ACCERR; if unlikely(access_error(cause, vma)) { (*tsk).thread.bad_cause = cause; bad_area(regs, mm, code, addr); return; }
    fault = handle_mm_fault(vma, addr, flags, regs);
    if fault_signal_pending(fault, regs) { if !user_mode(regs) { no_context(regs, addr); } return; }
    if fault & VM_FAULT_COMPLETED != 0 { return; }
    if unlikely(fault & VM_FAULT_RETRY != 0) { flags |= FAULT_FLAG_TRIED; goto_retry!(); }
    mmap_read_unlock(mm);
    if unlikely(fault & VM_FAULT_ERROR != 0) { (*tsk).thread.bad_cause = cause; mm_fault_error(regs, addr, fault); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
