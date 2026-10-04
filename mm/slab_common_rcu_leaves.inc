/* SPDX-License-Identifier: GPL-2.0 */
/* Header/macro leaves only. Include with RSC_LEAF selecting declarations or definitions. */
RSC_LEAF(struct slab *, rust_slab_common_rcu_virt_to_slab, (const void *ptr),
{ return virt_to_slab(ptr); })
RSC_LEAF(int, rust_slab_common_rcu_slab_nid, (const struct slab *slab),
{ return slab_nid(slab); })
RSC_LEAF(int, rust_slab_common_rcu_numa_mem_id, (void),
{ return numa_mem_id(); })
RSC_LEAF(bool, rust_slab_common_rcu_is_vmalloc_addr, (const void *ptr),
{ return is_vmalloc_addr(ptr); })
RSC_LEAF(void, rust_slab_common_rcu_might_sleep, (void),
{ might_sleep(); })
RSC_LEAF(void, rust_slab_common_rcu_kasan_record_aux_stack, (void *ptr),
{ kasan_record_aux_stack(ptr); })
RSC_LEAF(void, rust_slab_common_rcu_cpus_read_lock, (void),
{ cpus_read_lock(); })
RSC_LEAF(void, rust_slab_common_rcu_cpus_read_unlock, (void),
{ cpus_read_unlock(); })

