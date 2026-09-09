// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

pub unsafe fn fixup_exception(regs: *mut pt_regs) -> i32 {
    let fixup: *const exception_table_entry;

    fixup = search_exception_tables(instruction_pointer(regs));
    if !fixup.is_null() {
        (*regs).pc = (*fixup).fixup;

        return 1;
    }

    0
}

unsafe fn is_write(regs: *mut pt_regs) -> bool {
    match trap_no(regs) {
        VEC_TLBINVALIDS => true,
        VEC_TLBMODIFIED => true,
        _ => false,
    }
}

#[cfg(CONFIG_CPU_HAS_LDSTEX)]
unsafe fn csky_cmpxchg_fixup(_regs: *mut pt_regs) {
    return;
}

#[cfg(not(CONFIG_CPU_HAS_LDSTEX))]
extern "C" {
    static mut csky_cmpxchg_ldw: ::core::ffi::c_ulong;
    static mut csky_cmpxchg_stw: ::core::ffi::c_ulong;
}

#[cfg(not(CONFIG_CPU_HAS_LDSTEX))]
unsafe fn csky_cmpxchg_fixup(regs: *mut pt_regs) {
    if trap_no(regs) != VEC_TLBMODIFIED {
        return;
    }

    if instruction_pointer(regs) == (&csky_cmpxchg_stw as *const _ as ::core::ffi::c_ulong) {
        instruction_pointer_set(regs, &csky_cmpxchg_ldw as *const _ as ::core::ffi::c_ulong);
    }
}

unsafe fn no_context(regs: *mut pt_regs, addr: ::core::ffi::c_ulong) {
    (*current).thread.trap_no = trap_no(regs);

    /* Are we prepared to handle this kernel fault? */
    if fixup_exception(regs) != 0 {
        return;
    }

    /*
     * Oops. The kernel tried to access some bad page. We'll have to
     * terminate things with extreme prejudice.
     */
    bust_spinlocks(1);
    pr_alert!("Unable to handle kernel paging request at virtual addr 0x%08lx, pc: 0x%08lx\n", addr, (*regs).pc);
    die(regs, "Oops");
    make_task_dead(SIGKILL);
}

unsafe fn mm_fault_error(regs: *mut pt_regs, addr: ::core::ffi::c_ulong, fault: vm_fault_t) {
    (*current).thread.trap_no = trap_no(regs);

    if fault & VM_FAULT_OOM != 0 {
        /*
         * We ran out of memory, call the OOM killer, and return the userspace
         * (which will retry the fault, or kill us if we got oom-killed).
         */
        if !user_mode(regs) {
            no_context(regs, addr);
            return;
        }
        pagefault_out_of_memory();
        return;
    } else if fault & VM_FAULT_SIGBUS != 0 {
        /* Kernel mode? Handle exceptions or die */
        if !user_mode(regs) {
            no_context(regs, addr);
            return;
        }
        do_trap(regs, SIGBUS, BUS_ADRERR, addr);
        return;
    }
    BUG!();
}

unsafe fn bad_area_nosemaphore(
    regs: *mut pt_regs,
    _mm: *mut mm_struct,
    code: i32,
    addr: ::core::ffi::c_ulong,
) {
    /*
     * Something tried to access memory that isn't in our memory map.
     * Fix it, but check if it's kernel or user first.
     */
    /* User mode accesses just cause a SIGSEGV */
    if user_mode(regs) {
        do_trap(regs, SIGSEGV, code, addr);
        return;
    }

    no_context(regs, addr);
}

unsafe fn vmalloc_fault(regs: *mut pt_regs, code: i32, addr: ::core::ffi::c_ulong) {
    let mut pgd: *mut pgd_t;
    let mut pgd_k: *mut pgd_t;
    let mut pud: *mut pud_t;
    let mut pud_k: *mut pud_t;
    let mut pmd: *mut pmd_t;
    let mut pmd_k: *mut pmd_t;
    let pte_k: *mut pte_t;
    let offset: i32;

    /* User mode accesses just cause a SIGSEGV */
    if user_mode(regs) {
        do_trap(regs, SIGSEGV, code, addr);
        return;
    }

    /* Synchronize this task's top level page-table with the 'reference' page table. */
    offset = pgd_index(addr);

    pgd = get_pgd().offset(offset as isize);
    pgd_k = init_mm.pgd.offset(offset as isize);

    if !pgd_present(*pgd_k) {
        no_context(regs, addr);
        return;
    }
    set_pgd(pgd, *pgd_k);

    pud = pgd as *mut pud_t;
    pud_k = pgd_k as *mut pud_t;
    if !pud_present(*pud_k) {
        no_context(regs, addr);
        return;
    }

    pmd = pmd_offset(pud, addr);
    pmd_k = pmd_offset(pud_k, addr);
    if !pmd_present(*pmd_k) {
        no_context(regs, addr);
        return;
    }
    set_pmd(pmd, *pmd_k);

    pte_k = pte_offset_kernel(pmd_k, addr);
    if !pte_present(*pte_k) {
        no_context(regs, addr);
        return;
    }

    flush_tlb_one(addr);
}

unsafe fn access_error(regs: *mut pt_regs, vma: *mut vm_area_struct) -> bool {
    if is_write(regs) {
        if (*vma).vm_flags & VM_WRITE == 0 {
            return true;
        }
    } else if unlikely(!vma_is_accessible(vma)) {
        return true;
    }
    false
}

/*
 * This routine handles page faults.  It determines the address and the
 * problem, and then passes it off to one of the appropriate routines.
 */
pub unsafe extern "C" fn do_page_fault(regs: *mut pt_regs) {
    let tsk: *mut task_struct;
    let mut vma: *mut vm_area_struct;
    let mm: *mut mm_struct;
    let addr: ::core::ffi::c_ulong = read_mmu_entryhi() & PAGE_MASK;
    let mut flags: ::core::ffi::c_uint = FAULT_FLAG_DEFAULT;
    let mut code: i32 = SEGV_MAPERR;
    let fault: vm_fault_t;

    tsk = current;
    mm = (*tsk).mm;

    csky_cmpxchg_fixup(regs);

    if kprobe_page_fault(regs, (*tsk).thread.trap_no) {
        return;
    }

    if unlikely(addr >= VMALLOC_START && addr <= VMALLOC_END) {
        vmalloc_fault(regs, code, addr);
        return;
    }

    if likely((*regs).sr & BIT(6) != 0) {
        local_irq_enable();
    }

    if unlikely(faulthandler_disabled() || mm.is_null()) {
        no_context(regs, addr);
        return;
    }

    if user_mode(regs) {
        flags |= FAULT_FLAG_USER;
    }

    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, addr);

    if is_write(regs) {
        flags |= FAULT_FLAG_WRITE;
    }

    'retry: loop {
        vma = lock_mm_and_find_vma(mm, addr, regs);
        if unlikely(vma.is_null()) {
            bad_area_nosemaphore(regs, mm, code, addr);
            return;
        }

        code = SEGV_ACCERR;

        if unlikely(access_error(regs, vma)) {
            mmap_read_unlock(mm);
            bad_area_nosemaphore(regs, mm, code, addr);
            return;
        }

        fault = handle_mm_fault(vma, addr, flags, regs);

        if fault_signal_pending(fault, regs) {
            if !user_mode(regs) {
                no_context(regs, addr);
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

        if unlikely(fault & VM_FAULT_ERROR != 0) {
            mm_fault_error(regs, addr, fault);
            return;
        }
        return;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
