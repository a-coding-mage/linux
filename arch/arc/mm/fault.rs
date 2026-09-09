// SPDX-License-Identifier: GPL-2.0-only
/* Page Fault Handling for ARC (TLB Miss / ProtV)
 *
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

/* Dependencies supplied by the Linux kernel and architecture headers. */

/*
 * kernel virtual address is required to implement vmalloc/pkmap/fixmap
 * Refer to asm/processor.h for System Memory Map
 *
 * It simply copies the PMD entry (pointer to 2nd level page table or hugepage)
 * from swapper pgdir to task pgdir. The 2nd level table/page is thus shared
 */
#[no_mangle]
unsafe fn handle_kernel_vaddr_fault(address: c_ulong) -> c_int {
    /*
     * Synchronize this task's top level page-table
     * with the 'reference' page table.
     */
    let mut pgd: *mut pgd_t;
    let mut pgd_k: *mut pgd_t;
    let mut p4d: *mut p4d_t;
    let mut p4d_k: *mut p4d_t;
    let mut pud: *mut pud_t;
    let mut pud_k: *mut pud_t;
    let mut pmd: *mut pmd_t;
    let mut pmd_k: *mut pmd_t;

    pgd = pgd_offset((*current).active_mm, address);
    pgd_k = pgd_offset_k(address);

    if pgd_none(*pgd_k) {
        return 1;
    }
    if !pgd_present(*pgd) {
        set_pgd(pgd, *pgd_k);
    }

    p4d = p4d_offset(pgd, address);
    p4d_k = p4d_offset(pgd_k, address);
    if p4d_none(*p4d_k) {
        return 1;
    }
    if !p4d_present(*p4d) {
        set_p4d(p4d, *p4d_k);
    }

    pud = pud_offset(p4d, address);
    pud_k = pud_offset(p4d_k, address);
    if pud_none(*pud_k) {
        return 1;
    }
    if !pud_present(*pud) {
        set_pud(pud, *pud_k);
    }

    pmd = pmd_offset(pud, address);
    pmd_k = pmd_offset(pud_k, address);
    if pmd_none(*pmd_k) {
        return 1;
    }
    if !pmd_present(*pmd) {
        set_pmd(pmd, *pmd_k);
    }

    /* XXX: create the TLB entry here */
    0
}

#[no_mangle]
pub unsafe fn do_page_fault(address: c_ulong, regs: *mut pt_regs) {
    let mut vma: *mut vm_area_struct = core::ptr::null_mut();
    let tsk: *mut task_struct = current;
    let mm: *mut mm_struct = (*tsk).mm;
    let mut sig: c_int;
    let mut si_code: c_int = SEGV_MAPERR;
    let mut write: c_uint = 0;
    let mut exec: c_uint = 0;
    let mut mask: c_uint;
    let mut fault: vm_fault_t = VM_FAULT_SIGSEGV;
    let mut flags: c_uint;

    /*
     * NOTE! We MUST NOT take any locks for this case. We may
     * be in an interrupt or a critical region, and should
     * only copy the information from the master page table,
     * nothing more.
     */
    'no_context: loop {
    if address >= VMALLOC_START && !user_mode(regs) {
        if unlikely(handle_kernel_vaddr_fault(address) != 0) {
            break 'no_context;
        } else {
            return;
        }
    }

    /*
     * If we're in an interrupt or have no user
     * context, we must not take the fault..
     */
    if faulthandler_disabled() || mm.is_null() {
        break 'no_context;
    }

    if (*regs).ecr.cause & ECR_C_PROTV_STORE != 0 {
        write = 1;
    } else if (*regs).ecr.vec == ECR_V_PROTV
        && (*regs).ecr.cause == ECR_C_PROTV_INST_FETCH
    {
        exec = 1;
    }

    flags = FAULT_FLAG_DEFAULT;
    if user_mode(regs) {
        flags |= FAULT_FLAG_USER;
    }
    if write != 0 {
        flags |= FAULT_FLAG_WRITE;
    }

    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, address);

    'retry: loop {
        vma = lock_mm_and_find_vma(mm, address, regs);
        if vma.is_null() {
            break;
        }

        mask = VM_READ;
        if write != 0 {
            mask = VM_WRITE;
        }
        if exec != 0 {
            mask = VM_EXEC;
        }

        if (*vma).vm_flags & mask == 0 {
            si_code = SEGV_ACCERR;
            mmap_read_unlock(mm);
            break;
        }

        fault = handle_mm_fault(vma, address, flags, regs);

        if fault_signal_pending(fault, regs) {
            if !user_mode(regs) {
                break 'no_context;
            }
            return;
        }
        if fault & VM_FAULT_COMPLETED != 0 {
            return;
        }
        if unlikely(fault & VM_FAULT_RETRY != 0) {
            flags |= FAULT_FLAG_TRIED;
            continue 'retry;
        }
        mmap_read_unlock(mm);
        break;
    }

    if likely(fault & VM_FAULT_ERROR == 0) {
        return;
    }
    if !user_mode(regs) {
        break 'no_context;
    }
    if fault & VM_FAULT_OOM != 0 {
        pagefault_out_of_memory();
        return;
    }
    if fault & VM_FAULT_SIGBUS != 0 {
        sig = SIGBUS;
        si_code = BUS_ADRERR;
    } else {
        sig = SIGSEGV;
    }
    (*tsk).thread.fault_address = address;
    force_sig_fault(sig, si_code, address as *mut core::ffi::c_void);
    return;

    if fixup_exception(regs) {
        return;
    }
    die("Oops", regs, address);
    break 'no_context;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
