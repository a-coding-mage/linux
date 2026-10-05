/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_USER_ACCOUNTING_NATIVE_BINDINGS_H
#define LUPOS_USER_ACCOUNTING_NATIVE_BINDINGS_H

/* Inactive owner-local interface; configured native headers own all types. */
#include <linux/init.h>
#include <linux/list.h>
#include <linux/sched/user.h>
#include <linux/slab.h>
#include <linux/user_namespace.h>

#define USER_ACCOUNTING_UIDHASH_BITS (IS_ENABLED(CONFIG_BASE_SMALL) ? 3 : 7)
#define USER_ACCOUNTING_UIDHASH_SZ (1 << USER_ACCOUNTING_UIDHASH_BITS)

#ifdef __BINDGEN__
static const unsigned int LUPOS_USER_ACCOUNTING_UIDHASH_BITS =
	USER_ACCOUNTING_UIDHASH_BITS;
static const unsigned int LUPOS_USER_ACCOUNTING_UIDHASH_SZ =
	USER_ACCOUNTING_UIDHASH_SZ;
#endif

struct hlist_head *lupos_user_accounting_hash_bucket(size_t index);
void lupos_user_accounting_hash_init(struct hlist_head *head);
void lupos_user_accounting_hash_insert(struct user_struct *up,
				       struct hlist_head *head);
void lupos_user_accounting_hash_remove(struct user_struct *up);
struct user_struct *lupos_user_accounting_hash_first(struct hlist_head *head);
struct user_struct *lupos_user_accounting_hash_next(struct user_struct *up);
kuid_t lupos_user_accounting_uid(const struct user_struct *up);
void lupos_user_accounting_set_uid(struct user_struct *up, kuid_t uid);
uid_t lupos_user_accounting_kuid_val(kuid_t uid);
bool lupos_user_accounting_uid_eq(kuid_t left, kuid_t right);
void lupos_user_accounting_ref_inc(struct user_struct *up);
void lupos_user_accounting_ref_set(struct user_struct *up, int count);

/* Both save operations require writable unsigned-long output storage. */
void lupos_user_accounting_lock_irqsave(unsigned long *flags);
void lupos_user_accounting_unlock_irqrestore(unsigned long flags);
bool lupos_user_accounting_ref_dec_and_lock_irqsave(struct user_struct *up,
						  unsigned long *flags);
void lupos_user_accounting_lock_irq(void);
void lupos_user_accounting_unlock_irq(void);

#ifdef CONFIG_EPOLL
int lupos_user_accounting_epoll_init(struct user_struct *up);
void lupos_user_accounting_epoll_destroy(struct user_struct *up);
#endif
void lupos_user_accounting_ratelimit_init(struct user_struct *up);
void lupos_user_accounting_ratelimit_set_flags(struct user_struct *up);
struct kmem_cache *lupos_user_accounting_cache_create(void);
struct user_struct *lupos_user_accounting_cache_zalloc(struct kmem_cache *cache);
void lupos_user_accounting_cache_free(struct kmem_cache *cache,
				      struct user_struct *up);
struct user_struct *lupos_user_accounting_root_user(void);
kuid_t lupos_user_accounting_root_uid(void);
void __noreturn lupos_user_accounting_root_epoll_panic(void);

/* Direct call from native init metadata, not a Rust function-pointer adapter. */
int __init lupos_user_accounting_uid_cache_init(void);

#endif /* LUPOS_USER_ACCOUNTING_NATIVE_BINDINGS_H */
