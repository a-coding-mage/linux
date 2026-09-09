// SPDX-License-Identifier: GPL-2.0
/* Page fault handlers for the Sparc. */

// Declarations supplied by the surrounding kernel translation unit.
use crate::*;

pub static mut show_unhandled_signals: i32 = 1;

unsafe fn unhandled_fault(address: libc::c_ulong, tsk: *mut task_struct, regs: *mut pt_regs) -> ! {
    if address < PAGE_SIZE as libc::c_ulong {
        printk(KERN_ALERT, "Unable to handle kernel NULL pointer dereference\n");
    } else {
        printk(KERN_ALERT, "Unable to handle kernel paging request at virtual address %08lx\n", address);
    }
    printk(KERN_ALERT, "tsk->{mm,active_mm}->context = %08lx\n",
        if !(*tsk).mm.is_null() { (*(*tsk).mm).context } else { (*(*tsk).active_mm).context });
    printk(KERN_ALERT, "tsk->{mm,active_mm}->pgd = %08lx\n",
        if !(*tsk).mm.is_null() { (*(*tsk).mm).pgd as libc::c_ulong } else { (*(*tsk).active_mm).pgd as libc::c_ulong });
    die_if_kernel("Oops", regs);
    loop {}
}

unsafe fn show_signal_msg(regs: *mut pt_regs, sig: i32, code: i32,
                          address: libc::c_ulong, tsk: *mut task_struct) {
    if !unhandled_signal(tsk, sig) || !printk_ratelimit() { return; }
    printk("%s%s[%d]: segfault at %lx ip %px (rpc %px) sp %px error %x",
        if task_pid_nr(tsk) > 1 { KERN_INFO } else { KERN_EMERG }, (*tsk).comm,
        task_pid_nr(tsk), address, (*regs).pc as *mut _, (*regs).u_regs[UREG_I7] as *mut _,
        (*regs).u_regs[UREG_FP] as *mut _, code);
    print_vma_addr(KERN_CONT " in ", (*regs).pc);
    printk(KERN_CONT "\n");
}

unsafe fn __do_fault_siginfo(code: i32, sig: i32, regs: *mut pt_regs, addr: libc::c_ulong) {
    if show_unhandled_signals != 0 { show_signal_msg(regs, sig, code, addr, current); }
    force_sig_fault(sig, code, addr as *mut libc::c_void);
}

unsafe fn compute_si_addr(regs: *mut pt_regs, text_fault: i32) -> libc::c_ulong {
    if text_fault != 0 { return (*regs).pc; }
    let insn: u32;
    if (*regs).psr & PSR_PS != 0 { insn = *((*regs).pc as *const u32); }
    else { insn = 0; __get_user(&mut *(core::ptr::addr_of!(insn) as *mut u32), (*regs).pc as *mut u32); }
    safe_compute_effective_address(regs, insn)
}

unsafe fn do_fault_siginfo(code: i32, sig: i32, regs: *mut pt_regs, text_fault: i32) {
    __do_fault_siginfo(code, sig, regs, compute_si_addr(regs, text_fault));
}

pub unsafe fn do_sparc_fault(regs: *mut pt_regs, text_fault: i32, write: i32, mut address: libc::c_ulong) {
    let tsk = current;
    let mm = (*tsk).mm;
    let from_user = ((*regs).psr & PSR_PS) == 0;
    let mut code = SEGV_MAPERR;
    let mut flags = FAULT_FLAG_DEFAULT;
    if text_fault != 0 { address = (*regs).pc; }
    if address >= TASK_SIZE { return vmalloc_fault(regs, tsk, address, text_fault, write); }
    if pagefault_disabled() || mm.is_null() || (!from_user && address >= PAGE_OFFSET) { return no_context(regs, tsk, address); }
    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, address);
    loop {
        let vma = lock_mm_and_find_vma(mm, address, regs);
        if vma.is_null() { return bad_area_nosemaphore(code, regs, text_fault, from_user, tsk, address); }
        code = SEGV_ACCERR;
        if write != 0 { if (*vma).vm_flags & VM_WRITE == 0 { mmap_read_unlock(mm); return bad_area_nosemaphore(code, regs, text_fault, from_user, tsk, address); } }
        else if (*vma).vm_flags & (VM_READ | VM_EXEC) == 0 { mmap_read_unlock(mm); return bad_area_nosemaphore(code, regs, text_fault, from_user, tsk, address); }
        if from_user { flags |= FAULT_FLAG_USER; } if write != 0 { flags |= FAULT_FLAG_WRITE; }
        let fault = handle_mm_fault(vma, address, flags, regs);
        if fault_signal_pending(fault, regs) { if !from_user { return no_context(regs, tsk, address); } return; }
        if fault & VM_FAULT_COMPLETED != 0 { return; }
        if fault & VM_FAULT_ERROR != 0 { if fault & VM_FAULT_OOM != 0 { mmap_read_unlock(mm); if from_user { pagefault_out_of_memory(); return; } return no_context(regs, tsk, address); } if fault & VM_FAULT_SIGSEGV != 0 { mmap_read_unlock(mm); return bad_area_nosemaphore(code, regs, text_fault, from_user, tsk, address); } if fault & VM_FAULT_SIGBUS != 0 { mmap_read_unlock(mm); do_fault_siginfo(BUS_ADRERR, SIGBUS, regs, text_fault); if !from_user { return no_context(regs, tsk, address); } return; } BUG(); }
        if fault & VM_FAULT_RETRY != 0 { flags |= FAULT_FLAG_TRIED; continue; }
        mmap_read_unlock(mm); return;
    }
}

