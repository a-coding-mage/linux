/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Native fields, storage, primitive operations and callback calls only.
 * All participant, queue, retry, completion and park policy stays in Rust.
 */
STOP_LEAF(struct cpu_stopper *, stopper, (unsigned int cpu),
	return per_cpu_ptr(&cpu_stopper, cpu);)
STOP_LEAF(struct task_struct *, thread, (const struct cpu_stopper *s),
	return s->thread;)
STOP_LEAF(bool, enabled, (const struct cpu_stopper *s), return s->enabled;)
STOP_LEAF(void, set_enabled, (struct cpu_stopper *s, bool enabled),
	s->enabled = enabled;)
STOP_LEAF(struct cpu_stop_work *, stop_work, (struct cpu_stopper *s),
	return &s->stop_work;)
STOP_LEAF(unsigned int, task_cpu, (const struct task_struct *task),
	return task_cpu(task);)
STOP_LEAF(void, print_info, (const char *level, const struct cpu_stopper *s),
	printk("%sStopper: %pS <- %pS\n", level, s->fn, (void *)s->caller);)
STOP_LEAF(void, mutex_lock, (void), mutex_lock(&stop_cpus_mutex);)
STOP_LEAF(void, mutex_unlock, (void), mutex_unlock(&stop_cpus_mutex);)
STOP_LEAF(bool, mutex_trylock, (void), return mutex_trylock(&stop_cpus_mutex);)
STOP_LEAF(bool, in_progress, (void), return stop_cpus_in_progress;)
STOP_LEAF(void, set_in_progress, (bool value), stop_cpus_in_progress = value;)
STOP_LEAF(bool, initialized, (void), return stop_machine_initialized;)
STOP_LEAF(void, set_initialized, (void), stop_machine_initialized = true;)
STOP_LEAF(void, done_zero, (struct cpu_stop_done *done),
	memset(done, 0, sizeof(*done));)
STOP_LEAF(void, done_count_set, (struct cpu_stop_done *done, unsigned int count),
	atomic_set(&done->nr_todo, count);)
STOP_LEAF(void, completion_init, (struct cpu_stop_done *done),
	init_completion(&done->completion);)
STOP_LEAF(bool, done_dec_and_test, (struct cpu_stop_done *done),
	return atomic_dec_and_test(&done->nr_todo);)
STOP_LEAF(void, complete, (struct cpu_stop_done *done), complete(&done->completion);)
STOP_LEAF(void, wait_completion, (struct cpu_stop_done *done),
	wait_for_completion(&done->completion);)
STOP_LEAF(bool, completion_done, (struct cpu_stop_done *done),
	return completion_done(&done->completion);)
STOP_LEAF(int, done_ret, (const struct cpu_stop_done *done), return done->ret;)
STOP_LEAF(void, set_done_ret, (struct cpu_stop_done *done, int ret), done->ret = ret;)
STOP_LEAF(void, list_add_tail, (struct cpu_stopper *s, struct cpu_stop_work *work),
	list_add_tail(&work->list, &s->works);)
STOP_LEAF(bool, list_empty, (const struct cpu_stopper *s),
	return list_empty(&s->works);)
STOP_LEAF(struct cpu_stop_work *, list_first, (struct cpu_stopper *s),
	return list_first_entry(&s->works, struct cpu_stop_work, list);)
STOP_LEAF(void, list_del_init, (struct cpu_stop_work *work),
	list_del_init(&work->list);)
STOP_LEAF(void, list_init, (struct cpu_stopper *s), INIT_LIST_HEAD(&s->works);)
STOP_LEAF(void, lock_init, (struct cpu_stopper *stopper),
	raw_spin_lock_init(&stopper->lock);)
STOP_LEAF(unsigned long, lock_irqsave, (struct cpu_stopper *s),
	unsigned long flags; raw_spin_lock_irqsave(&s->lock, flags); return flags;)
STOP_LEAF(void, unlock_irqrestore, (struct cpu_stopper *s, unsigned long flags),
	raw_spin_unlock_irqrestore(&s->lock, flags);)
