/* SPDX-License-Identifier: GPL-2.0-only */
/* Deliberately multiply included declaration/definition list. No traversal,
 * request-validation, permission or cleanup decision algorithm belongs here.
 */
NSTREE_LEAF(void, writer_lock, (void), { write_seqlock(&ns_tree_lock); })
NSTREE_LEAF(void, writer_unlock, (void), { write_sequnlock(&ns_tree_lock); })
NSTREE_LEAF(void, reader_lock, (void), { read_seqlock_excl(&ns_tree_lock); })
NSTREE_LEAF(void, reader_unlock, (void), { read_sequnlock_excl(&ns_tree_lock); })
NSTREE_LEAF(unsigned int, seqbegin, (void), { return read_seqbegin(&ns_tree_lock); })
NSTREE_LEAF(bool, seqretry, (unsigned int seq), { return read_seqretry(&ns_tree_lock, seq); })
NSTREE_LEAF(void, rcu_lock, (void), { rcu_read_lock(); })
NSTREE_LEAF(void, rcu_unlock, (void), { rcu_read_unlock(); })
NSTREE_LEAF(struct ns_tree_root *, unified_root, (void), { return &ns_unified_root; })
NSTREE_LEAF(struct rb_node *, root_node, (struct ns_tree_root *root), { return root->ns_rb.rb_node; })
NSTREE_LEAF(const struct list_head *, list_head, (struct ns_tree_root *root), { return &root->ns_list_head; })
NSTREE_LEAF(struct ns_tree_root *, owner_root, (struct ns_common *ns), { return &ns->ns_owner_root; })
NSTREE_LEAF(struct rb_node *, left, (const struct rb_node *node), { return node->rb_left; })
NSTREE_LEAF(struct rb_node *, right, (const struct rb_node *node), { return node->rb_right; })
NSTREE_LEAF(struct ns_common *, node_ns, (const struct rb_node *node), { return rb_entry(node, struct ns_common, ns_tree_node.ns_node); })
NSTREE_LEAF(struct ns_common *, node_unified, (const struct rb_node *node), { return rb_entry(node, struct ns_common, ns_unified_node.ns_node); })
NSTREE_LEAF(struct ns_common *, node_owner, (const struct rb_node *node), { return rb_entry(node, struct ns_common, ns_owner_node.ns_node); })
NSTREE_LEAF(u64, id, (const struct ns_common *ns), { return ns->ns_id; })
NSTREE_LEAF(void, set_id, (struct ns_common *ns, u64 id), { ns->ns_id = id; })
NSTREE_LEAF(u32, type, (const struct ns_common *ns), { return ns->ns_type; })
NSTREE_LEAF(const struct proc_ns_operations *, ops, (const struct ns_common *ns), { return ns->ops; })
NSTREE_LEAF(struct user_namespace *, owner, (const struct proc_ns_operations *ops, struct ns_common *ns), { return ops->owner(ns); })
NSTREE_LEAF(void, put, (const struct proc_ns_operations *ops, struct ns_common *ns), { ops->put(ns); })
NSTREE_LEAF(struct ns_common *, get, (struct ns_common *ns), { return ns_get_unless_inactive(ns); })
NSTREE_LEAF(struct ns_common *, user_common, (struct user_namespace *ns), { return to_ns_common(ns); })
NSTREE_LEAF(struct user_namespace *, to_user, (struct ns_common *ns), { return to_user_ns(ns); })
NSTREE_LEAF(struct user_namespace *, current_user, (void), { return current_user_ns(); })
NSTREE_LEAF(void, put_user, (struct user_namespace *ns), { put_user_ns(ns); })
NSTREE_LEAF(bool, is_current, (struct ns_common *ns), { return is_current_namespace(ns); })
NSTREE_LEAF(bool, may_see_all, (void), { return may_see_all_namespaces(); })
NSTREE_LEAF(struct ns_tree_node *, type_node, (struct ns_common *ns), { return &ns->ns_tree_node; })
NSTREE_LEAF(struct ns_tree_node *, unified_node, (struct ns_common *ns), { return &ns->ns_unified_node; })
NSTREE_LEAF(struct ns_tree_node *, owner_node, (struct ns_common *ns), { return &ns->ns_owner_node; })
NSTREE_LEAF(void, clear_node, (struct ns_tree_node *node), { RB_CLEAR_NODE(&node->ns_node); })
NSTREE_LEAF(void, init_list, (struct ns_tree_node *node), { INIT_LIST_HEAD(&node->ns_list_entry); })
NSTREE_LEAF(void, init_root_rb, (struct ns_tree_root *root), { root->ns_rb = RB_ROOT; })
NSTREE_LEAF(void, init_root_list, (struct ns_tree_root *root), { INIT_LIST_HEAD(&root->ns_list_head); })
NSTREE_LEAF(bool, empty_node, (const struct ns_tree_node *node), { return RB_EMPTY_NODE(&node->ns_node); })
NSTREE_LEAF(struct rb_node *, find_add, (struct ns_tree_node *node, struct ns_tree_root *root, const struct rust_nstree_cmp_context *cmp), { return rb_find_add_rcu(&node->ns_node, &root->ns_rb, cmp->cmp); })
NSTREE_LEAF(struct rb_node *, prev, (const struct ns_tree_node *node), { return rb_prev(&node->ns_node); })
NSTREE_LEAF(struct ns_tree_node *, tree_node, (const struct rb_node *node), { return rb_entry(node, struct ns_tree_node, ns_node); })
NSTREE_LEAF(void, add_head, (struct ns_tree_node *node, struct ns_tree_root *root), { list_add_rcu(&node->ns_list_entry, &root->ns_list_head); })
NSTREE_LEAF(void, add_after, (struct ns_tree_node *node, struct ns_tree_node *prev), { list_add_rcu(&node->ns_list_entry, &prev->ns_list_entry); })
NSTREE_LEAF(void, erase, (struct ns_tree_node *node, struct ns_tree_root *root), { rb_erase(&node->ns_node, &root->ns_rb); })
NSTREE_LEAF(void, del_list, (struct ns_tree_node *node), { list_bidir_del_rcu(&node->ns_list_entry); })
NSTREE_LEAF(const struct rust_nstree_cmp_context *, type_cmp, (void), { return &type_cmp; })
NSTREE_LEAF(const struct rust_nstree_cmp_context *, unified_cmp, (void), { return &unified_cmp; })
NSTREE_LEAF(const struct rust_nstree_cmp_context *, owner_cmp, (void), { return &owner_cmp; })
NSTREE_LEAF(u64, key_id, (const void *key), { return *(const u64 *)key; })
NSTREE_LEAF(struct rb_node *, find_type, (const u64 *id, struct ns_tree_root *root), { return rb_find_rcu(id, &root->ns_rb, ns_find); })
NSTREE_LEAF(struct rb_node *, find_unified_node, (const u64 *id), { return rb_find_rcu(id, &ns_unified_root.ns_rb, ns_find_unified); })
NSTREE_LEAF(struct list_head *, previous_entry, (struct ns_common *ns), { return rcu_dereference(list_bidir_prev_rcu(&ns->ns_tree_node.ns_list_entry)); })
NSTREE_LEAF(struct list_head *, next_entry, (struct ns_common *ns), { return rcu_dereference(list_next_rcu(&ns->ns_tree_node.ns_list_entry)); })
NSTREE_LEAF(bool, entry_is_head, (const struct list_head *entry, const struct list_head *head), { return list_is_head(entry, head); })
NSTREE_LEAF(struct ns_common *, entry_ns, (struct list_head *entry), { return list_entry_rcu(entry, struct ns_common, ns_tree_node.ns_list_entry); })
NSTREE_LEAF(struct ns_common *, error, (long err), { return ERR_PTR(err); })
NSTREE_LEAF(u64, next_id, (void), { return atomic64_inc_return(&namespace_cookie); })

