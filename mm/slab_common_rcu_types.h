/* SPDX-License-Identifier: GPL-2.0 */
/* Native layouts from mm/slab_common.c, commit e1d84f501551943a11f4c5271e9f5c85d7e15168. */
#ifndef RUST_SLAB_COMMON_RCU_TYPES_H
#define RUST_SLAB_COMMON_RCU_TYPES_H

enum {
	RSC_RCU_SLAB_FREE_DEFAULT = SLAB_FREE_DEFAULT,
	RSC_RCU_SLAB_FREE_NOLOCK = SLAB_FREE_NOLOCK,
};

#ifdef CONFIG_KVFREE_RCU_BATCHED
#define KFREE_DRAIN_JIFFIES (5 * HZ)
#define KFREE_N_BATCHES 2
#define FREE_N_CHANNELS 2

struct kvfree_rcu_bulk_data {
	struct list_head list;
	struct rcu_gp_seq gp_snap;
	unsigned long nr_records;
	void *records[] __counted_by(nr_records);
};

#define KVFREE_BULK_MAX_ENTR \
	((PAGE_SIZE - sizeof(struct kvfree_rcu_bulk_data)) / sizeof(void *))

struct kfree_rcu_cpu_work {
	struct rcu_work rcu_work;
	struct kvfree_rcu_head *head_free;
	struct rcu_gp_seq head_free_gp_snap;
	struct list_head bulk_head_free[FREE_N_CHANNELS];
	struct kfree_rcu_cpu *krcp;
};

struct kfree_rcu_cpu {
	// Objects queued on a linked list
	// through their rcu_head structures.
	struct kvfree_rcu_head *head;
	unsigned long head_gp_snap;
	atomic_t head_count;

	// Objects queued on a bulk-list.
	struct list_head bulk_head[FREE_N_CHANNELS];
	atomic_t bulk_count[FREE_N_CHANNELS];

	struct kfree_rcu_cpu_work krw_arr[KFREE_N_BATCHES];
	raw_spinlock_t lock;
	struct delayed_work monitor_work;
	bool initialized;

	struct delayed_work page_cache_work;
	atomic_t backoff_page_cache_fill;
	atomic_t work_in_progress;
	struct hrtimer hrtimer;

	struct llist_head bkvcache;
	int nr_bkv_objs;
};

enum {
	RSC_RCU_KFREE_DRAIN_JIFFIES = KFREE_DRAIN_JIFFIES,
	RSC_RCU_KFREE_N_BATCHES = KFREE_N_BATCHES,
	RSC_RCU_FREE_N_CHANNELS = FREE_N_CHANNELS,
	RSC_RCU_KVFREE_BULK_MAX_ENTR = KVFREE_BULK_MAX_ENTR,
	RSC_RCU_GFP_PAGE = GFP_KERNEL | __GFP_NORETRY | __GFP_NOMEMALLOC | __GFP_NOWARN,
	RSC_RCU_SCHEDULER_RUNNING = RCU_SCHEDULER_RUNNING,
	RSC_RCU_HRTIMER_NORESTART = HRTIMER_NORESTART,
	RSC_RCU_HRTIMER_MODE_REL = HRTIMER_MODE_REL,
	RSC_RCU_CLOCK_MONOTONIC = CLOCK_MONOTONIC,
	RSC_RCU_MSEC_PER_SEC = MSEC_PER_SEC,
	RSC_RCU_SHRINK_EMPTY = SHRINK_EMPTY,
	RSC_RCU_SHRINK_STOP = SHRINK_STOP,
	RSC_RCU_WQ_FLAGS = WQ_UNBOUND | WQ_MEM_RECLAIM,
};

extern int rust_slab_common_rcu_min_cached_objs;
extern int rust_slab_common_rcu_delay_page_cache_fill_msec;
#endif /* CONFIG_KVFREE_RCU_BATCHED */
#endif /* RUST_SLAB_COMMON_RCU_TYPES_H */
