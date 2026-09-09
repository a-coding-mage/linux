// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * Amit Bhor, Kanika Nema: Codito Technologies 2004
 */

// Linux and architecture headers from the original implementation provide the
// external types, constants, macros, and functions referenced below.

pub unsafe fn arc_settls(user_tls_data_ptr: *mut core::ffi::c_void) -> isize {
    task_thread_info(current()).thr_ptr = user_tls_data_ptr as u32;
    0
}

pub unsafe fn arc_gettls() -> u32 {
    task_thread_info(current()).thr_ptr
}

pub unsafe fn arc_usr_cmpxchg(
    uaddr: *mut i32,
    expected: i32,
    new: i32,
) -> i32 {
    let regs: *mut pt_regs = current_pt_regs();
    let mut uval: u32 = 0;
    let mut ret: i32;

    // WARN_ON_ONCE(IS_ENABLED(CONFIG_SMP));
    (*regs).status32 &= !STATUS_Z_MASK;

    ret = access_ok(uaddr as *const core::ffi::c_void, core::mem::size_of::<i32>());
    if ret == 0 {
        force_sig(SIGSEGV);
        return ret;
    }

    loop {
        preempt_disable();

        ret = __get_user(&mut uval, uaddr);
        if ret != 0 {
            preempt_enable();
            if ret != -EFAULT {
                force_sig(SIGSEGV);
                return ret;
            }
            mmap_read_lock((*current()).mm);
            ret = fixup_user_fault(
                (*current()).mm,
                uaddr as usize,
                FAULT_FLAG_WRITE,
                core::ptr::null_mut(),
            );
            mmap_read_unlock((*current()).mm);
            if ret == 0 {
                continue;
            }
            force_sig(SIGSEGV);
            return ret;
        }

        if uval != expected as u32 {
            preempt_enable();
            return uval as i32;
        }

        ret = __put_user(new, uaddr);
        if ret != 0 {
            preempt_enable();
            if ret != -EFAULT {
                force_sig(SIGSEGV);
                return ret;
            }
            mmap_read_lock((*current()).mm);
            ret = fixup_user_fault(
                (*current()).mm,
                uaddr as usize,
                FAULT_FLAG_WRITE,
                core::ptr::null_mut(),
            );
            mmap_read_unlock((*current()).mm);
            if ret == 0 {
                continue;
            }
            force_sig(SIGSEGV);
            return ret;
        }

        (*regs).status32 |= STATUS_Z_MASK;
        preempt_enable();
        return uval as i32;
    }
}

#[cfg(CONFIG_ISA_ARCV2)]
pub unsafe fn arch_cpu_idle() {
    // Re-enable interrupts <= default irq priority before committing SLEEP.
    let arg: u32 = 0x10 | ARCV2_IRQ_DEF_PRIO;
    asm!("sleep {0}", in(reg) arg);
    raw_local_irq_disable();
}

#[cfg(not(CONFIG_ISA_ARCV2))]
pub unsafe fn arch_cpu_idle() {
    // sleep, but enable both set E1/E2 (levels of interrupts) before committing
    asm!("sleep 0x3");
    raw_local_irq_disable();
}

pub unsafe extern "C" fn ret_from_fork() -> !;

pub unsafe fn copy_thread(
    p: *mut task_struct,
    args: *const kernel_clone_args,
) -> i32 {
    let clone_flags: u64 = (*args).flags;
    let usp: usize = (*args).stack;
    let tls: usize = (*args).tls;
    let c_regs: *mut pt_regs;
    let childksp: *mut usize;
    let c_callee: *mut callee_regs;
    let parent_callee: *mut callee_regs;
    let regs: *mut pt_regs = current_pt_regs();

    c_regs = task_pt_regs(p);
    childksp = c_regs.sub(2) as *mut usize;
    c_callee = (childksp as *mut callee_regs).sub(1);
    (*task_thread_info(p)).ksp = c_callee as usize;

    *childksp.add(0) = 0;
    *childksp.add(1) = ret_from_fork as usize;

    if !(*args).fn_.is_null() {
        core::ptr::write_bytes(c_regs as *mut u8, 0, core::mem::size_of::<pt_regs>());
        (*c_callee).r13 = (*args).fn_arg as usize;
        (*c_callee).r14 = (*args).fn_ as usize;
        return 0;
    }

    *childksp.add(0) = 0;
    *childksp.add(1) = ret_from_fork as usize;
    *c_regs = *regs;
    if usp != 0 {
        (*c_regs).sp = usp;
    }
    (*c_regs).r0 = 0;

    parent_callee = (regs as *mut callee_regs).sub(1);
    *c_callee = *parent_callee;

    if clone_flags & CLONE_SETTLS as u64 != 0 {
        (*task_thread_info(p)).thr_ptr = tls as u32;
    } else {
        (*task_thread_info(p)).thr_ptr = (*task_thread_info(current())).thr_ptr;
    }
    (*c_callee).r25 = (*task_thread_info(p)).thr_ptr as usize;
    0
}

pub unsafe fn start_thread(regs: *mut pt_regs, pc: usize, usp: usize) {
    (*regs).sp = usp;
    (*regs).ret = pc;
    (*regs).status32 = STATUS_U_MASK | STATUS_L_MASK | ISA_INIT_STATUS_BITS;
    fpu_init_task(regs);
    (*regs).lp_start = 0x10;
    (*regs).lp_end = 0x80;
}

pub unsafe fn flush_thread() {}

pub unsafe fn elf_check_arch(x: *const elf32_hdr) -> i32 {
    let eflags: u32;
    if (*x).e_machine != EM_ARC_INUSE {
        pr_err("ELF not built for %s ISA\n", if is_isa_arcompact() { "ARCompact" } else { "ARCv2" });
        return 0;
    }
    eflags = (*x).e_flags;
    if eflags & EF_ARC_OSABI_MSK != EF_ARC_OSABI_CURRENT {
        pr_err("ABI mismatch - you need newer toolchain\n");
        force_fatal_sig(SIGSEGV);
        return 0;
    }
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
