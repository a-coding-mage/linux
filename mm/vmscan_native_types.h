/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_VMSCAN_NATIVE_TYPES_H
#define LUPOS_VMSCAN_NATIVE_TYPES_H
#include <linux/mm.h>
#include <linux/sched/mm.h>
#include <linux/module.h>
#include <linux/gfp.h>
#include <linux/kernel_stat.h>
#include <linux/swap.h>
#include <linux/pagemap.h>
#include <linux/init.h>
#include <linux/highmem.h>
#include <linux/vmpressure.h>
#include <linux/vmstat.h>
#include <linux/file.h>
#include <linux/writeback.h>
#include <linux/blk_plug.h>
#include <linux/buffer_head.h>	/* for buffer_heads_over_limit */
#include <linux/mm_inline.h>
#include <linux/backing-dev.h>
#include <linux/rmap.h>
#include <linux/topology.h>
#include <linux/cpu.h>
#include <linux/cpuset.h>
#include <linux/compaction.h>
#include <linux/notifier.h>
#include <linux/delay.h>
#include <linux/kthread.h>
#include <linux/freezer.h>
#include <linux/memcontrol.h>
#include <linux/migrate.h>
#include <linux/delayacct.h>
#include <linux/sysctl.h>
#include <linux/memory-tiers.h>
#include <linux/oom.h>
#include <linux/folio_batch.h>
#include <linux/prefetch.h>
#include <linux/printk.h>
#include <linux/dax.h>
#include <linux/psi.h>
#include <linux/pagewalk.h>
#include <linux/shmem_fs.h>
#include <linux/ctype.h>
#include <linux/debugfs.h>
#include <linux/khugepaged.h>
#include <linux/rculist_nulls.h>
#include <linux/random.h>
#include <linux/mmu_notifier.h>
#include <linux/parser.h>
#include <linux/swap_ops.h>

#include <asm/tlbflush.h>
#include <asm/div64.h>

#include <linux/swapops.h>
#include <linux/sched/sysctl.h>

#include "internal.h"
#include "page_alloc.h"
#include "swap.h"
struct scan_control {
	/* How many pages shrink_list() should reclaim */
	unsigned long nr_to_reclaim;

	/*
	 * Nodemask of nodes allowed by the caller. If NULL, all nodes
	 * are scanned.
	 */
	const nodemask_t *nodemask;

	/*
	 * The memory cgroup that hit its limit and as a result is the
	 * primary target of this reclaim invocation.
	 */
	struct mem_cgroup *target_mem_cgroup;

	/*
	 * Scan pressure balancing between anon and file LRUs
	 */
	unsigned long	anon_cost;
	unsigned long	file_cost;

	/* Swappiness value for proactive reclaim. Always use sc_swappiness()! */
	int *proactive_swappiness;

	/* Can active folios be deactivated as part of reclaim? */
#define DEACTIVATE_ANON 1
#define DEACTIVATE_FILE 2
	unsigned int may_deactivate:2;
	unsigned int force_deactivate:1;
	unsigned int skipped_deactivate:1;

	/* zone_reclaim_mode, boost reclaim */
	unsigned int may_writepage:1;

	/* zone_reclaim_mode */
	unsigned int may_unmap:1;

	/* zone_reclaim_mode, boost reclaim, cgroup restrictions */
	unsigned int may_swap:1;

	/* Not allow cache_trim_mode to be turned on as part of reclaim? */
	unsigned int no_cache_trim_mode:1;

	/* Has cache_trim_mode failed at least once? */
	unsigned int cache_trim_mode_failed:1;

	/* Proactive reclaim invoked by userspace */
	unsigned int proactive:1;

	/*
	 * Cgroup memory below memory.low is protected as long as we
	 * don't threaten to OOM. If any cgroup is reclaimed at
	 * reduced force or passed over entirely due to its memory.low
	 * setting (memcg_low_skipped), and nothing is reclaimed as a
	 * result, then go back for one more cycle that reclaims the protected
	 * memory (memcg_low_reclaim) to avert OOM.
	 */
	unsigned int memcg_low_reclaim:1;
	unsigned int memcg_low_skipped:1;

	/* Shared cgroup tree walk failed, rescan the whole tree */
	unsigned int memcg_full_walk:1;

	unsigned int hibernation_mode:1;

	/* One of the zones is ready for compaction */
	unsigned int compaction_ready:1;

	/* There is easily reclaimable cold cache in the current node */
	unsigned int cache_trim_mode:1;

	/* The file folios on the current node are dangerously low */
	unsigned int file_is_tiny:1;

	/* Always discard instead of demoting to lower tier memory */
	unsigned int no_demotion:1;

	/* Allocation order */
	s8 order;

	/* Scan (total_size >> priority) pages at once */
	s8 priority;

	/* The highest zone to isolate folios for reclaim from */
	s8 reclaim_idx;

	/* This context's GFP mask */
	gfp_t gfp_mask;

	/* Incremented by the number of inactive pages that were scanned */
	unsigned long nr_scanned;

	/* Number of pages freed so far during a call to shrink_zones() */
	unsigned long nr_reclaimed;

	struct {
		unsigned int dirty;
		unsigned int congested;
		unsigned int writeback;
		unsigned int immediate;
		unsigned int taken;
	} nr;

	/* for recording the reclaimed slab by now */
	struct reclaim_state reclaim_state;
};

#ifdef CONFIG_LRU_GEN
struct ctrl_pos {
	unsigned long refaulted;
	unsigned long total;
	int gain;
};
#endif

enum {
	MEMORY_RECLAIM_SWAPPINESS = 0,
	MEMORY_RECLAIM_SWAPPINESS_MAX,
	MEMORY_RECLAIM_NULL,
};
#endif
