/* SPDX-License-Identifier: GPL-2.0 */

// Original C header guard: REBOOT_H

use core::ffi::{c_ulong, c_void};

unsafe extern "C" {
    pub fn call_with_stack(
        fn_: Option<unsafe extern "C" fn(*mut c_void)>,
        arg: *mut c_void,
        sp: *mut c_void,
    );
    pub fn _soft_restart(addr: c_ulong, disable_l2: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
