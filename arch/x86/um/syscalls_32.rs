// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the surrounding kernel translation.

/// `arch_prctl` system call.
pub unsafe fn arch_prctl(
    option: ::core::ffi::c_int,
    arg2: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    let _ = (option, arg2);
    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
