/* SPDX-License-Identifier: GPL-2.0-only */
/* Deliberately guardless X-macro: declarations for bindgen, definitions for C.
 * Every RVM entry is remaining native executable work, not metadata-only. */
RVM(void, rust_vmstat_atomic_add, (long value, atomic_long_t *p), { atomic_long_add(value, p); })
RVM(long, rust_vmstat_atomic_read, (const atomic_long_t *p), { return atomic_long_read(p); })
RVM(void, rust_vmstat_atomic_set, (atomic_long_t *p, long value), { atomic_long_set(p, value); })
RVM(void, rust_vmstat_cpus_read_lock, (void), { cpus_read_lock(); })
RVM(void, rust_vmstat_cpus_read_unlock, (void), { cpus_read_unlock(); })
RVM(struct zone *, rust_vmstat_first_zone, (void), { return first_online_pgdat()->node_zones; })
RVM(int, rust_vmstat_next_online_cpu, (int cpu), { return cpumask_next(cpu, cpu_online_mask); })
RVM(int, rust_vmstat_nr_cpu_ids, (void), { return nr_cpu_ids; })
RVM(struct per_cpu_zonestat *, rust_vmstat_zone_cpu, (struct per_cpu_zonestat __percpu *p, int cpu), { return per_cpu_ptr(p, cpu); })
RVM(struct per_cpu_pages *, rust_vmstat_pages_cpu, (struct per_cpu_pages __percpu *p, int cpu), { return per_cpu_ptr(p, cpu); })
RVM(struct zone *, rust_vmstat_page_zone, (const struct page *p), { return page_zone(p); })
RVM(struct pglist_data *, rust_vmstat_page_pgdat, (const struct page *p), { return page_pgdat(p); })
RVM(struct pglist_data *, rust_vmstat_node_data, (int node), { return NODE_DATA(node); })
RVM(unsigned long, rust_vmstat_min_wmark, (const struct zone *z), { return min_wmark_pages(z); })
RVM(unsigned long, rust_vmstat_low_wmark, (const struct zone *z), { return low_wmark_pages(z); })
RVM(unsigned long, rust_vmstat_high_wmark, (const struct zone *z), { return high_wmark_pages(z); })
RVM(unsigned long, rust_vmstat_promo_wmark, (const struct zone *z), { return promo_wmark_pages(z); })
RVM(unsigned long, rust_vmstat_managed_pages, (struct zone *z), { return zone_managed_pages(z); })
RVM(unsigned long, rust_vmstat_cma_pages, (struct zone *z), { return zone_cma_pages(z); })
RVM(bool, rust_vmstat_item_in_bytes, (int item), { return vmstat_item_in_bytes(item); })
RVM(bool, rust_vmstat_item_print_in_thp, (enum node_stat_item item), { return vmstat_item_print_in_thp(item); })
RVM(unsigned long, rust_vmstat_zone_end_pfn, (const struct zone *z), { return zone_end_pfn(z); })
RVM(unsigned long, rust_vmstat_pageblock_nr_pages, (void), { return pageblock_nr_pages; })
RVM(unsigned int, rust_vmstat_pageblock_order, (void), { return pageblock_order; })
RVM(unsigned long, rust_vmstat_get_pageblock_migratetype, (const struct page *p), { return get_pageblock_migratetype(p); })
RVM(struct page *, rust_vmstat_pfn_to_online_page, (unsigned long pfn), { return pfn_to_online_page(pfn); })
RVM(bool, rust_vmstat_node_has_memory, (int node), { return node_state(node, N_MEMORY); })
RVM(void, rust_vmstat_cond_resched, (void), { cond_resched(); })
RVM(unsigned long, rust_vmstat_zone_lock_irqsave, (struct zone *z), { unsigned long flags; spin_lock_irqsave(&z->lock, flags); return flags; })
RVM(void, rust_vmstat_zone_unlock_irqrestore, (struct zone *z, unsigned long flags), { spin_unlock_irqrestore(&z->lock, flags); })
RVM(void, rust_vmstat_zone_lock_irq, (struct zone *z), { spin_lock_irq(&z->lock); })
RVM(void, rust_vmstat_zone_unlock_irq, (struct zone *z), { spin_unlock_irq(&z->lock); })
RVM(unsigned long, rust_vmstat_free_blocks, (struct zone *z, unsigned int order), { return data_race(z->free_area[order].nr_free); })
RVM(u64, rust_vmstat_div_u64, (u64 dividend, u32 divisor), { return div_u64(dividend, divisor); })
RVM(void *, rust_vmstat_kmalloc_array, (size_t n, size_t size, gfp_t flags), { return kmalloc_array(n, size, flags); })
RVM(void *, rust_vmstat_err_ptr, (long error), { return ERR_PTR(error); })
RVM(struct workqueue_struct * __init, rust_vmstat_alloc_workqueue, (const char *name, unsigned int flags, int max_active), { return alloc_workqueue("%s", flags, max_active, name); })
#ifdef CONFIG_VM_EVENT_COUNTERS
RVM(struct vm_event_state *, rust_vmstat_event_cpu, (int cpu), { return &per_cpu(vm_event_states, cpu); })
RVM(void, rust_vmstat_count_events, (enum vm_event_item item, long delta), { count_vm_events(item, delta); })
#endif
#ifdef CONFIG_NUMA
RVM(unsigned long, rust_vmstat_xchg_ulong, (unsigned long *p, unsigned long value), { return xchg(p, value); })
RVM(void, rust_vmstat_warn_node_read_bytes, (bool condition), { VM_WARN_ON_ONCE(condition); })
#ifdef CONFIG_PROC_FS
RVM(void, rust_vmstat_numa_lock, (void), { mutex_lock(&vm_numa_stat_lock); })
RVM(void, rust_vmstat_numa_unlock, (void), { mutex_unlock(&vm_numa_stat_lock); })
RVM(void, rust_vmstat_numa_enable, (void), { static_branch_enable(&vm_numa_stat_key); })
RVM(void, rust_vmstat_numa_disable, (void), { static_branch_disable(&vm_numa_stat_key); })
RVM(void, rust_vmstat_log_numa_enabled, (void), { pr_info("enable numa statistics\n"); })
RVM(void, rust_vmstat_log_numa_disabled, (void), { pr_info("disable numa statistics, and clear numa counters\n"); })
#endif
#endif
#ifdef CONFIG_SMP
RVM(unsigned int, rust_vmstat_num_online_cpus, (void), { return num_online_cpus(); })
RVM(int, rust_vmstat_fls, (unsigned int word), { return fls(word); })
RVM(struct per_cpu_nodestat *, rust_vmstat_node_cpu, (struct per_cpu_nodestat __percpu *p, int cpu), { return per_cpu_ptr(p, cpu); })
RVM(struct per_cpu_pages *, rust_vmstat_this_pages, (struct per_cpu_pages __percpu *p), { return this_cpu_ptr(p); })
RVM(void, rust_vmstat_preempt_disable_nested, (void), { preempt_disable_nested(); })
RVM(void, rust_vmstat_preempt_enable_nested, (void), { preempt_enable_nested(); })
RVM(s8, rust_vmstat_raw_read_s8, (const s8 __percpu *p), { return __this_cpu_read(*p); })
RVM(void, rust_vmstat_raw_write_s8, (s8 __percpu *p, s8 value), { __this_cpu_write(*p, value); })
RVM(s8, rust_vmstat_raw_inc_s8, (s8 __percpu *p), { return __this_cpu_inc_return(*p); })
RVM(s8, rust_vmstat_raw_dec_s8, (s8 __percpu *p), { return __this_cpu_dec_return(*p); })
RVM(s8, rust_vmstat_this_xchg_s8, (s8 __percpu *p, s8 value), { return this_cpu_xchg(*p, value); })
#ifdef CONFIG_HAVE_CMPXCHG_LOCAL
RVM(s8, rust_vmstat_this_read_s8, (const s8 __percpu *p), { return this_cpu_read(*p); })
RVM(bool, rust_vmstat_this_cmpxchg_s8, (s8 __percpu *p, s8 *old, s8 value), { return this_cpu_try_cmpxchg(*p, old, value); })
#else
RVM(unsigned long, rust_vmstat_irq_save, (void), { unsigned long flags; local_irq_save(flags); return flags; })
RVM(void, rust_vmstat_irq_restore, (unsigned long flags), { local_irq_restore(flags); })
#endif
RVM(void, rust_vmstat_warn_mod_node_bytes, (bool condition), { VM_WARN_ON_ONCE(condition); })
RVM(void, rust_vmstat_warn_inc_node_bytes, (bool condition), { VM_WARN_ON_ONCE(condition); })
RVM(void, rust_vmstat_warn_dec_node_bytes, (bool condition), { VM_WARN_ON_ONCE(condition); })
#ifdef CONFIG_HAVE_CMPXCHG_LOCAL
RVM(void, rust_vmstat_warn_cmpxchg_node_bytes, (bool condition), { VM_WARN_ON_ONCE(condition); })
#endif
#ifdef CONFIG_NUMA
RVM(u8, rust_vmstat_raw_read_u8, (const u8 __percpu *p), { return __this_cpu_read(*p); })
RVM(void, rust_vmstat_raw_write_u8, (u8 __percpu *p, u8 value), { __this_cpu_write(*p, value); })
RVM(u8, rust_vmstat_raw_dec_u8, (u8 __percpu *p), { return __this_cpu_dec_return(*p); })
RVM(int, rust_vmstat_raw_read_int, (const int __percpu *p), { return __this_cpu_read(*p); })
RVM(int, rust_vmstat_numa_node_id, (void), { return numa_node_id(); })
RVM(int, rust_vmstat_zone_to_nid, (const struct zone *z), { return zone_to_nid(z); })
#endif
RVM(int, rust_vmstat_next_possible_cpu, (int cpu), { return cpumask_next(cpu, cpu_possible_mask); })
RVM(int, rust_vmstat_smp_processor_id, (void), { return smp_processor_id(); })
RVM(struct delayed_work *, rust_vmstat_work_cpu, (int cpu), { return &per_cpu(vmstat_work, cpu); })
RVM(struct delayed_work *, rust_vmstat_this_work, (void), { return this_cpu_ptr(&vmstat_work); })
RVM(struct delayed_work *, rust_vmstat_shepherd_work, (void), { return &shepherd; })
RVM(bool, rust_vmstat_work_pending, (struct delayed_work *w), { return delayed_work_pending(w); })
RVM(void, rust_vmstat_flush_workqueue, (struct workqueue_struct *w), { flush_workqueue(w); })
RVM(bool, rust_vmstat_schedule_delayed, (struct delayed_work *w, unsigned long delay), { return schedule_delayed_work(w, delay); })
RVM(void, rust_vmstat_rcu_read_lock, (void), { rcu_read_lock(); })
RVM(void, rust_vmstat_rcu_read_unlock, (void), { rcu_read_unlock(); })
RVM(bool, rust_vmstat_cpu_is_isolated, (int cpu), { return cpu_is_isolated(cpu); })
RVM(void __init, rust_vmstat_init_cpu_work, (int cpu), { INIT_DEFERRABLE_WORK(per_cpu_ptr(&vmstat_work, cpu), rust_vmstat_update); })
RVM(bool, rust_vmstat_cpu_online, (int cpu), { return cpu_online(cpu); })
RVM(int, rust_vmstat_next_online_node, (int node), { return node < 0 ? first_online_node : next_online_node(node); })
RVM(bool, rust_vmstat_node_cpumask_empty, (int node), { return cpumask_empty(cpumask_of_node(node)); })
RVM(bool, rust_vmstat_node_has_cpu, (int node), { return node_state(node, N_CPU); })
RVM(void, rust_vmstat_node_set_cpu, (int node), { node_set_state(node, N_CPU); })
RVM(void, rust_vmstat_node_clear_cpu, (int node), { node_clear_state(node, N_CPU); })
RVM(int, rust_vmstat_cpu_to_node, (int cpu), { return cpu_to_node(cpu); })
RVM(int __init, rust_vmstat_cpuhp_setup, (enum cpuhp_state state, const char *name, int (*startup)(unsigned int), int (*teardown)(unsigned int)), { return cpuhp_setup_state_nocalls(state, name, startup, teardown); })
RVM(void __init, rust_vmstat_log_dead_error, (void), { pr_err("vmstat: failed to register 'dead' hotplug state\n"); })
RVM(void __init, rust_vmstat_log_online_error, (void), { pr_err("vmstat: failed to register 'online' hotplug state\n"); })
#ifdef CONFIG_PROC_FS
RVM(void, rust_vmstat_warn_negative, (const char *name, long value), { pr_warn("vmstat_refresh: %s %ld\n", name, value); })
#endif
#endif
#ifdef CONFIG_COMPACTION
RVM(bool, rust_vmstat_warn_order, (bool condition), { return WARN_ON_ONCE(condition); })
#endif
#ifdef CONFIG_PROC_FS
RVM(struct proc_dir_entry * __init, rust_vmstat_proc_create_seq, (const char *name, umode_t mode, struct proc_dir_entry *parent, const struct seq_operations *ops), { return proc_create_seq(name, mode, parent, ops); })
/* Header register_sysctl_init is a no-op without CONFIG_SYSCTL. */
#ifdef CONFIG_SYSCTL
RVM(void __init, rust_vmstat_register_sysctl, (const char *path, const struct ctl_table *table, const char *name, size_t count), { __register_sysctl_init(path, table, name, count); })
#else
RVM(void __init, rust_vmstat_register_sysctl, (const char *path, const struct ctl_table *table, const char *name, size_t count), { register_sysctl_init(path, table); })
#endif
#ifdef CONFIG_PAGE_OWNER
RVM(bool, rust_vmstat_page_owner_inited, (void), { return static_branch_unlikely(&page_owner_inited); })
#endif
#endif
#if defined(CONFIG_DEBUG_FS) && defined(CONFIG_COMPACTION)
RVM(struct dentry * __init, rust_vmstat_debugfs_create_file, (const char *name, umode_t mode, struct dentry *parent, void *data, const struct file_operations *ops), { return debugfs_create_file(name, mode, parent, data, ops); })
#endif
