// SPDX-License-Identifier: GPL-2.0
#include <linux/stringhash.h>
#include "utf8-core-bindings.h"

struct unicode_map *rust_utf8_core_alloc(void)
{
	return kzalloc_obj(struct unicode_map);
}

const struct utf8data_table *rust_utf8_core_symbol_request(void)
{
	return symbol_request(utf8_data_table);
}

void rust_utf8_core_symbol_put(void)
{
	symbol_put(utf8_data_table);
}

unsigned long rust_utf8_core_init_name_hash(const void *salt)
{
	return init_name_hash(salt);
}

unsigned long rust_utf8_core_partial_name_hash(unsigned long c,
					      unsigned long hash)
{
	return partial_name_hash(c, hash);
}

unsigned int rust_utf8_core_end_name_hash(unsigned long hash)
{
	return end_name_hash(hash);
}
