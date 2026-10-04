/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SCHED_IDLE_BINDINGS_H
#define LUPOS_SCHED_IDLE_BINDINGS_H
#include <linux/cpuidle.h>
#include <linux/suspend.h>
#include <linux/livepatch.h>
#include <linux/cpu.h>
#include <linux/objtool_types.h>
#include "sched.h"
#include "smp.h"
#include "stats.h"
#include "pelt.h"

extern char __cpuidle_text_start[], __cpuidle_text_end[];
/* Verbatim private idle.c type; derive hrtimer and all offsets from headers. */
struct idle_timer { struct hrtimer timer; int done; };
static const unsigned int LUPOS_IDLE_PF_IDLE = PF_IDLE;
static const unsigned int LUPOS_IDLE_PF_KTHREAD = PF_KTHREAD;
static const unsigned int LUPOS_IDLE_PF_NO_SETAFFINITY = PF_NO_SETAFFINITY;
static const int LUPOS_IDLE_SCHED_FIFO = SCHED_FIFO;
static const int LUPOS_IDLE_EBUSY = EBUSY;
static const long LUPOS_IDLE_NSEC_PER_USEC = NSEC_PER_USEC;
static const unsigned int LUPOS_IDLE_PWR_EVENT_EXIT = PWR_EVENT_EXIT;
static const unsigned int LUPOS_IDLE_LOAD_AVG_MAX = LOAD_AVG_MAX;
static const unsigned int LUPOS_IDLE_SCHED_CAPACITY_SHIFT = SCHED_CAPACITY_SHIFT;
static const unsigned int LUPOS_IDLE_ANNOTYPE_INSTR_BEGIN = ANNOTYPE_INSTR_BEGIN;
static const unsigned int LUPOS_IDLE_ANNOTYPE_INSTR_END = ANNOTYPE_INSTR_END;

