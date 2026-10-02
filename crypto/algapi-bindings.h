/* SPDX-License-Identifier: GPL-2.0-or-later */
#ifndef LUPOS_CRYPTO_ALGAPI_BINDINGS_H
#define LUPOS_CRYPTO_ALGAPI_BINDINGS_H
#include "internal.h"
#include <linux/err.h>
#include <linux/errno.h>
#include <linux/fips.h>
#include <linux/rtnetlink.h>
#include <linux/slab.h>
#include <linux/string.h>

/* Configuration and inline/macro boundaries; all algapi control flow is Rust. */
enum {
 RUST_ALGAPI_SELFTESTS = IS_ENABLED(CONFIG_CRYPTO_SELFTESTS),
 RUST_ALGAPI_BUILTIN = IS_BUILTIN(CONFIG_CRYPTO_ALGAPI),
 RUST_ALGAPI_UNALIGNED = IS_ENABLED(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS),
 RUST_ALGAPI_MODULES = IS_ENABLED(CONFIG_MODULES),
};
bool rust_algapi_list_empty(const struct list_head *p);
void rust_algapi_list_init(struct list_head *p);
void rust_algapi_list_add(struct list_head *p, struct list_head *h);
void rust_algapi_list_add_tail(struct list_head *p, struct list_head *h);
void rust_algapi_list_del(struct list_head *p);
void rust_algapi_list_del_init(struct list_head *p);
void rust_algapi_list_move(struct list_head *p, struct list_head *h);
void rust_algapi_hlist_del(struct hlist_node *p);
void rust_algapi_hlist_add(struct hlist_node *p, struct hlist_head *h);
void rust_algapi_ref_set(refcount_t *r, int n);
unsigned int rust_algapi_ref_read(const refcount_t *r);
struct crypto_alg *rust_algapi_get(struct crypto_alg *alg);
void rust_algapi_put(struct crypto_alg *alg);
bool rust_algapi_tmpl_get(struct crypto_template *tmpl);
void rust_algapi_init_work(struct work_struct *w, work_func_t fn);
void rust_algapi_schedule_work(struct work_struct *w);
bool rust_algapi_boot_test_finished(void);
void rust_algapi_set_boot_test_finished(void);
int rust_algapi_fips_enabled(void);
bool rust_algapi_module_sig_ok(struct module *mod);
const char *rust_algapi_module_name(struct module *mod);
bool rust_algapi_warn_dup(bool condition);
bool rust_algapi_warn_inst(bool condition);
void rust_algapi_warn_ref(bool condition);
bool rust_algapi_warn_unregister(int ret, const char *name);
void rust_algapi_unexpected_test(const char *name, int err);
void *rust_algapi_kmemdup(const void *p, size_t size);
void rust_algapi_request_module(const char *name);
unsigned int rust_algapi_ctx_alignment(void);
void *rust_algapi_create_tfm(struct crypto_alg *alg, const struct crypto_type *frontend);
void rust_algapi_init_proc(void);
void rust_algapi_exit_proc(void);
int rust_crypto_algapi_init(void);
void rust_crypto_algapi_exit(void);
#endif
