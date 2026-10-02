// SPDX-License-Identifier: GPL-2.0+
/* C declaration, macro and inline boundaries for the Rust rseq provider.
 * Registration, critical-section validation, event processing and slice
 * policy are implemented in rseq.rs. No RSEQ_BUILD_SLOW_PATH here: its
 * out-of-line debug algorithm is supplied by Rust as well.
 */
#include "rseq_bindings.h"
#define CREATE_TRACE_POINTS
#include <trace/events/rseq.h>

/* The Rust file_operations tables take these addresses. Retain Clang's
 * declaration-derived CFI type metadata as in the original C tables. */
#ifdef CONFIG_CFI
__ADDRESSABLE(seq_read);
__ADDRESSABLE(seq_lseek);
__ADDRESSABLE(single_release);
#endif

DEFINE_STATIC_KEY_MAYBE(CONFIG_RSEQ_DEBUG_DEFAULT_ENABLE, rseq_debug_enabled);
struct task_struct *rust_rseq_current(void) { return current; }
unsigned int rust_rseq_cpu_limit(void) { return nr_cpu_ids; }
struct rseq_data *rust_rseq_data(struct task_struct *t) { return &t->rseq; }
unsigned int rust_rseq_task_flags(struct task_struct *t) { return t->flags; }
unsigned long rust_rseq_task_size(void) { return TASK_SIZE; }
unsigned long rust_rseq_ip(struct pt_regs *r) { return instruction_pointer(r); }
void rust_rseq_set_ip(struct pt_regs *r, unsigned long ip) { instruction_pointer_set(r, ip); }
void rust_rseq_irq_disable(void) { local_irq_disable(); }
void rust_rseq_irq_enable(void) { local_irq_enable(); }
void rust_rseq_preempt_disable(void) { preempt_disable(); }
void rust_rseq_preempt_enable(void) { preempt_enable(); }
void rust_rseq_cond_resched(void) { cond_resched(); }
u32 rust_rseq_task_cpu(struct task_struct *t) { return task_cpu(t); }
u32 rust_rseq_task_mm_cid(struct task_struct *t) { return task_mm_cid(t); }
u32 rust_rseq_cpu_to_node(u32 cpu) { return cpu_to_node(cpu); }
bool rust_rseq_debug_enabled(void) { return static_branch_unlikely(&rseq_debug_enabled); }
void rust_rseq_control_debug(bool on)
{
 if (on) static_branch_enable(&rseq_debug_enabled);
 else static_branch_disable(&rseq_debug_enabled);
}
bool rust_rseq_slice_enabled(void) { return rseq_slice_extension_enabled(); }
void rust_rseq_force_update(void) { rseq_force_update(); }
void rust_rseq_reset(struct task_struct *t) { rseq_reset(t); }
u32 rust_rseq_alloc_align(void) { return rseq_alloc_align(); }
bool rust_rseq_access_ok(const void __user *p, u32 len) { return access_ok(p, len); }
bool rust_rseq_read_csaddr(struct rseq __user *p, u64 *value)
{
 scoped_user_read_access(p, efault) unsafe_get_user(*value, &p->rseq_cs, efault);
 return true;
efault: return false;
}
bool rust_rseq_read_cs_u64(struct rseq_cs __user *p, size_t offset, u64 *value)
{
 scoped_user_rw_access(p, efault)
  unsafe_get_user(*value, (u64 __user *)((char __user *)p + offset), efault);
 return true;
efault: return false;
}
bool rust_rseq_read_rseq_u32(struct rseq __user *p, size_t offset, u32 *value)
{
 scoped_user_rw_access(p, efault)
  unsafe_get_user(*value, (u32 __user *)((char __user *)p + offset), efault);
 return true;
efault: return false;
}
/* A packet of primitive stores keeps the original order and partial-fault
 * behavior within one scoped user access. Rust supplies every value/offset.
 */