STOP_LEAF(void, lock_irq, (struct cpu_stopper *s), raw_spin_lock_irq(&s->lock);)
STOP_LEAF(void, lock_nested, (struct cpu_stopper *s),
	raw_spin_lock_nested(&s->lock, SINGLE_DEPTH_NESTING);)
STOP_LEAF(void, unlock, (struct cpu_stopper *s), raw_spin_unlock(&s->lock);)
STOP_LEAF(void, unlock_irq, (struct cpu_stopper *s), raw_spin_unlock_irq(&s->lock);)
STOP_LEAF(void, preempt_disable, (void), preempt_disable();)
STOP_LEAF(void, preempt_enable, (void), preempt_enable();)
STOP_LEAF(void, preempt_count_inc, (void), preempt_count_inc();)
STOP_LEAF(void, preempt_count_dec, (void), preempt_count_dec();)
STOP_LEAF(void, wake, (struct task_struct *task), wake_up_process(task);)
STOP_LEAF(void, cond_resched, (void), cond_resched();)
STOP_LEAF(void, relax, (void), cpu_relax();)
STOP_LEAF(void, barrier, (void), barrier();)
STOP_LEAF(void, wmb, (void), smp_wmb();)
STOP_LEAF(unsigned long, save_flags, (void),
	unsigned long flags; local_save_flags(flags); return flags;)
STOP_LEAF(unsigned long, irq_save, (void),
	unsigned long flags; local_irq_save(flags); return flags;)
STOP_LEAF(void, irq_disable, (void), local_irq_disable();)
STOP_LEAF(void, hard_irq_disable, (void), hard_irq_disable();)
STOP_LEAF(void, irq_restore, (unsigned long flags), local_irq_restore(flags);)
STOP_LEAF(void, yield, (const struct cpumask *mask), stop_machine_yield(mask);)
STOP_LEAF(void, touch_nmi_watchdog, (void), touch_nmi_watchdog();)
STOP_LEAF(void, rcu_momentary_eqs, (void), rcu_momentary_eqs();)
STOP_LEAF(unsigned int, processor_id, (void), return smp_processor_id();)
STOP_LEAF(unsigned int, raw_processor_id, (void), return raw_smp_processor_id();)
STOP_LEAF(const struct cpumask *, online_mask, (void), return cpu_online_mask;)
STOP_LEAF(const struct cpumask *, active_mask, (void), return cpu_active_mask;)
STOP_LEAF(const struct cpumask *, cpumask_of, (unsigned int cpu),
	return cpumask_of(cpu);)
STOP_LEAF(unsigned int, mask_first, (const struct cpumask *mask),
	return cpumask_first(mask);)
/* Match for_each_cpu's find_next_bit and limit directly. cpumask_next would
 * add a cpumask_check diagnostic absent from the native iterator macro.
 */
STOP_LEAF(unsigned int, mask_find_next, (unsigned int start,
	const struct cpumask *mask),
	return find_next_bit(cpumask_bits(mask), small_cpumask_bits, start);)
STOP_LEAF(unsigned int, mask_limit, (void), return small_cpumask_bits;)
/* for_each_possible_cpu is distinct from for_each_cpu at NR_CPUS == 1.
 * Keep the configured primitive, not a cached mask or a dense SMP shortcut.
 */
#if NR_CPUS == 1
STOP_LEAF(unsigned int, possible_find_next, (unsigned int start), return start;)
STOP_LEAF(unsigned int, possible_limit, (void), return 1;)
#else
STOP_LEAF(unsigned int, possible_find_next, (unsigned int start),
	return find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits, start);)
STOP_LEAF(unsigned int, possible_limit, (void), return small_cpumask_bits;)
#endif
STOP_LEAF(bool, mask_test, (unsigned int cpu, const struct cpumask *mask),
	return cpumask_test_cpu(cpu, mask);)
STOP_LEAF(unsigned int, mask_weight, (const struct cpumask *mask),
	return cpumask_weight(mask);)
STOP_LEAF(unsigned int, num_online, (void), return num_online_cpus();)
STOP_LEAF(unsigned int, num_active, (void), return num_active_cpus();)
#ifdef CONFIG_SCHED_SMT
STOP_LEAF(const struct cpumask *, smt_mask, (unsigned int cpu),
	return cpu_smt_mask(cpu);)
