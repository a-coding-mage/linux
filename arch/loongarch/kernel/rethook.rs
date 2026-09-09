// SPDX-License-Identifier: GPL-2.0
/*
 * Generic return hook for LoongArch.
 */

// Dependencies supplied by the surrounding kernel sources:
// linux/kprobes.h, linux/rethook.h, and the local rethook.h.

use core::ffi::c_ulong;

#[repr(C)]
pub struct PtRegs {
    pub regs: [c_ulong; 32],
}

#[repr(C)]
pub struct RethookNode {
    pub frame: c_ulong,
    pub ret_addr: c_ulong,
}

unsafe extern "C" {
    fn rethook_trampoline_handler(regs: *mut PtRegs, frame: c_ulong) -> c_ulong;
    fn arch_rethook_trampoline();
}

/* This is called from arch_rethook_trampoline() */
#[no_mangle]
pub unsafe extern "C" fn arch_rethook_trampoline_callback(regs: *mut PtRegs) -> c_ulong {
    unsafe { rethook_trampoline_handler(regs, 0) }
}

// NOKPROBE_SYMBOL(arch_rethook_trampoline_callback);

pub unsafe extern "C" fn arch_rethook_prepare(
    rhn: *mut RethookNode,
    regs: *mut PtRegs,
    _mcount: bool,
) {
    unsafe {
        (*rhn).frame = 0;
        (*rhn).ret_addr = (*regs).regs[1];

        /* replace return addr with trampoline */
        (*regs).regs[1] = arch_rethook_trampoline as *const () as c_ulong;
    }
}

// NOKPROBE_SYMBOL(arch_rethook_prepare);

/* ASM function that handles the rethook must not be probed itself */
// NOKPROBE_SYMBOL(arch_rethook_trampoline);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
