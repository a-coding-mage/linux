// SPDX-License-Identifier: GPL-2.0-or-later

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_ulong;

extern "C" {
    fn arch_rethook_trampoline();
    fn rethook_trampoline_handler(
        regs: *mut pt_regs,
        frame: c_ulong,
    ) -> c_ulong;
}

pub unsafe fn arch_rethook_prepare(
    rh: *mut rethook_node,
    regs: *mut pt_regs,
    _mcount: bool,
) {
    (*rh).ret_addr = (*regs).gprs[14];
    (*rh).frame = (*regs).gprs[15];

    /* Replace the return addr with trampoline addr */
    (*regs).gprs[14] = arch_rethook_trampoline as usize as c_ulong;
}

pub unsafe fn arch_rethook_fixup_return(regs: *mut pt_regs, correct_ret_addr: c_ulong) {
    /* Replace fake return address with real one. */
    (*regs).gprs[14] = correct_ret_addr;
}

/*
 * Called from arch_rethook_trampoline
 */
pub unsafe fn arch_rethook_trampoline_callback(regs: *mut pt_regs) -> c_ulong {
    rethook_trampoline_handler(regs, (*regs).gprs[15])
}

/* assembler function that handles the rethook must not be probed itself */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
