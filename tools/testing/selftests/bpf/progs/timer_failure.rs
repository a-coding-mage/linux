// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// Translated from C. External BPF helper types, constants, section/log/failure
// annotations, and verifier-test metadata are supplied by the surrounding
// selftest build environment.

use core::arch::asm;
use core::ffi::{c_int, c_long, c_ulong, c_void};

#[used]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct bpf_timer {
	_priv: [u8; 0],
}

#[repr(C)]
pub struct elem {
	pub t: bpf_timer,
}

pub const BPF_MAP_TYPE_ARRAY: u32 = 2;
pub const BPF_F_TEST_STATE_FREQ: u32 = 1;
pub const CLOCK_BOOTTIME: c_int = 7;

// C declaration:
// struct {
// 	__uint(type, BPF_MAP_TYPE_ARRAY);
// 	__uint(max_entries, 1);
// 	__type(key, int);
// 	__type(value, struct elem);
// } timer_map SEC(".maps");
#[repr(C)]
pub struct timer_map_def {
	pub type_: u32,
	pub max_entries: u32,
	pub key_size: u32,
	pub value_size: u32,
}

#[used]
#[link_section = ".maps"]
pub static mut timer_map: timer_map_def = timer_map_def {
	type_: BPF_MAP_TYPE_ARRAY,
	max_entries: 1,
	key_size: core::mem::size_of::<c_int>() as u32,
	value_size: core::mem::size_of::<elem>() as u32,
};

unsafe extern "C" {
	fn bpf_get_prandom_u32() -> u32;
	fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
	fn bpf_timer_init(timer: *mut bpf_timer, map: *mut c_void, clockid: c_int) -> c_int;
	fn bpf_timer_set_callback(
		timer: *mut bpf_timer,
		callback: unsafe extern "C" fn() -> c_ulong,
	) -> c_int;
	fn bpf_timer_start(timer: *mut bpf_timer, nsecs: u64, flags: u64) -> c_int;
}

#[inline(never)]
#[used]
unsafe extern "C" fn timer_cb_ret_bad() -> c_ulong {
	unsafe {
		asm!(
			"call {bpf_get_prandom_u32}",
			"if r0 s> 1000 goto 1f",
			"r0 = 0",
			"1:",
			"goto +0", // checkpoint
			// async callback is expected to return 0, so branch above
			// skipping r0 = 0; should lead to a failure, but if exit
			// instruction doesn't enforce r0's precision, this callback
			// will be successfully verified
			"exit",
			bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
			options(noreturn),
		);
	}
}

// SEC("fentry/bpf_fentry_test1")
// __log_level(2)
// __flag(BPF_F_TEST_STATE_FREQ)
// __failure
// check that fallthrough code path marks r0 as precise
// __msg("mark_precise: frame0: regs=r0 stack= before")
// __msg(": (85) call bpf_get_prandom_u32#7") /* anchor message */
// check that branch code path marks r0 as precise
// __msg("mark_precise: frame0: regs=r0 stack= before ") __msg(": (85) call bpf_get_prandom_u32#7")
// __msg("should have been in [0, 0]")
#[link_section = "fentry/bpf_fentry_test1"]
pub unsafe extern "C" fn test_bad_ret(a: c_int) -> c_long {
	let mut key: c_int = 0;
	let mut timer: *mut bpf_timer;

	timer = unsafe {
		bpf_map_lookup_elem(
			&raw mut timer_map as *mut c_void,
			&raw mut key as *mut c_int as *const c_void,
		) as *mut bpf_timer
	};
	if !timer.is_null() {
		unsafe {
			bpf_timer_init(timer, &raw mut timer_map as *mut c_void, CLOCK_BOOTTIME);
			bpf_timer_set_callback(timer, timer_cb_ret_bad);
			bpf_timer_start(timer, 1000, 0);
		}
	}

	0
}
