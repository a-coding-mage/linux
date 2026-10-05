// SPDX-License-Identifier: GPL-2.0-only
/*
 * CID-local native storage, configuration, primitive and BTF adapters.
 * This is an explicit, unqualified C runtime boundary. It is not Rust
 * coverage of these operations and does not include the old cid.c/ext.c.
 */
#error "SOURCE ONLY HOLD: sched_ext CID native ABI/protection admission is pending"

#include "sched_ext_cid_bindings.h"

/* These seven slots belong to CID, not shared ext state. */
u32 scx_nr_cid_shards;
s16 __rcu *scx_cid_to_cpu_tbl;
s16 __rcu *scx_cpu_to_cid_tbl;
s32 __rcu *scx_cid_to_shard;
s32 __rcu *scx_shard_node;
struct scx_cid_shard __rcu *scx_cid_shard_ranges;
struct scx_cid_topo __rcu *scx_cid_topo;

void *lupos_scx_cid_kzalloc(size_t size)
{
	return kzalloc(size, GFP_KERNEL);
}

void *lupos_scx_cid_kvcalloc(size_t count, size_t size)
{
	return kvcalloc(count, size, GFP_KERNEL);
}

void *lupos_scx_cid_kcalloc(size_t count, size_t size)
{
	return kcalloc(count, size, GFP_KERNEL);
}

void *lupos_scx_cid_kmemdup(const void *src, size_t size)
{
	return kmemdup(src, size, GFP_KERNEL);
}

void lupos_scx_cid_kfree(const void *ptr)
{
	kfree(ptr);
}

void lupos_scx_cid_kvfree(const void *ptr)
{
	kvfree(ptr);
}

struct scx_cid_tables *lupos_scx_cid_from_rcu(struct rcu_head *rcu)
{
	return container_of(rcu, struct scx_cid_tables, rcu);
}

/* The callback itself keeps the configured C callback ABI/CFI type. */
static void lupos_scx_cid_rcu_callback(struct rcu_head *rcu)
{
	lupos_scx_cid_free_rcu(rcu);
}

void lupos_scx_cid_call_rcu(struct rcu_head *rcu)
{
	call_rcu(rcu, lupos_scx_cid_rcu_callback);
}

void lupos_scx_cid_publish(struct scx_cid_tables *tbls)
{
	scx_nr_cid_shards = tbls->nr_shards;
	rcu_assign_pointer(scx_cid_to_cpu_tbl, tbls->cid_to_cpu);
	rcu_assign_pointer(scx_cpu_to_cid_tbl, tbls->cpu_to_cid);
	rcu_assign_pointer(scx_cid_to_shard, tbls->cid_to_shard);
	rcu_assign_pointer(scx_shard_node, tbls->shard_node);
	rcu_assign_pointer(scx_cid_shard_ranges, tbls->shard_ranges);
	rcu_assign_pointer(scx_cid_topo, tbls->topo);
}

void lupos_scx_cid_unpublish(void)
{
	RCU_INIT_POINTER(scx_cid_to_cpu_tbl, NULL);
	RCU_INIT_POINTER(scx_cpu_to_cid_tbl, NULL);
	RCU_INIT_POINTER(scx_cid_to_shard, NULL);
	RCU_INIT_POINTER(scx_shard_node, NULL);
	RCU_INIT_POINTER(scx_cid_shard_ranges, NULL);
	RCU_INIT_POINTER(scx_cid_topo, NULL);
}

void lupos_scx_cid_assert_enable_held(void)
{
	lockdep_assert_held(&scx_enable_mutex);
}

void lupos_scx_cid_assert_cpus_held(void)
{
	lockdep_assert_cpus_held();
}

void lupos_scx_cid_rcu_lock(void)
{
	rcu_read_lock();
}

void lupos_scx_cid_rcu_unlock(void)
{
	rcu_read_unlock();
}

s16 *lupos_scx_cid_to_cpu_dereference(void)
{
	return rcu_dereference_all(scx_cid_to_cpu_tbl);
}

s16 *lupos_scx_cpu_to_cid_dereference(void)
{
	return rcu_dereference_all(scx_cpu_to_cid_tbl);
}

s32 *lupos_scx_cid_to_shard_dereference(void)
{
	return rcu_dereference_all(scx_cid_to_shard);
}

struct scx_cid_shard *lupos_scx_cid_ranges_dereference(void)
{
	return rcu_dereference_all(scx_cid_shard_ranges);
}

struct scx_cid_topo *lupos_scx_cid_topo_dereference(void)
{
	return rcu_dereference(scx_cid_topo);
}

struct scx_sched *lupos_scx_cid_prog_sched(const struct bpf_prog_aux *aux)
{
	return scx_prog_sched(aux);
}

