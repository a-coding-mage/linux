/*
 * Copyright (C) 2009 Wind River Systems Inc
 *   Implemented by fredrik.markstrom@gmail.com and ivarholmqvist@gmail.com
 *
 * based on arch/mips/mm/fault.c which is:
 *
 * Copyright (C) 1995-2000 Ralf Baechle
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies are supplied by the surrounding kernel translation unit.

const EXC_SUPERV_INSN_ACCESS: c_int = 9;
const EXC_SUPERV_DATA_ACCESS: c_int = 11;
const EXC_X_PROTECTION_FAULT: c_int = 13;
const EXC_R_PROTECTION_FAULT: c_int = 14;
const EXC_W_PROTECTION_FAULT: c_int = 15;

/*
 * This routine handles page faults.  It determines the address,
 * and the problem, and then passes it off to one of the appropriate
 * routines.
 */
pub unsafe fn do_page_fault(
    regs: *mut pt_regs,
    mut cause: c_ulong,
    address: c_ulong,
) {
    let mut vma: *mut vm_area_struct = core::ptr::null_mut();
    let tsk: *mut task_struct = current;
    let mm: *mut mm_struct = (*tsk).mm;
    let mut code: c_int = SEGV_MAPERR;
    let mut fault: vm_fault_t;
    let mut flags: c_uint = FAULT_FLAG_DEFAULT;

    cause >>= 2;

    /* Restart the instruction */
    (*regs).ea = (*regs).ea.wrapping_sub(4);

    /*
     * We fault-in kernel-space virtual memory on-demand. The
     * 'reference' page table is init_mm.pgd.
     *
     * NOTE! We MUST NOT take any locks for this case. We may
     * be in an interrupt or a critical region, and should
     * only copy the information from the master page table,
     * nothing more.
     */
    if unlikely(address >= VMALLOC_START && address <= VMALLOC_END) {
        if user_mode(regs) {
            goto_bad_area_nosemaphore!(regs, address, code);
        } else {
            goto_vmalloc_fault!(regs, address, cause, mm, tsk);
        }
    }

    if unlikely(address >= TASK_SIZE) {
        goto_bad_area_nosemaphore!(regs, address, code);
    }

    /*
     * If we're in an interrupt or have no user
     * context, we must not take the fault..
     */
    if faulthandler_disabled() || mm.is_null() {
        goto_bad_area_nosemaphore!(regs, address, code);
    }

    if user_mode(regs) {
        flags |= FAULT_FLAG_USER;
    }

    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, address);

    'retry: loop {
        vma = lock_mm_and_find_vma(mm, address, regs);
        if vma.is_null() {
            goto_bad_area_nosemaphore!(regs, address, code);
        }

        /*
         * Ok, we have a good vm_area for this memory access, so
         * we can handle it..
         */
        code = SEGV_ACCERR;

        match cause as c_int {
            EXC_SUPERV_INSN_ACCESS | EXC_SUPERV_DATA_ACCESS => {
                goto_bad_area!(regs, address, code, mm);
            }
            EXC_X_PROTECTION_FAULT => {
                if (*vma).vm_flags & VM_EXEC == 0 {
                    goto_bad_area!(regs, address, code, mm);
                }
            }
            EXC_R_PROTECTION_FAULT => {
                if (*vma).vm_flags & VM_READ == 0 {
                    goto_bad_area!(regs, address, code, mm);
                }
            }
            EXC_W_PROTECTION_FAULT => {
                if (*vma).vm_flags & VM_WRITE == 0 {
                    goto_bad_area!(regs, address, code, mm);
                }
                flags = FAULT_FLAG_WRITE;
            }
            _ => {}
        }

        /*
         * If for any reason at all we couldn't handle the fault,
         * make sure we exit gracefully rather than endlessly redo
         * the fault.
         */
        fault = handle_mm_fault(vma, address, flags, regs);

        if fault_signal_pending(fault, regs) {
            if !user_mode(regs) {
                goto_no_context!(regs, address, cause, mm);
            }
            return;
        }

        /* The fault is fully completed (including releasing mmap lock) */
        if fault & VM_FAULT_COMPLETED != 0 {
            return;
        }

        if unlikely(fault & VM_FAULT_ERROR != 0) {
            if fault & VM_FAULT_OOM != 0 {
                goto_out_of_memory!(regs, address, cause, mm);
            } else if fault & VM_FAULT_SIGSEGV != 0 {
                goto_bad_area!(regs, address, code, mm);
            } else if fault & VM_FAULT_SIGBUS != 0 {
                goto_do_sigbus!(regs, address, mm);
            }
            BUG!();
        }

        if fault & VM_FAULT_RETRY != 0 {
            flags |= FAULT_FLAG_TRIED;
            continue 'retry;
        }

        mmap_read_unlock(mm);
        return;
    }
}

/*
 * The C implementation's bad_area, bad_area_nosemaphore, no_context,
 * out_of_memory, do_sigbus, and vmalloc_fault labels are represented by
 * the corresponding control-flow targets above.  Their operations remain
 * below in the same order as the original source.
 */

unsafe fn vmalloc_fault(address: c_ulong, regs: *mut pt_regs) {
    let offset: c_int = pgd_index(address);
    let pgd: *mut pgd_t = pgd_current.add(offset as usize);
    let pgd_k: *mut pgd_t = init_mm.pgd.add(offset as usize);
    if !pgd_present(*pgd_k) {
        if fixup_exception(regs) { return; }
        panic("Oops");
    }
    set_pgd(pgd, *pgd_k);

    let p4d = p4d_offset(pgd, address);
    let p4d_k = p4d_offset(pgd_k, address);
    if !p4d_present(*p4d_k) { if fixup_exception(regs) { return; } panic("Oops"); }
    let pud = pud_offset(p4d, address);
    let pud_k = pud_offset(p4d_k, address);
    if !pud_present(*pud_k) { if fixup_exception(regs) { return; } panic("Oops"); }
    let pmd = pmd_offset(pud, address);
    let pmd_k = pmd_offset(pud_k, address);
    if !pmd_present(*pmd_k) { if fixup_exception(regs) { return; } panic("Oops"); }
    set_pmd(pmd, *pmd_k);

    let pte_k = pte_offset_kernel(pmd_k, address);
    if !pte_present(*pte_k) { if fixup_exception(regs) { return; } panic("Oops"); }
    flush_tlb_kernel_page(address);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
