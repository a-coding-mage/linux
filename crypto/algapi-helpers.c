// SPDX-License-Identifier: GPL-2.0-or-later
/* Only canonical inline/macro and declaration boundaries for algapi.rs. */
#include "algapi-bindings.h"

bool rust_algapi_list_empty(const struct list_head *p) { return list_empty(p); }
void rust_algapi_list_init(struct list_head *p) { INIT_LIST_HEAD(p); }
void rust_algapi_list_add(struct list_head *p, struct list_head *h) { list_add(p, h); }
void rust_algapi_list_add_tail(struct list_head *p, struct list_head *h) { list_add_tail(p, h); }
void rust_algapi_list_del(struct list_head *p) { list_del(p); }
void rust_algapi_list_del_init(struct list_head *p) { list_del_init(p); }
void rust_algapi_list_move(struct list_head *p, struct list_head *h) { list_move(p, h); }
void rust_algapi_hlist_del(struct hlist_node *p) { hlist_del(p); }
void rust_algapi_hlist_add(struct hlist_node *p, struct hlist_head *h) { hlist_add_head(p, h); }
void rust_algapi_ref_set(refcount_t *r, int n) { refcount_set(r, n); }
unsigned int rust_algapi_ref_read(const refcount_t *r) { return refcount_read(r); }
struct crypto_alg *rust_algapi_get(struct crypto_alg *alg) { return crypto_alg_get(alg); }
void rust_algapi_put(struct crypto_alg *alg) { crypto_alg_put(alg); }
bool rust_algapi_tmpl_get(struct crypto_template *tmpl) { return crypto_tmpl_get(tmpl); }
void rust_algapi_init_work(struct work_struct *w, work_func_t fn) { INIT_WORK(w, fn); }
void rust_algapi_schedule_work(struct work_struct *w) { schedule_work(w); }
bool rust_algapi_boot_test_finished(void) { return crypto_boot_test_finished(); }
void rust_algapi_set_boot_test_finished(void) { set_crypto_boot_test_finished(); }
int rust_algapi_fips_enabled(void) { return fips_enabled; }
bool rust_algapi_module_sig_ok(struct module *mod) { return module_sig_ok(mod); }
const char *rust_algapi_module_name(struct module *mod) { return module_name(mod); }
bool rust_algapi_warn_dup(bool condition) { return WARN_ON_ONCE(condition); }
bool rust_algapi_warn_inst(bool condition) { return WARN_ON_ONCE(condition); }
void rust_algapi_warn_ref(bool condition) { WARN_ON(condition); }
bool rust_algapi_warn_unregister(int ret, const char *name)
{
	return WARN(ret, "Algorithm %s is not registered", name);
}
void rust_algapi_unexpected_test(const char *name, int err)
{
	pr_err("alg: Unexpected test result for %s: %d\n", name, err);
}
void *rust_algapi_kmemdup(const void *p, size_t size) { return kmemdup(p, size, GFP_KERNEL); }
void rust_algapi_request_module(const char *name) { request_module("crypto-%s", name); }
unsigned int rust_algapi_ctx_alignment(void) { return crypto_tfm_ctx_alignment(); }
void *rust_algapi_create_tfm(struct crypto_alg *alg, const struct crypto_type *frontend)
{
	return crypto_create_tfm(alg, frontend);
}
void __init rust_algapi_init_proc(void) { crypto_init_proc(); }
void __exit rust_algapi_exit_proc(void) { crypto_exit_proc(); }

/* module_init's alias must name a definition in this C translation unit. */
static int __init crypto_algapi_init(void) { return rust_crypto_algapi_init(); }
static void __exit crypto_algapi_exit(void) { rust_crypto_algapi_exit(); }
late_initcall(crypto_algapi_init);
module_exit(crypto_algapi_exit);
MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("Cryptographic algorithms API");
MODULE_SOFTDEP("pre: cryptomgr");
