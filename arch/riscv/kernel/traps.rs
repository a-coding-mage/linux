// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 Regents of the University of California
 */

// Linux and architecture headers from the original translation unit provide
// the external types, constants, functions, and macros referenced below.

pub static mut show_unhandled_signals: ::core::ffi::c_int = 1;

unsafe fn copy_code(regs: *mut pt_regs, val: *mut u16, insns: *const u16) -> c_long {
    let uaddr = insns as *const c_void;
    if !user_mode(regs) { return get_kernel_nofault(val.read(), insns); }
    if regs != task_pt_regs(current) { return -EPERM; }
    copy_from_user_nofault(val, uaddr, core::mem::size_of::<u16>())
}

unsafe fn dump_instr(loglvl: *const c_char, regs: *mut pt_regs) {
    let mut strbuf = [0u8; core::mem::size_of::<[u8; 5]>() * 12 + 3];
    let mut p = 0usize;
    let insns = instruction_pointer(regs) as *const u16;
    for i in -10..2 {
        let mut val = 0u16;
        let bad = copy_code(regs, &mut val, insns.offset(i));
        if bad == 0 {
            p += sprintf(strbuf.as_mut_ptr().add(p), if i == 0 { b"(%04hx) \\0".as_ptr() } else { b"%04hx \\0".as_ptr() }, val);
        } else {
            printk(b"%sCode: Unable to access instruction at 0x%px.\n\0".as_ptr(), loglvl, insns.offset(i));
            return;
        }
    }
    strbuf[p] = 0;
    printk(b"%sCode: %s\n\0".as_ptr(), loglvl, strbuf.as_ptr());
}

pub unsafe fn die(regs: *mut pt_regs, str: *const c_char) {
    static mut die_counter: c_int = 0;
    let mut flags = 0ul;
    oops_enter(); raw_spin_lock_irqsave(&die_lock, &mut flags); console_verbose(); bust_spinlocks(1);
    die_counter += 1; pr_emerg(b"%s [#%d]\n\0".as_ptr(), str, die_counter); print_modules();
    if !regs.is_null() { show_regs(regs); dump_instr(KERN_EMERG, regs); }
    let cause = if !regs.is_null() { (*regs).cause } else { -1 };
    let ret = notify_die(DIE_OOPS, str, regs, 0, cause, SIGSEGV);
    if kexec_should_crash(current) { crash_kexec(regs); }
    bust_spinlocks(0); add_taint(TAINT_DIE, LOCKDEP_NOW_UNRELIABLE); raw_spin_unlock_irqrestore(&die_lock, flags); oops_exit();
    if in_interrupt() { panic(b"Fatal exception in interrupt\0".as_ptr()); }
    if panic_on_oops != 0 { panic(b"Fatal exception\0".as_ptr()); }
    if ret != NOTIFY_STOP { make_task_dead(SIGSEGV); }
}

pub unsafe fn do_trap(regs: *mut pt_regs, signo: c_int, code: c_int, addr: c_ulong) {
    let tsk = current;
    if show_unhandled_signals != 0 && unhandled_signal(tsk, signo) && printk_ratelimit() {
        pr_info(b"%s[%d]: unhandled signal %d code 0x%x at 0x\0".as_ptr(), (*tsk).comm.as_ptr(), task_pid_nr(tsk), signo, code, addr);
        print_vma_addr(KERN_CONT, instruction_pointer(regs)); pr_cont(b"\n\0".as_ptr()); __show_regs(regs); dump_instr(KERN_INFO, regs);
    }
    force_sig_fault(signo, code, addr as *mut c_void);
}

unsafe fn do_trap_error(regs: *mut pt_regs, signo: c_int, code: c_int, addr: c_ulong, str: *const c_char) {
    (*current).thread.bad_cause = (*regs).cause;
    if user_mode(regs) { do_trap(regs, signo, code, addr); } else if !fixup_exception(regs) { die(regs, str); }
}

macro_rules! do_error_info { ($name:ident, $signo:expr, $code:expr, $msg:expr) => {
    pub unsafe extern "C" fn $name(regs: *mut pt_regs) {
        if user_mode(regs) { irqentry_enter_from_user_mode(regs); local_irq_enable(); do_trap_error(regs, $signo, $code, (*regs).epc, concat!("Oops - ", $msg, "\0").as_ptr() as *const c_char); local_irq_disable(); irqentry_exit_to_user_mode(regs); }
        else { let state = irqentry_nmi_enter(regs); do_trap_error(regs, $signo, $code, (*regs).epc, concat!("Oops - ", $msg, "\0").as_ptr() as *const c_char); irqentry_nmi_exit(regs, state); }
    }
}; }

