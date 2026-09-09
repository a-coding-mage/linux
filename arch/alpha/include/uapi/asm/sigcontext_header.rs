/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

#[repr(C)]
pub struct sigcontext {
	/*
	 * What should we have here? I'd probably better use the same
	 * stack layout as OSF/1, just in case we ever want to try
	 * running their binaries..
	 *
	 * This is the basic layout, but I don't know if we'll ever
	 * actually fill in all the values..
	 */
	pub sc_onstack: ::core::ffi::c_long,
	pub sc_mask: ::core::ffi::c_long,
	pub sc_pc: ::core::ffi::c_long,
	pub sc_ps: ::core::ffi::c_long,
	pub sc_regs: [::core::ffi::c_long; 32],
	pub sc_ownedfp: ::core::ffi::c_long,
	pub sc_fpregs: [::core::ffi::c_long; 32],
	pub sc_fpcr: ::core::ffi::c_ulong,
	pub sc_fp_control: ::core::ffi::c_ulong,
	pub sc_reserved1: ::core::ffi::c_ulong,
	pub sc_reserved2: ::core::ffi::c_ulong,
	pub sc_ssize: ::core::ffi::c_ulong,
	pub sc_sbase: *mut ::core::ffi::c_char,
	pub sc_traparg_a0: ::core::ffi::c_ulong,
	pub sc_traparg_a1: ::core::ffi::c_ulong,
	pub sc_traparg_a2: ::core::ffi::c_ulong,
	pub sc_fp_trap_pc: ::core::ffi::c_ulong,
	pub sc_fp_trigger_sum: ::core::ffi::c_ulong,
	pub sc_fp_trigger_inst: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
