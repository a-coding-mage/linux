/* SPDX-License-Identifier: GPL-2.0-only */
/* Native macro-owned registration records; bodies are in Rust. */
extern int rust_vs_kswapd_init(void);
module_init(rust_vs_kswapd_init);
static const struct ctl_table vmscan_sysctl_table[] = {
	{
		.procname	= "swappiness",
		.data		= &vm_swappiness,
		.maxlen		= sizeof(vm_swappiness),
		.mode		= 0644,
		.proc_handler	= proc_dointvec_minmax,
		.extra1		= SYSCTL_ZERO,
		.extra2		= SYSCTL_TWO_HUNDRED,
	},
#ifdef CONFIG_NUMA
	{
		.procname	= "zone_reclaim_mode",
		.data		= &node_reclaim_mode,
		.maxlen		= sizeof(node_reclaim_mode),
		.mode		= 0644,
		.proc_handler	= proc_dointvec_minmax,
		.extra1		= SYSCTL_ZERO,
	}
#endif
};
static const match_table_t tokens = {
	{ MEMORY_RECLAIM_SWAPPINESS, "swappiness=%d"},
	{ MEMORY_RECLAIM_SWAPPINESS_MAX, "swappiness=max"},
	{ MEMORY_RECLAIM_NULL, NULL },
};

EXPORT_SYMBOL_GPL(check_move_unevictable_folios);
#if defined(CONFIG_SYSFS) && defined(CONFIG_NUMA)
extern ssize_t rust_vs_reclaim_store(struct device *, struct device_attribute *, const char *, size_t);
#define reclaim_store rust_vs_reclaim_store
static DEVICE_ATTR_WO(reclaim);
#undef reclaim_store
#endif
