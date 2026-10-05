/* SPDX-License-Identifier: GPL-2.0 */
/* Site names refer to unchanged rt.c authority lines. Retain each exact macro:
 * regular WARN_ON must not silently become once-only. Native codegen/caller
 * metadata still requires later qualification; these are not waivers. */
#ifdef CONFIG_RT_GROUP_SCHED
RT_WARN(warn_173, WARN_ON_ONCE)
RT_WARN(warn_181, WARN_ON)
RT_WARN(warn_187, WARN_ON)
RT_WARN(warn_195, WARN_ON)
RT_WARN(warn_496, WARN_ON)
RT_WARN(warn_726, WARN_ON_ONCE)
RT_WARN(warn_1134, WARN_ON)
#ifdef CONFIG_SCHED_CORE
RT_WARN(warn_2593, WARN_ON)
#endif
#endif
RT_WARN(warn_1097, WARN_ON)
RT_WARN(warn_1179, WARN_ON)
RT_WARN(warn_1190, WARN_ON)
RT_WARN(warn_1191, WARN_ON)
RT_WARN(warn_1351, WARN_ON_ONCE)
RT_WARN(warn_1371, WARN_ON_ONCE)
RT_WARN(warn_1694, WARN_ON_ONCE)
RT_WARN(warn_2026, WARN_ON)
RT_WARN(warn_2326, WARN_ON)
RT_WARN(warn_2327, WARN_ON)
RT_BUG(bug_1015)
RT_BUG(bug_1020)
RT_BUG(bug_1032)
RT_BUG(bug_1691)
RT_BUG(bug_1885)
RT_BUG(bug_1886)
RT_BUG(bug_1887)
RT_BUG(bug_1888)
RT_BUG(bug_1890)
RT_BUG(bug_1891)
