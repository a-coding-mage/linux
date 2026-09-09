/*
 * Copyright (C) 2014 Altera Corporation
 * Copyright (C) 2010 Tobias Klauser <tklauser@distanz.ch>
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file COPYING in the main directory of this
 * archive for more details.
 */

unsafe fn genregs_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 {
    let regs: *const pt_regs = task_pt_regs(target);
    let sw: *const switch_stack = (regs as *const switch_stack).offset(-1);
    membuf_zero(&mut to, 4); // R0
    membuf_write(&mut to, &(*regs).r1 as *const _, 7 * 4); // R1..R7
    membuf_write(&mut to, &(*regs).r8 as *const _, 8 * 4); // R8..R15
    membuf_write(&mut to, sw, 8 * 4); // R16..R23
    membuf_zero(&mut to, 2 * 4); /* et and bt */
    membuf_store(&mut to, (*regs).gp);
    membuf_store(&mut to, (*regs).sp);
    membuf_store(&mut to, (*regs).fp);
    membuf_store(&mut to, (*regs).ea);
    membuf_zero(&mut to, 4); // PTR_BA
    membuf_store(&mut to, (*regs).ra);
    membuf_store(&mut to, (*regs).ea); /* use ea for PC */
    membuf_zero(&mut to, (NUM_PTRACE_REG - PTR_PC) * 4)
}

/* Set the thread state from a regset passed in via ptrace */
unsafe fn genregs_set(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, mut kbuf: *const core::ffi::c_void, mut ubuf: *const core::ffi::c_void) -> i32 {
    let regs: *mut pt_regs = task_pt_regs(target);
    let sw: *const switch_stack = (regs as *const switch_stack).offset(-1);
    let mut ret: i32 = 0;
    macro_rules! reg_ignore_range { ($start:expr, $end:expr) => { if ret == 0 { user_regset_copyin_ignore(&mut pos, &mut count, &mut kbuf, &mut ubuf, $start * 4, ($end * 4) + 4); } }; }
    macro_rules! reg_in_one { ($ptr:expr, $loc:expr) => { if ret == 0 { ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf, $ptr as *mut core::ffi::c_void, $loc * 4, ($loc * 4) + 4); } }; }
    macro_rules! reg_in_range { ($ptr:expr, $start:expr, $end:expr) => { if ret == 0 { ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf, $ptr as *mut core::ffi::c_void, $start * 4, ($end * 4) + 4); } }; }
    reg_ignore_range!(PTR_R0, PTR_R0);
    reg_in_range!(&mut (*regs).r1, PTR_R1, PTR_R7);
    reg_in_range!(&mut (*regs).r8, PTR_R8, PTR_R15);
    reg_in_range!(sw, PTR_R16, PTR_R23);
    reg_ignore_range!(PTR_R24, PTR_R25); /* et and bt */
    reg_in_one!(&mut (*regs).gp, PTR_GP);
    reg_in_one!(&mut (*regs).sp, PTR_SP);
    reg_in_one!(&mut (*regs).fp, PTR_FP);
    reg_in_one!(&mut (*regs).ea, PTR_EA);
    reg_ignore_range!(PTR_BA, PTR_BA);
    reg_in_one!(&mut (*regs).ra, PTR_RA);
    reg_in_one!(&mut (*regs).ea, PTR_PC); /* use ea for PC */
    if ret == 0 { user_regset_copyin_ignore(&mut pos, &mut count, &mut kbuf, &mut ubuf, PTR_STATUS * 4, -1); }
    ret
}

/* Define the register sets available on Nios2 under Linux */
#[repr(C)]
enum nios2_regset { REGSET_GENERAL }

static nios2_regsets: [user_regset; 1] = [user_regset {
    note_type: USER_REGSET_NOTE_TYPE(PRSTATUS), n: NUM_PTRACE_REG,
    size: core::mem::size_of::<c_ulong>(), align: core::mem::size_of::<c_ulong>(),
    regset_get: Some(genregs_get), set: Some(genregs_set),
}];

static nios2_user_view: user_regset_view = user_regset_view {
    name: "nios2", e_machine: ELF_ARCH, ei_osabi: ELF_OSABI,
    regsets: nios2_regsets.as_ptr(), n: nios2_regsets.len(),
};

pub unsafe fn task_user_regset_view(_task: *mut task_struct) -> *const user_regset_view { &nios2_user_view }
unsafe fn ptrace_disable(_child: *mut task_struct) {}
unsafe fn arch_ptrace(child: *mut task_struct, request: c_long, addr: c_ulong, data: c_ulong) -> c_long { ptrace_request(child, request, addr, data) }

unsafe fn do_syscall_trace_enter() -> i32 {
    let mut ret = 0;
    if test_thread_flag(TIF_SYSCALL_TRACE) { ret = (!ptrace_report_syscall_permit_entry(task_pt_regs(current))) as i32; }
    ret
}

unsafe fn do_syscall_trace_exit() {
    if test_thread_flag(TIF_SYSCALL_TRACE) { ptrace_report_syscall_exit(task_pt_regs(current), 0); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
