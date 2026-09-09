// SPDX-License-Identifier: GPL-2.0
/*
 *  S390 version
 *    Copyright IBM Corp. 1999
 *    Author(s): Hartmut Penner (hp@de.ibm.com)
 *               Ulrich Weigand (uweigand@de.ibm.com)
 *
 *  Derived from "arch/i386/mm/fault.c"
 *    Copyright (C) 1995  Linus Torvalds
 */

// Kernel and architecture dependencies are supplied by other translation units.

unsafe fn is_kernel_fault(regs: *mut pt_regs) -> bool {
    let teid = teid { val: (*regs).int_parm_long };
    if user_mode(regs) { return false; }
    if teid.as_ == PSW_BITS_AS_SECONDARY { return false; }
    true
}

unsafe fn get_fault_address(regs: *mut pt_regs) -> c_ulong {
    let teid = teid { val: (*regs).int_parm_long };
    teid.addr * PAGE_SIZE
}

#[inline(always)]
unsafe fn fault_is_write(regs: *mut pt_regs) -> bool {
    let teid = teid { val: (*regs).int_parm_long };
    if test_facility(75) { return teid.fsi == TEID_FSI_STORE; }
    false
}

unsafe fn dump_pagetable(asce: c_ulong, address: c_ulong) {
    let mut entry: c_ulong;
    let mut table = __va(asce & _ASCE_ORIGIN) as *mut c_ulong;
    pr_alert!("AS:%016lx ", asce);
    match asce & _ASCE_TYPE_MASK {
        _ASCE_TYPE_REGION1 => {
            table = table.add(((address & _REGION1_INDEX) >> _REGION1_SHIFT) as usize);
            if get_kernel_nofault(&mut entry, table) != 0 { pr_cont!("BAD\n"); return; }
            pr_cont!("R1:%016lx ", entry);
            if entry & _REGION_ENTRY_INVALID != 0 { pr_cont!("\n"); return; }
            table = __va(entry & _REGION_ENTRY_ORIGIN) as *mut c_ulong;
        }
        _ASCE_TYPE_REGION2 => {}
        _ => {}
    }
    if (asce & _ASCE_TYPE_MASK) == _ASCE_TYPE_REGION1 || (asce & _ASCE_TYPE_MASK) == _ASCE_TYPE_REGION2 {
        table = table.add(((address & _REGION2_INDEX) >> _REGION2_SHIFT) as usize);
        if get_kernel_nofault(&mut entry, table) != 0 { pr_cont!("BAD\n"); return; }
        pr_cont!("R2:%016lx ", entry);
        if entry & _REGION_ENTRY_INVALID != 0 { pr_cont!("\n"); return; }
        table = __va(entry & _REGION_ENTRY_ORIGIN) as *mut c_ulong;
    }
    if (asce & _ASCE_TYPE_MASK) <= _ASCE_TYPE_REGION3 {
        table = table.add(((address & _REGION3_INDEX) >> _REGION3_SHIFT) as usize);
        if get_kernel_nofault(&mut entry, table) != 0 { pr_cont!("BAD\n"); return; }
        pr_cont!("R3:%016lx ", entry);
        if entry & (_REGION_ENTRY_INVALID | _REGION3_ENTRY_LARGE) != 0 { pr_cont!("\n"); return; }
        table = __va(entry & _REGION_ENTRY_ORIGIN) as *mut c_ulong;
    }
    table = table.add(((address & _SEGMENT_INDEX) >> _SEGMENT_SHIFT) as usize);
    if get_kernel_nofault(&mut entry, table) != 0 { pr_cont!("BAD\n"); return; }
    pr_cont!("S:%016lx ", entry);
    if entry & (_SEGMENT_ENTRY_INVALID | _SEGMENT_ENTRY_LARGE) != 0 { pr_cont!("\n"); return; }
    table = __va(entry & _SEGMENT_ENTRY_ORIGIN) as *mut c_ulong;
    table = table.add(((address & _PAGE_INDEX) >> PAGE_SHIFT) as usize);
    if get_kernel_nofault(&mut entry, table) != 0 { pr_cont!("BAD\n"); return; }
    pr_cont!("P:%016lx ", entry);
    pr_cont!("\n");
}

unsafe fn dump_fault_info(regs: *mut pt_regs) {
    let teid = teid { val: (*regs).int_parm_long };
    let asce;
    pr_alert!("Failing address: %016lx TEID: %016lx", get_fault_address(regs), teid.val);
    if test_facility(131) { pr_cont!(" ESOP-2"); } else if machine_has_esop() { pr_cont!(" ESOP-1"); } else { pr_cont!(" SOP"); }
    if test_facility(75) { pr_cont!(" FSI"); }
    pr_cont!("\n");
    pr_alert!("Fault in ");
    match teid.as_ { PSW_BITS_AS_HOME => pr_cont!("home space "), PSW_BITS_AS_SECONDARY => pr_cont!("secondary space "), PSW_BITS_AS_ACCREG => pr_cont!("access register "), PSW_BITS_AS_PRIMARY => pr_cont!("primary space "), _ => {} }
    pr_cont!("mode while using ");
    if is_kernel_fault(regs) { asce = get_lowcore().kernel_asce.val; pr_cont!("kernel "); } else { asce = get_lowcore().user_asce.val; pr_cont!("user "); }
    pr_cont!("ASCE.\n");
    dump_pagetable(asce, get_fault_address(regs));
}

