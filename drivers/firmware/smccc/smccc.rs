// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 Arm Limited
 */

// Linux kernel headers and build-time annotations are supplied by the surrounding crate.

static mut smccc_version: u32 = ARM_SMCCC_VERSION_1_0;
static mut smccc_conduit: arm_smccc_conduit = SMCCC_CONDUIT_NONE;

pub static mut smccc_trng_available: bool = false;
pub static mut smccc_soc_id_version: i32 = SMCCC_RET_NOT_SUPPORTED;
pub static mut smccc_soc_id_revision: i32 = SMCCC_RET_NOT_SUPPORTED;

pub unsafe fn arm_smccc_version_init(version: u32, conduit: arm_smccc_conduit) {
    let mut res: arm_smccc_res = core::mem::zeroed();

    smccc_version = version;
    smccc_conduit = conduit;

    smccc_trng_available = smccc_probe_trng();

    if smccc_version >= ARM_SMCCC_VERSION_1_2 && smccc_conduit != SMCCC_CONDUIT_NONE {
        arm_smccc_1_1_invoke(ARM_SMCCC_ARCH_FEATURES_FUNC_ID, ARM_SMCCC_ARCH_SOC_ID, &mut res);
        if res.a0 as i32 >= 0 {
            arm_smccc_1_1_invoke(ARM_SMCCC_ARCH_SOC_ID, 0, &mut res);
            smccc_soc_id_version = res.a0 as i32;
            arm_smccc_1_1_invoke(ARM_SMCCC_ARCH_SOC_ID, 1, &mut res);
            smccc_soc_id_revision = res.a0 as i32;
        }
    }
}

pub unsafe fn arm_smccc_1_1_get_conduit() -> arm_smccc_conduit {
    if smccc_version < ARM_SMCCC_VERSION_1_1 {
        return SMCCC_CONDUIT_NONE;
    }

    smccc_conduit
}

pub unsafe fn arm_smccc_get_version() -> u32 {
    smccc_version
}

pub unsafe fn arm_smccc_get_soc_id_version() -> i32 {
    smccc_soc_id_version
}

pub unsafe fn arm_smccc_get_soc_id_revision() -> i32 {
    smccc_soc_id_revision
}

pub unsafe fn arm_smccc_hypervisor_has_uuid(hyp_uuid: *const uuid_t) -> bool {
    let mut res: arm_smccc_res = core::mem::zeroed();
    let mut uuid: uuid_t;

    arm_smccc_1_1_invoke(ARM_SMCCC_VENDOR_HYP_CALL_UID_FUNC_ID, &mut res);
    if res.a0 == SMCCC_RET_NOT_SUPPORTED as _ {
        return false;
    }

    uuid = smccc_res_to_uuid(res.a0, res.a1, res.a2, res.a3);
    uuid_equal(&uuid, hyp_uuid)
}

unsafe fn smccc_devices_init() -> i32 {
    let mut pdev: *mut platform_device;

    if smccc_trng_available {
        pdev = platform_device_register_simple("smccc_trng\0".as_ptr() as *const _, -1, core::ptr::null(), 0);
        if IS_ERR(pdev) {
            pr_err!("smccc_trng: could not register device: %ld\n", PTR_ERR(pdev));
        }
    }

    0
}

// device_initcall(smccc_devices_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
