/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arizona.h - Wolfson Arizona class device shared support
 *
 * Copyright 2012 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

/* C includes translated as external Rust dependencies:
 * linux/completion.h, linux/notifier.h, linux/mfd/arizona/core.h,
 * sound/soc.h, and "wm_adsp.h".
 */

pub const ARIZONA_CLK_SYSCLK: i32 = 1;
pub const ARIZONA_CLK_ASYNCCLK: i32 = 2;
pub const ARIZONA_CLK_OPCLK: i32 = 3;
pub const ARIZONA_CLK_ASYNC_OPCLK: i32 = 4;

pub const ARIZONA_CLK_SRC_MCLK1: i32 = 0x0;
pub const ARIZONA_CLK_SRC_MCLK2: i32 = 0x1;
pub const ARIZONA_CLK_SRC_FLL1: i32 = 0x4;
pub const ARIZONA_CLK_SRC_FLL2: i32 = 0x5;
pub const ARIZONA_CLK_SRC_AIF1BCLK: i32 = 0x8;
pub const ARIZONA_CLK_SRC_AIF2BCLK: i32 = 0x9;
pub const ARIZONA_CLK_SRC_AIF3BCLK: i32 = 0xa;

pub const ARIZONA_FLL_SRC_NONE: i32 = -1;
pub const ARIZONA_FLL_SRC_MCLK1: i32 = 0;
pub const ARIZONA_FLL_SRC_MCLK2: i32 = 1;
pub const ARIZONA_FLL_SRC_SLIMCLK: i32 = 3;
pub const ARIZONA_FLL_SRC_FLL1: i32 = 4;
pub const ARIZONA_FLL_SRC_FLL2: i32 = 5;
pub const ARIZONA_FLL_SRC_AIF1BCLK: i32 = 8;
pub const ARIZONA_FLL_SRC_AIF2BCLK: i32 = 9;
pub const ARIZONA_FLL_SRC_AIF3BCLK: i32 = 10;
pub const ARIZONA_FLL_SRC_AIF1LRCLK: i32 = 12;
pub const ARIZONA_FLL_SRC_AIF2LRCLK: i32 = 13;
pub const ARIZONA_FLL_SRC_AIF3LRCLK: i32 = 14;

pub const ARIZONA_MIXER_VOL_MASK: u32 = 0x00FE;
pub const ARIZONA_MIXER_VOL_SHIFT: u32 = 1;
pub const ARIZONA_MIXER_VOL_WIDTH: u32 = 7;

pub const ARIZONA_CLK_6MHZ: i32 = 0;
pub const ARIZONA_CLK_12MHZ: i32 = 1;
pub const ARIZONA_CLK_24MHZ: i32 = 2;
pub const ARIZONA_CLK_49MHZ: i32 = 3;
pub const ARIZONA_CLK_73MHZ: i32 = 4;
pub const ARIZONA_CLK_98MHZ: i32 = 5;
pub const ARIZONA_CLK_147MHZ: i32 = 6;

pub const ARIZONA_MAX_DAI: usize = 10;
pub const ARIZONA_MAX_ADSP: usize = 4;

pub const ARIZONA_DVFS_SR1_RQ: u32 = 0x001;
pub const ARIZONA_DVFS_ADSP1_RQ: u32 = 0x100;

/* Notifier events */
pub const ARIZONA_NOTIFY_VOICE_TRIGGER: u32 = 0x1;

#[repr(C)]
pub struct arizona_dai_priv {
    pub clk: ::core::ffi::c_int,

    pub constraint: snd_pcm_hw_constraint_list,
}

#[repr(C)]
pub struct arizona_priv {
    pub adsp: [wm_adsp; ARIZONA_MAX_ADSP],
    pub arizona: *mut arizona,
    pub sysclk: ::core::ffi::c_int,
    pub asyncclk: ::core::ffi::c_int,
    pub dai: [arizona_dai_priv; ARIZONA_MAX_DAI],

    pub num_inputs: ::core::ffi::c_int,
    pub in_pending: ::core::ffi::c_uint,

    pub out_up_pending: ::core::ffi::c_uint,
    pub out_up_delay: ::core::ffi::c_uint,
    pub out_down_pending: ::core::ffi::c_uint,
    pub out_down_delay: ::core::ffi::c_uint,

    pub dvfs_reqs: ::core::ffi::c_uint,
    pub dvfs_lock: mutex,
    pub dvfs_cached: bool,

