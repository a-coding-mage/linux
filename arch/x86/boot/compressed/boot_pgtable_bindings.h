/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_BOOT_PGTABLE_BINDINGS_H
#define LUPOS_BOOT_PGTABLE_BINDINGS_H
#ifdef __BINDGEN__
/* Expose native declarations without changing their types or definitions. */
#pragma GCC visibility push(default)
#endif
#include "misc.h"
#include <asm/processor-flags.h>

/* Macro evaluation only. The trampoline algorithm and data live in Rust. */
enum {
	LUPOS_BOOT_PGTABLE_PAGE_SIZE = PAGE_SIZE,
	LUPOS_BOOT_PGTABLE_PAGE_MASK = PAGE_MASK,
	LUPOS_BOOT_PGTABLE_TRAMPOLINE_SIZE = TRAMPOLINE_32BIT_SIZE,
	LUPOS_BOOT_PGTABLE_CODE_OFFSET = TRAMPOLINE_32BIT_CODE_OFFSET,
	LUPOS_BOOT_PGTABLE_CODE_SIZE = TRAMPOLINE_32BIT_CODE_SIZE,
	LUPOS_BOOT_PGTABLE_CR4_LA57 = X86_CR4_LA57,
	LUPOS_BOOT_PGTABLE_PAGE_TABLE_NOENC = _PAGE_TABLE_NOENC,
	LUPOS_BOOT_PGTABLE_PGD_ALLOWED_BITS = PGD_ALLOWED_BITS,
#ifndef CONFIG_DYNAMIC_PHYSICAL_MASK
	LUPOS_BOOT_PGTABLE_PHYSICAL_MASK = __PHYSICAL_MASK,
#endif
};
#ifdef __BINDGEN__
#pragma GCC visibility pop
#endif
#endif
