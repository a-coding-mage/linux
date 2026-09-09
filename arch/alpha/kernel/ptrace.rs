// SPDX-License-Identifier: GPL-2.0
/* ptrace.c - source-level Rust translation */

// Kernel headers and architecture dependencies are supplied externally.

const BREAKINST: u32 = 0x00000080; /* call_pal bpt */

enum RegIndex { REG_R0 = 0, REG_F0 = 32, REG_FPCR = 63, REG_PC = 64 }

// These macros depend on the kernel's C layout definitions.
macro_rules! PT_REG { ($r:ident) => { PAGE_SIZE * 2 - core::mem::size_of::<pt_regs>() + core::mem::offset_of!(pt_regs, $r) }; }
macro_rules! SW_REG { ($r:ident) => { PAGE_SIZE * 2 - core::mem::size_of::<pt_regs>() - core::mem::size_of::<switch_stack>() + core::mem::offset_of!(switch_stack, $r) }; }
macro_rules! FP_REG { ($r:ident) => { core::mem::offset_of!(thread_info, $r) }; }

static mut regoff: [isize; 65] = [
    PT_REG!(r0), PT_REG!(r1), PT_REG!(r2), PT_REG!(r3), PT_REG!(r4), PT_REG!(r5), PT_REG!(r6), PT_REG!(r7),
    PT_REG!(r8), SW_REG!(r9), SW_REG!(r10), SW_REG!(r11), SW_REG!(r12), SW_REG!(r13), SW_REG!(r14), SW_REG!(r15),
    PT_REG!(r16), PT_REG!(r17), PT_REG!(r18), PT_REG!(r19), PT_REG!(r20), PT_REG!(r21), PT_REG!(r22), PT_REG!(r23),
    PT_REG!(r24), PT_REG!(r25), PT_REG!(r26), PT_REG!(r27), PT_REG!(r28), PT_REG!(gp), -1, -1,
    FP_REG!(fp[0]), FP_REG!(fp[1]), FP_REG!(fp[2]), FP_REG!(fp[3]), FP_REG!(fp[4]), FP_REG!(fp[5]), FP_REG!(fp[6]), FP_REG!(fp[7]),
    FP_REG!(fp[8]), FP_REG!(fp[9]), FP_REG!(fp[10]), FP_REG!(fp[11]), FP_REG!(fp[12]), FP_REG!(fp[13]), FP_REG!(fp[14]), FP_REG!(fp[15]),
    FP_REG!(fp[16]), FP_REG!(fp[17]), FP_REG!(fp[18]), FP_REG!(fp[19]), FP_REG!(fp[20]), FP_REG!(fp[21]), FP_REG!(fp[22]), FP_REG!(fp[23]),
    FP_REG!(fp[24]), FP_REG!(fp[25]), FP_REG!(fp[26]), FP_REG!(fp[27]), FP_REG!(fp[28]), FP_REG!(fp[29]), FP_REG!(fp[30]), FP_REG!(fp[31]),
    PT_REG!(pc)
];
static mut zero: c_ulong = 0;

unsafe fn get_reg_addr(task: *mut task_struct, regno: c_ulong) -> *mut c_ulong {
    if regno == 30 { &mut (*task_thread_info(task)).pcb.usp }
    else if regno == 65 { &mut (*task_thread_info(task)).pcb.unique }
    else if regno == 31 || regno > 65 { zero = 0; &mut zero }
    else { (task_stack_page(task) as *mut u8).offset(regoff[regno as usize]) as *mut c_ulong }
}

unsafe fn get_reg(task: *mut task_struct, regno: c_ulong) -> c_ulong {
    if regno == 63 {
        let fpcr = *get_reg_addr(task, regno);
        let swcr = swcr_update_status((*task_thread_info(task)).ieee_state & IEEE_SW_MASK, fpcr);
        return fpcr | swcr;
    }
    *get_reg_addr(task, regno)
}

