// SPDX-License-Identifier: GPL-2.0-or-later
/* PowerPC version; translated from fault.c. */

// C headers and configuration-provided symbols are supplied by the surrounding kernel bindings.

unsafe fn __bad_area_nosemaphore(regs: *mut pt_regs, address: ulong, si_code: c_int) -> c_int {
    if !user_mode(regs) { return SIGSEGV; }
    _exception(SIGSEGV, regs, si_code, address);
    0
}

unsafe fn bad_area_nosemaphore(regs: *mut pt_regs, address: ulong) -> c_int {
    __bad_area_nosemaphore(regs, address, SEGV_MAPERR)
}

unsafe fn __bad_area(regs: *mut pt_regs, address: ulong, si_code: c_int,
                     mm: *mut mm_struct, vma: *mut vm_area_struct) -> c_int {
    if !mm.is_null() { mmap_read_unlock(mm); } else { vma_end_read(vma); }
    __bad_area_nosemaphore(regs, address, si_code)
}

unsafe fn bad_access_pkey(regs: *mut pt_regs, address: ulong,
                          mm: *mut mm_struct, vma: *mut vm_area_struct) -> c_int {
    let pkey = vma_pkey(vma);
    if !mm.is_null() { mmap_read_unlock(mm); } else { vma_end_read(vma); }
    if !user_mode(regs) { return SIGSEGV; }
    _exception_pkey(regs, address, pkey);
    0
}

unsafe fn bad_access(regs: *mut pt_regs, address: ulong, mm: *mut mm_struct,
                     vma: *mut vm_area_struct) -> c_int {
    __bad_area(regs, address, SEGV_ACCERR, mm, vma)
}

unsafe fn do_sigbus(regs: *mut pt_regs, address: ulong, fault: vm_fault_t) -> c_int {
    if !user_mode(regs) { return SIGBUS; }
    (*(*current).thread.trap_nr) = BUS_ADRERR;
    #[cfg(CONFIG_MEMORY_FAILURE)]
    {
        if fault & (VM_FAULT_HWPOISON | VM_FAULT_HWPOISON_LARGE) != 0 {
            let mut lsb: uint = 0;
            pr_err!("MCE: Killing {}:{} due to hardware memory corruption fault at {:x}\n", (*current).comm, (*current).pid, address);
            if fault & VM_FAULT_HWPOISON_LARGE != 0 { lsb = hstate_index_to_shift(VM_FAULT_GET_HINDEX(fault)); }
            if fault & VM_FAULT_HWPOISON != 0 { lsb = PAGE_SHIFT; }
            force_sig_mceerr(BUS_MCEERR_AR, address as *mut c_void, lsb);
            return 0;
        }
    }
    force_sig_fault(SIGBUS, BUS_ADRERR, address as *mut c_void);
    0
}

unsafe fn mm_fault_error(regs: *mut pt_regs, addr: ulong, fault: vm_fault_t) -> c_int {
    if fatal_signal_pending(current) && !user_mode(regs) { return SIGKILL; }
    if fault & VM_FAULT_OOM != 0 {
        if !user_mode(regs) { return SIGSEGV; }
        pagefault_out_of_memory();
    } else if fault & (VM_FAULT_SIGBUS | VM_FAULT_HWPOISON | VM_FAULT_HWPOISON_LARGE) != 0 {
        return do_sigbus(regs, addr, fault);
    } else if fault & VM_FAULT_SIGSEGV != 0 { return bad_area_nosemaphore(regs, addr); }
    else { BUG!(); }
    0
}

