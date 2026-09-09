/* Translated from perf_cpum_cf_events.c. External kernel symbols and macros are dependencies. */
// SPDX-License-Identifier: GPL-2.0
/*
 * Perf PMU sysfs events attributes for available CPU-measurement counters
 *
 */



/* BEGIN: CPUM_CF COUNTER DEFINITIONS =================================== */

cpumf_event_attr!(cf_fvn1, CPU_CYCLES, 0x0000);
cpumf_event_attr!(cf_fvn1, INSTRUCTIONS, 0x0001);
cpumf_event_attr!(cf_fvn1, L1I_DIR_WRITES, 0x0002);
cpumf_event_attr!(cf_fvn1, L1I_PENALTY_CYCLES, 0x0003);
cpumf_event_attr!(cf_fvn1, PROBLEM_STATE_CPU_CYCLES, 0x0020);
cpumf_event_attr!(cf_fvn1, PROBLEM_STATE_INSTRUCTIONS, 0x0021);
cpumf_event_attr!(cf_fvn1, PROBLEM_STATE_L1I_DIR_WRITES, 0x0022);
cpumf_event_attr!(cf_fvn1, PROBLEM_STATE_L1I_PENALTY_CYCLES, 0x0023);
cpumf_event_attr!(cf_fvn1, PROBLEM_STATE_L1D_DIR_WRITES, 0x0024);
cpumf_event_attr!(cf_fvn1, PROBLEM_STATE_L1D_PENALTY_CYCLES, 0x0025);
cpumf_event_attr!(cf_fvn1, L1D_DIR_WRITES, 0x0004);
cpumf_event_attr!(cf_fvn1, L1D_PENALTY_CYCLES, 0x0005);
cpumf_event_attr!(cf_fvn3, CPU_CYCLES, 0x0000);
cpumf_event_attr!(cf_fvn3, INSTRUCTIONS, 0x0001);
cpumf_event_attr!(cf_fvn3, L1I_DIR_WRITES, 0x0002);
cpumf_event_attr!(cf_fvn3, L1I_PENALTY_CYCLES, 0x0003);
cpumf_event_attr!(cf_fvn3, PROBLEM_STATE_CPU_CYCLES, 0x0020);
cpumf_event_attr!(cf_fvn3, PROBLEM_STATE_INSTRUCTIONS, 0x0021);
cpumf_event_attr!(cf_fvn3, L1D_DIR_WRITES, 0x0004);
cpumf_event_attr!(cf_fvn3, L1D_PENALTY_CYCLES, 0x0005);
cpumf_event_attr!(cf_svn_12345, PRNG_FUNCTIONS, 0x0040);
cpumf_event_attr!(cf_svn_12345, PRNG_CYCLES, 0x0041);
cpumf_event_attr!(cf_svn_12345, PRNG_BLOCKED_FUNCTIONS, 0x0042);
cpumf_event_attr!(cf_svn_12345, PRNG_BLOCKED_CYCLES, 0x0043);
cpumf_event_attr!(cf_svn_12345, SHA_FUNCTIONS, 0x0044);
cpumf_event_attr!(cf_svn_12345, SHA_CYCLES, 0x0045);
cpumf_event_attr!(cf_svn_12345, SHA_BLOCKED_FUNCTIONS, 0x0046);
cpumf_event_attr!(cf_svn_12345, SHA_BLOCKED_CYCLES, 0x0047);
cpumf_event_attr!(cf_svn_12345, DEA_FUNCTIONS, 0x0048);
cpumf_event_attr!(cf_svn_12345, DEA_CYCLES, 0x0049);
cpumf_event_attr!(cf_svn_12345, DEA_BLOCKED_FUNCTIONS, 0x004a);
cpumf_event_attr!(cf_svn_12345, DEA_BLOCKED_CYCLES, 0x004b);
cpumf_event_attr!(cf_svn_12345, AES_FUNCTIONS, 0x004c);
cpumf_event_attr!(cf_svn_12345, AES_CYCLES, 0x004d);
cpumf_event_attr!(cf_svn_12345, AES_BLOCKED_FUNCTIONS, 0x004e);
cpumf_event_attr!(cf_svn_12345, AES_BLOCKED_CYCLES, 0x004f);
cpumf_event_attr!(cf_svn_6, ECC_FUNCTION_COUNT, 0x0050);
cpumf_event_attr!(cf_svn_6, ECC_CYCLES_COUNT, 0x0051);
cpumf_event_attr!(cf_svn_6, ECC_BLOCKED_FUNCTION_COUNT, 0x0052);
cpumf_event_attr!(cf_svn_6, ECC_BLOCKED_CYCLES_COUNT, 0x0053);
cpumf_event_attr!(cf_z10, L1I_L2_SOURCED_WRITES, 0x0080);
cpumf_event_attr!(cf_z10, L1D_L2_SOURCED_WRITES, 0x0081);
cpumf_event_attr!(cf_z10, L1I_L3_LOCAL_WRITES, 0x0082);
cpumf_event_attr!(cf_z10, L1D_L3_LOCAL_WRITES, 0x0083);
cpumf_event_attr!(cf_z10, L1I_L3_REMOTE_WRITES, 0x0084);
cpumf_event_attr!(cf_z10, L1D_L3_REMOTE_WRITES, 0x0085);
cpumf_event_attr!(cf_z10, L1D_LMEM_SOURCED_WRITES, 0x0086);
cpumf_event_attr!(cf_z10, L1I_LMEM_SOURCED_WRITES, 0x0087);
cpumf_event_attr!(cf_z10, L1D_RO_EXCL_WRITES, 0x0088);
cpumf_event_attr!(cf_z10, L1I_CACHELINE_INVALIDATES, 0x0089);
cpumf_event_attr!(cf_z10, ITLB1_WRITES, 0x008a);
cpumf_event_attr!(cf_z10, DTLB1_WRITES, 0x008b);
cpumf_event_attr!(cf_z10, TLB2_PTE_WRITES, 0x008c);
cpumf_event_attr!(cf_z10, TLB2_CRSTE_WRITES, 0x008d);
cpumf_event_attr!(cf_z10, TLB2_CRSTE_HPAGE_WRITES, 0x008e);
cpumf_event_attr!(cf_z10, ITLB1_MISSES, 0x0091);
cpumf_event_attr!(cf_z10, DTLB1_MISSES, 0x0092);
cpumf_event_attr!(cf_z10, L2C_STORES_SENT, 0x0093);
cpumf_event_attr!(cf_z196, L1D_L2_SOURCED_WRITES, 0x0080);
cpumf_event_attr!(cf_z196, L1I_L2_SOURCED_WRITES, 0x0081);
cpumf_event_attr!(cf_z196, DTLB1_MISSES, 0x0082);
cpumf_event_attr!(cf_z196, ITLB1_MISSES, 0x0083);
cpumf_event_attr!(cf_z196, L2C_STORES_SENT, 0x0085);
cpumf_event_attr!(cf_z196, L1D_OFFBOOK_L3_SOURCED_WRITES, 0x0086);
cpumf_event_attr!(cf_z196, L1D_ONBOOK_L4_SOURCED_WRITES, 0x0087);
cpumf_event_attr!(cf_z196, L1I_ONBOOK_L4_SOURCED_WRITES, 0x0088);
cpumf_event_attr!(cf_z196, L1D_RO_EXCL_WRITES, 0x0089);
cpumf_event_attr!(cf_z196, L1D_OFFBOOK_L4_SOURCED_WRITES, 0x008a);
cpumf_event_attr!(cf_z196, L1I_OFFBOOK_L4_SOURCED_WRITES, 0x008b);
cpumf_event_attr!(cf_z196, DTLB1_HPAGE_WRITES, 0x008c);
cpumf_event_attr!(cf_z196, L1D_LMEM_SOURCED_WRITES, 0x008d);
cpumf_event_attr!(cf_z196, L1I_LMEM_SOURCED_WRITES, 0x008e);
cpumf_event_attr!(cf_z196, L1I_OFFBOOK_L3_SOURCED_WRITES, 0x008f);
cpumf_event_attr!(cf_z196, DTLB1_WRITES, 0x0090);
cpumf_event_attr!(cf_z196, ITLB1_WRITES, 0x0091);
cpumf_event_attr!(cf_z196, TLB2_PTE_WRITES, 0x0092);
cpumf_event_attr!(cf_z196, TLB2_CRSTE_HPAGE_WRITES, 0x0093);
cpumf_event_attr!(cf_z196, TLB2_CRSTE_WRITES, 0x0094);
cpumf_event_attr!(cf_z196, L1D_ONCHIP_L3_SOURCED_WRITES, 0x0096);
cpumf_event_attr!(cf_z196, L1D_OFFCHIP_L3_SOURCED_WRITES, 0x0098);
cpumf_event_attr!(cf_z196, L1I_ONCHIP_L3_SOURCED_WRITES, 0x0099);
cpumf_event_attr!(cf_z196, L1I_OFFCHIP_L3_SOURCED_WRITES, 0x009b);
cpumf_event_attr!(cf_zec12, DTLB1_MISSES, 0x0080);
cpumf_event_attr!(cf_zec12, ITLB1_MISSES, 0x0081);
cpumf_event_attr!(cf_zec12, L1D_L2I_SOURCED_WRITES, 0x0082);
cpumf_event_attr!(cf_zec12, L1I_L2I_SOURCED_WRITES, 0x0083);
cpumf_event_attr!(cf_zec12, L1D_L2D_SOURCED_WRITES, 0x0084);
cpumf_event_attr!(cf_zec12, DTLB1_WRITES, 0x0085);
cpumf_event_attr!(cf_zec12, L1D_LMEM_SOURCED_WRITES, 0x0087);
cpumf_event_attr!(cf_zec12, L1I_LMEM_SOURCED_WRITES, 0x0089);
cpumf_event_attr!(cf_zec12, L1D_RO_EXCL_WRITES, 0x008a);
cpumf_event_attr!(cf_zec12, DTLB1_HPAGE_WRITES, 0x008b);
cpumf_event_attr!(cf_zec12, ITLB1_WRITES, 0x008c);
cpumf_event_attr!(cf_zec12, TLB2_PTE_WRITES, 0x008d);
cpumf_event_attr!(cf_zec12, TLB2_CRSTE_HPAGE_WRITES, 0x008e);
cpumf_event_attr!(cf_zec12, TLB2_CRSTE_WRITES, 0x008f);
cpumf_event_attr!(cf_zec12, L1D_ONCHIP_L3_SOURCED_WRITES, 0x0090);
cpumf_event_attr!(cf_zec12, L1D_OFFCHIP_L3_SOURCED_WRITES, 0x0091);
cpumf_event_attr!(cf_zec12, L1D_OFFBOOK_L3_SOURCED_WRITES, 0x0092);
cpumf_event_attr!(cf_zec12, L1D_ONBOOK_L4_SOURCED_WRITES, 0x0093);
cpumf_event_attr!(cf_zec12, L1D_OFFBOOK_L4_SOURCED_WRITES, 0x0094);
cpumf_event_attr!(cf_zec12, TX_NC_TEND, 0x0095);
cpumf_event_attr!(cf_zec12, L1D_ONCHIP_L3_SOURCED_WRITES_IV, 0x0096);
cpumf_event_attr!(cf_zec12, L1D_OFFCHIP_L3_SOURCED_WRITES_IV, 0x0097);
cpumf_event_attr!(cf_zec12, L1D_OFFBOOK_L3_SOURCED_WRITES_IV, 0x0098);
cpumf_event_attr!(cf_zec12, L1I_ONCHIP_L3_SOURCED_WRITES, 0x0099);
cpumf_event_attr!(cf_zec12, L1I_OFFCHIP_L3_SOURCED_WRITES, 0x009a);
cpumf_event_attr!(cf_zec12, L1I_OFFBOOK_L3_SOURCED_WRITES, 0x009b);
cpumf_event_attr!(cf_zec12, L1I_ONBOOK_L4_SOURCED_WRITES, 0x009c);
cpumf_event_attr!(cf_zec12, L1I_OFFBOOK_L4_SOURCED_WRITES, 0x009d);
cpumf_event_attr!(cf_zec12, TX_C_TEND, 0x009e);
cpumf_event_attr!(cf_zec12, L1I_ONCHIP_L3_SOURCED_WRITES_IV, 0x009f);
cpumf_event_attr!(cf_zec12, L1I_OFFCHIP_L3_SOURCED_WRITES_IV, 0x00a0);
cpumf_event_attr!(cf_zec12, L1I_OFFBOOK_L3_SOURCED_WRITES_IV, 0x00a1);
cpumf_event_attr!(cf_zec12, TX_NC_TABORT, 0x00b1);
cpumf_event_attr!(cf_zec12, TX_C_TABORT_NO_SPECIAL, 0x00b2);
cpumf_event_attr!(cf_zec12, TX_C_TABORT_SPECIAL, 0x00b3);
cpumf_event_attr!(cf_z13, L1D_RO_EXCL_WRITES, 0x0080);
cpumf_event_attr!(cf_z13, DTLB1_WRITES, 0x0081);
cpumf_event_attr!(cf_z13, DTLB1_MISSES, 0x0082);
cpumf_event_attr!(cf_z13, DTLB1_HPAGE_WRITES, 0x0083);
cpumf_event_attr!(cf_z13, DTLB1_GPAGE_WRITES, 0x0084);
cpumf_event_attr!(cf_z13, L1D_L2D_SOURCED_WRITES, 0x0085);
cpumf_event_attr!(cf_z13, ITLB1_WRITES, 0x0086);
cpumf_event_attr!(cf_z13, ITLB1_MISSES, 0x0087);
cpumf_event_attr!(cf_z13, L1I_L2I_SOURCED_WRITES, 0x0088);
cpumf_event_attr!(cf_z13, TLB2_PTE_WRITES, 0x0089);
cpumf_event_attr!(cf_z13, TLB2_CRSTE_HPAGE_WRITES, 0x008a);
cpumf_event_attr!(cf_z13, TLB2_CRSTE_WRITES, 0x008b);
cpumf_event_attr!(cf_z13, TX_C_TEND, 0x008c);
cpumf_event_attr!(cf_z13, TX_NC_TEND, 0x008d);
cpumf_event_attr!(cf_z13, L1C_TLB1_MISSES, 0x008f);
cpumf_event_attr!(cf_z13, L1D_ONCHIP_L3_SOURCED_WRITES, 0x0090);
cpumf_event_attr!(cf_z13, L1D_ONCHIP_L3_SOURCED_WRITES_IV, 0x0091);
cpumf_event_attr!(cf_z13, L1D_ONNODE_L4_SOURCED_WRITES, 0x0092);
cpumf_event_attr!(cf_z13, L1D_ONNODE_L3_SOURCED_WRITES_IV, 0x0093);
cpumf_event_attr!(cf_z13, L1D_ONNODE_L3_SOURCED_WRITES, 0x0094);
cpumf_event_attr!(cf_z13, L1D_ONDRAWER_L4_SOURCED_WRITES, 0x0095);
cpumf_event_attr!(cf_z13, L1D_ONDRAWER_L3_SOURCED_WRITES_IV, 0x0096);
cpumf_event_attr!(cf_z13, L1D_ONDRAWER_L3_SOURCED_WRITES, 0x0097);
cpumf_event_attr!(cf_z13, L1D_OFFDRAWER_SCOL_L4_SOURCED_WRITES, 0x0098);
cpumf_event_attr!(cf_z13, L1D_OFFDRAWER_SCOL_L3_SOURCED_WRITES_IV, 0x0099);
cpumf_event_attr!(cf_z13, L1D_OFFDRAWER_SCOL_L3_SOURCED_WRITES, 0x009a);
cpumf_event_attr!(cf_z13, L1D_OFFDRAWER_FCOL_L4_SOURCED_WRITES, 0x009b);
cpumf_event_attr!(cf_z13, L1D_OFFDRAWER_FCOL_L3_SOURCED_WRITES_IV, 0x009c);
cpumf_event_attr!(cf_z13, L1D_OFFDRAWER_FCOL_L3_SOURCED_WRITES, 0x009d);
cpumf_event_attr!(cf_z13, L1D_ONNODE_MEM_SOURCED_WRITES, 0x009e);
cpumf_event_attr!(cf_z13, L1D_ONDRAWER_MEM_SOURCED_WRITES, 0x009f);
cpumf_event_attr!(cf_z13, L1D_OFFDRAWER_MEM_SOURCED_WRITES, 0x00a0);
cpumf_event_attr!(cf_z13, L1D_ONCHIP_MEM_SOURCED_WRITES, 0x00a1);
cpumf_event_attr!(cf_z13, L1I_ONCHIP_L3_SOURCED_WRITES, 0x00a2);
cpumf_event_attr!(cf_z13, L1I_ONCHIP_L3_SOURCED_WRITES_IV, 0x00a3);
cpumf_event_attr!(cf_z13, L1I_ONNODE_L4_SOURCED_WRITES, 0x00a4);
cpumf_event_attr!(cf_z13, L1I_ONNODE_L3_SOURCED_WRITES_IV, 0x00a5);
cpumf_event_attr!(cf_z13, L1I_ONNODE_L3_SOURCED_WRITES, 0x00a6);
cpumf_event_attr!(cf_z13, L1I_ONDRAWER_L4_SOURCED_WRITES, 0x00a7);
cpumf_event_attr!(cf_z13, L1I_ONDRAWER_L3_SOURCED_WRITES_IV, 0x00a8);
cpumf_event_attr!(cf_z13, L1I_ONDRAWER_L3_SOURCED_WRITES, 0x00a9);
cpumf_event_attr!(cf_z13, L1I_OFFDRAWER_SCOL_L4_SOURCED_WRITES, 0x00aa);
cpumf_event_attr!(cf_z13, L1I_OFFDRAWER_SCOL_L3_SOURCED_WRITES_IV, 0x00ab);
cpumf_event_attr!(cf_z13, L1I_OFFDRAWER_SCOL_L3_SOURCED_WRITES, 0x00ac);
cpumf_event_attr!(cf_z13, L1I_OFFDRAWER_FCOL_L4_SOURCED_WRITES, 0x00ad);
cpumf_event_attr!(cf_z13, L1I_OFFDRAWER_FCOL_L3_SOURCED_WRITES_IV, 0x00ae);
cpumf_event_attr!(cf_z13, L1I_OFFDRAWER_FCOL_L3_SOURCED_WRITES, 0x00af);
cpumf_event_attr!(cf_z13, L1I_ONNODE_MEM_SOURCED_WRITES, 0x00b0);
cpumf_event_attr!(cf_z13, L1I_ONDRAWER_MEM_SOURCED_WRITES, 0x00b1);
cpumf_event_attr!(cf_z13, L1I_OFFDRAWER_MEM_SOURCED_WRITES, 0x00b2);
cpumf_event_attr!(cf_z13, L1I_ONCHIP_MEM_SOURCED_WRITES, 0x00b3);
cpumf_event_attr!(cf_z13, TX_NC_TABORT, 0x00da);
cpumf_event_attr!(cf_z13, TX_C_TABORT_NO_SPECIAL, 0x00db);
cpumf_event_attr!(cf_z13, TX_C_TABORT_SPECIAL, 0x00dc);
cpumf_event_attr!(cf_z13, MT_DIAG_CYCLES_ONE_THR_ACTIVE, 0x01c0);
cpumf_event_attr!(cf_z13, MT_DIAG_CYCLES_TWO_THR_ACTIVE, 0x01c1);
cpumf_event_attr!(cf_z14, L1D_RO_EXCL_WRITES, 0x0080);
cpumf_event_attr!(cf_z14, DTLB2_WRITES, 0x0081);
cpumf_event_attr!(cf_z14, DTLB2_MISSES, 0x0082);
cpumf_event_attr!(cf_z14, DTLB2_HPAGE_WRITES, 0x0083);
cpumf_event_attr!(cf_z14, DTLB2_GPAGE_WRITES, 0x0084);
cpumf_event_attr!(cf_z14, L1D_L2D_SOURCED_WRITES, 0x0085);
cpumf_event_attr!(cf_z14, ITLB2_WRITES, 0x0086);
cpumf_event_attr!(cf_z14, ITLB2_MISSES, 0x0087);
cpumf_event_attr!(cf_z14, L1I_L2I_SOURCED_WRITES, 0x0088);
cpumf_event_attr!(cf_z14, TLB2_PTE_WRITES, 0x0089);
cpumf_event_attr!(cf_z14, TLB2_CRSTE_WRITES, 0x008a);
cpumf_event_attr!(cf_z14, TLB2_ENGINES_BUSY, 0x008b);
cpumf_event_attr!(cf_z14, TX_C_TEND, 0x008c);
cpumf_event_attr!(cf_z14, TX_NC_TEND, 0x008d);
cpumf_event_attr!(cf_z14, L1C_TLB2_MISSES, 0x008f);
cpumf_event_attr!(cf_z14, L1D_ONCHIP_L3_SOURCED_WRITES, 0x0090);
cpumf_event_attr!(cf_z14, L1D_ONCHIP_MEMORY_SOURCED_WRITES, 0x0091);
cpumf_event_attr!(cf_z14, L1D_ONCHIP_L3_SOURCED_WRITES_IV, 0x0092);
cpumf_event_attr!(cf_z14, L1D_ONCLUSTER_L3_SOURCED_WRITES, 0x0093);
cpumf_event_attr!(cf_z14, L1D_ONCLUSTER_MEMORY_SOURCED_WRITES, 0x0094);
cpumf_event_attr!(cf_z14, L1D_ONCLUSTER_L3_SOURCED_WRITES_IV, 0x0095);
cpumf_event_attr!(cf_z14, L1D_OFFCLUSTER_L3_SOURCED_WRITES, 0x0096);
cpumf_event_attr!(cf_z14, L1D_OFFCLUSTER_MEMORY_SOURCED_WRITES, 0x0097);
cpumf_event_attr!(cf_z14, L1D_OFFCLUSTER_L3_SOURCED_WRITES_IV, 0x0098);
cpumf_event_attr!(cf_z14, L1D_OFFDRAWER_L3_SOURCED_WRITES, 0x0099);
cpumf_event_attr!(cf_z14, L1D_OFFDRAWER_MEMORY_SOURCED_WRITES, 0x009a);
cpumf_event_attr!(cf_z14, L1D_OFFDRAWER_L3_SOURCED_WRITES_IV, 0x009b);
cpumf_event_attr!(cf_z14, L1D_ONDRAWER_L4_SOURCED_WRITES, 0x009c);
cpumf_event_attr!(cf_z14, L1D_OFFDRAWER_L4_SOURCED_WRITES, 0x009d);
cpumf_event_attr!(cf_z14, L1D_ONCHIP_L3_SOURCED_WRITES_RO, 0x009e);
cpumf_event_attr!(cf_z14, L1I_ONCHIP_L3_SOURCED_WRITES, 0x00a2);
cpumf_event_attr!(cf_z14, L1I_ONCHIP_MEMORY_SOURCED_WRITES, 0x00a3);
cpumf_event_attr!(cf_z14, L1I_ONCHIP_L3_SOURCED_WRITES_IV, 0x00a4);
cpumf_event_attr!(cf_z14, L1I_ONCLUSTER_L3_SOURCED_WRITES, 0x00a5);
cpumf_event_attr!(cf_z14, L1I_ONCLUSTER_MEMORY_SOURCED_WRITES, 0x00a6);
cpumf_event_attr!(cf_z14, L1I_ONCLUSTER_L3_SOURCED_WRITES_IV, 0x00a7);
cpumf_event_attr!(cf_z14, L1I_OFFCLUSTER_L3_SOURCED_WRITES, 0x00a8);
cpumf_event_attr!(cf_z14, L1I_OFFCLUSTER_MEMORY_SOURCED_WRITES, 0x00a9);
cpumf_event_attr!(cf_z14, L1I_OFFCLUSTER_L3_SOURCED_WRITES_IV, 0x00aa);
cpumf_event_attr!(cf_z14, L1I_OFFDRAWER_L3_SOURCED_WRITES, 0x00ab);
cpumf_event_attr!(cf_z14, L1I_OFFDRAWER_MEMORY_SOURCED_WRITES, 0x00ac);
cpumf_event_attr!(cf_z14, L1I_OFFDRAWER_L3_SOURCED_WRITES_IV, 0x00ad);
cpumf_event_attr!(cf_z14, L1I_ONDRAWER_L4_SOURCED_WRITES, 0x00ae);
cpumf_event_attr!(cf_z14, L1I_OFFDRAWER_L4_SOURCED_WRITES, 0x00af);
cpumf_event_attr!(cf_z14, BCD_DFP_EXECUTION_SLOTS, 0x00e0);
cpumf_event_attr!(cf_z14, VX_BCD_EXECUTION_SLOTS, 0x00e1);
cpumf_event_attr!(cf_z14, DECIMAL_INSTRUCTIONS, 0x00e2);
cpumf_event_attr!(cf_z14, LAST_HOST_TRANSLATIONS, 0x00e8);
cpumf_event_attr!(cf_z14, TX_NC_TABORT, 0x00f3);
cpumf_event_attr!(cf_z14, TX_C_TABORT_NO_SPECIAL, 0x00f4);
cpumf_event_attr!(cf_z14, TX_C_TABORT_SPECIAL, 0x00f5);
cpumf_event_attr!(cf_z14, MT_DIAG_CYCLES_ONE_THR_ACTIVE, 0x01c0);
cpumf_event_attr!(cf_z14, MT_DIAG_CYCLES_TWO_THR_ACTIVE, 0x01c1);
cpumf_event_attr!(cf_z15, L1D_RO_EXCL_WRITES, 0x0080);
cpumf_event_attr!(cf_z15, DTLB2_WRITES, 0x0081);
cpumf_event_attr!(cf_z15, DTLB2_MISSES, 0x0082);
cpumf_event_attr!(cf_z15, DTLB2_HPAGE_WRITES, 0x0083);
cpumf_event_attr!(cf_z15, DTLB2_GPAGE_WRITES, 0x0084);
cpumf_event_attr!(cf_z15, L1D_L2D_SOURCED_WRITES, 0x0085);
cpumf_event_attr!(cf_z15, ITLB2_WRITES, 0x0086);
cpumf_event_attr!(cf_z15, ITLB2_MISSES, 0x0087);
cpumf_event_attr!(cf_z15, L1I_L2I_SOURCED_WRITES, 0x0088);
cpumf_event_attr!(cf_z15, TLB2_PTE_WRITES, 0x0089);
cpumf_event_attr!(cf_z15, TLB2_CRSTE_WRITES, 0x008a);
cpumf_event_attr!(cf_z15, TLB2_ENGINES_BUSY, 0x008b);
cpumf_event_attr!(cf_z15, TX_C_TEND, 0x008c);
cpumf_event_attr!(cf_z15, TX_NC_TEND, 0x008d);
cpumf_event_attr!(cf_z15, L1C_TLB2_MISSES, 0x008f);
cpumf_event_attr!(cf_z15, L1D_ONCHIP_L3_SOURCED_WRITES, 0x0090);
cpumf_event_attr!(cf_z15, L1D_ONCHIP_MEMORY_SOURCED_WRITES, 0x0091);
cpumf_event_attr!(cf_z15, L1D_ONCHIP_L3_SOURCED_WRITES_IV, 0x0092);
cpumf_event_attr!(cf_z15, L1D_ONCLUSTER_L3_SOURCED_WRITES, 0x0093);
cpumf_event_attr!(cf_z15, L1D_ONCLUSTER_MEMORY_SOURCED_WRITES, 0x0094);
cpumf_event_attr!(cf_z15, L1D_ONCLUSTER_L3_SOURCED_WRITES_IV, 0x0095);
cpumf_event_attr!(cf_z15, L1D_OFFCLUSTER_L3_SOURCED_WRITES, 0x0096);
cpumf_event_attr!(cf_z15, L1D_OFFCLUSTER_MEMORY_SOURCED_WRITES, 0x0097);
cpumf_event_attr!(cf_z15, L1D_OFFCLUSTER_L3_SOURCED_WRITES_IV, 0x0098);
cpumf_event_attr!(cf_z15, L1D_OFFDRAWER_L3_SOURCED_WRITES, 0x0099);
cpumf_event_attr!(cf_z15, L1D_OFFDRAWER_MEMORY_SOURCED_WRITES, 0x009a);
cpumf_event_attr!(cf_z15, L1D_OFFDRAWER_L3_SOURCED_WRITES_IV, 0x009b);
cpumf_event_attr!(cf_z15, L1D_ONDRAWER_L4_SOURCED_WRITES, 0x009c);
cpumf_event_attr!(cf_z15, L1D_OFFDRAWER_L4_SOURCED_WRITES, 0x009d);
cpumf_event_attr!(cf_z15, L1D_ONCHIP_L3_SOURCED_WRITES_RO, 0x009e);
cpumf_event_attr!(cf_z15, L1I_ONCHIP_L3_SOURCED_WRITES, 0x00a2);
cpumf_event_attr!(cf_z15, L1I_ONCHIP_MEMORY_SOURCED_WRITES, 0x00a3);
cpumf_event_attr!(cf_z15, L1I_ONCHIP_L3_SOURCED_WRITES_IV, 0x00a4);
cpumf_event_attr!(cf_z15, L1I_ONCLUSTER_L3_SOURCED_WRITES, 0x00a5);
cpumf_event_attr!(cf_z15, L1I_ONCLUSTER_MEMORY_SOURCED_WRITES, 0x00a6);
cpumf_event_attr!(cf_z15, L1I_ONCLUSTER_L3_SOURCED_WRITES_IV, 0x00a7);
cpumf_event_attr!(cf_z15, L1I_OFFCLUSTER_L3_SOURCED_WRITES, 0x00a8);
cpumf_event_attr!(cf_z15, L1I_OFFCLUSTER_MEMORY_SOURCED_WRITES, 0x00a9);
cpumf_event_attr!(cf_z15, L1I_OFFCLUSTER_L3_SOURCED_WRITES_IV, 0x00aa);
cpumf_event_attr!(cf_z15, L1I_OFFDRAWER_L3_SOURCED_WRITES, 0x00ab);
cpumf_event_attr!(cf_z15, L1I_OFFDRAWER_MEMORY_SOURCED_WRITES, 0x00ac);
cpumf_event_attr!(cf_z15, L1I_OFFDRAWER_L3_SOURCED_WRITES_IV, 0x00ad);
cpumf_event_attr!(cf_z15, L1I_ONDRAWER_L4_SOURCED_WRITES, 0x00ae);
cpumf_event_attr!(cf_z15, L1I_OFFDRAWER_L4_SOURCED_WRITES, 0x00af);
cpumf_event_attr!(cf_z15, BCD_DFP_EXECUTION_SLOTS, 0x00e0);
cpumf_event_attr!(cf_z15, VX_BCD_EXECUTION_SLOTS, 0x00e1);
cpumf_event_attr!(cf_z15, DECIMAL_INSTRUCTIONS, 0x00e2);
cpumf_event_attr!(cf_z15, LAST_HOST_TRANSLATIONS, 0x00e8);
cpumf_event_attr!(cf_z15, TX_NC_TABORT, 0x00f3);
cpumf_event_attr!(cf_z15, TX_C_TABORT_NO_SPECIAL, 0x00f4);
cpumf_event_attr!(cf_z15, TX_C_TABORT_SPECIAL, 0x00f5);
cpumf_event_attr!(cf_z15, DFLT_ACCESS, 0x00f7);
cpumf_event_attr!(cf_z15, DFLT_CYCLES, 0x00fc);
cpumf_event_attr!(cf_z15, DFLT_CC, 0x0108);
cpumf_event_attr!(cf_z15, DFLT_CCFINISH, 0x0109);
cpumf_event_attr!(cf_z15, MT_DIAG_CYCLES_ONE_THR_ACTIVE, 0x01c0);
cpumf_event_attr!(cf_z15, MT_DIAG_CYCLES_TWO_THR_ACTIVE, 0x01c1);
cpumf_event_attr!(cf_z16, L1D_RO_EXCL_WRITES, 0x0080);
cpumf_event_attr!(cf_z16, DTLB2_WRITES, 0x0081);
cpumf_event_attr!(cf_z16, DTLB2_MISSES, 0x0082);
cpumf_event_attr!(cf_z16, CRSTE_1MB_WRITES, 0x0083);
cpumf_event_attr!(cf_z16, DTLB2_GPAGE_WRITES, 0x0084);
cpumf_event_attr!(cf_z16, ITLB2_WRITES, 0x0086);
cpumf_event_attr!(cf_z16, ITLB2_MISSES, 0x0087);
cpumf_event_attr!(cf_z16, TLB2_PTE_WRITES, 0x0089);
cpumf_event_attr!(cf_z16, TLB2_CRSTE_WRITES, 0x008a);
cpumf_event_attr!(cf_z16, TLB2_ENGINES_BUSY, 0x008b);
cpumf_event_attr!(cf_z16, TX_C_TEND, 0x008c);
cpumf_event_attr!(cf_z16, TX_NC_TEND, 0x008d);
cpumf_event_attr!(cf_z16, L1C_TLB2_MISSES, 0x008f);
cpumf_event_attr!(cf_z16, DCW_REQ, 0x0091);
cpumf_event_attr!(cf_z16, DCW_REQ_IV, 0x0092);
cpumf_event_attr!(cf_z16, DCW_REQ_CHIP_HIT, 0x0093);
cpumf_event_attr!(cf_z16, DCW_REQ_DRAWER_HIT, 0x0094);
cpumf_event_attr!(cf_z16, DCW_ON_CHIP, 0x0095);
cpumf_event_attr!(cf_z16, DCW_ON_CHIP_IV, 0x0096);
cpumf_event_attr!(cf_z16, DCW_ON_CHIP_CHIP_HIT, 0x0097);
cpumf_event_attr!(cf_z16, DCW_ON_CHIP_DRAWER_HIT, 0x0098);
cpumf_event_attr!(cf_z16, DCW_ON_MODULE, 0x0099);
cpumf_event_attr!(cf_z16, DCW_ON_DRAWER, 0x009a);
cpumf_event_attr!(cf_z16, DCW_OFF_DRAWER, 0x009b);
cpumf_event_attr!(cf_z16, DCW_ON_CHIP_MEMORY, 0x009c);
cpumf_event_attr!(cf_z16, DCW_ON_MODULE_MEMORY, 0x009d);
cpumf_event_attr!(cf_z16, DCW_ON_DRAWER_MEMORY, 0x009e);
cpumf_event_attr!(cf_z16, DCW_OFF_DRAWER_MEMORY, 0x009f);
cpumf_event_attr!(cf_z16, IDCW_ON_MODULE_IV, 0x00a0);
cpumf_event_attr!(cf_z16, IDCW_ON_MODULE_CHIP_HIT, 0x00a1);
cpumf_event_attr!(cf_z16, IDCW_ON_MODULE_DRAWER_HIT, 0x00a2);
cpumf_event_attr!(cf_z16, IDCW_ON_DRAWER_IV, 0x00a3);
cpumf_event_attr!(cf_z16, IDCW_ON_DRAWER_CHIP_HIT, 0x00a4);
cpumf_event_attr!(cf_z16, IDCW_ON_DRAWER_DRAWER_HIT, 0x00a5);
cpumf_event_attr!(cf_z16, IDCW_OFF_DRAWER_IV, 0x00a6);
cpumf_event_attr!(cf_z16, IDCW_OFF_DRAWER_CHIP_HIT, 0x00a7);
cpumf_event_attr!(cf_z16, IDCW_OFF_DRAWER_DRAWER_HIT, 0x00a8);
cpumf_event_attr!(cf_z16, ICW_REQ, 0x00a9);
cpumf_event_attr!(cf_z16, ICW_REQ_IV, 0x00aa);
cpumf_event_attr!(cf_z16, ICW_REQ_CHIP_HIT, 0x00ab);
cpumf_event_attr!(cf_z16, ICW_REQ_DRAWER_HIT, 0x00ac);
cpumf_event_attr!(cf_z16, ICW_ON_CHIP, 0x00ad);
cpumf_event_attr!(cf_z16, ICW_ON_CHIP_IV, 0x00ae);
cpumf_event_attr!(cf_z16, ICW_ON_CHIP_CHIP_HIT, 0x00af);
cpumf_event_attr!(cf_z16, ICW_ON_CHIP_DRAWER_HIT, 0x00b0);
cpumf_event_attr!(cf_z16, ICW_ON_MODULE, 0x00b1);
cpumf_event_attr!(cf_z16, ICW_ON_DRAWER, 0x00b2);
cpumf_event_attr!(cf_z16, ICW_OFF_DRAWER, 0x00b3);
cpumf_event_attr!(cf_z16, ICW_ON_CHIP_MEMORY, 0x00b4);
cpumf_event_attr!(cf_z16, ICW_ON_MODULE_MEMORY, 0x00b5);
cpumf_event_attr!(cf_z16, ICW_ON_DRAWER_MEMORY, 0x00b6);
cpumf_event_attr!(cf_z16, ICW_OFF_DRAWER_MEMORY, 0x00b7);
cpumf_event_attr!(cf_z16, BCD_DFP_EXECUTION_SLOTS, 0x00e0);
cpumf_event_attr!(cf_z16, VX_BCD_EXECUTION_SLOTS, 0x00e1);
cpumf_event_attr!(cf_z16, DECIMAL_INSTRUCTIONS, 0x00e2);
cpumf_event_attr!(cf_z16, LAST_HOST_TRANSLATIONS, 0x00e8);
cpumf_event_attr!(cf_z16, TX_NC_TABORT, 0x00f4);
cpumf_event_attr!(cf_z16, TX_C_TABORT_NO_SPECIAL, 0x00f5);
cpumf_event_attr!(cf_z16, TX_C_TABORT_SPECIAL, 0x00f6);
cpumf_event_attr!(cf_z16, DFLT_ACCESS, 0x00f8);
cpumf_event_attr!(cf_z16, DFLT_CYCLES, 0x00fd);
cpumf_event_attr!(cf_z16, SORTL, 0x0100);
cpumf_event_attr!(cf_z16, DFLT_CC, 0x0109);
cpumf_event_attr!(cf_z16, DFLT_CCFINISH, 0x010a);
cpumf_event_attr!(cf_z16, NNPA_INVOCATIONS, 0x010b);
cpumf_event_attr!(cf_z16, NNPA_COMPLETIONS, 0x010c);
cpumf_event_attr!(cf_z16, NNPA_WAIT_LOCK, 0x010d);
cpumf_event_attr!(cf_z16, NNPA_HOLD_LOCK, 0x010e);
cpumf_event_attr!(cf_z16, MT_DIAG_CYCLES_ONE_THR_ACTIVE, 0x01c0);
cpumf_event_attr!(cf_z16, MT_DIAG_CYCLES_TWO_THR_ACTIVE, 0x01c1);
cpumf_event_attr!(cf_z17, L1D_RO_EXCL_WRITES, 0x0080);
cpumf_event_attr!(cf_z17, DTLB2_WRITES, 0x0081);
cpumf_event_attr!(cf_z17, DTLB2_MISSES, 0x0082);
cpumf_event_attr!(cf_z17, CRSTE_1MB_WRITES, 0x0083);
cpumf_event_attr!(cf_z17, DTLB2_GPAGE_WRITES, 0x0084);
cpumf_event_attr!(cf_z17, ITLB2_WRITES, 0x0086);
cpumf_event_attr!(cf_z17, ITLB2_MISSES, 0x0087);
cpumf_event_attr!(cf_z17, TLB2_PTE_WRITES, 0x0089);
cpumf_event_attr!(cf_z17, TLB2_CRSTE_WRITES, 0x008a);
cpumf_event_attr!(cf_z17, TLB2_ENGINES_BUSY, 0x008b);
cpumf_event_attr!(cf_z17, TX_C_TEND, 0x008c);
cpumf_event_attr!(cf_z17, TX_NC_TEND, 0x008d);
cpumf_event_attr!(cf_z17, L1C_TLB2_MISSES, 0x008f);
cpumf_event_attr!(cf_z17, DCW_REQ, 0x0091);
cpumf_event_attr!(cf_z17, DCW_REQ_IV, 0x0092);
cpumf_event_attr!(cf_z17, DCW_REQ_CHIP_HIT, 0x0093);
cpumf_event_attr!(cf_z17, DCW_REQ_DRAWER_HIT, 0x0094);
cpumf_event_attr!(cf_z17, DCW_ON_CHIP, 0x0095);
cpumf_event_attr!(cf_z17, DCW_ON_CHIP_IV, 0x0096);
cpumf_event_attr!(cf_z17, DCW_ON_CHIP_CHIP_HIT, 0x0097);
cpumf_event_attr!(cf_z17, DCW_ON_CHIP_DRAWER_HIT, 0x0098);
cpumf_event_attr!(cf_z17, DCW_ON_MODULE, 0x0099);
cpumf_event_attr!(cf_z17, DCW_ON_DRAWER, 0x009a);
cpumf_event_attr!(cf_z17, DCW_OFF_DRAWER, 0x009b);
cpumf_event_attr!(cf_z17, DCW_ON_CHIP_MEMORY, 0x009c);
cpumf_event_attr!(cf_z17, DCW_ON_MODULE_MEMORY, 0x009d);
cpumf_event_attr!(cf_z17, DCW_ON_DRAWER_MEMORY, 0x009e);
cpumf_event_attr!(cf_z17, DCW_OFF_DRAWER_MEMORY, 0x009f);
cpumf_event_attr!(cf_z17, IDCW_ON_MODULE_IV, 0x00a0);
cpumf_event_attr!(cf_z17, IDCW_ON_MODULE_CHIP_HIT, 0x00a1);
cpumf_event_attr!(cf_z17, IDCW_ON_MODULE_DRAWER_HIT, 0x00a2);
cpumf_event_attr!(cf_z17, IDCW_ON_DRAWER_IV, 0x00a3);
cpumf_event_attr!(cf_z17, IDCW_ON_DRAWER_CHIP_HIT, 0x00a4);
cpumf_event_attr!(cf_z17, IDCW_ON_DRAWER_DRAWER_HIT, 0x00a5);
cpumf_event_attr!(cf_z17, IDCW_OFF_DRAWER_IV, 0x00a6);
cpumf_event_attr!(cf_z17, IDCW_OFF_DRAWER_CHIP_HIT, 0x00a7);
cpumf_event_attr!(cf_z17, IDCW_OFF_DRAWER_DRAWER_HIT, 0x00a8);
cpumf_event_attr!(cf_z17, ICW_REQ, 0x00a9);
cpumf_event_attr!(cf_z17, ICW_REQ_IV, 0x00aa);
cpumf_event_attr!(cf_z17, ICW_REQ_CHIP_HIT, 0x00ab);
cpumf_event_attr!(cf_z17, ICW_REQ_DRAWER_HIT, 0x00ac);
cpumf_event_attr!(cf_z17, ICW_ON_CHIP, 0x00ad);
cpumf_event_attr!(cf_z17, ICW_ON_CHIP_IV, 0x00ae);
cpumf_event_attr!(cf_z17, ICW_ON_CHIP_CHIP_HIT, 0x00af);
cpumf_event_attr!(cf_z17, ICW_ON_CHIP_DRAWER_HIT, 0x00b0);
cpumf_event_attr!(cf_z17, ICW_ON_MODULE, 0x00b1);
cpumf_event_attr!(cf_z17, ICW_ON_DRAWER, 0x00b2);
cpumf_event_attr!(cf_z17, ICW_OFF_DRAWER, 0x00b3);
cpumf_event_attr!(cf_z17, CYCLES_SAMETHRD, 0x00ca);
cpumf_event_attr!(cf_z17, CYCLES_DIFFTHRD, 0x00cb);
cpumf_event_attr!(cf_z17, INST_SAMETHRD, 0x00cc);
cpumf_event_attr!(cf_z17, INST_DIFFTHRD, 0x00cd);
cpumf_event_attr!(cf_z17, WRONG_BRANCH_PREDICTION, 0x00ce);
cpumf_event_attr!(cf_z17, VX_BCD_EXECUTION_SLOTS, 0x00e1);
cpumf_event_attr!(cf_z17, DECIMAL_INSTRUCTIONS, 0x00e2);
cpumf_event_attr!(cf_z17, LAST_HOST_TRANSLATIONS, 0x00e8);
cpumf_event_attr!(cf_z17, TX_NC_TABORT, 0x00f4);
cpumf_event_attr!(cf_z17, TX_C_TABORT_NO_SPECIAL, 0x00f5);
cpumf_event_attr!(cf_z17, TX_C_TABORT_SPECIAL, 0x00f6);
cpumf_event_attr!(cf_z17, DFLT_ACCESS, 0x00f8);
cpumf_event_attr!(cf_z17, DFLT_CYCLES, 0x00fd);
cpumf_event_attr!(cf_z17, SORTL, 0x0100);
cpumf_event_attr!(cf_z17, DFLT_CC, 0x0109);
cpumf_event_attr!(cf_z17, DFLT_CCFINISH, 0x010a);
cpumf_event_attr!(cf_z17, NNPA_INVOCATIONS, 0x010b);
cpumf_event_attr!(cf_z17, NNPA_COMPLETIONS, 0x010c);
cpumf_event_attr!(cf_z17, NNPA_WAIT_LOCK, 0x010d);
cpumf_event_attr!(cf_z17, NNPA_HOLD_LOCK, 0x010e);
cpumf_event_attr!(cf_z17, NNPA_INST_ONCHIP, 0x0110);
cpumf_event_attr!(cf_z17, NNPA_INST_OFFCHIP, 0x0111);
cpumf_event_attr!(cf_z17, NNPA_INST_DIFF, 0x0112);
cpumf_event_attr!(cf_z17, NNPA_4K_PREFETCH, 0x0114);
cpumf_event_attr!(cf_z17, NNPA_COMPL_LOCK, 0x0115);
cpumf_event_attr!(cf_z17, NNPA_RETRY_LOCK, 0x0116);
cpumf_event_attr!(cf_z17, NNPA_RETRY_LOCK_WITH_PLO, 0x0117);
cpumf_event_attr!(cf_z17, MT_DIAG_CYCLES_ONE_THR_ACTIVE, 0x01c0);
cpumf_event_attr!(cf_z17, MT_DIAG_CYCLES_TWO_THR_ACTIVE, 0x01c1);

