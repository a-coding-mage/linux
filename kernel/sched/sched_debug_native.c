// SPDX-License-Identifier: GPL-2.0-only
/* Native header/locking/formatting/registration leaves only. No debug.c owners.
 * Source proposal must inherit build_utility.c's complete policy before admission.
 */
#include "sched_debug_bindings.h"
#error "Lupos scheduler debug native leaves are source-only and not build-admitted"
#include "sched_debug_printk_index.h"

__read_mostly bool sched_debug_verbose;
struct dentry *lupos_debug_root;
struct dentry *lupos_debug_sd_dentry;
static cpumask_var_t sd_sysctl_cpus;
#define SCHED_FEAT(name, enabled) #name,
static const char * const sched_feat_names[] = {
#include "features.h"
};
#undef SCHED_FEAT
#ifdef CONFIG_JUMP_LABEL
#define jump_label_key__true STATIC_KEY_INIT_TRUE
#define jump_label_key__false STATIC_KEY_INIT_FALSE
#define SCHED_FEAT(name, enabled) jump_label_key__##enabled,
struct static_key sched_feat_keys[__SCHED_FEAT_NR] = {
#include "features.h"
};
#undef SCHED_FEAT
#undef jump_label_key__true
#undef jump_label_key__false
void lupos_debug_feat_disable(int i) { static_key_disable_cpuslocked(&sched_feat_keys[i]); }
void lupos_debug_feat_enable(int i) { static_key_enable_cpuslocked(&sched_feat_keys[i]); }
#endif
const char *lupos_debug_feat_name(int i) { return sched_feat_names[i]; }

