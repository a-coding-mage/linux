// SPDX-License-Identifier: GPL-2.0-only
#include "page_alloc_bindings.h"
/* Native registration and static lock/percpu initialization retain compiler ABI. */
DEFINE_MUTEX(pcp_batch_high_lock);
DEFINE_MUTEX(pcpu_drain_mutex);
#ifdef CONFIG_USE_PERCPU_NUMA_NODE_ID
DEFINE_PER_CPU(int, numa_node);
EXPORT_PER_CPU_SYMBOL(numa_node);
#endif
#ifdef CONFIG_HAVE_MEMORYLESS_NODES
DEFINE_PER_CPU(int, _numa_mem_);
EXPORT_PER_CPU_SYMBOL(_numa_mem_);
#endif
#ifdef CONFIG_NUMA
DEFINE_STATIC_KEY_TRUE(vm_numa_stat_key);
#endif
#ifdef CONFIG_DEFERRED_STRUCT_PAGE_INIT
DEFINE_STATIC_KEY_TRUE(deferred_pages);
#endif
#ifdef CONFIG_GCC_PLUGIN_LATENT_ENTROPY
volatile unsigned long latent_entropy __latent_entropy;
EXPORT_SYMBOL(latent_entropy);
#endif
EXPORT_SYMBOL(node_states);
EXPORT_SYMBOL(movable_zone);
#if MAX_NUMNODES > 1
EXPORT_SYMBOL(nr_node_ids);
EXPORT_SYMBOL(nr_online_nodes);
#endif
EXPORT_SYMBOL_GPL(split_page);
unsigned long rust_pa_page_to_pfn(const struct page *p) { return page_to_pfn(p); }
struct page *rust_pa_pfn_to_page(unsigned long pfn) { return pfn_to_page(pfn); }
struct zone *rust_pa_page_zone(const struct page *p) { return page_zone(p); }
unsigned int rust_pa_pageblock_order(void) { return pageblock_order; }
unsigned long rust_pa_pageblock_nr_pages(void) { return pageblock_nr_pages; }
unsigned long rust_pa_pageblock_start_pfn(unsigned long pfn) { return pageblock_start_pfn(pfn); }
unsigned long rust_pa_pageblock_end_pfn(unsigned long pfn) { return pageblock_end_pfn(pfn); }
int rust_pa_get_pageblock_migratetype(const struct page *p) { return get_pfnblock_migratetype(p, page_to_pfn(p)); }
bool rust_pa_get_pageblock_isolate(const struct page *p) { return get_pageblock_isolate((struct page *)p); }
void rust_pa_set_pageblock_isolate(struct page *p) { set_pageblock_isolate(p); }
void rust_pa_clear_pageblock_isolate(struct page *p) { clear_pageblock_isolate(p); }
unsigned long rust_pa_read_ulong(const unsigned long *p) { return READ_ONCE(*p); }
int rust_pa_read_int(const int *p) { return READ_ONCE(*p); }
bool rust_pa_try_cmpxchg_ulong(unsigned long *p, unsigned long *old, unsigned long new) { return try_cmpxchg(p, old, new); }
bool rust_pa_test_bit(unsigned long bit, const unsigned long *p) { return test_bit(bit, p); }
void rust_pa_set_bit(unsigned long bit, unsigned long *p) { set_bit(bit, p); }
void rust_pa_clear_bit(unsigned long bit, unsigned long *p) { clear_bit(bit, p); }
bool rust_pa_warn_once(bool c) { return WARN_ON_ONCE(c); }
void rust_pa_warn(bool c) { VM_WARN_ON(c); }
void rust_pa_bug(bool c) { BUG_ON(c); }
void rust_pa_bug_page(bool c, const struct page *p) { VM_BUG_ON_PAGE(c, p); }
void rust_pa_warn_page(bool c, const struct page *p) { VM_WARN_ON_PAGE(c, p); }
void rust_pa_warn_isolation(int w) {
 if (w == 0) VM_WARN_ONCE(1, "Use set_pageblock_isolate() for pageblock isolation");
 if (w == 1) VM_WARN_ONCE(1, "Use clear_pageblock_isolate() to unisolate pageblock");
 if (w == 2) VM_WARN_ONCE(1, "Set isolate=true to isolate pageblock with a migratetype");
}
void rust_pa_migrate_disable(void) { migrate_disable(); }
void rust_pa_migrate_enable(void) { migrate_enable(); }
void rust_pa_preempt_disable(void) { preempt_disable(); }
void rust_pa_preempt_enable(void) { preempt_enable(); }
struct per_cpu_pages *rust_pa_this_cpu_pcp(struct per_cpu_pages *p) { return this_cpu_ptr(p); }
struct per_cpu_pages *rust_pa_per_cpu_pcp(struct per_cpu_pages *p, unsigned int cpu) { return per_cpu_ptr(p, cpu); }
void rust_pa_spin_lock(spinlock_t *l) { spin_lock(l); }
void rust_pa_spin_unlock(spinlock_t *l) { spin_unlock(l); }
bool rust_pa_spin_trylock(spinlock_t *l) { return spin_trylock(l); }
unsigned long rust_pa_spin_lock_irqsave(spinlock_t *l) { unsigned long f; spin_lock_irqsave(l, f); return f; }
bool rust_pa_spin_trylock_irqsave(spinlock_t *l, unsigned long *f) { return spin_trylock_irqsave(l, *f); }
void rust_pa_spin_unlock_irqrestore(spinlock_t *l, unsigned long f) { spin_unlock_irqrestore(l, f); }
bool rust_pa_can_spin_trylock(void) { return can_spin_trylock(); }
void rust_pa_assert_zone_locked(struct zone *z) { lockdep_assert_held(&z->lock); }
#ifdef CONFIG_SPARSEMEM
unsigned long *rust_pa_section_usemap(unsigned long pfn) { return section_to_usemap(__pfn_to_section(pfn)); }
#endif
bool rust_pa_deferred_pages_enabled(void) { return deferred_pages_enabled(); }
#ifdef CONFIG_DEBUG_VM
unsigned int rust_pa_zone_span_seqbegin(struct zone *z) { return zone_span_seqbegin(z); }
bool rust_pa_zone_span_seqretry(struct zone *z, unsigned int s) { return zone_span_seqretry(z, s); }
void rust_pa_bad_zone(unsigned long pfn, struct zone *z, unsigned long s, unsigned long e) {
 pr_err("page 0x%lx outside node %d zone %s [ 0x%lx - 0x%lx ]\n", pfn, zone_to_nid(z), z->name, s, e);
}
#endif
unsigned long rust_pa_jiffies(void) { return jiffies; }
void rust_pa_bad_suppressed(unsigned long n) { pr_alert("BUG: Bad page state: %lu messages suppressed\n", n); }
void rust_pa_bad_process(struct page *p) { pr_alert("BUG: Bad page state in process %s  pfn:%05lx\n", current->comm, page_to_pfn(p)); }
void rust_pa_dump_stack(void) { dump_stack(); }
bool rust_pa_page_buddy(const struct page *p) { return PageBuddy((struct page *)p); }
void rust_pa_clear_page_buddy(struct page *p) { __ClearPageBuddy(p); }
void rust_pa_set_page_buddy(struct page *p) { __SetPageBuddy(p); }
void rust_pa_set_page_head(struct page *p) { __SetPageHead(p); }
void rust_pa_prep_compound_head(struct page *p, unsigned int o) { prep_compound_head(p,o); }
void rust_pa_prep_compound_tail(struct page *p, struct page *h, unsigned int o) { prep_compound_tail(p,h,o); }
void rust_pa_set_page_private(struct page *p, unsigned long v) { set_page_private(p,v); }
unsigned long rust_pa_page_private(const struct page *p) { return page_private(p); }
struct list_head *rust_pa_page_buddy_list(struct page *p) { return &p->buddy_list; }
struct list_head *rust_pa_page_pcp_list(struct page *p) { return &p->pcp_list; }
struct page *rust_pa_page_from_pcp(struct list_head *p) { return list_entry(p, struct page, pcp_list); }
bool rust_pa_is_pmd_order(unsigned int o) { return is_pmd_order(o); }
int rust_pa_buddy_order(const struct page *p) { return buddy_order((struct page *)p); }
bool rust_pa_page_lru(const struct page *p) { return PageLRU(p); }
bool rust_pa_page_has_movable_ops(struct page *p) { return page_has_movable_ops(p); }
int rust_pa_page_to_nid(const struct page *p) { return page_to_nid(p); }
int rust_pa_zone_to_nid(const struct zone *z) { return zone_to_nid(z); }
int rust_pa_zone_idx(const struct zone *z) { return zone_idx(z); }
unsigned long rust_pa_zone_managed_pages(const struct zone *z) { return zone_managed_pages((struct zone *)z); }
unsigned long rust_pa_zone_page_state(struct zone *z, enum zone_stat_item i) { return zone_page_state(z,i); }
unsigned int rust_pa_smp_processor_id(void) { return smp_processor_id(); }
unsigned int rust_pa_next_online_cpu(unsigned int c) { return cpumask_next(c, cpu_online_mask); }
bool rust_pa_cpu_online(unsigned int c) { return cpu_online(c); }
unsigned int rust_pa_nr_cpu_ids(void) { return nr_cpu_ids; }
void rust_pa_cpumask_set_cpu(unsigned int c, struct cpumask *m) { cpumask_set_cpu(c,m); }
void rust_pa_cpumask_clear_cpu(unsigned int c, struct cpumask *m) { cpumask_clear_cpu(c,m); }
bool rust_pa_cpumask_test_cpu(unsigned int c, const struct cpumask *m) { return cpumask_test_cpu(c,m); }
void rust_pa_count_vm_events(enum vm_event_item i, long n) { __count_vm_events(i,n); }
void rust_pa_count_zid_vm_events(enum vm_event_item i, int z, long n) { __count_vm_events(i - ZONE_NORMAL + z,n); }
void rust_pa_count_numa_events(struct zone *z, enum numa_stat_item i, long n) { __count_numa_events(z,i,n); }
bool rust_pa_numa_stat_enabled(void) {
#ifdef CONFIG_NUMA
 return static_branch_likely(&vm_numa_stat_key);
#else
 return false;
#endif
}
int rust_pa_numa_node_id(void) { return numa_node_id(); }
bool rust_pa_kswapd_test_hopeless(struct pglist_data *p) { return kswapd_test_hopeless(p); }
void rust_pa_kswapd_clear_hopeless(struct pglist_data *p) { kswapd_clear_hopeless(p, KSWAPD_CLEAR_HOPELESS_PCP); }
int rust_pa_next_memory_node(int n) { return next_memory_node(n); }
unsigned int rust_pa_online_nodes(void) { return nr_online_nodes; }
struct page *rust_pa_folio_page(struct folio *f) { return &f->page; }
unsigned int rust_pa_folio_order(struct folio *f) { return folio_order(f); }
void *rust_pa_folio_private(struct folio *f) { return f->private; }
void rust_pa_set_folio_private(struct folio *f, void *p) { f->private = p; }
void rust_pa_trace_free_batched(struct page *p) { trace_mm_page_free_batched(p); }
void rust_pa_folio_batch_reinit(struct folio_batch *f) { folio_batch_reinit(f); }
bool rust_pa_page_compound(struct page *p) { return PageCompound(p); }
int rust_pa_page_count(struct page *p) { return page_count(p); }
void rust_pa_set_page_refcounted(struct page *p) { set_page_refcounted(p); }
void rust_pa_split_page_owner(struct page *p, unsigned int o) { split_page_owner(p,o,0); }
void rust_pa_pgalloc_tag_split(struct page *p, unsigned int o) { pgalloc_tag_split(page_folio(p),o,0); }
void rust_pa_split_page_memcg(struct page *p, unsigned int o) { split_page_memcg(p,o); }
void rust_pa_trace_extfrag(struct page *p, int o, int f, int mt, int ft) { trace_mm_page_alloc_extfrag(p,o,f,mt,ft); }
struct zoneref *rust_pa_first_zones_zonelist(struct zonelist *l, int h, const nodemask_t *m) { return first_zones_zonelist(l,h,m); }
struct zoneref *rust_pa_next_zones_zonelist(struct zoneref *z, int h, const nodemask_t *m) { return next_zones_zonelist(z,h,m); }
bool rust_pa_cpusets_enabled(void) { return cpusets_enabled(); }
bool rust_pa_cpuset_zone_allowed(struct zone *z, gfp_t g) { return __cpuset_zone_allowed(z,g); }
bool rust_pa_node_reclaim_enabled(void) { return node_reclaim_enabled(); }
bool rust_pa_waitqueue_active(wait_queue_head_t *w) { return waitqueue_active(w); }
int rust_pa_node_distance(int a, int b) { return node_distance(a,b); }
int rust_pa_gfp_migratetype(gfp_t g) { return gfp_migratetype(g); }
#include "page_alloc_buddy_helpers.c"
#include "page_alloc_late_helpers.c"
struct list_head *rust_pa_read_list_ptr(struct list_head * const *p) { return READ_ONCE(*p); }
void rust_pa_write_list_ptr(struct list_head **p, struct list_head *v) { WRITE_ONCE(*p,v); }
#ifdef CONFIG_LIST_HARDENED
bool rust_pa_list_add_report(struct list_head *n, struct list_head *p, struct list_head *q) { return __list_add_valid_or_report(n,p,q); }
bool rust_pa_list_del_report(struct list_head *p) { return __list_del_entry_valid_or_report(p); }
#endif
bool rust_pa_warn_get_pb_bit(bool c) { return WARN_ON_ONCE(c); }
bool rust_pa_warn_set_pb_bit(bool c) { return WARN_ON_ONCE(c); }
bool rust_pa_warn_clear_pb_bit(bool c) { return WARN_ON_ONCE(c); }
bool rust_pa_warn_highatomic_underflow(bool c) { return WARN_ON_ONCE(c); }
bool rust_pa_warn_highatomic_boundary(bool c) { return WARN_ON_ONCE(c); }
void rust_pa_warn_isolate_state(bool isolate) { VM_WARN_ONCE(1, "%s a pageblock that is already in that state", isolate ? "Isolate" : "Unisolate"); }
/* Native enum identity at public entry points; decisions are in Rust. */
bool get_pfnblock_bit(const struct page *p, unsigned long pfn, enum pageblock_bits bit) { return rust_pa_impl_get_pfnblock_bit(p,pfn,bit); }
enum migratetype get_pfnblock_migratetype(const struct page *p, unsigned long pfn) { return rust_pa_impl_get_pfnblock_migratetype(p,pfn); }
void set_pfnblock_bit(const struct page *p, unsigned long pfn, enum pageblock_bits bit) { rust_pa_impl_set_pfnblock_bit(p,pfn,bit); }
void clear_pfnblock_bit(const struct page *p, unsigned long pfn, enum pageblock_bits bit) { rust_pa_impl_clear_pfnblock_bit(p,pfn,bit); }
void __meminit init_pageblock_migratetype(struct page *p, enum migratetype mt, bool isolate) { rust_pa_impl_init_pageblock_migratetype(p,mt,isolate); }
void __meminit __free_pages_core(struct page *p, unsigned int order, enum meminit_context context) { rust_pa_impl_free_pages_core(p,order,context); }
enum fallback_result find_suitable_fallback(struct free_area *a, unsigned int order, int mt, bool claim, int *out) { return rust_pa_impl_find_suitable_fallback(a,order,mt,claim,out); }
bool rust_pa_mutex_trylock(struct mutex *m) { return mutex_trylock(m); }
void rust_pa_mutex_lock(struct mutex *m) { mutex_lock(m); }
void rust_pa_mutex_unlock(struct mutex *m) { mutex_unlock(m); }