unsafe fn alpha_elf_fpregs_get(target: *mut task_struct, fpregs: *mut elf_fpreg_t) {
    core::ptr::copy_nonoverlapping((*task_thread_info(target)).fp.as_ptr(), fpregs, ELF_NFPREG);
}
unsafe fn alpha_elf_fpregs_set(target: *mut task_struct, fpregs: *const elf_fpreg_t, nwords: usize) {
    core::ptr::copy_nonoverlapping(fpregs, (*task_thread_info(target)).fp.as_mut_ptr(), core::cmp::min(nwords, ELF_NFPREG));
}

unsafe fn alpha_elf_gregs_set(child: *mut task_struct, src: *const elf_greg_t, nwords: usize) {
    let pt = task_pt_regs(child); let ti = task_thread_info(child);
    let sw = (pt as *mut switch_stack).offset(-1);
    macro_rules! set { ($n:expr, $p:expr) => { if nwords > $n { $p = *src.add($n); } }; }
    set!(0, (*pt).r0); set!(1, (*pt).r1); set!(2, (*pt).r2); set!(3, (*pt).r3); set!(4, (*pt).r4); set!(5, (*pt).r5); set!(6, (*pt).r6); set!(7, (*pt).r7); set!(8, (*pt).r8);
    set!(9, (*sw).r9); set!(10, (*sw).r10); set!(11, (*sw).r11); set!(12, (*sw).r12); set!(13, (*sw).r13); set!(14, (*sw).r14); set!(15, (*sw).r15);
    set!(16, (*pt).r16); set!(17, (*pt).r17); set!(18, (*pt).r18); set!(19, (*pt).r19); set!(20, (*pt).r20); set!(21, (*pt).r21); set!(22, (*pt).r22); set!(23, (*pt).r23); set!(24, (*pt).r24); set!(25, (*pt).r25); set!(26, (*pt).r26); set!(27, (*pt).r27); set!(28, (*pt).r28);
    if nwords > 29 { (*pt).gp = *src.add(29); }
    if nwords > 30 { (*ti).pcb.usp = *src.add(30); if child == current { wrusp(*src.add(30)); } }
    if nwords > 31 { (*pt).pc = *src.add(31); }
    if nwords > 32 { (*ti).pcb.unique = *src.add(32); }
    if (*pt).r1 == c_ulong::MAX && (*pt).r19 == 0 && (*pt).r0 > 0 && (*pt).r0 < MAX_ERRNO { (*pt).r19 = 1; }
}

unsafe fn put_reg(task: *mut task_struct, regno: c_ulong, mut data: c_ulong) -> c_int {
    let regs = task_pt_regs(task);
    if regno == 63 { (*task_thread_info(task)).ieee_state = ((*task_thread_info(task)).ieee_state & !IEEE_SW_MASK) | (data & IEEE_SW_MASK); data = (data & FPCR_DYN_MASK) | ieee_swcr_to_fpcr(data); }
    *get_reg_addr(task, regno) = data;
    if regno == 0 && data == c_ulong::MAX { (*regs).r1 = data; (*regs).r19 = 0; }
    0
}

unsafe fn read_int(task: *mut task_struct, addr: c_ulong, data: *mut c_int) -> c_int { let copied = access_process_vm(task, addr, data as *mut _, core::mem::size_of::<c_int>(), FOLL_FORCE); if copied == core::mem::size_of::<c_int>() { 0 } else { -EIO } }
unsafe fn write_int(task: *mut task_struct, addr: c_ulong, data: c_int) -> c_int { let mut d = data; let copied = access_process_vm(task, addr, &mut d as *mut _ as *mut _, core::mem::size_of::<c_int>(), FOLL_FORCE | FOLL_WRITE); if copied == core::mem::size_of::<c_int>() { 0 } else { -EIO } }