bool lupos_scx_cid_cpu_valid(struct scx_sched *sch, s32 cpu)
{
	return scx_cpu_valid(sch, cpu, NULL);
}

u32 lupos_scx_cid_shard_size(struct scx_sched *sch)
{
	return sch->ops.cid_shard_size;
}

u32 lupos_scx_cid_nr_possible(void) { return num_possible_cpus(); }
u32 lupos_scx_cid_nr_cpu_ids(void) { return nr_cpu_ids; }
u32 lupos_scx_cid_nr_node_ids(void) { return nr_node_ids; }
const struct cpumask *lupos_scx_cid_online_mask(void) { return cpu_online_mask; }
const struct cpumask *lupos_scx_cid_possible_mask(void) { return cpu_possible_mask; }
const struct cpumask *lupos_scx_cid_node_mask(s32 node) { return cpumask_of_node(node); }
const struct cpumask *lupos_scx_cid_sibling_mask(s32 cpu)
{
	return topology_sibling_cpumask(cpu);
}

/* Only the cacheinfo layout access; fallback policy stays in Rust. */
const struct cpumask *lupos_scx_cid_cache_mask(s32 cpu)
{
	struct cpu_cacheinfo *ci = get_cpu_cacheinfo(cpu);

	if (!ci || !ci->info_list || !ci->num_leaves)
		return NULL;
	return &ci->info_list[ci->num_leaves - 1].shared_cpu_map;
}

s32 lupos_scx_cid_cpu_node(s32 cpu) { return cpu_to_node(cpu); }
bool lupos_scx_cid_cpu_online(s32 cpu) { return cpu_online(cpu); }
bool lupos_scx_cid_node_valid(s32 node) { return numa_valid_node(node); }
/* for_each_cpu uses this scan, not cpumask_first/next. */
u32 lupos_scx_cid_mask_scan(u32 offset, const struct cpumask *mask)
{
	return find_next_bit(cpumask_bits(mask), small_cpumask_bits, offset);
}
u32 lupos_scx_cid_mask_scan_limit(void) { return small_cpumask_bits; }

/* Match for_each_possible_cpu's native uniprocessor specialization. */
u32 lupos_scx_cid_possible_cpu_scan(u32 offset)
{
#if NR_CPUS == 1
	return offset;
#else
	return find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits,
			     offset);
#endif
}
u32 lupos_scx_cid_possible_cpu_limit(void)
{
#if NR_CPUS == 1
	return 1;
#else
	return small_cpumask_bits;
#endif
}

/* Keep the oracle's standalone cpumask_first calls distinct from scans. */
u32 lupos_scx_cid_mask_first(const struct cpumask *mask) { return cpumask_first(mask); }
u32 lupos_scx_cid_mask_weight(const struct cpumask *mask) { return cpumask_weight(mask); }
bool lupos_scx_cid_mask_empty(const struct cpumask *mask) { return cpumask_empty(mask); }
bool lupos_scx_cid_mask_test(s32 cpu, const struct cpumask *mask)
{
	return cpumask_test_cpu(cpu, mask);
}
bool lupos_scx_cid_mask_test_set(s32 cpu, struct cpumask *mask)
{
	return cpumask_test_and_set_cpu(cpu, mask);
}
void lupos_scx_cid_mask_set(s32 cpu, struct cpumask *mask) { cpumask_set_cpu(cpu, mask); }
void lupos_scx_cid_mask_clear(s32 cpu, struct cpumask *mask) { cpumask_clear_cpu(cpu, mask); }
void lupos_scx_cid_mask_copy(struct cpumask *dst, const struct cpumask *src)
{
	cpumask_copy(dst, src);
}
void lupos_scx_cid_mask_and(struct cpumask *dst, const struct cpumask *a,
			  const struct cpumask *b)
{
	cpumask_and(dst, a, b);
}

/*
 * cpumask_var_t may be an array or pointer. Native automatic storage retains
 * its configured form and cleanup; Rust may borrow these raw masks only for
 * this synchronous callback. Allocation order matches the pinned C owner.
 */
