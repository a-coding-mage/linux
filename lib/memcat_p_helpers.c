// SPDX-License-Identifier: GPL-2.0
/*
 * Native leaf for the Rust __memcat_p() owner: the kmalloc_array() macro
 * family, which carries the configured allocation hooks. The merge itself
 * lives in lib/memcat_p.rs.
 */
#include <linux/slab.h>

void **lupos_memcat_p_alloc(int nr);

void **lupos_memcat_p_alloc(int nr)
{
	return kmalloc_array(nr, sizeof(void *), GFP_KERNEL);
}
