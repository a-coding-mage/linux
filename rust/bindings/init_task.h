/* SPDX-License-Identifier: GPL-2.0-only */
/* Declaration/layout input only: no C-owned initial task objects or bodies. */
#ifndef _RUST_BINDINGS_INIT_TASK_H
#define _RUST_BINDINGS_INIT_TASK_H
#pragma push_macro("MODULE")
#undef MODULE
#include <linux/hrtimer_types.h>
#include <linux/init_task.h>
#include <linux/mqueue.h>
#include <linux/sched.h>
#include <linux/sched/sysctl.h>
#include <linux/sched/rt.h>
#include <linux/sched/task.h>
#include <linux/sched/ext.h>
#include <linux/sched/exec_state.h>
#include <linux/user_namespace.h>
#include <linux/init.h>
#include <linux/fs.h>
#include <linux/mm.h>
#include <linux/audit.h>
#include <linux/numa.h>
#include <linux/scs.h>
#include <linux/plist.h>
#include <linux/uaccess.h>

/* Export expressions bindgen cannot discover as object-like integer macros. */
enum { RUST_INIT_TASK_RLIM_INFINITY = RLIM_INFINITY };
static const unsigned long RUST_INIT_TASK_STACK_LIMIT = _STK_LIM;
static const unsigned long RUST_INIT_TASK_MLOCK_LIMIT = MLOCK_LIMIT;
static const unsigned long RUST_INIT_TASK_CAP_VALID_MASK = CAP_VALID_MASK;
static const unsigned long RUST_INIT_TASK_INITIAL_JIFFIES = INITIAL_JIFFIES;
enum { RUST_INIT_TASK_LIST_POISON1 = (unsigned long)LIST_POISON1 };
#ifdef CONFIG_LOCKDEP
enum { RUST_INIT_TASK_INITIAL_CHAIN_KEY = (unsigned long)INITIAL_CHAIN_KEY };
#endif
#ifdef CONFIG_AUDIT
enum { RUST_INIT_TASK_AUDIT_SID_UNSET = AUDIT_SID_UNSET };
#endif
#ifdef CONFIG_LIVEPATCH
enum { RUST_INIT_TASK_KLP_TRANSITION_IDLE = KLP_TRANSITION_IDLE };
#endif
enum {
	RUST_INIT_TASK_PIDTYPE_PID = PIDTYPE_PID,
	RUST_INIT_TASK_PIDTYPE_TGID = PIDTYPE_TGID,
	RUST_INIT_TASK_PIDTYPE_PGID = PIDTYPE_PGID,
	RUST_INIT_TASK_PIDTYPE_SID = PIDTYPE_SID,
	RUST_INIT_TASK_LD_WAIT_SPIN = LD_WAIT_SPIN,
	RUST_INIT_TASK_LD_WAIT_CONFIG = LD_WAIT_CONFIG,
	RUST_INIT_TASK_LD_WAIT_SLEEP = LD_WAIT_SLEEP,
	RUST_INIT_TASK_L1_CACHE_BYTES = L1_CACHE_BYTES,
	RUST_INIT_TASK_NR_CPUS = NR_CPUS,
	RUST_INIT_TASK_RR_TIMESLICE = RR_TIMESLICE,
	RUST_INIT_TASK_SCHED_CAPACITY_SCALE = SCHED_CAPACITY_SCALE,
	RUST_INIT_TASK_SIZE = sizeof(struct task_struct),
	RUST_INIT_TASK_ALIGN = __alignof__(struct task_struct),
	RUST_INIT_TASK_THREAD_OFFSET = offsetof(struct task_struct, thread),
	RUST_INIT_TASK_THREAD_SIZE = sizeof(struct thread_struct),
	RUST_INIT_TASK_THREAD_ALIGN = __alignof__(struct thread_struct),
	RUST_INIT_TASK_STACK_OFFSET = offsetof(struct task_struct, stack),
	RUST_INIT_TASK_RESTART_OFFSET = offsetof(struct task_struct, restart_block),
	RUST_INIT_TASK_RESTART_FN_OFFSET = offsetof(struct restart_block, fn),
	RUST_INIT_TASK_RESTART_FN_SIZE = sizeof(((struct restart_block *)0)->fn),
	RUST_INIT_TASK_SE_OFFSET = offsetof(struct task_struct, se),
	RUST_INIT_TASK_RT_OFFSET = offsetof(struct task_struct, rt),
	RUST_INIT_TASK_TASKS_OFFSET = offsetof(struct task_struct, tasks),
	RUST_INIT_TASK_THREAD_NODE_OFFSET = offsetof(struct task_struct, thread_node),
	RUST_INIT_TASK_ALLOC_LOCK_OFFSET = offsetof(struct task_struct, alloc_lock),
	RUST_INIT_TASK_PI_LOCK_OFFSET = offsetof(struct task_struct, pi_lock),
	RUST_INIT_TASK_SIGNAL_SIZE = sizeof(struct signal_struct),
	RUST_INIT_TASK_SIGNAL_ALIGN = __alignof__(struct signal_struct),
	RUST_INIT_TASK_SIGHAND_SIZE = sizeof(struct sighand_struct),
	RUST_INIT_TASK_SIGHAND_ALIGN = __alignof__(struct sighand_struct),
	RUST_INIT_TASK_CRED_SIZE = sizeof(struct cred),
	RUST_INIT_TASK_CRED_ALIGN = __alignof__(struct cred),
	RUST_INIT_TASK_GROUP_SIZE = sizeof(struct group_info),
	RUST_INIT_TASK_GROUP_ALIGN = __alignof__(struct group_info),
	RUST_INIT_TASK_EXEC_SIZE = sizeof(struct task_exec_state),
	RUST_INIT_TASK_EXEC_ALIGN = __alignof__(struct task_exec_state),
	RUST_INIT_TASK_INFO_SIZE = sizeof(struct thread_info),
	RUST_INIT_TASK_INFO_ALIGN = __alignof__(struct thread_info),
	RUST_INIT_TASK_RAW_LOCK_SIZE = sizeof(raw_spinlock_t),
	RUST_INIT_TASK_RAW_LOCK_ALIGN = __alignof__(raw_spinlock_t),
	RUST_INIT_TASK_SPIN_LOCK_SIZE = sizeof(spinlock_t),
	RUST_INIT_TASK_SPIN_LOCK_ALIGN = __alignof__(spinlock_t),
	RUST_INIT_TASK_MUTEX_SIZE = sizeof(struct mutex),
	RUST_INIT_TASK_MUTEX_ALIGN = __alignof__(struct mutex),
	RUST_INIT_TASK_RWSEM_SIZE = sizeof(struct rw_semaphore),
	RUST_INIT_TASK_RWSEM_ALIGN = __alignof__(struct rw_semaphore),
};
#ifndef CONFIG_PREEMPT_RT
enum { RUST_INIT_TASK_SPIN_RAW_OFFSET = offsetof(spinlock_t, rlock) };
#endif
#ifdef CONFIG_SCHED_MM_CID
static const unsigned int RUST_INIT_TASK_MM_CID_UNSET = MM_CID_UNSET;
#endif
#ifdef CONFIG_SCHED_CLASS_EXT
static const unsigned long long RUST_INIT_TASK_SCX_DSQ_INVALID = SCX_DSQ_INVALID;
enum { RUST_INIT_TASK_SCX_SLICE_DFL = SCX_SLICE_DFL };
#endif
#ifdef CONFIG_X86_64
/* The Rust binding's sp is pointer-valued so a link-time relocation is legal. */
enum {
	RUST_INIT_TASK_SP_OFFSET = offsetof(struct thread_struct, sp),
	RUST_INIT_TASK_SP_SIZE = sizeof(((struct thread_struct *)0)->sp),
	RUST_INIT_TASK_SP_ALIGN = __alignof__(((struct thread_struct *)0)->sp),
};
#endif
#ifdef CONFIG_ARM64
static const unsigned long RUST_INIT_TASK_THREAD_FLAGS = _TIF_FOREIGN_FPSTATE;
static const unsigned long RUST_INIT_TASK_PREEMPT_COUNT = INIT_PREEMPT_COUNT;
#endif
#ifdef CONFIG_SHADOW_CALL_STACK
enum { RUST_INIT_TASK_SCS_SIZE = SCS_SIZE };
enum { RUST_INIT_TASK_SCS_END_MAGIC = SCS_END_MAGIC };
#endif
#pragma pop_macro("MODULE")
#endif
