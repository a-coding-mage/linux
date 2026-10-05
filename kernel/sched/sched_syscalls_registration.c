// SPDX-License-Identifier: GPL-2.0-only
/* Native syscall metadata and symbol-export shell; algorithms remain in Rust.
 * This source proposal has no admission to the build. Keep it hard-blocked.
 */
#error "Lupos scheduler syscalls are SOURCE ONLY; native/ABI/protection review required"
#include "sched_syscalls_bindings.h"

long lupos_syscalls_sys_nice(int increment);
long lupos_syscalls_sys_sched_setscheduler(pid_t pid, int policy,
					 struct sched_param __user *param);
long lupos_syscalls_sys_sched_setparam(pid_t pid, struct sched_param __user *param);
long lupos_syscalls_sys_sched_setattr(pid_t pid, struct sched_attr __user *attr,
				    unsigned int flags);
long lupos_syscalls_sys_sched_getscheduler(pid_t pid);
long lupos_syscalls_sys_sched_getparam(pid_t pid, struct sched_param __user *param);
long lupos_syscalls_sys_sched_getattr(pid_t pid, struct sched_attr __user *attr,
				    unsigned int size, unsigned int flags);
long lupos_syscalls_sys_sched_setaffinity(pid_t pid, unsigned int len,
					unsigned long __user *mask);
long lupos_syscalls_sys_sched_getaffinity(pid_t pid, unsigned int len,
					unsigned long __user *mask);
long lupos_syscalls_sys_sched_yield(void);
long lupos_syscalls_sys_sched_get_priority_max(int policy);
long lupos_syscalls_sys_sched_get_priority_min(int policy);
long lupos_syscalls_sys_sched_rr_get_interval(pid_t pid,
					    struct __kernel_timespec __user *interval);
#ifdef CONFIG_COMPAT_32BIT_TIME
long lupos_syscalls_sys_sched_rr_get_interval_time32(pid_t pid,
						   struct old_timespec32 __user *interval);
#endif

#ifdef __ARCH_WANT_SYS_NICE
SYSCALL_DEFINE1(nice, int, increment)
{
	return lupos_syscalls_sys_nice(increment);
}
#endif
SYSCALL_DEFINE3(sched_setscheduler, pid_t, pid, int, policy,
	       struct sched_param __user *, param)
{
	return lupos_syscalls_sys_sched_setscheduler(pid, policy, param);
}
SYSCALL_DEFINE2(sched_setparam, pid_t, pid, struct sched_param __user *, param)
{
	return lupos_syscalls_sys_sched_setparam(pid, param);
}
SYSCALL_DEFINE3(sched_setattr, pid_t, pid, struct sched_attr __user *, uattr,
	       unsigned int, flags)
{
	return lupos_syscalls_sys_sched_setattr(pid, uattr, flags);
}
SYSCALL_DEFINE1(sched_getscheduler, pid_t, pid)
{
	return lupos_syscalls_sys_sched_getscheduler(pid);
}
SYSCALL_DEFINE2(sched_getparam, pid_t, pid, struct sched_param __user *, param)
{
	return lupos_syscalls_sys_sched_getparam(pid, param);
}
SYSCALL_DEFINE4(sched_getattr, pid_t, pid, struct sched_attr __user *, uattr,
	       unsigned int, usize, unsigned int, flags)
{
	return lupos_syscalls_sys_sched_getattr(pid, uattr, usize, flags);
}
SYSCALL_DEFINE3(sched_setaffinity, pid_t, pid, unsigned int, len,
	       unsigned long __user *, user_mask_ptr)
{
	return lupos_syscalls_sys_sched_setaffinity(pid, len, user_mask_ptr);
}
SYSCALL_DEFINE3(sched_getaffinity, pid_t, pid, unsigned int, len,
	       unsigned long __user *, user_mask_ptr)
{
	return lupos_syscalls_sys_sched_getaffinity(pid, len, user_mask_ptr);
}
SYSCALL_DEFINE0(sched_yield)
{
	return lupos_syscalls_sys_sched_yield();
}
SYSCALL_DEFINE1(sched_get_priority_max, int, policy)
{
	return lupos_syscalls_sys_sched_get_priority_max(policy);
}
SYSCALL_DEFINE1(sched_get_priority_min, int, policy)
{
	return lupos_syscalls_sys_sched_get_priority_min(policy);
}
SYSCALL_DEFINE2(sched_rr_get_interval, pid_t, pid,
	       struct __kernel_timespec __user *, interval)
{
	return lupos_syscalls_sys_sched_rr_get_interval(pid, interval);
}
#ifdef CONFIG_COMPAT_32BIT_TIME
SYSCALL_DEFINE2(sched_rr_get_interval_time32, pid_t, pid,
	       struct old_timespec32 __user *, interval)
{
	return lupos_syscalls_sys_sched_rr_get_interval_time32(pid, interval);
}
#endif

EXPORT_SYMBOL(set_user_nice);
EXPORT_SYMBOL_GPL(sched_setattr_nocheck);
EXPORT_SYMBOL_GPL(sched_set_fifo);
EXPORT_SYMBOL_GPL(sched_set_fifo_low);
EXPORT_SYMBOL_GPL(sched_set_normal);
EXPORT_SYMBOL(yield);
EXPORT_SYMBOL_GPL(yield_to);
