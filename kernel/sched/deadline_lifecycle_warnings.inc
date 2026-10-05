/* SPDX-License-Identifier: GPL-2.0 */
/* Distinct sites preserve the original warning multiplicity. */
DL_WARN_ONCE(lifecycle_defer_nonserver)
DL_WARN_ONCE(lifecycle_defer_timer_start)
DL_WARN_ONCE(lifecycle_start_offline)
DL_WARN(lifecycle_init_fair_server)
#ifdef CONFIG_SCHED_CLASS_EXT
DL_WARN(lifecycle_init_ext_server)
#endif
DL_WARN_ONCE(lifecycle_swap_rq)
