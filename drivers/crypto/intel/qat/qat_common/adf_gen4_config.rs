// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */
// Dependencies supplied by the corresponding QAT headers are intentionally
// referenced here as external symbols.

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub unsafe fn adf_crypto_dev_config(accel_dev: *mut adf_accel_dev) -> c_int {
    let mut key = [0 as c_char; ADF_CFG_MAX_KEY_LEN_IN_BYTES];
    let banks: c_int = GET_MAX_BANKS(accel_dev);
    let cpus: c_int = num_online_cpus();
    let mut bank: c_ulong;
    let mut val: c_ulong;
    let instances: c_int = if adf_hw_dev_has_crypto(accel_dev) {
        core::cmp::min(cpus, banks / 2)
    } else { 0 };
    let mut i = 0;
    while i < instances {
        val = i as c_ulong;
        bank = (i * 2) as c_ulong;
        snprintf_key(&mut key, ADF_CY, i, ADF_RING_ASYM_BANK_NUM);
        let mut ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, key.as_ptr(), &bank, ADF_DEC);
        if ret != 0 { return crypto_config_error(accel_dev, ret); }
        bank += 1;
        snprintf_key(&mut key, ADF_CY, i, ADF_RING_SYM_BANK_NUM);
        ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, key.as_ptr(), &bank, ADF_DEC);
        if ret != 0 { return crypto_config_error(accel_dev, ret); }
        snprintf_key(&mut key, ADF_CY, i, ADF_ETRMGR_CORE_AFFINITY);
        ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, key.as_ptr(), &val, ADF_DEC);
        if ret != 0 { return crypto_config_error(accel_dev, ret); }
        snprintf_key(&mut key, ADF_CY, i, ADF_RING_ASYM_SIZE); val = 128;
        ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, key.as_ptr(), &val, ADF_DEC);
        if ret != 0 { return crypto_config_error(accel_dev, ret); }
        val = 512; snprintf_key(&mut key, ADF_CY, i, ADF_RING_SYM_SIZE);
        ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, key.as_ptr(), &val, ADF_DEC);
        if ret != 0 { return crypto_config_error(accel_dev, ret); }
        val = 0; snprintf_key(&mut key, ADF_CY, i, ADF_RING_ASYM_TX);
        ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, key.as_ptr(), &val, ADF_DEC);
        if ret != 0 { return crypto_config_error(accel_dev, ret); }
        snprintf_key(&mut key, ADF_CY, i, ADF_RING_SYM_TX);
        ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, key.as_ptr(), &val, ADF_DEC);
        if ret != 0 { return crypto_config_error(accel_dev, ret); }
        val = 1; snprintf_key(&mut key, ADF_CY, i, ADF_RING_ASYM_RX);
        ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, key.as_ptr(), &val, ADF_DEC);
        if ret != 0 { return crypto_config_error(accel_dev, ret); }
        snprintf_key(&mut key, ADF_CY, i, ADF_RING_SYM_RX);
        ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, key.as_ptr(), &val, ADF_DEC);
        if ret != 0 { return crypto_config_error(accel_dev, ret); }
        val = ADF_COALESCING_DEF_TIME; snprintf_key(&mut key, ADF_ETRMGR_COALESCE_TIMER_FORMAT, i, "");
        ret = adf_cfg_add_key_value_param(accel_dev, ADF_ACCELERATOR0, key.as_ptr(), &val, ADF_DEC);
        if ret != 0 { return crypto_config_error(accel_dev, ret); }
        i += 1;
    }
    val = i as c_ulong;
    let mut ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, ADF_NUM_CY, &val, ADF_DEC);
    if ret != 0 { return crypto_config_error(accel_dev, ret); }
    val = 0;
    ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, ADF_NUM_DC, &val, ADF_DEC);
    if ret != 0 { return crypto_config_error(accel_dev, ret); }
    0
}

