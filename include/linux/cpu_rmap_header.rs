/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * cpu_rmap.c: CPU affinity reverse-map support
 * Copyright 2011 Solarflare Communications Inc.
 *
 * C dependencies retained as external Rust types: cpumask_types, gfp, slab,
 * and kref.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct Kref {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Cpumask {
    _private: [u8; 0],
}

pub type GfpT = u32;

#[repr(C)]
pub struct CpuRmapNear {
    pub index: u16,
    pub dist: u16,
}

/**
 * struct cpu_rmap - CPU affinity reverse-map
 * @refcount: kref for object
 * @size: Number of objects to be reverse-mapped
 * @obj: Pointer to array of object pointers
 * @near: For each CPU, the index and distance to the nearest object,
 *      based on affinity masks
 */
#[repr(C)]
pub struct CpuRmap {
    pub refcount: Kref,
    pub size: u16,
    pub obj: *mut *mut c_void,
    pub near: [CpuRmapNear; 0],
}

pub const CPU_RMAP_DIST_INF: u16 = 0xffff;

extern "C" {
    pub fn alloc_cpu_rmap(size: u32, flags: GfpT) -> *mut CpuRmap;
    pub fn cpu_rmap_get(rmap: *mut CpuRmap);
    pub fn cpu_rmap_put(rmap: *mut CpuRmap) -> i32;

    pub fn cpu_rmap_add(rmap: *mut CpuRmap, obj: *mut c_void) -> i32;
    pub fn cpu_rmap_update(
        rmap: *mut CpuRmap,
        index: u16,
        affinity: *const Cpumask,
    ) -> i32;
}

pub unsafe fn cpu_rmap_lookup_index(rmap: *mut CpuRmap, cpu: u32) -> u16 {
    (*rmap).near[cpu as usize].index
}

pub unsafe fn cpu_rmap_lookup_obj(rmap: *mut CpuRmap, cpu: u32) -> *mut c_void {
    (*rmap).obj[(*rmap).near[cpu as usize].index as usize]
}

/**
 * alloc_irq_cpu_rmap - allocate CPU affinity reverse-map for IRQs
 * @size: Number of objects to be mapped
 *
 * Must be called in process context.
 */
pub unsafe fn alloc_irq_cpu_rmap(size: u32) -> *mut CpuRmap {
    alloc_cpu_rmap(size, GFP_KERNEL)
}

pub const GFP_KERNEL: GfpT = 0;

extern "C" {
    pub fn free_irq_cpu_rmap(rmap: *mut CpuRmap);
    pub fn irq_cpu_rmap_remove(rmap: *mut CpuRmap, irq: i32) -> i32;
    pub fn irq_cpu_rmap_add(rmap: *mut CpuRmap, irq: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
