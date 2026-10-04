/* SPDX-License-Identifier: GPL-2.0 */
/* Header/arch/trace/profiling leaves only; all compaction decisions are Rust. */
#ifndef RUST_COMPACTION_NATIVE_PRIMITIVES_H
#define RUST_COMPACTION_NATIVE_PRIMITIVES_H
unsigned int rust_compaction_pageblock_order(void);
struct page * rust_compaction_pfn_to_page(unsigned long pfn);
struct page * rust_compaction_pfn_to_online_page(unsigned long pfn);
struct page * rust_compaction_pageblock_pfn_to_page(unsigned long start, unsigned long end, struct zone *zone);
unsigned long rust_compaction_page_to_pfn(const struct page *page);
struct zone * rust_compaction_page_zone(const struct page *page);
#if defined(CONFIG_COMPACTION)
bool rust_compaction_pfn_valid(unsigned long pfn);
#endif
#if defined(CONFIG_COMPACTION) && defined(CONFIG_SPARSEMEM)
unsigned long rust_compaction_pfn_to_section_nr(unsigned long pfn);
#endif
#if defined(CONFIG_COMPACTION) && defined(CONFIG_SPARSEMEM)
bool rust_compaction_online_section_nr(unsigned long nr);
#endif
#if defined(CONFIG_COMPACTION) && defined(CONFIG_SPARSEMEM)
unsigned long rust_compaction_section_nr_to_pfn(unsigned long nr);
#endif
#if defined(CONFIG_COMPACTION) && defined(CONFIG_SPARSEMEM)
unsigned long rust_compaction_highest_present_section_nr(void);
#endif
unsigned long rust_compaction_zone_end_pfn(const struct zone *zone);
#if defined(CONFIG_COMPACTION)
bool rust_compaction_populated_zone(struct zone *zone);
#endif
bool rust_compaction_PageBuddy(struct page *page);
bool rust_compaction_PageCompound(struct page *page);
bool rust_compaction_PageHuge(struct page *page);
bool rust_compaction_PageLRU(struct page *page);
bool rust_compaction_PageMovableOpsIsolated(struct page *page);
struct page * rust_compaction_compound_head(struct page *page);
unsigned int rust_compaction_compound_order(struct page *page);
unsigned int rust_compaction_buddy_order(struct page *page);
unsigned long rust_compaction_buddy_order_unsafe(struct page *page);
bool rust_compaction_page_has_movable_ops(struct page *page);
struct folio * rust_compaction_page_folio(struct page *page);
#if defined(CONFIG_COMPACTION)
struct folio * rust_compaction_page_rmappable_folio(struct page *page);
#endif
struct folio * rust_compaction_folio_get_nontail_page(struct page *page);
struct list_head * rust_compaction_page_lru(struct page *page);
#if defined(CONFIG_COMPACTION)
struct list_head * rust_compaction_page_buddy_list(struct page *page);
#endif
struct page * rust_compaction_lru_page(struct list_head *entry);
#if defined(CONFIG_COMPACTION)
struct page * rust_compaction_buddy_list_page(struct list_head *entry);
#endif
struct list_head * rust_compaction_folio_lru(struct folio *folio);
#if defined(CONFIG_COMPACTION)
struct page * rust_compaction_folio_page(struct folio *folio);
#endif
#if defined(CONFIG_COMPACTION)
int rust_compaction_get_pageblock_migratetype(struct page *page);
#endif
#if defined(CONFIG_COMPACTION)
bool rust_compaction_get_pageblock_skip(struct page *page);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_set_pageblock_skip(struct page *page);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_clear_pageblock_skip(struct page *page);
#endif
void rust_compaction_set_page_refcounted(struct page *page);
unsigned long rust_compaction_folio_nr_pages(const struct folio *folio);
unsigned int rust_compaction_folio_order(struct folio *folio);
unsigned long rust_compaction_folio_page_idx(const struct folio *folio, const struct page *page);
int rust_compaction_folio_ref_count(struct folio *folio);
int rust_compaction_folio_mapcount(struct folio *folio);
struct address_space * rust_compaction_folio_mapping(struct folio *folio);
struct mem_cgroup * rust_compaction_folio_memcg(struct folio *folio);
bool rust_compaction_folio_test_hugetlb(struct folio *folio);
bool rust_compaction_folio_test_lru(struct folio *folio);
bool rust_compaction_folio_test_large(struct folio *folio);
bool rust_compaction_folio_test_dirty(struct folio *folio);
bool rust_compaction_folio_test_unevictable(struct folio *folio);
bool rust_compaction_folio_test_writeback(struct folio *folio);
bool rust_compaction_folio_test_clear_lru(struct folio *folio);
bool rust_compaction_folio_trylock(struct folio *folio);
bool rust_compaction_folio_put_testzero(struct folio *folio);
void rust_compaction_folio_set_lru(struct folio *folio);
void rust_compaction_folio_put(struct folio *folio);
bool rust_compaction_folio_is_file_lru(struct folio *folio);
bool rust_compaction_mapping_inaccessible(struct address_space *mapping);
bool rust_compaction_mapping_has_migrate_folio(struct address_space *mapping);
struct lruvec * rust_compaction_folio_lruvec(struct folio *folio);
struct mem_cgroup * rust_compaction_lruvec_memcg(struct lruvec *lruvec);
void rust_compaction_lruvec_unlock_irqrestore(struct lruvec *lruvec, unsigned long flags);
void rust_compaction_lruvec_del_folio(struct lruvec *lruvec, struct folio *folio);
void rust_compaction_node_stat_mod_folio(struct folio *folio, enum node_stat_item item, long nr);
unsigned long rust_compaction_node_page_state(pg_data_t *pgdat, enum node_stat_item item);
#if defined(CONFIG_COMPACTION)
unsigned long rust_compaction_zone_page_state(struct zone *zone, enum zone_stat_item item);
#endif
#if defined(CONFIG_COMPACTION)
unsigned long rust_compaction_zone_page_state_snapshot(struct zone *zone, enum zone_stat_item item);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_count_vm_event(enum vm_event_item item);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_count_vm_events(enum vm_event_item item, long delta);
#endif
void rust_compaction_list_del(struct list_head *entry);
void rust_compaction_list_add(struct list_head *entry, struct list_head *head);
void rust_compaction_list_add_tail(struct list_head *entry, struct list_head *head);
void rust_compaction_list_splice_tail(struct list_head *entry, struct list_head *head);
#if defined(CONFIG_COMPACTION)
void rust_compaction_list_cut_before(struct list_head *list, struct list_head *head, struct list_head *entry);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_list_cut_position(struct list_head *list, struct list_head *head, struct list_head *entry);
#endif
void rust_compaction_rcu_read_lock(void);
void rust_compaction_spin_lock_irqsave(spinlock_t *lock, unsigned long *flags);
bool rust_compaction_spin_trylock_irqsave(spinlock_t *lock, unsigned long *flags);
void rust_compaction_spin_unlock_irqrestore(spinlock_t *lock, unsigned long flags);
void rust_compaction_cond_resched(void);
struct task_struct * rust_compaction_current(void);
bool rust_compaction_fatal_signal_pending(struct task_struct *task);
#if defined(CONFIG_COMPACTION)
bool rust_compaction_need_resched(void);
#endif
#if defined(CONFIG_COMPACTION)
bool rust_compaction_is_migrate_cma(int mt);
#endif
#if defined(CONFIG_COMPACTION)
bool rust_compaction_is_migrate_movable(int mt);
#endif
#if defined(CONFIG_COMPACTION)
bool rust_compaction_free_area_empty(struct free_area *area, int mt);
#endif
#if defined(CONFIG_COMPACTION)
unsigned long rust_compaction_compact_gap(unsigned int order);
#endif
#if defined(CONFIG_COMPACTION)
unsigned int rust_compaction_hpage_order(void);
#endif
#if defined(CONFIG_COMPACTION)
unsigned long rust_compaction_low_wmark_pages(struct zone *zone);
#endif
#if defined(CONFIG_COMPACTION)
unsigned long rust_compaction_min_wmark_pages(struct zone *zone);
#endif
#if defined(CONFIG_COMPACTION)
unsigned long rust_compaction_high_wmark_pages(struct zone *zone);
#endif
#if defined(CONFIG_COMPACTION)
unsigned long rust_compaction_wmark_pages(struct zone *zone, unsigned int index);
#endif
#if defined(CONFIG_COMPACTION)
int rust_compaction_gfp_migratetype(gfp_t mask);
#endif
#if defined(CONFIG_COMPACTION)
bool rust_compaction_gfp_compaction_allowed(gfp_t mask);
#endif
#if defined(CONFIG_COMPACTION)
bool rust_compaction_cpusets_enabled(void);
#endif
#if defined(CONFIG_COMPACTION)
bool rust_compaction_cpuset_zone_allowed(struct zone *zone, gfp_t mask);
#endif
#if defined(CONFIG_COMPACTION)
struct zoneref * rust_compaction_first_zones_zonelist(struct zonelist *list, enum zone_type highest, const nodemask_t *nodes);
#endif
#if defined(CONFIG_COMPACTION)
struct zoneref * rust_compaction_next_zones_zonelist(struct zoneref *z, enum zone_type highest, const nodemask_t *nodes);
#endif
#if defined(CONFIG_COMPACTION)
struct zone * rust_compaction_zonelist_zone(struct zoneref *z);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_write_capture_zone(struct capture_control *capc, struct zone *zone);
#endif
#if defined(CONFIG_COMPACTION)
struct page * rust_compaction_read_capture_page(struct capture_control *capc);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_pgdat_kswapd_lock(pg_data_t *pgdat);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_pgdat_kswapd_unlock(pg_data_t *pgdat);
#endif
#if defined(CONFIG_COMPACTION)
bool rust_compaction_task_is_running(struct task_struct *task);
#endif
#if defined(CONFIG_COMPACTION)
u64 rust_compaction_div64_ul(u64 dividend, unsigned long divisor);
#endif
#if defined(CONFIG_COMPACTION)
int rust_compaction_first_online_node(void);
#endif
#if defined(CONFIG_COMPACTION)
int rust_compaction_next_online_node(int nid);
#endif
#if defined(CONFIG_COMPACTION)
int rust_compaction_first_memory_node(void);
#endif
#if defined(CONFIG_COMPACTION)
int rust_compaction_next_memory_node(int nid);
#endif
#if defined(CONFIG_COMPACTION)
int rust_compaction_nr_node_ids(void);
#endif
#if defined(CONFIG_COMPACTION)
pg_data_t * rust_compaction_NODE_DATA(int nid);
#endif
#if defined(CONFIG_COMPACTION) && defined(CONFIG_SYSFS) && defined(CONFIG_NUMA)
bool rust_compaction_node_online(int nid);
#endif
#if defined(CONFIG_COMPACTION) && defined(CONFIG_SYSFS) && defined(CONFIG_NUMA)
int rust_compaction_device_id(struct device *dev);
#endif
#if defined(CONFIG_COMPACTION) && defined(CONFIG_SYSFS) && defined(CONFIG_NUMA)
struct device * rust_compaction_node_device(struct node *node);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_wake_up_interruptible(wait_queue_head_t *wait);
#endif
#if defined(CONFIG_COMPACTION)
bool rust_compaction_wq_has_sleeper(wait_queue_head_t *wait);
#endif
#if defined(CONFIG_COMPACTION)
unsigned long rust_compaction_msecs_to_jiffies(unsigned int m);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_set_freezable(void);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_current_set_kcompactd(bool set);
#endif
#if defined(CONFIG_COMPACTION)
long rust_compaction_wait_event_freezable_timeout(pg_data_t *pgdat, bool (*condition)(pg_data_t *), long timeout);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_psi_memstall_enter(unsigned long *flags);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_psi_memstall_leave(unsigned long *flags);
#endif
#if defined(CONFIG_COMPACTION)
struct task_struct * rust_compaction_kthread_create_on_node(int (*threadfn)(void *), void *data, int nid);
#endif
#if defined(CONFIG_COMPACTION)
bool rust_compaction_IS_ERR(const void *p);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_report_kcompactd_start_failure(int nid);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_warn_RT_change(const char *name);
#endif
#if defined(CONFIG_COMPACTION)
bool rust_compaction_warn_min_pfn(bool condition);
#endif
#if defined(CONFIG_COMPACTION) && defined(CONFIG_DEBUG_VM)
void rust_compaction_bug_free_pfn(bool condition);
#endif
#if defined(CONFIG_COMPACTION) && defined(CONFIG_DEBUG_VM)
void rust_compaction_bug_migratepages(bool condition);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_register_sysctl_init(const char *path, const struct ctl_table *table, size_t count, const char *name);
#endif
struct page * rust_compaction_mark_allocated_hooks(struct page *(*noprof)(struct page *, unsigned int, gfp_t), struct page *page, unsigned int order, gfp_t flags);
#if defined(CONFIG_COMPACTION)
struct folio * rust_compaction_alloc_hooks(new_folio_t noprof, struct folio *src, unsigned long data);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_defer_compaction(struct zone *zone, int order);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_deferred(struct zone *zone, int order);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_defer_reset(struct zone *zone, int order);
#endif
void rust_compaction_trace_isolate_freepages(unsigned long start, unsigned long end, unsigned long scanned, unsigned long taken);
void rust_compaction_trace_isolate_migratepages(unsigned long start, unsigned long end, unsigned long scanned, unsigned long taken);
#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_fast_isolate_freepages(unsigned long start, unsigned long end, unsigned long scanned, unsigned long taken);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_finished(struct zone *zone, int order, int ret);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_suitable(struct zone *zone, int order, int ret);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_begin(struct compact_control *cc, unsigned long start, unsigned long end, bool sync);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_end(struct compact_control *cc, unsigned long start, unsigned long end, bool sync, int ret);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_migratepages(unsigned int nr, unsigned int succeeded);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_try_to_compact_pages(int order, gfp_t mask, int prio);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_wakeup_kcompactd(int nid, int order, enum zone_type highest);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_kcompactd_wake(int nid, int order, enum zone_type highest);
#endif
#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_kcompactd_sleep(int nid);
#endif
int rust_compaction_isolate_or_dissolve_huge_folio(struct folio *folio, struct list_head *list);
#if defined(CONFIG_COMPACTION)
bool rust_compaction_zone_watermark_ok(struct zone *zone, unsigned int order, unsigned long mark, int highest, unsigned int flags, unsigned long free_pages);
#endif
void rust_compaction_wake_throttle_isolated(pg_data_t *pgdat);
void rust_compaction_init_list_head(struct list_head *head);
bool rust_compaction_list_empty(const struct list_head *head);
#endif
