// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mm/fault.c
 *
 *  Copyright (C) 1995  Linus Torvalds
 *  Modifications for ARM processor (c) 1995-2004 Russell King
 */

// CONFIG_MMU

#[inline]
pub unsafe fn copy_from_kernel_nofault_allowed(unsafe_src: *const core::ffi::c_void, size: usize) -> bool {
    let addr = unsafe_src as usize;
    addr >= TASK_SIZE && usize::MAX - addr >= size
}

pub unsafe fn show_pte(lvl: *const core::ffi::c_char, mut mm: *mut mm_struct, addr: usize) {
    if mm.is_null() { mm = &raw mut init_mm; }
    let pgd = pgd_offset(mm, addr);
    printk(b"%s[%08lx] *pgd=%08llx\0".as_ptr() as _, lvl, addr, pgd_val(*pgd) as i64);
    loop {
        let p4d = p4d_offset(pgd, addr);
        if p4d_none(*p4d) { break; }
        if p4d_bad(*p4d) { pr_cont(b"(bad)\0".as_ptr() as _); break; }
        let pud = pud_offset(p4d, addr);
        if PTRS_PER_PUD != 1 { pr_cont(b", *pud=%08llx\0".as_ptr() as _, pud_val(*pud) as i64); }
        if pud_none(*pud) { break; }
        if pud_bad(*pud) { pr_cont(b"(bad)\0".as_ptr() as _); break; }
        let pmd = pmd_offset(pud, addr);
        if PTRS_PER_PMD != 1 { pr_cont(b", *pmd=%08llx\0".as_ptr() as _, pmd_val(*pmd) as i64); }
        if pmd_none(*pmd) { break; }
        if pmd_bad(*pmd) { pr_cont(b"(bad)\0".as_ptr() as _); break; }
        if PageHighMem(pfn_to_page(pmd_val(*pmd) >> PAGE_SHIFT)) { break; }
        let pte = pte_offset_map(pmd, addr);
        if pte.is_null() { break; }
        pr_cont(b", *pte=%08llx\0".as_ptr() as _, pte_val(*pte) as i64);
        // #ifndef CONFIG_ARM_LPAE
        pr_cont(b", *ppte=%08llx\0".as_ptr() as _, pte_val(*pte.add(PTE_HWTABLE_PTRS)) as i64);
        pte_unmap(pte);
    }
    pr_cont(b"\n\0".as_ptr() as _);
}

#[inline] unsafe fn is_write_fault(fsr: u32) -> bool { (fsr & FSR_WRITE) != 0 && (fsr & FSR_CM) == 0 }

unsafe fn die_kernel_fault(msg: *const core::ffi::c_char, mm: *mut mm_struct, addr: usize, fsr: u32, regs: *mut pt_regs) {
    bust_spinlocks(1); pr_alert(b"8<--- cut here ---\n\0".as_ptr() as _);
    pr_alert(b"Unable to handle kernel %s at virtual address %08lx when %s\n\0".as_ptr() as _, msg, addr, if fsr & FSR_LNX_PF != 0 { b"execute\0".as_ptr() as _ } else { str_write_read(fsr & FSR_WRITE) });
    show_pte(KERN_ALERT, mm, addr); die(b"Oops\0".as_ptr() as _, regs, fsr); bust_spinlocks(0); make_task_dead(SIGKILL);
}

unsafe fn __do_kernel_fault(mm: *mut mm_struct, addr: usize, fsr: u32, regs: *mut pt_regs) {
    if fixup_exception(regs) { return; }
    let msg = if addr < PAGE_SIZE { b"NULL pointer dereference\0".as_ptr() as _ } else if is_permission_fault(fsr) && fsr & FSR_LNX_PF != 0 { b"execution of memory\0".as_ptr() as _ } else { if is_translation_fault(fsr) && kfence_handle_page_fault(addr, is_write_fault(fsr), regs) { return; } b"paging request\0".as_ptr() as _ };
    die_kernel_fault(msg, mm, addr, fsr, regs);
}

unsafe fn __do_user_fault(addr: usize, fsr: u32, sig: u32, code: i32, regs: *mut pt_regs) {
    let tsk = current; local_irq_enable();
    tsk.thread.address = addr; tsk.thread.error_code = fsr; tsk.thread.trap_no = 14;
    force_sig_fault(sig, code, addr as *mut core::ffi::c_void);
}

