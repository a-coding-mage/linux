/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_PID_BINDINGS_H
#define LUPOS_PID_BINDINGS_H
/* Same configured definitions as kernel/pid.c; no reproduced kernel layouts. */
#include <linux/mm.h>
#include <linux/export.h>
#include <linux/slab.h>
#include <linux/init.h>
#include <linux/rculist.h>
#include <linux/memblock.h>
#include <linux/pid_namespace.h>
#include <linux/init_task.h>
#include <linux/syscalls.h>
#include <linux/proc_ns.h>
#include <linux/refcount.h>
#include <linux/anon_inodes.h>
#include <linux/sched/signal.h>
#include <linux/sched/task.h>
#include <linux/idr.h>
#include <linux/pidfs.h>
#include <net/sock.h>
#include <uapi/linux/pidfd.h>
#include <linux/file.h>
#include <linux/ptrace.h>
#include <linux/sysctl.h>
#include "pid_layout.h"

/* Declaration-only aligned storage; Rust owns its sole definition. */
extern spinlock_t lupos_pidmap_lock_state __cacheline_aligned_in_smp;
struct lupos_pidmap_lock_storage { spinlock_t lock; }
    __aligned(__alignof__(lupos_pidmap_lock_state));
static const size_t LUPOS_PID_MAP_LOCK_SIZE = sizeof(struct lupos_pidmap_lock_storage);
static const size_t LUPOS_PID_MAP_LOCK_ALIGN = __alignof__(struct lupos_pidmap_lock_storage);
static const size_t LUPOS_PID_MAP_LOCK_OFFSET = offsetof(struct lupos_pidmap_lock_storage, lock);
static const size_t LUPOS_PID_RAW_LOCK_SIZE = sizeof(raw_spinlock_t);
static const size_t LUPOS_PID_RAW_LOCK_ALIGN = __alignof__(raw_spinlock_t);
static const size_t LUPOS_PID_SPIN_LOCK_SIZE = sizeof(spinlock_t);
static const size_t LUPOS_PID_SPIN_LOCK_ALIGN = __alignof__(spinlock_t);
#ifndef CONFIG_PREEMPT_RT
static const size_t LUPOS_PID_SPIN_RAW_OFFSET = offsetof(spinlock_t, rlock);
#endif
static const gfp_t LUPOS_PID_IDR_MARKER = IDR_RT_MARKER;
static const unsigned int LUPOS_PID_NS_TYPE = ns_common_type(&init_pid_ns);
static const u64 LUPOS_PID_NS_ID = ns_init_id(&init_pid_ns);
static const unsigned int LUPOS_PID_NS_INUM = ns_init_inum(&init_pid_ns);
static const unsigned long LUPOS_PID_SPIN_OWNER = (unsigned long)SPINLOCK_OWNER_INIT;
static const unsigned int LUPOS_PID_SPIN_OWNER_CPU = -1;
static const unsigned int LUPOS_PID_LD_WAIT_SPIN = LD_WAIT_SPIN;
static const unsigned int LUPOS_PID_LD_WAIT_CONFIG = LD_WAIT_CONFIG;
#if !defined(CONFIG_SMP) && defined(CONFIG_DEBUG_SPINLOCK)
static const unsigned int LUPOS_PID_UP_UNLOCKED = __ARCH_SPIN_LOCK_UNLOCKED;
#endif

/* Expose typed macro values to bindgen, including config-dependent slab flags. */
static const gfp_t LUPOS_PID_GFP_KERNEL = GFP_KERNEL;
static const gfp_t LUPOS_PID_GFP_ATOMIC = GFP_ATOMIC;
static const slab_flags_t LUPOS_PID_SLAB_FLAGS =
	SLAB_HWCACHE_ALIGN | SLAB_PANIC | SLAB_ACCOUNT;
static const size_t LUPOS_PID_INITIAL_SIZE = struct_size_t(struct pid, numbers, 1);
static const size_t LUPOS_PID_ALIGN = __alignof__(struct pid);
static const int LUPOS_PID_MAX_LIMIT = PID_MAX_LIMIT;
static const int LUPOS_PID_MAX_DEFAULT = PID_MAX_DEFAULT;
static const int LUPOS_PID_RESERVED_PIDS = RESERVED_PIDS;
static const int LUPOS_PID_PIDS_PER_CPU_DEFAULT = PIDS_PER_CPU_DEFAULT;
static const int LUPOS_PID_PIDS_PER_CPU_MIN = PIDS_PER_CPU_MIN;

