// SPDX-License-Identifier: GPL-2.0-only
/*
 * stacktrace.c : stacktracing APIs needed by rest of kernel
 *                 (wrappers over ARC dwarf based unwinder)
 *
 * Rust translation of the ARC kernel stack tracing implementation.
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    pub fp: usize,
    pub sp: usize,
    pub blink: usize,
    pub ret: usize,
}

#[repr(C)]
pub struct unwind_regs {
    pub r27: usize,
    pub r28: usize,
    pub r31: usize,
    pub r63: usize,
}

#[repr(C)]
pub struct unwind_frame_info {
    pub task: *mut task_struct,
    pub regs: unwind_regs,
    pub call_frame: c_int,
}

#[repr(C)]
pub struct stack_trace {
    pub nr_entries: usize,
    pub max_entries: usize,
    pub entries: *mut u32,
    pub skip: usize,
}

extern "C" {
    static mut current: *mut task_struct;
    fn task_is_running(tsk: *mut task_struct) -> c_int;
    fn arc_unwind(frame_info: *mut unwind_frame_info) -> c_int;
    fn __kernel_text_address(address: u32) -> c_int;
    fn in_sched_functions(address: u32) -> c_int;
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn pr_warn_once(fmt: *const c_char, ...);
    fn __switch_to();
}

#[inline]
unsafe fn unw_pc(frame_info: *const unwind_frame_info) -> u32 {
    (*frame_info).regs.r63 as u32
}

unsafe fn seed_unwind_frame_info(
    tsk: *mut task_struct,
    regs: *mut pt_regs,
    frame_info: *mut unwind_frame_info,
) -> c_int {
    if !regs.is_null() {
        (*frame_info).task = tsk;
        (*frame_info).regs.r27 = (*regs).fp;
        (*frame_info).regs.r28 = (*regs).sp;
        (*frame_info).regs.r31 = (*regs).blink;
        (*frame_info).regs.r63 = (*regs).ret;
        (*frame_info).call_frame = 0;
    } else if tsk.is_null() || tsk == current {
        (*frame_info).task = current;
        let (fp, sp, blink, ret): (usize, usize, usize, usize);
        core::arch::asm!(
            "mov {0}, r27", "mov {1}, r28", "mov {2}, r31", "mov {3}, r63",
            out(reg) fp, out(reg) sp, out(reg) blink, out(reg) ret
        );
        (*frame_info).regs.r27 = fp;
        (*frame_info).regs.r28 = sp;
        (*frame_info).regs.r31 = blink;
        (*frame_info).regs.r63 = ret;
        (*frame_info).call_frame = 0;
    } else {
        if task_is_running(tsk) != 0 {
            return -1;
        }
        (*frame_info).task = tsk;
        // TSK_K_FP, TSK_K_ESP, and TSK_K_BLINK are supplied by the ARC task-switching headers.
        (*frame_info).regs.r27 = 0;
        (*frame_info).regs.r28 = 60;
        (*frame_info).regs.r31 = 0;
        (*frame_info).regs.r63 = __switch_to as usize as u32 as usize;
        (*frame_info).regs.r27 = 0;
        (*frame_info).regs.r28 = (*frame_info).regs.r28.wrapping_add(60);
        (*frame_info).call_frame = 0;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn arc_unwind_core(
    tsk: *mut task_struct,
    regs: *mut pt_regs,
    consumer_fn: Option<unsafe extern "C" fn(u32, *mut c_void) -> c_int>,
    arg: *mut c_void,
) -> u32 {
    // CONFIG_ARC_DW2_UNWIND is a build-time kernel condition.
    let mut ret = 0;
    let mut cnt = 0;
    let mut address;
    let mut frame_info = core::mem::MaybeUninit::<unwind_frame_info>::uninit();
    if seed_unwind_frame_info(tsk, regs, frame_info.as_mut_ptr()) != 0 { return 0; }
    let frame_info = frame_info.as_mut_ptr();
    loop {
        address = unw_pc(frame_info);
        if address == 0 || __kernel_text_address(address) == 0 { break; }
        if consumer_fn.map_or(false, |f| f(address, arg) == -1) { break; }
        ret = arc_unwind(frame_info);
        if ret != 0 { break; }
        (*frame_info).regs.r63 = (*frame_info).regs.r31;
        cnt += 1;
        if cnt > 128 {
            printk(b"unwinder looping too long, aborting !\0".as_ptr() as *const c_char);
            return 0;
        }
    }
    address
}

unsafe extern "C" fn __print_sym(address: u32, arg: *mut c_void) -> c_int {
    printk(b"%s  %pS\n\0".as_ptr() as *const c_char, arg, address as usize as *mut c_void);
    0
}

unsafe extern "C" fn __collect_all(address: u32, arg: *mut c_void) -> c_int {
    let trace = &mut *(arg as *mut stack_trace);
    if trace.skip > 0 { trace.skip -= 1; } else { *trace.entries.add(trace.nr_entries) = address; trace.nr_entries += 1; }
    if trace.nr_entries >= trace.max_entries { return -1; }
    0
}

unsafe extern "C" fn __collect_all_but_sched(address: u32, arg: *mut c_void) -> c_int {
    if in_sched_functions(address) != 0 { return 0; }
    __collect_all(address, arg)
}

unsafe extern "C" fn __get_first_nonsched(address: u32, _unused: *mut c_void) -> c_int {
    if in_sched_functions(address) != 0 { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn show_stacktrace(tsk: *mut task_struct, regs: *mut pt_regs, loglvl: *const c_char) {
    printk(b"%s\nStack Trace:\n\0".as_ptr() as *const c_char, loglvl);
    arc_unwind_core(tsk, regs, Some(__print_sym), loglvl as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn show_stack(tsk: *mut task_struct, _sp: *mut usize, loglvl: *const c_char) {
    show_stacktrace(tsk, core::ptr::null_mut(), loglvl);
}

#[no_mangle]
pub unsafe extern "C" fn __get_wchan(tsk: *mut task_struct) -> u32 {
    arc_unwind_core(tsk, core::ptr::null_mut(), Some(__get_first_nonsched), core::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn save_stack_trace_tsk(tsk: *mut task_struct, trace: *mut stack_trace) {
    arc_unwind_core(tsk, core::ptr::null_mut(), Some(__collect_all_but_sched), trace as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn save_stack_trace(trace: *mut stack_trace) {
    arc_unwind_core(core::ptr::null_mut(), core::ptr::null_mut(), Some(__collect_all), trace as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
