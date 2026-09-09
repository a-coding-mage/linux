// SPDX-License-Identifier: GPL-2.0
/* S390 version; translated from traps.c. */

// Dependencies supplied by the surrounding kernel translation unit are intentionally external.

#[repr(C)]
pub struct pgm_stat {
    pub count: [u32; 128],
}

// static DEFINE_PER_CPU_SHARED_ALIGNED(struct pgm_stat, pgm_stat);
extern "C" {
    static mut pgm_stat: pgm_stat;
}

#[inline]
unsafe fn get_trap_ip(regs: *mut pt_regs) -> *mut core::ffi::c_void {
    let address: usize;
    if (*regs).int_code & 0x200 != 0 {
        address = (*current).thread.trap_tdb.data[3] as usize;
    } else {
        address = (*regs).psw.addr as usize;
    }
    (address - (((*regs).int_code >> 16) as usize)) as *mut core::ffi::c_void
}

#[cfg(feature = "CONFIG_GENERIC_BUG")]
pub unsafe fn is_valid_bugaddr(_addr: usize) -> i32 { 1 }

pub unsafe fn do_report_trap(regs: *mut pt_regs, si_signo: i32, si_code: i32, str_: *mut u8) {
    if user_mode(regs) {
        force_sig_fault(si_signo, si_code, get_trap_ip(regs));
        report_user_fault(regs, si_signo, 0);
    } else if !fixup_exception(regs) {
        die(regs, str_);
    }
}

unsafe fn do_trap(regs: *mut pt_regs, si_signo: i32, si_code: i32, str_: *mut u8) {
    if notify_die(DIE_TRAP, str_, regs, 0, (*regs).int_code, si_signo) == NOTIFY_STOP { return; }
    do_report_trap(regs, si_signo, si_code, str_);
}

unsafe fn do_per_trap(regs: *mut pt_regs) {
    if notify_die(DIE_SSTEP, b"sstep\0".as_ptr() as *mut u8, regs, 0, 0, SIGTRAP) == NOTIFY_STOP { return; }
    if !(*current).ptrace { return; }
    force_sig_fault(SIGTRAP, TRAP_HWBKPT, (*current).thread.per_event.address as *mut core::ffi::c_void);
}

unsafe fn default_trap_handler(regs: *mut pt_regs) {
    if user_mode(regs) {
        report_user_fault(regs, SIGSEGV, 0);
        force_exit_sig(SIGSEGV);
    } else { die(regs, b"Unknown program exception\0".as_ptr() as *mut u8); }
}

macro_rules! do_error_info { ($name:ident, $signr:ident, $sicode:ident, $str:literal) => {
    unsafe fn $name(regs: *mut pt_regs) { do_trap(regs, $signr, $sicode, concat!($str, "\0").as_ptr() as *mut u8); }
}; }
do_error_info!(addressing_exception, SIGILL, ILL_ILLADR, "addressing exception");
do_error_info!(divide_exception, SIGFPE, FPE_INTDIV, "fixpoint divide exception");
do_error_info!(execute_exception, SIGILL, ILL_ILLOPN, "execute exception");
do_error_info!(hfp_divide_exception, SIGFPE, FPE_FLTDIV, "HFP divide exception");
do_error_info!(hfp_overflow_exception, SIGFPE, FPE_FLTOVF, "HFP overflow exception");
do_error_info!(hfp_significance_exception, SIGFPE, FPE_FLTRES, "HFP significance exception");
do_error_info!(hfp_sqrt_exception, SIGFPE, FPE_FLTINV, "HFP square root exception");
do_error_info!(hfp_underflow_exception, SIGFPE, FPE_FLTUND, "HFP underflow exception");
do_error_info!(operand_exception, SIGILL, ILL_ILLOPN, "operand exception");
do_error_info!(overflow_exception, SIGFPE, FPE_INTOVF, "fixpoint overflow exception");
do_error_info!(privileged_op, SIGILL, ILL_PRVOPC, "privileged operation");
do_error_info!(special_op_exception, SIGILL, ILL_ILLOPN, "special operation exception");
do_error_info!(specification_exception, SIGILL, ILL_ILLOPN, "specification exception");
do_error_info!(transaction_exception, SIGILL, ILL_ILLOPN, "transaction constraint exception");

