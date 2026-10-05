// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Native compiler, field, allocation, RCU and header-inline boundaries only.
 * All kernel/cred.c policy, branching, loops and lifetime ordering stay in the
 * continued cred.rs bodies. Never include or call the old cred.c owner body.
 */
#include "cred_native_bindings.h"
#include <linux/export.h>

#error "SOURCE ONLY HOLD: credential Rust owner is not admitted"

struct cred *lupos_cred_from_rcu(struct rcu_head *rcu)
{
	return container_of(rcu, struct cred, rcu);
}
struct rcu_head *lupos_cred_rcu(struct cred *cred) { return &cred->rcu; }
long lupos_cred_usage_read(const struct cred *cred)
{
	return atomic_long_read(&cred->usage);
}
void lupos_cred_usage_set(struct cred *cred, long value)
{
	atomic_long_set(&cred->usage, value);
}
int lupos_cred_non_rcu(const struct cred *cred) { return cred->non_rcu; }
void lupos_cred_set_non_rcu(struct cred *cred, int value) { cred->non_rcu = value; }
void lupos_cred_copy(struct cred *new, const struct cred *old)
{
	memcpy(new, old, sizeof(struct cred));
}
void lupos_cred_assign(struct cred *new, const struct cred *old) { *new = *old; }
/* Distinct native sites, in original cred.c order; attribution is relocated. */
void lupos_cred_bug_put_usage(struct cred *cred)
{
	BUG_ON(atomic_long_read(&cred->usage) != 0);
}
void lupos_cred_bug_put_current_cred(struct cred *cred)
{
	BUG_ON(cred == current->cred);
}
void lupos_cred_bug_put_current_real_cred(struct cred *cred)
{
	BUG_ON(cred == current->real_cred);
}
void lupos_cred_bug_task_cred_null(const struct cred *cred)
{
	BUG_ON(!cred);
}
void lupos_cred_bug_commit_overridden(struct task_struct *task,
				    const struct cred *old)
{
	BUG_ON(task->cred != old);
}
void lupos_cred_bug_commit_usage(struct cred *new)
{
	BUG_ON(atomic_long_read(&new->usage) < 1);
}
void lupos_cred_bug_abort_usage(struct cred *new)
{
	BUG_ON(atomic_long_read(&new->usage) < 1);
}
void __noreturn lupos_cred_bad_usage(const struct cred *cred)
{
	panic("CRED: put_cred_rcu() sees %p with usage %ld\n",
	      cred, atomic_long_read(&cred->usage));
}
bool lupos_cred_warn_null_daemon(struct task_struct *daemon)
{
	return WARN_ON_ONCE(!daemon);
}
void lupos_cred_rcu_read_lock(void) { rcu_read_lock(); }
void lupos_cred_rcu_read_unlock(void) { rcu_read_unlock(); }
const struct cred *lupos_cred_task_rcu(struct task_struct *task) { return __task_cred(task); }
const struct cred *lupos_cred_get_rcu(const struct cred *cred) { return get_cred_rcu(cred); }
const struct cred *lupos_cred_get(const struct cred *cred) { return get_cred(cred); }
const struct cred *lupos_cred_get_many(const struct cred *cred, int nr)
{
	return get_cred_many(cred, nr);
}
void lupos_cred_put(const struct cred *cred) { put_cred(cred); }
void lupos_cred_put_many(const struct cred *cred, int nr) { put_cred_many(cred, nr); }
/* Signature-only adapter: all destruction decisions and ordering stay Rust. */
static void lupos_cred_put_rcu_callback(struct rcu_head *rcu)
{
	lupos_cred_put_rcu_body(rcu);
}
void lupos_cred_call_rcu(struct rcu_head *rcu)
{
	call_rcu(rcu, lupos_cred_put_rcu_callback);
}

/* These access native fields, never a duplicated or projected object layout. */
#define CRED_GET(type, name) \
	type lupos_cred_##name(const struct cred *cred) { return cred->name; }
#define CRED_SET(type, name) \
	void lupos_cred_set_##name(struct cred *cred, type value) { cred->name = value; }
