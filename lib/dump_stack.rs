// SPDX-License-Identifier: GPL-2.0
/*
 * Provide a default dump_stack() function for architectures
 * which don't implement their own.
 */

// Dependencies: linux/kernel.h, linux/buildid.h, linux/sched.h,
// linux/sched/debug.h, linux/smp.h, linux/atomic.h, linux/kexec.h,
// linux/utsname.h, linux/stop_machine.h.
//
// The C-variadic dump_stack_set_arch_desc() uses the compiler's VaList ABI,
// admitted for this object by lib/Makefile (RUST_DUMP_STACK requires the
// inspected rustc 1.85.1, as RUST_PANIC does).

use core::ffi::VaList;
use core::ptr::{addr_of, addr_of_mut};
use kernel::bindings::{
    __printk_cpu_sync_put, __printk_cpu_sync_try_get, __printk_cpu_sync_wait, _printk, get_current,
    get_taint, panic_on_this_cpu, preempt_model_str, print_tainted, print_tainted_verbose,
    print_worker_info, strcspn, task_struct,
};
use kernel::ffi::{c_char, c_int, c_uint, c_ulong};

// task_struct embeds zero-sized lockdep keys; only pointers cross here.
#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "vsnprintf"]
    fn native_vsnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, args: VaList<'_, '_>) -> c_int;
    fn show_stack(task: *mut task_struct, sp: *mut c_ulong, loglvl: *const c_char);

    fn lupos_dump_stack_cpu() -> c_int;
    fn lupos_dump_stack_current_euid() -> c_uint;
    fn lupos_dump_stack_kexec_crash_loaded() -> bool;
    fn lupos_dump_stack_uts_release() -> *const c_char;
    fn lupos_dump_stack_uts_version() -> *const c_char;
    fn lupos_dump_stack_print_stop_info(log_lvl: *const c_char, task: *mut task_struct);
    fn lupos_dump_stack_print_scx_info(log_lvl: *const c_char, task: *mut task_struct);
    fn lupos_dump_stack_irq_save() -> c_ulong;
    fn lupos_dump_stack_irq_restore(flags: c_ulong);
}

#[cfg(CONFIG_STACKTRACE_BUILD_ID)]
extern "C" {
    static vmlinux_build_id: [u8; 20];
}

static mut DUMP_STACK_ARCH_DESC_STR: [c_char; 128] = [0; 128];

/**
 * dump_stack_set_arch_desc - set arch-specific str to show with task dumps
 * @fmt: printf-style format string
 * @...: arguments for the format string
 *
 * The configured string will be printed right after utsname during task
 * dumps.  Usually used to add arch-specific system identifiers.  If an
 * arch wants to make use of such an ID string, it should initialize this
 * as soon as possible during boot.
 */
/// # Safety
/// `fmt` is a printf format matching the variadic arguments; called during
/// early boot.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn dump_stack_set_arch_desc(fmt: *const c_char, mut args: ...) {
    // SAFETY: va_start/va_end bracket the bounded vsnprintf(), as in C.
    unsafe {
        let buf = addr_of_mut!(DUMP_STACK_ARCH_DESC_STR).cast::<c_char>();
        native_vsnprintf(buf, 128, fmt, args.as_va_list());
    }
}

/**
 * dump_stack_print_info - print generic debug info for dump_stack()
 * @log_lvl: log level
 *
 * Arch-specific dump_stack() implementations can use this function to
 * print out the same debug information as the generic dump_stack().
 */
