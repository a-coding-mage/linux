/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2016 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// Dependency supplied by the Linux types translation.

/**
 * struct yamon_mem_region - Represents a contiguous range of physical RAM.
 * @start:      Start physical address.
 * @size:       Maximum size of region.
 * @discard:    Length of additional memory to discard after the region.
 */
#[repr(C)]
pub struct yamon_mem_region {
    pub start: phys_addr_t,
    pub size: phys_addr_t,
    pub discard: phys_addr_t,
}

/**
 * yamon_dt_append_cmdline() - Append YAMON-provided command line to /chosen
 * @fdt: the FDT blob
 *
 * Write the YAMON-provided command line to the bootargs property of the
 * /chosen node in @fdt.
 *
 * Return: 0 on success, else -errno
 *
 * The C __init annotation is retained as source intent in this declaration.
 */
pub unsafe extern "C" fn yamon_dt_append_cmdline(
    fdt: *mut core::ffi::c_void,
) -> core::ffi::c_int;

/**
 * yamon_dt_append_memory() - Append YAMON-provided memory info to /memory
 * @fdt:       the FDT blob
 * @regions:   zero size terminated array of physical memory regions
 *
 * Generate a /memory node in @fdt based upon memory size information provided
 * by YAMON in its environment and the @regions array.
 *
 * Return: 0 on success, else -errno
 *
 * The C __init annotation is retained as source intent in this declaration.
 */
pub unsafe extern "C" fn yamon_dt_append_memory(
    fdt: *mut core::ffi::c_void,
    regions: *const yamon_mem_region,
) -> core::ffi::c_int;

/**
 * yamon_dt_serial_config() - Append YAMON-provided serial config to /chosen
 * @fdt: the FDT blob
 *
 * Generate a stdout-path property in the /chosen node of @fdt, based upon
 * information provided in the YAMON environment about the UART configuration
 * of the system.
 *
 * Return: 0 on success, else -errno
 *
 * The C __init annotation is retained as source intent in this declaration.
 */
pub unsafe extern "C" fn yamon_dt_serial_config(
    fdt: *mut core::ffi::c_void,
) -> core::ffi::c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
