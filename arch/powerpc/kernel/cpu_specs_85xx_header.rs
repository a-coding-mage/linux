/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (C) 2001 Ben. Herrenschmidt (benh@kernel.crashing.org)
 */

const COMMON_USER_BOOKE: _ = PPC_FEATURE_32 | PPC_FEATURE_HAS_MMU | PPC_FEATURE_BOOKE;

static mut cpu_specs: [cpu_spec; 3] = [
    cpu_spec {
        /* e500 */
        pvr_mask: 0xffff0000,
        pvr_value: 0x80200000,
        cpu_name: b"e500\0".as_ptr() as *const _,
        cpu_features: CPU_FTRS_E500,
        cpu_user_features: COMMON_USER_BOOKE
            | PPC_FEATURE_HAS_SPE_COMP
            | PPC_FEATURE_HAS_EFP_SINGLE_COMP,
        cpu_user_features2: PPC_FEATURE2_ISEL,
        mmu_features: MMU_FTR_TYPE_FSL_E,
        icache_bsize: 32,
        dcache_bsize: 32,
        num_pmcs: 4,
        cpu_setup: Some(__setup_cpu_e500v1),
        machine_check: Some(machine_check_e500),
        platform: b"ppc8540\0".as_ptr() as *const _,
        ..unsafe { core::mem::zeroed() }
    },
    cpu_spec {
        /* e500v2 */
        pvr_mask: 0xffff0000,
        pvr_value: 0x80210000,
        cpu_name: b"e500v2\0".as_ptr() as *const _,
        cpu_features: CPU_FTRS_E500_2,
        cpu_user_features: COMMON_USER_BOOKE
            | PPC_FEATURE_HAS_SPE_COMP
            | PPC_FEATURE_HAS_EFP_SINGLE_COMP
            | PPC_FEATURE_HAS_EFP_DOUBLE_COMP,
        cpu_user_features2: PPC_FEATURE2_ISEL,
        mmu_features: MMU_FTR_TYPE_FSL_E | MMU_FTR_BIG_PHYS,
        icache_bsize: 32,
        dcache_bsize: 32,
        num_pmcs: 4,
        cpu_setup: Some(__setup_cpu_e500v2),
        machine_check: Some(machine_check_e500),
        platform: b"ppc8548\0".as_ptr() as *const _,
        cpu_down_flush: Some(cpu_down_flush_e500v2),
        ..unsafe { core::mem::zeroed() }
    },
    cpu_spec {
        /* default match */
        pvr_mask: 0x00000000,
        pvr_value: 0x00000000,
        cpu_name: b"(generic E500 PPC)\0".as_ptr() as *const _,
        cpu_features: CPU_FTRS_E500,
        cpu_user_features: COMMON_USER_BOOKE
            | PPC_FEATURE_HAS_SPE_COMP
            | PPC_FEATURE_HAS_EFP_SINGLE_COMP,
        mmu_features: MMU_FTR_TYPE_FSL_E,
        icache_bsize: 32,
        dcache_bsize: 32,
        machine_check: Some(machine_check_e500),
        platform: b"powerpc\0".as_ptr() as *const _,
        ..unsafe { core::mem::zeroed() }
    },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
