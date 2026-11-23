// SPDX-License-Identifier: GPL-2.0
/*
 * Native leaves for the Rust kasprintf owner: the kmalloc_track_caller()
 * macro family with its allocation hooks, given kvasprintf()'s return
 * address as the C unit's _RET_IP_, and the vsnprintf() mismatch warning.
 * The formatting passes live in lib/kasprintf.rs.
 */
#include <linux/bug.h>
#include <linux/slab.h>

char *lupos_kasprintf_kmalloc_track_caller(size_t size, gfp_t gfp, unsigned long caller);
void lupos_kasprintf_warn_mismatch(unsigned int first, unsigned int second, const char *fmt);

char *lupos_kasprintf_kmalloc_track_caller(size_t size, gfp_t gfp, unsigned long caller)
{
	return alloc_hooks(kmalloc_node_track_caller_noprof(size, gfp, NUMA_NO_NODE, caller));
}

void lupos_kasprintf_warn_mismatch(unsigned int first, unsigned int second, const char *fmt)
{
	WARN(1, "different return values (%u and %u) from vsnprintf(\"%s\", ...)",
	     first, second, fmt);
}