    /* Variables used by arizona-jack.c code */
    pub lock: mutex,
    pub hpdet_work: delayed_work,
    pub micd_detect_work: delayed_work,
    pub micd_timeout_work: delayed_work,
    pub jack: *mut snd_soc_jack,
    pub micvdd: *mut regulator,
    pub micd_pol_gpio: *mut gpio_desc,
    pub hpdet_id_gpio: *mut gpio_desc,

    pub last_jackdet: u16,

    pub micd_mode: ::core::ffi::c_int,
    pub micd_modes: *const arizona_micd_config,
    pub micd_num_modes: ::core::ffi::c_int,

    pub micd_button_mask: ::core::ffi::c_int,
    pub micd_ranges: *const arizona_micd_range,
    pub num_micd_ranges: ::core::ffi::c_int,

    pub micd_reva: bool,
    pub micd_clamp: bool,

    pub hpdet_active: bool,
    pub hpdet_done: bool,
    pub hpdet_retried: bool,

    pub mic: bool,
    pub detecting: bool,

    pub num_hpdet_res: ::core::ffi::c_int,
    pub hpdet_res: [::core::ffi::c_uint; 3],

    pub jack_flips: ::core::ffi::c_int,
    pub hpdet_ip_version: ::core::ffi::c_int,
}

#[repr(C)]
pub struct arizona_voice_trigger_info {
    pub core: ::core::ffi::c_int,
}

pub const ARIZONA_NUM_MIXER_INPUTS: usize = 104;

unsafe extern "C" {
    pub static arizona_mixer_tlv: [::core::ffi::c_uint; 0];
    pub static arizona_mixer_texts: [*const ::core::ffi::c_char; ARIZONA_NUM_MIXER_INPUTS];
    pub static mut arizona_mixer_values: [::core::ffi::c_uint; ARIZONA_NUM_MIXER_INPUTS];
}

/* ARIZONA_GAINMUX_CONTROLS(name, base):
 * SOC_SINGLE_RANGE_TLV(name " Input Volume", base + 1,
 *                      ARIZONA_MIXER_VOL_SHIFT, 0x20, 0x50, 0,
 *                      arizona_mixer_tlv)
 */

/* ARIZONA_MIXER_CONTROLS(name, base):
 * SOC_SINGLE_RANGE_TLV controls for Input 1..4 Volume at base + 1, +3, +5, +7.
 */

/* ARIZONA_MUX_ENUM_DECL(name, reg):
 * SOC_VALUE_ENUM_SINGLE_AUTODISABLE_DECL(
 *     name, reg, 0, 0xff, arizona_mixer_texts, arizona_mixer_values)
 */

/* ARIZONA_MUX_CTL_DECL(name):
 * const struct snd_kcontrol_new name##_mux = SOC_DAPM_ENUM("Route", name##_enum)
 */

/* ARIZONA_MUX_ENUMS(name, base_reg):
 * static ARIZONA_MUX_ENUM_DECL(name##_enum, base_reg);
 * static ARIZONA_MUX_CTL_DECL(name)
 */

/* ARIZONA_MIXER_ENUMS(name, base_reg):
 * ARIZONA_MUX_ENUMS(name##_in1, base_reg);
 * ARIZONA_MUX_ENUMS(name##_in2, base_reg + 2);
 * ARIZONA_MUX_ENUMS(name##_in3, base_reg + 4);
 * ARIZONA_MUX_ENUMS(name##_in4, base_reg + 6)
 */

/* ARIZONA_DSP_AUX_ENUMS(name, base_reg):
 * ARIZONA_MUX_ENUMS(name##_aux1, base_reg);
 * ARIZONA_MUX_ENUMS(name##_aux2, base_reg + 8);
 * ARIZONA_MUX_ENUMS(name##_aux3, base_reg + 16);
 * ARIZONA_MUX_ENUMS(name##_aux4, base_reg + 24);
 * ARIZONA_MUX_ENUMS(name##_aux5, base_reg + 32);
 * ARIZONA_MUX_ENUMS(name##_aux6, base_reg + 40)
 */

/* ARIZONA_MUX(name, ctrl):
 * SND_SOC_DAPM_MUX(name, SND_SOC_NOPM, 0, 0, ctrl)
 */

/* ARIZONA_MUX_WIDGETS(name, name_str):
 * ARIZONA_MUX(name_str " Input", &name##_mux)
 */

/* ARIZONA_MIXER_WIDGETS(name, name_str):
 * MUX widgets for Input 1..4 and SND_SOC_DAPM_MIXER(name_str " Mixer", ...).
 */

