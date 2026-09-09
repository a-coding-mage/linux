/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995 - 2000 by Ralf Baechle
 */

// Linux and MIPS dependencies are supplied by other translation units.

pub static mut show_unhandled_signals: i32 = 1;

/*
 * This routine handles page faults.  It determines the address,
 * and the problem, and then passes it off to one of the appropriate
 * routines.
 */
unsafe fn __do_page_fault(regs: *mut pt_regs, write: c_ulong, address: c_ulong) {
    let mut vma: *mut vm_area_struct = core::ptr::null_mut();
    let tsk: *mut task_struct = current;
    let mm: *mut mm_struct = (*tsk).mm;
    let field: i32 = (core::mem::size_of::<c_ulong>() * 2) as i32;
    let mut si_code: i32;
    let mut fault: vm_fault_t;
    let mut flags: u32 = FAULT_FLAG_DEFAULT;

    // static DEFINE_RATELIMIT_STATE(ratelimit_state, 5 * HZ, 10);
    let mut ratelimit_state = DEFINE_RATELIMIT_STATE!(5 * HZ, 10);

    // #if 0: diagnostic printk intentionally disabled in the original source.

    // CONFIG_KPROBES conditionally notifies the kprobes fault handler.
    #[cfg(CONFIG_KPROBES)]
    {
        if notify_die(DIE_PAGE_FAULT, "page fault", regs, -1,
                      (*current).thread.trap_nr, SIGSEGV) == NOTIFY_STOP {
            return;
        }
    }

    si_code = SEGV_MAPERR;

    // CONFIG_64BIT selects no_context; otherwise the target is vmalloc_fault.
    if unlikely(address >= VMALLOC_START && address <= VMALLOC_END) {
        #[cfg(CONFIG_64BIT)]
        goto!(no_context);
        #[cfg(not(CONFIG_64BIT))]
        goto!(vmalloc_fault);
    }
    // MODULES_VADDR is a build-time conditional in the original source.
    #[cfg(MODULES_VADDR)]
    if unlikely(address >= MODULES_VADDR && address < MODULES_END) {
        #[cfg(CONFIG_64BIT)]
        goto!(no_context);
        #[cfg(not(CONFIG_64BIT))]
        goto!(vmalloc_fault);
    }

    if faulthandler_disabled() || mm.is_null() {
        goto!(bad_area_nosemaphore);
    }

    if user_mode(regs) {
        flags |= FAULT_FLAG_USER;
    }

    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, address);
retry:
    vma = lock_mm_and_find_vma(mm, address, regs);
    if vma.is_null() {
        goto!(bad_area_nosemaphore);
    }

    si_code = SEGV_ACCERR;

    if write != 0 {
        if (*vma).vm_flags & VM_WRITE == 0 {
            goto!(bad_area);
        }
        flags |= FAULT_FLAG_WRITE;
    } else if cpu_has_rixi {
        if address == (*regs).cp0_epc && (*vma).vm_flags & VM_EXEC == 0 {
            goto!(bad_area);
        }
        if (*vma).vm_flags & VM_READ == 0 && exception_epc(regs) != address {
            goto!(bad_area);
        }
    } else if unlikely(!vma_is_accessible(vma)) {
        goto!(bad_area);
    }

    fault = handle_mm_fault(vma, address, flags, regs);
    if fault_signal_pending(fault, regs) {
        if !user_mode(regs) {
            goto!(no_context);
        }
        return;
    }
    if fault & VM_FAULT_COMPLETED != 0 {
        return;
    }
    if unlikely(fault & VM_FAULT_ERROR != 0) {
        if fault & VM_FAULT_OOM != 0 {
            goto!(out_of_memory);
        } else if fault & VM_FAULT_SIGSEGV != 0 {
            goto!(bad_area);
        } else if fault & VM_FAULT_SIGBUS != 0 {
            goto!(do_sigbus);
        }
        BUG!();
    }
    if fault & VM_FAULT_RETRY != 0 {
        flags |= FAULT_FLAG_TRIED;
        goto!(retry);
    }
    mmap_read_unlock(mm);
    return;

