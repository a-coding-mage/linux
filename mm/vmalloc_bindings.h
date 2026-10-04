/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_VMALLOC_BINDINGS_H
#define LUPOS_VMALLOC_BINDINGS_H
#include <linux/vmalloc.h>
#include <linux/mm.h>
#include <linux/module.h>
#include <linux/highmem.h>
#include <linux/sched/signal.h>
#include <linux/slab.h>
#include <linux/spinlock.h>
#include <linux/interrupt.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/set_memory.h>
#include <linux/debugobjects.h>
#include <linux/kallsyms.h>
#include <linux/list.h>
#include <linux/notifier.h>
#include <linux/rbtree.h>
#include <linux/xarray.h>
#include <linux/io.h>
#include <linux/rcupdate.h>
#include <linux/pfn.h>
#include <linux/kmemleak.h>
#include <linux/atomic.h>
#include <linux/compiler.h>
#include <linux/memcontrol.h>
#include <linux/llist.h>
#include <linux/uio.h>
#include <linux/bitops.h>
#include <linux/rbtree_augmented.h>
#include <linux/overflow.h>
#include <linux/pgtable.h>
#include <linux/hugetlb.h>
#include <linux/sched/mm.h>
#include <asm/tlbflush.h>
#include <asm/shmparam.h>
#include <linux/page_owner.h>
#include <linux/random.h>
#include <linux/cleanup.h>
#include "internal.h"
#include "pgalloc-track.h"
#include "vmalloc.h"

/* Private native layouts copied verbatim from the authoritative vmalloc.c.
 * Rust consumes bindgen output, never a handwritten approximation. */
struct vfree_deferred { struct llist_head list; struct work_struct wq; };
struct rb_list { struct rb_root root; struct list_head head; spinlock_t lock; };
#define MAX_VA_SIZE_PAGES 256
struct vmap_pool { struct list_head head; unsigned long len; };
struct vmap_node {
 struct vmap_pool pool[MAX_VA_SIZE_PAGES];
 spinlock_t pool_lock;
 bool skip_populate;
 struct rb_list busy;
 struct rb_list lazy;
 struct list_head purge_list;
 struct work_struct purge_work;
 unsigned long nr_purged;
};
#if BITS_PER_LONG == 32
#define VMALLOC_SPACE (128UL*1024*1024)
#else
#define VMALLOC_SPACE (128UL*1024*1024*1024)
#endif
#define VMALLOC_PAGES (VMALLOC_SPACE / PAGE_SIZE)
#define VMAP_MAX_ALLOC BITS_PER_LONG
#define VMAP_BBMAP_BITS_MAX 1024
#define VMAP_BBMAP_BITS_MIN (VMAP_MAX_ALLOC*2)
#define VMAP_MIN(x, y) ((x) < (y) ? (x) : (y))
#define VMAP_MAX(x, y) ((x) > (y) ? (x) : (y))
#define VMAP_BBMAP_BITS VMAP_MIN(VMAP_BBMAP_BITS_MAX, VMAP_MAX(VMAP_BBMAP_BITS_MIN, VMALLOC_PAGES / roundup_pow_of_two(NR_CPUS) / 16))
#define VMAP_BLOCK_SIZE (VMAP_BBMAP_BITS * PAGE_SIZE)
#define VMAP_PURGE_THRESHOLD (VMAP_BBMAP_BITS / 4)
#define VMAP_RAM 0x1
#define VMAP_BLOCK 0x2
#define VMAP_FLAGS_MASK 0x3
struct vmap_block_queue { spinlock_t lock; struct list_head free; struct xarray vmap_blocks; };
struct vmap_block {
 spinlock_t lock;
 struct vmap_area *va;
 unsigned long free, dirty;
 DECLARE_BITMAP(used_map, VMAP_BBMAP_BITS);
 unsigned long dirty_min, dirty_max;
 struct list_head free_list;
 struct rcu_head rcu_head;
 struct list_head purge;
 unsigned int cpu;
};
#ifdef CONFIG_VMAP_PFN
struct vmap_pfn_data { unsigned long *pfns; pgprot_t prot; unsigned int idx; };
#endif

enum {
 RVM_PAGE_SHIFT = PAGE_SHIFT,
 RVM_PAGE_SIZE = PAGE_SIZE,
 RVM_BITS_PER_LONG = BITS_PER_LONG,
 RVM_BITS_PER_BYTE = BITS_PER_BYTE,
 RVM_MAX_VA_SIZE_PAGES = MAX_VA_SIZE_PAGES,
 RVM_VMAP_MAX_ALLOC = VMAP_MAX_ALLOC,
 RVM_VMAP_BBMAP_BITS = VMAP_BBMAP_BITS,
 RVM_VMAP_BLOCK_SIZE = VMAP_BLOCK_SIZE,
 RVM_VMAP_PURGE_THRESHOLD = VMAP_PURGE_THRESHOLD,
 RVM_PGTBL_PTE_MODIFIED = PGTBL_PTE_MODIFIED,
 RVM_PGTBL_PMD_MODIFIED = PGTBL_PMD_MODIFIED,
 RVM_PGTBL_PUD_MODIFIED = PGTBL_PUD_MODIFIED,
 RVM_PGTBL_P4D_MODIFIED = PGTBL_P4D_MODIFIED,
 RVM_PGTBL_PGD_MODIFIED = PGTBL_PGD_MODIFIED,
 RVM_GFP_KERNEL = GFP_KERNEL,
 RVM_GFP_NOWAIT = GFP_NOWAIT,
 RVM_GFP_RECLAIM_MASK = GFP_RECLAIM_MASK,
 RVM_GFP_NOWARN = __GFP_NOWARN,
 RVM_NUMA_NO_NODE = NUMA_NO_NODE,
 RVM_VMAP_AREA_SIZE = sizeof(struct vmap_area),
 RVM_VMAP_AREA_ALIGN = __alignof__(struct vmap_area),
 RVM_VMAP_NODE_SIZE = sizeof(struct vmap_node),
 RVM_VMAP_NODE_ALIGN = __alignof__(struct vmap_node),
};
unsigned long rust_vmalloc_module_range_start(void);
unsigned long rust_vmalloc_module_range_end(void);
#include "vmalloc_constants.h"
#include "vmalloc_private_decls.h"
#define RVM_LEAF(ret, name, args, expression) ret rust_vmalloc_##name args;
#include "vmalloc_primitives.inc"
#undef RVM_LEAF
#endif
