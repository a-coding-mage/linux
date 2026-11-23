// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Native leaf for the Rust exception-table owner: the within_module_init()
 * inline over the module's init memory types. Sorting and searching live
 * in lib/extable.rs.
 */
#include <linux/build_bug.h>
#include <linux/extable.h>
#include <linux/module.h>
#include <linux/stddef.h>

/* lib/extable.rs implements x86's relative insn/fixup/data entries. */
#ifndef ARCH_HAS_RELATIVE_EXTABLE
#error "lib/extable.rs implements relative exception table entries"
#endif
static_assert(sizeof(struct exception_table_entry) == 12);
static_assert(offsetof(struct exception_table_entry, insn) == 0);
static_assert(offsetof(struct exception_table_entry, fixup) == 4);
static_assert(offsetof(struct exception_table_entry, data) == 8);

bool lupos_extable_within_module_init(unsigned long addr, const struct module *mod);

bool lupos_extable_within_module_init(unsigned long addr, const struct module *mod)
{
	return within_module_init(addr, mod);
}
