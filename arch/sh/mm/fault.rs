/*
 * Page fault handler for SH with an MMU.
 *
 *  Copyright (C) 1999  Niibe Yutaka
 *  Copyright (C) 2003 - 2012  Paul Mundt
 *
 *  Based on linux/arch/i386/mm/fault.c:
 *   Copyright (C) 1995  Linus Torvalds
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation unit.

unsafe fn force_sig_info_fault(si_signo: i32, si_code: i32, address: usize) {
    force_sig_fault(si_signo, si_code, address as *mut core::ffi::c_void);
}

/*
 * This is useful to dump out the page tables associated with
 * 'addr' in mm 'mm'.
 */
unsafe fn show_pte(mm: *mut mm_struct, addr: usize) {
    let mut pgd: *mut pgd_t;

    if !mm.is_null() {
        pgd = (*mm).pgd;
    } else {
        pgd = get_TTB();

        if unlikely(pgd.is_null()) {
            pgd = swapper_pg_dir;
        }
    }

    pr_alert!("pgd = %p\n", pgd);
    pgd = pgd.add(pgd_index(addr) as usize);
    pr_alert!("[%08lx] *pgd=%0*llx", addr, (core::mem::size_of::<pgd_t>() * 2) as u32,
              pgd_val(*pgd) as u64);

    loop {
        let p4d: *mut p4d_t;
        let pud: *mut pud_t;
        let pmd: *mut pmd_t;
        let pte: *mut pte_t;

        if pgd_none(*pgd) { break; }
        if pgd_bad(*pgd) { pr_cont!("(bad)"); break; }

        p4d = p4d_offset(pgd, addr);
        if PTRS_PER_P4D != 1 {
            pr_cont!(", *p4d=%0*Lx", (core::mem::size_of::<p4d_t>() * 2) as u32,
                     p4d_val(*p4d) as u64);
        }
        if p4d_none(*p4d) { break; }
        if p4d_bad(*p4d) { pr_cont!("(bad)"); break; }

        pud = pud_offset(p4d, addr);
        if PTRS_PER_PUD != 1 {
            pr_cont!(", *pud=%0*llx", (core::mem::size_of::<pud_t>() * 2) as u32,
                     pud_val(*pud) as u64);
        }
        if pud_none(*pud) { break; }
        if pud_bad(*pud) { pr_cont!("(bad)"); break; }

        pmd = pmd_offset(pud, addr);
        if PTRS_PER_PMD != 1 {
            pr_cont!(", *pmd=%0*llx", (core::mem::size_of::<pmd_t>() * 2) as u32,
                     pmd_val(*pmd) as u64);
        }
        if pmd_none(*pmd) { break; }
        if pmd_bad(*pmd) { pr_cont!("(bad)"); break; }

        /* We must not map this if we have highmem enabled */
        if PageHighMem(pfn_to_page(pmd_val(*pmd) >> PAGE_SHIFT)) { break; }

        pte = pte_offset_kernel(pmd, addr);
        pr_cont!(", *pte=%0*llx", (core::mem::size_of::<pte_t>() * 2) as u32,
                 pte_val(*pte) as u64);
        break;
    }

    pr_cont!("\n");
}

unsafe fn vmalloc_sync_one(mut pgd: *mut pgd_t, address: usize) -> *mut pmd_t {
    let index = pgd_index(address) as usize;
    let pgd_k: *mut pgd_t;
    let p4d: *mut p4d_t;
    let p4d_k: *mut p4d_t;
    let pud: *mut pud_t;
    let pud_k: *mut pud_t;
    let pmd: *mut pmd_t;
    let pmd_k: *mut pmd_t;

    pgd = pgd.add(index);
    pgd_k = init_mm.pgd.add(index);
    if !pgd_present(*pgd_k) { return core::ptr::null_mut(); }

    p4d = p4d_offset(pgd, address);
    p4d_k = p4d_offset(pgd_k, address);
    if !p4d_present(*p4d_k) { return core::ptr::null_mut(); }

    pud = pud_offset(p4d, address);
    pud_k = pud_offset(p4d_k, address);
    if !pud_present(*pud_k) { return core::ptr::null_mut(); }
    if !pud_present(*pud) { set_pud(pud, *pud_k); }

    pmd = pmd_offset(pud, address);
    pmd_k = pmd_offset(pud_k, address);
    if !pmd_present(*pmd_k) { return core::ptr::null_mut(); }
    if !pmd_present(*pmd) {
        set_pmd(pmd, *pmd_k);
    } else {
        /*
         * The page tables are fully synchronised so there must
         * be another reason for the fault. Return NULL here to
         * signal that we have not taken care of the fault.
         */
        BUG_ON(pmd_page(*pmd) != pmd_page(*pmd_k));
        return core::ptr::null_mut();
    }
    pmd_k
}

#[cfg(CONFIG_SH_STORE_QUEUES)]
const __FAULT_ADDR_LIMIT: usize = P3_ADDR_MAX;
#[cfg(not(CONFIG_SH_STORE_QUEUES))]
const __FAULT_ADDR_LIMIT: usize = VMALLOC_END;

/* Handle a fault on the vmalloc or module mapping area */
unsafe fn vmalloc_fault(address: usize) -> i32 {
    if !(address >= VMALLOC_START && address < __FAULT_ADDR_LIMIT) { return -1; }
    let pgd_k = get_TTB();
    let pmd_k = vmalloc_sync_one(pgd_k, address);
    if pmd_k.is_null() { return -1; }
    let pte_k = pte_offset_kernel(pmd_k, address);
    if !pte_present(*pte_k) { return -1; }
    0
}