void lupos_debug_printf(struct seq_file *m, unsigned int source_line, const char *fmt, ...)
{
 va_list args;
 struct va_format vaf;
 /* Metadata is supplied at the original literal sites, not at the %pV bridge.
  * source_line must match fmt and its original site in the native manifest.
  */
 lupos_debug_print_index(source_line);
 va_start(args, fmt);
 if (m) seq_vprintf(m, fmt, args);
 else {
  vaf.fmt = fmt;
  vaf.va = &args;
  _printk(KERN_CONT "%pV", &vaf);
 }
 va_end(args);
}
void lupos_debug_seq_printf(struct seq_file *m, const char *fmt, ...)
{
 va_list args;
 va_start(args, fmt);
 seq_vprintf(m, fmt, args);
 va_end(args);
}
void lupos_debug_error(const char *fmt, ...)
{
 va_list args;
 struct va_format vaf;
 va_start(args, fmt);
 vaf.fmt = fmt; vaf.va = &args;
 LUPOS_DEBUG_INDEX(KERN_ERR pr_fmt("sched: CPU %d need_resched set for > %llu ns (%d ticks) without schedule\n"),
                   "resched_latency_warn", 1538);
 _printk(KERN_ERR pr_fmt("%pV"), &vaf);
 va_end(args);
}
void lupos_debug_info(const char *fmt, ...)
{
 va_list args;
 struct va_format vaf;
 va_start(args, fmt);
 vaf.fmt = fmt; vaf.va = &args;
 LUPOS_DEBUG_INDEX(KERN_INFO pr_fmt("%s server %sabled on CPU %d%s.\n"),
                   "sched_server_write_common", 432);
 _printk(KERN_INFO pr_fmt("%pV"), &vaf);
 va_end(args);
}
int lupos_debug_snprintf(char *buf, size_t size, const char *fmt, ...)
{
 int ret;
 va_list args;
 va_start(args, fmt);
 ret = vsnprintf(buf, size, fmt, args);
 va_end(args);
 return ret;
}
void lupos_debug_seq_puts(struct seq_file *m, const char *s) { seq_puts(m, s); }
unsigned long lupos_debug_copy_from_user(void *to, const void __user *from, unsigned long n) { return copy_from_user(to, from, n); }
int lupos_debug_strcmp(const char *a, const char *b) { return strcmp(a, b); }
int lupos_debug_strncmp(const char *a, const char *b, size_t n) { return strncmp(a, b, n); }
char *lupos_debug_strstrip(char *s) { return strstrip(s); }
size_t lupos_debug_strcspn(const char *s, const char *reject) { return strcspn(s, reject); }
struct inode *lupos_debug_file_inode(struct file *f) { return file_inode(f); }
void lupos_debug_inode_lock(struct inode *i) { inode_lock(i); }
void lupos_debug_inode_unlock(struct inode *i) { inode_unlock(i); }
void lupos_debug_cpus_read_lock(void) { cpus_read_lock(); }
void lupos_debug_cpus_read_unlock(void) { cpus_read_unlock(); }
int lupos_debug_kstrtouint_from_user(const char __user *s, size_t n, unsigned int base, unsigned int *out) { return kstrtouint_from_user(s, n, base, out); }
int lupos_debug_kstrtoull_from_user(const char __user *s, size_t n, unsigned int base, u64 *out) { return kstrtoull_from_user(s, n, base, out); }
int lupos_debug_update_scaling(void) { return sched_update_scaling(); }
#ifdef CONFIG_SCHED_CACHE
int lupos_debug_kstrtobool_from_user(const char __user *s, size_t n, bool *out) { return kstrtobool_from_user(s, n, out); }
void lupos_debug_cache_active_set(void) { sched_cache_active_set(); }
#endif
#ifdef CONFIG_PREEMPT_DYNAMIC
int lupos_debug_dynamic_mode(const char *s) { return sched_dynamic_mode(s); }
void lupos_debug_dynamic_update(int mode) { sched_dynamic_update(mode); }
int lupos_debug_dynamic_read(void) { return READ_ONCE(preempt_dynamic_mode); }
const char *lupos_debug_preempt_mode(int index) { return preempt_modes[index]; }
#endif
#ifdef CONFIG_FAIR_GROUP_SCHED
static int cgroup_mode = 2;
void lupos_debug_cgroup_mode_update(int mode) { __sched_cgroup_mode_update(mode); }
void lupos_debug_cgroup_mode_write(int mode) { WRITE_ONCE(cgroup_mode, mode); }
int lupos_debug_cgroup_mode_read(void) { return READ_ONCE(cgroup_mode); }
#endif
void lupos_debug_domains_lock(void) { sched_domains_mutex_lock(); }
void lupos_debug_domains_unlock(void) { sched_domains_mutex_unlock(); }
ssize_t lupos_debug_write_file_bool(struct file *f, const char __user *s, size_t n, loff_t *pos) { return debugfs_write_file_bool(f, s, n, pos); }
struct rq *lupos_debug_cpu_rq(int cpu) { return cpu_rq(cpu); }
struct task_struct *lupos_debug_rq_curr(struct rq *rq) { return rq->curr; }
int lupos_debug_cpu_of(struct rq *rq) { return cpu_of(rq); }
bool lupos_debug_cpu_online(int cpu) { return cpu_online(cpu); }
void lupos_debug_rq_lock(struct rq *rq, struct rq_flags *rf) { rq_lock_irqsave(rq, rf); }
void lupos_debug_rq_unlock(struct rq *rq, struct rq_flags *rf) { rq_unlock_irqrestore(rq, rf); }
void lupos_debug_update_rq_clock(struct rq *rq) { update_rq_clock(rq); }
void lupos_debug_server_stop(struct sched_dl_entity *se) { dl_server_stop(se); }
int lupos_debug_server_apply_params(struct sched_dl_entity *se, u64 runtime, u64 period, bool init) { return dl_server_apply_params(se, runtime, period, init); }
void lupos_debug_server_start(struct sched_dl_entity *se) { dl_server_start(se); }

