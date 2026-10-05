/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_UMH_NATIVE_BINDINGS_H
#define LUPOS_UMH_NATIVE_BINDINGS_H

/* Inactive source proposal: configured native headers are the only authority. */
#include <linux/umh.h>
#include <linux/sched.h>
#include <linux/sched/task.h>
#include <linux/sched/signal.h>
#include <linux/binfmts.h>
#include <linux/slab.h>
#include <linux/completion.h>
#include <linux/cred.h>
#include <linux/fs_struct.h>
#include <linux/init.h>
#include <linux/initrd.h>
#include <linux/kernel.h>
#include <linux/jiffies.h>
#include <linux/rwsem.h>
#include <linux/string.h>
#include <linux/freezer.h>

static const int LUPOS_UMH_NO_WAIT = UMH_NO_WAIT;
static const int LUPOS_UMH_WAIT_PROC = UMH_WAIT_PROC;
static const int LUPOS_UMH_KILLABLE = UMH_KILLABLE;
static const int LUPOS_UMH_FREEZABLE = UMH_FREEZABLE;
static const enum umh_disable_depth LUPOS_UMH_ENABLED = UMH_ENABLED;
static const enum umh_disable_depth LUPOS_UMH_DISABLED = UMH_DISABLED;
static const unsigned int LUPOS_UMH_TASK_UNINTERRUPTIBLE = TASK_UNINTERRUPTIBLE;
static const unsigned int LUPOS_UMH_TASK_INTERRUPTIBLE = TASK_INTERRUPTIBLE;
static const unsigned int LUPOS_UMH_TASK_KILLABLE = TASK_KILLABLE;
static const unsigned int LUPOS_UMH_TASK_FREEZABLE = TASK_FREEZABLE;
static const unsigned long LUPOS_UMH_SIGCHLD = SIGCHLD;
static const unsigned long LUPOS_UMH_CLONE_PARENT = CLONE_PARENT;
static const long LUPOS_UMH_RUNNING_HELPERS_TIMEOUT = 5 * HZ;
static const gfp_t LUPOS_UMH_GFP_ATOMIC = GFP_ATOMIC;
static const gfp_t LUPOS_UMH_GFP_KERNEL = GFP_KERNEL;
static const int LUPOS_UMH_ENOMEM = ENOMEM;
static const int LUPOS_UMH_EINVAL = EINVAL;
static const int LUPOS_UMH_EBUSY = EBUSY;
static const int LUPOS_UMH_EAGAIN = EAGAIN;
static const int LUPOS_UMH_EPERM = EPERM;
static const int LUPOS_UMH_CAP_SETPCAP = CAP_SETPCAP;
static const int LUPOS_UMH_CAP_SYS_MODULE = CAP_SYS_MODULE;

/* Real native fields, allocation, callbacks and macro primitives; no policy. */
struct subprocess_info *lupos_umh_alloc(gfp_t flags);
struct subprocess_info *lupos_umh_from_work(struct work_struct *work);
struct work_struct *lupos_umh_work(struct subprocess_info *info);
struct completion *lupos_umh_xchg_complete(struct subprocess_info *info,
					struct completion *value);
void lupos_umh_set_complete(struct subprocess_info *info, struct completion *value);
const char *lupos_umh_path(const struct subprocess_info *info);
void lupos_umh_set_path(struct subprocess_info *info, const char *path);
char **lupos_umh_argv(const struct subprocess_info *info);
void lupos_umh_set_argv(struct subprocess_info *info, char **argv);
char **lupos_umh_envp(const struct subprocess_info *info);
void lupos_umh_set_envp(struct subprocess_info *info, char **envp);
int lupos_umh_wait(const struct subprocess_info *info);
void lupos_umh_set_wait(struct subprocess_info *info, int wait);
int lupos_umh_retval(const struct subprocess_info *info);
int *lupos_umh_retval_ptr(struct subprocess_info *info);
void lupos_umh_set_retval(struct subprocess_info *info, int retval);
typedef int (*lupos_umh_init_fn)(struct subprocess_info *, struct cred *);
typedef void (*lupos_umh_cleanup_fn)(struct subprocess_info *);
/* Native callback carrier for the signature-only public setup thunk. */
struct lupos_umh_callbacks {
	lupos_umh_init_fn init;
	lupos_umh_cleanup_fn cleanup;
};
lupos_umh_init_fn lupos_umh_setup_init(const struct lupos_umh_callbacks *callbacks);
lupos_umh_cleanup_fn lupos_umh_setup_cleanup(const struct lupos_umh_callbacks *callbacks);
struct subprocess_info *lupos_umh_setup_body(const char *path, char **argv,
		char **envp, gfp_t gfp_mask,
		const struct lupos_umh_callbacks *callbacks, void *data);