static mut cpumcf_fvn1_pmu_event_attr: &[*mut attribute] = &[
	cpumf_event_ptr!(cf_fvn1, CPU_CYCLES),
	cpumf_event_ptr!(cf_fvn1, INSTRUCTIONS),
	cpumf_event_ptr!(cf_fvn1, L1I_DIR_WRITES),
	cpumf_event_ptr!(cf_fvn1, L1I_PENALTY_CYCLES),
	cpumf_event_ptr!(cf_fvn1, PROBLEM_STATE_CPU_CYCLES),
	cpumf_event_ptr!(cf_fvn1, PROBLEM_STATE_INSTRUCTIONS),
	cpumf_event_ptr!(cf_fvn1, PROBLEM_STATE_L1I_DIR_WRITES),
	cpumf_event_ptr!(cf_fvn1, PROBLEM_STATE_L1I_PENALTY_CYCLES),
	cpumf_event_ptr!(cf_fvn1, PROBLEM_STATE_L1D_DIR_WRITES),
	cpumf_event_ptr!(cf_fvn1, PROBLEM_STATE_L1D_PENALTY_CYCLES),
	cpumf_event_ptr!(cf_fvn1, L1D_DIR_WRITES),
	cpumf_event_ptr!(cf_fvn1, L1D_PENALTY_CYCLES),
	core::ptr::null_mut(),
};

