// SPDX-License-Identifier: GPL-2.0
/*
 * x86-specific bits of KMSAN shadow implementation.
 *
 * Copyright (C) 2022 Google LLC
 * Author: Alexander Potapenko <glider@google.com>
 */

// Dependency supplied by the surrounding kernel translation:
// CPU_ENTRY_AREA_SIZE

/*
 * Addresses within the CPU entry area (including e.g. exception stacks) do not
 * have struct page entries corresponding to them, so they need separate
 * handling.
 * arch_kmsan_get_meta_or_null() (declared in the header) maps the addresses in
 * CPU entry area to addresses in cpu_entry_area_shadow/cpu_entry_area_origin.
 */
// DEFINE_PER_CPU(char[CPU_ENTRY_AREA_SIZE], cpu_entry_area_shadow);
pub static mut cpu_entry_area_shadow: [core::ffi::c_char; CPU_ENTRY_AREA_SIZE] =
    [0 as core::ffi::c_char; CPU_ENTRY_AREA_SIZE];

// DEFINE_PER_CPU(char[CPU_ENTRY_AREA_SIZE], cpu_entry_area_origin);
pub static mut cpu_entry_area_origin: [core::ffi::c_char; CPU_ENTRY_AREA_SIZE] =
    [0 as core::ffi::c_char; CPU_ENTRY_AREA_SIZE];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