unsafe fn bad_kernel_fault(regs: *mut pt_regs, _error_code: ulong, address: ulong, is_write: bool) -> bool {
    let is_exec = TRAP(regs) == INTERRUPT_INST_STORAGE;
    if is_exec {
        pr_crit_ratelimited!("kernel tried to execute {} page ({:x}) - exploit attempt? (uid: {})\n", if address >= TASK_SIZE { "exec-protected" } else { "user" }, address, from_kuid(&init_user_ns, current_uid()));
        return true;
    }
    if address >= TASK_SIZE { return true; }
    if bad_kuap_fault(regs, address, is_write) {
        pr_crit_ratelimited!("Kernel attempted to {} user page ({:x}) - exploit attempt? (uid: {})\n", str_write_read(is_write), address, from_kuid(&init_user_ns, current_uid()));
        if !search_exception_tables((*regs).nip) { return true; }
        return WARN!(true, "Bug: {} fault blocked by KUAP!", if is_write { "Write" } else { "Read" });
    }
    false
}

unsafe fn access_pkey_error(is_write: bool, is_exec: bool, is_pkey: bool,
                            vma: *mut vm_area_struct) -> bool {
    !arch_vma_access_permitted(vma, is_write, is_exec, 0)
}

unsafe fn access_error(is_write: bool, is_exec: bool, vma: *mut vm_area_struct) -> bool {
    if is_exec { return !((*vma).vm_flags & VM_EXEC != 0) && (cpu_has_feature(CPU_FTR_NOEXECUTE) || (*vma).vm_flags & (VM_READ | VM_WRITE) == 0); }
    if is_write { return unlikely!((*vma).vm_flags & VM_WRITE == 0); }
    if unlikely!(!vma_is_accessible(vma)) { return true; }
    (*vma).vm_flags & VM_ACCESS_FLAGS == VM_EXEC
}

#[cfg(CONFIG_PPC_SMLPAR)]
unsafe fn cmo_account_page_fault() {
    if firmware_has_feature(FW_FEATURE_CMO) {
        let mut page_ins: u32;
        preempt_disable();
        page_ins = be32_to_cpu((*get_lppaca()).page_ins);
        page_ins += 1 << PAGE_FACTOR;
        (*get_lppaca()).page_ins = cpu_to_be32(page_ins);
        preempt_enable();
    }
}
#[cfg(not(CONFIG_PPC_SMLPAR))]
unsafe fn cmo_account_page_fault() {}

unsafe fn sanity_check_fault(_is_write: bool, is_user: bool, error_code: ulong, address: ulong) {
    if is_user && address >= TASK_SIZE {
        if address as long == -1 { return; }
        pr_crit_ratelimited!("{}[{}]: User access of kernel address ({:x}) - exploit attempt? (uid: {})\n", (*current).comm, (*current).pid, address, from_kuid(&init_user_ns, current_uid()));
        return;
    }
    if !IS_ENABLED!(CONFIG_PPC_BOOK3S) { return; }
    if radix_enabled() || _is_write { return; }
    WARN_ON_ONCE!(error_code & DSISR_PROTFAULT != 0);
}

#[cfg(CONFIG_BOOKE)]
#[inline] unsafe fn page_fault_is_write(err: ulong) -> ulong { err & ESR_DST }
#[cfg(not(CONFIG_BOOKE))]
#[inline] unsafe fn page_fault_is_write(err: ulong) -> ulong { err & DSISR_ISSTORE }

#[cfg(CONFIG_BOOKE)]
unsafe fn page_fault_is_bad(_err: ulong) -> ulong { 0 }
#[cfg(all(not(CONFIG_BOOKE), CONFIG_PPC_8xx))]
unsafe fn page_fault_is_bad(err: ulong) -> ulong { err & DSISR_NOEXEC_OR_G }
#[cfg(all(not(CONFIG_BOOKE), not(CONFIG_PPC_8xx), CONFIG_PPC64))]
unsafe fn page_fault_is_bad(err: ulong) -> ulong {
    let mut flag = DSISR_BAD_FAULT_64S;
    if mmu_has_feature(MMU_FTR_NX_DSI) { flag &= !DSISR_BAD_COPYPASTE; }
    err & flag
}
#[cfg(all(not(CONFIG_BOOKE), not(CONFIG_PPC_8xx), not(CONFIG_PPC64)))]
unsafe fn page_fault_is_bad(err: ulong) -> ulong { err & DSISR_BAD_FAULT_32S }