static mut cpumcf_fvn3_pmu_event_attr: &[*mut attribute] = &[
	cpumf_event_ptr!(cf_fvn3, CPU_CYCLES),
	cpumf_event_ptr!(cf_fvn3, INSTRUCTIONS),
	cpumf_event_ptr!(cf_fvn3, L1I_DIR_WRITES),
	cpumf_event_ptr!(cf_fvn3, L1I_PENALTY_CYCLES),
	cpumf_event_ptr!(cf_fvn3, PROBLEM_STATE_CPU_CYCLES),
	cpumf_event_ptr!(cf_fvn3, PROBLEM_STATE_INSTRUCTIONS),
	cpumf_event_ptr!(cf_fvn3, L1D_DIR_WRITES),
	cpumf_event_ptr!(cf_fvn3, L1D_PENALTY_CYCLES),
	core::ptr::null_mut(),
};

static mut cpumcf_svn_12345_pmu_event_attr: &[*mut attribute] = &[
	cpumf_event_ptr!(cf_svn_12345, PRNG_FUNCTIONS),
	cpumf_event_ptr!(cf_svn_12345, PRNG_CYCLES),
	cpumf_event_ptr!(cf_svn_12345, PRNG_BLOCKED_FUNCTIONS),
	cpumf_event_ptr!(cf_svn_12345, PRNG_BLOCKED_CYCLES),
	cpumf_event_ptr!(cf_svn_12345, SHA_FUNCTIONS),
	cpumf_event_ptr!(cf_svn_12345, SHA_CYCLES),
	cpumf_event_ptr!(cf_svn_12345, SHA_BLOCKED_FUNCTIONS),
	cpumf_event_ptr!(cf_svn_12345, SHA_BLOCKED_CYCLES),
	cpumf_event_ptr!(cf_svn_12345, DEA_FUNCTIONS),
	cpumf_event_ptr!(cf_svn_12345, DEA_CYCLES),
	cpumf_event_ptr!(cf_svn_12345, DEA_BLOCKED_FUNCTIONS),
	cpumf_event_ptr!(cf_svn_12345, DEA_BLOCKED_CYCLES),
	cpumf_event_ptr!(cf_svn_12345, AES_FUNCTIONS),
	cpumf_event_ptr!(cf_svn_12345, AES_CYCLES),
	cpumf_event_ptr!(cf_svn_12345, AES_BLOCKED_FUNCTIONS),
	cpumf_event_ptr!(cf_svn_12345, AES_BLOCKED_CYCLES),
	core::ptr::null_mut(),
};

