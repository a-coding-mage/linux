// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C file:
// <vmlinux.h>, <bpf/bpf_helpers.h>, and "bpf_misc.h".

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::arch::global_asm;
use core::ffi::c_void;

type __u32 = u32;
type __u64 = u64;

#[repr(C)]
pub struct bpf_timer {
	_opaque: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
	_opaque: [u8; 0],
}

#[repr(C)]
pub struct timer_value {
	pub timer: bpf_timer,
}

// Original BPF map declaration:
// struct {
//	__uint(type, BPF_MAP_TYPE_ARRAY);
//	__uint(max_entries, 1);
//	__type(key, __u32);
//	__type(value, struct timer_value);
// } timer_map SEC(".maps");
#[repr(C)]
pub struct timer_map_def {
	pub type_: __u32,
	pub max_entries: __u32,
	pub key_size: __u32,
	pub value_size: __u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static timer_map: timer_map_def = timer_map_def {
	type_: BPF_MAP_TYPE_ARRAY,
	max_entries: 1,
	key_size: core::mem::size_of::<__u32>() as __u32,
	value_size: core::mem::size_of::<timer_value>() as __u32,
};

extern "C" {
	pub static BPF_MAP_TYPE_ARRAY: __u32;
	fn bpf_timer_set_callback(timer: *mut bpf_timer, callback_fn: *const c_void) -> i32;
	fn bpf_for_each_map_elem(
		map: *mut bpf_map,
		callback_fn: *const c_void,
		callback_ctx: *mut c_void,
		flags: __u64,
	) -> __u64;
}

// SEC("?raw_tp")
// __success __log_level(4)
// __msg("subprog 0 (stats_main_only) main insns_self 2 insns_total 2 stack 0")
// __msg("processed 2 insns")
global_asm!(
	".section ?raw_tp,\"ax\"",
	".global stats_main_only",
	".type stats_main_only,@function",
	"stats_main_only:",
	"r0 = 0",
	"exit",
);

global_asm!(
	".text",
	".type stats_chain_leaf,@function",
	"stats_chain_leaf:",
	"r0 = 0",
	"exit",
);

global_asm!(
	".text",
	".type stats_chain_parent,@function",
	"stats_chain_parent:",
	"call stats_chain_leaf",
	"exit",
);

// SEC("?raw_tp")
// __success __log_level(4)
/*
 * self: 2 + 2 + 2 = 6
 * totals: leaf 2, parent 2 + 2 = 4, main 2 + 4 = 6
 */
// __msg("subprog 0 (stats_static_chain) main insns_self 2 insns_total 6 stack 0")
// __msg("subprog {{[0-9]+}} (stats_chain_parent) static insns_self 2 insns_total 4 stack 0")
// __msg("subprog {{[0-9]+}} (stats_chain_leaf) static insns_self 2 insns_total 2 stack 0")
// __msg("processed 6 insns")
global_asm!(
	".section ?raw_tp,\"ax\"",
	".global stats_static_chain",
	".type stats_static_chain,@function",
	"stats_static_chain:",
	"call stats_chain_parent",
	"exit",
);

global_asm!(
	".text",
	".type stats_shared_leaf,@function",
	"stats_shared_leaf:",
	"r0 = 0",
	"exit",
);

global_asm!(
	".text",
	".global stats_global_root",
	".type stats_global_root,@function",
	"stats_global_root:",
	"call stats_shared_leaf",
	"exit",
);

// SEC("?raw_tp")
// __success __log_level(4)
/*
 * stats_shared_leaf is explored once under each independent root.
 * self: main 3 + leaf 4 + global 2 = 9
 * root totals: main 5 + global 4 = 9
 */
// __msg("subprog 0 (stats_shared_roots) main insns_self 3 insns_total 5 stack 0")
// __msg("subprog {{[0-9]+}} (stats_shared_leaf) static insns_self 4 insns_total 4 stack 0")
// __msg("subprog {{[0-9]+}} (stats_global_root) global insns_self 2 insns_total 4 stack 0")
// __msg("processed 9 insns")
global_asm!(
	".section ?raw_tp,\"ax\"",
	".global stats_shared_roots",
	".type stats_shared_roots,@function",
	"stats_shared_roots:",
	"call stats_shared_leaf",
	"call stats_global_root",
	"exit",
);

#[no_mangle]
pub unsafe extern "C" fn stats_async_leaf(
	_map: *mut c_void,
	_key: *mut __u32,
	_timer: *mut bpf_timer,
) -> i32 {
	0
}

#[no_mangle]
pub unsafe extern "C" fn stats_async_schedule(
	_map: *mut bpf_map,
	_key: *mut __u32,
	value: *mut timer_value,
	_ctx: *mut c_void,
) -> __u64 {
	core::arch::asm!(
		"r1 = {timer}",
		"r2 = {stats_async_leaf}",
		"call {bpf_timer_set_callback}",
		timer = in(reg) value,
		stats_async_leaf = sym stats_async_leaf,
		bpf_timer_set_callback = sym bpf_timer_set_callback,
		clobber_abi("C"),
	);
	0
}

// SEC("?raw_tp")
// __success __log_level(4)
/*
 * self: 9 + 7 + 2 = 18
 * totals: leaf 2, scheduler 7, main root 18
 */
// __msg("subprog 0 (stats_async_direct) main insns_self 9 insns_total 18 stack 0")
// __msg("subprog {{[0-9]+}} (stats_async_schedule) static insns_self 7 insns_total 7 stack 0")
// __msg("subprog {{[0-9]+}} (stats_async_leaf) static insns_self 2 insns_total 2 stack 0")
// __msg("processed 18 insns")
global_asm!(
	".section ?raw_tp,\"ax\"",
	".global stats_async_direct",
	".type stats_async_direct,@function",
	"stats_async_direct:",
	"r1 = timer_map ll",
	"r2 = stats_async_schedule",
	"r3 = 0",
	"r4 = 0",
	"call bpf_for_each_map_elem",
	"r0 = 0",
	"exit",
);

#[no_mangle]
pub unsafe extern "C" fn stats_async_nested_leaf(
	_map: *mut c_void,
	_key: *mut __u32,
	_timer: *mut bpf_timer,
) -> i32 {
	0
}

#[no_mangle]
pub unsafe extern "C" fn stats_async_outer(
	_map: *mut c_void,
	_key: *mut __u32,
	timer: *mut bpf_timer,
) -> i32 {
	core::arch::asm!(
		"r1 = {timer}",
		"r2 = {stats_async_nested_leaf}",
		"call {bpf_timer_set_callback}",
		timer = in(reg) timer,
		stats_async_nested_leaf = sym stats_async_nested_leaf,
		bpf_timer_set_callback = sym bpf_timer_set_callback,
		clobber_abi("C"),
	);
	0
}

#[no_mangle]
pub unsafe extern "C" fn stats_async_nested_schedule(
	_map: *mut bpf_map,
	_key: *mut __u32,
	value: *mut timer_value,
	_ctx: *mut c_void,
) -> __u64 {
	core::arch::asm!(
		"r1 = {timer}",
		"r2 = {stats_async_outer}",
		"call {bpf_timer_set_callback}",
		timer = in(reg) value,
		stats_async_outer = sym stats_async_outer,
		bpf_timer_set_callback = sym bpf_timer_set_callback,
		clobber_abi("C"),
	);
	0
}

// SEC("?raw_tp")
// __success __log_level(4)
/*
 * self: 9 + 7 + 7 + 2 = 25
 * totals: leaf 2, outer 7, scheduler 7, main root 25
 */
// __msg("subprog 0 (stats_async_nested) main insns_self 9 insns_total 25 stack 0")
// __msg("subprog {{[0-9]+}} (stats_async_nested_schedule) static insns_self 7 insns_total 7 stack 0")
// __msg("subprog {{[0-9]+}} (stats_async_outer) static insns_self 7 insns_total 7 stack 0")
// __msg("subprog {{[0-9]+}} (stats_async_nested_leaf) static insns_self 2 insns_total 2 stack 0")
// __msg("processed 25 insns")
global_asm!(
	".section ?raw_tp,\"ax\"",
	".global stats_async_nested",
	".type stats_async_nested,@function",
	"stats_async_nested:",
	"r1 = timer_map ll",
	"r2 = stats_async_nested_schedule",
	"r3 = 0",
	"r4 = 0",
	"call bpf_for_each_map_elem",
	"r0 = 0",
	"exit",
);

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
