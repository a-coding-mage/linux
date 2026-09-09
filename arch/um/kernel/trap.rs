// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

/*
 * NOTE: UML does not have exception tables. As such, this is almost a copy
 * of the code in mm/memory.c, only adjusting the logic to simply check whether
 * we are coming from the kernel instead of doing an additional lookup in the
 * exception table.
 * We can do this simplification because we never get here if the exception was
 * fixable.
 */
unsafe fn get_mmap_lock_carefully(mm: *mut mm_struct, is_user: bool) -> bool {
    if likely(mmap_read_trylock(mm)) {
        return true;
    }

    if !is_user {
        return false;
    }

    !mmap_read_lock_killable(mm)
}

unsafe fn mmap_upgrade_trylock(_mm: *mut mm_struct) -> bool {
    /*
     * We don't have this operation yet.
     *
     * It should be easy enough to do: it's basically a
     *    atomic_long_try_cmpxchg_acquire()
     * from RWSEM_READER_BIAS -> RWSEM_WRITER_LOCKED, but
     * it also needs the proper lockdep magic etc.
     */
    false
}

unsafe fn upgrade_mmap_lock_carefully(mm: *mut mm_struct, is_user: bool) -> bool {
    mmap_read_unlock(mm);
    if !is_user {
        return false;
    }

    !mmap_write_lock_killable(mm)
}

/*
 * Helper for page fault handling.
 *
 * This is kind of equivalend to "mmap_read_lock()" followed
 * by "find_extend_vma()", except it's a lot more careful about
 * the locking (and will drop the lock on failure).
 *
 * For example, if we have a kernel bug that causes a page
 * fault, we don't want to just use mmap_read_lock() to get
 * the mm lock, because that would deadlock if the bug were
 * to happen while we're holding the mm lock for writing.
 *
 * So this checks the exception tables on kernel faults in
 * order to only do this all for instructions that are actually
 * expected to fault.
 *
 * We can also actually take the mm lock for writing if we
 * need to extend the vma, which helps the VM layer a lot.
 */
unsafe fn um_lock_mm_and_find_vma(
    mm: *mut mm_struct,
    addr: c_ulong,
    is_user: bool,
) -> *mut vm_area_struct {
    let mut vma: *mut vm_area_struct;

    if !get_mmap_lock_carefully(mm, is_user) {
        return core::ptr::null_mut();
    }

    vma = find_vma(mm, addr);
    if likely(!vma.is_null() && (*vma).vm_start <= addr) {
        return vma;
    }

    /*
     * Well, dang. We might still be successful, but only
     * if we can extend a vma to do so.
     */
    if vma.is_null() || ((*vma).vm_flags & VM_GROWSDOWN) == 0 {
        mmap_read_unlock(mm);
        return core::ptr::null_mut();
    }

    /*
     * We can try to upgrade the mmap lock atomically,
     * in which case we can continue to use the vma
     * we already looked up.
     *
     * Otherwise we'll have to drop the mmap lock and
     * re-take it, and also look up the vma again,
     * re-checking it.
     */
    if !mmap_upgrade_trylock(mm) {
        if !upgrade_mmap_lock_carefully(mm, is_user) {
            return core::ptr::null_mut();
        }

        vma = find_vma(mm, addr);
        if vma.is_null() {
            mmap_write_unlock(mm);
            return core::ptr::null_mut();
        }
        if (*vma).vm_start <= addr {
            mmap_write_downgrade(mm);
            return vma;
        }
        if ((*vma).vm_flags & VM_GROWSDOWN) == 0 {
            mmap_write_unlock(mm);
            return core::ptr::null_mut();
        }
    }

    if expand_stack_locked(vma, addr) != 0 {
        mmap_write_unlock(mm);
        return core::ptr::null_mut();
    }

    mmap_write_downgrade(mm);
    vma
}

/*
 * Note this is constrained to return 0, -EFAULT, -EACCES, -ENOMEM by
 * segv().
 */
