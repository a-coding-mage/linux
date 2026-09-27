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
	pub sc_onstack: ::kernel::ffi::c_long,
	pub sc_mask: ::kernel::ffi::c_long,
	pub sc_pc: ::kernel::ffi::c_long,
	pub sc_ps: ::kernel::ffi::c_long,
	pub sc_regs: [::kernel::ffi::c_long; 32],
	pub sc_ownedfp: ::kernel::ffi::c_long,
	pub sc_fpregs: [::kernel::ffi::c_long; 32],
	pub sc_fpcr: ::kernel::ffi::c_ulong,
	pub sc_fp_control: ::kernel::ffi::c_ulong,
	pub sc_reserved1: ::kernel::ffi::c_ulong,
	pub sc_reserved2: ::kernel::ffi::c_ulong,
	pub sc_ssize: ::kernel::ffi::c_ulong,
	pub sc_sbase: *mut ::kernel::ffi::c_char,
	pub sc_traparg_a0: ::kernel::ffi::c_ulong,
	pub sc_traparg_a1: ::kernel::ffi::c_ulong,
	pub sc_traparg_a2: ::kernel::ffi::c_ulong,
	pub sc_fp_trap_pc: ::kernel::ffi::c_ulong,
	pub sc_fp_trigger_sum: ::kernel::ffi::c_ulong,
	pub sc_fp_trigger_inst: ::kernel::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
