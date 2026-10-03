// SPDX-License-Identifier: GPL-2.0-only
/* Native macro/registration boundary. No fork.c algorithm is retained here. */
#include "fork_includes.h"
#define CREATE_TRACE_POINTS
#include <trace/events/task.h>
#include "fork_header_primitives.h"
#include "fork_task_primitives.h"
#include "fork_mm_primitives.h"

#if !defined(CONFIG_VMAP_STACK) && THREAD_SIZE < PAGE_SIZE
extern void rust_fork_thread_stack_cache_init(void);
void thread_stack_cache_init(void) { rust_fork_thread_stack_cache_init(); }
#endif

extern long rust_fork_set_tid_address(int __user *);
extern long rust_fork_sys_fork(void);
extern long rust_fork_sys_vfork(void);
extern long rust_fork_sys_clone(unsigned long, unsigned long,
                              int __user *, int __user *, unsigned long);
extern long rust_fork_sys_clone3(struct clone_args __user *, size_t);
extern long rust_fork_sys_unshare(unsigned long);

SYSCALL_DEFINE1(set_tid_address, int __user *, tidptr)
{ return rust_fork_set_tid_address(tidptr); }
#ifdef __ARCH_WANT_SYS_FORK
SYSCALL_DEFINE0(fork) { return rust_fork_sys_fork(); }
#endif
#ifdef __ARCH_WANT_SYS_VFORK
SYSCALL_DEFINE0(vfork) { return rust_fork_sys_vfork(); }
#endif
#ifdef __ARCH_WANT_SYS_CLONE
#ifdef CONFIG_CLONE_BACKWARDS
SYSCALL_DEFINE5(clone, unsigned long, clone_flags, unsigned long, newsp,
               int __user *, parent_tidptr, unsigned long, tls, int __user *, child_tidptr)
#elif defined(CONFIG_CLONE_BACKWARDS2)
SYSCALL_DEFINE5(clone, unsigned long, newsp, unsigned long, clone_flags,
               int __user *, parent_tidptr, int __user *, child_tidptr, unsigned long, tls)
#elif defined(CONFIG_CLONE_BACKWARDS3)
SYSCALL_DEFINE6(clone, unsigned long, clone_flags, unsigned long, newsp,
               int, stack_size, int __user *, parent_tidptr,
               int __user *, child_tidptr, unsigned long, tls)
#else
SYSCALL_DEFINE5(clone, unsigned long, clone_flags, unsigned long, newsp,
               int __user *, parent_tidptr, int __user *, child_tidptr, unsigned long, tls)
#endif
{ return rust_fork_sys_clone(clone_flags, newsp, parent_tidptr, child_tidptr, tls); }
#endif
SYSCALL_DEFINE2(clone3, struct clone_args __user *, uargs, size_t, size)
{ return rust_fork_sys_clone3(uargs, size); }
SYSCALL_DEFINE1(unshare, unsigned long, unshare_flags)
{ return rust_fork_sys_unshare(unshare_flags); }

#ifdef CONFIG_PROVE_RCU
EXPORT_SYMBOL_GPL(lockdep_tasklist_lock_is_held);
#endif
EXPORT_SYMBOL(free_task);
EXPORT_SYMBOL_GPL(__mmdrop);
EXPORT_SYMBOL_GPL(__put_task_struct);
EXPORT_SYMBOL_GPL(__put_task_struct_rcu_cb);
EXPORT_SYMBOL_IF_KUNIT(mm_alloc);
EXPORT_SYMBOL_GPL(mmput);
#if defined(CONFIG_MMU) || defined(CONFIG_FUTEX_PRIVATE_HASH)
EXPORT_SYMBOL_GPL(mmput_async);
#endif
EXPORT_SYMBOL_GPL(get_task_mm);

/* The descriptor and initialization algorithm are Rust-owned. */
extern int rust_fork_init_sysctl(void);
subsys_initcall(rust_fork_init_sysctl);
