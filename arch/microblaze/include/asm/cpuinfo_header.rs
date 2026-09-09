/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Generic support for queying CPU info
 *
 * Copyright (C) 2007-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2007 John Williams <jwilliams@itee.uq.edu.au>
 */

/* Translated from the C header; Linux device-tree declarations are external dependencies. */

#[repr(C)]
pub struct cpu_ver_key {
    pub s: *const ::core::ffi::c_char,
    pub k: ::core::ffi::c_uint,
}

extern "C" {
    pub static cpu_ver_lookup: cpu_ver_key;
}

#[repr(C)]
pub struct family_string_key {
    pub s: *const ::core::ffi::c_char,
    pub k: ::core::ffi::c_uint,
}

extern "C" {
    pub static family_string_lookup: family_string_key;
}

#[repr(C)]
pub struct cpuinfo {
    /* Core CPU configuration */
    pub use_instr: u32,
    pub use_mult: u32,
    pub use_fpu: u32,
    pub use_exc: u32,
    pub ver_code: u32,
    pub mmu: u32,
    pub mmu_privins: u32,
    pub endian: u32,

    /* CPU caches */
    pub use_icache: u32,
    pub icache_tagbits: u32,
    pub icache_write: u32,
    pub icache_line_length: u32,
    pub icache_size: u32,
    pub icache_base: ::core::ffi::c_ulong,
    pub icache_high: ::core::ffi::c_ulong,

    pub use_dcache: u32,
    pub dcache_tagbits: u32,
    pub dcache_write: u32,
    pub dcache_line_length: u32,
    pub dcache_size: u32,
    pub dcache_wb: u32,
    pub dcache_base: ::core::ffi::c_ulong,
    pub dcache_high: ::core::ffi::c_ulong,

    /* Bus connections */
    pub use_dopb: u32,
    pub use_iopb: u32,
    pub use_dlmb: u32,
    pub use_ilmb: u32,
    pub num_fsl: u32,

    /* CPU interrupt line info */
    pub irq_edge: u32,
    pub irq_positive: u32,

    pub area_optimised: u32,

    /* HW debug support */
    pub hw_debug: u32,
    pub num_pc_brk: u32,
    pub num_rd_brk: u32,
    pub num_wr_brk: u32,
    pub cpu_clock_freq: u32, /* store real freq of cpu */

    /* FPGA family */
    pub fpga_family_code: u32,

    /* User define */
    pub pvr_user1: u32,
    pub pvr_user2: u32,
}

extern "C" {
    pub static mut cpuinfo: cpuinfo;

    /* fwd declarations of the various CPUinfo populators */
    pub fn setup_cpuinfo();
    pub fn setup_cpuinfo_clk();

    pub fn set_cpuinfo_static(ci: *mut cpuinfo, cpu: *mut device_node);
    pub fn set_cpuinfo_pvr_full(ci: *mut cpuinfo, cpu: *mut device_node);

    pub fn of_property_read_u32(
        np: *mut device_node,
        propname: *const ::core::ffi::c_char,
        out_value: *mut u32,
    ) -> ::core::ffi::c_int;
}

/* External Linux device-tree type supplied by another translation unit. */
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

pub unsafe fn fcpu(cpu: *mut device_node, n: *mut ::core::ffi::c_char) -> ::core::ffi::c_uint {
    let mut val: u32 = 0;

    of_property_read_u32(cpu, n as *const ::core::ffi::c_char, &mut val);

    val
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