CRED_GET(kuid_t, uid)
CRED_GET(kuid_t, suid)
CRED_GET(kuid_t, euid)
CRED_GET(kuid_t, fsuid)
CRED_GET(kgid_t, gid)
CRED_GET(kgid_t, sgid)
CRED_GET(kgid_t, egid)
CRED_GET(kgid_t, fsgid)
CRED_SET(kuid_t, suid)
CRED_SET(kuid_t, fsuid)
CRED_SET(kgid_t, sgid)
CRED_SET(kgid_t, fsgid)
CRED_GET(kernel_cap_t, cap_permitted)
CRED_GET(struct user_struct *, user)
CRED_GET(struct user_namespace *, user_ns)
CRED_GET(struct ucounts *, ucounts)
CRED_SET(struct ucounts *, ucounts)
CRED_GET(struct group_info *, group_info)

bool lupos_cred_uid_eq(kuid_t a, kuid_t b) { return uid_eq(a, b); }
bool lupos_cred_uid_lt(kuid_t a, kuid_t b) { return uid_lt(a, b); }
bool lupos_cred_uid_gt(kuid_t a, kuid_t b) { return uid_gt(a, b); }
bool lupos_cred_uid_valid(kuid_t uid) { return uid_valid(uid); }
bool lupos_cred_gid_eq(kgid_t a, kgid_t b) { return gid_eq(a, b); }
bool lupos_cred_gid_lt(kgid_t a, kgid_t b) { return gid_lt(a, b); }
bool lupos_cred_gid_gt(kgid_t a, kgid_t b) { return gid_gt(a, b); }
bool lupos_cred_gid_valid(kgid_t gid) { return gid_valid(gid); }
bool lupos_cred_cap_issubset(kernel_cap_t subset, kernel_cap_t set)
{
	return cap_issubset(subset, set);
}
struct user_struct *lupos_cred_get_uid(struct user_struct *user) { return get_uid(user); }
void lupos_cred_free_uid(struct user_struct *user) { free_uid(user); }
struct user_namespace *lupos_cred_get_user_ns(struct user_namespace *ns)
{
	return get_user_ns(ns);
}
void lupos_cred_put_user_ns(struct user_namespace *ns) { put_user_ns(ns); }
struct group_info *lupos_cred_get_group_info(struct group_info *info)
{
	return get_group_info(info);
}
void lupos_cred_put_group_info(struct group_info *info) { put_group_info(info); }
struct ucounts *lupos_cred_get_ucounts(struct ucounts *ucounts) { return get_ucounts(ucounts); }
int lupos_cred_create_user_ns(struct cred *new) { return create_user_ns(new); }
struct user_namespace *lupos_cred_init_user_ns(void) { return &init_user_ns; }
struct user_namespace *lupos_cred_ns_parent(const struct user_namespace *ns) { return ns->parent; }
kuid_t lupos_cred_ns_owner(const struct user_namespace *ns) { return ns->owner; }
struct user_namespace *lupos_cred_ucounts_ns(const struct ucounts *ucounts) { return ucounts->ns; }
kuid_t lupos_cred_ucounts_uid(const struct ucounts *ucounts) { return ucounts->uid; }
int lupos_cred_ngroups(const struct group_info *info) { return info->ngroups; }
kgid_t lupos_cred_group_gid(const struct group_info *info, int index) { return info->gid[index]; }
kuid_t lupos_cred_inode_uid(const struct inode *inode) { return inode->i_uid; }
kgid_t lupos_cred_inode_gid(const struct inode *inode) { return inode->i_gid; }

