/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Intel(R) Processor Trace PMU driver for perf
 * Copyright (c) 2013-2014, Intel Corporation.
 *
 * Intel PT is specified in the Intel Architecture Instruction Set Extensions
 * Programming Reference:
 * http://software.intel.com/en-us/intel-isa-extensions
 */

/*
 * Single-entry ToPA: when this close to region boundary, switch
 * buffers to avoid losing data.
 */
pub const TOPA_PMI_MARGIN: u32 = 512;

pub const TOPA_SHIFT: u32 = 12;

pub const fn sizes(tsz: u32) -> u32 {
    1u32 << (tsz + TOPA_SHIFT)
}

#[repr(transparent)]
pub struct topa_entry {
    pub bits: u64,
}

impl topa_entry {
    pub const END_SHIFT: u32 = 0;
    pub const RSVD0_SHIFT: u32 = 1;
    pub const INTR_SHIFT: u32 = 2;
    pub const RSVD1_SHIFT: u32 = 3;
    pub const STOP_SHIFT: u32 = 4;
    pub const RSVD2_SHIFT: u32 = 5;
    pub const SIZE_SHIFT: u32 = 6;
    pub const RSVD3_SHIFT: u32 = 10;
    pub const BASE_SHIFT: u32 = 12;
    pub const RSVD4_SHIFT: u32 = 52;

    pub const END_MASK: u64 = 1 << Self::END_SHIFT;
    pub const RSVD0_MASK: u64 = 1 << Self::RSVD0_SHIFT;
    pub const INTR_MASK: u64 = 1 << Self::INTR_SHIFT;
    pub const RSVD1_MASK: u64 = 1 << Self::RSVD1_SHIFT;
    pub const STOP_MASK: u64 = 1 << Self::STOP_SHIFT;
    pub const RSVD2_MASK: u64 = 1 << Self::RSVD2_SHIFT;
    pub const SIZE_MASK: u64 = 0xf << Self::SIZE_SHIFT;
    pub const RSVD3_MASK: u64 = 0x3 << Self::RSVD3_SHIFT;
    pub const BASE_MASK: u64 = 0xffffffffff << Self::BASE_SHIFT;
    pub const RSVD4_MASK: u64 = 0xfff << Self::RSVD4_SHIFT;
}

#[repr(C)]
pub struct pt_pmu {
    pub pmu: pmu,
    pub caps: [u32; PT_CPUID_REGS_NUM * PT_CPUID_LEAVES],
    pub vmx: bool,
    pub branch_en_always_on: bool,
    pub max_nonturbo_ratio: ::core::ffi::c_ulong,
    pub tsc_art_num: u32,
    pub tsc_art_den: u32,
}

/**
 * struct pt_buffer - buffer configuration; one buffer per task_struct or
 *	cpu, depending on perf event configuration
 * @tables:	list of ToPA tables in this buffer
 * @first:	shorthand for first topa table
 * @last:	shorthand for last topa table
 * @cur:	current topa table
 * @nr_pages:	buffer size in pages
 * @cur_idx:	current output region's index within @cur table
 * @output_off:	offset within the current output region
 * @data_size:	running total of the amount of data in this buffer
 * @head:	logical write offset inside the buffer
 * @snapshot:	if this is for a snapshot/overwrite counter
 * @single:	use Single Range Output instead of ToPA
 * @wrapped:	buffer advance wrapped back to the first topa table
 * @stop_pos:	STOP topa entry index
 * @intr_pos:	INT topa entry index
 * @stop_te:	STOP topa entry pointer
 * @intr_te:	INT topa entry pointer
 * @data_pages:	array of pages from perf
 */
#[repr(C)]
pub struct pt_buffer {
    pub tables: list_head,
    pub first: *mut topa,
    pub last: *mut topa,
    pub cur: *mut topa,
    pub cur_idx: u32,
    pub output_off: usize,
    pub nr_pages: ::core::ffi::c_ulong,
    pub data_size: local_t,
    pub head: local64_t,
    pub snapshot: bool,
    pub single: bool,
    pub wrapped: bool,
    pub stop_pos: isize,
    pub intr_pos: isize,
    pub stop_te: *mut topa_entry,
    pub intr_te: *mut topa_entry,
    pub data_pages: *mut *mut ::core::ffi::c_void,
}

pub const PT_FILTERS_NUM: usize = 4;

/**
 * struct pt_filter - IP range filter configuration
 * @msr_a: range start, goes to RTIT_ADDRn_A
 * @msr_b: range end, goes to RTIT_ADDRn_B
 * @config: 4-bit field in RTIT_CTL
 */
#[repr(C)]
pub struct pt_filter {
    pub msr_a: ::core::ffi::c_ulong,
    pub msr_b: ::core::ffi::c_ulong,
    pub config: ::core::ffi::c_ulong,
}

/**
 * struct pt_filters - IP range filtering context
 * @filter: filters defined for this context
 * @nr_filters: number of defined filters in the @filter array
 */
#[repr(C)]
pub struct pt_filters {
    pub filter: [pt_filter; PT_FILTERS_NUM],
    pub nr_filters: u32,
}

/**
 * struct pt - per-cpu pt context
 * @handle: perf output handle
 * @filters: last configured filters
 * @handle_nmi: do handle PT PMI on this cpu, there's an active event
 * @vmx_on: 1 if VMX is ON on this cpu
 * @pause_allowed: PERF_EF_PAUSE is allowed to stop tracing
 * @resume_allowed: PERF_EF_RESUME is allowed to start tracing
 * @output_base: cached RTIT_OUTPUT_BASE MSR value
 * @output_mask: cached RTIT_OUTPUT_MASK MSR value
 */
#[repr(C)]
pub struct pt {
    pub handle: perf_output_handle,
    pub filters: pt_filters,
    pub handle_nmi: i32,
    pub vmx_on: i32,
    pub pause_allowed: i32,
    pub resume_allowed: i32,
    pub output_base: u64,
    pub output_mask: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
