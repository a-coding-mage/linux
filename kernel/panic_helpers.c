// SPDX-License-Identifier: GPL-2.0-only
/* Native header/compiler/metadata boundary for panic.rs. No original panic.c
 * decision, warning/oops algorithm, taint policy or owned data is implemented
 * here. In particular panic()/vpanic() are Rust C-variadic entry points. */
#include "panic_bindings.h"

#if !defined(CONFIG_X86_64) || !defined(CONFIG_PRINTK) || defined(CONFIG_PRINTK_INDEX) || defined(CONFIG_CFI)
#error "Rust panic native boundary is outside its admitted configuration"
#endif
#if defined(CONFIG_BUG) && !defined(__WARN_FLAGS)
#error "x86_64 warning macro ABI changed; review Rust warning selection"
#endif

int lupos_panic_raw_cpu(void) { return raw_smp_processor_id(); }
int lupos_panic_smp_cpu(void) { return smp_processor_id(); }
unsigned int lupos_panic_nr_cpu_ids(void) { return nr_cpu_ids; }
bool lupos_panic_cpu_online(unsigned int cpu) { return cpu_online(cpu); }
void lupos_panic_cpu_relax(void) { cpu_relax(); }
void lupos_panic_smp_send_stop(void) { smp_send_stop(); }
void lupos_panic_arch_irq_disable(void) { arch_local_irq_disable(); }
void lupos_panic_arch_irq_enable(void) { arch_local_irq_enable(); }
bool lupos_panic_arch_irqs_disabled(void) { return arch_irqs_disabled(); }
void lupos_panic_preempt_disable_notrace(void) { preempt_disable_notrace(); }
int lupos_panic_atomic_read(const atomic_t *v) { return atomic_read(v); }
void lupos_panic_atomic_set(atomic_t *v, int i) { atomic_set(v, i); }
bool lupos_panic_atomic_try_cmpxchg(atomic_t *v, int *old, int new)
{ return atomic_try_cmpxchg(v, old, new); }
int lupos_panic_atomic_inc_return(atomic_t *v) { return atomic_inc_return(v); }
unsigned int lupos_panic_read_uint(const unsigned int *v) { return READ_ONCE(*v); }
bool lupos_panic_test_bit(unsigned int bit, const unsigned long *value)
{ return test_bit(bit, value); }
void lupos_panic_set_bit(unsigned int bit, unsigned long *value) { set_bit(bit, value); }
bool lupos_panic_capable(int cap) { return capable(cap); }
int lupos_panic_debug_locks_off(void) { return __debug_locks_off(); }
unsigned long lupos_panic_spin_lock_irqsave(spinlock_t *lock)
{ unsigned long flags; spin_lock_irqsave(lock, flags); return flags; }
void lupos_panic_spin_unlock_irqrestore(spinlock_t *lock, unsigned long flags)
{ spin_unlock_irqrestore(lock, flags); }
void lupos_panic_spin_lock(spinlock_t *lock) { spin_lock(lock); }
void lupos_panic_spin_unlock(spinlock_t *lock) { spin_unlock(lock); }
void *lupos_panic_kmalloc(size_t size, gfp_t flags) { return kmalloc(size, flags); }
void lupos_panic_mdelay(unsigned long msecs) { mdelay(msecs); }
void lupos_panic_touch_nmi_watchdog(void) { touch_nmi_watchdog(); }
void lupos_panic_touch_softlockup_watchdog(void) { touch_softlockup_watchdog(); }
void lupos_panic_trigger_all_backtrace(void) { trigger_all_cpu_backtrace(); }
void lupos_panic_trigger_allbutcpu_backtrace(int cpu) { trigger_allbutcpu_cpu_backtrace(cpu); }
void lupos_panic_kgdb(const char *msg) { kgdb_panic(msg); }
void lupos_panic_crash_kexec(struct pt_regs *regs) { __crash_kexec(regs); }
void lupos_panic_kmsg_dump_desc(enum kmsg_dump_reason reason, const char *desc)
{ kmsg_dump_desc(reason, desc); }
void lupos_panic_console_unblank(void) { console_unblank(); }
void lupos_panic_tracing_off(void) { tracing_off(); }
void lupos_panic_disable_trace_on_warning(void) { disable_trace_on_warning(); }
void lupos_panic_nbcon_emergency_enter(void) { nbcon_cpu_emergency_enter(); }
void lupos_panic_nbcon_emergency_exit(void) { nbcon_cpu_emergency_exit(); }
void lupos_panic_print_modules(void) { print_modules(); }
struct task_struct *lupos_panic_current(void) { return current; }
void lupos_panic_print_irqtrace_events(struct task_struct *task) { print_irqtrace_events(task); }
void lupos_panic_trace_error_report_end(enum error_detector detector, unsigned long id)
{ trace_error_report_end(detector, id); }
void lupos_panic_seq_buf_init(struct seq_buf *s, char *buf, unsigned int size)
{ seq_buf_init(s, buf, size); }
const char *lupos_panic_seq_buf_str(struct seq_buf *s) { return seq_buf_str(s); }
void lupos_panic_vprintk_args(struct warn_args *args) { vprintk(args->fmt, args->args); }
bool lupos_panic_warn_rcu_enter(void) { return warn_rcu_enter(); }
void lupos_panic_warn_rcu_exit(bool rcu) { warn_rcu_exit(rcu); }
bool lupos_panic_kunit_suppressed(bool warn_slowpath)
{ return kunit_is_suppressed_warning(warn_slowpath); }
void lupos_panic_generic_bug_clear_once(void) { generic_bug_clear_once(); }
struct dentry *lupos_panic_debugfs_create_file_unsafe(const char *name, umode_t mode,
	struct dentry *parent, void *data, const struct file_operations *fops)
{ return debugfs_create_file_unsafe(name, mode, parent, data, fops); }
ssize_t lupos_panic_debugfs_attr_read(struct file *file, char __user *buf,
	size_t len, loff_t *ppos) { return debugfs_attr_read(file, buf, len, ppos); }
