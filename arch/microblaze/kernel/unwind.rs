/*
 * Backtrace support for Microblaze
 *
 * Copyright (C) 2010  Digital Design Corporation
 *
 * Based on arch/sh/kernel/cpu/sh5/unwind.c code which is:
 * Copyright (C) 2004  Paul Mundt
 * Copyright (C) 2004  Richard Curnow
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/* Dependencies supplied by the surrounding kernel translation. */
use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct stack_trace {
    pub skip: c_int,
    pub nr_entries: c_int,
    pub max_entries: c_int,
    pub entries: *mut c_ulong,
}

#[repr(C)]
pub struct task_struct {
    pub stack: *mut c_void,
    pub pid: c_ulong,
    pub comm: [c_char; 16],
}

#[repr(C)]
pub struct pt_regs {
    pub pc: c_ulong,
    pub r1: c_ulong,
    pub r15: c_ulong,
}

#[repr(C)]
pub struct thread_info {
    pub cpu_context: cpu_context,
}

#[repr(C)]
pub struct cpu_context {
    pub r1: c_ulong,
    pub r15: c_ulong,
}

#[repr(C)]
pub struct trap_handler_info {
    pub start_addr: c_ulong,
    pub end_addr: c_ulong,
    pub trap_name: *const c_char,
}

