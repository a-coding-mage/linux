/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SELINUX_POLICYDB_RUST_H
#define LUPOS_SELINUX_POLICYDB_RUST_H
#ifdef MODULE
#define LUPOS_POLICYDB_RESTORE_MODULE
#undef MODULE
#endif
#include <linux/kernel.h>
#include <linux/sched.h>
#include <linux/slab.h>
#include <linux/string.h>
#include <linux/stringhash.h>
#include <linux/errno.h>
#include <linux/audit.h>
#include <linux/sort.h>
#include <linux/stdarg.h>
#include "flask.h"
#include "security.h"
#include "policydb.h"
#include "conditional.h"
#include "mls.h"
#include "services.h"

enum {
 LUPOS_POLICYDB_GFP_KERNEL = GFP_KERNEL,
 LUPOS_POLICYDB_GFP_NOWARN = __GFP_NOWARN,
#ifdef CONFIG_SECURITY_SELINUX_DEBUG
 LUPOS_POLICYDB_DEBUG = 1,
#else
 LUPOS_POLICYDB_DEBUG = 0,
#endif
 LUPOS_POLICYDB_SIZE_POLICYDB = sizeof(struct policydb),
 LUPOS_POLICYDB_ALIGN_POLICYDB = __alignof__(struct policydb),
 LUPOS_POLICYDB_OFFSET_SYMTAB = offsetof(struct policydb, symtab),
 LUPOS_POLICYDB_OFFSET_BOOLS = offsetof(struct policydb, bool_val_to_struct),
 LUPOS_POLICYDB_OFFSET_TYPE_ATTR = offsetof(struct policydb, type_attr_map_array),
 LUPOS_POLICYDB_OFFSET_LEN = offsetof(struct policydb, len),
 LUPOS_POLICYDB_SIZE_HASHTAB = sizeof(struct hashtab),
 LUPOS_POLICYDB_SIZE_EBITMAP = sizeof(struct ebitmap),
 LUPOS_POLICYDB_SIZE_AVTAB = sizeof(struct avtab),
 LUPOS_POLICYDB_SIZE_CONTEXT = sizeof(struct context),
 LUPOS_POLICYDB_OFFSET_CONTEXT_RANGE = offsetof(struct context, range),
 LUPOS_POLICYDB_SIZE_CLASS = sizeof(struct class_datum),
 LUPOS_POLICYDB_SIZE_TYPE = sizeof(struct type_datum),
 LUPOS_POLICYDB_SIZE_LEVEL = sizeof(struct level_datum),
 LUPOS_POLICYDB_SIZE_CAT = sizeof(struct cat_datum),
 LUPOS_POLICYDB_SIZE_OCONTEXT = sizeof(struct ocontext),
 LUPOS_POLICYDB_SIZE_FILE = sizeof(struct policy_file),
};
void *lupos_policydb_kmalloc(size_t size, gfp_t flags);
void *lupos_policydb_kzalloc(size_t size, gfp_t flags);
void *lupos_policydb_kcalloc(size_t n, size_t size, gfp_t flags);
void *lupos_policydb_kvcalloc(size_t n, size_t size, gfp_t flags);
void *lupos_policydb_kmemdup(const void *src, size_t len, gfp_t flags);
char *lupos_policydb_kstrdup(const char *src, gfp_t flags);
int lupos_policydb_next_entry(void *buf, struct policy_file *fp, size_t bytes);
int lupos_policydb_put_entry(const void *buf, size_t bytes, size_t n, struct policy_file *fp);
int lupos_policydb_size_check(size_t bytes, size_t n, const struct policy_file *fp);
const char *lupos_policydb_sym_name(const struct policydb *p, unsigned int sym, unsigned int bit);
void lupos_policydb_context_destroy(struct context *c);
void lupos_policydb_ebitmap_init(struct ebitmap *e);
u32 lupos_policydb_ebitmap_start_positive(const struct ebitmap *e, struct ebitmap_node **n);
u32 lupos_policydb_ebitmap_next_positive(const struct ebitmap *e, struct ebitmap_node **n, u32 bit);
int lupos_policydb_hashtab_insert(struct hashtab *h, void *key, void *datum, struct hashtab_key_params params);
void *lupos_policydb_hashtab_search(const struct hashtab *h, const void *key, struct hashtab_key_params params);
void lupos_policydb_hashtab_stat(struct hashtab *h, struct hashtab_info *info);
void lupos_policydb_avtab_hash_eval(struct avtab *h, const char *name);
u32 lupos_policydb_jhash_3words(u32 a, u32 b, u32 c, u32 init);
void lupos_policydb_cond_resched(void);
int lupos_policydb_mls_level_eq(const struct mls_level *a, const struct mls_level *b);
void lupos_policydb_error(const char *fmt, ...) __printf(1, 2);
void lupos_policydb_warn(const char *fmt, ...) __printf(1, 2);
void lupos_policydb_debug(const char *fmt, ...) __printf(1, 2);
#ifdef LUPOS_POLICYDB_RESTORE_MODULE
#define MODULE
#undef LUPOS_POLICYDB_RESTORE_MODULE
#endif
#endif
