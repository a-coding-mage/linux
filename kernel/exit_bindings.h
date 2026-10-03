/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_EXIT_BINDINGS_H
#define RUST_EXIT_BINDINGS_H
#include "exit_includes.h"
#include <linux/sysctl.h>
#include <linux/sem.h>
#include <linux/page_ref.h>
#include <linux/err.h>
#include "exit_layout.h"

/* Genuine C layout and config metadata; no runtime storage is defined here. */
static const unsigned long RUST_EXIT_THREAD_SIZE = THREAD_SIZE;
static const unsigned int RUST_EXIT_PGTY_SLAB = PGTY_slab;
static const unsigned int RUST_EXIT_PGTY_LARGE_KMALLOC = PGTY_large_kmalloc;
/* Force Clang to publish the configured address as an integer enumerator.
 * A pointer-derived static-const expression becomes an unresolved bindgen static. */
enum { RUST_EXIT_LIST_POISON2 = (unsigned long)LIST_POISON2 };
static_assert((unsigned long)RUST_EXIT_LIST_POISON2 == (unsigned long)LIST_POISON2);
static const unsigned long RUST_EXIT_SEND_SIG_PRIV = (unsigned long)SEND_SIG_PRIV;
static const unsigned int RUST_EXIT_PT_EVENT_FLAG_BASE = PT_EVENT_FLAG(0);
static const int RUST_EXIT_TIF_NOTIFY_SIGNAL = TIF_NOTIFY_SIGNAL;
static const int RUST_EXIT_TIF_SIGPENDING = TIF_SIGPENDING;
static const size_t RUST_EXIT_RAW_LOCK_SIZE = sizeof(raw_spinlock_t);
static const size_t RUST_EXIT_RAW_LOCK_ALIGN = __alignof__(raw_spinlock_t);
static const size_t RUST_EXIT_SPIN_LOCK_SIZE = sizeof(spinlock_t);
static const size_t RUST_EXIT_SPIN_LOCK_ALIGN = __alignof__(spinlock_t);
#ifndef CONFIG_PREEMPT_RT
static const size_t RUST_EXIT_SPIN_RAW_OFFSET = offsetof(spinlock_t, rlock);
#endif
static const unsigned long RUST_EXIT_SPIN_OWNER = (unsigned long)SPINLOCK_OWNER_INIT;
static const unsigned int RUST_EXIT_SPIN_OWNER_CPU = -1;
static const unsigned int RUST_EXIT_LD_WAIT_SPIN = LD_WAIT_SPIN;
static const unsigned int RUST_EXIT_LD_WAIT_CONFIG = LD_WAIT_CONFIG;
#if !defined(CONFIG_SMP) && defined(CONFIG_DEBUG_SPINLOCK)
static const unsigned int RUST_EXIT_UP_UNLOCKED = __ARCH_SPIN_LOCK_UNLOCKED;
#endif
#if defined(CONFIG_DEBUG_STACK_USAGE) && defined(CONFIG_VM_EVENT_COUNTERS)
static const unsigned int RUST_EXIT_KSTACK_FIRST = KSTACK_1K;
#endif
static const size_t RUST_EXIT_SI_SIGNO = offsetof(struct siginfo, si_signo);
static const size_t RUST_EXIT_SI_ERRNO = offsetof(struct siginfo, si_errno);
static const size_t RUST_EXIT_SI_CODE = offsetof(struct siginfo, si_code);
static const size_t RUST_EXIT_SI_PID = offsetof(struct siginfo, si_pid);
static const size_t RUST_EXIT_SI_UID = offsetof(struct siginfo, si_uid);
static const size_t RUST_EXIT_SI_STATUS = offsetof(struct siginfo, si_status);
#ifdef CONFIG_COMPAT
static const size_t RUST_EXIT_COMPAT_SIGINFO_SIZE = sizeof(struct compat_siginfo);
static const size_t RUST_EXIT_COMPAT_SI_SIGNO = offsetof(struct compat_siginfo, si_signo);
static const size_t RUST_EXIT_COMPAT_SI_ERRNO = offsetof(struct compat_siginfo, si_errno);
static const size_t RUST_EXIT_COMPAT_SI_CODE = offsetof(struct compat_siginfo, si_code);
static const size_t RUST_EXIT_COMPAT_SI_PID = offsetof(struct compat_siginfo, si_pid);
static const size_t RUST_EXIT_COMPAT_SI_UID = offsetof(struct compat_siginfo, si_uid);
static const size_t RUST_EXIT_COMPAT_SI_STATUS = offsetof(struct compat_siginfo, si_status);
static_assert(sizeof(compat_pid_t) == sizeof(unsigned int));
static_assert(sizeof_field(struct compat_siginfo, si_uid) == sizeof(unsigned int));
#else
/* The shared store routine is also built without a compat syscall entry. */
static const size_t RUST_EXIT_COMPAT_SIGINFO_SIZE = 0;
static const size_t RUST_EXIT_COMPAT_SI_SIGNO = 0;
static const size_t RUST_EXIT_COMPAT_SI_ERRNO = 0;
static const size_t RUST_EXIT_COMPAT_SI_CODE = 0;
static const size_t RUST_EXIT_COMPAT_SI_PID = 0;
static const size_t RUST_EXIT_COMPAT_SI_UID = 0;
static const size_t RUST_EXIT_COMPAT_SI_STATUS = 0;
#endif
static_assert(sizeof_field(struct siginfo, si_signo) == sizeof(unsigned int));
static_assert(sizeof_field(struct siginfo, si_errno) == sizeof(unsigned int));
static_assert(sizeof_field(struct siginfo, si_code) == sizeof(unsigned int));
static_assert(sizeof_field(struct siginfo, si_pid) == sizeof(unsigned int));
static_assert(sizeof_field(struct siginfo, si_uid) == sizeof(unsigned int));
static_assert(sizeof_field(struct siginfo, si_status) == sizeof(unsigned int));

#define RX(result, name, args, ...) result rust_exit_##name args;
#include "exit_primitives.inc"
#undef RX
/* ABI callback shims contain only a call to the corresponding Rust owner. */
void rust_exit_delayed_put_callback(struct rcu_head *head);
int rust_exit_child_wait_callback_native(wait_queue_entry_t *wait, unsigned int mode, int sync, void *key);
#ifdef CONFIG_SYSFS
ssize_t rust_exit_oops_count_show_native(struct kobject *, struct kobj_attribute *, char *);
#endif
int rust_exit_user_write_six(void __user *base, size_t size,
        unsigned int __user * const *fields, const unsigned int *values);
#endif