#ifdef CONFIG_KVFREE_RCU_BATCHED
RSC_LEAF(void, rust_slab_common_rcu_local_irq_save, (unsigned long *flags),
{ local_irq_save(*flags); })
RSC_LEAF(void, rust_slab_common_rcu_raw_spin_lock, (raw_spinlock_t *lock),
{ raw_spin_lock(lock); })
RSC_LEAF(void, rust_slab_common_rcu_raw_spin_lock_irqsave,
	 (raw_spinlock_t *lock, unsigned long *flags),
{ raw_spin_lock_irqsave(lock, *flags); })
RSC_LEAF(void, rust_slab_common_rcu_raw_spin_unlock_irqrestore,
	 (raw_spinlock_t *lock, unsigned long flags),
{ raw_spin_unlock_irqrestore(lock, flags); })
RSC_LEAF(struct kfree_rcu_cpu *, rust_slab_common_rcu_this_cpu, (void),
{ return this_cpu_ptr(&krc); })
RSC_LEAF(struct kfree_rcu_cpu *, rust_slab_common_rcu_per_cpu, (int cpu),
{ return per_cpu_ptr(&krc, cpu); })
/* One native mask operation per leaf; the CPU loops remain in Rust. */
#if NR_CPUS == 1
RSC_LEAF(unsigned int, rust_slab_common_rcu_first_cpu, (void),
{ return 0; })
RSC_LEAF(unsigned int, rust_slab_common_rcu_next_cpu, (int cpu),
{ return cpu + 1; })
RSC_LEAF(unsigned int, rust_slab_common_rcu_cpu_limit, (void),
{ return 1; })
#else
RSC_LEAF(unsigned int, rust_slab_common_rcu_first_cpu, (void),
{ return find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits, 0); })
RSC_LEAF(unsigned int, rust_slab_common_rcu_next_cpu, (int cpu),
{ return find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits, cpu + 1); })
RSC_LEAF(unsigned int, rust_slab_common_rcu_cpu_limit, (void),
{ return small_cpumask_bits; })
#endif
RSC_LEAF(int, rust_slab_common_rcu_atomic_read, (const atomic_t *v),
{ return atomic_read(v); })
RSC_LEAF(void, rust_slab_common_rcu_atomic_set, (atomic_t *v, int value),
{ atomic_set(v, value); })
RSC_LEAF(void, rust_slab_common_rcu_atomic_inc, (atomic_t *v),
{ atomic_inc(v); })
RSC_LEAF(void, rust_slab_common_rcu_atomic_sub, (int value, atomic_t *v),
{ atomic_sub(value, v); })
RSC_LEAF(int, rust_slab_common_rcu_atomic_xchg, (atomic_t *v, int value),
{ return atomic_xchg(v, value); })
RSC_LEAF(int, rust_slab_common_rcu_read_nr_bkv_objs, (const struct kfree_rcu_cpu *krcp),
{ return READ_ONCE(krcp->nr_bkv_objs); })
RSC_LEAF(void, rust_slab_common_rcu_write_nr_bkv_objs, (struct kfree_rcu_cpu *krcp, int value),
{ WRITE_ONCE(krcp->nr_bkv_objs, value); })
RSC_LEAF(struct kvfree_rcu_head *, rust_slab_common_rcu_read_head,
	 (const struct kfree_rcu_cpu *krcp),
{ return READ_ONCE(krcp->head); })
RSC_LEAF(void, rust_slab_common_rcu_write_head,
	 (struct kfree_rcu_cpu *krcp, struct kvfree_rcu_head *head),
{ WRITE_ONCE(krcp->head, head); })
RSC_LEAF(struct llist_node *, rust_slab_common_rcu_llist_del_all, (struct llist_head *head),
{ return llist_del_all(head); })
RSC_LEAF(bool, rust_slab_common_rcu_llist_add,
	 (struct llist_node *node, struct llist_head *head),
{ return llist_add(node, head); })
RSC_LEAF(void, rust_slab_common_rcu_init_list_head, (struct list_head *head),
{ INIT_LIST_HEAD(head); })
RSC_LEAF(bool, rust_slab_common_rcu_list_empty, (const struct list_head *head),
{ return list_empty(head); })
RSC_LEAF(void, rust_slab_common_rcu_list_replace_init,
	 (struct list_head *old, struct list_head *new),
{ list_replace_init(old, new); })
RSC_LEAF(void, rust_slab_common_rcu_list_move,
	 (struct list_head *node, struct list_head *head),
{ list_move(node, head); })
RSC_LEAF(void, rust_slab_common_rcu_list_add,
	 (struct list_head *node, struct list_head *head),
{ list_add(node, head); })
RSC_LEAF(struct kvfree_rcu_bulk_data *, rust_slab_common_rcu_first_bulk,
	 (struct list_head *head),
{ return list_first_entry_or_null(head, struct kvfree_rcu_bulk_data, list); })
RSC_LEAF(void, rust_slab_common_rcu_get_state_full, (struct rcu_gp_seq *snap),
{ get_state_synchronize_rcu_full(snap); })
RSC_LEAF(bool, rust_slab_common_rcu_poll_state_full, (struct rcu_gp_seq *snap),
{ return poll_state_synchronize_rcu_full(snap); })
RSC_LEAF(unsigned long, rust_slab_common_rcu_get_state, (void),
{ return get_state_synchronize_rcu(); })
RSC_LEAF(bool, rust_slab_common_rcu_poll_state, (unsigned long snap),
{ return poll_state_synchronize_rcu(snap); })
RSC_LEAF(void, rust_slab_common_rcu_callback_acquire, (void),
{ rcu_lock_acquire(&rcu_callback_map); })
RSC_LEAF(void, rust_slab_common_rcu_callback_release, (void),
{ rcu_lock_release(&rcu_callback_map); })
RSC_LEAF(int, rust_slab_common_rcu_debug_head_queue, (void *ptr),
{ return debug_rcu_head_queue(ptr); })
RSC_LEAF(void, rust_slab_common_rcu_debug_head_unqueue, (void *ptr),
{ debug_rcu_head_unqueue(ptr); })
RSC_LEAF(void, rust_slab_common_rcu_cond_resched_tasks, (void),
{ cond_resched_tasks_rcu_qs(); })
RSC_LEAF(void *, rust_slab_common_rcu_obj_start, (void *head),
{ return kvmalloc_obj_start_addr(head); })
RSC_LEAF(void, rust_slab_common_rcu_trace_bulk, (struct kvfree_rcu_bulk_data *bnode),
{ trace_rcu_invoke_kfree_bulk_callback("slab", bnode->nr_records, bnode->records); })
RSC_LEAF(void, rust_slab_common_rcu_trace_bulk_record,
	 (struct kvfree_rcu_bulk_data *bnode, int index),
{ trace_rcu_invoke_kvfree_callback("slab", bnode->records[index], 0); })
RSC_LEAF(void, rust_slab_common_rcu_trace_list,
	 (struct kvfree_rcu_head *head, unsigned long offset),
{ trace_rcu_invoke_kvfree_callback("slab", head, offset); })
/* Separate leaves preserve the separate WARN_ON_ONCE call sites. */
RSC_LEAF(bool, rust_slab_common_rcu_warn_bulk_gp, (bool condition),
{ return WARN_ON_ONCE(condition); })
RSC_LEAF(bool, rust_slab_common_rcu_warn_head_gp, (bool condition),
{ return WARN_ON_ONCE(condition); })
RSC_LEAF(void, rust_slab_common_rcu_warn_queue, (bool condition),
{ WARN_ON_ONCE(condition); })
RSC_LEAF(void, rust_slab_common_rcu_warn_double_free, (struct kvfree_rcu_head *head),
{ WARN_ONCE(1, "%s(): Double-freed call. rcu_head %p\n", "kvfree_call_rcu", head); })
RSC_LEAF(void, rust_slab_common_rcu_warn_workqueue, (bool condition),
{ WARN_ON(condition); })
RSC_LEAF(void, rust_slab_common_rcu_info_delay, (int delay),
{ pr_info("Adjusting rcutree.rcu_delay_page_cache_fill_msec to %d ms.\n", delay); })
RSC_LEAF(void, rust_slab_common_rcu_error_shrinker, (void),
{ pr_err("Failed to allocate kfree_rcu() shrinker!\n"); })
/* Separate allocation macro sites retain separate allocation profiling tags. */
RSC_LEAF(unsigned long, rust_slab_common_rcu_get_free_page_fill, (gfp_t flags),
{ return __get_free_page(flags); })
RSC_LEAF(unsigned long, rust_slab_common_rcu_get_free_page_add, (gfp_t flags),
{ return __get_free_page(flags); })
RSC_LEAF(void, rust_slab_common_rcu_free_page, (unsigned long address),
{ free_page(address); })
RSC_LEAF(void, rust_slab_common_rcu_kfree_bulk, (size_t size, void **p),
{ kfree_bulk(size, p); })
RSC_LEAF(bool, rust_slab_common_rcu_delayed_work_pending, (struct delayed_work *work),
{ return delayed_work_pending(work); })
RSC_LEAF(unsigned long, rust_slab_common_rcu_jiffies, (void),
{ return jiffies; })
RSC_LEAF(bool, rust_slab_common_rcu_mod_delayed_work,
	 (struct workqueue_struct *wq, struct delayed_work *work, unsigned long delay),
{ return mod_delayed_work(wq, work, delay); })
RSC_LEAF(bool, rust_slab_common_rcu_queue_delayed_work,
	 (struct workqueue_struct *wq, struct delayed_work *work, unsigned long delay),
{ return queue_delayed_work(wq, work, delay); })
RSC_LEAF(unsigned long, rust_slab_common_rcu_msecs_to_jiffies, (unsigned int msecs),
{ return msecs_to_jiffies(msecs); })
RSC_LEAF(void, rust_slab_common_rcu_hrtimer_start,
	 (struct hrtimer *timer, ktime_t time, enum hrtimer_mode mode),
{ hrtimer_start(timer, time, mode); })
RSC_LEAF(void, rust_slab_common_rcu_kmemleak_ignore, (const void *ptr),
{ kmemleak_ignore(ptr); })
RSC_LEAF(bool, rust_slab_common_rcu_cache_has_sheaves, (struct kmem_cache *s),
{ return cache_has_sheaves(s); })
/* INIT_* macros retain their native lockdep keys and timer registrations. */
RSC_LEAF(void, rust_slab_common_rcu_init_rcu_work, (struct rcu_work *work, work_func_t func),
{ INIT_RCU_WORK(work, func); })
RSC_LEAF(void, rust_slab_common_rcu_init_monitor_work,
	 (struct delayed_work *work, work_func_t func),
{ INIT_DELAYED_WORK(work, func); })
RSC_LEAF(void, rust_slab_common_rcu_init_page_cache_work,
	 (struct delayed_work *work, work_func_t func),
{ INIT_DELAYED_WORK(work, func); })
RSC_LEAF(struct workqueue_struct *, rust_slab_common_rcu_alloc_workqueue,
	 (unsigned int flags),
{ return alloc_workqueue("kvfree_rcu_reclaim", flags, 0); })
#endif /* CONFIG_KVFREE_RCU_BATCHED */
