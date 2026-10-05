// SPDX-License-Identifier: GPL-2.0-only
/* Native VFS callback tables and initcall registration. No scheduler decisions. */
#include "sched_debug_bindings.h"
#error "Lupos scheduler debug registration is source-only and not build-admitted"

#define DEBUG_RW_FOPS(name, data) \
 static int name##_open(struct inode *inode, struct file *file) \
 { return single_open(file, lupos_debug_##name##_show, data); } \
 static const struct file_operations name##_fops = { \
  .open = name##_open, .write = lupos_debug_##name##_write, \
  .read = seq_read, .llseek = seq_lseek, .release = single_release, \
 }
DEBUG_RW_FOPS(feat, NULL);
DEBUG_RW_FOPS(scaling, NULL);
DEBUG_RW_FOPS(fair_runtime, inode->i_private);
DEBUG_RW_FOPS(fair_period, inode->i_private);
#ifdef CONFIG_PREEMPT_DYNAMIC
DEBUG_RW_FOPS(dynamic, NULL);
#endif
#ifdef CONFIG_SCHED_CACHE
DEBUG_RW_FOPS(cache_enable, NULL);
#endif
#ifdef CONFIG_FAIR_GROUP_SCHED
DEBUG_RW_FOPS(cgroup, NULL);
#endif
#ifdef CONFIG_SCHED_CLASS_EXT
DEBUG_RW_FOPS(ext_runtime, inode->i_private);
DEBUG_RW_FOPS(ext_period, inode->i_private);
#endif
#undef DEBUG_RW_FOPS

static const struct file_operations verbose_fops = {
 .read = debugfs_read_file_bool,
 .write = lupos_debug_verbose_write,
 .open = simple_open,
 .llseek = default_llseek,
};
static const struct seq_operations debug_sops = {
 .start = lupos_debug_start, .next = lupos_debug_next,
 .stop = lupos_debug_stop, .show = lupos_debug_show,
};
static int debug_open(struct inode *inode, struct file *file)
{ return seq_open(file, &debug_sops); }
static const struct file_operations debug_fops = {
 .open = debug_open, .read = seq_read, .llseek = seq_lseek,
 .release = seq_release,
};
static int sd_flags_open(struct inode *inode, struct file *file)
{ return single_open(file, lupos_debug_sd_flags_show, inode->i_private); }
static const struct file_operations sd_flags_fops = {
 .open = sd_flags_open, .read = seq_read, .llseek = seq_lseek,
 .release = single_release,
};

/* Sparse, configuration-selected table is registration metadata only.
 * The Rust registration owners only select entries under the same config gates.
 */
static const struct file_operations * const debug_file_ops[] = {
 [LUPOS_DEBUG_FEATURES] = &feat_fops,
 [LUPOS_DEBUG_SCALING] = &scaling_fops,
#ifdef CONFIG_PREEMPT_DYNAMIC
 [LUPOS_DEBUG_PREEMPT] = &dynamic_fops,
#endif
#ifdef CONFIG_SCHED_CACHE
 [LUPOS_DEBUG_CACHE] = &cache_enable_fops,
#endif
#ifdef CONFIG_FAIR_GROUP_SCHED
 [LUPOS_DEBUG_CGROUP] = &cgroup_fops,
#endif
 [LUPOS_DEBUG_DUMP] = &debug_fops,
 [LUPOS_DEBUG_FAIR_RUNTIME] = &fair_runtime_fops,
 [LUPOS_DEBUG_FAIR_PERIOD] = &fair_period_fops,
#ifdef CONFIG_SCHED_CLASS_EXT
 [LUPOS_DEBUG_EXT_RUNTIME] = &ext_runtime_fops,
 [LUPOS_DEBUG_EXT_PERIOD] = &ext_period_fops,
#endif
 [LUPOS_DEBUG_SD_FLAGS] = &sd_flags_fops,
};
struct dentry *lupos_debug_create_dir(const char *name, struct dentry *parent)
{ return debugfs_create_dir(name, parent); }
void lupos_debug_create_file(const char *name, umode_t mode, struct dentry *parent,
                            void *data, enum lupos_debug_file_kind kind)
{ debugfs_create_file(name, mode, parent, data, debug_file_ops[kind]); }
void lupos_debug_create_verbose(struct dentry *parent)
{ debugfs_create_file_unsafe("verbose", 0644, parent, &sched_debug_verbose, &verbose_fops); }
void lupos_debug_create_u32(const char *name, umode_t mode, struct dentry *parent, u32 *value)
{ debugfs_create_u32(name, mode, parent, value); }
void lupos_debug_create_u64(const char *name, umode_t mode, struct dentry *parent, u64 *value)
{ debugfs_create_u64(name, mode, parent, value); }
void lupos_debug_create_ulong(const char *name, umode_t mode, struct dentry *parent, unsigned long *value)
{ debugfs_create_ulong(name, mode, parent, value); }
void lupos_debug_create_str(const char *name, umode_t mode, struct dentry *parent, char **value)
{ debugfs_create_str(name, mode, parent, value); }
void lupos_debug_remove(struct dentry *entry) { debugfs_remove(entry); }
void lupos_debug_lookup_and_remove(const char *name, struct dentry *parent)
{ debugfs_lookup_and_remove(name, parent); }
static int __init sched_init_debug(void) { return lupos_debug_init(); }
late_initcall(sched_init_debug);
