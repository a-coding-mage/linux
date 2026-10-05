/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_TOPOLOGY_BUILD_BINDINGS_H
#define LUPOS_SCHED_TOPOLOGY_BUILD_BINDINGS_H
/* Included after native s_data/s_alloc and configured scheduler headers. */
#define LUPOS_TOPOLOGY_BUILD_SA_ROOTDOMAIN sa_rootdomain
#define LUPOS_TOPOLOGY_BUILD_SA_SD sa_sd
#define LUPOS_TOPOLOGY_BUILD_SA_SD_SHARED sa_sd_shared
#define LUPOS_TOPOLOGY_BUILD_SA_SD_STORAGE sa_sd_storage
#define LUPOS_TOPOLOGY_BUILD_SA_NONE sa_none
/* Caller contract: construction is serialized by sched_domains_mutex and stable
 * CPU topology. d/sdd are live exclusive native objects; allocators require
 * sleepable context. A successful per-CPU allocation is zeroed; setters/getters
 * require an allocated array and valid possible CPU. The caller owns installed
 * objects until claiming them by nulling slots, then frees only the remainder.
 * Array-free leaves release the array, not objects still stored in its slots.
 * data_zero requires storage without outstanding resources. */
void lupos_topology_build_data_zero(struct s_data *d);
bool lupos_topology_build_data_sd_alloc(struct s_data *d);
void lupos_topology_build_data_sd_free(struct s_data *d);
void lupos_topology_build_data_sd_set(struct s_data *d, int cpu, struct sched_domain *sd);
bool lupos_topology_build_data_sds_alloc(struct s_data *d);
void lupos_topology_build_data_sds_free(struct s_data *d);
struct sched_domain_shared *lupos_topology_build_data_sds(struct s_data *d, int cpu);
void lupos_topology_build_data_sds_set(struct s_data *d, int cpu, struct sched_domain_shared *sds);
bool lupos_topology_build_sdd_sd_alloc(struct sd_data *sdd);
bool lupos_topology_build_sdd_sg_alloc(struct sd_data *sdd);
bool lupos_topology_build_sdd_sgc_alloc(struct sd_data *sdd);
void lupos_topology_build_sdd_sd_free(struct sd_data *sdd);
void lupos_topology_build_sdd_sg_free(struct sd_data *sdd);
void lupos_topology_build_sdd_sgc_free(struct sd_data *sdd);
void lupos_topology_build_sdd_sd_set(struct sd_data *sdd, int cpu, struct sched_domain *sd);
void lupos_topology_build_sdd_sg_set(struct sd_data *sdd, int cpu, struct sched_group *sg);
void lupos_topology_build_sdd_sgc_set(struct sd_data *sdd, int cpu, struct sched_group_capacity *sgc);
/* Caller owns each non-null zeroed allocation and must install it in a cleanup
 * slot or release it. Native sizeof and cpumask_size own all tail-mask sizing. */
struct sched_domain *lupos_topology_build_sd_alloc(int cpu);
struct sched_group *lupos_topology_build_sg_alloc(int cpu);
struct sched_group_capacity *lupos_topology_build_sgc_alloc(int cpu);
struct sched_domain_shared *lupos_topology_build_sds_alloc(int cpu);
/* Object pointers must remain live. Ref pointers borrow the native atomic
 * fields; alloc_flags access is limited to serialized construction, before the
 * union becomes runtime nr_idle_scan. These leaves do not acquire references. */
atomic_t *lupos_topology_build_sds_ref(struct sched_domain_shared *sds);
atomic_t *lupos_topology_build_sg_ref(struct sched_group *sg);
atomic_t *lupos_topology_build_sgc_ref(struct sched_group_capacity *sgc);
int lupos_topology_build_alloc_flags(struct sched_domain_shared *sds);
void lupos_topology_build_alloc_flags_set(struct sched_domain_shared *sds, int flags);
/* Valid CPU/mask storage and domain-mutex serialization are required for LLC ID
 * reads/writes and non-atomic bitmap updates; bit indices must be in range. */
