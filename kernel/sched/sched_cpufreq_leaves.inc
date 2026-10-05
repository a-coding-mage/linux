/* SPDX-License-Identifier: GPL-2.0 */
/* Native ABI/storage/inline/macro leaves, not governor algorithm owners.
 * Included for declarations by the binding header and definitions by native C.
 */
SUGOV_LEAF(bool, limits_read, (struct sugov_policy *p),
{ return unlikely(READ_ONCE(p->limits_changed)); })
SUGOV_LEAF(void, limits_write, (struct sugov_policy *p, bool value),
{ WRITE_ONCE(p->limits_changed, value); })
SUGOV_LEAF(void, full_barrier, (void), { smp_mb(); })
SUGOV_LEAF(void, write_barrier, (void), { smp_wmb(); })
SUGOV_LEAF(unsigned int, policy_cpu, (struct cpufreq_policy *p), { return p->cpu; })
SUGOV_LEAF(unsigned int, policy_max_freq, (struct cpufreq_policy *p),
{ return p->cpuinfo.max_freq; })
SUGOV_LEAF(unsigned int, policy_cur, (struct cpufreq_policy *p), { return p->cur; })
SUGOV_LEAF(bool, policy_fast, (struct cpufreq_policy *p),
{ return p->fast_switch_enabled; })
SUGOV_LEAF(bool, policy_shared, (struct cpufreq_policy *p),
{ return policy_is_shared(p); })
SUGOV_LEAF(struct sugov_policy *, policy_data, (struct cpufreq_policy *p),
{ return p->governor_data; })
SUGOV_LEAF(void, policy_set_data, (struct cpufreq_policy *p, struct sugov_policy *sg),
{ p->governor_data = sg; })
SUGOV_LEAF(struct cpufreq_governor *, policy_governor, (struct cpufreq_policy *p),
{ return p->governor; })
SUGOV_LEAF(unsigned int, first_cpu, (struct cpufreq_policy *p),
{ return find_next_bit(cpumask_bits(p->cpus), small_cpumask_bits, 0); })
SUGOV_LEAF(unsigned int, next_cpu, (struct cpufreq_policy *p, unsigned int cpu),
{ return find_next_bit(cpumask_bits(p->cpus), small_cpumask_bits, cpu + 1); })
/* Keep the for_each_cpu/for_each_set_bit scan bound, including small masks. */
SUGOV_LEAF(bool, valid_cpu, (unsigned int cpu), { return cpu < small_cpumask_bits; })
SUGOV_LEAF(unsigned int, arch_ref_freq, (unsigned int cpu),
{ return arch_scale_freq_ref(cpu); })
SUGOV_LEAF(bool, arch_invariant, (void), { return arch_scale_freq_invariant(); })
SUGOV_LEAF(unsigned long, arch_capacity, (unsigned int cpu),
{ return arch_scale_cpu_capacity(cpu); })
SUGOV_LEAF(unsigned long, map_freq,
(unsigned long util, unsigned long freq, unsigned long capacity),
{ return map_util_freq(util, freq, capacity); })
SUGOV_LEAF(unsigned long, map_perf, (unsigned long util),
{ return map_util_perf(util); })
SUGOV_LEAF(unsigned long, scx_target, (unsigned int cpu),
{ return scx_cpuperf_target(cpu); })
SUGOV_LEAF(bool, scx_all, (void), { return scx_switched_all(); })
SUGOV_LEAF(unsigned long, dl_bw, (unsigned int cpu),
{ return cpu_bw_dl(cpu_rq(cpu)); })
#ifdef CONFIG_NO_HZ_COMMON
SUGOV_LEAF(bool, capped, (unsigned int cpu),
{ return uclamp_rq_is_capped(cpu_rq(cpu)); })
SUGOV_LEAF(unsigned long, idle_calls, (unsigned int cpu),
{ return tick_nohz_get_idle_calls_cpu(cpu); })
#endif
SUGOV_LEAF(struct sugov_cpu *, cpu_storage, (unsigned int cpu),
{ return &per_cpu(sugov_cpu, cpu); })
SUGOV_LEAF(void, cpu_zero, (struct sugov_cpu *cpu),
{ memset(cpu, 0, sizeof(*cpu)); })
SUGOV_LEAF(struct sugov_cpu *, cpu_from_hook, (struct update_util_data *hook),
{ return container_of(hook, struct sugov_cpu, update_util); })
SUGOV_LEAF(struct sugov_policy *, policy_from_work, (struct kthread_work *work),
{ return container_of(work, struct sugov_policy, work); })
SUGOV_LEAF(struct sugov_policy *, policy_from_irq, (struct irq_work *work),
{ return container_of(work, struct sugov_policy, irq_work); })
SUGOV_LEAF(struct sugov_tunables *, tunables_from_attr, (struct gov_attr_set *attr),
{ return container_of(attr, struct sugov_tunables, attr_set); })
SUGOV_LEAF(struct sugov_tunables *, tunables_from_kobj, (struct kobject *kobj),
{ return container_of(to_gov_attr_set(kobj), struct sugov_tunables, attr_set); })
SUGOV_LEAF(struct sugov_policy *, policy_alloc, (void),
{ return kzalloc_obj(struct sugov_policy); })
SUGOV_LEAF(void, policy_free, (struct sugov_policy *p), { kfree(p); })
SUGOV_LEAF(struct sugov_tunables *, tunables_alloc, (void),
{ return kzalloc_obj(struct sugov_tunables); })
SUGOV_LEAF(void, tunables_free, (struct sugov_tunables *t), { kfree(t); })
SUGOV_LEAF(void, update_lock_init, (struct sugov_policy *p),
{ raw_spin_lock_init(&p->update_lock); })
SUGOV_LEAF(void, update_lock, (struct sugov_policy *p),
{ raw_spin_lock(&p->update_lock); })
SUGOV_LEAF(void, update_unlock, (struct sugov_policy *p),
{ raw_spin_unlock(&p->update_lock); })
SUGOV_LEAF(unsigned long, update_lock_irqsave, (struct sugov_policy *p),
{ unsigned long flags; raw_spin_lock_irqsave(&p->update_lock, flags); return flags; })
SUGOV_LEAF(void, update_unlock_irqrestore, (struct sugov_policy *p, unsigned long flags),
{ raw_spin_unlock_irqrestore(&p->update_lock, flags); })
SUGOV_LEAF(void, work_lock_init, (struct sugov_policy *p),
{ mutex_init(&p->work_lock); })
SUGOV_LEAF(void, work_lock, (struct sugov_policy *p), { mutex_lock(&p->work_lock); })
SUGOV_LEAF(void, work_unlock, (struct sugov_policy *p), { mutex_unlock(&p->work_lock); })
SUGOV_LEAF(void, work_lock_destroy, (struct sugov_policy *p),
{ mutex_destroy(&p->work_lock); })
SUGOV_LEAF(void, queue_irq, (struct sugov_policy *p), { irq_work_queue(&p->irq_work); })
SUGOV_LEAF(void, queue_work, (struct sugov_policy *p),
{ kthread_queue_work(&p->worker, &p->work); })
SUGOV_LEAF(void, work_init, (struct sugov_policy *p),
{ kthread_init_work(&p->work, lupos_sugov_work); })
SUGOV_LEAF(void, worker_init, (struct sugov_policy *p),
{ kthread_init_worker(&p->worker); })
SUGOV_LEAF(void, irq_init, (struct sugov_policy *p),
{ init_irq_work(&p->irq_work, lupos_sugov_irq_work); })
SUGOV_LEAF(struct task_struct *, thread_create, (struct sugov_policy *p),
{ return kthread_create(kthread_worker_fn, &p->worker, "sugov:%d",
			cpumask_first(p->policy->related_cpus)); })