static mut cpumcf_svn_678_pmu_event_attr: &[*mut attribute] = &[
	cpumf_event_ptr!(cf_svn_12345, PRNG_FUNCTIONS),
	cpumf_event_ptr!(cf_svn_12345, PRNG_CYCLES),
	cpumf_event_ptr!(cf_svn_12345, PRNG_BLOCKED_FUNCTIONS),
	cpumf_event_ptr!(cf_svn_12345, PRNG_BLOCKED_CYCLES),
	cpumf_event_ptr!(cf_svn_12345, SHA_FUNCTIONS),
	cpumf_event_ptr!(cf_svn_12345, SHA_CYCLES),
	cpumf_event_ptr!(cf_svn_12345, SHA_BLOCKED_FUNCTIONS),
	cpumf_event_ptr!(cf_svn_12345, SHA_BLOCKED_CYCLES),
	cpumf_event_ptr!(cf_svn_12345, DEA_FUNCTIONS),
	cpumf_event_ptr!(cf_svn_12345, DEA_CYCLES),
	cpumf_event_ptr!(cf_svn_12345, DEA_BLOCKED_FUNCTIONS),
	cpumf_event_ptr!(cf_svn_12345, DEA_BLOCKED_CYCLES),
	cpumf_event_ptr!(cf_svn_12345, AES_FUNCTIONS),
	cpumf_event_ptr!(cf_svn_12345, AES_CYCLES),
	cpumf_event_ptr!(cf_svn_12345, AES_BLOCKED_FUNCTIONS),
	cpumf_event_ptr!(cf_svn_12345, AES_BLOCKED_CYCLES),
	cpumf_event_ptr!(cf_svn_6, ECC_FUNCTION_COUNT),
	cpumf_event_ptr!(cf_svn_6, ECC_CYCLES_COUNT),
	cpumf_event_ptr!(cf_svn_6, ECC_BLOCKED_FUNCTION_COUNT),
	cpumf_event_ptr!(cf_svn_6, ECC_BLOCKED_CYCLES_COUNT),
	core::ptr::null_mut(),
};

