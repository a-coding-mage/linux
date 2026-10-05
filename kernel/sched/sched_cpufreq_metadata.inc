/* SPDX-License-Identifier: GPL-2.0 */
/* Native sysfs, kobject and governor metadata; lifecycle bodies remain Rust. */
static struct governor_attr rate_limit_us =
	__ATTR(rate_limit_us, 0644, lupos_sugov_rate_limit_us_show,
	       lupos_sugov_rate_limit_us_store);
static struct attribute *sugov_attrs[] = {
	&rate_limit_us.attr,
	NULL,
};
ATTRIBUTE_GROUPS(sugov);
static const struct kobj_type sugov_tunables_ktype = {
	.default_groups = sugov_groups,
	.sysfs_ops = &governor_sysfs_ops,
	.release = lupos_sugov_tunables_release,
};
static struct cpufreq_governor schedutil_gov = {
	.name = "schedutil",
	.owner = THIS_MODULE,
	.flags = CPUFREQ_GOV_DYNAMIC_SWITCHING,
	.init = lupos_sugov_init,
	.exit = lupos_sugov_exit,
	.start = lupos_sugov_start,
	.stop = lupos_sugov_stop,
	.limits = lupos_sugov_limits,
};
cpufreq_governor_init(schedutil_gov);
