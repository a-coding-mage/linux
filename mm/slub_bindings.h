/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SLUB_BINDINGS_H
#define LUPOS_SLUB_BINDINGS_H
/* Header ABI authority: exact configured headers and private declarations from
 * mm/slub.c at e1d84f501551943a11f4c5271e9f5c85d7e15168. No algorithm bodies. */
#include <linux/mm.h>
#include <linux/swap.h> /* mm_account_reclaimed_pages() */
#include <linux/module.h>
#include <linux/bit_spinlock.h>
#include <linux/interrupt.h>
#include <linux/swab.h>
#include <linux/bitops.h>
#include <linux/slab.h>
#include "slab.h"
#include <linux/vmalloc.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/kasan.h>
#include <linux/node.h>
#include <linux/kmsan.h>
#include <linux/cpu.h>
#include <linux/cpuset.h>
#include <linux/mempolicy.h>
#include <linux/ctype.h>
#include <linux/stackdepot.h>
#include <linux/debugobjects.h>
#include <linux/kallsyms.h>
#include <linux/kfence.h>
#include <linux/memory.h>
#include <linux/math64.h>
#include <linux/fault-inject.h>
#include <linux/kmemleak.h>
#include <linux/stacktrace.h>
#include <linux/prefetch.h>
#include <linux/memcontrol.h>
#include <linux/random.h>
#include <linux/prandom.h>
#include <kunit/test.h>
#include <kunit/test-bug.h>
#include <linux/sort.h>
#include <linux/irq_work.h>
#include <linux/kprobes.h>
#include <linux/debugfs.h>
#include <trace/events/kmem.h>

#include "internal.h"
#include "page_alloc.h"

enum slab_flags {
	SL_locked = PG_locked,
	SL_partial = PG_workingset,	/* Historical reasons for this bit */
	SL_pfmemalloc = PG_active,	/* Historical reasons for this bit */
};
struct slab_alloc_context {
	unsigned long caller_addr;
	size_t orig_size;
	unsigned int alloc_flags;
	struct list_lru *lru;
};

/* Structure holding parameters for get_partial_node_bulk() */
struct partial_bulk_context {
	gfp_t flags;
	unsigned int min_objects;
	unsigned int max_objects;
	struct list_head slabs;
};

/* Structure used to iterate over objects within a slab */
struct slab_obj_iter {
	unsigned long pos;
	void *start;
#ifdef CONFIG_SLAB_FREELIST_RANDOM
	unsigned long freelist_count;
	unsigned long page_limit;
	bool random;
#endif
};
#ifndef CONFIG_SLUB_TINY
/*
 * Minimum number of partial slabs. These will be left on the partial
 * lists even if they are empty. kmem_cache_shrink may reclaim them.
 */
#define MIN_PARTIAL 5

/*
 * Maximum number of desirable partial slabs.
 * The existence of more partial slabs makes kmem_cache_shrink
 * sort the partial list by the number of objects in use.
 */
#define MAX_PARTIAL 10
#else
#define MIN_PARTIAL 0
#define MAX_PARTIAL 0
#endif

#define DEBUG_DEFAULT_FLAGS (SLAB_CONSISTENCY_CHECKS | SLAB_RED_ZONE | \
				SLAB_POISON | SLAB_STORE_USER)

/*
 * These debug flags cannot use CMPXCHG because there might be consistency
 * issues when checking or reading debug information
 */
#define SLAB_NO_CMPXCHG (SLAB_CONSISTENCY_CHECKS | SLAB_STORE_USER | \
				SLAB_TRACE)


/*
 * Debugging flags that require metadata to be stored in the slab.  These get
 * disabled when slab_debug=O is used and a cache's min order increases with
 * metadata.
 */
#define DEBUG_METADATA_FLAGS (SLAB_RED_ZONE | SLAB_POISON | SLAB_STORE_USER)

#define OO_SHIFT	16
#define OO_MASK		((1 << OO_SHIFT) - 1)
#define MAX_OBJS_PER_PAGE	32767 /* since slab.objects is u15 */

