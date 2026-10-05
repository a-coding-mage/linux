// SPDX-License-Identifier: GPL-2.0-only
#error "SOURCE ONLY HOLD: usermode-helper native primitives are unqualified"
/*
 * Inactive, unqualified native interface/storage for the existing Rust UMH
 * owner. No process, credential, completion-ownership or wait policy lives here.
 */
#include "umh_native_bindings.h"

static kernel_cap_t usermodehelper_bset = CAP_FULL_SET;
static kernel_cap_t usermodehelper_inheritable = CAP_FULL_SET;
static DEFINE_SPINLOCK(umh_sysctl_lock);
static DECLARE_RWSEM(umhelper_sem);
static enum umh_disable_depth usermodehelper_disabled = UMH_DISABLED;
static atomic_t running_helpers = ATOMIC_INIT(0);
static DECLARE_WAIT_QUEUE_HEAD(running_helpers_waitq);
static DECLARE_WAIT_QUEUE_HEAD(usermodehelper_disabled_waitq);

struct subprocess_info *lupos_umh_alloc(gfp_t flags)
{
	return kzalloc_obj(struct subprocess_info, flags);
}

struct subprocess_info *lupos_umh_from_work(struct work_struct *work)
{
	return container_of(work, struct subprocess_info, work);
}

struct work_struct *lupos_umh_work(struct subprocess_info *info)
{
	return &info->work;
}

struct completion *lupos_umh_xchg_complete(struct subprocess_info *info,
					struct completion *value)
{
	return xchg(&info->complete, value);
}

void lupos_umh_set_complete(struct subprocess_info *info, struct completion *value)
{
	info->complete = value;
}

const char *lupos_umh_path(const struct subprocess_info *info)
{
	return info->path;
}

void lupos_umh_set_path(struct subprocess_info *info, const char *path)
{
	info->path = path;
}

char **lupos_umh_argv(const struct subprocess_info *info)
{
	return info->argv;
}

void lupos_umh_set_argv(struct subprocess_info *info, char **argv)
{
	info->argv = argv;
}

char **lupos_umh_envp(const struct subprocess_info *info)
{
	return info->envp;
}

void lupos_umh_set_envp(struct subprocess_info *info, char **envp)
{
	info->envp = envp;
}

int lupos_umh_wait(const struct subprocess_info *info)
{
	return info->wait;
}

void lupos_umh_set_wait(struct subprocess_info *info, int wait)
{
	info->wait = wait;
}

int lupos_umh_retval(const struct subprocess_info *info)
{
	return info->retval;
}

int *lupos_umh_retval_ptr(struct subprocess_info *info)
{
	return &info->retval;
}

void lupos_umh_set_retval(struct subprocess_info *info, int retval)
{
	info->retval = retval;
}

lupos_umh_init_fn lupos_umh_init(const struct subprocess_info *info)
{
	return info->init;
}

lupos_umh_init_fn lupos_umh_setup_init(const struct lupos_umh_callbacks *callbacks)
{
	return callbacks->init;
}

lupos_umh_cleanup_fn lupos_umh_setup_cleanup(const struct lupos_umh_callbacks *callbacks)
{
	return callbacks->cleanup;
}

void lupos_umh_set_init(struct subprocess_info *info, lupos_umh_init_fn init)
{
	info->init = init;
}

lupos_umh_cleanup_fn lupos_umh_cleanup(const struct subprocess_info *info)
{
	return info->cleanup;
}

void lupos_umh_set_cleanup(struct subprocess_info *info, lupos_umh_cleanup_fn cleanup)
{
	info->cleanup = cleanup;
}

void lupos_umh_set_data(struct subprocess_info *info, void *data)
{
	info->data = data;
}

void lupos_umh_init_work(struct work_struct *work, work_func_t callback)
{
	INIT_WORK(work, callback);
}

bool lupos_umh_queue_work(struct work_struct *work)
{
	return queue_work(system_dfl_wq, work);
}

void lupos_umh_init_completion(struct completion *done)
{
#ifdef CONFIG_LOCKDEP
	init_completion(done);
#else
	/*
	 * Preserve DECLARE_COMPLETION_ONSTACK's native initializer branch.
	 * Self-links refer directly to final storage, never to a temporary.
	 */
	*done = (struct completion)COMPLETION_INITIALIZER(*done);
#endif
}

size_t lupos_umh_strlen(const char *path)
{
	return strlen(path);
}

#ifdef CONFIG_STATIC_USERMODEHELPER
static const char usermodehelper_path[] = CONFIG_STATIC_USERMODEHELPER_PATH;

