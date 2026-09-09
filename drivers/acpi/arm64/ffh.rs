// SPDX-License-Identifier: GPL-2.0-only
// External declarations corresponding to <linux/acpi.h>, <linux/arm-smccc.h>,
// and <linux/slab.h> are supplied by the surrounding translation unit.

/*
 * Implements ARM64 specific callbacks to support ACPI FFH Operation Region as
 * specified in https://developer.arm.com/docs/den0048/latest
 */
#[repr(C)]
pub struct acpi_ffh_data {
    pub info: acpi_ffh_info,
    pub invoke_ffh_fn: Option<unsafe extern "C" fn(
        a0: c_ulong,
        a1: c_ulong,
        a2: c_ulong,
        a3: c_ulong,
        a4: c_ulong,
        a5: c_ulong,
        a6: c_ulong,
        a7: c_ulong,
        args: *mut arm_smccc_res,
        res: *mut arm_smccc_quirk,
    )>,
    pub invoke_ffh64_fn: Option<unsafe extern "C" fn(
        args: *const arm_smccc_1_2_regs,
        res: *mut arm_smccc_1_2_regs,
    )>,
}

pub unsafe extern "C" fn acpi_ffh_address_space_arch_setup(
    handler_ctxt: *mut c_void,
    region_ctxt: *mut *mut c_void,
) -> c_int {
    let conduit: arm_smccc_conduit;
    let ffh_ctxt: *mut acpi_ffh_data;

    if arm_smccc_get_version() < ARM_SMCCC_VERSION_1_2 {
        return -EOPNOTSUPP;
    }

    conduit = arm_smccc_1_1_get_conduit();
    if conduit == SMCCC_CONDUIT_NONE {
        pr_err!("{}: invalid SMCCC conduit\n", "acpi_ffh_address_space_arch_setup");
        return -EOPNOTSUPP;
    }

    ffh_ctxt = kzalloc_obj::<acpi_ffh_data>();
    if ffh_ctxt.is_null() {
        return -ENOMEM;
    }

    if conduit == SMCCC_CONDUIT_SMC {
        (*ffh_ctxt).invoke_ffh_fn = Some(__arm_smccc_smc);
        (*ffh_ctxt).invoke_ffh64_fn = Some(arm_smccc_1_2_smc);
    } else {
        (*ffh_ctxt).invoke_ffh_fn = Some(__arm_smccc_hvc);
        (*ffh_ctxt).invoke_ffh64_fn = Some(arm_smccc_1_2_hvc);
    }

    core::ptr::copy_nonoverlapping(
        handler_ctxt as *const u8,
        ffh_ctxt as *mut u8,
        core::mem::size_of::<acpi_ffh_info>(),
    );

    *region_ctxt = ffh_ctxt as *mut c_void;
    AE_OK
}

unsafe fn acpi_ffh_smccc_owner_allowed(fid: u32) -> bool {
    let owner = ARM_SMCCC_OWNER_NUM(fid);

    if owner == ARM_SMCCC_OWNER_STANDARD
        || owner == ARM_SMCCC_OWNER_SIP
        || owner == ARM_SMCCC_OWNER_OEM
    {
        return true;
    }

    false
}

pub unsafe extern "C" fn acpi_ffh_address_space_arch_handler(
    value: *mut acpi_integer,
    region_context: *mut c_void,
) -> c_int {
    let mut ret: c_int = 0;
    let ffh_ctxt = region_context as *mut acpi_ffh_data;

    if (*ffh_ctxt).info.offset == 0 {
        /* SMC/HVC 32bit call */
        let mut res: arm_smccc_res = core::mem::zeroed();
        let mut a: [u32; 8] = [0; 8];
        let ptr = value as *mut u32;

        if !ARM_SMCCC_IS_FAST_CALL(*ptr)
            || ARM_SMCCC_IS_64(*ptr)
            || !acpi_ffh_smccc_owner_allowed(*ptr)
            || (*ffh_ctxt).info.length > 32
        {
            ret = AE_ERROR;
        } else {
            let len = (*ffh_ctxt).info.length >> 2;

            for idx in 0..len {
                a[idx as usize] = *ptr.add(idx as usize);
            }

            ((*ffh_ctxt).invoke_ffh_fn.unwrap())(
                a[0] as c_ulong, a[1] as c_ulong, a[2] as c_ulong, a[3] as c_ulong,
                a[4] as c_ulong, a[5] as c_ulong, a[6] as c_ulong, a[7] as c_ulong,
                &mut res, core::ptr::null_mut(),
            );
            core::ptr::copy_nonoverlapping(
                &res as *const arm_smccc_res as *const u8,
                value as *mut u8,
                core::mem::size_of::<arm_smccc_res>(),
            );
        }
    } else if (*ffh_ctxt).info.offset == 1 {
        /* SMC/HVC 64bit call */
        let r = value as *mut arm_smccc_1_2_regs;

        if !ARM_SMCCC_IS_FAST_CALL((*r).a0)
            || !ARM_SMCCC_IS_64((*r).a0)
            || !acpi_ffh_smccc_owner_allowed((*r).a0)
            || (*ffh_ctxt).info.length > core::mem::size_of::<arm_smccc_1_2_regs>()
        {
            ret = AE_ERROR;
        } else {
            ((*ffh_ctxt).invoke_ffh64_fn.unwrap())(r, r);
            core::ptr::copy_nonoverlapping(
                r as *const u8,
                value as *mut u8,
                (*ffh_ctxt).info.length as usize,
            );
        }
    } else {
        ret = AE_ERROR;
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