extern "C" {
    static mut current: *mut task_struct;
    static microblaze_trap_handlers: trap_handler_info;
    static _switch_to: c_void;
    static _hw_exception_handler: c_void;
    static ex_handler_unhandled: c_void;

    fn kernel_text_address(addr: c_ulong) -> bool;
    fn task_pt_regs(task: *mut task_struct) -> *const pt_regs;
    fn print_ip_sym(loglvl: *const c_char, pc: c_ulong);
    fn printk(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
}

const EINVAL: c_int = 22;

/// get_frame_size - Extract the stack adjustment from an
///                  "addik r1, r1, adjust" instruction
/// @instr : Microblaze instruction
///
/// Return - Number of stack bytes the instruction reserves or reclaims
#[inline]
unsafe fn get_frame_size(instr: c_ulong) -> isize {
    (instr as u16 as i16).unsigned_abs() as isize
}

/// find_frame_creation - Search backward to find the instruction that creates
///                       the stack frame (hopefully, for the same function the
///                       initial PC is in).
/// @pc : Program counter at which to begin the search
///
/// Return - PC at which stack frame creation occurs
///          NULL if this cannot be found, i.e. a leaf function
unsafe fn find_frame_creation(mut pc: *mut c_ulong) -> *mut c_ulong {
    for _i in 0..1000 {
        let instr: c_ulong;
        let frame_size: i16;

        if !kernel_text_address(pc as c_ulong) {
            return core::ptr::null_mut();
        }

        instr = *pc;

        /* addik r1, r1, foo ? */
        if (instr & 0xFFFF0000) != 0x30210000 {
            pc = pc.sub(1);
            continue;
        }

        frame_size = get_frame_size(instr) as i16;
        if frame_size < 8 || (frame_size & 3) != 0 {
            pr_debug(b"    Invalid frame size %d at 0x%p\0".as_ptr() as *const c_char,
                     frame_size as c_int, pc);
            return core::ptr::null_mut();
        }

        pr_debug(b"    Found frame creation at 0x%p, size %d\n\0".as_ptr() as *const c_char,
                 pc, frame_size as c_int);
        return pc;
    }

    core::ptr::null_mut()
}

/// lookup_prev_stack_frame - Find the stack frame of the previous function.
unsafe fn lookup_prev_stack_frame(
    fp: c_ulong,
    pc: c_ulong,
    leaf_return: c_ulong,
    pprev_fp: *mut c_ulong,
    pprev_pc: *mut c_ulong,
) -> c_int {
    let mut prologue: *mut c_ulong = core::ptr::null_mut();

    /* _switch_to is a special leaf function */
    if pc != (&_switch_to as *const c_void as c_ulong) {
        prologue = find_frame_creation(pc as *mut c_ulong);
    }

    if !prologue.is_null() {
        let frame_size = get_frame_size(*prologue) as c_ulong;
        *pprev_fp = fp.wrapping_add(frame_size);
        *pprev_pc = *(fp as *const c_ulong);
    } else {
        if leaf_return == 0 {
            return -EINVAL;
        }
        *pprev_pc = leaf_return;
        *pprev_fp = fp;
    }

    if *pprev_pc == 0 || (*pprev_pc & 3) != 0 { -EINVAL } else { 0 }
}

unsafe fn microblaze_unwind_inner(
    task: *mut task_struct,
    pc: c_ulong,
    fp: c_ulong,
    leaf_return: c_ulong,
    trace: *mut stack_trace,
    loglvl: *const c_char,
);

/// unwind_trap - Unwind through a system trap, that stored previous state
///                  on the stack.
#[inline]
unsafe fn unwind_trap(
    _task: *mut task_struct,
    _pc: c_ulong,
    _fp: c_ulong,
    _trace: *mut stack_trace,
    _loglvl: *const c_char,
) {
    /* To be implemented */
}

unsafe fn microblaze_unwind_inner(
    task: *mut task_struct,
    mut pc: c_ulong,
    mut fp: c_ulong,
    mut leaf_return: c_ulong,
    trace: *mut stack_trace,
    loglvl: *const c_char,
) {
    let mut ofs: c_ulong = 0;

    pr_debug(b"    Unwinding with PC=%p, FP=%p\n\0".as_ptr() as *const c_char,
             pc as *const c_void, fp as *const c_void);
    if pc == 0 || fp == 0 || (pc & 3) != 0 || (fp & 3) != 0 {
        pr_debug(b"    Invalid state for unwind, aborting\n\0".as_ptr() as *const c_char);
        return;
    }
    while pc != 0 {
        let mut next_fp: c_ulong = 0;
        let mut next_pc: c_ulong = 0;
        let return_to = pc.wrapping_add(2 * core::mem::size_of::<c_ulong>() as c_ulong);
        let mut handler: *const trap_handler_info = &microblaze_trap_handlers;

        if return_to >= (&_hw_exception_handler as *const c_void as c_ulong)
            && return_to < (&ex_handler_unhandled as *const c_void as c_ulong)
        {
            printk(b"%sHW EXCEPTION\n\0".as_ptr() as *const c_char, loglvl);
            return;
        }

        while (*handler).start_addr != 0 {
            if return_to >= (*handler).start_addr && return_to <= (*handler).end_addr {
                if trace.is_null() {
                    printk(b"%s%s\n\0".as_ptr() as *const c_char,
                           loglvl, (*handler).trap_name);
                }
                unwind_trap(task, pc, fp, trace, loglvl);
                return;
            }
            handler = handler.add(1);
        }
        pc = pc.wrapping_sub(ofs);

        if !trace.is_null() {
            /* CONFIG_STACKTRACE is a build-time condition in the C source. */
            if (*trace).skip > 0 {
                (*trace).skip -= 1;
            } else {
                *(*trace).entries.add((*trace).nr_entries as usize) = pc;
                (*trace).nr_entries += 1;
            }
            if (*trace).nr_entries >= (*trace).max_entries { break; }
        } else {
            if unlikely(pc == (*task_pt_regs(task)).pc) {
                printk(b"%s[<%p>] PID %lu [%s]\n\0".as_ptr() as *const c_char,
                       loglvl, pc as *const c_void, (*task).pid, (*task).comm.as_ptr());
                break;
            } else {
                print_ip_sym(loglvl, pc);
            }
        }

        if !kernel_text_address(pc) { break; }

        if lookup_prev_stack_frame(fp, pc, leaf_return, &mut next_fp, &mut next_pc) == 0 {
            ofs = core::mem::size_of::<c_ulong>() as c_ulong;
            pc = next_pc & !3;
            fp = next_fp;
            leaf_return = 0;
        } else {
            pr_debug(b"    Failed to find previous stack frame\n\0".as_ptr() as *const c_char);
            break;
        }

        pr_debug(b"    Next PC=%p, next FP=%p\n\0".as_ptr() as *const c_char,
                 next_pc as *const c_void, next_fp as *const c_void);
    }
}

#[inline]
unsafe fn unlikely(value: bool) -> bool { value }

/// microblaze_unwind - Stack unwinder for Microblaze (external entry point)
pub unsafe fn microblaze_unwind(
    task: *mut task_struct,
    trace: *mut stack_trace,
    loglvl: *const c_char,
) {
    if !task.is_null() {
        if task == current {
            let regs = task_pt_regs(task);
            microblaze_unwind_inner(task, (*regs).pc, (*regs).r1, (*regs).r15, trace, loglvl);
        } else {
            let thread_info = (*task).stack as *const thread_info;
            let cpu_context = &(*thread_info).cpu_context;
            microblaze_unwind_inner(task, &_switch_to as *const c_void as c_ulong,
                                    cpu_context.r1, cpu_context.r15, trace, loglvl);
        }
    } else {
        let mut pc: c_ulong;
        let mut fp: c_ulong;

        core::arch::asm!("or {0}, r1, r0", out(reg) fp);
        core::arch::asm!("brlid {0}, 0f; nop; 0:", out(reg) pc);

        /* Since we are not a leaf function, use leaf_return = 0 */
        microblaze_unwind_inner(current, pc, fp, 0, trace, loglvl);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
