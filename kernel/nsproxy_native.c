// SPDX-License-Identifier: GPL-2.0-only
/* Native storage, syscall ABI and header/field primitives only.
 * Namespace preparation, validation, commit and unwind remain in nsproxy.rs.
 */
#include "nsproxy_native_bindings.h"
#include <linux/syscalls.h>

#error "Lupos nsproxy source proposal is not build-admitted; qualification is incomplete"

/* Native initializer from the pinned owner; no Rust refcount/layout projection. */
struct nsproxy init_nsproxy = {
	.count			= REFCOUNT_INIT(1),
	.uts_ns			= &init_uts_ns,
#if defined(CONFIG_POSIX_MQUEUE) || defined(CONFIG_SYSVIPC)
	.ipc_ns			= &init_ipc_ns,
#endif
	.mnt_ns			= NULL,
	.pid_ns_for_children	= &init_pid_ns,
#ifdef CONFIG_NET
	.net_ns			= &init_net,
#endif
#ifdef CONFIG_CGROUPS
	.cgroup_ns		= &init_cgroup_ns,
#endif
#ifdef CONFIG_TIME_NS
	.time_ns		= &init_time_ns,
	.time_ns_for_children	= &init_time_ns,
#endif
};

#define NSPROXY_LEAF(ret, name, args, body) ret rust_nsproxy_##name args body
#include "nsproxy_native_primitives.inc"
#undef NSPROXY_LEAF

SYSCALL_DEFINE2(setns, int, fd, int, flags)
{
	return rust_nsproxy_setns(fd, flags);
}

/* Preserve the configured __init contract, not only the section name.
 * Cache ownership and initialization decisions remain in the Rust owner.
 */
int __init nsproxy_cache_init(void)
{
	return rust_nsproxy_cache_init();
}
