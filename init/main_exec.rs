// SPDX-License-Identifier: GPL-2.0-only
//! Init executable attempts for the staged, currently unselected boot owner.
//!
//! This is only the execution portion of `kernel_init`: it does not replace
//! boot initialization, lifetime transitions, or the freeing of init memory.

use super::{bindings, main_debug::main_debug, main_globals::*, main_printk::main_printk};
use kernel::ffi::{c_char, c_int};

/// Attempt one executable with the original boot argument/environment owners.
///
/// # Safety
/// The caller serializes boot-state access and provides terminated, live C
/// strings in `name` and both null-terminated argument/environment arrays.
pub(super) unsafe fn run_init_process(name: *const c_char) -> c_int {
    // SAFETY: Boot owns the arrays and strings throughout this attempt. Raw
    // pointers avoid holding Rust references across printk or kernel_execve.
    unsafe {
        argv_init[0] = name;
        main_printk!("run_init_process", b"\x016Run %s as init process\n\0", name);
        main_debug!("run_init_process", "  with arguments:\n");
        let mut argument = core::ptr::addr_of!(argv_init).cast::<*const c_char>();
        while !argument.read().is_null() {
            main_debug!("run_init_process", "    %s\n", argument.read());
            argument = argument.add(1);
        }
        main_debug!("run_init_process", "  with environment:\n");
        let mut environment = core::ptr::addr_of!(envp_init).cast::<*const c_char>();
        while !environment.read().is_null() {
            main_debug!("run_init_process", "    %s\n", environment.read());
            environment = environment.add(1);
        }
        bindings::kernel_execve(
            name,
            core::ptr::addr_of!(argv_init).cast(),
            core::ptr::addr_of!(envp_init).cast(),
        )
    }
}

/// Try a fallback executable, suppressing only the original ENOENT diagnostic.
///
/// # Safety
/// The same string and boot-state requirements as `run_init_process` apply.
pub(super) unsafe fn try_to_run_init_process(name: *const c_char) -> c_int {
    // SAFETY: The caller grants the same serialized ownership for this attempt.
    let result = unsafe { run_init_process(name) };
    if result != 0 && result != -(bindings::ENOENT as c_int) {
        // SAFETY: The format matches the live name pointer and promoted C int.
        #[cfg_attr(not(CONFIG_PRINTK), allow(unused_unsafe))]
        unsafe {
            main_printk!(
                "try_to_run_init_process",
                b"\x013Starting init: %s exists but couldn't execute it (error %d)\n\0",
                name,
                result
            );
        }
    }
    result
}

/// Execute the original ramdisk, explicit, default, and conventional fallbacks.
///
/// This is called only after the still-unimplemented preceding `kernel_init`
/// lifecycle has completed; success has the original zero return value.
///
/// # Safety
/// The caller completed that boot lifecycle and grants exclusive use of the
/// initialized boot globals and live strings for all attempted executions.
pub(super) unsafe fn execute_init_processes() -> c_int {
    // SAFETY: The caller establishes the original kernel_init execution phase.
    unsafe {
        if !ramdisk_execute_command.is_null() {
            let result = run_init_process(ramdisk_execute_command);
            if result == 0 {
                return 0;
            }
            main_printk!(
                "kernel_init",
                b"\x013Failed to execute %s (error %d)\n\0",
                ramdisk_execute_command,
                result
            );
        }
        if !execute_command.is_null() {
            let result = run_init_process(execute_command);
            if result == 0 {
                return 0;
            }
            bindings::panic(
                b"Requested init %s failed (error %d).\0".as_ptr().cast(),
                execute_command,
                result,
            );
        }
        if bindings::RUST_INIT_MAIN_DEFAULT_INIT[0] != 0 {
            let default = bindings::RUST_INIT_MAIN_DEFAULT_INIT.as_ptr().cast();
            let result = run_init_process(default);
            if result == 0 {
                return 0;
            }
            main_printk!(@index bindings::RUST_INIT_MAIN_DEFAULT_INIT[0] != 0,
                         "kernel_init", b"\x013Default init %s failed (error %d)\n\0",
                         default, result);
        }
        if try_to_run_init_process(b"/sbin/init\0".as_ptr().cast()) == 0
            || try_to_run_init_process(b"/etc/init\0".as_ptr().cast()) == 0
            || try_to_run_init_process(b"/bin/init\0".as_ptr().cast()) == 0
            || try_to_run_init_process(b"/bin/sh\0".as_ptr().cast()) == 0
        {
            return 0;
        }
        bindings::panic(
            concat!(
                "No working init found.  Try passing init= option to kernel. ",
                "See Linux Documentation/admin-guide/init.rst for guidance.\0",
            )
            .as_ptr()
            .cast(),
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
