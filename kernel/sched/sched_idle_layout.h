/* SPDX-License-Identifier: GPL-2.0-only */
/* Native layout facts for every aggregate and member directly used by idle.rs.
 * Type size/alignment checks catch omitted anonymous-union arms before use. */
#define LUPOS_IDLE_LAYOUT_TYPE(type) \
 static const size_t LUPOS_IDLE_SIZE_##type = sizeof(struct type); \
 static const size_t LUPOS_IDLE_ALIGN_##type = __alignof__(struct type)
#define LUPOS_IDLE_LAYOUT_FIELD(type, field) \
 static const size_t LUPOS_IDLE_OFF_##type##_##field = offsetof(struct type, field); \
 static const size_t LUPOS_IDLE_WIDTH_##type##_##field = sizeof(((struct type *)0)->field)
LUPOS_IDLE_LAYOUT_TYPE(rq);
LUPOS_IDLE_LAYOUT_TYPE(task_struct);
LUPOS_IDLE_LAYOUT_TYPE(sched_entity);
LUPOS_IDLE_LAYOUT_TYPE(sched_avg);
LUPOS_IDLE_LAYOUT_TYPE(cfs_rq);
LUPOS_IDLE_LAYOUT_TYPE(cpuidle_device);
LUPOS_IDLE_LAYOUT_TYPE(cpuidle_driver);
LUPOS_IDLE_LAYOUT_TYPE(idle_timer);
LUPOS_IDLE_LAYOUT_TYPE(hrtimer);
LUPOS_IDLE_LAYOUT_FIELD(rq, idle);
LUPOS_IDLE_LAYOUT_FIELD(rq, curr);
LUPOS_IDLE_LAYOUT_FIELD(rq, cfs);
LUPOS_IDLE_LAYOUT_FIELD(rq, avg_rt);
LUPOS_IDLE_LAYOUT_FIELD(rq, avg_dl);
LUPOS_IDLE_LAYOUT_FIELD(rq, lost_idle_time);
LUPOS_IDLE_LAYOUT_FIELD(rq, clock_pelt);
LUPOS_IDLE_LAYOUT_FIELD(rq, fair_server);
#ifdef CONFIG_CPU_IDLE
LUPOS_IDLE_LAYOUT_FIELD(rq, idle_state);
#endif
#ifdef CONFIG_SCHEDSTATS
LUPOS_IDLE_LAYOUT_FIELD(rq, sched_goidle);
#endif
#ifdef CONFIG_SCHED_CLASS_EXT
LUPOS_IDLE_LAYOUT_FIELD(rq, ext_server);
#endif
LUPOS_IDLE_LAYOUT_FIELD(task_struct, flags);
LUPOS_IDLE_LAYOUT_FIELD(task_struct, policy);
LUPOS_IDLE_LAYOUT_FIELD(task_struct, nr_cpus_allowed);
LUPOS_IDLE_LAYOUT_FIELD(task_struct, mm);
LUPOS_IDLE_LAYOUT_FIELD(task_struct, se);
LUPOS_IDLE_LAYOUT_FIELD(task_struct, prio);
LUPOS_IDLE_LAYOUT_FIELD(sched_entity, exec_start);
LUPOS_IDLE_LAYOUT_FIELD(cfs_rq, avg);
LUPOS_IDLE_LAYOUT_FIELD(sched_avg, util_sum);
LUPOS_IDLE_LAYOUT_FIELD(cpuidle_device, last_residency_ns);
LUPOS_IDLE_LAYOUT_FIELD(cpuidle_device, forced_idle_latency_limit_ns);
LUPOS_IDLE_LAYOUT_FIELD(cpuidle_driver, state_count);
LUPOS_IDLE_LAYOUT_FIELD(idle_timer, timer);
LUPOS_IDLE_LAYOUT_FIELD(idle_timer, done);
#undef LUPOS_IDLE_LAYOUT_TYPE
#undef LUPOS_IDLE_LAYOUT_FIELD
