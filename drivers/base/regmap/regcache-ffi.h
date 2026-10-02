/* SPDX-License-Identifier: GPL-2.0 */
#ifndef _REGCACHE_RUST_FFI_H
#define _REGCACHE_RUST_FFI_H
#include "internal.h"

/* Only pre-existing macro/inline infrastructure crosses this boundary. */
void *rust_regcache_kmalloc(size_t bytes);
void *rust_regcache_dup_defaults(const struct reg_default *defaults, unsigned int count);
struct reg_default *rust_regcache_alloc_defaults(int count);
bool rust_regcache_warn(bool condition);
bool rust_regcache_test_bit(unsigned int bit, const unsigned long *address);
struct rb_node *rust_regcache_rb_first(const struct rb_root *root);
void rust_regcache_trace_read(struct regmap *map, unsigned int reg, unsigned int val);
void rust_regcache_trace_sync(struct regmap *map, const char *name, const char *status);
void rust_regcache_trace_drop(struct regmap *map, unsigned int min, unsigned int max);
void rust_regcache_trace_only(struct regmap *map, bool enable);
void rust_regcache_trace_bypass(struct regmap *map, bool enable);
void rust_regcache_dbg_hw(struct device *dev);
void rust_regcache_warn_defaults(struct device *dev);
void rust_regcache_err_count(struct device *dev);
void rust_regcache_err_defaults(struct device *dev);
void rust_regcache_err_type(struct device *dev, enum regcache_type type);
void rust_regcache_warn_unsorted(struct device *dev);
void rust_regcache_dbg_init(struct device *dev, const char *name);
void rust_regcache_dbg_populate(struct device *dev, const char *name);
void rust_regcache_dbg_exit(struct device *dev, const char *name);
void rust_regcache_err_read(struct device *dev, unsigned int reg, int ret);
void rust_regcache_err_sync(struct device *dev, unsigned int reg, int ret);
void rust_regcache_dbg_synced(struct device *dev, unsigned int reg, unsigned int val);
void rust_regcache_dbg_sync(struct device *dev, const char *name);
void rust_regcache_err_write(struct device *dev, unsigned int reg, unsigned int val, int ret);
void rust_regcache_dbg_region(struct device *dev, const char *name, unsigned int min, unsigned int max);
void rust_regcache_dbg_raw(struct device *dev, size_t bytes, int count, unsigned int base, unsigned int end);
void rust_regcache_err_raw(struct device *dev, unsigned int base, unsigned int end, int ret);
#endif