static mut cpumcf_z10_pmu_event_attr: &[*mut attribute] = &[
	cpumf_event_ptr!(cf_z10, L1I_L2_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z10, L1D_L2_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z10, L1I_L3_LOCAL_WRITES),
	cpumf_event_ptr!(cf_z10, L1D_L3_LOCAL_WRITES),
	cpumf_event_ptr!(cf_z10, L1I_L3_REMOTE_WRITES),
	cpumf_event_ptr!(cf_z10, L1D_L3_REMOTE_WRITES),
	cpumf_event_ptr!(cf_z10, L1D_LMEM_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z10, L1I_LMEM_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z10, L1D_RO_EXCL_WRITES),
	cpumf_event_ptr!(cf_z10, L1I_CACHELINE_INVALIDATES),
	cpumf_event_ptr!(cf_z10, ITLB1_WRITES),
	cpumf_event_ptr!(cf_z10, DTLB1_WRITES),
	cpumf_event_ptr!(cf_z10, TLB2_PTE_WRITES),
	cpumf_event_ptr!(cf_z10, TLB2_CRSTE_WRITES),
	cpumf_event_ptr!(cf_z10, TLB2_CRSTE_HPAGE_WRITES),
	cpumf_event_ptr!(cf_z10, ITLB1_MISSES),
	cpumf_event_ptr!(cf_z10, DTLB1_MISSES),
	cpumf_event_ptr!(cf_z10, L2C_STORES_SENT),
	core::ptr::null_mut(),
};

