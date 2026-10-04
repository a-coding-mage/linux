// SPDX-License-Identifier: GPL-2.0-only
/* Included by page_alloc_helpers.c; native header ABI, atomic/compiler and
 * architecture primitives. No original page_alloc.c function body is included. */
struct task_struct *rust_pa_b_current(void) { return current; }
struct capture_control *rust_pa_b_task_capture(struct task_struct *t) {
#ifdef CONFIG_COMPACTION
 return t->capture_control;
#else
 return NULL;
#endif
}
bool rust_pa_b_is_migrate_cma(int mt) { return is_migrate_cma(mt); }
bool rust_pa_b_is_migrate_isolate(int mt) { return is_migrate_isolate(mt); }
bool rust_pa_b_mt_mergeable(int mt) { return migratetype_is_mergeable(mt); }
void rust_pa_b_trace_extfrag(struct page *p, int o, int f, int mt, int ft) { trace_mm_page_alloc_extfrag(p,o,f,mt,ft); }
void rust_pa_b_assert_zone_locked(struct zone *z) { lockdep_assert_held(&z->lock); }
void rust_pa_b_mod_zone_state(struct zone *z, enum zone_stat_item i, long n) { __mod_zone_page_state(z,i,n); }
void rust_pa_b_write_highatomic(struct zone *z, unsigned long n) { WRITE_ONCE(z->nr_free_highatomic,n); }
void rust_pa_b_warn_add_mt(struct page *p, int mt, int n) { VM_WARN_ONCE(get_pageblock_migratetype(p) != mt, "page type is %d, passed migratetype is %d (nr=%d)\n", get_pageblock_migratetype(p), mt, n); }
void rust_pa_b_warn_move_mt(struct page *p, int mt, int n) { VM_WARN_ONCE(get_pageblock_migratetype(p) != mt, "page type is %d, passed migratetype is %d (nr=%d)\n", get_pageblock_migratetype(p), mt, n); }
void rust_pa_b_warn_del_mt(struct page *p, int mt, int n) { VM_WARN_ONCE(get_pageblock_migratetype(p) != mt, "page type is %d, passed migratetype is %d (nr=%d)\n", get_pageblock_migratetype(p), mt, n); }
bool rust_pa_b_page_reported(struct page *p) { return page_reported(p); }
void rust_pa_b_clear_reported(struct page *p) { __ClearPageReported(p); }
void rust_pa_b_clear_buddy(struct page *p) { __ClearPageBuddy(p); }
struct page *rust_pa_b_page_from_buddy(struct list_head *p) { return list_entry(p,struct page,buddy_list); }
bool rust_pa_b_zone_initialized(struct zone *z) { return zone_is_initialized(z); }
unsigned long rust_pa_b_page_flags(struct page *p) { return p->flags.f; }
void rust_pa_b_write_page_flags(struct page *p, unsigned long f) { p->flags.f = f; }
void rust_pa_b_vm_bug(bool c) { VM_BUG_ON(c); }
void rust_pa_b_vm_bug_page(bool c, struct page *p) { VM_BUG_ON_PAGE(c,p); }
int rust_pa_b_get_pfnblock_mt(struct page *p, unsigned long pfn) { return get_pfnblock_migratetype(p,pfn); }
bool rust_pa_b_page_is_guard(struct page *p) { return page_is_guard(p); }
void rust_pa_b_clear_guard(struct zone *z, struct page *p, unsigned int o) { clear_page_guard(z,p,o); }
bool rust_pa_b_is_shuffle_order(unsigned int o) { return is_shuffle_order(o); }
bool rust_pa_b_shuffle_pick_tail(void) { return shuffle_pick_tail(); }
void rust_pa_b_reporting_notify(unsigned int o) { page_reporting_notify_free(o); }
int rust_pa_b_mapcount(struct page *p) { return atomic_read(&p->_mapcount); }
void *rust_pa_b_mapping(struct page *p) { return p->mapping; }
int rust_pa_b_page_ref_count(struct page *p) { return page_ref_count(p); }
unsigned long rust_pa_b_memcg_data(struct page *p) {
#ifdef CONFIG_MEMCG
 return p->memcg_data;
#else
 return 0;
#endif
}
bool rust_pa_b_pool_page_is_pp(struct page *p) { return page_pool_page_is_pp(p); }
bool rust_pa_b_check_pages_enabled(void) { return static_branch_unlikely(&check_pages_enabled); }
void rust_pa_b_assert_tail_poison_alignment(void) { BUILD_BUG_ON((unsigned long)LIST_POISON1 & 1); }
int rust_pa_b_folio_large_mapcount(struct folio *f) { return folio_large_mapcount(f); }
int rust_pa_b_folio_nr_mapped(struct folio *f) { return atomic_read(&f->_nr_pages_mapped); }
int rust_pa_b_folio_mm_mapcount(struct folio *f, int n) { return f->_mm_id_mapcount[n]; }
int rust_pa_b_folio_entire_mapcount(struct folio *f) { return atomic_read(&f->_entire_mapcount); }
int rust_pa_b_folio_pincount(struct folio *f) { return atomic_read(&f->_pincount); }
bool rust_pa_b_folio_deferred_empty(struct folio *f) { return list_empty(&f->_deferred_list); }
bool rust_pa_b_page_tail(struct page *p) { return PageTail(p); }
struct page *rust_pa_b_compound_head(struct page *p) { return compound_head(p); }
void rust_pa_b_clear_mapping(struct page *p) { p->mapping = NULL; }
void rust_pa_b_clear_compound_head(struct page *p) { clear_compound_head(p); }
bool rust_pa_b_deferred_pages_enabled(void) { return deferred_pages_enabled(); }
u8 rust_pa_b_page_kasan_tag(struct page *p) { return page_kasan_tag(p); }
void rust_pa_b_kasan_disable_current(void) { kasan_disable_current(); }
void rust_pa_b_kasan_enable_current(void) { kasan_enable_current(); }
void rust_pa_b_clear_low_pages(struct page *p, int n) { clear_pages(kasan_reset_tag(page_address(p)),n); }
void rust_pa_b_clear_highpage(struct page *p) { clear_highpage_kasan_tagged(p); }
#ifdef CONFIG_MEM_ALLOC_PROFILING
bool rust_pa_b_get_page_tag_ref(struct page *p, union codetag_ref *r, union pgtag_ref_handle *h) { return get_page_tag_ref(p,r,h); }
void rust_pa_b_set_codetag_empty(union codetag_ref *r) { set_codetag_empty(r); }
void rust_pa_b_update_page_tag_ref(union pgtag_ref_handle h, union codetag_ref *r) { update_page_tag_ref(h,r); }
void rust_pa_b_put_page_tag_ref(union pgtag_ref_handle h) { put_page_tag_ref(h); }
void rust_pa_b_alloc_tag_add(union codetag_ref *r, struct alloc_tag *t, unsigned long n) { alloc_tag_add(r,t,n); }
void rust_pa_b_alloc_tag_sub(union codetag_ref *r, unsigned long n) { alloc_tag_sub(r,n); }
struct alloc_tag *rust_pa_b_task_alloc_tag(struct task_struct *t) { return t->alloc_tag; }
void rust_pa_b_alloc_tag_add_early_pfn(unsigned long pfn, unsigned int flags) { alloc_tag_add_early_pfn(pfn,flags); }
void rust_pa_b_alloc_tag_inaccurate(struct alloc_tag *t) { alloc_tag_set_inaccurate(t); }
bool rust_pa_b_profiling_enabled(void) { return mem_alloc_profiling_enabled(); }
void rust_pa_b_tag_counter_sub(struct alloc_tag *t, unsigned long n) { this_cpu_sub(t->counters->bytes,n); }
#endif
bool rust_pa_b_want_init_on_free(void) { return want_init_on_free(); }
bool rust_pa_b_want_init_on_alloc(gfp_t g) { return want_init_on_alloc(g); }
bool rust_pa_b_page_compound(struct page *p) { return PageCompound(p); }
struct folio *rust_pa_b_page_folio(struct page *p) { return page_folio(p); }
void rust_pa_b_trace_free(struct page *p, unsigned int o) { trace_mm_page_free(p,o); }
void rust_pa_b_kmsan_free_page(struct page *p, unsigned int o) { kmsan_free_page(p,o); }
bool rust_pa_b_memcg_kmem_online(void) { return memcg_kmem_online(); }
bool rust_pa_b_page_memcg_kmem(struct page *p) { return PageMemcgKmem(p); }
void rust_pa_b_memcg_kmem_uncharge_page(struct page *p, unsigned int o) { __memcg_kmem_uncharge_page(p,o); }
bool rust_pa_b_folio_mlocked(struct folio *f) { return folio_test_mlocked(f); }
long rust_pa_b_folio_nr_pages(struct folio *f) { return folio_nr_pages(f); }
void rust_pa_b_folio_clear_mlocked(struct folio *f) { __folio_clear_mlocked(f); }
void rust_pa_b_zone_stat_mod_folio(struct folio *f, enum zone_stat_item i, long n) { zone_stat_mod_folio(f,i,n); }
void rust_pa_b_count_vm_events(enum vm_event_item i, long n) { count_vm_events(i,n); }
bool rust_pa_b_page_hwpoison(struct page *p) { return PageHWPoison(p); }
void rust_pa_b_reset_page_owner(struct page *p, unsigned int o) { reset_page_owner(p,o); }
void rust_pa_b_page_table_check_free(struct page *p, unsigned int o) { page_table_check_free(p,o); }
void rust_pa_b_clear_page_tag_ref(struct page *p) { clear_page_tag_ref(p); }
unsigned int rust_pa_b_compound_order(struct page *p) { return compound_order(p); }
void rust_pa_b_clear_folio_nr_pages(struct folio *f) {
#ifdef NR_PAGES_IN_LARGE_FOLIO
 f->_nr_pages = 0;
#endif
}
bool rust_pa_b_folio_anon(struct folio *f) { return folio_test_anon(f); }
void rust_pa_b_mod_mthp_anon(int o, int n) { mod_mthp_stat(o,MTHP_STAT_NR_ANON,n); }
void rust_pa_b_clear_folio_mapping(struct folio *f) { f->mapping = NULL; }
bool rust_pa_b_page_has_type(struct page *p) { return page_has_type(p); }
void rust_pa_b_reset_page_type(struct page *p) { p->page_type = UINT_MAX; }
void rust_pa_b_page_cpupid_reset(struct page *p) { page_cpupid_reset_last(p); }
bool rust_pa_b_page_highmem(struct page *p) { return PageHighMem(p); }
void *rust_pa_b_page_address(struct page *p) { return page_address(p); }
void rust_pa_b_debug_check_no_locks_freed(void *p, unsigned long n) { debug_check_no_locks_freed(p,n); }
void rust_pa_b_debug_check_no_obj_freed(void *p, unsigned long n) { debug_check_no_obj_freed(p,n); }
void rust_pa_b_kernel_poison_pages(struct page *p, int n) { kernel_poison_pages(p,n); }
void rust_pa_b_kernel_unpoison_pages(struct page *p, int n) { kernel_unpoison_pages(p,n); }
void rust_pa_b_kasan_poison_pages(struct page *p, unsigned int o, bool init) { kasan_poison_pages(p,o,init); }
bool rust_pa_b_kasan_integrated_init(void) { return kasan_has_integrated_init(); }
void rust_pa_b_arch_free_page(struct page *p, unsigned int o) { arch_free_page(p,o); }
void rust_pa_b_debug_unmap_pages(struct page *p, unsigned int n) { debug_pagealloc_unmap_pages(p,n); }
unsigned long rust_pa_b_zone_lock_irqsave(struct zone *z) { unsigned long f; spin_lock_irqsave(&z->lock,f); return f; }
void rust_pa_b_zone_unlock_irqrestore(struct zone *z, unsigned long f) { spin_unlock_irqrestore(&z->lock,f); }
bool rust_pa_b_zone_trylock_irqsave(struct zone *z, unsigned long *f) { return spin_trylock_irqsave(&z->lock,*f); }
struct page *rust_pa_b_page_from_pcp(struct list_head *p) { return list_entry(p,struct page,pcp_list); }
void rust_pa_b_trace_pcpu_drain(struct page *p, unsigned int o, int mt) { trace_mm_page_pcpu_drain(p,o,mt); }
void rust_pa_b_warn_split_alignment(unsigned long pfn, unsigned int o) { VM_WARN_ON_ONCE(!IS_ALIGNED(pfn,1 << o)); }
void rust_pa_b_warn_split_buddy(struct page *p) { VM_WARN_ON_ONCE(PageBuddy(p)); }
struct page *rust_pa_b_pfn_to_page(unsigned long pfn) { return pfn_to_page(pfn); }
struct llist_node *rust_pa_b_page_pcp_llist(struct page *p) { return &p->pcp_llist; }
void rust_pa_b_llist_add(struct llist_node *p, struct llist_head *h) { llist_add(p,h); }
bool rust_pa_b_llist_empty(struct llist_head *h) { return llist_empty(h); }
struct llist_node *rust_pa_b_llist_del_all(struct llist_head *h) { return llist_del_all(h); }
struct page *rust_pa_b_page_from_llist(struct llist_node *p) { return llist_entry(p,struct page,pcp_llist); }
bool rust_pa_b_can_spin_trylock(void) { return can_spin_trylock(); }
void rust_pa_b_count_vm_events_local(enum vm_event_item i, unsigned long n) { __count_vm_events(i,n); }
void rust_pa_b_warn_core_reserved(struct page *p) { VM_WARN_ON_ONCE(PageReserved(p)); }
void rust_pa_b_clear_offline(struct page *p) { __ClearPageOffline(p); }
void rust_pa_b_set_page_count(struct page *p, int n) { set_page_count(p,n); }
void rust_pa_b_clear_reserved(struct page *p) { __ClearPageReserved(p); }
void rust_pa_b_managed_pages_add(struct zone *z, long n) { atomic_long_add(n,&z->managed_pages); }
void rust_pa_b_accept_memory(phys_addr_t p, unsigned long n) { accept_memory(p,n); }
phys_addr_t rust_pa_b_page_to_phys(struct page *p) { return page_to_phys(p); }
bool rust_pa_b_pfn_valid(unsigned long pfn) { return pfn_valid(pfn); }
struct page *rust_pa_b_pfn_to_online_page(unsigned long pfn) { return pfn_to_online_page(pfn); }
unsigned long rust_pa_b_page_zone_id(struct page *p) { return page_zone_id(p); }
bool rust_pa_b_set_guard(struct zone *z, struct page *p, unsigned int o) { return set_page_guard(z,p,o); }
bool rust_pa_b_page_buddy(struct page *p) { return PageBuddy(p); }
bool rust_pa_b_kasan_hw_tags_enabled(void) { return kasan_hw_tags_enabled(); }
void rust_pa_b_arch_alloc_page(struct page *p, unsigned int o) { arch_alloc_page(p,o); }
void rust_pa_b_debug_map_pages(struct page *p, unsigned int n) { debug_pagealloc_map_pages(p,n); }
bool rust_pa_b_tag_clear_highpages(struct page *p, int n, bool init) { return tag_clear_highpages(p,n,init); }
bool rust_pa_b_kasan_unpoison_pages(struct page *p, unsigned int o, bool init) { return kasan_unpoison_pages(p,o,init); }
void rust_pa_b_page_kasan_tag_reset(struct page *p) { page_kasan_tag_reset(p); }
void rust_pa_b_set_page_owner(struct page *p, unsigned int o, gfp_t g) { set_page_owner(p,o,g); }
void rust_pa_b_page_table_check_alloc(struct page *p, unsigned int o) { page_table_check_alloc(p,o); }
void rust_pa_b_set_pfmemalloc(struct page *p) { set_page_pfmemalloc(p); }
void rust_pa_b_clear_pfmemalloc(struct page *p) { clear_page_pfmemalloc(p); }
void rust_pa_b_trace_zone_locked(struct page *p, unsigned int o, int mt, bool pcp) { trace_mm_page_alloc_zone_locked(p,o,mt,pcp); }

unsigned long rust_pa_b_page_flags_check_at_prep(void) { return PAGE_FLAGS_CHECK_AT_PREP; }
