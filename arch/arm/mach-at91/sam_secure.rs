// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2022, Microchip
 */

// Dependencies supplied by the Linux ARM SMCCC, device-tree, and sam_secure
// headers are intentionally referenced here rather than reimplemented.

static mut optee_available: bool = false;

#[inline]
unsafe fn sam_sip_smc_std_call_val(func_num: u32) -> u32 {
    ARM_SMCCC_CALL_VAL(
        ARM_SMCCC_STD_CALL,
        ARM_SMCCC_SMC_32,
        ARM_SMCCC_OWNER_SIP,
        func_num,
    )
}

pub unsafe fn sam_smccc_call(fn_: u32, arg0: u32, arg1: u32) -> arm_smccc_res {
    let mut res = arm_smccc_res {
        a0: u64::MAX,
        ..core::mem::zeroed()
    };

    if WARN_ON!(!optee_available) {
        return res;
    }

    arm_smccc_smc(
        sam_sip_smc_std_call_val(fn_),
        arg0,
        arg1,
        0,
        0,
        0,
        0,
        0,
        &mut res,
    );

    res
}

pub unsafe fn sam_linux_is_optee_available() -> bool {
    /* If optee has been detected, then we are running in normal world */
    optee_available
}

// __init
pub unsafe fn sam_secure_init() {
    let np: *mut device_node;

    /*
     * We only check that the OP-TEE node is present and available. The
     * OP-TEE kernel driver is not needed for the type of interaction made
     * with OP-TEE here so the driver's status is not checked.
     */
    np = of_find_node_by_path(c"/firmware/optee".as_ptr());
    if !np.is_null() && of_device_is_available(np) {
        optee_available = true;
    }
    of_node_put(np);

    if optee_available {
        pr_info!("Running under OP-TEE firmware\n");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
