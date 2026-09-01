/* SPDX-License-Identifier: GPL-2.0
 *
 * CS35L41 ALSA HDA audio driver
 *
 * Copyright 2021 Cirrus Logic, Inc.
 *
 * Author: Lucas Tanure <tanureal@opensource.cirrus.com>
 */

/* C header dependencies:
 * linux/acpi.h
 * linux/efi.h
 * linux/regulator/consumer.h
 * linux/gpio/consumer.h
 * linux/device.h
 * sound/cs35l41.h
 * sound/cs-amp-lib.h
 * linux/firmware/cirrus/cs_dsp.h
 * linux/firmware/cirrus/wmfw.h
 */

use core::ffi::{c_char, c_int, c_ulong};

pub const CS35L41_MAX_ACCEPTABLE_SPI_SPEED_HZ: u32 = 1000000;
pub const DEFAULT_AMP_GAIN_PCM: u32 = 17; /* 17.5dB Gain */
pub const DEFAULT_AMP_GAIN_PDM: u32 = 19; /* 19.5dB Gain */

#[repr(C, packed)]
pub struct cs35l41_amp_cal_data {
    pub calTarget: [u32; 2],
    pub calTime: [u32; 2],
    pub calAmbient: i8,
    pub calStatus: u8,
    pub calR: u16,
}

#[repr(C, packed)]
pub struct cs35l41_amp_efi_data {
    pub size: u32,
    pub count: u32,
    /* Flexible array member: struct cs35l41_amp_cal_data data[]; */
    pub data: [cs35l41_amp_cal_data; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs35l41_hda_spk_pos {
    CS35L41_LEFT,
    CS35L41_RIGHT,
    CS35L41_CENTER,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cs35l41_hda_gpio_function {
    CS35L41_NOT_USED,
    CS35l41_VSPK_SWITCH,
    CS35L41_INTERRUPT,
    CS35l41_SYNC,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum control_bus {
    I2C,
    SPI,
}

/* Forward declaration from C: struct snd_kcontrol; */
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cs35l41_hda {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub cs_gpio: *mut gpio_desc,
    pub hw_cfg: cs35l41_hw_cfg,
    pub codec: *mut hda_codec,

    pub irq: c_int,
    pub index: c_int,
    pub channel_index: c_int,
    pub irq_errors: c_ulong,
    pub amp_name: *const c_char,
    pub acpi_subsystem_id: *const c_char,
    pub firmware_type: c_int,
    pub speaker_id: c_int,
    pub fw_mutex: mutex,
    pub fw_load_work: work_struct,
    pub fw_type_ctl: *mut snd_kcontrol,
    pub fw_load_ctl: *mut snd_kcontrol,
    pub mute_override_ctl: *mut snd_kcontrol,

    pub irq_data: *mut regmap_irq_chip_data,
    pub firmware_running: bool,
    pub request_fw_load: bool,
    pub fw_request_ongoing: bool,
    pub halo_initialized: bool,
    pub playback_started: bool,
    pub cs_dsp: cs_dsp,
    pub dacpi: *mut acpi_device,
    pub mute_override: bool,
    pub control_bus: control_bus,
    pub bypass_fw: bool,
    pub tuning_gain: u32,
    pub cal_data: cirrus_amp_cal_data,
    pub cal_data_valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum halo_state {
    HALO_STATE_CODE_INIT_DOWNLOAD = 0,
    HALO_STATE_CODE_START,
    HALO_STATE_CODE_RUN,
}

unsafe extern "C" {
    pub static cs35l41_hda_pm_ops: dev_pm_ops;

    pub fn cs35l41_hda_probe(
        dev: *mut device,
        device_name: *const c_char,
        id: c_int,
        irq: c_int,
        regmap: *mut regmap,
        control_bus: control_bus,
    ) -> c_int;
    pub fn cs35l41_hda_remove(dev: *mut device);
    pub fn cs35l41_get_speaker_id(
        dev: *mut device,
        amp_index: c_int,
        num_amps: c_int,
        fixed_gpio_id: c_int,
    ) -> c_int;
    pub fn cs35l41_hda_parse_acpi(
        cs35l41: *mut cs35l41_hda,
        physdev: *mut device,
        id: c_int,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
