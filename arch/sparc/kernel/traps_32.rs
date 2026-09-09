// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sparc/kernel/traps.c
 *
 * Copyright 1995, 2008 David S. Miller (davem@davemloft.net)
 * Copyright 2000 Jakub Jelinek (jakub@redhat.com)
 */

/* I hate traps on the sparc, grrr... */

// C dependencies supplied by the surrounding kernel translation unit.

unsafe fn instruction_dump(pc: *mut ::core::ffi::c_ulong) {
    if (pc as usize) & 3 != 0 { return; }
    for i in -3..6 {
        unsafe { printk(b"%c%08lx%c\0".as_ptr(), if i != 0 { b' ' } else { b'<' }, *pc.offset(i), if i != 0 { b' ' } else { b'>' }); }
    }
    unsafe { printk(b"\n\0".as_ptr()); }
}

// __SAVE / __RESTORE are the original SPARC inline assembly operations.
macro_rules! __SAVE { () => { unsafe { core::arch::asm!("save %sp, -0x40, %sp"); } }; }
macro_rules! __RESTORE { () => { unsafe { core::arch::asm!("restore %g0, %g0, %g0"); } }; }

pub unsafe fn die_if_kernel(str_: *mut ::core::ffi::c_char, regs: *mut pt_regs) -> ! {
    static mut DIE_COUNTER: i32 = 0;
    let mut count = 0;
    unsafe { printk(b"              \\|/ ____ \\|/\n              \"@'/ ,. \\`@\"\n              /_| \\__/ |_\\\n                 \\__U_/\n\0".as_ptr()); }
    unsafe { DIE_COUNTER += 1; printk(b"%s(%d): %s [#%d]\n\0".as_ptr(), current.comm.as_ptr(), task_pid_nr(current), str_, DIE_COUNTER); }
    unsafe { show_regs(regs); add_taint(TAINT_DIE, LOCKDEP_NOW_UNRELIABLE); }
    __SAVE!(); __SAVE!(); __SAVE!(); __SAVE!(); __SAVE!(); __SAVE!(); __SAVE!(); __SAVE!();
    __RESTORE!(); __RESTORE!(); __RESTORE!(); __RESTORE!(); __RESTORE!(); __RESTORE!(); __RESTORE!(); __RESTORE!();
    unsafe {
        let mut rw = (*regs).u_regs[UREG_FP] as *mut reg_window32;
        while !rw.is_null() && { count += 1; count } < 30 && (rw as usize) >= PAGE_OFFSET && (rw as usize) & 7 == 0 {
            printk(b"Caller[%08lx]: %pS\n\0".as_ptr(), (*rw).ins[7], (*rw).ins[7] as *mut ::core::ffi::c_void);
            rw = (*rw).ins[6] as *mut reg_window32;
        }
        printk(b"Instruction DUMP:\0".as_ptr());
        instruction_dump((*regs).pc as *mut ::core::ffi::c_ulong);
        make_task_dead(if (*regs).psr & PSR_PS != 0 { SIGKILL } else { SIGSEGV });
    }
}

pub unsafe fn do_hw_interrupt(regs: *mut pt_regs, type_: ::core::ffi::c_ulong) {
    if type_ < 0x80 { unsafe { printk(b"Unimplemented Sparc TRAP, type = %02lx\n\0".as_ptr(), type_); die_if_kernel(b"Whee... Hello Mr. Penguin\0".as_ptr() as *mut _ , regs); } }
    unsafe { if (*regs).psr & PSR_PS != 0 { die_if_kernel(b"Kernel bad trap\0".as_ptr() as *mut _, regs); } force_sig_fault_trapno(SIGILL, ILL_ILLTRP, (*regs).pc as *mut _, type_ - 0x80); }
}

pub unsafe fn do_illegal_instruction(regs: *mut pt_regs, pc: ::core::ffi::c_ulong, _npc: ::core::ffi::c_ulong, psr: ::core::ffi::c_ulong) {
    unsafe { if psr & PSR_PS != 0 { die_if_kernel(b"Kernel illegal instruction\0".as_ptr() as *mut _, regs); } send_sig_fault(SIGILL, ILL_ILLOPC, pc as *mut _, current); }
}

pub unsafe fn do_priv_instruction(regs: *mut pt_regs, pc: ::core::ffi::c_ulong, _npc: ::core::ffi::c_ulong, psr: ::core::ffi::c_ulong) {
    unsafe { if psr & PSR_PS != 0 { die_if_kernel(b"Penguin instruction from Penguin mode??!?!\0".as_ptr() as *mut _, regs); } send_sig_fault(SIGILL, ILL_PRVOPC, pc as *mut _, current); }
}