static mut cpumcf_z196_pmu_event_attr: &[*mut attribute] = &[
	cpumf_event_ptr!(cf_z196, L1D_L2_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z196, L1I_L2_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z196, DTLB1_MISSES),
	cpumf_event_ptr!(cf_z196, ITLB1_MISSES),
	cpumf_event_ptr!(cf_z196, L2C_STORES_SENT),
	cpumf_event_ptr!(cf_z196, L1D_OFFBOOK_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z196, L1D_ONBOOK_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z196, L1I_ONBOOK_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z196, L1D_RO_EXCL_WRITES),
	cpumf_event_ptr!(cf_z196, L1D_OFFBOOK_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z196, L1I_OFFBOOK_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z196, DTLB1_HPAGE_WRITES),
	cpumf_event_ptr!(cf_z196, L1D_LMEM_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z196, L1I_LMEM_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z196, L1I_OFFBOOK_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z196, DTLB1_WRITES),
	cpumf_event_ptr!(cf_z196, ITLB1_WRITES),
	cpumf_event_ptr!(cf_z196, TLB2_PTE_WRITES),
	cpumf_event_ptr!(cf_z196, TLB2_CRSTE_HPAGE_WRITES),
	cpumf_event_ptr!(cf_z196, TLB2_CRSTE_WRITES),
	cpumf_event_ptr!(cf_z196, L1D_ONCHIP_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z196, L1D_OFFCHIP_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z196, L1I_ONCHIP_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z196, L1I_OFFCHIP_L3_SOURCED_WRITES),
	core::ptr::null_mut(),
};

static mut cpumcf_zec12_pmu_event_attr: &[*mut attribute] = &[
	cpumf_event_ptr!(cf_zec12, DTLB1_MISSES),
	cpumf_event_ptr!(cf_zec12, ITLB1_MISSES),
	cpumf_event_ptr!(cf_zec12, L1D_L2I_SOURCED_WRITES),
	cpumf_event_ptr!(cf_zec12, L1I_L2I_SOURCED_WRITES),
	cpumf_event_ptr!(cf_zec12, L1D_L2D_SOURCED_WRITES),
	cpumf_event_ptr!(cf_zec12, DTLB1_WRITES),
	cpumf_event_ptr!(cf_zec12, L1D_LMEM_SOURCED_WRITES),
	cpumf_event_ptr!(cf_zec12, L1I_LMEM_SOURCED_WRITES),
	cpumf_event_ptr!(cf_zec12, L1D_RO_EXCL_WRITES),
	cpumf_event_ptr!(cf_zec12, DTLB1_HPAGE_WRITES),
	cpumf_event_ptr!(cf_zec12, ITLB1_WRITES),
	cpumf_event_ptr!(cf_zec12, TLB2_PTE_WRITES),
	cpumf_event_ptr!(cf_zec12, TLB2_CRSTE_HPAGE_WRITES),
	cpumf_event_ptr!(cf_zec12, TLB2_CRSTE_WRITES),
	cpumf_event_ptr!(cf_zec12, L1D_ONCHIP_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_zec12, L1D_OFFCHIP_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_zec12, L1D_OFFBOOK_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_zec12, L1D_ONBOOK_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_zec12, L1D_OFFBOOK_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_zec12, TX_NC_TEND),
	cpumf_event_ptr!(cf_zec12, L1D_ONCHIP_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_zec12, L1D_OFFCHIP_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_zec12, L1D_OFFBOOK_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_zec12, L1I_ONCHIP_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_zec12, L1I_OFFCHIP_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_zec12, L1I_OFFBOOK_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_zec12, L1I_ONBOOK_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_zec12, L1I_OFFBOOK_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_zec12, TX_C_TEND),
	cpumf_event_ptr!(cf_zec12, L1I_ONCHIP_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_zec12, L1I_OFFCHIP_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_zec12, L1I_OFFBOOK_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_zec12, TX_NC_TABORT),
	cpumf_event_ptr!(cf_zec12, TX_C_TABORT_NO_SPECIAL),
	cpumf_event_ptr!(cf_zec12, TX_C_TABORT_SPECIAL),
	core::ptr::null_mut(),
};

static mut cpumcf_z13_pmu_event_attr: &[*mut attribute] = &[
	cpumf_event_ptr!(cf_z13, L1D_RO_EXCL_WRITES),
	cpumf_event_ptr!(cf_z13, DTLB1_WRITES),
	cpumf_event_ptr!(cf_z13, DTLB1_MISSES),
	cpumf_event_ptr!(cf_z13, DTLB1_HPAGE_WRITES),
	cpumf_event_ptr!(cf_z13, DTLB1_GPAGE_WRITES),
	cpumf_event_ptr!(cf_z13, L1D_L2D_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, ITLB1_WRITES),
	cpumf_event_ptr!(cf_z13, ITLB1_MISSES),
	cpumf_event_ptr!(cf_z13, L1I_L2I_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, TLB2_PTE_WRITES),
	cpumf_event_ptr!(cf_z13, TLB2_CRSTE_HPAGE_WRITES),
	cpumf_event_ptr!(cf_z13, TLB2_CRSTE_WRITES),
	cpumf_event_ptr!(cf_z13, TX_C_TEND),
	cpumf_event_ptr!(cf_z13, TX_NC_TEND),
	cpumf_event_ptr!(cf_z13, L1C_TLB1_MISSES),
	cpumf_event_ptr!(cf_z13, L1D_ONCHIP_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1D_ONCHIP_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z13, L1D_ONNODE_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1D_ONNODE_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z13, L1D_ONNODE_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1D_ONDRAWER_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1D_ONDRAWER_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z13, L1D_ONDRAWER_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1D_OFFDRAWER_SCOL_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1D_OFFDRAWER_SCOL_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z13, L1D_OFFDRAWER_SCOL_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1D_OFFDRAWER_FCOL_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1D_OFFDRAWER_FCOL_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z13, L1D_OFFDRAWER_FCOL_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1D_ONNODE_MEM_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1D_ONDRAWER_MEM_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1D_OFFDRAWER_MEM_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1D_ONCHIP_MEM_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1I_ONCHIP_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1I_ONCHIP_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z13, L1I_ONNODE_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1I_ONNODE_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z13, L1I_ONNODE_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1I_ONDRAWER_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1I_ONDRAWER_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z13, L1I_ONDRAWER_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1I_OFFDRAWER_SCOL_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1I_OFFDRAWER_SCOL_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z13, L1I_OFFDRAWER_SCOL_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1I_OFFDRAWER_FCOL_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1I_OFFDRAWER_FCOL_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z13, L1I_OFFDRAWER_FCOL_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1I_ONNODE_MEM_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1I_ONDRAWER_MEM_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1I_OFFDRAWER_MEM_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, L1I_ONCHIP_MEM_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z13, TX_NC_TABORT),
	cpumf_event_ptr!(cf_z13, TX_C_TABORT_NO_SPECIAL),
	cpumf_event_ptr!(cf_z13, TX_C_TABORT_SPECIAL),
	cpumf_event_ptr!(cf_z13, MT_DIAG_CYCLES_ONE_THR_ACTIVE),
	cpumf_event_ptr!(cf_z13, MT_DIAG_CYCLES_TWO_THR_ACTIVE),
	core::ptr::null_mut(),
};

static mut cpumcf_z14_pmu_event_attr: &[*mut attribute] = &[
	cpumf_event_ptr!(cf_z14, L1D_RO_EXCL_WRITES),
	cpumf_event_ptr!(cf_z14, DTLB2_WRITES),
	cpumf_event_ptr!(cf_z14, DTLB2_MISSES),
	cpumf_event_ptr!(cf_z14, DTLB2_HPAGE_WRITES),
	cpumf_event_ptr!(cf_z14, DTLB2_GPAGE_WRITES),
	cpumf_event_ptr!(cf_z14, L1D_L2D_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, ITLB2_WRITES),
	cpumf_event_ptr!(cf_z14, ITLB2_MISSES),
	cpumf_event_ptr!(cf_z14, L1I_L2I_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, TLB2_PTE_WRITES),
	cpumf_event_ptr!(cf_z14, TLB2_CRSTE_WRITES),
	cpumf_event_ptr!(cf_z14, TLB2_ENGINES_BUSY),
	cpumf_event_ptr!(cf_z14, TX_C_TEND),
	cpumf_event_ptr!(cf_z14, TX_NC_TEND),
	cpumf_event_ptr!(cf_z14, L1C_TLB2_MISSES),
	cpumf_event_ptr!(cf_z14, L1D_ONCHIP_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1D_ONCHIP_MEMORY_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1D_ONCHIP_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z14, L1D_ONCLUSTER_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1D_ONCLUSTER_MEMORY_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1D_ONCLUSTER_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z14, L1D_OFFCLUSTER_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1D_OFFCLUSTER_MEMORY_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1D_OFFCLUSTER_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z14, L1D_OFFDRAWER_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1D_OFFDRAWER_MEMORY_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1D_OFFDRAWER_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z14, L1D_ONDRAWER_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1D_OFFDRAWER_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1D_ONCHIP_L3_SOURCED_WRITES_RO),
	cpumf_event_ptr!(cf_z14, L1I_ONCHIP_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1I_ONCHIP_MEMORY_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1I_ONCHIP_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z14, L1I_ONCLUSTER_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1I_ONCLUSTER_MEMORY_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1I_ONCLUSTER_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z14, L1I_OFFCLUSTER_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1I_OFFCLUSTER_MEMORY_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1I_OFFCLUSTER_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z14, L1I_OFFDRAWER_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1I_OFFDRAWER_MEMORY_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1I_OFFDRAWER_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z14, L1I_ONDRAWER_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, L1I_OFFDRAWER_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z14, BCD_DFP_EXECUTION_SLOTS),
	cpumf_event_ptr!(cf_z14, VX_BCD_EXECUTION_SLOTS),
	cpumf_event_ptr!(cf_z14, DECIMAL_INSTRUCTIONS),
	cpumf_event_ptr!(cf_z14, LAST_HOST_TRANSLATIONS),
	cpumf_event_ptr!(cf_z14, TX_NC_TABORT),
	cpumf_event_ptr!(cf_z14, TX_C_TABORT_NO_SPECIAL),
	cpumf_event_ptr!(cf_z14, TX_C_TABORT_SPECIAL),
	cpumf_event_ptr!(cf_z14, MT_DIAG_CYCLES_ONE_THR_ACTIVE),
	cpumf_event_ptr!(cf_z14, MT_DIAG_CYCLES_TWO_THR_ACTIVE),
	core::ptr::null_mut(),
};

