// SPDX-License-Identifier: GPL-2.0-only
// Compile-time checks against the same configured native headers.
const _: () = {
    assert!(core::mem::size_of::<fd>() == LUPOS_PID_SIZE_fd as usize);
    assert!(core::mem::align_of::<fd>() == LUPOS_PID_ALIGN_fd as usize);
    assert!(core::mem::offset_of!(fd, word) == LUPOS_PID_OFFSET_fd_word as usize);
    assert!(core::mem::size_of::<file>() == LUPOS_PID_SIZE_file as usize);
    assert!(core::mem::align_of::<file>() == LUPOS_PID_ALIGN_file as usize);
    assert!(core::mem::offset_of!(file, f_flags) == LUPOS_PID_OFFSET_file_f_flags as usize);
    assert!(core::mem::size_of::<hlist_head>() == LUPOS_PID_SIZE_hlist_head as usize);
    assert!(core::mem::align_of::<hlist_head>() == LUPOS_PID_ALIGN_hlist_head as usize);
    assert!(core::mem::offset_of!(hlist_head, first) == LUPOS_PID_OFFSET_hlist_head_first as usize);
    assert!(core::mem::size_of::<hlist_node>() == LUPOS_PID_SIZE_hlist_node as usize);
    assert!(core::mem::align_of::<hlist_node>() == LUPOS_PID_ALIGN_hlist_node as usize);
    assert!(core::mem::offset_of!(hlist_node, next) == LUPOS_PID_OFFSET_hlist_node_next as usize);
    assert!(core::mem::offset_of!(hlist_node, pprev) == LUPOS_PID_OFFSET_hlist_node_pprev as usize);
    assert!(core::mem::size_of::<ns_common>() == LUPOS_PID_SIZE_ns_common as usize);
    assert!(core::mem::align_of::<ns_common>() == LUPOS_PID_ALIGN_ns_common as usize);
    assert!(
        core::mem::offset_of!(ns_common, ns_type) == LUPOS_PID_OFFSET_ns_common_ns_type as usize
    );
    assert!(
        core::mem::offset_of!(ns_common, stashed) == LUPOS_PID_OFFSET_ns_common_stashed as usize
    );
    assert!(core::mem::offset_of!(ns_common, ops) == LUPOS_PID_OFFSET_ns_common_ops as usize);
    assert!(core::mem::offset_of!(ns_common, inum) == LUPOS_PID_OFFSET_ns_common_inum as usize);
    assert!(
        core::mem::offset_of!(ns_common, __bindgen_anon_1.__ns_ref)
            == LUPOS_PID_OFFSET_ns_common___ns_ref as usize
    );
    assert!(
        core::mem::offset_of!(ns_common, __bindgen_anon_2.tree.ns_id)
            == LUPOS_PID_OFFSET_ns_common_ns_id as usize
    );
    assert!(
        core::mem::offset_of!(ns_common, __bindgen_anon_2.tree.__ns_ref_active)
            == LUPOS_PID_OFFSET_ns_common___ns_ref_active as usize
    );
    assert!(
        core::mem::offset_of!(
            ns_common,
            __bindgen_anon_2.tree.ns_unified_node.ns_list_entry
        ) == LUPOS_PID_OFFSET_ns_common_ns_unified_node_ns_list_entry as usize
    );
    assert!(
        core::mem::offset_of!(ns_common, __bindgen_anon_2.tree.ns_tree_node.ns_list_entry)
            == LUPOS_PID_OFFSET_ns_common_ns_tree_node_ns_list_entry as usize
    );
    assert!(
        core::mem::offset_of!(ns_common, __bindgen_anon_2.tree.ns_owner_node.ns_list_entry)
            == LUPOS_PID_OFFSET_ns_common_ns_owner_node_ns_list_entry as usize
    );
    assert!(
        core::mem::offset_of!(ns_common, __bindgen_anon_2.tree.ns_owner_root.ns_list_head)
            == LUPOS_PID_OFFSET_ns_common_ns_owner_root_ns_list_head as usize
    );
    assert!(core::mem::size_of::<ns_tree>() == LUPOS_PID_SIZE_ns_tree as usize);
    assert!(core::mem::align_of::<ns_tree>() == LUPOS_PID_ALIGN_ns_tree as usize);
    assert!(core::mem::offset_of!(ns_tree, ns_id) == LUPOS_PID_OFFSET_ns_tree_ns_id as usize);
    assert!(
        core::mem::offset_of!(ns_tree, __ns_ref_active)
            == LUPOS_PID_OFFSET_ns_tree___ns_ref_active as usize
    );
    assert!(
        core::mem::offset_of!(ns_tree, ns_unified_node)
            == LUPOS_PID_OFFSET_ns_tree_ns_unified_node as usize
    );
    assert!(
        core::mem::offset_of!(ns_tree, ns_tree_node)
            == LUPOS_PID_OFFSET_ns_tree_ns_tree_node as usize
    );
    assert!(
        core::mem::offset_of!(ns_tree, ns_owner_node)
            == LUPOS_PID_OFFSET_ns_tree_ns_owner_node as usize
    );
    assert!(
        core::mem::offset_of!(ns_tree, ns_owner_root)
            == LUPOS_PID_OFFSET_ns_tree_ns_owner_root as usize
    );
    assert!(core::mem::size_of::<pid>() == LUPOS_PID_SIZE_pid as usize);
    assert!(core::mem::align_of::<pid>() == LUPOS_PID_ALIGN_pid as usize);
    assert!(core::mem::offset_of!(pid, count) == LUPOS_PID_OFFSET_pid_count as usize);
    assert!(core::mem::offset_of!(pid, level) == LUPOS_PID_OFFSET_pid_level as usize);
    assert!(core::mem::offset_of!(pid, lock) == LUPOS_PID_OFFSET_pid_lock as usize);
    assert!(core::mem::offset_of!(pid, tasks) == LUPOS_PID_OFFSET_pid_tasks as usize);
    assert!(core::mem::offset_of!(pid, inodes) == LUPOS_PID_OFFSET_pid_inodes as usize);
    assert!(core::mem::offset_of!(pid, wait_pidfd) == LUPOS_PID_OFFSET_pid_wait_pidfd as usize);
    assert!(core::mem::offset_of!(pid, rcu) == LUPOS_PID_OFFSET_pid_rcu as usize);
    assert!(core::mem::offset_of!(pid, numbers) == LUPOS_PID_OFFSET_pid_numbers as usize);
    assert!(core::mem::size_of::<pid_namespace>() == LUPOS_PID_SIZE_pid_namespace as usize);
    assert!(core::mem::align_of::<pid_namespace>() == LUPOS_PID_ALIGN_pid_namespace as usize);
    assert!(
        core::mem::offset_of!(pid_namespace, idr) == LUPOS_PID_OFFSET_pid_namespace_idr as usize
    );
    assert!(
        core::mem::offset_of!(pid_namespace, pid_allocated)
            == LUPOS_PID_OFFSET_pid_namespace_pid_allocated as usize
    );
    assert!(
        core::mem::offset_of!(pid_namespace, child_reaper)
            == LUPOS_PID_OFFSET_pid_namespace_child_reaper as usize
    );
    assert!(
        core::mem::offset_of!(pid_namespace, pid_cachep)
            == LUPOS_PID_OFFSET_pid_namespace_pid_cachep as usize
    );
    assert!(
        core::mem::offset_of!(pid_namespace, level)
            == LUPOS_PID_OFFSET_pid_namespace_level as usize
    );
    assert!(
        core::mem::offset_of!(pid_namespace, pid_max)
            == LUPOS_PID_OFFSET_pid_namespace_pid_max as usize
    );
    assert!(
        core::mem::offset_of!(pid_namespace, parent)
            == LUPOS_PID_OFFSET_pid_namespace_parent as usize
    );
    assert!(
        core::mem::offset_of!(pid_namespace, user_ns)
            == LUPOS_PID_OFFSET_pid_namespace_user_ns as usize
    );
    assert!(core::mem::offset_of!(pid_namespace, ns) == LUPOS_PID_OFFSET_pid_namespace_ns as usize);
    assert!(
        core::mem::offset_of!(pid_namespace, work) == LUPOS_PID_OFFSET_pid_namespace_work as usize
    );
    assert!(core::mem::size_of::<signal_struct>() == LUPOS_PID_SIZE_signal_struct as usize);
    assert!(core::mem::align_of::<signal_struct>() == LUPOS_PID_ALIGN_signal_struct as usize);
    assert!(
        core::mem::offset_of!(signal_struct, pids) == LUPOS_PID_OFFSET_signal_struct_pids as usize
    );
    assert!(
        core::mem::offset_of!(signal_struct, exec_update_lock)
            == LUPOS_PID_OFFSET_signal_struct_exec_update_lock as usize
    );
    assert!(core::mem::size_of::<task_struct>() == LUPOS_PID_SIZE_task_struct as usize);
    assert!(core::mem::align_of::<task_struct>() == LUPOS_PID_ALIGN_task_struct as usize);
    assert!(
        core::mem::offset_of!(task_struct, thread_pid)
            == LUPOS_PID_OFFSET_task_struct_thread_pid as usize
    );
    assert!(
        core::mem::offset_of!(task_struct, pid_links)
            == LUPOS_PID_OFFSET_task_struct_pid_links as usize
    );
    assert!(
        core::mem::offset_of!(task_struct, signal) == LUPOS_PID_OFFSET_task_struct_signal as usize
    );
    assert!(core::mem::offset_of!(task_struct, pid) == LUPOS_PID_OFFSET_task_struct_pid as usize);
    assert!(
        core::mem::offset_of!(task_struct, flags) == LUPOS_PID_OFFSET_task_struct_flags as usize
    );
    assert!(core::mem::size_of::<upid>() == LUPOS_PID_SIZE_upid as usize);
    assert!(core::mem::align_of::<upid>() == LUPOS_PID_ALIGN_upid as usize);
    assert!(core::mem::offset_of!(upid, nr) == LUPOS_PID_OFFSET_upid_nr as usize);
    assert!(core::mem::offset_of!(upid, ns) == LUPOS_PID_OFFSET_upid_ns as usize);
};
#[cfg(CONFIG_SYSCTL)]
const _: () = {
    assert!(core::mem::size_of::<ctl_table>() == LUPOS_PID_SIZE_ctl_table as usize);
    assert!(core::mem::align_of::<ctl_table>() == LUPOS_PID_ALIGN_ctl_table as usize);
    assert!(
        core::mem::offset_of!(ctl_table, procname) == LUPOS_PID_OFFSET_ctl_table_procname as usize
    );
    assert!(core::mem::offset_of!(ctl_table, data) == LUPOS_PID_OFFSET_ctl_table_data as usize);
    assert!(core::mem::offset_of!(ctl_table, maxlen) == LUPOS_PID_OFFSET_ctl_table_maxlen as usize);
    assert!(core::mem::offset_of!(ctl_table, mode) == LUPOS_PID_OFFSET_ctl_table_mode as usize);
    assert!(
        core::mem::offset_of!(ctl_table, proc_handler)
            == LUPOS_PID_OFFSET_ctl_table_proc_handler as usize
    );
    assert!(core::mem::offset_of!(ctl_table, extra1) == LUPOS_PID_OFFSET_ctl_table_extra1 as usize);
    assert!(core::mem::offset_of!(ctl_table, extra2) == LUPOS_PID_OFFSET_ctl_table_extra2 as usize);
    assert!(core::mem::size_of::<ctl_table_header>() == LUPOS_PID_SIZE_ctl_table_header as usize);
    assert!(core::mem::align_of::<ctl_table_header>() == LUPOS_PID_ALIGN_ctl_table_header as usize);
    assert!(
        core::mem::offset_of!(ctl_table_header, ctl_table_arg)
            == LUPOS_PID_OFFSET_ctl_table_header_ctl_table_arg as usize
    );
    assert!(
        core::mem::offset_of!(ctl_table_header, set)
            == LUPOS_PID_OFFSET_ctl_table_header_set as usize
    );
    assert!(core::mem::size_of::<ctl_table_root>() == LUPOS_PID_SIZE_ctl_table_root as usize);
    assert!(core::mem::align_of::<ctl_table_root>() == LUPOS_PID_ALIGN_ctl_table_root as usize);
    assert!(
        core::mem::offset_of!(ctl_table_root, default_set)
            == LUPOS_PID_OFFSET_ctl_table_root_default_set as usize
    );
    assert!(
        core::mem::offset_of!(ctl_table_root, lookup)
            == LUPOS_PID_OFFSET_ctl_table_root_lookup as usize
    );
    assert!(
        core::mem::offset_of!(ctl_table_root, set_ownership)
            == LUPOS_PID_OFFSET_ctl_table_root_set_ownership as usize
    );
    assert!(
        core::mem::offset_of!(ctl_table_root, permissions)
            == LUPOS_PID_OFFSET_ctl_table_root_permissions as usize
    );
    assert!(core::mem::size_of::<ctl_table_set>() == LUPOS_PID_SIZE_ctl_table_set as usize);
    assert!(core::mem::align_of::<ctl_table_set>() == LUPOS_PID_ALIGN_ctl_table_set as usize);
    assert!(
        core::mem::offset_of!(ctl_table_set, is_seen)
            == LUPOS_PID_OFFSET_ctl_table_set_is_seen as usize
    );
    assert!(
        core::mem::offset_of!(ctl_table_set, dir) == LUPOS_PID_OFFSET_ctl_table_set_dir as usize
    );
    assert!(
        core::mem::offset_of!(pid_namespace, set) == LUPOS_PID_OFFSET_pid_namespace_set as usize
    );
    assert!(
        core::mem::offset_of!(pid_namespace, sysctls)
            == LUPOS_PID_OFFSET_pid_namespace_sysctls as usize
    );
};