static mut show_unhandled_signals: c_int = 1;

static s390_fault_sysctl_table: [ctl_table; 2] = [
    ctl_table {
        procname: c_str!("userprocess_debug"),
        data: unsafe { &mut show_unhandled_signals as *mut c_int as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];

unsafe fn init_s390_fault_sysctls() -> c_int {
    register_sysctl_init(c_str!("kernel"), s390_fault_sysctl_table);
    0
}

arch_initcall!(init_s390_fault_sysctls);

pub unsafe fn report_user_fault(regs: *mut pt_regs, signr: c_long, is_mm_fault: c_int) {
    static mut rs: ratelimit_state = DEFINE_RATELIMIT_STATE!(DEFAULT_RATELIMIT_INTERVAL, DEFAULT_RATELIMIT_BURST);
    if task_pid_nr(current) > 1 && !show_unhandled_signals { return; }
    if !unhandled_signal(current, signr) || !__ratelimit(&mut rs) { return; }
    pr_alert!("User process fault: interruption code %04x ilc:%d ", (*regs).int_code & 0xffff, (*regs).int_code >> 17);
    print_vma_addr(KERN_CONT, c_str!("in "), (*regs).psw.addr);
    pr_cont!("\n");
    if is_mm_fault != 0 { dump_fault_info(regs); }
    show_regs(regs);
}

unsafe fn do_sigsegv(regs: *mut pt_regs, si_code: c_int) {
    report_user_fault(regs, SIGSEGV, 1);
    force_sig_fault(SIGSEGV, si_code, get_fault_address(regs) as *mut c_void);
}

unsafe fn handle_fault_error_nolock(regs: *mut pt_regs, mut si_code: c_int) {
    let address; let is_write;
    if user_mode(regs) { if WARN_ON_ONCE(si_code == 0) { si_code = SEGV_MAPERR; } return do_sigsegv(regs, si_code); }
    if fixup_exception(regs) { return; }
    if is_kernel_fault(regs) { address = get_fault_address(regs); is_write = fault_is_write(regs); if kfence_handle_page_fault(address, is_write, regs) { return; } pr_alert!("Unable to handle kernel pointer dereference in virtual kernel address space\n"); } else { pr_alert!("Unable to handle kernel paging request in virtual user address space\n"); }
    dump_fault_info(regs); die(regs, c_str!("Oops"));
}

unsafe fn handle_fault_error(regs: *mut pt_regs, si_code: c_int) { let mm = current.mm; mmap_read_unlock(mm); handle_fault_error_nolock(regs, si_code); }

unsafe fn do_sigbus(regs: *mut pt_regs) { force_sig_fault(SIGBUS, BUS_ADRERR, get_fault_address(regs) as *mut c_void); }

/* Page-fault handling, including address-space lookup, fault resolution, and signal/error paths. */
unsafe fn do_exception(regs: *mut pt_regs, mut access: c_int) {
    let mut vma: *mut vm_area_struct; let address = get_fault_address(regs); let mm = current.mm; let mut flags; let fault; let is_write = fault_is_write(regs);
    clear_thread_flag(TIF_PER_TRAP); if kprobe_page_fault(regs, 14) { return; }
    if is_kernel_fault(regs) || faulthandler_disabled() || mm.is_null() { return handle_fault_error_nolock(regs, 0); }
    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, address); flags = FAULT_FLAG_DEFAULT;
    if user_mode(regs) { flags |= FAULT_FLAG_USER; } if is_write { access = VM_WRITE; } if access == VM_WRITE { flags |= FAULT_FLAG_WRITE; }
    if flags & FAULT_FLAG_USER == 0 { goto_lock_mmap(mm, regs, address, flags, access); return; }
    vma = lock_vma_under_rcu(mm, address); if vma.is_null() { goto_lock_mmap(mm, regs, address, flags, access); return; }
    if (*vma).vm_flags & access == 0 { vma_end_read(vma); count_vm_vma_lock_event(VMA_LOCK_SUCCESS); return handle_fault_error_nolock(regs, SEGV_ACCERR); }
    fault = handle_mm_fault(vma, address, flags | FAULT_FLAG_VMA_LOCK, regs); if fault & (VM_FAULT_RETRY | VM_FAULT_COMPLETED) == 0 { vma_end_read(vma); }
    if fault & VM_FAULT_RETRY == 0 { count_vm_vma_lock_event(VMA_LOCK_SUCCESS); } else { count_vm_vma_lock_event(VMA_LOCK_RETRY); if fault & VM_FAULT_MAJOR != 0 { flags |= FAULT_FLAG_TRIED; } if fault_signal_pending(fault, regs) { if !user_mode(regs) { handle_fault_error_nolock(regs, 0); } return; } goto_lock_mmap(mm, regs, address, flags, access); return; }
    if fault & VM_FAULT_ERROR != 0 { if fault & VM_FAULT_OOM != 0 { if !user_mode(regs) { handle_fault_error_nolock(regs, 0); } else { pagefault_out_of_memory(); } } else if fault & VM_FAULT_SIGSEGV != 0 { if !user_mode(regs) { handle_fault_error_nolock(regs, 0); } else { do_sigsegv(regs, SEGV_MAPERR); } } else if fault & (VM_FAULT_SIGBUS | VM_FAULT_HWPOISON | VM_FAULT_HWPOISON_LARGE) != 0 { if !user_mode(regs) { handle_fault_error_nolock(regs, 0); } else { do_sigbus(regs); } } else { pr_emerg!("Unexpected fault flags: %08x\n", fault); BUG!(); } }
}

unsafe fn goto_lock_mmap(mm: *mut mm_struct, regs: *mut pt_regs, address: c_ulong, mut flags: c_ulong, access: c_int) {
    let mut vma; let fault;
    loop { vma = lock_mm_and_find_vma(mm, address, regs); if vma.is_null() { return handle_fault_error_nolock(regs, SEGV_MAPERR); } if (*vma).vm_flags & access == 0 { return handle_fault_error(regs, SEGV_ACCERR); } fault = handle_mm_fault(vma, address, flags, regs); if fault_signal_pending(fault, regs) { if !user_mode(regs) { handle_fault_error_nolock(regs, 0); } return; } if fault & VM_FAULT_COMPLETED != 0 { return; } if fault & VM_FAULT_RETRY == 0 { break; } flags |= FAULT_FLAG_TRIED; }
    mmap_read_unlock(mm); if fault & VM_FAULT_ERROR == 0 { return; }
    if fault & VM_FAULT_OOM != 0 { if !user_mode(regs) { handle_fault_error_nolock(regs, 0); } else { pagefault_out_of_memory(); } } else if fault & VM_FAULT_SIGSEGV != 0 { if !user_mode(regs) { handle_fault_error_nolock(regs, 0); } else { do_sigsegv(regs, SEGV_MAPERR); } } else if fault & (VM_FAULT_SIGBUS | VM_FAULT_HWPOISON | VM_FAULT_HWPOISON_LARGE) != 0 { if !user_mode(regs) { handle_fault_error_nolock(regs, 0); } else { do_sigbus(regs); } } else { pr_emerg!("Unexpected fault flags: %08x\n", fault); BUG!(); }
}

pub unsafe fn do_protection_exception(regs: *mut pt_regs) { let teid = teid { val: (*regs).int_parm_long }; if (*regs).int_code & 0x200 == 0 { (*regs).psw.addr = __rewind_psw((*regs).psw, (*regs).int_code >> 16); set_pt_regs_flag(regs, PIF_PSW_ADDR_ADJUSTED); } if unlikely(!teid.b61) { if user_mode(regs) { dump_fault_info(regs); die(regs, c_str!("Unexpected TEID")); } return handle_fault_error_nolock(regs, 0); } if unlikely(cpu_has_nx() && teid.b56) { (*regs).int_parm_long = teid.addr * PAGE_SIZE | ((*regs).psw.addr & PAGE_MASK); return handle_fault_error_nolock(regs, SEGV_ACCERR); } do_exception(regs, VM_WRITE); }

pub unsafe fn do_dat_exception(regs: *mut pt_regs) { do_exception(regs, VM_ACCESS_FLAGS); }

// CONFIG_KVM conditional: retained as source intent; dependent declarations are external.
#[cfg(CONFIG_KVM)]
pub unsafe fn do_secure_storage_access(regs: *mut pt_regs) {
    let teid = teid { val: (*regs).int_parm_long }; let addr = get_fault_address(regs); let mm = current.mm;
    if uv_has_feature(BIT_UV_FEAT_MISC) && !teid.b61 { if user_mode(regs) { return handle_fault_error_nolock(regs, SEGV_ACCERR); } panic!("Unexpected PGM 0x3d with TEID bit 61=0"); }
    if is_kernel_fault(regs) { if is_vmalloc_addr(addr as *mut c_void) { return handle_fault_error_nolock(regs, 0); } if uv_convert_from_secure(__pa(addr)) != 0 { return handle_fault_error_nolock(regs, 0); } } else if faulthandler_disabled() || mm.is_null() { return handle_fault_error_nolock(regs, 0); } else { let vma = lock_mm_and_find_vma(mm, addr, regs); if vma.is_null() { return handle_fault_error_nolock(regs, SEGV_MAPERR); } let fw: folio_walk = core::mem::zeroed(); let folio = folio_walk_start(&fw, vma, addr, 0); if !folio.is_null() { folio_get(folio); let rc = arch_make_folio_accessible(folio); folio_put(folio); folio_walk_end(&fw, vma); if rc != 0 { return handle_fault_error(regs, SEGV_ACCERR); } } mmap_read_unlock(mm); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
