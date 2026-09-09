/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Memory layout definitions for the Hexagon architecture
 *
 * Copyright (c) 2010-2013, The Linux Foundation. All rights reserved.
 */

// Originally included <linux/const.h>.

/*
 * Have to do this for ginormous numbers, else they get printed as
 * negative numbers, which the linker no likey when you try to
 * assign it to the location counter.
 */

pub const PAGE_OFFSET: u32 = 0xc0000000u32;

/*
 * Compiling for a platform that needs a crazy physical offset
 * (like if the memory starts at 1GB and up) means we need
 * an actual PHYS_OFFSET.  Should be set up in head.S.
 */

#[cfg(CONFIG_HEXAGON_PHYS_OFFSET)]
unsafe extern "C" {
    pub static mut __phys_offset: core::ffi::c_ulong;
}

#[cfg(CONFIG_HEXAGON_PHYS_OFFSET)]
macro_rules! PHYS_OFFSET {
    () => {
        __phys_offset
    };
}

#[cfg(not(CONFIG_HEXAGON_PHYS_OFFSET))]
macro_rules! PHYS_OFFSET {
    () => {
        0
    };
}

macro_rules! PHYS_PFN_OFFSET {
    () => {
        PHYS_OFFSET!() >> PAGE_SHIFT
    };
}

macro_rules! ARCH_PFN_OFFSET {
    () => {
        PHYS_PFN_OFFSET!()
    };
}

macro_rules! TASK_SIZE {
    () => {
        PAGE_OFFSET
    };
}

/* not sure how these are used yet */
macro_rules! STACK_TOP {
    () => {
        TASK_SIZE!()
    };
}

macro_rules! STACK_TOP_MAX {
    () => {
        TASK_SIZE!()
    };
}

#[repr(i32)]
pub enum fixed_addresses {
    FIX_KMAP_BEGIN,
    FIX_KMAP_END, /* check for per-cpuism */
    __end_of_fixed_addresses,
}

macro_rules! MIN_KERNEL_SEG {
    () => {
        PAGE_OFFSET >> PGDIR_SHIFT
    };
}

unsafe extern "C" {
    pub static mut max_kernel_seg: core::ffi::c_int;
}

/*
 * Start of vmalloc virtual address space for kernel;
 * supposed to be based on the amount of physical memory available
 */

macro_rules! VMALLOC_START {
    () => {
        __va(high_memory + VMALLOC_OFFSET!())
    };
}

/* Gap between physical ram and vmalloc space for guard purposes. */
macro_rules! VMALLOC_OFFSET {
    () => {
        PAGE_SIZE
    };
}

/*
 * Create the space between VMALLOC_START and FIXADDR_TOP backwards
 * from the ... "top".
 *
 * Permanent IO mappings will live at 0xfexx_xxxx
 * Hypervisor occupies the last 16MB page at 0xffxxxxxx
 */

pub const FIXADDR_TOP: u32 = 0xfe000000;

macro_rules! FIXADDR_SIZE {
    () => {
        (__end_of_fixed_addresses as usize) << PAGE_SHIFT
    };
}

macro_rules! FIXADDR_START {
    () => {
        FIXADDR_TOP - FIXADDR_SIZE!()
    };
}

/*
 * "permanent kernel mappings", defined as long-lasting mappings of
 * high-memory page frames into the kernel address space.
 */

macro_rules! LAST_PKMAP {
    () => {
        PTRS_PER_PTE
    };
}

macro_rules! LAST_PKMAP_MASK {
    () => {
        LAST_PKMAP!() - 1
    };
}

macro_rules! PKMAP_NR {
    ($virt:expr) => {
        (($virt - PKMAP_BASE!()) >> PAGE_SHIFT)
    };
}

macro_rules! PKMAP_ADDR {
    ($nr:expr) => {
        (PKMAP_BASE!() + (($nr) << PAGE_SHIFT))
    };
}

/*
 * To the "left" of the fixed map space is the kmap space
 *
 * "Permanent Kernel Mappings"; fancy (or less fancy) PTE table
 * that looks like it's actually walked.
 * Need to check the alignment/shift usage; some archs use
 * PMD_MASK on this value
 */
macro_rules! PKMAP_BASE {
    () => {
        (FIXADDR_START!() - PAGE_SIZE * LAST_PKMAP!())
    };
}

/*
 * 2 pages of guard gap between where vmalloc area ends
 * and pkmap_base begins.
 */
macro_rules! VMALLOC_END {
    () => {
        (PKMAP_BASE!() - PAGE_SIZE * 2)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
