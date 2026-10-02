// SPDX-License-Identifier: GPL-2.0-only
/* Real-header declarations for scatterlist Rust FFI primitives. */
#ifndef _RUST_SCATTERLIST_HELPERS_H
#define _RUST_SCATTERLIST_HELPERS_H
#include <linux/slab.h>
#include <linux/scatterlist.h>
#include <linux/highmem.h>
#include <linux/kmemleak.h>
#include <linux/uio.h>
#include <linux/folio_queue.h>
#include <linux/vmalloc.h>
struct scatterlist *rust_sg_next(struct scatterlist *s);
bool rust_sg_is_last(struct scatterlist *s);
struct scatterlist *rust_sg_chain_ptr(struct scatterlist *s);
void rust_sg_chain(struct scatterlist *s, unsigned n, struct scatterlist *t);
void rust_sg_chain_entry(struct scatterlist *s, struct scatterlist *t);
void rust_sg_mark_end(struct scatterlist *s);
void rust_sg_init_marker(struct scatterlist *s, unsigned n);
void rust_sg_set_page(struct scatterlist *s, struct page *p, unsigned len, unsigned off);
void rust_sg_set_buf(struct scatterlist *s, const void *buf, unsigned len);
struct page *rust_sg_page(struct scatterlist *s);
u64 rust_sg_phys(struct scatterlist *s);
unsigned rust_sg_dma_len(struct scatterlist *s);
unsigned long rust_sg_page_to_pfn(struct page *p);
struct page *rust_sg_pfn_to_page(unsigned long pfn);
bool rust_sg_same_pgmap(struct page *a, struct page *b);
struct scatterlist *rust_sg_alloc_page(gfp_t gfp);
void rust_sg_free_page(struct scatterlist *s);
void rust_sg_kmemleak_alloc(struct scatterlist *s, gfp_t gfp);
void rust_sg_kmemleak_free(struct scatterlist *s);
struct scatterlist *rust_sg_kmalloc(unsigned n, gfp_t gfp);
void rust_sg_kfree(struct scatterlist *s);
struct page *rust_sg_alloc_pages(gfp_t gfp, unsigned order);
void rust_sg_free_pages(struct page *p, unsigned order);
gfp_t rust_sg_without_dma(gfp_t gfp);
void rust_sg_bug_on(bool condition);
bool rust_sg_warn_on(bool condition);
bool rust_sg_warn_no_chain(bool condition);
bool rust_sg_warn_folioq_next(bool condition);
bool rust_sg_warn_folioq_count(bool condition);
void *rust_sg_kmap(struct page *p);
void *rust_sg_kmap_atomic(struct page *p);
void *rust_sg_kmap_local_page(struct page *p);
void rust_sg_kunmap(struct page *p);
void rust_sg_kunmap_atomic(void *addr);
void rust_sg_kunmap_local(void *addr);
void rust_sg_flush_dcache_page(struct page *p);
bool rust_sg_pagefault_disabled(void);
bool rust_sg_warn_atomic_pagefault(bool condition);
struct page *rust_sg_iter_page(struct sg_page_iter *p);
const struct folio_queue *rust_sg_folioq_next(const struct folio_queue *q);
unsigned rust_sg_folioq_slots(const struct folio_queue *q);
struct folio *rust_sg_folioq_folio(const struct folio_queue *q, unsigned slot);
size_t rust_sg_folioq_size(const struct folio_queue *q, unsigned slot);
struct page *rust_sg_folio_page(struct folio *f);
size_t rust_sg_folio_size(struct folio *f);
size_t rust_sg_offset_in_folio(struct folio *f, loff_t start);
bool rust_sg_is_hugetlb(struct folio *f);
bool rust_sg_is_vmalloc_addr(unsigned long addr);
struct page *rust_sg_vmalloc_to_page(unsigned long addr);
struct page *rust_sg_virt_to_page(unsigned long addr);
void rust_sg_xas_init(struct xa_state *s, struct xarray *xa, unsigned long index);
struct folio *rust_sg_xas_find(struct xa_state *s);
struct folio *rust_sg_xas_next(struct xa_state *s);
bool rust_sg_xas_retry(struct xa_state *s, struct folio *f);
bool rust_sg_xa_is_value(struct folio *f);
void rust_sg_rcu_lock(void);
void rust_sg_rcu_unlock(void);
void rust_sg_unsupported(u8 type);

#endif
