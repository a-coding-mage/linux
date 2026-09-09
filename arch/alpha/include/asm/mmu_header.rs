/* SPDX-License-Identifier: GPL-2.0 */

/* The alpha MMU context is one "unsigned long" bitmap per CPU */
pub type mm_context_t = [core::ffi::c_ulong; NR_CPUS];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
