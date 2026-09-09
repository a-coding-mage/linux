// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/m68k/mm/fault.c
 *
 *  Copyright (C) 1995  Hamish Macdonald
 */

// Dependencies are supplied by the surrounding kernel translation unit.

extern "C" {
    fn die_if_kernel(str: *mut i8, regs: *mut pt_regs, error_code: i64);
}

#[repr(C)]
pub struct pt_regs {
    pub sr: u32,
    pub pc: usize,
}

#[repr(C)]
pub struct thread_struct {
    pub signo: i32,
    pub code: i32,
    pub faddr: usize,
}

#[repr(C)]
pub struct mm_struct {
    pub pgd: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct task_struct {
    pub mm: *mut mm_struct,
    pub thread: thread_struct,
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_start: usize,
    pub vm_flags: usize,
}

extern "C" {
    static mut current: *mut task_struct;
}

pub unsafe fn send_fault_sig(regs: *mut pt_regs) -> i32 {
    let signo: i32 = (*current).thread.signo;
    let si_code: i32 = (*current).thread.code;
    let addr = (*current).thread.faddr as *mut core::ffi::c_void;
    pr_debug_send_fault_sig(addr, signo, si_code);

    if user_mode(regs) {
        force_sig_fault(signo, si_code, addr);
    } else {
        if fixup_exception(regs) {
            return -1;
        }

        /*
         * Oops. The kernel tried to access some bad page. We'll have to
         * terminate things with extreme prejudice.
         */
        if addr as usize < PAGE_SIZE {
            pr_alert_null_pointer();
        } else {
            pr_alert_kernel_access();
        }
        pr_cont_virtual_address(addr);
        die_if_kernel(b"Oops\0".as_ptr() as *mut i8, regs, 0 /*error_code*/);
        make_task_dead(SIGKILL);
    }

    1
}

/*
 * This routine handles page faults.  It determines the problem, and
 * then passes it off to one of the appropriate routines.
 *
 * error_code:
 *\tbit 0 == 0 means no page found, 1 means protection fault
 *\tbit 1 == 0 means read, 1 means write
 *
 * If this routine detects a bad access, it returns 1, otherwise it
 * returns 0.
 */
pub unsafe fn do_page_fault(
    regs: *mut pt_regs,
    address: usize,
    error_code: usize,
) -> i32 {
    let mm = (*current).mm;
    let mut vma: *mut vm_area_struct;
    let mut fault: u32;
    let mut flags: u32 = FAULT_FLAG_DEFAULT;

    pr_debug_page_fault((*regs).sr, (*regs).pc, address, error_code, if mm.is_null() { core::ptr::null_mut() } else { (*mm).pgd });

    if faulthandler_disabled() || mm.is_null() {
        goto_no_context: {
            (*current).thread.signo = SIGBUS;
            (*current).thread.faddr = address;
            return send_fault_sig(regs);
        }
    }

    if user_mode(regs) {
        flags |= FAULT_FLAG_USER;
    }

    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, address);

    'retry: loop {
        mmap_read_lock(mm);
        vma = find_vma(mm, address);
        if vma.is_null() {
            mmap_read_unlock(mm);
            (*current).thread.signo = SIGSEGV;
            (*current).thread.code = SEGV_MAPERR;
            (*current).thread.faddr = address;
            return send_fault_sig(regs);
        }
        if (*vma).vm_start > address && (*vma).vm_flags & VM_GROWSDOWN == 0 {
            mmap_read_unlock(mm);
            (*current).thread.signo = SIGSEGV;
            (*current).thread.code = SEGV_MAPERR;
            (*current).thread.faddr = address;
            return send_fault_sig(regs);
        }
        if (*vma).vm_start > address && user_mode(regs) && address.wrapping_add(256) < rdusp() {
            mmap_read_unlock(mm);
            (*current).thread.signo = SIGSEGV;
            (*current).thread.code = SEGV_MAPERR;
            (*current).thread.faddr = address;
            return send_fault_sig(regs);
        }
        if (*vma).vm_start > address {
            vma = expand_stack(mm, address);
            if vma.is_null() {
                (*current).thread.signo = SIGSEGV;
                (*current).thread.code = SEGV_MAPERR;
                (*current).thread.faddr = address;
                return send_fault_sig(regs);
            }
        }

        match error_code & 3 {
            2 | 3 => {
                if (*vma).vm_flags & VM_WRITE == 0 { goto acc_err; }
                flags |= FAULT_FLAG_WRITE;
            }
            1 => { goto acc_err; }
            0 => { if !vma_is_accessible(vma) { goto acc_err; } }
            _ => unreachable!(),
        }

        fault = handle_mm_fault(vma, address, flags, regs);
        if fault_signal_pending(fault, regs) {
            if !user_mode(regs) { goto no_context; }
            return 0;
        }
        if fault & VM_FAULT_COMPLETED != 0 { return 0; }
        if fault & VM_FAULT_ERROR != 0 {
            if fault & VM_FAULT_OOM != 0 { mmap_read_unlock(mm); if !user_mode(regs) { goto no_context; } pagefault_out_of_memory(); return 0; }
            if fault & VM_FAULT_SIGSEGV != 0 { goto map_err; }
            if fault & VM_FAULT_SIGBUS != 0 { goto bus_err; }
            BUG();
        }
        if fault & VM_FAULT_RETRY != 0 {
            flags |= FAULT_FLAG_TRIED;
            continue 'retry;
        }
        mmap_read_unlock(mm);
        return 0;

        no_context: {
            (*current).thread.signo = SIGBUS;
            (*current).thread.faddr = address;
            return send_fault_sig(regs);
        }
        bus_err: {
            (*current).thread.signo = SIGBUS; (*current).thread.code = BUS_ADRERR; (*current).thread.faddr = address;
            mmap_read_unlock(mm); return send_fault_sig(regs);
        }
        map_err: {
            mmap_read_unlock(mm);
            (*current).thread.signo = SIGSEGV; (*current).thread.code = SEGV_MAPERR; (*current).thread.faddr = address;
            return send_fault_sig(regs);
        }
        acc_err: {
            (*current).thread.signo = SIGSEGV; (*current).thread.code = SEGV_ACCERR; (*current).thread.faddr = address;
            mmap_read_unlock(mm); return send_fault_sig(regs);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
