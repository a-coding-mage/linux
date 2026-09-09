// SPDX-License-Identifier: GPL-2.0+
// Dependencies corresponding to <asm/ptrace.h>, <linux/error-injection.h>,
// and <linux/kprobes.h> are supplied by the surrounding translation.

use crate::bindings::pt_regs;

/// Emulate `br 14`. `regs` is captured by kprobes on entry to some kernel
/// function.
#[no_mangle]
pub unsafe extern "C" fn override_function_with_return(regs: *mut pt_regs) {
    // `regs` is a C pointer; dereference it directly to preserve C behavior.
    (*regs).psw.addr = (*regs).gprs[14];
}

// NOKPROBE_SYMBOL(override_function_with_return);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
