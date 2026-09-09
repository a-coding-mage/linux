// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here without reimplementing them.

static ADF_CFG_SERVICES: [*const core::ffi::c_char; SVC_COUNT] = [
    [SVC_ASYM] = ADF_CFG_ASYM,
    [SVC_SYM] = ADF_CFG_SYM,
    [SVC_DC] = ADF_CFG_DC,
    [SVC_DCC] = ADF_CFG_DCC,
    [SVC_DECOMP] = ADF_CFG_DECOMP,
];

/*
 * Ensure that the size of the array matches the number of services,
 * SVC_COUNT, that is used to size the bitmap.
 */
const _: () = assert!(ADF_CFG_SERVICES.len() == SVC_COUNT);

/*
 * Ensure that the maximum number of concurrent services that can be
 * enabled on a device is less than or equal to the number of total
 * supported services.
 */
const _: () = assert!(ADF_CFG_SERVICES.len() >= MAX_NUM_CONCURR_SVC);

/*
 * Ensure that the number of services fit a single unsigned long, as each
 * service is represented by a bit in the mask.
 */
const _: () = assert!(usize::BITS >= SVC_COUNT as u32);

/*
 * Ensure that size of the concatenation of all service strings is smaller
 * than the size of the buffer that will contain them.
 */
const _: () = assert!(
    core::mem::size_of_val(&(ADF_CFG_SYM ADF_SERVICES_DELIMITER ADF_CFG_ASYM
        ADF_SERVICES_DELIMITER ADF_CFG_DC ADF_SERVICES_DELIMITER ADF_CFG_DECOMP
        ADF_SERVICES_DELIMITER ADF_CFG_DCC)) < ADF_CFG_MAX_VAL_LEN_IN_BYTES
);

unsafe fn adf_service_string_to_mask(
    accel_dev: *mut adf_accel_dev,
    buf: *const core::ffi::c_char,
    out_mask: *mut usize,
) -> i32 {
    let hw_data = GET_HW_DATA(accel_dev);
    let mut services = [0i8; ADF_CFG_MAX_VAL_LEN_IN_BYTES];
    let mut mask: usize = 0;
    let mut substr: *mut core::ffi::c_char;
    let mut token: *mut core::ffi::c_char;
    let mut id: i32;
    let mut num_svc: i32 = 0;

    if strscpy_pad(services.as_mut_ptr(), buf) < 0 {
        return -EINVAL;
    }

    substr = services.as_mut_ptr();

    while {
        token = strsep(&mut substr, ADF_SERVICES_DELIMITER);
        !token.is_null()
    } {
        id = sysfs_match_string(ADF_CFG_SERVICES.as_ptr(), token);
        if id < 0 {
            return id;
        }

        if test_and_set_bit(id as usize, &mut mask) {
            return -EINVAL;
        }

        num_svc += 1;
        if num_svc - 1 == MAX_NUM_CONCURR_SVC {
            return -EINVAL;
        }
    }

    if !hw_data.services_supported.is_null()
        && !((*hw_data.services_supported)(mask))
    {
        return -EINVAL;
    }

    *out_mask = mask;
    0
}

unsafe fn adf_service_mask_to_string(mask: usize, buf: *mut core::ffi::c_char, len: usize) -> i32 {
    let mut offset: i32 = 0;
    let mut bit: i32;

    if len < ADF_CFG_MAX_VAL_LEN_IN_BYTES {
        return -ENOSPC;
    }

    for_each_set_bit!(&mask, SVC_COUNT, bit) {
        if offset != 0 {
            offset += scnprintf(
                buf.add(offset as usize),
                len - offset as usize,
                ADF_SERVICES_DELIMITER "%s",
                ADF_CFG_SERVICES[bit as usize],
            );
        } else {
            offset += scnprintf(
                buf,
                len,
                "%s",
                ADF_CFG_SERVICES[bit as usize],
            );
        }
    }

    0
}

pub unsafe fn adf_parse_service_string(
    accel_dev: *mut adf_accel_dev,
    input: *const core::ffi::c_char,
    out: *mut core::ffi::c_char,
    out_len: usize,
) -> i32 {
    let mut mask: usize = 0;
    let ret = adf_service_string_to_mask(accel_dev, input, &mut mask);
    if ret != 0 {
        return ret;
    }

    if mask == 0 {
        return -EINVAL;
    }

    adf_service_mask_to_string(mask, out, out_len)
}

pub unsafe fn adf_get_service_mask(accel_dev: *mut adf_accel_dev, mask: *mut usize) -> i32 {
    let mut services = [0i8; ADF_CFG_MAX_VAL_LEN_IN_BYTES];
    let ret = adf_cfg_get_param_value(accel_dev, ADF_GENERAL_SEC, ADF_SERVICES_ENABLED, services.as_mut_ptr());
    if ret != 0 {
        dev_err!(GET_DEV(accel_dev), "%s param not found\n", ADF_SERVICES_ENABLED);
        return ret;
    }

    let ret = adf_service_string_to_mask(accel_dev, services.as_ptr(), mask);
    if ret != 0 {
        dev_err!(GET_DEV(accel_dev), "Invalid value of %s param: %s\n", ADF_SERVICES_ENABLED, services.as_ptr());
    }
    ret
}

pub unsafe fn adf_get_service_enabled(accel_dev: *mut adf_accel_dev) -> i32 {
    let mut mask: usize = 0;
    let ret = adf_get_service_mask(accel_dev, &mut mask);
    if ret != 0 {
        return ret;
    }

    if test_bit(SVC_SYM, &mask) && test_bit(SVC_ASYM, &mask) { return SVC_SYM_ASYM; }
    if test_bit(SVC_SYM, &mask) && test_bit(SVC_DC, &mask) { return SVC_SYM_DC; }
    if test_bit(SVC_ASYM, &mask) && test_bit(SVC_DC, &mask) { return SVC_ASYM_DC; }
    if test_bit(SVC_SYM, &mask) { return SVC_SYM; }
    if test_bit(SVC_ASYM, &mask) { return SVC_ASYM; }
    if test_bit(SVC_DC, &mask) { return SVC_DC; }
    if test_bit(SVC_DECOMP, &mask) { return SVC_DECOMP; }
    if test_bit(SVC_DCC, &mask) { return SVC_DCC; }
    -EINVAL
}

pub fn adf_srv_to_cfg_svc_type(svc: adf_base_services) -> adf_cfg_service_type {
    match svc {
        SVC_ASYM => ASYM,
        SVC_SYM => SYM,
        SVC_DC => COMP,
        SVC_DECOMP => DECOMP,
        _ => UNUSED,
    }
}

pub unsafe fn adf_is_service_enabled(accel_dev: *mut adf_accel_dev, svc: adf_base_services) -> bool {
    let arb_srv = adf_srv_to_cfg_svc_type(svc);
    let hw_data = GET_HW_DATA(accel_dev);
    let rps_per_bundle: u8 = hw_data.num_banks_per_vf;

    for i in 0..rps_per_bundle {
        if GET_SRV_TYPE(accel_dev, i as i32) == arb_srv {
            return true;
        }
    }

    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
