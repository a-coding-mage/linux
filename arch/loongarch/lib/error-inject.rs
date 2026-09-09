// SPDX-License-Identifier: GPL-2.0

// Dependency declarations corresponding to <linux/error-injection.h> and
// <linux/kprobes.h> are supplied by the surrounding translation unit.

pub unsafe fn override_function_with_return(regs: *mut pt_regs) {
    instruction_pointer_set(regs, (*regs).regs[1]);
}

// NOKPROBE_SYMBOL(override_function_with_return);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
