// SPDX-License-Identifier: GPL-2.0
/*
 * SuperH process tracing
 *
 * Copyright (C) 1999, 2000  Kaz Kojima & Niibe Yutaka
 * Copyright (C) 2002 - 2009  Paul Mundt
 *
 * Audit support by Yuichi Nakamura <ynakam@hitachisoft.jp>
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

#[inline]
unsafe fn get_stack_long(task: *mut task_struct, offset: i32) -> i32 {
    let mut stack = task_pt_regs(task) as *mut u8;
    stack = stack.offset(offset as isize);
    *(stack as *mut i32)
}

#[inline]
unsafe fn put_stack_long(task: *mut task_struct, offset: i32, data: c_ulong) -> i32 {
    let mut stack = task_pt_regs(task) as *mut u8;
    stack = stack.offset(offset as isize);
    *(stack as *mut c_ulong) = data;
    0
}

unsafe fn ptrace_triggered(
    bp: *mut perf_event,
    _data: *mut perf_sample_data,
    _regs: *mut pt_regs,
) {
    let mut attr = (*bp).attr;
    attr.disabled = true;
    modify_user_hw_breakpoint(bp, &mut attr);
}

unsafe fn set_single_step(tsk: *mut task_struct, addr: c_ulong) -> i32 {
    let thread = &mut (*tsk).thread;
    let mut bp = thread.ptrace_bps[0];
    let mut attr: perf_event_attr;

    if bp.is_null() {
        attr = core::mem::zeroed();
        ptrace_breakpoint_init(&mut attr);
        attr.bp_addr = addr;
        attr.bp_len = HW_BREAKPOINT_LEN_2;
        attr.bp_type = HW_BREAKPOINT_R;
        bp = register_user_hw_breakpoint(&mut attr, ptrace_triggered, core::ptr::null_mut(), tsk);
        if IS_ERR(bp) {
            return PTR_ERR(bp);
        }
        thread.ptrace_bps[0] = bp;
    } else {
        let err;
        attr = (*bp).attr;
        attr.bp_addr = addr;
        attr.disabled = false;
        err = modify_user_hw_breakpoint(bp, &mut attr);
        if unlikely(err != 0) {
            return err;
        }
    }
    0
}

unsafe fn user_enable_single_step(child: *mut task_struct) {
    let pc = get_stack_long(child, core::mem::offset_of!(pt_regs, pc)) as c_ulong;
    set_tsk_thread_flag(child, TIF_SINGLESTEP);
    set_single_step(child, pc);
}

unsafe fn user_disable_single_step(child: *mut task_struct) {
    clear_tsk_thread_flag(child, TIF_SINGLESTEP);
}

unsafe fn ptrace_disable(child: *mut task_struct) {
    user_disable_single_step(child);
}

unsafe fn genregs_get(
    target: *mut task_struct,
    _regset: *const user_regset,
    mut to: membuf,
) -> i32 {
    let regs = task_pt_regs(target);
    membuf_write(&mut to, regs as *const _, core::mem::size_of::<pt_regs>())
}

unsafe fn genregs_set(
    target: *mut task_struct,
    _regset: *const user_regset,
    mut pos: c_uint,
    mut count: c_uint,
    mut kbuf: *const c_void,
    mut ubuf: *const c_void,
) -> i32 {
    let regs = task_pt_regs(target);
    let mut ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf,
        (*regs).regs.as_mut_ptr() as *mut c_void, 0,
        16 * core::mem::size_of::<c_ulong>());
    if ret == 0 && count > 0 {
        ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf,
            &mut (*regs).pc as *mut _ as *mut c_void,
            core::mem::offset_of!(pt_regs, pc), core::mem::size_of::<pt_regs>());
    }
    if ret == 0 {
        user_regset_copyin_ignore(&mut pos, &mut count, &mut kbuf, &mut ubuf,
            core::mem::size_of::<pt_regs>(), usize::MAX);
    }
    ret
}

#[cfg(CONFIG_SH_FPU)]
unsafe fn fpregs_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 {
    let ret = init_fpu(target);
    if ret != 0 { return ret; }
    membuf_write(&mut to, (*target).thread.xstate as *const _, core::mem::size_of::<user_fpu_struct>())
}

#[cfg(CONFIG_SH_FPU)]
unsafe fn fpregs_set(target: *mut task_struct, _regset: *const user_regset,
    mut pos: c_uint, mut count: c_uint, mut kbuf: *const c_void, mut ubuf: *const c_void) -> i32 {
    let ret = init_fpu(target);
    if ret != 0 { return ret; }
    set_stopped_child_used_math(target);
    if (boot_cpu_data.flags & CPU_HAS_FPU) != 0 {
        return user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf,
            &mut (*(*target).thread.xstate).hardfpu as *mut _ as *mut c_void, 0, usize::MAX);
    }
    user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf,
        &mut (*(*target).thread.xstate).softfpu as *mut _ as *mut c_void, 0, usize::MAX)
}

#[cfg(CONFIG_SH_FPU)]
unsafe fn fpregs_active(target: *mut task_struct, regset: *const user_regset) -> i32 {
    if tsk_used_math(target) { (*regset).n as i32 } else { 0 }
}

#[cfg(CONFIG_SH_DSP)]
unsafe fn dspregs_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 {
    let regs = &(*target).thread.dsp_status.dsp_regs as *const _ as *const pt_dspregs;
    membuf_write(&mut to, regs, core::mem::size_of::<pt_dspregs>())
}

#[cfg(CONFIG_SH_DSP)]
unsafe fn dspregs_set(target: *mut task_struct, _regset: *const user_regset,
    mut pos: c_uint, mut count: c_uint, mut kbuf: *const c_void, mut ubuf: *const c_void) -> i32 {
    let regs = &mut (*target).thread.dsp_status.dsp_regs as *mut _ as *mut pt_dspregs;
    let ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf,
        regs as *mut c_void, 0, core::mem::size_of::<pt_dspregs>());
    if ret == 0 { user_regset_copyin_ignore(&mut pos, &mut count, &mut kbuf, &mut ubuf,
        core::mem::size_of::<pt_dspregs>(), usize::MAX); }
    ret
}

#[cfg(CONFIG_SH_DSP)]
unsafe fn dspregs_active(target: *mut task_struct, regset: *const user_regset) -> i32 {
    let regs = task_pt_regs(target);
    if ((*regs).sr & SR_DSP) != 0 { (*regset).n as i32 } else { 0 }
}

#[repr(C)]
#[allow(non_camel_case_types)]
enum sh_regset {
    REGSET_GENERAL,
    #[cfg(CONFIG_SH_FPU)]
    REGSET_FPU,
    #[cfg(CONFIG_SH_DSP)]
    REGSET_DSP,
}

// The REG_OFFSET_NAME/REGS_OFFSET_NAME helpers expand to architecture-specific
// pt_regs_offset initializers in the surrounding kernel bindings.
extern "C" {
    static regoffset_table: [pt_regs_offset; 24];
}

// Native regset descriptors retain the C layout and callback relationships.
extern "C" {
    static sh_regsets: [user_regset; 1];
}

extern "C" {
    static user_sh_native_view: user_regset_view;
}

unsafe fn task_user_regset_view(_task: *mut task_struct) -> *const user_regset_view {
    &user_sh_native_view
}

unsafe fn arch_ptrace(child: *mut task_struct, request: c_long, addr: c_ulong, data: c_ulong) -> c_long {
    let datap = data as *mut c_ulong;
    let mut ret: i32;
    match request {
        PTRACE_PEEKUSR => {
            let mut tmp: c_ulong = 0;
            ret = -EIO;
            if (addr & 3) != 0 || addr > core::mem::size_of::<user>() - 3 { return ret as c_long; }
            if addr < core::mem::size_of::<pt_regs>() {
                tmp = get_stack_long(child, addr as i32) as c_ulong;
            } else if addr >= core::mem::offset_of!(user, fpu) && addr < core::mem::offset_of!(user, u_fpvalid) {
                if !tsk_used_math(child) {
                    tmp = if addr == core::mem::offset_of!(user, fpu.fpscr) { FPSCR_INIT } else { 0 };
                } else {
                    let ret_fpu = init_fpu(child);
                    if ret_fpu != 0 { return ret_fpu as c_long; }
                    let index = addr - core::mem::offset_of!(user, fpu);
                    tmp = *((*child).thread.xstate as *const c_ulong).add((index >> 2) as usize);
                }
            } else if addr == core::mem::offset_of!(user, u_fpvalid) {
                tmp = tsk_used_math(child) as c_ulong;
            } else if addr == PT_TEXT_ADDR { tmp = (*(*child).mm).start_code;
            } else if addr == PT_DATA_ADDR { tmp = (*(*child).mm).start_data;
            } else if addr == PT_TEXT_END_ADDR { tmp = (*(*child).mm).end_code;
            } else if addr == PT_TEXT_LEN { tmp = (*(*child).mm).end_code - (*(*child).mm).start_code; }
            ret = put_user(tmp, datap);
        }
        PTRACE_POKEUSR => {
            ret = -EIO;
            if (addr & 3) != 0 || addr > core::mem::size_of::<user>() - 3 { return ret as c_long; }
            if addr < core::mem::size_of::<pt_regs>() { ret = put_stack_long(child, addr as i32, data); }
            else if addr >= core::mem::offset_of!(user, fpu) && addr < core::mem::offset_of!(user, u_fpvalid) {
                let ret_fpu = init_fpu(child);
                if ret_fpu != 0 { return ret_fpu as c_long; }
                let index = addr - core::mem::offset_of!(user, fpu);
                set_stopped_child_used_math(child);
                *((*child).thread.xstate as *mut c_ulong).add((index >> 2) as usize) = data;
                ret = 0;
            } else if addr == core::mem::offset_of!(user, u_fpvalid) {
                conditional_stopped_child_used_math(data, child); ret = 0;
            }
        }
        PTRACE_GETREGS => return copy_regset_to_user(child, &user_sh_native_view, REGSET_GENERAL, 0, core::mem::size_of::<pt_regs>(), datap) as c_long,
        PTRACE_SETREGS => return copy_regset_from_user(child, &user_sh_native_view, REGSET_GENERAL, 0, core::mem::size_of::<pt_regs>(), datap) as c_long,
        #[cfg(CONFIG_SH_FPU)]
        PTRACE_GETFPREGS => return copy_regset_to_user(child, &user_sh_native_view, REGSET_FPU, 0, core::mem::size_of::<user_fpu_struct>(), datap) as c_long,
        #[cfg(CONFIG_SH_FPU)]
        PTRACE_SETFPREGS => return copy_regset_from_user(child, &user_sh_native_view, REGSET_FPU, 0, core::mem::size_of::<user_fpu_struct>(), datap) as c_long,
        #[cfg(CONFIG_SH_DSP)]
        PTRACE_GETDSPREGS => return copy_regset_to_user(child, &user_sh_native_view, REGSET_DSP, 0, core::mem::size_of::<pt_dspregs>(), datap) as c_long,
        #[cfg(CONFIG_SH_DSP)]
        PTRACE_SETDSPREGS => return copy_regset_from_user(child, &user_sh_native_view, REGSET_DSP, 0, core::mem::size_of::<pt_dspregs>(), datap) as c_long,
        _ => { ret = ptrace_request(child, request, addr, data); }
    }
    ret as c_long
}

unsafe fn do_syscall_trace_enter(regs: *mut pt_regs) -> c_long {
    if test_thread_flag(TIF_SYSCALL_TRACE) && !ptrace_report_syscall_permit_entry(regs) {
        (*regs).regs[0] = (-ENOSYS) as _; return -1;
    }
    if !seccomp_permit_syscall() { return -1; }
    if unlikely(test_thread_flag(TIF_SYSCALL_TRACEPOINT)) { trace_sys_enter(regs, (*regs).regs[0]); }
    audit_syscall_entry((*regs).regs[3], (*regs).regs[4], (*regs).regs[5], (*regs).regs[6], (*regs).regs[7]);
    0
}

unsafe fn do_syscall_trace_leave(regs: *mut pt_regs) {
    audit_syscall_exit(regs);
    if unlikely(test_thread_flag(TIF_SYSCALL_TRACEPOINT)) { trace_sys_exit(regs, (*regs).regs[0]); }
    let step = test_thread_flag(TIF_SINGLESTEP);
    if step || test_thread_flag(TIF_SYSCALL_TRACE) { ptrace_report_syscall_exit(regs, step); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
