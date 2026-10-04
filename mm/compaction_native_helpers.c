// SPDX-License-Identifier: GPL-2.0
#include "compaction_native_includes.h"
#if defined(CONFIG_COMPACTION) || defined(CONFIG_CMA)
#define CREATE_TRACE_POINTS
#include <trace/events/compaction.h>
#include "compaction_native_primitives.h"
unsigned int rust_compaction_pageblock_order(void)
{
return pageblock_order;
}

struct page * rust_compaction_pfn_to_page(unsigned long pfn)
{
return pfn_to_page(pfn);
}

struct page * rust_compaction_pfn_to_online_page(unsigned long pfn)
{
return pfn_to_online_page(pfn);
}

struct page * rust_compaction_pageblock_pfn_to_page(unsigned long start, unsigned long end, struct zone *zone)
{
return pageblock_pfn_to_page(start, end, zone);
}

unsigned long rust_compaction_page_to_pfn(const struct page *page)
{
return page_to_pfn(page);
}

struct zone * rust_compaction_page_zone(const struct page *page)
{
return page_zone(page);
}

#if defined(CONFIG_COMPACTION)
bool rust_compaction_pfn_valid(unsigned long pfn)
{
return pfn_valid(pfn);
}
#endif

#if defined(CONFIG_COMPACTION) && defined(CONFIG_SPARSEMEM)
unsigned long rust_compaction_pfn_to_section_nr(unsigned long pfn)
{
return pfn_to_section_nr(pfn);
}
#endif

#if defined(CONFIG_COMPACTION) && defined(CONFIG_SPARSEMEM)
bool rust_compaction_online_section_nr(unsigned long nr)
{
return online_section_nr(nr);
}
#endif

#if defined(CONFIG_COMPACTION) && defined(CONFIG_SPARSEMEM)
unsigned long rust_compaction_section_nr_to_pfn(unsigned long nr)
{
return section_nr_to_pfn(nr);
}
#endif

#if defined(CONFIG_COMPACTION) && defined(CONFIG_SPARSEMEM)
unsigned long rust_compaction_highest_present_section_nr(void)
{
return __highest_present_section_nr;
}
#endif

unsigned long rust_compaction_zone_end_pfn(const struct zone *zone)
{
return zone_end_pfn(zone);
}

#if defined(CONFIG_COMPACTION)
bool rust_compaction_populated_zone(struct zone *zone)
{
return populated_zone(zone);
}
#endif

bool rust_compaction_PageBuddy(struct page *page)
{
return PageBuddy(page);
}

bool rust_compaction_PageCompound(struct page *page)
{
return PageCompound(page);
}

bool rust_compaction_PageHuge(struct page *page)
{
return PageHuge(page);
}

bool rust_compaction_PageLRU(struct page *page)
{
return PageLRU(page);
}

bool rust_compaction_PageMovableOpsIsolated(struct page *page)
{
return PageMovableOpsIsolated(page);
}

struct page * rust_compaction_compound_head(struct page *page)
{
return compound_head(page);
}

unsigned int rust_compaction_compound_order(struct page *page)
{
return compound_order(page);
}

unsigned int rust_compaction_buddy_order(struct page *page)
{
return buddy_order(page);
}

unsigned long rust_compaction_buddy_order_unsafe(struct page *page)
{
return buddy_order_unsafe(page);
}

bool rust_compaction_page_has_movable_ops(struct page *page)
{
return page_has_movable_ops(page);
}

struct folio * rust_compaction_page_folio(struct page *page)
{
return page_folio(page);
}

#if defined(CONFIG_COMPACTION)
struct folio * rust_compaction_page_rmappable_folio(struct page *page)
{
return page_rmappable_folio(page);
}
#endif

struct folio * rust_compaction_folio_get_nontail_page(struct page *page)
{
return folio_get_nontail_page(page);
}

struct list_head * rust_compaction_page_lru(struct page *page)
{
return &page->lru;
}

#if defined(CONFIG_COMPACTION)
struct list_head * rust_compaction_page_buddy_list(struct page *page)
{
return &page->buddy_list;
}
#endif

struct page * rust_compaction_lru_page(struct list_head *entry)
{
return list_entry(entry, struct page, lru);
}

#if defined(CONFIG_COMPACTION)
struct page * rust_compaction_buddy_list_page(struct list_head *entry)
{
return list_entry(entry, struct page, buddy_list);
}
#endif

struct list_head * rust_compaction_folio_lru(struct folio *folio)
{
return &folio->lru;
}

#if defined(CONFIG_COMPACTION)
struct page * rust_compaction_folio_page(struct folio *folio)
{
return &folio->page;
}
#endif

