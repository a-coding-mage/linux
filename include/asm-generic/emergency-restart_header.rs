/* SPDX-License-Identifier: GPL-2.0 */

// Header guard: _ASM_GENERIC_EMERGENCY_RESTART_H

// Supplied by another translation unit/header.
unsafe extern "C" {
    fn machine_restart(cmd: *const core::ffi::c_char);
}

#[inline]
pub unsafe fn machine_emergency_restart() {
    unsafe {
        machine_restart(core::ptr::null());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
