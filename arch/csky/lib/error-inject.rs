// SPDX-License-Identifier: GPL-2.0

// External kernel types and helpers supplied by the surrounding Rust bindings.
#[repr(C)]
pub struct pt_regs {
    pub lr: usize,
}

unsafe extern "C" {
    fn instruction_pointer_set(regs: *mut pt_regs, val: usize);
}

#[no_mangle]
pub unsafe extern "C" fn override_function_with_return(regs: *mut pt_regs) {
    instruction_pointer_set(regs, (*regs).lr);
}

// NOKPROBE_SYMBOL(override_function_with_return);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
