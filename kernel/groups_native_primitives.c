// SPDX-License-Identifier: GPL-2.0
/* Native primitives only; group loops, search, policy and ownership stay Rust. */
#error "SOURCE ONLY HOLD: supplementary-group native primitives are not admitted"

#include "groups_native_bindings.h"
#include <linux/slab.h>
#include <linux/sort.h>
#include <linux/uaccess.h>
#include <linux/vmalloc.h>

struct group_info *lupos_groups_alloc_flex(int gidsetsize, gfp_t flags)
{
	struct group_info *gi;

	return kvmalloc_flex(*gi, gid, gidsetsize, flags);
}

void lupos_groups_kvfree(struct group_info *info)
{
	kvfree(info);
}

void lupos_groups_usage_set(struct group_info *info, int count)
{
	refcount_set(&info->usage, count);
}

int lupos_groups_ngroups(const struct group_info *info)
{
	return info->ngroups;
}

void lupos_groups_set_ngroups(struct group_info *info, int count)
{
	info->ngroups = count;
}

kgid_t lupos_groups_gid(const struct group_info *info, unsigned int index)
{
	return info->gid[index];
}

void lupos_groups_set_gid(struct group_info *info, unsigned int index, kgid_t gid)
{
	info->gid[index] = gid;
}

struct group_info *lupos_groups_get_group_info(struct group_info *info)
{
	return get_group_info(info);
}

void lupos_groups_put_group_info(struct group_info *info)
{
	put_group_info(info);
}

const struct cred *lupos_groups_current_cred(void)
{
	return current_cred();
}

struct user_namespace *lupos_groups_current_user_ns(void)
{
	return current_user_ns();
}

struct group_info *lupos_groups_cred_group_info(const struct cred *cred)
{
	return cred->group_info;
}

void lupos_groups_cred_set_group_info(struct cred *cred, struct group_info *info)
{
	cred->group_info = info;
}

kgid_t lupos_groups_cred_fsgid(const struct cred *cred)
{
	return cred->fsgid;
}

kgid_t lupos_groups_cred_egid(const struct cred *cred)
{
	return cred->egid;
}

int lupos_groups_get_user_gid(gid_t *value, const gid_t __user *base,
			      unsigned int index)
{
	return get_user(*value, base + index);
}

int lupos_groups_put_user_gid(gid_t value, gid_t __user *base,
			      unsigned int index)
{
	return put_user(value, base + index);
}

gid_t lupos_groups_from_kgid_munged(struct user_namespace *ns, kgid_t gid)
{
	return from_kgid_munged(ns, gid);
}

kgid_t lupos_groups_make_kgid(struct user_namespace *ns, gid_t gid)
{
	return make_kgid(ns, gid);
}

bool lupos_groups_gid_valid(kgid_t gid)
{
	return gid_valid(gid);
}

bool lupos_groups_gid_gt(kgid_t a, kgid_t b)
{
	return gid_gt(a, b);
}

bool lupos_groups_gid_lt(kgid_t a, kgid_t b)
{
	return gid_lt(a, b);
}

bool lupos_groups_gid_eq(kgid_t a, kgid_t b)
{
	return gid_eq(a, b);
}

kgid_t lupos_groups_sort_gid_value(const void *element)
{
	/* Original sort convention: element points to a native kgid_t value. */
	return *(const kgid_t *)element;
}

void lupos_groups_sort(struct group_info *info)
{
	/* Native prototype checks comparator type; last argument is swap=NULL. */
	sort(info->gid, info->ngroups, sizeof(*info->gid),
	     lupos_groups_gid_cmp, NULL);
}

int lupos_groups_security_task_fix_setgroups(struct cred *new,
					   const struct cred *old)
{
	return security_task_fix_setgroups(new, old);
}

bool lupos_groups_userns_may_setgroups(const struct user_namespace *ns)
{
	return userns_may_setgroups(ns);
}
