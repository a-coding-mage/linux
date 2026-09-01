// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm_adsp.h  --  Wolfson ADSP support
 *
 * Copyright 2012 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

// C header dependencies:
// linux/firmware/cirrus/cs_dsp.h
// linux/firmware/cirrus/wmfw.h
// sound/soc.h
// sound/soc-dapm.h
// sound/compress_driver.h

use core::ffi::{c_char, c_int, c_uint, c_void};

/* Return values for wm_adsp_compr_handle_irq */
pub const WM_ADSP_COMPR_OK: c_int = 0;
pub const WM_ADSP_COMPR_VOICE_TRIGGER: c_int = 1;

#[repr(C)]
pub struct wm_adsp_compr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wm_adsp_compr_buf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cs_dsp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cs_dsp_coeff_ctl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct firmware {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_caps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_compr_tstamp64 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub type irqreturn_t = c_uint;
pub type size_t = usize;

#[repr(C)]
pub struct wm_adsp {
    pub cs_dsp: cs_dsp,
    pub part: *const c_char,
    pub fwf_name: *const c_char,
    pub system_name: *const c_char,
    pub fwf_suffix: *const c_char,
    pub component: *mut snd_soc_component,

    pub sys_config_size: c_uint,

    pub fw: c_int,
    pub wmfw_optional: bool,
    pub bin_mandatory: bool,

    pub boot_work: work_struct,
    pub control_add:
        Option<unsafe extern "C" fn(dsp: *mut wm_adsp, cs_ctl: *mut cs_dsp_coeff_ctl) -> c_int>,
    pub pre_run: Option<unsafe extern "C" fn(dsp: *mut wm_adsp) -> c_int>,

    pub preloaded: bool,
    pub fatal_error: bool,

    pub compr_list: list_head,
    pub buffer_list: list_head,

