// SPDX-License-Identifier: GPL-2.0-only
/* Only native-header/architecture primitives, exact static declarations, and
 * callback-table/late-init registration. All vmscan MGLRU algorithms are Rust. */
#include "vmscan_native_types.h"
#include <linux/mm_inline.h>
#include <linux/pagewalk.h>
#include <linux/mmu_notifier.h>
#include <linux/rculist_nulls.h>
#include <linux/ctype.h>
#include <linux/debugfs.h>
#include <linux/hash.h>
#include <linux/huge_mm.h>
#include <linux/sched/sysctl.h>
#include <linux/sched/mm.h>
#include <linux/slab.h>
#include <linux/uaccess.h>
#include "internal.h"
#include <trace/events/vmscan.h>
#include "vmscan_mglru_native.h"

#ifdef CONFIG_LRU_GEN
DEFINE_STATIC_KEY_FALSE(lru_switch);
#ifdef CONFIG_LRU_GEN_ENABLED
DEFINE_STATIC_KEY_ARRAY_TRUE(lru_gen_caps, NR_LRU_GEN_CAPS);
#else
DEFINE_STATIC_KEY_ARRAY_FALSE(lru_gen_caps, NR_LRU_GEN_CAPS);
#endif
static DEFINE_MUTEX(state_mutex);
static unsigned long lru_gen_min_ttl __read_mostly;
#ifdef CONFIG_LRU_GEN_WALKS_MMU
static struct lru_gen_mm_list mm_list = {
    .fifo = LIST_HEAD_INIT(mm_list.fifo),
    .lock = __SPIN_LOCK_UNLOCKED(mm_list.lock),
};
struct lru_gen_mm_list *rust_mglru_fallback_mm_list(void) { return &mm_list; }
struct mm_struct *rust_mglru_mm_from_list(struct list_head *p) { return list_entry(p, struct mm_struct, lru_gen.list); }
struct list_head *rust_mglru_mm_list_ptr(struct mm_struct *mm) { return &mm->lru_gen.list; }
unsigned long *rust_mglru_mm_bitmap_ptr(struct mm_struct *mm) { return &mm->lru_gen.bitmap; }
#ifdef CONFIG_MEMCG
struct mem_cgroup *rust_mglru_mm_memcg(struct mm_struct *mm) { return mm->lru_gen.memcg; }
void rust_mglru_mm_set_memcg(struct mm_struct *mm, struct mem_cgroup *memcg) { mm->lru_gen.memcg = memcg; }
struct task_struct *rust_mglru_mm_owner_protected(struct mm_struct *mm) { return rcu_dereference_protected(mm->owner, true); }
void rust_mglru_assert_task_alloc_lock(struct task_struct *task) { lockdep_assert_held(&task->alloc_lock); }
#endif
#endif
struct mutex *rust_mglru_state_mutex(void) { return &state_mutex; }
unsigned long *rust_mglru_min_ttl_ptr(void) { return &lru_gen_min_ttl; }
bool rust_mglru_get_cap(int cap) {
#ifdef CONFIG_LRU_GEN_ENABLED
#define MGLRU_CAP_VALUE(i) static_branch_likely(&lru_gen_caps[i])
#else
#define MGLRU_CAP_VALUE(i) static_branch_unlikely(&lru_gen_caps[i])
#endif
    /* Jump-label assembly requires a constant key address at each site. */
    switch (cap) {
    case LRU_GEN_CORE: return MGLRU_CAP_VALUE(LRU_GEN_CORE);
    case LRU_GEN_MM_WALK: return MGLRU_CAP_VALUE(LRU_GEN_MM_WALK);
    case LRU_GEN_NONLEAF_YOUNG: return MGLRU_CAP_VALUE(LRU_GEN_NONLEAF_YOUNG);
    default: WARN_ON_ONCE(1); return false;
    }
#undef MGLRU_CAP_VALUE
}
void rust_mglru_switch_enable_cpuslocked(void) { static_branch_enable_cpuslocked(&lru_switch); }
void rust_mglru_switch_disable_cpuslocked(void) { static_branch_disable_cpuslocked(&lru_switch); }
void rust_mglru_cap_set_cpuslocked(int cap, bool enabled) { if (enabled) static_branch_enable_cpuslocked(&lru_gen_caps[cap]); else static_branch_disable_cpuslocked(&lru_gen_caps[cap]); }
void rust_mglru_cap_set(int cap, bool enabled) { if (enabled) static_branch_enable(&lru_gen_caps[cap]); else static_branch_disable(&lru_gen_caps[cap]); }
bool rust_mglru_arch_has_hw_pte_young(void) { return arch_has_hw_pte_young(); }
bool rust_mglru_arch_has_hw_nonleaf_pmd_young(void) { return arch_has_hw_nonleaf_pmd_young(); }
u32 rust_mglru_hash_ptr(void *p, unsigned int bits) { return hash_ptr(p, bits); }
bool rust_mglru_test_bit(int n, const unsigned long *p) { return test_bit(n, p); }
void rust_mglru_set_bit(int n, unsigned long *p) { set_bit(n, p); }
void rust_mglru___set_bit(int n, unsigned long *p) { __set_bit(n, p); }
void rust_mglru_clear_bit(int n, unsigned long *p) { clear_bit(n, p); }
void rust_mglru_bitmap_zero(unsigned long *p, unsigned int n) { bitmap_zero(p, n); }
unsigned long rust_mglru_find_next_bit(const unsigned long *p, unsigned long n, unsigned long start) { return find_next_bit(p, n, start); }
void rust_mglru_assert_mm_list_lock(struct lru_gen_mm_list *p) { lockdep_assert_held(&p->lock); }
void rust_mglru_assert_lru_lock(struct lruvec *p) { lockdep_assert_held(&p->lru_lock); }
void rust_mglru_assert_ptl(spinlock_t *p) { lockdep_assert_held(p); }
void rust_mglru_set_mask_bits(unsigned long *p, unsigned long mask, unsigned long bits) { set_mask_bits(p, mask, bits); }
bool rust_mglru_try_cmpxchg_ulong(unsigned long *p, unsigned long *old, unsigned long new) { return try_cmpxchg(p, old, new); }
void rust_mglru_store_release_ulong(unsigned long *p, unsigned long v) { smp_store_release(p, v); }
long rust_mglru_read_long(const long *p) { return READ_ONCE(*p); }
void rust_mglru_write_long(long *p, long v) { WRITE_ONCE(*p, v); }
u8 rust_mglru_read_byte(const u8 *p) { return READ_ONCE(*p); }
void rust_mglru_write_byte(u8 *p, u8 v) { WRITE_ONCE(*p, v); }
unsigned long *rust_mglru_read_filter(unsigned long *const *p) { return READ_ONCE(*p); }
void rust_mglru_write_filter(unsigned long **p, unsigned long *v) { WRITE_ONCE(*p, v); }
unsigned long rust_mglru_jiffies(void) { return jiffies; }
bool rust_mglru_time_is_before_jiffies(unsigned long a) { return time_is_before_jiffies(a); }
unsigned int rust_mglru_jiffies_to_msecs(unsigned long a) { return jiffies_to_msecs(a); }
unsigned long rust_mglru_msecs_to_jiffies(unsigned int a) { return msecs_to_jiffies(a); }
int rust_mglru_fls_long(unsigned long a) { return fls_long(a); }
int rust_mglru_first_memory_node(void) { return first_memory_node; }
int rust_mglru_next_memory_node(int nid) { return next_memory_node(nid); }
int rust_mglru_first_node(void) { return first_node(node_possible_map); }
int rust_mglru_next_node(int nid) { return next_node(nid, node_possible_map); }
bool rust_mglru_node_memory(int nid) { return node_state(nid, N_MEMORY); }
unsigned long rust_mglru_vma_start(struct vm_area_struct *v) { return v->vm_start; }
unsigned long rust_mglru_vma_end(struct vm_area_struct *v) { return v->vm_end; }
bool rust_mglru_vma_locked_or_special(struct vm_area_struct *v) { return v->vm_flags & (VM_LOCKED | VM_SPECIAL); }
bool rust_mglru_vma_special(struct vm_area_struct *v) { return v->vm_flags & VM_SPECIAL; }
const vma_flags_t *rust_mglru_vma_flags_ptr(struct vm_area_struct *v) { return &v->flags; }
void rust_mglru_vma_iter_init(struct vma_iterator *vmi, struct mm_struct *mm, unsigned long addr) { vma_iter_init(vmi, mm, addr); }
struct vm_area_struct *rust_mglru_vma_next(struct vma_iterator *vmi) { return vma_next(vmi); }
unsigned long rust_mglru_pte_pfn(pte_t p) { return pte_pfn(p); }
bool rust_mglru_pte_present(pte_t p) { return pte_present(p); }
bool rust_mglru_pte_special(pte_t p) { return pte_special(p); }
bool rust_mglru_pte_young(pte_t p) { return pte_young(p); }
bool rust_mglru_pte_dirty(pte_t p) { return pte_dirty(p); }
bool rust_mglru_is_zero_pfn(unsigned long pfn) { return is_zero_pfn(pfn); }
unsigned long rust_mglru_pmd_pfn(pmd_t p) { return pmd_pfn(p); }
bool rust_mglru_pmd_present(pmd_t p) { return pmd_present(p); }
bool rust_mglru_is_huge_zero_pmd(pmd_t p) { return is_huge_zero_pmd(p); }
bool rust_mglru_pmd_young(pmd_t p) { return pmd_young(p); }
bool rust_mglru_pmd_dirty(pmd_t p) { return pmd_dirty(p); }
bool rust_mglru_pmd_trans_huge(pmd_t p) { return pmd_trans_huge(p); }
bool rust_mglru_pmd_same(pmd_t a, pmd_t b) { return pmd_same(a, b); }
bool rust_mglru_pfn_valid(unsigned long pfn) { return pfn_valid(pfn); }
struct folio *rust_mglru_pfn_folio(unsigned long pfn) { return pfn_folio(pfn); }
unsigned int rust_mglru_cache_line_size(void) { return cache_line_size(); }
pte_t *rust_mglru_pte_offset_map_rw_nolock(struct mm_struct *mm, pmd_t *pmd, unsigned long addr, pmd_t *val, spinlock_t **ptl) { return pte_offset_map_rw_nolock(mm, pmd, addr, val, ptl); }
void rust_mglru_pte_unmap(pte_t *p) { pte_unmap(p); }
void rust_mglru_pte_unmap_unlock(pte_t *p, spinlock_t *ptl) { pte_unmap_unlock(p, ptl); }
pmd_t rust_mglru_pmdp_get_lockless(pmd_t *p) { return pmdp_get_lockless(p); }
pte_t rust_mglru_ptep_get(pte_t *p) { return ptep_get(p); }
void rust_mglru_lazy_mmu_mode_enable(void) { lazy_mmu_mode_enable(); }
void rust_mglru_lazy_mmu_mode_disable(void) { lazy_mmu_mode_disable(); }
unsigned long rust_mglru_pte_index(unsigned long addr) { return pte_index(addr); }
unsigned long rust_mglru_pmd_index(unsigned long addr) { return pmd_index(addr); }
unsigned long rust_mglru_pud_index(unsigned long addr) { return pud_index(addr); }
unsigned int rust_mglru_folio_pte_batch(struct folio *f, pte_t *p, pte_t *v, unsigned int nr) { return folio_pte_batch_flags(f, NULL, p, v, nr, FPB_MERGE_YOUNG_DIRTY); }
bool rust_mglru_clear_young_ptes(struct vm_area_struct *vma, unsigned long addr, pte_t *p, unsigned int nr) { return test_and_clear_young_ptes_notify(vma, addr, p, nr); }
bool rust_mglru_pud_leaf(pud_t p) { return pud_leaf(p); }
bool rust_mglru_p4d_leaf(p4d_t p) { return p4d_leaf(p); }
bool rust_mglru_pud_present(pud_t p) { return pud_present(p); }
pud_t rust_mglru_pudp_get(pud_t *p) { return pudp_get(p); }
pmd_t *rust_mglru_pmd_offset(pud_t *p, unsigned long addr) { return pmd_offset(p, addr); }
pud_t *rust_mglru_pud_offset(p4d_t *p, unsigned long addr) { return pud_offset(p, addr); }
spinlock_t *rust_mglru_pmd_lockptr(struct mm_struct *mm, pmd_t *p) { return pmd_lockptr(mm, p); }
int rust_mglru_pmdp_test_and_clear_young(struct vm_area_struct *vma, unsigned long addr, pmd_t *p) { return pmdp_test_and_clear_young(vma, addr, p); }
bool rust_mglru_pmdp_test_and_clear_young_notify(struct vm_area_struct *vma, unsigned long addr, pmd_t *p) { return pmdp_test_and_clear_young_notify(vma, addr, p); }
unsigned long rust_mglru_pmd_addr_end(unsigned long addr, unsigned long end) { return pmd_addr_end(addr, end); }
unsigned long rust_mglru_pud_addr_end(unsigned long addr, unsigned long end) { return pud_addr_end(addr, end); }
struct lru_gen_mm_walk *rust_mglru_alloc_mm_walk(void) { return kzalloc_obj(struct lru_gen_mm_walk, __GFP_HIGH | __GFP_NOMEMALLOC | __GFP_NOWARN); }
unsigned long rust_mglru_spin_lock_irqsave(spinlock_t *lock) { unsigned long flags; spin_lock_irqsave(lock, flags); return flags; }
void rust_mglru_spin_unlock_irqrestore(spinlock_t *lock, unsigned long flags) { spin_unlock_irqrestore(lock, flags); }
bool rust_mglru_hlist_nulls_unhashed(const struct hlist_nulls_node *p) { return hlist_nulls_unhashed(p); }
void rust_mglru_hlist_nulls_del_rcu(struct hlist_nulls_node *p) { hlist_nulls_del_rcu(p); }
void rust_mglru_hlist_nulls_del_init_rcu(struct hlist_nulls_node *p) { hlist_nulls_del_init_rcu(p); }
void rust_mglru_hlist_nulls_add_head_rcu(struct hlist_nulls_node *p, struct hlist_nulls_head *h) { hlist_nulls_add_head_rcu(p, h); }
void rust_mglru_hlist_nulls_add_tail_rcu(struct hlist_nulls_node *p, struct hlist_nulls_head *h) { hlist_nulls_add_tail_rcu(p, h); }
struct hlist_nulls_node *rust_mglru_hlist_first_rcu(const struct hlist_nulls_head *h) { barrier(); return rcu_dereference_raw(hlist_nulls_first_rcu(h)); }
struct hlist_nulls_node *rust_mglru_hlist_next_rcu(struct hlist_nulls_node *p) { return rcu_dereference_raw(hlist_nulls_next_rcu(p)); }
bool rust_mglru_is_a_nulls(const struct hlist_nulls_node *p) { return is_a_nulls(p); }
unsigned long rust_mglru_get_nulls_value(const struct hlist_nulls_node *p) { return get_nulls_value(p); }
struct lru_gen_folio *rust_mglru_lrugen_from_list(struct hlist_nulls_node *p) { return hlist_nulls_entry(p, struct lru_gen_folio, list); }
struct lruvec *rust_mglru_lruvec_from_lrugen(struct lru_gen_folio *p) { return container_of(p, struct lruvec, lrugen); }
struct folio *rust_mglru_folio_from_lru(struct list_head *p) { return list_entry(p, struct folio, lru); }
int rust_mglru_numa_balancing_mode(void) { return sysctl_numa_balancing_mode; }
unsigned long rust_mglru_wmark_pages(struct zone *zone, int mark) { return wmark_pages(zone, mark); }
int rust_mglru_is_file_lru(int lru) { return is_file_lru(lru); }
bool rust_mglru_is_active_lru(int lru) { return is_active_lru(lru); }
int rust_mglru_tolower(int c) { return tolower(c); }
void *rust_mglru_err_ptr(long err) { return ERR_PTR(err); }
bool rust_mglru_is_err_or_null(const void *p) { return IS_ERR_OR_NULL(p); }
unsigned long rust_mglru_debugfs_get_aux_num(struct file *file) { return debugfs_get_aux_num(file); }
unsigned long rust_mglru_copy_from_user(void *dst, const void __user *src, unsigned long len) { return copy_from_user(dst, src, len); }
int rust_mglru_scan_cmd(const char *cur, char *cmd, u64 *memcg, unsigned int *nid, unsigned long *seq, int *end, char *swap, unsigned long *opt) { return sscanf(cur, "%c %llu %u %lu %n %4s %n %lu %n", cmd, memcg, nid, seq, end, swap, end, opt, end); }
void rust_mglru_init_hlist_nulls_head(struct hlist_nulls_head *head, unsigned long val) { INIT_HLIST_NULLS_HEAD(head, val); }
void rust_mglru_spin_lock_init(int site, spinlock_t *p) { switch (site) { case 0: spin_lock_init(p); break; case 1: spin_lock_init(p); break; default: WARN_ON_ONCE(1); } }
void rust_mglru_poison_lrugen_list(struct lruvec *v) { v->lrugen.list.next = LIST_POISON1; }
void rust_mglru_trace_isolate(int reclaim_idx, int order, unsigned long nr, unsigned long scanned, unsigned long skipped, unsigned long isolated, int lru) { trace_mm_vmscan_lru_isolate(reclaim_idx, order, nr, scanned, skipped, isolated, lru); }
void rust_mglru_trace_shrink_inactive(int nid, unsigned long scanned, unsigned long reclaimed, struct reclaim_stat *stat, int priority, int lru) { trace_mm_vmscan_lru_shrink_inactive(nid, scanned, reclaimed, stat, priority, lru); }

