/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_PAGE_ALLOC_BINDINGS_H
#define LUPOS_PAGE_ALLOC_BINDINGS_H
#include <linux/stddef.h>
#include <linux/mm.h>
#include <linux/highmem.h>
#include <linux/interrupt.h>
#include <linux/jiffies.h>
#include <linux/compiler.h>
#include <linux/kernel.h>
#include <linux/kasan.h>
#include <linux/kmsan.h>
#include <linux/module.h>
#include <linux/suspend.h>
#include <linux/ratelimit.h>
#include <linux/oom.h>
#include <linux/topology.h>
#include <linux/sysctl.h>
#include <linux/cpu.h>
#include <linux/cpuset.h>
#include <linux/folio_batch.h>
#include <linux/memory_hotplug.h>
#include <linux/nodemask.h>
#include <linux/vmstat.h>
#include <linux/fault-inject.h>
#include <linux/compaction.h>
#include <trace/events/kmem.h>
#include <trace/events/oom.h>
#include <linux/prefetch.h>
#include <linux/mm_inline.h>
#include <linux/mmu_notifier.h>
#include <linux/migrate.h>
#include <linux/sched/mm.h>
#include <linux/page_owner.h>
#include <linux/page_table_check.h>
#include <linux/memcontrol.h>
#include <linux/ftrace.h>
#include <linux/lockdep.h>
#include <linux/psi.h>
#include <linux/khugepaged.h>
#include <linux/delayacct.h>
#include <linux/cacheinfo.h>
#include <linux/pgalloc_tag.h>
#include <asm/div64.h>
#include "internal.h"
#include "mm_init.h"
#include "page_alloc.h"
#include "shuffle.h"
#include "page_reporting.h"
#define PA_CONST(x) RUST_PA_##x = x
 static_assert(__MIGRATE_TYPE_END <= PAGEBLOCK_MIGRATETYPE_MASK);
/* Bindgen sees the configured header values, never invented extern objects. */
enum {
 RUST_PA_SIZEOF_PAGE = sizeof(struct page), RUST_PA_ALIGNOF_PAGE = __alignof__(struct page),
 RUST_PA_SIZEOF_ZONE = sizeof(struct zone), RUST_PA_SIZEOF_PCP = sizeof(struct per_cpu_pages),
 RUST_PA_PGALLOC = PGALLOC_NORMAL, PA_CONST(MAX_NUMNODES), PA_CONST(MAX_NR_ZONES), PA_CONST(MAX_PAGE_ORDER), PA_CONST(NR_PAGE_ORDERS),
 PA_CONST(NR_PAGEBLOCK_BITS), PA_CONST(PAGEBLOCK_MIGRATETYPE_MASK), PA_CONST(PAGEBLOCK_ISO_MASK),
 PA_CONST(PAGE_ALLOC_COSTLY_ORDER), PA_CONST(NR_LOWORDER_PCP_LISTS), PA_CONST(NR_PCP_LISTS),
 PA_CONST(PAGE_SHIFT), PA_CONST(PAGE_SIZE), PA_CONST(HZ), PA_CONST(CONFIG_PCP_BATCH_SCALE_MAX),
 PA_CONST(PCPF_PREV_FREE_HIGH_ORDER), PA_CONST(PCPF_FREE_HIGH_BATCH),
 PA_CONST(ALLOC_WMARK_MIN), PA_CONST(ALLOC_WMARK_LOW), PA_CONST(ALLOC_WMARK_HIGH), PA_CONST(ALLOC_WMARK_MASK),
 PA_CONST(ALLOC_NO_WATERMARKS), PA_CONST(ALLOC_MIN_RESERVE), PA_CONST(ALLOC_NON_BLOCK), PA_CONST(ALLOC_HIGHATOMIC),
 PA_CONST(ALLOC_CPUSET), PA_CONST(ALLOC_CMA), PA_CONST(ALLOC_NOFRAGMENT), PA_CONST(ALLOC_KSWAPD), PA_CONST(ALLOC_OOM),
 PA_CONST(ALLOC_RESERVES), PA_CONST(ALLOC_NOLOCK), PA_CONST(ALLOC_DEFAULT),
#ifdef CONFIG_SPARSEMEM
 PA_CONST(PAGES_PER_SECTION),
#endif
#ifdef CONFIG_TRANSPARENT_HUGEPAGE
 PA_CONST(HPAGE_PMD_ORDER),
#endif
#ifdef CONFIG_NUMA
 PA_CONST(RECLAIM_DISTANCE),
#endif
};
#undef PA_CONST
static const unsigned long RUST_PA_INITIAL_JIFFIES = INITIAL_JIFFIES;
static const gfp_t RUST_PA_GFP_BOOT_MASK = GFP_BOOT_MASK;
static const gfp_t RUST_PA___GFP_KSWAPD_RECLAIM = __GFP_KSWAPD_RECLAIM;
static const gfp_t RUST_PA___GFP_DIRECT_RECLAIM = __GFP_DIRECT_RECLAIM;
static const gfp_t RUST_PA___GFP_HIGH = __GFP_HIGH;