pub unsafe fn adf_comp_dev_config(accel_dev: *mut adf_accel_dev) -> c_int {
    let banks = GET_MAX_BANKS(accel_dev); let cpus = num_online_cpus();
    let instances = if adf_hw_dev_has_compression(accel_dev) { core::cmp::min(cpus, banks) } else { 0 };
    let mut key = [0 as c_char; ADF_CFG_MAX_KEY_LEN_IN_BYTES]; let mut val: c_ulong = 0; let mut i = 0;
    while i < instances {
        val = i as c_ulong; snprintf_key(&mut key, ADF_DC, i, ADF_RING_DC_BANK_NUM);
        let mut ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, key.as_ptr(), &val, ADF_DEC); if ret != 0 { return comp_config_error(accel_dev, ret); }
        val = 512; snprintf_key(&mut key, ADF_DC, i, ADF_RING_DC_SIZE); ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, key.as_ptr(), &val, ADF_DEC); if ret != 0 { return comp_config_error(accel_dev, ret); }
        val = 0; snprintf_key(&mut key, ADF_DC, i, ADF_RING_DC_TX); ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, key.as_ptr(), &val, ADF_DEC); if ret != 0 { return comp_config_error(accel_dev, ret); }
        val = 1; snprintf_key(&mut key, ADF_DC, i, ADF_RING_DC_RX); ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, key.as_ptr(), &val, ADF_DEC); if ret != 0 { return comp_config_error(accel_dev, ret); }
        val = ADF_COALESCING_DEF_TIME; snprintf_key(&mut key, ADF_ETRMGR_COALESCE_TIMER_FORMAT, i, ""); ret = adf_cfg_add_key_value_param(accel_dev, ADF_ACCELERATOR0, key.as_ptr(), &val, ADF_DEC); if ret != 0 { return comp_config_error(accel_dev, ret); }
        i += 1;
    }
    val = i as c_ulong; let mut ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, ADF_NUM_DC, &val, ADF_DEC); if ret != 0 { return comp_config_error(accel_dev, ret); }
    val = 0; ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, ADF_NUM_CY, &val, ADF_DEC); if ret != 0 { return comp_config_error(accel_dev, ret); } 0
}

pub unsafe fn adf_no_dev_config(accel_dev: *mut adf_accel_dev) -> c_int {
    let val: c_ulong = 0;
    let ret = adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, ADF_NUM_DC, &val, ADF_DEC);
    if ret != 0 { return ret; }
    adf_cfg_add_key_value_param(accel_dev, ADF_KERNEL_SEC, ADF_NUM_CY, &val, ADF_DEC)
}

pub unsafe fn adf_gen4_dev_config(accel_dev: *mut adf_accel_dev) -> c_int {
    let mut ret = adf_cfg_section_add(accel_dev, ADF_KERNEL_SEC); if ret != 0 { return gen4_config_error(accel_dev, ret); }
    ret = adf_cfg_section_add(accel_dev, ADF_ACCELERATOR0); if ret != 0 { return gen4_config_error(accel_dev, ret); }
    ret = match adf_get_service_enabled(accel_dev) { SVC_SYM_ASYM => adf_crypto_dev_config(accel_dev), SVC_DC | SVC_DCC => adf_comp_dev_config(accel_dev), _ => adf_no_dev_config(accel_dev) };
    if ret != 0 { return gen4_config_error(accel_dev, ret); }
    set_bit(ADF_STATUS_CONFIGURED, &mut (*accel_dev).status); ret
}

pub unsafe fn adf_gen4_cfg_dev_init(accel_dev: *mut adf_accel_dev) -> c_int {
    let config = if (*accel_dev).accel_id % 2 != 0 { ADF_CFG_DC } else { ADF_CFG_CY };
    let ret = adf_cfg_section_add(accel_dev, ADF_GENERAL_SEC); if ret != 0 { return ret; }
    let ret = adf_cfg_add_key_value_param(accel_dev, ADF_GENERAL_SEC, ADF_SERVICES_ENABLED, config, ADF_STR); if ret != 0 { return ret; }
    adf_heartbeat_save_cfg_param(accel_dev, ADF_CFG_HB_TIMER_MIN_MS); 0
}

extern "C" {
    fn snprintf_key(key: *mut c_char, key_len: usize, prefix: *const c_char, index: c_int, suffix: *const c_char);
    fn crypto_config_error(accel_dev: *mut adf_accel_dev, ret: c_int) -> c_int;
    fn comp_config_error(accel_dev: *mut adf_accel_dev, ret: c_int) -> c_int;
    fn gen4_config_error(accel_dev: *mut adf_accel_dev, ret: c_int) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
