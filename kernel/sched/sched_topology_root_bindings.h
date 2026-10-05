/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_TOPOLOGY_ROOT_BINDINGS_H
#define LUPOS_SCHED_TOPOLOGY_ROOT_BINDINGS_H
#define LUPOS_TOPOLOGY_ROOT_SPAN 0U
#define LUPOS_TOPOLOGY_ROOT_ONLINE 1U
#define LUPOS_TOPOLOGY_ROOT_DLO 2U
#define LUPOS_TOPOLOGY_ROOT_RTO 3U
extern struct root_domain def_root_domain;
struct root_domain *lupos_topology_root_alloc(void);
struct root_domain *lupos_topology_root_from_rcu(struct rcu_head *rcu);
struct cpumask *lupos_topology_root_mask(struct root_domain *rd, unsigned int slot);
bool lupos_topology_root_mask_alloc(struct root_domain *rd, unsigned int slot);
void lupos_topology_root_mask_free(struct root_domain *rd, unsigned int slot);
void lupos_topology_root_init_push(struct root_domain *rd);
void lupos_topology_root_call_rcu(struct root_domain *rd);
void lupos_topology_free_rootdomain(struct rcu_head *rcu);
void __init lupos_topology_init_defrootdomain(void);
void lupos_topology_rq_lock(struct rq *rq, struct rq_flags *rf);
void lupos_topology_rq_unlock(struct rq *rq, struct rq_flags *rf);
bool lupos_topology_rq_fair_server_active(struct rq *rq);
#ifdef CONFIG_SCHED_CLASS_EXT
bool lupos_topology_rq_ext_server_active(struct rq *rq);
#endif
void lupos_topology_atomic_inc(atomic_t *a);
bool lupos_topology_atomic_dec_and_test(atomic_t *a);
void lupos_topology_atomic_set(atomic_t *a, int value);
int lupos_topology_atomic_read(const atomic_t *a);
void lupos_topology_mask_set_cpu(int cpu, struct cpumask *mask);
void lupos_topology_mask_clear_cpu(int cpu, struct cpumask *mask);
struct sched_domain *lupos_topology_domain_from_rcu(struct rcu_head *rcu);
void lupos_topology_domain_call_rcu(struct sched_domain *sd);
void lupos_topology_destroy_sched_domains_rcu(struct rcu_head *rcu);
struct sched_domain *lupos_topology_highest_flag_domain(int cpu, int flag);
struct sched_domain *lupos_topology_lowest_flag_domain(int cpu, int flag);
void lupos_topology_warn_missing_shared(bool missing);
void lupos_topology_sd_llc_assign(int cpu, struct sched_domain *sd);
void lupos_topology_sd_llc_size_set(int cpu, int size);
void lupos_topology_sd_llc_shared_assign(int cpu, struct sched_domain_shared *sds);
void lupos_topology_sd_share_id_set(int cpu, int id);
void lupos_topology_sd_numa_assign(int cpu, struct sched_domain *sd);
void lupos_topology_sd_asym_packing_assign(int cpu, struct sched_domain *sd);
void lupos_topology_sd_asym_cpucapacity_assign(int cpu, struct sched_domain *sd);
void lupos_topology_sd_balance_shared_assign(int cpu, struct sched_domain_shared *sds);
void lupos_topology_rq_sd_assign(struct rq *rq, struct sched_domain *sd);
bool lupos_topology_sgc_put(struct sched_group_capacity *sgc);
bool lupos_topology_sg_put(struct sched_group *sg);
bool lupos_topology_sds_put(struct sched_domain_shared *sds);
#endif
