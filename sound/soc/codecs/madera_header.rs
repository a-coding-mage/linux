/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Cirrus Logic Madera class codecs common support
 *
 * Copyright (C) 2015-2018 Cirrus Logic, Inc. and
 *                         Cirrus Logic International Semiconductor Ltd.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong};

pub const MADERA_FLL1_REFCLK: c_int = 1;
pub const MADERA_FLL2_REFCLK: c_int = 2;
pub const MADERA_FLL3_REFCLK: c_int = 3;
pub const MADERA_FLLAO_REFCLK: c_int = 4;
pub const MADERA_FLL1_SYNCCLK: c_int = 5;
pub const MADERA_FLL2_SYNCCLK: c_int = 6;
pub const MADERA_FLL3_SYNCCLK: c_int = 7;
pub const MADERA_FLLAO_SYNCCLK: c_int = 8;

pub const MADERA_FLL_SRC_NONE: c_int = -1;
pub const MADERA_FLL_SRC_MCLK1: c_int = 0;
pub const MADERA_FLL_SRC_MCLK2: c_int = 1;
pub const MADERA_FLL_SRC_MCLK3: c_int = 2;
pub const MADERA_FLL_SRC_SLIMCLK: c_int = 3;
pub const MADERA_FLL_SRC_FLL1: c_int = 4;
pub const MADERA_FLL_SRC_FLL2: c_int = 5;
pub const MADERA_FLL_SRC_AIF1BCLK: c_int = 8;
pub const MADERA_FLL_SRC_AIF2BCLK: c_int = 9;
pub const MADERA_FLL_SRC_AIF3BCLK: c_int = 10;
pub const MADERA_FLL_SRC_AIF4BCLK: c_int = 11;
pub const MADERA_FLL_SRC_AIF1LRCLK: c_int = 12;
pub const MADERA_FLL_SRC_AIF2LRCLK: c_int = 13;
pub const MADERA_FLL_SRC_AIF3LRCLK: c_int = 14;
pub const MADERA_FLL_SRC_AIF4LRCLK: c_int = 15;

pub const MADERA_CLK_SYSCLK_1: c_int = 1;
pub const MADERA_CLK_ASYNCCLK_1: c_int = 2;
pub const MADERA_CLK_OPCLK: c_int = 3;
pub const MADERA_CLK_ASYNC_OPCLK: c_int = 4;
pub const MADERA_CLK_SYSCLK_2: c_int = 5;
pub const MADERA_CLK_SYSCLK_3: c_int = 6;
pub const MADERA_CLK_ASYNCCLK_2: c_int = 7;
pub const MADERA_CLK_DSPCLK: c_int = 8;
pub const MADERA_CLK_OUTCLK: c_int = 9;

pub const MADERA_CLK_SRC_MCLK1: c_int = 0x0;
pub const MADERA_CLK_SRC_MCLK2: c_int = 0x1;
pub const MADERA_CLK_SRC_MCLK3: c_int = 0x2;
pub const MADERA_CLK_SRC_FLL1: c_int = 0x4;
pub const MADERA_CLK_SRC_FLL2: c_int = 0x5;
pub const MADERA_CLK_SRC_FLL3: c_int = 0x6;
pub const MADERA_CLK_SRC_FLLAO_HI: c_int = 0x7;
pub const MADERA_CLK_SRC_FLL1_DIV6: c_int = 0x7;
pub const MADERA_CLK_SRC_AIF1BCLK: c_int = 0x8;
pub const MADERA_CLK_SRC_AIF2BCLK: c_int = 0x9;
pub const MADERA_CLK_SRC_AIF3BCLK: c_int = 0xA;
pub const MADERA_CLK_SRC_AIF4BCLK: c_int = 0xB;
pub const MADERA_CLK_SRC_FLLAO: c_int = 0xF;

pub const MADERA_OUTCLK_SYSCLK: c_int = 0;
pub const MADERA_OUTCLK_ASYNCCLK: c_int = 1;
pub const MADERA_OUTCLK_MCLK1: c_int = 4;
pub const MADERA_OUTCLK_MCLK2: c_int = 5;
pub const MADERA_OUTCLK_MCLK3: c_int = 6;

pub const MADERA_MIXER_VOL_MASK: c_int = 0x00FE;
pub const MADERA_MIXER_VOL_SHIFT: c_int = 1;
pub const MADERA_MIXER_VOL_WIDTH: c_int = 7;

