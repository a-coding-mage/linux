/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_AUTOGROUP_BINDINGS_H
#define LUPOS_SCHED_AUTOGROUP_BINDINGS_H

/* Native configured headers are the only type/layout/constant authority. */
#include "sched.h"
#include "autogroup.h"
#include <linux/nospec.h>
#include <linux/rwsem.h>
#include <linux/security.h>
#include <linux/slab.h>

struct autogroup *lupos_autogroup_default(void);
struct task_group *lupos_autogroup_root(void);
struct autogroup *lupos_autogroup_alloc(void);
void lupos_autogroup_free(struct autogroup *ag);
bool lupos_autogroup_is_err(struct task_group *tg);
struct task_group *lupos_autogroup_tg(struct autogroup *ag);
void lupos_autogroup_set_tg(struct autogroup *ag, struct task_group *tg);
struct autogroup *lupos_autogroup_tg_autogroup(struct task_group *tg);
void lupos_autogroup_tg_set_autogroup(struct task_group *tg,
				   struct autogroup *ag);
bool lupos_autogroup_is_autogroup(struct task_group *tg);
void lupos_autogroup_ref_init(struct autogroup *ag);
void lupos_autogroup_ref_get(struct autogroup *ag);
void lupos_autogroup_ref_put(struct autogroup *ag);
void __init lupos_autogroup_default_lock_init(void);
void lupos_autogroup_lock_init(struct autogroup *ag);
int lupos_autogroup_next_id(void);
void lupos_autogroup_set_id(struct autogroup *ag, unsigned long id);
struct signal_struct *lupos_autogroup_task_signal(struct task_struct *p);
struct autogroup *lupos_autogroup_signal_get(struct signal_struct *sig);
void lupos_autogroup_signal_set(struct signal_struct *sig, struct autogroup *ag);
struct task_struct *lupos_autogroup_current(void);
bool lupos_autogroup_task_exiting(struct task_struct *p);
bool lupos_autogroup_lock_sighand(struct task_struct *p, unsigned long *flags);
void lupos_autogroup_unlock_sighand(struct task_struct *p, unsigned long *flags);
bool lupos_autogroup_warn_move_unlocked(bool unlocked);
struct task_struct *lupos_autogroup_thread_first(struct task_struct *p);
struct task_struct *lupos_autogroup_thread_next(struct task_struct *p,
					     struct task_struct *t);
bool lupos_autogroup_printk_ratelimit(void);
void lupos_autogroup_warn_create(bool group_failed);
void __init lupos_autogroup_disable(void);
int lupos_autogroup_format_path(struct autogroup *ag, char *buf, int buflen);

/* Rust definitions called by native release/registration primitives. */
void __init lupos_autogroup_init(struct task_struct *init_task);
void lupos_autogroup_destroy(struct autogroup *ag);
int __init lupos_autogroup_setup(char *str);

#ifdef CONFIG_SYSCTL
void __init lupos_autogroup_sysctl_init(void);
#endif

#ifdef CONFIG_RT_GROUP_SCHED
void lupos_autogroup_clear_rt(struct task_group *tg);
void lupos_autogroup_redirect_rt(struct task_group *tg);
#endif

#ifdef CONFIG_PROC_FS
#define LUPOS_AUTOGROUP_MIN_NICE MIN_NICE
#define LUPOS_AUTOGROUP_MAX_NICE MAX_NICE
#define LUPOS_AUTOGROUP_EINVAL EINVAL
#define LUPOS_AUTOGROUP_EPERM EPERM
#define LUPOS_AUTOGROUP_EAGAIN EAGAIN
#define LUPOS_AUTOGROUP_HZ HZ
int lupos_autogroup_security_setnice(struct task_struct *p, int nice);
bool lupos_autogroup_capable_admin(void);
bool lupos_autogroup_time_before(unsigned long a, unsigned long b);
unsigned long lupos_autogroup_jiffies(void);
unsigned long lupos_autogroup_next_read(void);
void lupos_autogroup_next_write(unsigned long next);
int lupos_autogroup_index_nospec(int index);
unsigned long lupos_autogroup_scaled_weight(int index);
void lupos_autogroup_down_write(struct autogroup *ag);
void lupos_autogroup_up_write(struct autogroup *ag);
void lupos_autogroup_down_read(struct autogroup *ag);
void lupos_autogroup_up_read(struct autogroup *ag);
void lupos_autogroup_set_nice(struct autogroup *ag, int nice);
void lupos_autogroup_show(struct seq_file *m, struct autogroup *ag);
#endif /* CONFIG_PROC_FS */

#endif /* LUPOS_SCHED_AUTOGROUP_BINDINGS_H */
