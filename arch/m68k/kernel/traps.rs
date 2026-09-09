/* Rust translation of linux/arch/m68k/kernel/traps.c. */

/* Symbols and types below are supplied by the surrounding kernel translation. */

static VEC_NAMES: [&str; 64] = [
    "RESET SP", "RESET PC", "BUS ERROR", "ADDRESS ERROR", "ILLEGAL INSTRUCTION",
    "ZERO DIVIDE", "CHK", "TRAPcc", "PRIVILEGE VIOLATION", "TRACE", "LINE 1010",
    "LINE 1111", "UNASSIGNED RESERVED 12", "COPROCESSOR PROTOCOL VIOLATION",
    "FORMAT ERROR", "UNINITIALIZED INTERRUPT", "UNASSIGNED RESERVED 16",
    "UNASSIGNED RESERVED 17", "UNASSIGNED RESERVED 18", "UNASSIGNED RESERVED 19",
    "UNASSIGNED RESERVED 20", "UNASSIGNED RESERVED 21", "UNASSIGNED RESERVED 22",
    "UNASSIGNED RESERVED 23", "SPURIOUS INTERRUPT", "LEVEL 1 INT", "LEVEL 2 INT",
    "LEVEL 3 INT", "LEVEL 4 INT", "LEVEL 5 INT", "LEVEL 6 INT", "LEVEL 7 INT",
    "SYSCALL", "TRAP #1", "TRAP #2", "TRAP #3", "TRAP #4", "TRAP #5", "TRAP #6",
    "TRAP #7", "TRAP #8", "TRAP #9", "TRAP #10", "TRAP #11", "TRAP #12", "TRAP #13",
    "TRAP #14", "TRAP #15", "FPCP BSUN", "FPCP INEXACT", "FPCP DIV BY 0",
    "FPCP UNDERFLOW", "FPCP OPERAND ERROR", "FPCP OVERFLOW", "FPCP SNAN",
    "FPCP UNSUPPORTED OPERATION", "MMU CONFIGURATION ERROR", "MMU ILLEGAL OPERATION ERROR",
    "MMU ACCESS LEVEL VIOLATION ERROR", "UNASSIGNED RESERVED 59", "UNASSIGNED RESERVED 60",
    "UNASSIGNED RESERVED 61", "UNASSIGNED RESERVED 62", "UNASSIGNED RESERVED 63",
];

static SPACE_NAMES: [&str; 8] = ["Space 0", "User Data", "User Program", "Space 3", "Space 4", "Super Data", "Super Program", "CPU"];

extern "C" { fn die_if_kernel(s: *mut i8, fp: *mut pt_regs, nr: i32); }

#[cfg(feature = "CONFIG_M68060")]
unsafe fn access_error060(fp: *mut frame) {
    let fslw = (*fp).un_.fmt4.pc;
    pr_debug!("fslw={:#x}, fa={:#x}\n", fslw, (*fp).un_.fmt4.effaddr);
    if fslw & MMU060_BPE != 0 {
        // movec cacr,d0; orl #0x00400000,d0; movec d0,cacr
        if fslw & MMU060_ERR_BITS == 0 && fslw & MMU060_SEE == 0 { return; }
    }
    if fslw & (MMU060_DESC_ERR | MMU060_WP | MMU060_SP) != 0 {
        let mut addr = (*fp).un_.fmt4.effaddr;
        if fslw & MMU060_MA != 0 { addr = (addr + PAGE_SIZE - 1) & PAGE_MASK; }
        let mut errorcode = 1;
        if fslw & MMU060_DESC_ERR != 0 { __flush_tlb040_one(addr); errorcode = 0; }
        if fslw & MMU060_W != 0 { errorcode |= 2; }
        pr_debug!("errorcode = {}\n", errorcode);
        do_page_fault(&mut (*fp).ptregs, addr, errorcode);
    } else if fslw & MMU060_SEE != 0 { send_fault_sig(&mut (*fp).ptregs); }
    else if fslw & (MMU060_RE | MMU060_WE) == 0 || send_fault_sig(&mut (*fp).ptregs) > 0 {
        pr_err!("pc={:#x}, fa={:#x}\n", (*fp).ptregs.pc, (*fp).un_.fmt4.effaddr);
        pr_err!("68060 access error, fslw={:x}\n", fslw); trap_c(fp);
    }
}

#[cfg(feature = "CONFIG_M68040")]
unsafe fn do_040writeback1(wbs: u16, wba: usize, wbd: usize) -> i32 {
    set_fc(wbs);
    let res = match wbs & WBSIZ_040 {
        BA_SIZE_BYTE => put_user((wbd & 0xff) as u8, wba as *mut u8),
        BA_SIZE_WORD => put_user((wbd & 0xffff) as u16, wba as *mut u16),
        BA_SIZE_LONG => put_user(wbd, wba as *mut usize),
        _ => 0,
    };
    set_fc(USER_DATA); pr_debug!("do_040writeback1, res={}\n", res); res
}