do_error_info!(do_trap_unknown, SIGILL, ILL_ILLTRP, "unknown exception");
do_error_info!(do_trap_hardware_error, SIGBUS, BUS_MCEERR_AR, "hardware error");
do_error_info!(do_trap_insn_misaligned, SIGBUS, BUS_ADRALN, "instruction address misaligned");
do_error_info!(do_trap_insn_fault, SIGSEGV, SEGV_ACCERR, "instruction access fault");
do_error_info!(do_trap_load_fault, SIGSEGV, SEGV_ACCERR, "load access fault");
do_error_info!(do_trap_store_fault, SIGSEGV, SEGV_ACCERR, "store (or AMO) access fault");
do_error_info!(do_trap_ecall_s, SIGILL, ILL_ILLTRP, "environment call from S-mode");
do_error_info!(do_trap_ecall_m, SIGILL, ILL_ILLTRP, "environment call from M-mode");

pub unsafe extern "C" fn do_trap_insn_illegal(regs: *mut pt_regs) {
    if user_mode(regs) { irqentry_enter_from_user_mode(regs); local_irq_enable(); if !riscv_v_first_use_handler(regs) { do_trap_error(regs, SIGILL, ILL_ILLOPC, (*regs).epc, b"Oops - illegal instruction\0".as_ptr() as _); } local_irq_disable(); irqentry_exit_to_user_mode(regs); }
    else { let state = irqentry_nmi_enter(regs); do_trap_error(regs, SIGILL, ILL_ILLOPC, (*regs).epc, b"Oops - illegal instruction\0".as_ptr() as _); irqentry_nmi_exit(regs, state); }
}

#[repr(C)] pub enum misaligned_access_type { MISALIGNED_STORE, MISALIGNED_LOAD }
static misaligned_handler: [MisalignedHandler; 2] = [
    MisalignedHandler { type_str: b"Oops - store (or AMO) address misaligned\0", handler: handle_misaligned_store },
    MisalignedHandler { type_str: b"Oops - load address misaligned\0", handler: handle_misaligned_load },
];
#[repr(C)] struct MisalignedHandler { type_str: &'static [u8], handler: unsafe extern "C" fn(*mut pt_regs) -> c_int }

unsafe fn do_trap_misaligned(regs: *mut pt_regs, ty: misaligned_access_type) {
    let state; if user_mode(regs) { irqentry_enter_from_user_mode(regs); local_irq_enable(); state = core::mem::zeroed(); } else { state = irqentry_nmi_enter(regs); }
    if (misaligned_handler[ty as usize].handler)(regs) != 0 { do_trap_error(regs, SIGBUS, BUS_ADRALN, (*regs).epc, misaligned_handler[ty as usize].type_str.as_ptr() as _); }
    if user_mode(regs) { local_irq_disable(); irqentry_exit_to_user_mode(regs); } else { irqentry_nmi_exit(regs, state); }
}
pub unsafe extern "C" fn do_trap_load_misaligned(r: *mut pt_regs) { do_trap_misaligned(r, misaligned_access_type::MISALIGNED_LOAD); }
pub unsafe extern "C" fn do_trap_store_misaligned(r: *mut pt_regs) { do_trap_misaligned(r, misaligned_access_type::MISALIGNED_STORE); }

unsafe fn get_break_insn_length(pc: c_ulong) -> c_ulong { let mut insn = core::mem::zeroed(); if get_kernel_nofault(insn, pc as *const bug_insn_t) != 0 { 0 } else { GET_INSN_LENGTH(insn) } }
unsafe fn probe_single_step_handler(r: *mut pt_regs) -> bool { if user_mode(r) { uprobe_single_step_handler(r) } else { kprobe_single_step_handler(r) } }
unsafe fn probe_breakpoint_handler(r: *mut pt_regs) -> bool { if user_mode(r) { uprobe_breakpoint_handler(r) } else { kprobe_breakpoint_handler(r) } }
pub unsafe fn handle_break(r: *mut pt_regs) { if probe_single_step_handler(r) || probe_breakpoint_handler(r) { return; } (*current).thread.bad_cause=(*r).cause; if user_mode(r) { force_sig_fault(SIGTRAP, TRAP_BRKPT, (*r).epc as *mut c_void); } else if report_bug((*r).epc,r)==BUG_TRAP_TYPE_WARN || handle_cfi_failure(r)==BUG_TRAP_TYPE_WARN { (*r).epc += get_break_insn_length((*r).epc); } else { die(r,b"Kernel BUG\0".as_ptr() as _); } }