/// # Safety
/// `log_lvl` is a NUL-terminated printk level.
#[no_mangle]
pub unsafe extern "C" fn dump_stack_print_info(log_lvl: *const c_char) {
    // SAFETY: current and the init uts namespace are live; every string
    // passed to printk is NUL-terminated.
    unsafe {
        let current = get_current();
        let version = lupos_dump_stack_uts_version();
        let kexec: *const c_char = if lupos_dump_stack_kexec_crash_loaded() {
            c"Kdump: loaded ".as_ptr().cast()
        } else {
            c"".as_ptr().cast()
        };

        #[cfg(not(CONFIG_STACKTRACE_BUILD_ID))]
        _printk(
            c"%sCPU: %d UID: %u PID: %d Comm: %.20s %s%s %s %.*s %s %s\n".as_ptr().cast(),
            log_lvl,
            lupos_dump_stack_cpu(),
            lupos_dump_stack_current_euid(),
            (*current).pid,
            addr_of!((*current).comm).cast::<c_char>(),
            kexec,
            print_tainted(),
            lupos_dump_stack_uts_release(),
            strcspn(version, c" ".as_ptr().cast()) as c_int,
            version,
            preempt_model_str(),
            c"".as_ptr(),
        );
        #[cfg(CONFIG_STACKTRACE_BUILD_ID)]
        _printk(
            c"%sCPU: %d UID: %u PID: %d Comm: %.20s %s%s %s %.*s %s  %20phN\n".as_ptr().cast(),
            log_lvl,
            lupos_dump_stack_cpu(),
            lupos_dump_stack_current_euid(),
            (*current).pid,
            addr_of!((*current).comm).cast::<c_char>(),
            kexec,
            print_tainted(),
            lupos_dump_stack_uts_release(),
            strcspn(version, c" ".as_ptr().cast()) as c_int,
            version,
            preempt_model_str(),
            addr_of!(vmlinux_build_id).cast::<u8>(),
        );

        if get_taint() != 0 {
            _printk(c"%s%s\n".as_ptr().cast(), log_lvl, print_tainted_verbose());
        }

        let desc = addr_of!(DUMP_STACK_ARCH_DESC_STR).cast::<c_char>();
        if *desc != 0 {
            _printk(c"%sHardware name: %s\n".as_ptr().cast(), log_lvl, desc);
        }

        print_worker_info(log_lvl, current);
        lupos_dump_stack_print_stop_info(log_lvl, current);
        lupos_dump_stack_print_scx_info(log_lvl, current);
    }
}

/**
 * show_regs_print_info - print generic debug info for show_regs()
 * @log_lvl: log level
 *
 * show_regs() implementations can use this function to print out generic
 * debug information.
 */
/// # Safety
/// `log_lvl` is a NUL-terminated printk level.
#[no_mangle]
pub unsafe extern "C" fn show_regs_print_info(log_lvl: *const c_char) {
    // SAFETY: forwarded caller contract.
    unsafe { dump_stack_print_info(log_lvl) };
}

/// # Safety
/// `log_lvl` is a NUL-terminated printk level.
unsafe fn __dump_stack(log_lvl: *const c_char) {
    // SAFETY: forwarded caller contract.
    unsafe {
        dump_stack_print_info(log_lvl);
        show_stack(core::ptr::null_mut(), core::ptr::null_mut(), log_lvl);
    }
}

/**
 * dump_stack_lvl - dump the current task information and its stack trace
 * @log_lvl: log level
 *
 * Architectures can override this implementation by implementing its own.
 */
/// # Safety
/// `log_lvl` is a NUL-terminated printk level.
#[no_mangle]
pub unsafe extern "C" fn dump_stack_lvl(log_lvl: *const c_char) {
    // SAFETY: the printk CPU sync and IRQ state are paired as in C.
    unsafe {
        let in_panic = panic_on_this_cpu();
        let mut flags: c_ulong = 0;

        /*
         * Permit this cpu to perform nested stack dumps while serialising
         * against other CPUs, unless this CPU is in panic.
         *
         * When in panic, non-panic CPUs are not permitted to store new
         * printk messages so there is no need to synchronize the output.
         * This avoids potential deadlock in panic() if another CPU is
         * holding and unable to release the printk_cpu_sync.
         */
        if !in_panic {
            // printk_cpu_sync_get_irqsave(flags)
            loop {
                flags = lupos_dump_stack_irq_save();
                if __printk_cpu_sync_try_get() != 0 {
                    break;
                }
                lupos_dump_stack_irq_restore(flags);
                __printk_cpu_sync_wait();
            }
        }

        __dump_stack(log_lvl);

        if !in_panic {
            // printk_cpu_sync_put_irqrestore(flags)
            __printk_cpu_sync_put();
            lupos_dump_stack_irq_restore(flags);
        }
    }
}
// EXPORT_SYMBOL(dump_stack_lvl);

/// Dumps the current task information and its stack trace.
#[no_mangle]
pub extern "C" fn dump_stack() {
    // SAFETY: KERN_DEFAULT is the empty level string.
    unsafe { dump_stack_lvl(c"".as_ptr().cast()) };
}
// EXPORT_SYMBOL(dump_stack);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
