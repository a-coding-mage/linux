// SPDX-License-Identifier: GPL-2.0
/*
 * Native leaves for the Rust DO_ONCE() owner: the static once_lock and
 * once_mutex with their lockdep classes, the work-item allocation and
 * INIT_WORK() key, schedule_work(), the static-key operations, the module
 * reference and the deferred-disable BUG site. The DO_ONCE() slow paths
 * live in lib/once.rs.
 */
#include <linux/bug.h>
#include <linux/build_bug.h>
#include <linux/jump_label.h>
#include <linux/module.h>
#include <linux/mutex.h>
#include <linux/slab.h>
#include <linux/spinlock.h>
#include <linux/stddef.h>
#include <linux/workqueue.h>

/* lib/once.rs mirrors struct once_work. */
struct once_work {
	struct work_struct work;
	struct static_key_true *key;
	struct module *module;
};
static_assert(offsetof(struct once_work, work) == 0);

static DEFINE_SPINLOCK(once_lock);
static DEFINE_MUTEX(once_mutex);

struct once_work *lupos_once_alloc_work(void);
void lupos_once_init_work(struct work_struct *work, work_func_t func);
bool lupos_once_schedule_work(struct work_struct *work);
bool lupos_once_key_enabled(struct static_key_true *key);
void lupos_once_branch_disable(struct static_key_true *key);
void lupos_once_module_get(struct module *module);
void lupos_once_bug_deferred_disabled(void);
unsigned long lupos_once_lock(void);
void lupos_once_unlock(unsigned long flags);
void lupos_once_mutex_lock(void);
void lupos_once_mutex_unlock(void);

struct once_work *lupos_once_alloc_work(void)
{
	struct once_work *w;

	w = kmalloc_obj(*w, GFP_ATOMIC);
	return w;
}

/* One INIT_WORK() lockdep class, as in once_disable_jump(). */
void lupos_once_init_work(struct work_struct *work, work_func_t func)
{
	INIT_WORK(work, func);
}

bool lupos_once_schedule_work(struct work_struct *work)
{
	return schedule_work(work);
}

bool lupos_once_key_enabled(struct static_key_true *key)
{
	return static_key_enabled(key);
}

void lupos_once_branch_disable(struct static_key_true *key)
{
	static_branch_disable(key);
}

void lupos_once_module_get(struct module *module)
{
	__module_get(module);
}

/* once_deferred(): the key must still be enabled. */
void lupos_once_bug_deferred_disabled(void)
{
	BUG();
}

unsigned long lupos_once_lock(void)
{
	unsigned long flags;

	spin_lock_irqsave(&once_lock, flags);
	return flags;
}

void lupos_once_unlock(unsigned long flags)
{
	spin_unlock_irqrestore(&once_lock, flags);
}

void lupos_once_mutex_lock(void)
{
	mutex_lock(&once_mutex);
}

void lupos_once_mutex_unlock(void)
{
	mutex_unlock(&once_mutex);
}