#define RSEQ_STORES(p, fields, count) do { \
 for (size_t i = 0; i < (count); i++) { \
  if ((fields)[i].wide) { \
   unsafe_put_user((fields)[i].value, (u64 __user *)((char __user *)(p) + (fields)[i].offset), efault); \
  } else { \
   unsafe_put_user((u32)(fields)[i].value, (u32 __user *)((char __user *)(p) + (fields)[i].offset), efault); \
  } \
 } \
} while (0)
bool rust_rseq_write_fields(struct rseq __user *p, const struct rust_rseq_user_write *fields, size_t count, bool rw)
{
 if (rw) { scoped_user_rw_access(p, efault) RSEQ_STORES(p, fields, count); }
 else { scoped_user_write_access(p, efault) RSEQ_STORES(p, fields, count); }
 return true;
efault: return false;
}
int rust_rseq_get_u32(const u32 __user *p, u32 *v) { return get_user(*v, p); }
int rust_rseq_get_u64(const u64 __user *p, u64 *v) { return get_user(*v, p); }
int rust_rseq_put_u32(u32 __user *p, u32 v) { return put_user(v, p); }
#ifdef CONFIG_RSEQ_STATS
DEFINE_PER_CPU(struct rseq_stats, rseq_stats);
unsigned int rust_rseq_next_cpu(unsigned int cpu) { return cpumask_next(cpu, cpu_possible_mask); }
void rust_rseq_read_stats(unsigned int cpu, struct rseq_stats *s)
{
#define READ_STAT(f) s->f = data_race(per_cpu(rseq_stats.f, cpu))
 READ_STAT(exit); READ_STAT(signal); READ_STAT(slowpath); READ_STAT(fastpath);
 READ_STAT(ids); READ_STAT(cs); READ_STAT(clear); READ_STAT(fixup);
 READ_STAT(s_granted); READ_STAT(s_expired); READ_STAT(s_revoked);
 READ_STAT(s_yielded); READ_STAT(s_aborted);
#undef READ_STAT
}
#endif
void rust_rseq_stat_inc(unsigned int field)
{
#ifdef CONFIG_RSEQ_STATS
 switch (field) {
#define STAT_CASE(n, f) case n: this_cpu_inc(rseq_stats.f); break
 STAT_CASE(0, exit); STAT_CASE(1, signal); STAT_CASE(2, slowpath); STAT_CASE(3, fastpath);
 STAT_CASE(4, ids); STAT_CASE(5, cs); STAT_CASE(6, clear); STAT_CASE(7, fixup);
 STAT_CASE(8, s_granted); STAT_CASE(9, s_expired); STAT_CASE(10, s_revoked);
 STAT_CASE(11, s_yielded); STAT_CASE(12, s_aborted);
#undef STAT_CASE
 }
#endif
}
struct dentry *rust_rseq_debugfs_dir(const char *name) { return debugfs_create_dir(name, NULL); }
void rust_rseq_debugfs_file(const char *n, umode_t mode, struct dentry *d, const struct file_operations *f) { debugfs_create_file(n, mode, d, NULL, f); }
void *rust_rseq_inode_private(struct inode *i) { return i->i_private; }
#ifdef CONFIG_TRACEPOINTS
void rust_rseq_trace_update(struct task_struct *t) { trace_rseq_update(t); }
void rust_rseq_trace_ip_fixup(unsigned long ip, unsigned long start, unsigned long offset, unsigned long abort) { trace_rseq_ip_fixup(ip, start, offset, abort); }
void rust_rseq_trace_fixup_if_enabled(unsigned long ip, unsigned long start, unsigned long offset, unsigned long abort) { rseq_trace_ip_fixup(ip, start, offset, abort); }
void rust_rseq_trace_update_if_enabled(struct task_struct *t) { rseq_trace_update(t, &t->rseq.ids); }
#endif

/* The syscall macros own the architecture ABI and metadata, not their bodies. */
extern long rust_sys_rseq(struct rseq __user *, u32, int, u32);
SYSCALL_DEFINE4(rseq, struct rseq __user *, rseq, u32, len, int, flags, u32, sig)
{ return rust_sys_rseq(rseq, len, flags, sig); }
extern int rust_rseq_setup_debug(char *str);
extern int rust_rseq_debugfs_init(void);
__setup("rseq_debug=", rust_rseq_setup_debug);
__initcall(rust_rseq_debugfs_init);

#ifdef CONFIG_RSEQ_SLICE_EXTENSION
static DEFINE_PER_CPU(struct slice_timer, slice_timer);
DEFINE_STATIC_KEY_TRUE(rseq_slice_extension_key);
struct slice_timer *rust_rseq_this_timer(void) { return this_cpu_ptr(&slice_timer); }
struct slice_timer *rust_rseq_cpu_timer(unsigned int cpu) { return per_cpu_ptr(&slice_timer, cpu); }
unsigned int rust_rseq_next_timer_cpu(unsigned int cpu) { return cpumask_next(cpu, cpu_possible_mask); }
void rust_rseq_lockdep_irqs_disabled(void) { lockdep_assert_irqs_disabled(); }
void rust_rseq_need_resched(void) { set_need_resched_current(); }
void rust_rseq_set_syscall_work(struct task_struct *t) { set_task_syscall_work(t, SYSCALL_RSEQ_SLICE); }
void rust_rseq_clear_syscall_work(struct task_struct *t) { clear_task_syscall_work(t, SYSCALL_RSEQ_SLICE); }
void rust_rseq_timer_start(struct hrtimer *t, ktime_t expires) { hrtimer_start(t, expires, HRTIMER_MODE_ABS_PINNED_HARD); }
/* C enum return types have their own normalized KCFI identity. Preserve
 * hrtimer's exact callback declaration while the callback body stays Rust. */
extern unsigned int rust_rseq_slice_expired(struct hrtimer *t);
static enum hrtimer_restart rust_rseq_slice_expired_callback(struct hrtimer *t)
{
 return rust_rseq_slice_expired(t);
}
void rust_rseq_timer_setup(struct hrtimer *t, clockid_t clock, enum hrtimer_mode mode)
{
 hrtimer_setup(t, rust_rseq_slice_expired_callback, clock, mode);
}
void rust_rseq_slice_disable(void) { static_branch_disable(&rseq_slice_extension_key); }
extern long rust_sys_rseq_slice_yield(void);
SYSCALL_DEFINE0(rseq_slice_yield) { return rust_sys_rseq_slice_yield(); }
extern int rust_rseq_slice_cmdline(char *str);
extern int rust_rseq_slice_init(void);
__setup("rseq_slice_ext=", rust_rseq_slice_cmdline);
device_initcall(rust_rseq_slice_init);
#endif
