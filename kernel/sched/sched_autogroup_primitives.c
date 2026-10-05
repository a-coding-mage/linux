// SPDX-License-Identifier: GPL-2.0
/*
 * Native storage, registration, and primitive leaves for autogroup.rs.
 * Source-only continuation; no autogroup.c algorithm is included or called.
 * Native compilation/instrumentation policy origin: build_utility.c.
 */
#include "sched_autogroup_bindings.h"

#error "SOURCE ONLY HOLD: scheduler autogroup is not admitted"

#ifndef CONFIG_SCHED_AUTOGROUP
#error "autogroup native primitives require CONFIG_SCHED_AUTOGROUP"
#endif

unsigned int __read_mostly sysctl_sched_autogroup_enabled = 1;
static struct autogroup autogroup_default;
static atomic_t autogroup_seq_nr;

#ifdef CONFIG_SYSCTL
static const struct ctl_table sched_autogroup_sysctls[] = {
	{
		.procname = "sched_autogroup_enabled",
		.data = &sysctl_sched_autogroup_enabled,
		.maxlen = sizeof(unsigned int),
		.mode = 0644,
		.proc_handler = proc_dointvec_minmax,
		.extra1 = SYSCTL_ZERO,
		.extra2 = SYSCTL_ONE,
	},
};

void __init lupos_autogroup_sysctl_init(void)
{
	register_sysctl_init("kernel", sched_autogroup_sysctls);
}
#endif /* CONFIG_SYSCTL */

/*
 * Preserve native __init entry attributes; all initialization decisions and
 * the boot argument action stay in Rust. These are ABI/registration adapters.
 */
void __init autogroup_init(struct task_struct *init_task)
{
	lupos_autogroup_init(init_task);
}

static int __init setup_autogroup(char *str)
{
	return lupos_autogroup_setup(str);
}
__setup("noautogroup", setup_autogroup);

/* Keep the original export surface. */
EXPORT_SYMBOL(sched_autogroup_create_attach);
EXPORT_SYMBOL(sched_autogroup_detach);

struct autogroup *lupos_autogroup_default(void)
{
	return &autogroup_default;
}

struct task_group *lupos_autogroup_root(void)
{
	return &root_task_group;
}

struct autogroup *lupos_autogroup_alloc(void)
{
	struct autogroup *ag = kzalloc_obj(*ag);

	return ag;
}

void lupos_autogroup_free(struct autogroup *ag)
{
	kfree(ag);
}

bool lupos_autogroup_is_err(struct task_group *tg)
{
	return IS_ERR(tg);
}

struct task_group *lupos_autogroup_tg(struct autogroup *ag)
{
	return ag->tg;
}

void lupos_autogroup_set_tg(struct autogroup *ag, struct task_group *tg)
{
	ag->tg = tg;
}

struct autogroup *lupos_autogroup_tg_autogroup(struct task_group *tg)
{
	return tg->autogroup;
}

void lupos_autogroup_tg_set_autogroup(struct task_group *tg,
				   struct autogroup *ag)
{
	tg->autogroup = ag;
}

bool lupos_autogroup_is_autogroup(struct task_group *tg)
{
	return task_group_is_autogroup(tg);
}

void lupos_autogroup_ref_init(struct autogroup *ag)
{
	kref_init(&ag->kref);
}

void lupos_autogroup_ref_get(struct autogroup *ag)
{
	kref_get(&ag->kref);
}

static void lupos_autogroup_release(struct kref *ref)
{
	/* Native callback type/container calculation; Rust owns destruction. */
	lupos_autogroup_destroy(container_of(ref, struct autogroup, kref));
}

void lupos_autogroup_ref_put(struct autogroup *ag)
{
	kref_put(&ag->kref, lupos_autogroup_release);
}

void __init lupos_autogroup_default_lock_init(void)
{
	/* Separate native macro sites preserve default/create lockdep classes. */
	init_rwsem(&autogroup_default.lock);
}

void lupos_autogroup_lock_init(struct autogroup *ag)
{
	init_rwsem(&ag->lock);
}

int lupos_autogroup_next_id(void)
{
	return atomic_inc_return(&autogroup_seq_nr);
}

void lupos_autogroup_set_id(struct autogroup *ag, unsigned long id)
{
	ag->id = id;
}

struct signal_struct *lupos_autogroup_task_signal(struct task_struct *p)
{
	return p->signal;
}

struct autogroup *lupos_autogroup_signal_get(struct signal_struct *sig)
{
	return sig->autogroup;
}

void lupos_autogroup_signal_set(struct signal_struct *sig, struct autogroup *ag)
{
	sig->autogroup = ag;
}

