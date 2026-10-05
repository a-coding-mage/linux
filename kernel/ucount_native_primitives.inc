/* SPDX-License-Identifier: GPL-2.0-only */
/* Deliberately multiply included through UCOUNT_LEAF. Native leaves implement
 * field/macro/atomic/reference operations, never an owner traversal/rollback.
 */
UCOUNT_LEAF(struct user_namespace *, namespace, (struct ucounts *ucounts),
{
	return ucounts->ns;
})
UCOUNT_LEAF(struct ucounts *, namespace_ucounts, (struct user_namespace *ns),
{
	return ns->ucounts;
})
UCOUNT_LEAF(kuid_t, uid, (struct ucounts *ucounts),
{
	return ucounts->uid;
})
UCOUNT_LEAF(bool, uid_eq, (kuid_t left, kuid_t right),
{
	return uid_eq(left, right);
})
UCOUNT_LEAF(void, set_namespace, (struct ucounts *ucounts, struct user_namespace *ns),
{
	ucounts->ns = ns;
})
UCOUNT_LEAF(void, set_uid, (struct ucounts *ucounts, kuid_t uid),
{
	ucounts->uid = uid;
})
UCOUNT_LEAF(struct hlist_nulls_head *, hashentry,
	   (struct user_namespace *ns, kuid_t uid),
{
	return ucounts_hashentry(ns, uid);
})
UCOUNT_LEAF(void, rcu_read_lock, (void),
{
	rcu_read_lock();
})
UCOUNT_LEAF(void, rcu_read_unlock, (void),
{
	rcu_read_unlock();
})
UCOUNT_LEAF(struct hlist_nulls_node *, first_rcu, (struct hlist_nulls_head *head),
{
	/* Exact initial barrier and load of hlist_nulls_for_each_entry_rcu. */
	barrier();
	return rcu_dereference_raw(hlist_nulls_first_rcu(head));
})
UCOUNT_LEAF(struct hlist_nulls_node *, next_rcu, (struct hlist_nulls_node *pos),
{
	return rcu_dereference_raw(hlist_nulls_next_rcu(pos));
})
UCOUNT_LEAF(bool, is_nulls, (struct hlist_nulls_node *pos),
{
	return is_a_nulls(pos);
})
UCOUNT_LEAF(struct ucounts *, from_node, (struct hlist_nulls_node *pos),
{
	return hlist_nulls_entry(pos, struct ucounts, node);
})
UCOUNT_LEAF(void, lock_irq, (void),
{
	spin_lock_irq(&ucounts_lock);
})
UCOUNT_LEAF(void, unlock_irq, (void),
{
	spin_unlock_irq(&ucounts_lock);
})
UCOUNT_LEAF(unsigned long, lock_irqsave, (void),
{
	unsigned long flags;

	spin_lock_irqsave(&ucounts_lock, flags);
	return flags;
})
UCOUNT_LEAF(void, unlock_irqrestore, (unsigned long flags),
{
	spin_unlock_irqrestore(&ucounts_lock, flags);
})
UCOUNT_LEAF(void, add_head_rcu,
	   (struct ucounts *ucounts, struct hlist_nulls_head *head),
{
	hlist_nulls_add_head_rcu(&ucounts->node, head);
})
UCOUNT_LEAF(void, del_rcu, (struct ucounts *ucounts),
{
	hlist_nulls_del_rcu(&ucounts->node);
})
UCOUNT_LEAF(struct ucounts *, zalloc, (void),
{
	struct ucounts *new;

	return kzalloc_obj(*new);
})
UCOUNT_LEAF(void, free, (struct ucounts *ucounts),
{
	kfree(ucounts);
})
UCOUNT_LEAF(void, free_rcu, (struct ucounts *ucounts),
{
	kfree_rcu(ucounts, rcu);
})
UCOUNT_LEAF(void, ref_init, (struct ucounts *ucounts),
{
	rcuref_init(&ucounts->count, 1);
})
UCOUNT_LEAF(bool, ref_get, (struct ucounts *ucounts),
{
	return rcuref_get(&ucounts->count);
})
UCOUNT_LEAF(bool, ref_put, (struct ucounts *ucounts),
{
	return rcuref_put(&ucounts->count);
})
UCOUNT_LEAF(struct ucounts *, get_ucounts, (struct ucounts *ucounts),
{
	return get_ucounts(ucounts);
})
UCOUNT_LEAF(void, get_user_ns, (struct user_namespace *ns),
{
	get_user_ns(ns);
})
UCOUNT_LEAF(void, put_user_ns, (struct user_namespace *ns),
{
	put_user_ns(ns);
})
UCOUNT_LEAF(atomic_long_t *, counter, (struct ucounts *ucounts, enum ucount_type type),
{
	return &ucounts->ucount[type];
})
UCOUNT_LEAF(atomic_long_t *, rlimit_counter,
	   (struct ucounts *ucounts, enum rlimit_type type),
{
	return &ucounts->rlimit[type];
})
UCOUNT_LEAF(long, namespace_max, (struct user_namespace *ns, enum ucount_type type),
{
	return READ_ONCE(ns->ucount_max[type]);
})
UCOUNT_LEAF(long, rlimit_max, (struct user_namespace *ns, enum rlimit_type type),
{
	return get_userns_rlimit_max(ns, type);
})
UCOUNT_LEAF(long, rlimit_value, (struct ucounts *ucounts, enum rlimit_type type),
{
	return get_rlimit_value(ucounts, type);
})
UCOUNT_LEAF(enum rlimit_type, nproc_type, (void),
{
	return UCOUNT_RLIMIT_NPROC;
})
UCOUNT_LEAF(long, atomic_read, (const atomic_long_t *v),
{
	return atomic_long_read(v);
})
UCOUNT_LEAF(bool, atomic_try_cmpxchg, (atomic_long_t *v, long *old, long new),
{
	return atomic_long_try_cmpxchg(v, old, new);
})
UCOUNT_LEAF(void, atomic_dec, (atomic_long_t *v),
{
	atomic_long_dec(v);
})
UCOUNT_LEAF(long, atomic_dec_if_positive, (atomic_long_t *v),
{
	return atomic_long_dec_if_positive(v);
})
UCOUNT_LEAF(long, atomic_add_return, (long value, atomic_long_t *v),
{
	return atomic_long_add_return(value, v);
})
UCOUNT_LEAF(long, atomic_sub_return, (long value, atomic_long_t *v),
{
	return atomic_long_sub_return(value, v);
})
/* Four distinct once-state sites. Do not merge these identical expressions. */
UCOUNT_LEAF(void, warn_dec_ucount, (long dec),
{
	WARN_ON_ONCE(dec < 0);
})
UCOUNT_LEAF(void, warn_dec_rlimit, (long dec),
{
	WARN_ON_ONCE(dec < 0);
})
UCOUNT_LEAF(void, warn_do_dec_rlimit, (long dec),
{
	WARN_ON_ONCE(dec < 0);
})
UCOUNT_LEAF(void, warn_inc_rlimit_unwind, (long dec),
{
	WARN_ON_ONCE(dec < 0);
})