lupos_umh_init_fn lupos_umh_init(const struct subprocess_info *info);
void lupos_umh_set_init(struct subprocess_info *info, lupos_umh_init_fn init);
lupos_umh_cleanup_fn lupos_umh_cleanup(const struct subprocess_info *info);
void lupos_umh_set_cleanup(struct subprocess_info *info, lupos_umh_cleanup_fn cleanup);
void lupos_umh_set_data(struct subprocess_info *info, void *data);
void lupos_umh_init_work(struct work_struct *work, work_func_t callback);
bool lupos_umh_queue_work(struct work_struct *work);
void lupos_umh_init_completion(struct completion *done);
size_t lupos_umh_strlen(const char *path);
#ifdef CONFIG_STATIC_USERMODEHELPER
const char *lupos_umh_static_path(void);
#endif

struct task_struct *lupos_umh_current(void);
void lupos_umh_lock_siglock_irq(struct task_struct *task);
void lupos_umh_unlock_siglock_irq(struct task_struct *task);
void lupos_umh_set_umask(struct task_struct *task, int mask);
void lupos_umh_sigchld_default(void);
void lupos_umh_sigchld_ignore(void);
void lupos_umh_wait_for_initramfs(void);
kernel_cap_t *lupos_umh_cred_bset(struct cred *cred);
kernel_cap_t *lupos_umh_cred_inheritable(struct cred *cred);
kernel_cap_t *lupos_umh_bset(void);
kernel_cap_t *lupos_umh_inheritable(void);
void lupos_umh_cap_intersect(kernel_cap_t *out, const kernel_cap_t *a,
			    const kernel_cap_t *b);
void lupos_umh_sysctl_lock(void);
void lupos_umh_sysctl_unlock(void);

void lupos_umh_down_read(void);
void lupos_umh_up_read(void);
void lupos_umh_down_write(void);
void lupos_umh_up_write(void);
enum umh_disable_depth lupos_umh_disabled(void);
void lupos_umh_set_disabled(enum umh_disable_depth depth);
struct wait_queue_head *lupos_umh_disabled_waitq(void);
struct wait_queue_head *lupos_umh_running_waitq(void);
void lupos_umh_wake_up(struct wait_queue_head *head);
void lupos_umh_init_wait(struct wait_queue_entry *wait);
void lupos_umh_try_to_freeze(void);
void lupos_umh_might_sleep(void);
int lupos_umh_running_read(void);
void lupos_umh_running_inc(void);
bool lupos_umh_running_dec_and_test(void);
void lupos_umh_mb_after_atomic(void);

#ifdef CONFIG_SYSCTL
bool lupos_umh_capable(int cap);
kernel_cap_t *lupos_umh_table_cap(const struct ctl_table *table);
void lupos_umh_copy_table(struct ctl_table *out, const struct ctl_table *table);
void lupos_umh_set_table_data(struct ctl_table *table, void *data);
u64 lupos_umh_cap_value(const kernel_cap_t *cap);
void lupos_umh_set_cap_value(kernel_cap_t *cap, u64 value);
void __init lupos_umh_register_sysctl(void);
/* Existing Rust bodies, referenced by the native table and init entry. */
int lupos_umh_proc_cap_handler(const struct ctl_table *table, int write,
			     void *buffer, size_t *lenp, loff_t *ppos);
/* Rust body uses .init.text; the native registered entry owns full __init. */
int lupos_umh_init_sysctls(void);
#endif
#endif /* LUPOS_UMH_NATIVE_BINDINGS_H */
