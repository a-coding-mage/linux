/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Driver for Cirrus Logic CS35L56 smart amp
 *
 * Copyright (C) 2023 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 */

// C header dependencies:
// linux/completion.h
// linux/container_of.h
// linux/regulator/consumer.h
// linux/pm_runtime.h
// linux/workqueue.h
// sound/cs35l56.h
// "wm_adsp.h"

pub const CS35L56_SDW_GEN_INT_STAT_1: u32 = 0xc0;
pub const CS35L56_SDW_GEN_INT_MASK_1: u32 = 0xc1;
pub const CS35L56_SDW_INT_MASK_CODEC_IRQ: u32 = 1u32 << 0;

pub const CS35L56_RX_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE;
pub const CS35L56_TX_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

pub const CS35L56_RATES: u32 = SNDRV_PCM_RATE_48000;

#[repr(C)]
pub struct sdw_slave {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cs35l56_private {
    pub dsp: wm_adsp, /* must be first member */
    pub base: cs35l56_base,
    pub dsp_work: work_struct,
    pub dsp_wq: *mut workqueue_struct,
    pub component: *mut snd_soc_component,
    pub supplies: [regulator_bulk_data; CS35L56_NUM_BULK_SUPPLIES],
    pub sdw_peripheral: *mut sdw_slave,
    pub sdw_bus_regmap: *mut regmap,
    pub fallback_fw_suffix: *const ::core::ffi::c_char,
    pub soft_resetting: bool,
    pub sdw_attached: bool,
    pub init_completion: completion,

    pub speaker_id: ::core::ffi::c_int,
    pub rx_mask: u32,
    pub tx_mask: u32,
    pub asp_slot_width: u8,
    pub asp_slot_count: u8,
    pub tdm_mode: bool,
    pub sysclk_set: bool,
    pub sdw_link_num: u8,
    pub sdw_unique_id: u8,

    pub ambient_ctl_value: u8,
}

#[inline]
pub unsafe fn cs35l56_private_from_base(
    cs35l56_base: *mut cs35l56_base,
) -> *mut cs35l56_private {
    (cs35l56_base as *mut u8).sub(::core::mem::offset_of!(cs35l56_private, base))
        as *mut cs35l56_private
}

unsafe extern "C" {
    pub static cs35l56_pm_ops_i2c_spi: dev_pm_ops;

    pub fn cs35l56_mask_soundwire_interrupts(cs35l56: *mut cs35l56_private);
    pub fn cs35l56_unmask_soundwire_interrupts(cs35l56: *mut cs35l56_private);

    pub fn cs35l56_system_suspend(dev: *mut device) -> ::core::ffi::c_int;
    pub fn cs35l56_system_suspend_late(dev: *mut device) -> ::core::ffi::c_int;
    pub fn cs35l56_system_suspend_no_irq(dev: *mut device) -> ::core::ffi::c_int;
    pub fn cs35l56_system_resume_no_irq(dev: *mut device) -> ::core::ffi::c_int;
    pub fn cs35l56_system_resume_early(dev: *mut device) -> ::core::ffi::c_int;
    pub fn cs35l56_system_resume(dev: *mut device) -> ::core::ffi::c_int;
    pub fn cs35l56_irq_request(
        cs35l56_base: *mut cs35l56_base,
        irq: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn cs35l56_common_probe(
        cs35l56: *mut cs35l56_private,
        irq: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn cs35l56_init(cs35l56: *mut cs35l56_private) -> ::core::ffi::c_int;
    pub fn cs35l56_remove(cs35l56: *mut cs35l56_private);

    // C conditional: #if IS_ENABLED(CONFIG_KUNIT)
    pub fn cs35l56_set_fw_suffix(cs35l56: *mut cs35l56_private) -> ::core::ffi::c_int;
    pub fn cs35l56_set_fw_name(component: *mut snd_soc_component) -> ::core::ffi::c_int;
    pub fn cs35l56_process_xu_properties(cs35l56: *mut cs35l56_private)
        -> ::core::ffi::c_int;
    pub fn cs35l56_get_firmware_uid(cs35l56: *mut cs35l56_private) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
