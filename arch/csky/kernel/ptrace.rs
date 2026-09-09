// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Dependencies are supplied by the surrounding kernel translation.

pub const TRACE_MODE_SI: u32 = 1 << 14;
pub const TRACE_MODE_RUN: u32 = 0;
pub const TRACE_MODE_MASK: u32 = !(0x3 << 14);

#[repr(C)]
pub struct task_struct { _private: [u8; 0] }
#[repr(C)]
pub struct pt_regs { _private: [u8; 0] }
#[repr(C)]
pub struct user_regset { _private: [u8; 0] }
#[repr(C)]
pub struct membuf { _private: [u8; 0] }
#[repr(C)]
pub struct user_regset_view { _private: [u8; 0] }

extern "C" {
    fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
    fn task_thread_info(task: *mut task_struct) -> *mut thread_info;
    fn membuf_write(to: *mut membuf, from: *const core::ffi::c_void, size: usize) -> i32;
    fn user_regset_copyin(pos: *mut u32, count: *mut u32, kbuf: *mut *const core::ffi::c_void,
                           ubuf: *mut *const core::ffi::c_void, kdata: *mut core::ffi::c_void,
                           start: usize, end: usize) -> i32;
    fn ptrace_request(child: *mut task_struct, request: i64, addr: usize, data: usize) -> i64;
    fn ptrace_report_syscall_permit_entry(regs: *mut pt_regs) -> bool;
    fn ptrace_report_syscall_exit(regs: *mut pt_regs, why: i32);
    fn seccomp_permit_syscall() -> bool;
    fn test_thread_flag(flag: u32) -> bool;
    fn trace_sys_enter(regs: *mut pt_regs, nr: i64);
    fn trace_sys_exit(regs: *mut pt_regs, ret: i64);
    fn syscall_get_nr(task: *mut task_struct, regs: *mut pt_regs) -> i64;
    fn syscall_get_return_value(task: *mut task_struct, regs: *mut pt_regs) -> i64;
    fn audit_syscall_entry(id: i64, a0: usize, a1: usize, a2: usize, a3: usize);
    fn audit_syscall_exit(regs: *mut pt_regs);
    fn regs_syscallid(regs: *mut pt_regs) -> i64;
    fn kernel_stack_pointer(regs: *mut pt_regs) -> usize;
    fn strcmp(a: *const i8, b: *const i8) -> i32;
}

#[repr(C)] pub struct thread_info { pub tp_value: usize }

unsafe fn singlestep_disable(tsk: *mut task_struct) {
    let regs = task_pt_regs(tsk);
    // Field access is supplied by the kernel's pt_regs definition.
    (*regs).sr = ((*regs).sr & TRACE_MODE_MASK as _) | TRACE_MODE_RUN as _;
    (*regs).sr |= 1 << 6;
}

unsafe fn singlestep_enable(tsk: *mut task_struct) {
    let regs = task_pt_regs(tsk);
    (*regs).sr = ((*regs).sr & TRACE_MODE_MASK as _) | TRACE_MODE_SI as _;
    (*regs).sr &= !(1 << 6);
}

pub unsafe fn user_enable_single_step(child: *mut task_struct) { singlestep_enable(child); }
pub unsafe fn user_disable_single_step(child: *mut task_struct) { singlestep_disable(child); }

#[repr(i32)]
pub enum csky_regset { REGSET_GPR, REGSET_FPR }

pub unsafe fn gpr_get(target: *mut task_struct, _regset: *const user_regset,
                      to: *mut membuf) -> i32 {
    let regs = task_pt_regs(target);
    (*regs).tls = (*task_thread_info(target)).tp_value;
    membuf_write(to, regs as *const _, core::mem::size_of::<pt_regs>())
}