/* Each warning is its own native once site; preserve the original diagnostic
 * expression rather than merging unrelated reports into one generic bool site.
 */
NSTREE_LEAF(void, warn_add_id, (struct ns_common *ns), { VFS_WARN_ON_ONCE(!ns->ns_id); })
NSTREE_LEAF(void, warn_add_owner, (const struct proc_ns_operations *ops), { VFS_WARN_ON_ONCE(!ops->owner); })
NSTREE_LEAF(void, warn_add_owner_type, (struct ns_common *owner), { VFS_WARN_ON_ONCE(owner->ns_type != CLONE_NEWUSER); })
NSTREE_LEAF(void, warn_add_initial, (struct ns_common *ns), { VFS_WARN_ON_ONCE(ns != to_ns_common(&init_user_ns)); })
NSTREE_LEAF(void, warn_add_duplicate, (struct rb_node *node), { VFS_WARN_ON_ONCE(node); })
NSTREE_LEAF(void, warn_remove_node, (struct ns_common *ns), { VFS_WARN_ON_ONCE(ns_tree_node_empty(&ns->ns_tree_node)); })
NSTREE_LEAF(void, warn_remove_list, (struct ns_common *ns), { VFS_WARN_ON_ONCE(list_empty(&ns->ns_tree_node.ns_list_entry)); })
NSTREE_LEAF(void, warn_lookup_owner_type, (struct ns_common *owner), { VFS_WARN_ON_ONCE(owner->ns_type != CLONE_NEWUSER); })
NSTREE_LEAF(void, warn_userns_id, (const u64 *id), { VFS_WARN_ON_ONCE(!*id); })
NSTREE_LEAF(void, warn_lookup_rcu, (void), { RCU_LOCKDEP_WARN(!rcu_read_lock_held(), "suspicious ns_tree_lookup_rcu() usage"); })
NSTREE_LEAF(void, warn_adjoined_rcu, (void), { RCU_LOCKDEP_WARN(!rcu_read_lock_held(), "suspicious ns_tree_adjoined_rcu() usage"); })

