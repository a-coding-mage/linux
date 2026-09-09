// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2015 - 2021 Intel Corporation */
// C dependencies are supplied by the surrounding translation unit.

static mut C62XIOV_CLASS: adf_hw_device_class = adf_hw_device_class {
    name: ADF_C62XVF_DEVICE_NAME,
    type_: DEV_C62XVF,
};

unsafe fn get_accel_mask(_self_: *mut adf_hw_device_data) -> u32 {
    ADF_C62XIOV_ACCELERATORS_MASK
}

unsafe fn get_ae_mask(_self_: *mut adf_hw_device_data) -> u32 {
    ADF_C62XIOV_ACCELENGINES_MASK
}

unsafe fn get_num_accels(_self_: *mut adf_hw_device_data) -> u32 {
    ADF_C62XIOV_MAX_ACCELERATORS
}

unsafe fn get_num_aes(_self_: *mut adf_hw_device_data) -> u32 {
    ADF_C62XIOV_MAX_ACCELENGINES
}

unsafe fn get_misc_bar_id(_self_: *mut adf_hw_device_data) -> u32 {
    ADF_C62XIOV_PMISC_BAR
}

unsafe fn get_etr_bar_id(_self_: *mut adf_hw_device_data) -> u32 {
    ADF_C62XIOV_ETR_BAR
}

unsafe fn get_sku(_self_: *mut adf_hw_device_data) -> dev_sku_info {
    DEV_SKU_VF
}

unsafe fn adf_vf_int_noop(_accel_dev: *mut adf_accel_dev) -> i32 {
    0
}

unsafe fn adf_vf_void_noop(_accel_dev: *mut adf_accel_dev) {}

pub unsafe fn adf_init_hw_data_c62xiov(hw_data: *mut adf_hw_device_data) {
    (*hw_data).dev_class = &raw mut C62XIOV_CLASS;
    (*hw_data).num_banks = ADF_C62XIOV_ETR_MAX_BANKS;
    (*hw_data).num_rings_per_bank = ADF_ETR_MAX_RINGS_PER_BANK;
    (*hw_data).num_accel = ADF_C62XIOV_MAX_ACCELERATORS;
    (*hw_data).num_logical_accel = 1;
    (*hw_data).num_engines = ADF_C62XIOV_MAX_ACCELENGINES;
    (*hw_data).tx_rx_gap = ADF_C62XIOV_RX_RINGS_OFFSET;
    (*hw_data).tx_rings_mask = ADF_C62XIOV_TX_RINGS_MASK;
    (*hw_data).ring_to_svc_map = ADF_GEN2_DEFAULT_RING_TO_SRV_MAP;
    (*hw_data).alloc_irq = adf_vf_isr_resource_alloc;
    (*hw_data).free_irq = adf_vf_isr_resource_free;
    (*hw_data).enable_error_correction = adf_vf_void_noop;
    (*hw_data).init_admin_comms = adf_vf_int_noop;
    (*hw_data).exit_admin_comms = adf_vf_void_noop;
    (*hw_data).send_admin_init = adf_vf2pf_notify_init;
    (*hw_data).init_arb = adf_vf_int_noop;
    (*hw_data).exit_arb = adf_vf_void_noop;
    (*hw_data).disable_iov = adf_vf2pf_notify_shutdown;
    (*hw_data).get_accel_mask = get_accel_mask;
    (*hw_data).get_ae_mask = get_ae_mask;
    (*hw_data).get_num_accels = get_num_accels;
    (*hw_data).get_num_aes = get_num_aes;
    (*hw_data).get_etr_bar_id = get_etr_bar_id;
    (*hw_data).get_misc_bar_id = get_misc_bar_id;
    (*hw_data).get_sku = get_sku;
    (*hw_data).enable_ints = adf_vf_void_noop;
    (*(*hw_data).dev_class).instances += 1;
    (*hw_data).dev_config = adf_gen2_dev_config;
    adf_devmgr_update_class_index(hw_data);
    adf_gen2_init_vf_pfvf_ops(&mut (*hw_data).pfvf_ops);
    adf_gen2_init_hw_csr_ops(&mut (*hw_data).csr_ops);
    adf_gen2_init_dc_ops(&mut (*hw_data).dc_ops);
}

pub unsafe fn adf_clean_hw_data_c62xiov(hw_data: *mut adf_hw_device_data) {
    (*(*hw_data).dev_class).instances -= 1;
    adf_devmgr_update_class_index(hw_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
