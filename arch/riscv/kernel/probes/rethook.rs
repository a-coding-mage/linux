// SPDX-License-Identifier: GPL-2.0-only
/*
 * Generic return hook for riscv.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kprobes.h, linux/rethook.h, and "rethook.h".

/// This is called from arch_rethook_trampoline().
#[no_mangle]
pub unsafe extern "C" fn arch_rethook_trampoline_callback(
    regs: *mut pt_regs,
) -> ::core::ffi::c_ulong {
    rethook_trampoline_handler(regs, (*regs).s0)
}

// NOKPROBE_SYMBOL(arch_rethook_trampoline_callback)

#[no_mangle]
pub unsafe extern "C" fn arch_rethook_prepare(
    rhn: *mut rethook_node,
    regs: *mut pt_regs,
    _mcount: bool,
) {
    (*rhn).ret_addr = (*regs).ra;
    (*rhn).frame = (*regs).s0;

    /* replace return addr with trampoline */
    (*regs).ra = arch_rethook_trampoline as usize as ::core::ffi::c_ulong;
}

// NOKPROBE_SYMBOL(arch_rethook_prepare)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