pub unsafe fn do_bad_area(addr: usize, fsr: u32, regs: *mut pt_regs) {
    let tsk = current; let mm = tsk.active_mm;
    if user_mode(regs) { __do_user_fault(addr, fsr, SIGSEGV, SEGV_MAPERR, regs); } else { __do_kernel_fault(mm, addr, fsr, regs); }
}

#[inline]
unsafe fn ttbr0_usermode_access_allowed(regs: *mut pt_regs) -> bool {
    if user_mode(regs) { return true; }
    !(to_svc_pt_regs(regs).ttbcr & TTBCR_EPD0 != 0)
}

unsafe fn vmalloc_fault(addr: usize) -> bool {
    let index = pgd_index(addr);
    let pgd = cpu_get_pgd().add(index); let pgd_k = init_mm.pgd.add(index);
    let p4d = p4d_offset(pgd, addr); let p4d_k = p4d_offset(pgd_k, addr);
    if p4d_none(*p4d_k) { return false; } if !p4d_present(*p4d) { set_p4d(p4d, *p4d_k); }
    let pud = pud_offset(p4d, addr); let pud_k = pud_offset(p4d_k, addr);
    if pud_none(*pud_k) { return false; } if !pud_present(*pud) { set_pud(pud, *pud_k); }
    let pmd = pmd_offset(pud, addr); let pmd_k = pmd_offset(pud_k, addr);
    // CONFIG_ARM_LPAE: index = 0; otherwise the hardware-entry pair is selected.
    let index = (addr >> SECTION_SHIFT) & 1;
    if pmd_none(*pmd_k.add(index)) { return false; }
    copy_pmd(pmd, pmd_k); true
}

unsafe fn do_kernel_address_page_fault(mm: *mut mm_struct, addr: usize, fsr: u32, regs: *mut pt_regs) -> i32 {
    if user_mode(regs) {
        harden_branch_predictor(); __do_user_fault(addr, fsr, SIGSEGV, SEGV_MAPERR, regs);
    } else {
        if interrupts_enabled(regs) { local_irq_enable(); }
        __do_kernel_fault(mm, addr, fsr, regs);
    }
    0
}

unsafe fn do_page_fault(addr: usize, fsr: u32, regs: *mut pt_regs) -> i32 {
    let mm = (*current).mm;
    if kprobe_page_fault(regs, fsr) { return 0; }
    if addr >= TASK_SIZE { return do_kernel_address_page_fault(mm, addr, fsr, regs); }
    if interrupts_enabled(regs) { local_irq_enable(); }
    if faulthandler_disabled() || mm.is_null() { __do_kernel_fault(mm, addr, fsr, regs); return 0; }
    let mut flags = FAULT_FLAG_DEFAULT; let mut vm_flags = VM_ACCESS_FLAGS;
    if user_mode(regs) { flags |= FAULT_FLAG_USER; }
    if is_write_fault(fsr) { flags |= FAULT_FLAG_WRITE; vm_flags = VM_WRITE; }
    if fsr & FSR_LNX_PF != 0 { vm_flags = VM_EXEC; if is_permission_fault(fsr) && !user_mode(regs) { die_kernel_fault(b"execution of memory\0".as_ptr() as _, mm, addr, fsr, regs); } }
    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, addr);
    if !ttbr0_usermode_access_allowed(regs) { __do_kernel_fault(mm, addr, fsr, regs); return 0; }
    let vma = lock_mm_and_find_vma(mm, addr, regs);
    if vma.is_null() { __do_user_fault(addr, fsr, SIGSEGV, SEGV_MAPERR, regs); return 0; }
    if (*vma).vm_flags & vm_flags == 0 { mmap_read_unlock(mm); __do_user_fault(addr, fsr, SIGSEGV, SEGV_ACCERR, regs); return 0; }
    let fault = handle_mm_fault(vma, addr & PAGE_MASK, flags, regs);
    if fault & VM_FAULT_COMPLETED != 0 { return 0; }
    mmap_read_unlock(mm);
    if fault & VM_FAULT_ERROR == 0 { return 0; }
    if !user_mode(regs) { __do_kernel_fault(mm, addr, fsr, regs); return 0; }
    if fault & VM_FAULT_OOM != 0 { pagefault_out_of_memory(); return 0; }
    if fault & VM_FAULT_SIGBUS != 0 { __do_user_fault(addr, fsr, SIGBUS, BUS_ADRERR, regs); } else { __do_user_fault(addr, fsr, SIGSEGV, SEGV_MAPERR, regs); } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
