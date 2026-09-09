// SPDX-License-Identifier: GPL-2.0
/*
 * This file handles the architecture dependent parts of process handling.
 *
 *    Copyright IBM Corp. 1999, 2009
 *    Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>,
 *               Hartmut Penner <hp@de.ibm.com>,
 *               Denis Joseph Barrow,
 */

// Linux and s390 dependencies supplied by the surrounding kernel translation.

pub unsafe extern "C" fn ret_from_fork();

pub unsafe extern "C" fn __ret_from_fork(
    prev: *mut task_struct,
    regs: *mut pt_regs,
) {
    let mut func: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32> = None;

    schedule_tail(prev);

    if !user_mode(regs) {
        // Kernel thread
        func = Some(core::mem::transmute((*regs).gprs[9]));
        func.unwrap()(core::mem::transmute((*regs).gprs[10]));
    }
    clear_pt_regs_flag(regs, PIF_SYSCALL);
    syscall_exit_to_user_mode(regs);
}

pub unsafe extern "C" fn flush_thread() {}

pub unsafe extern "C" fn arch_setup_new_exec() {
    if (*get_lowcore()).current_pid != (*current).pid {
        (*get_lowcore()).current_pid = (*current).pid;
        if test_facility(40) {
            lpp(&mut (*get_lowcore()).lpp);
        }
    }
}

pub unsafe extern "C" fn arch_release_task_struct(tsk: *mut task_struct) {
    runtime_instr_release(tsk);
    guarded_storage_release(tsk);
}

pub unsafe extern "C" fn arch_dup_task_struct(
    dst: *mut task_struct,
    src: *mut task_struct,
) -> i32 {
    save_user_fpu_regs();

    *dst = *src;
    (*dst).thread.kfpu_flags = 0;

    /*
     * Don't transfer over the runtime instrumentation or the guarded
     * storage control block pointers. These fields are cleared here instead
     * of in copy_thread() to avoid premature freeing of associated memory
     * on fork() failure. Wait to clear the RI flag because ->stack still
     * refers to the source thread.
     */
    (*dst).thread.ri_cb = core::ptr::null_mut();
    (*dst).thread.gs_cb = core::ptr::null_mut();
    (*dst).thread.gs_bc_cb = core::ptr::null_mut();

    0
}

pub unsafe extern "C" fn copy_thread(
    p: *mut task_struct,
    args: *const kernel_clone_args,
) -> i32 {
    let clone_flags: u64 = (*args).flags;
    let new_stackp: usize = (*args).stack;
    let tls: usize = (*args).tls;

    let frame = container_of(task_pt_regs(p), fake_frame, childregs);
    (*p).thread.ksp = frame as usize;
    // Save access registers to new thread structure.
    save_access_regs(&mut (*p).thread.acrs[0]);
    // start new process with ar4 pointing to the correct address space
    // Don't copy debug registers
    core::ptr::write_bytes(
        &mut (*p).thread.per_user as *mut _,
        0,
        core::mem::size_of_val(&(*p).thread.per_user),
    );
    core::ptr::write_bytes(
        &mut (*p).thread.per_event as *mut _,
        0,
        core::mem::size_of_val(&(*p).thread.per_event),
    );
    clear_tsk_thread_flag(p, TIF_SINGLE_STEP);
    (*p).thread.per_flags = 0;
    // Initialize per thread user and system timer values
    (*p).thread.user_timer = 0;
    (*p).thread.guest_timer = 0;
    (*p).thread.system_timer = 0;
    (*p).thread.hardirq_timer = 0;
    (*p).thread.softirq_timer = 0;
    (*p).thread.last_break = 1;

    (*frame).sf.back_chain = 0;
    (*frame).sf.gprs[11 - 6] = (&(*frame).childregs as *const _) as usize;
    (*frame).sf.gprs[12 - 6] = p as usize;
    // new return point is ret_from_fork
    (*frame).sf.gprs[14 - 6] = ret_from_fork as usize;
    // fake return stack for resume(), don't go back to schedule
    (*frame).sf.gprs[15 - 6] = frame as usize;

    // Store access registers to kernel stack of new process.
    if !(*args).r#fn.is_null() {
        // kernel thread
        core::ptr::write_bytes(
            &mut (*frame).childregs as *mut pt_regs,
            0,
            core::mem::size_of::<pt_regs>(),
        );
        (*frame).childregs.psw.mask = PSW_KERNEL_BITS | PSW_MASK_IO |
            PSW_MASK_EXT | PSW_MASK_MCHECK;
        (*frame).childregs.gprs[9] = (*args).r#fn as usize;
        (*frame).childregs.gprs[10] = (*args).fn_arg as usize;
        (*frame).childregs.orig_gpr2 = -1;
        (*frame).childregs.last_break = 1;
        return 0;
    }
    (*frame).childregs = *current_pt_regs();
    (*frame).childregs.gprs[2] = 0; // child returns 0 on fork.
    (*frame).childregs.flags = 0;
    if new_stackp != 0 {
        (*frame).childregs.gprs[15] = new_stackp;
    }
    /*
     * Clear the runtime instrumentation flag after the above childregs
     * copy. The CB pointer was already cleared in arch_dup_task_struct().
     */
    (*frame).childregs.psw.mask &= !PSW_MASK_RI;

    // Set a new TLS ?
    if clone_flags & CLONE_SETTLS != 0 {
        (*p).thread.acrs[0] = (tls >> 32) as u32;
        (*p).thread.acrs[1] = tls as u32;
    }
    /*
     * s390 stores the svc return address in arch_data when calling
     * sigreturn()/restart_syscall() via vdso. 1 means no valid address
     * stored.
     */
    (*p).restart_block.arch_data = 1;
    0
}

