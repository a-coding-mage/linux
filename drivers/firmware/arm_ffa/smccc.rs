// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 ARM Ltd.
 */

// Dependencies supplied by the corresponding common definitions.
use crate::common::{
    arm_smccc_1_1_get_conduit, arm_smccc_1_2_hvc, arm_smccc_1_2_smc,
    arm_smccc_get_version, ffa_fn, ffa_value_t, ARM_SMCCC_VERSION_1_2,
    SMCCC_CONDUIT_HVC, SMCCC_CONDUIT_NONE, SMCCC_CONDUIT_SMC,
};

unsafe fn __arm_ffa_fn_smc(args: ffa_value_t, res: *mut ffa_value_t) {
    arm_smccc_1_2_smc(&args, res);
}

unsafe fn __arm_ffa_fn_hvc(args: ffa_value_t, res: *mut ffa_value_t) {
    arm_smccc_1_2_hvc(&args, res);
}

pub unsafe fn ffa_transport_init(invoke_ffa_fn: *mut ffa_fn) -> i32 {
    let conduit;

    if arm_smccc_get_version() < ARM_SMCCC_VERSION_1_2 {
        return -95; // -EOPNOTSUPP
    }

    conduit = arm_smccc_1_1_get_conduit();
    if conduit == SMCCC_CONDUIT_NONE {
        pr_err!("{}: invalid SMCCC conduit\n", "ffa_transport_init");
        return -95; // -EOPNOTSUPP
    }

    if conduit == SMCCC_CONDUIT_SMC {
        *invoke_ffa_fn = __arm_ffa_fn_smc;
    } else {
        // The C implementation selects HVC for every non-SMC conduit.
        let _ = SMCCC_CONDUIT_HVC;
        *invoke_ffa_fn = __arm_ffa_fn_hvc;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
