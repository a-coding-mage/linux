// SPDX-License-Identifier: GPL-2.0-only
// Only existing inline/macro/allocator and printk boundaries; policy algorithms
// remain in policydb-{core,symbols,wire}.rs. Never include policydb.c here.
#include "policydb-rust.h"
void *lupos_policydb_kmalloc(size_t s, gfp_t f) { return kmalloc(s, f); }
void *lupos_policydb_kzalloc(size_t s, gfp_t f) { return kzalloc(s, f); }
void *lupos_policydb_kcalloc(size_t n, size_t s, gfp_t f) { return kcalloc(n, s, f); }
void *lupos_policydb_kvcalloc(size_t n, size_t s, gfp_t f) { return kvcalloc(n, s, f); }
void *lupos_policydb_kmemdup(const void *p, size_t n, gfp_t f) { return kmemdup(p, n, f); }
char *lupos_policydb_kstrdup(const char *p, gfp_t f) { return kstrdup(p, f); }
int lupos_policydb_next_entry(void *b, struct policy_file *p, size_t n) { return next_entry(b, p, n); }
int lupos_policydb_put_entry(const void *b, size_t s, size_t n, struct policy_file *p) { return put_entry(b, s, n, p); }
int lupos_policydb_size_check(size_t s, size_t n, const struct policy_file *p) { return size_check(s, n, p); }
const char *lupos_policydb_sym_name(const struct policydb *p, unsigned int s, unsigned int b) { return sym_name(p, s, b); }
void lupos_policydb_context_destroy(struct context *c) { context_destroy(c); }
void lupos_policydb_ebitmap_init(struct ebitmap *e) { ebitmap_init(e); }
u32 lupos_policydb_ebitmap_start_positive(const struct ebitmap *e, struct ebitmap_node **n) { return ebitmap_start_positive(e, n); }
u32 lupos_policydb_ebitmap_next_positive(const struct ebitmap *e, struct ebitmap_node **n, u32 bit) { return ebitmap_next_positive(e, n, bit); }
int lupos_policydb_hashtab_insert(struct hashtab *h, void *k, void *d, struct hashtab_key_params p) { return hashtab_insert(h, k, d, p); }
void *lupos_policydb_hashtab_search(const struct hashtab *h, const void *k, struct hashtab_key_params p) { return hashtab_search(h, k, p); }
void lupos_policydb_hashtab_stat(struct hashtab *h, struct hashtab_info *i) { hashtab_stat(h, i); }
void lupos_policydb_avtab_hash_eval(struct avtab *h, const char *n) { avtab_hash_eval(h, n); }
u32 lupos_policydb_jhash_3words(u32 a, u32 b, u32 c, u32 i) { return jhash_3words(a, b, c, i); }
void lupos_policydb_cond_resched(void) { cond_resched(); }
int lupos_policydb_mls_level_eq(const struct mls_level *a, const struct mls_level *b) { return mls_level_eq(a, b); }
void lupos_policydb_error(const char *fmt, ...)
{
 va_list args;
 struct va_format vaf;
 va_start(args, fmt);
 vaf.fmt = fmt;
 vaf.va = &args;
 pr_err("%pV", &vaf);
 va_end(args);
}
void lupos_policydb_warn(const char *fmt, ...)
{
 va_list args;
 struct va_format vaf;
 va_start(args, fmt);
 vaf.fmt = fmt;
 vaf.va = &args;
 pr_warn("%pV", &vaf);
 va_end(args);
}
void lupos_policydb_debug(const char *fmt, ...)
{
 va_list args;
 struct va_format vaf;
 va_start(args, fmt);
 vaf.fmt = fmt;
 vaf.va = &args;
 pr_debug("%pV", &vaf);
 va_end(args);
}
