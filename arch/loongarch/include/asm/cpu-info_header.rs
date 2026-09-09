/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

use core::ffi::c_char;

/* cache_desc->flags */
pub const CACHE_PRESENT: u32 = 1 << 0;
pub const CACHE_PRIVATE: u32 = 1 << 1; /* core private cache */
pub const CACHE_INCLUSIVE: u32 = 1 << 2; /* include the inner level caches */

/* Descriptor for a cache */
#[repr(C)]
pub struct cache_desc {
    pub type_: u8,
    pub level: u8,
    pub sets: u16, /* Number of lines per set */
    pub ways: u8, /* Number of ways */
    pub linesz: u8, /* Size of line in bytes */
    pub flags: u8, /* Flags describing cache properties */
}

pub const CACHE_LEVEL_MAX: u32 = 3;
pub const CACHE_LEAVES_MAX: usize = 6;

#[repr(C)]
pub struct cpuinfo_loongarch {
    pub asid_cache: u64,
    pub asid_mask: usize,

    /* Capability and feature descriptor structure for LoongArch CPU */
    pub options: u64,
    pub processor_id: u32,
    pub fpu_vers: u32,
    pub fpu_csr0: u32,
    pub fpu_mask: u32,
    pub cputype: u32,
    pub isa_level: i32,
    pub tlbsize: i32,
    pub tlbsizemtlb: i32,
    pub tlbsizestlbsets: i32,
    pub tlbsizestlbways: i32,
    pub cache_leaves_present: i32, /* number of cache_leaves[] elements */
    pub cache_leaves: [cache_desc; CACHE_LEAVES_MAX],
    pub core: i32, /* physical core number in package */
    pub package: i32, /* physical package number */
    pub global_id: i32, /* physical global thread number */
    pub vabits: i32, /* Virtual Address size in bits */
    pub pabits: i32, /* Physical Address size in bits */
    pub timerbits: i32, /* Width of arch timer in bits */
    pub ksave_mask: u32, /* Usable KSave mask. */
    pub watch_dreg_count: u32, /* Number data breakpoints */
    pub watch_ireg_count: u32, /* Number instruction breakpoints */
    pub watch_reg_use_cnt: u32, /* min(NUM_WATCH_REGS, watch_dreg_count + watch_ireg_count), Usable by ptrace */
}

/* C __aligned(SMP_CACHE_BYTES) is supplied by the target configuration. */
extern "C" {
    pub static mut cpu_data: [cpuinfo_loongarch; 0];
    pub fn smp_processor_id() -> usize;
    pub fn raw_smp_processor_id() -> usize;
    pub fn cpu_probe();
    pub static __cpu_family: [*const c_char; 0];
    pub static __cpu_full_name: [*const c_char; 0];
}

#[inline]
pub unsafe fn boot_cpu_data() -> &'static mut cpuinfo_loongarch {
    &mut cpu_data[0]
}

#[inline]
pub unsafe fn current_cpu_data() -> &'static mut cpuinfo_loongarch {
    &mut *cpu_data.as_mut_ptr().add(smp_processor_id())
}

#[inline]
pub unsafe fn raw_current_cpu_data() -> &'static mut cpuinfo_loongarch {
    &mut *cpu_data.as_mut_ptr().add(raw_smp_processor_id())
}

#[inline]
pub unsafe fn cpu_family_string() -> *const c_char {
    __cpu_family[raw_smp_processor_id()]
}

#[inline]
pub unsafe fn cpu_full_name_string() -> *const c_char {
    __cpu_full_name[raw_smp_processor_id()]
}

#[inline]
pub unsafe fn cpus_are_siblings(cpua: i32, cpub: i32) -> bool {
    let infoa = &*cpu_data.as_ptr().add(cpua as usize);
    let infob = &*cpu_data.as_ptr().add(cpub as usize);
    if infoa.package != infob.package {
        return false;
    }
    if infoa.core != infob.core {
        return false;
    }
    true
}

#[inline]
pub unsafe fn cpu_asid_mask(cpuinfo: *mut cpuinfo_loongarch) -> usize {
    (*cpuinfo).asid_mask
}

#[inline]
pub unsafe fn set_cpu_asid_mask(cpuinfo: *mut cpuinfo_loongarch, asid_mask: usize) {
    (*cpuinfo).asid_mask = asid_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
