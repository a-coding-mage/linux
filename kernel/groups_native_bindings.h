/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_GROUPS_NATIVE_BINDINGS_H
#define LUPOS_GROUPS_NATIVE_BINDINGS_H

/* No replacement layouts: these configured native headers own every type. */
#include <linux/cred.h>
#include <linux/errno.h>
#include <linux/gfp_types.h>
#include <linux/limits.h>
#include <linux/security.h>
#include <linux/user_namespace.h>

#ifdef __BINDGEN__
static const gfp_t LUPOS_GROUPS_GFP_KERNEL_ACCOUNT = GFP_KERNEL_ACCOUNT;
static const int LUPOS_GROUPS_EFAULT = EFAULT;
static const int LUPOS_GROUPS_EINVAL = EINVAL;
static const int LUPOS_GROUPS_ENOMEM = ENOMEM;
static const int LUPOS_GROUPS_EPERM = EPERM;
static const unsigned int LUPOS_GROUPS_NGROUPS_MAX = NGROUPS_MAX;
static const int LUPOS_GROUPS_CAP_SETGID = CAP_SETGID;
#endif

struct group_info *lupos_groups_alloc_flex(int gidsetsize, gfp_t flags);
void lupos_groups_kvfree(struct group_info *info);
void lupos_groups_usage_set(struct group_info *info, int count);
int lupos_groups_ngroups(const struct group_info *info);
void lupos_groups_set_ngroups(struct group_info *info, int count);
kgid_t lupos_groups_gid(const struct group_info *info, unsigned int index);
void lupos_groups_set_gid(struct group_info *info, unsigned int index, kgid_t gid);
struct group_info *lupos_groups_get_group_info(struct group_info *info);
void lupos_groups_put_group_info(struct group_info *info);

const struct cred *lupos_groups_current_cred(void);
struct user_namespace *lupos_groups_current_user_ns(void);
struct group_info *lupos_groups_cred_group_info(const struct cred *cred);
void lupos_groups_cred_set_group_info(struct cred *cred, struct group_info *info);
kgid_t lupos_groups_cred_fsgid(const struct cred *cred);
kgid_t lupos_groups_cred_egid(const struct cred *cred);

int lupos_groups_get_user_gid(gid_t *value, const gid_t __user *base,
			      unsigned int index);
int lupos_groups_put_user_gid(gid_t value, gid_t __user *base,
			      unsigned int index);
gid_t lupos_groups_from_kgid_munged(struct user_namespace *ns, kgid_t gid);
kgid_t lupos_groups_make_kgid(struct user_namespace *ns, gid_t gid);
bool lupos_groups_gid_valid(kgid_t gid);
bool lupos_groups_gid_gt(kgid_t a, kgid_t b);
bool lupos_groups_gid_lt(kgid_t a, kgid_t b);
bool lupos_groups_gid_eq(kgid_t a, kgid_t b);

/* Callback has exactly cmp_func_t's native int(const void *, const void *). */
int lupos_groups_gid_cmp(const void *a, const void *b);
kgid_t lupos_groups_sort_gid_value(const void *element);
void lupos_groups_sort(struct group_info *info);
int lupos_groups_security_task_fix_setgroups(struct cred *new,
					   const struct cred *old);
bool lupos_groups_userns_may_setgroups(const struct user_namespace *ns);

/* Rust bodies; native SYSCALL_DEFINE2 retains architecture entry ownership. */
long lupos_groups_getgroups(int gidsetsize, gid_t __user *grouplist);
long lupos_groups_setgroups(int gidsetsize, gid_t __user *grouplist);

#endif /* LUPOS_GROUPS_NATIVE_BINDINGS_H */
