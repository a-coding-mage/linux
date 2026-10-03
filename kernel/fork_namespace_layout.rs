// SPDX-License-Identifier: GPL-2.0-only
// Check canonical records and every namespace field directly accessed by fork.
const _: () = {
    macro_rules! record {
        ($ty:ty, $size:expr, $align:expr) => {
            assert!(size_of::<$ty>() == $size as usize);
            assert!(core::mem::align_of::<$ty>() == $align as usize);
        };
    }
    macro_rules! field {
        ($ty:ty, $field:tt, $native:expr) => {
            assert!(offset_of!($ty, $field) == $native as usize);
        };
    }
    record!(
        ns_tree,
        RUST_FORK_LAYOUT_SIZE_ns_tree,
        RUST_FORK_LAYOUT_ALIGN_ns_tree
    );
    field!(ns_tree, ns_id, RUST_FORK_LAYOUT_OFFSET_ns_tree_ns_id);
    field!(
        ns_tree,
        __ns_ref_active,
        RUST_FORK_LAYOUT_OFFSET_ns_tree___ns_ref_active
    );
    field!(
        ns_tree,
        ns_unified_node,
        RUST_FORK_LAYOUT_OFFSET_ns_tree_ns_unified_node
    );
    field!(
        ns_tree,
        ns_tree_node,
        RUST_FORK_LAYOUT_OFFSET_ns_tree_ns_tree_node
    );
    field!(
        ns_tree,
        ns_owner_node,
        RUST_FORK_LAYOUT_OFFSET_ns_tree_ns_owner_node
    );
    field!(
        ns_tree,
        ns_owner_root,
        RUST_FORK_LAYOUT_OFFSET_ns_tree_ns_owner_root
    );
    record!(
        ns_common,
        RUST_FORK_LAYOUT_SIZE_ns_common,
        RUST_FORK_LAYOUT_ALIGN_ns_common
    );
    field!(
        ns_common,
        ns_type,
        RUST_FORK_LAYOUT_OFFSET_ns_common_ns_type
    );
    field!(
        ns_common,
        stashed,
        RUST_FORK_LAYOUT_OFFSET_ns_common_stashed
    );
    field!(ns_common, ops, RUST_FORK_LAYOUT_OFFSET_ns_common_ops);
    field!(ns_common, inum, RUST_FORK_LAYOUT_OFFSET_ns_common_inum);
    assert!(
        offset_of!(ns_common, __bindgen_anon_2)
            + offset_of!(ns_common__bindgen_ty_2, tree)
            + offset_of!(ns_tree, ns_id)
            == RUST_FORK_LAYOUT_OFFSET_ns_common_ns_id as usize
    );
    assert!(
        offset_of!(ns_common, __bindgen_anon_2) + offset_of!(ns_common__bindgen_ty_2, ns_rcu)
            == RUST_FORK_LAYOUT_OFFSET_ns_common_ns_rcu as usize
    );
    record!(
        user_namespace,
        RUST_FORK_LAYOUT_SIZE_user_namespace,
        RUST_FORK_LAYOUT_ALIGN_user_namespace
    );
    field!(
        user_namespace,
        ns,
        RUST_FORK_LAYOUT_OFFSET_user_namespace_ns
    );
    field!(
        user_namespace,
        flags,
        RUST_FORK_LAYOUT_OFFSET_user_namespace_flags
    );
    field!(
        user_namespace,
        ucounts,
        RUST_FORK_LAYOUT_OFFSET_user_namespace_ucounts
    );
    field!(
        user_namespace,
        ucount_max,
        RUST_FORK_LAYOUT_OFFSET_user_namespace_ucount_max
    );
    field!(
        user_namespace,
        rlimit_max,
        RUST_FORK_LAYOUT_OFFSET_user_namespace_rlimit_max
    );
    record!(
        nsproxy,
        RUST_FORK_LAYOUT_SIZE_nsproxy,
        RUST_FORK_LAYOUT_ALIGN_nsproxy
    );
    field!(
        nsproxy,
        pid_ns_for_children,
        RUST_FORK_LAYOUT_OFFSET_nsproxy_pid_ns_for_children
    );
    record!(
        pid_namespace,
        RUST_FORK_LAYOUT_SIZE_pid_namespace,
        RUST_FORK_LAYOUT_ALIGN_pid_namespace
    );
    field!(
        pid_namespace,
        pid_allocated,
        RUST_FORK_LAYOUT_OFFSET_pid_namespace_pid_allocated
    );
};
