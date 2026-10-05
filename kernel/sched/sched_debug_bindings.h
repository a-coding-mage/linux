/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SCHED_DEBUG_BINDINGS_H
#define LUPOS_SCHED_DEBUG_BINDINGS_H
/* Layouts, bitfields, ABI widths and constants come only from native headers. */
#include <linux/debugfs.h>
#include <linux/nmi.h>
#include <linux/log2.h>
#include <linux/ratelimit.h>
#include <linux/sched/clock.h>
#include <linux/sched/debug.h>
#include <linux/sched/task.h>
#include <linux/sched_clock.h>
#include <linux/utsname.h>
#include <linux/timex.h>
#include "sched.h"
#include "stats.h"
#include "autogroup.h"

static const unsigned int LUPOS_DEBUG_NR_CPUS = NR_CPUS;
static const size_t LUPOS_DEBUG_RQ_NR_RUNNING_SIZE = sizeof(((struct rq *)0)->nr_running);
static const size_t LUPOS_DEBUG_RQ_NR_SWITCHES_SIZE = sizeof(((struct rq *)0)->nr_switches);
static const size_t LUPOS_DEBUG_RQ_NR_UNINTERRUPTIBLE_SIZE = sizeof(((struct rq *)0)->nr_uninterruptible);
static const unsigned int LUPOS_DEBUG_SCHED_FEAT_NR = __SCHED_FEAT_NR;
static const unsigned int LUPOS_DEBUG_TUNABLESCALING_END = SCHED_TUNABLESCALING_END;
static const unsigned int LUPOS_DEBUG_SD_FLAG_CNT = __SD_FLAG_CNT;
static const unsigned int LUPOS_DEBUG_SD_ASYM_PACKING = SD_ASYM_PACKING;
static const unsigned int LUPOS_DEBUG_UTIL_AVG_UNCHANGED = UTIL_AVG_UNCHANGED;
static const int LUPOS_DEBUG_EINVAL = EINVAL;
static const int LUPOS_DEBUG_EFAULT = EFAULT;
static const int LUPOS_DEBUG_EBUSY = EBUSY;
static const unsigned long LUPOS_DEBUG_SERVER_PERIOD_MAX = (1UL << 22) * NSEC_PER_USEC;
static const unsigned long LUPOS_DEBUG_SERVER_PERIOD_MIN = 100 * NSEC_PER_USEC;
#ifdef CONFIG_CGROUP_SCHED
static const unsigned int LUPOS_DEBUG_PATH_MAX = PATH_MAX;
#endif
#ifdef CONFIG_UCLAMP_TASK
static const unsigned int LUPOS_DEBUG_UCLAMP_MIN = UCLAMP_MIN;
static const unsigned int LUPOS_DEBUG_UCLAMP_MAX = UCLAMP_MAX;
#endif

/* Private registration IDs, not substituted Linux constants or ABI layouts. */
enum lupos_debug_file_kind {
 LUPOS_DEBUG_FEATURES, LUPOS_DEBUG_SCALING, LUPOS_DEBUG_PREEMPT,
 LUPOS_DEBUG_CACHE, LUPOS_DEBUG_CGROUP, LUPOS_DEBUG_DUMP,
 LUPOS_DEBUG_FAIR_RUNTIME, LUPOS_DEBUG_FAIR_PERIOD,
 LUPOS_DEBUG_EXT_RUNTIME, LUPOS_DEBUG_EXT_PERIOD, LUPOS_DEBUG_SD_FLAGS,
};
extern struct dentry *lupos_debug_root;
extern struct dentry *lupos_debug_sd_dentry;

/* Rust-owned callbacks: registration glue must never call original owners. */
#define LUPOS_DEBUG_RW_CALLBACKS(name) \
 int lupos_debug_##name##_show(struct seq_file *, void *); \
 ssize_t lupos_debug_##name##_write(struct file *, const char __user *, size_t, loff_t *)
LUPOS_DEBUG_RW_CALLBACKS(feat);
LUPOS_DEBUG_RW_CALLBACKS(scaling);
LUPOS_DEBUG_RW_CALLBACKS(fair_runtime);
LUPOS_DEBUG_RW_CALLBACKS(fair_period);
#ifdef CONFIG_PREEMPT_DYNAMIC
LUPOS_DEBUG_RW_CALLBACKS(dynamic);
#endif
#ifdef CONFIG_SCHED_CACHE
LUPOS_DEBUG_RW_CALLBACKS(cache_enable);
#endif
#ifdef CONFIG_FAIR_GROUP_SCHED
LUPOS_DEBUG_RW_CALLBACKS(cgroup);
#endif
#ifdef CONFIG_SCHED_CLASS_EXT
LUPOS_DEBUG_RW_CALLBACKS(ext_runtime);
LUPOS_DEBUG_RW_CALLBACKS(ext_period);
#endif
#undef LUPOS_DEBUG_RW_CALLBACKS
ssize_t lupos_debug_verbose_write(struct file *, const char __user *, size_t, loff_t *);
int lupos_debug_sd_flags_show(struct seq_file *, void *);
int lupos_debug_show(struct seq_file *, void *);
void *lupos_debug_start(struct seq_file *, loff_t *);
void *lupos_debug_next(struct seq_file *, void *, loff_t *);
void lupos_debug_stop(struct seq_file *, void *);
int __init lupos_debug_init(void);

