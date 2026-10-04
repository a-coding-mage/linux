/* SPDX-License-Identifier: GPL-2.0-only */
/* Header/assembly primitives only; reclaim algorithms live in Rust. */
#define pr_fmt(fmt) "vmscan: " fmt
#include "vmscan_native_helpers.h"
#define CREATE_TRACE_POINTS
#include <trace/events/vmscan.h>
#include "vmscan_native_storage.inc"
#ifdef ARCH_HAS_PREFETCHW
#define prefetchw_prev_lru_folio(_folio, _base, _field)			\
	do {								\
		if ((_folio)->lru.prev != _base) {			\
			struct folio *prev;				\
									\
			prev = lru_to_folio(&(_folio->lru));		\
			prefetchw(&prev->_field);			\
		}							\
	} while (0)
#else
#define prefetchw_prev_lru_folio(_folio, _base, _field) do { } while (0)
#endif

struct task_struct * rust_vs_current(void) { return current; }
struct pglist_data * rust_vs_node_data(int nid) { return NODE_DATA(nid); }
struct list_head * rust_vs_folio_lru_ptr(struct folio *f) { return &f->lru; }
unsigned long * rust_vs_folio_flags_ptr(struct folio *f) { return &f->flags.f; }
struct list_head * rust_vs_folio_deferred_ptr(struct folio *f) { return &f->_deferred_list; }
swp_entry_t rust_vs_folio_swap(struct folio *f) { return f->swap; }
struct page * rust_vs_folio_page(struct folio *f) { return &f->page; }
struct folio * rust_vs_folio_from_lru(struct list_head *l) { return list_entry(l, struct folio, lru); }
struct folio * rust_vs_lru_to_folio(struct list_head *l) { return lru_to_folio(l); }
unsigned long rust_vs_read_ulong(const unsigned long *p) { return READ_ONCE(*p); }
void rust_vs_write_ulong(unsigned long *p, unsigned long v) { WRITE_ONCE(*p, v); }
int rust_vs_read_int(const int *p) { return READ_ONCE(*p); }
void rust_vs_write_int(int *p, int v) { WRITE_ONCE(*p, v); }
int rust_vs_first_online_node(void) { return first_node(node_states[N_ONLINE]); }
int rust_vs_next_online_node(int nid) { return next_node(nid, node_states[N_ONLINE]); }
int rust_vs_first_memory_node(void) { return first_node(node_states[N_MEMORY]); }
int rust_vs_next_memory_node(int nid) { return next_node(nid, node_states[N_MEMORY]); }
bool rust_vs_nodes_empty(const nodemask_t *m) { return nodes_empty(*m); }
bool rust_vs_node_state(int nid, enum node_states state) { return node_state(nid, state); }
int rust_vs_numa_node_id(void) { return numa_node_id(); }
unsigned int rust_vs_native_pageblock_order(void) { return pageblock_order; }
void rust_vs_init_list_head(struct list_head *l) { INIT_LIST_HEAD(l); }
void rust_vs_list_add(struct list_head *n, struct list_head *h) { list_add(n, h); }
void rust_vs_list_add_tail(struct list_head *n, struct list_head *h) { list_add_tail(n, h); }
void rust_vs_list_del(struct list_head *n) { list_del(n); }
bool rust_vs_list_empty(const struct list_head *h) { return list_empty(h); }
void rust_vs_list_move(struct list_head *n, struct list_head *h) { list_move(n, h); }
void rust_vs_list_splice(const struct list_head *l, struct list_head *h) { list_splice(l, h); }
void rust_vs_list_splice_init(struct list_head *l, struct list_head *h) { list_splice_init(l, h); }
void rust_vs_spin_lock(spinlock_t *l) { spin_lock(l); }
void rust_vs_spin_unlock(spinlock_t *l) { spin_unlock(l); }
unsigned long rust_vs_spin_lock_irqsave(spinlock_t *l) { unsigned long flags; spin_lock_irqsave(l, flags); return flags; }
void rust_vs_spin_unlock_irqrestore(spinlock_t *l, unsigned long flags) { spin_unlock_irqrestore(l, flags); }
void rust_vs_xa_lock_irq(struct xarray *x) { xa_lock_irq(x); }
void rust_vs_xa_unlock_irq(struct xarray *x) { xa_unlock_irq(x); }
bool rust_vs_test_bit(unsigned long bit, const unsigned long *p) { return test_bit(bit, p); }
void rust_vs_set_bit(unsigned long bit, unsigned long *p) { set_bit(bit, p); }
void rust_vs_clear_bit(unsigned long bit, unsigned long *p) { clear_bit(bit, p); }
bool rust_vs_test_and_set_bit_lock(unsigned long bit, unsigned long *p) { return test_and_set_bit_lock(bit, p); }
void rust_vs_clear_bit_unlock(unsigned long bit, unsigned long *p) { clear_bit_unlock(bit, p); }
#ifdef CONFIG_LRU_GEN
void rust_vs_set_mask_bits(unsigned long *p, unsigned long mask, unsigned long bits) { set_mask_bits(p, mask, bits); }
#endif
#ifdef CONFIG_LRU_GEN
int rust_vs_folio_lru_refs(const struct folio *f) { return folio_lru_refs(f); }
#endif
void rust_vs_atomic_dec(atomic_t *p) { atomic_dec(p); }
int rust_vs_atomic_inc_return(atomic_t *p) { return atomic_inc_return(p); }
int rust_vs_atomic_read(const atomic_t *p) { return atomic_read(p); }
void rust_vs_atomic_set(atomic_t *p, int v) { atomic_set(p, v); }
int rust_vs_atomic_xchg(atomic_t *p, int v) { return atomic_xchg(p, v); }
void rust_vs_init_wait(struct wait_queue_entry *w) { init_wait(w); }
bool rust_vs_waitqueue_active(const struct wait_queue_head *w) { return waitqueue_active(w); }
void rust_vs_wake_up(struct wait_queue_head *w) { wake_up(w); }
void rust_vs_wake_up_all(struct wait_queue_head *w) { wake_up_all(w); }
void rust_vs_wake_up_interruptible(struct wait_queue_head *w) { wake_up_interruptible(w); }
void rust_vs_wait_pfmemalloc_interruptible_timeout(struct pglist_data *p, bool (*condition)(struct pglist_data *), long timeout) { wait_event_interruptible_timeout(p->pfmemalloc_wait, condition(p), timeout); }
void rust_vs_wait_pfmemalloc_killable(struct pglist_data *p, bool (*condition)(struct pglist_data *)) { wait_event_killable(p->pfmemalloc_wait, condition(p)); }
void rust_vs_cond_resched(void) { cond_resched(); }
void rust_vs_cond_resched_tasks_rcu_qs(void) { cond_resched_tasks_rcu_qs(); }
bool rust_vs_current_is_kswapd(void) { return current_is_kswapd(); }
bool rust_vs_current_is_khugepaged(void) { return current_is_khugepaged(); }
bool rust_vs_fatal_signal_pending(struct task_struct *t) { return fatal_signal_pending(t); }
bool rust_vs_signal_pending(struct task_struct *t) { return signal_pending(t); }
bool rust_vs_freezing(struct task_struct *t) { return freezing(t); }
void rust_vs_set_freezable(void) { set_freezable(); }
unsigned int rust_vs_memalloc_noreclaim_save(void) { return memalloc_noreclaim_save(); }
void rust_vs_memalloc_noreclaim_restore(unsigned int flags) { memalloc_noreclaim_restore(flags); }
gfp_t rust_vs_current_gfp_context(gfp_t flags) { return current_gfp_context(flags); }
void rust_vs_delayacct_freepages_start(void) { delayacct_freepages_start(); }
void rust_vs_delayacct_freepages_end(void) { delayacct_freepages_end(); }
void rust_vs_fs_reclaim_acquire(gfp_t gfp) { fs_reclaim_acquire(gfp); }
void rust_vs_fs_reclaim_release(gfp_t gfp) { fs_reclaim_release(gfp); }
void rust_vs_fs_reclaim_acquire_balance(void) { __fs_reclaim_acquire(_THIS_IP_); }
void rust_vs_fs_reclaim_release_balance(void) { __fs_reclaim_release(_THIS_IP_); }
void rust_vs_fs_reclaim_acquire_freeze(void) { __fs_reclaim_acquire(_THIS_IP_); }
void rust_vs_fs_reclaim_release_freeze(void) { __fs_reclaim_release(_THIS_IP_); }
void rust_vs_psi_memstall_enter(unsigned long *flags) { psi_memstall_enter(flags); }
void rust_vs_psi_memstall_leave(unsigned long *flags) { psi_memstall_leave(flags); }
bool rust_vs_is_err(const void *p) { return IS_ERR(p); }
struct task_struct * rust_vs_create_kswapd(int (*fn)(void *), struct pglist_data *p, int nid) { return kthread_create_on_node(fn, p, nid, "kswapd%d", nid); }
void rust_vs_pgdat_kswapd_lock(struct pglist_data *p) { pgdat_kswapd_lock(p); }
void rust_vs_pgdat_kswapd_unlock(struct pglist_data *p) { pgdat_kswapd_unlock(p); }
void rust_vs_error_kswapd_start(int nid, struct task_struct *p) { pr_err("Failed to start kswapd on node %d, ret=%pe\n", nid, p); }
void rust_vs_warn_reclaim_overwrite(bool condition) { WARN_ON_ONCE(condition); }
void rust_vs_warn_reclaim_null(bool condition) { WARN_ON_ONCE(condition); }
void rust_vs_warn_throttle_reason(void) { WARN_ON_ONCE(1); }
void rust_vs_warn_anon_only(bool condition) { WARN_ON_ONCE(condition); }
#ifdef CONFIG_MEMCG
void rust_vs_warn_softlimit_reclaim_state(bool condition) { WARN_ON_ONCE(condition); }
#endif
void rust_vs_bug_remove_unlocked(bool condition) { BUG_ON(condition); }
void rust_vs_bug_remove_mapping(bool condition) { BUG_ON(condition); }
void rust_vs_bug_kswapd_boot(bool condition) { BUG_ON(condition); }
void rust_vs_bug_shrink_active(bool condition, struct folio *f) { VM_BUG_ON_FOLIO(condition, f); }
void rust_vs_bug_activate_active(bool condition, struct folio *f) { VM_BUG_ON_FOLIO(condition, f); }
void rust_vs_bug_keep_lru(bool condition, struct folio *f) { VM_BUG_ON_FOLIO(condition, f); }
void rust_vs_bug_isolate_ref(bool condition, struct folio *f) { VM_BUG_ON_FOLIO(condition, f); }
void rust_vs_bug_move_lru(bool condition, struct folio *f) { VM_BUG_ON_FOLIO(condition, f); }
void rust_vs_prefetch_prev_lru_flags(struct folio *f, struct list_head *base) { prefetchw_prev_lru_folio(f, base, flags); }
bool rust_vs_vma_flags_test(const vma_flags_t *flags, unsigned int bit) { return vma_flags_test(flags, bit); }
bool rust_vs_folio_is_file_lru(const struct folio *f) { return folio_is_file_lru(f); }
bool rust_vs_folio_test_active(const struct folio *f) { return folio_test_active(f); }
bool rust_vs_folio_test_anon(const struct folio *f) { return folio_test_anon(f); }
bool rust_vs_folio_test_dirty(const struct folio *f) { return folio_test_dirty(f); }
bool rust_vs_folio_test_hugetlb(const struct folio *f) { return folio_test_hugetlb(f); }
bool rust_vs_folio_test_large(const struct folio *f) { return folio_test_large(f); }
bool rust_vs_folio_test_lazyfree(const struct folio *f) { return folio_test_lazyfree(f); }
bool rust_vs_folio_test_locked(const struct folio *f) { return folio_test_locked(f); }
bool rust_vs_folio_test_lru(const struct folio *f) { return folio_test_lru(f); }
bool rust_vs_folio_test_mlocked(const struct folio *f) { return folio_test_mlocked(f); }
bool rust_vs_folio_test_pmd_mappable(const struct folio *f) { return folio_test_pmd_mappable(f); }
bool rust_vs_folio_test_private(const struct folio *f) { return folio_test_private(f); }
bool rust_vs_folio_test_reclaim(const struct folio *f) { return folio_test_reclaim(f); }
bool rust_vs_folio_test_referenced(const struct folio *f) { return folio_test_referenced(f); }
bool rust_vs_folio_test_swapbacked(const struct folio *f) { return folio_test_swapbacked(f); }
bool rust_vs_folio_test_swapcache(const struct folio *f) { return folio_test_swapcache(f); }
bool rust_vs_folio_test_unevictable(const struct folio *f) { return folio_test_unevictable(f); }
bool rust_vs_folio_test_workingset(const struct folio *f) { return folio_test_workingset(f); }
bool rust_vs_folio_test_writeback(const struct folio *f) { return folio_test_writeback(f); }
bool rust_vs_folio_test_clear_lru(struct folio *f) { return folio_test_clear_lru(f); }
bool rust_vs_folio_test_clear_referenced(struct folio *f) { return folio_test_clear_referenced(f); }
void rust_vs_folio_set_active(struct folio *f) { folio_set_active(f); }
void rust_vs_folio_set_lru(struct folio *f) { folio_set_lru(f); }
void rust_vs_folio_set_reclaim(struct folio *f) { folio_set_reclaim(f); }
void rust_vs_folio_set_referenced(struct folio *f) { folio_set_referenced(f); }
void rust_vs_folio_set_workingset(struct folio *f) { folio_set_workingset(f); }
void rust_vs_folio_clear_active(struct folio *f) { folio_clear_active(f); }
void rust_vs_folio_clear_reclaim(struct folio *f) { folio_clear_reclaim(f); }
void rust_vs_folio_clear_unevictable(struct folio *f) { folio_clear_unevictable(f); }
void rust_vs___folio_clear_lru_flags(struct folio *f) { __folio_clear_lru_flags(f); }
unsigned long rust_vs_folio_nr_pages(const struct folio *f) { return folio_nr_pages(f); }
unsigned int rust_vs_folio_order(const struct folio *f) { return folio_order(f); }
int rust_vs_folio_nid(const struct folio *f) { return folio_nid(f); }
unsigned int rust_vs_folio_zonenum(const struct folio *f) { return folio_zonenum(f); }
unsigned long rust_vs_folio_pfn(const struct folio *f) { return folio_pfn(f); }
int rust_vs_folio_ref_count(const struct folio *f) { return folio_ref_count(f); }
int rust_vs_folio_expected_ref_count(const struct folio *f) { return folio_expected_ref_count(f); }
bool rust_vs_folio_ref_freeze(struct folio *f, int count) { return folio_ref_freeze(f, count); }
void rust_vs_folio_ref_unfreeze(struct folio *f, int count) { folio_ref_unfreeze(f, count); }
bool rust_vs_folio_try_get(struct folio *f) { return folio_try_get(f); }
void rust_vs_folio_get(struct folio *f) { folio_get(f); }
void rust_vs_folio_put(struct folio *f) { folio_put(f); }
bool rust_vs_folio_put_testzero(struct folio *f) { return folio_put_testzero(f); }
bool rust_vs_folio_maybe_dma_pinned(const struct folio *f) { return folio_maybe_dma_pinned(f); }
bool rust_vs_folio_mapped(const struct folio *f) { return folio_mapped(f); }
bool rust_vs_folio_trylock(struct folio *f) { return folio_trylock(f); }
void rust_vs_folio_lock(struct folio *f) { folio_lock(f); }
void rust_vs_folio_unlock(struct folio *f) { folio_unlock(f); }
void rust_vs_folio_wait_writeback(struct folio *f) { folio_wait_writeback(f); }
bool rust_vs_folio_contain_hwpoisoned_page(struct folio *f) { return folio_contain_hwpoisoned_page(f); }
bool rust_vs_folio_evictable(struct folio *f) { return folio_evictable(f); }
bool rust_vs_folio_needs_release(struct folio *f) { return folio_needs_release(f); }
bool rust_vs_folio_deferred_partially_mapped(struct folio *f) { return data_race(!list_empty(&f->_deferred_list) && folio_test_partially_mapped(f)); }
void rust_vs_folio_unqueue_deferred_split(struct folio *f) { folio_unqueue_deferred_split(f); }
struct address_space * rust_vs_folio_mapping(struct folio *f) { return folio_mapping(f); }
void rust_vs_folio_batch_init(struct folio_batch *b) { folio_batch_init(b); }
unsigned int rust_vs_folio_batch_add(struct folio_batch *b, struct folio *f) { return folio_batch_add(b, f); }
bool rust_vs_folio_free_swap(struct folio *f) { return folio_free_swap(f); }
int rust_vs_folio_alloc_swap(struct folio *f) { return folio_alloc_swap(f); }
bool rust_vs_page_has_movable_ops(const struct page *p) { return page_has_movable_ops(p); }
int rust_vs_split_folio_to_list(struct folio *f, struct list_head *l) { return split_folio_to_list(f, l); }
bool rust_vs_thp_migration_supported(void) { return thp_migration_supported(); }
void rust_vs_try_to_unmap_flush(void) { try_to_unmap_flush(); }
void rust_vs_try_to_unmap_flush_dirty(void) { try_to_unmap_flush_dirty(); }
struct swap_info_struct * rust_vs___swap_entry_to_info(swp_entry_t entry) { return __swap_entry_to_info(entry); }
struct swap_cluster_info * rust_vs_swap_cluster_get_and_lock_irq(struct folio *f) { return swap_cluster_get_and_lock_irq(f); }
void rust_vs_swap_cluster_unlock_irq(struct swap_cluster_info *ci) { swap_cluster_unlock_irq(ci); }
void rust_vs___memcg1_swapout(struct folio *f, struct swap_cluster_info *ci) { __memcg1_swapout(f, ci); }
void rust_vs___swap_cache_del_folio(struct swap_cluster_info *ci, struct folio *f, swp_entry_t entry, void *shadow) { __swap_cache_del_folio(ci, f, entry, shadow); }
int rust_vs_swap_writeout(struct swap_io_ctx *ctx, struct folio *f) { return swap_writeout(ctx, f); }
void rust_vs_swap_write_submit(struct swap_io_ctx *ctx) { swap_write_submit(ctx); }
bool rust_vs_shmem_mapping(struct address_space *m) { return shmem_mapping(m); }
bool rust_vs_dax_mapping(struct address_space *m) { return dax_mapping(m); }
bool rust_vs_mapping_exiting(struct address_space *m) { return mapping_exiting(m); }
bool rust_vs_mapping_shrinkable(struct address_space *m) { return mapping_shrinkable(m); }
bool rust_vs_mapping_writeback_may_deadlock_on_reclaim(struct address_space *m) { return mapping_writeback_may_deadlock_on_reclaim(m); }
void rust_vs_mapping_set_error(struct address_space *m, int error) { mapping_set_error(m, error); }
bool rust_vs_gfp_has_io_fs(gfp_t gfp) { return gfp_has_io_fs(gfp); }
bool rust_vs_gfp_compaction_allowed(gfp_t gfp) { return gfp_compaction_allowed(gfp); }
bool rust_vs_gfpflags_allow_blocking(gfp_t gfp) { return gfpflags_allow_blocking(gfp); }
unsigned int rust_vs_gfp_zone(gfp_t gfp) { return gfp_zone(gfp); }
unsigned long rust_vs_compact_gap(unsigned int order) { return compact_gap(order); }
bool rust_vs_compaction_suitable(struct zone *z, int order, unsigned long mark, int highest) { return compaction_suitable(z, order, mark, highest); }
void rust_vs_reset_isolation_suitable(struct pglist_data *p) { reset_isolation_suitable(p); }
void rust_vs_wakeup_kcompactd(struct pglist_data *p, int order, int highest) { wakeup_kcompactd(p, order, highest); }
bool rust_vs_cpuset_zone_allowed(struct zone *z, gfp_t gfp) { return cpuset_zone_allowed(z, gfp); }
bool rust_vs_managed_zone(struct zone *z) { return managed_zone(z); }
int rust_vs_zone_idx(const struct zone *z) { return zone_idx(z); }
int rust_vs_zone_to_nid(struct zone *z) { return zone_to_nid(z); }
unsigned long rust_vs_min_wmark_pages(struct zone *z) { return min_wmark_pages(z); }
unsigned long rust_vs_high_wmark_pages(struct zone *z) { return high_wmark_pages(z); }
unsigned long rust_vs_promo_wmark_pages(struct zone *z) { return promo_wmark_pages(z); }
unsigned long rust_vs_zone_page_state(struct zone *z, unsigned int item) { return zone_page_state(z, item); }
unsigned long rust_vs_zone_page_state_snapshot(struct zone *z, unsigned int item) { return zone_page_state_snapshot(z, item); }
unsigned long rust_vs_sum_zone_node_page_state(int nid, unsigned int item) { return sum_zone_node_page_state(nid, item); }
bool rust_vs_zone_watermark_ok(struct zone *z, unsigned int order, unsigned long mark, int highest, unsigned int flags) { return zone_watermark_ok(z, order, mark, highest, flags); }
struct zoneref * rust_vs_first_zones_zonelist(struct zonelist *zl, int highest, const nodemask_t *mask) { return first_zones_zonelist(zl, highest, mask); }
struct zoneref * rust_vs_next_zones_zonelist(struct zoneref *zr, int highest, const nodemask_t *mask) { return next_zones_zonelist(zr, highest, mask); }
struct zonelist * rust_vs_node_zonelist(int nid, gfp_t gfp) { return node_zonelist(nid, gfp); }
unsigned long rust_vs_node_page_state(struct pglist_data *p, unsigned int item) { return node_page_state(p, item); }
unsigned long rust_vs_node_page_state_pages(struct pglist_data *p, unsigned int item) { return node_page_state_pages(p, item); }
void rust_vs_node_stat_add_folio(struct folio *f, unsigned int item) { node_stat_add_folio(f, item); }
void rust_vs_node_stat_mod_folio(struct folio *f, unsigned int item, long delta) { node_stat_mod_folio(f, item, delta); }
void rust_vs___mod_node_page_state(struct pglist_data *p, unsigned int item, long delta) { __mod_node_page_state(p, item, delta); }
void rust_vs_mod_node_page_state(struct pglist_data *p, unsigned int item, long delta) { mod_node_page_state(p, item, delta); }
void rust_vs_count_vm_event(unsigned int item) { count_vm_event(item); }
void rust_vs_count_vm_events(unsigned int item, unsigned long delta) { count_vm_events(item, delta); }
void rust_vs___count_vm_events(unsigned int item, unsigned long delta) { __count_vm_events(item, delta); }
void rust_vs___count_zid_vm_events(unsigned int item, int zid, unsigned long delta) { __count_zid_vm_events(item, zid, delta); }
void rust_vs_count_mthp_stat(unsigned int order, unsigned int item) { count_mthp_stat(order, item); }
void rust_vs_set_pgdat_normal_threshold(struct pglist_data *p) { set_pgdat_percpu_threshold(p, calculate_normal_threshold); }
void rust_vs_set_pgdat_pressure_threshold(struct pglist_data *p) { set_pgdat_percpu_threshold(p, calculate_pressure_threshold); }
bool rust_vs_is_active_lru(unsigned int lru) { return is_active_lru(lru); }
bool rust_vs_is_file_lru(unsigned int lru) { return is_file_lru(lru); }
bool rust_vs_lru_gen_enabled(void) { return lru_gen_enabled(); }
bool rust_vs_lru_gen_switching(void) { return lru_gen_switching(); }
struct pglist_data * rust_vs_lruvec_pgdat(struct lruvec *lv) { return lruvec_pgdat(lv); }
struct mem_cgroup * rust_vs_lruvec_memcg(struct lruvec *lv) { return lruvec_memcg(lv); }
unsigned long rust_vs_lruvec_page_state(struct lruvec *lv, unsigned int item) { return lruvec_page_state(lv, item); }
unsigned long rust_vs_lruvec_page_state_monotonic(struct lruvec *lv, unsigned int item) { return lruvec_page_state_monotonic(lv, item); }
void rust_vs_lruvec_stat_mod_folio(struct folio *f, unsigned int item, long delta) { lruvec_stat_mod_folio(f, item, delta); }
void rust_vs_mod_lruvec_state(struct lruvec *lv, unsigned int item, long delta) { mod_lruvec_state(lv, item, delta); }
void rust_vs_update_lru_size(struct lruvec *lv, unsigned int lru, int zid, long delta) { update_lru_size(lv, lru, zid, delta); }
void rust_vs_lruvec_lock_irq(struct lruvec *lv) { lruvec_lock_irq(lv); }
void rust_vs_lruvec_unlock_irq(struct lruvec *lv) { lruvec_unlock_irq(lv); }
struct lruvec * rust_vs_folio_lruvec_lock_irq(struct folio *f) { return folio_lruvec_lock_irq(f); }
struct lruvec * rust_vs_folio_lruvec_relock_irq(struct folio *f, struct lruvec *lv) { return folio_lruvec_relock_irq(f, lv); }
void rust_vs_lruvec_add_folio(struct lruvec *lv, struct folio *f) { lruvec_add_folio(lv, f); }
void rust_vs_lruvec_del_folio(struct lruvec *lv, struct folio *f) { lruvec_del_folio(lv, f); }
void rust_vs_wake_throttle_isolated(struct pglist_data *p) { wake_throttle_isolated(p); }
unsigned long rust_vs_get_nr_swap_pages(void) { return get_nr_swap_pages(); }
bool rust_vs_mem_cgroup_disabled(void) { return mem_cgroup_disabled(); }
bool rust_vs_mem_cgroup_is_root(struct mem_cgroup *m) { return mem_cgroup_is_root(m); }
#ifdef CONFIG_CGROUP_WRITEBACK
bool rust_vs_memory_cgroup_on_dfl(void) { return cgroup_subsys_on_dfl(memory_cgrp_subsys); }
#endif
bool rust_vs_mem_cgroup_online(struct mem_cgroup *m) { return mem_cgroup_online(m); }
bool rust_vs_memcg_is_dying(struct mem_cgroup *m) { return memcg_is_dying(m); }
int rust_vs_mem_cgroup_swappiness(struct mem_cgroup *m) { return mem_cgroup_swappiness(m); }
unsigned long rust_vs_mem_cgroup_get_nr_swap_pages(struct mem_cgroup *m) { return mem_cgroup_get_nr_swap_pages(m); }
unsigned long rust_vs_mem_cgroup_get_zone_lru_size(struct lruvec *lv, unsigned int lru, int zid) { return mem_cgroup_get_zone_lru_size(lv, lru, zid); }
struct mem_cgroup * rust_vs_mem_cgroup_iter(struct mem_cgroup *root, struct mem_cgroup *prev, struct mem_cgroup_reclaim_cookie *cookie) { return mem_cgroup_iter(root, prev, cookie); }
void rust_vs_mem_cgroup_iter_break(struct mem_cgroup *root, struct mem_cgroup *prev) { mem_cgroup_iter_break(root, prev); }
struct lruvec * rust_vs_mem_cgroup_lruvec(struct mem_cgroup *m, struct pglist_data *p) { return mem_cgroup_lruvec(m, p); }
void rust_vs_mem_cgroup_flush_stats_ratelimited(struct mem_cgroup *m) { mem_cgroup_flush_stats_ratelimited(m); }
void rust_vs_mem_cgroup_node_filter_allowed(struct mem_cgroup *m, nodemask_t *mask) { mem_cgroup_node_filter_allowed(m, mask); }
void rust_vs_mem_cgroup_protection(struct mem_cgroup *root, struct mem_cgroup *m, unsigned long *min, unsigned long *low, unsigned long *usage) { mem_cgroup_protection(root, m, min, low, usage); }
void rust_vs_mem_cgroup_calculate_protection(struct mem_cgroup *root, struct mem_cgroup *m) { mem_cgroup_calculate_protection(root, m); }
bool rust_vs_mem_cgroup_below_min(struct mem_cgroup *root, struct mem_cgroup *m) { return mem_cgroup_below_min(root, m); }
bool rust_vs_mem_cgroup_below_low(struct mem_cgroup *root, struct mem_cgroup *m) { return mem_cgroup_below_low(root, m); }
bool rust_vs_mem_cgroup_swap_full(struct folio *f) { return mem_cgroup_swap_full(f); }
void rust_vs_mem_cgroup_uncharge_folios(struct folio_batch *batch) { mem_cgroup_uncharge_folios(batch); }
void rust_vs_count_memcg_events(struct mem_cgroup *m, unsigned int item, unsigned long delta) { count_memcg_events(m, item, delta); }
void rust_vs_count_memcg_folio_events(struct folio *f, unsigned int item, unsigned long delta) { count_memcg_folio_events(f, item, delta); }
void rust_vs_memcg_memory_event(struct mem_cgroup *m, unsigned int event) { memcg_memory_event(m, event); }
unsigned long rust_vs_memcg1_soft_limit_reclaim(struct pglist_data *p, int order, gfp_t gfp, unsigned long *scanned) { return memcg1_soft_limit_reclaim(p, order, gfp, scanned); }
void rust_vs_node_get_allowed_targets(struct pglist_data *p, nodemask_t *mask) { node_get_allowed_targets(p, mask); }
int rust_vs_next_demotion_node(int nid, const nodemask_t *mask) { return next_demotion_node(nid, mask); }
void rust_vs_vmpressure(gfp_t gfp, int order, struct mem_cgroup *m, bool tree, unsigned long scanned, unsigned long reclaimed) { vmpressure(gfp, order, m, tree, scanned, reclaimed); }
void rust_vs_vmpressure_prio(gfp_t gfp, struct mem_cgroup *m, int priority) { vmpressure_prio(gfp, m, priority); }
void rust_vs_blk_start_plug(struct blk_plug *p) { blk_start_plug(p); }
void rust_vs_blk_finish_plug(struct blk_plug *p) { blk_finish_plug(p); }
unsigned int rust_vs_jiffies_to_usecs(unsigned long j) { return jiffies_to_usecs(j); }
u64 rust_vs_div64_u64(u64 n, u64 d) { return div64_u64(n, d); }
u64 rust_vs_div64_u64_round_up(u64 n, u64 d) { return DIV64_U64_ROUND_UP(n, d); }
void rust_vs_try_to_unmap(struct folio *f, unsigned int flags) { try_to_unmap(f, flags); }
void __init rust_vs_register_vmscan_sysctl(void) { register_sysctl_init("vm", vmscan_sysctl_table); }
int rust_vs_match_reclaim_token(char *s, substring_t *args) { return match_token(s, tokens, args); }
#if defined(CONFIG_SYSFS) && defined(CONFIG_NUMA)
int rust_vs_device_create_reclaim_file(struct device *d) { return device_create_file(d, &dev_attr_reclaim); }
#endif
#if defined(CONFIG_SYSFS) && defined(CONFIG_NUMA)
void rust_vs_device_remove_reclaim_file(struct device *d) { device_remove_file(d, &dev_attr_reclaim); }
#endif
#ifdef CONFIG_LRU_GEN
long rust_vs_atomic_long_read(const atomic_long_t *p) { return atomic_long_read(p); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_atomic_long_set(atomic_long_t *p, long v) { atomic_long_set(p, v); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_bitmap_clear(unsigned long *p, unsigned int start, unsigned int nr) { bitmap_clear(p, start, nr); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_bitmap_free(const unsigned long *p) { bitmap_free(p); }
#endif
#ifdef CONFIG_LRU_GEN
unsigned long * rust_vs_bitmap_zalloc(unsigned int n, gfp_t gfp) { return bitmap_zalloc(n, gfp); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_cgroup_lock(void) { cgroup_lock(); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_cgroup_unlock(void) { cgroup_unlock(); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_cpus_read_lock(void) { cpus_read_lock(); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_cpus_read_unlock(void) { cpus_read_unlock(); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_folio_activate(struct folio *f) { folio_activate(f); }
#endif
#ifdef CONFIG_LRU_GEN
int rust_vs_folio_lru_gen(const struct folio *f) { return folio_lru_gen(f); }
#endif
#ifdef CONFIG_LRU_GEN
struct mem_cgroup * rust_vs_folio_memcg(struct folio *f) { return folio_memcg(f); }
#endif
#ifdef CONFIG_LRU_GEN
struct pglist_data * rust_vs_folio_pgdat(const struct folio *f) { return folio_pgdat(f); }
#endif
#ifdef CONFIG_LRU_GEN
struct mem_cgroup * rust_vs_get_mem_cgroup_from_folio(struct folio *f) { return get_mem_cgroup_from_folio(f); }
#endif
#ifdef CONFIG_LRU_GEN
struct mem_cgroup * rust_vs_get_mem_cgroup_from_mm(struct mm_struct *mm) { return get_mem_cgroup_from_mm(mm); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_get_online_mems(void) { get_online_mems(); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_put_online_mems(void) { put_online_mems(); }
#endif
#ifdef CONFIG_LRU_GEN
u32 rust_vs_get_random_u32_below(u32 ceiling) { return get_random_u32_below(ceiling); }
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_is_vm_hugetlb_page(struct vm_area_struct *v) { return is_vm_hugetlb_page(v); }
#endif
#ifdef CONFIG_LRU_GEN
void * rust_vs_kvmalloc(size_t size, gfp_t gfp) { return kvmalloc(size, gfp); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_list_del_init(struct list_head *l) { list_del_init(l); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_list_move_tail(struct list_head *l, struct list_head *h) { list_move_tail(l, h); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_list_splice_tail_init(struct list_head *l, struct list_head *h) { list_splice_tail_init(l, h); }
#endif
#ifdef CONFIG_LRU_GEN
int rust_vs_lru_gen_from_seq(unsigned long seq) { return lru_gen_from_seq(seq); }
#endif
#ifdef CONFIG_LRU_GEN
int rust_vs_lru_hist_from_seq(unsigned long seq) { return lru_hist_from_seq(seq); }
#endif
#ifdef CONFIG_LRU_GEN
int rust_vs_lru_tier_from_refs(int refs, bool workingset) { return lru_tier_from_refs(refs, workingset); }
#endif
#ifdef CONFIG_LRU_GEN
unsigned long rust_vs_lru_gen_folio_seq(const struct lruvec *lv, const struct folio *f, bool reclaiming) { return lru_gen_folio_seq(lv, f, reclaiming); }
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_lru_gen_is_active(const struct lruvec *lv, int gen) { return lru_gen_is_active(lv, gen); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_lru_gen_update_size(struct lruvec *lv, struct folio *f, int old_gen, int new_gen) { lru_gen_update_size(lv, f, old_gen, new_gen); }
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_lru_gen_add_folio(struct lruvec *lv, struct folio *f, bool reclaiming) { return lru_gen_add_folio(lv, f, reclaiming); }
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_lru_gen_del_folio(struct lruvec *lv, struct folio *f, bool reclaiming) { return lru_gen_del_folio(lv, f, reclaiming); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs___update_lru_size(struct lruvec *lv, unsigned int lru, int zid, long delta) { __update_lru_size(lv, lru, zid, delta); }
#endif
#ifdef CONFIG_LRU_GEN
struct lruvec * rust_vs_lruvec_live_lock_irq(struct lruvec *lv) { return lruvec_live_lock_irq(lv); }
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_mapping_unevictable(const struct address_space *m) { return mapping_unevictable(m); }
#endif
#ifdef CONFIG_LRU_GEN
struct mem_cgroup * rust_vs_mem_cgroup_from_task(struct task_struct *t) { return mem_cgroup_from_task(t); }
#endif
#ifdef CONFIG_LRU_GEN
struct mem_cgroup * rust_vs_mem_cgroup_get_from_id(u64 id) { return mem_cgroup_get_from_id(id); }
#endif
#ifdef CONFIG_LRU_GEN
u64 rust_vs_mem_cgroup_id(struct mem_cgroup *m) { return mem_cgroup_id(m); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_mem_cgroup_put(struct mem_cgroup *m) { mem_cgroup_put(m); }
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_mem_cgroup_tryget(struct mem_cgroup *m) { return mem_cgroup_tryget(m); }
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_mm_has_notifiers(struct mm_struct *mm) { return mm_has_notifiers(mm); }
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_mmap_read_trylock(struct mm_struct *mm) { return mmap_read_trylock(mm); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_mmap_read_unlock(struct mm_struct *mm) { mmap_read_unlock(mm); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_mmgrab(struct mm_struct *mm) { mmgrab(mm); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_mmdrop(struct mm_struct *mm) { mmdrop(mm); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_mutex_lock(struct mutex *m) { mutex_lock(m); }
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_mutex_trylock(struct mutex *m) { return mutex_trylock(m); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_mutex_unlock(struct mutex *m) { mutex_unlock(m); }
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_need_resched(void) { return need_resched(); }
#endif
#ifdef CONFIG_LRU_GEN
unsigned long rust_vs_pgdat_end_pfn(struct pglist_data *p) { return pgdat_end_pfn(p); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_rcu_read_lock(void) { rcu_read_lock(); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_rcu_read_unlock(void) { rcu_read_unlock(); }
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_spin_is_contended(spinlock_t *l) { return spin_is_contended(l); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_spin_lock_irq(spinlock_t *l) { spin_lock_irq(l); }
#endif
#ifdef CONFIG_LRU_GEN
void rust_vs_spin_unlock_irq(spinlock_t *l) { spin_unlock_irq(l); }
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_spin_trylock(spinlock_t *l) { return spin_trylock(l); }
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_vma_has_recency(const struct vm_area_struct *v) { return vma_has_recency(v); }
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_vma_is_accessible(const struct vm_area_struct *v) { return vma_is_accessible(v); }
#endif
#ifdef CONFIG_LRU_GEN
bool rust_vs_vma_is_anonymous(const struct vm_area_struct *v) { return vma_is_anonymous(v); }
#endif
#ifdef CONFIG_MEMCG
int rust_vs_cgroup_path(struct cgroup *c, char *buf, size_t len) { return cgroup_path(c, buf, len); }
#endif
#ifdef CONFIG_MEMCG
void rust_vs_mem_cgroup_update_lru_size(struct lruvec *lv, unsigned int lru, int zid, long delta) { mem_cgroup_update_lru_size(lv, lru, zid, delta); }
#endif
bool rust_vs_native_numa_demotion_enabled(void) { return numa_demotion_enabled; }
long rust_vs_native_total_swap_pages(void) { return total_swap_pages; }
int rust_vs_native_numa_balancing_mode(void) { return sysctl_numa_balancing_mode; }
int rust_vs_native_buffer_heads_over_limit(void) { return buffer_heads_over_limit; }
int rust_vs_folio_referenced(struct folio *f, int locked, struct mem_cgroup *m, vma_flags_t *flags) { return folio_referenced(f, locked, m, flags); }
struct folio * rust_vs_alloc_migration_target(struct folio *f, unsigned long private) { return alloc_migration_target(f, private); }
int rust_vs_migrate_pages(struct list_head *l, new_folio_t alloc, free_folio_t free, unsigned long private, enum migrate_mode mode, enum migrate_reason reason, unsigned int *succeeded) { return migrate_pages(l, alloc, free, private, mode, reason, succeeded); }
int rust_vs_unmap_poisoned_folio(struct folio *f, unsigned long pfn, bool must_kill) { return unmap_poisoned_folio(f, pfn, must_kill); }
int rust_vs_read_zone_type(const enum zone_type *p) { return (int)READ_ONCE(*p); }
void rust_vs_write_zone_type(enum zone_type *p, int v) { WRITE_ONCE(*p, (enum zone_type)v); }
void rust_vs_trace_mm_vmscan_kswapd_sleep(int nid) { trace_mm_vmscan_kswapd_sleep(nid); }
void rust_vs_trace_mm_vmscan_kswapd_wake(int nid, int zid, int order) { trace_mm_vmscan_kswapd_wake(nid, zid, order); }
void rust_vs_trace_mm_vmscan_balance_pgdat_begin(int nid, int order, int highest_zoneidx) { trace_mm_vmscan_balance_pgdat_begin(nid, order, highest_zoneidx); }
void rust_vs_trace_mm_vmscan_balance_pgdat_end(int nid, int order, int highest_zoneidx, unsigned long nr_reclaimed) { trace_mm_vmscan_balance_pgdat_end(nid, order, highest_zoneidx, nr_reclaimed); }
void rust_vs_trace_mm_vmscan_wakeup_kswapd(int nid, int zid, int order, gfp_t gfp_flags) { trace_mm_vmscan_wakeup_kswapd(nid, zid, order, gfp_flags); }
void rust_vs_trace_mm_vmscan_direct_reclaim_begin(gfp_t gfp_flags, int order, struct mem_cgroup *memcg) { trace_mm_vmscan_direct_reclaim_begin(gfp_flags, order, memcg); }
#ifdef CONFIG_MEMCG
void rust_vs_trace_mm_vmscan_memcg_reclaim_begin(gfp_t gfp_flags, int order, struct mem_cgroup *memcg) { trace_mm_vmscan_memcg_reclaim_begin(gfp_flags, order, memcg); }
#endif
#ifdef CONFIG_MEMCG
void rust_vs_trace_mm_vmscan_memcg_softlimit_reclaim_begin(gfp_t gfp_flags, int order, struct mem_cgroup *memcg) { trace_mm_vmscan_memcg_softlimit_reclaim_begin(gfp_flags, order, memcg); }
#endif
void rust_vs_trace_mm_vmscan_direct_reclaim_end(unsigned long nr_reclaimed, struct mem_cgroup *memcg) { trace_mm_vmscan_direct_reclaim_end(nr_reclaimed, memcg); }
#ifdef CONFIG_MEMCG
void rust_vs_trace_mm_vmscan_memcg_reclaim_end(unsigned long nr_reclaimed, struct mem_cgroup *memcg) { trace_mm_vmscan_memcg_reclaim_end(nr_reclaimed, memcg); }
#endif
#ifdef CONFIG_MEMCG
void rust_vs_trace_mm_vmscan_memcg_softlimit_reclaim_end(unsigned long nr_reclaimed, struct mem_cgroup *memcg) { trace_mm_vmscan_memcg_softlimit_reclaim_end(nr_reclaimed, memcg); }
#endif
void rust_vs_trace_mm_vmscan_lru_isolate(int highest_zoneidx, int order, unsigned long nr_requested, unsigned long nr_scanned, unsigned long nr_skipped, unsigned long nr_taken, int lru) { trace_mm_vmscan_lru_isolate(highest_zoneidx, order, nr_requested, nr_scanned, nr_skipped, nr_taken, lru); }
void rust_vs_trace_mm_vmscan_write_folio(struct folio *folio) { trace_mm_vmscan_write_folio(folio); }
void rust_vs_trace_mm_vmscan_reclaim_pages(int nid, unsigned long nr_scanned, unsigned long nr_reclaimed, struct reclaim_stat *stat) { trace_mm_vmscan_reclaim_pages(nid, nr_scanned, nr_reclaimed, stat); }
void rust_vs_trace_mm_vmscan_lru_shrink_inactive(int nid, unsigned long nr_scanned, unsigned long nr_reclaimed, struct reclaim_stat *stat, int priority, int file) { trace_mm_vmscan_lru_shrink_inactive(nid, nr_scanned, nr_reclaimed, stat, priority, file); }
void rust_vs_trace_mm_vmscan_lru_shrink_active(int nid, unsigned long nr_taken, unsigned long nr_active, unsigned long nr_deactivated, unsigned long nr_referenced, int priority, int file) { trace_mm_vmscan_lru_shrink_active(nid, nr_taken, nr_active, nr_deactivated, nr_referenced, priority, file); }
void rust_vs_trace_mm_vmscan_node_reclaim_begin(int nid, int order, gfp_t gfp_flags) { trace_mm_vmscan_node_reclaim_begin(nid, order, gfp_flags); }
void rust_vs_trace_mm_vmscan_node_reclaim_end(unsigned long nr_reclaimed, struct mem_cgroup *memcg) { trace_mm_vmscan_node_reclaim_end(nr_reclaimed, memcg); }
void rust_vs_trace_mm_vmscan_throttled(int nid, int usec_timeout, int usec_delayed, int reason) { trace_mm_vmscan_throttled(nid, usec_timeout, usec_delayed, reason); }
void rust_vs_trace_mm_vmscan_kswapd_reclaim_fail(int nid, int failures) { trace_mm_vmscan_kswapd_reclaim_fail(nid, failures); }
void rust_vs_trace_mm_vmscan_kswapd_clear_hopeless(int nid, int reason) { trace_mm_vmscan_kswapd_clear_hopeless(nid, reason); }
