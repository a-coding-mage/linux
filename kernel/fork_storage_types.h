/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_FORK_STORAGE_TYPES_H
#define RUST_FORK_STORAGE_TYPES_H
#include <linux/idr.h>
/* Type/constant metadata only. No fork-owned runtime storage is defined here. */
struct rust_fork_tasklist_storage { rwlock_t value; } __aligned(SMP_CACHE_BYTES);
#if defined(CONFIG_SMP) && defined(CONFIG_X86_VSMP)
struct rust_fork_mmlist_storage { spinlock_t value; } __aligned(PAGE_SIZE);
#elif defined(CONFIG_SMP)
struct rust_fork_mmlist_storage { spinlock_t value; } __aligned(SMP_CACHE_BYTES);
#else
struct rust_fork_mmlist_storage { spinlock_t value; };
#endif
enum {
 RUST_FORK_RWLOCK_SIZE = sizeof(rwlock_t),
 RUST_FORK_RAW_LOCK_SIZE = sizeof(raw_spinlock_t),
 RUST_FORK_RAW_LOCK_ALIGN = __alignof__(raw_spinlock_t),
 RUST_FORK_SPIN_LOCK_SIZE = sizeof(spinlock_t),
 RUST_FORK_SPIN_LOCK_ALIGN = __alignof__(spinlock_t),
 RUST_FORK_TASKLIST_OFFSET = offsetof(struct rust_fork_tasklist_storage, value),
 RUST_FORK_MMLIST_OFFSET = offsetof(struct rust_fork_mmlist_storage, value),
 RUST_FORK_MMF_DUMP_FILTER_DEFAULT = MMF_DUMP_FILTER_DEFAULT,
};
#ifndef CONFIG_PREEMPT_RT
enum { RUST_FORK_SPIN_RAW_OFFSET = offsetof(spinlock_t, rlock) };
#endif
#ifdef CONFIG_DEBUG_LOCK_ALLOC
enum { RUST_FORK_LD_WAIT_SPIN = LD_WAIT_SPIN, RUST_FORK_LD_WAIT_CONFIG = LD_WAIT_CONFIG };
#endif
#ifdef CONFIG_PREEMPT_RT
enum { RUST_FORK_READER_BIAS = READER_BIAS };
#endif
#ifdef CONFIG_MM_ID
enum { RUST_FORK_IDA_INIT_FLAGS = IDA_INIT_FLAGS };
#endif
#endif