#endif
STOP_LEAF(void, cpus_read_lock, (void), cpus_read_lock();)
STOP_LEAF(void, cpus_read_unlock, (void), cpus_read_unlock();)
STOP_LEAF(void, assert_cpus_held, (void), lockdep_assert_cpus_held();)
STOP_LEAF(void, park_thread, (struct task_struct *thread), kthread_park(thread);)
STOP_LEAF(void, unpark_thread, (struct task_struct *thread), kthread_unpark(thread);)
STOP_LEAF(void, set_stop_task, (unsigned int cpu, struct task_struct *task),
	sched_set_stop_task(cpu, task);)
STOP_LEAF(void, work_init, (struct cpu_stop_work *work,
	const struct lupos_stop_callback *fn, void *arg,
	struct cpu_stop_done *done, unsigned long caller),
	*work = (struct cpu_stop_work) {
		.fn = fn->fn, .arg = arg, .done = done, .caller = caller,
	};)
/* Static work nodes are reused; do not reinitialize their list in this path. */
STOP_LEAF(void, work_set, (struct cpu_stop_work *work,
	const struct lupos_stop_callback *fn, void *arg,
	struct cpu_stop_done *done, unsigned long caller),
	work->fn = fn->fn; work->arg = arg; work->done = done; work->caller = caller;)
STOP_LEAF(void, work_callback, (const struct cpu_stop_work *work,
	struct lupos_stop_callback *fn), fn->fn = work->fn;)
STOP_LEAF(void *, work_arg, (const struct cpu_stop_work *work), return work->arg;)
STOP_LEAF(struct cpu_stop_done *, work_done, (const struct cpu_stop_work *work),
	return work->done;)
STOP_LEAF(unsigned long, work_caller, (const struct cpu_stop_work *work),
	return work->caller;)
STOP_LEAF(void, set_caller, (struct cpu_stopper *s, unsigned long caller),
	s->caller = caller;)
STOP_LEAF(void, set_callback, (struct cpu_stopper *s,
	const struct lupos_stop_callback *fn), s->fn = fn->fn;)
STOP_LEAF(void, clear_callback, (struct cpu_stopper *s), s->fn = NULL;)
STOP_LEAF(int, invoke, (const struct lupos_stop_callback *fn, void *arg),
	return fn->fn(arg);)
STOP_LEAF(const struct lupos_stop_callback *, multi_callback, (void),
	return &multi_cpu_stop_callback;)
STOP_LEAF(void, msdata_init, (struct multi_stop_data *data,
	const struct lupos_stop_callback *fn, void *arg, unsigned int count,
	const struct cpumask *mask),
	*data = (struct multi_stop_data) {
		.fn = fn->fn, .data = arg, .num_threads = count, .active_cpus = mask,
	};)
STOP_LEAF(unsigned int, num_threads, (const struct multi_stop_data *data),
	return data->num_threads;)
STOP_LEAF(void, set_num_threads, (struct multi_stop_data *data, unsigned int count),
	data->num_threads = count;)
STOP_LEAF(const struct cpumask *, active_cpus, (const struct multi_stop_data *data),
	return data->active_cpus;)
STOP_LEAF(void, ack_set, (struct multi_stop_data *data, unsigned int count),
	atomic_set(&data->thread_ack, count);)
STOP_LEAF(bool, ack_dec_and_test, (struct multi_stop_data *data),
	return atomic_dec_and_test(&data->thread_ack);)
STOP_LEAF(enum multi_stop_state, state, (const struct multi_stop_data *data),
	return data->state;)
STOP_LEAF(enum multi_stop_state, read_state, (const struct multi_stop_data *data),
	return READ_ONCE(data->state);)
STOP_LEAF(void, write_state, (struct multi_stop_data *data,
	enum multi_stop_state state), WRITE_ONCE(data->state, state);)
STOP_LEAF(int, invoke_msdata, (struct multi_stop_data *data),
	return data->fn(data->data);)