SUGOV_LEAF(bool, thread_is_err, (struct task_struct *thread), { return IS_ERR(thread); })
SUGOV_LEAF(int, thread_error, (struct task_struct *thread), { return PTR_ERR(thread); })
SUGOV_LEAF(void, thread_create_error, (struct task_struct *thread),
{ pr_err("failed to create sugov thread: %pe\n", thread); })
SUGOV_LEAF(void, thread_sched_error, (void),
{ pr_warn("%s: failed to set SCHED_DEADLINE\n", "sugov_kthread_create"); })
SUGOV_LEAF(int, thread_setattr, (struct task_struct *thread),
{
	/* This is the original native ABI object, not a null/fabricated attr. */
	struct sched_attr attr = {
		.size = sizeof(struct sched_attr),
		.sched_policy = SCHED_DEADLINE,
		.sched_flags = SCHED_FLAG_SUGOV,
		.sched_nice = 0,
		.sched_priority = 0,
		.sched_runtime = NSEC_PER_MSEC,
		.sched_deadline = 10 * NSEC_PER_MSEC,
		.sched_period = 10 * NSEC_PER_MSEC,
	};
	return sched_setattr_nocheck(thread, &attr);
})
SUGOV_LEAF(void, thread_allowed, (struct sugov_policy *p),
{ set_cpus_allowed_ptr(p->thread, p->policy->related_cpus); })
SUGOV_LEAF(void, thread_bind, (struct sugov_policy *p),
{ kthread_bind_mask(p->thread, p->policy->related_cpus); })
SUGOV_LEAF(void, worker_flush, (struct sugov_policy *p),
{ kthread_flush_worker(&p->worker); })
SUGOV_LEAF(void, irq_sync, (struct sugov_policy *p), { irq_work_sync(&p->irq_work); })
SUGOV_LEAF(void, work_cancel_sync, (struct sugov_policy *p),
{ kthread_cancel_work_sync(&p->work); })
SUGOV_LEAF(void, global_lock, (void), { mutex_lock(&global_tunables_lock); })
SUGOV_LEAF(void, global_unlock, (void), { mutex_unlock(&global_tunables_lock); })
SUGOV_LEAF(struct sugov_tunables *, global_read, (void), { return global_tunables; })
SUGOV_LEAF(void, global_write, (struct sugov_tunables *t), { global_tunables = t; })
SUGOV_LEAF(bool, warn_per_policy, (bool condition), { return WARN_ON(condition); })
SUGOV_LEAF(void, attr_init, (struct sugov_tunables *t, struct sugov_policy *p),
{ gov_attr_set_init(&t->attr_set, &p->tunables_hook); })
SUGOV_LEAF(void, attr_get, (struct sugov_tunables *t, struct sugov_policy *p),
{ gov_attr_set_get(&t->attr_set, &p->tunables_hook); })
SUGOV_LEAF(unsigned int, attr_put, (struct sugov_tunables *t, struct sugov_policy *p),
{ return gov_attr_set_put(&t->attr_set, &p->tunables_hook); })
SUGOV_LEAF(struct sugov_policy *, attr_first, (struct gov_attr_set *a),
{
	struct list_head *node = a->policy_list.next;
	return node == &a->policy_list ? NULL : list_entry(node, struct sugov_policy, tunables_hook);
})
SUGOV_LEAF(struct sugov_policy *, attr_next, (struct gov_attr_set *a, struct sugov_policy *p),
{
	struct list_head *node = p->tunables_hook.next;
	return node == &a->policy_list ? NULL : list_entry(node, struct sugov_policy, tunables_hook);
})
SUGOV_LEAF(int, tunables_kobject_add, (struct sugov_tunables *t, struct cpufreq_policy *p),
{ return kobject_init_and_add(&t->attr_set.kobj, &sugov_tunables_ktype,
		get_governor_parent_kobj(p), "%s", schedutil_gov.name); })
