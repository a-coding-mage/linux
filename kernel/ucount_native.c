// SPDX-License-Identifier: GPL-2.0-only
/* Native storage, signature/registration metadata and primitive boundary.
 * Inactive proposal; all ucount.c decision bodies remain in ucount.rs.
 * Preserve original kernel/ucount.o compilation/instrumentation provenance
 * when and only when a separately authorized integrator admits this owner.
 */
#include "ucount_native_bindings.h"
#include <linux/export.h>
#include <linux/hash.h>
#include <linux/init.h>
#include <linux/kmemleak.h>

#error "SOURCE ONLY HOLD: user-accounting native boundary is not admitted"

struct ucounts init_ucounts = {
	.ns = &init_user_ns,
	.uid = GLOBAL_ROOT_UID,
	.count = RCUREF_INIT(1),
};

static struct hlist_nulls_head ucounts_hashtable[UCOUNTS_HASHTABLE_ENTRIES] = {
	[0 ... UCOUNTS_HASHTABLE_ENTRIES - 1] = HLIST_NULLS_HEAD_INIT(0)
};
static DEFINE_SPINLOCK(ucounts_lock);

#define ucounts_hashfn(ns, uid) \
	hash_long((unsigned long)__kuid_val(uid) + (unsigned long)(ns), \
		  UCOUNTS_HASHTABLE_BITS)
#define ucounts_hashentry(ns, uid) \
	(ucounts_hashtable + ucounts_hashfn(ns, uid))

#ifdef CONFIG_SYSCTL
/* Native callback types and KCFI identities, not Rust Option<fn> guesses. */
static struct ctl_table_set *set_lookup(struct ctl_table_root *root)
{
	return lupos_ucount_set_lookup(root);
}

static int set_is_seen(struct ctl_table_set *set)
{
	return lupos_ucount_set_is_seen(set);
}

static int set_permissions(struct ctl_table_header *head,
			   const struct ctl_table *table)
{
	return lupos_ucount_set_permissions(head, table);
}

static struct ctl_table_root set_root = {
	.lookup = set_lookup,
	.permissions = set_permissions,
};

static long ue_zero = 0;
static long ue_int_max = INT_MAX;

#define UCOUNT_ENTRY(name) \
	{ \
		.procname = name, \
		.maxlen = sizeof(long), \
		.mode = 0644, \
		.proc_handler = proc_doulongvec_minmax, \
		.extra1 = &ue_zero, \
		.extra2 = &ue_int_max, \
	}
static const struct ctl_table user_table[] = {
	UCOUNT_ENTRY("max_user_namespaces"),
	UCOUNT_ENTRY("max_pid_namespaces"),
	UCOUNT_ENTRY("max_uts_namespaces"),
	UCOUNT_ENTRY("max_ipc_namespaces"),
	UCOUNT_ENTRY("max_net_namespaces"),
	UCOUNT_ENTRY("max_mnt_namespaces"),
	UCOUNT_ENTRY("max_cgroup_namespaces"),
	UCOUNT_ENTRY("max_time_namespaces"),
#ifdef CONFIG_INOTIFY_USER
	UCOUNT_ENTRY("max_inotify_instances"),
	UCOUNT_ENTRY("max_inotify_watches"),
#endif
#ifdef CONFIG_FANOTIFY
	UCOUNT_ENTRY("max_fanotify_groups"),
	UCOUNT_ENTRY("max_fanotify_marks"),
#endif
#if IS_ENABLED(CONFIG_BINFMT_MISC)
	UCOUNT_ENTRY("max_binfmt_misc_interpreters"),
#endif
};
#undef UCOUNT_ENTRY

/* Original function-static objects had static storage, not __initdata. */
static struct ctl_table_header *user_header;
static struct ctl_table empty[1];
#endif /* CONFIG_SYSCTL */

#define UCOUNT_LEAF(ret, name, args, body) ret lupos_ucount_##name args body
#include "ucount_native_primitives.inc"
#undef UCOUNT_LEAF

/* Every public signature is also declared by native user_namespace.h. These
 * wrappers provide only exact native ABI/CFI identity and direct dispatch.
 */
bool setup_userns_sysctls(struct user_namespace *ns)
{
	return lupos_ucount_setup_userns_sysctls(ns);
}

void retire_userns_sysctls(struct user_namespace *ns)
{
	lupos_ucount_retire_userns_sysctls(ns);
}

struct ucounts *alloc_ucounts(struct user_namespace *ns, kuid_t uid)
{
	return lupos_ucount_alloc_ucounts(ns, uid);
}

void put_ucounts(struct ucounts *ucounts)
{
	lupos_ucount_put_ucounts(ucounts);
}

struct ucounts *inc_ucount(struct user_namespace *ns, kuid_t uid,
			   enum ucount_type type)
{
	return lupos_ucount_inc_ucount(ns, uid, type);
}
EXPORT_SYMBOL_FOR_MODULES(inc_ucount, "binfmt_misc");

void dec_ucount(struct ucounts *ucounts, enum ucount_type type)
{
	lupos_ucount_dec_ucount(ucounts, type);
}
EXPORT_SYMBOL_FOR_MODULES(dec_ucount, "binfmt_misc");

long inc_rlimit_ucounts(struct ucounts *ucounts, enum rlimit_type type, long v)
{
	return lupos_ucount_inc_rlimit_ucounts(ucounts, type, v);
}

bool dec_rlimit_ucounts(struct ucounts *ucounts, enum rlimit_type type, long v)
{
	return lupos_ucount_dec_rlimit_ucounts(ucounts, type, v);
}

void dec_rlimit_put_ucounts(struct ucounts *ucounts, enum rlimit_type type)
{
	lupos_ucount_dec_rlimit_put_ucounts(ucounts, type);
}

long inc_rlimit_get_ucounts(struct ucounts *ucounts, enum rlimit_type type,
			    bool override_rlimit)
{
	return lupos_ucount_inc_rlimit_get_ucounts(ucounts, type, override_rlimit);
}

bool is_rlimit_overlimit(struct ucounts *ucounts, enum rlimit_type type,
			unsigned long rlimit)
{
	return lupos_ucount_is_rlimit_overlimit(ucounts, type, rlimit);
}

/* Exact native init annotation and initcall metadata. The sole Rust body also
 * remains in .init.text; there is no alternate algorithm or second initcall.
 */
static __init int user_namespace_sysctl_init(void)
{
	return lupos_ucount_user_namespace_sysctl_init();
}
subsys_initcall(user_namespace_sysctl_init);