extern struct mutex pcp_batch_high_lock;
extern struct mutex pcpu_drain_mutex;
#ifdef CONFIG_NUMA
extern struct static_key_true vm_numa_stat_key;
#endif
#ifdef CONFIG_DEFERRED_STRUCT_PAGE_INIT
extern struct static_key_true deferred_pages;
#endif
unsigned long rust_pa_page_to_pfn(const struct page *p);
struct page *rust_pa_pfn_to_page(unsigned long pfn);
struct zone *rust_pa_page_zone(const struct page *p);
unsigned int rust_pa_pageblock_order(void);
unsigned long rust_pa_pageblock_nr_pages(void);
unsigned long rust_pa_pageblock_start_pfn(unsigned long pfn);
unsigned long rust_pa_pageblock_end_pfn(unsigned long pfn);
int rust_pa_get_pageblock_migratetype(const struct page *p);
bool rust_pa_get_pageblock_isolate(const struct page *p);
void rust_pa_set_pageblock_isolate(struct page *p);
void rust_pa_clear_pageblock_isolate(struct page *p);
unsigned long rust_pa_read_ulong(const unsigned long *p);
int rust_pa_read_int(const int *p);
bool rust_pa_try_cmpxchg_ulong(unsigned long *p, unsigned long *old, unsigned long new);
bool rust_pa_test_bit(unsigned long bit, const unsigned long *p);
void rust_pa_set_bit(unsigned long bit, unsigned long *p);
void rust_pa_clear_bit(unsigned long bit, unsigned long *p);
bool rust_pa_warn_once(bool condition);
bool rust_pa_mutex_trylock(struct mutex *m);
void rust_pa_mutex_lock(struct mutex *m);
void rust_pa_mutex_unlock(struct mutex *m);
bool rust_pa_warn_get_pb_bit(bool condition);
bool rust_pa_warn_set_pb_bit(bool condition);
bool rust_pa_warn_clear_pb_bit(bool condition);
bool rust_pa_warn_highatomic_underflow(bool condition);
bool rust_pa_warn_highatomic_boundary(bool condition);
void rust_pa_warn_isolate_state(bool isolate);
void rust_pa_warn(bool condition);
void rust_pa_bug(bool condition);
void rust_pa_bug_page(bool condition, const struct page *page);
void rust_pa_warn_page(bool condition, const struct page *page);
void rust_pa_warn_isolation(int which);
void rust_pa_migrate_disable(void);
void rust_pa_migrate_enable(void);
void rust_pa_preempt_disable(void);
void rust_pa_preempt_enable(void);
struct per_cpu_pages *rust_pa_this_cpu_pcp(struct per_cpu_pages *pcp);
struct per_cpu_pages *rust_pa_per_cpu_pcp(struct per_cpu_pages *pcp, unsigned int cpu);
void rust_pa_spin_lock(spinlock_t *lock);
void rust_pa_spin_unlock(spinlock_t *lock);
bool rust_pa_spin_trylock(spinlock_t *lock);
unsigned long rust_pa_spin_lock_irqsave(spinlock_t *lock);
bool rust_pa_spin_trylock_irqsave(spinlock_t *lock, unsigned long *flags);
void rust_pa_spin_unlock_irqrestore(spinlock_t *lock, unsigned long flags);
bool rust_pa_can_spin_trylock(void);
void rust_pa_assert_zone_locked(struct zone *z);
#ifdef CONFIG_SPARSEMEM
unsigned long *rust_pa_section_usemap(unsigned long pfn);
#endif
bool rust_pa_deferred_pages_enabled(void);
#ifdef CONFIG_DEBUG_VM
unsigned int rust_pa_zone_span_seqbegin(struct zone *z);
bool rust_pa_zone_span_seqretry(struct zone *z, unsigned int seq);
void rust_pa_bad_zone(unsigned long pfn, struct zone *z, unsigned long start, unsigned long end);
#endif
unsigned long rust_pa_jiffies(void);
void rust_pa_bad_suppressed(unsigned long count);
void rust_pa_bad_process(struct page *p);
void rust_pa_dump_stack(void);
bool rust_pa_page_buddy(const struct page *p);
void rust_pa_clear_page_buddy(struct page *p);
void rust_pa_set_page_buddy(struct page *p);
void rust_pa_set_page_head(struct page *p);
void rust_pa_prep_compound_head(struct page *p, unsigned int order);
void rust_pa_prep_compound_tail(struct page *p, struct page *head, unsigned int order);
void rust_pa_set_page_private(struct page *p, unsigned long val);
unsigned long rust_pa_page_private(const struct page *p);
struct list_head *rust_pa_page_buddy_list(struct page *p);
struct list_head *rust_pa_page_pcp_list(struct page *p);
struct page *rust_pa_page_from_pcp(struct list_head *p);
bool rust_pa_is_pmd_order(unsigned int order);
int rust_pa_buddy_order(const struct page *p);
bool rust_pa_page_lru(const struct page *p);
bool rust_pa_page_has_movable_ops(struct page *p);
int rust_pa_page_to_nid(const struct page *p);
int rust_pa_zone_to_nid(const struct zone *z);
int rust_pa_zone_idx(const struct zone *z);
unsigned long rust_pa_zone_managed_pages(const struct zone *z);
unsigned long rust_pa_zone_page_state(struct zone *z, enum zone_stat_item item);
unsigned int rust_pa_smp_processor_id(void);
unsigned int rust_pa_next_online_cpu(unsigned int cpu);
bool rust_pa_cpu_online(unsigned int cpu);
unsigned int rust_pa_nr_cpu_ids(void);
void rust_pa_cpumask_set_cpu(unsigned int cpu, struct cpumask *mask);
void rust_pa_cpumask_clear_cpu(unsigned int cpu, struct cpumask *mask);
bool rust_pa_cpumask_test_cpu(unsigned int cpu, const struct cpumask *mask);
void rust_pa_count_vm_events(enum vm_event_item item, long count);
void rust_pa_count_zid_vm_events(enum vm_event_item item, int zid, long count);
void rust_pa_count_numa_events(struct zone *z, enum numa_stat_item item, long count);
bool rust_pa_numa_stat_enabled(void);
int rust_pa_numa_node_id(void);
bool rust_pa_kswapd_test_hopeless(struct pglist_data *pgdat);
void rust_pa_kswapd_clear_hopeless(struct pglist_data *pgdat);
int rust_pa_next_memory_node(int node);
unsigned int rust_pa_online_nodes(void);
struct page *rust_pa_folio_page(struct folio *f);
unsigned int rust_pa_folio_order(struct folio *f);
void *rust_pa_folio_private(struct folio *f);
void rust_pa_set_folio_private(struct folio *f, void *p);
void rust_pa_trace_free_batched(struct page *p);
void rust_pa_folio_batch_reinit(struct folio_batch *f);
bool rust_pa_page_compound(struct page *p);
int rust_pa_page_count(struct page *p);
void rust_pa_set_page_refcounted(struct page *p);
void rust_pa_split_page_owner(struct page *p, unsigned int order);
void rust_pa_pgalloc_tag_split(struct page *p, unsigned int order);
void rust_pa_split_page_memcg(struct page *p, unsigned int order);
void rust_pa_trace_extfrag(struct page *p, int order, int fallback_order, int mt, int fallback_mt);
struct zoneref *rust_pa_first_zones_zonelist(struct zonelist *list, int highest, const nodemask_t *mask);
struct zoneref *rust_pa_next_zones_zonelist(struct zoneref *z, int highest, const nodemask_t *mask);
bool rust_pa_cpusets_enabled(void);
bool rust_pa_cpuset_zone_allowed(struct zone *z, gfp_t gfp);
bool rust_pa_node_reclaim_enabled(void);
bool rust_pa_waitqueue_active(wait_queue_head_t *wq);
int rust_pa_node_distance(int a, int b);
int rust_pa_gfp_migratetype(gfp_t gfp);
static const unsigned long RUST_PA_LIST_POISON1 = (unsigned long)LIST_POISON1;
static const unsigned long RUST_PA_LIST_POISON2 = (unsigned long)LIST_POISON2;
struct list_head *rust_pa_read_list_ptr(struct list_head * const *p);
void rust_pa_write_list_ptr(struct list_head **p, struct list_head *v);
#ifdef CONFIG_LIST_HARDENED
bool rust_pa_list_add_report(struct list_head *n, struct list_head *p, struct list_head *q);
bool rust_pa_list_del_report(struct list_head *p);
#endif
bool rust_pa_impl_get_pfnblock_bit(const struct page *p, unsigned long pfn, unsigned int bit);
unsigned int rust_pa_impl_get_pfnblock_migratetype(const struct page *p, unsigned long pfn);
void rust_pa_impl_set_pfnblock_bit(const struct page *p, unsigned long pfn, unsigned int bit);
void rust_pa_impl_clear_pfnblock_bit(const struct page *p, unsigned long pfn, unsigned int bit);
void rust_pa_impl_init_pageblock_migratetype(struct page *p, unsigned int mt, bool isolate);
void rust_pa_impl_free_pages_core(struct page *p, unsigned int order, unsigned int context);
unsigned int rust_pa_impl_find_suitable_fallback(struct free_area *a, unsigned int order, int mt, bool claim, int *out);
#include "page_alloc_buddy_helpers.h"
#include "page_alloc_late_helpers.h"
#endif
