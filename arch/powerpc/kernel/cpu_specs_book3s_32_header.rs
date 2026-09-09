/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (C) 2001 Ben. Herrenschmidt (benh@kernel.crashing.org) */

// The following names and the `cpu_spec` layout are supplied by the kernel's
// other translation units.  Configuration guards are retained as Rust cfgs.
const COMMON_USER: u64 = PPC_FEATURE_32 | PPC_FEATURE_HAS_FPU | PPC_FEATURE_HAS_MMU;

#[allow(non_upper_case_globals)]
#[cfg(any(CONFIG_PPC_BOOK3S_603, CONFIG_PPC_BOOK3S_604))]
pub static mut cpu_specs: &[cpu_spec] = &[
#[cfg(CONFIG_PPC_BOOK3S_603)]
cpu_spec { pvr_mask: 0xffff0000, pvr_value: 0x00030000, cpu_name: "603", cpu_features: CPU_FTRS_603, cpu_user_features: COMMON_USER | PPC_FEATURE_PPC_LE, mmu_features: 0, icache_bsize: 32, dcache_bsize: 32, cpu_setup: __setup_cpu_603, machine_check: machine_check_generic, platform: "ppc603" },
#[cfg(CONFIG_PPC_BOOK3S_603)]
cpu_spec { pvr_mask: 0xffff0000, pvr_value: 0x00060000, cpu_name: "603e", cpu_features: CPU_FTRS_603, cpu_user_features: COMMON_USER | PPC_FEATURE_PPC_LE, mmu_features: 0, icache_bsize: 32, dcache_bsize: 32, cpu_setup: __setup_cpu_603, machine_check: machine_check_generic, platform: "ppc603" },
#[cfg(CONFIG_PPC_BOOK3S_603)]
cpu_spec { pvr_mask: 0xffff0000, pvr_value: 0x00070000, cpu_name: "603ev", cpu_features: CPU_FTRS_603, cpu_user_features: COMMON_USER | PPC_FEATURE_PPC_LE, mmu_features: 0, icache_bsize: 32, dcache_bsize: 32, cpu_setup: __setup_cpu_603, machine_check: machine_check_generic, platform: "ppc603" },
#[cfg(CONFIG_PPC_BOOK3S_603)]
cpu_spec { pvr_mask: 0x7fff0000, pvr_value: 0x00810000, cpu_name: "82xx", cpu_features: CPU_FTRS_82XX, cpu_user_features: COMMON_USER, mmu_features: 0, icache_bsize: 32, dcache_bsize: 32, cpu_setup: __setup_cpu_603, machine_check: machine_check_generic, platform: "ppc603" },
#[cfg(CONFIG_PPC_BOOK3S_603)]
cpu_spec { pvr_mask: 0x7fff0000, pvr_value: 0x00820000, cpu_name: "G2_LE", cpu_features: CPU_FTRS_G2_LE, cpu_user_features: COMMON_USER, mmu_features: MMU_FTR_USE_HIGH_BATS, icache_bsize: 32, dcache_bsize: 32, cpu_setup: __setup_cpu_603, machine_check: machine_check_generic, platform: "ppc603" },
#[cfg(all(CONFIG_PPC_BOOK3S_603, CONFIG_PPC_83xx))]
cpu_spec { pvr_mask: 0x7fff0000, pvr_value: 0x00830000, cpu_name: "e300c1", cpu_features: CPU_FTRS_E300, cpu_user_features: COMMON_USER, mmu_features: MMU_FTR_USE_HIGH_BATS, icache_bsize: 32, dcache_bsize: 32, cpu_setup: __setup_cpu_603, machine_check: machine_check_83xx, platform: "ppc603" },
#[cfg(all(CONFIG_PPC_BOOK3S_603, CONFIG_PPC_83xx))]
cpu_spec { pvr_mask: 0x7fff0000, pvr_value: 0x00840000, cpu_name: "e300c2", cpu_features: CPU_FTRS_E300C2, cpu_user_features: PPC_FEATURE_32 | PPC_FEATURE_HAS_MMU, mmu_features: MMU_FTR_USE_HIGH_BATS | MMU_FTR_NEED_DTLB_SW_LRU, icache_bsize: 32, dcache_bsize: 32, cpu_setup: __setup_cpu_603, machine_check: machine_check_83xx, platform: "ppc603" },
#[cfg(all(CONFIG_PPC_BOOK3S_603, CONFIG_PPC_83xx))]
cpu_spec { pvr_mask: 0x7fff0000, pvr_value: 0x00850000, cpu_name: "e300c3", cpu_features: CPU_FTRS_E300, cpu_user_features: COMMON_USER, mmu_features: MMU_FTR_USE_HIGH_BATS | MMU_FTR_NEED_DTLB_SW_LRU, icache_bsize: 32, dcache_bsize: 32, cpu_setup: __setup_cpu_603, machine_check: machine_check_83xx, num_pmcs: 4, platform: "ppc603" },
#[cfg(all(CONFIG_PPC_BOOK3S_603, CONFIG_PPC_83xx))]
cpu_spec { pvr_mask: 0x7fff0000, pvr_value: 0x00860000, cpu_name: "e300c4", cpu_features: CPU_FTRS_E300, cpu_user_features: COMMON_USER, mmu_features: MMU_FTR_USE_HIGH_BATS | MMU_FTR_NEED_DTLB_SW_LRU, icache_bsize: 32, dcache_bsize: 32, cpu_setup: __setup_cpu_603, machine_check: machine_check_83xx, num_pmcs: 4, platform: "ppc603" },
#[cfg(CONFIG_PPC_BOOK3S_604)]
cpu_spec { pvr_mask: 0xffff0000, pvr_value: 0x00040000, cpu_name: "604", cpu_features: CPU_FTRS_604, cpu_user_features: COMMON_USER | PPC_FEATURE_PPC_LE, mmu_features: MMU_FTR_HPTE_TABLE, icache_bsize: 32, dcache_bsize: 32, num_pmcs: 2, cpu_setup: __setup_cpu_604, machine_check: machine_check_generic, platform: "ppc604" },
#[cfg(CONFIG_PPC_BOOK3S_604)]
cpu_spec { pvr_mask: 0xfffff000, pvr_value: 0x00090000, cpu_name: "604e", cpu_features: CPU_FTRS_604, cpu_user_features: COMMON_USER | PPC_FEATURE_PPC_LE, mmu_features: MMU_FTR_HPTE_TABLE, icache_bsize: 32, dcache_bsize: 32, num_pmcs: 4, cpu_setup: __setup_cpu_604, machine_check: machine_check_generic, platform: "ppc604" },
#[cfg(CONFIG_PPC_BOOK3S_604)]
cpu_spec { pvr_mask: 0xffff0000, pvr_value: 0x00090000, cpu_name: "604r", cpu_features: CPU_FTRS_604, cpu_user_features: COMMON_USER | PPC_FEATURE_PPC_LE, mmu_features: MMU_FTR_HPTE_TABLE, icache_bsize: 32, dcache_bsize: 32, num_pmcs: 4, cpu_setup: __setup_cpu_604, machine_check: machine_check_generic, platform: "ppc604" },
#[cfg(CONFIG_PPC_BOOK3S_604)]
cpu_spec { pvr_mask: 0xffff0000, pvr_value: 0x000a0000, cpu_name: "604ev", cpu_features: CPU_FTRS_604, cpu_user_features: COMMON_USER | PPC_FEATURE_PPC_LE, mmu_features: MMU_FTR_HPTE_TABLE, icache_bsize: 32, dcache_bsize: 32, num_pmcs: 4, cpu_setup: __setup_cpu_604, machine_check: machine_check_generic, platform: "ppc604" },
#[cfg(CONFIG_PPC_BOOK3S_604)]
cpu_spec { pvr_mask: 0xffffffff, pvr_value: 0x00084202, cpu_name: "740/750", cpu_features: CPU_FTRS_740_NOTAU, cpu_user_features: COMMON_USER | PPC_FEATURE_PPC_LE, mmu_features: MMU_FTR_HPTE_TABLE, icache_bsize: 32, dcache_bsize: 32, num_pmcs: 4, cpu_setup: __setup_cpu_750, machine_check: machine_check_generic, platform: "ppc750" },
#[cfg(CONFIG_PPC_BOOK3S_604)]
cpu_spec { pvr_mask: 0xfffffff0, pvr_value: 0x00080100, cpu_name: "750CX", cpu_features: CPU_FTRS_750, cpu_user_features: COMMON_USER | PPC_FEATURE_PPC_LE, mmu_features: MMU_FTR_HPTE_TABLE, icache_bsize: 32, dcache_bsize: 32, num_pmcs: 4, pmc_type: PPC_PMC_IBM, cpu_setup: __setup_cpu_750cx, machine_check: machine_check_generic, platform: "ppc750" },
#[cfg(CONFIG_PPC_BOOK3S_604)]
cpu_spec { pvr_mask: 0xffff0000, pvr_value: 0x70000000, cpu_name: "750FX", cpu_features: CPU_FTRS_750FX, cpu_user_features: COMMON_USER | PPC_FEATURE_PPC_LE, mmu_features: MMU_FTR_HPTE_TABLE | MMU_FTR_USE_HIGH_BATS, icache_bsize: 32, dcache_bsize: 32, num_pmcs: 4, pmc_type: PPC_PMC_IBM, cpu_setup: __setup_cpu_750fx, machine_check: machine_check_generic, platform: "ppc750" },
#[cfg(CONFIG_PPC_BOOK3S_604)]
cpu_spec { pvr_mask: 0xffff0000, pvr_value: 0x80000000, cpu_name: "7450", cpu_features: CPU_FTRS_7450_23, cpu_user_features: COMMON_USER | PPC_FEATURE_HAS_ALTIVEC_COMP | PPC_FEATURE_PPC_LE, mmu_features: MMU_FTR_HPTE_TABLE, icache_bsize: 32, dcache_bsize: 32, num_pmcs: 6, pmc_type: PPC_PMC_G4, cpu_setup: __setup_cpu_745x, machine_check: machine_check_generic, platform: "ppc7450" },
#[cfg(CONFIG_PPC_BOOK3S_604)]
cpu_spec { pvr_mask: 0x00000000, pvr_value: 0x00000000, cpu_name: "(generic PPC)", cpu_features: CPU_FTRS_CLASSIC32, cpu_user_features: COMMON_USER, mmu_features: MMU_FTR_HPTE_TABLE, icache_bsize: 32, dcache_bsize: 32, machine_check: machine_check_generic, platform: "ppc603" },
];

#[cfg(not(any(CONFIG_PPC_BOOK3S_603, CONFIG_PPC_BOOK3S_604)))]
pub static mut cpu_specs: &[cpu_spec] = &[];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
