/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_STARTUP_MAP_BINDINGS_H
#define LUPOS_STARTUP_MAP_BINDINGS_H
#ifdef __BINDGEN__
/* Bindgen omits hidden declarations; this changes visibility, not their ABI. */
#pragma GCC visibility push(default)
#endif
/* Keep map_kernel.c's header environment, including its CPU feature predicate. */
#include <linux/init.h>
#include <linux/linkage.h>
#include <linux/types.h>
#include <linux/kernel.h>
#include <linux/pgtable.h>
#include <asm/init.h>
#include <asm/sections.h>
#include <asm/setup.h>
#include <asm/sev.h>

extern pmd_t early_dynamic_pgts[EARLY_DYNAMIC_PAGE_TABLES][PTRS_PER_PMD];
extern unsigned int next_early_pgt;

/* Native declarations/layouts and constants only; no C runtime implementation. */
enum {
	LUPOS_STARTUP_MAP_START_KERNEL_MAP = __START_KERNEL_map,
	LUPOS_STARTUP_MAP_CR4_LA57 = X86_CR4_LA57,
	LUPOS_STARTUP_MAP_PMD_SHIFT = PMD_SHIFT,
	LUPOS_STARTUP_MAP_PMD_SIZE = PMD_SIZE,
	LUPOS_STARTUP_MAP_PMD_MASK = PMD_MASK,
	LUPOS_STARTUP_MAP_PUD_SHIFT = PUD_SHIFT,
	LUPOS_STARTUP_MAP_P4D_SHIFT = P4D_SHIFT,
	LUPOS_STARTUP_MAP_PTRS_PER_PMD = PTRS_PER_PMD,
	LUPOS_STARTUP_MAP_PTRS_PER_PUD = PTRS_PER_PUD,
	LUPOS_STARTUP_MAP_PTRS_PER_PGD = PTRS_PER_PGD,
	LUPOS_STARTUP_MAP_MAX_PTRS_PER_P4D = MAX_PTRS_PER_P4D,
	LUPOS_STARTUP_MAP_FIXMAP_PMD_TOP = FIXMAP_PMD_TOP,
	LUPOS_STARTUP_MAP_FIXMAP_PMD_NUM = FIXMAP_PMD_NUM,
	LUPOS_STARTUP_MAP_KERNPG_TABLE_NOENC = _KERNPG_TABLE_NOENC,
	LUPOS_STARTUP_MAP_PAGE_TABLE_NOENC = _PAGE_TABLE_NOENC,
	LUPOS_STARTUP_MAP_PAGE_KERNEL_LARGE_EXEC = __PAGE_KERNEL_LARGE_EXEC,
	LUPOS_STARTUP_MAP_PAGE_GLOBAL = _PAGE_GLOBAL,
	LUPOS_STARTUP_MAP_PAGE_PRESENT = _PAGE_PRESENT,
	LUPOS_STARTUP_MAP_FEATURE_LA57 = X86_FEATURE_LA57,
	LUPOS_STARTUP_MAP_LA57_DISABLED = !!DISABLED_MASK_BIT_SET(X86_FEATURE_LA57),
	LUPOS_STARTUP_MAP_LA57_REQUIRED = !!REQUIRED_MASK_BIT_SET(X86_FEATURE_LA57),
};

/* Evaluate the dynamic macro's two results without replacing its predicate. */
#undef pgtable_l5_enabled
#define pgtable_l5_enabled() 0
enum { LUPOS_STARTUP_MAP_MAX_PHYSMEM_L4 = MAX_PHYSMEM_BITS };
#undef pgtable_l5_enabled
#define pgtable_l5_enabled() 1
enum { LUPOS_STARTUP_MAP_MAX_PHYSMEM_L5 = MAX_PHYSMEM_BITS };
#undef pgtable_l5_enabled
#ifdef __BINDGEN__
#pragma GCC visibility pop
#endif
#endif