SUGOV_LEAF(void, tunables_kobject_put, (struct sugov_tunables *t),
{ kobject_put(&t->attr_set.kobj); })
SUGOV_LEAF(ssize_t, rate_emit, (char *buf, unsigned int rate),
{ return sysfs_emit(buf, "%u\n", rate); })
SUGOV_LEAF(void, init_error, (int error),
{ pr_err("initialization failed (error %d)\n", error); })
SUGOV_LEAF(void, policy_apply_limits, (struct cpufreq_policy *p),
{ cpufreq_policy_apply_limits(p); })
SUGOV_LEAF(struct cpufreq_governor *, governor, (void), { return &schedutil_gov; })
SUGOV_LEAF(void, rebuild_domains, (void), { em_rebuild_sched_domains(); })
SUGOV_LEAF(void, add_shared_hook, (int cpu, struct update_util_data *data),
{ cpufreq_add_update_util_hook(cpu, data, lupos_sugov_update_shared); })
SUGOV_LEAF(void, add_single_perf_hook, (int cpu, struct update_util_data *data),
{ cpufreq_add_update_util_hook(cpu, data, lupos_sugov_update_single_perf); })
SUGOV_LEAF(void, add_single_freq_hook, (int cpu, struct update_util_data *data),
{ cpufreq_add_update_util_hook(cpu, data, lupos_sugov_update_single_freq); })