const char *lupos_umh_static_path(void)
{
	return usermodehelper_path;
}
#endif

struct task_struct *lupos_umh_current(void)
{
	return current;
}

void lupos_umh_lock_siglock_irq(struct task_struct *task)
{
	spin_lock_irq(&task->sighand->siglock);
}

void lupos_umh_unlock_siglock_irq(struct task_struct *task)
{
	spin_unlock_irq(&task->sighand->siglock);
}

void lupos_umh_set_umask(struct task_struct *task, int mask)
{
	task->fs->umask = mask;
}

void lupos_umh_sigchld_default(void)
{
	kernel_sigaction(SIGCHLD, SIG_DFL);
}

void lupos_umh_sigchld_ignore(void)
{
	kernel_sigaction(SIGCHLD, SIG_IGN);
}

void lupos_umh_wait_for_initramfs(void)
{
	wait_for_initramfs();
}

kernel_cap_t *lupos_umh_cred_bset(struct cred *cred)
{
	return &cred->cap_bset;
}

kernel_cap_t *lupos_umh_cred_inheritable(struct cred *cred)
{
	return &cred->cap_inheritable;
}

kernel_cap_t *lupos_umh_bset(void)
{
	return &usermodehelper_bset;
}

kernel_cap_t *lupos_umh_inheritable(void)
{
	return &usermodehelper_inheritable;
}

void lupos_umh_cap_intersect(kernel_cap_t *out, const kernel_cap_t *a,
			    const kernel_cap_t *b)
{
	*out = cap_intersect(*a, *b);
}

void lupos_umh_sysctl_lock(void)
{
	spin_lock(&umh_sysctl_lock);
}

void lupos_umh_sysctl_unlock(void)
{
	spin_unlock(&umh_sysctl_lock);
}

void lupos_umh_down_read(void)
{
	down_read(&umhelper_sem);
}

void lupos_umh_up_read(void)
{
	up_read(&umhelper_sem);
}

void lupos_umh_down_write(void)
{
	down_write(&umhelper_sem);
}

void lupos_umh_up_write(void)
{
	up_write(&umhelper_sem);
}

enum umh_disable_depth lupos_umh_disabled(void)
{
	return usermodehelper_disabled;
}

void lupos_umh_set_disabled(enum umh_disable_depth depth)
{
	usermodehelper_disabled = depth;
}

struct wait_queue_head *lupos_umh_disabled_waitq(void)
{
	return &usermodehelper_disabled_waitq;
}

struct wait_queue_head *lupos_umh_running_waitq(void)
{
	return &running_helpers_waitq;
}

void lupos_umh_wake_up(struct wait_queue_head *head)
{
	wake_up(head);
}

void lupos_umh_init_wait(struct wait_queue_entry *wait)
{
	init_wait(wait);
}

void lupos_umh_try_to_freeze(void)
{
	try_to_freeze();
}

void lupos_umh_might_sleep(void)
{
	might_sleep();
}

int lupos_umh_running_read(void)
{
	return atomic_read(&running_helpers);
}

void lupos_umh_running_inc(void)
{
	atomic_inc(&running_helpers);
}

bool lupos_umh_running_dec_and_test(void)
{
	return atomic_dec_and_test(&running_helpers);
}

void lupos_umh_mb_after_atomic(void)
{
	smp_mb__after_atomic();
}

#ifdef CONFIG_SYSCTL
bool lupos_umh_capable(int cap)
{
	return capable(cap);
}

kernel_cap_t *lupos_umh_table_cap(const struct ctl_table *table)
{
	return table->data;
}

void lupos_umh_copy_table(struct ctl_table *out, const struct ctl_table *table)
{
	*out = *table;
}

void lupos_umh_set_table_data(struct ctl_table *table, void *data)
{
	table->data = data;
}

u64 lupos_umh_cap_value(const kernel_cap_t *cap)
{
	return cap->val;
}

void lupos_umh_set_cap_value(kernel_cap_t *cap, u64 value)
{
	cap->val = value;
}

static const struct ctl_table usermodehelper_table[] = {
	{
		.procname = "bset",
		.data = &usermodehelper_bset,
		.maxlen = 2 * sizeof(unsigned long),
		.mode = 0600,
		.proc_handler = lupos_umh_proc_cap_handler,
	},
	{
		.procname = "inheritable",
		.data = &usermodehelper_inheritable,
		.maxlen = 2 * sizeof(unsigned long),
		.mode = 0600,
		.proc_handler = lupos_umh_proc_cap_handler,
	},
};

void __init lupos_umh_register_sysctl(void)
{
	register_sysctl_init("kernel/usermodehelper", usermodehelper_table);
}
#endif