pub const MADERA_DOM_GRP_FX: c_int = 0;
pub const MADERA_DOM_GRP_ASRC1: c_int = 1;
pub const MADERA_DOM_GRP_ASRC2: c_int = 2;
pub const MADERA_DOM_GRP_ISRC1: c_int = 3;
pub const MADERA_DOM_GRP_ISRC2: c_int = 4;
pub const MADERA_DOM_GRP_ISRC3: c_int = 5;
pub const MADERA_DOM_GRP_ISRC4: c_int = 6;
pub const MADERA_DOM_GRP_OUT: c_int = 7;
pub const MADERA_DOM_GRP_SPD: c_int = 8;
pub const MADERA_DOM_GRP_DSP1: c_int = 9;
pub const MADERA_DOM_GRP_DSP2: c_int = 10;
pub const MADERA_DOM_GRP_DSP3: c_int = 11;
pub const MADERA_DOM_GRP_DSP4: c_int = 12;
pub const MADERA_DOM_GRP_DSP5: c_int = 13;
pub const MADERA_DOM_GRP_DSP6: c_int = 14;
pub const MADERA_DOM_GRP_DSP7: c_int = 15;
pub const MADERA_DOM_GRP_AIF1: c_int = 16;
pub const MADERA_DOM_GRP_AIF2: c_int = 17;
pub const MADERA_DOM_GRP_AIF3: c_int = 18;
pub const MADERA_DOM_GRP_AIF4: c_int = 19;
pub const MADERA_DOM_GRP_SLIMBUS: c_int = 20;
pub const MADERA_DOM_GRP_PWM: c_int = 21;
pub const MADERA_DOM_GRP_DFC: c_int = 22;
pub const MADERA_N_DOM_GRPS: usize = 23;

pub const MADERA_MAX_DAI: usize = 11;
pub const MADERA_MAX_ADSP: usize = 7;

pub const MADERA_NUM_MIXER_INPUTS: usize = 148;

pub const MADERA_RATES: c_uint = SNDRV_PCM_RATE_KNOT;
pub const MADERA_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

pub const MADERA_OSR_ENUM_SIZE: usize = 5;
pub const MADERA_SYNC_RATE_ENUM_SIZE: usize = 3;
pub const MADERA_ASYNC_RATE_ENUM_SIZE: usize = 2;
pub const MADERA_RATE_ENUM_SIZE: usize = MADERA_SYNC_RATE_ENUM_SIZE + MADERA_ASYNC_RATE_ENUM_SIZE;
pub const MADERA_SAMPLE_RATE_ENUM_SIZE: usize = 16;
pub const MADERA_DFC_TYPE_ENUM_SIZE: usize = 5;
pub const MADERA_DFC_WIDTH_ENUM_SIZE: usize = 5;

#[repr(C)]
pub struct madera {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct wm_adsp {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct soc_enum {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    _unused: [u8; 0],
}

pub type irq_handler_t = Option<unsafe extern "C" fn()>;

unsafe extern "C" {
    pub static SNDRV_PCM_RATE_KNOT: c_uint;
    pub static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    pub static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    pub static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    pub static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    pub static MADERA_MAX_AIF: usize;
}

#[repr(C)]
pub struct madera_voice_trigger_info {
    /** Which core triggered, 1-based (1 = DSP1, ...) */
    pub core_num: c_int,
}

#[repr(C)]
pub struct madera_dai_priv {
    pub clk: c_int,
    pub constraint: snd_pcm_hw_constraint_list,
}

#[repr(C)]
pub struct madera_priv {
    pub adsp: [wm_adsp; MADERA_MAX_ADSP],
    pub madera: *mut madera,
    pub dev: *mut device,
    pub sysclk: c_int,
    pub asyncclk: c_int,
    pub dspclk: c_int,
    pub dai: [madera_dai_priv; MADERA_MAX_DAI],

    pub num_inputs: c_int,

    pub in_pending: c_uint,

    pub out_up_pending: c_uint,
    pub out_up_delay: c_uint,
    pub out_down_pending: c_uint,
    pub out_down_delay: c_uint,

    pub adsp_rate_cache: [c_uint; MADERA_MAX_ADSP],

    pub rate_lock: mutex,

    pub tdm_width: [c_int; MADERA_MAX_AIF],
    pub tdm_slots: [c_int; MADERA_MAX_AIF],

