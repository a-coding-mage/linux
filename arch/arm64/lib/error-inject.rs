// SPDX-License-Identifier: GPL-2.0

// External declarations supplied by <linux/error-injection.h> and
// <linux/kprobes.h>.
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn instruction_pointer_set(regs: *mut pt_regs, val: u64);
    fn procedure_link_pointer(regs: *mut pt_regs) -> u64;
}

#[no_mangle]
pub unsafe extern "C" fn override_function_with_return(regs: *mut pt_regs) {
    /*
     * 'regs' represents the state on entry of a predefined function in
     * the kernel/module and which is captured on a kprobe.
     *
     * When kprobe returns back from exception it will override the end
     * of probed function and directly return to the predefined
     * function's caller.
     */
    instruction_pointer_set(regs, procedure_link_pointer(regs));
}

// NOKPROBE_SYMBOL(override_function_with_return);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
