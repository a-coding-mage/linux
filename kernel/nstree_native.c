// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2025 Christian Brauner <brauner@kernel.org> */
/* Inactive native storage, macro leaves, callback and syscall metadata. */
#error "SOURCE ONLY HOLD: namespace-tree native storage and metadata are not admitted"
#include "nstree_native_bindings.h"

/* The writer, locked reader and sequence readers all use this single object. */
static __cacheline_aligned_in_smp DEFINE_SEQLOCK(ns_tree_lock);
static struct ns_tree_root ns_unified_root = {
	.ns_rb = RB_ROOT,
	.ns_list_head = LIST_HEAD_INIT(ns_unified_root.ns_list_head),
};
#define NSTREE_ROOT(name) struct ns_tree_root name = { \
	.ns_rb = RB_ROOT, .ns_list_head = LIST_HEAD_INIT(name.ns_list_head), \
}
NSTREE_ROOT(mnt_ns_tree);
NSTREE_ROOT(net_ns_tree);
NSTREE_ROOT(uts_ns_tree);
NSTREE_ROOT(user_ns_tree);
NSTREE_ROOT(ipc_ns_tree);
NSTREE_ROOT(pid_ns_tree);
NSTREE_ROOT(cgroup_ns_tree);
NSTREE_ROOT(time_ns_tree);
#undef NSTREE_ROOT
EXPORT_SYMBOL_GPL(net_ns_tree);
EXPORT_SYMBOL_GPL(__ns_tree_remove);

static atomic64_t namespace_cookie = ATOMIC64_INIT(NS_LAST_INIT_ID + 1);

/* Native callbacks retain the exact contextual C function types; no guessed
 * Rust nullable-function-pointer CFI encodings are installed.
 */
static int ns_cmp(struct rb_node *a, const struct rb_node *b)
{
	return rust_nstree_cmp(a, b);
}
static int ns_cmp_unified(struct rb_node *a, const struct rb_node *b)
{
	return rust_nstree_cmp_unified(a, b);
}
static int ns_cmp_owner(struct rb_node *a, const struct rb_node *b)
{
	return rust_nstree_cmp_owner(a, b);
}
static int ns_find(const void *key, const struct rb_node *node)
{
	return rust_nstree_find(key, node);
}
static int ns_find_unified(const void *key, const struct rb_node *node)
{
	return rust_nstree_find_unified(key, node);
}
static const struct rust_nstree_cmp_context type_cmp = { .cmp = ns_cmp };
static const struct rust_nstree_cmp_context unified_cmp = { .cmp = ns_cmp_unified };
static const struct rust_nstree_cmp_context owner_cmp = { .cmp = ns_cmp_owner };

struct rb_node *ns_tree_node_add(struct ns_tree_node *node,
		struct ns_tree_root *root,
		int (*cmp)(struct rb_node *, const struct rb_node *))
{
	const struct rust_nstree_cmp_context context = { .cmp = cmp };

	return rust_nstree_node_add(node, root, &context);
}

#define NSTREE_LEAF(ret, name, args, body) ret rust_nstree_##name args body
#include "nstree_native_primitives.inc"
#undef NSTREE_LEAF

/* Keep arch wrappers, native argument widths, syscall tracing and metadata. */
SYSCALL_DEFINE4(listns, const struct ns_id_req __user *, req,
		u64 __user *, ns_ids, size_t, nr_ns_ids, unsigned int, flags)
{
	return rust_nstree_listns(req, ns_ids, nr_ns_ids, flags);
}
