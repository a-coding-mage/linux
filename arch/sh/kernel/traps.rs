// SPDX-License-Identifier: GPL-2.0
// C headers omitted; their declarations are supplied by the surrounding kernel.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct pt_regs {
    pub pc: c_ulong,
    pub regs: [c_ulong; 16],
}

#[repr(C)]
pub struct exception_table_entry {
    pub fixup: c_ulong,
}

#[repr(C)]
pub struct bug_entry {
    pub flags: c_ulong,
}

extern "C" {
    static mut unwinder_faulted: c_int;
    static mut panic_on_oops: c_int;
    static mut current: *mut c_void;

    fn oops_enter();
    fn spin_lock_irq(lock: *mut c_void);
    fn console_verbose();
    fn bust_spinlocks(on: c_int);
    fn printk(fmt: *const c_char, ...);
    fn print_modules();
    fn show_regs(regs: *mut pt_regs);
    fn task_pid_nr(task: *mut c_void) -> c_int;
    fn task_stack_page(task: *mut c_void) -> *mut c_void;
    fn user_mode(regs: *mut pt_regs) -> c_int;
    fn in_interrupt() -> c_int;
    fn dump_mem(name: *const c_char, level: *const c_char, start: c_ulong, end: c_ulong);
    fn notify_die(name: c_int, str: *const c_char, regs: *mut pt_regs, err: c_long, trap: c_int, sig: c_int) -> c_int;
    fn add_taint(taint: c_int, lockdep: c_int);
    fn spin_unlock_irq(lock: *mut c_void);
    fn oops_exit();
    fn kexec_should_crash(task: *mut c_void) -> c_int;
    fn crash_kexec(regs: *mut pt_regs);
    fn panic(fmt: *const c_char) -> !;
    fn make_task_dead(sig: c_int) -> !;
    fn search_exception_tables(pc: c_ulong) -> *const exception_table_entry;
    fn die(str: *const c_char, regs: *mut pt_regs, err: c_long) -> !;
    fn is_valid_bugaddr(addr: c_ulong) -> c_int;
    fn find_bug(addr: c_ulong) -> *const bug_entry;
    fn report_bug(addr: c_ulong, regs: *mut pt_regs) -> c_int;
    fn instruction_size(addr: c_ulong) -> c_ulong;
    fn get_kernel_nofault(value: *mut c_void, addr: *const c_void) -> c_int;
    fn __raw_readw(addr: c_ulong) -> u16;
    fn force_sig(sig: c_int);
    fn __kernel_text_address(addr: c_ulong) -> c_int;
    fn instruction_pointer(regs: *mut pt_regs) -> c_ulong;
    fn arch_ftrace_nmi_enter();
    fn nmi_enter();
    fn this_cpu_inc_nmi_count();
    fn nmi_exit();
    fn arch_ftrace_nmi_exit();
}

static mut die_lock: *mut c_void = core::ptr::null_mut();

pub unsafe fn die(str_: *const c_char, regs: *mut pt_regs, err: c_long) -> ! {
    static mut die_counter: c_int = 0;

    oops_enter();
    spin_lock_irq(&mut die_lock as *mut _ as *mut c_void);
    console_verbose();
    bust_spinlocks(1);
    die_counter += 1;
    printk(b"%s: %04lx [#%d]\n\0".as_ptr() as *const c_char, str_, err & 0xffff, die_counter);
    print_modules();
    show_regs(regs);
    printk(b"Process: %s (pid: %d, stack limit = %p)\n\0".as_ptr() as *const c_char,
        (*current).cast::<c_char>(), task_pid_nr(current), task_stack_page(current).add(1));

    if user_mode(regs) == 0 || in_interrupt() != 0 {
        dump_mem(b"Stack: \0".as_ptr() as *const c_char, b"\0".as_ptr() as *const c_char,
            (*regs).regs[15], 0 + task_stack_page(current) as c_ulong);
    }
    notify_die(0, str_, regs, err, 255, 11);
    bust_spinlocks(0);
    add_taint(0, 0);
    spin_unlock_irq(&mut die_lock as *mut _ as *mut c_void);
    oops_exit();
    if kexec_should_crash(current) != 0 { crash_kexec(regs); }
    if in_interrupt() != 0 { panic(b"Fatal exception in interrupt\0".as_ptr() as *const c_char); }
    if panic_on_oops != 0 { panic(b"Fatal exception\0".as_ptr() as *const c_char); }
    make_task_dead(11)
}

pub unsafe fn die_if_kernel(str_: *const c_char, regs: *mut pt_regs, err: c_long) {
    if user_mode(regs) == 0 { die(str_, regs, err); }
}

pub unsafe fn die_if_no_fixup(str_: *const c_char, regs: *mut pt_regs, err: c_long) {
    if user_mode(regs) == 0 {
        let fixup = search_exception_tables((*regs).pc);
        if !fixup.is_null() { (*regs).pc = (*fixup).fixup; return; }
        die(str_, regs, err);
    }
}

// CONFIG_GENERIC_BUG conditional section.
unsafe fn handle_BUG(regs: *mut pt_regs) {
    let bugaddr = (*regs).pc;
    if is_valid_bugaddr(bugaddr) == 0 { die(b"Kernel BUG\0".as_ptr() as *const c_char, regs, 0); }
    let bug = find_bug(bugaddr);
    if (*bug).flags & 1 != 0 { unwinder_faulted = 1; }
    if report_bug(bugaddr, regs) == 1 { (*regs).pc += instruction_size(bugaddr); return; }
    die(b"Kernel BUG\0".as_ptr() as *const c_char, regs, 0);
}

pub unsafe fn is_valid_bugaddr(addr: c_ulong) -> c_int {
    if addr < 0 { return 0; }
    let mut opcode: u16 = 0;
    if get_kernel_nofault(&mut opcode as *mut _ as *mut c_void, addr as *const c_void) != 0 { return 0; }
    if opcode == 0xc300 { return 1; }
    0
}

pub unsafe fn debug(regs: *mut pt_regs, vec: c_int) {
    (*regs).pc -= instruction_size(__raw_readw((*regs).pc - 4) as c_ulong);
    if notify_die(0, b"debug trap\0".as_ptr() as *const c_char, regs, 0, vec & 0xff, 5) == 0 { return; }
    force_sig(5);
}

pub unsafe fn bug(regs: *mut pt_regs, _vec: c_int) {
    (*regs).pc -= instruction_size(__raw_readw((*regs).pc - 4) as c_ulong);
    if notify_die(0, b"bug trap\0".as_ptr() as *const c_char, regs, 0, 0xc3 & 0xff, 5) == 0 { return; }
    if __kernel_text_address(instruction_pointer(regs)) != 0 {
        let insn = *(instruction_pointer(regs) as *const u16);
        if insn == 0xc300 { handle_BUG(regs); }
        return;
    }
    force_sig(5);
}

pub unsafe fn nmi(regs: *mut pt_regs, vec: c_int) {
    arch_ftrace_nmi_enter();
    nmi_enter();
    this_cpu_inc_nmi_count();
    match notify_die(2, b"NMI\0".as_ptr() as *const c_char, regs, 0, vec & 0xff, 2) {
        1 | 0 => {},
        2 => die(b"Fatal Non-Maskable Interrupt\0".as_ptr() as *const c_char, regs, 2),
        _ => printk(b"Got NMI, but nobody cared. Ignoring...\n\0".as_ptr() as *const c_char),
    }
    nmi_exit();
    arch_ftrace_nmi_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
