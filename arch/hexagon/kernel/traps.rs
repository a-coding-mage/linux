// SPDX-License-Identifier: GPL-2.0-only
/* Kernel traps/events for Hexagon processor */

// C headers and configuration conditionals are supplied by the surrounding
// kernel translation unit.

const TRAP_SYSCALL: i32 = 1;
const TRAP_DEBUG: i32 = 0xdb;

#[cfg(CONFIG_GENERIC_BUG)]
pub unsafe extern "C" fn is_valid_bugaddr(_addr: usize) -> i32 { 1 }

unsafe fn ex_name(ex: i32) -> &'static str {
    match ex {
        HVM_GE_C_XPROT | HVM_GE_C_XUSER => "Execute protection fault",
        HVM_GE_C_RPROT | HVM_GE_C_RUSER => "Read protection fault",
        HVM_GE_C_WPROT | HVM_GE_C_WUSER => "Write protection fault",
        HVM_GE_C_XMAL => "Misaligned instruction",
        HVM_GE_C_WREG => "Multiple writes to same register in packet",
        HVM_GE_C_PCAL => "Program counter values that are not properly aligned",
        HVM_GE_C_RMAL => "Misaligned data load",
        HVM_GE_C_WMAL => "Misaligned data store",
        HVM_GE_C_INVI | HVM_GE_C_PRIVI => "Illegal instruction",
        HVM_GE_C_BUS => "Precise bus error",
        HVM_GE_C_CACHE => "Cache error",
        0xdb => "Debugger trap",
        _ => "Unrecognized exception",
    }
}

unsafe fn do_show_stack(mut task: *mut task_struct, mut fp: *mut usize,
                        mut ip: usize, loglvl: *const i8) {
    let mut kstack_depth_to_print = 24;
    let (mut offset, mut size): (usize, usize);
    let (mut name, mut modname): (*const i8, *mut i8) = (core::ptr::null(), core::ptr::null_mut());
    let mut newfp: *mut usize;
    let (mut low, mut high): (usize, usize);
    let mut tmpstr = [0i8; 128];
    let _ = &mut tmpstr;
    let mut i = 0;

    if task.is_null() { task = current; }
    printk(b"%sCPU#%d, %s/%d, Call Trace:\0".as_ptr() as *const i8,
           loglvl, raw_smp_processor_id(), (*task).comm.as_ptr(), task_pid_nr(task));

    if fp.is_null() {
        if task == current { core::arch::asm!("{0} = r30", out(reg) fp); }
        else { fp = (*((*task).thread.switch_sp as *mut hexagon_switch_stack)).fp as *mut usize; }
    }
    if (fp as usize & 3) != 0 || fp as usize < 0x1000 {
        printk(b"%s-- Corrupt frame pointer %p\0".as_ptr() as *const i8, loglvl, fp); return;
    }
    if ip == 0 { ip = *fp.add(1); }
    low = task_stack_page(task) as usize;
    high = low + THREAD_SIZE - 8;
    low += core::mem::size_of::<thread_info>();

    while i < kstack_depth_to_print {
        name = kallsyms_lookup(ip, &mut size, &mut offset, &mut modname, tmpstr.as_mut_ptr());
        printk(b"%s[%p] 0x%lx: %s + 0x%lx\0".as_ptr() as *const i8, loglvl, fp, ip, name, offset);
        if fp as usize < low || high < fp as usize { printk(b" (FP out of bounds!)\0".as_ptr() as *const i8); }
        if !modname.is_null() { printk(b" [%s] \0".as_ptr() as *const i8, modname); }
        printk(b"\n\0".as_ptr() as *const i8);
        newfp = *fp as *mut usize;
        if newfp as usize & 3 != 0 { printk(b"%s-- Corrupt frame pointer %p\0".as_ptr() as *const i8, loglvl, newfp); break; }
        if newfp.is_null() {
            let regs = (fp as *mut u8).add(8) as *mut pt_regs;
            if (*regs).syscall_nr != -1 { printk(b"%s-- trap0 -- syscall_nr: %ld\0".as_ptr() as *const i8, loglvl, (*regs).syscall_nr); printk(b"  psp: %lx  elr: %lx\n\0".as_ptr() as *const i8, pt_psp(regs), pt_elr(regs)); break; }
            kstack_depth_to_print += 6;
            printk(b"%s-- %s (0x%lx)  badva: %lx\n\0".as_ptr() as *const i8, loglvl, ex_name(pt_cause(regs)), pt_cause(regs), pt_badva(regs));
            newfp = (*regs).r30 as *mut usize; ip = pt_elr(regs);
        } else { ip = *newfp.add(1); }
        if ip == 0 { break; }
        if newfp > fp { fp = newfp; } else { break; }
        i += 1;
    }
}

pub unsafe extern "C" fn show_stack(task: *mut task_struct, fp: *mut usize, loglvl: *const i8) { do_show_stack(task, fp, 0, loglvl); }