/* Every warning callsite has independent once state; the predicates remain in Rust. */
void rust_mglru_vmwarn(unsigned int site, bool condition) {
    switch (site) {
    case 1: VM_WARN_ON_ONCE(condition); break;
    case 2: VM_WARN_ON_ONCE(condition); break;
    case 3: VM_WARN_ON_ONCE(condition); break;
    case 4: VM_WARN_ON_ONCE(condition); break;
    case 5: VM_WARN_ON_ONCE(condition); break;
    case 6: VM_WARN_ON_ONCE(condition); break;
    case 7: VM_WARN_ON_ONCE(condition); break;
    case 8: VM_WARN_ON_ONCE(condition); break;
    case 9: VM_WARN_ON_ONCE(condition); break;
    case 10: VM_WARN_ON_ONCE(condition); break;
    case 11: VM_WARN_ON_ONCE(condition); break;
    case 12: VM_WARN_ON_ONCE(condition); break;
    case 13: VM_WARN_ON_ONCE(condition); break;
    case 14: VM_WARN_ON_ONCE(condition); break;
    case 15: VM_WARN_ON_ONCE(condition); break;
    case 16: VM_WARN_ON_ONCE(condition); break;
    case 17: VM_WARN_ON_ONCE(condition); break;
    case 18: VM_WARN_ON_ONCE(condition); break;
    case 19: VM_WARN_ON_ONCE(condition); break;
    case 20: VM_WARN_ON_ONCE(condition); break;
    case 21: VM_WARN_ON_ONCE(condition); break;
    case 22: VM_WARN_ON_ONCE(condition); break;
    case 23: VM_WARN_ON_ONCE(condition); break;
    case 24: VM_WARN_ON_ONCE(condition); break;
    case 25: VM_WARN_ON_ONCE(condition); break;
    case 26: VM_WARN_ON_ONCE(condition); break;
    case 27: VM_WARN_ON_ONCE(condition); break;
    case 28: VM_WARN_ON_ONCE(condition); break;
    case 29: VM_WARN_ON_ONCE(condition); break;
    case 30: VM_WARN_ON_ONCE(condition); break;
    case 31: VM_WARN_ON_ONCE(condition); break;
    case 32: VM_WARN_ON_ONCE(condition); break;
    case 33: VM_WARN_ON_ONCE(condition); break;
    case 34: VM_WARN_ON_ONCE(condition); break;
    case 35: VM_WARN_ON_ONCE(condition); break;
    case 36: VM_WARN_ON_ONCE(condition); break;
    case 37: VM_WARN_ON_ONCE(condition); break;
    case 38: VM_WARN_ON_ONCE(condition); break;
    case 39: VM_WARN_ON_ONCE(condition); break;
    case 40: VM_WARN_ON_ONCE(condition); break;
    case 41: VM_WARN_ON_ONCE(condition); break;
    default: WARN_ON_ONCE(1);
    }
}
bool rust_mglru_warn(unsigned int site, bool condition) {
    switch (site) {
    case 1: return WARN_ON_ONCE(condition);
    case 2: return WARN_ON_ONCE(condition);
    case 3: return WARN_ON_ONCE(condition);
    case 4: return WARN_ON_ONCE(condition);
    case 5: return WARN_ON_ONCE(condition);
    case 6: return WARN_ON_ONCE(condition);
    case 7: return WARN_ON_ONCE(condition);
    default: WARN_ON_ONCE(1); return condition;
    }
}
void rust_mglru_folio_warn(unsigned int site, bool condition, struct folio *folio) {
    switch (site) {
    case 1: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 2: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 3: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 4: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 5: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 6: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 7: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 8: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 9: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 10: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 11: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 12: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 13: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 14: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 15: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 16: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 17: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 18: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 19: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    case 20: VM_WARN_ON_ONCE_FOLIO(condition, folio); break;
    default: WARN_ON_ONCE(1);
    }
}

