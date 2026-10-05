/* SPDX-License-Identifier: GPL-2.0-only */
/* Native prototypes of Rust callbacks; ABI signatures copied from core.c. */
int __init setup_resched_latency_warn_ms(char *str);
#ifdef CONFIG_PREEMPT_DYNAMIC
int __init setup_preempt_mode(char *str);
#endif
int __init migration_init(void);
#ifdef CONFIG_CGROUP_SCHED
struct cgroup_subsys_state *
cpu_cgroup_css_alloc(struct cgroup_subsys_state *parent_css);
#endif
#ifdef CONFIG_CGROUP_SCHED
int cpu_cgroup_css_online(struct cgroup_subsys_state *css);
#endif
#ifdef CONFIG_CGROUP_SCHED
void cpu_cgroup_css_offline(struct cgroup_subsys_state *css);
#endif
#ifdef CONFIG_CGROUP_SCHED
void cpu_cgroup_css_released(struct cgroup_subsys_state *css);
#endif
#ifdef CONFIG_CGROUP_SCHED
void cpu_cgroup_css_free(struct cgroup_subsys_state *css);
#endif
#ifdef CONFIG_CGROUP_SCHED
int cpu_cgroup_can_attach(struct cgroup_taskset *tset);
#endif
#ifdef CONFIG_CGROUP_SCHED
void cpu_cgroup_attach(struct cgroup_taskset *tset);
#endif
#ifdef CONFIG_CGROUP_SCHED
void cpu_cgroup_cancel_attach(struct cgroup_taskset *tset);
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_UCLAMP_TASK_GROUP
ssize_t cpu_uclamp_min_write(struct kernfs_open_file *of,
				    char *buf, size_t nbytes,
				    loff_t off);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_UCLAMP_TASK_GROUP
ssize_t cpu_uclamp_max_write(struct kernfs_open_file *of,
				    char *buf, size_t nbytes,
				    loff_t off);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_UCLAMP_TASK_GROUP
int cpu_uclamp_min_show(struct seq_file *sf, void *v);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_UCLAMP_TASK_GROUP
int cpu_uclamp_max_show(struct seq_file *sf, void *v);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_GROUP_SCHED_WEIGHT
int cpu_shares_write_u64(struct cgroup_subsys_state *css,
				struct cftype *cftype, u64 shareval);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_GROUP_SCHED_WEIGHT
u64 cpu_shares_read_u64(struct cgroup_subsys_state *css,
			       struct cftype *cft);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_CFS_BANDWIDTH
int cpu_cfs_stat_show(struct seq_file *sf, void *v);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_CFS_BANDWIDTH
int cpu_cfs_local_stat_show(struct seq_file *sf, void *v);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_GROUP_SCHED_BANDWIDTH
u64 cpu_period_read_u64(struct cgroup_subsys_state *css,
			       struct cftype *cft);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_GROUP_SCHED_BANDWIDTH
s64 cpu_quota_read_s64(struct cgroup_subsys_state *css,
			      struct cftype *cft);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_GROUP_SCHED_BANDWIDTH
u64 cpu_burst_read_u64(struct cgroup_subsys_state *css,
			      struct cftype *cft);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_GROUP_SCHED_BANDWIDTH
int cpu_period_write_u64(struct cgroup_subsys_state *css,
				struct cftype *cftype, u64 period_us);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_GROUP_SCHED_BANDWIDTH
int cpu_quota_write_s64(struct cgroup_subsys_state *css,
			       struct cftype *cftype, s64 quota_us);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_GROUP_SCHED_BANDWIDTH
int cpu_burst_write_u64(struct cgroup_subsys_state *css,
			       struct cftype *cftype, u64 burst_us);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_RT_GROUP_SCHED
int cpu_rt_runtime_write(struct cgroup_subsys_state *css,
				struct cftype *cft, s64 val);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_RT_GROUP_SCHED
s64 cpu_rt_runtime_read(struct cgroup_subsys_state *css,
			       struct cftype *cft);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_RT_GROUP_SCHED
int cpu_rt_period_write_uint(struct cgroup_subsys_state *css,
				    struct cftype *cftype, u64 rt_period_us);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_RT_GROUP_SCHED
u64 cpu_rt_period_read_uint(struct cgroup_subsys_state *css,
				   struct cftype *cft);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_GROUP_SCHED_WEIGHT
s64 cpu_idle_read_s64(struct cgroup_subsys_state *css,
			       struct cftype *cft);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_GROUP_SCHED_WEIGHT
int cpu_idle_write_s64(struct cgroup_subsys_state *css,
				struct cftype *cft, s64 idle);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_RT_GROUP_SCHED
int __init setup_rt_group_sched(char *str);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_RT_GROUP_SCHED
int __init cpu_rt_group_init(void);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
int cpu_extra_stat_show(struct seq_file *sf,
			       struct cgroup_subsys_state *css);
#endif
#ifdef CONFIG_CGROUP_SCHED
int cpu_local_stat_show(struct seq_file *sf,
			       struct cgroup_subsys_state *css);
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_GROUP_SCHED_WEIGHT
u64 cpu_weight_read_u64(struct cgroup_subsys_state *css,
			       struct cftype *cft);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_GROUP_SCHED_WEIGHT
int cpu_weight_write_u64(struct cgroup_subsys_state *css,
				struct cftype *cft, u64 cgrp_weight);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_GROUP_SCHED_WEIGHT
s64 cpu_weight_nice_read_s64(struct cgroup_subsys_state *css,
				    struct cftype *cft);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_GROUP_SCHED_WEIGHT
int cpu_weight_nice_write_s64(struct cgroup_subsys_state *css,
				     struct cftype *cft, s64 nice);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_GROUP_SCHED_BANDWIDTH
int cpu_max_show(struct seq_file *sf, void *v);
#endif
#endif
#ifdef CONFIG_CGROUP_SCHED
#ifdef CONFIG_GROUP_SCHED_BANDWIDTH
ssize_t cpu_max_write(struct kernfs_open_file *of,
			     char *buf, size_t nbytes, loff_t off);
#endif
#endif
