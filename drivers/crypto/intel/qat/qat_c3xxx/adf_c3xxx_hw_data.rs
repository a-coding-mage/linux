// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2021 Intel Corporation */

// Declarations supplied by the surrounding driver and hardware-specific headers

/* Worker thread to service arbiter mappings */
static THRD_TO_ARB_MAP: [u32; ADF_C3XXX_MAX_ACCELENGINES as usize] = [
    0x12222AAA, 0x11222AAA, 0x12222AAA,
    0x11222AAA, 0x12222AAA, 0x11222AAA,
];

static mut C3XXX_CLASS: adf_hw_device_class = adf_hw_device_class {
    name: ADF_C3XXX_DEVICE_NAME,
    r#type: DEV_C3XXX,
    instances: 0,
};

unsafe fn get_accel_mask(self_: *mut adf_hw_device_data) -> u32 {
    let fuses = (*self_).fuses[ADF_FUSECTL0 as usize];
    let straps = (*self_).straps;
    let mut accel = (!(fuses | straps)) >> ADF_C3XXX_ACCELERATORS_REG_OFFSET;
    accel &= ADF_C3XXX_ACCELERATORS_MASK;
    accel
}

unsafe fn get_ae_mask(self_: *mut adf_hw_device_data) -> u32 {
    let fuses = (*self_).fuses[ADF_FUSECTL0 as usize];
    let mut straps = (*self_).straps;
    let disabled = !get_accel_mask(self_) & ADF_C3XXX_ACCELERATORS_MASK;
    let ae_disable = (1u32 << 1) | (1u32 << 0);

    let mut accel = 0u32;
    while accel < ADF_C3XXX_MAX_ACCELERATORS {
        if (disabled & (1u32 << accel)) != 0 {
            straps |= ae_disable << (accel << 1);
        }
        accel += 1;
    }

    !(fuses | straps) & ADF_C3XXX_ACCELENGINES_MASK
}

unsafe fn get_ts_clock(self_: *mut adf_hw_device_data) -> u32 {
    /* Timestamp update interval is 16 AE clock ticks for c3xxx. */
    (*self_).clock_frequency / 16
}

unsafe fn measure_clock(accel_dev: *mut adf_accel_dev) -> i32 {
    let mut frequency = 0u32;
    let ret = adf_dev_measure_clock(
        accel_dev,
        &mut frequency,
        ADF_C3XXX_MIN_AE_FREQ,
        ADF_C3XXX_MAX_AE_FREQ,
    );
    if ret != 0 {
        return ret;
    }
    (*(*accel_dev).hw_device).clock_frequency = frequency;
    0
}

fn get_misc_bar_id(_self_: *mut adf_hw_device_data) -> u32 { ADF_C3XXX_PMISC_BAR }
fn get_etr_bar_id(_self_: *mut adf_hw_device_data) -> u32 { ADF_C3XXX_ETR_BAR }
fn get_sram_bar_id(_self_: *mut adf_hw_device_data) -> u32 { ADF_C3XXX_SRAM_BAR }

unsafe fn get_sku(self_: *mut adf_hw_device_data) -> dev_sku_info {
    let aes = ((*self_).get_num_aes.unwrap())(self_);
    if aes == 6 { DEV_SKU_4 } else { DEV_SKU_UNKNOWN }
}

fn adf_get_arbiter_mapping(_accel_dev: *mut adf_accel_dev) -> *const u32 {
    THRD_TO_ARB_MAP.as_ptr()
}

fn configure_iov_threads(accel_dev: *mut adf_accel_dev, enable: bool) {
    adf_gen2_cfg_iov_thds(
        accel_dev,
        enable,
        ADF_C3XXX_AE2FUNC_MAP_GRP_A_NUM_REGS,
        ADF_C3XXX_AE2FUNC_MAP_GRP_B_NUM_REGS,
    );
}