/* Internal SLUB flags */
/* Poison object */
#define __OBJECT_POISON		__SLAB_FLAG_BIT(_SLAB_OBJECT_POISON)
/* Use cmpxchg_double */

#ifdef system_has_freelist_aba
#define __CMPXCHG_DOUBLE	__SLAB_FLAG_BIT(_SLAB_CMPXCHG_DOUBLE)
#else
#define __CMPXCHG_DOUBLE	__SLAB_FLAG_UNUSED
#endif

/*
 * Tracking user of a slab.
 */
#define TRACK_ADDRS_COUNT 16
struct track {
	unsigned long addr;	/* Called from address */
#ifdef CONFIG_STACKDEPOT
	depot_stack_handle_t handle;
#endif
	int cpu;		/* Was running on cpu */
	int pid;		/* Pid context */
	unsigned long when;	/* When did the operation occur */
};

enum track_item { TRACK_ALLOC, TRACK_FREE };
enum add_mode {
	ADD_TO_HEAD,
	ADD_TO_TAIL,
};

enum stat_item {
	ALLOC_FASTPATH,		/* Allocation from percpu sheaves */
	ALLOC_SLOWPATH,		/* Allocation from partial or new slab */
	FREE_RCU_SHEAF,		/* Free to rcu_free sheaf */
	FREE_RCU_SHEAF_FAIL,	/* Failed to free to a rcu_free sheaf */
	FREE_FASTPATH,		/* Free to percpu sheaves */
	FREE_SLOWPATH,		/* Free to a slab */
	FREE_ADD_PARTIAL,	/* Freeing moves slab to partial list */
	FREE_REMOVE_PARTIAL,	/* Freeing removes last object */
	ALLOC_SLAB,		/* New slab acquired from page allocator */
	ALLOC_NODE_MISMATCH,	/* Requested node different from cpu sheaf */
	FREE_SLAB,		/* Slab freed to the page allocator */
	ORDER_FALLBACK,		/* Number of times fallback was necessary */
	CMPXCHG_DOUBLE_FAIL,	/* Failures of slab freelist update */
	SHEAF_FLUSH,		/* Objects flushed from a sheaf */
	SHEAF_REFILL,		/* Objects refilled to a sheaf */
	SHEAF_ALLOC,		/* Allocation of an empty sheaf including oversized ones */
	SHEAF_FREE,		/* Freeing of an empty sheaf including oversized ones */
	BARN_GET,		/* Got full sheaf from barn */
	BARN_GET_FAIL,		/* Failed to get full sheaf from barn */
	BARN_PUT,		/* Put full sheaf to barn */
	BARN_PUT_FAIL,		/* Failed to put full sheaf to barn */
	SHEAF_PREFILL_FAST,	/* Sheaf prefill grabbed the spare sheaf */
	SHEAF_PREFILL_SLOW,	/* Sheaf prefill found no spare sheaf */
	SHEAF_PREFILL_OVERSIZE,	/* Allocation of oversize sheaf for prefill */
	SHEAF_RETURN_FAST,	/* Sheaf return reattached spare sheaf */
	SHEAF_RETURN_SLOW,	/* Sheaf return could not reattach spare */
	NR_SLUB_STAT_ITEMS
};

#ifdef CONFIG_SLUB_STATS
struct kmem_cache_stats {
	unsigned int stat[NR_SLUB_STAT_ITEMS];
};
#endif
#define MAX_FULL_SHEAVES	10
#define MAX_EMPTY_SHEAVES	10

struct node_barn {
	spinlock_t lock;
	struct list_head sheaves_full;
	struct list_head sheaves_empty;
	unsigned int nr_full;
	unsigned int nr_empty;
};

struct slab_sheaf {
	union {
		struct rcu_head rcu_head;
		struct list_head barn_list;
		/* only used to defer call_rcu() in unknown context */
		struct llist_node llnode;
		/* only used for prefilled sheafs */
		struct {
			unsigned int capacity;
			bool pfmemalloc;
		};
	};
	struct kmem_cache *cache;
	unsigned int size;
	int node; /* only used for rcu_sheaf */
	void *objects[];
};