struct task_struct *lupos_autogroup_current(void)
{
	return current;
}

bool lupos_autogroup_task_exiting(struct task_struct *p)
{
	return p->flags & PF_EXITING;
}

bool lupos_autogroup_lock_sighand(struct task_struct *p, unsigned long *flags)
{
	return lock_task_sighand(p, flags) != NULL;
}

void lupos_autogroup_unlock_sighand(struct task_struct *p, unsigned long *flags)
{
	unlock_task_sighand(p, flags);
}

bool lupos_autogroup_warn_move_unlocked(bool unlocked)
{
	/* One diagnostic site, matching autogroup_move_group's WARN_ON_ONCE. */
	return WARN_ON_ONCE(unlocked);
}

struct task_struct *lupos_autogroup_thread_first(struct task_struct *p)
{
	struct task_struct *t;

	/* Preserve the original macro's initial RCU/lockdep check and read. */
	for_each_thread(p, t)
		return t;
	return NULL;
}

struct task_struct *lupos_autogroup_thread_next(struct task_struct *p,
					     struct task_struct *t)
{
	/*
	 * Exact increment/read and sentinel check from list_for_each_entry_rcu.
	 * The siglock held by Rust keeps the signal and membership live.
	 */
	t = list_entry_rcu(t->thread_node.next, struct task_struct, thread_node);
	return &t->thread_node != &p->signal->thread_head ? t : NULL;
}

bool lupos_autogroup_printk_ratelimit(void)
{
	/*
	 * printk_ratelimit() expands to __printk_ratelimit(__func__). Keep the
	 * original site name rather than changing the suppressed-message label.
	 */
#ifdef CONFIG_PRINTK
	return __printk_ratelimit("autogroup_create");
#else
	return printk_ratelimit();
#endif
}

void lupos_autogroup_warn_create(bool group_failed)
{
	printk(KERN_WARNING "autogroup_create: %s failure.\n",
	       group_failed ? "sched_create_group()" : "kzalloc()");
}

void __init lupos_autogroup_disable(void)
{
	sysctl_sched_autogroup_enabled = 0;
}

int lupos_autogroup_format_path(struct autogroup *ag, char *buf, int buflen)
{
	return snprintf(buf, buflen, "%s-%ld", "/autogroup", ag->id);
}

#ifdef CONFIG_RT_GROUP_SCHED
void lupos_autogroup_clear_rt(struct task_group *tg)
{
	tg->rt_se = NULL;
	tg->rt_rq = NULL;
}

void lupos_autogroup_redirect_rt(struct task_group *tg)
{
	tg->rt_se = root_task_group.rt_se;
	tg->rt_rq = root_task_group.rt_rq;
}
#endif /* CONFIG_RT_GROUP_SCHED */

#ifdef CONFIG_PROC_FS
/* Native non-atomic shared deadline and initialization match the C oracle. */
static unsigned long autogroup_next = INITIAL_JIFFIES;

int lupos_autogroup_security_setnice(struct task_struct *p, int nice)
{
	return security_task_setnice(p, nice);
}

bool lupos_autogroup_capable_admin(void)
{
	return capable(CAP_SYS_ADMIN);
}

bool lupos_autogroup_time_before(unsigned long a, unsigned long b)
{
	return time_before(a, b);
}

unsigned long lupos_autogroup_jiffies(void)
{
	return jiffies;
}

unsigned long lupos_autogroup_next_read(void)
{
	return autogroup_next;
}

void lupos_autogroup_next_write(unsigned long next)
{
	autogroup_next = next;
}

int lupos_autogroup_index_nospec(int index)
{
	return array_index_nospec(index, 40);
}

unsigned long lupos_autogroup_scaled_weight(int index)
{
	return scale_load(sched_prio_to_weight[index]);
}

void lupos_autogroup_down_write(struct autogroup *ag)
{
	down_write(&ag->lock);
}

void lupos_autogroup_up_write(struct autogroup *ag)
{
	up_write(&ag->lock);
}

void lupos_autogroup_down_read(struct autogroup *ag)
{
	down_read(&ag->lock);
}

void lupos_autogroup_up_read(struct autogroup *ag)
{
	up_read(&ag->lock);
}

void lupos_autogroup_set_nice(struct autogroup *ag, int nice)
{
	ag->nice = nice;
}

void lupos_autogroup_show(struct seq_file *m, struct autogroup *ag)
{
	seq_printf(m, "/autogroup-%ld nice %d\n", ag->id, ag->nice);
}
#endif /* CONFIG_PROC_FS */