pub unsafe fn adf_init_hw_data_c3xxx(hw_data: *mut adf_hw_device_data) {
    (*hw_data).dev_class = &mut C3XXX_CLASS;
    (*hw_data).instance_id = C3XXX_CLASS.instances;
    C3XXX_CLASS.instances += 1;
    (*hw_data).num_banks = ADF_C3XXX_ETR_MAX_BANKS;
    (*hw_data).num_rings_per_bank = ADF_ETR_MAX_RINGS_PER_BANK;
    (*hw_data).num_accel = ADF_C3XXX_MAX_ACCELERATORS;
    (*hw_data).num_logical_accel = 1;
    (*hw_data).num_engines = ADF_C3XXX_MAX_ACCELENGINES;
    (*hw_data).tx_rx_gap = ADF_GEN2_RX_RINGS_OFFSET;
    (*hw_data).tx_rings_mask = ADF_GEN2_TX_RINGS_MASK;
    (*hw_data).ring_to_svc_map = ADF_GEN2_DEFAULT_RING_TO_SRV_MAP;
    (*hw_data).alloc_irq = Some(adf_isr_resource_alloc);
    (*hw_data).free_irq = Some(adf_isr_resource_free);
    (*hw_data).enable_error_correction = Some(adf_gen2_enable_error_correction);
    (*hw_data).get_accel_mask = Some(get_accel_mask);
    (*hw_data).get_ae_mask = Some(get_ae_mask);
    (*hw_data).get_accel_cap = Some(adf_gen2_get_accel_cap);
    (*hw_data).get_num_accels = Some(adf_gen2_get_num_accels);
    (*hw_data).get_num_aes = Some(adf_gen2_get_num_aes);
    (*hw_data).get_sram_bar_id = Some(get_sram_bar_id);
    (*hw_data).get_etr_bar_id = Some(get_etr_bar_id);
    (*hw_data).get_misc_bar_id = Some(get_misc_bar_id);
    (*hw_data).get_admin_info = Some(adf_gen2_get_admin_info);
    (*hw_data).get_arb_info = Some(adf_gen2_get_arb_info);
    (*hw_data).get_sku = Some(get_sku);
    (*hw_data).fw_name = ADF_C3XXX_FW;
    (*hw_data).fw_mmp_name = ADF_C3XXX_MMP;
    (*hw_data).init_admin_comms = Some(adf_init_admin_comms);
    (*hw_data).exit_admin_comms = Some(adf_exit_admin_comms);
    (*hw_data).configure_iov_threads = Some(configure_iov_threads);
    (*hw_data).send_admin_init = Some(adf_send_admin_init);
    (*hw_data).init_arb = Some(adf_init_arb);
    (*hw_data).exit_arb = Some(adf_exit_arb);
    (*hw_data).get_arb_mapping = Some(adf_get_arbiter_mapping);
    (*hw_data).enable_ints = Some(adf_gen2_enable_ints);
    (*hw_data).reset_device = Some(adf_reset_flr);
    (*hw_data).set_ssm_wdtimer = Some(adf_gen2_set_ssm_wdtimer);
    (*hw_data).disable_iov = Some(adf_disable_sriov);
    (*hw_data).dev_config = Some(adf_gen2_dev_config);
    (*hw_data).measure_clock = Some(measure_clock);
    (*hw_data).get_hb_clock = Some(get_ts_clock);
    (*hw_data).num_hb_ctrs = ADF_NUM_HB_CNT_PER_AE;
    (*hw_data).check_hb_ctrs = Some(adf_heartbeat_check_ctrs);

    adf_gen2_init_pf_pfvf_ops(&mut (*hw_data).pfvf_ops);
    adf_gen2_init_hw_csr_ops(&mut (*hw_data).csr_ops);
    adf_gen2_init_dc_ops(&mut (*hw_data).dc_ops);
}

pub unsafe fn adf_clean_hw_data_c3xxx(hw_data: *mut adf_hw_device_data) {
    (*(*hw_data).dev_class).instances -= 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