pub unsafe fn do_memaccess_unaligned(regs: *mut pt_regs, pc: ::core::ffi::c_ulong, npc: ::core::ffi::c_ulong, _psr: ::core::ffi::c_ulong) {
    unsafe { if (*regs).psr & PSR_PS != 0 { printk(b"KERNEL MNA at pc %08lx npc %08lx called by %08lx\n\0".as_ptr(), pc, npc, (*regs).u_regs[UREG_RETPC]); die_if_kernel(b"BOGUS\0".as_ptr() as *mut _, regs); } send_sig_fault(SIGBUS, BUS_ADRALN, 0 as *mut _, current); }
}

static mut init_fsr: ::core::ffi::c_ulong = 0;
#[repr(align(8))] static mut init_fregs: [::core::ffi::c_ulong; 32] = [::core::ffi::c_ulong::MAX; 32];

pub unsafe fn do_fpd_trap(regs: *mut pt_regs, _pc: ::core::ffi::c_ulong, _npc: ::core::ffi::c_ulong, psr: ::core::ffi::c_ulong) {
    unsafe { if psr & PSR_PS != 0 { die_if_kernel(b"Kernel gets FloatingPenguinUnit disabled trap\0".as_ptr() as *mut _, regs); } put_psr(get_psr() | PSR_EF); (*regs).psr |= PSR_EF; }
    // CONFIG_SMP selects the corresponding original C branch.
    #[cfg(not(CONFIG_SMP))] unsafe {
        if last_task_used_math == current { return; }
        if !last_task_used_math.is_null() { let fptask = last_task_used_math; fpsave((*fptask).thread.float_regs.as_mut_ptr(), &mut (*fptask).thread.fsr, (*fptask).thread.fpqueue.as_mut_ptr(), &mut (*fptask).thread.fpqdepth); }
        last_task_used_math = current;
        if used_math() { fpload((*current).thread.float_regs.as_mut_ptr(), &mut (*current).thread.fsr); } else { fpload(init_fregs.as_mut_ptr(), &mut init_fsr); set_used_math(); }
    }
    #[cfg(CONFIG_SMP)] unsafe { if !used_math() { fpload(init_fregs.as_mut_ptr(), &mut init_fsr); set_used_math(); } else { fpload((*current).thread.float_regs.as_mut_ptr(), &mut (*current).thread.fsr); } set_thread_flag(TIF_USEDFPU); }
}

#[repr(align(8))] static mut fake_regs: [::core::ffi::c_ulong; 32] = [0; 32];
static mut fake_fsr: ::core::ffi::c_ulong = 0;
#[repr(align(8))] static mut fake_queue: [::core::ffi::c_ulong; 32] = [0; 32];
static mut fake_depth: ::core::ffi::c_ulong = 0;

pub unsafe fn do_fpe_trap(regs: *mut pt_regs, pc: ::core::ffi::c_ulong, _npc: ::core::ffi::c_ulong, psr: ::core::ffi::c_ulong) {
    static mut calls: i32 = 0; let mut fsr; let mut ret = 0; let mut code;
    #[cfg(not(CONFIG_SMP))] let fpt = unsafe { last_task_used_math };
    #[cfg(CONFIG_SMP)] let fpt = unsafe { current };
    unsafe { put_psr(get_psr() | PSR_EF); }
    #[cfg(not(CONFIG_SMP))] if fpt.is_null() { unsafe { fpsave(fake_regs.as_mut_ptr(), &mut fake_fsr, fake_queue.as_mut_ptr(), &mut fake_depth); (*regs).psr &= !PSR_EF; } return; }
    #[cfg(CONFIG_SMP)] if unsafe { !test_tsk_thread_flag(fpt, TIF_USEDFPU) } { unsafe { fpsave(fake_regs.as_mut_ptr(), &mut fake_fsr, fake_queue.as_mut_ptr(), &mut fake_depth); (*regs).psr &= !PSR_EF; } return; }
    unsafe { fpsave((*fpt).thread.float_regs.as_mut_ptr(), &mut (*fpt).thread.fsr, (*fpt).thread.fpqueue.as_mut_ptr(), &mut (*fpt).thread.fpqdepth); }
    unsafe { match (*fpt).thread.fsr & 0x1c000 { 2 << 14 | 3 << 14 => { ret = do_mathemu(regs, fpt); }, _ => {} } }
    if ret != 0 { unsafe { fpload((*current).thread.float_regs.as_mut_ptr(), &mut (*current).thread.fsr); } return; }
    #[cfg(CONFIG_SMP)] unsafe { clear_tsk_thread_flag(fpt, TIF_USEDFPU); }
    unsafe { if psr & PSR_PS != 0 { printk(b"WARNING: FPU exception from kernel mode. at pc=%08lx\n\0".as_ptr(), (*regs).pc); (*regs).pc = (*regs).npc; (*regs).npc += 4; calls += 1; if calls > 2 { die_if_kernel(b"Too many Penguin-FPU traps from kernel mode\0".as_ptr() as *mut _, regs); } return; }
        fsr = (*fpt).thread.fsr; code = FPE_FLTUNK; if fsr & 0x1c000 == 1 << 14 { code = if fsr & 0x10 != 0 { FPE_FLTINV } else if fsr & 0x08 != 0 { FPE_FLTOVF } else if fsr & 0x04 != 0 { FPE_FLTUND } else if fsr & 0x02 != 0 { FPE_FLTDIV } else if fsr & 0x01 != 0 { FPE_FLTRES } else { FPE_FLTUNK }; } send_sig_fault(SIGFPE, code, pc as *mut _, fpt); }
    #[cfg(not(CONFIG_SMP))] unsafe { last_task_used_math = core::ptr::null_mut(); }
    unsafe { (*regs).psr &= !PSR_EF; if calls > 0 { calls = 0; } }
}