ssize_t lupos_panic_debugfs_attr_write(struct file *file, const char __user *buf,
	size_t len, loff_t *ppos) { return debugfs_attr_write(file, buf, len, ppos); }
#ifdef CONFIG_SYSCTL
void __init lupos_panic_register_sysctl(const char *path, const struct ctl_table *table,
	const char *table_name, size_t size) { __register_sysctl_init(path, table, table_name, size); }
#endif
#ifdef CONFIG_S390
void lupos_panic_disabled_wait(void) { disabled_wait(); }
#endif

/* Native ABI aggregation only: canonical va_list copying and destruction.
 * Suppression, RCU, output, caller selection and warning policy stay in Rust. */
void lupos_panic_warn_with_args(const char *file, int line, void *caller,
	unsigned int taint, const char *fmt, va_list args)
{
	struct warn_args native_args = { .fmt = fmt };
	va_copy(native_args.args, args);
	__warn(file, line, caller, taint, NULL, &native_args);
	va_end(native_args.args);
}

extern void lupos_panic_default_smp_self_stop(void) __noreturn;
extern void lupos_panic_default_nmi_self_stop(struct pt_regs *regs) __noreturn;
extern void lupos_panic_default_crash_smp_send_stop(void);
void __weak __noreturn panic_smp_self_stop(void) { lupos_panic_default_smp_self_stop(); }
void __weak __noreturn nmi_panic_self_stop(struct pt_regs *regs)
{ lupos_panic_default_nmi_self_stop(regs); }
void __weak crash_smp_send_stop(void) { lupos_panic_default_crash_smp_send_stop(); }
#if defined(CONFIG_SMP) && defined(CONFIG_CRASH_DUMP)
extern int lupos_panic_default_redirect_cpu(int target_cpu, void *msg);
int __weak panic_smp_redirect_cpu(int target_cpu, void *msg)
{ return lupos_panic_default_redirect_cpu(target_cpu, msg); }
#endif