// Remaining entry points retain the original control-flow and external kernel APIs.
pub unsafe extern "C" fn do_trap_break(r:*mut pt_regs){if user_mode(r){irqentry_enter_from_user_mode(r);local_irq_enable();handle_break(r);local_irq_disable();irqentry_exit_to_user_mode(r)}else{let s=irqentry_nmi_enter(r);handle_break(r);irqentry_nmi_exit(r,s)}}

pub unsafe extern "C" fn do_trap_ecall_u(r: *mut pt_regs) {
    if user_mode(r) { let mut syscall=(*r).a7; (*r).epc+=4; (*r).orig_a0=(*r).a0; (*r).a0=(-ENOSYS) as _; riscv_v_vstate_discard(r); if syscall_enter_from_user_mode_randomize_stack(r,&mut syscall) && syscall>=0 && syscall<NR_syscalls { syscall_handler(r,array_index_nospec(syscall,NR_syscalls)); } syscall_exit_to_user_mode(r); }
    else { let s=irqentry_nmi_enter(r); do_trap_error(r,SIGILL,ILL_ILLTRP,(*r).epc,b"Oops - environment call from U-mode\0".as_ptr() as _); irqentry_nmi_exit(r,s); }
}

const CFI_TVAL_FCFI_CODE: c_ulong=2; const CFI_TVAL_BCFI_CODE:c_ulong=3;
pub unsafe fn handle_user_cfi_violation(r:*mut pt_regs)->bool { let tval=csr_read(CSR_TVAL); let is_fcfi=tval==CFI_TVAL_FCFI_CODE&&cpu_supports_indirect_br_lp_instr(); let is_bcfi=tval==CFI_TVAL_BCFI_CODE&&cpu_supports_shadow_stack(); if is_fcfi&&probe_breakpoint_handler(r){(*r).status&=!SR_ELP;return true} if is_fcfi||is_bcfi{do_trap_error(r,SIGSEGV,SEGV_CPERR,(*r).epc,b"Oops - control flow violation\0".as_ptr() as _);return true} false }

pub unsafe extern "C" fn do_trap_software_check(r:*mut pt_regs){if user_mode(r){irqentry_enter_from_user_mode(r);if !handle_user_cfi_violation(r){do_trap_unknown(r)}irqentry_exit_to_user_mode(r)}else{die(r,b"Kernel BUG\0".as_ptr() as _)}}

#[cfg(CONFIG_MMU)] pub unsafe extern "C" fn do_page_fault(r:*mut pt_regs){let s=irqentry_enter(r);handle_page_fault(r);local_irq_disable();irqentry_exit(r,s)}
unsafe fn handle_riscv_irq(r:*mut pt_regs){irq_enter_rcu();let old=set_irq_regs(r);handle_arch_irq(r);set_irq_regs(old);irq_exit_rcu()}
pub unsafe extern "C" fn do_irq(r:*mut pt_regs){let s=irqentry_enter(r);if IS_ENABLED(CONFIG_IRQ_STACKS)&&on_thread_stack(){call_on_irq_stack(r,handle_riscv_irq)}else{handle_riscv_irq(r)}irqentry_exit(r,s)}

#[cfg(CONFIG_GENERIC_BUG)] pub unsafe fn is_valid_bugaddr(pc:c_ulong)->c_int{let mut insn=core::mem::zeroed();if pc<VMALLOC_START||get_kernel_nofault(insn,pc as *const bug_insn_t)!=0{return 0}if (insn&__INSN_LENGTH_MASK)==__INSN_LENGTH_32{(insn==__BUG_INSN_32) as c_int}else{((insn&__COMPRESSED_INSN_MASK)==__BUG_INSN_16) as c_int}}

#[cfg(CONFIG_VMAP_STACK)] pub static mut overflow_stack:[[c_ulong;OVERFLOW_STACK_SIZE/core::mem::size_of::<c_ulong>()];1]=[[0;OVERFLOW_STACK_SIZE/core::mem::size_of::<c_ulong>()];1];
#[cfg(CONFIG_VMAP_STACK)] pub unsafe extern "C" fn handle_bad_stack(r:*mut pt_regs){let tsk_stk=(*current).stack as c_ulong;let ovf_stk=this_cpu_ptr(overflow_stack.as_mut_ptr()) as c_ulong;console_verbose();pr_emerg(b"Insufficient stack space to handle exception!\n\0".as_ptr());pr_emerg(b"Task stack:     [0x%016lx..0x%016lx]\n\0".as_ptr(),tsk_stk,tsk_stk+THREAD_SIZE);pr_emerg(b"Overflow stack: [0x%016lx..0x%016lx]\n\0".as_ptr(),ovf_stk,ovf_stk+OVERFLOW_STACK_SIZE);__show_regs(r);panic(b"Kernel stack overflow\0".as_ptr());loop{wait_for_interrupt()}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