// The following routines retain the C handler's externally supplied kernel types and symbols.
unsafe fn ___do_page_fault(regs: *mut pt_regs, address: ulong, error_code: ulong) -> c_int {
    let mut vma: *mut vm_area_struct;
    let mm = (*current).mm;
    let mut flags = FAULT_FLAG_DEFAULT;
    let is_exec = TRAP(regs) == INTERRUPT_INST_STORAGE;
    let is_user = user_mode(regs);
    let is_write = page_fault_is_write(error_code) != 0;
    let mut major: vm_fault_t = 0;
    let mut fault: vm_fault_t;
    let kprobe_fault = kprobe_page_fault(regs, 11);
    if unlikely!(debugger_fault_handler(regs) || kprobe_fault) { return 0; }
    if unlikely!(page_fault_is_bad(error_code) != 0) {
        if is_user { _exception(SIGBUS, regs, BUS_OBJERR, address); return 0; }
        return SIGBUS;
    }
    sanity_check_fault(is_write, is_user, error_code, address);
    if unlikely!(!is_user && bad_kernel_fault(regs, error_code, address, is_write)) {
        if is_kfence_address(address as *mut c_void) && !search_exception_tables(instruction_pointer(regs)) && kfence_handle_page_fault(address, is_write, regs) { return 0; }
        return SIGSEGV;
    }
    if unlikely!(faulthandler_disabled() || mm.is_null()) {
        if is_user { printk_ratelimited!(KERN_ERR "Page fault in user mode with faulthandler_disabled()={} mm={:p}\n", faulthandler_disabled(), mm); }
        return bad_area_nosemaphore(regs, address);
    }
    interrupt_cond_local_irq_enable(regs);
    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, address);
    if is_user { flags |= FAULT_FLAG_USER; }
    if is_write { flags |= FAULT_FLAG_WRITE; }
    if is_exec { flags |= FAULT_FLAG_INSTRUCTION; }
    if flags & FAULT_FLAG_USER != 0 {
        vma = lock_vma_under_rcu(mm, address);
        if !vma.is_null() {
            if unlikely!(access_pkey_error(is_write, is_exec, (error_code & DSISR_KEYFAULT) != 0, vma)) { count_vm_vma_lock_event(VMA_LOCK_SUCCESS); return bad_access_pkey(regs, address, core::ptr::null_mut(), vma); }
            if unlikely!(access_error(is_write, is_exec, vma)) { count_vm_vma_lock_event(VMA_LOCK_SUCCESS); return bad_access(regs, address, core::ptr::null_mut(), vma); }
            fault = handle_mm_fault(vma, address, flags | FAULT_FLAG_VMA_LOCK, regs);
            if fault & (VM_FAULT_RETRY | VM_FAULT_COMPLETED) == 0 { vma_end_read(vma); }
            if fault & VM_FAULT_RETRY == 0 { count_vm_vma_lock_event(VMA_LOCK_SUCCESS); goto done; }
            count_vm_vma_lock_event(VMA_LOCK_RETRY);
            if fault & VM_FAULT_MAJOR != 0 { flags |= FAULT_FLAG_TRIED; }
            if fault_signal_pending(fault, regs) { return if user_mode(regs) { 0 } else { SIGBUS }; }
        }
    }
    'retry: loop {
        vma = lock_mm_and_find_vma(mm, address, regs);
        if vma.is_null() { return bad_area_nosemaphore(regs, address); }
        if unlikely!(access_pkey_error(is_write, is_exec, (error_code & DSISR_KEYFAULT) != 0, vma)) { return bad_access_pkey(regs, address, mm, vma); }
        if unlikely!(access_error(is_write, is_exec, vma)) { return bad_access(regs, address, mm, vma); }
        fault = handle_mm_fault(vma, address, flags, regs);
        major |= fault & VM_FAULT_MAJOR;
        if fault_signal_pending(fault, regs) { return if user_mode(regs) { 0 } else { SIGBUS }; }
        if fault & VM_FAULT_COMPLETED != 0 { break; }
        if unlikely!(fault & VM_FAULT_RETRY != 0) { flags |= FAULT_FLAG_TRIED; continue 'retry; }
        mmap_read_unlock((*current).mm);
        break;
    }
    if unlikely!(fault & VM_FAULT_ERROR != 0) { return mm_fault_error(regs, address, fault); }
    if major != 0 { cmo_account_page_fault(); }
    return 0;
    'done: {
        if unlikely!(fault & VM_FAULT_ERROR != 0) { return mm_fault_error(regs, address, fault); }
        if major != 0 { cmo_account_page_fault(); }
        0
    }
}