/* ARIZONA_DSP_WIDGETS(name, name_str):
 * Left/right mixer widgets and Aux 1..6 mux widgets.
 */

/* ARIZONA_MUX_ROUTES(widget, name):
 * { widget, NULL, name " Input" }, ARIZONA_MIXER_INPUT_ROUTES(name " Input")
 */

/* ARIZONA_MIXER_ROUTES(widget, name):
 * route widget through name " Mixer", Input 1..4, and mixer input routes.
 */

/* ARIZONA_DSP_ROUTES(name):
 * routes DSP name, preloader/preload, Aux 1..6, mixer inputs, and L/R mixers.
 */

/* ARIZONA_EQ_CONTROL(xname, xbase):
 * snd_kcontrol_new initializer using snd_soc_bytes_info/get,
 * arizona_eq_coeff_put, and private soc_bytes { base = xbase, num_regs = 20,
 * mask = ~ARIZONA_EQ1_B1_MODE }.
 */

/* ARIZONA_LHPF_CONTROL(xname, xbase):
 * snd_kcontrol_new initializer using snd_soc_bytes_info/get,
 * arizona_lhpf_coeff_put, and private soc_bytes { base = xbase, num_regs = 1 }.
 */

pub const ARIZONA_RATE_ENUM_SIZE: usize = 4;
pub const ARIZONA_SAMPLE_RATE_ENUM_SIZE: usize = 14;

/* SND_JACK_* mask for supported cable/switch types */
pub const ARIZONA_JACK_MASK: u32 = SND_JACK_HEADSET | SND_JACK_LINEOUT | SND_JACK_MECHANICAL;