pub unsafe extern "C" fn die(str_: *const i8, regs: *mut pt_regs, err: isize) -> i32 {
    console_verbose(); oops_enter(); spin_lock_irq(&mut DIE.lock); bust_spinlocks(1);
    printk(b"Oops: %s[#%d]:\n\0".as_ptr() as *const i8, str_, { DIE.counter += 1; DIE.counter });
    if notify_die(DIE_OOPS, str_, regs, err, pt_cause(regs), SIGSEGV) == NOTIFY_STOP { spin_unlock_irq(&mut DIE.lock); return 1; }
    print_modules(); show_regs(regs); do_show_stack(current, &mut (*regs).r30, pt_elr(regs), KERN_EMERG);
    bust_spinlocks(0); add_taint(TAINT_DIE, LOCKDEP_NOW_UNRELIABLE); spin_unlock_irq(&mut DIE.lock);
    if in_interrupt() { panic(b"Fatal exception in interrupt\0".as_ptr() as *const i8); }
    if panic_on_oops { panic(b"Fatal exception\0".as_ptr() as *const i8); }
    oops_exit(); make_task_dead(err); 0
}

pub unsafe extern "C" fn die_if_kernel(str_: *mut i8, regs: *mut pt_regs, err: isize) -> i32 { if !user_mode(regs) { die(str_, regs, err) } else { 0 } }

unsafe fn misaligned_instruction(regs: *mut pt_regs) { die_if_kernel(b"Misaligned Instruction\0".as_ptr() as *mut i8, regs, 0); force_sig(SIGBUS); }
unsafe fn misaligned_data_load(regs: *mut pt_regs) { die_if_kernel(b"Misaligned Data Load\0".as_ptr() as *mut i8, regs, 0); force_sig(SIGBUS); }
unsafe fn misaligned_data_store(regs: *mut pt_regs) { die_if_kernel(b"Misaligned Data Store\0".as_ptr() as *mut i8, regs, 0); force_sig(SIGBUS); }
unsafe fn illegal_instruction(regs: *mut pt_regs) { die_if_kernel(b"Illegal Instruction\0".as_ptr() as *mut i8, regs, 0); force_sig(SIGILL); }
unsafe fn precise_bus_error(regs: *mut pt_regs) { die_if_kernel(b"Precise Bus Error\0".as_ptr() as *mut i8, regs, 0); force_sig(SIGBUS); }
unsafe fn cache_error(regs: *mut pt_regs) { die(b"Cache Error\0".as_ptr() as *const i8, regs, 0); }

pub unsafe extern "C" fn do_genex(regs: *mut pt_regs) {
    match pt_cause(regs) {
        HVM_GE_C_XPROT | HVM_GE_C_XUSER => execute_protection_fault(regs),
        HVM_GE_C_RPROT | HVM_GE_C_RUSER => read_protection_fault(regs),
        HVM_GE_C_WPROT | HVM_GE_C_WUSER => write_protection_fault(regs),
        HVM_GE_C_XMAL | HVM_GE_C_PCAL => misaligned_instruction(regs),
        HVM_GE_C_WREG | HVM_GE_C_INVI | HVM_GE_C_PRIVI => illegal_instruction(regs),
        HVM_GE_C_RMAL => misaligned_data_load(regs), HVM_GE_C_WMAL => misaligned_data_store(regs),
        HVM_GE_C_BUS => precise_bus_error(regs), HVM_GE_C_CACHE => cache_error(regs),
        _ => panic(b"Unrecognized exception 0x%lx\n\0".as_ptr() as *const i8, pt_cause(regs)),
    }
}

pub unsafe extern "C" fn do_trap0(regs: *mut pt_regs) {
    match pt_cause(regs) {
        TRAP_SYSCALL => {
            if unlikely(test_thread_flag(TIF_SYSCALL_TRACE) && !ptrace_report_syscall_permit_entry(regs)) { return; }
            __vmsetie(VM_INT_ENABLE); (*regs).syscall_nr = (*regs).r06; (*regs).restart_r0 = (*regs).r00;
            if (*regs).syscall_nr as usize >= __NR_syscalls { (*regs).r00 = -1; }
            else { let syscall = sys_call_table[(*regs).syscall_nr as usize]; (*regs).r00 = syscall((*regs).r00, (*regs).r01, (*regs).r02, (*regs).r03, (*regs).r04, (*regs).r05); }
            if unlikely(test_thread_flag(TIF_SYSCALL_TRACE)) { ptrace_report_syscall_exit(regs, 0); }
        }
        TRAP_DEBUG => { if user_mode(regs) { force_sig_fault(SIGTRAP, TRAP_BRKPT, pt_elr(regs) as *mut core::ffi::c_void); } else { #[cfg(CONFIG_KGDB)] kgdb_handle_exception(pt_cause(regs), SIGTRAP, TRAP_BRKPT, regs); } }
        _ => {}
    }
}

pub unsafe extern "C" fn do_machcheck(_regs: *mut pt_regs) { __vmstop(); }
pub unsafe extern "C" fn do_debug_exception(regs: *mut pt_regs) { (*regs).hvmer.vmest &= !HVM_VMEST_CAUSE_MSK; (*regs).hvmer.vmest |= TRAP_DEBUG << HVM_VMEST_CAUSE_SFT; do_trap0(regs); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
