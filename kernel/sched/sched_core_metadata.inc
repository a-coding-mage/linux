/* SPDX-License-Identifier: GPL-2.0-only */
/* Export/init/probe metadata for Rust-owned tail functions. */
EXPORT_SYMBOL(single_task_running);
__setup("resched_latency_warn_ms=", setup_resched_latency_warn_ms);
#if defined(CONFIG_PREEMPTION) && (defined(CONFIG_DEBUG_PREEMPT) ||  				defined(CONFIG_TRACE_PREEMPT_TOGGLE))
EXPORT_SYMBOL(preempt_count_add);
#endif
#if defined(CONFIG_PREEMPTION) && (defined(CONFIG_DEBUG_PREEMPT) ||  				defined(CONFIG_TRACE_PREEMPT_TOGGLE))
NOKPROBE_SYMBOL(preempt_count_add);
#endif
#if defined(CONFIG_PREEMPTION) && (defined(CONFIG_DEBUG_PREEMPT) ||  				defined(CONFIG_TRACE_PREEMPT_TOGGLE))
EXPORT_SYMBOL(preempt_count_sub);
#endif
#if defined(CONFIG_PREEMPTION) && (defined(CONFIG_DEBUG_PREEMPT) ||  				defined(CONFIG_TRACE_PREEMPT_TOGGLE))
NOKPROBE_SYMBOL(preempt_count_sub);
#endif
EXPORT_SYMBOL(schedule);
#ifdef CONFIG_PREEMPT_RT
NOKPROBE_SYMBOL(schedule_rtlock);
#endif
#ifdef CONFIG_PREEMPTION
NOKPROBE_SYMBOL(preempt_schedule);
#endif
#ifdef CONFIG_PREEMPTION
EXPORT_SYMBOL(preempt_schedule);
#endif
#ifdef CONFIG_PREEMPTION
#ifdef CONFIG_PREEMPT_DYNAMIC
#ifdef CONFIG_HAVE_PREEMPT_DYNAMIC_CALL
#elif defined(CONFIG_HAVE_PREEMPT_DYNAMIC_KEY)
NOKPROBE_SYMBOL(dynamic_preempt_schedule);
#endif
#endif
#endif
#ifdef CONFIG_PREEMPTION
#ifdef CONFIG_PREEMPT_DYNAMIC
#ifdef CONFIG_HAVE_PREEMPT_DYNAMIC_CALL
#elif defined(CONFIG_HAVE_PREEMPT_DYNAMIC_KEY)
EXPORT_SYMBOL(dynamic_preempt_schedule);
#endif
#endif
#endif
#ifdef CONFIG_PREEMPTION
EXPORT_SYMBOL_GPL(preempt_schedule_notrace);
#endif
#ifdef CONFIG_PREEMPTION
#ifdef CONFIG_PREEMPT_DYNAMIC
#if defined(CONFIG_HAVE_PREEMPT_DYNAMIC_CALL)
#elif defined(CONFIG_HAVE_PREEMPT_DYNAMIC_KEY)
NOKPROBE_SYMBOL(dynamic_preempt_schedule_notrace);
#endif
#endif
#endif
#ifdef CONFIG_PREEMPTION
#ifdef CONFIG_PREEMPT_DYNAMIC
#if defined(CONFIG_HAVE_PREEMPT_DYNAMIC_CALL)
#elif defined(CONFIG_HAVE_PREEMPT_DYNAMIC_KEY)
EXPORT_SYMBOL(dynamic_preempt_schedule_notrace);
#endif
#endif
#endif
EXPORT_SYMBOL(default_wake_function);
#if !defined(CONFIG_PREEMPTION) || defined(CONFIG_PREEMPT_DYNAMIC)
EXPORT_SYMBOL(__cond_resched);
#endif
#ifdef CONFIG_PREEMPT_DYNAMIC
#ifdef CONFIG_HAVE_PREEMPT_DYNAMIC_CALL
#elif defined(CONFIG_HAVE_PREEMPT_DYNAMIC_KEY)
EXPORT_SYMBOL(dynamic_cond_resched);
#endif
#endif
#ifdef CONFIG_PREEMPT_DYNAMIC
#ifdef CONFIG_HAVE_PREEMPT_DYNAMIC_CALL
#elif defined(CONFIG_HAVE_PREEMPT_DYNAMIC_KEY)
EXPORT_SYMBOL(dynamic_might_resched);
#endif
#endif
EXPORT_SYMBOL(__cond_resched_lock);
EXPORT_SYMBOL(__cond_resched_rwlock_read);
EXPORT_SYMBOL(__cond_resched_rwlock_write);
#ifdef CONFIG_PREEMPT_DYNAMIC
__setup("preempt=", setup_preempt_mode);
#endif
EXPORT_SYMBOL(io_schedule_timeout);
EXPORT_SYMBOL(io_schedule);
EXPORT_SYMBOL_GPL(sched_show_task);
early_initcall(migration_init);
#ifdef CONFIG_DEBUG_ATOMIC_SLEEP
EXPORT_SYMBOL(__might_sleep);
#endif
#ifdef CONFIG_DEBUG_ATOMIC_SLEEP
EXPORT_SYMBOL(__might_resched);
#endif
#ifdef CONFIG_DEBUG_ATOMIC_SLEEP
EXPORT_SYMBOL_GPL(__cant_sleep);
#endif
#ifdef CONFIG_DEBUG_ATOMIC_SLEEP
#ifdef CONFIG_SMP
EXPORT_SYMBOL_GPL(__cant_migrate);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_RT_GROUP_SCHED
__setup("rt_group_sched=", setup_rt_group_sched);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_RT_GROUP_SCHED
subsys_initcall(cpu_rt_group_init);
#endif
#endif
#ifdef CONFIG_PREEMPT_DYNAMIC
EXPORT_SYMBOL_GPL(preempt_model_none);
EXPORT_SYMBOL_GPL(preempt_model_voluntary);
EXPORT_SYMBOL_GPL(preempt_model_full);
EXPORT_SYMBOL_GPL(preempt_model_lazy);
#endif