int lupos_topology_build_llc_id(int cpu);
void lupos_topology_build_llc_id_set(int cpu, int lid);
unsigned int lupos_topology_build_mask_first_zero(const struct cpumask *mask);
unsigned int lupos_topology_build_mask_last(const struct cpumask *mask);
unsigned int lupos_topology_build_mask_any(const struct cpumask *mask);
unsigned int lupos_topology_build_nr_cpumask_bits(void);
void lupos_topology_build_mask_set_nonatomic(int cpu, struct cpumask *mask);
void lupos_topology_build_mask_clear_nonatomic(int cpu, struct cpumask *mask);
/* Static-key updates require the native cpuslocked/hotplug context and balanced
 * per-partition accounting. asym_present only tests an RCU pointer; cpu_domain
 * requires an RCU read section that also covers every use of the returned tree.
 * Housekeeping masks are borrowed and must remain stable during construction. */
void lupos_topology_build_asym_inc(void);
void lupos_topology_build_asym_dec(void);
void lupos_topology_build_cluster_inc(void);
void lupos_topology_build_cluster_dec(void);
bool lupos_topology_build_cluster_active(void);
bool lupos_topology_build_asym_present(int cpu);
struct sched_domain *lupos_topology_build_cpu_domain(int cpu);
const struct cpumask *lupos_topology_build_housekeeping_mask(void);

/* Native cpumask_var_t owns both off-stack and inline-array representations.
 * masks_alloc returns caller-owned array storage; mask_alloc initializes one
 * in-range element and mask_free requires a successfully initialized element.
 * mask borrows that element's mask until it is freed. A failed allocation may
 * leave only a prefix initialized. These leaves never free the array itself. */
cpumask_var_t *lupos_topology_build_masks_alloc(unsigned int ndoms);
bool lupos_topology_build_mask_alloc(cpumask_var_t *doms, unsigned int i);
void lupos_topology_build_mask_free(cpumask_var_t *doms, unsigned int i);
struct cpumask *lupos_topology_build_mask(cpumask_var_t *doms, unsigned int i);
/* Serialized partition bookkeeping owns installed arrays/attributes. Getters
 * borrow pointers, setters neither allocate nor release; callers must finish
 * old-state use, free old ownership, and transfer new ownership in that order.
 * fallback is static storage and must never be passed to the array deallocator.
 * Its init-only allocation must precede use; the original caller ignores failure. */
cpumask_var_t *lupos_topology_build_doms_cur(void);
void lupos_topology_build_doms_cur_set(cpumask_var_t *doms);
int lupos_topology_build_ndoms_cur(void);
void lupos_topology_build_ndoms_cur_set(int ndoms);
struct sched_domain_attr *lupos_topology_build_dattr_cur(void);
void lupos_topology_build_dattr_cur_set(struct sched_domain_attr *attr);
cpumask_var_t *lupos_topology_build_fallback(void);
bool __init lupos_topology_build_fallback_alloc(void);
/* attr_init writes a native default into exclusive storage. attr_compare reads
 * exactly two live native objects; both must remain stable for the comparison. */
void lupos_topology_build_attr_init(struct sched_domain_attr *attr);
int lupos_topology_build_attr_compare(const struct sched_domain_attr *a,
                                     const struct sched_domain_attr *b);
/* Scheduler initialization or hotplug/domain-mutex serialization is required.
 * init_domains is the init-only Rust callback: native scheduler objects and the
 * CPU mask must be initialized, stable and valid while init memory remains live. */
int lupos_topology_build_arch_update(void);
void lupos_topology_build_update_debugfs(void);
void lupos_topology_build_rebuild_dl(void);
int __init lupos_topology_build_init_domains(const struct cpumask *cpu_map);

/* Separate native sites retain original WARN_ON versus WARN_ON_ONCE scope.
 * Diagnostics do not validate or acquire objects: debug_broken needs two live
 * named domains, and debug_root needs a live readable mask during the call. */
void lupos_topology_build_warn_claim(bool mismatch);
void lupos_topology_build_warn_llc(bool invalid);
void lupos_topology_build_warn_parent(bool missing);
bool lupos_topology_build_warn_shared(bool missing);
bool lupos_topology_build_warn_empty(bool empty);
bool lupos_topology_build_warn_span(bool invalid);
void lupos_topology_build_warn_attrs(bool present);
void lupos_topology_build_debug_broken(struct sched_domain *child, struct sched_domain *sd);
void lupos_topology_build_debug_root(const struct cpumask *cpu_map);
#endif