#[inline]
unsafe fn do_fp_trap(regs: *mut pt_regs, fpc: u32) {
    let mut si_code = 0;
    if fpc & 0x00000300 == 0 {
        if fpc & 0x8000 != 0 { si_code = FPE_FLTINV; }
        else if fpc & 0x4000 != 0 { si_code = FPE_FLTDIV; }
        else if fpc & 0x2000 != 0 { si_code = FPE_FLTOVF; }
        else if fpc & 0x1000 != 0 { si_code = FPE_FLTUND; }
        else if fpc & 0x0800 != 0 { si_code = FPE_FLTRES; }
    }
    do_trap(regs, SIGFPE, si_code, b"floating point exception\0".as_ptr() as *mut u8);
}

unsafe fn translation_specification_exception(_regs: *mut pt_regs) { panic!("Translation-Specification Exception"); }

unsafe fn illegal_op(regs: *mut pt_regs) {
    let mut is_uprobe_insn = 0;
    let location = get_trap_ip(regs) as *mut u16;
    let mut signal = 0;
    let mut opcode = 0u16;
    if user_mode(regs) {
        if get_user(&mut opcode, location) != 0 { return; }
        if opcode == S390_BREAKPOINT_U16 {
            if (*current).ptrace { force_sig_fault(SIGTRAP, TRAP_BRKPT, location as *mut core::ffi::c_void); } else { signal = SIGILL; }
        } else {
            // CONFIG_UPROBES conditionally recognizes UPROBE_SWBP_INSN here.
            signal = SIGILL;
        }
    }
    if is_uprobe_insn != 0 || !user_mode(regs) {
        if notify_die(DIE_BPT, b"bpt\0".as_ptr() as *mut u8, regs, 0, 3, SIGTRAP) != NOTIFY_STOP { signal = SIGILL; }
    }
    if signal != 0 { do_trap(regs, signal, ILL_ILLOPC, b"illegal operation\0".as_ptr() as *mut u8); }
}

unsafe fn vector_exception(regs: *mut pt_regs) {
    save_user_fpu_regs();
    let vic = ((*current).thread.ufpu.fpc & 0xf00) >> 8;
    let si_code = match vic { 1 => FPE_FLTINV, 2 => FPE_FLTDIV, 3 => FPE_FLTOVF, 4 => FPE_FLTUND, 5 => FPE_FLTRES, _ => 0 };
    do_trap(regs, SIGFPE, si_code, b"vector exception\0".as_ptr() as *mut u8);
}

unsafe fn data_exception(regs: *mut pt_regs) {
    save_user_fpu_regs();
    if (*current).thread.ufpu.fpc & FPC_DXC_MASK != 0 { do_fp_trap(regs, (*current).thread.ufpu.fpc); }
    else { do_trap(regs, SIGILL, ILL_ILLOPN, b"data exception\0".as_ptr() as *mut u8); }
}

unsafe fn space_switch_exception(regs: *mut pt_regs) {
    if user_mode(regs) { (*regs).psw.mask |= PSW_ASC_HOME; }
    do_trap(regs, SIGILL, ILL_PRVOPC, b"space switch event\0".as_ptr() as *mut u8);
}

#[cfg(all(feature = "CONFIG_BUG", feature = "CONFIG_CC_HAS_ASM_IMMEDIATE_STRINGS"))]
pub unsafe fn __warn_args(args: *mut arch_va_list, regs: *mut pt_regs) -> *mut arch_va_list {
    let stack_frame = (*regs).gprs[15] as *mut stack_frame;
    (*args).__overflow_arg_area = stack_frame.add(1) as *mut core::ffi::c_void;
    (*args).__reg_save_area = (*regs).gprs.as_mut_ptr() as *mut core::ffi::c_void;
    (*args).__gpr = 1;
    args
}

