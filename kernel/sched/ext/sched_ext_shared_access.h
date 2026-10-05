/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_SHARED_ACCESS_H
#define LUPOS_SCHED_EXT_SHARED_ACCESS_H

#include <linux/types.h>

struct sched_ext_entity;
struct task_struct;
struct sched_class;
struct scx_dump_data;
struct scx_dispatch_q;

/* Transitional native FIELD ACCESS boundary, not a synchronization API.
 * Each definition is exactly one original plain load, store or compound
 * assignment through its actual native member type. No atomics, volatile,
 * READ_ONCE/WRITE_ONCE, barriers, locks or lifetime acquisition are supplied.
 * No caller may infer whole-program race safety from crossing this boundary.
 * Native/BPF races, widths, alignment and compiler behavior remain held for
 * configuration-specific qualification. All algorithms stay with Rust owners.
 *
 * Every pointer is borrowed synchronously and must designate the original live
 * entity under the original caller lifetime/locking protocol. The entity form
 * also accepts the task iterator's actual embedded cursor, never a fake task.
 * Compound flags updates retain the separate original &= and |= operations;
 * slice subtraction remains native unsigned subtraction, not an atomic RMW.
 */
u32 scx_shared_flags_read(const struct sched_ext_entity *scx);
void scx_shared_flags_write(struct sched_ext_entity *scx, u32 value);
void scx_shared_flags_and(struct sched_ext_entity *scx, u32 mask);
void scx_shared_flags_or(struct sched_ext_entity *scx, u32 bits);
u64 scx_shared_slice_read(const struct sched_ext_entity *scx);
void scx_shared_slice_write(struct sched_ext_entity *scx, u64 value);
void scx_shared_slice_sub(struct sched_ext_entity *scx, u64 value);
u64 scx_shared_vtime_read(const struct sched_ext_entity *scx);
void scx_shared_vtime_write(struct sched_ext_entity *scx, u64 value);
const struct sched_class *scx_shared_class_read(const struct task_struct *p);
void scx_shared_class_write(struct task_struct *p, const struct sched_class *class);
s32 scx_shared_dump_cpu_read(const struct scx_dump_data *dd);
void scx_shared_dump_cpu_write(struct scx_dump_data *dd, s32 cpu);
u64 scx_shared_dsq_id_read(const struct scx_dispatch_q *dsq);
void scx_shared_dsq_id_write(struct scx_dispatch_q *dsq, u64 id);

#endif /* LUPOS_SCHED_EXT_SHARED_ACCESS_H */
