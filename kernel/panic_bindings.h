/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_PANIC_BINDINGS_H
#define LUPOS_PANIC_BINDINGS_H
/* Match the authoritative panic.c include environment. Layouts come from
 * these configured declarations; no handwritten kernel layout is used. */
#include <linux/debug_locks.h>
#include <linux/sched/debug.h>
#include <linux/interrupt.h>
#include <linux/kgdb.h>
#include <linux/kmsg_dump.h>
#include <linux/kallsyms.h>
#include <linux/notifier.h>
#include <linux/vt_kern.h>
#include <linux/module.h>
#include <linux/random.h>
#include <linux/ftrace.h>
#include <linux/reboot.h>
#include <linux/delay.h>
#include <linux/kexec.h>
#include <linux/panic_notifier.h>
#include <linux/sched.h>
#include <linux/string_helpers.h>
#include <linux/sysrq.h>
#include <linux/init.h>
#include <linux/nmi.h>
#include <linux/console.h>
#include <linux/bug.h>
#include <linux/ratelimit.h>
#include <linux/debugfs.h>
#include <linux/sysfs.h>
#include <linux/context_tracking.h>
#include <linux/seq_buf.h>
#include <linux/sys_info.h>
#include <trace/events/error_report.h>
#include <asm/sections.h>
#include <kunit/test-bug.h>

/* This is the private type declared in panic.c, exposed verbatim for bindgen. */
struct warn_args {
	const char *fmt;
	va_list args;
};

/* Preserve typedef-specific alignment which cannot be expressed by a Rust
 * type alias. The native-derived wrapper has no owned state in this header. */
struct lupos_panic_csd_storage {
	call_single_data_t data;
} __aligned(__alignof__(call_single_data_t));
static const size_t LUPOS_PANIC_CSD_SIZE = sizeof(call_single_data_t);
static const size_t LUPOS_PANIC_CSD_ALIGN = __alignof__(call_single_data_t);
static const size_t LUPOS_PANIC_CSD_STORAGE_SIZE = sizeof(struct lupos_panic_csd_storage);
static const size_t LUPOS_PANIC_CSD_DATA_OFFSET = offsetof(struct lupos_panic_csd_storage, data);
static const size_t LUPOS_PANIC_SPIN_SIZE = sizeof(spinlock_t);
static const size_t LUPOS_PANIC_SPIN_ALIGN = __alignof__(spinlock_t);
static const size_t LUPOS_PANIC_RAW_SIZE = sizeof(raw_spinlock_t);
static const size_t LUPOS_PANIC_RAW_ALIGN = __alignof__(raw_spinlock_t);
#ifndef CONFIG_PREEMPT_RT
static const size_t LUPOS_PANIC_SPIN_RAW_OFFSET = offsetof(spinlock_t, rlock);
#endif
static const unsigned long LUPOS_PANIC_SPIN_OWNER = (unsigned long)SPINLOCK_OWNER_INIT;
static const unsigned int LUPOS_PANIC_SPIN_OWNER_CPU = -1;
static const unsigned int LUPOS_PANIC_LD_WAIT_SPIN = LD_WAIT_SPIN;
static const unsigned int LUPOS_PANIC_LD_WAIT_CONFIG = LD_WAIT_CONFIG;
#if !defined(CONFIG_SMP) && defined(CONFIG_DEBUG_SPINLOCK)
static const unsigned int LUPOS_PANIC_UP_UNLOCKED = __ARCH_SPIN_LOCK_UNLOCKED;
#endif
static const size_t LUPOS_PANIC_VA_SIZE = sizeof(va_list);
static const size_t LUPOS_PANIC_VA_ALIGN = __alignof__(va_list);
static const gfp_t LUPOS_PANIC_GFP_KERNEL = GFP_KERNEL;
static const int LUPOS_PANIC_TIMEOUT = CONFIG_PANIC_TIMEOUT;

/* Weak architecture entry symbols. Their native definitions only forward to
 * Rust defaults, and architecture overrides remain stronger ELF definitions. */
void panic_smp_self_stop(void) __noreturn;
void nmi_panic_self_stop(struct pt_regs *regs) __noreturn;
int panic_smp_redirect_cpu(int target_cpu, void *msg);

