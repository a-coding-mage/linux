/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Intel SoC PMIC Driver
 *
 * Copyright (C) 2012-2014 Intel Corporation. All rights reserved.
 *
 * Author: Yang, Bin <bin.yang@intel.com>
 * Author: Zhu, Lejun <lejun.zhu@linux.intel.com>
 */

// Dependency intent: the C header includes <linux/regmap.h>.

#[repr(i32)]
pub enum intel_cht_wc_models {
    INTEL_CHT_WC_UNKNOWN,
    INTEL_CHT_WC_GPD_WIN_POCKET,
    INTEL_CHT_WC_XIAOMI_MIPAD2,
    INTEL_CHT_WC_LENOVO_YOGABOOK1,
    INTEL_CHT_WC_LENOVO_YT3_X90,
}

/**
 * struct intel_soc_pmic - Intel SoC PMIC data
 * @irq: Master interrupt number of the parent PMIC device
 * @regmap: Pointer to the parent PMIC device regmap structure
 * @irq_chip_data: IRQ chip data for the PMIC itself
 * @irq_chip_data_pwrbtn: Chained IRQ chip data for the Power Button
 * @irq_chip_data_tmu: Chained IRQ chip data for the Time Management Unit
 * @irq_chip_data_bcu: Chained IRQ chip data for the Burst Control Unit
 * @irq_chip_data_adc: Chained IRQ chip data for the General Purpose ADC
 * @irq_chip_data_chgr: Chained IRQ chip data for the External Charger
 * @irq_chip_data_crit: Chained IRQ chip data for the Critical Event Handler
 * @dev: Pointer to the parent PMIC device
 * @scu: Pointer to the SCU IPC device data structure
 */
#[repr(C)]
pub struct intel_soc_pmic {
    pub irq: core::ffi::c_int,
    pub regmap: *mut regmap,
    pub irq_chip_data: *mut regmap_irq_chip_data,
    pub irq_chip_data_pwrbtn: *mut regmap_irq_chip_data,
    pub irq_chip_data_tmu: *mut regmap_irq_chip_data,
    pub irq_chip_data_bcu: *mut regmap_irq_chip_data,
    pub irq_chip_data_adc: *mut regmap_irq_chip_data,
    pub irq_chip_data_chgr: *mut regmap_irq_chip_data,
    pub irq_chip_data_crit: *mut regmap_irq_chip_data,
    pub dev: *mut device,
    pub scu: *mut intel_scu_ipc_dev,
    pub cht_wc_model: intel_cht_wc_models,
}

extern "C" {
    pub fn intel_soc_pmic_exec_mipi_pmic_seq_element(
        i2c_address: u16,
        reg_address: u32,
        value: u32,
        mask: u32,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