extern int rust_mglru_should_skip_vma(unsigned long, unsigned long, struct mm_walk *);
extern int rust_mglru_walk_pud_range(p4d_t *, unsigned long, unsigned long, struct mm_walk *);
static const struct mm_walk_ops mm_walk_ops = {
    .test_walk = rust_mglru_should_skip_vma,
    .p4d_entry = rust_mglru_walk_pud_range,
    .walk_lock = PGWALK_RDLOCK,
};
const struct mm_walk_ops *rust_mglru_mm_walk_ops(void) { return &mm_walk_ops; }
extern ssize_t rust_mglru_min_ttl_ms_show(struct kobject *, struct kobj_attribute *, char *);
extern ssize_t rust_mglru_min_ttl_ms_store(struct kobject *, struct kobj_attribute *, const char *, size_t);
extern ssize_t rust_mglru_enabled_show(struct kobject *, struct kobj_attribute *, char *);
extern ssize_t rust_mglru_enabled_store(struct kobject *, struct kobj_attribute *, const char *, size_t);
static struct kobj_attribute lru_gen_min_ttl_attr = __ATTR(min_ttl_ms, 0644, rust_mglru_min_ttl_ms_show, rust_mglru_min_ttl_ms_store);
static struct kobj_attribute lru_gen_enabled_attr = __ATTR(enabled, 0644, rust_mglru_enabled_show, rust_mglru_enabled_store);
static struct attribute *lru_gen_attrs[] = { &lru_gen_min_ttl_attr.attr, &lru_gen_enabled_attr.attr, NULL };
static const struct attribute_group lru_gen_attr_group = { .name = "lru_gen", .attrs = lru_gen_attrs };
extern void *rust_mglru_seq_start(struct seq_file *, loff_t *);
extern void rust_mglru_seq_stop(struct seq_file *, void *);
extern void *rust_mglru_seq_next(struct seq_file *, void *, loff_t *);
extern int rust_mglru_seq_show(struct seq_file *, void *);
extern int rust_mglru_seq_open(struct inode *, struct file *);
extern ssize_t rust_mglru_seq_write(struct file *, const char __user *, size_t, loff_t *);
static const struct seq_operations lru_gen_seq_ops = { .start = rust_mglru_seq_start, .stop = rust_mglru_seq_stop, .next = rust_mglru_seq_next, .show = rust_mglru_seq_show };
const struct seq_operations *rust_mglru_seq_ops(void) { return &lru_gen_seq_ops; }
static const struct file_operations lru_gen_rw_fops = { .open = rust_mglru_seq_open, .read = seq_read, .write = rust_mglru_seq_write, .llseek = seq_lseek, .release = seq_release };
static const struct file_operations lru_gen_ro_fops = { .open = rust_mglru_seq_open, .read = seq_read, .llseek = seq_lseek, .release = seq_release };
int rust_mglru_sysfs_create_group(void) { return sysfs_create_group(mm_kobj, &lru_gen_attr_group); }
void rust_mglru_report_sysfs_error(void) { pr_err("lru_gen: failed to create sysfs group\n"); }
void rust_mglru_debugfs_create(bool full) { if (full) debugfs_create_file_aux_num("lru_gen_full", 0444, NULL, NULL, true, &lru_gen_ro_fops); else debugfs_create_file_aux_num("lru_gen", 0644, NULL, NULL, false, &lru_gen_rw_fops); }
extern int rust_mglru_init(void);
static int __init init_lru_gen(void) { return rust_mglru_init(); }
late_initcall(init_lru_gen);
#endif /* CONFIG_LRU_GEN */
