// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/helper_restricted.c */

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::arch::asm;

#[repr(C)]
pub struct bpf_spin_lock {
	pub val: u32,
}

#[repr(C)]
pub struct val {
	pub cnt: i32,
	pub l: bpf_spin_lock,
}

#[repr(C)]
pub struct map_spin_lock_def {
	// __uint(type, BPF_MAP_TYPE_ARRAY);
	pub type_: u32,
	// __uint(max_entries, 1);
	pub max_entries: u32,
	// __type(key, int);
	pub key: i32,
	// __type(value, struct val);
	pub value: val,
}

// SEC(".maps")
#[no_mangle]
pub static mut map_spin_lock: map_spin_lock_def = map_spin_lock_def {
	type_: BPF_MAP_TYPE_ARRAY,
	max_entries: 1,
	key: 0,
	value: val {
		cnt: 0,
		l: bpf_spin_lock { val: 0 },
	},
};

extern "C" {
	pub static BPF_MAP_TYPE_ARRAY: u32;
	fn bpf_ktime_get_coarse_ns() -> u64;
	fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
	fn bpf_spin_lock(lock: *mut bpf_spin_lock);
}

// SEC("kprobe")
// __description("bpf_ktime_get_coarse_ns is forbidden in BPF_PROG_TYPE_KPROBE")
// __failure __msg("program of this type cannot use helper bpf_ktime_get_coarse_ns")
// __naked
#[no_mangle]
pub unsafe extern "C" fn in_bpf_prog_type_kprobe_1() {
	asm!(
		"call {bpf_ktime_get_coarse_ns}",
		"r0 = 0",
		"exit",
		bpf_ktime_get_coarse_ns = sym bpf_ktime_get_coarse_ns,
		options(noreturn)
	);
}

// SEC("tracepoint")
// __description("bpf_ktime_get_coarse_ns is forbidden in BPF_PROG_TYPE_TRACEPOINT")
// __failure __msg("program of this type cannot use helper bpf_ktime_get_coarse_ns")
// __naked
#[no_mangle]
pub unsafe extern "C" fn in_bpf_prog_type_tracepoint_1() {
	asm!(
		"call {bpf_ktime_get_coarse_ns}",
		"r0 = 0",
		"exit",
		bpf_ktime_get_coarse_ns = sym bpf_ktime_get_coarse_ns,
		options(noreturn)
	);
}

// SEC("perf_event")
// __description("bpf_ktime_get_coarse_ns is forbidden in BPF_PROG_TYPE_PERF_EVENT")
// __failure __msg("program of this type cannot use helper bpf_ktime_get_coarse_ns")
// __naked
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_type_perf_event_1() {
	asm!(
		"call {bpf_ktime_get_coarse_ns}",
		"r0 = 0",
		"exit",
		bpf_ktime_get_coarse_ns = sym bpf_ktime_get_coarse_ns,
		options(noreturn)
	);
}

// SEC("raw_tracepoint")
// __description("bpf_ktime_get_coarse_ns is forbidden in BPF_PROG_TYPE_RAW_TRACEPOINT")
// __failure __msg("program of this type cannot use helper bpf_ktime_get_coarse_ns")
// __naked
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_type_raw_tracepoint_1() {
	asm!(
		"call {bpf_ktime_get_coarse_ns}",
		"r0 = 0",
		"exit",
		bpf_ktime_get_coarse_ns = sym bpf_ktime_get_coarse_ns,
		options(noreturn)
	);
}

// SEC("kprobe")
// __description("bpf_spin_lock is forbidden in BPF_PROG_TYPE_KPROBE")
// __failure __msg("tracing progs cannot use bpf_spin_lock yet")
// __naked
#[no_mangle]
pub unsafe extern "C" fn in_bpf_prog_type_kprobe_3() {
	asm!(
		"r2 = r10",
		"r2 += -8",
		"r1 = 0",
		"*(u64*)(r2 + 0) = r1",
		"r1 = {map_spin_lock} ll",
		"call {bpf_map_lookup_elem}",
		"if r0 == 0 goto 0f",
		"r1 = r0",
		"call {bpf_spin_lock}",
		"0:",
		"exit",
		map_spin_lock = sym map_spin_lock,
		bpf_map_lookup_elem = sym bpf_map_lookup_elem,
		bpf_spin_lock = sym bpf_spin_lock,
		options(noreturn)
	);
}

// SEC("tracepoint")
// __description("bpf_spin_lock is forbidden in BPF_PROG_TYPE_TRACEPOINT")
// __failure __msg("tracing progs cannot use bpf_spin_lock yet")
// __naked
#[no_mangle]
pub unsafe extern "C" fn in_bpf_prog_type_tracepoint_3() {
	asm!(
		"r2 = r10",
		"r2 += -8",
		"r1 = 0",
		"*(u64*)(r2 + 0) = r1",
		"r1 = {map_spin_lock} ll",
		"call {bpf_map_lookup_elem}",
		"if r0 == 0 goto 0f",
		"r1 = r0",
		"call {bpf_spin_lock}",
		"0:",
		"exit",
		map_spin_lock = sym map_spin_lock,
		bpf_map_lookup_elem = sym bpf_map_lookup_elem,
		bpf_spin_lock = sym bpf_spin_lock,
		options(noreturn)
	);
}

// SEC("perf_event")
// __description("bpf_spin_lock is forbidden in BPF_PROG_TYPE_PERF_EVENT")
// __failure __msg("tracing progs cannot use bpf_spin_lock yet")
// __naked
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_type_perf_event_3() {
	asm!(
		"r2 = r10",
		"r2 += -8",
		"r1 = 0",
		"*(u64*)(r2 + 0) = r1",
		"r1 = {map_spin_lock} ll",
		"call {bpf_map_lookup_elem}",
		"if r0 == 0 goto 0f",
		"r1 = r0",
		"call {bpf_spin_lock}",
		"0:",
		"exit",
		map_spin_lock = sym map_spin_lock,
		bpf_map_lookup_elem = sym bpf_map_lookup_elem,
		bpf_spin_lock = sym bpf_spin_lock,
		options(noreturn)
	);
}

// SEC("raw_tracepoint")
// __description("bpf_spin_lock is forbidden in BPF_PROG_TYPE_RAW_TRACEPOINT")
// __failure __msg("tracing progs cannot use bpf_spin_lock yet")
// __naked
#[no_mangle]
pub unsafe extern "C" fn bpf_prog_type_raw_tracepoint_3() {
	asm!(
		"r2 = r10",
		"r2 += -8",
		"r1 = 0",
		"*(u64*)(r2 + 0) = r1",
		"r1 = {map_spin_lock} ll",
		"call {bpf_map_lookup_elem}",
		"if r0 == 0 goto 0f",
		"r1 = r0",
		"call {bpf_spin_lock}",
		"0:",
		"exit",
		map_spin_lock = sym map_spin_lock,
		bpf_map_lookup_elem = sym bpf_map_lookup_elem,
		bpf_spin_lock = sym bpf_spin_lock,
		options(noreturn)
	);
}

// SEC("license")
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
