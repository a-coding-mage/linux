/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 */

#[repr(C)]
pub struct cpuinfo {
	/* Core CPU configuration */
	pub cpu_impl: [i8; 12],
	pub cpu_clock_freq: u32,
	pub mmu: bool,
	pub has_div: bool,
	pub has_mul: bool,
	pub has_mulx: bool,
	pub has_bmx: bool,
	pub has_cdx: bool,

	/* CPU caches */
	pub icache_line_size: u32,
	pub icache_size: u32,
	pub dcache_line_size: u32,
	pub dcache_size: u32,

	/* TLB */
	pub tlb_pid_num_bits: u32, /* number of bits used for the PID in TLBMISC */
	pub tlb_num_ways: u32,
	pub tlb_num_ways_log2: u32,
	pub tlb_num_entries: u32,
	pub tlb_num_lines: u32,
	pub tlb_ptr_sz: u32,

	/* Addresses */
	pub reset_addr: u32,
	pub exception_addr: u32,
	pub fast_tlb_miss_exc_addr: u32,
}

extern "C" {
	pub static mut cpuinfo: cpuinfo;

	pub fn setup_cpuinfo();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
