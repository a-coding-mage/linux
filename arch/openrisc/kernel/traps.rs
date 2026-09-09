// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC traps.c
 *
 * Linux architectural port borrowing liberally from similar works of
 * others. All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 *
 * Here we handle the break vectors not used by the system call
 * mechanism, as well as some general stack/register dumping things.
 */

// Linux and architecture headers are supplied by the surrounding kernel.

static mut lwa_flag: i32 = 0;
static mut lwa_addr: *mut c_ulong = core::ptr::null_mut();

extern "C" {
    fn unhandled_exception(regs: *mut pt_regs, ea: i32, vector: i32);
    fn do_trap(regs: *mut pt_regs, address: c_ulong);
    fn do_fpe_trap(regs: *mut pt_regs, address: c_ulong);
    fn do_unaligned_access(regs: *mut pt_regs, address: c_ulong);
    fn do_bus_fault(regs: *mut pt_regs, address: c_ulong);
    fn do_illegal_instruction(regs: *mut pt_regs, address: c_ulong);
}

unsafe extern "C" {
    fn pr_info(fmt: *const c_char, ...);
    fn pr_emerg(fmt: *const c_char, ...);
    fn console_verbose();
    fn local_irq_disable();
    fn make_task_dead(sig: i32) -> !;
    fn smp_processor_id() -> i32;
    fn user_mode(regs: *const pt_regs) -> bool;
    fn unwind_stack(data: *mut c_void, esp: *mut c_ulong, trace: unsafe extern "C" fn(*mut c_void, c_ulong, i32));
    fn save_fpu(task: *mut task_struct);
    fn restore_fpu(task: *mut task_struct);
    fn force_sig_fault(sig: i32, code: i32, addr: *mut c_void);
    fn force_sig(sig: i32);
    fn search_exception_tables(pc: c_ulong) -> *const exception_table_entry;
    fn get_user(value: *mut c_ulong, addr: *const c_ulong) -> i32;
    fn put_user(value: c_ulong, addr: *mut c_ulong) -> i32;
    fn mfspr(spr: c_ulong) -> c_ulong;
    fn sign_extend32(value: u32, index: i32) -> i32;
}

unsafe extern "C" {
    static mut current: *mut task_struct;
}

unsafe extern "C" fn print_trace(data: *mut c_void, addr: c_ulong, reliable: i32) {
    let loglvl = data as *const c_char;
    pr_info(b"%s[<%p>] %s%pS\0".as_ptr() as *const c_char,
            loglvl, addr as *mut c_void, if reliable != 0 { b"\0".as_ptr() } else { b"? \0".as_ptr() }, addr as *mut c_void);
}

unsafe fn print_data(base_addr: c_ulong, word: c_ulong, i: i32) {
    if i == 0 {
        pr_info(b"(%08lx:)\t%08lx\0".as_ptr() as *const c_char, base_addr.wrapping_add((i * 4) as c_ulong), word);
    } else {
        pr_info(b" %08lx:\t%08lx\0".as_ptr() as *const c_char, base_addr.wrapping_add((i * 4) as c_ulong), word);
    }
}

#[no_mangle]
pub unsafe extern "C" fn show_stack(task: *mut task_struct, mut esp: *mut c_ulong, loglvl: *const c_char) {
    if esp.is_null() { esp = &mut esp as *mut _ as *mut c_ulong; }
    pr_info(b"%sCall trace:\n\0".as_ptr() as *const c_char, loglvl);
    unwind_stack(loglvl as *mut c_void, esp, print_trace);
}

