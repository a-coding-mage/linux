/* SPDX-License-Identifier: GPL-2.0 */
/*
 * sched-domains (multiprocessor balancing) flag declarations.
 */

// The C header requires SD_FLAG to be defined by the importing context.

/*
 * Hierarchical metaflags
 *
 * SHARED_CHILD: These flags are meant to be set from the base domain upwards.
 * If a domain has this flag set, all of its children should have it set. This
 * is usually because the flag describes some shared resource (all CPUs in that
 * domain share the same resource), or because they are tied to a scheduling
 * behaviour that we want to disable at some point in the hierarchy for
 * scalability reasons.
 *
 * In those cases it doesn't make sense to have the flag set for a domain but
 * not have it in (some of) its children: sched domains ALWAYS span their child
 * domains, so operations done with parent domains will cover CPUs in the lower
 * child domains.
 *
 *
 * SHARED_PARENT: These flags are meant to be set from the highest domain
 * downwards. If a domain has this flag set, all of its parents should have it
 * set. This is usually for topology properties that start to appear above a
 * certain level (e.g. domain starts spanning CPUs outside of the base CPU's
 * socket).
 */
pub const SDF_SHARED_CHILD: u32 = 0x1;
pub const SDF_SHARED_PARENT: u32 = 0x2;

/*
 * Behavioural metaflags
 *
 * NEEDS_GROUPS: These flags are only relevant if the domain they are set on has
 * more than one group. This is usually for balancing flags (load balancing
 * involves equalizing a metric between groups), or for flags describing some
 * shared resource (which would be shared between groups).
 */
pub const SDF_NEEDS_GROUPS: u32 = 0x4;

/*
 * Balance when about to become idle
 *
 * SHARED_CHILD: Set from the base domain up to cpuset.sched_relax_domain_level.
 * NEEDS_GROUPS: Load balancing flag.
 */
SD_FLAG!(SD_BALANCE_NEWIDLE, SDF_SHARED_CHILD | SDF_NEEDS_GROUPS);

/* Balance on exec */
SD_FLAG!(SD_BALANCE_EXEC, SDF_SHARED_CHILD | SDF_NEEDS_GROUPS);

/* Balance on fork, clone */
SD_FLAG!(SD_BALANCE_FORK, SDF_SHARED_CHILD | SDF_NEEDS_GROUPS);

/* Balance on wakeup */
SD_FLAG!(SD_BALANCE_WAKE, SDF_SHARED_CHILD | SDF_NEEDS_GROUPS);

/* Consider waking task on waking CPU. */
SD_FLAG!(SD_WAKE_AFFINE, SDF_SHARED_CHILD);

/* Domain members have different CPU capacities */
SD_FLAG!(SD_ASYM_CPUCAPACITY, SDF_SHARED_PARENT | SDF_NEEDS_GROUPS);

/* Domain members have different CPU capacities spanning all unique CPU capacity values. */
SD_FLAG!(SD_ASYM_CPUCAPACITY_FULL, SDF_SHARED_PARENT | SDF_NEEDS_GROUPS);

/* Domain members share CPU capacity (i.e. SMT) */
SD_FLAG!(SD_SHARE_CPUCAPACITY, SDF_SHARED_CHILD | SDF_NEEDS_GROUPS);

/* Domain members share CPU cluster (LLC tags or L2 cache) */
SD_FLAG!(SD_CLUSTER, SDF_NEEDS_GROUPS);

/* Domain members share CPU Last Level Caches */
SD_FLAG!(SD_SHARE_LLC, SDF_SHARED_CHILD | SDF_NEEDS_GROUPS);

/* Only a single load balancing instance */
SD_FLAG!(SD_SERIALIZE, SDF_SHARED_PARENT | SDF_NEEDS_GROUPS);

/* Place busy tasks earlier in the domain */
SD_FLAG!(SD_ASYM_PACKING, SDF_NEEDS_GROUPS);

/* Prefer to place tasks in a sibling domain */
SD_FLAG!(SD_PREFER_SIBLING, SDF_NEEDS_GROUPS);

/* Cross-node balancing */
SD_FLAG!(SD_NUMA, SDF_SHARED_PARENT | SDF_NEEDS_GROUPS);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
