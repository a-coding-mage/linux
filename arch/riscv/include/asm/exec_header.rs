/* SPDX-License-Identifier: GPL-2.0-only */

extern "C" {
    pub fn arch_align_stack(sp: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
