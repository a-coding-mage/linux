/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2019 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *	Atish Patra <atish.patra@wdc.com>
 */

/* Translated from the Linux hrtimer-dependent C header. */

#[repr(C)]
pub struct kvm_guest_timer {
	/* Mult & Shift values to get nanoseconds from cycles */
	pub nsec_mult: u32,
	pub nsec_shift: u32,
	/* Time delta value */
	pub time_delta: u64,
}

#[repr(C)]
pub struct kvm_vcpu_timer {
	/* Flag for whether init is done */
	pub init_done: bool,
	/* Flag for whether timer event is configured */
	pub next_set: bool,
	/* Next timer event cycles */
	pub next_cycles: u64,
	/* Underlying hrtimer instance */
	pub hrt: hrtimer,

	/* Flag to check if sstc is enabled or not */
	pub sstc_enabled: bool,
	/* A function pointer to switch between stimecmp or hrtimer at runtime */
	pub timer_next_event: Option<unsafe extern "C" fn(vcpu: *mut kvm_vcpu, ncycles: u64) -> i32>,
}

extern "C" {
	pub fn kvm_riscv_vcpu_timer_next_event(
		vcpu: *mut kvm_vcpu,
		ncycles: u64,
	) -> i32;
	pub fn kvm_riscv_vcpu_get_reg_timer(
		vcpu: *mut kvm_vcpu,
		reg: *const kvm_one_reg,
	) -> i32;
	pub fn kvm_riscv_vcpu_set_reg_timer(
		vcpu: *mut kvm_vcpu,
		reg: *const kvm_one_reg,
	) -> i32;
	pub fn kvm_riscv_vcpu_timer_init(vcpu: *mut kvm_vcpu) -> i32;
	pub fn kvm_riscv_vcpu_timer_deinit(vcpu: *mut kvm_vcpu) -> i32;
	pub fn kvm_riscv_vcpu_timer_reset(vcpu: *mut kvm_vcpu) -> i32;
	pub fn kvm_riscv_vcpu_timer_restore(vcpu: *mut kvm_vcpu);
	pub fn kvm_riscv_guest_timer_init(kvm: *mut kvm);
	pub fn kvm_riscv_vcpu_timer_sync(vcpu: *mut kvm_vcpu);
	pub fn kvm_riscv_vcpu_timer_save(vcpu: *mut kvm_vcpu);
	pub fn kvm_riscv_vcpu_timer_pending(vcpu: *mut kvm_vcpu) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
