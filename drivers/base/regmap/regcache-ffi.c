// SPDX-License-Identifier: GPL-2.0
/*
 * Canonical allocation, diagnostic and tracing macro/inline boundary for the
 * Rust regcache owner. No regcache algorithm or state transition lives here.
 * The original regmap.c continues to own CREATE_TRACE_POINTS and cache backend
 * implementations continue to own their own operations.
 */
#include <linux/slab.h>
#include "trace.h"
#include "regcache-ffi.h"

void *rust_regcache_kmalloc(size_t bytes)
{
	return kmalloc(bytes, GFP_KERNEL);
}

void *rust_regcache_dup_defaults(const struct reg_default *defaults, unsigned int count)
{
	return kmemdup_array(defaults, count, sizeof(*defaults), GFP_KERNEL);
}

struct reg_default *rust_regcache_alloc_defaults(int count)
{
	return kmalloc_objs(struct reg_default, count);
}

struct rb_node *rust_regcache_rb_first(const struct rb_root *root) { return rb_first(root); }
bool rust_regcache_warn(bool condition) { return WARN_ON(condition); }
bool rust_regcache_test_bit(unsigned int bit, const unsigned long *address)
{
	return test_bit(bit, address);
}
void rust_regcache_trace_read(struct regmap *map, unsigned int reg, unsigned int val)
{
	trace_regmap_reg_read_cache(map, reg, val);
}
void rust_regcache_trace_sync(struct regmap *map, const char *name, const char *status)
{
	trace_regcache_sync(map, name, status);
}
void rust_regcache_trace_drop(struct regmap *map, unsigned int min, unsigned int max)
{
	trace_regcache_drop_region(map, min, max);
}
void rust_regcache_trace_only(struct regmap *map, bool enable) { trace_regmap_cache_only(map, enable); }
void rust_regcache_trace_bypass(struct regmap *map, bool enable) { trace_regmap_cache_bypass(map, enable); }
void rust_regcache_dbg_hw(struct device *dev) { dev_dbg(dev, "No cache defaults, reading back from HW\n"); }
void rust_regcache_warn_defaults(struct device *dev) { dev_warn(dev, "No cache used with register defaults set!\n"); }
void rust_regcache_err_count(struct device *dev) { dev_err(dev, "Register defaults are set without the number!\n"); }
void rust_regcache_err_defaults(struct device *dev) { dev_err(dev, "Register defaults number are set without the reg!\n"); }
void rust_regcache_err_type(struct device *dev, enum regcache_type type) { dev_err(dev, "Could not match cache type: %d\n", type); }
void rust_regcache_warn_unsorted(struct device *dev) { dev_warn(dev, "Driver needs fixing: Unsorted reg_defaults, sorting the copy\n"); }
void rust_regcache_dbg_init(struct device *dev, const char *name) { dev_dbg(dev, "Initializing %s cache\n", name); }
void rust_regcache_dbg_populate(struct device *dev, const char *name) { dev_dbg(dev, "Populating %s cache\n", name); }
void rust_regcache_dbg_exit(struct device *dev, const char *name) { dev_dbg(dev, "Destroying %s cache\n", name); }
void rust_regcache_err_read(struct device *dev, unsigned int reg, int ret) { dev_err(dev, "Failed to read %x: %d\n", reg, ret); }
void rust_regcache_err_sync(struct device *dev, unsigned int reg, int ret) { dev_err(dev, "Unable to sync register %#x. %d\n", reg, ret); }
void rust_regcache_dbg_synced(struct device *dev, unsigned int reg, unsigned int val) { dev_dbg(dev, "Synced register %#x, value %#x\n", reg, val); }
void rust_regcache_dbg_sync(struct device *dev, const char *name) { dev_dbg(dev, "Syncing %s cache\n", name); }
void rust_regcache_err_write(struct device *dev, unsigned int reg, unsigned int val, int ret) { dev_err(dev, "Failed to write %x = %x: %d\n", reg, val, ret); }
void rust_regcache_dbg_region(struct device *dev, const char *name, unsigned int min, unsigned int max) { dev_dbg(dev, "Syncing %s cache from %#x-%#x\n", name, min, max); }
void rust_regcache_dbg_raw(struct device *dev, size_t bytes, int count, unsigned int base, unsigned int end) { dev_dbg(dev, "Writing %zu bytes for %d registers from 0x%x-0x%x\n", bytes, count, base, end); }
void rust_regcache_err_raw(struct device *dev, unsigned int base, unsigned int end, int ret) { dev_err(dev, "Unable to sync registers %#x-%#x. %d\n", base, end, ret); }
