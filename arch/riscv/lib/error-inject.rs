// SPDX-License-Identifier: GPL-2.0

// Translated from linux/error-injection.h and linux/kprobes.h dependencies.

#[repr(C)]
pub struct pt_regs {
    pub ra: usize,
}

extern "C" {
    fn instruction_pointer_set(regs: *mut pt_regs, value: usize);
}

pub unsafe fn override_function_with_return(regs: *mut pt_regs) {
    instruction_pointer_set(regs, (*regs).ra);
}

// NOKPROBE_SYMBOL(override_function_with_return);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
