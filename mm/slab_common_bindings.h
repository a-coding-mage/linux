/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SLAB_COMMON_BINDINGS_H
#define LUPOS_SLAB_COMMON_BINDINGS_H
/* Configured native headers, not hand-copied Rust layouts, are ABI authority. */
#include <linux/slab.h>
#include <linux/mm.h>
#include <linux/poison.h>
#include <linux/interrupt.h>
#include <linux/memory.h>
#include <linux/cache.h>
#include <linux/compiler.h>
#include <linux/kfence.h>
#include <linux/module.h>
#include <linux/cpu.h>
#include <linux/uaccess.h>
#include <linux/seq_file.h>
#include <linux/dma-mapping.h>
#include <linux/swiotlb.h>
#include <linux/proc_fs.h>
#include <linux/debugfs.h>
#include <linux/kmemleak.h>
#include <linux/kasan.h>
#include <asm/cacheflush.h>
#include <asm/tlbflush.h>
#include <asm/page.h>
#include <linux/memcontrol.h>
#include <linux/stackdepot.h>
#include <linux/hrtimer.h>
#include <linux/shrinker.h>
#include <linux/workqueue.h>
#include <linux/llist.h>
#include <linux/random.h>
#include <trace/events/rcu.h>
#include "../kernel/rcu/rcu.h"
#include "internal.h"
#include "slab.h"
#include "slab_common_rcu_types.h"

#if defined(CONFIG_CC_HAS_SANE_FUNCTION_ALIGNMENT) || (CONFIG_FUNCTION_ALIGNMENT == 0)
static const bool RSC_CFG_INIT_COLD = true;
#endif

#define RSC_CONST(x) RSC_##x = (x)
enum {
 RSC_CONST(PAGE_SIZE), RSC_CONST(BITS_PER_LONG), RSC_CONST(KMALLOC_MAX_SIZE),
 RSC_CONST(KMALLOC_MAX_CACHE_SIZE), RSC_CONST(KMALLOC_SHIFT_LOW),
 RSC_CONST(KMALLOC_SHIFT_HIGH), RSC_CONST(KMALLOC_MIN_SIZE),
 RSC_CONST(KMALLOC_PARTITION_CACHES_NR), RSC_CONST(ARCH_KMALLOC_MINALIGN),
 RSC_CONST(EINVAL), RSC_CONST(ENOMEM),
 RSC_SLAB_SUPPORTS_SYSFS = __is_defined(SLAB_SUPPORTS_SYSFS),
 RSC_SIZEOF_CACHE = sizeof(struct kmem_cache),
 RSC_ALIGNOF_CACHE = __alignof__(struct kmem_cache),
 RSC_SIZEOF_ARGS = sizeof(struct kmem_cache_args),
 RSC_SIZEOF_KMALLOC_INFO = sizeof(struct kmalloc_info_struct),
};
#undef RSC_CONST
/* These compile-time constants are materialized by bindgen, never declared as
 * linker-provided pseudo-statics. Their native typedefs preserve integer width. */
#define RSC_FLAG(x) static const slab_flags_t RSC_##x = (x)
RSC_FLAG(SLAB_DEBUG_FLAGS); RSC_FLAG(SLAB_TYPESAFE_BY_RCU);
RSC_FLAG(SLAB_NOLEAKTRACE); RSC_FLAG(SLAB_FAILSLAB); RSC_FLAG(SLAB_NO_MERGE);
RSC_FLAG(SLAB_OBJ_EXT_IN_OBJ); RSC_FLAG(SLAB_RECLAIM_ACCOUNT);
RSC_FLAG(SLAB_CACHE_DMA); RSC_FLAG(SLAB_CACHE_DMA32); RSC_FLAG(SLAB_ACCOUNT);
RSC_FLAG(SLAB_MAY_ACCOUNT); RSC_FLAG(SLAB_HWCACHE_ALIGN); RSC_FLAG(SLAB_STORE_USER);
RSC_FLAG(SLAB_FLAGS_PERMITTED); RSC_FLAG(SLAB_PANIC); RSC_FLAG(SLAB_KMALLOC);
RSC_FLAG(SLAB_NO_OBJ_EXT);
#undef RSC_FLAG
#define RSC_GFP(x) static const gfp_t RSC_##x = (x)
RSC_GFP(GFP_KERNEL); RSC_GFP(GFP_NOWAIT); RSC_GFP(__GFP_ZERO);
RSC_GFP(GFP_SLAB_BUG_MASK);
#undef RSC_GFP
#ifdef CONFIG_SLUB_DEBUG
extern const struct seq_operations slabinfo_op;
extern const struct proc_ops slabinfo_proc_ops;
#endif
#define RSC_LEAF(ret, name, params, ...) ret name params;
#include "slab_common_leaves.inc"
#include "slab_common_rcu_leaves.inc"
#undef RSC_LEAF
#endif
