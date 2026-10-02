/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_WORKINGSET_BINDINGS_H
#define LUPOS_WORKINGSET_BINDINGS_H
#include <linux/memcontrol.h>
#include <linux/mm_inline.h>
#include <linux/writeback.h>
#include <linux/shmem_fs.h>
#include <linux/pagemap.h>
#include <linux/atomic.h>
#include <linux/module.h>
#include <linux/swap.h>
#include <linux/dax.h>
#include <linux/fs.h>
#include <linux/mm.h>
#include "swap_table.h"
#include "internal.h"

enum {
 RUST_WS_BITS_PER_LONG = BITS_PER_LONG,
 RUST_WS_BITS_PER_XA_VALUE = BITS_PER_XA_VALUE,
 RUST_WS_NODES_SHIFT = NODES_SHIFT,
 RUST_WS_MEM_CGROUP_ID_SHIFT = MEM_CGROUP_ID_SHIFT,
 RUST_WS_PAGE_SHIFT = PAGE_SHIFT,
 RUST_WS_XA_CHUNK_SHIFT = XA_CHUNK_SHIFT,
 RUST_WS_SHRINKER_NUMA_AWARE = SHRINKER_NUMA_AWARE,
 RUST_WS_SHRINKER_MEMCG_AWARE = SHRINKER_MEMCG_AWARE,
};
#ifdef CONFIG_LRU_GEN
enum {
 RUST_WS_LRU_GEN_WIDTH = LRU_GEN_WIDTH,
};
#endif
unsigned int rust_ws_swap_count_shift(void);
void *rust_ws_xa_mk_value(unsigned long value);
unsigned long rust_ws_xa_to_value(const void *entry);
struct pglist_data *rust_ws_node_data(int nid);
int rust_ws_node_id(struct pglist_data *pgdat);
unsigned long rust_ws_node_present_pages(int nid);
void rust_ws_atomic_long_add(long delta, atomic_long_t *value);
long rust_ws_atomic_long_read(const atomic_long_t *value);
unsigned long rust_ws_read_once_ulong(const unsigned long *value);
void rust_ws_rcu_read_lock(void);
void rust_ws_rcu_read_unlock(void);
struct pglist_data *rust_ws_folio_pgdat(struct folio *folio);
bool rust_ws_folio_is_file_lru(struct folio *folio);
unsigned long rust_ws_folio_nr_pages(struct folio *folio);
bool rust_ws_folio_test_workingset(struct folio *folio);
void rust_ws_folio_set_workingset(struct folio *folio);
void rust_ws_folio_set_active(struct folio *folio);
void rust_ws_assert_eviction_folio(struct folio *folio);
void rust_ws_assert_locked_folio(struct folio *folio);
bool rust_ws_lru_gen_enabled(void);
struct lruvec *rust_ws_parent_lruvec(struct lruvec *lruvec);
struct lruvec *rust_ws_mem_cgroup_lruvec(struct mem_cgroup *memcg, struct pglist_data *pgdat);
struct mem_cgroup *rust_ws_lruvec_memcg(struct lruvec *lruvec);
unsigned short rust_ws_mem_cgroup_private_id(struct mem_cgroup *memcg);
struct mem_cgroup *rust_ws_mem_cgroup_from_private_id(unsigned short id);
bool rust_ws_mem_cgroup_tryget(struct mem_cgroup *memcg);
void rust_ws_mem_cgroup_put(struct mem_cgroup *memcg);
bool rust_ws_mem_cgroup_disabled(void);
void rust_ws_mem_cgroup_flush_stats_ratelimited(struct mem_cgroup *memcg);
unsigned long rust_ws_mem_cgroup_get_nr_swap_pages(struct mem_cgroup *memcg);
unsigned long rust_ws_lruvec_page_state(struct lruvec *lruvec, enum node_stat_item item);
unsigned long rust_ws_lruvec_page_state_local(struct lruvec *lruvec, enum node_stat_item item);
void rust_ws_mod_lruvec_state(struct lruvec *lruvec, enum node_stat_item item, long delta);
struct mem_cgroup *rust_ws_get_mem_cgroup_from_folio(struct folio *folio);
bool rust_ws_folio_memcg_charged(struct folio *folio);
struct lruvec *rust_ws_folio_lruvec(struct folio *folio);
#ifdef CONFIG_LRU_GEN
struct mem_cgroup *rust_ws_folio_memcg(struct folio *folio);
int rust_ws_folio_lru_refs(struct folio *folio);
int rust_ws_lru_tier_from_refs(int refs, bool workingset);
int rust_ws_lru_hist_from_seq(unsigned long seq);
bool rust_ws_lru_gen_in_fault(void);
void rust_ws_set_mask_bits(unsigned long *flags, unsigned long mask, unsigned long bits);
unsigned long *rust_ws_folio_flags(struct folio *folio);
unsigned int rust_ws_lru_refs_width(void);
unsigned int rust_ws_lru_refs_pgoff(void);
unsigned long rust_ws_lru_refs_mask(void);
#endif
void rust_ws_assert_xa_locked(struct xarray *xa);
struct page *rust_ws_virt_to_page(const void *p);
void rust_ws_inc_node_page_state(struct page *page, enum node_stat_item item);
void rust_ws_dec_node_page_state(struct page *page, enum node_stat_item item);
bool rust_ws_list_empty(const struct list_head *head);
unsigned long rust_ws_list_lru_shrink_count(struct list_lru *lru, struct shrink_control *sc);
int rust_ws_list_lru_init(struct list_lru *lru, struct shrinker *shrinker, struct lock_class_key *key);
bool rust_ws_xa_trylock(struct xarray *xa);
void rust_ws_xa_unlock(struct xarray *xa);
void rust_ws_xa_unlock_irq(struct xarray *xa);
bool rust_ws_inode_trylock(struct inode *inode);
void rust_ws_inode_unlock(struct inode *inode);
void rust_ws_spin_unlock(spinlock_t *lock);
void rust_ws_spin_unlock_irq(spinlock_t *lock);
bool rust_ws_warn_no_values(bool condition);
bool rust_ws_warn_count_mismatch(bool condition);
void rust_ws_mod_lruvec_kmem_state(void *p, enum node_stat_item item, int delta);
bool rust_ws_mapping_shrinkable(struct address_space *mapping);
void rust_ws_cond_resched(void);
unsigned long rust_ws_totalram_pages(void);
int rust_ws_fls_long(unsigned long value);
void rust_ws_log_init(unsigned int timestamp_bits, unsigned int timestamp_bits_anon,
 unsigned int max_order, unsigned int file_bucket_order, unsigned int anon_bucket_order);
/* C's enum return is a distinct KCFI type. The thunk preserves that ABI. */
enum lru_status rust_ws_shadow_lru_isolate(struct list_head *item, struct list_lru_one *lru, void *arg);
unsigned long rust_ws_list_lru_shrink_walk_irq(struct list_lru *lru, struct shrink_control *sc,
 list_lru_walk_cb isolate, void *arg);
#endif
