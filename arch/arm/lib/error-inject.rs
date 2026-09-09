// SPDX-License-Identifier: GPL-2.0

// Declarations supplied by the Linux kernel headers.
#[repr(C)]
pub struct pt_regs {
    pub ARM_lr: usize,
}

extern "C" {
    fn instruction_pointer_set(regs: *mut pt_regs, value: usize);
}

pub unsafe fn override_function_with_return(regs: *mut pt_regs) {
    instruction_pointer_set(regs, (*regs).ARM_lr);
}

// NOKPROBE_SYMBOL(override_function_with_return);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
