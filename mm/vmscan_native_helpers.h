/* SPDX-License-Identifier: GPL-2.0-only */
/* Generated from vmscan_native_primitives.def by review/generate-native.js. */
#ifndef LUPOS_VMSCAN_NATIVE_HELPERS_H
#define LUPOS_VMSCAN_NATIVE_HELPERS_H
#include "vmscan_native_types.h"
struct task_struct * rust_vs_current(void);
struct pglist_data * rust_vs_node_data(int nid);
struct list_head * rust_vs_folio_lru_ptr(struct folio *f);
unsigned long * rust_vs_folio_flags_ptr(struct folio *f);
struct list_head * rust_vs_folio_deferred_ptr(struct folio *f);
swp_entry_t rust_vs_folio_swap(struct folio *f);
struct page * rust_vs_folio_page(struct folio *f);
struct folio * rust_vs_folio_from_lru(struct list_head *l);
struct folio * rust_vs_lru_to_folio(struct list_head *l);
unsigned long rust_vs_read_ulong(const unsigned long *p);
void rust_vs_write_ulong(unsigned long *p, unsigned long v);
int rust_vs_read_int(const int *p);
void rust_vs_write_int(int *p, int v);
int rust_vs_first_online_node(void);
int rust_vs_next_online_node(int nid);
int rust_vs_first_memory_node(void);
int rust_vs_next_memory_node(int nid);
bool rust_vs_nodes_empty(const nodemask_t *m);
bool rust_vs_node_state(int nid, enum node_states state);
int rust_vs_numa_node_id(void);
unsigned int rust_vs_native_pageblock_order(void);
void rust_vs_init_list_head(struct list_head *l);
void rust_vs_list_add(struct list_head *n, struct list_head *h);
void rust_vs_list_add_tail(struct list_head *n, struct list_head *h);
void rust_vs_list_del(struct list_head *n);
bool rust_vs_list_empty(const struct list_head *h);
void rust_vs_list_move(struct list_head *n, struct list_head *h);
void rust_vs_list_splice(const struct list_head *l, struct list_head *h);
void rust_vs_list_splice_init(struct list_head *l, struct list_head *h);
void rust_vs_spin_lock(spinlock_t *l);
void rust_vs_spin_unlock(spinlock_t *l);
unsigned long rust_vs_spin_lock_irqsave(spinlock_t *l);
void rust_vs_spin_unlock_irqrestore(spinlock_t *l, unsigned long flags);
void rust_vs_xa_lock_irq(struct xarray *x);
void rust_vs_xa_unlock_irq(struct xarray *x);
bool rust_vs_test_bit(unsigned long bit, const unsigned long *p);
void rust_vs_set_bit(unsigned long bit, unsigned long *p);
void rust_vs_clear_bit(unsigned long bit, unsigned long *p);
bool rust_vs_test_and_set_bit_lock(unsigned long bit, unsigned long *p);
void rust_vs_clear_bit_unlock(unsigned long bit, unsigned long *p);
#ifdef CONFIG_LRU_GEN
void rust_vs_set_mask_bits(unsigned long *p, unsigned long mask, unsigned long bits);
#endif
#ifdef CONFIG_LRU_GEN
int rust_vs_folio_lru_refs(const struct folio *f);
#endif
void rust_vs_atomic_dec(atomic_t *p);
int rust_vs_atomic_inc_return(atomic_t *p);
int rust_vs_atomic_read(const atomic_t *p);
void rust_vs_atomic_set(atomic_t *p, int v);
int rust_vs_atomic_xchg(atomic_t *p, int v);
void rust_vs_init_wait(struct wait_queue_entry *w);
bool rust_vs_waitqueue_active(const struct wait_queue_head *w);
void rust_vs_wake_up(struct wait_queue_head *w);
void rust_vs_wake_up_all(struct wait_queue_head *w);
void rust_vs_wake_up_interruptible(struct wait_queue_head *w);
void rust_vs_wait_pfmemalloc_interruptible_timeout(struct pglist_data *p, bool (*condition)(struct pglist_data *), long timeout);
void rust_vs_wait_pfmemalloc_killable(struct pglist_data *p, bool (*condition)(struct pglist_data *));
void rust_vs_cond_resched(void);
void rust_vs_cond_resched_tasks_rcu_qs(void);
bool rust_vs_current_is_kswapd(void);
bool rust_vs_current_is_khugepaged(void);
bool rust_vs_fatal_signal_pending(struct task_struct *t);
bool rust_vs_signal_pending(struct task_struct *t);
bool rust_vs_freezing(struct task_struct *t);
void rust_vs_set_freezable(void);
unsigned int rust_vs_memalloc_noreclaim_save(void);
void rust_vs_memalloc_noreclaim_restore(unsigned int flags);
gfp_t rust_vs_current_gfp_context(gfp_t flags);
void rust_vs_delayacct_freepages_start(void);
void rust_vs_delayacct_freepages_end(void);
void rust_vs_fs_reclaim_acquire(gfp_t gfp);
void rust_vs_fs_reclaim_release(gfp_t gfp);
void rust_vs_fs_reclaim_acquire_balance(void);
void rust_vs_fs_reclaim_release_balance(void);
void rust_vs_fs_reclaim_acquire_freeze(void);
void rust_vs_fs_reclaim_release_freeze(void);
void rust_vs_psi_memstall_enter(unsigned long *flags);
void rust_vs_psi_memstall_leave(unsigned long *flags);
bool rust_vs_is_err(const void *p);
struct task_struct * rust_vs_create_kswapd(int (*fn)(void *), struct pglist_data *p, int nid);
void rust_vs_pgdat_kswapd_lock(struct pglist_data *p);
void rust_vs_pgdat_kswapd_unlock(struct pglist_data *p);
void rust_vs_error_kswapd_start(int nid, struct task_struct *p);
void rust_vs_warn_reclaim_overwrite(bool condition);
void rust_vs_warn_reclaim_null(bool condition);
void rust_vs_warn_throttle_reason(void);
void rust_vs_warn_anon_only(bool condition);
#ifdef CONFIG_MEMCG
void rust_vs_warn_softlimit_reclaim_state(bool condition);
#endif
void rust_vs_bug_remove_unlocked(bool condition);
void rust_vs_bug_remove_mapping(bool condition);
void rust_vs_bug_kswapd_boot(bool condition);
void rust_vs_bug_shrink_active(bool condition, struct folio *f);
void rust_vs_bug_activate_active(bool condition, struct folio *f);
void rust_vs_bug_keep_lru(bool condition, struct folio *f);
void rust_vs_bug_isolate_ref(bool condition, struct folio *f);
void rust_vs_bug_move_lru(bool condition, struct folio *f);
void rust_vs_prefetch_prev_lru_flags(struct folio *f, struct list_head *base);
bool rust_vs_vma_flags_test(const vma_flags_t *flags, unsigned int bit);
bool rust_vs_folio_is_file_lru(const struct folio *f);
bool rust_vs_folio_test_active(const struct folio *f);
bool rust_vs_folio_test_anon(const struct folio *f);
bool rust_vs_folio_test_dirty(const struct folio *f);
bool rust_vs_folio_test_hugetlb(const struct folio *f);
bool rust_vs_folio_test_large(const struct folio *f);
bool rust_vs_folio_test_lazyfree(const struct folio *f);
bool rust_vs_folio_test_locked(const struct folio *f);
bool rust_vs_folio_test_lru(const struct folio *f);
bool rust_vs_folio_test_mlocked(const struct folio *f);
bool rust_vs_folio_test_pmd_mappable(const struct folio *f);
bool rust_vs_folio_test_private(const struct folio *f);
bool rust_vs_folio_test_reclaim(const struct folio *f);
bool rust_vs_folio_test_referenced(const struct folio *f);
bool rust_vs_folio_test_swapbacked(const struct folio *f);
bool rust_vs_folio_test_swapcache(const struct folio *f);
bool rust_vs_folio_test_unevictable(const struct folio *f);
bool rust_vs_folio_test_workingset(const struct folio *f);
bool rust_vs_folio_test_writeback(const struct folio *f);
bool rust_vs_folio_test_clear_lru(struct folio *f);
bool rust_vs_folio_test_clear_referenced(struct folio *f);
void rust_vs_folio_set_active(struct folio *f);
void rust_vs_folio_set_lru(struct folio *f);
void rust_vs_folio_set_reclaim(struct folio *f);
void rust_vs_folio_set_referenced(struct folio *f);
void rust_vs_folio_set_workingset(struct folio *f);
void rust_vs_folio_clear_active(struct folio *f);
void rust_vs_folio_clear_reclaim(struct folio *f);
void rust_vs_folio_clear_unevictable(struct folio *f);
void rust_vs___folio_clear_lru_flags(struct folio *f);
unsigned long rust_vs_folio_nr_pages(const struct folio *f);
unsigned int rust_vs_folio_order(const struct folio *f);
int rust_vs_folio_nid(const struct folio *f);
unsigned int rust_vs_folio_zonenum(const struct folio *f);
unsigned long rust_vs_folio_pfn(const struct folio *f);
int rust_vs_folio_ref_count(const struct folio *f);
int rust_vs_folio_expected_ref_count(const struct folio *f);
bool rust_vs_folio_ref_freeze(struct folio *f, int count);
void rust_vs_folio_ref_unfreeze(struct folio *f, int count);
bool rust_vs_folio_try_get(struct folio *f);
void rust_vs_folio_get(struct folio *f);
void rust_vs_folio_put(struct folio *f);
bool rust_vs_folio_put_testzero(struct folio *f);
bool rust_vs_folio_maybe_dma_pinned(const struct folio *f);
bool rust_vs_folio_mapped(const struct folio *f);
bool rust_vs_folio_trylock(struct folio *f);
void rust_vs_folio_lock(struct folio *f);
void rust_vs_folio_unlock(struct folio *f);
void rust_vs_folio_wait_writeback(struct folio *f);
bool rust_vs_folio_contain_hwpoisoned_page(struct folio *f);
bool rust_vs_folio_evictable(struct folio *f);
bool rust_vs_folio_needs_release(struct folio *f);
bool rust_vs_folio_deferred_partially_mapped(struct folio *f);
void rust_vs_folio_unqueue_deferred_split(struct folio *f);
struct address_space * rust_vs_folio_mapping(struct folio *f);
void rust_vs_folio_batch_init(struct folio_batch *b);
unsigned int rust_vs_folio_batch_add(struct folio_batch *b, struct folio *f);
bool rust_vs_folio_free_swap(struct folio *f);
int rust_vs_folio_alloc_swap(struct folio *f);
bool rust_vs_page_has_movable_ops(const struct page *p);
int rust_vs_split_folio_to_list(struct folio *f, struct list_head *l);
bool rust_vs_thp_migration_supported(void);
void rust_vs_try_to_unmap_flush(void);
void rust_vs_try_to_unmap_flush_dirty(void);
struct swap_info_struct * rust_vs___swap_entry_to_info(swp_entry_t entry);
struct swap_cluster_info * rust_vs_swap_cluster_get_and_lock_irq(struct folio *f);
void rust_vs_swap_cluster_unlock_irq(struct swap_cluster_info *ci);
void rust_vs___memcg1_swapout(struct folio *f, struct swap_cluster_info *ci);
void rust_vs___swap_cache_del_folio(struct swap_cluster_info *ci, struct folio *f, swp_entry_t entry, void *shadow);
int rust_vs_swap_writeout(struct swap_io_ctx *ctx, struct folio *f);
void rust_vs_swap_write_submit(struct swap_io_ctx *ctx);
bool rust_vs_shmem_mapping(struct address_space *m);
bool rust_vs_dax_mapping(struct address_space *m);
bool rust_vs_mapping_exiting(struct address_space *m);
bool rust_vs_mapping_shrinkable(struct address_space *m);
bool rust_vs_mapping_writeback_may_deadlock_on_reclaim(struct address_space *m);
void rust_vs_mapping_set_error(struct address_space *m, int error);
bool rust_vs_gfp_has_io_fs(gfp_t gfp);
bool rust_vs_gfp_compaction_allowed(gfp_t gfp);
bool rust_vs_gfpflags_allow_blocking(gfp_t gfp);
unsigned int rust_vs_gfp_zone(gfp_t gfp);
unsigned long rust_vs_compact_gap(unsigned int order);
bool rust_vs_compaction_suitable(struct zone *z, int order, unsigned long mark, int highest);
void rust_vs_reset_isolation_suitable(struct pglist_data *p);
void rust_vs_wakeup_kcompactd(struct pglist_data *p, int order, int highest);
bool rust_vs_cpuset_zone_allowed(struct zone *z, gfp_t gfp);
bool rust_vs_managed_zone(struct zone *z);
int rust_vs_zone_idx(const struct zone *z);
int rust_vs_zone_to_nid(struct zone *z);
unsigned long rust_vs_min_wmark_pages(struct zone *z);
unsigned long rust_vs_high_wmark_pages(struct zone *z);
unsigned long rust_vs_promo_wmark_pages(struct zone *z);
unsigned long rust_vs_zone_page_state(struct zone *z, unsigned int item);
unsigned long rust_vs_zone_page_state_snapshot(struct zone *z, unsigned int item);
unsigned long rust_vs_sum_zone_node_page_state(int nid, unsigned int item);
bool rust_vs_zone_watermark_ok(struct zone *z, unsigned int order, unsigned long mark, int highest, unsigned int flags);
struct zoneref * rust_vs_first_zones_zonelist(struct zonelist *zl, int highest, const nodemask_t *mask);
struct zoneref * rust_vs_next_zones_zonelist(struct zoneref *zr, int highest, const nodemask_t *mask);
struct zonelist * rust_vs_node_zonelist(int nid, gfp_t gfp);
unsigned long rust_vs_node_page_state(struct pglist_data *p, unsigned int item);
unsigned long rust_vs_node_page_state_pages(struct pglist_data *p, unsigned int item);
void rust_vs_node_stat_add_folio(struct folio *f, unsigned int item);
void rust_vs_node_stat_mod_folio(struct folio *f, unsigned int item, long delta);
void rust_vs___mod_node_page_state(struct pglist_data *p, unsigned int item, long delta);
void rust_vs_mod_node_page_state(struct pglist_data *p, unsigned int item, long delta);
void rust_vs_count_vm_event(unsigned int item);
void rust_vs_count_vm_events(unsigned int item, unsigned long delta);
void rust_vs___count_vm_events(unsigned int item, unsigned long delta);
void rust_vs___count_zid_vm_events(unsigned int item, int zid, unsigned long delta);
void rust_vs_count_mthp_stat(unsigned int order, unsigned int item);
void rust_vs_set_pgdat_normal_threshold(struct pglist_data *p);
void rust_vs_set_pgdat_pressure_threshold(struct pglist_data *p);
bool rust_vs_is_active_lru(unsigned int lru);
bool rust_vs_is_file_lru(unsigned int lru);
bool rust_vs_lru_gen_enabled(void);
bool rust_vs_lru_gen_switching(void);
struct pglist_data * rust_vs_lruvec_pgdat(struct lruvec *lv);
struct mem_cgroup * rust_vs_lruvec_memcg(struct lruvec *lv);
unsigned long rust_vs_lruvec_page_state(struct lruvec *lv, unsigned int item);
unsigned long rust_vs_lruvec_page_state_monotonic(struct lruvec *lv, unsigned int item);
void rust_vs_lruvec_stat_mod_folio(struct folio *f, unsigned int item, long delta);
void rust_vs_mod_lruvec_state(struct lruvec *lv, unsigned int item, long delta);
void rust_vs_update_lru_size(struct lruvec *lv, unsigned int lru, int zid, long delta);
void rust_vs_lruvec_lock_irq(struct lruvec *lv);
void rust_vs_lruvec_unlock_irq(struct lruvec *lv);
struct lruvec * rust_vs_folio_lruvec_lock_irq(struct folio *f);
struct lruvec * rust_vs_folio_lruvec_relock_irq(struct folio *f, struct lruvec *lv);
void rust_vs_lruvec_add_folio(struct lruvec *lv, struct folio *f);
void rust_vs_lruvec_del_folio(struct lruvec *lv, struct folio *f);
void rust_vs_wake_throttle_isolated(struct pglist_data *p);
unsigned long rust_vs_get_nr_swap_pages(void);
bool rust_vs_mem_cgroup_disabled(void);
bool rust_vs_mem_cgroup_is_root(struct mem_cgroup *m);
#ifdef CONFIG_CGROUP_WRITEBACK
bool rust_vs_memory_cgroup_on_dfl(void);
#endif
bool rust_vs_mem_cgroup_online(struct mem_cgroup *m);
bool rust_vs_memcg_is_dying(struct mem_cgroup *m);
int rust_vs_mem_cgroup_swappiness(struct mem_cgroup *m);
unsigned long rust_vs_mem_cgroup_get_nr_swap_pages(struct mem_cgroup *m);
unsigned long rust_vs_mem_cgroup_get_zone_lru_size(struct lruvec *lv, unsigned int lru, int zid);
struct mem_cgroup * rust_vs_mem_cgroup_iter(struct mem_cgroup *root, struct mem_cgroup *prev, struct mem_cgroup_reclaim_cookie *cookie);
void rust_vs_mem_cgroup_iter_break(struct mem_cgroup *root, struct mem_cgroup *prev);
struct lruvec * rust_vs_mem_cgroup_lruvec(struct mem_cgroup *m, struct pglist_data *p);
void rust_vs_mem_cgroup_flush_stats_ratelimited(struct mem_cgroup *m);
void rust_vs_mem_cgroup_node_filter_allowed(struct mem_cgroup *m, nodemask_t *mask);
void rust_vs_mem_cgroup_protection(struct mem_cgroup *root, struct mem_cgroup *m, unsigned long *min, unsigned long *low, unsigned long *usage);
void rust_vs_mem_cgroup_calculate_protection(struct mem_cgroup *root, struct mem_cgroup *m);
bool rust_vs_mem_cgroup_below_min(struct mem_cgroup *root, struct mem_cgroup *m);
bool rust_vs_mem_cgroup_below_low(struct mem_cgroup *root, struct mem_cgroup *m);
bool rust_vs_mem_cgroup_swap_full(struct folio *f);
void rust_vs_mem_cgroup_uncharge_folios(struct folio_batch *batch);
void rust_vs_count_memcg_events(struct mem_cgroup *m, unsigned int item, unsigned long delta);
void rust_vs_count_memcg_folio_events(struct folio *f, unsigned int item, unsigned long delta);
void rust_vs_memcg_memory_event(struct mem_cgroup *m, unsigned int event);
unsigned long rust_vs_memcg1_soft_limit_reclaim(struct pglist_data *p, int order, gfp_t gfp, unsigned long *scanned);
void rust_vs_node_get_allowed_targets(struct pglist_data *p, nodemask_t *mask);
int rust_vs_next_demotion_node(int nid, const nodemask_t *mask);
void rust_vs_vmpressure(gfp_t gfp, int order, struct mem_cgroup *m, bool tree, unsigned long scanned, unsigned long reclaimed);
void rust_vs_vmpressure_prio(gfp_t gfp, struct mem_cgroup *m, int priority);
void rust_vs_blk_start_plug(struct blk_plug *p);
void rust_vs_blk_finish_plug(struct blk_plug *p);
unsigned int rust_vs_jiffies_to_usecs(unsigned long j);
u64 rust_vs_div64_u64(u64 n, u64 d);
u64 rust_vs_div64_u64_round_up(u64 n, u64 d);
void rust_vs_try_to_unmap(struct folio *f, unsigned int flags);
void __init rust_vs_register_vmscan_sysctl(void);
int rust_vs_match_reclaim_token(char *s, substring_t *args);
#if defined(CONFIG_SYSFS) && defined(CONFIG_NUMA)
int rust_vs_device_create_reclaim_file(struct device *d);
#endif
#if defined(CONFIG_SYSFS) && defined(CONFIG_NUMA)
void rust_vs_device_remove_reclaim_file(struct device *d);
#endif
#ifdef CONFIG_LRU_GEN
long rust_vs_atomic_long_read(const atomic_long_t *p);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_atomic_long_set(atomic_long_t *p, long v);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_bitmap_clear(unsigned long *p, unsigned int start, unsigned int nr);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_bitmap_free(const unsigned long *p);
#endif
#ifdef CONFIG_LRU_GEN
unsigned long * rust_vs_bitmap_zalloc(unsigned int n, gfp_t gfp);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_cgroup_lock(void);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_cgroup_unlock(void);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_cpus_read_lock(void);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_cpus_read_unlock(void);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_folio_activate(struct folio *f);
#endif
#ifdef CONFIG_LRU_GEN
int rust_vs_folio_lru_gen(const struct folio *f);
#endif
#ifdef CONFIG_LRU_GEN
struct mem_cgroup * rust_vs_folio_memcg(struct folio *f);
#endif
#ifdef CONFIG_LRU_GEN
struct pglist_data * rust_vs_folio_pgdat(const struct folio *f);
#endif
#ifdef CONFIG_LRU_GEN
struct mem_cgroup * rust_vs_get_mem_cgroup_from_folio(struct folio *f);
#endif
#ifdef CONFIG_LRU_GEN
struct mem_cgroup * rust_vs_get_mem_cgroup_from_mm(struct mm_struct *mm);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_get_online_mems(void);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_put_online_mems(void);
#endif
#ifdef CONFIG_LRU_GEN
u32 rust_vs_get_random_u32_below(u32 ceiling);
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_is_vm_hugetlb_page(struct vm_area_struct *v);
#endif
#ifdef CONFIG_LRU_GEN
void * rust_vs_kvmalloc(size_t size, gfp_t gfp);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_list_del_init(struct list_head *l);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_list_move_tail(struct list_head *l, struct list_head *h);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_list_splice_tail_init(struct list_head *l, struct list_head *h);
#endif
#ifdef CONFIG_LRU_GEN
int rust_vs_lru_gen_from_seq(unsigned long seq);
#endif
#ifdef CONFIG_LRU_GEN
int rust_vs_lru_hist_from_seq(unsigned long seq);
#endif
#ifdef CONFIG_LRU_GEN
int rust_vs_lru_tier_from_refs(int refs, bool workingset);
#endif
#ifdef CONFIG_LRU_GEN
unsigned long rust_vs_lru_gen_folio_seq(const struct lruvec *lv, const struct folio *f, bool reclaiming);
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_lru_gen_is_active(const struct lruvec *lv, int gen);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_lru_gen_update_size(struct lruvec *lv, struct folio *f, int old_gen, int new_gen);
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_lru_gen_add_folio(struct lruvec *lv, struct folio *f, bool reclaiming);
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_lru_gen_del_folio(struct lruvec *lv, struct folio *f, bool reclaiming);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs___update_lru_size(struct lruvec *lv, unsigned int lru, int zid, long delta);
#endif
#ifdef CONFIG_LRU_GEN
struct lruvec * rust_vs_lruvec_live_lock_irq(struct lruvec *lv);
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_mapping_unevictable(const struct address_space *m);
#endif
#ifdef CONFIG_LRU_GEN
struct mem_cgroup * rust_vs_mem_cgroup_from_task(struct task_struct *t);
#endif
#ifdef CONFIG_LRU_GEN
struct mem_cgroup * rust_vs_mem_cgroup_get_from_id(u64 id);
#endif
#ifdef CONFIG_LRU_GEN
u64 rust_vs_mem_cgroup_id(struct mem_cgroup *m);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_mem_cgroup_put(struct mem_cgroup *m);
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_mem_cgroup_tryget(struct mem_cgroup *m);
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_mm_has_notifiers(struct mm_struct *mm);
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_mmap_read_trylock(struct mm_struct *mm);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_mmap_read_unlock(struct mm_struct *mm);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_mmgrab(struct mm_struct *mm);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_mmdrop(struct mm_struct *mm);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_mutex_lock(struct mutex *m);
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_mutex_trylock(struct mutex *m);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_mutex_unlock(struct mutex *m);
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_need_resched(void);
#endif
#ifdef CONFIG_LRU_GEN
unsigned long rust_vs_pgdat_end_pfn(struct pglist_data *p);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_rcu_read_lock(void);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_rcu_read_unlock(void);
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_spin_is_contended(spinlock_t *l);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_spin_lock_irq(spinlock_t *l);
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_spin_unlock_irq(spinlock_t *l);
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_spin_trylock(spinlock_t *l);
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_vma_has_recency(const struct vm_area_struct *v);
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_vma_is_accessible(const struct vm_area_struct *v);
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_vma_is_anonymous(const struct vm_area_struct *v);
#endif
#ifdef CONFIG_MEMCG
int rust_vs_cgroup_path(struct cgroup *c, char *buf, size_t len);
#endif
#ifdef CONFIG_MEMCG
void rust_vs_mem_cgroup_update_lru_size(struct lruvec *lv, unsigned int lru, int zid, long delta);
#endif
bool rust_vs_native_numa_demotion_enabled(void);
long rust_vs_native_total_swap_pages(void);
int rust_vs_native_numa_balancing_mode(void);
int rust_vs_native_buffer_heads_over_limit(void);
int rust_vs_folio_referenced(struct folio *f, int locked, struct mem_cgroup *m, vma_flags_t *flags);
struct folio * rust_vs_alloc_migration_target(struct folio *f, unsigned long private);
int rust_vs_migrate_pages(struct list_head *l, new_folio_t alloc, free_folio_t free, unsigned long private, enum migrate_mode mode, enum migrate_reason reason, unsigned int *succeeded);
int rust_vs_unmap_poisoned_folio(struct folio *f, unsigned long pfn, bool must_kill);
int rust_vs_read_zone_type(const enum zone_type *p);
void rust_vs_write_zone_type(enum zone_type *p, int v);
void rust_vs_trace_mm_vmscan_kswapd_sleep(int nid);
void rust_vs_trace_mm_vmscan_kswapd_wake(int nid, int zid, int order);
void rust_vs_trace_mm_vmscan_balance_pgdat_begin(int nid, int order, int highest_zoneidx);
void rust_vs_trace_mm_vmscan_balance_pgdat_end(int nid, int order, int highest_zoneidx, unsigned long nr_reclaimed);
void rust_vs_trace_mm_vmscan_wakeup_kswapd(int nid, int zid, int order, gfp_t gfp_flags);
void rust_vs_trace_mm_vmscan_direct_reclaim_begin(gfp_t gfp_flags, int order, struct mem_cgroup *memcg);
#ifdef CONFIG_MEMCG
void rust_vs_trace_mm_vmscan_memcg_reclaim_begin(gfp_t gfp_flags, int order, struct mem_cgroup *memcg);
#endif
#ifdef CONFIG_MEMCG
void rust_vs_trace_mm_vmscan_memcg_softlimit_reclaim_begin(gfp_t gfp_flags, int order, struct mem_cgroup *memcg);
#endif
void rust_vs_trace_mm_vmscan_direct_reclaim_end(unsigned long nr_reclaimed, struct mem_cgroup *memcg);
#ifdef CONFIG_MEMCG
void rust_vs_trace_mm_vmscan_memcg_reclaim_end(unsigned long nr_reclaimed, struct mem_cgroup *memcg);
#endif
#ifdef CONFIG_MEMCG
void rust_vs_trace_mm_vmscan_memcg_softlimit_reclaim_end(unsigned long nr_reclaimed, struct mem_cgroup *memcg);
#endif
void rust_vs_trace_mm_vmscan_lru_isolate(int highest_zoneidx, int order, unsigned long nr_requested, unsigned long nr_scanned, unsigned long nr_skipped, unsigned long nr_taken, int lru);
void rust_vs_trace_mm_vmscan_write_folio(struct folio *folio);
void rust_vs_trace_mm_vmscan_reclaim_pages(int nid, unsigned long nr_scanned, unsigned long nr_reclaimed, struct reclaim_stat *stat);
void rust_vs_trace_mm_vmscan_lru_shrink_inactive(int nid, unsigned long nr_scanned, unsigned long nr_reclaimed, struct reclaim_stat *stat, int priority, int file);
void rust_vs_trace_mm_vmscan_lru_shrink_active(int nid, unsigned long nr_taken, unsigned long nr_active, unsigned long nr_deactivated, unsigned long nr_referenced, int priority, int file);
void rust_vs_trace_mm_vmscan_node_reclaim_begin(int nid, int order, gfp_t gfp_flags);
void rust_vs_trace_mm_vmscan_node_reclaim_end(unsigned long nr_reclaimed, struct mem_cgroup *memcg);
void rust_vs_trace_mm_vmscan_throttled(int nid, int usec_timeout, int usec_delayed, int reason);
void rust_vs_trace_mm_vmscan_kswapd_reclaim_fail(int nid, int failures);
void rust_vs_trace_mm_vmscan_kswapd_clear_hopeless(int nid, int reason);

#endif