pub unsafe extern "C" fn execve_tail() {
    (*current).thread.ufpu.fpc = 0;
    fpu_sfpc(0);
}

pub unsafe extern "C" fn __switch_to(
    prev: *mut task_struct,
    next: *mut task_struct,
) -> *mut task_struct {
    save_user_fpu_regs();
    save_kernel_fpu_regs(&mut (*prev).thread);
    save_access_regs(&mut (*prev).thread.acrs[0]);
    save_ri_cb((*prev).thread.ri_cb);
    save_gs_cb((*prev).thread.gs_cb);
    update_cr_regs(next);
    restore_kernel_fpu_regs(&mut (*next).thread);
    restore_access_regs(&mut (*next).thread.acrs[0]);
    restore_ri_cb((*next).thread.ri_cb, (*prev).thread.ri_cb);
    restore_gs_cb((*next).thread.gs_cb);
    __switch_to_asm(prev, next)
}

pub unsafe extern "C" fn __get_wchan(p: *mut task_struct) -> usize {
    let mut state: unwind_state = core::mem::zeroed();
    let mut ip: usize = 0;

    if try_get_task_stack(p) == 0 {
        return 0;
    }

    unwind_for_each_frame!(&mut state, p, core::ptr::null_mut(), 0, {
        if state.stack_info.r#type != STACK_TYPE_TASK {
            ip = 0;
            break;
        }
        ip = unwind_get_return_address(&mut state);
        if ip == 0 || !in_sched_functions(ip) {
            break;
        }
    });

    put_task_stack(p);
    ip
}

pub unsafe extern "C" fn arch_align_stack(mut sp: usize) -> usize {
    if ((*current).personality & ADDR_NO_RANDOMIZE) == 0 && randomize_va_space != 0 {
        sp = sp.wrapping_sub(get_random_u32_below(PAGE_SIZE) as usize);
    }
    sp & !0xf
}

#[inline]
unsafe fn brk_rnd() -> usize {
    ((get_random_u16() as usize) & BRK_RND_MASK) << PAGE_SHIFT
}

pub unsafe extern "C" fn arch_randomize_brk(mm: *mut mm_struct) -> usize {
    let ret = page_align((*mm).brk.wrapping_add(brk_rnd()));
    if ret > (*mm).brk { ret } else { (*mm).brk }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