/* C's list_entry_rcu forms may represent the sentinel container. Rust never
 * dereferences that synthetic ns_common; head tests also remain native leaves.
 */
NSTREE_LEAF(struct ns_common *, first_type, (const struct list_head *head), { return list_entry_rcu(head->next, struct ns_common, ns_tree_node.ns_list_entry); })
NSTREE_LEAF(struct ns_common *, first_unified, (const struct list_head *head), { return list_entry_rcu(head->next, struct ns_common, ns_unified_node.ns_list_entry); })
NSTREE_LEAF(struct ns_common *, first_owner, (const struct list_head *head), { return list_entry_rcu(head->next, struct ns_common, ns_owner_node.ns_list_entry); })
NSTREE_LEAF(struct ns_common *, next_type, (struct ns_common *ns), { return list_entry_rcu(ns->ns_tree_node.ns_list_entry.next, struct ns_common, ns_tree_node.ns_list_entry); })
NSTREE_LEAF(struct ns_common *, next_unified, (struct ns_common *ns), { return list_entry_rcu(ns->ns_unified_node.ns_list_entry.next, struct ns_common, ns_unified_node.ns_list_entry); })
NSTREE_LEAF(struct ns_common *, next_owner, (struct ns_common *ns), { return list_entry_rcu(ns->ns_owner_node.ns_list_entry.next, struct ns_common, ns_owner_node.ns_list_entry); })
NSTREE_LEAF(bool, type_is_head, (struct ns_common *ns, const struct list_head *head), { return &ns->ns_tree_node.ns_list_entry == head; })
NSTREE_LEAF(bool, unified_is_head, (struct ns_common *ns, const struct list_head *head), { return &ns->ns_unified_node.ns_list_entry == head; })
NSTREE_LEAF(bool, owner_is_head, (struct ns_common *ns, const struct list_head *head), { return &ns->ns_owner_node.ns_list_entry == head; })
NSTREE_LEAF(unsigned int, hweight, (u32 type), { return hweight32(type); })

/* Exact user-address operations; __user stays visible to native checking.
 * Bounds/order/error decisions are in Rust, including get_user's EFAULT map.
 */
NSTREE_LEAF(int, get_size, (const struct ns_id_req __user *req, size_t *size), {
	BUILD_BUG_ON(sizeof(struct ns_id_req) != NS_ID_REQ_SIZE_VER0);
	return get_user(*size, &req->size);
})
NSTREE_LEAF(size_t, page_size, (void), { return PAGE_SIZE; })
NSTREE_LEAF(void, zero_request, (struct ns_id_req *req), { memset(req, 0, sizeof(*req)); })
NSTREE_LEAF(int, copy_request, (struct ns_id_req *dst, const struct ns_id_req __user *src, size_t size), { return copy_struct_from_user(dst, sizeof(*dst), src, size); })
NSTREE_LEAF(u32, request_spare, (const struct ns_id_req *req), { return req->spare; })
NSTREE_LEAF(u32, request_type, (const struct ns_id_req *req), { return req->ns_type; })
NSTREE_LEAF(u64, request_id, (const struct ns_id_req *req), { return req->ns_id; })
NSTREE_LEAF(u64, request_user, (const struct ns_id_req *req), { return req->user_ns_id; })
NSTREE_LEAF(bool, output_access_ok, (u64 __user *ids, size_t count), { return access_ok(ids, count * sizeof(*ids)); })
/* Keep the value operand inside put_user: native might_fault/access_ok ordering
 * and conditional operand evaluation must not become a Rust-side preload.
 */
NSTREE_LEAF(int, store_id, (const struct ns_common *valid, u64 __user *ids, ssize_t index), { return put_user(valid->ns_id, ids + index); })