pub unsafe fn handle_tag_overflow(regs: *mut pt_regs, pc: ::core::ffi::c_ulong, _npc: ::core::ffi::c_ulong, psr: ::core::ffi::c_ulong) { unsafe { if psr & PSR_PS != 0 { die_if_kernel(b"Penguin overflow trap from kernel mode\0".as_ptr() as *mut _, regs); } send_sig_fault(SIGEMT, EMT_TAGOVF, pc as *mut _, current); } }
pub unsafe fn handle_watchpoint(regs: *mut pt_regs, _pc: ::core::ffi::c_ulong, _npc: ::core::ffi::c_ulong, psr: ::core::ffi::c_ulong) { unsafe { if psr & PSR_PS != 0 { panic(b"Tell me what a watchpoint trap is, and I'll then deal with such a beast...\0".as_ptr()); } } }
pub unsafe fn handle_reg_access(_regs: *mut pt_regs, pc: ::core::ffi::c_ulong, _npc: ::core::ffi::c_ulong, _psr: ::core::ffi::c_ulong) { unsafe { force_sig_fault(SIGBUS, BUS_OBJERR, pc as *mut _); } }
pub unsafe fn handle_cp_disabled(_regs: *mut pt_regs, pc: ::core::ffi::c_ulong, _npc: ::core::ffi::c_ulong, _psr: ::core::ffi::c_ulong) { unsafe { send_sig_fault(SIGILL, ILL_COPROC, pc as *mut _, current); } }
pub unsafe fn handle_cp_exception(_regs: *mut pt_regs, pc: ::core::ffi::c_ulong, _npc: ::core::ffi::c_ulong, _psr: ::core::ffi::c_ulong) { unsafe { send_sig_fault(SIGILL, ILL_COPROC, pc as *mut _, current); } }
pub unsafe fn handle_hw_divzero(_regs: *mut pt_regs, pc: ::core::ffi::c_ulong, _npc: ::core::ffi::c_ulong, _psr: ::core::ffi::c_ulong) { unsafe { send_sig_fault(SIGFPE, FPE_INTDIV, pc as *mut _, current); } }

#[cfg(CONFIG_DEBUG_BUGVERBOSE)] pub unsafe fn do_BUG(file: *const ::core::ffi::c_char, line: i32) { unsafe { printk(b"kernel BUG at %s:%d!\n\0".as_ptr(), file, line); } }

pub unsafe fn trap_init() {
    unsafe { extern "C" { fn thread_info_offsets_are_bolixed_pete(); }
        if TI_UWINMASK != core::mem::offset_of!(thread_info, uwinmask) || TI_TASK != core::mem::offset_of!(thread_info, task) || TI_FLAGS != core::mem::offset_of!(thread_info, flags) || TI_CPU != core::mem::offset_of!(thread_info, cpu) || TI_PREEMPT != core::mem::offset_of!(thread_info, preempt_count) || TI_SOFTIRQ != core::mem::offset_of!(thread_info, softirq_count) || TI_HARDIRQ != core::mem::offset_of!(thread_info, hardirq_count) || TI_KSP != core::mem::offset_of!(thread_info, ksp) || TI_KPC != core::mem::offset_of!(thread_info, kpc) || TI_KPSR != core::mem::offset_of!(thread_info, kpsr) || TI_KWIM != core::mem::offset_of!(thread_info, kwim) || TI_REG_WINDOW != core::mem::offset_of!(thread_info, reg_window) || TI_RWIN_SPTRS != core::mem::offset_of!(thread_info, rwbuf_stkptrs) || TI_W_SAVED != core::mem::offset_of!(thread_info, w_saved) { thread_info_offsets_are_bolixed_pete(); }
        mmgrab(&mut init_mm); (*current).active_mm = &mut init_mm;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