#[cfg(feature = "CONFIG_M68040")]
unsafe fn fix_xframe040(fp: *mut frame, wba: usize, wbs: u16) {
    (*fp).un_.fmt7.faddr = wba; (*fp).un_.fmt7.ssw = wbs & 0xff;
    if wba != current().thread.faddr { (*fp).un_.fmt7.ssw |= MA_040; }
}

#[cfg(feature = "CONFIG_M68040")]
unsafe fn do_040writebacks(fp: *mut frame) {
    let mut res = 0;
    if (*fp).un_.fmt7.wb2s & WBV_040 != 0 && (*fp).un_.fmt7.wb2s & WBTT_040 == 0 {
        res = do_040writeback1((*fp).un_.fmt7.wb2s, (*fp).un_.fmt7.wb2a, (*fp).un_.fmt7.wb2d);
        if res != 0 { fix_xframe040(fp, (*fp).un_.fmt7.wb2a, (*fp).un_.fmt7.wb2s); } else { (*fp).un_.fmt7.wb2s = 0; }
    }
    if (*fp).un_.fmt7.wb3s & WBV_040 != 0 && (res == 0 || (*fp).un_.fmt7.wb3s & 4 != 0) {
        res = do_040writeback1((*fp).un_.fmt7.wb3s, (*fp).un_.fmt7.wb3a, (*fp).un_.fmt7.wb3d);
        if res != 0 { fix_xframe040(fp, (*fp).un_.fmt7.wb3a, (*fp).un_.fmt7.wb3s); (*fp).un_.fmt7.wb2s = (*fp).un_.fmt7.wb3s; (*fp).un_.fmt7.wb3s &= !WBV_040; (*fp).un_.fmt7.wb2a = (*fp).un_.fmt7.wb3a; (*fp).un_.fmt7.wb2d = (*fp).un_.fmt7.wb3d; } else { (*fp).un_.fmt7.wb3s = 0; }
    }
    if res != 0 { send_fault_sig(&mut (*fp).ptregs); }
}

#[cfg(feature = "CONFIG_M68040")]
pub unsafe extern "C" fn berr_040cleanup(fp: *mut frame) { (*fp).un_.fmt7.wb2s &= !4; (*fp).un_.fmt7.wb3s &= !4; do_040writebacks(fp); }

/* The remaining handlers retain the original kernel control flow; architecture-specific
 * frame fields, constants, logging macros, and external routines are provided by headers. */

pub unsafe extern "C" fn buserr_c(fp: *mut frame) {
    if user_mode(&mut (*fp).ptregs) { current_mut().thread.esp0 = fp as usize; }
    pr_debug!("*** Bus Error *** Format is {:x}\n", (*fp).ptregs.format);
    match (*fp).ptregs.format {
        #[cfg(feature = "CONFIG_M68060")] 4 => access_error060(fp),
        #[cfg(feature = "CONFIG_M68040")] 7 => access_error040(fp),
        #[cfg(any(feature = "CPU_M68020_OR_M68030"))] 10 | 11 => bus_error030(fp),
        _ => { die_if_kernel(b"bad frame format\0".as_ptr() as *mut i8, &mut (*fp).ptregs, 0); force_sig(SIGSEGV); }
    }
}

pub unsafe extern "C" fn trap_c(fp: *mut frame) {
    let vector = ((*fp).ptregs.vector >> 2) & 0xff;
    if (*fp).ptregs.sr & PS_S != 0 {
        if vector == VEC_TRACE { return; }
        #[cfg(feature = "CONFIG_MMU")] if fixup_exception(&mut (*fp).ptregs) { return; }
        bad_super_trap(fp); return;
    }
    let (sig, code) = match vector {
        VEC_ADDRERR => (SIGBUS, BUS_ADRALN),
        VEC_ILLEGAL | VEC_LINE10 | VEC_LINE11 => (SIGILL, ILL_ILLOPC),
        VEC_PRIV => (SIGILL, ILL_PRVOPC), VEC_COPROC => (SIGILL, ILL_COPROC),
        VEC_TRAP1..=VEC_TRAP14 => (SIGILL, ILL_ILLTRP),
        VEC_FPBRUC | VEC_FPOE | VEC_FPNAN => (SIGFPE, FPE_FLTINV),
        VEC_FPIR => (SIGFPE, FPE_FLTRES), VEC_FPDIVZ => (SIGFPE, FPE_FLTDIV),
        VEC_FPUNDER => (SIGFPE, FPE_FLTUND), VEC_FPOVER => (SIGFPE, FPE_FLTOVF),
        VEC_ZERODIV => (SIGFPE, FPE_INTDIV), VEC_CHK | VEC_TRAP => (SIGFPE, FPE_INTOVF),
        VEC_TRACE => (SIGTRAP, TRAP_TRACE), VEC_TRAP15 => (SIGTRAP, TRAP_BRKPT),
        _ => (SIGILL, ILL_ILLOPC),
    };
    let addr = match (*fp).ptregs.format {
        2 => (*fp).un_.fmt2.iaddr, 7 => (*fp).un_.fmt7.effaddr,
        9 => (*fp).un_.fmt9.iaddr, 10 => (*fp).un_.fmta.daddr,
        11 => (*fp).un_.fmtb.daddr, _ => (*fp).ptregs.pc,
    } as *mut core::ffi::c_void;
    force_sig_fault(sig, code, addr);
}

