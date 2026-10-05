/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_REGISTRATION_INIT_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_REGISTRATION_INIT_BINDINGS_H

/* F17 reuses the configured native universe. No copied layouts, callback
 * pointer casts, exported private metadata or separate native object. */
#include <linux/init.h>
#include <linux/notifier.h>
#include <linux/suspend.h>
#include <linux/sysrq.h>
#include "internal.h"

void lupos_scx_ri_update_curr_body(struct rq *rq);
void lupos_scx_ri_sysrq_reset_body(u8 key);
void lupos_scx_ri_sysrq_dump_body(u8 key, struct scx_exit_info *ei);
void lupos_scx_ri_dump_one_body(struct scx_sched *sch, struct scx_exit_info *ei);
int lupos_scx_ri_pm_body(struct notifier_block *nb, unsigned long event, void *ptr);
void __init lupos_scx_ri_init_class_body(void);
void __init lupos_scx_ri_init_cpu_body(s32 cpu);
int __init lupos_scx_ri_init_body(void);

struct scx_sched *lupos_scx_ri_reset_root_rcu(void);
struct scx_sched *lupos_scx_ri_pm_root_rcu(void);
bool lupos_scx_ri_likely_sched(struct scx_sched *sch);
void lupos_scx_ri_info_not_loaded(void);
void lupos_scx_ri_for_each_sched_rcu(struct scx_exit_info *ei);

void __init lupos_scx_ri_idle_init_masks(void);
void __init lupos_scx_ri_for_each_possible_cpu(void);
struct rq *__init lupos_scx_ri_cpu_rq(s32 cpu);
void __init lupos_scx_ri_init_local_dsq(struct rq *rq);
#ifdef CONFIG_EXT_SUB_SCHED
void __init lupos_scx_ri_init_reject_dsq(struct rq *rq);
void __init lupos_scx_ri_rescue_init(struct rq *rq);
void __init lupos_scx_ri_init_sched_hash(void);
#endif
void __init lupos_scx_ri_init_list(struct list_head *head);
void __init lupos_scx_ri_alloc_cpus_to_sync(struct rq *rq, int n);
void __init lupos_scx_ri_init_reenq_lock(struct rq *rq);
void __init lupos_scx_ri_init_deferred_irq(struct rq *rq);
void __init lupos_scx_ri_init_kick_irq(struct rq *rq);
bool __init lupos_scx_ri_cpu_online(s32 cpu);
void __init lupos_scx_ri_mark_online(s32 cpu);
void __init lupos_scx_ri_register_reset_key(void);
void __init lupos_scx_ri_register_dump_key(void);
void __init lupos_scx_ri_init_watchdog(void);

int __init lupos_scx_ri_register_enqueue_dispatch(void);
int __init lupos_scx_ri_register_dispatch(void);
int __init lupos_scx_ri_register_cpu_release(void);
int __init lupos_scx_ri_register_unlocked_struct_ops(void);
int __init lupos_scx_ri_register_unlocked_syscall(void);
int __init lupos_scx_ri_register_any_struct_ops(void);
int __init lupos_scx_ri_register_any_tracing(void);
int __init lupos_scx_ri_register_any_syscall(void);
int __init lupos_scx_ri_idle_init(void);
int __init lupos_scx_ri_cid_kfunc_init(void);
int __init lupos_scx_ri_register_cpu_ops(void);
int __init lupos_scx_ri_register_cid_ops(void);
int __init lupos_scx_ri_register_pm(void);
struct kset *__init lupos_scx_ri_create_kset(void);
int __init lupos_scx_ri_create_global_group(void);
void __init lupos_scx_ri_error_kfunc_sets(int ret);
void __init lupos_scx_ri_error_idle_init(int ret);
void __init lupos_scx_ri_error_cid_kfuncs(int ret);
void __init lupos_scx_ri_error_cpu_ops(int ret);
void __init lupos_scx_ri_error_cid_ops(int ret);
void __init lupos_scx_ri_error_pm(int ret);
void __init lupos_scx_ri_error_kset(void);
void __init lupos_scx_ri_error_global_group(void);

#endif /* LUPOS_SCHED_EXT_CORE_REGISTRATION_INIT_BINDINGS_H */