/* The C compiler must own this entry's noinstr annotations and return-address
 * acquisition. The original fatal message/panic action is in the Rust body. */
#ifdef CONFIG_STACKPROTECTOR
extern void lupos_panic_stack_chk_fail(void *caller) __noreturn;
__visible noinstr void __stack_chk_fail(void)
{
	unsigned long flags;
	instrumentation_begin();
	flags = user_access_save();
	lupos_panic_stack_chk_fail(__builtin_return_address(0));
	user_access_restore(flags);
	instrumentation_end();
}
EXPORT_SYMBOL(__stack_chk_fail);
#endif

/* Native initcall/setup/parameter/export sections; the callbacks and tables
 * referenced by these declarations are Rust definitions. */
#ifdef CONFIG_SYSCTL
extern int __init lupos_panic_sysctls_init(void);
late_initcall(lupos_panic_sysctls_init);
#endif
#ifdef CONFIG_SYSFS
extern int __init lupos_panic_sysfs_init(void);
late_initcall(lupos_panic_sysfs_init);
#endif
extern int __init lupos_panic_setup_sys_info(char *buf);
__setup("panic_sys_info=", lupos_panic_setup_sys_info);
#if defined(CONFIG_SMP) && defined(CONFIG_CRASH_DUMP)
extern int __init lupos_panic_force_cpu_setup(char *str);
extern int __init lupos_panic_force_cpu_late_init(void);
early_param("panic_force_cpu", lupos_panic_force_cpu_setup);
late_initcall(lupos_panic_force_cpu_late_init);
#endif
extern int __init lupos_panic_alloc_taint_buf(void);
postcore_initcall(lupos_panic_alloc_taint_buf);
#ifdef CONFIG_BUG
extern int __init lupos_panic_register_warn_debugfs(void);
device_initcall(lupos_panic_register_warn_debugfs);
#ifndef __WARN_FLAGS
EXPORT_SYMBOL(warn_slowpath_fmt);
#else
EXPORT_SYMBOL(__warn_printk);
#endif
#endif

extern int lupos_panic_pause_on_oops;
extern bool lupos_panic_console_replay;
extern const struct kernel_param_ops lupos_panic_print_ops;
core_param(panic, panic_timeout, int, 0644);
core_param(pause_on_oops, lupos_panic_pause_on_oops, int, 0644);
core_param(panic_on_warn, panic_on_warn, int, 0644);
core_param(crash_kexec_post_notifiers, crash_kexec_post_notifiers, bool, 0644);
core_param(panic_console_replay, lupos_panic_console_replay, bool, 0644);
__core_param_cb(panic_print, &lupos_panic_print_ops, &panic_print, 0644);
extern int __init lupos_panic_oops_setup(char *s);
extern int __init lupos_panic_on_taint_setup(char *s);
early_param("oops", lupos_panic_oops_setup);
early_param("panic_on_taint", lupos_panic_on_taint_setup);

EXPORT_SYMBOL_GPL(panic_timeout);
EXPORT_SYMBOL(panic_notifier_list);
EXPORT_SYMBOL(panic_blink);
EXPORT_SYMBOL(panic_try_start);
EXPORT_SYMBOL(panic_reset);
EXPORT_SYMBOL(panic_in_progress);
EXPORT_SYMBOL(panic_on_this_cpu);
EXPORT_SYMBOL(panic_on_other_cpu);
EXPORT_SYMBOL(nmi_panic);
EXPORT_SYMBOL(vpanic);
EXPORT_SYMBOL(panic);
EXPORT_SYMBOL(test_taint);
EXPORT_SYMBOL(add_taint);
