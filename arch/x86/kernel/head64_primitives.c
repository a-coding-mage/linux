// SPDX-License-Identifier: GPL-2.0
/* Six compiler/architecture macro boundaries. No head64 algorithm lives here. */
#include "head64_bindings.h"

/* Preserve CONFIG_PARAVIRT_XXL's PVOP/alternative machinery. */
unsigned long __init rust_head64_read_cr3(void) { return __read_cr3(); }
void __init rust_head64_write_cr3(unsigned long value) { write_cr3(value); }
/* Preserve GS addressing, SMP/UP selection, and exact per-CPU access width. */
unsigned long __init rust_head64_this_cpu_cr4(void)
{
	return this_cpu_read(cpu_tlbstate.cr4);
}
void __init rust_head64_this_cpu_write_cr4(unsigned long value)
{
	this_cpu_write(cpu_tlbstate.cr4, value);
}
/* Preserve ALTERNATIVE_2, custom clear routine clobbers and KMSAN metadata. */
void __init rust_head64_clear_page(void *page) { clear_page(page); }
/* Architecture BUG instruction, bug table, and unreachable annotation. */
void __init __noreturn rust_head64_bug(void) { BUG(); unreachable(); }
