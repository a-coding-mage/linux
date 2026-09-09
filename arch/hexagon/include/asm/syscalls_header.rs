/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <asm-generic/syscalls.h>

unsafe extern "C" {
    pub fn sys_hexagon_fadvise64_64(
        fd: ::core::ffi::c_int,
        advice: ::core::ffi::c_int,
        a2: u32,
        a3: u32,
        a4: u32,
        a5: u32,
    ) -> ::core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
