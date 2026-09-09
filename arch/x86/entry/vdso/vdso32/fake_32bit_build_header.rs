/* SPDX-License-Identifier: GPL-2.0 */

// Equivalent of: #ifdef CONFIG_X86_64
//
// In case of a 32 bit VDSO for a 64 bit kernel, fake a 32 bit kernel
// configuration.
//
// The following C configuration symbols are undefined in this configuration:
// CONFIG_64BIT, CONFIG_X86_64, CONFIG_COMPAT, CONFIG_PGTABLE_LEVELS,
// CONFIG_ILLEGAL_POINTER_VALUE, CONFIG_SPARSEMEM_VMEMMAP,
// CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP, CONFIG_NR_CPUS, CONFIG_PARAVIRT_XXL.

#[cfg(config_x86_64)]
pub const CONFIG_X86_32: i32 = 1;

#[cfg(config_x86_64)]
pub const CONFIG_PGTABLE_LEVELS: i32 = 2;

#[cfg(config_x86_64)]
pub const CONFIG_PAGE_OFFSET: i32 = 0;

#[cfg(config_x86_64)]
pub const CONFIG_ILLEGAL_POINTER_VALUE: i32 = 0;

#[cfg(config_x86_64)]
pub const CONFIG_NR_CPUS: i32 = 1;

#[cfg(config_x86_64)]
pub const BUILD_VDSO32_64: bool = true;

// Equivalent of: #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