pub unsafe fn handle_page_fault(
    address: c_ulong,
    _ip: c_ulong,
    is_write: c_int,
    is_user: c_int,
    code_out: *mut c_int,
) -> c_int {
    let mm = (*current).mm;
    let mut pmd: *mut pmd_t;
    let mut pte: *mut pte_t;
    let mut err: c_int = -EFAULT;
    let mut flags: c_uint = FAULT_FLAG_DEFAULT;

    *code_out = SEGV_MAPERR;

    /* If the fault was with pagefaults disabled, don't take the fault, just fail. */
    if faulthandler_disabled() {
        return err;
    }

    if is_user != 0 {
        flags |= FAULT_FLAG_USER;
    }
retry:
    let vma = um_lock_mm_and_find_vma(mm, address, is_user != 0);
    if vma.is_null() {
        return err;
    }

    *code_out = SEGV_ACCERR;
    if is_write != 0 {
        if ((*vma).vm_flags & VM_WRITE) == 0 {
            mmap_read_unlock(mm);
            return err;
        }
        flags |= FAULT_FLAG_WRITE;
    } else if ((*vma).vm_flags & (VM_READ | VM_EXEC)) == 0 {
        mmap_read_unlock(mm);
        return err;
    }

    loop {
        let fault = handle_mm_fault(vma, address, flags, core::ptr::null_mut());

        if (fault & VM_FAULT_RETRY) != 0 && fatal_signal_pending(current) {
            return err;
        }
        if (fault & VM_FAULT_COMPLETED) != 0 {
            return 0;
        }
        if unlikely((fault & VM_FAULT_ERROR) != 0) {
            if (fault & VM_FAULT_OOM) != 0 {
                mmap_read_unlock(mm);
                if is_user == 0 {
                    return err;
                }
                pagefault_out_of_memory();
                return 0;
            } else if (fault & VM_FAULT_SIGSEGV) != 0 {
                mmap_read_unlock(mm);
                return err;
            } else if (fault & VM_FAULT_SIGBUS) != 0 {
                err = -EACCES;
                mmap_read_unlock(mm);
                return err;
            }
            BUG();
        }
        if (fault & VM_FAULT_RETRY) != 0 {
            flags |= FAULT_FLAG_TRIED;
            continue 'retry;
        }

        pmd = pmd_off(mm, address);
        pte = pte_offset_kernel(pmd, address);
        if pte_present(*pte) {
            break;
        }
    }
    err = 0;
    mmap_read_unlock(mm);
    err
}

unsafe fn show_segv_info(regs: *mut uml_pt_regs) {
    let tsk = current;
    let fi = UPT_FAULTINFO(regs);

    if !unhandled_signal(tsk, SIGSEGV) || !printk_ratelimit() {
        return;
    }

    printk(
        "%s%s[%d]: segfault at %lx ip %px sp %px error %x",
        if task_pid_nr(tsk) > 1 { KERN_INFO } else { KERN_EMERG },
        (*tsk).comm,
        task_pid_nr(tsk),
        FAULT_ADDRESS(*fi),
        UPT_IP(regs) as *mut core::ffi::c_void,
        UPT_SP(regs) as *mut core::ffi::c_void,
        (*fi).error_code,
    );
    print_vma_addr(KERN_CONT " in ", UPT_IP(regs));
    printk(KERN_CONT "\n");
}

unsafe fn bad_segv(fi: faultinfo, _ip: c_ulong) {
    (*current).thread.arch.faultinfo = fi;
    force_sig_fault(SIGSEGV, SEGV_ACCERR, FAULT_ADDRESS(fi) as *mut core::ffi::c_void);
}

pub unsafe fn fatal_sigsegv() {
    force_fatal_sig(SIGSEGV);
    do_signal(&mut (*current).thread.regs);
    /* This is to tell gcc that we're not returning - do_signal can, in general, return. */
    os_dump_core();
}

pub unsafe fn segv_handler(
    _sig: c_int,
    _unused_si: *mut siginfo,
    regs: *mut uml_pt_regs,
    mc: *mut core::ffi::c_void,
) {
    let fi = UPT_FAULTINFO(regs);
    if UPT_IS_USER(regs) && !SEGV_IS_FIXABLE(fi) {
        show_segv_info(regs);
        bad_segv(*fi, UPT_IP(regs));
        return;
    }
    segv(*fi, UPT_IP(regs), UPT_IS_USER(regs) as c_int, regs, mc);
}

