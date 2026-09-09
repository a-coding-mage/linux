/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (C) 2001 Ben. Herrenschmidt (benh@kernel.crashing.org)
 */

static mut cpu_specs: [cpu_spec; 1] = [
    cpu_spec {
        pvr_mask: 0xffff0000,
        pvr_value: PVR_8xx,
        cpu_name: b"8xx\0".as_ptr(),
        /*
         * CPU_FTR_MAYBE_CAN_DOZE is possible,
         * if the 8xx code is there....
         */
        cpu_features: CPU_FTRS_8XX,
        cpu_user_features: PPC_FEATURE_32 | PPC_FEATURE_HAS_MMU,
        mmu_features: MMU_FTR_TYPE_8xx,
        icache_bsize: 16,
        dcache_bsize: 16,
        machine_check: machine_check_8xx,
        platform: b"ppc823\0".as_ptr(),
    },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