unsafe extern "C" {
    pub static arizona_rate_text: [*const ::core::ffi::c_char; ARIZONA_RATE_ENUM_SIZE];
    pub static arizona_rate_val: [::core::ffi::c_uint; ARIZONA_RATE_ENUM_SIZE];
    pub static arizona_sample_rate_text: [*const ::core::ffi::c_char; ARIZONA_SAMPLE_RATE_ENUM_SIZE];
    pub static arizona_sample_rate_val: [::core::ffi::c_uint; ARIZONA_SAMPLE_RATE_ENUM_SIZE];

    pub static arizona_isrc_fsl: [soc_enum; 0];
    pub static arizona_isrc_fsh: [soc_enum; 0];
    pub static arizona_asrc_rate1: soc_enum;

    pub static arizona_in_vi_ramp: soc_enum;
    pub static arizona_in_vd_ramp: soc_enum;

    pub static arizona_out_vi_ramp: soc_enum;
    pub static arizona_out_vd_ramp: soc_enum;

    pub static arizona_lhpf1_mode: soc_enum;
    pub static arizona_lhpf2_mode: soc_enum;
    pub static arizona_lhpf3_mode: soc_enum;
    pub static arizona_lhpf4_mode: soc_enum;

    pub static arizona_ng_hold: soc_enum;
    pub static arizona_in_hpf_cut_enum: soc_enum;
    pub static arizona_in_dmic_osr: [soc_enum; 0];

    pub static arizona_adsp2_rate_controls: [snd_kcontrol_new; 0];

    pub static arizona_anc_input_src: [soc_enum; 0];
    pub static arizona_anc_ng_enum: soc_enum;
    pub static arizona_output_anc_src: [soc_enum; 0];

    pub static arizona_voice_trigger_switch: [snd_kcontrol_new; 0];

    pub fn arizona_in_ev(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn arizona_out_ev(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn arizona_hp_ev(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn arizona_anc_ev(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn arizona_eq_coeff_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> ::core::ffi::c_int;
    pub fn arizona_lhpf_coeff_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> ::core::ffi::c_int;

    pub fn arizona_clk_ev(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn arizona_set_sysclk(
        component: *mut snd_soc_component,
        clk_id: ::core::ffi::c_int,
        source: ::core::ffi::c_int,
        freq: ::core::ffi::c_uint,
        dir: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub static arizona_dai_ops: snd_soc_dai_ops;
    pub static arizona_simple_dai_ops: snd_soc_dai_ops;
}

pub const ARIZONA_FLL_NAME_LEN: usize = 20;

#[repr(C)]
pub struct arizona_fll {
    pub arizona: *mut arizona,
    pub id: ::core::ffi::c_int,
    pub base: ::core::ffi::c_uint,
    pub vco_mult: ::core::ffi::c_uint,

    pub fout: ::core::ffi::c_uint,
    pub sync_src: ::core::ffi::c_int,
    pub sync_freq: ::core::ffi::c_uint,
    pub ref_src: ::core::ffi::c_int,
    pub ref_freq: ::core::ffi::c_uint,

    pub lock_name: [::core::ffi::c_char; ARIZONA_FLL_NAME_LEN],
    pub clock_ok_name: [::core::ffi::c_char; ARIZONA_FLL_NAME_LEN],
}

unsafe extern "C" {
    pub fn arizona_dvfs_up(
        component: *mut snd_soc_component,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn arizona_dvfs_down(
        component: *mut snd_soc_component,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn arizona_dvfs_sysclk_ev(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn arizona_init_dvfs(priv_: *mut arizona_priv);

    pub fn arizona_init_fll(
        arizona: *mut arizona,
        id: ::core::ffi::c_int,
        base: ::core::ffi::c_int,
        lock_irq: ::core::ffi::c_int,
        ok_irq: ::core::ffi::c_int,
        fll: *mut arizona_fll,
    ) -> ::core::ffi::c_int;
    pub fn arizona_set_fll_refclk(
        fll: *mut arizona_fll,
        source: ::core::ffi::c_int,
        Fref: ::core::ffi::c_uint,
        Fout: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn arizona_set_fll(
        fll: *mut arizona_fll,
        source: ::core::ffi::c_int,
        Fref: ::core::ffi::c_uint,
        Fout: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn arizona_init_spk(component: *mut snd_soc_component) -> ::core::ffi::c_int;
    pub fn arizona_init_gpio(component: *mut snd_soc_component) -> ::core::ffi::c_int;
    pub fn arizona_init_mono(component: *mut snd_soc_component) -> ::core::ffi::c_int;

    pub fn arizona_init_common(arizona: *mut arizona) -> ::core::ffi::c_int;
    pub fn arizona_init_vol_limit(arizona: *mut arizona) -> ::core::ffi::c_int;

    pub fn arizona_init_spk_irqs(arizona: *mut arizona) -> ::core::ffi::c_int;
    pub fn arizona_free_spk_irqs(arizona: *mut arizona) -> ::core::ffi::c_int;

    pub fn arizona_init_dai(priv_: *mut arizona_priv, id: ::core::ffi::c_int) -> ::core::ffi::c_int;

    pub fn arizona_set_output_mode(
        component: *mut snd_soc_component,
        output: ::core::ffi::c_int,
        diff: bool,
    ) -> ::core::ffi::c_int;

    pub fn arizona_input_analog(component: *mut snd_soc_component, shift: ::core::ffi::c_int) -> bool;

    pub fn arizona_sample_rate_val_to_name(rate_val: ::core::ffi::c_uint)
        -> *const ::core::ffi::c_char;
}

pub type arizona_notifier_fn = unsafe extern "C" fn(
    nb: *mut notifier_block,
    action: ::core::ffi::c_ulong,
    data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;

#[inline]
pub unsafe fn arizona_register_notifier(
    component: *mut snd_soc_component,
    nb: *mut notifier_block,
    notify: arizona_notifier_fn,
) -> ::core::ffi::c_int {
    let priv_ = unsafe { snd_soc_component_get_drvdata(component) as *mut arizona_priv };
    let arizona = unsafe { (*priv_).arizona };

    unsafe {
        (*nb).notifier_call = Some(notify);
        blocking_notifier_chain_register(&mut (*arizona).notifier, nb)
    }
}

#[inline]
pub unsafe fn arizona_unregister_notifier(
    component: *mut snd_soc_component,
    nb: *mut notifier_block,
) -> ::core::ffi::c_int {
    let priv_ = unsafe { snd_soc_component_get_drvdata(component) as *mut arizona_priv };
    let arizona = unsafe { (*priv_).arizona };

    unsafe { blocking_notifier_chain_unregister(&mut (*arizona).notifier, nb) }
}

unsafe extern "C" {
    pub fn arizona_of_get_audio_pdata(arizona: *mut arizona) -> ::core::ffi::c_int;

    pub fn arizona_jack_codec_dev_probe(
        info: *mut arizona_priv,
        dev: *mut device,
    ) -> ::core::ffi::c_int;
    pub fn arizona_jack_codec_dev_remove(info: *mut arizona_priv) -> ::core::ffi::c_int;

    pub fn arizona_jack_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
