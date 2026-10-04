/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_BOOT_MAXMEM_BINDINGS_H
#define LUPOS_BOOT_MAXMEM_BINDINGS_H

/* Keep the original C helper's header and declaration unchanged. */
#include "boot_bindings.h"

#if !defined(CONFIG_X86_64) || !defined(CONFIG_SPARSEMEM)
#error "Rust compressed MAXMEM requires the native x86-64 sparsemem contract"
#endif
#if !defined(USE_EARLY_PGTABLE_L5) || defined(pgtable_l5_enabled)
#error "Rust compressed MAXMEM requires the native early-boot variable predicate"
#endif

/*
 * Constants only: evaluate the native expression at each value of its boolean
 * predicate. The real pgtable_l5_enabled() is already defined by misc.h's native
 * includes; these temporary macros affect only the two expressions below.
 * Rust reads the native __pgtable_l5_enabled declaration and selects the value.
 */
#define pgtable_l5_enabled() 0
enum { LUPOS_BOOT_MAXMEM_L4 = MAXMEM };
#undef pgtable_l5_enabled
#define pgtable_l5_enabled() 1
enum { LUPOS_BOOT_MAXMEM_L5 = MAXMEM };
#undef pgtable_l5_enabled

#endif
