/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (C) 2001 Ben. Herrenschmidt (benh@kernel.crashing.org)
 */

const COMMON_USER_BOOKE: u64 = PPC_FEATURE_32 | PPC_FEATURE_HAS_MMU | PPC_FEATURE_BOOKE;

static mut cpu_specs: [cpu_spec; 5] = [
    cpu_spec {
        /* 476 DD2 core */
        pvr_mask: 0xffff_ffff,
        pvr_value: 0x11a5_2080,
        cpu_name: "476",
        cpu_features: CPU_FTRS_47X | CPU_FTR_476_DD2,
        cpu_user_features: COMMON_USER_BOOKE | PPC_FEATURE_HAS_FPU,
        mmu_features: MMU_FTR_TYPE_47x
            | MMU_FTR_USE_TLBIVAX_BCAST
            | MMU_FTR_LOCK_BCAST_INVAL,
        icache_bsize: 32,
        dcache_bsize: 128,
        machine_check: machine_check_47x,
        platform: "ppc470",
    },
    cpu_spec {
        /* 476fpe */
        pvr_mask: 0xffff_0000,
        pvr_value: 0x7ff5_0000,
        cpu_name: "476fpe",
        cpu_features: CPU_FTRS_47X | CPU_FTR_476_DD2,
        cpu_user_features: COMMON_USER_BOOKE | PPC_FEATURE_HAS_FPU,
        mmu_features: MMU_FTR_TYPE_47x
            | MMU_FTR_USE_TLBIVAX_BCAST
            | MMU_FTR_LOCK_BCAST_INVAL,
        icache_bsize: 32,
        dcache_bsize: 128,
        machine_check: machine_check_47x,
        platform: "ppc470",
    },
    cpu_spec {
        /* 476 iss */
        pvr_mask: 0xffff_0000,
        pvr_value: 0x0005_0000,
        cpu_name: "476",
        cpu_features: CPU_FTRS_47X,
        cpu_user_features: COMMON_USER_BOOKE | PPC_FEATURE_HAS_FPU,
        mmu_features: MMU_FTR_TYPE_47x
            | MMU_FTR_USE_TLBIVAX_BCAST
            | MMU_FTR_LOCK_BCAST_INVAL,
        icache_bsize: 32,
        dcache_bsize: 128,
        machine_check: machine_check_47x,
        platform: "ppc470",
    },
    cpu_spec {
        /* 476 others */
        pvr_mask: 0xffff_0000,
        pvr_value: 0x11a5_0000,
        cpu_name: "476",
        cpu_features: CPU_FTRS_47X,
        cpu_user_features: COMMON_USER_BOOKE | PPC_FEATURE_HAS_FPU,
        mmu_features: MMU_FTR_TYPE_47x
            | MMU_FTR_USE_TLBIVAX_BCAST
            | MMU_FTR_LOCK_BCAST_INVAL,
        icache_bsize: 32,
        dcache_bsize: 128,
        machine_check: machine_check_47x,
        platform: "ppc470",
    },
    cpu_spec {
        /* default match */
        pvr_mask: 0x0000_0000,
        pvr_value: 0x0000_0000,
        cpu_name: "(generic 47x PPC)",
        cpu_features: CPU_FTRS_47X,
        cpu_user_features: COMMON_USER_BOOKE,
        mmu_features: MMU_FTR_TYPE_47x,
        icache_bsize: 32,
        dcache_bsize: 128,
        machine_check: machine_check_47x,
        platform: "ppc470",
    },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
