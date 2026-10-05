// SPDX-License-Identifier: GPL-2.0-only
/* Native storage/primitive leaves. Hash traversal and lifetime policy stay Rust. */
#error "SOURCE ONLY HOLD: user-accounting native primitives are not admitted"

#include "user_accounting_native_bindings.h"
#include <linux/interrupt.h>
#include <linux/panic.h>

static struct hlist_head uidhash_table[USER_ACCOUNTING_UIDHASH_SZ];

/* free_uid may arrive with IRQs already disabled; preserve the saved state. */
static DEFINE_SPINLOCK(uidhash_lock);

struct hlist_head *lupos_user_accounting_hash_bucket(size_t index)
{
	return uidhash_table + index;
}

void lupos_user_accounting_hash_init(struct hlist_head *head)
{
	INIT_HLIST_HEAD(head);
}

void lupos_user_accounting_hash_insert(struct user_struct *up,
				       struct hlist_head *head)
{
	hlist_add_head(&up->uidhash_node, head);
}

void lupos_user_accounting_hash_remove(struct user_struct *up)
{
	hlist_del_init(&up->uidhash_node);
}

struct user_struct *lupos_user_accounting_hash_first(struct hlist_head *head)
{
	return hlist_entry_safe(head->first, struct user_struct, uidhash_node);
}

struct user_struct *lupos_user_accounting_hash_next(struct user_struct *up)
{
	return hlist_entry_safe(up->uidhash_node.next, struct user_struct,
				uidhash_node);
}

kuid_t lupos_user_accounting_uid(const struct user_struct *up)
{
	return up->uid;
}

void lupos_user_accounting_set_uid(struct user_struct *up, kuid_t uid)
{
	up->uid = uid;
}

uid_t lupos_user_accounting_kuid_val(kuid_t uid)
{
	return __kuid_val(uid);
}

bool lupos_user_accounting_uid_eq(kuid_t left, kuid_t right)
{
	return uid_eq(left, right);
}

void lupos_user_accounting_ref_inc(struct user_struct *up)
{
	refcount_inc(&up->__count);
}

void lupos_user_accounting_ref_set(struct user_struct *up, int count)
{
	refcount_set(&up->__count, count);
}

void lupos_user_accounting_lock_irqsave(unsigned long *flags)
	__acquires(&uidhash_lock)
{
	spin_lock_irqsave(&uidhash_lock, *flags);
}

void lupos_user_accounting_unlock_irqrestore(unsigned long flags)
	__releases(&uidhash_lock)
{
	spin_unlock_irqrestore(&uidhash_lock, flags);
}

bool lupos_user_accounting_ref_dec_and_lock_irqsave(struct user_struct *up,
						  unsigned long *flags)
	__cond_acquires(true, &uidhash_lock)
{
	return refcount_dec_and_lock_irqsave(&up->__count, &uidhash_lock, flags);
}

void lupos_user_accounting_lock_irq(void)
	__acquires(&uidhash_lock)
{
	spin_lock_irq(&uidhash_lock);
}

void lupos_user_accounting_unlock_irq(void)
	__releases(&uidhash_lock)
{
	spin_unlock_irq(&uidhash_lock);
}

#ifdef CONFIG_EPOLL
int lupos_user_accounting_epoll_init(struct user_struct *up)
{
	return percpu_counter_init(&up->epoll_watches, 0, GFP_KERNEL);
}

void lupos_user_accounting_epoll_destroy(struct user_struct *up)
{
	percpu_counter_destroy(&up->epoll_watches);
}
#endif

void lupos_user_accounting_ratelimit_init(struct user_struct *up)
{
	ratelimit_state_init(&up->ratelimit, HZ, 100);
}

void lupos_user_accounting_ratelimit_set_flags(struct user_struct *up)
{
	ratelimit_set_flags(&up->ratelimit, RATELIMIT_MSG_ON_RELEASE);
}

struct kmem_cache *lupos_user_accounting_cache_create(void)
{
	return kmem_cache_create("uid_cache", sizeof(struct user_struct), 0,
				 SLAB_HWCACHE_ALIGN | SLAB_PANIC, NULL);
}

struct user_struct *lupos_user_accounting_cache_zalloc(struct kmem_cache *cache)
{
	return kmem_cache_zalloc(cache, GFP_KERNEL);
}

void lupos_user_accounting_cache_free(struct kmem_cache *cache,
				      struct user_struct *up)
{
	kmem_cache_free(cache, up);
}

struct user_struct *lupos_user_accounting_root_user(void)
{
	return &root_user;
}

kuid_t lupos_user_accounting_root_uid(void)
{
	return GLOBAL_ROOT_UID;
}

void __noreturn lupos_user_accounting_root_epoll_panic(void)
{
	panic("root_user epoll percpu counter alloc failed");
}
