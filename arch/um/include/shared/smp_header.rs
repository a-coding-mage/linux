/* SPDX-License-Identifier: GPL-2.0 */

// Equivalent to: #if IS_ENABLED(CONFIG_SMP)
#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub static mut uml_ncpus: ::core::ffi::c_int;
}

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn uml_curr_cpu() -> ::core::ffi::c_int;
    pub fn uml_start_secondary(opaque: *mut ::core::ffi::c_void);
    pub fn uml_ipi_handler(vector: ::core::ffi::c_int);
}

// Equivalent to: #else /* !CONFIG_SMP */
#[cfg(not(feature = "CONFIG_SMP"))]
pub const uml_ncpus: ::core::ffi::c_int = 1;

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub const fn uml_curr_cpu() -> ::core::ffi::c_int {
    0
}

// Equivalent to: #endif /* CONFIG_SMP */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