pub unsafe fn gpr_set(target: *mut task_struct, _regset: *const user_regset,
                      mut pos: u32, mut count: u32,
                      kbuf: *const core::ffi::c_void,
                      ubuf: *const core::ffi::c_void) -> i32 {
    let mut regs = core::mem::MaybeUninit::<pt_regs>::uninit();
    let ret = user_regset_copyin(&mut pos, &mut count, &mut (kbuf as *mut _),
        &mut (ubuf as *mut _), regs.as_mut_ptr() as *mut _, 0, usize::MAX);
    if ret != 0 { return ret; }
    let mut regs = regs.assume_init();
    regs.sr = (regs.sr & 1) | ((*task_pt_regs(target)).sr & !1);
    (*task_thread_info(target)).tp_value = regs.tls;
    *task_pt_regs(target) = regs;
    0
}

pub unsafe fn fpr_get(target: *mut task_struct, _regset: *const user_regset,
                      to: *mut membuf) -> i32 {
    let regs = ((*target as *mut u8).add(0)) as *const core::ffi::c_void;
    membuf_write(to, regs, 0)
}

pub unsafe fn fpr_set(target: *mut task_struct, _regset: *const user_regset,
                      mut pos: u32, mut count: u32,
                      kbuf: *const core::ffi::c_void,
                      ubuf: *const core::ffi::c_void) -> i32 {
    user_regset_copyin(&mut pos, &mut count, &mut (kbuf as *mut _),
        &mut (ubuf as *mut _), target as *mut _, 0, usize::MAX)
}

#[repr(C)]
pub struct pt_regs_offset { pub name: *const i8, pub offset: i32 }

pub unsafe fn task_user_regset_view(_task: *mut task_struct) -> *const user_regset_view {
    &user_csky_view
}

static user_csky_view: user_regset_view = user_regset_view { _private: [] };
static regoffset_table: [pt_regs_offset; 1] = [pt_regs_offset { name: core::ptr::null(), offset: 0 }];

pub unsafe fn regs_query_register_offset(name: *const i8) -> i32 {
    let mut roff = regoffset_table.as_ptr();
    while !(*roff).name.is_null() {
        if strcmp((*roff).name, name) == 0 { return (*roff).offset; }
        roff = roff.add(1);
    }
    -22
}

unsafe fn regs_within_kernel_stack(regs: *mut pt_regs, addr: usize) -> bool {
    (addr & !(THREAD_SIZE - 1)) == (kernel_stack_pointer(regs) & !(THREAD_SIZE - 1))
}

pub unsafe fn regs_get_kernel_stack_nth(regs: *mut pt_regs, n: u32) -> usize {
    let addr = (kernel_stack_pointer(regs) as *mut usize).add(n as usize);
    if regs_within_kernel_stack(regs, addr as usize) { *addr } else { 0 }
}

pub unsafe fn ptrace_disable(child: *mut task_struct) { singlestep_disable(child); }

pub unsafe fn arch_ptrace(child: *mut task_struct, request: i64, addr: usize, data: usize) -> i64 {
    let mut ret = -5i64;
    match request { _ => { ret = ptrace_request(child, request, addr, data); } }
    ret
}

pub unsafe fn syscall_trace_enter(regs: *mut pt_regs) -> i32 {
    if test_thread_flag(1) && !ptrace_report_syscall_permit_entry(regs) { return -1; }
    if !seccomp_permit_syscall() { return -1; }
    if test_thread_flag(2) { trace_sys_enter(regs, syscall_get_nr(core::ptr::null_mut(), regs)); }
    audit_syscall_entry(regs_syscallid(regs), (*regs).a0, (*regs).a1, (*regs).a2, (*regs).a3);
    0
}

pub unsafe fn syscall_trace_exit(regs: *mut pt_regs) {
    audit_syscall_exit(regs);
    if test_thread_flag(1) { ptrace_report_syscall_exit(regs, 0); }
    if test_thread_flag(2) { trace_sys_exit(regs, syscall_get_return_value(core::ptr::null_mut(), regs)); }
}

pub unsafe fn show_regs(fp: *mut pt_regs) {
    // The diagnostic output and target-specific TLB helpers are provided by the kernel environment.
    let _ = fp;
}

extern "C" { static THREAD_SIZE: usize; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
