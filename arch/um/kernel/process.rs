// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2015 Anton Ivanov (aivanov@{brocade.com,kot-begemot.co.uk})
 * Copyright (C) 2015 Thomas Meyer (thomas@m3y3r.de)
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 * Copyright 2003 PathScale, Inc.
 */

// Kernel and UML dependencies supplied by other translation units.

#[repr(C)]
pub struct task_struct { pub thread: thread_struct, pub personality: usize }
#[repr(C)] pub struct thread_struct { pub prev_sched: *mut task_struct, pub switch_buf: *mut switch_buf, pub regs: regs_wrapper, pub request: request_union, pub arch: arch_thread }
#[repr(C)] pub struct switch_buf { pub JB_SP: usize }
#[repr(C)] pub struct regs_wrapper { pub regs: pt_regs }
#[repr(C)] pub struct pt_regs { pub gp: usize, pub fp: usize }
#[repr(C)] pub struct request_union { pub thread: thread_request }
#[repr(C)] pub struct thread_request { pub proc: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>, pub arg: *mut core::ffi::c_void }
#[repr(C)] pub struct arch_thread { pub _private: usize }
#[repr(C)] pub struct kernel_clone_args { pub flags: u64, pub stack: usize, pub tls: usize, pub fn_: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>, pub fn_arg: *mut core::ffi::c_void }

pub type gfp_t = u32;
pub type exitcall_t = unsafe extern "C" fn();

