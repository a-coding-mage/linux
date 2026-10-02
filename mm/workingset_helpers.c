// SPDX-License-Identifier: GPL-2.0
/* Header inline/macro and C declaration boundaries. Workingset policy, shadow
 * token construction, aging, accounting decisions, and shrinking live in Rust.
 */
#include "workingset_bindings.h"
unsigned int rust_ws_swap_count_shift(void)
{
 const unsigned int eviction_shift = (BITS_PER_LONG - BITS_PER_XA_VALUE) + 1 +
  NODES_SHIFT + MEM_CGROUP_ID_SHIFT;
 BUILD_BUG_ON(BITS_PER_LONG < eviction_shift);
#ifdef CONFIG_LRU_GEN
 BUILD_BUG_ON(LRU_GEN_WIDTH + LRU_REFS_WIDTH >
  BITS_PER_LONG - max(eviction_shift, eviction_shift + SWAP_COUNT_SHIFT));
#endif
 return SWAP_COUNT_SHIFT;
}
void *rust_ws_xa_mk_value(unsigned long value) { return xa_mk_value(value); }
unsigned long rust_ws_xa_to_value(const void *entry) { return xa_to_value(entry); }
struct pglist_data *rust_ws_node_data(int nid) { return NODE_DATA(nid); }
int rust_ws_node_id(struct pglist_data *pgdat) { return pgdat->node_id; }
unsigned long rust_ws_node_present_pages(int nid) { return node_present_pages(nid); }
void rust_ws_atomic_long_add(long delta, atomic_long_t *value) { atomic_long_add(delta, value); }
long rust_ws_atomic_long_read(const atomic_long_t *value) { return atomic_long_read(value); }
unsigned long rust_ws_read_once_ulong(const unsigned long *value) { return READ_ONCE(*value); }
void rust_ws_rcu_read_lock(void) { rcu_read_lock(); }
void rust_ws_rcu_read_unlock(void) { rcu_read_unlock(); }
struct pglist_data *rust_ws_folio_pgdat(struct folio *folio) { return folio_pgdat(folio); }
bool rust_ws_folio_is_file_lru(struct folio *folio) { return folio_is_file_lru(folio); }
unsigned long rust_ws_folio_nr_pages(struct folio *folio) { return folio_nr_pages(folio); }
bool rust_ws_folio_test_workingset(struct folio *folio) { return folio_test_workingset(folio); }
void rust_ws_folio_set_workingset(struct folio *folio) { folio_set_workingset(folio); }
void rust_ws_folio_set_active(struct folio *folio) { folio_set_active(folio); }
void rust_ws_assert_eviction_folio(struct folio *folio)
{
 VM_BUG_ON_FOLIO(folio_test_lru(folio), folio);
 VM_BUG_ON_FOLIO(folio_ref_count(folio), folio);
 VM_BUG_ON_FOLIO(!folio_test_locked(folio), folio);
}
void rust_ws_assert_locked_folio(struct folio *folio) { VM_BUG_ON_FOLIO(!folio_test_locked(folio), folio); }
bool rust_ws_lru_gen_enabled(void) { return lru_gen_enabled(); }
struct lruvec *rust_ws_parent_lruvec(struct lruvec *lruvec) { return parent_lruvec(lruvec); }
struct lruvec *rust_ws_mem_cgroup_lruvec(struct mem_cgroup *memcg, struct pglist_data *pgdat) { return mem_cgroup_lruvec(memcg, pgdat); }
struct mem_cgroup *rust_ws_lruvec_memcg(struct lruvec *lruvec) { return lruvec_memcg(lruvec); }
unsigned short rust_ws_mem_cgroup_private_id(struct mem_cgroup *memcg) { return mem_cgroup_private_id(memcg); }
struct mem_cgroup *rust_ws_mem_cgroup_from_private_id(unsigned short id) { return mem_cgroup_from_private_id(id); }
bool rust_ws_mem_cgroup_tryget(struct mem_cgroup *memcg) { return mem_cgroup_tryget(memcg); }
void rust_ws_mem_cgroup_put(struct mem_cgroup *memcg) { mem_cgroup_put(memcg); }
bool rust_ws_mem_cgroup_disabled(void) { return mem_cgroup_disabled(); }
void rust_ws_mem_cgroup_flush_stats_ratelimited(struct mem_cgroup *memcg) { mem_cgroup_flush_stats_ratelimited(memcg); }
unsigned long rust_ws_mem_cgroup_get_nr_swap_pages(struct mem_cgroup *memcg) { return mem_cgroup_get_nr_swap_pages(memcg); }
unsigned long rust_ws_lruvec_page_state(struct lruvec *lruvec, enum node_stat_item item) { return lruvec_page_state(lruvec, item); }
unsigned long rust_ws_lruvec_page_state_local(struct lruvec *lruvec, enum node_stat_item item) { return lruvec_page_state_local(lruvec, item); }
void rust_ws_mod_lruvec_state(struct lruvec *lruvec, enum node_stat_item item, long delta) { mod_lruvec_state(lruvec, item, delta); }
struct mem_cgroup *rust_ws_get_mem_cgroup_from_folio(struct folio *folio) { return get_mem_cgroup_from_folio(folio); }
bool rust_ws_folio_memcg_charged(struct folio *folio) { return folio_memcg_charged(folio); }
struct lruvec *rust_ws_folio_lruvec(struct folio *folio) { return folio_lruvec(folio); }
#ifdef CONFIG_LRU_GEN
struct mem_cgroup *rust_ws_folio_memcg(struct folio *folio) { return folio_memcg(folio); }
int rust_ws_folio_lru_refs(struct folio *folio) { return folio_lru_refs(folio); }
int rust_ws_lru_tier_from_refs(int refs, bool workingset) { return lru_tier_from_refs(refs, workingset); }
int rust_ws_lru_hist_from_seq(unsigned long seq) { return lru_hist_from_seq(seq); }
bool rust_ws_lru_gen_in_fault(void) { return lru_gen_in_fault(); }
void rust_ws_set_mask_bits(unsigned long *flags, unsigned long mask, unsigned long bits) { set_mask_bits(flags, mask, bits); }
unsigned long *rust_ws_folio_flags(struct folio *folio) { return &folio->flags.f; }
unsigned int rust_ws_lru_refs_width(void) { return LRU_REFS_WIDTH; }
unsigned int rust_ws_lru_refs_pgoff(void) { return LRU_REFS_PGOFF; }
unsigned long rust_ws_lru_refs_mask(void) { return LRU_REFS_MASK; }
#endif
void rust_ws_assert_xa_locked(struct xarray *xa) { lockdep_assert_held(&xa->xa_lock); }
struct page *rust_ws_virt_to_page(const void *p) { return virt_to_page(p); }
void rust_ws_inc_node_page_state(struct page *page, enum node_stat_item item) { __inc_node_page_state(page, item); }
void rust_ws_dec_node_page_state(struct page *page, enum node_stat_item item) { __dec_node_page_state(page, item); }
bool rust_ws_list_empty(const struct list_head *head) { return list_empty(head); }
unsigned long rust_ws_list_lru_shrink_count(struct list_lru *lru, struct shrink_control *sc) { return list_lru_shrink_count(lru, sc); }
int rust_ws_list_lru_init(struct list_lru *lru, struct shrinker *shrinker, struct lock_class_key *key) { return list_lru_init_memcg_key(lru, shrinker, key); }
bool rust_ws_xa_trylock(struct xarray *xa) { return xa_trylock(xa); }
void rust_ws_xa_unlock(struct xarray *xa) { xa_unlock(xa); }
void rust_ws_xa_unlock_irq(struct xarray *xa) { xa_unlock_irq(xa); }
bool rust_ws_inode_trylock(struct inode *inode) { return spin_trylock(&inode->i_lock); }
void rust_ws_inode_unlock(struct inode *inode) { spin_unlock(&inode->i_lock); }
void rust_ws_spin_unlock(spinlock_t *lock) { spin_unlock(lock); }
void rust_ws_spin_unlock_irq(spinlock_t *lock) { spin_unlock_irq(lock); }
bool rust_ws_warn_no_values(bool condition) { return WARN_ON_ONCE(condition); }
bool rust_ws_warn_count_mismatch(bool condition) { return WARN_ON_ONCE(condition); }
void rust_ws_mod_lruvec_kmem_state(void *p, enum node_stat_item item, int delta) { mod_lruvec_kmem_state(p, item, delta); }
bool rust_ws_mapping_shrinkable(struct address_space *mapping) { return mapping_shrinkable(mapping); }
void rust_ws_cond_resched(void) { cond_resched(); }
unsigned long rust_ws_totalram_pages(void) { return totalram_pages(); }
int rust_ws_fls_long(unsigned long value) { return fls_long(value); }
void rust_ws_log_init(unsigned int timestamp_bits, unsigned int timestamp_bits_anon,
 unsigned int max_order, unsigned int file_bucket_order, unsigned int anon_bucket_order)
{
 pr_info("workingset: timestamp_bits=%d (anon: %d) max_order=%d bucket_order=%u (anon: %d)\n",
  timestamp_bits, timestamp_bits_anon, max_order, file_bucket_order, anon_bucket_order);
}
extern unsigned int rust_workingset_shadow_lru_isolate(struct list_head *, struct list_lru_one *, void *);
enum lru_status rust_ws_shadow_lru_isolate(struct list_head *item, struct list_lru_one *lru, void *arg)
{
 return rust_workingset_shadow_lru_isolate(item, lru, arg);
}
unsigned long rust_ws_list_lru_shrink_walk_irq(struct list_lru *lru, struct shrink_control *sc,
 list_lru_walk_cb isolate, void *arg)
{
 return list_lru_shrink_walk_irq(lru, sc, isolate, arg);
}
extern int rust_workingset_init(void);
static int __init workingset_init(void) { return rust_workingset_init(); }
module_init(workingset_init);