pub unsafe fn ptrace_set_bpt(child: *mut task_struct) -> c_int {
    let mut insn: u32 = 0; let pc = get_reg(child, REG_PC as c_ulong); let mut res = read_int(child, pc, &mut insn as *mut _ as *mut c_int); if res < 0 { return res; }
    let op_code = insn >> 26; let mut nsaved = 0; let mut addrs = [0 as c_ulong; 2];
    if op_code >= 0x30 { let displ = ((insn << 11) as i32 >> 9) as i64; addrs[nsaved] = pc + 4; nsaved += 1; if displ != 0 { addrs[nsaved] = (pc as i64 + 4 + displ) as c_ulong; nsaved += 1; } }
    else if op_code == 0x1a { let reg_b = ((insn >> 16) & 0x1f) as c_ulong; addrs[nsaved] = get_reg(child, reg_b); nsaved += 1; }
    else { addrs[nsaved] = pc + 4; nsaved += 1; }
    for i in 0..nsaved { res = read_int(child, addrs[i], &mut insn as *mut _ as *mut c_int); if res < 0 { return res; } (*task_thread_info(child)).bpt_addr[i] = addrs[i]; (*task_thread_info(child)).bpt_insn[i] = insn; res = write_int(child, addrs[i], BREAKINST as c_int); if res < 0 { return res; } }
    (*task_thread_info(child)).bpt_nsaved = nsaved as c_int; 0
}

pub unsafe fn ptrace_cancel_bpt(child: *mut task_struct) -> c_int { let mut nsaved = (*task_thread_info(child)).bpt_nsaved; (*task_thread_info(child)).bpt_nsaved = 0; if nsaved > 2 { nsaved = 2; } for i in 0..nsaved { write_int(child, (*task_thread_info(child)).bpt_addr[i as usize], (*task_thread_info(child)).bpt_insn[i as usize]); } (nsaved != 0) as c_int }
pub unsafe fn user_enable_single_step(child: *mut task_struct) { (*task_thread_info(child)).bpt_nsaved = -1; }
pub unsafe fn user_disable_single_step(child: *mut task_struct) { ptrace_cancel_bpt(child); }
pub unsafe fn ptrace_disable(child: *mut task_struct) { user_disable_single_step(child); }

pub unsafe fn arch_ptrace(child: *mut task_struct, request: c_long, addr: c_ulong, data: c_ulong) -> c_long {
    let mut tmp = 0 as c_ulong;
    match request {
        PTRACE_PEEKTEXT | PTRACE_PEEKDATA => { let copied = ptrace_access_vm(child, addr, &mut tmp as *mut _ as *mut _, core::mem::size_of::<c_ulong>(), FOLL_FORCE); if copied != core::mem::size_of::<c_ulong>() { -EIO } else { force_successful_syscall_return(); tmp as c_long } }
        PTRACE_PEEKUSR => { force_successful_syscall_return(); get_reg(child, addr) as c_long }
        PTRACE_POKETEXT | PTRACE_POKEDATA => generic_ptrace_pokedata(child, addr, data),
        PTRACE_POKEUSR => put_reg(child, addr, data) as c_long,
        _ => ptrace_request(child, request, addr, data),
    }
}

pub unsafe fn syscall_trace_enter() -> c_ulong {
    let regs = current_pt_regs();
    if test_thread_flag(TIF_SYSCALL_TRACE) && !ptrace_report_syscall_permit_entry(regs) { syscall_set_nr(current, regs, -1); if (*regs).r19 == 0 && (*regs).r0 == c_ulong::MAX { syscall_set_return_value(current, regs, -ENOSYS, 0); } return c_ulong::MAX; }
    if !seccomp_permit_syscall() { if (*regs).r19 == 0 && (*regs).r0 == c_ulong::MAX { syscall_set_return_value(current, regs, -ENOSYS, 0); } syscall_set_nr(current, regs, -1); return c_ulong::MAX; }
    #[cfg(CONFIG_AUDITSYSCALL)] audit_syscall_entry(syscall_get_nr(current, regs), (*regs).r16, (*regs).r17, (*regs).r18, (*regs).r19);
    syscall_get_nr(current, regs)
}
pub unsafe fn syscall_trace_leave() { audit_syscall_exit(current_pt_regs()); if test_thread_flag(TIF_SYSCALL_TRACE) { ptrace_report_syscall_exit(current_pt_regs(), 0); } }