struct task_struct *lupos_cred_current(void) { return current; }
const struct cred *lupos_cred_task_cred(const struct task_struct *task) { return task->cred; }
const struct cred *lupos_cred_task_real_cred(const struct task_struct *task) { return task->real_cred; }
void lupos_cred_set_task_cred(struct task_struct *task, const struct cred *cred) { task->cred = cred; }
void lupos_cred_set_task_real_cred(struct task_struct *task, const struct cred *cred) { task->real_cred = cred; }
void lupos_cred_publish_task_cred(struct task_struct *task, const struct cred *cred)
{
	rcu_assign_pointer(task->cred, cred);
}
void lupos_cred_publish_task_real_cred(struct task_struct *task, const struct cred *cred)
{
	rcu_assign_pointer(task->real_cred, cred);
}
struct ucounts *lupos_cred_task_ucounts(struct task_struct *task) { return task_ucounts(task); }
bool lupos_cred_task_has_mm(const struct task_struct *task) { return task->mm != NULL; }
void lupos_cred_clear_pdeath_signal(struct task_struct *task) { task->pdeath_signal = 0; }
void lupos_cred_set_dumpable(void) { task_exec_state_set_dumpable(suid_dumpable); }
void lupos_cred_smp_wmb(void) { smp_wmb(); }
void lupos_cred_proc_id_connector(struct task_struct *task, int which_id)
{
	proc_id_connector(task, which_id);
}
void lupos_cred_key_fsuid_changed(struct cred *new) { key_fsuid_changed(new); }
void lupos_cred_key_fsgid_changed(struct cred *new) { key_fsgid_changed(new); }

/* Signature/metadata only; the cache allocation and store stay in Rust. */
void __init cred_init(void)
{
	lupos_cred_init_body();
}

struct kmem_cache * __init lupos_cred_cache_create(void)
{
	return KMEM_CACHE(cred, SLAB_HWCACHE_ALIGN | SLAB_PANIC | SLAB_ACCOUNT);
}
struct cred *lupos_cred_cache_alloc(struct kmem_cache *cache, gfp_t flags)
{
	return kmem_cache_alloc(cache, flags);
}
struct cred *lupos_cred_cache_zalloc(struct kmem_cache *cache, gfp_t flags)
{
	return kmem_cache_zalloc(cache, flags);
}
void lupos_cred_cache_free(struct kmem_cache *cache, struct cred *cred)
{
	kmem_cache_free(cache, cred);
}
int lupos_cred_security_alloc_blank(struct cred *new, gfp_t flags)
{
	return security_cred_alloc_blank(new, flags);
}
void lupos_cred_security_free(struct cred *cred) { security_cred_free(cred); }
int lupos_cred_security_prepare(struct cred *new, const struct cred *old, gfp_t flags)
{
	return security_prepare_creds(new, old, flags);
}
int lupos_cred_security_act_as(struct cred *new, u32 secid)
{
	return security_kernel_act_as(new, secid);
}
int lupos_cred_security_create_files_as(struct cred *new, struct inode *inode)
{
	return security_kernel_create_files_as(new, inode);
}

#ifdef CONFIG_SECURITY
void lupos_cred_clear_security(struct cred *cred) { cred->security = NULL; }
#endif
#ifdef CONFIG_KEYS
CRED_GET(struct key *, session_keyring)
CRED_GET(struct key *, process_keyring)
CRED_GET(struct key *, thread_keyring)
CRED_GET(struct key *, request_key_auth)
CRED_SET(struct key *, session_keyring)
CRED_SET(struct key *, process_keyring)
CRED_SET(struct key *, thread_keyring)
CRED_SET(struct key *, request_key_auth)
CRED_SET(unsigned char, jit_keyring)
struct key *lupos_cred_key_get(struct key *key) { return key_get(key); }
#endif
#ifdef CONFIG_KEYS_REQUEST_CACHE
struct key *lupos_cred_cached_requested_key(const struct task_struct *task)
{
	return task->cached_requested_key;
}
void lupos_cred_clear_cached_requested_key(struct task_struct *task)
{
	task->cached_requested_key = NULL;
}
#endif
#undef CRED_GET
#undef CRED_SET

/* Preserve the original exported owner symbols when future admission occurs. */
EXPORT_SYMBOL(__put_cred);
EXPORT_SYMBOL(get_task_cred);
EXPORT_SYMBOL(prepare_creds);
EXPORT_SYMBOL(commit_creds);
EXPORT_SYMBOL(abort_creds);
EXPORT_SYMBOL(cred_fscmp);
EXPORT_SYMBOL(prepare_kernel_cred);
EXPORT_SYMBOL(set_security_override);
EXPORT_SYMBOL(set_create_files_as);
