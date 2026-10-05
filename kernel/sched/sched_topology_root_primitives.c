// SPDX-License-Identifier: GPL-2.0
/* Unqualified native runtime storage/header leaves; no topology.c fallback. */
#include "sched_topology_bindings.h"
#error "SOURCE ONLY HOLD: scheduler topology root lifecycle is not admitted"
struct root_domain def_root_domain;
void __init init_defrootdomain(void) { lupos_topology_init_defrootdomain(); }
struct root_domain *lupos_topology_root_alloc(void) { return kzalloc_obj(struct root_domain); }
struct root_domain *lupos_topology_root_from_rcu(struct rcu_head *rcu) { return container_of(rcu, struct root_domain, rcu); }
/* Caller supplies one of the four native LUPOS_TOPOLOGY_ROOT_* slot tags. */
struct cpumask *lupos_topology_root_mask(struct root_domain *rd, unsigned int slot)
{
    struct cpumask *masks[] = { rd->span, rd->online, rd->dlo_mask, rd->rto_mask };

    return masks[slot];
}
bool lupos_topology_root_mask_alloc(struct root_domain *rd, unsigned int slot)
{
    cpumask_var_t *masks[] = { &rd->span, &rd->online, &rd->dlo_mask, &rd->rto_mask };

    return zalloc_cpumask_var(masks[slot], GFP_KERNEL);
}
void lupos_topology_root_mask_free(struct root_domain *rd, unsigned int slot) { free_cpumask_var(lupos_topology_root_mask(rd, slot)); }
void lupos_topology_root_init_push(struct root_domain *rd)
{
#ifdef HAVE_RT_PUSH_IPI
    rd->rto_cpu = -1;
    raw_spin_lock_init(&rd->rto_lock);
    rd->rto_push_work = IRQ_WORK_INIT_HARD(rto_push_irq_work_func);
#endif
}
void lupos_topology_root_call_rcu(struct root_domain *rd) { call_rcu(&rd->rcu, lupos_topology_free_rootdomain); }
void lupos_topology_rq_lock(struct rq *rq, struct rq_flags *rf) { rq_lock_irqsave(rq, rf); }
void lupos_topology_rq_unlock(struct rq *rq, struct rq_flags *rf) { rq_unlock_irqrestore(rq, rf); }
bool lupos_topology_rq_fair_server_active(struct rq *rq) { return rq->fair_server.dl_server; }
#ifdef CONFIG_SCHED_CLASS_EXT
bool lupos_topology_rq_ext_server_active(struct rq *rq) { return rq->ext_server.dl_server; }
#endif
void lupos_topology_atomic_inc(atomic_t *a) { atomic_inc(a); }
bool lupos_topology_atomic_dec_and_test(atomic_t *a) { return atomic_dec_and_test(a); }
void lupos_topology_atomic_set(atomic_t *a, int value) { atomic_set(a, value); }
int lupos_topology_atomic_read(const atomic_t *a) { return atomic_read(a); }
void lupos_topology_mask_set_cpu(int cpu, struct cpumask *mask) { cpumask_set_cpu(cpu, mask); }
void lupos_topology_mask_clear_cpu(int cpu, struct cpumask *mask) { cpumask_clear_cpu(cpu, mask); }
struct sched_domain *lupos_topology_domain_from_rcu(struct rcu_head *rcu) { return container_of(rcu, struct sched_domain, rcu); }
void lupos_topology_domain_call_rcu(struct sched_domain *sd) { call_rcu(&sd->rcu, lupos_topology_destroy_sched_domains_rcu); }
DEFINE_PER_CPU(struct sched_domain __rcu *, sd_llc);
DEFINE_PER_CPU(int, sd_llc_size);
DEFINE_PER_CPU(int, sd_llc_id) = -1;
DEFINE_PER_CPU(int, sd_share_id);
DEFINE_PER_CPU(struct sched_domain_shared __rcu *, sd_llc_shared);
DEFINE_PER_CPU(struct sched_domain_shared __rcu *, sd_balance_shared);
DEFINE_PER_CPU(struct sched_domain __rcu *, sd_numa);
DEFINE_PER_CPU(struct sched_domain __rcu *, sd_asym_packing);
DEFINE_PER_CPU(struct sched_domain __rcu *, sd_asym_cpucapacity);
DEFINE_STATIC_KEY_FALSE(sched_asym_cpucapacity);
DEFINE_STATIC_KEY_FALSE(sched_cluster_active);
struct sched_domain *lupos_topology_highest_flag_domain(int cpu, int flag) { return highest_flag_domain(cpu, flag); }
struct sched_domain *lupos_topology_lowest_flag_domain(int cpu, int flag) { return lowest_flag_domain(cpu, flag); }
void lupos_topology_warn_missing_shared(bool missing) { WARN_ON_ONCE(missing); }
void lupos_topology_sd_llc_assign(int cpu, struct sched_domain *sd) { rcu_assign_pointer(per_cpu(sd_llc, cpu), sd); }
void lupos_topology_sd_llc_size_set(int cpu, int size) { per_cpu(sd_llc_size, cpu) = size; }
void lupos_topology_sd_llc_shared_assign(int cpu, struct sched_domain_shared *sds) { rcu_assign_pointer(per_cpu(sd_llc_shared, cpu), sds); }
void lupos_topology_sd_share_id_set(int cpu, int id) { per_cpu(sd_share_id, cpu) = id; }
void lupos_topology_sd_numa_assign(int cpu, struct sched_domain *sd) { rcu_assign_pointer(per_cpu(sd_numa, cpu), sd); }
void lupos_topology_sd_asym_packing_assign(int cpu, struct sched_domain *sd) { rcu_assign_pointer(per_cpu(sd_asym_packing, cpu), sd); }
void lupos_topology_sd_asym_cpucapacity_assign(int cpu, struct sched_domain *sd) { rcu_assign_pointer(per_cpu(sd_asym_cpucapacity, cpu), sd); }
void lupos_topology_sd_balance_shared_assign(int cpu, struct sched_domain_shared *sds) { rcu_assign_pointer(per_cpu(sd_balance_shared, cpu), sds); }
void lupos_topology_rq_sd_assign(struct rq *rq, struct sched_domain *sd) { rcu_assign_pointer(rq->sd, sd); }
bool lupos_topology_sgc_put(struct sched_group_capacity *sgc) { return atomic_dec_and_test(&sgc->ref); }
bool lupos_topology_sg_put(struct sched_group *sg) { return atomic_dec_and_test(&sg->ref); }
bool lupos_topology_sds_put(struct sched_domain_shared *sds) { return atomic_dec_and_test(&sds->ref); }