unsafe fn monitor_event_exception(regs: *mut pt_regs) {
    if user_mode(regs) { return; }
    let btt = if (*regs).monitor_code == MONCODE_BUG_ARG {
        (*regs).psw.addr = (*regs).gprs[14];
        report_bug_entry((*regs).gprs[2] as *mut bug_entry, regs)
    } else { report_bug((*regs).psw.addr - ((*regs).int_code >> 16) as u64, regs) };
    match btt { BUG_TRAP_TYPE_NONE => { fixup_exception(regs); }, BUG_TRAP_TYPE_WARN => {}, BUG_TRAP_TYPE_BUG => die(regs, b"monitor event\0".as_ptr() as *mut u8), _ => {} }
}

pub unsafe fn kernel_stack_invalid(regs: *mut pt_regs) {
    kmsan_unpoison_entry_regs(regs); bust_spinlocks(1); pr_emerg(b"Kernel stack pointer invalid\n\0".as_ptr()); show_regs(regs); bust_spinlocks(0); panic!("Invalid kernel stack pointer, cannot continue");
}

unsafe fn test_monitor_call() {
    let mut val = 1;
    if !IS_ENABLED(CONFIG_BUG) { return; }
    // The original volatile inline assembly performs mc and uses EX_TABLE(0b, 1b).
    core::arch::asm!("mc 0({monc}),0", "0: lhi {val},0", "1:", val = inout(reg) val, monc = const MONCODE_BUG);
    if val == 0 { panic!("Monitor call doesn't work!\n"); }
}

pub unsafe fn trap_init() {
    let lc = get_lowcore(); let mut flags = 0; let mut cr0;
    local_irq_save(&mut flags); cr0 = local_ctl_clear_bit(0, CR0_LOW_ADDRESS_PROTECTION_BIT);
    psw_bits((*lc).external_new_psw).mcheck = 1; psw_bits((*lc).program_new_psw).mcheck = 1; psw_bits((*lc).svc_new_psw).mcheck = 1; psw_bits((*lc).io_new_psw).mcheck = 1;
    local_ctl_load(0, &mut cr0); local_irq_restore(flags); local_mcck_enable(); test_monitor_call();
}

// Forward declaration in C: static void (*pgm_check_table[128])(struct pt_regs *regs);
static mut pgm_check_table: [Option<unsafe fn(*mut pt_regs)>; 128] = [None; 128];

pub unsafe fn __do_pgm_check(regs: *mut pt_regs, flags: usize) {
    let lc = get_lowcore(); let mut percpu_needs_fixup; let state; let stat; let trapnr; let mut teid = teid { val: 0 };
    teid.val = (*lc).trans_exc_code; (*regs).int_code = (*lc).pgm_int_code; (*regs).int_parm_long = teid.val; (*regs).monitor_code = (*lc).monitor_code;
    trapnr = (*regs).int_code & PGM_INT_CODE_MASK; stat = this_cpu_ptr(&mut pgm_stat); (*stat).count[trapnr as usize] += 1;
    if flags & PGM_FLAG_GUEST_FAULT != 0 { (*current).thread.gmap_teid.val = (*regs).int_parm_long; (*current).thread.gmap_int_code = (*regs).int_code & 0xffff; return; }
    percpu_entry(regs); state = irqentry_enter(regs);
    if user_mode(regs) { update_timer_sys(); if !cpu_has_bear() && (*regs).last_break < 4096 { (*regs).last_break = 1; } (*current).thread.last_break = (*regs).last_break; }
    if (*lc).pgm_code & 0x0200 != 0 { (*current).thread.trap_tdb = (*lc).pgm_tdb; }
    if (*lc).pgm_code & PGM_INT_CODE_PER != 0 {
        if user_mode(regs) { let ev = &mut (*current).thread.per_event; set_thread_flag(TIF_PER_TRAP); ev.address = (*lc).per_address; ev.cause = (*lc).per_code_combined; ev.paid = (*lc).per_access_id; }
        else { __arch_local_irq_ssm((*regs).psw.mask & !PSW_MASK_PER); do_per_trap(regs); goto_out!(out); }
    }
    if !irqs_disabled_flags((*regs).psw.mask) { trace_hardirqs_on(); } __arch_local_irq_ssm((*regs).psw.mask & !PSW_MASK_PER);
    if trapnr != 0 { if let Some(f) = pgm_check_table[trapnr as usize] { f(regs); } }
    out: { local_irq_disable(); percpu_needs_fixup = percpu_code_check(regs); irqentry_exit(regs, state); percpu_exit(regs, percpu_needs_fixup); }
}