unsafe fn bad_super_trap(fp: *mut frame) {
    let vector = ((*fp).ptregs.vector >> 2) & 0xff;
    console_verbose();
    if vector < VEC_NAMES.len() { pr_err!("*** {} *** FORMAT={:X}\n", VEC_NAMES[vector], (*fp).ptregs.format); }
    else { pr_err!("*** Exception {} *** FORMAT={:X}\n", vector, (*fp).ptregs.format); }
    pr_err!("Current process id is {}\n", task_pid_nr(current()));
    die_if_kernel(b"BAD KERNEL TRAP\0".as_ptr() as *mut i8, &mut (*fp).ptregs, 0);
}

static mut KSTACK_DEPTH_TO_PRINT: i32 = 48;

unsafe fn show_trace(mut stack: *mut usize, loglvl: *const i8) {
    printk(loglvl, b"Call Trace:\0".as_ptr());
    let endstack = (((stack as usize) + THREAD_SIZE - 1) & !((THREAD_SIZE) - 1)) as *mut usize;
    let mut i = 0;
    while stack.add(1) <= endstack {
        let addr = *stack; stack = stack.add(1);
        if __kernel_text_address(addr) { pr_cont!(" [<{:08x}>] %pS\n", addr, addr as *mut core::ffi::c_void); i += 1; }
    }
    pr_cont!("\n");
}

pub unsafe fn show_stack(task: *mut task_struct, stack: *mut usize, loglvl: *const i8) {
    let mut p = if stack.is_null() { if !task.is_null() && task != current() { (*task).thread.ksp as *mut usize } else { &stack as *const _ as *mut usize } } else { stack };
    let endstack = (((p as usize) + THREAD_SIZE - 1) & !(THREAD_SIZE - 1)) as *mut usize;
    printk(loglvl, b"Stack from %08lx:\0".as_ptr(), p as usize);
    for i in 0..KSTACK_DEPTH_TO_PRINT { if p.add(1) > endstack { break; } if i % 8 == 0 { pr_cont!("\n       "); } pr_cont!(" {:08x}", *p); p = p.add(1); }
    pr_cont!("\n"); show_trace(if stack.is_null() { p } else { stack }, loglvl);
}

pub unsafe fn show_registers(regs: *mut pt_regs) {
    let fp = regs as *mut frame;
    print_modules();
    pr_info!("PC: [<{:08x}>] %pS\n", (*regs).pc, (*regs).pc as *mut core::ffi::c_void);
    pr_info!("SR: {:04x}  SP: %p  a2: {:08x}\n", (*regs).sr, regs, (*regs).a2);
    pr_info!("d0: {:08x} d1: {:08x} d2: {:08x} d3: {:08x}\n", (*regs).d0, (*regs).d1, (*regs).d2, (*regs).d3);
    pr_info!("d4: {:08x} d5: {:08x} a0: {:08x} a1: {:08x}\n", (*regs).d4, (*regs).d5, (*regs).a0, (*regs).a1);
    pr_info!("Process {} (pid: {}, task=%p)\n", current().comm, task_pid_nr(current()), current());
    show_stack(core::ptr::null_mut(), (&mut (*fp).un_ as *mut _ as *mut usize), KERN_INFO);
}

pub unsafe fn die_if_kernel_rs(s: *mut i8, fp: *mut pt_regs, nr: i32) {
    if (*fp).sr & PS_S == 0 { return; }
    console_verbose(); pr_crit!("{}: {:08x}\n", cstr(s), nr); show_registers(fp);
    add_taint(TAINT_DIE, LOCKDEP_NOW_UNRELIABLE); make_task_dead(SIGSEGV);
}

pub unsafe extern "C" fn set_esp0(ssp: usize) { current_mut().thread.esp0 = ssp; }
pub unsafe extern "C" fn fpsp040_die() { force_exit_sig(SIGSEGV); }

#[cfg(feature = "CONFIG_M68KFPU_EMU")]
pub unsafe extern "C" fn fpemu_signal(signal: i32, code: i32, addr: *mut core::ffi::c_void) { force_sig_fault(signal, code, addr); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
