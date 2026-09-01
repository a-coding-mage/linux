/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm_hubs.h  --  WM899x common code
 *
 * Copyright 2009 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

/* Dependencies in the original C header:
 * <linux/completion.h>
 * <linux/interrupt.h>
 * <linux/list.h>
 * <sound/control.h>
 */

pub const WM_HUBS_SPKMIX_TLV_INCOMPLETE_ARRAY: &str =
    "extern const unsigned int wm_hubs_spkmix_tlv[]";

unsafe extern "C" {
    pub static wm_hubs_spkmix_tlv: [::core::ffi::c_uint; 0];
}

/* This *must* be the first element of the codec->private_data struct */
#[repr(C)]
pub struct wm_hubs_data {
    pub dcs_codes_l: ::core::ffi::c_int,
    pub dcs_codes_r: ::core::ffi::c_int,
    pub dcs_readback_mode: ::core::ffi::c_int,
    pub hp_startup_mode: ::core::ffi::c_int,
    pub series_startup: ::core::ffi::c_int,
    pub no_series_update: ::core::ffi::c_int,
    pub micd_scthr: bool,

    pub no_cache_dac_hp_direct: bool,
    pub dcs_cache: list_head,
    pub check_class_w_digital:
        Option<unsafe extern "C" fn(*mut snd_soc_component) -> bool>,

    pub micb1_delay: ::core::ffi::c_int,
    pub micb2_delay: ::core::ffi::c_int,

    pub lineout1_se: bool,
    pub lineout1n_ena: bool,
    pub lineout1p_ena: bool,

    pub lineout2_se: bool,
    pub lineout2n_ena: bool,
    pub lineout2p_ena: bool,

    pub dcs_done_irq: bool,
    pub dcs_done: completion,

    pub component: *mut snd_soc_component,
}

unsafe extern "C" {
    pub fn wm_hubs_add_analogue_controls(
        arg1: *mut snd_soc_component,
    ) -> ::core::ffi::c_int;
    pub fn wm_hubs_add_analogue_routes(
        arg1: *mut snd_soc_component,
        arg2: ::core::ffi::c_int,
        arg3: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn wm_hubs_handle_analogue_pdata(
        arg1: *mut snd_soc_component,
        lineout1_diff: ::core::ffi::c_int,
        lineout2_diff: ::core::ffi::c_int,
        lineout1fb: ::core::ffi::c_int,
        lineout2fb: ::core::ffi::c_int,
        jd_scthr: ::core::ffi::c_int,
        jd_thr: ::core::ffi::c_int,
        micbias1_delay: ::core::ffi::c_int,
        micbias2_delay: ::core::ffi::c_int,
        micbias1_lvl: ::core::ffi::c_int,
        micbias2_lvl: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn wm_hubs_dcs_done(
        irq: ::core::ffi::c_int,
        data: *mut ::core::ffi::c_void,
    ) -> irqreturn_t;
    pub fn wm_hubs_vmid_ena(component: *mut snd_soc_component);
    pub fn wm_hubs_set_bias_level(
        component: *mut snd_soc_component,
        level: snd_soc_bias_level,
    );
    pub fn wm_hubs_update_class_w(component: *mut snd_soc_component);

    pub static wm_hubs_hpl_mux: snd_kcontrol_new;
    pub static wm_hubs_hpr_mux: snd_kcontrol_new;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
