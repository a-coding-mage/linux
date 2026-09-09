/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2021 Western Digital Corporation or its affiliates.
 * Copyright (c) 2022 Ventana Micro Systems Inc.
 */

// Dependency intent: `pt_regs` is supplied by asm/ptrace.h.

#[repr(C)]
pub struct suspend_context {
	/* Saved and restored by low-level functions */
	pub regs: pt_regs,
	/* Saved and restored by high-level functions */
	pub envcfg: ::core::ffi::c_ulong,
	pub tvec: ::core::ffi::c_ulong,
	pub ie: ::core::ffi::c_ulong,
	// CONFIG_MMU controls whether the following fields are present.
	#[cfg(CONFIG_MMU)]
	pub satp: ::core::ffi::c_ulong,
	#[cfg(CONFIG_MMU)]
	pub stimecmp: ::core::ffi::c_ulong,
	// __riscv_xlen < 64 controls whether this field is present.
	#[cfg(all(CONFIG_MMU, not(target_pointer_width = "64")))]
	pub stimecmph: ::core::ffi::c_ulong,
}

/* Used by hibernation core and cleared during resume sequence */
pub static mut in_suspend: ::core::ffi::c_int;

unsafe extern "C" {
	/* Low-level CPU suspend entry function */
	pub fn __cpu_suspend_enter(context: *mut suspend_context) -> ::core::ffi::c_int;

	/* High-level CPU suspend which will save context and call finish() */
	pub fn cpu_suspend(
		arg: ::core::ffi::c_ulong,
		finish: Option<unsafe extern "C" fn(::core::ffi::c_ulong, ::core::ffi::c_ulong, ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
	) -> ::core::ffi::c_int;

	/* Low-level CPU resume entry function */
	pub fn __cpu_resume_enter(
		hartid: ::core::ffi::c_ulong,
		context: ::core::ffi::c_ulong,
	) -> ::core::ffi::c_int;

	/* Used to save and restore the CSRs */
	pub fn suspend_save_csrs(context: *mut suspend_context);
	pub fn suspend_restore_csrs(context: *mut suspend_context);

	/* Low-level API to support hibernation */
	pub fn swsusp_arch_suspend() -> ::core::ffi::c_int;
	pub fn swsusp_arch_resume() -> ::core::ffi::c_int;
	pub fn arch_hibernation_header_save(
		addr: *mut ::core::ffi::c_void,
		max_size: ::core::ffi::c_uint,
	) -> ::core::ffi::c_int;
	pub fn arch_hibernation_header_restore(addr: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
	pub fn __hibernate_cpu_resume() -> ::core::ffi::c_int;

	/* Used to resume on the CPU we hibernated on */
	pub fn hibernate_resume_nonboot_cpu_disable() -> ::core::ffi::c_int;

	// `asmlinkage` is a C calling-convention annotation with no separate Rust syntax.
	pub fn hibernate_restore_image(
		resume_satp: ::core::ffi::c_ulong,
		satp_temp: ::core::ffi::c_ulong,
		cpu_resume: ::core::ffi::c_ulong,
	);
	pub fn hibernate_core_restore_code() -> ::core::ffi::c_int;
	pub fn riscv_sbi_hsm_is_supported() -> bool;
	pub fn riscv_sbi_suspend_state_is_valid(state: u32) -> bool;
	pub fn riscv_sbi_hart_suspend(state: u32) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
