// SPDX-License-Identifier: GPL-2.0-only
/* Exact native initializers from user.c; no user-cache algorithm lives here. */
#error "SOURCE ONLY HOLD: user-accounting native storage is not admitted"

#include <linux/init.h>
#include <linux/sched.h>
#include <linux/slab.h>
#include <linux/bitops.h>
#include <linux/key.h>
#include <linux/sched/user.h>
#include <linux/interrupt.h>
#include <linux/export.h>
#include <linux/user_namespace.h>
#include <linux/binfmts.h>
#include <linux/proc_ns.h>

#if IS_ENABLED(CONFIG_BINFMT_MISC)
struct binfmt_misc init_binfmt_misc = {
	.entries = HLIST_HEAD_INIT,
	.enabled = true,
	.entries_lock = __SPIN_LOCK_UNLOCKED(init_binfmt_misc.entries_lock),
};
EXPORT_SYMBOL_GPL(init_binfmt_misc);
#endif

/*
 * userns count is 1 for root user, 1 for init_uts_ns,
 * and 1 for... ?
 */
struct user_namespace init_user_ns = {
	.ns = NS_COMMON_INIT(init_user_ns),
	.uid_map = {
		{
			.extent[0] = {
				.first = 0,
				.lower_first = 0,
				.count = 4294967295U,
			},
			.nr_extents = 1,
		},
	},
	.gid_map = {
		{
			.extent[0] = {
				.first = 0,
				.lower_first = 0,
				.count = 4294967295U,
			},
			.nr_extents = 1,
		},
	},
	.projid_map = {
		{
			.extent[0] = {
				.first = 0,
				.lower_first = 0,
				.count = 4294967295U,
			},
			.nr_extents = 1,
		},
	},
	.owner = GLOBAL_ROOT_UID,
	.group = GLOBAL_ROOT_GID,
	.flags = USERNS_INIT_FLAGS,
#ifdef CONFIG_KEYS
	.keyring_name_list = LIST_HEAD_INIT(init_user_ns.keyring_name_list),
	.keyring_sem = __RWSEM_INITIALIZER(init_user_ns.keyring_sem),
#endif
#if IS_ENABLED(CONFIG_BINFMT_MISC)
	.binfmt_misc = &init_binfmt_misc,
#endif
};
EXPORT_SYMBOL_GPL(init_user_ns);

/* root_user.__count is 1, for init task cred */
struct user_struct root_user = {
	.__count	= REFCOUNT_INIT(1),
	.uid		= GLOBAL_ROOT_UID,
	.ratelimit	= RATELIMIT_STATE_INIT(root_user.ratelimit, 0, 0),
};