s32 lupos_scx_cid_with_build_masks(struct scx_cid_tables *tbls, u32 shard_size)
{
	cpumask_var_t to_walk __free(free_cpumask_var) = CPUMASK_VAR_NULL;
	cpumask_var_t node_scratch __free(free_cpumask_var) = CPUMASK_VAR_NULL;
	cpumask_var_t llc_scratch __free(free_cpumask_var) = CPUMASK_VAR_NULL;
	cpumask_var_t core_scratch __free(free_cpumask_var) = CPUMASK_VAR_NULL;
	cpumask_var_t llc_fallback __free(free_cpumask_var) = CPUMASK_VAR_NULL;
	cpumask_var_t online_no_topo __free(free_cpumask_var) = CPUMASK_VAR_NULL;

	BUILD_BUG_ON(NR_CPUS > 8192);
	if (!zalloc_cpumask_var(&to_walk, GFP_KERNEL) ||
	    !zalloc_cpumask_var(&node_scratch, GFP_KERNEL) ||
	    !zalloc_cpumask_var(&llc_scratch, GFP_KERNEL) ||
	    !zalloc_cpumask_var(&core_scratch, GFP_KERNEL) ||
	    !zalloc_cpumask_var(&llc_fallback, GFP_KERNEL) ||
	    !zalloc_cpumask_var(&online_no_topo, GFP_KERNEL))
		return -ENOMEM;
	return lupos_scx_cid_build(tbls, shard_size, to_walk, node_scratch,
				   llc_scratch, core_scratch, llc_fallback,
				   online_no_topo);
}

void lupos_scx_cid_with_seen(const s32 *cpu_to_cid, u32 cpu_count,
			   const s32 *shard_start, u32 shard_count,
			   const struct bpf_prog_aux *aux)
{
	cpumask_var_t seen __free(free_cpumask_var) = CPUMASK_VAR_NULL;
	bool allocated = zalloc_cpumask_var(&seen, GFP_KERNEL);

	lupos_scx_cid_override_seen(cpu_to_cid, cpu_count, shard_start,
				    shard_count, aux, seen, allocated);
}

u64 *lupos_scx_cid_cmask_bits(struct scx_cmask *mask) { return mask->bits; }
const u64 *lupos_scx_cid_cmask_bits_const(const struct scx_cmask *mask)
{
	return mask->bits;
}
u32 lupos_scx_cid_cmask_nr_words(u32 nr_cids) { return SCX_CMASK_NR_WORDS(nr_cids); }
u32 lupos_scx_cid_read_u32(const u32 *ptr) { return READ_ONCE(*ptr); }
u64 lupos_scx_cid_read_u64(const u64 *ptr) { return READ_ONCE(*ptr); }
void lupos_scx_cid_write_u32(u32 *ptr, u32 value) { WRITE_ONCE(*ptr, value); }
void lupos_scx_cid_write_u64(u64 *ptr, u64 value) { WRITE_ONCE(*ptr, value); }

bool lupos_scx_cid_likely_valid(bool condition) { return likely(condition); }
bool lupos_scx_cid_unlikely_override_no_sched(bool condition) { return unlikely(condition); }
bool lupos_scx_cid_unlikely_to_cpu_no_sched(bool condition) { return unlikely(condition); }
bool lupos_scx_cid_unlikely_to_cid_no_sched(bool condition) { return unlikely(condition); }
bool lupos_scx_cid_unlikely_topo_no_sched(bool condition) { return unlikely(condition); }
bool lupos_scx_cid_unlikely_to_cpu_no_table(bool condition) { return unlikely(condition); }
bool lupos_scx_cid_unlikely_to_cid_no_table(bool condition) { return unlikely(condition); }
bool lupos_scx_cid_unlikely_topo_no_table(bool condition) { return unlikely(condition); }
bool lupos_scx_cid_unlikely_ref_invalid(bool condition) { return unlikely(condition); }
bool lupos_scx_cid_likely_ref_init_nonempty(bool condition) { return likely(condition); }
bool lupos_scx_cid_likely_ref_kern_nonempty(bool condition) { return likely(condition); }

