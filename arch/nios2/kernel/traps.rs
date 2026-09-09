/*
 * Hardware exception handling
 *
 * Copyright (C) 2010 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2004 Microtronix Datacom Ltd.
 * Copyright (C) 2001 Vic Phillips
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file COPYING in the main directory of this
 * archive for more details.
 */

// C dependencies: linux/sched.h, linux/sched/debug.h, linux/kernel.h,
// linux/signal.h, linux/export.h, linux/mm.h, linux/ptrace.h,
// asm/traps.h, asm/sections.h, and linux/uaccess.h.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct pt_regs {
    pub ea: c_ulong,
}

#[repr(C)]
pub struct task_struct {
    pub thread: thread_struct,
}

#[repr(C)]
pub struct thread_struct {
    pub ksp: c_ulong,
}

extern "C" {
    fn force_sig_fault(signo: c_int, code: c_int, addr: *mut c_void);
    fn console_verbose();
    fn spin_lock_irq(lock: *mut c_void);
    fn spin_unlock_irq(lock: *mut c_void);
    fn show_regs(regs: *mut pt_regs);
    fn make_task_dead(err: c_long) -> !;
    fn user_mode(regs: *const pt_regs) -> bool;
    fn printk(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_alert(fmt: *const c_char, ...);
    fn pr_emerg(fmt: *const c_char, ...);
    fn fixup_exception(regs: *mut pt_regs) -> bool;
    fn RDCTL(control: c_ulong) -> c_ulong;
    static mut _stext: c_char;
    static mut _etext: c_char;
}

static mut die_lock: c_void = c_void;

unsafe fn _send_sig(signo: c_int, code: c_int, addr: c_ulong) {
    force_sig_fault(signo, code, addr as *mut c_void);
}

pub unsafe fn die(str_: *const c_char, regs: *mut pt_regs, err: c_long) {
    console_verbose();
    spin_lock_irq(&mut die_lock as *mut c_void);
    pr_warn(b"Oops: %s, sig: %ld\n\0".as_ptr() as *const c_char, str_, err);
    show_regs(regs);
    spin_unlock_irq(&mut die_lock as *mut c_void);
    /*
     * make_task_dead() should take care of panic'ing from an interrupt
     * context so we don't handle it here
     */
    make_task_dead(err);
}

pub unsafe fn _exception(
    signo: c_int,
    regs: *mut pt_regs,
    code: c_int,
    addr: c_ulong,
) {
    if !user_mode(regs) {
        die(b"Exception in kernel mode\0".as_ptr() as *const c_char, regs, signo as c_long);
    }

    _send_sig(signo, code, addr);
}

/*
 * The show_stack() is external API which we do not use ourselves.
 */

pub static mut kstack_depth_to_print: c_int = 48;

pub unsafe fn show_stack(
    task: *mut task_struct,
    mut stack: *mut c_ulong,
    loglvl: *const c_char,
) {
    let mut endstack: *mut c_ulong;
    let mut addr: c_ulong;
    let mut i: c_int;

    if stack.is_null() {
        if !task.is_null() {
            stack = task.cast::<task_struct>().as_ref().unwrap().thread.ksp as *mut c_ulong;
        } else {
            stack = (&mut stack as *mut *mut c_ulong).cast::<c_ulong>();
        }
    }

    addr = stack as c_ulong;
    endstack = (((addr + 4095) & !4095) as *mut c_ulong);

    printk(b"%sStack from %08lx:\0".as_ptr() as *const c_char, loglvl, stack as c_ulong);
    i = 0;
    while i < kstack_depth_to_print {
        if stack.add(1) > endstack {
            break;
        }
        if i % 8 == 0 {
            printk(b"%s\n       \0".as_ptr() as *const c_char, loglvl);
        }
        printk(b"%s %08lx\0".as_ptr() as *const c_char, loglvl, *stack);
        stack = stack.add(1);
        i += 1;
    }

    printk(b"%s\nCall Trace:\0".as_ptr() as *const c_char, loglvl);
    i = 0;
    while stack.add(1) <= endstack {
        addr = *stack;
        stack = stack.add(1);
        /*
         * If the address is either in the text segment of the
         * kernel, or in the region which contains vmalloc'ed
         * memory, it *may* be the address of a calling
         * routine; if so, print it so that someone tracing
         * down the cause of the crash will be able to figure
         * out the call path that was taken.
         */
        if addr >= (&_stext as *mut c_char as c_ulong) && addr <= (&_etext as *mut c_char as c_ulong) {
            if i % 4 == 0 {
                pr_emerg(b"\n       \0".as_ptr() as *const c_char);
            }
            printk(b"%s [<%08lx>]\0".as_ptr() as *const c_char, loglvl, addr);
            i += 1;
        }
    }
    printk(b"%s\n\0".as_ptr() as *const c_char, loglvl);
}

/* Breakpoint handler */
pub unsafe extern "C" fn breakpoint_c(fp: *mut pt_regs) {
    /*
     * The breakpoint entry code has moved the PC on by 4 bytes, so we must
     * move it back. This could be done on the host but we do it here
     * because monitor.S of JTAG gdbserver does it too.
     */
    (*fp).ea = (*fp).ea.wrapping_sub(4);
    _exception(SIGTRAP, fp, TRAP_BRKPT, (*fp).ea);
}

/* Alignment exception handler; omitted when CONFIG_NIOS2_ALIGNMENT_TRAP is enabled. */
#[cfg(not(CONFIG_NIOS2_ALIGNMENT_TRAP))]
pub unsafe extern "C" fn handle_unaligned_c(fp: *mut pt_regs, mut cause: c_int) {
    let addr = RDCTL(CTL_BADADDR);

    cause >>= 2;
    (*fp).ea = (*fp).ea.wrapping_sub(4);

    if fixup_exception(fp) {
        return;
    }

    if !user_mode(fp) {
        pr_alert(b"Unaligned access from kernel mode, this might be a hardware\n\0".as_ptr() as *const c_char);
        pr_alert(b"problem, dump registers and restart the instruction\n\0".as_ptr() as *const c_char);
        pr_alert(b"  BADADDR 0x%08lx\n\0".as_ptr() as *const c_char, addr);
        pr_alert(b"  cause   %d\n\0".as_ptr() as *const c_char, cause);
        pr_alert(b"  op-code 0x%08lx\n\0".as_ptr() as *const c_char, *((*fp).ea as *const c_ulong));
        show_regs(fp);
        return;
    }

    _exception(SIGBUS, fp, BUS_ADRALN, addr);
}

/* Illegal instruction handler */
pub unsafe extern "C" fn handle_illegal_c(fp: *mut pt_regs) {
    (*fp).ea = (*fp).ea.wrapping_sub(4);
    _exception(SIGILL, fp, ILL_ILLOPC, (*fp).ea);
}

/* Supervisor instruction handler */
pub unsafe extern "C" fn handle_supervisor_instr(fp: *mut pt_regs) {
    (*fp).ea = (*fp).ea.wrapping_sub(4);
    _exception(SIGILL, fp, ILL_PRVOPC, (*fp).ea);
}

/* Division error handler */
pub unsafe extern "C" fn handle_diverror_c(fp: *mut pt_regs) {
    (*fp).ea = (*fp).ea.wrapping_sub(4);
    _exception(SIGFPE, fp, FPE_INTDIV, (*fp).ea);
}

/* Unhandled exception handler */
pub unsafe extern "C" fn unhandled_exception(regs: *mut pt_regs, mut cause: c_int) {
    let addr = RDCTL(CTL_BADADDR);

    cause /= 4;

    pr_emerg(
        b"Unhandled exception #%d in %s mode (badaddr=0x%08lx)\n\0".as_ptr() as *const c_char,
        cause,
        if user_mode(regs) { b"user\0".as_ptr() } else { b"kernel\0".as_ptr() },
        addr,
    );

    (*regs).ea = (*regs).ea.wrapping_sub(4);
    show_regs(regs);

    pr_emerg(b"opcode: 0x%08lx\n\0".as_ptr() as *const c_char, *((*regs).ea as *const c_ulong));
}

pub unsafe extern "C" fn handle_trap_1_c(fp: *mut pt_regs) {
    _send_sig(SIGUSR1, 0, (*fp).ea);
}

pub unsafe extern "C" fn handle_trap_2_c(fp: *mut pt_regs) {
    _send_sig(SIGUSR2, 0, (*fp).ea);
}

pub unsafe extern "C" fn handle_trap_3_c(fp: *mut pt_regs) {
    _send_sig(SIGILL, ILL_ILLTRP, (*fp).ea);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
