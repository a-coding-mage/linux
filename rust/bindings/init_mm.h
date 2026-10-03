/* SPDX-License-Identifier: GPL-2.0-only */
/* Native declarations and layout constants only; no C data provider or body. */
#ifndef _RUST_BINDINGS_INIT_MM_H
#define _RUST_BINDINGS_INIT_MM_H
#pragma push_macro("MODULE")
#undef MODULE
#include <linux/mm_types.h>
#include <linux/maple_tree.h>
#include <linux/rwsem.h>
#include <linux/spinlock.h>
#include <linux/list.h>
#include <linux/cpumask.h>
#include <linux/mman.h>
#include <linux/pgtable.h>
#include <linux/atomic.h>
#include <linux/user_namespace.h>
#include <linux/iommu.h>
#include <asm/mmu.h>

#define RUST_INIT_MM_LAYOUT(name, type) \
	RUST_INIT_MM_##name##_SIZE = sizeof(type), \
	RUST_INIT_MM_##name##_ALIGN = __alignof__(type)
#define RUST_INIT_MM_OFFSET(field) \
	RUST_INIT_MM_##field##_OFFSET = offsetof(struct mm_struct, field)
enum {
	RUST_INIT_MM_LAYOUT(BASE, struct mm_struct),
	RUST_INIT_MM_LAYOUT(VM_OPS, struct vm_operations_struct),
	RUST_INIT_MM_LAYOUT(RAW_LOCK, raw_spinlock_t),
	RUST_INIT_MM_LAYOUT(SPIN_LOCK, spinlock_t),
	RUST_INIT_MM_LAYOUT(RWSEM, struct rw_semaphore),
	RUST_INIT_MM_LAYOUT(MUTEX, struct mutex),
	RUST_INIT_MM_LAYOUT(SEQCOUNT, seqcount_t),
	RUST_INIT_MM_LAYOUT(MAPLE, struct maple_tree),
	RUST_INIT_MM_OFFSET(mm_count),
	RUST_INIT_MM_OFFSET(mm_mt),
	RUST_INIT_MM_OFFSET(pgd),
	RUST_INIT_MM_OFFSET(mm_users),
	RUST_INIT_MM_OFFSET(write_protect_seq),
	RUST_INIT_MM_OFFSET(mmap_lock),
	RUST_INIT_MM_OFFSET(page_table_lock),
	RUST_INIT_MM_OFFSET(arg_lock),
	RUST_INIT_MM_OFFSET(mmlist),
	RUST_INIT_MM_OFFSET(context),
	RUST_INIT_MM_OFFSET(start_code),
	RUST_INIT_MM_OFFSET(end_code),
	RUST_INIT_MM_OFFSET(end_data),
	RUST_INIT_MM_OFFSET(brk),
	RUST_INIT_MM_OFFSET(flexible_array),
#ifdef CONFIG_PER_VMA_LOCK
	RUST_INIT_MM_OFFSET(vma_writer_wait),
	RUST_INIT_MM_OFFSET(mm_lock_seq),
#endif
#ifdef CONFIG_SCHED_MM_CID
	RUST_INIT_MM_OFFSET(mm_cid),
#endif
	RUST_INIT_MM_FLEX_BYTES = sizeof(cpumask_t) + MM_CID_STATIC_SIZE,
	RUST_INIT_MM_MT_FLAGS = MM_MT_FLAGS,
	RUST_INIT_MM_LD_WAIT_SPIN = LD_WAIT_SPIN,
	RUST_INIT_MM_LD_WAIT_CONFIG = LD_WAIT_CONFIG,
	RUST_INIT_MM_LD_WAIT_SLEEP = LD_WAIT_SLEEP,
	RUST_INIT_MM_MAPLE_LOCK_OFFSET = offsetof(struct maple_tree, ma_lock),
#ifndef CONFIG_PREEMPT_RT
	RUST_INIT_MM_SPIN_RAW_OFFSET = offsetof(spinlock_t, rlock),
#endif
};
#undef RUST_INIT_MM_LAYOUT
#undef RUST_INIT_MM_OFFSET
#pragma pop_macro("MODULE")
#endif