static mut cpumcf_z15_pmu_event_attr: &[*mut attribute] = &[
	cpumf_event_ptr!(cf_z15, L1D_RO_EXCL_WRITES),
	cpumf_event_ptr!(cf_z15, DTLB2_WRITES),
	cpumf_event_ptr!(cf_z15, DTLB2_MISSES),
	cpumf_event_ptr!(cf_z15, DTLB2_HPAGE_WRITES),
	cpumf_event_ptr!(cf_z15, DTLB2_GPAGE_WRITES),
	cpumf_event_ptr!(cf_z15, L1D_L2D_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, ITLB2_WRITES),
	cpumf_event_ptr!(cf_z15, ITLB2_MISSES),
	cpumf_event_ptr!(cf_z15, L1I_L2I_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, TLB2_PTE_WRITES),
	cpumf_event_ptr!(cf_z15, TLB2_CRSTE_WRITES),
	cpumf_event_ptr!(cf_z15, TLB2_ENGINES_BUSY),
	cpumf_event_ptr!(cf_z15, TX_C_TEND),
	cpumf_event_ptr!(cf_z15, TX_NC_TEND),
	cpumf_event_ptr!(cf_z15, L1C_TLB2_MISSES),
	cpumf_event_ptr!(cf_z15, L1D_ONCHIP_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1D_ONCHIP_MEMORY_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1D_ONCHIP_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z15, L1D_ONCLUSTER_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1D_ONCLUSTER_MEMORY_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1D_ONCLUSTER_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z15, L1D_OFFCLUSTER_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1D_OFFCLUSTER_MEMORY_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1D_OFFCLUSTER_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z15, L1D_OFFDRAWER_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1D_OFFDRAWER_MEMORY_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1D_OFFDRAWER_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z15, L1D_ONDRAWER_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1D_OFFDRAWER_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1D_ONCHIP_L3_SOURCED_WRITES_RO),
	cpumf_event_ptr!(cf_z15, L1I_ONCHIP_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1I_ONCHIP_MEMORY_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1I_ONCHIP_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z15, L1I_ONCLUSTER_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1I_ONCLUSTER_MEMORY_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1I_ONCLUSTER_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z15, L1I_OFFCLUSTER_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1I_OFFCLUSTER_MEMORY_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1I_OFFCLUSTER_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z15, L1I_OFFDRAWER_L3_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1I_OFFDRAWER_MEMORY_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1I_OFFDRAWER_L3_SOURCED_WRITES_IV),
	cpumf_event_ptr!(cf_z15, L1I_ONDRAWER_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, L1I_OFFDRAWER_L4_SOURCED_WRITES),
	cpumf_event_ptr!(cf_z15, BCD_DFP_EXECUTION_SLOTS),
	cpumf_event_ptr!(cf_z15, VX_BCD_EXECUTION_SLOTS),
	cpumf_event_ptr!(cf_z15, DECIMAL_INSTRUCTIONS),
	cpumf_event_ptr!(cf_z15, LAST_HOST_TRANSLATIONS),
	cpumf_event_ptr!(cf_z15, TX_NC_TABORT),
	cpumf_event_ptr!(cf_z15, TX_C_TABORT_NO_SPECIAL),
	cpumf_event_ptr!(cf_z15, TX_C_TABORT_SPECIAL),
	cpumf_event_ptr!(cf_z15, DFLT_ACCESS),
	cpumf_event_ptr!(cf_z15, DFLT_CYCLES),
	cpumf_event_ptr!(cf_z15, DFLT_CC),
	cpumf_event_ptr!(cf_z15, DFLT_CCFINISH),
	cpumf_event_ptr!(cf_z15, MT_DIAG_CYCLES_ONE_THR_ACTIVE),
	cpumf_event_ptr!(cf_z15, MT_DIAG_CYCLES_TWO_THR_ACTIVE),
	core::ptr::null_mut(),
};

static mut cpumcf_z16_pmu_event_attr: &[*mut attribute] = &[
	cpumf_event_ptr!(cf_z16, L1D_RO_EXCL_WRITES),
	cpumf_event_ptr!(cf_z16, DTLB2_WRITES),
	cpumf_event_ptr!(cf_z16, DTLB2_MISSES),
	cpumf_event_ptr!(cf_z16, CRSTE_1MB_WRITES),
	cpumf_event_ptr!(cf_z16, DTLB2_GPAGE_WRITES),
	cpumf_event_ptr!(cf_z16, ITLB2_WRITES),
	cpumf_event_ptr!(cf_z16, ITLB2_MISSES),
	cpumf_event_ptr!(cf_z16, TLB2_PTE_WRITES),
	cpumf_event_ptr!(cf_z16, TLB2_CRSTE_WRITES),
	cpumf_event_ptr!(cf_z16, TLB2_ENGINES_BUSY),
	cpumf_event_ptr!(cf_z16, TX_C_TEND),
	cpumf_event_ptr!(cf_z16, TX_NC_TEND),
	cpumf_event_ptr!(cf_z16, L1C_TLB2_MISSES),
	cpumf_event_ptr!(cf_z16, DCW_REQ),
	cpumf_event_ptr!(cf_z16, DCW_REQ_IV),
	cpumf_event_ptr!(cf_z16, DCW_REQ_CHIP_HIT),
	cpumf_event_ptr!(cf_z16, DCW_REQ_DRAWER_HIT),
	cpumf_event_ptr!(cf_z16, DCW_ON_CHIP),
	cpumf_event_ptr!(cf_z16, DCW_ON_CHIP_IV),
	cpumf_event_ptr!(cf_z16, DCW_ON_CHIP_CHIP_HIT),
	cpumf_event_ptr!(cf_z16, DCW_ON_CHIP_DRAWER_HIT),
	cpumf_event_ptr!(cf_z16, DCW_ON_MODULE),
	cpumf_event_ptr!(cf_z16, DCW_ON_DRAWER),
	cpumf_event_ptr!(cf_z16, DCW_OFF_DRAWER),
	cpumf_event_ptr!(cf_z16, DCW_ON_CHIP_MEMORY),
	cpumf_event_ptr!(cf_z16, DCW_ON_MODULE_MEMORY),
	cpumf_event_ptr!(cf_z16, DCW_ON_DRAWER_MEMORY),
	cpumf_event_ptr!(cf_z16, DCW_OFF_DRAWER_MEMORY),
	cpumf_event_ptr!(cf_z16, IDCW_ON_MODULE_IV),
	cpumf_event_ptr!(cf_z16, IDCW_ON_MODULE_CHIP_HIT),
	cpumf_event_ptr!(cf_z16, IDCW_ON_MODULE_DRAWER_HIT),
	cpumf_event_ptr!(cf_z16, IDCW_ON_DRAWER_IV),
	cpumf_event_ptr!(cf_z16, IDCW_ON_DRAWER_CHIP_HIT),
	cpumf_event_ptr!(cf_z16, IDCW_ON_DRAWER_DRAWER_HIT),
	cpumf_event_ptr!(cf_z16, IDCW_OFF_DRAWER_IV),
	cpumf_event_ptr!(cf_z16, IDCW_OFF_DRAWER_CHIP_HIT),
	cpumf_event_ptr!(cf_z16, IDCW_OFF_DRAWER_DRAWER_HIT),
	cpumf_event_ptr!(cf_z16, ICW_REQ),
	cpumf_event_ptr!(cf_z16, ICW_REQ_IV),
	cpumf_event_ptr!(cf_z16, ICW_REQ_CHIP_HIT),
	cpumf_event_ptr!(cf_z16, ICW_REQ_DRAWER_HIT),
	cpumf_event_ptr!(cf_z16, ICW_ON_CHIP),
	cpumf_event_ptr!(cf_z16, ICW_ON_CHIP_IV),
	cpumf_event_ptr!(cf_z16, ICW_ON_CHIP_CHIP_HIT),
	cpumf_event_ptr!(cf_z16, ICW_ON_CHIP_DRAWER_HIT),
	cpumf_event_ptr!(cf_z16, ICW_ON_MODULE),
	cpumf_event_ptr!(cf_z16, ICW_ON_DRAWER),
	cpumf_event_ptr!(cf_z16, ICW_OFF_DRAWER),
	cpumf_event_ptr!(cf_z16, ICW_ON_CHIP_MEMORY),
	cpumf_event_ptr!(cf_z16, ICW_ON_MODULE_MEMORY),
	cpumf_event_ptr!(cf_z16, ICW_ON_DRAWER_MEMORY),
	cpumf_event_ptr!(cf_z16, ICW_OFF_DRAWER_MEMORY),
	cpumf_event_ptr!(cf_z16, BCD_DFP_EXECUTION_SLOTS),
	cpumf_event_ptr!(cf_z16, VX_BCD_EXECUTION_SLOTS),
	cpumf_event_ptr!(cf_z16, DECIMAL_INSTRUCTIONS),
	cpumf_event_ptr!(cf_z16, LAST_HOST_TRANSLATIONS),
	cpumf_event_ptr!(cf_z16, TX_NC_TABORT),
	cpumf_event_ptr!(cf_z16, TX_C_TABORT_NO_SPECIAL),
	cpumf_event_ptr!(cf_z16, TX_C_TABORT_SPECIAL),
	cpumf_event_ptr!(cf_z16, DFLT_ACCESS),
	cpumf_event_ptr!(cf_z16, DFLT_CYCLES),
	cpumf_event_ptr!(cf_z16, SORTL),
	cpumf_event_ptr!(cf_z16, DFLT_CC),
	cpumf_event_ptr!(cf_z16, DFLT_CCFINISH),
	cpumf_event_ptr!(cf_z16, NNPA_INVOCATIONS),
	cpumf_event_ptr!(cf_z16, NNPA_COMPLETIONS),
	cpumf_event_ptr!(cf_z16, NNPA_WAIT_LOCK),
	cpumf_event_ptr!(cf_z16, NNPA_HOLD_LOCK),
	cpumf_event_ptr!(cf_z16, MT_DIAG_CYCLES_ONE_THR_ACTIVE),
	cpumf_event_ptr!(cf_z16, MT_DIAG_CYCLES_TWO_THR_ACTIVE),
	core::ptr::null_mut(),
};

