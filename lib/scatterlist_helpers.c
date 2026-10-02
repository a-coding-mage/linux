// SPDX-License-Identifier: GPL-2.0-only
/* Only header primitives and allocation/mapping boundaries.  Scatterlist
 * algorithms, including every extraction loop, live in scatterlist.rs. */
#include <linux/slab.h>
#include <linux/scatterlist.h>
#include <linux/highmem.h>
#include <linux/kmemleak.h>
#include <linux/uio.h>
#include <linux/folio_queue.h>
#include <linux/vmalloc.h>
#include "scatterlist_helpers.h"
struct scatterlist *rust_sg_next(struct scatterlist *s) { return sg_next(s); }
bool rust_sg_is_last(struct scatterlist *s) { return sg_is_last(s); }
struct scatterlist *rust_sg_chain_ptr(struct scatterlist *s) { return sg_chain_ptr(s); }
void rust_sg_chain(struct scatterlist *s, unsigned n, struct scatterlist *t) { sg_chain(s, n, t); }
void rust_sg_chain_entry(struct scatterlist *s, struct scatterlist *t) { __sg_chain(s, t); }
void rust_sg_mark_end(struct scatterlist *s) { sg_mark_end(s); }
void rust_sg_init_marker(struct scatterlist *s, unsigned n) { sg_init_marker(s, n); }
void rust_sg_set_page(struct scatterlist *s, struct page *p, unsigned len, unsigned off) { sg_set_page(s, p, len, off); }
void rust_sg_set_buf(struct scatterlist *s, const void *buf, unsigned len) { sg_set_buf(s, buf, len); }
struct page *rust_sg_page(struct scatterlist *s) { return sg_page(s); }
u64 rust_sg_phys(struct scatterlist *s) { return sg_phys(s); }
unsigned rust_sg_dma_len(struct scatterlist *s) { return sg_dma_len(s); }
unsigned long rust_sg_page_to_pfn(struct page *p) { return page_to_pfn(p); }
struct page *rust_sg_pfn_to_page(unsigned long pfn) { return pfn_to_page(pfn); }
bool rust_sg_same_pgmap(struct page *a, struct page *b) { return zone_device_pages_have_same_pgmap(a, b); }
struct scatterlist *rust_sg_alloc_page(gfp_t gfp) { return (void *)__get_free_page(gfp); }
void rust_sg_free_page(struct scatterlist *s) { free_page((unsigned long)s); }
void rust_sg_kmemleak_alloc(struct scatterlist *s, gfp_t gfp) { kmemleak_alloc(s, PAGE_SIZE, 1, gfp); }
void rust_sg_kmemleak_free(struct scatterlist *s) { kmemleak_free(s); }
struct scatterlist *rust_sg_kmalloc(unsigned n, gfp_t gfp) { return kmalloc_objs(struct scatterlist, n, gfp); }
void rust_sg_kfree(struct scatterlist *s) { kfree(s); }
struct page *rust_sg_alloc_pages(gfp_t gfp, unsigned order) { return alloc_pages(gfp, order); }
void rust_sg_free_pages(struct page *p, unsigned order) { __free_pages(p, order); }
gfp_t rust_sg_without_dma(gfp_t gfp) { return gfp & ~GFP_DMA; }
void rust_sg_bug_on(bool condition) { BUG_ON(condition); }
bool rust_sg_warn_on(bool condition) { return WARN_ON(condition); }
/* Keep the original WARN_ON_ONCE sites independent. */
bool rust_sg_warn_no_chain(bool condition) { return WARN_ON_ONCE(condition); }
bool rust_sg_warn_folioq_next(bool condition) { return WARN_ON_ONCE(condition); }
bool rust_sg_warn_folioq_count(bool condition) { return WARN_ON_ONCE(condition); }
void *rust_sg_kmap(struct page *p) { return kmap(p); }
void *rust_sg_kmap_atomic(struct page *p) { return kmap_atomic(p); }
void *rust_sg_kmap_local_page(struct page *p) { return kmap_local_page(p); }
void rust_sg_kunmap(struct page *p) { kunmap(p); }
void rust_sg_kunmap_atomic(void *addr) { kunmap_atomic(addr); }
void rust_sg_kunmap_local(void *addr) { kunmap_local(addr); }
void rust_sg_flush_dcache_page(struct page *p) { flush_dcache_page(p); }
bool rust_sg_pagefault_disabled(void) { return pagefault_disabled(); }
bool rust_sg_warn_atomic_pagefault(bool condition) { return WARN_ON_ONCE(condition); }
struct page *rust_sg_iter_page(struct sg_page_iter *p) { return sg_page_iter_page(p); }
const struct folio_queue *rust_sg_folioq_next(const struct folio_queue *q) { return q->next; }
unsigned rust_sg_folioq_slots(const struct folio_queue *q) { return folioq_nr_slots(q); }
struct folio *rust_sg_folioq_folio(const struct folio_queue *q, unsigned slot) { return folioq_folio(q, slot); }
size_t rust_sg_folioq_size(const struct folio_queue *q, unsigned slot) { return folioq_folio_size(q, slot); }
struct page *rust_sg_folio_page(struct folio *f) { return folio_page(f, 0); }
size_t rust_sg_folio_size(struct folio *f) { return folio_size(f); }
size_t rust_sg_offset_in_folio(struct folio *f, loff_t start) { return offset_in_folio(f, start); }
bool rust_sg_is_hugetlb(struct folio *f) { return folio_test_hugetlb(f); }
bool rust_sg_is_vmalloc_addr(unsigned long addr) { return is_vmalloc_or_module_addr((void *)addr); }
struct page *rust_sg_vmalloc_to_page(unsigned long addr) { return vmalloc_to_page((void *)addr); }
struct page *rust_sg_virt_to_page(unsigned long addr) { return virt_to_page((void *)addr); }
void rust_sg_xas_init(struct xa_state *s, struct xarray *xa, unsigned long index) { *s = (struct xa_state)__XA_STATE(xa, index, 0, 0); }
struct folio *rust_sg_xas_find(struct xa_state *s) { return xas_find(s, ULONG_MAX); }
struct folio *rust_sg_xas_next(struct xa_state *s) { return xas_next_entry(s, ULONG_MAX); }
bool rust_sg_xas_retry(struct xa_state *s, struct folio *f) { return xas_retry(s, f); }
bool rust_sg_xa_is_value(struct folio *f) { return xa_is_value(f); }
void rust_sg_rcu_lock(void) { rcu_read_lock(); }
void rust_sg_rcu_unlock(void) { rcu_read_unlock(); }
void rust_sg_unsupported(u8 type) { pr_err("extract_iter_to_sg(%u) unsupported\n", type); WARN_ON_ONCE(1); }