const struct cpumask *lupos_debug_online_mask(void) { return cpu_online_mask; }
const struct cpumask *lupos_debug_possible_mask(void) { return cpu_possible_mask; }
unsigned int lupos_debug_mask_first(const struct cpumask *m) { return cpumask_first(m); }
unsigned int lupos_debug_mask_next(int cpu, const struct cpumask *m) { return cpumask_next(cpu, m); }
unsigned int lupos_debug_mask_bound(void) { return small_cpumask_bits; }
unsigned int lupos_debug_nr_cpu_ids(void) { return nr_cpu_ids; }
bool lupos_debug_sd_mask_available(void) { return cpumask_available(sd_sysctl_cpus); }
bool lupos_debug_sd_mask_alloc(void) { return alloc_cpumask_var(&sd_sysctl_cpus, GFP_KERNEL); }
struct cpumask *lupos_debug_sd_mask(void) { return sd_sysctl_cpus; }
void lupos_debug_mask_copy(struct cpumask *to, const struct cpumask *from) { cpumask_copy(to, from); }
bool lupos_debug_mask_empty(const struct cpumask *m) { return cpumask_empty(m); }
void lupos_debug_mask_clear_cpu(unsigned int cpu, struct cpumask *m) { __cpumask_clear_cpu(cpu, m); }
void lupos_debug_mask_set_cpu(unsigned int cpu, struct cpumask *m) { __cpumask_set_cpu(cpu, m); }
struct sched_domain *lupos_debug_first_domain(int cpu) { return rcu_dereference_sched_domain(cpu_rq(cpu)->sd); }
const char *lupos_debug_sd_flag_name(unsigned int i) { return sd_flag_debug[i].name; }
void lupos_debug_touch_nmi_watchdog(void) { touch_nmi_watchdog(); }
void lupos_debug_touch_all_softlockup_watchdogs(void) { touch_all_softlockup_watchdogs(); }
bool lupos_debug_latency_ratelimit(void)
{
 static DEFINE_RATELIMIT_STATE(latency_check_ratelimit, 60 * 60 * HZ, 1);
 return !likely(!___ratelimit(&latency_check_ratelimit, "resched_latency_warn"));
}
void lupos_debug_dump_stack(void) { dump_stack(); }
bool lupos_debug_schedstat_enabled(void) { return schedstat_enabled(); }
#ifdef CONFIG_FAIR_GROUP_SCHED
struct sched_entity *lupos_debug_tg_se(struct task_group *tg, int cpu) { return tg_se(tg, cpu); }
struct sched_statistics *lupos_debug_stats_from_se(struct sched_entity *se) { return __schedstats_from_se(se); }
long lupos_debug_atomic_long_read(const atomic_long_t *v) { return atomic_long_read(v); }
#endif
#ifdef CONFIG_CGROUP_SCHED
static DEFINE_SPINLOCK(sched_debug_lock);
static char group_path[PATH_MAX];
int lupos_debug_autogroup_path(struct task_group *tg, char *path, int plen) { return autogroup_path(tg, path, plen); }
int lupos_debug_cgroup_path(struct cgroup *cg, char *path, int plen) { return cgroup_path(cg, path, plen); }
bool lupos_debug_group_path_trylock(void) { return spin_trylock(&sched_debug_lock); }
char *lupos_debug_group_path_buffer(void) { return group_path; }
void lupos_debug_group_path_unlock(void) { spin_unlock(&sched_debug_lock); }
struct task_group *lupos_debug_task_group(struct task_struct *p) { return task_group(p); }
#endif
int lupos_debug_task_current(struct rq *rq, struct task_struct *p) { return task_current(rq, p); }
char lupos_debug_task_state_to_char(struct task_struct *p) { return task_state_to_char(p); }
int lupos_debug_task_pid_nr(struct task_struct *p) { return task_pid_nr(p); }
int lupos_debug_task_pid_nr_ns(struct task_struct *p, struct pid_namespace *ns) { return task_pid_nr_ns(p, ns); }
int lupos_debug_get_nr_threads(struct task_struct *p) { return get_nr_threads(p); }
int lupos_debug_entity_eligible(struct cfs_rq *rq, struct sched_entity *se) { return entity_eligible(rq, se); }
bool lupos_debug_custom_slice(const struct sched_entity *se) { return se->custom_slice; }
#ifdef CONFIG_NUMA_BALANCING
int lupos_debug_task_node(struct task_struct *p) { return task_node(p); }
pid_t lupos_debug_task_numa_group_id(struct task_struct *p) { return task_numa_group_id(p); }
void lupos_debug_show_numa_stats(struct task_struct *p, struct seq_file *m) { show_numa_stats(p, m); }
#endif
void lupos_debug_rcu_read_lock(void) { rcu_read_lock(); }
void lupos_debug_rcu_read_unlock(void) { rcu_read_unlock(); }
struct task_struct *lupos_debug_init_task(void) { return &init_task; }
struct task_struct *lupos_debug_next_task(struct task_struct *p) { return next_task(p); }
struct list_head *lupos_debug_thread_head(struct task_struct *p) { return &p->signal->thread_head; }
void lupos_debug_thread_check_rcu(void) { __list_check_rcu(dummy, lockdep_is_held(&tasklist_lock), 0); }
struct list_head *lupos_debug_list_next_rcu(struct list_head *node) { return READ_ONCE(node->next); }
struct task_struct *lupos_debug_task_from_thread_node(struct list_head *node) { return list_entry(node, struct task_struct, thread_node); }
int lupos_debug_task_cpu(struct task_struct *p) { return task_cpu(p); }
unsigned long lupos_debug_raw_rq_lock_irqsave(struct rq *rq)
{
 unsigned long flags;
 raw_spin_rq_lock_irqsave(rq, flags);
 return flags;
}
void lupos_debug_raw_rq_unlock_irqrestore(struct rq *rq, unsigned long flags) { raw_spin_rq_unlock_irqrestore(rq, flags); }
struct sched_entity *lupos_debug_pick_root_entity(struct cfs_rq *rq) { return __pick_root_entity(rq); }
struct sched_entity *lupos_debug_pick_first_entity(struct cfs_rq *rq) { return __pick_first_entity(rq); }
struct sched_entity *lupos_debug_pick_last_entity(struct cfs_rq *rq) { return __pick_last_entity(rq); }
u64 lupos_debug_avg_vruntime(struct cfs_rq *rq) { return avg_vruntime(rq); }
int lupos_debug_ilog2_abs(s64 n) { return ilog2(abs(n)); }
/* These three owners are in fair.c, rt.c and deadline.c, never debug.c. */
void lupos_debug_print_cfs_stats(struct seq_file *m, int cpu) { print_cfs_stats(m, cpu); }
void lupos_debug_print_rt_stats(struct seq_file *m, int cpu) { print_rt_stats(m, cpu); }
void lupos_debug_print_dl_stats(struct seq_file *m, int cpu) { print_dl_stats(m, cpu); }
unsigned long lupos_debug_local_irq_save(void)
{
 unsigned long flags;
 local_irq_save(flags);
 return flags;
}
void lupos_debug_local_irq_restore(unsigned long flags) { local_irq_restore(flags); }
u64 lupos_debug_ktime_get_ns(void) { return ktime_to_ns(ktime_get()); }
u64 lupos_debug_sched_clock(void) { return sched_clock(); }
u64 lupos_debug_local_clock(void) { return local_clock(); }
struct new_utsname *lupos_debug_init_utsname(void) { return init_utsname(); }
unsigned long lupos_debug_jiffies(void) { return jiffies; }
#ifdef CONFIG_HAVE_UNSTABLE_SCHED_CLOCK
int lupos_debug_sched_clock_stable(void) { return sched_clock_stable(); }
#endif
#ifdef CONFIG_UCLAMP_TASK
unsigned int lupos_debug_uclamp_req(struct task_struct *p, unsigned int id) { return p->uclamp_req[id].value; }
unsigned long lupos_debug_uclamp_eff(struct task_struct *p, unsigned int id) { return uclamp_eff_value(p, id); }
#endif
bool lupos_debug_task_has_dl_policy(struct task_struct *p) { return task_has_dl_policy(p); }
bool lupos_debug_fair_policy(int policy) { return fair_policy(policy); }
#ifdef CONFIG_SCHED_CLASS_EXT
bool lupos_debug_task_on_scx(struct task_struct *p) { return task_on_scx(p); }
#endif
unsigned int lupos_debug_raw_smp_processor_id(void) { return raw_smp_processor_id(); }
u64 lupos_debug_cpu_clock(int cpu) { return cpu_clock(cpu); }
#ifdef CONFIG_CFS_BANDWIDTH
bool lupos_debug_cfs_throttled(struct cfs_rq *rq) { return rq->throttled; }
#endif