#ifdef CONFIG_SYSCTL
UCOUNT_LEAF(struct ctl_table_set *, current_set, (void),
{
	return &current_user_ns()->set;
})
UCOUNT_LEAF(struct user_namespace *, head_namespace, (struct ctl_table_header *head),
{
	return container_of(head->set, struct user_namespace, set);
})
UCOUNT_LEAF(bool, ns_capable_resource, (struct user_namespace *ns),
{
	return ns_capable_noaudit(ns, CAP_SYS_RESOURCE);
})
UCOUNT_LEAF(int, table_mode, (const struct ctl_table *table),
{
	return table->mode;
})
UCOUNT_LEAF(void, check_user_table, (void),
{
	BUILD_BUG_ON(ARRAY_SIZE(user_table) != UCOUNT_COUNTS);
})
UCOUNT_LEAF(void, setup_set, (struct user_namespace *ns),
{
	setup_sysctl_set(&ns->set, &set_root, set_is_seen);
})
UCOUNT_LEAF(struct ctl_table *, dup_user_table, (void),
{
	return kmemdup(user_table, sizeof(user_table), GFP_KERNEL);
})
UCOUNT_LEAF(void, set_table_data,
	   (struct ctl_table *tbl, unsigned int i, struct user_namespace *ns),
{
	tbl[i].data = &ns->ucount_max[i];
})
UCOUNT_LEAF(struct ctl_table_header *, register_table,
	   (struct user_namespace *ns, const struct ctl_table *tbl),
{
	return __register_sysctl_table(&ns->set, "user", tbl, ARRAY_SIZE(user_table));
})
UCOUNT_LEAF(struct ctl_table_header *, sysctls, (struct user_namespace *ns),
{
	return ns->sysctls;
})
UCOUNT_LEAF(void, set_sysctls,
	   (struct user_namespace *ns, struct ctl_table_header *header),
{
	ns->sysctls = header;
})
UCOUNT_LEAF(const struct ctl_table *, header_table, (struct ctl_table_header *header),
{
	return header->ctl_table_arg;
})
UCOUNT_LEAF(void, unregister_table, (struct ctl_table_header *header),
{
	unregister_sysctl_table(header);
})
UCOUNT_LEAF(void, retire_set, (struct user_namespace *ns),
{
	retire_sysctl_set(&ns->set);
})
UCOUNT_LEAF(void, free_table, (const struct ctl_table *tbl),
{
	kfree(tbl);
})
UCOUNT_LEAF(struct ctl_table_header *, register_user_root, (void),
{
	return register_sysctl_sz("user", empty, 0);
})
UCOUNT_LEAF(void, store_user_header, (struct ctl_table_header *header),
{
	user_header = header;
})
UCOUNT_LEAF(void, kmemleak_ignore, (struct ctl_table_header *header),
{
	kmemleak_ignore(header);
})
UCOUNT_LEAF(void, bug_user_header, (struct ctl_table_header *user_header),
{
	BUG_ON(!user_header);
})
UCOUNT_LEAF(void, bug_setup_initial_sysctls, (void),
{
	/* Preserve BUG_ON's native evaluation envelope. The exact-signature
	 * setup_userns_sysctls entry below dispatches solely to the Rust owner.
	 */
	BUG_ON(!setup_userns_sysctls(&init_user_ns));
})
#endif /* CONFIG_SYSCTL */
