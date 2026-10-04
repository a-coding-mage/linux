/* SPDX-License-Identifier: GPL-2.0 */
/* Only native section/registration/per-CPU storage; algorithms are Rust. */
#ifdef CONFIG_KVFREE_RCU_BATCHED
int rust_slab_common_rcu_min_cached_objs = 5;
module_param_named(rcu_min_cached_objs, rust_slab_common_rcu_min_cached_objs, int, 0444);

int rust_slab_common_rcu_delay_page_cache_fill_msec = 5000;
module_param_named(rcu_delay_page_cache_fill_msec,
		   rust_slab_common_rcu_delay_page_cache_fill_msec, int, 0444);

static DEFINE_PER_CPU(struct kfree_rcu_cpu, krc) = {
	.lock = __RAW_SPIN_LOCK_UNLOCKED(krc.lock),
};
#endif /* CONFIG_KVFREE_RCU_BATCHED */