    /*
     * Flag indicating the preloader widget only needs power toggled
     * on state change rather than held on for the duration of the
     * preload, useful for devices that can retain firmware memory
     * across power down.
     */
    pub toggle_preload: bool,
}

// C macro:
// #define WM_ADSP1(wname, num) \
//     SND_SOC_DAPM_PGA_E(wname, SND_SOC_NOPM, num, 0, NULL, 0, \
//         wm_adsp1_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD)

// C macro:
// #define WM_ADSP2_PRELOAD_SWITCH(wname, num) \
//     SOC_SINGLE_EXT(wname " Preload Switch", SND_SOC_NOPM, num, 1, 0, \
//         wm_adsp2_preloader_get, wm_adsp2_preloader_put)

// C macro:
// #define WM_ADSP2(wname, num, event_fn) \
//     SND_SOC_DAPM_SPK(wname " Preload", NULL), \
// {   .id = snd_soc_dapm_supply, .name = wname " Preloader", \
//     .reg = SND_SOC_NOPM, .shift = num, .event = event_fn, \
//     .event_flags = SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD, \
//     .subseq = 100, /* Ensure we run after SYSCLK supply widget */ }, \
// {   .id = snd_soc_dapm_out_drv, .name = wname, \
//     .reg = SND_SOC_NOPM, .shift = num, .event = wm_adsp_event, \
//     .event_flags = SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD }

// C macro:
// #define WM_ADSP_FW_CONTROL(dspname, num) \
//     SOC_ENUM_EXT(dspname " Firmware", wm_adsp_fw_enum[num], \
//                  wm_adsp_fw_get, wm_adsp_fw_put)

#[repr(C)]
pub struct wm_adsp_fw_file {
    pub firmware: *const firmware,
    pub filename: *mut c_char,
}

#[repr(C)]
pub struct wm_adsp_fw_files {
    pub wmfw: wm_adsp_fw_file,
    pub coeff: wm_adsp_fw_file,
}

unsafe extern "C" {
    pub static wm_adsp_fw_enum: [soc_enum; 0];

    pub fn wm_adsp1_init(dsp: *mut wm_adsp) -> c_int;
    pub fn wm_adsp2_init(dsp: *mut wm_adsp) -> c_int;
    pub fn wm_adsp2_remove(dsp: *mut wm_adsp);
    pub fn wm_adsp2_component_probe(
        dsp: *mut wm_adsp,
        component: *mut snd_soc_component,
    ) -> c_int;
    pub fn wm_adsp2_component_remove(
        dsp: *mut wm_adsp,
        component: *mut snd_soc_component,
    ) -> c_int;
    pub fn wm_halo_init(dsp: *mut wm_adsp) -> c_int;

    pub fn wm_adsp1_event(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: c_int,
    ) -> c_int;

    pub fn wm_adsp_early_event(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: c_int,
    ) -> c_int;

    pub fn wm_adsp_power_up(dsp: *mut wm_adsp, load_firmware: bool) -> c_int;
    pub fn wm_adsp_power_down(dsp: *mut wm_adsp);

    pub fn wm_adsp2_bus_error(irq: c_int, data: *mut c_void) -> irqreturn_t;
    pub fn wm_halo_bus_error(irq: c_int, data: *mut c_void) -> irqreturn_t;
    pub fn wm_halo_wdt_expire(irq: c_int, data: *mut c_void) -> irqreturn_t;

    pub fn wm_adsp_run(dsp: *mut wm_adsp) -> c_int;
    pub fn wm_adsp_stop(dsp: *mut wm_adsp);
    pub fn wm_adsp_hibernate(dsp: *mut wm_adsp, hibernate: bool);
    pub fn wm_adsp_event(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: c_int,
    ) -> c_int;

    pub fn wm_adsp2_set_dspclk(w: *mut snd_soc_dapm_widget, freq: c_uint) -> c_int;

    pub fn wm_adsp2_preloader_get(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    pub fn wm_adsp2_preloader_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    pub fn wm_adsp_fw_get(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    pub fn wm_adsp_fw_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;

    pub fn wm_adsp_compr_open(dsp: *mut wm_adsp, stream: *mut snd_compr_stream) -> c_int;
    pub fn wm_adsp_compr_free(
        component: *mut snd_soc_component,
        stream: *mut snd_compr_stream,
    ) -> c_int;
    pub fn wm_adsp_compr_set_params(
        component: *mut snd_soc_component,
        stream: *mut snd_compr_stream,
        params: *mut snd_compr_params,
    ) -> c_int;
    pub fn wm_adsp_compr_get_caps(
        component: *mut snd_soc_component,
        stream: *mut snd_compr_stream,
        caps: *mut snd_compr_caps,
    ) -> c_int;
    pub fn wm_adsp_compr_trigger(
        component: *mut snd_soc_component,
        stream: *mut snd_compr_stream,
        cmd: c_int,
    ) -> c_int;
    pub fn wm_adsp_compr_handle_irq(dsp: *mut wm_adsp) -> c_int;
    pub fn wm_adsp_compr_pointer(
        component: *mut snd_soc_component,
        stream: *mut snd_compr_stream,
        tstamp: *mut snd_compr_tstamp64,
    ) -> c_int;
    pub fn wm_adsp_compr_copy(
        component: *mut snd_soc_component,
        stream: *mut snd_compr_stream,
        buf: *mut c_char,
        count: size_t,
    ) -> c_int;

    pub fn wm_adsp_control_add(cs_ctl: *mut cs_dsp_coeff_ctl) -> c_int;
    pub fn wm_adsp_write_ctl(
        dsp: *mut wm_adsp,
        name: *const c_char,
        type_: c_int,
        alg: c_uint,
        buf: *mut c_void,
        len: size_t,
    ) -> c_int;
    pub fn wm_adsp_read_ctl(
        dsp: *mut wm_adsp,
        name: *const c_char,
        type_: c_int,
        alg: c_uint,
        buf: *mut c_void,
        len: size_t,
    ) -> c_int;

    // Present in C only when IS_ENABLED(CONFIG_KUNIT).
    pub fn wm_adsp_get_fwf_name_by_index(index: c_int) -> *const c_char;
    pub fn wm_adsp_release_firmware_files(fw: *mut wm_adsp_fw_files);
    pub fn wm_adsp_firmware_request(
        firmware: *mut *const firmware,
        filename: *const c_char,
        dev: *mut device,
    ) -> c_int;
    pub fn wm_adsp_request_firmware_files(
        dsp: *mut wm_adsp,
        fw: *mut wm_adsp_fw_files,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
