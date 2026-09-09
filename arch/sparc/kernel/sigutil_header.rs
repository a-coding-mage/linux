/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _SIGUTIL_H

extern "C" {
    pub fn save_fpu_state(regs: *mut pt_regs, fpu: *mut __siginfo_fpu_t) -> i32;
    pub fn restore_fpu_state(regs: *mut pt_regs, fpu: *mut __siginfo_fpu_t) -> i32;
    pub fn save_rwin_state(wsaved: i32, rwin: *mut __siginfo_rwin_t) -> i32;
    pub fn restore_rwin_state(rp: *mut __siginfo_rwin_t) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
