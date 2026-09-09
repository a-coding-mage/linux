// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct pt_regs_offset {
    pub name: *const ::core::ffi::c_char,
    pub offset: i32,
}

#[cfg(CONFIG_ISA_ARCOMPACT)]
static regoffset_table: &[pt_regs_offset] = &[
    pt_regs_offset { name: b"bta\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"lp_start\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"lp_end\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"lp_count\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"status32\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"ret\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"blink\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"fp\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r26\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r12\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r11\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r10\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r9\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r8\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r7\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r6\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r5\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r4\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r3\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r2\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r1\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r0\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"sp\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"orig_r0\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"ecr\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: core::ptr::null(), offset: 0 },
];

#[cfg(not(CONFIG_ISA_ARCOMPACT))]
static regoffset_table: &[pt_regs_offset] = &[
    pt_regs_offset { name: b"orig_r0\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"ecr\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"bta\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r26\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"fp\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"sp\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r12\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r30\0".as_ptr() as *const _, offset: 0 },
    #[cfg(CONFIG_ARC_HAS_ACCL_REGS)] pt_regs_offset { name: b"r58\0".as_ptr() as *const _, offset: 0 },
    #[cfg(CONFIG_ARC_HAS_ACCL_REGS)] pt_regs_offset { name: b"r59\0".as_ptr() as *const _, offset: 0 },
    #[cfg(CONFIG_ARC_DSP_SAVE_RESTORE_REGS)] pt_regs_offset { name: b"DSP_CTRL\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r0\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r1\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r2\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r3\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r4\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r5\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r6\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r7\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r8\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r9\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r10\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"r11\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"blink\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"lp_end\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"lp_start\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"lp_count\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"ei\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"ldi\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"jli\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"ret\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: b"status32\0".as_ptr() as *const _, offset: 0 },
    pt_regs_offset { name: core::ptr::null(), offset: 0 },
];

extern "C" {
    fn task_callee_regs(tsk: *mut task_struct) -> *mut callee_regs;
}

unsafe fn task_callee_regs_local(tsk: *mut task_struct) -> *mut callee_regs {
    (*tsk).thread.callee_reg as *mut callee_regs
}

unsafe fn genregs_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 {
    let ptregs = task_pt_regs(target);
    let cregs = task_callee_regs_local(target);
    membuf_zero(&mut to, 4);
    membuf_store(&mut to, (*ptregs).bta); membuf_store(&mut to, (*ptregs).lp_start);
    membuf_store(&mut to, (*ptregs).lp_end); membuf_store(&mut to, (*ptregs).lp_count);
    membuf_store(&mut to, (*ptregs).status32); membuf_store(&mut to, (*ptregs).ret);
    membuf_store(&mut to, (*ptregs).blink); membuf_store(&mut to, (*ptregs).fp);
    membuf_store(&mut to, (*ptregs).r26); membuf_store(&mut to, (*ptregs).r12);
    membuf_store(&mut to, (*ptregs).r11); membuf_store(&mut to, (*ptregs).r10);
    membuf_store(&mut to, (*ptregs).r9); membuf_store(&mut to, (*ptregs).r8);
    membuf_store(&mut to, (*ptregs).r7); membuf_store(&mut to, (*ptregs).r6);
    membuf_store(&mut to, (*ptregs).r5); membuf_store(&mut to, (*ptregs).r4);
    membuf_store(&mut to, (*ptregs).r3); membuf_store(&mut to, (*ptregs).r2);
    membuf_store(&mut to, (*ptregs).r1); membuf_store(&mut to, (*ptregs).r0);
    membuf_store(&mut to, (*ptregs).sp); membuf_zero(&mut to, 4);
    membuf_store(&mut to, (*cregs).r25); membuf_store(&mut to, (*cregs).r24);
    membuf_store(&mut to, (*cregs).r23); membuf_store(&mut to, (*cregs).r22);
    membuf_store(&mut to, (*cregs).r21); membuf_store(&mut to, (*cregs).r20);
    membuf_store(&mut to, (*cregs).r19); membuf_store(&mut to, (*cregs).r18);
    membuf_store(&mut to, (*cregs).r17); membuf_store(&mut to, (*cregs).r16);
    membuf_store(&mut to, (*cregs).r15); membuf_store(&mut to, (*cregs).r14);
    membuf_store(&mut to, (*cregs).r13); membuf_store(&mut to, (*target).thread.fault_address);
    let stop_pc = if in_brkpt_trap(ptregs) { (*target).thread.fault_address } else { (*ptregs).ret };
    membuf_store(&mut to, stop_pc)
}

unsafe fn genregs_set(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, kbuf: *const ::core::ffi::c_void, ubuf: *const ::core::ffi::c_void) -> i32 {
    let ptregs = task_pt_regs(target); let cregs = task_callee_regs_local(target); let mut ret = 0;
    macro_rules! one { ($loc:expr, $ptr:expr) => { if ret == 0 { ret = user_regset_copyin(&mut pos, &mut count, &mut (kbuf as *mut _), &mut (ubuf as *mut _), $ptr, $loc, $loc + 4); } }; }
    macro_rules! ignore { ($loc:expr) => { if ret == 0 { user_regset_copyin_ignore(&mut pos, &mut count, &mut (kbuf as *mut _), &mut (ubuf as *mut _), $loc, $loc + 4); } }; }
    ignore!(0); one!(4, &mut (*ptregs).bta); one!(8, &mut (*ptregs).lp_start); one!(12, &mut (*ptregs).lp_end); one!(16, &mut (*ptregs).lp_count); ignore!(20);
    one!(24, &mut (*ptregs).ret); one!(28, &mut (*ptregs).blink); one!(32, &mut (*ptregs).fp); one!(36, &mut (*ptregs).r26); one!(40, &mut (*ptregs).r12); one!(44, &mut (*ptregs).r11); one!(48, &mut (*ptregs).r10); one!(52, &mut (*ptregs).r9); one!(56, &mut (*ptregs).r8); one!(60, &mut (*ptregs).r7); one!(64, &mut (*ptregs).r6); one!(68, &mut (*ptregs).r5); one!(72, &mut (*ptregs).r4); one!(76, &mut (*ptregs).r3); one!(80, &mut (*ptregs).r2); one!(84, &mut (*ptregs).r1); one!(88, &mut (*ptregs).r0); one!(92, &mut (*ptregs).sp); ignore!(96);
    one!(100, &mut (*cregs).r25); one!(104, &mut (*cregs).r24); one!(108, &mut (*cregs).r23); one!(112, &mut (*cregs).r22); one!(116, &mut (*cregs).r21); one!(120, &mut (*cregs).r20); one!(124, &mut (*cregs).r19); one!(128, &mut (*cregs).r18); one!(132, &mut (*cregs).r17); one!(136, &mut (*cregs).r16); one!(140, &mut (*cregs).r15); one!(144, &mut (*cregs).r14); one!(148, &mut (*cregs).r13); ignore!(152); ignore!(156); ret
}

#[cfg(CONFIG_ISA_ARCV2)]
unsafe fn arcv2regs_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 {
    let regs = task_pt_regs(target);
    if IS_ENABLED(CONFIG_ARC_HAS_ACCL_REGS) { return membuf_write(&mut to, &(*regs).r30 as *const _ as *const _, core::mem::size_of::<user_regs_arcv2>()); }
    membuf_write(&mut to, &(*regs).r30 as *const _ as *const _, 4);
    membuf_zero(&mut to, core::mem::size_of::<user_regs_arcv2>() - 4)
}

#[cfg(CONFIG_ISA_ARCV2)]
unsafe fn arcv2regs_set(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, kbuf: *const ::core::ffi::c_void, ubuf: *const ::core::ffi::c_void) -> i32 {
    let regs = task_pt_regs(target);
    let copy_sz = if IS_ENABLED(CONFIG_ARC_HAS_ACCL_REGS) { core::mem::size_of::<user_regs_arcv2>() } else { 4 };
    user_regset_copyin(&mut pos, &mut count, &mut (kbuf as *mut _), &mut (ubuf as *mut _), &mut (*regs).r30 as *mut _ as *mut _, 0, copy_sz)
}

#[repr(C)]
pub struct user_regset { pub n: usize, pub size: usize, pub align: usize, pub regset_get: Option<unsafe fn(*mut task_struct, *const user_regset, membuf) -> i32>, pub set: Option<unsafe fn(*mut task_struct, *const user_regset, u32, u32, *const ::core::ffi::c_void, *const ::core::ffi::c_void) -> i32> }

#[repr(C)]
pub struct user_regset_view { pub name: *const ::core::ffi::c_char, pub e_machine: u32, pub regsets: *const user_regset, pub n: usize }

#[repr(C)]
pub struct membuf;
#[repr(C)] pub struct task_struct { pub thread: thread_struct }
#[repr(C)] pub struct thread_struct { pub callee_reg: *mut ::core::ffi::c_void, pub fault_address: usize }
#[repr(C)] pub struct callee_regs { pub r25:u32,pub r24:u32,pub r23:u32,pub r22:u32,pub r21:u32,pub r20:u32,pub r19:u32,pub r18:u32,pub r17:u32,pub r16:u32,pub r15:u32,pub r14:u32,pub r13:u32 }
#[repr(C)] pub struct pt_regs { pub bta:u32,pub lp_start:u32,pub lp_end:u32,pub lp_count:u32,pub status32:u32,pub ret:u32,pub blink:u32,pub fp:u32,pub r26:u32,pub r12:u32,pub r11:u32,pub r10:u32,pub r9:u32,pub r8:i32,pub r7:u32,pub r6:u32,pub r5:u32,pub r4:u32,pub r3:u32,pub r2:u32,pub r1:u32,pub r0:u32,pub sp:u32 }

static arc_regsets: [user_regset; 2] = unsafe { core::mem::zeroed() };
static user_arc_view: user_regset_view = user_regset_view { name: b"arc\0".as_ptr() as *const _, e_machine: EM_ARC_INUSE, regsets: arc_regsets.as_ptr(), n: 2 };

pub unsafe fn task_user_regset_view(_task: *mut task_struct) -> *const user_regset_view { &user_arc_view }

pub unsafe fn ptrace_disable(_child: *mut task_struct) {}

pub unsafe fn arch_ptrace(child: *mut task_struct, request: i64, addr: usize, data: usize) -> i64 {
    let mut ret = -5;
    match request { PTRACE_GET_THREAD_AREA => ret = put_user((*task_thread_info(child)).thr_ptr, data as *mut usize) as i64, _ => ret = ptrace_request(child, request, addr, data) }
    ret
}

pub unsafe fn syscall_trace_enter(regs: *mut pt_regs) -> i32 {
    if test_thread_flag(TIF_SYSCALL_TRACE) && !ptrace_report_syscall_permit_entry(regs) { return usize::MAX as i32; }
    #[cfg(CONFIG_HAVE_SYSCALL_TRACEPOINTS)] if test_thread_flag(TIF_SYSCALL_TRACEPOINT) { trace_sys_enter(regs, syscall_get_nr(current, regs)); }
    (*regs).r8
}

pub unsafe fn syscall_trace_exit(regs: *mut pt_regs) {
    if test_thread_flag(TIF_SYSCALL_TRACE) { ptrace_report_syscall_exit(regs, 0); }
    #[cfg(CONFIG_HAVE_SYSCALL_TRACEPOINTS)] if test_thread_flag(TIF_SYSCALL_TRACEPOINT) { trace_sys_exit(regs, regs_return_value(regs)); }
}

pub unsafe fn regs_query_register_offset(name: *const ::core::ffi::c_char) -> i32 {
    for roff in regoffset_table { if !roff.name.is_null() && strcmp(roff.name, name) == 0 { return roff.offset; } } -22
}

pub unsafe fn regs_query_register_name(offset: u32) -> *const ::core::ffi::c_char {
    for roff in regoffset_table { if !roff.name.is_null() && roff.offset as u32 == offset { return roff.name; } } core::ptr::null()
}

pub unsafe fn regs_within_kernel_stack(regs: *mut pt_regs, addr: usize) -> bool {
    (addr & !(THREAD_SIZE - 1)) == (kernel_stack_pointer(regs) & !(THREAD_SIZE - 1))
}

pub unsafe fn regs_get_kernel_stack_nth(regs: *mut pt_regs, n: u32) -> usize {
    let addr = (kernel_stack_pointer(regs) as *mut usize).add(n as usize);
    if regs_within_kernel_stack(regs, addr as usize) { *addr } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