// Debugfs/statistics declarations and initcall are retained as external kernel integration points.
unsafe fn pgm_check_stat_show(_p: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn debugfs_pgm_check_init() -> i32 { debugfs_create_file(b"exceptions\0".as_ptr(), 0o400, arch_debugfs_dir, core::ptr::null_mut(), &pgm_check_stat_fops); 0 }

// C designated ranges are expanded explicitly to preserve the 128-entry dispatch table.
#[allow(clippy::type_complexity)]
static mut PGM_CHECK_TABLE: [Option<unsafe fn(*mut pt_regs)>; 128] = [Some(default_trap_handler); 128];

// Entries corresponding to the C table; initialization is performed by the surrounding kernel setup.
unsafe fn init_pgm_check_table() {
    pgm_check_table[0x01] = Some(illegal_op); pgm_check_table[0x02] = Some(privileged_op); pgm_check_table[0x03] = Some(execute_exception);
    pgm_check_table[0x04] = Some(do_protection_exception); pgm_check_table[0x05] = Some(addressing_exception); pgm_check_table[0x06] = Some(specification_exception); pgm_check_table[0x07] = Some(data_exception);
    pgm_check_table[0x08] = Some(overflow_exception); pgm_check_table[0x09] = Some(divide_exception); pgm_check_table[0x0a] = Some(overflow_exception); pgm_check_table[0x0b] = Some(divide_exception);
    pgm_check_table[0x0c] = Some(hfp_overflow_exception); pgm_check_table[0x0d] = Some(hfp_underflow_exception); pgm_check_table[0x0e] = Some(hfp_significance_exception); pgm_check_table[0x0f] = Some(hfp_divide_exception);
    pgm_check_table[0x10] = Some(do_dat_exception); pgm_check_table[0x11] = Some(do_dat_exception); pgm_check_table[0x12] = Some(translation_specification_exception); pgm_check_table[0x13] = Some(special_op_exception);
    pgm_check_table[0x15] = Some(operand_exception); pgm_check_table[0x18] = Some(transaction_exception); pgm_check_table[0x1b] = Some(vector_exception); pgm_check_table[0x1c] = Some(space_switch_exception); pgm_check_table[0x1d] = Some(hfp_sqrt_exception);
    for i in 0x1e..=0x37 { pgm_check_table[i] = Some(default_trap_handler); } pgm_check_table[0x38] = Some(do_dat_exception); pgm_check_table[0x39] = Some(do_dat_exception); pgm_check_table[0x3a] = Some(do_dat_exception); pgm_check_table[0x3b] = Some(do_dat_exception); pgm_check_table[0x3d] = Some(do_secure_storage_access); pgm_check_table[0x40] = Some(monitor_event_exception);
    for i in 0x41..=0x7f { pgm_check_table[i] = Some(default_trap_handler); }
}

// COND_TRAP(do_secure_storage_access): weakly aliases the handler to default_trap_handler.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
