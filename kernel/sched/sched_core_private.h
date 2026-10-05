/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SCHED_CORE_PRIVATE_H
#define LUPOS_SCHED_CORE_PRIVATE_H
/* Private C types/constants copied from frozen core.c, shared by bindgen and
 * the one native owner. Storage appears only in sched_core_state.inc. */
#ifdef CONFIG_NO_HZ_FULL
struct tick_work {
	int cpu;
	atomic_t state;
	struct delayed_work work;
};
#define TICK_SCHED_REMOTE_OFFLINE 0
#define TICK_SCHED_REMOTE_OFFLINING 1
#define TICK_SCHED_REMOTE_RUNNING 2
#endif
#ifdef CONFIG_UCLAMP_TASK_GROUP
#define _POW10(exp) ((unsigned int)1e##exp)
#define POW10(exp) _POW10(exp)
#define UCLAMP_PERCENT_SHIFT 2
#define UCLAMP_PERCENT_SCALE (100 * POW10(UCLAMP_PERCENT_SHIFT))
struct uclamp_request { s64 percent; u64 util; int ret; };
#endif
#ifdef CONFIG_CFS_BANDWIDTH
struct cfs_schedulable_data { struct task_group *tg; u64 period, quota; };
#endif
#define SM_IDLE (-1)
#define SM_NONE 0
#define SM_PREEMPT 1
#define SM_RTLOCK_WAIT 2
#ifdef CONFIG_PREEMPT_DYNAMIC
enum {
	preempt_dynamic_undefined = -1,
	preempt_dynamic_none,
	preempt_dynamic_voluntary,
	preempt_dynamic_full,
	preempt_dynamic_lazy,
};
extern int preempt_dynamic_mode;
#endif
extern bool sched_smp_initialized;
#ifdef CONFIG_GROUP_SCHED_BANDWIDTH
extern const u64 max_bw_quota_period_us;
#endif
/* Caller values must originate at the Rust call site with the exact native
 * compiler/frame contract. These are intentionally declarations only. */
unsigned long lupos_core_get_lock_parent_ip(void);
void lupos_core_trace_preempt_off_caller(unsigned long parent_ip);
void lupos_core_trace_preempt_on_caller(unsigned long parent_ip);
void lupos_core_profile_sched_caller(void);
#endif