/* source_line selects the literal/provenance in sched_debug_printk_sites.h. */
__printf(3, 4) void lupos_debug_printf(struct seq_file *, unsigned int source_line, const char *, ...);
__printf(2, 3) void lupos_debug_seq_printf(struct seq_file *, const char *, ...);
__printf(1, 2) void lupos_debug_error(const char *, ...);
__printf(1, 2) void lupos_debug_info(const char *, ...);
__printf(3, 4) int lupos_debug_snprintf(char *, size_t, const char *, ...);
void lupos_debug_seq_puts(struct seq_file *, const char *);
const char *lupos_debug_feat_name(int);
#ifdef CONFIG_JUMP_LABEL
void lupos_debug_feat_disable(int);
void lupos_debug_feat_enable(int);
#endif
unsigned long lupos_debug_copy_from_user(void *, const void __user *, unsigned long);
int lupos_debug_strcmp(const char *, const char *);
int lupos_debug_strncmp(const char *, const char *, size_t);
char *lupos_debug_strstrip(char *);
size_t lupos_debug_strcspn(const char *, const char *);
struct inode *lupos_debug_file_inode(struct file *);
void lupos_debug_inode_lock(struct inode *);
void lupos_debug_inode_unlock(struct inode *);
void lupos_debug_cpus_read_lock(void);
void lupos_debug_cpus_read_unlock(void);
int lupos_debug_kstrtouint_from_user(const char __user *, size_t, unsigned int, unsigned int *);
int lupos_debug_kstrtoull_from_user(const char __user *, size_t, unsigned int, u64 *);
int lupos_debug_update_scaling(void);
#ifdef CONFIG_SCHED_CACHE
int lupos_debug_kstrtobool_from_user(const char __user *, size_t, bool *);
void lupos_debug_cache_active_set(void);
#endif
#ifdef CONFIG_PREEMPT_DYNAMIC
int lupos_debug_dynamic_mode(const char *);
void lupos_debug_dynamic_update(int);
int lupos_debug_dynamic_read(void);
const char *lupos_debug_preempt_mode(int);
#endif
#ifdef CONFIG_FAIR_GROUP_SCHED
void lupos_debug_cgroup_mode_update(int);
void lupos_debug_cgroup_mode_write(int);
int lupos_debug_cgroup_mode_read(void);
#endif
void lupos_debug_domains_lock(void);
void lupos_debug_domains_unlock(void);
ssize_t lupos_debug_write_file_bool(struct file *, const char __user *, size_t, loff_t *);
struct rq *lupos_debug_cpu_rq(int);
struct task_struct *lupos_debug_rq_curr(struct rq *);
int lupos_debug_cpu_of(struct rq *);
bool lupos_debug_cpu_online(int);
void lupos_debug_rq_lock(struct rq *, struct rq_flags *);
void lupos_debug_rq_unlock(struct rq *, struct rq_flags *);
void lupos_debug_update_rq_clock(struct rq *);
void lupos_debug_server_stop(struct sched_dl_entity *);
int lupos_debug_server_apply_params(struct sched_dl_entity *, u64, u64, bool);
void lupos_debug_server_start(struct sched_dl_entity *);
struct dentry *lupos_debug_create_dir(const char *, struct dentry *);
void lupos_debug_create_file(const char *, umode_t, struct dentry *, void *, enum lupos_debug_file_kind);
void lupos_debug_create_verbose(struct dentry *);
void lupos_debug_create_u32(const char *, umode_t, struct dentry *, u32 *);
void lupos_debug_create_u64(const char *, umode_t, struct dentry *, u64 *);
void lupos_debug_create_ulong(const char *, umode_t, struct dentry *, unsigned long *);
void lupos_debug_create_str(const char *, umode_t, struct dentry *, char **);
void lupos_debug_remove(struct dentry *);
void lupos_debug_lookup_and_remove(const char *, struct dentry *);
const struct cpumask *lupos_debug_online_mask(void);
const struct cpumask *lupos_debug_possible_mask(void);
unsigned int lupos_debug_mask_first(const struct cpumask *);
unsigned int lupos_debug_mask_next(int, const struct cpumask *);
unsigned int lupos_debug_mask_bound(void);
unsigned int lupos_debug_nr_cpu_ids(void);
bool lupos_debug_sd_mask_available(void);
bool lupos_debug_sd_mask_alloc(void);
struct cpumask *lupos_debug_sd_mask(void);
void lupos_debug_mask_copy(struct cpumask *, const struct cpumask *);
bool lupos_debug_mask_empty(const struct cpumask *);
void lupos_debug_mask_clear_cpu(unsigned int, struct cpumask *);
void lupos_debug_mask_set_cpu(unsigned int, struct cpumask *);
struct sched_domain *lupos_debug_first_domain(int);
const char *lupos_debug_sd_flag_name(unsigned int);
void lupos_debug_touch_nmi_watchdog(void);
void lupos_debug_touch_all_softlockup_watchdogs(void);
bool lupos_debug_latency_ratelimit(void);
void lupos_debug_dump_stack(void);
bool lupos_debug_schedstat_enabled(void);
#ifdef CONFIG_FAIR_GROUP_SCHED
struct sched_entity *lupos_debug_tg_se(struct task_group *, int);
struct sched_statistics *lupos_debug_stats_from_se(struct sched_entity *);
long lupos_debug_atomic_long_read(const atomic_long_t *);
#endif
#ifdef CONFIG_CGROUP_SCHED
int lupos_debug_autogroup_path(struct task_group *, char *, int);
int lupos_debug_cgroup_path(struct cgroup *, char *, int);
bool lupos_debug_group_path_trylock(void);
char *lupos_debug_group_path_buffer(void);
void lupos_debug_group_path_unlock(void);
struct task_group *lupos_debug_task_group(struct task_struct *);
#endif
int lupos_debug_task_current(struct rq *, struct task_struct *);
char lupos_debug_task_state_to_char(struct task_struct *);
int lupos_debug_task_pid_nr(struct task_struct *);
int lupos_debug_task_pid_nr_ns(struct task_struct *, struct pid_namespace *);
int lupos_debug_get_nr_threads(struct task_struct *);
int lupos_debug_entity_eligible(struct cfs_rq *, struct sched_entity *);
bool lupos_debug_custom_slice(const struct sched_entity *);
#ifdef CONFIG_NUMA_BALANCING
int lupos_debug_task_node(struct task_struct *);
pid_t lupos_debug_task_numa_group_id(struct task_struct *);
void lupos_debug_show_numa_stats(struct task_struct *, struct seq_file *);
#endif
void lupos_debug_rcu_read_lock(void);
void lupos_debug_rcu_read_unlock(void);
struct task_struct *lupos_debug_init_task(void);
struct task_struct *lupos_debug_next_task(struct task_struct *);
struct list_head *lupos_debug_thread_head(struct task_struct *);
void lupos_debug_thread_check_rcu(void);
struct list_head *lupos_debug_list_next_rcu(struct list_head *);
struct task_struct *lupos_debug_task_from_thread_node(struct list_head *);
int lupos_debug_task_cpu(struct task_struct *);
unsigned long lupos_debug_raw_rq_lock_irqsave(struct rq *);
void lupos_debug_raw_rq_unlock_irqrestore(struct rq *, unsigned long);
struct sched_entity *lupos_debug_pick_root_entity(struct cfs_rq *);
struct sched_entity *lupos_debug_pick_first_entity(struct cfs_rq *);
struct sched_entity *lupos_debug_pick_last_entity(struct cfs_rq *);
u64 lupos_debug_avg_vruntime(struct cfs_rq *);
int lupos_debug_ilog2_abs(s64);
void lupos_debug_print_cfs_stats(struct seq_file *, int);
void lupos_debug_print_rt_stats(struct seq_file *, int);
void lupos_debug_print_dl_stats(struct seq_file *, int);
unsigned long lupos_debug_local_irq_save(void);
void lupos_debug_local_irq_restore(unsigned long);
u64 lupos_debug_ktime_get_ns(void);
u64 lupos_debug_sched_clock(void);
u64 lupos_debug_local_clock(void);
struct new_utsname *lupos_debug_init_utsname(void);
unsigned long lupos_debug_jiffies(void);
#ifdef CONFIG_HAVE_UNSTABLE_SCHED_CLOCK
int lupos_debug_sched_clock_stable(void);
#endif
#ifdef CONFIG_UCLAMP_TASK
unsigned int lupos_debug_uclamp_req(struct task_struct *, unsigned int);
unsigned long lupos_debug_uclamp_eff(struct task_struct *, unsigned int);
#endif
bool lupos_debug_task_has_dl_policy(struct task_struct *);
bool lupos_debug_fair_policy(int);
#ifdef CONFIG_SCHED_CLASS_EXT
bool lupos_debug_task_on_scx(struct task_struct *);
#endif
unsigned int lupos_debug_raw_smp_processor_id(void);
u64 lupos_debug_cpu_clock(int);
#ifdef CONFIG_CFS_BANDWIDTH
bool lupos_debug_cfs_throttled(struct cfs_rq *);
#endif
#endif /* LUPOS_SCHED_DEBUG_BINDINGS_H */