unsafe fn __do_page_fault(regs: *mut pt_regs) {
    let err = ___do_page_fault(regs, (*regs).dar, (*regs).dsisr);
    if unlikely!(err != 0) { bad_page_fault(regs, err); }
}

pub unsafe fn do_page_fault(regs: *mut pt_regs) { __do_page_fault(regs); }

#[cfg(CONFIG_PPC_BOOK3S_64)]
pub unsafe fn hash__do_page_fault(regs: *mut pt_regs) { __do_page_fault(regs); }

unsafe fn __bad_page_fault(regs: *mut pt_regs, sig: c_int) {
    let is_write = page_fault_is_write((*regs).dsisr) != 0;
    let msg = if (*regs).dar < PAGE_SIZE { "Kernel NULL pointer dereference" } else { "Unable to handle kernel data access" };
    match TRAP(regs) {
        INTERRUPT_DATA_STORAGE | INTERRUPT_H_DATA_STORAGE => pr_alert!("BUG: {} on {} at 0x{:08x}\n", msg, str_write_read(is_write), (*regs).dar),
        INTERRUPT_DATA_SEGMENT => pr_alert!("BUG: {} at 0x{:08x}\n", msg, (*regs).dar),
        INTERRUPT_INST_STORAGE | INTERRUPT_INST_SEGMENT => pr_alert!("BUG: Unable to handle kernel instruction fetch{}", if (*regs).nip < PAGE_SIZE { " (NULL pointer?)\n" } else { "\n" }),
        INTERRUPT_ALIGNMENT => pr_alert!("BUG: Unable to handle kernel unaligned access at 0x{:08x}\n", (*regs).dar),
        _ => pr_alert!("BUG: Unable to handle unknown paging fault at 0x{:08x}\n", (*regs).dar),
    }
    printk!(KERN_ALERT "Faulting instruction address: 0x{:08x}\n", (*regs).nip);
    if task_stack_end_corrupted(current) { printk!(KERN_ALERT "Thread overran stack, or stack corrupted\n"); }
    die("Kernel access of bad area", regs, sig);
}

pub unsafe fn bad_page_fault(regs: *mut pt_regs, sig: c_int) {
    let entry = search_exception_tables(instruction_pointer(regs));
    if !entry.is_null() { instruction_pointer_set(regs, extable_fixup(entry)); } else { __bad_page_fault(regs, sig); }
}

#[cfg(CONFIG_PPC_BOOK3S_64)]
pub unsafe fn do_bad_page_fault_segv(regs: *mut pt_regs) { bad_page_fault(regs, SIGSEGV); }

#[cfg(CONFIG_PPC_BOOK3S_64)]
pub unsafe fn do_bad_segment_interrupt(regs: *mut pt_regs) {
    let err = (*regs).result;
    if err == -EFAULT { if user_mode(regs) { _exception(SIGSEGV, regs, SEGV_BNDERR, (*regs).dar); } else { bad_page_fault(regs, SIGSEGV); } }
    else if err == -EINVAL { unrecoverable_exception(regs); } else { BUG!(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