static mut cpumcf_z17_pmu_event_attr: &[*mut attribute] = &[
	cpumf_event_ptr!(cf_z17, L1D_RO_EXCL_WRITES),
	cpumf_event_ptr!(cf_z17, DTLB2_WRITES),
	cpumf_event_ptr!(cf_z17, DTLB2_MISSES),
	cpumf_event_ptr!(cf_z17, CRSTE_1MB_WRITES),
	cpumf_event_ptr!(cf_z17, DTLB2_GPAGE_WRITES),
	cpumf_event_ptr!(cf_z17, ITLB2_WRITES),
	cpumf_event_ptr!(cf_z17, ITLB2_MISSES),
	cpumf_event_ptr!(cf_z17, TLB2_PTE_WRITES),
	cpumf_event_ptr!(cf_z17, TLB2_CRSTE_WRITES),
	cpumf_event_ptr!(cf_z17, TLB2_ENGINES_BUSY),
	cpumf_event_ptr!(cf_z17, TX_C_TEND),
	cpumf_event_ptr!(cf_z17, TX_NC_TEND),
	cpumf_event_ptr!(cf_z17, L1C_TLB2_MISSES),
	cpumf_event_ptr!(cf_z17, DCW_REQ),
	cpumf_event_ptr!(cf_z17, DCW_REQ_IV),
	cpumf_event_ptr!(cf_z17, DCW_REQ_CHIP_HIT),
	cpumf_event_ptr!(cf_z17, DCW_REQ_DRAWER_HIT),
	cpumf_event_ptr!(cf_z17, DCW_ON_CHIP),
	cpumf_event_ptr!(cf_z17, DCW_ON_CHIP_IV),
	cpumf_event_ptr!(cf_z17, DCW_ON_CHIP_CHIP_HIT),
	cpumf_event_ptr!(cf_z17, DCW_ON_CHIP_DRAWER_HIT),
	cpumf_event_ptr!(cf_z17, DCW_ON_MODULE),
	cpumf_event_ptr!(cf_z17, DCW_ON_DRAWER),
	cpumf_event_ptr!(cf_z17, DCW_OFF_DRAWER),
	cpumf_event_ptr!(cf_z17, DCW_ON_CHIP_MEMORY),
	cpumf_event_ptr!(cf_z17, DCW_ON_MODULE_MEMORY),
	cpumf_event_ptr!(cf_z17, DCW_ON_DRAWER_MEMORY),
	cpumf_event_ptr!(cf_z17, DCW_OFF_DRAWER_MEMORY),
	cpumf_event_ptr!(cf_z17, IDCW_ON_MODULE_IV),
	cpumf_event_ptr!(cf_z17, IDCW_ON_MODULE_CHIP_HIT),
	cpumf_event_ptr!(cf_z17, IDCW_ON_MODULE_DRAWER_HIT),
	cpumf_event_ptr!(cf_z17, IDCW_ON_DRAWER_IV),
	cpumf_event_ptr!(cf_z17, IDCW_ON_DRAWER_CHIP_HIT),
	cpumf_event_ptr!(cf_z17, IDCW_ON_DRAWER_DRAWER_HIT),
	cpumf_event_ptr!(cf_z17, IDCW_OFF_DRAWER_IV),
	cpumf_event_ptr!(cf_z17, IDCW_OFF_DRAWER_CHIP_HIT),
	cpumf_event_ptr!(cf_z17, IDCW_OFF_DRAWER_DRAWER_HIT),
	cpumf_event_ptr!(cf_z17, ICW_REQ),
	cpumf_event_ptr!(cf_z17, ICW_REQ_IV),
	cpumf_event_ptr!(cf_z17, ICW_REQ_CHIP_HIT),
	cpumf_event_ptr!(cf_z17, ICW_REQ_DRAWER_HIT),
	cpumf_event_ptr!(cf_z17, ICW_ON_CHIP),
	cpumf_event_ptr!(cf_z17, ICW_ON_CHIP_IV),
	cpumf_event_ptr!(cf_z17, ICW_ON_CHIP_CHIP_HIT),
	cpumf_event_ptr!(cf_z17, ICW_ON_CHIP_DRAWER_HIT),
	cpumf_event_ptr!(cf_z17, ICW_ON_MODULE),
	cpumf_event_ptr!(cf_z17, ICW_ON_DRAWER),
	cpumf_event_ptr!(cf_z17, ICW_OFF_DRAWER),
	cpumf_event_ptr!(cf_z17, CYCLES_SAMETHRD),
	cpumf_event_ptr!(cf_z17, CYCLES_DIFFTHRD),
	cpumf_event_ptr!(cf_z17, INST_SAMETHRD),
	cpumf_event_ptr!(cf_z17, INST_DIFFTHRD),
	cpumf_event_ptr!(cf_z17, WRONG_BRANCH_PREDICTION),
	cpumf_event_ptr!(cf_z17, VX_BCD_EXECUTION_SLOTS),
	cpumf_event_ptr!(cf_z17, DECIMAL_INSTRUCTIONS),
	cpumf_event_ptr!(cf_z17, LAST_HOST_TRANSLATIONS),
	cpumf_event_ptr!(cf_z17, TX_NC_TABORT),
	cpumf_event_ptr!(cf_z17, TX_C_TABORT_NO_SPECIAL),
	cpumf_event_ptr!(cf_z17, TX_C_TABORT_SPECIAL),
	cpumf_event_ptr!(cf_z17, DFLT_ACCESS),
	cpumf_event_ptr!(cf_z17, DFLT_CYCLES),
	cpumf_event_ptr!(cf_z17, SORTL),
	cpumf_event_ptr!(cf_z17, DFLT_CC),
	cpumf_event_ptr!(cf_z17, DFLT_CCFINISH),
	cpumf_event_ptr!(cf_z17, NNPA_INVOCATIONS),
	cpumf_event_ptr!(cf_z17, NNPA_COMPLETIONS),
	cpumf_event_ptr!(cf_z17, NNPA_WAIT_LOCK),
	cpumf_event_ptr!(cf_z17, NNPA_HOLD_LOCK),
	cpumf_event_ptr!(cf_z17, NNPA_INST_ONCHIP),
	cpumf_event_ptr!(cf_z17, NNPA_INST_OFFCHIP),
	cpumf_event_ptr!(cf_z17, NNPA_INST_DIFF),
	cpumf_event_ptr!(cf_z17, NNPA_4K_PREFETCH),
	cpumf_event_ptr!(cf_z17, NNPA_COMPL_LOCK),
	cpumf_event_ptr!(cf_z17, NNPA_RETRY_LOCK),
	cpumf_event_ptr!(cf_z17, NNPA_RETRY_LOCK_WITH_PLO),
	cpumf_event_ptr!(cf_z17, MT_DIAG_CYCLES_ONE_THR_ACTIVE),
	cpumf_event_ptr!(cf_z17, MT_DIAG_CYCLES_TWO_THR_ACTIVE),
	core::ptr::null_mut(),
};

/* END: CPUM_CF COUNTER DEFINITIONS ===================================== */

static mut cpumcf_pmu_events_group: attribute_group = attribute_group {
	.name = "events",
};

pmu_format_attr!(event, "config:0-63");

static mut cpumcf_pmu_format_attr: &[*mut attribute] = &[
	&format_attr_event.attr,
	core::ptr::null_mut(),
};

static mut cpumcf_pmu_format_group: attribute_group = attribute_group {
	.name = "format",
	.attrs = cpumcf_pmu_format_attr,
};

static cpumcf_pmu_attr_groups: &[*const attribute_group] = &[
	&cpumcf_pmu_events_group,
	&cpumcf_pmu_format_group,
	core::ptr::null_mut(),
};


unsafe fn merge_attr(a: *mut *mut attribute, b: *mut *mut attribute, c: *mut *mut attribute) -> *mut *mut attribute {
	struct attribute **new;
	int j, i;

	while a[j] { j = 0; j += 1; }
	while !(*b.add(i)).is_null() { i += 1; }
		j += 1;
	while !(*c.add(i)).is_null() { i += 1; }
		j += 1;
	j += 1;

	new = kmalloc_objs::<*mut attribute>(j);
	if new.is_null()
		return core::ptr::null_mut();
	j = 0;
	while *a.add(i) { new.add(j).write( *a.add(i); j += 1; i++; }
	while *b.add(i) { new.add(j).write( *b.add(i); j += 1; i++; }
	while *c.add(i) { new.add(j).write( *c.add(i); j += 1; i++; }
	new.add(j).write(core::ptr::null_mut()));

	return new;
}

unsafe fn cpumf_cf_event_group() -> *const *const attribute_group {
	let mut combined: *mut *mut attribute; let mut model: *mut *mut attribute; let mut cfvn: *mut *mut attribute; let mut csvn: *mut *mut attribute;
	let mut none: [*mut attribute; 1] = [core::ptr::null_mut()];
	let mut ci: cpumf_ctr_info;
	let mut cpu_id: cpuid;

	/* Determine generic counters set(s) */
	qctri(&mut ci);
	match ci.cfvn {
	1 =>
		cfvn = cpumcf_fvn1_pmu_event_attr;
		
	3 =>
		cfvn = cpumcf_fvn3_pmu_event_attr;
		
	_ =>
		cfvn = none;
	}

	/* Determine version specific crypto set */
	csvn = none;
	if ci.csvn >= 1 && ci.csvn <= 5
		csvn = cpumcf_svn_12345_pmu_event_attr;
	else if ci.csvn >= 6
		csvn = cpumcf_svn_678_pmu_event_attr;

	/* Determine model-specific counter set(s) */
	get_cpu_id(&mut cpu_id);
	match cpu_id.machine {
	0x2097 =>
	0x2098 =>
		model = cpumcf_z10_pmu_event_attr;
		
	0x2817 =>
	0x2818 =>
		model = cpumcf_z196_pmu_event_attr;
		
	0x2827 =>
	0x2828 =>
		model = cpumcf_zec12_pmu_event_attr;
		
	0x2964 =>
	0x2965 =>
		model = cpumcf_z13_pmu_event_attr;
		
	0x3906 =>
	0x3907 =>
		model = cpumcf_z14_pmu_event_attr;
		
	0x8561 =>
	0x8562 =>
		model = cpumcf_z15_pmu_event_attr;
		
	0x3931 =>
	0x3932 =>
		model = cpumcf_z16_pmu_event_attr;
		
	0x9175 =>
	0x9176 =>
		model = cpumcf_z17_pmu_event_attr;
		
	_ =>
		model = none;
		
	}

	combined = merge_attr(cfvn, csvn, model);
	if combined
		cpumcf_pmu_events_group.attrs = combined;
	cpumcf_pmu_attr_groups
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