#if defined(CONFIG_COMPACTION)
int rust_compaction_get_pageblock_migratetype(struct page *page)
{
return get_pageblock_migratetype(page);
}
#endif

#if defined(CONFIG_COMPACTION)
bool rust_compaction_get_pageblock_skip(struct page *page)
{
return get_pageblock_skip(page);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_set_pageblock_skip(struct page *page)
{
set_pageblock_skip(page);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_clear_pageblock_skip(struct page *page)
{
clear_pageblock_skip(page);
}
#endif

void rust_compaction_set_page_refcounted(struct page *page)
{
set_page_refcounted(page);
}

unsigned long rust_compaction_folio_nr_pages(const struct folio *folio)
{
return folio_nr_pages(folio);
}

unsigned int rust_compaction_folio_order(struct folio *folio)
{
return folio_order(folio);
}

unsigned long rust_compaction_folio_page_idx(const struct folio *folio, const struct page *page)
{
return folio_page_idx(folio, page);
}

int rust_compaction_folio_ref_count(struct folio *folio)
{
return folio_ref_count(folio);
}

int rust_compaction_folio_mapcount(struct folio *folio)
{
return folio_mapcount(folio);
}

struct address_space * rust_compaction_folio_mapping(struct folio *folio)
{
return folio_mapping(folio);
}

struct mem_cgroup * rust_compaction_folio_memcg(struct folio *folio)
{
return folio_memcg(folio);
}

bool rust_compaction_folio_test_hugetlb(struct folio *folio)
{
return folio_test_hugetlb(folio);
}

bool rust_compaction_folio_test_lru(struct folio *folio)
{
return folio_test_lru(folio);
}

bool rust_compaction_folio_test_large(struct folio *folio)
{
return folio_test_large(folio);
}

bool rust_compaction_folio_test_dirty(struct folio *folio)
{
return folio_test_dirty(folio);
}

bool rust_compaction_folio_test_unevictable(struct folio *folio)
{
return folio_test_unevictable(folio);
}

bool rust_compaction_folio_test_writeback(struct folio *folio)
{
return folio_test_writeback(folio);
}

bool rust_compaction_folio_test_clear_lru(struct folio *folio)
{
return folio_test_clear_lru(folio);
}

bool rust_compaction_folio_trylock(struct folio *folio)
{
return folio_trylock(folio);
}

bool rust_compaction_folio_put_testzero(struct folio *folio)
{
return folio_put_testzero(folio);
}

void rust_compaction_folio_set_lru(struct folio *folio)
{
folio_set_lru(folio);
}

void rust_compaction_folio_put(struct folio *folio)
{
folio_put(folio);
}

bool rust_compaction_folio_is_file_lru(struct folio *folio)
{
return folio_is_file_lru(folio);
}

bool rust_compaction_mapping_inaccessible(struct address_space *mapping)
{
return mapping_inaccessible(mapping);
}

bool rust_compaction_mapping_has_migrate_folio(struct address_space *mapping)
{
return mapping->a_ops->migrate_folio != NULL;
}

struct lruvec * rust_compaction_folio_lruvec(struct folio *folio)
{
return folio_lruvec(folio);
}

struct mem_cgroup * rust_compaction_lruvec_memcg(struct lruvec *lruvec)
{
return lruvec_memcg(lruvec);
}

void rust_compaction_lruvec_unlock_irqrestore(struct lruvec *lruvec, unsigned long flags)
{
lruvec_unlock_irqrestore(lruvec, flags);
}

void rust_compaction_lruvec_del_folio(struct lruvec *lruvec, struct folio *folio)
{
lruvec_del_folio(lruvec, folio);
}

void rust_compaction_node_stat_mod_folio(struct folio *folio, enum node_stat_item item, long nr)
{
node_stat_mod_folio(folio, item, nr);
}

unsigned long rust_compaction_node_page_state(pg_data_t *pgdat, enum node_stat_item item)
{
return node_page_state(pgdat, item);
}

#if defined(CONFIG_COMPACTION)
unsigned long rust_compaction_zone_page_state(struct zone *zone, enum zone_stat_item item)
{
return zone_page_state(zone, item);
}
#endif

#if defined(CONFIG_COMPACTION)
unsigned long rust_compaction_zone_page_state_snapshot(struct zone *zone, enum zone_stat_item item)
{
return zone_page_state_snapshot(zone, item);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_count_vm_event(enum vm_event_item item)
{
count_vm_event(item);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_count_vm_events(enum vm_event_item item, long delta)
{
count_vm_events(item, delta);
}
#endif

void rust_compaction_list_del(struct list_head *entry)
{
list_del(entry);
}

void rust_compaction_list_add(struct list_head *entry, struct list_head *head)
{
list_add(entry, head);
}

void rust_compaction_list_add_tail(struct list_head *entry, struct list_head *head)
{
list_add_tail(entry, head);
}

void rust_compaction_list_splice_tail(struct list_head *entry, struct list_head *head)
{
list_splice_tail(entry, head);
}

#if defined(CONFIG_COMPACTION)
void rust_compaction_list_cut_before(struct list_head *list, struct list_head *head, struct list_head *entry)
{
list_cut_before(list, head, entry);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_list_cut_position(struct list_head *list, struct list_head *head, struct list_head *entry)
{
list_cut_position(list, head, entry);
}
#endif

void rust_compaction_rcu_read_lock(void)
{
rcu_read_lock();
}

void rust_compaction_spin_lock_irqsave(spinlock_t *lock, unsigned long *flags)
{
spin_lock_irqsave(lock, *flags);
}

bool rust_compaction_spin_trylock_irqsave(spinlock_t *lock, unsigned long *flags)
{
return spin_trylock_irqsave(lock, *flags);
}

void rust_compaction_spin_unlock_irqrestore(spinlock_t *lock, unsigned long flags)
{
spin_unlock_irqrestore(lock, flags);
}

void rust_compaction_cond_resched(void)
{
cond_resched();
}

struct task_struct * rust_compaction_current(void)
{
return current;
}

bool rust_compaction_fatal_signal_pending(struct task_struct *task)
{
return fatal_signal_pending(task);
}

#if defined(CONFIG_COMPACTION)
bool rust_compaction_need_resched(void)
{
return need_resched();
}
#endif

#if defined(CONFIG_COMPACTION)
bool rust_compaction_is_migrate_cma(int mt)
{
return is_migrate_cma(mt);
}
#endif

#if defined(CONFIG_COMPACTION)
bool rust_compaction_is_migrate_movable(int mt)
{
return is_migrate_movable(mt);
}
#endif

#if defined(CONFIG_COMPACTION)
bool rust_compaction_free_area_empty(struct free_area *area, int mt)
{
return free_area_empty(area, mt);
}
#endif

#if defined(CONFIG_COMPACTION)
unsigned long rust_compaction_compact_gap(unsigned int order)
{
return compact_gap(order);
}
#endif

#if defined(CONFIG_COMPACTION)
unsigned int rust_compaction_hpage_order(void)
{
return
#if defined(CONFIG_TRANSPARENT_HUGEPAGE)
HPAGE_PMD_ORDER
#elif defined(CONFIG_HUGETLBFS)
HUGETLB_PAGE_ORDER
#else
(PMD_SHIFT - PAGE_SHIFT)
#endif
;
}
#endif

#if defined(CONFIG_COMPACTION)
unsigned long rust_compaction_low_wmark_pages(struct zone *zone)
{
return low_wmark_pages(zone);
}
#endif

#if defined(CONFIG_COMPACTION)
unsigned long rust_compaction_min_wmark_pages(struct zone *zone)
{
return min_wmark_pages(zone);
}
#endif

#if defined(CONFIG_COMPACTION)
unsigned long rust_compaction_high_wmark_pages(struct zone *zone)
{
return high_wmark_pages(zone);
}
#endif

#if defined(CONFIG_COMPACTION)
unsigned long rust_compaction_wmark_pages(struct zone *zone, unsigned int index)
{
return wmark_pages(zone, index);
}
#endif

#if defined(CONFIG_COMPACTION)
int rust_compaction_gfp_migratetype(gfp_t mask)
{
return gfp_migratetype(mask);
}
#endif

#if defined(CONFIG_COMPACTION)
bool rust_compaction_gfp_compaction_allowed(gfp_t mask)
{
return gfp_compaction_allowed(mask);
}
#endif

#if defined(CONFIG_COMPACTION)
bool rust_compaction_cpusets_enabled(void)
{
return cpusets_enabled();
}
#endif

#if defined(CONFIG_COMPACTION)
bool rust_compaction_cpuset_zone_allowed(struct zone *zone, gfp_t mask)
{
return __cpuset_zone_allowed(zone, mask);
}
#endif

#if defined(CONFIG_COMPACTION)
struct zoneref * rust_compaction_first_zones_zonelist(struct zonelist *list, enum zone_type highest, const nodemask_t *nodes)
{
return first_zones_zonelist(list, highest, nodes);
}
#endif

#if defined(CONFIG_COMPACTION)
struct zoneref * rust_compaction_next_zones_zonelist(struct zoneref *z, enum zone_type highest, const nodemask_t *nodes)
{
return next_zones_zonelist(z, highest, nodes);
}
#endif

#if defined(CONFIG_COMPACTION)
struct zone * rust_compaction_zonelist_zone(struct zoneref *z)
{
return zonelist_zone(z);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_write_capture_zone(struct capture_control *capc, struct zone *zone)
{
WRITE_ONCE(capc->zone, zone);
}
#endif

#if defined(CONFIG_COMPACTION)
struct page * rust_compaction_read_capture_page(struct capture_control *capc)
{
return READ_ONCE(capc->page);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_pgdat_kswapd_lock(pg_data_t *pgdat)
{
pgdat_kswapd_lock(pgdat);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_pgdat_kswapd_unlock(pg_data_t *pgdat)
{
pgdat_kswapd_unlock(pgdat);
}
#endif

#if defined(CONFIG_COMPACTION)
bool rust_compaction_task_is_running(struct task_struct *task)
{
return task_is_running(task);
}
#endif

#if defined(CONFIG_COMPACTION)
u64 rust_compaction_div64_ul(u64 dividend, unsigned long divisor)
{
return div64_ul(dividend, divisor);
}
#endif

#if defined(CONFIG_COMPACTION)
int rust_compaction_first_online_node(void)
{
return first_online_node;
}
#endif

#if defined(CONFIG_COMPACTION)
int rust_compaction_next_online_node(int nid)
{
return next_online_node(nid);
}
#endif

#if defined(CONFIG_COMPACTION)
int rust_compaction_first_memory_node(void)
{
return first_memory_node;
}
#endif

#if defined(CONFIG_COMPACTION)
int rust_compaction_next_memory_node(int nid)
{
return next_memory_node(nid);
}
#endif

#if defined(CONFIG_COMPACTION)
int rust_compaction_nr_node_ids(void)
{
return nr_node_ids;
}
#endif

#if defined(CONFIG_COMPACTION)
pg_data_t * rust_compaction_NODE_DATA(int nid)
{
return NODE_DATA(nid);
}
#endif

#if defined(CONFIG_COMPACTION) && defined(CONFIG_SYSFS) && defined(CONFIG_NUMA)
bool rust_compaction_node_online(int nid)
{
return node_online(nid);
}
#endif

#if defined(CONFIG_COMPACTION) && defined(CONFIG_SYSFS) && defined(CONFIG_NUMA)
int rust_compaction_device_id(struct device *dev)
{
return dev->id;
}
#endif

#if defined(CONFIG_COMPACTION) && defined(CONFIG_SYSFS) && defined(CONFIG_NUMA)
struct device * rust_compaction_node_device(struct node *node)
{
return &node->dev;
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_wake_up_interruptible(wait_queue_head_t *wait)
{
wake_up_interruptible(wait);
}
#endif

#if defined(CONFIG_COMPACTION)
bool rust_compaction_wq_has_sleeper(wait_queue_head_t *wait)
{
return wq_has_sleeper(wait);
}
#endif

#if defined(CONFIG_COMPACTION)
unsigned long rust_compaction_msecs_to_jiffies(unsigned int m)
{
return msecs_to_jiffies(m);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_set_freezable(void)
{
set_freezable();
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_current_set_kcompactd(bool set)
{
if (set) current->flags |= PF_KCOMPACTD; else current->flags &= ~PF_KCOMPACTD;
}
#endif

#if defined(CONFIG_COMPACTION)
long rust_compaction_wait_event_freezable_timeout(pg_data_t *pgdat, bool (*condition)(pg_data_t *), long timeout)
{
return wait_event_freezable_timeout(pgdat->kcompactd_wait, condition(pgdat), timeout);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_psi_memstall_enter(unsigned long *flags)
{
psi_memstall_enter(flags);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_psi_memstall_leave(unsigned long *flags)
{
psi_memstall_leave(flags);
}
#endif

#if defined(CONFIG_COMPACTION)
struct task_struct * rust_compaction_kthread_create_on_node(int (*threadfn)(void *), void *data, int nid)
{
return kthread_create_on_node(threadfn, data, nid, "kcompactd%d", nid);
}
#endif

#if defined(CONFIG_COMPACTION)
bool rust_compaction_IS_ERR(const void *p)
{
return IS_ERR(p);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_report_kcompactd_start_failure(int nid)
{
pr_err("Failed to start kcompactd on node %d\n", nid);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_warn_RT_change(const char *name)
{
pr_warn_once("sysctl attribute %s changed by %s[%d]\n", name, current->comm, task_pid_nr(current));
}
#endif

#if defined(CONFIG_COMPACTION)
bool rust_compaction_warn_min_pfn(bool condition)
{
return WARN_ON_ONCE(condition);
}
#endif

#if defined(CONFIG_COMPACTION) && defined(CONFIG_DEBUG_VM)
void rust_compaction_bug_free_pfn(bool condition)
{
VM_BUG_ON(condition);
}
#endif

#if defined(CONFIG_COMPACTION) && defined(CONFIG_DEBUG_VM)
void rust_compaction_bug_migratepages(bool condition)
{
VM_BUG_ON(condition);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_register_sysctl_init(const char *path, const struct ctl_table *table, size_t count, const char *name)
{
#ifdef CONFIG_SYSCTL
__register_sysctl_init(path, table, name, count);
#else
register_sysctl_init(path, table);
#endif
}
#endif

struct page * rust_compaction_mark_allocated_hooks(struct page *(*noprof)(struct page *, unsigned int, gfp_t), struct page *page, unsigned int order, gfp_t flags)
{
return alloc_hooks(noprof(page, order, flags));
}

#if defined(CONFIG_COMPACTION)
struct folio * rust_compaction_alloc_hooks(new_folio_t noprof, struct folio *src, unsigned long data)
{
return alloc_hooks(noprof(src, data));
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_defer_compaction(struct zone *zone, int order)
{
trace_mm_compaction_defer_compaction(zone, order);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_deferred(struct zone *zone, int order)
{
trace_mm_compaction_deferred(zone, order);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_defer_reset(struct zone *zone, int order)
{
trace_mm_compaction_defer_reset(zone, order);
}
#endif

void rust_compaction_trace_isolate_freepages(unsigned long start, unsigned long end, unsigned long scanned, unsigned long taken)
{
trace_mm_compaction_isolate_freepages(start, end, scanned, taken);
}

void rust_compaction_trace_isolate_migratepages(unsigned long start, unsigned long end, unsigned long scanned, unsigned long taken)
{
trace_mm_compaction_isolate_migratepages(start, end, scanned, taken);
}

#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_fast_isolate_freepages(unsigned long start, unsigned long end, unsigned long scanned, unsigned long taken)
{
trace_mm_compaction_fast_isolate_freepages(start, end, scanned, taken);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_finished(struct zone *zone, int order, int ret)
{
trace_mm_compaction_finished(zone, order, ret);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_suitable(struct zone *zone, int order, int ret)
{
trace_mm_compaction_suitable(zone, order, ret);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_begin(struct compact_control *cc, unsigned long start, unsigned long end, bool sync)
{
trace_mm_compaction_begin(cc, start, end, sync);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_end(struct compact_control *cc, unsigned long start, unsigned long end, bool sync, int ret)
{
trace_mm_compaction_end(cc, start, end, sync, ret);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_migratepages(unsigned int nr, unsigned int succeeded)
{
trace_mm_compaction_migratepages(nr, succeeded);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_try_to_compact_pages(int order, gfp_t mask, int prio)
{
trace_mm_compaction_try_to_compact_pages(order, mask, prio);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_wakeup_kcompactd(int nid, int order, enum zone_type highest)
{
trace_mm_compaction_wakeup_kcompactd(nid, order, highest);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_kcompactd_wake(int nid, int order, enum zone_type highest)
{
trace_mm_compaction_kcompactd_wake(nid, order, highest);
}
#endif

#if defined(CONFIG_COMPACTION)
void rust_compaction_trace_kcompactd_sleep(int nid)
{
trace_mm_compaction_kcompactd_sleep(nid);
}
#endif

int rust_compaction_isolate_or_dissolve_huge_folio(struct folio *folio, struct list_head *list)
{
return isolate_or_dissolve_huge_folio(folio, list);
}

#if defined(CONFIG_COMPACTION)
bool rust_compaction_zone_watermark_ok(struct zone *zone, unsigned int order, unsigned long mark, int highest, unsigned int flags, unsigned long free_pages)
{
return __zone_watermark_ok(zone, order, mark, highest, flags, free_pages);
}
#endif

void rust_compaction_wake_throttle_isolated(pg_data_t *pgdat)
{
wake_throttle_isolated(pgdat);
}

void rust_compaction_init_list_head(struct list_head *head)
{
INIT_LIST_HEAD(head);
}

bool rust_compaction_list_empty(const struct list_head *head)
{
return list_empty(head);
}

#ifdef CONFIG_COMPACTION
extern int rust_compaction_kcompactd_init(void);
subsys_initcall(rust_compaction_kcompactd_init);
#endif
#endif
