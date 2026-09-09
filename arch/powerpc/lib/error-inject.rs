// SPDX-License-Identifier: GPL-2.0+

// Declarations supplied by the kernel's error-injection, kprobes, and
// userspace-access dependencies.
use crate::bindings::pt_regs;

extern "C" {
    fn regs_set_return_ip(regs: *mut pt_regs, ip: usize);
}

pub unsafe extern "C" fn override_function_with_return(regs: *mut pt_regs) {
    /*
     * Emulate 'blr'. 'regs' represents the state on entry of a predefined
     * function in the kernel/module, captured on a kprobe. We don't need
     * to worry about 32-bit userspace on a 64-bit kernel.
     */
    regs_set_return_ip(regs, (*regs).link);
}

// NOKPROBE_SYMBOL(override_function_with_return);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
