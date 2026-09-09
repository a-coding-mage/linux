// TODO VM_EXEC flag work-around, cache aliasing
/*
 * arch/xtensa/mm/fault.c
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2010 Tensilica Inc.
 *
 * Chris Zankel <chris@zankel.net>
 * Joe Taylor <joe@tensilica.com, joetylr@yahoo.com>
 */

// Linux and architecture dependencies are supplied externally.

extern "C" {
    fn bad_page_fault(regs: *mut pt_regs, address: u64, sig: i32);
}

unsafe fn vmalloc_fault(regs: *mut pt_regs, address: u32) {
    // CONFIG_MMU controls this block at build time.
    #[cfg(CONFIG_MMU)]
    {
        /* Synchronize this task's top level page-table
         * with the 'reference' page table.
         */
        let act_mm = (*current).active_mm;
        let index = pgd_index(address);
        let mut pgd: *mut pgd_t;
        let pgd_k: *mut pgd_t;
        let p4d: *mut p4d_t;
        let p4d_k: *mut p4d_t;
        let pud: *mut pud_t;
        let pud_k: *mut pud_t;
        let pmd: *mut pmd_t;
        let pmd_k: *mut pmd_t;
        let pte_k: *mut pte_t;

        if act_mm.is_null() {
            bad_page_fault(regs, address as u64, SIGKILL);
            return;
        }

        pgd = (*act_mm).pgd.add(index as usize);
        pgd_k = init_mm.pgd.add(index as usize);

        if !pgd_present(*pgd_k) {
            bad_page_fault(regs, address as u64, SIGKILL);
            return;
        }

        *pgd = pgd_val(*pgd_k);

        let p4d_local = p4d_offset(pgd, address);
        let p4d_k_local = p4d_offset(pgd_k, address);
        if !p4d_present(*p4d_local) || !p4d_present(*p4d_k_local) {
            bad_page_fault(regs, address as u64, SIGKILL);
            return;
        }

        let pud_local = pud_offset(p4d_local, address);
        let pud_k_local = pud_offset(p4d_k_local, address);
        if !pud_present(*pud_local) || !pud_present(*pud_k_local) {
            bad_page_fault(regs, address as u64, SIGKILL);
            return;
        }

        let pmd_local = pmd_offset(pud_local, address);
        let pmd_k_local = pmd_offset(pud_k_local, address);
        if !pmd_present(*pmd_local) || !pmd_present(*pmd_k_local) {
            bad_page_fault(regs, address as u64, SIGKILL);
            return;
        }

        *pmd_local = pmd_val(*pmd_k_local);
        let pte_k_local = pte_offset_kernel(pmd_k_local, address);

        if !pte_present(*pte_k_local) {
            bad_page_fault(regs, address as u64, SIGKILL);
        }
    }
    #[cfg(not(CONFIG_MMU))]
    {
        WARN_ONCE!(true, "%s in noMMU configuration\n", "vmalloc_fault");
    }
}

/*
 * This routine handles page faults.  It determines the address,
 * and the problem, and then passes it off to one of the appropriate
 * routines.
 *
 * Note: does not handle Miss and MultiHit.
 */
pub unsafe fn do_page_fault(regs: *mut pt_regs) {
    let mut vma: *mut vm_area_struct;
    let mm = (*current).mm;
    let exccause = (*regs).exccause;
    let address = (*regs).excvaddr;
    let mut code = SEGV_MAPERR;
    let is_write = if exccause == EXCCAUSE_STORE_CACHE_ATTRIBUTE { 1 } else { 0 };
    let is_exec = if exccause == EXCCAUSE_ITLB_PRIVILEGE
        || exccause == EXCCAUSE_ITLB_MISS
        || exccause == EXCCAUSE_FETCH_CACHE_ATTRIBUTE { 1 } else { 0 };
    let mut flags = FAULT_FLAG_DEFAULT;

    if address >= TASK_SIZE && !user_mode(regs) {
        vmalloc_fault(regs, address);
        return;
    }

    if faulthandler_disabled() || mm.is_null() {
        bad_page_fault(regs, address as u64, SIGSEGV);
        return;
    }

    pr_debug!("[%s:%d:%08x:%d:%08lx:%s%s]\\n", (*current).comm, (*current).pid,
        address, exccause, (*regs).pc, if is_write != 0 { "w" } else { "" },
        if is_exec != 0 { "x" } else { "" });

    if user_mode(regs) { flags |= FAULT_FLAG_USER; }
    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, address);

    'retry: loop {
        vma = lock_mm_and_find_vma(mm, address, regs);
        if vma.is_null() { break; }
        code = SEGV_ACCERR;
        if is_write != 0 {
            if (*vma).vm_flags & VM_WRITE == 0 { break; }
            flags |= FAULT_FLAG_WRITE;
        } else if is_exec != 0 {
            if (*vma).vm_flags & VM_EXEC == 0 { break; }
        } else if (*vma).vm_flags & (VM_READ | VM_WRITE) == 0 { break; }

        let fault = handle_mm_fault(vma, address, flags, regs);
        if fault_signal_pending(fault, regs) {
            if !user_mode(regs) { bad_page_fault(regs, address as u64, SIGKILL); }
            return;
        }
        if fault & VM_FAULT_COMPLETED != 0 { return; }
        if fault & VM_FAULT_ERROR != 0 {
            if fault & VM_FAULT_OOM != 0 { mmap_read_unlock(mm); if !user_mode(regs) { bad_page_fault(regs, address as u64, SIGKILL); } else { pagefault_out_of_memory(); } return; }
            if fault & VM_FAULT_SIGSEGV != 0 { break; }
            if fault & VM_FAULT_SIGBUS != 0 { mmap_read_unlock(mm); force_sig_fault(SIGBUS, BUS_ADRERR, address as *mut core::ffi::c_void); if !user_mode(regs) { bad_page_fault(regs, address as u64, SIGBUS); } return; }
            BUG!();
        }
        if fault & VM_FAULT_RETRY != 0 { flags |= FAULT_FLAG_TRIED; continue; }
        mmap_read_unlock(mm);
        return;
    }

    mmap_read_unlock(mm);
    if user_mode(regs) { force_sig_fault(SIGSEGV, code, address as *mut core::ffi::c_void); return; }
    bad_page_fault(regs, address as u64, SIGSEGV);
}

pub unsafe fn bad_page_fault(regs: *mut pt_regs, address: u64, sig: i32) {
    extern "C" { fn die(msg: *const core::ffi::c_char, regs: *mut pt_regs, sig: i64) -> !; }
    let entry = search_exception_tables((*regs).pc);
    if !entry.is_null() {
        pr_debug!("%s: Exception at pc=%#010lx (%lx)\\n", (*current).comm, (*regs).pc, (*entry).fixup);
        (*regs).pc = (*entry).fixup;
        return;
    }
    pr_alert!("Unable to handle kernel paging request at virtual address %08lx\\n pc = %08lx, ra = %08lx\\n", address, (*regs).pc, (*regs).areg[0]);
    die(b"Oops\\0".as_ptr() as *const core::ffi::c_char, regs, sig as i64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
