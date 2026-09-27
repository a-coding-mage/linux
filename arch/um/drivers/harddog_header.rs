/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: UM_WATCHDOG_H

unsafe extern "C" {
    pub fn start_watchdog(
        in_fd_ret: *mut ::kernel::ffi::c_int,
        out_fd_ret: *mut ::kernel::ffi::c_int,
        sock: *mut ::kernel::ffi::c_char,
    ) -> ::kernel::ffi::c_int;
    pub fn stop_watchdog(in_fd: ::kernel::ffi::c_int, out_fd: ::kernel::ffi::c_int);
    pub fn ping_watchdog(fd: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
