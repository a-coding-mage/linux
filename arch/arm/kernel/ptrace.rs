// SPDX-License-Identifier: GPL-2.0-only
/* Translated from linux/arch/arm/kernel/ptrace.c. */

const REG_PC: usize = 15;
const REG_PSR: usize = 16;

#[cfg(not(any()))]
const BREAKINST_ARM: u32 = 0xef9f0001;
#[cfg(not(any()))]
const BREAKINST_THUMB: u32 = 0xdf00;
#[cfg(any())]
const BREAKINST_ARM: u32 = 0xe7f001f0;
#[cfg(any())]
const BREAKINST_THUMB: u32 = 0xde01;

#[repr(C)]
struct pt_regs_offset {
    name: *const core::ffi::c_char,
    offset: i32,
}

static regoffset_table: [pt_regs_offset; 19] = [
    pt_regs_offset { name: b"r0\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_r0) as i32 },
    pt_regs_offset { name: b"r1\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_r1) as i32 },
    pt_regs_offset { name: b"r2\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_r2) as i32 },
    pt_regs_offset { name: b"r3\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_r3) as i32 },
    pt_regs_offset { name: b"r4\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_r4) as i32 },
    pt_regs_offset { name: b"r5\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_r5) as i32 },
    pt_regs_offset { name: b"r6\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_r6) as i32 },
    pt_regs_offset { name: b"r7\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_r7) as i32 },
    pt_regs_offset { name: b"r8\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_r8) as i32 },
    pt_regs_offset { name: b"r9\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_r9) as i32 },
    pt_regs_offset { name: b"r10\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_r10) as i32 },
    pt_regs_offset { name: b"fp\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_fp) as i32 },
    pt_regs_offset { name: b"ip\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_ip) as i32 },
    pt_regs_offset { name: b"sp\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_sp) as i32 },
    pt_regs_offset { name: b"lr\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_lr) as i32 },
    pt_regs_offset { name: b"pc\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_pc) as i32 },
    pt_regs_offset { name: b"cpsr\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_cpsr) as i32 },
    pt_regs_offset { name: b"ORIG_r0\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ARM_ORIG_r0) as i32 },
    pt_regs_offset { name: core::ptr::null(), offset: 0 },
];

pub unsafe fn regs_query_register_offset(name: *const core::ffi::c_char) -> i32 {
    for r in &regoffset_table {
        if r.name.is_null() { break; }
        if strcmp(r.name, name) == 0 { return r.offset; }
    }
    -EINVAL
}

pub unsafe fn regs_query_register_name(offset: u32) -> *const core::ffi::c_char {
    for r in &regoffset_table {
        if r.name.is_null() { break; }
        if r.offset == offset as i32 { return r.name; }
    }
    core::ptr::null()
}

pub unsafe fn regs_within_kernel_stack(regs: *mut pt_regs, addr: ulong) -> bool {
    (addr & !(THREAD_SIZE - 1)) == (kernel_stack_pointer(regs) & !(THREAD_SIZE - 1))
}

pub unsafe fn regs_get_kernel_stack_nth(regs: *mut pt_regs, n: u32) -> ulong {
    let addr = (kernel_stack_pointer(regs) as *mut ulong).add(n as usize);
    if regs_within_kernel_stack(regs, addr as ulong) { *addr } else { 0 }
}

unsafe fn get_user_reg(task: *mut task_struct, offset: i32) -> c_long {
    (*task_pt_regs(task)).uregs[offset as usize]
}

unsafe fn put_user_reg(task: *mut task_struct, offset: i32, data: c_long) -> i32 {
    let regs = task_pt_regs(task);
    let mut newregs = *regs;
    newregs.uregs[offset as usize] = data;
    if valid_user_regs(&mut newregs) { (*regs).uregs[offset as usize] = data; 0 } else { -EINVAL }
}

pub unsafe fn ptrace_disable(_child: *mut task_struct) {}

pub unsafe fn ptrace_break(regs: *mut pt_regs) {
    force_sig_fault(SIGTRAP, TRAP_BRKPT, instruction_pointer(regs) as *mut _);
}

unsafe fn break_trap(regs: *mut pt_regs, _instr: u32) -> i32 { ptrace_break(regs); 0 }

static mut arm_break_hook: undef_hook = undef_hook { instr_mask: 0x0fffffff, instr_val: 0x07f001f0, cpsr_mask: PSR_T_BIT, cpsr_val: 0, fn_: Some(break_trap) };
static mut thumb_break_hook: undef_hook = undef_hook { instr_mask: 0xffffffff, instr_val: 0x0000de01, cpsr_mask: PSR_T_BIT, cpsr_val: PSR_T_BIT, fn_: Some(break_trap) };
static mut thumb2_break_hook: undef_hook = undef_hook { instr_mask: 0xffffffff, instr_val: 0xf7f0a000, cpsr_mask: PSR_T_BIT, cpsr_val: PSR_T_BIT, fn_: Some(break_trap) };

unsafe fn ptrace_break_init() -> i32 {
    register_undef_hook(&mut arm_break_hook); register_undef_hook(&mut thumb_break_hook); register_undef_hook(&mut thumb2_break_hook); 0
}

// core_initcall(ptrace_break_init)

unsafe fn ptrace_read_user(tsk: *mut task_struct, off: ulong, ret: *mut ulong) -> i32 {
    if off & 3 != 0 { return -EIO; }
    let mut tmp = 0;
    if off == PT_TEXT_ADDR { tmp = (*(*tsk).mm).start_code; }
    else if off == PT_DATA_ADDR { tmp = (*(*tsk).mm).start_data; }
    else if off == PT_TEXT_END_ADDR { tmp = (*(*tsk).mm).end_code; }
    else if off < core::mem::size_of::<pt_regs>() as ulong { tmp = get_user_reg(tsk, (off >> 2) as i32) as ulong; }
    else if off >= core::mem::size_of::<user>() as ulong { return -EIO; }
    put_user(tmp, ret)
}

unsafe fn ptrace_write_user(tsk: *mut task_struct, off: ulong, val: ulong) -> i32 {
    if off & 3 != 0 || off >= core::mem::size_of::<user>() as ulong { return -EIO; }
    if off >= core::mem::size_of::<pt_regs>() as ulong { return 0; }
    put_user_reg(tsk, (off >> 2) as i32, val as c_long)
}

#[cfg(CONFIG_IWMMXT)]
unsafe fn ptrace_getwmmxregs(tsk: *mut task_struct, ufp: *mut core::ffi::c_void) -> i32 {
    let thread = task_thread_info(tsk);
    if !test_ti_thread_flag(thread, TIF_USING_IWMMXT) { return -ENODATA; }
    iwmmxt_task_disable(thread);
    if copy_to_user(ufp, &(*thread).fpstate.iwmmxt as *const _, IWMMXT_SIZE) != 0 { -EFAULT } else { 0 }
}

#[cfg(CONFIG_IWMMXT)]
unsafe fn ptrace_setwmmxregs(tsk: *mut task_struct, ufp: *const core::ffi::c_void) -> i32 {
    let thread = task_thread_info(tsk);
    if !test_ti_thread_flag(thread, TIF_USING_IWMMXT) { return -EACCES; }
    iwmmxt_task_release(thread);
    if copy_from_user(&mut (*thread).fpstate.iwmmxt as *mut _, ufp, IWMMXT_SIZE) != 0 { -EFAULT } else { 0 }
}

unsafe fn gpr_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 {
    membuf_write(&mut to, task_pt_regs(target) as *const _, core::mem::size_of::<pt_regs>())
}
unsafe fn gpr_set(target: *mut task_struct, _r: *const user_regset, mut pos: u32, mut count: u32, kbuf: *const _, ubuf: *const _) -> i32 {
    let mut newregs = *task_pt_regs(target);
    let ret = user_regset_copyin(&mut pos, &mut count, &mut (kbuf as *const _), &mut (ubuf as *const _), &mut newregs as *mut _, 0, core::mem::size_of::<pt_regs>());
    if ret != 0 { return ret; }
    if !valid_user_regs(&mut newregs) { return -EINVAL; }
    *task_pt_regs(target) = newregs; 0
}
unsafe fn fpa_get(target: *mut task_struct, _r: *const user_regset, mut to: membuf) -> i32 {
    membuf_write(&mut to, &(*task_thread_info(target)).fpstate as *const _, core::mem::size_of::<user_fp>())
}
unsafe fn fpa_set(target: *mut task_struct, _r: *const user_regset, mut pos: u32, mut count: u32, kbuf: *const _, ubuf: *const _) -> i32 {
    user_regset_copyin(&mut pos, &mut count, &mut (kbuf as *const _), &mut (ubuf as *const _), &mut (*task_thread_info(target)).fpstate as *mut _, 0, core::mem::size_of::<user_fp>())
}

#[cfg(CONFIG_VFP)]
unsafe fn vfp_get(target: *mut task_struct, _r: *const user_regset, mut to: membuf) -> i32 {
    let thread = task_thread_info(target); let vfp = &(*thread).vfpstate.hard;
    let fpscr = core::mem::offset_of!(user_vfp, fpscr);
    vfp_sync_hwstate(thread);
    membuf_write(&mut to, vfp.fpregs.as_ptr() as *const _, core::mem::size_of_val(&vfp.fpregs));
    membuf_zero(&mut to, fpscr - core::mem::size_of_val(&vfp.fpregs));
    membuf_store(&mut to, vfp.fpscr)
}
#[cfg(CONFIG_VFP)]
unsafe fn vfp_set(target: *mut task_struct, _r: *const user_regset, mut pos: u32, mut count: u32, kbuf: *const _, ubuf: *const _) -> i32 {
    let thread = task_thread_info(target); vfp_sync_hwstate(thread); let mut nv = (*thread).vfpstate.hard;
    let fp = core::mem::offset_of!(user_vfp, fpregs); let fs = core::mem::offset_of!(user_vfp, fpscr);
    let mut kb = kbuf as *const _; let mut ub = ubuf as *const _;
    let ret = user_regset_copyin(&mut pos,&mut count,&mut kb,&mut ub,&mut nv.fpregs as *mut _,fp,fp+core::mem::size_of_val(&nv.fpregs)); if ret != 0 { return ret; }
    user_regset_copyin_ignore(&mut pos,&mut count,&mut kb,&mut ub,fp+core::mem::size_of_val(&nv.fpregs),fs);
    let ret = user_regset_copyin(&mut pos,&mut count,&mut kb,&mut ub,&mut nv.fpscr as *mut _,fs,fs+4); if ret != 0 { return ret; }
    (*thread).vfpstate.hard=nv; vfp_flush_hwstate(thread); 0
}

#[repr(C)]
enum arm_regset { REGSET_GPR, REGSET_FPR, #[cfg(CONFIG_VFP)] REGSET_VFP }

// The user_regset table and architecture-specific regset callbacks retain the
// kernel ABI layout; USER_REGSET_NOTE_TYPE initializers are supplied externally.
static user_arm_view: user_regset_view = user_regset_view { name: b"arm\0".as_ptr() as _, e_machine: ELF_ARCH, ei_osabi: ELF_OSABI, regsets: arm_regsets.as_ptr(), n: arm_regsets.len() };

pub unsafe fn task_user_regset_view(_task: *mut task_struct) -> *const user_regset_view { &user_arm_view }

pub unsafe fn arch_ptrace(child: *mut task_struct, request: c_long, addr: ulong, data: ulong) -> c_long {
    let datap = data as *mut ulong;
    match request {
        PTRACE_PEEKUSR => ptrace_read_user(child,addr,datap) as c_long,
        PTRACE_POKEUSR => ptrace_write_user(child,addr,data) as c_long,
        PTRACE_GETREGS => copy_regset_to_user(child,&user_arm_view,REGSET_GPR,0,core::mem::size_of::<pt_regs>(),datap) as c_long,
        PTRACE_SETREGS => copy_regset_from_user(child,&user_arm_view,REGSET_GPR,0,core::mem::size_of::<pt_regs>(),datap) as c_long,
        PTRACE_GETFPREGS => copy_regset_to_user(child,&user_arm_view,REGSET_FPR,0,core::mem::size_of::<fp_state>(),datap) as c_long,
        PTRACE_SETFPREGS => copy_regset_from_user(child,&user_arm_view,REGSET_FPR,0,core::mem::size_of::<fp_state>(),datap) as c_long,
        PTRACE_GET_THREAD_AREA => put_user((*task_thread_info(child)).tp_value[0],datap) as c_long,
        PTRACE_SET_SYSCALL => { (*task_thread_info(child)).abi_syscall=if data != !0 { data & __NR_SYSCALL_MASK } else { data }; 0 },
        _ => ptrace_request(child,request,addr,data) as c_long,
    }
}

#[repr(C)]
enum ptrace_syscall_dir { PTRACE_SYSCALL_ENTER=0, PTRACE_SYSCALL_EXIT }
unsafe fn report_syscall(regs: *mut pt_regs, dir: ptrace_syscall_dir) { let ip=(*regs).ARM_ip; (*regs).ARM_ip=dir as _; if matches!(dir,ptrace_syscall_dir::PTRACE_SYSCALL_EXIT) { ptrace_report_syscall_exit(regs,0); } else if !ptrace_report_syscall_permit_entry(regs) { (*current_thread_info()).abi_syscall= -1; } (*regs).ARM_ip=ip; }
pub unsafe fn syscall_trace_enter(regs: *mut pt_regs) -> i32 { if test_thread_flag(TIF_SYSCALL_TRACE) { report_syscall(regs,ptrace_syscall_dir::PTRACE_SYSCALL_ENTER); } if !seccomp_permit_syscall() { return -1; } let scno=syscall_get_nr(current,regs); if test_thread_flag(TIF_SYSCALL_TRACEPOINT) { trace_sys_enter(regs,scno); } audit_syscall_entry(scno,(*regs).ARM_r0,(*regs).ARM_r1,(*regs).ARM_r2,(*regs).ARM_r3); scno }
pub unsafe fn syscall_trace_exit(regs: *mut pt_regs) { audit_syscall_exit(regs); if test_thread_flag(TIF_SYSCALL_TRACEPOINT) { trace_sys_exit(regs,regs_return_value(regs)); } if test_thread_flag(TIF_SYSCALL_TRACE) { report_syscall(regs,ptrace_syscall_dir::PTRACE_SYSCALL_EXIT); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
