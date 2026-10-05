/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SCHED_EXT_CID_BINDINGS_H
#define LUPOS_SCHED_EXT_CID_BINDINGS_H

/* Configured native headers are the sole layout and constant authority. */
#include <linux/btf.h>
#include <linux/btf_ids.h>
#include <linux/cacheinfo.h>
#include <linux/rhashtable.h>
#include <linux/seq_buf.h>
#include <linux/slab.h>
#include "internal.h"
#include "cid.h"

#define LUPOS_SCX_CID_ENOMEM ENOMEM
#define LUPOS_SCX_CID_EINVAL EINVAL
#define LUPOS_SCX_CID_NUMA_NO_NODE NUMA_NO_NODE
#define LUPOS_SCX_CID_SHARD_SIZE_DFL SCX_CID_SHARD_SIZE_DFL
#define LUPOS_SCX_CID_SHARD_MAX_CPUS SCX_CID_SHARD_MAX_CPUS

/* Native storage, allocation, RCU and configured cpumask leaves. */
void *lupos_scx_cid_kzalloc(size_t size);
void *lupos_scx_cid_kvcalloc(size_t count, size_t size);
void *lupos_scx_cid_kcalloc(size_t count, size_t size);
void *lupos_scx_cid_kmemdup(const void *src, size_t size);
void lupos_scx_cid_kfree(const void *ptr);
void lupos_scx_cid_kvfree(const void *ptr);
struct scx_cid_tables *lupos_scx_cid_from_rcu(struct rcu_head *rcu);
void lupos_scx_cid_call_rcu(struct rcu_head *rcu);
void lupos_scx_cid_publish(struct scx_cid_tables *tbls);
void lupos_scx_cid_unpublish(void);
void lupos_scx_cid_assert_enable_held(void);
void lupos_scx_cid_assert_cpus_held(void);
void lupos_scx_cid_rcu_lock(void);
void lupos_scx_cid_rcu_unlock(void);
s16 *lupos_scx_cid_to_cpu_dereference(void);
s16 *lupos_scx_cpu_to_cid_dereference(void);
s32 *lupos_scx_cid_to_shard_dereference(void);
struct scx_cid_shard *lupos_scx_cid_ranges_dereference(void);
struct scx_cid_topo *lupos_scx_cid_topo_dereference(void);
struct scx_sched *lupos_scx_cid_prog_sched(const struct bpf_prog_aux *aux);
bool lupos_scx_cid_cpu_valid(struct scx_sched *sch, s32 cpu);
u32 lupos_scx_cid_shard_size(struct scx_sched *sch);
u32 lupos_scx_cid_nr_possible(void);
u32 lupos_scx_cid_nr_cpu_ids(void);
u32 lupos_scx_cid_nr_node_ids(void);
const struct cpumask *lupos_scx_cid_online_mask(void);
const struct cpumask *lupos_scx_cid_possible_mask(void);
const struct cpumask *lupos_scx_cid_node_mask(s32 node);
const struct cpumask *lupos_scx_cid_sibling_mask(s32 cpu);
const struct cpumask *lupos_scx_cid_cache_mask(s32 cpu);
s32 lupos_scx_cid_cpu_node(s32 cpu);
bool lupos_scx_cid_cpu_online(s32 cpu);
bool lupos_scx_cid_node_valid(s32 node);
u32 lupos_scx_cid_mask_scan(u32 offset, const struct cpumask *mask);
u32 lupos_scx_cid_mask_scan_limit(void);
u32 lupos_scx_cid_possible_cpu_scan(u32 offset);
u32 lupos_scx_cid_possible_cpu_limit(void);
u32 lupos_scx_cid_mask_first(const struct cpumask *mask);
u32 lupos_scx_cid_mask_weight(const struct cpumask *mask);
bool lupos_scx_cid_mask_empty(const struct cpumask *mask);
bool lupos_scx_cid_mask_test(s32 cpu, const struct cpumask *mask);
bool lupos_scx_cid_mask_test_set(s32 cpu, struct cpumask *mask);
void lupos_scx_cid_mask_set(s32 cpu, struct cpumask *mask);
void lupos_scx_cid_mask_clear(s32 cpu, struct cpumask *mask);
void lupos_scx_cid_mask_copy(struct cpumask *dst, const struct cpumask *src);
void lupos_scx_cid_mask_and(struct cpumask *dst, const struct cpumask *a,
			  const struct cpumask *b);
