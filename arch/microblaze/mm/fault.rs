/*
 *  arch/microblaze/mm/fault.c
 *
 *    Copyright (C) 2007 Xilinx, Inc.  All rights reserved.
 *
 *  Derived from "arch/ppc/mm/fault.c"
 *    Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
 *
 *  Derived from "arch/i386/mm/fault.c"
 *    Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
 *
 *  Modified by Cort Dougan and Paul Mackerras.
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file COPYING in the main directory of this
 * archive for more details.
 */

// C dependencies: linux/extable.h, linux/signal.h, linux/sched.h,
// linux/kernel.h, linux/errno.h, linux/string.h, linux/types.h,
// linux/ptrace.h, linux/mman.h, linux/mm.h, linux/interrupt.h,
// linux/perf_event.h, asm/page.h, asm/mmu.h, linux/mmu_context.h,
// linux/uaccess.h, asm/exceptions.h

static mut pte_misses: c_ulong = 0; // updated by do_page_fault()
static mut pte_errors: c_ulong = 0; // updated by do_page_fault()

/* Check whether the instruction at regs->pc is a store using
 * an update addressing form which will update r1. */
unsafe fn store_updates_sp(regs: *mut pt_regs) -> c_int {
    let mut inst: c_uint = 0;

    if get_user(&mut inst as *mut c_uint, regs_mut_pc(regs) as *const c_uint) != 0 {
        return 0;
    }
    // check for 1 in the rD field
    if ((inst >> 21) & 0x1f) != 1 {
        return 0;
    }
    // check for store opcodes
    if (inst & 0xd0000000) == 0xd0000000 {
        return 1;
    }
    0
}

/*
 * bad_page_fault is called when we have a bad access from the kernel.
 * It is called from do_page_fault above and from some of the procedures
 * in traps.c.
 */
pub unsafe fn bad_page_fault(regs: *mut pt_regs, address: c_ulong, sig: c_int) {
    let fixup: *const exception_table_entry;
    // MS: no context
    // Are we prepared to handle this fault?
    fixup = search_exception_tables((*regs).pc);
    if !fixup.is_null() {
        (*regs).pc = (*fixup).fixup;
        return;
    }

    // kernel has accessed a bad area
    die(c_str!("kernel access of bad area"), regs, sig);
}

/*
 * The error_code parameter is ESR for a data fault,
 * 0 for an instruction fault.
 */
pub unsafe fn do_page_fault(regs: *mut pt_regs, address: c_ulong, error_code: c_ulong) {
    let mut vma: *mut vm_area_struct;
    let mm: *mut mm_struct = (*current).mm;
    let mut code: c_int = SEGV_MAPERR;
    let mut is_write = error_code & ESR_S;
    let fault: vm_fault_t;
    let mut flags: c_uint = FAULT_FLAG_DEFAULT;

    macro_rules! goto_bad_area_nosemaphore {
        () => {{
            pte_errors += 1;
            if user_mode(regs) { _exception(SIGSEGV, regs, code, address); return; }
            bad_page_fault(regs, address, SIGSEGV); return;
        }};
    }
    macro_rules! goto_bad_area {
        () => {{ mmap_read_unlock(mm); goto_bad_area_nosemaphore!(); }};
    }
    macro_rules! goto_out_of_memory {
        () => {{
            mmap_read_unlock(mm);
            if !user_mode(regs) { bad_page_fault(regs, address, SIGKILL); }
            else { pagefault_out_of_memory(); }
            return;
        }};
    }
    macro_rules! goto_do_sigbus {
        () => {{
            mmap_read_unlock(mm);
            if user_mode(regs) {
                force_sig_fault(SIGBUS, BUS_ADRERR, address as *mut c_void);
                return;
            }
            bad_page_fault(regs, address, SIGBUS); return;
        }};
    }

    (*regs).ear = address;
    (*regs).esr = error_code;

    // On a kernel SLB miss we can only check for a valid exception entry
    if unlikely(kernel_mode(regs) && address >= TASK_SIZE) {
        pr_warn(c_str!("kernel task_size exceed"));
        _exception(SIGSEGV, regs, code, address);
    }

    // for instr TLB miss and instr storage exception ESR_S is undefined
    if (error_code & 0x13) == 0x13 || (error_code & 0x11) == 0x11 {
        is_write = 0;
    }

    if unlikely(faulthandler_disabled() || mm.is_null()) {
        if kernel_mode(regs) {
            goto_bad_area_nosemaphore!();
        }
        pr_emerg(c_str!("Page fault in user mode with faulthandler_disabled(), mm = %p\n"), mm);
        pr_emerg(c_str!("r15 = %lx  MSR = %lx\n"), (*regs).r15, (*regs).msr);
        die(c_str!("Weird page fault"), regs, SIGSEGV);
    }

    if user_mode(regs) { flags |= FAULT_FLAG_USER; }
    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, address);

    if unlikely(!mmap_read_trylock(mm)) {
        if kernel_mode(regs) && search_exception_tables((*regs).pc).is_null() {
            goto_bad_area_nosemaphore!();
        }
        mmap_read_lock(mm);
    }

    'retry: loop {
        vma = find_vma(mm, address);
        if unlikely(vma.is_null()) { goto_bad_area!(); }
        if (*vma).vm_start <= address { break; }
        if unlikely((*vma).vm_flags & VM_GROWSDOWN == 0) { goto_bad_area!(); }
        if unlikely(is_write == 0) { goto_bad_area!(); }

        if unlikely(address.wrapping_add(0x100000) < (*vma).vm_end) {
            let uregs = (*current).thread.regs;
            if uregs.is_null() { goto_bad_area!(); }
            if address.wrapping_add(2048) < (*uregs).r1
                && (kernel_mode(regs) || store_updates_sp(regs) == 0) { goto_bad_area!(); }
        }
        vma = expand_stack(mm, address);
        if vma.is_null() { goto_bad_area_nosemaphore!(); }
        break;

        // Labels below are represented by the direct exits in the C control flow.
    }

    code = SEGV_ACCERR;
    if unlikely(is_write != 0) {
        if unlikely((*vma).vm_flags & VM_WRITE == 0) { goto_bad_area!(); }
        flags |= FAULT_FLAG_WRITE;
    } else {
        if unlikely(error_code & 0x08000000 != 0) { goto_bad_area!(); }
        if unlikely((*vma).vm_flags & (VM_READ | VM_EXEC) == 0) { goto_bad_area!(); }
    }

    fault = handle_mm_fault(vma, address, flags, regs);
    if fault_signal_pending(fault, regs) {
        if !user_mode(regs) { bad_page_fault(regs, address, SIGBUS); }
        return;
    }
    if fault & VM_FAULT_COMPLETED != 0 { return; }
    if unlikely(fault & VM_FAULT_ERROR != 0) {
        if fault & VM_FAULT_OOM != 0 { goto_out_of_memory!(); }
        else if fault & VM_FAULT_SIGSEGV != 0 { goto_bad_area!(); }
        else if fault & VM_FAULT_SIGBUS != 0 { goto_do_sigbus!(); }
        BUG();
    }
    if fault & VM_FAULT_RETRY != 0 {
        flags |= FAULT_FLAG_TRIED;
        continue 'retry;
    }
    mmap_read_unlock(mm);
    pte_misses += 1;
    return;

}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