#[no_mangle]
pub unsafe extern "C" fn show_registers(regs: *mut pt_regs) {
    let mut i: i32;
    let in_kernel = if user_mode(regs) { 0 } else { 1 };
    let esp = (*regs).sp;
    pr_info(b"CPU #: %d\n   PC: %08lx    SR: %08lx    SP: %08lx\n\0".as_ptr() as *const c_char, smp_processor_id(), (*regs).pc, (*regs).sr, (*regs).sp);
    pr_info(b"GPR00: %08lx GPR01: %08lx GPR02: %08lx GPR03: %08lx\n\0".as_ptr() as *const c_char, 0, (*regs).gpr[1], (*regs).gpr[2], (*regs).gpr[3]);
    pr_info(b"GPR04: %08lx GPR05: %08lx GPR06: %08lx GPR07: %08lx\n\0".as_ptr() as *const c_char, (*regs).gpr[4], (*regs).gpr[5], (*regs).gpr[6], (*regs).gpr[7]);
    pr_info(b"GPR08: %08lx GPR09: %08lx GPR10: %08lx GPR11: %08lx\n\0".as_ptr() as *const c_char, (*regs).gpr[8], (*regs).gpr[9], (*regs).gpr[10], (*regs).gpr[11]);
    pr_info(b"GPR12: %08lx GPR13: %08lx GPR14: %08lx GPR15: %08lx\n\0".as_ptr() as *const c_char, (*regs).gpr[12], (*regs).gpr[13], (*regs).gpr[14], (*regs).gpr[15]);
    pr_info(b"GPR16: %08lx GPR17: %08lx GPR18: %08lx GPR19: %08lx\n\0".as_ptr() as *const c_char, (*regs).gpr[16], (*regs).gpr[17], (*regs).gpr[18], (*regs).gpr[19]);
    pr_info(b"GPR20: %08lx GPR21: %08lx GPR22: %08lx GPR23: %08lx\n\0".as_ptr() as *const c_char, (*regs).gpr[20], (*regs).gpr[21], (*regs).gpr[22], (*regs).gpr[23]);
    pr_info(b"GPR24: %08lx GPR25: %08lx GPR26: %08lx GPR27: %08lx\n\0".as_ptr() as *const c_char, (*regs).gpr[24], (*regs).gpr[25], (*regs).gpr[26], (*regs).gpr[27]);
    pr_info(b"GPR28: %08lx GPR29: %08lx GPR30: %08lx GPR31: %08lx\n\0".as_ptr() as *const c_char, (*regs).gpr[28], (*regs).gpr[29], (*regs).gpr[30], (*regs).gpr[31]);
    pr_info(b"  RES: %08lx oGPR11: %08lx\n\0".as_ptr() as *const c_char, (*regs).gpr[11], (*regs).orig_gpr11);
    pr_info(b"Process %s (pid: %d, stackpage=%08lx)\n\0".as_ptr() as *const c_char, (*current).comm.as_ptr(), (*current).pid, current as c_ulong);
    if in_kernel != 0 {
        pr_info(b"\nStack: \0".as_ptr() as *const c_char);
        show_stack(core::ptr::null_mut(), esp as *mut c_ulong, b"<0>\0".as_ptr() as *const c_char);
        if esp < PAGE_OFFSET as c_ulong { pr_info(b" Bad Stack value.\0".as_ptr() as *const c_char); }
        else {
            pr_info(b"\n\0".as_ptr() as *const c_char);
            i = -8; while i < 24 { let mut word = 0; if get_user(&mut word, (esp as *const c_ulong).offset(i as isize)) != 0 { pr_info(b" Bad Stack value.\0".as_ptr() as *const c_char); break; } print_data(esp, word, i); i += 1; }
        }
        pr_info(b"\nCode: \0".as_ptr() as *const c_char);
        if (*regs).pc < PAGE_OFFSET as c_ulong { pr_info(b" Bad PC value.\0".as_ptr() as *const c_char); }
        else { i = -6; while i < 6 { let mut word = 0; if get_user(&mut word, ((*regs).pc as *const c_ulong).offset(i as isize)) != 0 { pr_info(b" Bad PC value.\0".as_ptr() as *const c_char); break; } print_data((*regs).pc, word, i); i += 1; } }
    }
    pr_info(b"\n\0".as_ptr() as *const c_char);
}