bool lupos_scx_cid_warn_node(bool condition) { return WARN_ON_ONCE(condition); }
bool lupos_scx_cid_warn_llc(bool condition) { return WARN_ON_ONCE(condition); }
bool lupos_scx_cid_warn_core(bool condition) { return WARN_ON_ONCE(condition); }
void lupos_scx_cid_warn_cache(const struct cpumask *mask)
{
	pr_warn("scx_cid: cpus without cacheinfo, using node mask as llc: %*pbl\n",
		cpumask_pr_args(mask));
}
void lupos_scx_cid_warn_notopo(const struct cpumask *mask)
{
	pr_warn("scx_cid: online cpus with no usable topology: %*pbl\n",
		cpumask_pr_args(mask));
}
void lupos_scx_cid_error_invalid(struct scx_sched *sch, s32 cid)
{
	scx_error(sch, "invalid cid %d", cid);
}
void lupos_scx_cid_error_cpu_count(struct scx_sched *sch, u32 expected, u32 got)
{
	scx_error(sch, "scx_bpf_cid_override: cpu_to_cid expected %u entries, got %u",
		  expected, got);
}
void lupos_scx_cid_error_shard_count(struct scx_sched *sch, u32 count)
{
	scx_error(sch, "scx_bpf_cid_override: invalid shard_start count %u", count);
}
void lupos_scx_cid_error_alloc(struct scx_sched *sch)
{
	scx_error(sch, "scx_bpf_cid_override: allocation failed");
}
void lupos_scx_cid_error_start(struct scx_sched *sch, s32 value)
{
	scx_error(sch, "scx_bpf_cid_override: shard_start[0] must be 0, got %d", value);
}
void lupos_scx_cid_error_order(struct scx_sched *sch, s32 index)
{
	scx_error(sch, "scx_bpf_cid_override: shard_start not increasing at [%d]", index);
}
void lupos_scx_cid_error_bound(struct scx_sched *sch, s32 index, s32 value, u32 count)
{
	scx_error(sch, "scx_bpf_cid_override: shard_start[%d]=%d >= %u", index, value, count);
}
void lupos_scx_cid_error_span(struct scx_sched *sch, s32 index, s32 span)
{
	scx_error(sch, "scx_bpf_cid_override: shard[%d] span %d exceeds max %d",
		  index, span, SCX_CID_SHARD_MAX_CPUS);
}
void lupos_scx_cid_error_duplicate(struct scx_sched *sch, s32 cid)
{
	scx_error(sch, "cid %d assigned to multiple cpus", cid);
}
void lupos_scx_cid_error_capacity(struct scx_sched *sch, u32 capacity,
				u32 needed, s32 index)
{
	scx_error(sch, "scx_cmask_ref_shard: out alloc_words=%u < %u for shard %d",
		  capacity, needed, index);
}

/* BTF requires native functions and exact public argument annotations. */
__bpf_kfunc_start_defs();
__bpf_kfunc void scx_bpf_cid_override(const s32 *cpu_to_cid__arena, u32 cpu_to_cid_cnt,
				    const s32 *shard_start__arena, u32 shard_start_cnt,
				    const struct bpf_prog_aux *aux)
{
	lupos_scx_bpf_cid_override(cpu_to_cid__arena, cpu_to_cid_cnt,
				   shard_start__arena, shard_start_cnt, aux);
}
__bpf_kfunc s32 scx_bpf_cid_to_cpu(s32 cid, const struct bpf_prog_aux *aux)
{
	return lupos_scx_bpf_cid_to_cpu(cid, aux);
}
__bpf_kfunc s32 scx_bpf_cpu_to_cid(s32 cpu, const struct bpf_prog_aux *aux)
{
	return lupos_scx_bpf_cpu_to_cid(cpu, aux);
}
__bpf_kfunc void scx_bpf_cid_topo(s32 cid, struct scx_cid_topo *out__uninit,
				const struct bpf_prog_aux *aux)
{
	lupos_scx_bpf_cid_topo(cid, out__uninit, aux);
}
__bpf_kfunc_end_defs();

BTF_KFUNCS_START(scx_kfunc_ids_init_cids)
BTF_ID_FLAGS(func, scx_bpf_cid_override, KF_IMPLICIT_ARGS | KF_SLEEPABLE)
BTF_KFUNCS_END(scx_kfunc_ids_init_cids)

static const struct btf_kfunc_id_set lupos_scx_cid_init_set = {
	.owner = THIS_MODULE,
	.set = &scx_kfunc_ids_init_cids,
	.filter = scx_kfunc_context_filter,
};

BTF_KFUNCS_START(scx_kfunc_ids_cid)
BTF_ID_FLAGS(func, scx_bpf_cid_to_cpu, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_cpu_to_cid, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_cid_topo, KF_IMPLICIT_ARGS)
BTF_KFUNCS_END(scx_kfunc_ids_cid)

static const struct btf_kfunc_id_set lupos_scx_cid_set = {
	.owner = THIS_MODULE,
	.set = &scx_kfunc_ids_cid,
};

int lupos_scx_cid_register_init(void)
{
	return register_btf_kfunc_id_set(BPF_PROG_TYPE_STRUCT_OPS, &lupos_scx_cid_init_set);
}
int lupos_scx_cid_register_ops(void)
{
	return register_btf_kfunc_id_set(BPF_PROG_TYPE_STRUCT_OPS, &lupos_scx_cid_set);
}
int lupos_scx_cid_register_tracing(void)
{
	return register_btf_kfunc_id_set(BPF_PROG_TYPE_TRACING, &lupos_scx_cid_set);
}
int lupos_scx_cid_register_syscall(void)
{
	return register_btf_kfunc_id_set(BPF_PROG_TYPE_SYSCALL, &lupos_scx_cid_set);
}