struct task_struct *lupos_pid_current(void);
void lupos_pid_map_lock(void);
void lupos_pid_map_unlock(void);
void lupos_pid_assert_tasklist_write(void);
void lupos_pid_assert_tasklist_unheld(void);
void lupos_pid_warn_rcu(void);
void lupos_pid_warn_reaper(bool value);
void lupos_pid_warn_pending(bool value);
void lupos_pid_warn_transfer(bool value);
void lupos_pid_bug(bool value);
bool lupos_pid_ref_dec(refcount_t *ref);
void lupos_pid_ref_set(refcount_t *ref, int value);
struct pid *lupos_pid_get(struct pid *pid);
struct pid_namespace *lupos_pid_ns_get(struct pid_namespace *ns);
void lupos_pid_ns_put(struct pid_namespace *ns);
void lupos_pid_ns_active_get(struct pid_namespace *ns);
void lupos_pid_ns_active_put(struct pid_namespace *ns);
void lupos_pid_spin_init(spinlock_t *lock);
void lupos_pid_wait_init(wait_queue_head_t *wait);
void lupos_pid_hlist_init(struct hlist_head *head);
void lupos_pid_hlist_add(struct hlist_node *node, struct hlist_head *head);
void lupos_pid_hlist_del(struct hlist_node *node);
void lupos_pid_hlist_swap(struct hlist_head *left, struct hlist_head *right);
void lupos_pid_hlist_replace(struct hlist_node *old, struct hlist_node *new);
bool lupos_pid_hlist_empty(struct hlist_head *head);
struct hlist_node *lupos_pid_hlist_first(struct hlist_head *head);
void lupos_pid_rcu_lock(void);
void lupos_pid_rcu_unlock(void);
struct pid *lupos_pid_rcu_load(struct pid **ptr);
void lupos_pid_rcu_store(struct pid **ptr, struct pid *value);
void lupos_pid_write_nr(pid_t *ptr, pid_t value);
int lupos_pid_read_int(const int *ptr);
struct task_struct *lupos_pid_read_task(struct task_struct * const *ptr);
void lupos_pid_call_rcu(struct rcu_head *head);
void lupos_pid_get_task(struct task_struct *task);
void lupos_pid_put_task(struct task_struct *task);
bool lupos_pid_checkpoint_capable(struct user_namespace *ns);
void *lupos_pid_cache_alloc(struct kmem_cache *cache, gfp_t flags);
struct kmem_cache *lupos_pid_cache_create(const char *name, unsigned int size, unsigned int align, slab_flags_t flags);
void *lupos_pid_memdup(const void *src, size_t len, gfp_t flags);
unsigned int lupos_pid_idr_cursor(const struct idr *idr);
void lupos_pid_idr_set_cursor(struct idr *idr, unsigned int cursor);
void lupos_pid_idr_preload_end(void);
void lupos_pid_idr_init(struct idr *idr);
unsigned int lupos_pid_possible_cpus(void);
void lupos_pid_log_limits(unsigned int max, unsigned int min);
void *lupos_pid_err_ptr(long err);
bool lupos_pid_is_err(const void *ptr);
long lupos_pid_ptr_err(const void *ptr);
struct file *lupos_pid_fd_file(struct fd fd);
bool lupos_pid_fd_empty(struct fd fd);
void lupos_pid_fdput(struct fd fd);
#ifdef CONFIG_SYSCTL
kuid_t lupos_pid_current_euid(void);
bool lupos_pid_ns_capable_noaudit(struct user_namespace *ns, int cap);
int lupos_pid_in_egroup(kgid_t gid);
kuid_t lupos_pid_make_kuid(struct user_namespace *ns, uid_t uid);
kgid_t lupos_pid_make_kgid(struct user_namespace *ns, gid_t gid);
bool lupos_pid_uid_eq(kuid_t left, kuid_t right);
bool lupos_pid_uid_valid(kuid_t uid);
bool lupos_pid_gid_valid(kgid_t gid);
struct ctl_table_set *lupos_pid_sysctl_lookup(struct ctl_table_root *root);
int lupos_pid_sysctl_seen(struct ctl_table_set *set);
int lupos_pid_sysctl_permissions(struct ctl_table_header *head, const struct ctl_table *table);
void lupos_pid_sysctl_ownership(struct ctl_table_header *head, kuid_t *uid, kgid_t *gid);
#endif
#endif