struct slub_percpu_sheaves {
	local_trylock_t lock;
	struct slab_sheaf *main; /* never NULL when unlocked */
	struct slab_sheaf *spare; /* empty or full, may be NULL */
	struct slab_sheaf *rcu_free; /* for batching kfree_rcu() */
};

/*
 * The slab lists for all objects.
 */
struct kmem_cache_node {
	spinlock_t list_lock;
	unsigned long nr_partial;
	struct list_head partial;
#ifdef CONFIG_SLUB_DEBUG
	atomic_long_t nr_slabs;
	atomic_long_t total_objects;
	struct list_head full;
#endif
};
struct slub_flush_work {
	struct work_struct work;
	struct kmem_cache *s;
	bool skip;
};
#ifdef CONFIG_SLUB_RCU_DEBUG
struct rcu_delayed_free {
	struct rcu_head head;
	void *object;
};
#endif
struct deferred_percpu_work {
	struct llist_head objects;
	struct llist_head objects_by_rcu;
	struct llist_head rcu_sheaves;
	struct irq_work work;
};
#define OBJCGS_CLEAR_MASK (__GFP_DMA | __GFP_RECLAIMABLE | __GFP_ACCOUNT | __GFP_NOFAIL | __GFP_THISNODE | __GFP_COMP)
#define PCS_BATCH_MAX 32U
#define MAX_PARTIAL_TO_SCAN 10000
#define RSL_CONST(x) RSL_##x = (x)
enum {
 RSL_CONST(PAGE_SIZE), RSL_CONST(PAGE_SHIFT), RSL_CONST(BITS_PER_LONG),
 RSL_CONST(MIN_PARTIAL), RSL_CONST(MAX_PARTIAL), RSL_CONST(MAX_OBJS_PER_PAGE),
 RSL_CONST(OO_SHIFT), RSL_CONST(OO_MASK), RSL_CONST(TRACK_ADDRS_COUNT),
 RSL_CONST(MAX_FULL_SHEAVES), RSL_CONST(MAX_EMPTY_SHEAVES), RSL_CONST(PCS_BATCH_MAX),
 RSL_CONST(MAX_PARTIAL_TO_SCAN), RSL_CONST(MAX_NUMNODES),
 RSL_CONST(ALLOC_DEFAULT), RSL_CONST(KMALLOC_MAX_CACHE_SIZE), RSL_CONST(EINVAL), RSL_CONST(ENOMEM), RSL_CONST(E2BIG), RSL_CONST(EBUSY),
 RSL_CONST(PAGE_ALLOC_COSTLY_ORDER), RSL_CONST(MAX_PAGE_ORDER), RSL_CONST(INT_MAX), RSL_CONST(SLUB_RED_ACTIVE), RSL_CONST(SLUB_RED_INACTIVE), RSL_CONST(ENOSYS), RSL_CONST(ERANGE), RSL_CONST(EIO),
 RSL_CONST(KMALLOC_SHIFT_HIGH), RSL_CONST(NODE_ADDING_FIRST_MEMORY), RSL_CONST(NODE_REMOVING_LAST_MEMORY), RSL_CONST(NOTIFY_OK),
 RSL_SIZEOF_SLAB = sizeof(struct slab), RSL_ALIGNOF_SLAB = __alignof__(struct slab),
 RSL_SIZEOF_SHEAF = sizeof(struct slab_sheaf), RSL_SIZEOF_TRACK = sizeof(struct track),
 RSL_CONST(SLAB_ALLOC_DEFAULT), RSL_CONST(SLAB_ALLOC_NOLOCK), RSL_CONST(SLAB_ALLOC_NEW_SLAB),
 RSL_CONST(SLAB_ALLOC_NO_RECURSE), RSL_CONST(SLAB_ALLOC_NO_OBJ_EXT),
 RSL_CONST(SLAB_FREE_DEFAULT), RSL_CONST(SLAB_FREE_NOLOCK),
};
#undef RSL_CONST
#define RSL_FLAG(x) static const slab_flags_t RSL_##x = (x)
RSL_FLAG(SLAB_KMALLOC); RSL_FLAG(SLAB_DEBUG_FLAGS); RSL_FLAG(SLAB_RED_ZONE); RSL_FLAG(SLAB_POISON);
RSL_FLAG(SLAB_STORE_USER); RSL_FLAG(SLAB_CONSISTENCY_CHECKS); RSL_FLAG(SLAB_TRACE);
RSL_FLAG(SLAB_NO_CMPXCHG); RSL_FLAG(DEBUG_DEFAULT_FLAGS); RSL_FLAG(DEBUG_METADATA_FLAGS);
RSL_FLAG(__OBJECT_POISON); RSL_FLAG(__CMPXCHG_DOUBLE); RSL_FLAG(SLAB_ACCOUNT);
RSL_FLAG(SLAB_NO_OBJ_EXT); RSL_FLAG(SLAB_OBJ_EXT_IN_OBJ); RSL_FLAG(SLAB_NOLEAKTRACE);
RSL_FLAG(SLAB_TYPESAFE_BY_RCU); RSL_FLAG(SLAB_DEBUG_OBJECTS); RSL_FLAG(SLAB_RECLAIM_ACCOUNT);
RSL_FLAG(SLAB_KASAN); RSL_FLAG(SLAB_NO_SHEAVES); RSL_FLAG(SLAB_CACHE_DMA); RSL_FLAG(SLAB_CACHE_DMA32); RSL_FLAG(SLAB_HWCACHE_ALIGN); RSL_FLAG(SLAB_FAILSLAB); RSL_FLAG(SLAB_SKIP_KFENCE);
RSL_FLAG(SLAB_NO_USER_FLAGS);
#undef RSL_FLAG
#define RSL_GFP(x) static const gfp_t RSL_##x = (x)
RSL_GFP(GFP_KERNEL); RSL_GFP(GFP_NOWAIT); RSL_GFP(GFP_ATOMIC); RSL_GFP(GFP_RECLAIM_MASK);
RSL_GFP(__GFP_NOWARN); RSL_GFP(__GFP_NOMEMALLOC); RSL_GFP(__GFP_ACCOUNT); RSL_GFP(__GFP_ZERO);
RSL_GFP(__GFP_THISNODE); RSL_GFP(__GFP_NOFAIL); RSL_GFP(__GFP_COMP); RSL_GFP(__GFP_RECLAIMABLE);
RSL_GFP(OBJCGS_CLEAR_MASK);
RSL_GFP(GFP_SLAB_BUG_MASK); RSL_GFP(GFP_CONSTRAINT_MASK); RSL_GFP(__GFP_NORETRY); RSL_GFP(__GFP_DIRECT_RECLAIM); RSL_GFP(__GFP_RECLAIM);
RSL_GFP(__GFP_RETRY_MAYFAIL); RSL_GFP(GFP_DMA); RSL_GFP(GFP_DMA32);
#undef RSL_GFP
#ifdef CONFIG_SLAB_OBJ_EXT
static const unsigned long RSL_OBJEXTS_ALLOC_FAIL = OBJEXTS_ALLOC_FAIL;
static const unsigned long RSL_OBJEXTS_FLAGS_MASK = OBJEXTS_FLAGS_MASK;
#endif
#ifdef CONFIG_MEMCG
static const unsigned long RSL_MEMCG_DATA_OBJEXTS = MEMCG_DATA_OBJEXTS;
#endif
static const int RSL_NUMA_NO_NODE = NUMA_NO_NODE;
static const unsigned long RSL_ZERO_SIZE_PTR = (unsigned long)ZERO_SIZE_PTR;
extern struct mutex flush_lock;
#ifdef CONFIG_SLUB_DEBUG
extern spinlock_t object_map_lock;
#endif
#ifdef CONFIG_PRINTK
enum { RSL_KS_ADDRS_COUNT = KS_ADDRS_COUNT };
#endif
#include "slub_late_types.h"
#define RSL_LEAF(ret, name, params, ...) ret rust_slub_##name params;
#include "slub_leaves.inc"
#include "slub_allocation_leaves.inc"
#undef RSL_LEAF
#endif
