// SPDX-License-Identifier: GPL-2.0
#include "slub_bindings.h"
/* Only native macro/compiler ABI leaves and static registration storage. */
#ifdef CONFIG_SLUB_DEBUG
#ifdef CONFIG_SLUB_DEBUG_ON
DEFINE_STATIC_KEY_TRUE(slub_debug_enabled);
#else
DEFINE_STATIC_KEY_FALSE(slub_debug_enabled);
#endif
DEFINE_SPINLOCK(object_map_lock);
static DEFINE_RATELIMIT_STATE(slub_oom_rs, DEFAULT_RATELIMIT_INTERVAL, DEFAULT_RATELIMIT_BURST);
#endif
#ifdef CONFIG_NUMA
DEFINE_STATIC_KEY_FALSE(strict_numa);
#endif
#ifdef CONFIG_MEM_ALLOC_PROFILING
DEFINE_STATIC_KEY_MAYBE(CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT,
                       slab_obj_ext_has_codetag_key);
#endif
DEFINE_MUTEX(flush_lock);
DEFINE_PER_CPU(struct slub_flush_work, slub_flush);
extern void deferred_percpu_work_fn(struct irq_work *);
DEFINE_PER_CPU(struct deferred_percpu_work, deferred_percpu_work) = {
 .objects = LLIST_HEAD_INIT(objects),
 .objects_by_rcu = LLIST_HEAD_INIT(objects_by_rcu),
 .rcu_sheaves = LLIST_HEAD_INIT(rcu_sheaves),
 .work = IRQ_WORK_INIT(deferred_percpu_work_fn),
};
#ifdef CONFIG_SLAB_FREELIST_RANDOM
DEFINE_PER_CPU(struct rnd_state, slab_rnd_state);
#endif
#include "slub_allocation_exports.c"
#define RSL_LEAF(ret, name, params, ...) ret rust_slub_##name params { __VA_ARGS__ }
#include "slub_leaves.inc"
#include "slub_allocation_leaves.inc"
#undef RSL_LEAF
#ifdef CONFIG_SLUB_DEBUG
extern int setup_slub_debug(const char *, const struct kernel_param *);
static const struct kernel_param_ops param_ops_slab_debug __initconst = {
 .flags = KERNEL_PARAM_OPS_FL_NOARG, .set = setup_slub_debug,
};
__core_param_cb(slab_debug, &param_ops_slab_debug, NULL, 0);
__core_param_cb(slub_debug, &param_ops_slab_debug, NULL, 0);
#endif
