// SPDX-License-Identifier: GPL-2.0
/* BPF-facing typed thunks and exact idle/select-CPU BTF metadata. This remains
 * an explicit, unqualified C runtime boundary. Include in the native policy
 * translation unit only after independent admission; never assume standalone
 * object linkage for the locally emitted BTF ID sets/context-filter users. */
#error "SOURCE ONLY HOLD: sched_ext idle BTF and callback admission is pending"
#include "sched_ext_idle_bindings.h"

__bpf_kfunc_start_defs();
__bpf_kfunc s32 scx_bpf_cpu_node(s32 cpu, const struct bpf_prog_aux *aux)
{
	return lupos_scx_idle_bpf_cpu_node(cpu, aux);
}
__bpf_kfunc s32 scx_bpf_select_cpu_dfl(struct task_struct *p, s32 prev_cpu,
				    u64 wake_flags, bool *is_idle, const struct bpf_prog_aux *aux)
{
	return lupos_scx_idle_bpf_select_cpu_dfl(p, prev_cpu, wake_flags, is_idle, aux);
}
__bpf_kfunc s32 __scx_bpf_select_cpu_and(struct task_struct *p, const struct cpumask *cpus_allowed,
				      struct scx_bpf_select_cpu_and_args *args, const struct bpf_prog_aux *aux)
{
	return lupos_scx_idle_bpf_select_cpu_and(p, cpus_allowed, args, aux);
}
/* COMPAT: Will be removed in v6.22, matching the pinned source. */
__bpf_kfunc s32 scx_bpf_select_cpu_and(struct task_struct *p, s32 prev_cpu, u64 wake_flags,
				    const struct cpumask *cpus_allowed, u64 flags)
{
	return lupos_scx_idle_bpf_select_cpu_compat(p, prev_cpu, wake_flags, cpus_allowed, flags);
}
__bpf_kfunc const struct cpumask *scx_bpf_get_idle_cpumask_node(s32 node, const struct bpf_prog_aux *aux)
{
	return lupos_scx_idle_bpf_get_idle_cpumask_node(node, aux);
}
__bpf_kfunc const struct cpumask *scx_bpf_get_idle_cpumask(const struct bpf_prog_aux *aux)
{
	return lupos_scx_idle_bpf_get_idle_cpumask(aux);
}
__bpf_kfunc const struct cpumask *scx_bpf_get_idle_smtmask_node(s32 node, const struct bpf_prog_aux *aux)
{
	return lupos_scx_idle_bpf_get_idle_smtmask_node(node, aux);
}
__bpf_kfunc const struct cpumask *scx_bpf_get_idle_smtmask(const struct bpf_prog_aux *aux)
{
	return lupos_scx_idle_bpf_get_idle_smtmask(aux);
}
__bpf_kfunc void scx_bpf_put_idle_cpumask(const struct cpumask *idle_mask)
{
	lupos_scx_idle_bpf_put_idle_cpumask(idle_mask);
}
__bpf_kfunc bool scx_bpf_test_and_clear_cpu_idle(s32 cpu, const struct bpf_prog_aux *aux)
{
	return lupos_scx_idle_bpf_test_and_clear_cpu_idle(cpu, aux);
}
__bpf_kfunc s32 scx_bpf_pick_idle_cpu_node(const struct cpumask *cpus_allowed,
					s32 node, u64 flags, const struct bpf_prog_aux *aux)
{
	return lupos_scx_idle_bpf_pick_idle_cpu_node(cpus_allowed, node, flags, aux);
}
__bpf_kfunc s32 scx_bpf_pick_idle_cpu(const struct cpumask *cpus_allowed, u64 flags,
				   const struct bpf_prog_aux *aux)
{
	return lupos_scx_idle_bpf_pick_idle_cpu(cpus_allowed, flags, aux);
}
__bpf_kfunc s32 scx_bpf_pick_any_cpu_node(const struct cpumask *cpus_allowed,
				       s32 node, u64 flags, const struct bpf_prog_aux *aux)
{
	return lupos_scx_idle_bpf_pick_any_cpu_node(cpus_allowed, node, flags, aux);
}
__bpf_kfunc s32 scx_bpf_pick_any_cpu(const struct cpumask *cpus_allowed, u64 flags,
				  const struct bpf_prog_aux *aux)
{
	return lupos_scx_idle_bpf_pick_any_cpu(cpus_allowed, flags, aux);
}
__bpf_kfunc_end_defs();

BTF_KFUNCS_START(scx_kfunc_ids_idle)
BTF_ID_FLAGS(func, scx_bpf_cpu_node, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_get_idle_cpumask_node, KF_IMPLICIT_ARGS | KF_ACQUIRE)
BTF_ID_FLAGS(func, scx_bpf_get_idle_cpumask, KF_IMPLICIT_ARGS | KF_ACQUIRE)
BTF_ID_FLAGS(func, scx_bpf_get_idle_smtmask_node, KF_IMPLICIT_ARGS | KF_ACQUIRE)
BTF_ID_FLAGS(func, scx_bpf_get_idle_smtmask, KF_IMPLICIT_ARGS | KF_ACQUIRE)
BTF_ID_FLAGS(func, scx_bpf_put_idle_cpumask, KF_RELEASE)
BTF_ID_FLAGS(func, scx_bpf_test_and_clear_cpu_idle, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_pick_idle_cpu_node, KF_IMPLICIT_ARGS | KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_pick_idle_cpu, KF_IMPLICIT_ARGS | KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_pick_any_cpu_node, KF_IMPLICIT_ARGS | KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_pick_any_cpu, KF_IMPLICIT_ARGS | KF_RCU)
BTF_KFUNCS_END(scx_kfunc_ids_idle)

static const struct btf_kfunc_id_set scx_kfunc_set_idle = {
	.owner = THIS_MODULE,
	.set = &scx_kfunc_ids_idle,
	.filter = scx_kfunc_context_filter,
};

/* select_cpu is forbidden to arbitrary TRACING contexts: it may take pi_lock.
 * ext.c's unlocked/cpu-only/context-filter sets still own their memberships. */
BTF_KFUNCS_START(scx_kfunc_ids_select_cpu)
BTF_ID_FLAGS(func, __scx_bpf_select_cpu_and, KF_IMPLICIT_ARGS | KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_select_cpu_and, KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_select_cpu_dfl, KF_IMPLICIT_ARGS | KF_RCU)
BTF_KFUNCS_END(scx_kfunc_ids_select_cpu)

static const struct btf_kfunc_id_set scx_kfunc_set_select_cpu = {
	.owner = THIS_MODULE,
	.set = &scx_kfunc_ids_select_cpu,
	.filter = scx_kfunc_context_filter,
};
int lupos_scx_idle_register_ops(void)
{
	return register_btf_kfunc_id_set(BPF_PROG_TYPE_STRUCT_OPS, &scx_kfunc_set_idle);
}
int lupos_scx_idle_register_tracing(void)
{
	return register_btf_kfunc_id_set(BPF_PROG_TYPE_TRACING, &scx_kfunc_set_idle);
}
int lupos_scx_idle_register_syscall(void)
{
	return register_btf_kfunc_id_set(BPF_PROG_TYPE_SYSCALL, &scx_kfunc_set_idle);
}
int lupos_scx_idle_register_select_ops(void)
{
	return register_btf_kfunc_id_set(BPF_PROG_TYPE_STRUCT_OPS, &scx_kfunc_set_select_cpu);
}
int lupos_scx_idle_register_select_syscall(void)
{
	return register_btf_kfunc_id_set(BPF_PROG_TYPE_SYSCALL, &scx_kfunc_set_select_cpu);
}