extern "C" {
    static mut init_task: task_struct;
    static mut cpu_tasks: [*mut task_struct; NR_CPUS];
    static mut current: *mut task_struct;
    static mut __uml_exitcall_begin: exitcall_t;
    static mut __uml_exitcall_end: exitcall_t;
    static mut time_travel_mode: i32;
    static mut randomize_va_space: i32;
    fn free_pages(stack: usize, order: i32);
    fn __get_free_pages(flags: gfp_t, order: i32) -> usize;
    fn task_thread_info(task: *mut task_struct) -> *mut thread_info;
    fn switch_threads(from: *mut *mut core::ffi::c_void, to: *mut *mut core::ffi::c_void);
    fn arch_switch_to(task: *mut task_struct);
    fn read_thread_flags() -> usize;
    fn schedule();
    fn do_signal(regs: *mut pt_regs);
    fn resume_user_mode_work(regs: *mut pt_regs);
    fn task_pid_nr(task: *mut task_struct) -> i32;
    fn schedule_tail(task: *mut task_struct);
    fn userspace(regs: *mut pt_regs);
    fn current_pt_regs() -> *mut pt_regs;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn memset(dst: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn arch_copy_thread(from: *mut arch_thread, to: *mut arch_thread);
    fn get_safe_registers(gp: usize, fp: usize);
    fn new_thread(stack: *mut core::ffi::c_void, buf: *mut switch_buf, handler: unsafe extern "C" fn());
    fn task_stack_page(task: *mut task_struct) -> *mut core::ffi::c_void;
    fn clear_flushed_tls(task: *mut task_struct);
    fn arch_set_tls(task: *mut task_struct, tls: usize) -> i32;
    fn initial_thread_cb_skas(proc: unsafe extern "C" fn(*mut core::ffi::c_void), arg: *mut core::ffi::c_void);
    fn time_travel_sleep();
    fn os_idle_sleep();
    fn os_idle_prepare();
    fn in_atomic() -> i32;
    fn irqs_disabled() -> i32;
    fn in_interrupt() -> i32;
    fn need_resched() -> i32;
    fn kstrdup(s: *const i8, flags: gfp_t) -> *mut i8;
    fn copy_from_user(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void, size: i32) -> i32;
    fn test_thread_flag(flag: i32) -> i32;
    fn get_random_u32_below(n: u32) -> u32;
    fn in_sched_functions(ip: usize) -> i32;
    fn kernel_text_address(ip: usize) -> i32;
}

#[repr(C)] pub struct thread_info { pub cpu: usize }
const NR_CPUS: usize = 1;
const GFP_KERNEL: gfp_t = 0;
const GFP_ATOMIC: gfp_t = 0;
const _TIF_WORK_MASK: usize = usize::MAX;
const _TIF_NEED_RESCHED: usize = 1;
const _TIF_SIGPENDING: usize = 2;
const _TIF_NOTIFY_SIGNAL: usize = 4;
const _TIF_NOTIFY_RESUME: usize = 8;
const CLONE_SETTLS: u64 = 0x0008_0000;
const ADDR_NO_RANDOMIZE: usize = 0x0040_0000;
const TIF_SINGLESTEP: i32 = 0;
const TT_MODE_OFF: i32 = 0;
const THREAD_SIZE: usize = 8192;

#[no_mangle] pub static mut CPU_TASKS: [*mut task_struct; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];

pub unsafe extern "C" fn free_stack(stack: usize, order: i32) { free_pages(stack, order); }
pub unsafe extern "C" fn alloc_stack(order: i32, atomic: i32) -> usize { let flags = if atomic != 0 { GFP_ATOMIC } else { GFP_KERNEL }; __get_free_pages(flags, order) }
unsafe fn set_current(task: *mut task_struct) { CPU_TASKS[(*task_thread_info(task)).cpu] = task; }
pub unsafe extern "C" fn __switch_to(from: *mut task_struct, to: *mut task_struct) -> *mut task_struct { (*to).thread.prev_sched = from; set_current(to); switch_threads(&mut (*from).thread.switch_buf as *mut _, &mut (*to).thread.switch_buf as *mut _); arch_switch_to(current); (*current).thread.prev_sched }
pub unsafe extern "C" fn interrupt_end() { let regs = &mut (*current).thread.regs.regs; let mut flags = read_thread_flags(); while flags & _TIF_WORK_MASK != 0 { if flags & _TIF_NEED_RESCHED != 0 { schedule(); } if flags & (_TIF_SIGPENDING | _TIF_NOTIFY_SIGNAL) != 0 { do_signal(regs); } if flags & _TIF_NOTIFY_RESUME != 0 { resume_user_mode_work(regs); } flags = read_thread_flags(); } }
pub unsafe extern "C" fn get_current_pid() -> i32 { task_pid_nr(current) }
pub unsafe extern "C" fn new_thread_handler() { if !(*current).thread.prev_sched.is_null() { schedule_tail((*current).thread.prev_sched); } (*current).thread.prev_sched = core::ptr::null_mut(); let f = (*current).thread.request.thread.proc; let arg = (*current).thread.request.thread.arg; if let Some(f) = f { f(arg); } userspace(&mut (*current).thread.regs.regs); }
unsafe extern "C" fn fork_handler() { schedule_tail((*current).thread.prev_sched); arch_switch_to(current); (*current).thread.prev_sched = core::ptr::null_mut(); userspace(&mut (*current).thread.regs.regs); }

pub unsafe extern "C" fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> i32 { let clone_flags = (*args).flags; let sp = (*args).stack; let tls = (*args).tls; let mut ret = 0; (*p).thread = core::mem::zeroed(); if (*args).fn_.is_none() { memcpy(&mut (*p).thread.regs.regs as *mut _ as _, current_pt_regs() as _, core::mem::size_of::<pt_regs>()); if sp != 0 { (*p).thread.regs.regs.gp = sp; } arch_copy_thread(&mut (*current).thread.arch, &mut (*p).thread.arch); new_thread(task_stack_page(p), &mut (*p).thread.switch_buf as *mut _, fork_handler); clear_flushed_tls(p); if clone_flags & CLONE_SETTLS != 0 { ret = arch_set_tls(p, tls); } } else { get_safe_registers((*p).thread.regs.regs.gp, (*p).thread.regs.regs.fp); (*p).thread.request.thread.proc = (*args).fn_; (*p).thread.request.thread.arg = (*args).fn_arg; new_thread(task_stack_page(p), &mut (*p).thread.switch_buf as *mut _, new_thread_handler); } ret }
pub unsafe extern "C" fn initial_thread_cb(proc: unsafe extern "C" fn(*mut core::ffi::c_void), arg: *mut core::ffi::c_void) { initial_thread_cb_skas(proc, arg); }
pub unsafe extern "C" fn arch_dup_task_struct(dst: *mut task_struct, src: *mut task_struct) -> i32 { memcpy(dst as _, src as _, core::mem::size_of::<task_struct>()); 0 }
pub unsafe extern "C" fn um_idle_sleep() { if time_travel_mode != TT_MODE_OFF { time_travel_sleep(); } else { os_idle_sleep(); } }
pub unsafe extern "C" fn arch_cpu_idle() { um_idle_sleep(); }
pub unsafe extern "C" fn arch_cpu_idle_prepare() { os_idle_prepare(); }
pub unsafe extern "C" fn __uml_cant_sleep() -> i32 { in_atomic() != 0 as i32 || irqs_disabled() != 0 as i32 || in_interrupt() != 0 as i32 }
pub unsafe extern "C" fn uml_need_resched() -> i32 { need_resched() }
pub unsafe extern "C" fn do_uml_exitcalls() { let mut call = &raw mut __uml_exitcall_end; while (call as usize) > (&raw mut __uml_exitcall_begin as usize) { call = call.sub(1); (*call)(); } }
pub unsafe extern "C" fn uml_strdup(string: *const i8) -> *mut i8 { kstrdup(string, GFP_KERNEL) }
pub unsafe extern "C" fn copy_from_user_proc(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void, size: i32) -> i32 { copy_from_user(to, from, size) }
pub unsafe extern "C" fn singlestepping() -> i32 { test_thread_flag(TIF_SINGLESTEP) }
pub unsafe extern "C" fn arch_align_stack(mut sp: usize) -> usize { if ((*current).personality & ADDR_NO_RANDOMIZE) == 0 && randomize_va_space != 0 { sp = sp.wrapping_sub(get_random_u32_below(8192) as usize); } sp & !0xf }
pub unsafe extern "C" fn __get_wchan(p: *mut task_struct) -> usize { let stack_page = task_stack_page(p) as usize; if stack_page == 0 { return 0; } let mut sp = (*p).thread.switch_buf.as_ref().unwrap().JB_SP; let mut seen_sched = false; while sp < stack_page + THREAD_SIZE { let ip = *(sp as *const usize); if in_sched_functions(ip) != 0 { seen_sched = true; } else if kernel_text_address(ip) != 0 && seen_sched { return ip; } sp += core::mem::size_of::<usize>(); } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