unsafe fn show_fault_oops(regs: *mut pt_regs, address: usize) {
    if !oops_may_print() { return; }
    pr_alert!("BUG: unable to handle kernel %s at %08lx\n",
              if address < PAGE_SIZE { "NULL pointer dereference" } else { "paging request" }, address);
    pr_alert!("PC:");
    printk_address((*regs).pc, 1);
    show_pte(core::ptr::null_mut(), address);
}

unsafe fn no_context(regs: *mut pt_regs, error_code: usize, address: usize) {
    if fixup_exception(regs) { return; }
    if handle_trapped_io(regs, address) { return; }
    bust_spinlocks(1);
    show_fault_oops(regs, address);
    die("Oops", regs, error_code);
}

unsafe fn __bad_area_nosemaphore(regs: *mut pt_regs, error_code: usize, address: usize, si_code: i32) {
    if user_mode(regs) {
        local_irq_enable();
        force_sig_info_fault(SIGSEGV, si_code, address);
        return;
    }
    no_context(regs, error_code, address);
}

unsafe fn bad_area_nosemaphore(regs: *mut pt_regs, error_code: usize, address: usize) {
    __bad_area_nosemaphore(regs, error_code, address, SEGV_MAPERR);
}

unsafe fn __bad_area(regs: *mut pt_regs, error_code: usize, address: usize, si_code: i32) {
    let mm = (*current).mm;
    mmap_read_unlock(mm);
    __bad_area_nosemaphore(regs, error_code, address, si_code);
}

unsafe fn bad_area(regs: *mut pt_regs, error_code: usize, address: usize) {
    __bad_area(regs, error_code, address, SEGV_MAPERR);
}

unsafe fn bad_area_access_error(regs: *mut pt_regs, error_code: usize, address: usize) {
    __bad_area(regs, error_code, address, SEGV_ACCERR);
}

unsafe fn do_sigbus(regs: *mut pt_regs, error_code: usize, address: usize) {
    let tsk = current;
    let mm = (*tsk).mm;
    mmap_read_unlock(mm);
    if !user_mode(regs) { no_context(regs, error_code, address); }
    force_sig_info_fault(SIGBUS, BUS_ADRERR, address);
}

unsafe fn mm_fault_error(regs: *mut pt_regs, error_code: usize, address: usize, fault: vm_fault_t) -> i32 {
    if fault_signal_pending(fault, regs) {
        if !user_mode(regs) { no_context(regs, error_code, address); }
        return 1;
    }
    if (fault & VM_FAULT_RETRY) == 0 { mmap_read_unlock((*current).mm); }
    if (fault & VM_FAULT_ERROR) == 0 { return 0; }
    if (fault & VM_FAULT_OOM) != 0 {
        if !user_mode(regs) { no_context(regs, error_code, address); return 1; }
        pagefault_out_of_memory();
    } else if (fault & VM_FAULT_SIGBUS) != 0 {
        do_sigbus(regs, error_code, address);
    } else if (fault & VM_FAULT_SIGSEGV) != 0 {
        bad_area(regs, error_code, address);
    } else { BUG(); }
    1
}

unsafe fn access_error(error_code: i32, vma: *mut vm_area_struct) -> i32 {
    if (error_code & FAULT_CODE_WRITE) != 0 {
        if unlikely(((*vma).vm_flags & VM_WRITE) == 0) { return 1; }
        return 0;
    }
    if unlikely((error_code & FAULT_CODE_ITLB) != 0 && ((*vma).vm_flags & VM_EXEC) == 0) { return 1; }
    if unlikely(!vma_is_accessible(vma)) { return 1; }
    0
}

fn fault_in_kernel_space(address: usize) -> i32 { (address >= TASK_SIZE) as i32 }

/*
 * This routine handles page faults.  It determines the address,
 * and the problem, and then passes it off to one of the appropriate
 * routines.
 */
unsafe fn do_page_fault(regs: *mut pt_regs, error_code: usize, address: usize) {
    let tsk = current;
    let mm = (*tsk).mm;
    let vec = lookup_exception_vector();
    if unlikely(fault_in_kernel_space(address) != 0) {
        if vmalloc_fault(address) >= 0 { return; }
        if kprobe_page_fault(regs, vec) { return; }
        bad_area_nosemaphore(regs, error_code, address);
        return;
    }
    if unlikely(kprobe_page_fault(regs, vec)) { return; }
    if ((*regs).sr & SR_IMASK) != SR_IMASK { local_irq_enable(); }
    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, address);
    if unlikely(faulthandler_disabled() || mm.is_null()) {
        bad_area_nosemaphore(regs, error_code, address);
        return;
    }
    let mut flags = FAULT_FLAG_DEFAULT;
    loop {
        let vma = lock_mm_and_find_vma(mm, address, regs);
        if unlikely(vma.is_null()) { bad_area_nosemaphore(regs, error_code, address); return; }
        if unlikely(access_error(error_code as i32, vma) != 0) {
            bad_area_access_error(regs, error_code, address); return;
        }
        set_thread_fault_code(error_code);
        if user_mode(regs) { flags |= FAULT_FLAG_USER; }
        if (error_code & FAULT_CODE_WRITE) != 0 { flags |= FAULT_FLAG_WRITE; }
        let fault = handle_mm_fault(vma, address, flags, regs);
        if unlikely((fault & (VM_FAULT_RETRY | VM_FAULT_ERROR)) != 0) && mm_fault_error(regs, error_code, address, fault) != 0 { return; }
        if (fault & VM_FAULT_COMPLETED) != 0 { return; }
        if (fault & VM_FAULT_RETRY) != 0 {
            flags |= FAULT_FLAG_TRIED;
            continue;
        }
        mmap_read_unlock(mm);
        return;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
