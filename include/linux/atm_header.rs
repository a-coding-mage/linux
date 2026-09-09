/* SPDX-License-Identifier: GPL-2.0 */
/* atm.h - general ATM declarations */

// Dependency equivalent of: #include <uapi/linux/atm.h>

// CONFIG_COMPAT conditionally includes <linux/compat.h> and its declarations.
#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct compat_atmif_sioc {
    pub number: ::core::ffi::c_int,
    pub length: ::core::ffi::c_int,
    pub arg: compat_uptr_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
