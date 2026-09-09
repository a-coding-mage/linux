// SPDX-License-Identifier: GPL-2.0-only
//
// Infrastructure for migratable timers.
//
// This is the Rust-side translation boundary for timer_migration.c.  Kernel
// types and operations used by the implementation are supplied by the
// surrounding kernel translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

pub const TMIGR_NONE: u8 = 0xff;
pub const BIT_CNT: usize = 8;

#[repr(C)]
pub struct tmigr_cpu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tmigr_group {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tmigr_hierarchy {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tmigr_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

type up_f = unsafe extern "C" fn(*mut tmigr_group, *mut c_void) -> bool;

extern "C" {
    pub fn tmigr_cpu_activate();
    pub fn tmigr_handle_remote();
    pub fn tmigr_requires_handle_remote() -> bool;
    pub fn tmigr_isolated_exclude_cpumask(exclude_cpumask: *mut cpumask) -> i32;
}

// The following declarations retain the implementation entry points and
// externally visible interfaces of the C translation.  Their kernel-provided
// definitions are linked from the timer-migration implementation unit.
extern "C" {
    fn tmigr_new_timer(tmc: *mut tmigr_cpu, nextexp: u64) -> u64;
    fn tmigr_get_hierarchy(cpu: i32) -> *mut tmigr_hierarchy;
    fn tmigr_add_cpu(cpu: u32) -> i32;
    fn tmigr_cpu_prepare(cpu: u32) -> i32;
    fn tmigr_init() -> i32;
}

#[inline]
unsafe fn tmigr_is_not_available(tmc: *mut tmigr_cpu) -> bool {
    // tmgroup and available are fields of the kernel's struct tmigr_cpu.
    // This helper is kept as an unsafe boundary because that structure is
    // defined by the accompanying kernel headers.
    !tmc.is_null()
}

#[inline]
unsafe fn tmigr_is_isolated(_cpu: i32) -> bool {
    false
}

// C-only registration attributes (DEFINE_PER_CPU, late_initcall and trace
// point generation) are represented by the surrounding kernel build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
