// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008 ARM Limited
 * Copyright (C) 2014 Regents of the University of California
 */

// Kernel dependency declarations are supplied by other translated files.

#[cfg(feature = "CONFIG_FRAME_POINTER")]
mod frame_pointer {
    use super::*;

    #[inline]
    unsafe fn read_once_task_stack(task: *mut task_struct, x: usize) -> usize {
        let addr = x;
        if task == current {
            core::ptr::read_volatile(addr as *const usize)
        } else {
            core::ptr::read_volatile(addr as *const usize)
        }
    }

    extern "C" {
        fn handle_exception();
        static ret_from_exception_end: usize;
    }

    #[inline]
    unsafe fn fp_is_valid(fp: usize, sp: usize) -> bool {
        let low = sp.wrapping_add(core::mem::size_of::<stackframe>());
        let high = align(sp, THREAD_SIZE);
        !(fp < low || fp > high || (fp & 0x07) != 0)
    }

    pub unsafe extern "C" fn walk_stackframe(
        task: *mut task_struct,
        regs: *mut pt_regs,
        fn_: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize) -> bool>,
        arg: *mut core::ffi::c_void,
    ) {
        let (mut fp, mut sp, mut pc, mut level): (usize, usize, usize, isize);
        let mut graph_idx: i32 = 0;
        if !regs.is_null() {
            fp = frame_pointer(regs);
            sp = user_stack_pointer(regs);
            pc = instruction_pointer(regs);
            level = 0;
        } else if task.is_null() || task == current {
            fp = __builtin_frame_address(0) as usize;
            sp = current_stack_pointer;
            pc = walk_stackframe as usize;
            level = -1;
        } else {
            fp = (*task).thread.s[0];
            sp = (*task).thread.sp;
            pc = (*task).thread.ra;
            level = 0;
        }

        loop {
            if !__kernel_text_address(pc)
                || ({ level += 1; level - 1 >= 0 && !fn_.unwrap()(arg, pc) })
            {
                break;
            }
            if !fp_is_valid(fp, sp) { break; }
            let frame = (fp as *mut stackframe).offset(-1);
            sp = fp;
            if !regs.is_null() && (*regs).epc == pc && fp_is_valid((*frame).ra, sp) {
                fp = (*frame).ra;
                pc = (*regs).ra;
            } else {
                fp = read_once_task_stack(task, &(*frame).fp as *const _ as usize);
                pc = read_once_task_stack(task, &(*frame).ra as *const _ as usize);
                pc = ftrace_graph_ret_addr(task, &mut graph_idx, pc, &mut (*frame).ra);
                if pc >= handle_exception as usize && pc < (&ret_from_exception_end as *const _ as usize) {
                    if !fn_.unwrap()(arg, pc) { break; }
                    pc = (*(sp as *mut pt_regs)).epc;
                    fp = (*(sp as *mut pt_regs)).s0;
                }
            }
        }
    }
}

#[cfg(not(feature = "CONFIG_FRAME_POINTER"))]
pub unsafe extern "C" fn walk_stackframe(
    task: *mut task_struct, regs: *mut pt_regs,
    fn_: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize) -> bool>,
    arg: *mut core::ffi::c_void,
) {
    let (mut sp, mut pc): (usize, usize);
    if !regs.is_null() { sp = user_stack_pointer(regs); pc = instruction_pointer(regs); }
    else if task.is_null() || task == current { sp = current_stack_pointer; pc = walk_stackframe as usize; }
    else { sp = (*task).thread.sp; pc = (*task).thread.ra; }
    if (sp & 0x7) != 0 { return; }
    let mut ksp = sp as *mut usize;
    while !kstack_end(ksp) {
        if __kernel_text_address(pc) && !fn_.unwrap()(arg, pc) { break; }
        pc = core::ptr::read_volatile(ksp);
        ksp = ksp.add(1);
    }
}

unsafe extern "C" fn print_trace_address(arg: *mut core::ffi::c_void, pc: usize) -> bool {
    print_ip_sym(arg as *const i8, pc); true
}

pub unsafe extern "C" fn dump_backtrace(regs: *mut pt_regs, task: *mut task_struct, loglvl: *const i8) {
    walk_stackframe(task, regs, Some(print_trace_address), loglvl as *mut _);
}

pub unsafe extern "C" fn show_stack(task: *mut task_struct, _sp: *mut usize, loglvl: *const i8) {
    pr_cont(c"%sCall Trace:\n", loglvl);
    dump_backtrace(core::ptr::null_mut(), task, loglvl);
}

unsafe extern "C" fn save_wchan(arg: *mut core::ffi::c_void, pc: usize) -> bool {
    if !in_sched_functions(pc) { *(arg as *mut usize) = pc; return false; }
    true
}

pub unsafe extern "C" fn __get_wchan(task: *mut task_struct) -> usize {
    let mut pc = 0;
    if !try_get_task_stack(task) { return 0; }
    walk_stackframe(task, core::ptr::null_mut(), Some(save_wchan), &mut pc as *mut _ as *mut _);
    put_task_stack(task); pc
}

pub unsafe extern "C" fn arch_stack_walk(consume_entry: stack_trace_consume_fn, cookie: *mut core::ffi::c_void, task: *mut task_struct, regs: *mut pt_regs) {
    walk_stackframe(task, regs, Some(consume_entry), cookie);
}

unsafe fn unwind_user_frame(consume_entry: stack_trace_consume_fn, cookie: *mut core::ffi::c_void, mut fp: usize, reg_ra: usize) -> usize {
    let mut buftail: stackframe = core::mem::zeroed();
    let user_frame_tail = (fp.wrapping_sub(core::mem::size_of::<stackframe>())) as *const stackframe;
    if !access_ok(user_frame_tail as *const _, core::mem::size_of::<stackframe>()) { return 0; }
    if __copy_from_user_inatomic(&mut buftail, user_frame_tail, core::mem::size_of::<stackframe>()) != 0 { return 0; }
    let ra = if reg_ra != 0 { reg_ra } else { buftail.ra };
    fp = buftail.fp;
    if ra == 0 || !consume_entry(cookie, ra) { return 0; }
    fp
}

pub unsafe extern "C" fn arch_stack_walk_user(consume_entry: stack_trace_consume_fn, cookie: *mut core::ffi::c_void, regs: *const pt_regs) {
    let mut fp = (*regs).s0;
    if !consume_entry(cookie, (*regs).epc) { return; }
    fp = unwind_user_frame(consume_entry, cookie, fp, (*regs).ra);
    while fp != 0 && (fp & 0x7) == 0 { fp = unwind_user_frame(consume_entry, cookie, fp, 0); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
