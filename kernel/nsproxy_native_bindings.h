/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_NSPROXY_NATIVE_BINDINGS_H
#define LUPOS_NSPROXY_NATIVE_BINDINGS_H

/* Inactive proposal. Generate from the actual configured, pinned native tree.
 * nsproxy, nsset and fd are native non-randomized types. All other aggregate
 * pointees must remain opaque to this Rust owner; no translated header layouts.
 */
#include <linux/slab.h>
#include <linux/nsproxy.h>
#include <linux/ns/ns_common_types.h>
#include <linux/ns_common.h>
#include <linux/init_task.h>
#include <linux/mnt_namespace.h>
#include <linux/utsname.h>
#include <linux/pid_namespace.h>
#include <net/net_namespace.h>
#include <linux/ipc_namespace.h>
#include <linux/time_namespace.h>
#include <linux/fs_struct.h>
#include <linux/proc_fs.h>
#include <linux/proc_ns.h>
#include <linux/file.h>
#include <linux/cgroup.h>
#include <linux/perf_event.h>
#include <linux/nstree.h>
#include <linux/cred.h>
#include <linux/user_namespace.h>
#include <linux/capability.h>
#include <linux/ptrace.h>
#include <linux/sem.h>

#define NSPROXY_FLAG(name) static const u64 RUST_NSPROXY_##name = name;
NSPROXY_FLAG(CLONE_NS_ALL)
NSPROXY_FLAG(CLONE_NEWUSER)
NSPROXY_FLAG(CLONE_NEWPID)
NSPROXY_FLAG(CLONE_NEWNS)
NSPROXY_FLAG(CLONE_NEWUTS)
NSPROXY_FLAG(CLONE_NEWIPC)
NSPROXY_FLAG(CLONE_NEWCGROUP)
NSPROXY_FLAG(CLONE_NEWNET)
NSPROXY_FLAG(CLONE_NEWTIME)
NSPROXY_FLAG(CLONE_VM)
NSPROXY_FLAG(CLONE_SYSVSEM)
NSPROXY_FLAG(UNSHARE_EMPTY_MNTNS)
NSPROXY_FLAG(CLONE_EMPTY_MNTNS)
#undef NSPROXY_FLAG

#define NSPROXY_INT(name) static const int RUST_NSPROXY_##name = name;
NSPROXY_INT(ENOMEM)
NSPROXY_INT(EPERM)
NSPROXY_INT(EINVAL)
NSPROXY_INT(ESRCH)
NSPROXY_INT(EBADF)
NSPROXY_INT(CAP_SYS_ADMIN)
#undef NSPROXY_INT
static const unsigned int RUST_NSPROXY_PTRACE_MODE_READ_REALCREDS = PTRACE_MODE_READ_REALCREDS;

/* Native syscall registration calls the Rust decision body. */
int rust_nsproxy_setns(int fd, int flags);
/* Only the native __init entry calls this Rust initialization body. */
int __init rust_nsproxy_cache_init(void);

#define NSPROXY_LEAF(ret, name, args, body) ret rust_nsproxy_##name args;
#include "nsproxy_native_primitives.inc"
#undef NSPROXY_LEAF

#endif
