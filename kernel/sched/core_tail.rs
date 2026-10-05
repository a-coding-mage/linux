// SPDX-License-Identifier: GPL-2.0-only
// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
// Frozen source authority: e1d84f501551943a11f4c5271e9f5c85d7e15168.
// Algorithm ownership: kernel/sched/core.c:5532..11321.
// SOURCE ONLY: native leaves, configured bindings and protection admission are
// deliberately not claimed validated. See review/STATUS.md and native-boundaries.
include!("core_tail_stats.rs");
include!("core_tail_pick.rs");
include!("core_tail_proxy.rs");
include!("core_tail_schedule.rs");
include!("core_tail_preempt.rs");
include!("core_tail_hotplug.rs");
include!("core_tail_init.rs");
include!("core_tail_groups.rs");
include!("core_tail_bandwidth.rs");
include!("core_tail_debug.rs");
include!("core_tail_mmcid.rs");
include!("core_tail_change.rs");
