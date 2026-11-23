// SPDX-License-Identifier: GPL-2.0-only
/*
 * Native leaves for the Rust bitmap owner: the kmalloc_array() and
 * kmalloc_array_node() macro families with their allocation hooks, and
 * devm_add_action_or_reset() with the release action's name. The bitmap
 * operations live in lib/bitmap.rs.
 */
#include <linux/device.h>
#include <linux/slab.h>

unsigned long *lupos_bitmap_kmalloc_array(size_t n, gfp_t flags);
unsigned long *lupos_bitmap_kmalloc_array_node(size_t n, gfp_t flags, int node);
int lupos_bitmap_devm_add_action_or_reset(struct device *dev, void (*action)(void *),
					  void *data);

unsigned long *lupos_bitmap_kmalloc_array(size_t n, gfp_t flags)
{
	return kmalloc_array(n, sizeof(unsigned long), flags);
}

unsigned long *lupos_bitmap_kmalloc_array_node(size_t n, gfp_t flags, int node)
{
	return kmalloc_array_node(n, sizeof(unsigned long), flags, node);
}

/* devm_add_action_or_reset(dev, devm_bitmap_free, bitmap). */
int lupos_bitmap_devm_add_action_or_reset(struct device *dev, void (*action)(void *),
					  void *data)
{
	return __devm_add_action_or_reset(dev, action, data, "devm_bitmap_free");
}
