// SPDX-License-Identifier: GPL-2.0-only
/* Native compiler/architecture and ABI metadata boundaries for exit.rs.
 * No original kernel/exit.c algorithm or owned data definition is retained. */
#include "exit_bindings.h"
#define RX(result, name, args, ...) result rust_exit_##name args __VA_ARGS__
#include "exit_primitives.inc"
#undef RX

/* These shims preserve native CFI identities for C-invoked callbacks. */
extern void rust_exit_delayed_put_task_struct(struct rcu_head *);
void rust_exit_delayed_put_callback(struct rcu_head *head)
{ rust_exit_delayed_put_task_struct(head); }
extern int rust_exit_child_wait_callback(wait_queue_entry_t *, unsigned int, int, void *);
int rust_exit_child_wait_callback_native(wait_queue_entry_t *wait, unsigned int mode, int sync, void *key)
{ return rust_exit_child_wait_callback(wait, mode, sync, key); }
#ifdef CONFIG_SYSCTL
extern int rust_exit_kernel_exit_sysctls_init(void);
late_initcall(rust_exit_kernel_exit_sysctls_init);
#endif
#ifdef CONFIG_SYSFS
extern ssize_t rust_exit_oops_count_show(struct kobject *, struct kobj_attribute *, char *);
ssize_t rust_exit_oops_count_show_native(struct kobject *kobj, struct kobj_attribute *attr, char *page)
{ return rust_exit_oops_count_show(kobj, attr, page); }
extern int rust_exit_kernel_exit_sysfs_init(void);
late_initcall(rust_exit_kernel_exit_sysfs_init);
#endif

/* One compiler exception-table transaction. No callback or Rust return occurs
 * with user access enabled. Rust supplies the selected ABI's checked extent,
 * six field addresses and six values. The order equals the original unsafe
 * put_user sequence and padding is never modified. */
int rust_exit_user_write_six(void __user *base, size_t size,
        unsigned int __user * const *fields, const unsigned int *values)
{
    if (!user_write_access_begin(base, size))
        return -EFAULT;
    unsafe_put_user(values[0], fields[0], Efault);
    unsafe_put_user(values[1], fields[1], Efault);
    unsafe_put_user(values[2], fields[2], Efault);
    unsafe_put_user(values[3], fields[3], Efault);
    unsafe_put_user(values[4], fields[4], Efault);
    unsafe_put_user(values[5], fields[5], Efault);
    user_write_access_end();
    return 0;
Efault:
    user_write_access_end();
    return -EFAULT;
}

extern void __noreturn rust_exit_sys_exit(int);
extern void __noreturn rust_exit_sys_exit_group(int);
extern long rust_exit_sys_waitid(int, pid_t, struct siginfo __user *, int, struct rusage __user *);
extern long rust_exit_sys_wait4(pid_t, int __user *, int, struct rusage __user *);
SYSCALL_DEFINE1(exit, int, error_code) { rust_exit_sys_exit(error_code); }
SYSCALL_DEFINE1(exit_group, int, error_code) { rust_exit_sys_exit_group(error_code); }
SYSCALL_DEFINE5(waitid, int, which, pid_t, upid, struct siginfo __user *, infop,
               int, options, struct rusage __user *, ru)
{ return rust_exit_sys_waitid(which, upid, infop, options, ru); }
SYSCALL_DEFINE4(wait4, pid_t, upid, int __user *, stat_addr,
               int, options, struct rusage __user *, ru)
{ return rust_exit_sys_wait4(upid, stat_addr, options, ru); }
#ifdef __ARCH_WANT_SYS_WAITPID
extern long rust_exit_sys_waitpid(pid_t, int __user *, int);
SYSCALL_DEFINE3(waitpid, pid_t, pid, int __user *, stat_addr, int, options)
{ return rust_exit_sys_waitpid(pid, stat_addr, options); }
#endif
#ifdef CONFIG_COMPAT
extern long rust_exit_compat_wait4(compat_pid_t, compat_uint_t __user *, int, struct compat_rusage __user *);
extern long rust_exit_compat_waitid(int, compat_pid_t, struct compat_siginfo __user *, int, struct compat_rusage __user *);
COMPAT_SYSCALL_DEFINE4(wait4, compat_pid_t, pid, compat_uint_t __user *, stat_addr,
                      int, options, struct compat_rusage __user *, ru)
{ return rust_exit_compat_wait4(pid, stat_addr, options, ru); }
COMPAT_SYSCALL_DEFINE5(waitid, int, which, compat_pid_t, pid,
                      struct compat_siginfo __user *, infop, int, options,
                      struct compat_rusage __user *, uru)
{ return rust_exit_compat_waitid(which, pid, infop, options, uru); }
#endif
EXPORT_SYMBOL_GPL(rcuwait_wake_up);
EXPORT_SYMBOL(do_exit);
extern void __noreturn rust_exit_abort(void);
/* Native weak + __function_aligned entry; the complete BUG/panic body is Rust. */
__weak __function_aligned void abort(void) { rust_exit_abort(); }
EXPORT_SYMBOL(abort);