s32 lupos_scx_cid_with_build_masks(struct scx_cid_tables *tbls, u32 shard_size);
void lupos_scx_cid_with_seen(const s32 *cpu_to_cid, u32 cpu_count,
			   const s32 *shard_start, u32 shard_count,
			   const struct bpf_prog_aux *aux);
u64 *lupos_scx_cid_cmask_bits(struct scx_cmask *mask);
const u64 *lupos_scx_cid_cmask_bits_const(const struct scx_cmask *mask);
u32 lupos_scx_cid_cmask_nr_words(u32 nr_cids);
u32 lupos_scx_cid_read_u32(const u32 *ptr);
u64 lupos_scx_cid_read_u64(const u64 *ptr);
void lupos_scx_cid_write_u32(u32 *ptr, u32 value);
void lupos_scx_cid_write_u64(u64 *ptr, u64 value);
bool lupos_scx_cid_likely_valid(bool condition);
bool lupos_scx_cid_unlikely_override_no_sched(bool condition);
bool lupos_scx_cid_unlikely_to_cpu_no_sched(bool condition);
bool lupos_scx_cid_unlikely_to_cid_no_sched(bool condition);
bool lupos_scx_cid_unlikely_topo_no_sched(bool condition);
bool lupos_scx_cid_unlikely_to_cpu_no_table(bool condition);
bool lupos_scx_cid_unlikely_to_cid_no_table(bool condition);
bool lupos_scx_cid_unlikely_topo_no_table(bool condition);
bool lupos_scx_cid_unlikely_ref_invalid(bool condition);
bool lupos_scx_cid_likely_ref_init_nonempty(bool condition);
bool lupos_scx_cid_likely_ref_kern_nonempty(bool condition);

/* Each diagnostic callsite retains its own native WARN_ON_ONCE state. */
bool lupos_scx_cid_warn_node(bool condition);
bool lupos_scx_cid_warn_llc(bool condition);
bool lupos_scx_cid_warn_core(bool condition);
void lupos_scx_cid_warn_cache(const struct cpumask *mask);
void lupos_scx_cid_warn_notopo(const struct cpumask *mask);
void lupos_scx_cid_error_invalid(struct scx_sched *sch, s32 cid);
void lupos_scx_cid_error_cpu_count(struct scx_sched *sch, u32 expected, u32 got);
void lupos_scx_cid_error_shard_count(struct scx_sched *sch, u32 count);
void lupos_scx_cid_error_alloc(struct scx_sched *sch);
void lupos_scx_cid_error_start(struct scx_sched *sch, s32 value);
void lupos_scx_cid_error_order(struct scx_sched *sch, s32 index);
void lupos_scx_cid_error_bound(struct scx_sched *sch, s32 index, s32 value, u32 count);
void lupos_scx_cid_error_span(struct scx_sched *sch, s32 index, s32 span);
void lupos_scx_cid_error_duplicate(struct scx_sched *sch, s32 cid);
void lupos_scx_cid_error_capacity(struct scx_sched *sch, u32 capacity,
				u32 needed, s32 index);
int lupos_scx_cid_register_init(void);
int lupos_scx_cid_register_ops(void);
int lupos_scx_cid_register_tracing(void);
int lupos_scx_cid_register_syscall(void);

/* Rust continuations; no old cid.c or ext.c algorithm is called. */
void lupos_scx_cid_free_rcu(struct rcu_head *rcu);
s32 lupos_scx_cid_build(struct scx_cid_tables *tbls, u32 shard_size,
		      struct cpumask *to_walk, struct cpumask *node_scratch,
		      struct cpumask *llc_scratch, struct cpumask *core_scratch,
		      struct cpumask *llc_fallback, struct cpumask *online_no_topo);
void lupos_scx_cid_override_seen(const s32 *cpu_to_cid, u32 cpu_count,
			       const s32 *shard_start, u32 shard_count,
			       const struct bpf_prog_aux *aux, struct cpumask *seen,
			       bool allocated);
void lupos_scx_bpf_cid_override(const s32 *cpu_to_cid, u32 cpu_count,
			      const s32 *shard_start, u32 shard_count,
			      const struct bpf_prog_aux *aux);
s32 lupos_scx_bpf_cid_to_cpu(s32 cid, const struct bpf_prog_aux *aux);
s32 lupos_scx_bpf_cpu_to_cid(s32 cpu, const struct bpf_prog_aux *aux);
void lupos_scx_bpf_cid_topo(s32 cid, struct scx_cid_topo *out,
			  const struct bpf_prog_aux *aux);
#endif /* LUPOS_SCHED_EXT_CID_BINDINGS_H */
