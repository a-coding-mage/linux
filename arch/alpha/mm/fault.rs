// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/alpha/mm/fault.c
 *
 *  Copyright (C) 1995  Linus Torvalds
 */

// C headers omitted; the referenced kernel types, constants, and functions
// are supplied by the surrounding kernel translation.

extern "C" {
    fn die_if_kernel(name: *mut u8, regs: *mut pt_regs, cause: i64, address: *mut u64);
}

#[cfg(not(CONFIG_SMP))]
static mut last_asn: u64 = ASN_FIRST_VERSION;

pub unsafe extern "C" fn __load_new_mm_context(next_mm: *mut mm_struct) {
    let mmc: u64;
    let pcb: *mut pcb_struct;

    mmc = __get_new_mm_context(next_mm, smp_processor_id());
    (*next_mm).context[smp_processor_id() as usize] = mmc;

    pcb = &mut (*current_thread_info()).pcb;
    (*pcb).asn = mmc & HARDWARE_ASN_MASK;
    (*pcb).ptbr = (((*next_mm).pgd as u64) - IDENT_ADDR) >> PAGE_SHIFT;

    __reload_thread(pcb);
}

/* Macro for exception fixup code to access integer registers. */
unsafe fn dpf_reg(regs: *mut pt_regs, r: usize) -> u64 {
    let index = if r <= 8 { r } else if r <= 15 { r - 17 } else if r <= 18 { r + 11 } else { r - 10 };
    *((regs as *mut u64).add(index))
}

pub unsafe extern "C" fn do_page_fault(
    address: u64,
    mmcsr: u64,
    cause: i64,
    regs: *mut pt_regs,
) {
    let mut vma: *mut vm_area_struct;
    let mm: *mut mm_struct = (*current).mm;
    let mut fixup: *const exception_table_entry;
    let mut si_code: i32 = SEGV_MAPERR;
    let mut fault: vm_fault_t;
    let mut flags: u32 = FAULT_FLAG_DEFAULT;

    /* As of EV6, a load into $31/$f31 is a prefetch, and never faults. */
    if cause == 0 {
        let mut insn: u32 = 0;
        __get_user(&mut insn, (*regs).pc as *const u32);
        if ((insn >> 21) & 0x1f) == 0x1f
            && ((1u64 << (insn >> 26)) & 0x30f00001400u64) != 0
        {
            (*regs).pc += 4;
            return;
        }
    }

    /* If we're in an interrupt context, or have no user context, do not fault. */
    if mm.is_null() || faulthandler_disabled() {
        goto_no_context(address, cause, regs);
        return;
    }

    #[cfg(CONFIG_ALPHA_LARGE_VMALLOC)]
    if address >= TASK_SIZE {
        vmalloc_fault(address, cause, regs, si_code);
        return;
    }
    if user_mode(regs) {
        flags |= FAULT_FLAG_USER;
    }
    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, address);

    'retry: loop {
        vma = lock_mm_and_find_vma(mm, address, regs);
        if vma.is_null() {
            if user_mode(regs) { force_sig_fault(SIGSEGV, si_code, address as *mut u8); return; }
            goto_no_context(address, cause, regs); return;
        }
        si_code = SEGV_ACCERR;
        if cause < 0 {
            if (*vma).vm_flags & VM_EXEC == 0 { mmap_read_unlock(mm); if user_mode(regs) { force_sig_fault(SIGSEGV, si_code, address as *mut u8); return; } goto_no_context(address, cause, regs); return; }
        } else if cause == 0 {
            if (*vma).vm_flags & (VM_READ | VM_WRITE) == 0 { mmap_read_unlock(mm); if user_mode(regs) { force_sig_fault(SIGSEGV, si_code, address as *mut u8); return; } goto_no_context(address, cause, regs); return; }
        } else {
            if (*vma).vm_flags & VM_WRITE == 0 { mmap_read_unlock(mm); if user_mode(regs) { force_sig_fault(SIGSEGV, si_code, address as *mut u8); return; } goto_no_context(address, cause, regs); return; }
            flags |= FAULT_FLAG_WRITE;
        }
        fault = handle_mm_fault(vma, address, flags, regs);
        if fault_signal_pending(fault, regs) { if !user_mode(regs) { goto_no_context(address, cause, regs); } return; }
        if fault & VM_FAULT_COMPLETED != 0 { return; }
        if fault & VM_FAULT_ERROR != 0 {
            if fault & VM_FAULT_OOM != 0 { mmap_read_unlock(mm); if !user_mode(regs) { goto_no_context(address, cause, regs); } pagefault_out_of_memory(); return; }
            if fault & VM_FAULT_SIGSEGV != 0 { mmap_read_unlock(mm); if user_mode(regs) { force_sig_fault(SIGSEGV, si_code, address as *mut u8); return; } goto_no_context(address, cause, regs); return; }
            if fault & VM_FAULT_SIGBUS != 0 { mmap_read_unlock(mm); force_sig_fault(SIGBUS, BUS_ADRERR, address as *mut u8); if !user_mode(regs) { goto_no_context(address, cause, regs); } return; }
            BUG();
        }
        if fault & VM_FAULT_RETRY != 0 { flags |= FAULT_FLAG_TRIED; continue 'retry; }
        mmap_read_unlock(mm);
        return;
    }
}

unsafe fn goto_no_context(address: u64, cause: i64, regs: *mut pt_regs) {
    let fixup = search_exception_tables((*regs).pc);
    if !fixup.is_null() {
        (*regs).pc = fixup_exception(dpf_reg, fixup, (*regs).pc, regs);
        return;
    }
    printk(KERN_ALERT, "Unable to handle kernel paging request at virtual address %016lx\n", address);
    die_if_kernel("Oops\0".as_ptr() as *mut u8, regs, cause, regs.offset(-16) as *mut u64);
    make_task_dead(SIGKILL);
}

#[cfg(CONFIG_ALPHA_LARGE_VMALLOC)]
unsafe fn vmalloc_fault(address: u64, cause: i64, regs: *mut pt_regs, si_code: i32) {
    if user_mode(regs) {
        force_sig_fault(SIGSEGV, si_code, address as *mut u8);
        return;
    }
    /* Synchronize this task's top level page-table with init's reference table. */
    let index: isize = pgd_index(address);
    let pgd = (*(*current).active_mm).pgd.offset(index);
    let pgd_k = swapper_pg_dir.offset(index);
    if !pgd_present(*pgd) && pgd_present(*pgd_k) {
        pgd_val(*pgd) = pgd_val(*pgd_k);
        return;
    }
    goto_no_context(address, cause, regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