    pub domain_group_ref: [c_int; MADERA_N_DOM_GRPS],
}

#[repr(C)]
pub struct madera_fll_cfg {
    pub n: c_int,
    pub theta: c_uint,
    pub lambda: c_uint,
    pub refdiv: c_int,
    pub fratio: c_int,
    pub gain: c_int,
    pub alt_gain: c_int,
}

#[repr(C)]
pub struct madera_fll {
    pub madera: *mut madera,
    pub id: c_int,
    pub base: c_uint,

    pub fout: c_uint,

    pub sync_src: c_int,
    pub sync_freq: c_uint,

    pub ref_src: c_int,
    pub ref_freq: c_uint,
    pub ref_cfg: madera_fll_cfg,
}

#[repr(C)]
pub struct madera_enum {
    pub mixer_enum: soc_enum,
    pub val: c_int,
}

/*
 * The C header defines many ASoC construction macros:
 * MADERA_GAINMUX_CONTROLS, MADERA_MIXER_CONTROLS, MADERA_MUX_ENUM_DECL,
 * MADERA_MUX_CTL_DECL, MADERA_MUX_ENUMS, MADERA_MIXER_ENUMS,
 * MADERA_DSP_AUX_ENUMS, MADERA_MUX, MADERA_MUX_WIDGETS,
 * MADERA_MIXER_WIDGETS, MADERA_DSP_WIDGETS, MADERA_MUX_ROUTES,
 * MADERA_MIXER_ROUTES, MADERA_DSP_ROUTES, MADERA_RATE_ENUM,
 * MADERA_EQ_CONTROL, and MADERA_LHPF_CONTROL.
 *
 * They expand to external Linux/ASoC macros, token-pasted identifiers,
 * designated initializers, and compound literals. There is no direct
 * file-local Rust item with equivalent expansion semantics, so their intent
 * is preserved here for users of the translated declarations.
 */

unsafe extern "C" {
    pub static madera_ana_tlv: [c_uint; 0];
    pub static madera_eq_tlv: [c_uint; 0];
    pub static madera_digital_tlv: [c_uint; 0];
    pub static madera_noise_tlv: [c_uint; 0];
    pub static madera_ng_tlv: [c_uint; 0];

    pub static madera_mixer_tlv: [c_uint; 0];
    pub static madera_mixer_texts: [*const c_char; MADERA_NUM_MIXER_INPUTS];
    pub static madera_mixer_values: [c_uint; MADERA_NUM_MIXER_INPUTS];

    pub static madera_dai_ops: snd_soc_dai_ops;
    pub static madera_simple_dai_ops: snd_soc_dai_ops;

    pub static madera_inmux: [snd_kcontrol_new; 0];
    pub static madera_inmode: [snd_kcontrol_new; 0];

    pub static madera_rate_text: [*const c_char; MADERA_RATE_ENUM_SIZE];
    pub static madera_rate_val: [c_uint; MADERA_RATE_ENUM_SIZE];

    pub static madera_sample_rate: [soc_enum; 0];
    pub static madera_isrc_fsl: [soc_enum; 0];
    pub static madera_isrc_fsh: [soc_enum; 0];
    pub static madera_asrc1_rate: [soc_enum; 0];
    pub static madera_asrc1_bidir_rate: [soc_enum; 0];
    pub static madera_asrc2_rate: [soc_enum; 0];
    pub static madera_dfc_width: [soc_enum; 0];
    pub static madera_dfc_type: [soc_enum; 0];

    pub static madera_in_vi_ramp: soc_enum;
    pub static madera_in_vd_ramp: soc_enum;

    pub static madera_out_vi_ramp: soc_enum;
    pub static madera_out_vd_ramp: soc_enum;

    pub static madera_lhpf1_mode: soc_enum;
    pub static madera_lhpf2_mode: soc_enum;
    pub static madera_lhpf3_mode: soc_enum;
    pub static madera_lhpf4_mode: soc_enum;

    pub static madera_ng_hold: soc_enum;
    pub static madera_in_hpf_cut_enum: soc_enum;
    pub static madera_in_dmic_osr: [soc_enum; 0];

    pub static madera_output_anc_src: [soc_enum; 0];
    pub static madera_anc_input_src: [soc_enum; 0];
    pub static madera_anc_ng_enum: soc_enum;

    pub static madera_dsp_trigger_output_mux: [snd_kcontrol_new; 0];
    pub static madera_drc_activity_output_mux: [snd_kcontrol_new; 0];

    pub static madera_adsp_rate_controls: [snd_kcontrol_new; 0];

    pub fn madera_dfc_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;

    pub fn madera_lp_mode_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;

    pub fn madera_out1_demux_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    pub fn madera_out1_demux_get(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;

    pub fn madera_rate_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;

    pub fn madera_eq_coeff_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    pub fn madera_lhpf_coeff_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;

    pub fn madera_clk_ev(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: c_int,
    ) -> c_int;
    pub fn madera_sysclk_ev(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: c_int,
    ) -> c_int;
    pub fn madera_spk_ev(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: c_int,
    ) -> c_int;
    pub fn madera_in_ev(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: c_int,
    ) -> c_int;
    pub fn madera_out_ev(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: c_int,
    ) -> c_int;
    pub fn madera_hp_ev(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: c_int,
    ) -> c_int;
    pub fn madera_anc_ev(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: c_int,
    ) -> c_int;
    pub fn madera_domain_clk_ev(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: c_int,
    ) -> c_int;

    pub fn madera_set_adsp_clk(
        priv_: *mut madera_priv,
        dsp_num: c_int,
        freq: c_uint,
    ) -> c_int;

    pub fn madera_set_sysclk(
        component: *mut snd_soc_component,
        clk_id: c_int,
        source: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;

    pub fn madera_init_fll(
        madera: *mut madera,
        id: c_int,
        base: c_int,
        fll: *mut madera_fll,
    ) -> c_int;
    pub fn madera_set_fll_refclk(
        fll: *mut madera_fll,
        source: c_int,
        fref: c_uint,
        fout: c_uint,
    ) -> c_int;
    pub fn madera_set_fll_syncclk(
        fll: *mut madera_fll,
        source: c_int,
        fref: c_uint,
        fout: c_uint,
    ) -> c_int;
    pub fn madera_set_fll_ao_refclk(
        fll: *mut madera_fll,
        source: c_int,
        fin: c_uint,
        fout: c_uint,
    ) -> c_int;
    pub fn madera_fllhj_set_refclk(
        fll: *mut madera_fll,
        source: c_int,
        fin: c_uint,
        fout: c_uint,
    ) -> c_int;

    pub fn madera_core_init(priv_: *mut madera_priv) -> c_int;
    pub fn madera_core_free(priv_: *mut madera_priv) -> c_int;
    pub fn madera_init_overheat(priv_: *mut madera_priv) -> c_int;
    pub fn madera_free_overheat(priv_: *mut madera_priv) -> c_int;
    pub fn madera_init_inputs(component: *mut snd_soc_component) -> c_int;
    pub fn madera_init_outputs(
        component: *mut snd_soc_component,
        routes: *const snd_soc_dapm_route,
        n_mono_routes: c_int,
        n_real: c_int,
    ) -> c_int;
    pub fn madera_init_bus_error_irq(
        priv_: *mut madera_priv,
        dsp_num: c_int,
        handler: irq_handler_t,
    ) -> c_int;
    pub fn madera_free_bus_error_irq(priv_: *mut madera_priv, dsp_num: c_int);

    pub fn madera_init_dai(priv_: *mut madera_priv, id: c_int) -> c_int;

    pub fn madera_set_output_mode(
        component: *mut snd_soc_component,
        output: c_int,
        differential: bool,
    ) -> c_int;

    pub fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut core::ffi::c_void;
    pub fn blocking_notifier_chain_register(
        nh: *mut core::ffi::c_void,
        nb: *mut notifier_block,
    ) -> c_int;
    pub fn blocking_notifier_chain_unregister(
        nh: *mut core::ffi::c_void,
        nb: *mut notifier_block,
    ) -> c_int;
}

unsafe extern "C" {
    /*
     * This symbol models the address of struct madera::notifier used by the
     * C inline helpers. The complete struct madera layout is supplied by the
     * external dependency that owns struct madera.
     */
    pub fn madera_notifier(madera: *mut madera) -> *mut core::ffi::c_void;
}

/* Following functions are for use by machine drivers */
pub unsafe fn madera_register_notifier(
    component: *mut snd_soc_component,
    nb: *mut notifier_block,
) -> c_int {
    let priv_ = unsafe { snd_soc_component_get_drvdata(component) as *mut madera_priv };
    let madera = unsafe { (*priv_).madera };

    unsafe { blocking_notifier_chain_register(madera_notifier(madera), nb) }
}

pub unsafe fn madera_unregister_notifier(
    component: *mut snd_soc_component,
    nb: *mut notifier_block,
) -> c_int {
    let priv_ = unsafe { snd_soc_component_get_drvdata(component) as *mut madera_priv };
    let madera = unsafe { (*priv_).madera };

    unsafe { blocking_notifier_chain_unregister(madera_notifier(madera), nb) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
