// SPDX-License-Identifier: GPL-2.0
/* Native syscall/export metadata only; all policy bodies remain in groups.rs. */
#error "SOURCE ONLY HOLD: supplementary-group native metadata is not admitted"

#include "groups_native_bindings.h"
#include <linux/export.h>
#include <linux/syscalls.h>

EXPORT_SYMBOL(groups_alloc);
EXPORT_SYMBOL(groups_free);
EXPORT_SYMBOL(groups_sort);
EXPORT_SYMBOL(set_groups);
EXPORT_SYMBOL(set_current_groups);
EXPORT_SYMBOL(in_group_p);
EXPORT_SYMBOL(in_egroup_p);

SYSCALL_DEFINE2(getgroups, int, gidsetsize, gid_t __user *, grouplist)
{
	return lupos_groups_getgroups(gidsetsize, grouplist);
}

SYSCALL_DEFINE2(setgroups, int, gidsetsize, gid_t __user *, grouplist)
{
	return lupos_groups_setgroups(gidsetsize, grouplist);
}
