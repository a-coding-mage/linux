/* SPDX-License-Identifier: GPL-2.0 */
/*
 * List of cgroup subsystems.
 *
 * DO NOT ADD ANY SUBSYSTEM WITHOUT EXPLICIT ACKS FROM CGROUP MAINTAINERS.
 */

/*
 * This file must be included with the SUBSYS! macro defined.
 * The cfg feature names below preserve the corresponding IS_ENABLED()
 * conditions from the C header.
 */

#[cfg(feature = "CONFIG_CPUSETS")]
SUBSYS!(cpuset);

#[cfg(feature = "CONFIG_CGROUP_SCHED")]
SUBSYS!(cpu);

#[cfg(feature = "CONFIG_CGROUP_CPUACCT")]
SUBSYS!(cpuacct);

#[cfg(feature = "CONFIG_BLK_CGROUP")]
SUBSYS!(io);

#[cfg(feature = "CONFIG_MEMCG")]
SUBSYS!(memory);

#[cfg(feature = "CONFIG_CGROUP_DEVICE")]
SUBSYS!(devices);

#[cfg(feature = "CONFIG_CGROUP_FREEZER")]
SUBSYS!(freezer);

#[cfg(feature = "CONFIG_CGROUP_NET_CLASSID")]
SUBSYS!(net_cls);

#[cfg(feature = "CONFIG_CGROUP_PERF")]
SUBSYS!(perf_event);

#[cfg(feature = "CONFIG_CGROUP_NET_PRIO")]
SUBSYS!(net_prio);

#[cfg(feature = "CONFIG_CGROUP_HUGETLB")]
SUBSYS!(hugetlb);

#[cfg(feature = "CONFIG_CGROUP_PIDS")]
SUBSYS!(pids);

#[cfg(feature = "CONFIG_CGROUP_RDMA")]
SUBSYS!(rdma);

#[cfg(feature = "CONFIG_CGROUP_MISC")]
SUBSYS!(misc);

#[cfg(feature = "CONFIG_CGROUP_DMEM")]
SUBSYS!(dmem);

/*
 * The following subsystems are not supported on the default hierarchy.
 */
#[cfg(feature = "CONFIG_CGROUP_DEBUG")]
SUBSYS!(debug);

/*
 * DO NOT ADD ANY SUBSYSTEM WITHOUT EXPLICIT ACKS FROM CGROUP MAINTAINERS.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
