/*
 * arch/xtensa/kernel/process.c
 *
 * Xtensa Processor version.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Linux and Xtensa header dependencies are supplied by the surrounding tree.

unsafe extern "C" {
    fn ret_from_fork();
    fn ret_from_kernel_thread();
}

#[no_mangle]
pub static mut pm_power_off: Option<unsafe extern "C" fn()> = None;

#[cfg(CONFIG_STACKPROTECTOR)]
#[no_mangle]
pub static mut __stack_chk_guard: usize = 0;

#[cfg(XTENSA_HAVE_COPROCESSORS)]
pub unsafe extern "C" fn local_coprocessors_flush_release_all() {
    let coprocessor_owner: *mut *mut thread_info = (*this_cpu_ptr(&mut exc_table)).coprocessor_owner;
    xtensa_set_sr(XCHAL_CP_MASK, cpenable);

    let mut unique_owner: [*mut thread_info; XCHAL_CP_MAX] = [core::ptr::null_mut(); XCHAL_CP_MAX];
    let mut n = 0;
    for i in 0..XCHAL_CP_MAX {
        let ti = *coprocessor_owner.add(i);
        if !ti.is_null() {
            coprocessor_flush(ti, i);
            let mut j = 0;
            while j < n && unique_owner[j] != ti { j += 1; }
            if j == n { unique_owner[n] = ti; n += 1; }
            *coprocessor_owner.add(i) = core::ptr::null_mut();
        }
    }
    for i in 0..n {
        smp_wmb();
        (*unique_owner[i]).cpenable = 0;
    }
    xtensa_set_sr(0, cpenable);
}

#[cfg(XTENSA_HAVE_COPROCESSORS)]
unsafe extern "C" fn local_coprocessor_release_all(info: *mut core::ffi::c_void) {
    let ti = info as *mut thread_info;
    let owners: *mut *mut thread_info = (*this_cpu_ptr(&mut exc_table)).coprocessor_owner;
    for i in 0..XCHAL_CP_MAX {
        if *owners.add(i) == ti { *owners.add(i) = core::ptr::null_mut(); }
    }
    smp_wmb();
    (*ti).cpenable = 0;
    if ti == current_thread_info() { xtensa_set_sr(0, cpenable); }
}

#[cfg(XTENSA_HAVE_COPROCESSORS)]
pub unsafe extern "C" fn coprocessor_release_all(ti: *mut thread_info) {
    if (*ti).cpenable != 0 {
        smp_rmb();
        smp_call_function_single((*ti).cp_owner_cpu, Some(local_coprocessor_release_all), ti.cast(), true);
    }
}

#[cfg(XTENSA_HAVE_COPROCESSORS)]
unsafe extern "C" fn local_coprocessor_flush_all(info: *mut core::ffi::c_void) {
    let ti = info as *mut thread_info;
    let owners: *mut *mut thread_info = (*this_cpu_ptr(&mut exc_table)).coprocessor_owner;
    let old_cpenable = xtensa_xsr((*ti).cpenable, cpenable);
    for i in 0..XCHAL_CP_MAX {
        if *owners.add(i) == ti { coprocessor_flush(ti, i); }
    }
    xtensa_set_sr(old_cpenable, cpenable);
}

#[cfg(XTENSA_HAVE_COPROCESSORS)]
pub unsafe extern "C" fn coprocessor_flush_all(ti: *mut thread_info) {
    if (*ti).cpenable != 0 {
        smp_rmb();
        smp_call_function_single((*ti).cp_owner_cpu, Some(local_coprocessor_flush_all), ti.cast(), true);
    }
}

#[cfg(XTENSA_HAVE_COPROCESSORS)]
unsafe extern "C" fn local_coprocessor_flush_release_all(info: *mut core::ffi::c_void) {
    local_coprocessor_flush_all(info);
    local_coprocessor_release_all(info);
}

#[cfg(XTENSA_HAVE_COPROCESSORS)]
pub unsafe extern "C" fn coprocessor_flush_release_all(ti: *mut thread_info) {
    if (*ti).cpenable != 0 {
        smp_rmb();
        smp_call_function_single((*ti).cp_owner_cpu, Some(local_coprocessor_flush_release_all), ti.cast(), true);
    }
}

pub unsafe extern "C" fn arch_cpu_idle() {
    platform_idle();
    raw_local_irq_disable();
}

pub unsafe extern "C" fn exit_thread(tsk: *mut task_struct) {
    #[cfg(XTENSA_HAVE_COPROCESSORS)]
    coprocessor_release_all(task_thread_info(tsk));
}

pub unsafe extern "C" fn flush_thread() {
    #[cfg(XTENSA_HAVE_COPROCESSORS)]
    coprocessor_flush_release_all(current_thread_info());
    flush_ptrace_hw_breakpoint(current);
}

pub unsafe extern "C" fn arch_dup_task_struct(dst: *mut task_struct, src: *mut task_struct) -> i32 {
    #[cfg(XTENSA_HAVE_COPROCESSORS)]
    coprocessor_flush_all(task_thread_info(src));
    *dst = *src;
    0
}

pub unsafe extern "C" fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> i32 {
    let clone_flags = (*args).flags;
    let usp_thread_fn = (*args).stack;
    let tls = (*args).tls;
    let childregs = task_pt_regs(p);

    #[cfg(__XTENSA_WINDOWED_ABI__)]
    {
        *spill_slot(childregs, 1) = childregs as usize;
        *spill_slot(childregs, 0) = 0;
        (*p).thread.sp = childregs as usize;
    }
    #[cfg(__XTENSA_CALL0_ABI__)]
    { (*p).thread.sp = childregs as usize - 16; }

    if (*args).fn_.is_none() {
        let regs = current_pt_regs();
        let usp = if usp_thread_fn != 0 { usp_thread_fn } else { (*regs).areg[1] };
        (*p).thread.ra = make_ra_for_call(ret_from_fork as usize, 1);
        *childregs = *regs;
        (*childregs).areg[1] = usp;
        (*childregs).areg[2] = 0;
        if clone_flags & CLONE_VM != 0 {
            let len = (*childregs).wmask & !0xf;
            if (*regs).areg[1] == usp && len != 0 {
                let callinc = ((*regs).areg[0] >> 30) & 3;
                let caller_ars = XCHAL_NUM_AREGS - callinc * 4;
                put_user((*regs).areg[caller_ars + 1], (usp - 12) as *mut u64);
            }
            (*childregs).wmask = 1;
            (*childregs).windowstart = 1;
            (*childregs).windowbase = 0;
        }
        if clone_flags & CLONE_SETTLS != 0 { (*childregs).threadptr = tls; }
    } else {
        (*p).thread.ra = make_ra_for_call(ret_from_kernel_thread as usize, 1);
        #[cfg(__XTENSA_WINDOWED_ABI__)] {
            *spill_slot(childregs, 2) = (*args).fn_.unwrap() as usize;
            *spill_slot(childregs, 3) = (*args).fn_arg as usize;
        }
        #[cfg(__XTENSA_CALL0_ABI__)] {
            let sp = (*p).thread.sp as *mut usize;
            *sp.add(0) = (*args).fn_.unwrap() as usize;
            *sp.add(1) = (*args).fn_arg as usize;
        }
    }

    #[cfg(any(XTENSA_HAVE_COPROCESSORS, XTENSA_HAVE_IO_PORTS))]
    { (*task_thread_info(p)).cpenable = 0; }
    clear_ptrace_hw_breakpoint(p);
    0
}

pub unsafe extern "C" fn __get_wchan(p: *mut task_struct) -> usize {
    let mut sp = (*p).thread.sp;
    let mut pc = make_pc_from_ra((*p).thread.ra, _text);
    let stack_page = task_stack_page(p) as usize;
    let mut count = 0;
    loop {
        if sp < stack_page + core::mem::size_of::<task_struct>() || sp >= stack_page + THREAD_SIZE || pc == 0 { return 0; }
        if !in_sched_functions(pc) { return pc; }
        pc = make_pc_from_ra(*spill_slot(sp as *mut u8, 0), _text);
        sp = *spill_slot(sp as *mut u8, 1);
        count += 1;
        if count > 16 { return 0; }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