/* Header and architecture primitives. No panic.c branch or policy lives here. */
int lupos_panic_raw_cpu(void);
int lupos_panic_smp_cpu(void);
unsigned int lupos_panic_nr_cpu_ids(void);
bool lupos_panic_cpu_online(unsigned int cpu);
void lupos_panic_cpu_relax(void);
void lupos_panic_smp_send_stop(void);
void lupos_panic_arch_irq_disable(void);
void lupos_panic_arch_irq_enable(void);
bool lupos_panic_arch_irqs_disabled(void);
void lupos_panic_preempt_disable_notrace(void);
int lupos_panic_atomic_read(const atomic_t *v);
void lupos_panic_atomic_set(atomic_t *v, int i);
bool lupos_panic_atomic_try_cmpxchg(atomic_t *v, int *old, int new);
int lupos_panic_atomic_inc_return(atomic_t *v);
unsigned int lupos_panic_read_uint(const unsigned int *v);
bool lupos_panic_test_bit(unsigned int bit, const unsigned long *value);
void lupos_panic_set_bit(unsigned int bit, unsigned long *value);
bool lupos_panic_capable(int cap);
int lupos_panic_debug_locks_off(void);
unsigned long lupos_panic_spin_lock_irqsave(spinlock_t *lock);
void lupos_panic_spin_unlock_irqrestore(spinlock_t *lock, unsigned long flags);
void lupos_panic_spin_lock(spinlock_t *lock);
void lupos_panic_spin_unlock(spinlock_t *lock);
void *lupos_panic_kmalloc(size_t size, gfp_t flags);
void lupos_panic_mdelay(unsigned long msecs);
void lupos_panic_touch_nmi_watchdog(void);
void lupos_panic_touch_softlockup_watchdog(void);
void lupos_panic_trigger_all_backtrace(void);
void lupos_panic_trigger_allbutcpu_backtrace(int cpu);
void lupos_panic_kgdb(const char *msg);
void lupos_panic_crash_kexec(struct pt_regs *regs);
void lupos_panic_kmsg_dump_desc(enum kmsg_dump_reason reason, const char *desc);
void lupos_panic_console_unblank(void);
void lupos_panic_tracing_off(void);
void lupos_panic_disable_trace_on_warning(void);
void lupos_panic_nbcon_emergency_enter(void);
void lupos_panic_nbcon_emergency_exit(void);
void lupos_panic_print_modules(void);
struct task_struct *lupos_panic_current(void);
void lupos_panic_print_irqtrace_events(struct task_struct *task);
void lupos_panic_trace_error_report_end(enum error_detector detector, unsigned long id);
bool lupos_panic_seq_buf_warn_zero(bool zero);
/* Rust providers, also callable by a separately routed original-C test build. */
void lupos_panic_seq_buf_clear(struct seq_buf *s);
void lupos_panic_seq_buf_init(struct seq_buf *s, char *buf, unsigned int size);
const char *lupos_panic_seq_buf_str(struct seq_buf *s);
void lupos_panic_vprintk_args(struct warn_args *args);
void lupos_panic_warn_with_args(const char *file, int line, void *caller,
	unsigned int taint, const char *fmt, va_list args);
bool lupos_panic_warn_rcu_enter(void);
void lupos_panic_warn_rcu_exit(bool rcu);
bool lupos_panic_kunit_suppressed(bool warn_slowpath);
void lupos_panic_generic_bug_clear_once(void);
struct dentry *lupos_panic_debugfs_create_file_unsafe(const char *name, umode_t mode,
	struct dentry *parent, void *data, const struct file_operations *fops);
ssize_t lupos_panic_debugfs_attr_read(struct file *file, char __user *buf,
	size_t len, loff_t *ppos);
ssize_t lupos_panic_debugfs_attr_write(struct file *file, const char __user *buf,
	size_t len, loff_t *ppos);
#ifdef CONFIG_SYSCTL
void __init lupos_panic_register_sysctl(const char *path, const struct ctl_table *table,
	const char *table_name, size_t size);
#endif
#ifdef CONFIG_SPARC
extern int stop_a_enabled;
#endif
#ifdef CONFIG_S390
void lupos_panic_disabled_wait(void);
#endif
#endif
