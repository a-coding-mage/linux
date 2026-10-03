/* SPDX-License-Identifier: GPL-2.0-only */
/* Binding-only native layout facts; no runtime state or guessed padding. */
#ifndef RUST_FORK_NAMESPACE_LAYOUT_H
#define RUST_FORK_NAMESPACE_LAYOUT_H
#define RUST_FORK_LAYOUT_TYPE(t) \
    static const size_t RUST_FORK_LAYOUT_SIZE_##t = sizeof(struct t); \
    static const size_t RUST_FORK_LAYOUT_ALIGN_##t = __alignof__(struct t)
#define RUST_FORK_LAYOUT_FIELD(t, f) \
    static const size_t RUST_FORK_LAYOUT_OFFSET_##t##_##f = offsetof(struct t, f)
RUST_FORK_LAYOUT_TYPE(ns_tree);
RUST_FORK_LAYOUT_FIELD(ns_tree, ns_id);
RUST_FORK_LAYOUT_FIELD(ns_tree, __ns_ref_active);
RUST_FORK_LAYOUT_FIELD(ns_tree, ns_unified_node);
RUST_FORK_LAYOUT_FIELD(ns_tree, ns_tree_node);
RUST_FORK_LAYOUT_FIELD(ns_tree, ns_owner_node);
RUST_FORK_LAYOUT_FIELD(ns_tree, ns_owner_root);
RUST_FORK_LAYOUT_TYPE(ns_common);
RUST_FORK_LAYOUT_FIELD(ns_common, ns_type);
RUST_FORK_LAYOUT_FIELD(ns_common, stashed);
RUST_FORK_LAYOUT_FIELD(ns_common, ops);
RUST_FORK_LAYOUT_FIELD(ns_common, inum);
RUST_FORK_LAYOUT_FIELD(ns_common, ns_id);
RUST_FORK_LAYOUT_FIELD(ns_common, ns_rcu);
RUST_FORK_LAYOUT_TYPE(user_namespace);
RUST_FORK_LAYOUT_FIELD(user_namespace, ns);
RUST_FORK_LAYOUT_FIELD(user_namespace, flags);
RUST_FORK_LAYOUT_FIELD(user_namespace, ucounts);
RUST_FORK_LAYOUT_FIELD(user_namespace, ucount_max);
RUST_FORK_LAYOUT_FIELD(user_namespace, rlimit_max);
RUST_FORK_LAYOUT_TYPE(nsproxy);
RUST_FORK_LAYOUT_FIELD(nsproxy, pid_ns_for_children);
RUST_FORK_LAYOUT_TYPE(pid_namespace);
RUST_FORK_LAYOUT_FIELD(pid_namespace, pid_allocated);
#undef RUST_FORK_LAYOUT_FIELD
#undef RUST_FORK_LAYOUT_TYPE
#endif
