// SPDX-License-Identifier: GPL-2.0-only
/*
 * Memory fault handling for Hexagon
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

/*
 * Page fault handling for the Hexagon Virtual Machine.
 * Can also be called by a native port emulating the HVM
 * execptions.
 */

const FLT_IFETCH: libc::c_long = -1;
const FLT_LOAD: libc::c_long = 0;
const FLT_STORE: libc::c_long = 1;

/*
 * Canonical page fault handler
 */
unsafe fn do_page_fault(address: libc::c_ulong, cause: libc::c_long, regs: *mut pt_regs) {
    let mut vma: *mut vm_area_struct;
    let mm: *mut mm_struct = (*current).mm;
    let mut si_signo: libc::c_int;
    let mut si_code: libc::c_int = SEGV_MAPERR;
    let mut fault: vm_fault_t;
    let fixup: *const exception_table_entry;
    let mut flags: libc::c_uint = FAULT_FLAG_DEFAULT;

    /*
     * If we're in an interrupt or have no user context,
     * then must not take the fault.
     */
    if in_interrupt() || mm.is_null() {
        goto_no_context(address, regs, &mut fixup);
        return;
    }

    local_irq_enable();

    if user_mode(regs) != 0 {
        flags |= FAULT_FLAG_USER;
    }

    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, address);

    loop {
        vma = lock_mm_and_find_vma(mm, address, regs);
        if vma.is_null() {
            mmap_bad_area_nosemaphore(regs, si_code, address, &mut fixup);
            return;
        }

        /* Address space is OK.  Now check access rights. */
        si_code = SEGV_ACCERR;

        match cause {
            FLT_IFETCH => {
                if (*vma).vm_flags & VM_EXEC == 0 {
                    mmap_bad_area(mm, regs, si_code, address, &mut fixup);
                    return;
                }
            }
            FLT_LOAD => {
                if (*vma).vm_flags & VM_READ == 0 {
                    mmap_bad_area(mm, regs, si_code, address, &mut fixup);
                    return;
                }
            }
            FLT_STORE => {
                if (*vma).vm_flags & VM_WRITE == 0 {
                    mmap_bad_area(mm, regs, si_code, address, &mut fixup);
                    return;
                }
                flags |= FAULT_FLAG_WRITE;
            }
            _ => {}
        }

        fault = handle_mm_fault(vma, address, flags, regs);

        if fault_signal_pending(fault, regs) != 0 {
            if user_mode(regs) == 0 {
                goto_no_context(address, regs, &mut fixup);
            }
            return;
        }

        /* The fault is fully completed (including releasing mmap lock) */
        if fault & VM_FAULT_COMPLETED != 0 {
            return;
        }

        /* The most common case -- we are done. */
        if fault & VM_FAULT_ERROR == 0 {
            if fault & VM_FAULT_RETRY != 0 {
                flags |= FAULT_FLAG_TRIED;
                continue;
            }
            mmap_read_unlock(mm);
            return;
        }

        mmap_read_unlock(mm);

        /* Handle copyin/out exception cases */
        if user_mode(regs) == 0 {
            goto_no_context(address, regs, &mut fixup);
            return;
        }

        if fault & VM_FAULT_OOM != 0 {
            pagefault_out_of_memory();
            return;
        }

        /* User-mode address is in the memory map, but we are
         * unable to fix up the page fault.
         */
        if fault & VM_FAULT_SIGBUS != 0 {
            si_signo = SIGBUS;
            si_code = BUS_ADRERR;
        } else {
            /* Address is not in the memory map */
            si_signo = SIGSEGV;
            si_code = SEGV_ACCERR;
        }
        force_sig_fault(si_signo, si_code, address as *mut libc::c_void);
        return;
    }
}

unsafe fn mmap_bad_area(
    mm: *mut mm_struct, regs: *mut pt_regs, si_code: libc::c_int,
    address: libc::c_ulong, fixup: &mut *const exception_table_entry,
) {
    mmap_read_unlock(mm);
    mmap_bad_area_nosemaphore(regs, si_code, address, fixup);
}

unsafe fn mmap_bad_area_nosemaphore(
    regs: *mut pt_regs, si_code: libc::c_int, address: libc::c_ulong,
    fixup: &mut *const exception_table_entry,
) {
    if user_mode(regs) != 0 {
        force_sig_fault(SIGSEGV, si_code, address as *mut libc::c_void);
        return;
    }
    goto_no_context(address, regs, fixup);
}

unsafe fn goto_no_context(
    address: libc::c_ulong, regs: *mut pt_regs,
    fixup: &mut *const exception_table_entry,
) {
    *fixup = search_exception_tables(pt_elr(regs));
    if !(*fixup).is_null() {
        pt_set_elr(regs, (**fixup).fixup);
        return;
    }
    bust_spinlocks(1);
    printk(KERN_EMERG, b"Unable to handle kernel paging request at virtual address 0x%08lx, regs %p\0".as_ptr(), address, regs);
    die(b"Bad Kernel VA\0".as_ptr(), regs, SIGKILL);
}

pub unsafe fn read_protection_fault(regs: *mut pt_regs) {
    let badvadr: libc::c_ulong = pt_badva(regs);
    do_page_fault(badvadr, FLT_LOAD, regs);
}

pub unsafe fn write_protection_fault(regs: *mut pt_regs) {
    let badvadr: libc::c_ulong = pt_badva(regs);
    do_page_fault(badvadr, FLT_STORE, regs);
}

pub unsafe fn execute_protection_fault(regs: *mut pt_regs) {
    let badvadr: libc::c_ulong = pt_badva(regs);
    do_page_fault(badvadr, FLT_IFETCH, regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
