/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_NSTREE_NATIVE_BINDINGS_H
#define LUPOS_NSTREE_NATIVE_BINDINGS_H

/* Inactive proposal; use configured native headers, never translated layouts. */
#include <linux/nstree.h>
#include <linux/proc_ns.h>
#include <linux/ns_common.h>
#include <linux/user_namespace.h>
#include <linux/syscalls.h>
#include <linux/uaccess.h>
#include <linux/vfsdebug.h>

/* The callback itself never crosses the Rust FFI as Option<fn>. This native
 * context is borrowed synchronously for one rb_find_add_rcu invocation.
 */
struct rust_nstree_cmp_context {
	int (*cmp)(struct rb_node *, const struct rb_node *);
};

#ifdef __BINDGEN__
#define NSTREE_INT(name) static const int RUST_NSTREE_##name = name;
NSTREE_INT(CLONE_NEWCGROUP)
NSTREE_INT(CLONE_NEWIPC)
NSTREE_INT(CLONE_NEWNS)
NSTREE_INT(CLONE_NEWNET)
NSTREE_INT(CLONE_NEWPID)
NSTREE_INT(CLONE_NEWUSER)
NSTREE_INT(CLONE_NEWUTS)
NSTREE_INT(CLONE_NEWTIME)
NSTREE_INT(EINVAL)
NSTREE_INT(EOVERFLOW)
NSTREE_INT(EFAULT)
NSTREE_INT(E2BIG)
NSTREE_INT(EOPNOTSUPP)
NSTREE_INT(ENOENT)
#undef NSTREE_INT
static const u32 RUST_NSTREE_NS_ALL = PID_NS | USER_NS | MNT_NS | UTS_NS |
	IPC_NS | NET_NS | CGROUP_NS | TIME_NS;
static const u64 RUST_NSTREE_LISTNS_CURRENT_USER = LISTNS_CURRENT_USER;
static const size_t RUST_NSTREE_NS_ID_REQ_SIZE_VER0 = NS_ID_REQ_SIZE_VER0;
#endif /* __BINDGEN__ */

/* Implemented in Rust. The original public callback-bearing entry remains a
 * native wrapper with the exact include/linux/nstree.h signature.
 */
struct rb_node *rust_nstree_node_add(struct ns_tree_node *node,
		struct ns_tree_root *root, const struct rust_nstree_cmp_context *cmp);
int rust_nstree_cmp(struct rb_node *a, const struct rb_node *b);
int rust_nstree_cmp_unified(struct rb_node *a, const struct rb_node *b);
int rust_nstree_cmp_owner(struct rb_node *a, const struct rb_node *b);
int rust_nstree_find(const void *key, const struct rb_node *node);
int rust_nstree_find_unified(const void *key, const struct rb_node *node);
long rust_nstree_listns(const struct ns_id_req __user *req,
		u64 __user *ns_ids, size_t nr_ns_ids, unsigned int flags);

#define NSTREE_LEAF(ret, name, args, body) ret rust_nstree_##name args;
#include "nstree_native_primitives.inc"
#undef NSTREE_LEAF
#endif
