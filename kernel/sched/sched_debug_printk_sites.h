/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Literal-format / original-owner site manifest for debug.c at 126a30fa.
 * Included by the native metadata boundary only. Source-line keys also occur
 * at the corresponding Rust output expressions. Do not deduplicate records.
 * No CONFIG_PRINTK guard: native indexing also exists with PRINTK disabled.
 * STAT_SITE preserves the disabled schedstat_enabled() == 0 native scope.
 * Unconditional schedstat_val_or_zero output (line 990) stays unconditional.
 */
#ifdef CONFIG_FAIR_GROUP_SCHED
LUPOS_DEBUG_SITE(903, "print_cfs_group_stats", "  .%-30s: %lld.%06ld\n")
LUPOS_DEBUG_SITE(904, "print_cfs_group_stats", "  .%-30s: %lld.%06ld\n")
LUPOS_DEBUG_SITE(905, "print_cfs_group_stats", "  .%-30s: %lld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(911, "print_cfs_group_stats", "  .%-30s: %lld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(912, "print_cfs_group_stats", "  .%-30s: %lld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(913, "print_cfs_group_stats", "  .%-30s: %lld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(914, "print_cfs_group_stats", "  .%-30s: %lld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(915, "print_cfs_group_stats", "  .%-30s: %lld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(916, "print_cfs_group_stats", "  .%-30s: %lld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(917, "print_cfs_group_stats", "  .%-30s: %lld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(918, "print_cfs_group_stats", "  .%-30s: %lld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(919, "print_cfs_group_stats", "  .%-30s: %lld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(920, "print_cfs_group_stats", "  .%-30s: %lld\n")
LUPOS_DEBUG_SITE(923, "print_cfs_group_stats", "  .%-30s: %lld\n")
LUPOS_DEBUG_SITE(924, "print_cfs_group_stats", "  .%-30s: %lld\n")
LUPOS_DEBUG_SITE(925, "print_cfs_group_stats", "  .%-30s: %lld\n")
LUPOS_DEBUG_SITE(926, "print_cfs_group_stats", "  .%-30s: %lld\n")
#endif
LUPOS_DEBUG_SITE(974, "print_task", ">R")
LUPOS_DEBUG_SITE(976, "print_task", " %c")
LUPOS_DEBUG_SITE(978, "print_task", " %15s %5d %10ld %9Ld.%06ld   %c   %9Ld.%06ld %c %9Ld.%06ld %9Ld.%06ld %9Ld   %5d ")
LUPOS_DEBUG_SITE(990, "print_task", "%9lld.%06ld %9lld.%06ld %9lld.%06ld")
#ifdef CONFIG_NUMA_BALANCING
LUPOS_DEBUG_SITE(996, "print_task", "   %d      %d")
#endif
#ifdef CONFIG_CGROUP_SCHED
LUPOS_DEBUG_GROUP_SITE(999, "print_task", "        %s")
#endif
LUPOS_DEBUG_SITE(1002, "print_task", "\n")
LUPOS_DEBUG_SITE(1009, "print_rq", "\n")
LUPOS_DEBUG_SITE(1010, "print_rq", "runnable tasks:\n")
LUPOS_DEBUG_SITE(1011, "print_rq",
 " S            task   PID     weight       vruntime   eligible    "
 "deadline             slice          sum-exec      switches  "
 "prio         wait-time        sum-sleep       sum-block"
#ifdef CONFIG_NUMA_BALANCING
 "  node   group-id"
#endif
#ifdef CONFIG_CGROUP_SCHED
 "  group-path"
#endif
 "\n")
LUPOS_DEBUG_SITE(1021, "print_rq",
 "-------------------------------------------------------"
 "------------------------------------------------------"
 "------------------------------------------------------"
#ifdef CONFIG_NUMA_BALANCING
 "--------------"
#endif
#ifdef CONFIG_CGROUP_SCHED
 "--------------"
#endif
 "\n")
#ifdef CONFIG_FAIR_GROUP_SCHED
LUPOS_DEBUG_SITE(1054, "print_cfs_rq", "\n")
LUPOS_DEBUG_GROUP_SITE(1055, "print_cfs_rq", "cfs_rq[%d]:%s\n")
#else
LUPOS_DEBUG_SITE(1057, "print_cfs_rq", "\n")
LUPOS_DEBUG_SITE(1058, "print_cfs_rq", "cfs_rq[%d]:\n")
#endif
LUPOS_DEBUG_SITE(1078, "print_cfs_rq", "  .%-30s: %Ld.%06ld\n")
LUPOS_DEBUG_SITE(1080, "print_cfs_rq", "  .%-30s: %Ld.%06ld\n")
LUPOS_DEBUG_SITE(1082, "print_cfs_rq", "  .%-30s: %Ld.%06ld\n")
LUPOS_DEBUG_SITE(1084, "print_cfs_rq", "  .%-30s: %Ld (%d bits)\n")
LUPOS_DEBUG_SITE(1086, "print_cfs_rq", "  .%-30s: %Lu\n")
LUPOS_DEBUG_SITE(1088, "print_cfs_rq", "  .%-30s: %u\n")
LUPOS_DEBUG_SITE(1089, "print_cfs_rq", "  .%-30s: %Ld.%06ld\n")
LUPOS_DEBUG_SITE(1091, "print_cfs_rq", "  .%-30s: %Ld.%06ld\n")
LUPOS_DEBUG_SITE(1094, "print_cfs_rq", "  .%-30s: %Ld.%06ld\n")
LUPOS_DEBUG_SITE(1095, "print_cfs_rq", "  .%-30s: %d\n")
LUPOS_DEBUG_SITE(1096, "print_cfs_rq", "  .%-30s: %d\n")
LUPOS_DEBUG_SITE(1097, "print_cfs_rq", "  .%-30s: %d\n")
LUPOS_DEBUG_SITE(1098, "print_cfs_rq", "  .%-30s: %d\n")
LUPOS_DEBUG_SITE(1099, "print_cfs_rq", "  .%-30s: %ld\n")
LUPOS_DEBUG_SITE(1100, "print_cfs_rq", "  .%-30s: %lu\n")
LUPOS_DEBUG_SITE(1102, "print_cfs_rq", "  .%-30s: %lu\n")
LUPOS_DEBUG_SITE(1104, "print_cfs_rq", "  .%-30s: %lu\n")
LUPOS_DEBUG_SITE(1106, "print_cfs_rq", "  .%-30s: %u\n")
LUPOS_DEBUG_SITE(1108, "print_cfs_rq", "  .%-30s: %ld\n")
LUPOS_DEBUG_SITE(1110, "print_cfs_rq", "  .%-30s: %ld\n")
LUPOS_DEBUG_SITE(1112, "print_cfs_rq", "  .%-30s: %ld\n")
#ifdef CONFIG_FAIR_GROUP_SCHED
LUPOS_DEBUG_SITE(1115, "print_cfs_rq", "  .%-30s: %lu\n")
LUPOS_DEBUG_SITE(1117, "print_cfs_rq", "  .%-30s: %ld\n")
LUPOS_DEBUG_SITE(1119, "print_cfs_rq", "  .%-30s: %lu\n")
#endif
#ifdef CONFIG_CFS_BANDWIDTH
LUPOS_DEBUG_SITE(1123, "print_cfs_rq", "  .%-30s: %d\n")
LUPOS_DEBUG_SITE(1125, "print_cfs_rq", "  .%-30s: %d\n")
#endif
#ifdef CONFIG_RT_GROUP_SCHED
LUPOS_DEBUG_SITE(1137, "print_rt_rq", "\n")
LUPOS_DEBUG_GROUP_SITE(1138, "print_rt_rq", "rt_rq[%d]:%s\n")
#else
LUPOS_DEBUG_SITE(1140, "print_rt_rq", "\n")
LUPOS_DEBUG_SITE(1141, "print_rt_rq", "rt_rq[%d]:\n")
#endif
LUPOS_DEBUG_SITE(1151, "print_rt_rq", "  .%-30s: %lu\n")
#ifdef CONFIG_RT_GROUP_SCHED
LUPOS_DEBUG_SITE(1154, "print_rt_rq", "  .%-30s: %Ld\n")
LUPOS_DEBUG_SITE(1155, "print_rt_rq", "  .%-30s: %Ld.%06ld\n")
LUPOS_DEBUG_SITE(1156, "print_rt_rq", "  .%-30s: %Ld.%06ld\n")
#endif
LUPOS_DEBUG_SITE(1168, "print_dl_rq", "\n")
LUPOS_DEBUG_SITE(1169, "print_dl_rq", "dl_rq[%d]:\n")
LUPOS_DEBUG_SITE(1174, "print_dl_rq", "  .%-30s: %lu\n")
LUPOS_DEBUG_SITE(1176, "print_dl_rq", "  .%-30s: %lld\n")
LUPOS_DEBUG_SITE(1177, "print_dl_rq", "  .%-30s: %lld\n")
#ifdef CONFIG_X86
LUPOS_DEBUG_SITE(1190, "print_cpu", "cpu#%d, %u.%03u MHz\n")
#else
LUPOS_DEBUG_SITE(1194, "print_cpu", "cpu#%d\n")
#endif
LUPOS_DEBUG_SIZE_SITE(1208, nr_running)
LUPOS_DEBUG_SIZE_SITE(1209, nr_switches)
LUPOS_DEBUG_SIZE_SITE(1210, nr_uninterruptible)
LUPOS_DEBUG_SITE(1211, "print_cpu", "  .%-30s: %Ld.%06ld\n")
LUPOS_DEBUG_SITE(1212, "print_cpu", "  .%-30s: %ld\n")
LUPOS_DEBUG_SITE(1213, "print_cpu", "  .%-30s: %Ld.%06ld\n")
LUPOS_DEBUG_SITE(1214, "print_cpu", "  .%-30s: %Ld.%06ld\n")
LUPOS_DEBUG_SITE(1219, "print_cpu", "  .%-30s: %Ld\n")
LUPOS_DEBUG_SITE(1220, "print_cpu", "  .%-30s: %Ld\n")
LUPOS_DEBUG_STAT_SITE(1225, "print_cpu", "  .%-30s: %d\n")
LUPOS_DEBUG_STAT_SITE(1226, "print_cpu", "  .%-30s: %d\n")
LUPOS_DEBUG_STAT_SITE(1227, "print_cpu", "  .%-30s: %d\n")
LUPOS_DEBUG_STAT_SITE(1228, "print_cpu", "  .%-30s: %d\n")
LUPOS_DEBUG_STAT_SITE(1229, "print_cpu", "  .%-30s: %d\n")
LUPOS_DEBUG_SITE(1238, "print_cpu", "\n")
LUPOS_DEBUG_SITE(1258, "sched_debug_header", "Sched Debug Version: v0.11, %s %.*s\n")
LUPOS_DEBUG_SITE(1267, "sched_debug_header", "%-40s: %Ld.%06ld\n")
LUPOS_DEBUG_SITE(1268, "sched_debug_header", "%-40s: %Ld.%06ld\n")
LUPOS_DEBUG_SITE(1269, "sched_debug_header", "%-40s: %Ld.%06ld\n")
LUPOS_DEBUG_SITE(1270, "sched_debug_header", "%-40s: %Ld\n")
#ifdef CONFIG_HAVE_UNSTABLE_SCHED_CLOCK
LUPOS_DEBUG_SITE(1272, "sched_debug_header", "%-40s: %Ld\n")
#endif
LUPOS_DEBUG_SITE(1277, "sched_debug_header", "\n")
LUPOS_DEBUG_SITE(1278, "sched_debug_header", "sysctl_sched\n")
LUPOS_DEBUG_SITE(1284, "sched_debug_header", "  .%-40s: %Ld.%06ld\n")
LUPOS_DEBUG_SITE(1285, "sched_debug_header", "  .%-40s: %Ld\n")
LUPOS_DEBUG_SITE(1289, "sched_debug_header", "  .%-40s: %d (%s)\n")
LUPOS_DEBUG_SITE(1293, "sched_debug_header", "\n")
#ifdef CONFIG_NUMA_BALANCING
LUPOS_DEBUG_SITE(1384, "print_numa_stats", "numa_faults node=%d ")
LUPOS_DEBUG_SITE(1385, "print_numa_stats", "task_private=%lu task_shared=%lu ")
LUPOS_DEBUG_SITE(1386, "print_numa_stats", "group_private=%lu group_shared=%lu\n")
LUPOS_DEBUG_SITE(1395, "sched_show_numa", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1397, "sched_show_numa", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1398, "sched_show_numa", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1399, "sched_show_numa", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1400, "sched_show_numa", "current_node=%d, numa_group_id=%d\n")
#endif
LUPOS_DEBUG_SITE(1411, "proc_sched_show_task", "%s (%d, #threads: %d)\n")
LUPOS_DEBUG_SITE(1413, "proc_sched_show_task", "-------------------------------------------------------------------\n")
LUPOS_DEBUG_SITE(1420, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
LUPOS_DEBUG_SITE(1421, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
LUPOS_DEBUG_SITE(1422, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
LUPOS_DEBUG_SITE(1426, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_STAT_SITE(1431, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(1432, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(1433, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(1434, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(1435, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(1436, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(1437, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(1438, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(1439, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(1440, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(1441, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(1442, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_STAT_SITE(1443, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(1444, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_STAT_SITE(1445, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_STAT_SITE(1446, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_STAT_SITE(1447, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_STAT_SITE(1448, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_STAT_SITE(1449, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_STAT_SITE(1450, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_STAT_SITE(1451, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_STAT_SITE(1452, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_STAT_SITE(1453, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_STAT_SITE(1454, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_STAT_SITE(1455, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_STAT_SITE(1471, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
LUPOS_DEBUG_STAT_SITE(1472, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
#ifdef CONFIG_SCHED_CORE
LUPOS_DEBUG_STAT_SITE(1475, "proc_sched_show_task", "%-45s:%14Ld.%06ld\n")
#endif
LUPOS_DEBUG_SITE(1479, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1480, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1481, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1483, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1484, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1485, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1486, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1487, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1488, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1489, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1490, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1491, "proc_sched_show_task", "%-45s:%21Ld\n")
#ifdef CONFIG_UCLAMP_TASK
LUPOS_DEBUG_SITE(1493, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1494, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1495, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1496, "proc_sched_show_task", "%-45s:%21Ld\n")
#endif
LUPOS_DEBUG_SITE(1498, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1499, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1501, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1502, "proc_sched_show_task", "%-45s:%21Ld\n")
LUPOS_DEBUG_SITE(1504, "proc_sched_show_task", "%-45s:%21Ld\n")
#ifdef CONFIG_SCHED_CLASS_EXT
LUPOS_DEBUG_SITE(1507, "proc_sched_show_task", "%-45s:%21Ld\n")
#endif
LUPOS_DEBUG_SITE(1518, "proc_sched_show_task", "%-45s:%21Ld\n")