pub unsafe fn segv(
    fi: faultinfo,
    ip: c_ulong,
    is_user: c_int,
    regs: *mut uml_pt_regs,
    mc: *mut core::ffi::c_void,
) -> c_ulong {
    let mut si_code: c_int = 0;
    let mut err: c_int;
    let is_write = FAULT_WRITE(fi);
    let mut address = FAULT_ADDRESS(fi);

    if is_user == 0 && !regs.is_null() {
        (*current).thread.segv_regs = container_of(regs, pt_regs, regs);
    }

    if is_user == 0 && address >= start_vm && address < end_vm {
        err = um_tlb_sync(&mut init_mm);
        if err == -ENOMEM { report_enomem(); }
        if err != 0 { panic!("Failed to sync kernel TLBs: %d", err); }
        return 0;
    } else if (*current).pagefault_disabled != 0 {
        if mc.is_null() { show_regs(container_of(regs, pt_regs, regs)); panic!("Segfault with pagefaults disabled but no mcontext"); }
        if (*current).thread.segv_continue.is_null() { show_regs(container_of(regs, pt_regs, regs)); panic!("Segfault without recovery target"); }
        mc_set_rip(mc, (*current).thread.segv_continue);
        (*current).thread.segv_continue = core::ptr::null_mut();
        return 0;
    } else if (*current).mm.is_null() {
        show_regs(container_of(regs, pt_regs, regs));
        panic!("Segfault with no mm");
    } else if is_user == 0 && address > PAGE_SIZE && address < TASK_SIZE {
        show_regs(container_of(regs, pt_regs, regs));
        panic!("Kernel tried to access user memory at addr 0x%lx, ip 0x%lx", address, ip);
    }

    if SEGV_IS_FIXABLE(&fi) {
        err = handle_page_fault(address, ip, is_write, is_user, &mut si_code);
    } else {
        err = -EFAULT;
        address = 0;
    }

    if err == 0 {
        // out
    } else if is_user == 0 && arch_fixup(ip, regs) {
        // out
    } else if is_user == 0 {
        show_regs(container_of(regs, pt_regs, regs));
        panic!("Kernel mode fault at addr 0x%lx, ip 0x%lx", address, ip);
    } else {
        show_segv_info(regs);
        if err == -EACCES {
            (*current).thread.arch.faultinfo = fi;
            force_sig_fault(SIGBUS, BUS_ADRERR, address as *mut core::ffi::c_void);
        } else {
            BUG_ON(err != -EFAULT);
            (*current).thread.arch.faultinfo = fi;
            force_sig_fault(SIGSEGV, si_code, address as *mut core::ffi::c_void);
        }
    }

    if !regs.is_null() {
        (*current).thread.segv_regs = core::ptr::null_mut();
    }
    0
}

pub unsafe fn relay_signal(
    sig: c_int,
    si: *mut siginfo,
    regs: *mut uml_pt_regs,
    _mc: *mut core::ffi::c_void,
) {
    if !UPT_IS_USER(regs) {
        if sig == SIGBUS { printk(KERN_ERR "Bus error - the host /dev/shm or /tmp mount likely just ran out of space\n"); }
        panic!("Kernel mode signal %d", sig);
    }
    arch_examine_signal(sig, regs);
    let code = (*si).si_code;
    let err = (*si).si_errno;
    if err == 0 && siginfo_layout(sig, code) == SIL_FAULT {
        let fi = UPT_FAULTINFO(regs);
        (*current).thread.arch.faultinfo = *fi;
        force_sig_fault(sig, code, FAULT_ADDRESS(*fi) as *mut core::ffi::c_void);
    } else {
        printk(KERN_ERR "Attempted to relay unknown signal %d (si_code = %d) with errno %d\n", sig, code, err);
        force_sig(sig);
    }
}

pub unsafe fn winch(
    _sig: c_int,
    _unused_si: *mut siginfo,
    regs: *mut uml_pt_regs,
    _mc: *mut core::ffi::c_void,
) {
    do_IRQ(WINCH_IRQ, regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
