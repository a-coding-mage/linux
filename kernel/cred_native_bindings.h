/* SPDX-License-Identifier: GPL-2.0-or-later */
#ifndef LUPOS_CRED_NATIVE_BINDINGS_H
#define LUPOS_CRED_NATIVE_BINDINGS_H

/* Configured native headers are the only type, layout and constant authority. */
#include <linux/cred.h>
#include <linux/slab.h>
#include <linux/sched/coredump.h>
#include <linux/keyctl.h>
#include <linux/security.h>
#include <linux/binfmts.h>
#include <linux/cn_proc.h>
#include <linux/user_namespace.h>

static const u64 LUPOS_CRED_CLONE_THREAD = CLONE_THREAD;
static const u64 LUPOS_CRED_CLONE_NEWUSER = CLONE_NEWUSER;
static const int LUPOS_CRED_ENOMEM = ENOMEM;
static const int LUPOS_CRED_EAGAIN = EAGAIN;
static const int LUPOS_CRED_EINVAL = EINVAL;
static const gfp_t LUPOS_CRED_GFP_KERNEL = GFP_KERNEL;
static const gfp_t LUPOS_CRED_GFP_KERNEL_ACCOUNT = GFP_KERNEL_ACCOUNT;
static const enum rlimit_type LUPOS_CRED_UCOUNT_RLIMIT_NPROC = UCOUNT_RLIMIT_NPROC;
static const int LUPOS_CRED_PROC_EVENT_UID = PROC_EVENT_UID;
static const int LUPOS_CRED_PROC_EVENT_GID = PROC_EVENT_GID;

struct cred *lupos_cred_from_rcu(struct rcu_head *rcu);
struct rcu_head *lupos_cred_rcu(struct cred *cred);
long lupos_cred_usage_read(const struct cred *cred);
void lupos_cred_usage_set(struct cred *cred, long value);
int lupos_cred_non_rcu(const struct cred *cred);
void lupos_cred_set_non_rcu(struct cred *cred, int value);
void lupos_cred_copy(struct cred *new, const struct cred *old);
void lupos_cred_assign(struct cred *new, const struct cred *old);
/* Separate native BUG sites retain each original cred.c operand expression. */
void lupos_cred_bug_put_usage(struct cred *cred);
void lupos_cred_bug_put_current_cred(struct cred *cred);
void lupos_cred_bug_put_current_real_cred(struct cred *cred);
void lupos_cred_bug_task_cred_null(const struct cred *cred);
void lupos_cred_bug_commit_overridden(struct task_struct *task,
				    const struct cred *old);
void lupos_cred_bug_commit_usage(struct cred *new);
void lupos_cred_bug_abort_usage(struct cred *new);
void __noreturn lupos_cred_bad_usage(const struct cred *cred);
bool lupos_cred_warn_null_daemon(struct task_struct *daemon);
void lupos_cred_rcu_read_lock(void) __acquires_shared(RCU);
void lupos_cred_rcu_read_unlock(void) __releases_shared(RCU);
const struct cred *lupos_cred_task_rcu(struct task_struct *task);
const struct cred *lupos_cred_get_rcu(const struct cred *cred);
const struct cred *lupos_cred_get(const struct cred *cred);
const struct cred *lupos_cred_get_many(const struct cred *cred, int nr);
void lupos_cred_put(const struct cred *cred);
void lupos_cred_put_many(const struct cred *cred, int nr);
/* The exact native callback signature terminates in a C-only trampoline.
 * This direct Rust body declaration and enqueue leaf carry no callback value.
 */
void lupos_cred_put_rcu_body(struct rcu_head *rcu);
void lupos_cred_call_rcu(struct rcu_head *rcu);

kuid_t lupos_cred_uid(const struct cred *cred);
kuid_t lupos_cred_suid(const struct cred *cred);
kuid_t lupos_cred_euid(const struct cred *cred);
kuid_t lupos_cred_fsuid(const struct cred *cred);
kgid_t lupos_cred_gid(const struct cred *cred);
kgid_t lupos_cred_sgid(const struct cred *cred);
kgid_t lupos_cred_egid(const struct cred *cred);
kgid_t lupos_cred_fsgid(const struct cred *cred);
void lupos_cred_set_suid(struct cred *cred, kuid_t value);
void lupos_cred_set_fsuid(struct cred *cred, kuid_t value);
void lupos_cred_set_sgid(struct cred *cred, kgid_t value);
void lupos_cred_set_fsgid(struct cred *cred, kgid_t value);
bool lupos_cred_uid_eq(kuid_t a, kuid_t b);
bool lupos_cred_uid_lt(kuid_t a, kuid_t b);
bool lupos_cred_uid_gt(kuid_t a, kuid_t b);
bool lupos_cred_uid_valid(kuid_t uid);
bool lupos_cred_gid_eq(kgid_t a, kgid_t b);
bool lupos_cred_gid_lt(kgid_t a, kgid_t b);
bool lupos_cred_gid_gt(kgid_t a, kgid_t b);
bool lupos_cred_gid_valid(kgid_t gid);
kernel_cap_t lupos_cred_cap_permitted(const struct cred *cred);
bool lupos_cred_cap_issubset(kernel_cap_t subset, kernel_cap_t set);