struct rq *lupos_idle_this_rq(void);
struct task_struct *lupos_idle_current(void);
int lupos_idle_cpu(void);
void lupos_idle_irq_disable(void);
void lupos_idle_irq_enable(void);
void lupos_idle_raw_irq_disable(void);
void lupos_idle_raw_irq_enable(void);
bool lupos_idle_irqs_disabled(void);
void lupos_idle_cpu_relax(void);
bool lupos_idle_tif_need_resched(void);
bool lupos_idle_need_resched(void);
bool lupos_idle_clr_polling_test(void);
void lupos_idle_set_polling(void);
void lupos_idle_clr_polling(void);
bool lupos_idle_cpu_offline(int cpu);
void lupos_idle_preempt_set_need_resched(void);
void lupos_idle_preempt_fold_need_resched(void);
void lupos_idle_preempt_disable(void);
void lupos_idle_preempt_enable(void);
void lupos_idle_set_task_need_resched(struct task_struct *p);
void lupos_idle_mb_after_atomic(void);
void lupos_idle_smp_wmb(void);
void lupos_idle_trace(unsigned int state, unsigned int cpu);
void lupos_idle_stop_critical_timings(void);
void lupos_idle_start_critical_timings(void);
void lupos_idle_ct_enter(void);
void lupos_idle_ct_exit(void);
bool lupos_idle_broadcast_expired(void);
#ifdef CONFIG_GENERIC_CLOCKEVENTS_BROADCAST_IDLE
bool lupos_idle_needs_broadcast(void);
void lupos_idle_tick_broadcast_enter(void);
void lupos_idle_tick_broadcast_exit(void);
#endif
bool lupos_idle_tick_stopped(void);
void lupos_idle_stop_tick(void);
void lupos_idle_retain_tick(void);
void lupos_idle_restart_tick(void);
void lupos_idle_tick_enter(void);
void lupos_idle_tick_exit(void);
bool lupos_idle_got_tick(void);
struct cpuidle_device *lupos_idle_get_device(void);
struct cpuidle_driver *lupos_idle_get_cpu_driver(struct cpuidle_device *dev);
bool lupos_idle_not_available(struct cpuidle_driver *drv, struct cpuidle_device *dev);
int lupos_idle_enter_s2idle(struct cpuidle_driver *drv, struct cpuidle_device *dev, u64 max_latency_ns);
int lupos_idle_enter(struct cpuidle_driver *drv, struct cpuidle_device *dev, int state);
int lupos_idle_find_deepest_state(struct cpuidle_driver *drv, struct cpuidle_device *dev, u64 max_latency_ns);
int lupos_idle_select(struct cpuidle_driver *drv, struct cpuidle_device *dev, bool *stop_tick);
void lupos_idle_reflect(struct cpuidle_device *dev, int state);
void lupos_idle_use_deepest_state(u64 latency_ns);
bool lupos_idle_should_s2idle(void);
s32 lupos_idle_wakeup_latency_qos_limit(void);
void lupos_idle_nohz_balance(int cpu);
void lupos_idle_rcu_flush(void);
void lupos_idle_rcu_sleep_check(void);
void lupos_idle_flush_smp_queue(void);
bool lupos_idle_patch_pending(struct task_struct *p);
void lupos_idle_update_patch(struct task_struct *p);
void lupos_idle_hrtimer_setup(struct hrtimer *timer, enum hrtimer_restart (*fn)(struct hrtimer *));
void lupos_idle_hrtimer_start(struct hrtimer *timer, s64 duration);
int lupos_idle_task_cpu(struct task_struct *p);
bool lupos_idle_scx_enabled(void);
bool lupos_idle_smt_active(void);
#ifdef CONFIG_SCHEDSTATS
bool lupos_idle_schedstat_enabled(void);
#endif
u64 lupos_idle_rq_clock_task(struct rq *rq);
u64 lupos_idle_rq_clock(struct rq *rq);
void lupos_idle_assert_clock(struct rq *rq);
void lupos_idle_store_clock_idle(struct rq *rq, u64 value);
void lupos_idle_store_clock_pelt_idle(struct rq *rq, u64 value);
void lupos_idle_rq_lock_irq(struct rq *rq);
void lupos_idle_rq_unlock_irq(struct rq *rq);
void lupos_idle_print_bad_schedule(void);
void lupos_idle_dump_stack(void);
void lupos_idle_cpuhp_report_dead(void);
void lupos_idle_cpuhp_online(enum cpuhp_state state);
void lupos_idle_bug_switching_to(void) __noreturn;
void lupos_idle_bug_prio_changed(void) __noreturn;
void lupos_idle_warn_poll_negative(bool condition);
bool lupos_idle_warn_irqs_disabled(bool condition);
void lupos_idle_warn_offline_resched(bool condition);
void lupos_idle_warn_policy(bool condition);
void lupos_idle_warn_affinity(bool condition);
void lupos_idle_warn_kthread(bool condition);
void lupos_idle_warn_no_setaffinity(bool condition);
void lupos_idle_warn_duration(bool condition);
void lupos_idle_warn_mm(bool condition);
bool lupos_idle_warn_balance(bool condition);

void lupos_idle_arch_prepare_default(void);
void lupos_idle_arch_enter_default(void);
void lupos_idle_arch_exit_default(void);
void lupos_idle_arch_dead_default(void) __noreturn;
void lupos_idle_arch_idle_default(void);
#ifdef CONFIG_GENERIC_IDLE_POLL_SETUP
int lupos_idle_poll_setup(char *unused);
int lupos_idle_nopoll_setup(char *unused);
#endif
int lupos_idle_select_task_rq(struct task_struct *p, int cpu, int flags);
int lupos_idle_balance(struct rq *rq, struct rq_flags *rf);
void lupos_idle_wakeup_preempt(struct rq *rq, struct task_struct *p, int flags);
void lupos_idle_put_prev_task(struct rq *rq, struct task_struct *prev, struct task_struct *next);
void lupos_idle_set_next_task(struct rq *rq, struct task_struct *next, bool first);
bool lupos_idle_dequeue_task(struct rq *rq, struct task_struct *p, int flags);
void lupos_idle_task_tick(struct rq *rq, struct task_struct *curr, int queued);
void lupos_idle_switching_to(struct rq *rq, struct task_struct *p);
void lupos_idle_prio_changed(struct rq *rq, struct task_struct *p, u64 oldprio);
void lupos_idle_update_curr(struct rq *rq);

#include "sched_idle_layout.h"
#endif