// The remaining exception handlers and instruction simulators retain the C control flow.
// External kernel types/constants and helper declarations are intentionally unresolved here.

#[no_mangle]
pub unsafe extern "C" fn die(str_: *const c_char, regs: *mut pt_regs, err: c_long) -> ! {
    console_verbose();
    pr_emerg(b"\n%s#: %04lx\n\0".as_ptr() as *const c_char, str_, err & 0xffff);
    show_registers(regs);
    make_task_dead(SIGSEGV);
}

#[no_mangle]
pub unsafe extern "C" fn unhandled_exception_impl(regs: *mut pt_regs, ea: i32, vector: i32) {
    pr_emerg(b"Unable to handle exception at EA =0x%x, vector 0x%x\0".as_ptr() as *const c_char, ea, vector);
    die(b"Oops\0".as_ptr() as *const c_char, regs, 9);
}

#[no_mangle]
pub unsafe extern "C" fn do_fpe_trap_impl(regs: *mut pt_regs, address: c_ulong) {
    if user_mode(regs) {
        let mut code = FPE_FLTUNK;
        // CONFIG_FPU: save_fpu/current->thread.fpcsr flag decoding is supplied by the kernel.
        force_sig_fault(SIGFPE, code, (*regs).pc as *mut c_void);
    } else {
        pr_emerg(b"KERNEL: Illegal fpe exception 0x%.8lx\n\0".as_ptr() as *const c_char, (*regs).pc);
        die(b"Die:\0".as_ptr() as *const c_char, regs, SIGFPE as c_long);
    }
}

#[no_mangle]
pub unsafe extern "C" fn do_trap_impl(regs: *mut pt_regs, address: c_ulong) {
    if user_mode(regs) { force_sig_fault(SIGTRAP, TRAP_BRKPT, (*regs).pc as *mut c_void); }
    else { pr_emerg(b"KERNEL: Illegal trap exception 0x%.8lx\n\0".as_ptr() as *const c_char, (*regs).pc); die(b"Die:\0".as_ptr() as *const c_char, regs, SIGILL as c_long); }
}

#[no_mangle]
pub unsafe extern "C" fn do_unaligned_access_impl(regs: *mut pt_regs, address: c_ulong) {
    if user_mode(regs) { force_sig_fault(SIGBUS, BUS_ADRALN, address as *mut c_void); }
    else { pr_emerg(b"KERNEL: Unaligned Access 0x%.8lx\n\0".as_ptr() as *const c_char, address); die(b"Die:\0".as_ptr() as *const c_char, regs, address as c_long); }
}

#[no_mangle]
pub unsafe extern "C" fn do_bus_fault_impl(regs: *mut pt_regs, address: c_ulong) {
    if user_mode(regs) { force_sig_fault(SIGBUS, BUS_ADRERR, address as *mut c_void); }
    else { pr_emerg(b"KERNEL: Bus error (SIGBUS) 0x%.8lx\n\0".as_ptr() as *const c_char, address); die(b"Die:\0".as_ptr() as *const c_char, regs, address as c_long); }
}

// CONFIG_FPU and CONFIG_OPENRISC_NO_SPR_SR_DSX select the corresponding kernel paths.
const INSN_LWA: u32 = 0x1b;
const INSN_SWA: u32 = 0x33;

#[no_mangle]
pub unsafe extern "C" fn do_illegal_instruction_impl(regs: *mut pt_regs, address: c_ulong) {
    let insn = *(address as *const u32);
    let op = insn >> 26;
    if op == INSN_LWA || op == INSN_SWA { (*regs).sr &= !SPR_SR_F; return; }
    if user_mode(regs) { force_sig_fault(SIGILL, ILL_ILLOPC, address as *mut c_void); }
    else { pr_emerg(b"KERNEL: Illegal instruction (SIGILL) 0x%.8lx\n\0".as_ptr() as *const c_char, address); die(b"Die:\0".as_ptr() as *const c_char, regs, address as c_long); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