unsafe fn bad_area_nosemaphore(code: i32, regs: *mut pt_regs, text_fault: i32, from_user: bool, tsk: *mut task_struct, address: libc::c_ulong) {
    if from_user { do_fault_siginfo(code, SIGSEGV, regs, text_fault); } else { no_context(regs, tsk, address); }
}

unsafe fn no_context(regs: *mut pt_regs, tsk: *mut task_struct, address: libc::c_ulong) {
    let entry = search_exception_tables((*regs).pc);
    if !entry.is_null() { (*regs).pc = (*entry).fixup; (*regs).npc = (*regs).pc + 4; return; }
    unhandled_fault(address, tsk, regs);
}

unsafe fn vmalloc_fault(regs: *mut pt_regs, tsk: *mut task_struct, address: libc::c_ulong, text_fault: i32, _write: i32) {
    let offset = pgd_index(address); let pgd = (*(*tsk).active_mm).pgd.add(offset); let pgd_k = init_mm.pgd.add(offset);
    if !pgd_present(*pgd) { if !pgd_present(*pgd_k) { return bad_area_nosemaphore(SEGV_MAPERR, regs, text_fault, false, tsk, address); } *pgd = *pgd_k; return; }
    let p4d = p4d_offset(pgd, address); let pud = pud_offset(p4d, address); let pmd = pmd_offset(pud, address);
    let p4d_k = p4d_offset(pgd_k, address); let pud_k = pud_offset(p4d_k, address); let pmd_k = pmd_offset(pud_k, address);
    if pmd_present(*pmd) || !pmd_present(*pmd_k) { return bad_area_nosemaphore(SEGV_MAPERR, regs, text_fault, false, tsk, address); } *pmd = *pmd_k;
}

unsafe fn force_user_fault(address: libc::c_ulong, write: i32) {
    let tsk = current; let mm = (*tsk).mm; let mut flags = FAULT_FLAG_USER; let mut code = SEGV_MAPERR;
    let vma = lock_mm_and_find_vma(mm, address, core::ptr::null_mut());
    if vma.is_null() { return __do_fault_siginfo(code, SIGSEGV, (*tsk).thread.kregs, address); }
    code = SEGV_ACCERR;
    if write != 0 { if (*vma).vm_flags & VM_WRITE == 0 { mmap_read_unlock(mm); return __do_fault_siginfo(code, SIGSEGV, (*tsk).thread.kregs, address); } flags |= FAULT_FLAG_WRITE; }
    else if (*vma).vm_flags & (VM_READ | VM_EXEC) == 0 { mmap_read_unlock(mm); return __do_fault_siginfo(code, SIGSEGV, (*tsk).thread.kregs, address); }
    let fault = handle_mm_fault(vma, address, flags, core::ptr::null_mut());
    if fault == VM_FAULT_SIGBUS || fault == VM_FAULT_OOM { mmap_read_unlock(mm); __do_fault_siginfo(BUS_ADRERR, SIGBUS, (*tsk).thread.kregs, address); } else { mmap_read_unlock(mm); }
}

unsafe fn check_stack_aligned(sp: libc::c_ulong) { if sp & 7 != 0 { force_sig(SIGILL); } }
pub unsafe fn window_overflow_fault() { let sp = (*current_thread_info()).rwbuf_stkptrs[0]; if ((sp + 0x38) & PAGE_MASK) != (sp & PAGE_MASK) { force_user_fault(sp + 0x38, 1); } force_user_fault(sp, 1); check_stack_aligned(sp); }
pub unsafe fn window_underflow_fault(sp: libc::c_ulong) { if ((sp + 0x38) & PAGE_MASK) != (sp & PAGE_MASK) { force_user_fault(sp + 0x38, 0); } force_user_fault(sp, 0); check_stack_aligned(sp); }
pub unsafe fn window_ret_fault(regs: *mut pt_regs) { let sp = (*regs).u_regs[UREG_FP]; if ((sp + 0x38) & PAGE_MASK) != (sp & PAGE_MASK) { force_user_fault(sp + 0x38, 0); } force_user_fault(sp, 0); check_stack_aligned(sp); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