bad_area:
    mmap_read_unlock(mm);
bad_area_nosemaphore:
    if user_mode(regs) {
        (*tsk).thread.cp0_badvaddr = address;
        (*tsk).thread.error_code = write;
        if show_unhandled_signals != 0 && unhandled_signal(tsk, SIGSEGV)
            && __ratelimit(&mut ratelimit_state) {
            pr_info!("do_page_fault(): sending SIGSEGV to %s for invalid %s %0*lx\n",
                     (*tsk).comm, if write != 0 { "write access to" } else { "read access from" },
                     field, address);
            pr_info!("epc = %0*lx in", field, (*regs).cp0_epc as c_ulong);
            print_vma_addr!(KERN_CONT " ", (*regs).cp0_epc);
            pr_cont!("\n");
            pr_info!("ra  = %0*lx in", field, (*regs).regs[31]);
            print_vma_addr!(KERN_CONT " ", (*regs).regs[31]);
            pr_cont!("\n");
        }
        (*current).thread.trap_nr = ((*regs).cp0_cause >> 2) & 0x1f;
        force_sig_fault(SIGSEGV, si_code, address as *mut core::ffi::c_void);
        return;
    }

no_context:
    if fixup_exception(regs) {
        (*current).thread.cp0_baduaddr = address;
        return;
    }
    bust_spinlocks(1);
    printk!(KERN_ALERT "CPU %d Unable to handle kernel paging request at virtual address %0*lx, epc == %0*lx, ra == %0*lx\n",
            raw_smp_processor_id(), field, address, field, (*regs).cp0_epc,
            field, (*regs).regs[31]);
    die("Oops", regs);

out_of_memory:
    mmap_read_unlock(mm);
    if !user_mode(regs) {
        goto!(no_context);
    }
    pagefault_out_of_memory();
    return;

do_sigbus:
    mmap_read_unlock(mm);
    if !user_mode(regs) {
        goto!(no_context);
    }
    (*current).thread.trap_nr = ((*regs).cp0_cause >> 2) & 0x1f;
    (*tsk).thread.cp0_badvaddr = address;
    force_sig_fault(SIGBUS, BUS_ADRERR, address as *mut core::ffi::c_void);
    return;

// #ifndef CONFIG_64BIT
vmalloc_fault:
    {
        let offset = pgd_index(address);
        let mut pgd: *mut pgd_t;
        let mut pgd_k: *mut pgd_t;
        let mut p4d: *mut p4d_t;
        let mut p4d_k: *mut p4d_t;
        let mut pud: *mut pud_t;
        let mut pud_k: *mut pud_t;
        let mut pmd: *mut pmd_t;
        let mut pmd_k: *mut pmd_t;
        let pte_k: *mut pte_t;

        pgd = pgd_current[raw_smp_processor_id()] .add(offset as usize);
        pgd_k = init_mm.pgd.add(offset as usize);
        if !pgd_present(*pgd_k) { goto!(no_context); }
        set_pgd(pgd, *pgd_k);
        p4d = p4d_offset(pgd, address);
        p4d_k = p4d_offset(pgd_k, address);
        if !p4d_present(*p4d_k) { goto!(no_context); }
        pud = pud_offset(p4d, address);
        pud_k = pud_offset(p4d_k, address);
        if !pud_present(*pud_k) { goto!(no_context); }
        pmd = pmd_offset(pud, address);
        pmd_k = pmd_offset(pud_k, address);
        if !pmd_present(*pmd_k) { goto!(no_context); }
        set_pmd(pmd, *pmd_k);
        pte_k = pte_offset_kernel(pmd_k, address);
        if !pte_present(*pte_k) { goto!(no_context); }
        return;
    }
// #endif
}

pub unsafe fn do_page_fault(regs: *mut pt_regs, write: c_ulong, address: c_ulong) {
    let prev_state: ctx_state = exception_enter();
    __do_page_fault(regs, write, address);
    exception_exit(prev_state);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