struct user_struct *lupos_cred_user(const struct cred *cred);
struct user_namespace *lupos_cred_user_ns(const struct cred *cred);
struct ucounts *lupos_cred_ucounts(const struct cred *cred);
void lupos_cred_set_ucounts(struct cred *cred, struct ucounts *value);
struct group_info *lupos_cred_group_info(const struct cred *cred);
struct user_struct *lupos_cred_get_uid(struct user_struct *user);
void lupos_cred_free_uid(struct user_struct *user);
struct user_namespace *lupos_cred_get_user_ns(struct user_namespace *ns);
void lupos_cred_put_user_ns(struct user_namespace *ns);
struct group_info *lupos_cred_get_group_info(struct group_info *info);
void lupos_cred_put_group_info(struct group_info *info);
struct ucounts *lupos_cred_get_ucounts(struct ucounts *ucounts);
int lupos_cred_create_user_ns(struct cred *new);
struct user_namespace *lupos_cred_init_user_ns(void);
struct user_namespace *lupos_cred_ns_parent(const struct user_namespace *ns);
kuid_t lupos_cred_ns_owner(const struct user_namespace *ns);
struct user_namespace *lupos_cred_ucounts_ns(const struct ucounts *ucounts);
kuid_t lupos_cred_ucounts_uid(const struct ucounts *ucounts);
int lupos_cred_ngroups(const struct group_info *info);
kgid_t lupos_cred_group_gid(const struct group_info *info, int index);
kuid_t lupos_cred_inode_uid(const struct inode *inode);
kgid_t lupos_cred_inode_gid(const struct inode *inode);

struct task_struct *lupos_cred_current(void);
const struct cred *lupos_cred_task_cred(const struct task_struct *task);
const struct cred *lupos_cred_task_real_cred(const struct task_struct *task);
void lupos_cred_set_task_cred(struct task_struct *task, const struct cred *cred);
void lupos_cred_set_task_real_cred(struct task_struct *task, const struct cred *cred);
void lupos_cred_publish_task_cred(struct task_struct *task, const struct cred *cred);
void lupos_cred_publish_task_real_cred(struct task_struct *task, const struct cred *cred);
struct ucounts *lupos_cred_task_ucounts(struct task_struct *task);
bool lupos_cred_task_has_mm(const struct task_struct *task);
void lupos_cred_clear_pdeath_signal(struct task_struct *task);
void lupos_cred_set_dumpable(void);
void lupos_cred_smp_wmb(void);
void lupos_cred_proc_id_connector(struct task_struct *task, int which_id);
void lupos_cred_key_fsuid_changed(struct cred *new);
void lupos_cred_key_fsgid_changed(struct cred *new);

/* Direct Rust body behind the exact native __init public entry shell. */
void __init lupos_cred_init_body(void);
struct kmem_cache * __init lupos_cred_cache_create(void);
struct cred *lupos_cred_cache_alloc(struct kmem_cache *cache, gfp_t flags);
struct cred *lupos_cred_cache_zalloc(struct kmem_cache *cache, gfp_t flags);
void lupos_cred_cache_free(struct kmem_cache *cache, struct cred *cred);
int lupos_cred_security_alloc_blank(struct cred *new, gfp_t flags);
void lupos_cred_security_free(struct cred *cred);
int lupos_cred_security_prepare(struct cred *new, const struct cred *old, gfp_t flags);
int lupos_cred_security_act_as(struct cred *new, u32 secid);
int lupos_cred_security_create_files_as(struct cred *new, struct inode *inode);

#ifdef CONFIG_SECURITY
void lupos_cred_clear_security(struct cred *cred);
#endif
#ifdef CONFIG_KEYS
static const unsigned char LUPOS_CRED_KEY_REQKEY_DEFL_THREAD_KEYRING =
	KEY_REQKEY_DEFL_THREAD_KEYRING;
struct key *lupos_cred_session_keyring(const struct cred *cred);
struct key *lupos_cred_process_keyring(const struct cred *cred);
struct key *lupos_cred_thread_keyring(const struct cred *cred);
struct key *lupos_cred_request_key_auth(const struct cred *cred);
void lupos_cred_set_session_keyring(struct cred *cred, struct key *key);
void lupos_cred_set_process_keyring(struct cred *cred, struct key *key);
void lupos_cred_set_thread_keyring(struct cred *cred, struct key *key);
void lupos_cred_set_request_key_auth(struct cred *cred, struct key *key);
void lupos_cred_set_jit_keyring(struct cred *cred, unsigned char value);
struct key *lupos_cred_key_get(struct key *key);
#endif
#ifdef CONFIG_KEYS_REQUEST_CACHE
struct key *lupos_cred_cached_requested_key(const struct task_struct *task);
void lupos_cred_clear_cached_requested_key(struct task_struct *task);
#endif

#endif /* LUPOS_CRED_NATIVE_BINDINGS_H */