unsafe fn alpha_regset_set(target: *mut task_struct, _regset: *const user_regset, mut pos: c_uint, mut count: c_uint, kbuf: *const c_void, ubuf: *const c_void) -> c_int { let mut gregs: elf_gregset_t = core::mem::zeroed(); if pos + count > core::mem::size_of_val(&gregs) as c_uint { return -EIO; } dump_elf_thread(gregs.as_mut_ptr(), task_pt_regs(target), task_thread_info(target)); if user_regset_copyin(&mut pos, &mut count, &kbuf, &mut (ubuf as *const c_void), gregs.as_mut_ptr(), 0, core::mem::size_of_val(&gregs)) != 0 { return -EFAULT; } alpha_elf_gregs_set(target, gregs.as_ptr(), core::mem::size_of_val(&gregs) / core::mem::size_of::<elf_greg_t>()); 0 }
unsafe fn alpha_fpregset_set(target: *mut task_struct, _regset: *const user_regset, mut pos: c_uint, mut count: c_uint, kbuf: *const c_void, ubuf: *const c_void) -> c_int { let mut fpregs: elf_fpregset_t = core::mem::zeroed(); if pos + count > core::mem::size_of_val(&fpregs) as c_uint { return -EIO; } alpha_elf_fpregs_get(target, fpregs.as_mut_ptr()); if user_regset_copyin(&mut pos, &mut count, &kbuf, &mut (ubuf as *const c_void), fpregs.as_mut_ptr(), 0, core::mem::size_of_val(&fpregs)) != 0 { return -EFAULT; } alpha_elf_fpregs_set(target, fpregs.as_ptr(), core::mem::size_of_val(&fpregs) / core::mem::size_of::<elf_fpreg_t>()); 0 }
unsafe fn alpha_regset_get(target: *mut task_struct, _regset: *const user_regset, to: membuf) -> c_int { let mut g: elf_gregset_t = core::mem::zeroed(); dump_elf_thread(g.as_mut_ptr(), task_pt_regs(target), task_thread_info(target)); membuf_write(&to, g.as_ptr() as *const _, core::mem::size_of_val(&g)) }
unsafe fn alpha_fpregset_get(target: *mut task_struct, _regset: *const user_regset, to: membuf) -> c_int { let mut f: elf_fpregset_t = core::mem::zeroed(); alpha_elf_fpregs_get(target, f.as_mut_ptr()); membuf_write(&to, f.as_ptr() as *const _, core::mem::size_of_val(&f)) }

#[repr(C)] pub enum alpha_regset { REGSET_GPR, REGSET_FPR }
// Equivalent ABI descriptors for the two Alpha register sets.
#[no_mangle]
pub static alpha_user_regsets: [user_regset; 2] = [
    user_regset { core_note_type: NT_PRSTATUS, n: ELF_NGREG, size: core::mem::size_of::<elf_greg_t>(), align: core::mem::size_of::<elf_greg_t>(), regset_get: Some(alpha_regset_get), set: Some(alpha_regset_set), ..unsafe { core::mem::zeroed() } },
    user_regset { core_note_type: NT_PRFPREG, core_note_name: b"CORE\0".as_ptr() as *const _, n: ELF_NFPREG, size: core::mem::size_of::<elf_fpreg_t>(), align: core::mem::size_of::<elf_fpreg_t>(), regset_get: Some(alpha_fpregset_get), set: Some(alpha_fpregset_set), ..unsafe { core::mem::zeroed() } },
];
#[no_mangle]
pub static user_alpha_view: user_regset_view = user_regset_view { name: b"alpha\0".as_ptr() as *const _, e_machine: EM_ALPHA, ei_osabi: ELF_OSABI, regsets: alpha_user_regsets.as_ptr(), n: 2 };
pub unsafe fn task_user_regset_view(_task: *mut task_struct) -> *const user_regset_view { &user_alpha_view }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
