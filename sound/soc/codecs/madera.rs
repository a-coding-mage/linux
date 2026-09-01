// SPDX-License-Identifier: GPL-2.0-only
//
// Rust source-level translation of soc/codecs/madera.c.
// External Linux/ASoC/Madera symbols are intentionally left as future dependencies.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(clippy::missing_safety_doc)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type u16 = u16;
pub type u32 = u32;
pub type u64 = u64;
pub type s16 = i16;
pub type __be16 = u16;
pub type bool_ = bool;
pub type irqreturn_t = c_int;
pub type irq_handler_t = Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>;

#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context, pub reg: c_uint, pub shift: c_uint }
#[repr(C)] pub struct snd_kcontrol { pub private_value: usize, pub id: snd_ctl_elem_id }
#[repr(C)] pub struct snd_ctl_elem_id { pub name: *const c_char }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_union }
#[repr(C)] pub union snd_ctl_elem_value_union { pub enumerated: snd_ctl_elem_value_enumerated, pub bytes: snd_ctl_elem_value_bytes }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 4] }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_bytes { pub data: [u8; 512] }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime }
#[repr(C)] pub struct snd_pcm_runtime { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component, pub dev: *mut device, pub id: c_int, pub driver: *mut snd_soc_dai_driver }
#[repr(C)] pub struct snd_soc_dai_driver { pub base: c_int, pub playback: snd_soc_pcm_stream, pub capture: snd_soc_pcm_stream }
#[repr(C)] pub struct snd_soc_pcm_stream { pub stream_name: *const c_char, pub channels_max: c_int }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_ops { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct soc_enum { pub reg: c_uint, pub shift_l: c_uint, pub shift_r: c_uint, pub items: c_uint, pub mask: c_uint, pub texts: *const *const c_char, pub values: *const c_uint }
#[repr(C)] pub struct soc_mixer_control { pub reg: c_uint }
#[repr(C)] pub struct soc_bytes { pub base: c_uint, pub num_regs: c_int }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct reg_sequence { pub reg: c_uint, pub def: c_uint }
#[repr(C)] pub struct madera_mclk { pub clk: *mut clk }
#[repr(C)] pub struct cs_dsp { pub base: c_uint, pub num: c_int, pub regmap: *mut regmap, pub dev: *mut device }
#[repr(C)] pub struct wm_adsp { pub cs_dsp: cs_dsp }
#[repr(C)] pub struct madera_codec_pdata { pub inmode: [[u32; MADERA_MAX_MUXED_CHANNELS as usize]; MADERA_MAX_INPUT as usize], pub out_mono: [bool; MADERA_MAX_OUTPUT as usize], pub max_channels_clocked: [u32; MADERA_MAX_AIF as usize], pub pdm_fmt: [u32; MADERA_MAX_PDM_SPK as usize], pub pdm_mute: [u32; MADERA_MAX_PDM_SPK as usize], pub dmic_ref: [u32; MADERA_MAX_INPUT as usize] }
#[repr(C)] pub struct madera_pdata { pub codec: madera_codec_pdata }
#[repr(C)] pub struct madera { pub dev: *mut device, pub regmap: *mut regmap, pub pdata: madera_pdata, pub mclk: [madera_mclk; 3], pub out_clamp: [bool; MADERA_MAX_HP_OUTPUT as usize], pub out_shorted: [bool; MADERA_MAX_HP_OUTPUT as usize], pub hp_ena: c_uint, pub type_: c_int, pub rev: c_int }
#[repr(C)] pub struct madera_dai_priv { pub clk: c_int, pub constraint: snd_pcm_hw_constraint_list }
#[repr(C)] pub struct snd_pcm_hw_constraint_list { pub count: c_uint, pub list: *const c_uint, pub mask: c_uint }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct madera_priv { pub madera: *mut madera, pub rate_lock: mutex, pub domain_group_ref: [c_int; 32], pub adsp_rate_cache: [c_uint; MADERA_MAX_ADSP as usize], pub adsp: [wm_adsp; MADERA_MAX_ADSP as usize], pub dai: [madera_dai_priv; MADERA_MAX_AIF as usize], pub tdm_width: [c_int; MADERA_MAX_AIF as usize], pub tdm_slots: [c_int; MADERA_MAX_AIF as usize], pub num_inputs: c_int, pub in_pending: c_int, pub out_up_pending: c_int, pub out_up_delay: c_int, pub out_down_pending: c_int, pub out_down_delay: c_int }
#[repr(C)] pub struct madera_fll { pub id: c_int, pub base: c_int, pub madera: *mut madera, pub ref_src: c_int, pub sync_src: c_int, pub ref_freq: c_uint, pub sync_freq: c_uint, pub fout: c_uint }
#[repr(C)] pub struct madera_fll_cfg { pub refdiv: c_int, pub fratio: c_int, pub n: c_uint, pub theta: c_uint, pub lambda: c_uint, pub gain: c_int, pub alt_gain: c_int }

unsafe extern "C" {
    static mut madera_rate_text: [*const c_char; MADERA_RATE_ENUM_SIZE as usize];
    static mut madera_rate_val: [c_uint; MADERA_RATE_ENUM_SIZE as usize];
}

// The original C include dependencies are omitted from executable Rust and are expected from surrounding bindings.

// C: // SPDX-License-Identifier: GPL-2.0-only
// C: //
// C: // Cirrus Logic Madera class codecs common support
// C: //
// C: // Copyright (C) 2015-2019 Cirrus Logic, Inc. and
// C: //                         Cirrus Logic International Semiconductor Ltd.
// C: //
// C: 
// C: #include <linux/cleanup.h>
// C: #include <linux/delay.h>
// C: #include <linux/gcd.h>
// C: #include <linux/module.h>
// C: #include <linux/pm_runtime.h>
// C: #include <linux/slab.h>
// C: #include <linux/string_choices.h>
// C: #include <sound/pcm.h>
// C: #include <sound/pcm_params.h>
// C: #include <sound/tlv.h>
// C: 
// C: #include <linux/irqchip/irq-madera.h>
// C: #include <linux/mfd/madera/core.h>
// C: #include <linux/mfd/madera/registers.h>
// C: #include <linux/mfd/madera/pdata.h>
// C: #include <sound/madera-pdata.h>
// C: 
// C: #include <dt-bindings/sound/madera.h>
// C: 
// C: #include "madera.h"
// C: 
pub const MADERA_AIF_BCLK_CTRL: c_int = 0x00;
pub const MADERA_AIF_TX_PIN_CTRL: c_int = 0x01;
pub const MADERA_AIF_RX_PIN_CTRL: c_int = 0x02;
pub const MADERA_AIF_RATE_CTRL: c_int = 0x03;
pub const MADERA_AIF_FORMAT: c_int = 0x04;
pub const MADERA_AIF_RX_BCLK_RATE: c_int = 0x06;
pub const MADERA_AIF_FRAME_CTRL_1: c_int = 0x07;
pub const MADERA_AIF_FRAME_CTRL_2: c_int = 0x08;
pub const MADERA_AIF_FRAME_CTRL_3: c_int = 0x09;
pub const MADERA_AIF_FRAME_CTRL_4: c_int = 0x0A;
pub const MADERA_AIF_FRAME_CTRL_5: c_int = 0x0B;
pub const MADERA_AIF_FRAME_CTRL_6: c_int = 0x0C;
pub const MADERA_AIF_FRAME_CTRL_7: c_int = 0x0D;
pub const MADERA_AIF_FRAME_CTRL_8: c_int = 0x0E;
pub const MADERA_AIF_FRAME_CTRL_9: c_int = 0x0F;
pub const MADERA_AIF_FRAME_CTRL_10: c_int = 0x10;
pub const MADERA_AIF_FRAME_CTRL_11: c_int = 0x11;
pub const MADERA_AIF_FRAME_CTRL_12: c_int = 0x12;
pub const MADERA_AIF_FRAME_CTRL_13: c_int = 0x13;
pub const MADERA_AIF_FRAME_CTRL_14: c_int = 0x14;
pub const MADERA_AIF_FRAME_CTRL_15: c_int = 0x15;
pub const MADERA_AIF_FRAME_CTRL_16: c_int = 0x16;
pub const MADERA_AIF_FRAME_CTRL_17: c_int = 0x17;
pub const MADERA_AIF_FRAME_CTRL_18: c_int = 0x18;
pub const MADERA_AIF_TX_ENABLES: c_int = 0x19;
pub const MADERA_AIF_RX_ENABLES: c_int = 0x1A;
pub const MADERA_AIF_FORCE_WRITE: c_int = 0x1B;
// C: 
pub const MADERA_DSP_CONFIG_1_OFFS: c_int = 0x00;
pub const MADERA_DSP_CONFIG_2_OFFS: c_int = 0x02;
// C: 
pub const MADERA_DSP_CLK_SEL_MASK: c_int = 0x70000;
pub const MADERA_DSP_CLK_SEL_SHIFT: c_int = 16;
// C: 
pub const MADERA_DSP_RATE_MASK: c_int = 0x7800;
pub const MADERA_DSP_RATE_SHIFT: c_int = 11;
// C: 
pub const MADERA_SYSCLK_6MHZ: c_int = 0;
pub const MADERA_SYSCLK_12MHZ: c_int = 1;
pub const MADERA_SYSCLK_24MHZ: c_int = 2;
pub const MADERA_SYSCLK_49MHZ: c_int = 3;
pub const MADERA_SYSCLK_98MHZ: c_int = 4;
// C: 
pub const MADERA_DSPCLK_9MHZ: c_int = 0;
pub const MADERA_DSPCLK_18MHZ: c_int = 1;
pub const MADERA_DSPCLK_36MHZ: c_int = 2;
pub const MADERA_DSPCLK_73MHZ: c_int = 3;
pub const MADERA_DSPCLK_147MHZ: c_int = 4;
// C: 
pub const MADERA_FLL_VCO_CORNER: c_int = 141900000;
pub const MADERA_FLL_MAX_FREF: c_int = 13500000;
pub const MADERA_FLL_MAX_N: c_int = 1023;
pub const MADERA_FLL_MIN_FOUT: c_int = 90000000;
pub const MADERA_FLL_MAX_FOUT: c_int = 100000000;
pub const MADERA_FLL_MAX_FRATIO: c_int = 16;
pub const MADERA_FLL_MAX_REFDIV: c_int = 8;
pub const MADERA_FLL_OUTDIV: c_int = 3;
pub const MADERA_FLL_VCO_MULT: c_int = 3;
pub const MADERA_FLLAO_MAX_FREF: c_int = 12288000;
pub const MADERA_FLLAO_MIN_N: c_int = 4;
pub const MADERA_FLLAO_MAX_N: c_int = 1023;
pub const MADERA_FLLAO_MAX_FBDIV: c_int = 254;
pub const MADERA_FLLHJ_INT_MAX_N: c_int = 1023;
pub const MADERA_FLLHJ_INT_MIN_N: c_int = 1;
pub const MADERA_FLLHJ_FRAC_MAX_N: c_int = 255;
pub const MADERA_FLLHJ_FRAC_MIN_N: c_int = 4;
pub const MADERA_FLLHJ_LOW_THRESH: c_int = 192000;
pub const MADERA_FLLHJ_MID_THRESH: c_int = 1152000;
pub const MADERA_FLLHJ_MAX_THRESH: c_int = 13000000;
pub const MADERA_FLLHJ_LOW_GAINS: c_int = 0x23f0;
pub const MADERA_FLLHJ_MID_GAINS: c_int = 0x22f2;
pub const MADERA_FLLHJ_HIGH_GAINS: c_int = 0x21f0;
// C: 
pub const MADERA_FLL_SYNCHRONISER_OFFS: c_int = 0x10;
pub const CS47L35_FLL_SYNCHRONISER_OFFS: c_int = 0xE;
pub const MADERA_FLL_CONTROL_1_OFFS: c_int = 0x1;
pub const MADERA_FLL_CONTROL_2_OFFS: c_int = 0x2;
pub const MADERA_FLL_CONTROL_3_OFFS: c_int = 0x3;
pub const MADERA_FLL_CONTROL_4_OFFS: c_int = 0x4;
pub const MADERA_FLL_CONTROL_5_OFFS: c_int = 0x5;
pub const MADERA_FLL_CONTROL_6_OFFS: c_int = 0x6;
pub const MADERA_FLL_GAIN_OFFS: c_int = 0x8;
pub const MADERA_FLL_CONTROL_7_OFFS: c_int = 0x9;
pub const MADERA_FLL_EFS_2_OFFS: c_int = 0xA;
pub const MADERA_FLL_SYNCHRONISER_1_OFFS: c_int = 0x1;
pub const MADERA_FLL_SYNCHRONISER_2_OFFS: c_int = 0x2;
pub const MADERA_FLL_SYNCHRONISER_3_OFFS: c_int = 0x3;
pub const MADERA_FLL_SYNCHRONISER_4_OFFS: c_int = 0x4;
pub const MADERA_FLL_SYNCHRONISER_5_OFFS: c_int = 0x5;
pub const MADERA_FLL_SYNCHRONISER_6_OFFS: c_int = 0x6;
pub const MADERA_FLL_SYNCHRONISER_7_OFFS: c_int = 0x7;
pub const MADERA_FLL_SPREAD_SPECTRUM_OFFS: c_int = 0x9;
pub const MADERA_FLL_GPIO_CLOCK_OFFS: c_int = 0xA;
pub const MADERA_FLL_CONTROL_10_OFFS: c_int = 0xA;
pub const MADERA_FLL_CONTROL_11_OFFS: c_int = 0xB;
pub const MADERA_FLL1_DIGITAL_TEST_1_OFFS: c_int = 0xD;
// C: 
pub const MADERA_FLLAO_CONTROL_1_OFFS: c_int = 0x1;
pub const MADERA_FLLAO_CONTROL_2_OFFS: c_int = 0x2;
pub const MADERA_FLLAO_CONTROL_3_OFFS: c_int = 0x3;
pub const MADERA_FLLAO_CONTROL_4_OFFS: c_int = 0x4;
pub const MADERA_FLLAO_CONTROL_5_OFFS: c_int = 0x5;
pub const MADERA_FLLAO_CONTROL_6_OFFS: c_int = 0x6;
pub const MADERA_FLLAO_CONTROL_7_OFFS: c_int = 0x8;
pub const MADERA_FLLAO_CONTROL_8_OFFS: c_int = 0xA;
pub const MADERA_FLLAO_CONTROL_9_OFFS: c_int = 0xB;
pub const MADERA_FLLAO_CONTROL_10_OFFS: c_int = 0xC;
pub const MADERA_FLLAO_CONTROL_11_OFFS: c_int = 0xD;
// C: 
pub const MADERA_FMT_DSP_MODE_A: c_int = 0;
pub const MADERA_FMT_DSP_MODE_B: c_int = 1;
pub const MADERA_FMT_I2S_MODE: c_int = 2;
pub const MADERA_FMT_LEFT_JUSTIFIED_MODE: c_int = 3;
// C: 
// C: #define madera_fll_err(_fll, fmt, ...) \
// C: 	dev_err(_fll->madera->dev, "FLL%d: " fmt, _fll->id, ##__VA_ARGS__)
// C: #define madera_fll_warn(_fll, fmt, ...) \
// C: 	dev_warn(_fll->madera->dev, "FLL%d: " fmt, _fll->id, ##__VA_ARGS__)
// C: #define madera_fll_dbg(_fll, fmt, ...) \
// C: 	dev_dbg(_fll->madera->dev, "FLL%d: " fmt, _fll->id, ##__VA_ARGS__)
// C: 
// C: #define madera_aif_err(_dai, fmt, ...) \
// C: 	dev_err(_dai->dev, "AIF%d: " fmt, _dai->id, ##__VA_ARGS__)
// C: #define madera_aif_warn(_dai, fmt, ...) \
// C: 	dev_warn(_dai->dev, "AIF%d: " fmt, _dai->id, ##__VA_ARGS__)
// C: #define madera_aif_dbg(_dai, fmt, ...) \
// C: 	dev_dbg(_dai->dev, "AIF%d: " fmt, _dai->id, ##__VA_ARGS__)
// C: 
// C: static const int madera_dsp_bus_error_irqs[MADERA_MAX_ADSP] = {
// C: 	MADERA_IRQ_DSP1_BUS_ERR,
// C: 	MADERA_IRQ_DSP2_BUS_ERR,
// C: 	MADERA_IRQ_DSP3_BUS_ERR,
// C: 	MADERA_IRQ_DSP4_BUS_ERR,
// C: 	MADERA_IRQ_DSP5_BUS_ERR,
// C: 	MADERA_IRQ_DSP6_BUS_ERR,
// C: 	MADERA_IRQ_DSP7_BUS_ERR,
// C: };
// C: 
// C: int madera_clk_ev(struct snd_soc_dapm_widget *w,
// C: 		  struct snd_kcontrol *kcontrol, int event)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_to_component(w->dapm);
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera *madera = priv->madera;
// C: 	unsigned int val;
// C: 	int clk_idx;
// C: 	int ret;
// C: 
// C: 	ret = regmap_read(madera->regmap, w->reg, &val);
// C: 	if (ret) {
// C: 		dev_err(madera->dev, "Failed to check clock source: %d\n", ret);
// C: 		return ret;
// C: 	}
// C: 
// C: 	switch ((val & MADERA_SYSCLK_SRC_MASK) >> MADERA_SYSCLK_SRC_SHIFT) {
// C: 	case MADERA_CLK_SRC_MCLK1:
// C: 		clk_idx = MADERA_MCLK1;
// C: 		break;
// C: 	case MADERA_CLK_SRC_MCLK2:
// C: 		clk_idx = MADERA_MCLK2;
// C: 		break;
// C: 	case MADERA_CLK_SRC_MCLK3:
// C: 		clk_idx = MADERA_MCLK3;
// C: 		break;
// C: 	default:
// C: 		return 0;
// C: 	}
// C: 
// C: 	switch (event) {
// C: 	case SND_SOC_DAPM_PRE_PMU:
// C: 		return clk_prepare_enable(madera->mclk[clk_idx].clk);
// C: 	case SND_SOC_DAPM_POST_PMD:
// C: 		clk_disable_unprepare(madera->mclk[clk_idx].clk);
// C: 		return 0;
// C: 	default:
// C: 		return 0;
// C: 	}
// C: }
// C: EXPORT_SYMBOL_GPL(madera_clk_ev);
// C: 
// C: static void madera_spin_sysclk(struct madera_priv *priv)
// C: {
// C: 	struct madera *madera = priv->madera;
// C: 	unsigned int val;
// C: 	int ret, i;
// C: 
// C: 	/* Skip this if the chip is down */
// C: 	if (pm_runtime_suspended(madera->dev))
// C: 		return;
// C: 
// C: 	/*
// C: 	 * Just read a register a few times to ensure the internal
// C: 	 * oscillator sends out a few clocks.
// C: 	 */
// C: 	for (i = 0; i < 4; i++) {
// C: 		ret = regmap_read(madera->regmap, MADERA_SOFTWARE_RESET, &val);
// C: 		if (ret)
// C: 			dev_err(madera->dev,
// C: 				"Failed to read sysclk spin %d: %d\n", i, ret);
// C: 	}
// C: 
// C: 	udelay(300);
// C: }
// C: 
// C: int madera_sysclk_ev(struct snd_soc_dapm_widget *w,
// C: 		     struct snd_kcontrol *kcontrol, int event)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_to_component(w->dapm);
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 
// C: 	switch (event) {
// C: 	case SND_SOC_DAPM_POST_PMU:
// C: 	case SND_SOC_DAPM_PRE_PMD:
// C: 		madera_spin_sysclk(priv);
// C: 		break;
// C: 	default:
// C: 		break;
// C: 	}
// C: 
// C: 	return madera_clk_ev(w, kcontrol, event);
// C: }
// C: EXPORT_SYMBOL_GPL(madera_sysclk_ev);
// C: 
// C: static int madera_check_speaker_overheat(struct madera *madera,
// C: 					 bool *warn, bool *shutdown)
// C: {
// C: 	unsigned int val;
// C: 	int ret;
// C: 
// C: 	ret = regmap_read(madera->regmap, MADERA_IRQ1_RAW_STATUS_15, &val);
// C: 	if (ret) {
// C: 		dev_err(madera->dev, "Failed to read thermal status: %d\n",
// C: 			ret);
// C: 		return ret;
// C: 	}
// C: 
// C: 	*warn = val & MADERA_SPK_OVERHEAT_WARN_STS1;
// C: 	*shutdown = val & MADERA_SPK_OVERHEAT_STS1;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: int madera_spk_ev(struct snd_soc_dapm_widget *w,
// C: 		  struct snd_kcontrol *kcontrol, int event)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_to_component(w->dapm);
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera *madera = priv->madera;
// C: 	bool warn, shutdown;
// C: 	int ret;
// C: 
// C: 	switch (event) {
// C: 	case SND_SOC_DAPM_POST_PMU:
// C: 		ret = madera_check_speaker_overheat(madera, &warn, &shutdown);
// C: 		if (ret)
// C: 			return ret;
// C: 
// C: 		if (shutdown) {
// C: 			dev_crit(madera->dev,
// C: 				 "Speaker not enabled due to temperature\n");
// C: 			return -EBUSY;
// C: 		}
// C: 
// C: 		regmap_update_bits(madera->regmap, MADERA_OUTPUT_ENABLES_1,
// C: 				   1 << w->shift, 1 << w->shift);
// C: 		break;
// C: 	case SND_SOC_DAPM_PRE_PMD:
// C: 		regmap_update_bits(madera->regmap, MADERA_OUTPUT_ENABLES_1,
// C: 				   1 << w->shift, 0);
// C: 		break;
// C: 	default:
// C: 		break;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_spk_ev);
// C: 
// C: static irqreturn_t madera_thermal_warn(int irq, void *data)
// C: {
// C: 	struct madera *madera = data;
// C: 	bool warn, shutdown;
// C: 	int ret;
// C: 
// C: 	ret = madera_check_speaker_overheat(madera, &warn, &shutdown);
// C: 	if (ret || shutdown) { /* for safety attempt to shutdown on error */
// C: 		dev_crit(madera->dev, "Thermal shutdown\n");
// C: 		ret = regmap_update_bits(madera->regmap,
// C: 					 MADERA_OUTPUT_ENABLES_1,
// C: 					 MADERA_OUT4L_ENA |
// C: 					 MADERA_OUT4R_ENA, 0);
// C: 		if (ret != 0)
// C: 			dev_crit(madera->dev,
// C: 				 "Failed to disable speaker outputs: %d\n",
// C: 				 ret);
// C: 	} else if (warn) {
// C: 		dev_alert(madera->dev, "Thermal warning\n");
// C: 	} else {
// C: 		dev_info(madera->dev, "Spurious thermal warning\n");
// C: 		return IRQ_NONE;
// C: 	}
// C: 
// C: 	return IRQ_HANDLED;
// C: }
// C: 
// C: int madera_init_overheat(struct madera_priv *priv)
// C: {
// C: 	struct madera *madera = priv->madera;
// C: 	struct device *dev = madera->dev;
// C: 	int ret;
// C: 
// C: 	ret = madera_request_irq(madera, MADERA_IRQ_SPK_OVERHEAT_WARN,
// C: 				 "Thermal warning", madera_thermal_warn,
// C: 				 madera);
// C: 	if (ret)
// C: 		dev_err(dev, "Failed to get thermal warning IRQ: %d\n", ret);
// C: 
// C: 	ret = madera_request_irq(madera, MADERA_IRQ_SPK_OVERHEAT,
// C: 				 "Thermal shutdown", madera_thermal_warn,
// C: 				 madera);
// C: 	if (ret)
// C: 		dev_err(dev, "Failed to get thermal shutdown IRQ: %d\n", ret);
// C: 
// C: 	return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_init_overheat);
// C: 
// C: int madera_free_overheat(struct madera_priv *priv)
// C: {
// C: 	struct madera *madera = priv->madera;
// C: 
// C: 	madera_free_irq(madera, MADERA_IRQ_SPK_OVERHEAT_WARN, madera);
// C: 	madera_free_irq(madera, MADERA_IRQ_SPK_OVERHEAT, madera);
// C: 
// C: 	return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_free_overheat);
// C: 
// C: static int madera_get_variable_u32_array(struct device *dev,
// C: 					 const char *propname,
// C: 					 u32 *dest, int n_max,
// C: 					 int multiple)
// C: {
// C: 	int n, ret;
// C: 
// C: 	n = device_property_count_u32(dev, propname);
// C: 	if (n < 0) {
// C: 		if (n == -EINVAL)
// C: 			return 0;	/* missing, ignore */
// C: 
// C: 		dev_warn(dev, "%s malformed (%d)\n", propname, n);
// C: 
// C: 		return n;
// C: 	} else if ((n % multiple) != 0) {
// C: 		dev_warn(dev, "%s not a multiple of %d entries\n",
// C: 			 propname, multiple);
// C: 
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	if (n > n_max)
// C: 		n = n_max;
// C: 
// C: 	ret = device_property_read_u32_array(dev, propname, dest, n);
// C: 	if (ret < 0)
// C: 		return ret;
// C: 
// C: 	return n;
// C: }
// C: 
// C: static void madera_prop_get_inmode(struct madera_priv *priv)
// C: {
// C: 	struct madera *madera = priv->madera;
// C: 	struct madera_codec_pdata *pdata = &madera->pdata.codec;
// C: 	u32 tmp[MADERA_MAX_INPUT * MADERA_MAX_MUXED_CHANNELS];
// C: 	int n, i, in_idx, ch_idx;
// C: 
// C: 	BUILD_BUG_ON(ARRAY_SIZE(pdata->inmode) != MADERA_MAX_INPUT);
// C: 	BUILD_BUG_ON(ARRAY_SIZE(pdata->inmode[0]) != MADERA_MAX_MUXED_CHANNELS);
// C: 
// C: 	n = madera_get_variable_u32_array(madera->dev, "cirrus,inmode",
// C: 					  tmp, ARRAY_SIZE(tmp),
// C: 					  MADERA_MAX_MUXED_CHANNELS);
// C: 	if (n < 0)
// C: 		return;
// C: 
// C: 	in_idx = 0;
// C: 	ch_idx = 0;
// C: 	for (i = 0; i < n; ++i) {
// C: 		pdata->inmode[in_idx][ch_idx] = tmp[i];
// C: 
// C: 		if (++ch_idx == MADERA_MAX_MUXED_CHANNELS) {
// C: 			ch_idx = 0;
// C: 			++in_idx;
// C: 		}
// C: 	}
// C: }
// C: 
// C: static void madera_prop_get_pdata(struct madera_priv *priv)
// C: {
// C: 	struct madera *madera = priv->madera;
// C: 	struct madera_codec_pdata *pdata = &madera->pdata.codec;
// C: 	u32 out_mono[ARRAY_SIZE(pdata->out_mono)];
// C: 	int i, n;
// C: 
// C: 	madera_prop_get_inmode(priv);
// C: 
// C: 	n = madera_get_variable_u32_array(madera->dev, "cirrus,out-mono",
// C: 					  out_mono, ARRAY_SIZE(out_mono), 1);
// C: 	if (n > 0)
// C: 		for (i = 0; i < n; ++i)
// C: 			pdata->out_mono[i] = !!out_mono[i];
// C: 
// C: 	madera_get_variable_u32_array(madera->dev,
// C: 				      "cirrus,max-channels-clocked",
// C: 				      pdata->max_channels_clocked,
// C: 				      ARRAY_SIZE(pdata->max_channels_clocked),
// C: 				      1);
// C: 
// C: 	madera_get_variable_u32_array(madera->dev, "cirrus,pdm-fmt",
// C: 				      pdata->pdm_fmt,
// C: 				      ARRAY_SIZE(pdata->pdm_fmt), 1);
// C: 
// C: 	madera_get_variable_u32_array(madera->dev, "cirrus,pdm-mute",
// C: 				      pdata->pdm_mute,
// C: 				      ARRAY_SIZE(pdata->pdm_mute), 1);
// C: 
// C: 	madera_get_variable_u32_array(madera->dev, "cirrus,dmic-ref",
// C: 				      pdata->dmic_ref,
// C: 				      ARRAY_SIZE(pdata->dmic_ref), 1);
// C: }
// C: 
// C: int madera_core_init(struct madera_priv *priv)
// C: {
// C: 	int i;
// C: 
// C: 	/* trap undersized array initializers */
// C: 	BUILD_BUG_ON(!madera_mixer_texts[MADERA_NUM_MIXER_INPUTS - 1]);
// C: 	BUILD_BUG_ON(!madera_mixer_values[MADERA_NUM_MIXER_INPUTS - 1]);
// C: 
// C: 	if (!dev_get_platdata(priv->madera->dev))
// C: 		madera_prop_get_pdata(priv);
// C: 
// C: 	mutex_init(&priv->rate_lock);
// C: 
// C: 	for (i = 0; i < MADERA_MAX_HP_OUTPUT; i++)
// C: 		priv->madera->out_clamp[i] = true;
// C: 
// C: 	return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_core_init);
// C: 
// C: int madera_core_free(struct madera_priv *priv)
// C: {
// C: 	mutex_destroy(&priv->rate_lock);
// C: 
// C: 	return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_core_free);
// C: 
// C: static void madera_debug_dump_domain_groups(const struct madera_priv *priv)
// C: {
// C: 	struct madera *madera = priv->madera;
// C: 	int i;
// C: 
// C: 	for (i = 0; i < ARRAY_SIZE(priv->domain_group_ref); ++i)
// C: 		dev_dbg(madera->dev, "domain_grp_ref[%d]=%d\n", i,
// C: 			priv->domain_group_ref[i]);
// C: }
// C: 
// C: int madera_domain_clk_ev(struct snd_soc_dapm_widget *w,
// C: 			 struct snd_kcontrol *kcontrol,
// C: 			 int event)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_to_component(w->dapm);
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	int dom_grp = w->shift;
// C: 
// C: 	if (dom_grp >= ARRAY_SIZE(priv->domain_group_ref)) {
// C: 		WARN(true, "%s dom_grp exceeds array size\n", __func__);
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	/*
// C: 	 * We can't rely on the DAPM mutex for locking because we need a lock
// C: 	 * that can safely be called in hw_params
// C: 	 */
// C: 	guard(mutex)(&priv->rate_lock);
// C: 
// C: 	switch (event) {
// C: 	case SND_SOC_DAPM_PRE_PMU:
// C: 		dev_dbg(priv->madera->dev, "Inc ref on domain group %d\n",
// C: 			dom_grp);
// C: 		++priv->domain_group_ref[dom_grp];
// C: 		break;
// C: 	case SND_SOC_DAPM_POST_PMD:
// C: 		dev_dbg(priv->madera->dev, "Dec ref on domain group %d\n",
// C: 			dom_grp);
// C: 		--priv->domain_group_ref[dom_grp];
// C: 		break;
// C: 	default:
// C: 		break;
// C: 	}
// C: 
// C: 	madera_debug_dump_domain_groups(priv);
// C: 
// C: 	return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_domain_clk_ev);
// C: 
// C: int madera_out1_demux_put(struct snd_kcontrol *kcontrol,
// C: 			  struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_kcontrol_to_component(kcontrol);
// C: 	struct snd_soc_dapm_context *dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera *madera = priv->madera;
// C: 	struct soc_enum *e = (struct soc_enum *)kcontrol->private_value;
// C: 	unsigned int ep_sel, mux, change;
// C: 	bool out_mono;
// C: 	int ret;
// C: 
// C: 	if (ucontrol->value.enumerated.item[0] > e->items - 1)
// C: 		return -EINVAL;
// C: 
// C: 	mux = ucontrol->value.enumerated.item[0];
// C: 
// C: 	snd_soc_dapm_mutex_lock(dapm);
// C: 
// C: 	ep_sel = mux << MADERA_EP_SEL_SHIFT;
// C: 
// C: 	change = snd_soc_component_test_bits(component, MADERA_OUTPUT_ENABLES_1,
// C: 					     MADERA_EP_SEL_MASK,
// C: 					     ep_sel);
// C: 	if (!change)
// C: 		goto end;
// C: 
// C: 	/* EP_SEL should not be modified while HP or EP driver is enabled */
// C: 	ret = regmap_update_bits(madera->regmap, MADERA_OUTPUT_ENABLES_1,
// C: 				 MADERA_OUT1L_ENA | MADERA_OUT1R_ENA, 0);
// C: 	if (ret)
// C: 		dev_warn(madera->dev, "Failed to disable outputs: %d\n", ret);
// C: 
// C: 	usleep_range(2000, 3000); /* wait for wseq to complete */
// C: 
// C: 	/* change demux setting */
// C: 	ret = 0;
// C: 	if (madera->out_clamp[0])
// C: 		ret = regmap_update_bits(madera->regmap,
// C: 					 MADERA_OUTPUT_ENABLES_1,
// C: 					 MADERA_EP_SEL_MASK, ep_sel);
// C: 	if (ret) {
// C: 		dev_err(madera->dev, "Failed to set OUT1 demux: %d\n", ret);
// C: 	} else {
// C: 		/* apply correct setting for mono mode */
// C: 		if (!ep_sel && !madera->pdata.codec.out_mono[0])
// C: 			out_mono = false; /* stereo HP */
// C: 		else
// C: 			out_mono = true; /* EP or mono HP */
// C: 
// C: 		ret = madera_set_output_mode(component, 1, out_mono);
// C: 		if (ret)
// C: 			dev_warn(madera->dev,
// C: 				 "Failed to set output mode: %d\n", ret);
// C: 	}
// C: 
// C: 	/*
// C: 	 * if HPDET has disabled the clamp while switching to HPOUT
// C: 	 * OUT1 should remain disabled
// C: 	 */
// C: 	if (ep_sel ||
// C: 	    (madera->out_clamp[0] && !madera->out_shorted[0])) {
// C: 		ret = regmap_update_bits(madera->regmap,
// C: 					 MADERA_OUTPUT_ENABLES_1,
// C: 					 MADERA_OUT1L_ENA | MADERA_OUT1R_ENA,
// C: 					 madera->hp_ena);
// C: 		if (ret)
// C: 			dev_warn(madera->dev,
// C: 				 "Failed to restore earpiece outputs: %d\n",
// C: 				 ret);
// C: 		else if (madera->hp_ena)
// C: 			msleep(34); /* wait for enable wseq */
// C: 		else
// C: 			usleep_range(2000, 3000); /* wait for disable wseq */
// C: 	}
// C: 
// C: end:
// C: 	snd_soc_dapm_mutex_unlock(dapm);
// C: 
// C: 	ret = snd_soc_dapm_mux_update_power(dapm, kcontrol, mux, e, NULL);
// C: 	if (ret < 0) {
// C: 		dev_err(madera->dev, "Failed to update demux power state: %d\n", ret);
// C: 		return ret;
// C: 	}
// C: 
// C: 	return change;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_out1_demux_put);
// C: 
// C: int madera_out1_demux_get(struct snd_kcontrol *kcontrol,
// C: 			  struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_kcontrol_to_component(kcontrol);
// C: 	unsigned int val;
// C: 
// C: 	val = snd_soc_component_read(component, MADERA_OUTPUT_ENABLES_1);
// C: 	val &= MADERA_EP_SEL_MASK;
// C: 	val >>= MADERA_EP_SEL_SHIFT;
// C: 	ucontrol->value.enumerated.item[0] = val;
// C: 
// C: 	return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_out1_demux_get);
// C: 
// C: static int madera_inmux_put(struct snd_kcontrol *kcontrol,
// C: 			    struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_kcontrol_to_component(kcontrol);
// C: 	struct snd_soc_dapm_context *dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera *madera = priv->madera;
// C: 	struct regmap *regmap = madera->regmap;
// C: 	struct soc_enum *e = (struct soc_enum *)kcontrol->private_value;
// C: 	unsigned int mux, val, mask;
// C: 	unsigned int inmode;
// C: 	bool changed;
// C: 	int ret;
// C: 
// C: 	mux = ucontrol->value.enumerated.item[0];
// C: 	if (mux > 1)
// C: 		return -EINVAL;
// C: 
// C: 	val = mux << e->shift_l;
// C: 	mask = (e->mask << e->shift_l) | MADERA_IN1L_SRC_SE_MASK;
// C: 
// C: 	switch (e->reg) {
// C: 	case MADERA_ADC_DIGITAL_VOLUME_1L:
// C: 		inmode = madera->pdata.codec.inmode[0][2 * mux];
// C: 		break;
// C: 	case MADERA_ADC_DIGITAL_VOLUME_1R:
// C: 		inmode = madera->pdata.codec.inmode[0][1 + (2 * mux)];
// C: 		break;
// C: 	case MADERA_ADC_DIGITAL_VOLUME_2L:
// C: 		inmode = madera->pdata.codec.inmode[1][2 * mux];
// C: 		break;
// C: 	case MADERA_ADC_DIGITAL_VOLUME_2R:
// C: 		inmode = madera->pdata.codec.inmode[1][1 + (2 * mux)];
// C: 		break;
// C: 	default:
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	if (inmode & MADERA_INMODE_SE)
// C: 		val |= 1 << MADERA_IN1L_SRC_SE_SHIFT;
// C: 
// C: 	dev_dbg(madera->dev, "mux=%u reg=0x%x inmode=0x%x mask=0x%x val=0x%x\n",
// C: 		mux, e->reg, inmode, mask, val);
// C: 
// C: 	ret = regmap_update_bits_check(regmap, e->reg, mask, val, &changed);
// C: 	if (ret < 0)
// C: 		return ret;
// C: 
// C: 	if (changed)
// C: 		return snd_soc_dapm_mux_update_power(dapm, kcontrol,
// C: 						     mux, e, NULL);
// C: 	else
// C: 		return 0;
// C: }
// C: 
// C: static const char * const madera_inmux_texts[] = {
// C: 	"A",
// C: 	"B",
// C: };
// C: 
// C: static SOC_ENUM_SINGLE_DECL(madera_in1muxl_enum,
// C: 			    MADERA_ADC_DIGITAL_VOLUME_1L,
// C: 			    MADERA_IN1L_SRC_SHIFT,
// C: 			    madera_inmux_texts);
// C: 
// C: static SOC_ENUM_SINGLE_DECL(madera_in1muxr_enum,
// C: 			    MADERA_ADC_DIGITAL_VOLUME_1R,
// C: 			    MADERA_IN1R_SRC_SHIFT,
// C: 			    madera_inmux_texts);
// C: 
// C: static SOC_ENUM_SINGLE_DECL(madera_in2muxl_enum,
// C: 			    MADERA_ADC_DIGITAL_VOLUME_2L,
// C: 			    MADERA_IN2L_SRC_SHIFT,
// C: 			    madera_inmux_texts);
// C: 
// C: static SOC_ENUM_SINGLE_DECL(madera_in2muxr_enum,
// C: 			    MADERA_ADC_DIGITAL_VOLUME_2R,
// C: 			    MADERA_IN2R_SRC_SHIFT,
// C: 			    madera_inmux_texts);
// C: 
// C: const struct snd_kcontrol_new madera_inmux[] = {
// C: 	SOC_DAPM_ENUM_EXT("IN1L Mux", madera_in1muxl_enum,
// C: 			  snd_soc_dapm_get_enum_double, madera_inmux_put),
// C: 	SOC_DAPM_ENUM_EXT("IN1R Mux", madera_in1muxr_enum,
// C: 			  snd_soc_dapm_get_enum_double, madera_inmux_put),
// C: 	SOC_DAPM_ENUM_EXT("IN2L Mux", madera_in2muxl_enum,
// C: 			  snd_soc_dapm_get_enum_double, madera_inmux_put),
// C: 	SOC_DAPM_ENUM_EXT("IN2R Mux", madera_in2muxr_enum,
// C: 			  snd_soc_dapm_get_enum_double, madera_inmux_put),
// C: };
// C: EXPORT_SYMBOL_GPL(madera_inmux);
// C: 
// C: static const char * const madera_dmode_texts[] = {
// C: 	"Analog",
// C: 	"Digital",
// C: };
// C: 
// C: static SOC_ENUM_SINGLE_DECL(madera_in1dmode_enum,
// C: 			    MADERA_IN1L_CONTROL,
// C: 			    MADERA_IN1_MODE_SHIFT,
// C: 			    madera_dmode_texts);
// C: 
// C: static SOC_ENUM_SINGLE_DECL(madera_in2dmode_enum,
// C: 			    MADERA_IN2L_CONTROL,
// C: 			    MADERA_IN2_MODE_SHIFT,
// C: 			    madera_dmode_texts);
// C: 
// C: static SOC_ENUM_SINGLE_DECL(madera_in3dmode_enum,
// C: 			    MADERA_IN3L_CONTROL,
// C: 			    MADERA_IN3_MODE_SHIFT,
// C: 			    madera_dmode_texts);
// C: 
// C: const struct snd_kcontrol_new madera_inmode[] = {
// C: 	SOC_DAPM_ENUM("IN1 Mode", madera_in1dmode_enum),
// C: 	SOC_DAPM_ENUM("IN2 Mode", madera_in2dmode_enum),
// C: 	SOC_DAPM_ENUM("IN3 Mode", madera_in3dmode_enum),
// C: };
// C: EXPORT_SYMBOL_GPL(madera_inmode);
// C: 
// C: static bool madera_can_change_grp_rate(const struct madera_priv *priv,
// C: 				       unsigned int reg)
// C: {
// C: 	int count;
// C: 
// C: 	switch (reg) {
// C: 	case MADERA_FX_CTRL1:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_FX];
// C: 		break;
// C: 	case MADERA_ASRC1_RATE1:
// C: 	case MADERA_ASRC1_RATE2:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_ASRC1];
// C: 		break;
// C: 	case MADERA_ASRC2_RATE1:
// C: 	case MADERA_ASRC2_RATE2:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_ASRC2];
// C: 		break;
// C: 	case MADERA_ISRC_1_CTRL_1:
// C: 	case MADERA_ISRC_1_CTRL_2:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_ISRC1];
// C: 		break;
// C: 	case MADERA_ISRC_2_CTRL_1:
// C: 	case MADERA_ISRC_2_CTRL_2:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_ISRC2];
// C: 		break;
// C: 	case MADERA_ISRC_3_CTRL_1:
// C: 	case MADERA_ISRC_3_CTRL_2:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_ISRC3];
// C: 		break;
// C: 	case MADERA_ISRC_4_CTRL_1:
// C: 	case MADERA_ISRC_4_CTRL_2:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_ISRC4];
// C: 		break;
// C: 	case MADERA_OUTPUT_RATE_1:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_OUT];
// C: 		break;
// C: 	case MADERA_SPD1_TX_CONTROL:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_SPD];
// C: 		break;
// C: 	case MADERA_DSP1_CONFIG_1:
// C: 	case MADERA_DSP1_CONFIG_2:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_DSP1];
// C: 		break;
// C: 	case MADERA_DSP2_CONFIG_1:
// C: 	case MADERA_DSP2_CONFIG_2:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_DSP2];
// C: 		break;
// C: 	case MADERA_DSP3_CONFIG_1:
// C: 	case MADERA_DSP3_CONFIG_2:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_DSP3];
// C: 		break;
// C: 	case MADERA_DSP4_CONFIG_1:
// C: 	case MADERA_DSP4_CONFIG_2:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_DSP4];
// C: 		break;
// C: 	case MADERA_DSP5_CONFIG_1:
// C: 	case MADERA_DSP5_CONFIG_2:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_DSP5];
// C: 		break;
// C: 	case MADERA_DSP6_CONFIG_1:
// C: 	case MADERA_DSP6_CONFIG_2:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_DSP6];
// C: 		break;
// C: 	case MADERA_DSP7_CONFIG_1:
// C: 	case MADERA_DSP7_CONFIG_2:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_DSP7];
// C: 		break;
// C: 	case MADERA_AIF1_RATE_CTRL:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_AIF1];
// C: 		break;
// C: 	case MADERA_AIF2_RATE_CTRL:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_AIF2];
// C: 		break;
// C: 	case MADERA_AIF3_RATE_CTRL:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_AIF3];
// C: 		break;
// C: 	case MADERA_AIF4_RATE_CTRL:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_AIF4];
// C: 		break;
// C: 	case MADERA_SLIMBUS_RATES_1:
// C: 	case MADERA_SLIMBUS_RATES_2:
// C: 	case MADERA_SLIMBUS_RATES_3:
// C: 	case MADERA_SLIMBUS_RATES_4:
// C: 	case MADERA_SLIMBUS_RATES_5:
// C: 	case MADERA_SLIMBUS_RATES_6:
// C: 	case MADERA_SLIMBUS_RATES_7:
// C: 	case MADERA_SLIMBUS_RATES_8:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_SLIMBUS];
// C: 		break;
// C: 	case MADERA_PWM_DRIVE_1:
// C: 		count = priv->domain_group_ref[MADERA_DOM_GRP_PWM];
// C: 		break;
// C: 	default:
// C: 		return false;
// C: 	}
// C: 
// C: 	dev_dbg(priv->madera->dev, "Rate reg 0x%x group ref %d\n", reg, count);
// C: 
// C: 	if (count)
// C: 		return false;
// C: 	else
// C: 		return true;
// C: }
// C: 
// C: static int madera_adsp_rate_get(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct soc_enum *e = (struct soc_enum *)kcontrol->private_value;
// C: 	unsigned int cached_rate;
// C: 	const int adsp_num = e->shift_l;
// C: 	int item;
// C: 
// C: 	scoped_guard(mutex, &priv->rate_lock)
// C: 		cached_rate = priv->adsp_rate_cache[adsp_num];
// C: 
// C: 	item = snd_soc_enum_val_to_item(e, cached_rate);
// C: 	ucontrol->value.enumerated.item[0] = item;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int madera_adsp_rate_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct soc_enum *e = (struct soc_enum *)kcontrol->private_value;
// C: 	const int adsp_num = e->shift_l;
// C: 	const unsigned int item = ucontrol->value.enumerated.item[0];
// C: 
// C: 	if (item >= e->items)
// C: 		return -EINVAL;
// C: 
// C: 	/*
// C: 	 * We don't directly write the rate register here but we want to
// C: 	 * maintain consistent behaviour that rate domains cannot be changed
// C: 	 * while in use since this is a hardware requirement
// C: 	 */
// C: 	guard(mutex)(&priv->rate_lock);
// C: 
// C: 	if (!madera_can_change_grp_rate(priv, priv->adsp[adsp_num].cs_dsp.base)) {
// C: 		dev_warn(priv->madera->dev,
// C: 			 "Cannot change '%s' while in use by active audio paths\n",
// C: 			 kcontrol->id.name);
// C: 		return -EBUSY;
// C: 	} else if (priv->adsp_rate_cache[adsp_num] != e->values[item]) {
// C: 		/* Volatile register so defer until the codec is powered up */
// C: 		priv->adsp_rate_cache[adsp_num] = e->values[item];
// C: 		return 1;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static const struct soc_enum madera_adsp_rate_enum[] = {
// C: 	SOC_VALUE_ENUM_SINGLE(SND_SOC_NOPM, 0, 0xf, MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: 	SOC_VALUE_ENUM_SINGLE(SND_SOC_NOPM, 1, 0xf, MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: 	SOC_VALUE_ENUM_SINGLE(SND_SOC_NOPM, 2, 0xf, MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: 	SOC_VALUE_ENUM_SINGLE(SND_SOC_NOPM, 3, 0xf, MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: 	SOC_VALUE_ENUM_SINGLE(SND_SOC_NOPM, 4, 0xf, MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: 	SOC_VALUE_ENUM_SINGLE(SND_SOC_NOPM, 5, 0xf, MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: 	SOC_VALUE_ENUM_SINGLE(SND_SOC_NOPM, 6, 0xf, MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: };
// C: 
// C: const struct snd_kcontrol_new madera_adsp_rate_controls[] = {
// C: 	SOC_ENUM_EXT("DSP1 Rate", madera_adsp_rate_enum[0],
// C: 		     madera_adsp_rate_get, madera_adsp_rate_put),
// C: 	SOC_ENUM_EXT("DSP2 Rate", madera_adsp_rate_enum[1],
// C: 		     madera_adsp_rate_get, madera_adsp_rate_put),
// C: 	SOC_ENUM_EXT("DSP3 Rate", madera_adsp_rate_enum[2],
// C: 		     madera_adsp_rate_get, madera_adsp_rate_put),
// C: 	SOC_ENUM_EXT("DSP4 Rate", madera_adsp_rate_enum[3],
// C: 		     madera_adsp_rate_get, madera_adsp_rate_put),
// C: 	SOC_ENUM_EXT("DSP5 Rate", madera_adsp_rate_enum[4],
// C: 		     madera_adsp_rate_get, madera_adsp_rate_put),
// C: 	SOC_ENUM_EXT("DSP6 Rate", madera_adsp_rate_enum[5],
// C: 		     madera_adsp_rate_get, madera_adsp_rate_put),
// C: 	SOC_ENUM_EXT("DSP7 Rate", madera_adsp_rate_enum[6],
// C: 		     madera_adsp_rate_get, madera_adsp_rate_put),
// C: };
// C: EXPORT_SYMBOL_GPL(madera_adsp_rate_controls);
// C: 
// C: static int madera_write_adsp_clk_setting(struct madera_priv *priv,
// C: 					 struct wm_adsp *dsp,
// C: 					 unsigned int freq)
// C: {
// C: 	unsigned int val;
// C: 	unsigned int mask = MADERA_DSP_RATE_MASK;
// C: 	int ret;
// C: 
// C: 	val = priv->adsp_rate_cache[dsp->cs_dsp.num - 1] << MADERA_DSP_RATE_SHIFT;
// C: 
// C: 	switch (priv->madera->type) {
// C: 	case CS47L35:
// C: 	case CS47L85:
// C: 	case WM1840:
// C: 		/* use legacy frequency registers */
// C: 		mask |= MADERA_DSP_CLK_SEL_MASK;
// C: 		val |= (freq << MADERA_DSP_CLK_SEL_SHIFT);
// C: 		break;
// C: 	default:
// C: 		/* Configure exact dsp frequency */
// C: 		dev_dbg(priv->madera->dev, "Set DSP frequency to 0x%x\n", freq);
// C: 
// C: 		ret = regmap_write(dsp->cs_dsp.regmap,
// C: 				   dsp->cs_dsp.base + MADERA_DSP_CONFIG_2_OFFS, freq);
// C: 		if (ret)
// C: 			goto err;
// C: 		break;
// C: 	}
// C: 
// C: 	ret = regmap_update_bits(dsp->cs_dsp.regmap,
// C: 				 dsp->cs_dsp.base + MADERA_DSP_CONFIG_1_OFFS,
// C: 				 mask, val);
// C: 	if (ret)
// C: 		goto err;
// C: 
// C: 	dev_dbg(priv->madera->dev, "Set DSP clocking to 0x%x\n", val);
// C: 
// C: 	return 0;
// C: 
// C: err:
// C: 	dev_err(dsp->cs_dsp.dev, "Failed to set DSP%d clock: %d\n", dsp->cs_dsp.num, ret);
// C: 
// C: 	return ret;
// C: }
// C: 
// C: int madera_set_adsp_clk(struct madera_priv *priv, int dsp_num,
// C: 			unsigned int freq)
// C: {
// C: 	struct wm_adsp *dsp = &priv->adsp[dsp_num];
// C: 	struct madera *madera = priv->madera;
// C: 	unsigned int cur, new;
// C: 	int ret;
// C: 
// C: 	/*
// C: 	 * This is called at a higher DAPM priority than the mux widgets so
// C: 	 * the muxes are still off at this point and it's safe to change
// C: 	 * the rate domain control.
// C: 	 * Also called at a lower DAPM priority than the domain group widgets
// C: 	 * so locking the reads of adsp_rate_cache is not necessary as we know
// C: 	 * changes are locked out by the domain_group_ref reference count.
// C: 	 */
// C: 
// C: 	ret = regmap_read(dsp->cs_dsp.regmap,  dsp->cs_dsp.base, &cur);
// C: 	if (ret) {
// C: 		dev_err(madera->dev,
// C: 			"Failed to read current DSP rate: %d\n", ret);
// C: 		return ret;
// C: 	}
// C: 
// C: 	cur &= MADERA_DSP_RATE_MASK;
// C: 
// C: 	new = priv->adsp_rate_cache[dsp->cs_dsp.num - 1] << MADERA_DSP_RATE_SHIFT;
// C: 
// C: 	if (new == cur) {
// C: 		dev_dbg(madera->dev, "DSP rate not changed\n");
// C: 		return madera_write_adsp_clk_setting(priv, dsp, freq);
// C: 	} else {
// C: 		dev_dbg(madera->dev, "DSP rate changed\n");
// C: 
// C: 		/* The write must be guarded by a number of SYSCLK cycles */
// C: 		madera_spin_sysclk(priv);
// C: 		ret = madera_write_adsp_clk_setting(priv, dsp, freq);
// C: 		madera_spin_sysclk(priv);
// C: 		return ret;
// C: 	}
// C: }
// C: EXPORT_SYMBOL_GPL(madera_set_adsp_clk);
// C: 
// C: int madera_rate_put(struct snd_kcontrol *kcontrol,
// C: 		    struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct soc_enum *e = (struct soc_enum *)kcontrol->private_value;
// C: 	unsigned int item = ucontrol->value.enumerated.item[0];
// C: 	unsigned int val;
// C: 	int ret;
// C: 
// C: 	if (item >= e->items)
// C: 		return -EINVAL;
// C: 
// C: 	/*
// C: 	 * Prevent the domain powering up while we're checking whether it's
// C: 	 * safe to change rate domain
// C: 	 */
// C: 	guard(mutex)(&priv->rate_lock);
// C: 
// C: 	val = snd_soc_component_read(component, e->reg);
// C: 	val >>= e->shift_l;
// C: 	val &= e->mask;
// C: 	if (snd_soc_enum_item_to_val(e, item) == val)
// C: 		return 0;
// C: 
// C: 	if (!madera_can_change_grp_rate(priv, e->reg)) {
// C: 		dev_warn(priv->madera->dev,
// C: 			 "Cannot change '%s' while in use by active audio paths\n",
// C: 			 kcontrol->id.name);
// C: 		ret = -EBUSY;
// C: 	} else {
// C: 		/* The write must be guarded by a number of SYSCLK cycles */
// C: 		madera_spin_sysclk(priv);
// C: 		ret = snd_soc_put_enum_double(kcontrol, ucontrol);
// C: 		madera_spin_sysclk(priv);
// C: 	}
// C: 
// C: 	return ret;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_rate_put);
// C: 
// C: static void madera_configure_input_mode(struct madera *madera)
// C: {
// C: 	unsigned int dig_mode, ana_mode_l, ana_mode_r;
// C: 	int max_analogue_inputs, max_dmic_sup, i;
// C: 
// C: 	switch (madera->type) {
// C: 	case CS47L15:
// C: 		max_analogue_inputs = 1;
// C: 		max_dmic_sup = 2;
// C: 		break;
// C: 	case CS47L35:
// C: 		max_analogue_inputs = 2;
// C: 		max_dmic_sup = 2;
// C: 		break;
// C: 	case CS47L85:
// C: 	case WM1840:
// C: 		max_analogue_inputs = 3;
// C: 		max_dmic_sup = 3;
// C: 		break;
// C: 	case CS47L90:
// C: 	case CS47L91:
// C: 		max_analogue_inputs = 2;
// C: 		max_dmic_sup = 2;
// C: 		break;
// C: 	default:
// C: 		max_analogue_inputs = 2;
// C: 		max_dmic_sup = 4;
// C: 		break;
// C: 	}
// C: 
// C: 	/*
// C: 	 * Initialize input modes from the A settings. For muxed inputs the
// C: 	 * B settings will be applied if the mux is changed
// C: 	 */
// C: 	for (i = 0; i < max_dmic_sup; i++) {
// C: 		dev_dbg(madera->dev, "IN%d mode %u:%u:%u:%u\n", i + 1,
// C: 			madera->pdata.codec.inmode[i][0],
// C: 			madera->pdata.codec.inmode[i][1],
// C: 			madera->pdata.codec.inmode[i][2],
// C: 			madera->pdata.codec.inmode[i][3]);
// C: 
// C: 		dig_mode = madera->pdata.codec.dmic_ref[i] <<
// C: 			   MADERA_IN1_DMIC_SUP_SHIFT;
// C: 
// C: 		switch (madera->pdata.codec.inmode[i][0]) {
// C: 		case MADERA_INMODE_DIFF:
// C: 			ana_mode_l = 0;
// C: 			break;
// C: 		case MADERA_INMODE_SE:
// C: 			ana_mode_l = 1 << MADERA_IN1L_SRC_SE_SHIFT;
// C: 			break;
// C: 		default:
// C: 			dev_warn(madera->dev,
// C: 				 "IN%dAL Illegal inmode %u ignored\n",
// C: 				 i + 1, madera->pdata.codec.inmode[i][0]);
// C: 			continue;
// C: 		}
// C: 
// C: 		switch (madera->pdata.codec.inmode[i][1]) {
// C: 		case MADERA_INMODE_DIFF:
// C: 			ana_mode_r = 0;
// C: 			break;
// C: 		case MADERA_INMODE_SE:
// C: 			ana_mode_r = 1 << MADERA_IN1R_SRC_SE_SHIFT;
// C: 			break;
// C: 		default:
// C: 			dev_warn(madera->dev,
// C: 				 "IN%dAR Illegal inmode %u ignored\n",
// C: 				 i + 1, madera->pdata.codec.inmode[i][1]);
// C: 			continue;
// C: 		}
// C: 
// C: 		dev_dbg(madera->dev,
// C: 			"IN%dA DMIC mode=0x%x Analogue mode=0x%x,0x%x\n",
// C: 			i + 1, dig_mode, ana_mode_l, ana_mode_r);
// C: 
// C: 		regmap_update_bits(madera->regmap,
// C: 				   MADERA_IN1L_CONTROL + (i * 8),
// C: 				   MADERA_IN1_DMIC_SUP_MASK, dig_mode);
// C: 
// C: 		if (i >= max_analogue_inputs)
// C: 			continue;
// C: 
// C: 		regmap_update_bits(madera->regmap,
// C: 				   MADERA_ADC_DIGITAL_VOLUME_1L + (i * 8),
// C: 				   MADERA_IN1L_SRC_SE_MASK, ana_mode_l);
// C: 
// C: 		regmap_update_bits(madera->regmap,
// C: 				   MADERA_ADC_DIGITAL_VOLUME_1R + (i * 8),
// C: 				   MADERA_IN1R_SRC_SE_MASK, ana_mode_r);
// C: 	}
// C: }
// C: 
// C: int madera_init_inputs(struct snd_soc_component *component)
// C: {
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera *madera = priv->madera;
// C: 
// C: 	madera_configure_input_mode(madera);
// C: 
// C: 	return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_init_inputs);
// C: 
// C: static const struct snd_soc_dapm_route madera_mono_routes[] = {
// C: 	{ "OUT1R", NULL, "OUT1L" },
// C: 	{ "OUT2R", NULL, "OUT2L" },
// C: 	{ "OUT3R", NULL, "OUT3L" },
// C: 	{ "OUT4R", NULL, "OUT4L" },
// C: 	{ "OUT5R", NULL, "OUT5L" },
// C: 	{ "OUT6R", NULL, "OUT6L" },
// C: };
// C: 
// C: int madera_init_outputs(struct snd_soc_component *component,
// C: 			const struct snd_soc_dapm_route *routes,
// C: 			int n_mono_routes, int n_real)
// C: {
// C: 	struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera *madera = priv->madera;
// C: 	const struct madera_codec_pdata *pdata = &madera->pdata.codec;
// C: 	unsigned int val;
// C: 	int i;
// C: 
// C: 	if (n_mono_routes > MADERA_MAX_OUTPUT) {
// C: 		dev_warn(madera->dev,
// C: 			 "Requested %d mono outputs, using maximum allowed %d\n",
// C: 			 n_mono_routes, MADERA_MAX_OUTPUT);
// C: 		n_mono_routes = MADERA_MAX_OUTPUT;
// C: 	}
// C: 
// C: 	if (!routes)
// C: 		routes = madera_mono_routes;
// C: 
// C: 	for (i = 0; i < n_mono_routes; i++) {
// C: 		/* Default is 0 so noop with defaults */
// C: 		if (pdata->out_mono[i]) {
// C: 			val = MADERA_OUT1_MONO;
// C: 			snd_soc_dapm_add_routes(dapm, &routes[i], 1);
// C: 		} else {
// C: 			val = 0;
// C: 		}
// C: 
// C: 		if (i >= n_real)
// C: 			continue;
// C: 
// C: 		regmap_update_bits(madera->regmap,
// C: 				   MADERA_OUTPUT_PATH_CONFIG_1L + (i * 8),
// C: 				   MADERA_OUT1_MONO, val);
// C: 
// C: 		dev_dbg(madera->dev, "OUT%d mono=0x%x\n", i + 1, val);
// C: 	}
// C: 
// C: 	for (i = 0; i < MADERA_MAX_PDM_SPK; i++) {
// C: 		dev_dbg(madera->dev, "PDM%d fmt=0x%x mute=0x%x\n", i + 1,
// C: 			pdata->pdm_fmt[i], pdata->pdm_mute[i]);
// C: 
// C: 		if (pdata->pdm_mute[i])
// C: 			regmap_update_bits(madera->regmap,
// C: 					   MADERA_PDM_SPK1_CTRL_1 + (i * 2),
// C: 					   MADERA_SPK1_MUTE_ENDIAN_MASK |
// C: 					   MADERA_SPK1_MUTE_SEQ1_MASK,
// C: 					   pdata->pdm_mute[i]);
// C: 
// C: 		if (pdata->pdm_fmt[i])
// C: 			regmap_update_bits(madera->regmap,
// C: 					   MADERA_PDM_SPK1_CTRL_2 + (i * 2),
// C: 					   MADERA_SPK1_FMT_MASK,
// C: 					   pdata->pdm_fmt[i]);
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_init_outputs);
// C: 
// C: int madera_init_bus_error_irq(struct madera_priv *priv, int dsp_num,
// C: 			      irq_handler_t handler)
// C: {
// C: 	struct madera *madera = priv->madera;
// C: 	int ret;
// C: 
// C: 	ret = madera_request_irq(madera,
// C: 				 madera_dsp_bus_error_irqs[dsp_num],
// C: 				 "ADSP2 bus error",
// C: 				 handler,
// C: 				 &priv->adsp[dsp_num]);
// C: 	if (ret)
// C: 		dev_err(madera->dev,
// C: 			"Failed to request DSP Lock region IRQ: %d\n", ret);
// C: 
// C: 	return ret;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_init_bus_error_irq);
// C: 
// C: void madera_free_bus_error_irq(struct madera_priv *priv, int dsp_num)
// C: {
// C: 	struct madera *madera = priv->madera;
// C: 
// C: 	madera_free_irq(madera,
// C: 			madera_dsp_bus_error_irqs[dsp_num],
// C: 			&priv->adsp[dsp_num]);
// C: }
// C: EXPORT_SYMBOL_GPL(madera_free_bus_error_irq);
// C: 
// C: const char * const madera_mixer_texts[] = {
// C: 	"None",
// C: 	"Tone Generator 1",
// C: 	"Tone Generator 2",
// C: 	"Haptics",
// C: 	"AEC1",
// C: 	"AEC2",
// C: 	"Mic Mute Mixer",
// C: 	"Noise Generator",
// C: 	"IN1L",
// C: 	"IN1R",
// C: 	"IN2L",
// C: 	"IN2R",
// C: 	"IN3L",
// C: 	"IN3R",
// C: 	"IN4L",
// C: 	"IN4R",
// C: 	"IN5L",
// C: 	"IN5R",
// C: 	"IN6L",
// C: 	"IN6R",
// C: 	"AIF1RX1",
// C: 	"AIF1RX2",
// C: 	"AIF1RX3",
// C: 	"AIF1RX4",
// C: 	"AIF1RX5",
// C: 	"AIF1RX6",
// C: 	"AIF1RX7",
// C: 	"AIF1RX8",
// C: 	"AIF2RX1",
// C: 	"AIF2RX2",
// C: 	"AIF2RX3",
// C: 	"AIF2RX4",
// C: 	"AIF2RX5",
// C: 	"AIF2RX6",
// C: 	"AIF2RX7",
// C: 	"AIF2RX8",
// C: 	"AIF3RX1",
// C: 	"AIF3RX2",
// C: 	"AIF3RX3",
// C: 	"AIF3RX4",
// C: 	"AIF4RX1",
// C: 	"AIF4RX2",
// C: 	"SLIMRX1",
// C: 	"SLIMRX2",
// C: 	"SLIMRX3",
// C: 	"SLIMRX4",
// C: 	"SLIMRX5",
// C: 	"SLIMRX6",
// C: 	"SLIMRX7",
// C: 	"SLIMRX8",
// C: 	"EQ1",
// C: 	"EQ2",
// C: 	"EQ3",
// C: 	"EQ4",
// C: 	"DRC1L",
// C: 	"DRC1R",
// C: 	"DRC2L",
// C: 	"DRC2R",
// C: 	"LHPF1",
// C: 	"LHPF2",
// C: 	"LHPF3",
// C: 	"LHPF4",
// C: 	"DSP1.1",
// C: 	"DSP1.2",
// C: 	"DSP1.3",
// C: 	"DSP1.4",
// C: 	"DSP1.5",
// C: 	"DSP1.6",
// C: 	"DSP2.1",
// C: 	"DSP2.2",
// C: 	"DSP2.3",
// C: 	"DSP2.4",
// C: 	"DSP2.5",
// C: 	"DSP2.6",
// C: 	"DSP3.1",
// C: 	"DSP3.2",
// C: 	"DSP3.3",
// C: 	"DSP3.4",
// C: 	"DSP3.5",
// C: 	"DSP3.6",
// C: 	"DSP4.1",
// C: 	"DSP4.2",
// C: 	"DSP4.3",
// C: 	"DSP4.4",
// C: 	"DSP4.5",
// C: 	"DSP4.6",
// C: 	"DSP5.1",
// C: 	"DSP5.2",
// C: 	"DSP5.3",
// C: 	"DSP5.4",
// C: 	"DSP5.5",
// C: 	"DSP5.6",
// C: 	"DSP6.1",
// C: 	"DSP6.2",
// C: 	"DSP6.3",
// C: 	"DSP6.4",
// C: 	"DSP6.5",
// C: 	"DSP6.6",
// C: 	"DSP7.1",
// C: 	"DSP7.2",
// C: 	"DSP7.3",
// C: 	"DSP7.4",
// C: 	"DSP7.5",
// C: 	"DSP7.6",
// C: 	"ASRC1IN1L",
// C: 	"ASRC1IN1R",
// C: 	"ASRC1IN2L",
// C: 	"ASRC1IN2R",
// C: 	"ASRC2IN1L",
// C: 	"ASRC2IN1R",
// C: 	"ASRC2IN2L",
// C: 	"ASRC2IN2R",
// C: 	"ISRC1INT1",
// C: 	"ISRC1INT2",
// C: 	"ISRC1INT3",
// C: 	"ISRC1INT4",
// C: 	"ISRC1DEC1",
// C: 	"ISRC1DEC2",
// C: 	"ISRC1DEC3",
// C: 	"ISRC1DEC4",
// C: 	"ISRC2INT1",
// C: 	"ISRC2INT2",
// C: 	"ISRC2INT3",
// C: 	"ISRC2INT4",
// C: 	"ISRC2DEC1",
// C: 	"ISRC2DEC2",
// C: 	"ISRC2DEC3",
// C: 	"ISRC2DEC4",
// C: 	"ISRC3INT1",
// C: 	"ISRC3INT2",
// C: 	"ISRC3INT3",
// C: 	"ISRC3INT4",
// C: 	"ISRC3DEC1",
// C: 	"ISRC3DEC2",
// C: 	"ISRC3DEC3",
// C: 	"ISRC3DEC4",
// C: 	"ISRC4INT1",
// C: 	"ISRC4INT2",
// C: 	"ISRC4DEC1",
// C: 	"ISRC4DEC2",
// C: 	"DFC1",
// C: 	"DFC2",
// C: 	"DFC3",
// C: 	"DFC4",
// C: 	"DFC5",
// C: 	"DFC6",
// C: 	"DFC7",
// C: 	"DFC8",
// C: };
// C: EXPORT_SYMBOL_GPL(madera_mixer_texts);
// C: 
// C: const unsigned int madera_mixer_values[] = {
// C: 	0x00,	/* None */
// C: 	0x04,	/* Tone Generator 1 */
// C: 	0x05,	/* Tone Generator 2 */
// C: 	0x06,	/* Haptics */
// C: 	0x08,	/* AEC */
// C: 	0x09,	/* AEC2 */
// C: 	0x0c,	/* Noise mixer */
// C: 	0x0d,	/* Comfort noise */
// C: 	0x10,	/* IN1L */
// C: 	0x11,
// C: 	0x12,
// C: 	0x13,
// C: 	0x14,
// C: 	0x15,
// C: 	0x16,
// C: 	0x17,
// C: 	0x18,
// C: 	0x19,
// C: 	0x1A,
// C: 	0x1B,
// C: 	0x20,	/* AIF1RX1 */
// C: 	0x21,
// C: 	0x22,
// C: 	0x23,
// C: 	0x24,
// C: 	0x25,
// C: 	0x26,
// C: 	0x27,
// C: 	0x28,	/* AIF2RX1 */
// C: 	0x29,
// C: 	0x2a,
// C: 	0x2b,
// C: 	0x2c,
// C: 	0x2d,
// C: 	0x2e,
// C: 	0x2f,
// C: 	0x30,	/* AIF3RX1 */
// C: 	0x31,
// C: 	0x32,
// C: 	0x33,
// C: 	0x34,	/* AIF4RX1 */
// C: 	0x35,
// C: 	0x38,	/* SLIMRX1 */
// C: 	0x39,
// C: 	0x3a,
// C: 	0x3b,
// C: 	0x3c,
// C: 	0x3d,
// C: 	0x3e,
// C: 	0x3f,
// C: 	0x50,	/* EQ1 */
// C: 	0x51,
// C: 	0x52,
// C: 	0x53,
// C: 	0x58,	/* DRC1L */
// C: 	0x59,
// C: 	0x5a,
// C: 	0x5b,
// C: 	0x60,	/* LHPF1 */
// C: 	0x61,
// C: 	0x62,
// C: 	0x63,
// C: 	0x68,	/* DSP1.1 */
// C: 	0x69,
// C: 	0x6a,
// C: 	0x6b,
// C: 	0x6c,
// C: 	0x6d,
// C: 	0x70,	/* DSP2.1 */
// C: 	0x71,
// C: 	0x72,
// C: 	0x73,
// C: 	0x74,
// C: 	0x75,
// C: 	0x78,	/* DSP3.1 */
// C: 	0x79,
// C: 	0x7a,
// C: 	0x7b,
// C: 	0x7c,
// C: 	0x7d,
// C: 	0x80,	/* DSP4.1 */
// C: 	0x81,
// C: 	0x82,
// C: 	0x83,
// C: 	0x84,
// C: 	0x85,
// C: 	0x88,	/* DSP5.1 */
// C: 	0x89,
// C: 	0x8a,
// C: 	0x8b,
// C: 	0x8c,
// C: 	0x8d,
// C: 	0xc0,	/* DSP6.1 */
// C: 	0xc1,
// C: 	0xc2,
// C: 	0xc3,
// C: 	0xc4,
// C: 	0xc5,
// C: 	0xc8,	/* DSP7.1 */
// C: 	0xc9,
// C: 	0xca,
// C: 	0xcb,
// C: 	0xcc,
// C: 	0xcd,
// C: 	0x90,	/* ASRC1IN1L */
// C: 	0x91,
// C: 	0x92,
// C: 	0x93,
// C: 	0x94,	/* ASRC2IN1L */
// C: 	0x95,
// C: 	0x96,
// C: 	0x97,
// C: 	0xa0,	/* ISRC1INT1 */
// C: 	0xa1,
// C: 	0xa2,
// C: 	0xa3,
// C: 	0xa4,	/* ISRC1DEC1 */
// C: 	0xa5,
// C: 	0xa6,
// C: 	0xa7,
// C: 	0xa8,	/* ISRC2DEC1 */
// C: 	0xa9,
// C: 	0xaa,
// C: 	0xab,
// C: 	0xac,	/* ISRC2INT1 */
// C: 	0xad,
// C: 	0xae,
// C: 	0xaf,
// C: 	0xb0,	/* ISRC3DEC1 */
// C: 	0xb1,
// C: 	0xb2,
// C: 	0xb3,
// C: 	0xb4,	/* ISRC3INT1 */
// C: 	0xb5,
// C: 	0xb6,
// C: 	0xb7,
// C: 	0xb8,	/* ISRC4INT1 */
// C: 	0xb9,
// C: 	0xbc,	/* ISRC4DEC1 */
// C: 	0xbd,
// C: 	0xf8,	/* DFC1 */
// C: 	0xf9,
// C: 	0xfa,
// C: 	0xfb,
// C: 	0xfc,
// C: 	0xfd,
// C: 	0xfe,
// C: 	0xff,	/* DFC8 */
// C: };
// C: EXPORT_SYMBOL_GPL(madera_mixer_values);
// C: 
// C: const DECLARE_TLV_DB_SCALE(madera_ana_tlv, 0, 100, 0);
// C: EXPORT_SYMBOL_GPL(madera_ana_tlv);
// C: 
// C: const DECLARE_TLV_DB_SCALE(madera_eq_tlv, -1200, 100, 0);
// C: EXPORT_SYMBOL_GPL(madera_eq_tlv);
// C: 
// C: const DECLARE_TLV_DB_SCALE(madera_digital_tlv, -6400, 50, 0);
// C: EXPORT_SYMBOL_GPL(madera_digital_tlv);
// C: 
// C: const DECLARE_TLV_DB_SCALE(madera_noise_tlv, -13200, 600, 0);
// C: EXPORT_SYMBOL_GPL(madera_noise_tlv);
// C: 
// C: const DECLARE_TLV_DB_SCALE(madera_ng_tlv, -12000, 600, 0);
// C: EXPORT_SYMBOL_GPL(madera_ng_tlv);
// C: 
// C: const DECLARE_TLV_DB_SCALE(madera_mixer_tlv, -3200, 100, 0);
// C: EXPORT_SYMBOL_GPL(madera_mixer_tlv);
// C: 
// C: const char * const madera_rate_text[MADERA_RATE_ENUM_SIZE] = {
// C: 	"SYNCCLK rate 1", "SYNCCLK rate 2", "SYNCCLK rate 3",
// C: 	"ASYNCCLK rate 1", "ASYNCCLK rate 2",
// C: };
// C: EXPORT_SYMBOL_GPL(madera_rate_text);
// C: 
// C: const unsigned int madera_rate_val[MADERA_RATE_ENUM_SIZE] = {
// C: 	0x0, 0x1, 0x2, 0x8, 0x9,
// C: };
// C: EXPORT_SYMBOL_GPL(madera_rate_val);
// C: 
// C: static const char * const madera_dfc_width_text[MADERA_DFC_WIDTH_ENUM_SIZE] = {
// C: 	"8 bit", "16 bit", "20 bit", "24 bit", "32 bit",
// C: };
// C: 
// C: static const unsigned int madera_dfc_width_val[MADERA_DFC_WIDTH_ENUM_SIZE] = {
// C: 	7, 15, 19, 23, 31,
// C: };
// C: 
// C: static const char * const madera_dfc_type_text[MADERA_DFC_TYPE_ENUM_SIZE] = {
// C: 	"Fixed", "Unsigned Fixed", "Single Precision Floating",
// C: 	"Half Precision Floating", "Arm Alternative Floating",
// C: };
// C: 
// C: static const unsigned int madera_dfc_type_val[MADERA_DFC_TYPE_ENUM_SIZE] = {
// C: 	0, 1, 2, 4, 5,
// C: };
// C: 
// C: const struct soc_enum madera_dfc_width[] = {
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC1_RX,
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_SHIFT,
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_MASK >>
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_width_text),
// C: 			      madera_dfc_width_text,
// C: 			      madera_dfc_width_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC1_TX,
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_SHIFT,
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_MASK >>
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_width_text),
// C: 			      madera_dfc_width_text,
// C: 			      madera_dfc_width_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC2_RX,
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_SHIFT,
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_MASK >>
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_width_text),
// C: 			      madera_dfc_width_text,
// C: 			      madera_dfc_width_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC2_TX,
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_SHIFT,
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_MASK >>
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_width_text),
// C: 			      madera_dfc_width_text,
// C: 			      madera_dfc_width_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC3_RX,
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_SHIFT,
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_MASK >>
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_width_text),
// C: 			      madera_dfc_width_text,
// C: 			      madera_dfc_width_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC3_TX,
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_SHIFT,
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_MASK >>
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_width_text),
// C: 			      madera_dfc_width_text,
// C: 			      madera_dfc_width_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC4_RX,
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_SHIFT,
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_MASK >>
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_width_text),
// C: 			      madera_dfc_width_text,
// C: 			      madera_dfc_width_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC4_TX,
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_SHIFT,
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_MASK >>
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_width_text),
// C: 			      madera_dfc_width_text,
// C: 			      madera_dfc_width_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC5_RX,
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_SHIFT,
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_MASK >>
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_width_text),
// C: 			      madera_dfc_width_text,
// C: 			      madera_dfc_width_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC5_TX,
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_SHIFT,
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_MASK >>
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_width_text),
// C: 			      madera_dfc_width_text,
// C: 			      madera_dfc_width_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC6_RX,
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_SHIFT,
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_MASK >>
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_width_text),
// C: 			      madera_dfc_width_text,
// C: 			      madera_dfc_width_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC6_TX,
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_SHIFT,
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_MASK >>
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_width_text),
// C: 			      madera_dfc_width_text,
// C: 			      madera_dfc_width_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC7_RX,
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_SHIFT,
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_MASK >>
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_width_text),
// C: 			      madera_dfc_width_text,
// C: 			      madera_dfc_width_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC7_TX,
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_SHIFT,
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_MASK >>
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_width_text),
// C: 			      madera_dfc_width_text,
// C: 			      madera_dfc_width_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC8_RX,
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_SHIFT,
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_MASK >>
// C: 			      MADERA_DFC1_RX_DATA_WIDTH_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_width_text),
// C: 			      madera_dfc_width_text,
// C: 			      madera_dfc_width_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC8_TX,
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_SHIFT,
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_MASK >>
// C: 			      MADERA_DFC1_TX_DATA_WIDTH_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_width_text),
// C: 			      madera_dfc_width_text,
// C: 			      madera_dfc_width_val),
// C: };
// C: EXPORT_SYMBOL_GPL(madera_dfc_width);
// C: 
// C: const struct soc_enum madera_dfc_type[] = {
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC1_RX,
// C: 			      MADERA_DFC1_RX_DATA_TYPE_SHIFT,
// C: 			      MADERA_DFC1_RX_DATA_TYPE_MASK >>
// C: 			      MADERA_DFC1_RX_DATA_TYPE_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_type_text),
// C: 			      madera_dfc_type_text,
// C: 			      madera_dfc_type_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC1_TX,
// C: 			      MADERA_DFC1_TX_DATA_TYPE_SHIFT,
// C: 			      MADERA_DFC1_TX_DATA_TYPE_MASK >>
// C: 			      MADERA_DFC1_TX_DATA_TYPE_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_type_text),
// C: 			      madera_dfc_type_text,
// C: 			      madera_dfc_type_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC2_RX,
// C: 			      MADERA_DFC1_RX_DATA_TYPE_SHIFT,
// C: 			      MADERA_DFC1_RX_DATA_TYPE_MASK >>
// C: 			      MADERA_DFC1_RX_DATA_TYPE_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_type_text),
// C: 			      madera_dfc_type_text,
// C: 			      madera_dfc_type_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC2_TX,
// C: 			      MADERA_DFC1_TX_DATA_TYPE_SHIFT,
// C: 			      MADERA_DFC1_TX_DATA_TYPE_MASK >>
// C: 			      MADERA_DFC1_TX_DATA_TYPE_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_type_text),
// C: 			      madera_dfc_type_text,
// C: 			      madera_dfc_type_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC3_RX,
// C: 			      MADERA_DFC1_RX_DATA_TYPE_SHIFT,
// C: 			      MADERA_DFC1_RX_DATA_TYPE_MASK >>
// C: 			      MADERA_DFC1_RX_DATA_TYPE_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_type_text),
// C: 			      madera_dfc_type_text,
// C: 			      madera_dfc_type_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC3_TX,
// C: 			      MADERA_DFC1_TX_DATA_TYPE_SHIFT,
// C: 			      MADERA_DFC1_TX_DATA_TYPE_MASK >>
// C: 			      MADERA_DFC1_TX_DATA_TYPE_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_type_text),
// C: 			      madera_dfc_type_text,
// C: 			      madera_dfc_type_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC4_RX,
// C: 			      MADERA_DFC1_RX_DATA_TYPE_SHIFT,
// C: 			      MADERA_DFC1_RX_DATA_TYPE_MASK >>
// C: 			      MADERA_DFC1_RX_DATA_TYPE_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_type_text),
// C: 			      madera_dfc_type_text,
// C: 			      madera_dfc_type_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC4_TX,
// C: 			      MADERA_DFC1_TX_DATA_TYPE_SHIFT,
// C: 			      MADERA_DFC1_TX_DATA_TYPE_MASK >>
// C: 			      MADERA_DFC1_TX_DATA_TYPE_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_type_text),
// C: 			      madera_dfc_type_text,
// C: 			      madera_dfc_type_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC5_RX,
// C: 			      MADERA_DFC1_RX_DATA_TYPE_SHIFT,
// C: 			      MADERA_DFC1_RX_DATA_TYPE_MASK >>
// C: 			      MADERA_DFC1_RX_DATA_TYPE_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_type_text),
// C: 			      madera_dfc_type_text,
// C: 			      madera_dfc_type_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC5_TX,
// C: 			      MADERA_DFC1_TX_DATA_TYPE_SHIFT,
// C: 			      MADERA_DFC1_TX_DATA_TYPE_MASK >>
// C: 			      MADERA_DFC1_TX_DATA_TYPE_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_type_text),
// C: 			      madera_dfc_type_text,
// C: 			      madera_dfc_type_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC6_RX,
// C: 			      MADERA_DFC1_RX_DATA_TYPE_SHIFT,
// C: 			      MADERA_DFC1_RX_DATA_TYPE_MASK >>
// C: 			      MADERA_DFC1_RX_DATA_TYPE_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_type_text),
// C: 			      madera_dfc_type_text,
// C: 			      madera_dfc_type_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC6_TX,
// C: 			      MADERA_DFC1_TX_DATA_TYPE_SHIFT,
// C: 			      MADERA_DFC1_TX_DATA_TYPE_MASK >>
// C: 			      MADERA_DFC1_TX_DATA_TYPE_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_type_text),
// C: 			      madera_dfc_type_text,
// C: 			      madera_dfc_type_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC7_RX,
// C: 			      MADERA_DFC1_RX_DATA_TYPE_SHIFT,
// C: 			      MADERA_DFC1_RX_DATA_TYPE_MASK >>
// C: 			      MADERA_DFC1_RX_DATA_TYPE_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_type_text),
// C: 			      madera_dfc_type_text,
// C: 			      madera_dfc_type_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC7_TX,
// C: 			      MADERA_DFC1_TX_DATA_TYPE_SHIFT,
// C: 			      MADERA_DFC1_TX_DATA_TYPE_MASK >>
// C: 			      MADERA_DFC1_TX_DATA_TYPE_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_type_text),
// C: 			      madera_dfc_type_text,
// C: 			      madera_dfc_type_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC8_RX,
// C: 			      MADERA_DFC1_RX_DATA_TYPE_SHIFT,
// C: 			      MADERA_DFC1_RX_DATA_TYPE_MASK >>
// C: 			      MADERA_DFC1_RX_DATA_TYPE_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_type_text),
// C: 			      madera_dfc_type_text,
// C: 			      madera_dfc_type_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DFC8_TX,
// C: 			      MADERA_DFC1_TX_DATA_TYPE_SHIFT,
// C: 			      MADERA_DFC1_TX_DATA_TYPE_MASK >>
// C: 			      MADERA_DFC1_TX_DATA_TYPE_SHIFT,
// C: 			      ARRAY_SIZE(madera_dfc_type_text),
// C: 			      madera_dfc_type_text,
// C: 			      madera_dfc_type_val),
// C: };
// C: EXPORT_SYMBOL_GPL(madera_dfc_type);
// C: 
// C: const struct soc_enum madera_isrc_fsh[] = {
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_ISRC_1_CTRL_1,
// C: 			      MADERA_ISRC1_FSH_SHIFT, 0xf,
// C: 			      MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_ISRC_2_CTRL_1,
// C: 			      MADERA_ISRC2_FSH_SHIFT, 0xf,
// C: 			      MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_ISRC_3_CTRL_1,
// C: 			      MADERA_ISRC3_FSH_SHIFT, 0xf,
// C: 			      MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_ISRC_4_CTRL_1,
// C: 			      MADERA_ISRC4_FSH_SHIFT, 0xf,
// C: 			      MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: };
// C: EXPORT_SYMBOL_GPL(madera_isrc_fsh);
// C: 
// C: const struct soc_enum madera_isrc_fsl[] = {
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_ISRC_1_CTRL_2,
// C: 			      MADERA_ISRC1_FSL_SHIFT, 0xf,
// C: 			      MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_ISRC_2_CTRL_2,
// C: 			      MADERA_ISRC2_FSL_SHIFT, 0xf,
// C: 			      MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_ISRC_3_CTRL_2,
// C: 			      MADERA_ISRC3_FSL_SHIFT, 0xf,
// C: 			      MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_ISRC_4_CTRL_2,
// C: 			      MADERA_ISRC4_FSL_SHIFT, 0xf,
// C: 			      MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: };
// C: EXPORT_SYMBOL_GPL(madera_isrc_fsl);
// C: 
// C: const struct soc_enum madera_asrc1_rate[] = {
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_ASRC1_RATE1,
// C: 			      MADERA_ASRC1_RATE1_SHIFT, 0xf,
// C: 			      MADERA_SYNC_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_ASRC1_RATE2,
// C: 			      MADERA_ASRC1_RATE1_SHIFT, 0xf,
// C: 			      MADERA_ASYNC_RATE_ENUM_SIZE,
// C: 			      madera_rate_text + MADERA_SYNC_RATE_ENUM_SIZE,
// C: 			      madera_rate_val + MADERA_SYNC_RATE_ENUM_SIZE),
// C: };
// C: EXPORT_SYMBOL_GPL(madera_asrc1_rate);
// C: 
// C: const struct soc_enum madera_asrc1_bidir_rate[] = {
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_ASRC1_RATE1,
// C: 			      MADERA_ASRC1_RATE1_SHIFT, 0xf,
// C: 			      MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_ASRC1_RATE2,
// C: 			      MADERA_ASRC1_RATE2_SHIFT, 0xf,
// C: 			      MADERA_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: };
// C: EXPORT_SYMBOL_GPL(madera_asrc1_bidir_rate);
// C: 
// C: const struct soc_enum madera_asrc2_rate[] = {
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_ASRC2_RATE1,
// C: 			      MADERA_ASRC2_RATE1_SHIFT, 0xf,
// C: 			      MADERA_SYNC_RATE_ENUM_SIZE,
// C: 			      madera_rate_text, madera_rate_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_ASRC2_RATE2,
// C: 			      MADERA_ASRC2_RATE2_SHIFT, 0xf,
// C: 			      MADERA_ASYNC_RATE_ENUM_SIZE,
// C: 			      madera_rate_text + MADERA_SYNC_RATE_ENUM_SIZE,
// C: 			      madera_rate_val + MADERA_SYNC_RATE_ENUM_SIZE),
// C: };
// C: EXPORT_SYMBOL_GPL(madera_asrc2_rate);
// C: 
// C: static const char * const madera_vol_ramp_text[] = {
// C: 	"0ms/6dB", "0.5ms/6dB", "1ms/6dB", "2ms/6dB", "4ms/6dB", "8ms/6dB",
// C: 	"15ms/6dB", "30ms/6dB",
// C: };
// C: 
// C: SOC_ENUM_SINGLE_DECL(madera_in_vd_ramp,
// C: 		     MADERA_INPUT_VOLUME_RAMP,
// C: 		     MADERA_IN_VD_RAMP_SHIFT,
// C: 		     madera_vol_ramp_text);
// C: EXPORT_SYMBOL_GPL(madera_in_vd_ramp);
// C: 
// C: SOC_ENUM_SINGLE_DECL(madera_in_vi_ramp,
// C: 		     MADERA_INPUT_VOLUME_RAMP,
// C: 		     MADERA_IN_VI_RAMP_SHIFT,
// C: 		     madera_vol_ramp_text);
// C: EXPORT_SYMBOL_GPL(madera_in_vi_ramp);
// C: 
// C: SOC_ENUM_SINGLE_DECL(madera_out_vd_ramp,
// C: 		     MADERA_OUTPUT_VOLUME_RAMP,
// C: 		     MADERA_OUT_VD_RAMP_SHIFT,
// C: 		     madera_vol_ramp_text);
// C: EXPORT_SYMBOL_GPL(madera_out_vd_ramp);
// C: 
// C: SOC_ENUM_SINGLE_DECL(madera_out_vi_ramp,
// C: 		     MADERA_OUTPUT_VOLUME_RAMP,
// C: 		     MADERA_OUT_VI_RAMP_SHIFT,
// C: 		     madera_vol_ramp_text);
// C: EXPORT_SYMBOL_GPL(madera_out_vi_ramp);
// C: 
// C: static const char * const madera_lhpf_mode_text[] = {
// C: 	"Low-pass", "High-pass"
// C: };
// C: 
// C: SOC_ENUM_SINGLE_DECL(madera_lhpf1_mode,
// C: 		     MADERA_HPLPF1_1,
// C: 		     MADERA_LHPF1_MODE_SHIFT,
// C: 		     madera_lhpf_mode_text);
// C: EXPORT_SYMBOL_GPL(madera_lhpf1_mode);
// C: 
// C: SOC_ENUM_SINGLE_DECL(madera_lhpf2_mode,
// C: 		     MADERA_HPLPF2_1,
// C: 		     MADERA_LHPF2_MODE_SHIFT,
// C: 		     madera_lhpf_mode_text);
// C: EXPORT_SYMBOL_GPL(madera_lhpf2_mode);
// C: 
// C: SOC_ENUM_SINGLE_DECL(madera_lhpf3_mode,
// C: 		     MADERA_HPLPF3_1,
// C: 		     MADERA_LHPF3_MODE_SHIFT,
// C: 		     madera_lhpf_mode_text);
// C: EXPORT_SYMBOL_GPL(madera_lhpf3_mode);
// C: 
// C: SOC_ENUM_SINGLE_DECL(madera_lhpf4_mode,
// C: 		     MADERA_HPLPF4_1,
// C: 		     MADERA_LHPF4_MODE_SHIFT,
// C: 		     madera_lhpf_mode_text);
// C: EXPORT_SYMBOL_GPL(madera_lhpf4_mode);
// C: 
// C: static const char * const madera_ng_hold_text[] = {
// C: 	"30ms", "120ms", "250ms", "500ms",
// C: };
// C: 
// C: SOC_ENUM_SINGLE_DECL(madera_ng_hold,
// C: 		     MADERA_NOISE_GATE_CONTROL,
// C: 		     MADERA_NGATE_HOLD_SHIFT,
// C: 		     madera_ng_hold_text);
// C: EXPORT_SYMBOL_GPL(madera_ng_hold);
// C: 
// C: static const char * const madera_in_hpf_cut_text[] = {
// C: 	"2.5Hz", "5Hz", "10Hz", "20Hz", "40Hz"
// C: };
// C: 
// C: SOC_ENUM_SINGLE_DECL(madera_in_hpf_cut_enum,
// C: 		     MADERA_HPF_CONTROL,
// C: 		     MADERA_IN_HPF_CUT_SHIFT,
// C: 		     madera_in_hpf_cut_text);
// C: EXPORT_SYMBOL_GPL(madera_in_hpf_cut_enum);
// C: 
// C: static const char * const madera_in_dmic_osr_text[MADERA_OSR_ENUM_SIZE] = {
// C: 	"384kHz", "768kHz", "1.536MHz", "3.072MHz", "6.144MHz",
// C: };
// C: 
// C: static const unsigned int madera_in_dmic_osr_val[MADERA_OSR_ENUM_SIZE] = {
// C: 	2, 3, 4, 5, 6,
// C: };
// C: 
// C: const struct soc_enum madera_in_dmic_osr[] = {
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DMIC1L_CONTROL, MADERA_IN1_OSR_SHIFT,
// C: 			      0x7, MADERA_OSR_ENUM_SIZE,
// C: 			      madera_in_dmic_osr_text, madera_in_dmic_osr_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DMIC2L_CONTROL, MADERA_IN2_OSR_SHIFT,
// C: 			      0x7, MADERA_OSR_ENUM_SIZE,
// C: 			      madera_in_dmic_osr_text, madera_in_dmic_osr_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DMIC3L_CONTROL, MADERA_IN3_OSR_SHIFT,
// C: 			      0x7, MADERA_OSR_ENUM_SIZE,
// C: 			      madera_in_dmic_osr_text, madera_in_dmic_osr_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DMIC4L_CONTROL, MADERA_IN4_OSR_SHIFT,
// C: 			      0x7, MADERA_OSR_ENUM_SIZE,
// C: 			      madera_in_dmic_osr_text, madera_in_dmic_osr_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DMIC5L_CONTROL, MADERA_IN5_OSR_SHIFT,
// C: 			      0x7, MADERA_OSR_ENUM_SIZE,
// C: 			      madera_in_dmic_osr_text, madera_in_dmic_osr_val),
// C: 	SOC_VALUE_ENUM_SINGLE(MADERA_DMIC6L_CONTROL, MADERA_IN6_OSR_SHIFT,
// C: 			      0x7, MADERA_OSR_ENUM_SIZE,
// C: 			      madera_in_dmic_osr_text, madera_in_dmic_osr_val),
// C: };
// C: EXPORT_SYMBOL_GPL(madera_in_dmic_osr);
// C: 
// C: static const char * const madera_anc_input_src_text[] = {
// C: 	"None", "IN1", "IN2", "IN3", "IN4", "IN5", "IN6",
// C: };
// C: 
// C: static const char * const madera_anc_channel_src_text[] = {
// C: 	"None", "Left", "Right", "Combine",
// C: };
// C: 
// C: const struct soc_enum madera_anc_input_src[] = {
// C: 	SOC_ENUM_SINGLE(MADERA_ANC_SRC,
// C: 			MADERA_IN_RXANCL_SEL_SHIFT,
// C: 			ARRAY_SIZE(madera_anc_input_src_text),
// C: 			madera_anc_input_src_text),
// C: 	SOC_ENUM_SINGLE(MADERA_FCL_ADC_REFORMATTER_CONTROL,
// C: 			MADERA_FCL_MIC_MODE_SEL_SHIFT,
// C: 			ARRAY_SIZE(madera_anc_channel_src_text),
// C: 			madera_anc_channel_src_text),
// C: 	SOC_ENUM_SINGLE(MADERA_ANC_SRC,
// C: 			MADERA_IN_RXANCR_SEL_SHIFT,
// C: 			ARRAY_SIZE(madera_anc_input_src_text),
// C: 			madera_anc_input_src_text),
// C: 	SOC_ENUM_SINGLE(MADERA_FCR_ADC_REFORMATTER_CONTROL,
// C: 			MADERA_FCR_MIC_MODE_SEL_SHIFT,
// C: 			ARRAY_SIZE(madera_anc_channel_src_text),
// C: 			madera_anc_channel_src_text),
// C: };
// C: EXPORT_SYMBOL_GPL(madera_anc_input_src);
// C: 
// C: static const char * const madera_anc_ng_texts[] = {
// C: 	"None", "Internal", "External",
// C: };
// C: 
// C: SOC_ENUM_SINGLE_DECL(madera_anc_ng_enum, SND_SOC_NOPM, 0, madera_anc_ng_texts);
// C: EXPORT_SYMBOL_GPL(madera_anc_ng_enum);
// C: 
// C: static const char * const madera_out_anc_src_text[] = {
// C: 	"None", "RXANCL", "RXANCR",
// C: };
// C: 
// C: const struct soc_enum madera_output_anc_src[] = {
// C: 	SOC_ENUM_SINGLE(MADERA_OUTPUT_PATH_CONFIG_1L,
// C: 			MADERA_OUT1L_ANC_SRC_SHIFT,
// C: 			ARRAY_SIZE(madera_out_anc_src_text),
// C: 			madera_out_anc_src_text),
// C: 	SOC_ENUM_SINGLE(MADERA_OUTPUT_PATH_CONFIG_1R,
// C: 			MADERA_OUT1R_ANC_SRC_SHIFT,
// C: 			ARRAY_SIZE(madera_out_anc_src_text),
// C: 			madera_out_anc_src_text),
// C: 	SOC_ENUM_SINGLE(MADERA_OUTPUT_PATH_CONFIG_2L,
// C: 			MADERA_OUT2L_ANC_SRC_SHIFT,
// C: 			ARRAY_SIZE(madera_out_anc_src_text),
// C: 			madera_out_anc_src_text),
// C: 	SOC_ENUM_SINGLE(MADERA_OUTPUT_PATH_CONFIG_2R,
// C: 			MADERA_OUT2R_ANC_SRC_SHIFT,
// C: 			ARRAY_SIZE(madera_out_anc_src_text),
// C: 			madera_out_anc_src_text),
// C: 	SOC_ENUM_SINGLE(MADERA_OUTPUT_PATH_CONFIG_3L,
// C: 			MADERA_OUT3L_ANC_SRC_SHIFT,
// C: 			ARRAY_SIZE(madera_out_anc_src_text),
// C: 			madera_out_anc_src_text),
// C: 	SOC_ENUM_SINGLE(MADERA_OUTPUT_PATH_CONFIG_3R,
// C: 			MADERA_OUT3R_ANC_SRC_SHIFT,
// C: 			ARRAY_SIZE(madera_out_anc_src_text),
// C: 			madera_out_anc_src_text),
// C: 	SOC_ENUM_SINGLE(MADERA_OUTPUT_PATH_CONFIG_4L,
// C: 			MADERA_OUT4L_ANC_SRC_SHIFT,
// C: 			ARRAY_SIZE(madera_out_anc_src_text),
// C: 			madera_out_anc_src_text),
// C: 	SOC_ENUM_SINGLE(MADERA_OUTPUT_PATH_CONFIG_4R,
// C: 			MADERA_OUT4R_ANC_SRC_SHIFT,
// C: 			ARRAY_SIZE(madera_out_anc_src_text),
// C: 			madera_out_anc_src_text),
// C: 	SOC_ENUM_SINGLE(MADERA_OUTPUT_PATH_CONFIG_5L,
// C: 			MADERA_OUT5L_ANC_SRC_SHIFT,
// C: 			ARRAY_SIZE(madera_out_anc_src_text),
// C: 			madera_out_anc_src_text),
// C: 	SOC_ENUM_SINGLE(MADERA_OUTPUT_PATH_CONFIG_5R,
// C: 			MADERA_OUT5R_ANC_SRC_SHIFT,
// C: 			ARRAY_SIZE(madera_out_anc_src_text),
// C: 			madera_out_anc_src_text),
// C: 	SOC_ENUM_SINGLE(MADERA_OUTPUT_PATH_CONFIG_6L,
// C: 			MADERA_OUT6L_ANC_SRC_SHIFT,
// C: 			ARRAY_SIZE(madera_out_anc_src_text),
// C: 			madera_out_anc_src_text),
// C: 	SOC_ENUM_SINGLE(MADERA_OUTPUT_PATH_CONFIG_6R,
// C: 			MADERA_OUT6R_ANC_SRC_SHIFT,
// C: 			ARRAY_SIZE(madera_out_anc_src_text),
// C: 			madera_out_anc_src_text),
// C: };
// C: EXPORT_SYMBOL_GPL(madera_output_anc_src);
// C: 
// C: int madera_dfc_put(struct snd_kcontrol *kcontrol,
// C: 		   struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
// C: 	struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
// C: 	struct soc_enum *e = (struct soc_enum *)kcontrol->private_value;
// C: 	unsigned int reg = e->reg;
// C: 	unsigned int val;
// C: 	int ret = 0;
// C: 
// C: 	reg = ((reg / 6) * 6) - 2;
// C: 
// C: 	snd_soc_dapm_mutex_lock(dapm);
// C: 
// C: 	val = snd_soc_component_read(component, reg);
// C: 	if (val & MADERA_DFC1_ENA) {
// C: 		ret = -EBUSY;
// C: 		dev_err(component->dev, "Can't change mode on an active DFC\n");
// C: 		goto exit;
// C: 	}
// C: 
// C: 	ret = snd_soc_put_enum_double(kcontrol, ucontrol);
// C: exit:
// C: 	snd_soc_dapm_mutex_unlock(dapm);
// C: 
// C: 	return ret;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_dfc_put);
// C: 
// C: int madera_lp_mode_put(struct snd_kcontrol *kcontrol,
// C: 		       struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct soc_mixer_control *mc =
// C: 		(struct soc_mixer_control *)kcontrol->private_value;
// C: 	struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
// C: 	struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
// C: 	unsigned int val, mask;
// C: 	int ret;
// C: 
// C: 	snd_soc_dapm_mutex_lock(dapm);
// C: 
// C: 	/* Cannot change lp mode on an active input */
// C: 	val = snd_soc_component_read(component, MADERA_INPUT_ENABLES);
// C: 	mask = (mc->reg - MADERA_ADC_DIGITAL_VOLUME_1L) / 4;
// C: 	mask ^= 0x1; /* Flip bottom bit for channel order */
// C: 
// C: 	if (val & (1 << mask)) {
// C: 		ret = -EBUSY;
// C: 		dev_err(component->dev,
// C: 			"Can't change lp mode on an active input\n");
// C: 		goto exit;
// C: 	}
// C: 
// C: 	ret = snd_soc_put_volsw(kcontrol, ucontrol);
// C: 
// C: exit:
// C: 	snd_soc_dapm_mutex_unlock(dapm);
// C: 
// C: 	return ret;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_lp_mode_put);
// C: 
// C: const struct snd_kcontrol_new madera_dsp_trigger_output_mux[] = {
// C: 	SOC_DAPM_SINGLE("Switch", SND_SOC_NOPM, 0, 1, 0),
// C: 	SOC_DAPM_SINGLE("Switch", SND_SOC_NOPM, 0, 1, 0),
// C: 	SOC_DAPM_SINGLE("Switch", SND_SOC_NOPM, 0, 1, 0),
// C: 	SOC_DAPM_SINGLE("Switch", SND_SOC_NOPM, 0, 1, 0),
// C: 	SOC_DAPM_SINGLE("Switch", SND_SOC_NOPM, 0, 1, 0),
// C: 	SOC_DAPM_SINGLE("Switch", SND_SOC_NOPM, 0, 1, 0),
// C: 	SOC_DAPM_SINGLE("Switch", SND_SOC_NOPM, 0, 1, 0),
// C: };
// C: EXPORT_SYMBOL_GPL(madera_dsp_trigger_output_mux);
// C: 
// C: const struct snd_kcontrol_new madera_drc_activity_output_mux[] = {
// C: 	SOC_DAPM_SINGLE("Switch", SND_SOC_NOPM, 0, 1, 0),
// C: 	SOC_DAPM_SINGLE("Switch", SND_SOC_NOPM, 0, 1, 0),
// C: };
// C: EXPORT_SYMBOL_GPL(madera_drc_activity_output_mux);
// C: 
// C: static void madera_in_set_vu(struct madera_priv *priv, bool enable)
// C: {
// C: 	unsigned int val;
// C: 	int i, ret;
// C: 
// C: 	if (enable)
// C: 		val = MADERA_IN_VU;
// C: 	else
// C: 		val = 0;
// C: 
// C: 	for (i = 0; i < priv->num_inputs; i++) {
// C: 		ret = regmap_update_bits(priv->madera->regmap,
// C: 					 MADERA_ADC_DIGITAL_VOLUME_1L + (i * 4),
// C: 					 MADERA_IN_VU, val);
// C: 		if (ret)
// C: 			dev_warn(priv->madera->dev,
// C: 				 "Failed to modify VU bits: %d\n", ret);
// C: 	}
// C: }
// C: 
// C: int madera_in_ev(struct snd_soc_dapm_widget *w, struct snd_kcontrol *kcontrol,
// C: 		 int event)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_to_component(w->dapm);
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	unsigned int reg, val;
// C: 
// C: 	if (w->shift % 2)
// C: 		reg = MADERA_ADC_DIGITAL_VOLUME_1L + ((w->shift / 2) * 8);
// C: 	else
// C: 		reg = MADERA_ADC_DIGITAL_VOLUME_1R + ((w->shift / 2) * 8);
// C: 
// C: 	switch (event) {
// C: 	case SND_SOC_DAPM_PRE_PMU:
// C: 		priv->in_pending++;
// C: 		break;
// C: 	case SND_SOC_DAPM_POST_PMU:
// C: 		priv->in_pending--;
// C: 		snd_soc_component_update_bits(component, reg,
// C: 					      MADERA_IN1L_MUTE, 0);
// C: 
// C: 		/* If this is the last input pending then allow VU */
// C: 		if (priv->in_pending == 0) {
// C: 			usleep_range(1000, 3000);
// C: 			madera_in_set_vu(priv, true);
// C: 		}
// C: 		break;
// C: 	case SND_SOC_DAPM_PRE_PMD:
// C: 		snd_soc_component_update_bits(component, reg,
// C: 					      MADERA_IN1L_MUTE | MADERA_IN_VU,
// C: 					      MADERA_IN1L_MUTE | MADERA_IN_VU);
// C: 		break;
// C: 	case SND_SOC_DAPM_POST_PMD:
// C: 		/* Disable volume updates if no inputs are enabled */
// C: 		val = snd_soc_component_read(component, MADERA_INPUT_ENABLES);
// C: 		if (!val)
// C: 			madera_in_set_vu(priv, false);
// C: 		break;
// C: 	default:
// C: 		break;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_in_ev);
// C: 
// C: int madera_out_ev(struct snd_soc_dapm_widget *w,
// C: 		  struct snd_kcontrol *kcontrol, int event)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_to_component(w->dapm);
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera *madera = priv->madera;
// C: 	int out_up_delay;
// C: 
// C: 	switch (madera->type) {
// C: 	case CS47L90:
// C: 	case CS47L91:
// C: 	case CS42L92:
// C: 	case CS47L92:
// C: 	case CS47L93:
// C: 		out_up_delay = 6000;
// C: 		break;
// C: 	default:
// C: 		out_up_delay = 17000;
// C: 		break;
// C: 	}
// C: 
// C: 	switch (event) {
// C: 	case SND_SOC_DAPM_PRE_PMU:
// C: 		switch (w->shift) {
// C: 		case MADERA_OUT1L_ENA_SHIFT:
// C: 		case MADERA_OUT1R_ENA_SHIFT:
// C: 		case MADERA_OUT2L_ENA_SHIFT:
// C: 		case MADERA_OUT2R_ENA_SHIFT:
// C: 		case MADERA_OUT3L_ENA_SHIFT:
// C: 		case MADERA_OUT3R_ENA_SHIFT:
// C: 			priv->out_up_pending++;
// C: 			priv->out_up_delay += out_up_delay;
// C: 			break;
// C: 		default:
// C: 			break;
// C: 		}
// C: 		break;
// C: 
// C: 	case SND_SOC_DAPM_POST_PMU:
// C: 		switch (w->shift) {
// C: 		case MADERA_OUT1L_ENA_SHIFT:
// C: 		case MADERA_OUT1R_ENA_SHIFT:
// C: 		case MADERA_OUT2L_ENA_SHIFT:
// C: 		case MADERA_OUT2R_ENA_SHIFT:
// C: 		case MADERA_OUT3L_ENA_SHIFT:
// C: 		case MADERA_OUT3R_ENA_SHIFT:
// C: 			priv->out_up_pending--;
// C: 			if (!priv->out_up_pending) {
// C: 				fsleep(priv->out_up_delay);
// C: 				priv->out_up_delay = 0;
// C: 			}
// C: 			break;
// C: 
// C: 		default:
// C: 			break;
// C: 		}
// C: 		break;
// C: 
// C: 	case SND_SOC_DAPM_PRE_PMD:
// C: 		switch (w->shift) {
// C: 		case MADERA_OUT1L_ENA_SHIFT:
// C: 		case MADERA_OUT1R_ENA_SHIFT:
// C: 		case MADERA_OUT2L_ENA_SHIFT:
// C: 		case MADERA_OUT2R_ENA_SHIFT:
// C: 		case MADERA_OUT3L_ENA_SHIFT:
// C: 		case MADERA_OUT3R_ENA_SHIFT:
// C: 			priv->out_down_pending++;
// C: 			priv->out_down_delay += 1000;
// C: 			break;
// C: 		default:
// C: 			break;
// C: 		}
// C: 		break;
// C: 
// C: 	case SND_SOC_DAPM_POST_PMD:
// C: 		switch (w->shift) {
// C: 		case MADERA_OUT1L_ENA_SHIFT:
// C: 		case MADERA_OUT1R_ENA_SHIFT:
// C: 		case MADERA_OUT2L_ENA_SHIFT:
// C: 		case MADERA_OUT2R_ENA_SHIFT:
// C: 		case MADERA_OUT3L_ENA_SHIFT:
// C: 		case MADERA_OUT3R_ENA_SHIFT:
// C: 			priv->out_down_pending--;
// C: 			if (!priv->out_down_pending) {
// C: 				fsleep(priv->out_down_delay);
// C: 				priv->out_down_delay = 0;
// C: 			}
// C: 			break;
// C: 		default:
// C: 			break;
// C: 		}
// C: 		break;
// C: 	default:
// C: 		break;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_out_ev);
// C: 
// C: int madera_hp_ev(struct snd_soc_dapm_widget *w,
// C: 		 struct snd_kcontrol *kcontrol, int event)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_to_component(w->dapm);
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera *madera = priv->madera;
// C: 	unsigned int mask = 1 << w->shift;
// C: 	unsigned int out_num = w->shift / 2;
// C: 	unsigned int val;
// C: 	unsigned int ep_sel = 0;
// C: 
// C: 	switch (event) {
// C: 	case SND_SOC_DAPM_POST_PMU:
// C: 		val = mask;
// C: 		break;
// C: 	case SND_SOC_DAPM_PRE_PMD:
// C: 		val = 0;
// C: 		break;
// C: 	case SND_SOC_DAPM_PRE_PMU:
// C: 	case SND_SOC_DAPM_POST_PMD:
// C: 		return madera_out_ev(w, kcontrol, event);
// C: 	default:
// C: 		return 0;
// C: 	}
// C: 
// C: 	/* Store the desired state for the HP outputs */
// C: 	madera->hp_ena &= ~mask;
// C: 	madera->hp_ena |= val;
// C: 
// C: 	switch (madera->type) {
// C: 	case CS42L92:
// C: 	case CS47L92:
// C: 	case CS47L93:
// C: 		break;
// C: 	default:
// C: 		/* if OUT1 is routed to EPOUT, ignore HP clamp and impedance */
// C: 		regmap_read(madera->regmap, MADERA_OUTPUT_ENABLES_1, &ep_sel);
// C: 		ep_sel &= MADERA_EP_SEL_MASK;
// C: 		break;
// C: 	}
// C: 
// C: 	/* Force off if HPDET has disabled the clamp for this output */
// C: 	if (!ep_sel &&
// C: 	    (!madera->out_clamp[out_num] || madera->out_shorted[out_num]))
// C: 		val = 0;
// C: 
// C: 	regmap_update_bits(madera->regmap, MADERA_OUTPUT_ENABLES_1, mask, val);
// C: 
// C: 	return madera_out_ev(w, kcontrol, event);
// C: }
// C: EXPORT_SYMBOL_GPL(madera_hp_ev);
// C: 
// C: int madera_anc_ev(struct snd_soc_dapm_widget *w, struct snd_kcontrol *kcontrol,
// C: 		  int event)
// C: {
// C: 	struct snd_soc_component *component = snd_soc_dapm_to_component(w->dapm);
// C: 	unsigned int val;
// C: 
// C: 	switch (event) {
// C: 	case SND_SOC_DAPM_POST_PMU:
// C: 		val = 1 << w->shift;
// C: 		break;
// C: 	case SND_SOC_DAPM_PRE_PMD:
// C: 		val = 1 << (w->shift + 1);
// C: 		break;
// C: 	default:
// C: 		return 0;
// C: 	}
// C: 
// C: 	snd_soc_component_write(component, MADERA_CLOCK_CONTROL, val);
// C: 
// C: 	return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_anc_ev);
// C: 
// C: static const unsigned int madera_opclk_ref_48k_rates[] = {
// C: 	6144000,
// C: 	12288000,
// C: 	24576000,
// C: 	49152000,
// C: };
// C: 
// C: static const unsigned int madera_opclk_ref_44k1_rates[] = {
// C: 	5644800,
// C: 	11289600,
// C: 	22579200,
// C: 	45158400,
// C: };
// C: 
// C: static int madera_set_opclk(struct snd_soc_component *component,
// C: 			    unsigned int clk, unsigned int freq)
// C: {
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	unsigned int mask = MADERA_OPCLK_DIV_MASK | MADERA_OPCLK_SEL_MASK;
// C: 	unsigned int reg, val;
// C: 	const unsigned int *rates;
// C: 	int ref, div, refclk;
// C: 
// C: 	BUILD_BUG_ON(ARRAY_SIZE(madera_opclk_ref_48k_rates) !=
// C: 		     ARRAY_SIZE(madera_opclk_ref_44k1_rates));
// C: 
// C: 	switch (clk) {
// C: 	case MADERA_CLK_OPCLK:
// C: 		reg = MADERA_OUTPUT_SYSTEM_CLOCK;
// C: 		refclk = priv->sysclk;
// C: 		break;
// C: 	case MADERA_CLK_ASYNC_OPCLK:
// C: 		reg = MADERA_OUTPUT_ASYNC_CLOCK;
// C: 		refclk = priv->asyncclk;
// C: 		break;
// C: 	default:
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	if (refclk % 4000)
// C: 		rates = madera_opclk_ref_44k1_rates;
// C: 	else
// C: 		rates = madera_opclk_ref_48k_rates;
// C: 
// C: 	for (ref = 0; ref < ARRAY_SIZE(madera_opclk_ref_48k_rates); ++ref) {
// C: 		if (rates[ref] > refclk)
// C: 			continue;
// C: 
// C: 		div = 2;
// C: 		while ((rates[ref] / div >= freq) && (div <= 30)) {
// C: 			if (rates[ref] / div == freq) {
// C: 				dev_dbg(component->dev, "Configured %dHz OPCLK\n",
// C: 					freq);
// C: 
// C: 				val = (div << MADERA_OPCLK_DIV_SHIFT) | ref;
// C: 
// C: 				snd_soc_component_update_bits(component, reg,
// C: 							      mask, val);
// C: 				return 0;
// C: 			}
// C: 			div += 2;
// C: 		}
// C: 	}
// C: 
// C: 	dev_err(component->dev, "Unable to generate %dHz OPCLK\n", freq);
// C: 
// C: 	return -EINVAL;
// C: }
// C: 
// C: static int madera_get_sysclk_setting(unsigned int freq)
// C: {
// C: 	switch (freq) {
// C: 	case 0:
// C: 	case 5644800:
// C: 	case 6144000:
// C: 		return 0;
// C: 	case 11289600:
// C: 	case 12288000:
// C: 		return MADERA_SYSCLK_12MHZ << MADERA_SYSCLK_FREQ_SHIFT;
// C: 	case 22579200:
// C: 	case 24576000:
// C: 		return MADERA_SYSCLK_24MHZ << MADERA_SYSCLK_FREQ_SHIFT;
// C: 	case 45158400:
// C: 	case 49152000:
// C: 		return MADERA_SYSCLK_49MHZ << MADERA_SYSCLK_FREQ_SHIFT;
// C: 	case 90316800:
// C: 	case 98304000:
// C: 		return MADERA_SYSCLK_98MHZ << MADERA_SYSCLK_FREQ_SHIFT;
// C: 	default:
// C: 		return -EINVAL;
// C: 	}
// C: }
// C: 
// C: static int madera_get_legacy_dspclk_setting(struct madera *madera,
// C: 					    unsigned int freq)
// C: {
// C: 	switch (freq) {
// C: 	case 0:
// C: 		return 0;
// C: 	case 45158400:
// C: 	case 49152000:
// C: 		switch (madera->type) {
// C: 		case CS47L85:
// C: 		case WM1840:
// C: 			if (madera->rev < 3)
// C: 				return -EINVAL;
// C: 			else
// C: 				return MADERA_SYSCLK_49MHZ <<
// C: 				       MADERA_SYSCLK_FREQ_SHIFT;
// C: 		default:
// C: 			return -EINVAL;
// C: 		}
// C: 	case 135475200:
// C: 	case 147456000:
// C: 		return MADERA_DSPCLK_147MHZ << MADERA_DSP_CLK_FREQ_LEGACY_SHIFT;
// C: 	default:
// C: 		return -EINVAL;
// C: 	}
// C: }
// C: 
// C: static int madera_get_dspclk_setting(struct madera *madera,
// C: 				     unsigned int freq,
// C: 				     unsigned int *clock_2_val)
// C: {
// C: 	switch (madera->type) {
// C: 	case CS47L35:
// C: 	case CS47L85:
// C: 	case WM1840:
// C: 		*clock_2_val = 0; /* don't use MADERA_DSP_CLOCK_2 */
// C: 		return madera_get_legacy_dspclk_setting(madera, freq);
// C: 	default:
// C: 		if (freq > 150000000)
// C: 			return -EINVAL;
// C: 
// C: 		/* Use new exact frequency control */
// C: 		*clock_2_val = freq / 15625; /* freq * (2^6) / (10^6) */
// C: 		return 0;
// C: 	}
// C: }
// C: 
// C: static int madera_set_outclk(struct snd_soc_component *component,
// C: 			     unsigned int source, unsigned int freq)
// C: {
// C: 	int div, div_inc, rate;
// C: 
// C: 	switch (source) {
// C: 	case MADERA_OUTCLK_SYSCLK:
// C: 		dev_dbg(component->dev, "Configured OUTCLK to SYSCLK\n");
// C: 		snd_soc_component_update_bits(component, MADERA_OUTPUT_RATE_1,
// C: 					      MADERA_OUT_CLK_SRC_MASK, source);
// C: 		return 0;
// C: 	case MADERA_OUTCLK_ASYNCCLK:
// C: 		dev_dbg(component->dev, "Configured OUTCLK to ASYNCCLK\n");
// C: 		snd_soc_component_update_bits(component, MADERA_OUTPUT_RATE_1,
// C: 					      MADERA_OUT_CLK_SRC_MASK, source);
// C: 		return 0;
// C: 	case MADERA_OUTCLK_MCLK1:
// C: 	case MADERA_OUTCLK_MCLK2:
// C: 	case MADERA_OUTCLK_MCLK3:
// C: 		break;
// C: 	default:
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	if (freq % 4000)
// C: 		rate = 5644800;
// C: 	else
// C: 		rate = 6144000;
// C: 
// C: 	div = 1;
// C: 	div_inc = 0;
// C: 	while (div <= 8) {
// C: 		if (freq / div == rate && !(freq % div)) {
// C: 			dev_dbg(component->dev, "Configured %dHz OUTCLK\n", rate);
// C: 			snd_soc_component_update_bits(component,
// C: 				MADERA_OUTPUT_RATE_1,
// C: 				MADERA_OUT_EXT_CLK_DIV_MASK |
// C: 				MADERA_OUT_CLK_SRC_MASK,
// C: 				(div_inc << MADERA_OUT_EXT_CLK_DIV_SHIFT) |
// C: 				source);
// C: 			return 0;
// C: 		}
// C: 		div_inc++;
// C: 		div *= 2;
// C: 	}
// C: 
// C: 	dev_err(component->dev,
// C: 		"Unable to generate %dHz OUTCLK from %dHz MCLK\n",
// C: 		rate, freq);
// C: 	return -EINVAL;
// C: }
// C: 
// C: int madera_set_sysclk(struct snd_soc_component *component, int clk_id,
// C: 		      int source, unsigned int freq, int dir)
// C: {
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera *madera = priv->madera;
// C: 	char *name;
// C: 	unsigned int reg, clock_2_val = 0;
// C: 	unsigned int mask = MADERA_SYSCLK_FREQ_MASK | MADERA_SYSCLK_SRC_MASK;
// C: 	unsigned int val = source << MADERA_SYSCLK_SRC_SHIFT;
// C: 	int clk_freq_sel, *clk;
// C: 	int ret = 0;
// C: 
// C: 	switch (clk_id) {
// C: 	case MADERA_CLK_SYSCLK_1:
// C: 		name = "SYSCLK";
// C: 		reg = MADERA_SYSTEM_CLOCK_1;
// C: 		clk = &priv->sysclk;
// C: 		clk_freq_sel = madera_get_sysclk_setting(freq);
// C: 		mask |= MADERA_SYSCLK_FRAC;
// C: 		break;
// C: 	case MADERA_CLK_ASYNCCLK_1:
// C: 		name = "ASYNCCLK";
// C: 		reg = MADERA_ASYNC_CLOCK_1;
// C: 		clk = &priv->asyncclk;
// C: 		clk_freq_sel = madera_get_sysclk_setting(freq);
// C: 		break;
// C: 	case MADERA_CLK_DSPCLK:
// C: 		name = "DSPCLK";
// C: 		reg = MADERA_DSP_CLOCK_1;
// C: 		clk = &priv->dspclk;
// C: 		clk_freq_sel = madera_get_dspclk_setting(madera, freq,
// C: 							 &clock_2_val);
// C: 		break;
// C: 	case MADERA_CLK_OPCLK:
// C: 	case MADERA_CLK_ASYNC_OPCLK:
// C: 		return madera_set_opclk(component, clk_id, freq);
// C: 	case MADERA_CLK_OUTCLK:
// C: 		return madera_set_outclk(component, source, freq);
// C: 	default:
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	if (clk_freq_sel < 0) {
// C: 		dev_err(madera->dev,
// C: 			"Failed to get clk setting for %dHZ\n", freq);
// C: 		return clk_freq_sel;
// C: 	}
// C: 
// C: 	*clk = freq;
// C: 
// C: 	if (freq == 0) {
// C: 		dev_dbg(madera->dev, "%s cleared\n", name);
// C: 		return 0;
// C: 	}
// C: 
// C: 	val |= clk_freq_sel;
// C: 
// C: 	if (clock_2_val) {
// C: 		ret = regmap_write(madera->regmap, MADERA_DSP_CLOCK_2,
// C: 				   clock_2_val);
// C: 		if (ret) {
// C: 			dev_err(madera->dev,
// C: 				"Failed to write DSP_CONFIG2: %d\n", ret);
// C: 			return ret;
// C: 		}
// C: 
// C: 		/*
// C: 		 * We're using the frequency setting in MADERA_DSP_CLOCK_2 so
// C: 		 * don't change the frequency select bits in MADERA_DSP_CLOCK_1
// C: 		 */
// C: 		mask = MADERA_SYSCLK_SRC_MASK;
// C: 	}
// C: 
// C: 	if (freq % 6144000)
// C: 		val |= MADERA_SYSCLK_FRAC;
// C: 
// C: 	dev_dbg(madera->dev, "%s set to %uHz\n", name, freq);
// C: 
// C: 	return regmap_update_bits(madera->regmap, reg, mask, val);
// C: }
// C: EXPORT_SYMBOL_GPL(madera_set_sysclk);
// C: 
// C: static int madera_set_fmt(struct snd_soc_dai *dai, unsigned int fmt)
// C: {
// C: 	struct snd_soc_component *component = dai->component;
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera *madera = priv->madera;
// C: 	int lrclk, bclk, mode, base;
// C: 
// C: 	base = dai->driver->base;
// C: 
// C: 	lrclk = 0;
// C: 	bclk = 0;
// C: 
// C: 	switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
// C: 	case SND_SOC_DAIFMT_DSP_A:
// C: 		mode = MADERA_FMT_DSP_MODE_A;
// C: 		break;
// C: 	case SND_SOC_DAIFMT_DSP_B:
// C: 		if ((fmt & SND_SOC_DAIFMT_MASTER_MASK) !=
// C: 		    SND_SOC_DAIFMT_CBP_CFP) {
// C: 			madera_aif_err(dai, "DSP_B not valid in slave mode\n");
// C: 			return -EINVAL;
// C: 		}
// C: 		mode = MADERA_FMT_DSP_MODE_B;
// C: 		break;
// C: 	case SND_SOC_DAIFMT_I2S:
// C: 		mode = MADERA_FMT_I2S_MODE;
// C: 		break;
// C: 	case SND_SOC_DAIFMT_LEFT_J:
// C: 		if ((fmt & SND_SOC_DAIFMT_MASTER_MASK) !=
// C: 		    SND_SOC_DAIFMT_CBP_CFP) {
// C: 			madera_aif_err(dai, "LEFT_J not valid in slave mode\n");
// C: 			return -EINVAL;
// C: 		}
// C: 		mode = MADERA_FMT_LEFT_JUSTIFIED_MODE;
// C: 		break;
// C: 	default:
// C: 		madera_aif_err(dai, "Unsupported DAI format %d\n",
// C: 			       fmt & SND_SOC_DAIFMT_FORMAT_MASK);
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	switch (fmt & SND_SOC_DAIFMT_MASTER_MASK) {
// C: 	case SND_SOC_DAIFMT_CBC_CFC:
// C: 		break;
// C: 	case SND_SOC_DAIFMT_CBC_CFP:
// C: 		lrclk |= MADERA_AIF1TX_LRCLK_MSTR;
// C: 		break;
// C: 	case SND_SOC_DAIFMT_CBP_CFC:
// C: 		bclk |= MADERA_AIF1_BCLK_MSTR;
// C: 		break;
// C: 	case SND_SOC_DAIFMT_CBP_CFP:
// C: 		bclk |= MADERA_AIF1_BCLK_MSTR;
// C: 		lrclk |= MADERA_AIF1TX_LRCLK_MSTR;
// C: 		break;
// C: 	default:
// C: 		madera_aif_err(dai, "Unsupported master mode %d\n",
// C: 			       fmt & SND_SOC_DAIFMT_MASTER_MASK);
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	switch (fmt & SND_SOC_DAIFMT_INV_MASK) {
// C: 	case SND_SOC_DAIFMT_NB_NF:
// C: 		break;
// C: 	case SND_SOC_DAIFMT_IB_IF:
// C: 		bclk |= MADERA_AIF1_BCLK_INV;
// C: 		lrclk |= MADERA_AIF1TX_LRCLK_INV;
// C: 		break;
// C: 	case SND_SOC_DAIFMT_IB_NF:
// C: 		bclk |= MADERA_AIF1_BCLK_INV;
// C: 		break;
// C: 	case SND_SOC_DAIFMT_NB_IF:
// C: 		lrclk |= MADERA_AIF1TX_LRCLK_INV;
// C: 		break;
// C: 	default:
// C: 		madera_aif_err(dai, "Unsupported invert mode %d\n",
// C: 			       fmt & SND_SOC_DAIFMT_INV_MASK);
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	regmap_update_bits(madera->regmap, base + MADERA_AIF_BCLK_CTRL,
// C: 			   MADERA_AIF1_BCLK_INV | MADERA_AIF1_BCLK_MSTR,
// C: 			   bclk);
// C: 	regmap_update_bits(madera->regmap, base + MADERA_AIF_TX_PIN_CTRL,
// C: 			   MADERA_AIF1TX_LRCLK_INV | MADERA_AIF1TX_LRCLK_MSTR,
// C: 			   lrclk);
// C: 	regmap_update_bits(madera->regmap, base + MADERA_AIF_RX_PIN_CTRL,
// C: 			   MADERA_AIF1RX_LRCLK_INV | MADERA_AIF1RX_LRCLK_MSTR,
// C: 			   lrclk);
// C: 	regmap_update_bits(madera->regmap, base + MADERA_AIF_FORMAT,
// C: 			   MADERA_AIF1_FMT_MASK, mode);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static const int madera_48k_bclk_rates[] = {
// C: 	-1,
// C: 	48000,
// C: 	64000,
// C: 	96000,
// C: 	128000,
// C: 	192000,
// C: 	256000,
// C: 	384000,
// C: 	512000,
// C: 	768000,
// C: 	1024000,
// C: 	1536000,
// C: 	2048000,
// C: 	3072000,
// C: 	4096000,
// C: 	6144000,
// C: 	8192000,
// C: 	12288000,
// C: 	24576000,
// C: };
// C: 
// C: static const int madera_44k1_bclk_rates[] = {
// C: 	-1,
// C: 	44100,
// C: 	58800,
// C: 	88200,
// C: 	117600,
// C: 	177640,
// C: 	235200,
// C: 	352800,
// C: 	470400,
// C: 	705600,
// C: 	940800,
// C: 	1411200,
// C: 	1881600,
// C: 	2822400,
// C: 	3763200,
// C: 	5644800,
// C: 	7526400,
// C: 	11289600,
// C: 	22579200,
// C: };
// C: 
// C: static const unsigned int madera_sr_vals[] = {
// C: 	0,
// C: 	12000,
// C: 	24000,
// C: 	48000,
// C: 	96000,
// C: 	192000,
// C: 	384000,
// C: 	768000,
// C: 	0,
// C: 	11025,
// C: 	22050,
// C: 	44100,
// C: 	88200,
// C: 	176400,
// C: 	352800,
// C: 	705600,
// C: 	4000,
// C: 	8000,
// C: 	16000,
// C: 	32000,
// C: 	64000,
// C: 	128000,
// C: 	256000,
// C: 	512000,
// C: };
// C: 
pub const MADERA_192K_48K_RATE_MASK: c_int = 0x0F003E;
pub const MADERA_192K_44K1_RATE_MASK: c_int = 0x003E00;
pub const MADERA_192K_RATE_MASK: c_int = (MADERA_192K_48K_RATE_MASK | ;
// C: 					 MADERA_192K_44K1_RATE_MASK)
pub const MADERA_384K_48K_RATE_MASK: c_int = 0x0F007E;
pub const MADERA_384K_44K1_RATE_MASK: c_int = 0x007E00;
pub const MADERA_384K_RATE_MASK: c_int = (MADERA_384K_48K_RATE_MASK | ;
// C: 					 MADERA_384K_44K1_RATE_MASK)
// C: 
// C: static const struct snd_pcm_hw_constraint_list madera_constraint = {
// C: 	.count	= ARRAY_SIZE(madera_sr_vals),
// C: 	.list	= madera_sr_vals,
// C: };
// C: 
// C: static int madera_startup(struct snd_pcm_substream *substream,
// C: 			  struct snd_soc_dai *dai)
// C: {
// C: 	struct snd_soc_component *component = dai->component;
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera_dai_priv *dai_priv = &priv->dai[dai->id - 1];
// C: 	struct madera *madera = priv->madera;
// C: 	unsigned int base_rate;
// C: 
// C: 	if (!substream->runtime)
// C: 		return 0;
// C: 
// C: 	switch (dai_priv->clk) {
// C: 	case MADERA_CLK_SYSCLK_1:
// C: 	case MADERA_CLK_SYSCLK_2:
// C: 	case MADERA_CLK_SYSCLK_3:
// C: 		base_rate = priv->sysclk;
// C: 		break;
// C: 	case MADERA_CLK_ASYNCCLK_1:
// C: 	case MADERA_CLK_ASYNCCLK_2:
// C: 		base_rate = priv->asyncclk;
// C: 		break;
// C: 	default:
// C: 		return 0;
// C: 	}
// C: 
// C: 	switch (madera->type) {
// C: 	case CS42L92:
// C: 	case CS47L92:
// C: 	case CS47L93:
// C: 		if (base_rate == 0)
// C: 			dai_priv->constraint.mask = MADERA_384K_RATE_MASK;
// C: 		else if (base_rate % 4000)
// C: 			dai_priv->constraint.mask = MADERA_384K_44K1_RATE_MASK;
// C: 		else
// C: 			dai_priv->constraint.mask = MADERA_384K_48K_RATE_MASK;
// C: 		break;
// C: 	default:
// C: 		if (base_rate == 0)
// C: 			dai_priv->constraint.mask = MADERA_192K_RATE_MASK;
// C: 		else if (base_rate % 4000)
// C: 			dai_priv->constraint.mask = MADERA_192K_44K1_RATE_MASK;
// C: 		else
// C: 			dai_priv->constraint.mask = MADERA_192K_48K_RATE_MASK;
// C: 		break;
// C: 	}
// C: 
// C: 	return snd_pcm_hw_constraint_list(substream->runtime, 0,
// C: 					  SNDRV_PCM_HW_PARAM_RATE,
// C: 					  &dai_priv->constraint);
// C: }
// C: 
// C: static int madera_hw_params_rate(struct snd_pcm_substream *substream,
// C: 				 struct snd_pcm_hw_params *params,
// C: 				 struct snd_soc_dai *dai)
// C: {
// C: 	struct snd_soc_component *component = dai->component;
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera_dai_priv *dai_priv = &priv->dai[dai->id - 1];
// C: 	int base = dai->driver->base;
// C: 	int i, sr_val;
// C: 	unsigned int reg, cur, tar;
// C: 	int ret;
// C: 
// C: 	for (i = 0; i < ARRAY_SIZE(madera_sr_vals); i++)
// C: 		if (madera_sr_vals[i] == params_rate(params))
// C: 			break;
// C: 
// C: 	if (i == ARRAY_SIZE(madera_sr_vals)) {
// C: 		madera_aif_err(dai, "Unsupported sample rate %dHz\n",
// C: 			       params_rate(params));
// C: 		return -EINVAL;
// C: 	}
// C: 	sr_val = i;
// C: 
// C: 	switch (dai_priv->clk) {
// C: 	case MADERA_CLK_SYSCLK_1:
// C: 		reg = MADERA_SAMPLE_RATE_1;
// C: 		tar = 0 << MADERA_AIF1_RATE_SHIFT;
// C: 		break;
// C: 	case MADERA_CLK_SYSCLK_2:
// C: 		reg = MADERA_SAMPLE_RATE_2;
// C: 		tar = 1 << MADERA_AIF1_RATE_SHIFT;
// C: 		break;
// C: 	case MADERA_CLK_SYSCLK_3:
// C: 		reg = MADERA_SAMPLE_RATE_3;
// C: 		tar = 2 << MADERA_AIF1_RATE_SHIFT;
// C: 		break;
// C: 	case MADERA_CLK_ASYNCCLK_1:
// C: 		reg = MADERA_ASYNC_SAMPLE_RATE_1;
// C: 		tar = 8 << MADERA_AIF1_RATE_SHIFT;
// C: 		break;
// C: 	case MADERA_CLK_ASYNCCLK_2:
// C: 		reg = MADERA_ASYNC_SAMPLE_RATE_2;
// C: 		tar = 9 << MADERA_AIF1_RATE_SHIFT;
// C: 		break;
// C: 	default:
// C: 		madera_aif_err(dai, "Invalid clock %d\n", dai_priv->clk);
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	snd_soc_component_update_bits(component, reg, MADERA_SAMPLE_RATE_1_MASK,
// C: 				      sr_val);
// C: 
// C: 	if (!base)
// C: 		return 0;
// C: 
// C: 	ret = regmap_read(priv->madera->regmap,
// C: 			  base + MADERA_AIF_RATE_CTRL, &cur);
// C: 	if (ret != 0) {
// C: 		madera_aif_err(dai, "Failed to check rate: %d\n", ret);
// C: 		return ret;
// C: 	}
// C: 
// C: 	if ((cur & MADERA_AIF1_RATE_MASK) == (tar & MADERA_AIF1_RATE_MASK))
// C: 		return 0;
// C: 
// C: 	guard(mutex)(&priv->rate_lock);
// C: 
// C: 	if (!madera_can_change_grp_rate(priv, base + MADERA_AIF_RATE_CTRL)) {
// C: 		madera_aif_warn(dai, "Cannot change rate while active\n");
// C: 		return -EBUSY;
// C: 	}
// C: 
// C: 	/* Guard the rate change with SYSCLK cycles */
// C: 	madera_spin_sysclk(priv);
// C: 	snd_soc_component_update_bits(component, base + MADERA_AIF_RATE_CTRL,
// C: 				      MADERA_AIF1_RATE_MASK, tar);
// C: 	madera_spin_sysclk(priv);
// C: 
// C: 	return ret;
// C: }
// C: 
// C: static int madera_aif_cfg_changed(struct snd_soc_component *component,
// C: 				  int base, int bclk, int lrclk, int frame)
// C: {
// C: 	unsigned int val;
// C: 
// C: 	val = snd_soc_component_read(component, base + MADERA_AIF_BCLK_CTRL);
// C: 	if (bclk != (val & MADERA_AIF1_BCLK_FREQ_MASK))
// C: 		return 1;
// C: 
// C: 	val = snd_soc_component_read(component, base + MADERA_AIF_RX_BCLK_RATE);
// C: 	if (lrclk != (val & MADERA_AIF1RX_BCPF_MASK))
// C: 		return 1;
// C: 
// C: 	val = snd_soc_component_read(component, base + MADERA_AIF_FRAME_CTRL_1);
// C: 	if (frame != (val & (MADERA_AIF1TX_WL_MASK |
// C: 			     MADERA_AIF1TX_SLOT_LEN_MASK)))
// C: 		return 1;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int madera_hw_params(struct snd_pcm_substream *substream,
// C: 			    struct snd_pcm_hw_params *params,
// C: 			    struct snd_soc_dai *dai)
// C: {
// C: 	struct snd_soc_component *component = dai->component;
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera *madera = priv->madera;
// C: 	int base = dai->driver->base;
// C: 	const int *rates;
// C: 	int i, ret;
// C: 	unsigned int val;
// C: 	unsigned int channels = params_channels(params);
// C: 	unsigned int rate = params_rate(params);
// C: 	unsigned int chan_limit =
// C: 			madera->pdata.codec.max_channels_clocked[dai->id - 1];
// C: 	int tdm_width = priv->tdm_width[dai->id - 1];
// C: 	int tdm_slots = priv->tdm_slots[dai->id - 1];
// C: 	int bclk, lrclk, wl, frame, bclk_target, num_rates;
// C: 	int reconfig;
// C: 	unsigned int aif_tx_state = 0, aif_rx_state = 0;
// C: 
// C: 	if (rate % 4000) {
// C: 		rates = &madera_44k1_bclk_rates[0];
// C: 		num_rates = ARRAY_SIZE(madera_44k1_bclk_rates);
// C: 	} else {
// C: 		rates = &madera_48k_bclk_rates[0];
// C: 		num_rates = ARRAY_SIZE(madera_48k_bclk_rates);
// C: 	}
// C: 
// C: 	wl = snd_pcm_format_width(params_format(params));
// C: 
// C: 	if (tdm_slots) {
// C: 		madera_aif_dbg(dai, "Configuring for %d %d bit TDM slots\n",
// C: 			       tdm_slots, tdm_width);
// C: 		bclk_target = tdm_slots * tdm_width * rate;
// C: 		channels = tdm_slots;
// C: 	} else {
// C: 		bclk_target = snd_soc_params_to_bclk(params);
// C: 		tdm_width = wl;
// C: 	}
// C: 
// C: 	if (chan_limit && chan_limit < channels) {
// C: 		madera_aif_dbg(dai, "Limiting to %d channels\n", chan_limit);
// C: 		bclk_target /= channels;
// C: 		bclk_target *= chan_limit;
// C: 	}
// C: 
// C: 	/* Force multiple of 2 channels for I2S mode */
// C: 	val = snd_soc_component_read(component, base + MADERA_AIF_FORMAT);
// C: 	val &= MADERA_AIF1_FMT_MASK;
// C: 	if ((channels & 1) && val == MADERA_FMT_I2S_MODE) {
// C: 		madera_aif_dbg(dai, "Forcing stereo mode\n");
// C: 		bclk_target /= channels;
// C: 		bclk_target *= channels + 1;
// C: 	}
// C: 
// C: 	for (i = 0; i < num_rates; i++) {
// C: 		if (rates[i] >= bclk_target && rates[i] % rate == 0) {
// C: 			bclk = i;
// C: 			break;
// C: 		}
// C: 	}
// C: 
// C: 	if (i == num_rates) {
// C: 		madera_aif_err(dai, "Unsupported sample rate %dHz\n", rate);
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	lrclk = rates[bclk] / rate;
// C: 
// C: 	madera_aif_dbg(dai, "BCLK %dHz LRCLK %dHz\n",
// C: 		       rates[bclk], rates[bclk] / lrclk);
// C: 
// C: 	frame = wl << MADERA_AIF1TX_WL_SHIFT | tdm_width;
// C: 
// C: 	reconfig = madera_aif_cfg_changed(component, base, bclk, lrclk, frame);
// C: 	if (reconfig < 0)
// C: 		return reconfig;
// C: 
// C: 	if (reconfig) {
// C: 		/* Save AIF TX/RX state */
// C: 		regmap_read(madera->regmap, base + MADERA_AIF_TX_ENABLES,
// C: 			    &aif_tx_state);
// C: 		regmap_read(madera->regmap, base + MADERA_AIF_RX_ENABLES,
// C: 			    &aif_rx_state);
// C: 		/* Disable AIF TX/RX before reconfiguring it */
// C: 		regmap_update_bits(madera->regmap,
// C: 				   base + MADERA_AIF_TX_ENABLES, 0xff, 0x0);
// C: 		regmap_update_bits(madera->regmap,
// C: 				   base + MADERA_AIF_RX_ENABLES, 0xff, 0x0);
// C: 	}
// C: 
// C: 	ret = madera_hw_params_rate(substream, params, dai);
// C: 	if (ret != 0)
// C: 		goto restore_aif;
// C: 
// C: 	if (reconfig) {
// C: 		regmap_update_bits(madera->regmap,
// C: 				   base + MADERA_AIF_BCLK_CTRL,
// C: 				   MADERA_AIF1_BCLK_FREQ_MASK, bclk);
// C: 		regmap_update_bits(madera->regmap,
// C: 				   base + MADERA_AIF_RX_BCLK_RATE,
// C: 				   MADERA_AIF1RX_BCPF_MASK, lrclk);
// C: 		regmap_update_bits(madera->regmap,
// C: 				   base + MADERA_AIF_FRAME_CTRL_1,
// C: 				   MADERA_AIF1TX_WL_MASK |
// C: 				   MADERA_AIF1TX_SLOT_LEN_MASK, frame);
// C: 		regmap_update_bits(madera->regmap,
// C: 				   base + MADERA_AIF_FRAME_CTRL_2,
// C: 				   MADERA_AIF1RX_WL_MASK |
// C: 				   MADERA_AIF1RX_SLOT_LEN_MASK, frame);
// C: 	}
// C: 
// C: restore_aif:
// C: 	if (reconfig) {
// C: 		/* Restore AIF TX/RX state */
// C: 		regmap_update_bits(madera->regmap,
// C: 				   base + MADERA_AIF_TX_ENABLES,
// C: 				   0xff, aif_tx_state);
// C: 		regmap_update_bits(madera->regmap,
// C: 				   base + MADERA_AIF_RX_ENABLES,
// C: 				   0xff, aif_rx_state);
// C: 	}
// C: 
// C: 	return ret;
// C: }
// C: 
// C: static int madera_is_syncclk(int clk_id)
// C: {
// C: 	switch (clk_id) {
// C: 	case MADERA_CLK_SYSCLK_1:
// C: 	case MADERA_CLK_SYSCLK_2:
// C: 	case MADERA_CLK_SYSCLK_3:
// C: 		return 1;
// C: 	case MADERA_CLK_ASYNCCLK_1:
// C: 	case MADERA_CLK_ASYNCCLK_2:
// C: 		return 0;
// C: 	default:
// C: 		return -EINVAL;
// C: 	}
// C: }
// C: 
// C: static int madera_dai_set_sysclk(struct snd_soc_dai *dai,
// C: 				 int clk_id, unsigned int freq, int dir)
// C: {
// C: 	struct snd_soc_component *component = dai->component;
// C: 	struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera_dai_priv *dai_priv = &priv->dai[dai->id - 1];
// C: 	struct snd_soc_dapm_route routes[2];
// C: 	int is_sync;
// C: 
// C: 	is_sync = madera_is_syncclk(clk_id);
// C: 	if (is_sync < 0) {
// C: 		dev_err(component->dev, "Illegal DAI clock id %d\n", clk_id);
// C: 		return is_sync;
// C: 	}
// C: 
// C: 	if (is_sync == madera_is_syncclk(dai_priv->clk))
// C: 		return 0;
// C: 
// C: 	if (snd_soc_dai_active(dai)) {
// C: 		dev_err(component->dev, "Can't change clock on active DAI %d\n",
// C: 			dai->id);
// C: 		return -EBUSY;
// C: 	}
// C: 
// C: 	dev_dbg(component->dev, "Setting AIF%d to %s\n", dai->id,
// C: 		is_sync ? "SYSCLK" : "ASYNCCLK");
// C: 
// C: 	/*
// C: 	 * A connection to SYSCLK is always required, we only add and remove
// C: 	 * a connection to ASYNCCLK
// C: 	 */
// C: 	memset(&routes, 0, sizeof(routes));
// C: 	routes[0].sink = dai->driver->capture.stream_name;
// C: 	routes[1].sink = dai->driver->playback.stream_name;
// C: 	routes[0].source = "ASYNCCLK";
// C: 	routes[1].source = "ASYNCCLK";
// C: 
// C: 	if (is_sync)
// C: 		snd_soc_dapm_del_routes(dapm, routes, ARRAY_SIZE(routes));
// C: 	else
// C: 		snd_soc_dapm_add_routes(dapm, routes, ARRAY_SIZE(routes));
// C: 
// C: 	dai_priv->clk = clk_id;
// C: 
// C: 	return snd_soc_dapm_sync(dapm);
// C: }
// C: 
// C: static int madera_set_tristate(struct snd_soc_dai *dai, int tristate)
// C: {
// C: 	struct snd_soc_component *component = dai->component;
// C: 	int base = dai->driver->base;
// C: 	unsigned int reg;
// C: 	int ret;
// C: 
// C: 	if (tristate)
// C: 		reg = MADERA_AIF1_TRI;
// C: 	else
// C: 		reg = 0;
// C: 
// C: 	ret = snd_soc_component_update_bits(component,
// C: 					    base + MADERA_AIF_RATE_CTRL,
// C: 					    MADERA_AIF1_TRI, reg);
// C: 	if (ret < 0)
// C: 		return ret;
// C: 	else
// C: 		return 0;
// C: }
// C: 
// C: static void madera_set_channels_to_mask(struct snd_soc_dai *dai,
// C: 					unsigned int base,
// C: 					int channels, unsigned int mask)
// C: {
// C: 	struct snd_soc_component *component = dai->component;
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera *madera = priv->madera;
// C: 	int slot, i;
// C: 
// C: 	for (i = 0; i < channels; ++i) {
// C: 		slot = ffs(mask) - 1;
// C: 		if (slot < 0)
// C: 			return;
// C: 
// C: 		regmap_write(madera->regmap, base + i, slot);
// C: 
// C: 		mask &= ~(1 << slot);
// C: 	}
// C: 
// C: 	if (mask)
// C: 		madera_aif_warn(dai, "Too many channels in TDM mask\n");
// C: }
// C: 
// C: static int madera_set_tdm_slot(struct snd_soc_dai *dai, unsigned int tx_mask,
// C: 			       unsigned int rx_mask, int slots, int slot_width)
// C: {
// C: 	struct snd_soc_component *component = dai->component;
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	int base = dai->driver->base;
// C: 	int rx_max_chan = dai->driver->playback.channels_max;
// C: 	int tx_max_chan = dai->driver->capture.channels_max;
// C: 
// C: 	/* Only support TDM for the physical AIFs */
// C: 	if (dai->id > MADERA_MAX_AIF)
// C: 		return -ENOTSUPP;
// C: 
// C: 	if (slots == 0) {
// C: 		tx_mask = (1 << tx_max_chan) - 1;
// C: 		rx_mask = (1 << rx_max_chan) - 1;
// C: 	}
// C: 
// C: 	madera_set_channels_to_mask(dai, base + MADERA_AIF_FRAME_CTRL_3,
// C: 				    tx_max_chan, tx_mask);
// C: 	madera_set_channels_to_mask(dai, base + MADERA_AIF_FRAME_CTRL_11,
// C: 				    rx_max_chan, rx_mask);
// C: 
// C: 	priv->tdm_width[dai->id - 1] = slot_width;
// C: 	priv->tdm_slots[dai->id - 1] = slots;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static const u64 madera_selectable_formats =
// C: 	SND_SOC_POSSIBLE_DAIFMT_I2S	|
// C: 	SND_SOC_POSSIBLE_DAIFMT_LEFT_J	|
// C: 	SND_SOC_POSSIBLE_DAIFMT_DSP_A	|
// C: 	SND_SOC_POSSIBLE_DAIFMT_DSP_B	|
// C: 	SND_SOC_POSSIBLE_DAIFMT_NB_NF	|
// C: 	SND_SOC_POSSIBLE_DAIFMT_NB_IF	|
// C: 	SND_SOC_POSSIBLE_DAIFMT_IB_NF	|
// C: 	SND_SOC_POSSIBLE_DAIFMT_IB_IF;
// C: 
// C: const struct snd_soc_dai_ops madera_dai_ops = {
// C: 	.startup = &madera_startup,
// C: 	.set_fmt = &madera_set_fmt,
// C: 	.set_tdm_slot = &madera_set_tdm_slot,
// C: 	.hw_params = &madera_hw_params,
// C: 	.set_sysclk = &madera_dai_set_sysclk,
// C: 	.set_tristate = &madera_set_tristate,
// C: 	.auto_selectable_formats = &madera_selectable_formats,
// C: 	.num_auto_selectable_formats = 1,
// C: };
// C: EXPORT_SYMBOL_GPL(madera_dai_ops);
// C: 
// C: const struct snd_soc_dai_ops madera_simple_dai_ops = {
// C: 	.startup = &madera_startup,
// C: 	.hw_params = &madera_hw_params_rate,
// C: 	.set_sysclk = &madera_dai_set_sysclk,
// C: };
// C: EXPORT_SYMBOL_GPL(madera_simple_dai_ops);
// C: 
// C: int madera_init_dai(struct madera_priv *priv, int id)
// C: {
// C: 	struct madera_dai_priv *dai_priv = &priv->dai[id];
// C: 
// C: 	dai_priv->clk = MADERA_CLK_SYSCLK_1;
// C: 	dai_priv->constraint = madera_constraint;
// C: 
// C: 	return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_init_dai);
// C: 
// C: static const struct {
// C: 	unsigned int min;
// C: 	unsigned int max;
// C: 	u16 fratio;
// C: 	int ratio;
// C: } fll_sync_fratios[] = {
// C: 	{       0,    64000, 4, 16 },
// C: 	{   64000,   128000, 3,  8 },
// C: 	{  128000,   256000, 2,  4 },
// C: 	{  256000,  1000000, 1,  2 },
// C: 	{ 1000000, 13500000, 0,  1 },
// C: };
// C: 
// C: static const unsigned int pseudo_fref_max[MADERA_FLL_MAX_FRATIO] = {
// C: 	13500000,
// C: 	 6144000,
// C: 	 6144000,
// C: 	 3072000,
// C: 	 3072000,
// C: 	 2822400,
// C: 	 2822400,
// C: 	 1536000,
// C: 	 1536000,
// C: 	 1536000,
// C: 	 1536000,
// C: 	 1536000,
// C: 	 1536000,
// C: 	 1536000,
// C: 	 1536000,
// C: 	  768000,
// C: };
// C: 
// C: struct madera_fll_gains {
// C: 	unsigned int min;
// C: 	unsigned int max;
// C: 	int gain;		/* main gain */
// C: 	int alt_gain;		/* alternate integer gain */
// C: };
// C: 
// C: static const struct madera_fll_gains madera_fll_sync_gains[] = {
// C: 	{       0,   256000, 0, -1 },
// C: 	{  256000,  1000000, 2, -1 },
// C: 	{ 1000000, 13500000, 4, -1 },
// C: };
// C: 
// C: static const struct madera_fll_gains madera_fll_main_gains[] = {
// C: 	{       0,   100000, 0, 2 },
// C: 	{  100000,   375000, 2, 2 },
// C: 	{  375000,   768000, 3, 2 },
// C: 	{  768001,  1500000, 3, 3 },
// C: 	{ 1500000,  6000000, 4, 3 },
// C: 	{ 6000000, 13500000, 5, 3 },
// C: };
// C: 
// C: static int madera_find_sync_fratio(unsigned int fref, int *fratio)
// C: {
// C: 	int i;
// C: 
// C: 	for (i = 0; i < ARRAY_SIZE(fll_sync_fratios); i++) {
// C: 		if (fll_sync_fratios[i].min <= fref &&
// C: 		    fref <= fll_sync_fratios[i].max) {
// C: 			if (fratio)
// C: 				*fratio = fll_sync_fratios[i].fratio;
// C: 
// C: 			return fll_sync_fratios[i].ratio;
// C: 		}
// C: 	}
// C: 
// C: 	return -EINVAL;
// C: }
// C: 
// C: static int madera_find_main_fratio(unsigned int fref, unsigned int fout,
// C: 				   int *fratio)
// C: {
// C: 	int ratio = 1;
// C: 
// C: 	while ((fout / (ratio * fref)) > MADERA_FLL_MAX_N)
// C: 		ratio++;
// C: 
// C: 	if (fratio)
// C: 		*fratio = ratio - 1;
// C: 
// C: 	return ratio;
// C: }
// C: 
// C: static int madera_find_fratio(struct madera_fll *fll, unsigned int fref,
// C: 			      bool sync, int *fratio)
// C: {
// C: 	switch (fll->madera->type) {
// C: 	case CS47L35:
// C: 		switch (fll->madera->rev) {
// C: 		case 0:
// C: 			/* rev A0 uses sync calculation for both loops */
// C: 			return madera_find_sync_fratio(fref, fratio);
// C: 		default:
// C: 			if (sync)
// C: 				return madera_find_sync_fratio(fref, fratio);
// C: 			else
// C: 				return madera_find_main_fratio(fref,
// C: 							       fll->fout,
// C: 							       fratio);
// C: 		}
// C: 		break;
// C: 	case CS47L85:
// C: 	case WM1840:
// C: 		/* these use the same calculation for main and sync loops */
// C: 		return madera_find_sync_fratio(fref, fratio);
// C: 	default:
// C: 		if (sync)
// C: 			return madera_find_sync_fratio(fref, fratio);
// C: 		else
// C: 			return madera_find_main_fratio(fref, fll->fout, fratio);
// C: 	}
// C: }
// C: 
// C: static int madera_calc_fratio(struct madera_fll *fll,
// C: 			      struct madera_fll_cfg *cfg,
// C: 			      unsigned int fref, bool sync)
// C: {
// C: 	int init_ratio, ratio;
// C: 	int refdiv, div;
// C: 
// C: 	/* fref must be <=13.5MHz, find initial refdiv */
// C: 	div = 1;
// C: 	cfg->refdiv = 0;
// C: 	while (fref > MADERA_FLL_MAX_FREF) {
// C: 		div *= 2;
// C: 		fref /= 2;
// C: 		cfg->refdiv++;
// C: 
// C: 		if (div > MADERA_FLL_MAX_REFDIV)
// C: 			return -EINVAL;
// C: 	}
// C: 
// C: 	/* Find an appropriate FLL_FRATIO */
// C: 	init_ratio = madera_find_fratio(fll, fref, sync, &cfg->fratio);
// C: 	if (init_ratio < 0) {
// C: 		madera_fll_err(fll, "Unable to find FRATIO for fref=%uHz\n",
// C: 			       fref);
// C: 		return init_ratio;
// C: 	}
// C: 
// C: 	if (!sync)
// C: 		cfg->fratio = init_ratio - 1;
// C: 
// C: 	switch (fll->madera->type) {
// C: 	case CS47L35:
// C: 		switch (fll->madera->rev) {
// C: 		case 0:
// C: 			if (sync)
// C: 				return init_ratio;
// C: 			break;
// C: 		default:
// C: 			return init_ratio;
// C: 		}
// C: 		break;
// C: 	case CS47L85:
// C: 	case WM1840:
// C: 		if (sync)
// C: 			return init_ratio;
// C: 		break;
// C: 	default:
// C: 		return init_ratio;
// C: 	}
// C: 
// C: 	/*
// C: 	 * For CS47L35 rev A0, CS47L85 and WM1840 adjust FRATIO/refdiv to avoid
// C: 	 * integer mode if possible
// C: 	 */
// C: 	refdiv = cfg->refdiv;
// C: 
// C: 	while (div <= MADERA_FLL_MAX_REFDIV) {
// C: 		/*
// C: 		 * start from init_ratio because this may already give a
// C: 		 * fractional N.K
// C: 		 */
// C: 		for (ratio = init_ratio; ratio > 0; ratio--) {
// C: 			if (fll->fout % (ratio * fref)) {
// C: 				cfg->refdiv = refdiv;
// C: 				cfg->fratio = ratio - 1;
// C: 				return ratio;
// C: 			}
// C: 		}
// C: 
// C: 		for (ratio = init_ratio + 1; ratio <= MADERA_FLL_MAX_FRATIO;
// C: 		     ratio++) {
// C: 			if ((MADERA_FLL_VCO_CORNER / 2) /
// C: 			    (MADERA_FLL_VCO_MULT * ratio) < fref)
// C: 				break;
// C: 
// C: 			if (fref > pseudo_fref_max[ratio - 1])
// C: 				break;
// C: 
// C: 			if (fll->fout % (ratio * fref)) {
// C: 				cfg->refdiv = refdiv;
// C: 				cfg->fratio = ratio - 1;
// C: 				return ratio;
// C: 			}
// C: 		}
// C: 
// C: 		div *= 2;
// C: 		fref /= 2;
// C: 		refdiv++;
// C: 		init_ratio = madera_find_fratio(fll, fref, sync, NULL);
// C: 	}
// C: 
// C: 	madera_fll_warn(fll, "Falling back to integer mode operation\n");
// C: 
// C: 	return cfg->fratio + 1;
// C: }
// C: 
// C: static int madera_find_fll_gain(struct madera_fll *fll,
// C: 				struct madera_fll_cfg *cfg,
// C: 				unsigned int fref,
// C: 				const struct madera_fll_gains *gains,
// C: 				int n_gains)
// C: {
// C: 	int i;
// C: 
// C: 	for (i = 0; i < n_gains; i++) {
// C: 		if (gains[i].min <= fref && fref <= gains[i].max) {
// C: 			cfg->gain = gains[i].gain;
// C: 			cfg->alt_gain = gains[i].alt_gain;
// C: 			return 0;
// C: 		}
// C: 	}
// C: 
// C: 	madera_fll_err(fll, "Unable to find gain for fref=%uHz\n", fref);
// C: 
// C: 	return -EINVAL;
// C: }
// C: 
// C: static int madera_calc_fll(struct madera_fll *fll,
// C: 			   struct madera_fll_cfg *cfg,
// C: 			   unsigned int fref, bool sync)
// C: {
// C: 	unsigned int gcd_fll;
// C: 	const struct madera_fll_gains *gains;
// C: 	int n_gains;
// C: 	int ratio, ret;
// C: 
// C: 	madera_fll_dbg(fll, "fref=%u Fout=%u fvco=%u\n",
// C: 		       fref, fll->fout, fll->fout * MADERA_FLL_VCO_MULT);
// C: 
// C: 	/* Find an appropriate FLL_FRATIO and refdiv */
// C: 	ratio = madera_calc_fratio(fll, cfg, fref, sync);
// C: 	if (ratio < 0)
// C: 		return ratio;
// C: 
// C: 	/* Apply the division for our remaining calculations */
// C: 	fref = fref / (1 << cfg->refdiv);
// C: 
// C: 	cfg->n = fll->fout / (ratio * fref);
// C: 
// C: 	if (fll->fout % (ratio * fref)) {
// C: 		gcd_fll = gcd(fll->fout, ratio * fref);
// C: 		madera_fll_dbg(fll, "GCD=%u\n", gcd_fll);
// C: 
// C: 		cfg->theta = (fll->fout - (cfg->n * ratio * fref))
// C: 			/ gcd_fll;
// C: 		cfg->lambda = (ratio * fref) / gcd_fll;
// C: 	} else {
// C: 		cfg->theta = 0;
// C: 		cfg->lambda = 0;
// C: 	}
// C: 
// C: 	/*
// C: 	 * Round down to 16bit range with cost of accuracy lost.
// C: 	 * Denominator must be bigger than numerator so we only
// C: 	 * take care of it.
// C: 	 */
// C: 	while (cfg->lambda >= (1 << 16)) {
// C: 		cfg->theta >>= 1;
// C: 		cfg->lambda >>= 1;
// C: 	}
// C: 
// C: 	switch (fll->madera->type) {
// C: 	case CS47L35:
// C: 		switch (fll->madera->rev) {
// C: 		case 0:
// C: 			/* Rev A0 uses the sync gains for both loops */
// C: 			gains = madera_fll_sync_gains;
// C: 			n_gains = ARRAY_SIZE(madera_fll_sync_gains);
// C: 			break;
// C: 		default:
// C: 			if (sync) {
// C: 				gains = madera_fll_sync_gains;
// C: 				n_gains = ARRAY_SIZE(madera_fll_sync_gains);
// C: 			} else {
// C: 				gains = madera_fll_main_gains;
// C: 				n_gains = ARRAY_SIZE(madera_fll_main_gains);
// C: 			}
// C: 			break;
// C: 		}
// C: 		break;
// C: 	case CS47L85:
// C: 	case WM1840:
// C: 		/* These use the sync gains for both loops */
// C: 		gains = madera_fll_sync_gains;
// C: 		n_gains = ARRAY_SIZE(madera_fll_sync_gains);
// C: 		break;
// C: 	default:
// C: 		if (sync) {
// C: 			gains = madera_fll_sync_gains;
// C: 			n_gains = ARRAY_SIZE(madera_fll_sync_gains);
// C: 		} else {
// C: 			gains = madera_fll_main_gains;
// C: 			n_gains = ARRAY_SIZE(madera_fll_main_gains);
// C: 		}
// C: 		break;
// C: 	}
// C: 
// C: 	ret = madera_find_fll_gain(fll, cfg, fref, gains, n_gains);
// C: 	if (ret)
// C: 		return ret;
// C: 
// C: 	madera_fll_dbg(fll, "N=%d THETA=%d LAMBDA=%d\n",
// C: 		       cfg->n, cfg->theta, cfg->lambda);
// C: 	madera_fll_dbg(fll, "FRATIO=0x%x(%d) REFCLK_DIV=0x%x(%d)\n",
// C: 		       cfg->fratio, ratio, cfg->refdiv, 1 << cfg->refdiv);
// C: 	madera_fll_dbg(fll, "GAIN=0x%x(%d)\n", cfg->gain, 1 << cfg->gain);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static bool madera_write_fll(struct madera *madera, unsigned int base,
// C: 			     struct madera_fll_cfg *cfg, int source,
// C: 			     bool sync, int gain)
// C: {
// C: 	bool change, fll_change;
// C: 
// C: 	fll_change = false;
// C: 	regmap_update_bits_check(madera->regmap,
// C: 				 base + MADERA_FLL_CONTROL_3_OFFS,
// C: 				 MADERA_FLL1_THETA_MASK,
// C: 				 cfg->theta, &change);
// C: 	fll_change |= change;
// C: 	regmap_update_bits_check(madera->regmap,
// C: 				 base + MADERA_FLL_CONTROL_4_OFFS,
// C: 				 MADERA_FLL1_LAMBDA_MASK,
// C: 				 cfg->lambda, &change);
// C: 	fll_change |= change;
// C: 	regmap_update_bits_check(madera->regmap,
// C: 				 base + MADERA_FLL_CONTROL_5_OFFS,
// C: 				 MADERA_FLL1_FRATIO_MASK,
// C: 				 cfg->fratio << MADERA_FLL1_FRATIO_SHIFT,
// C: 				 &change);
// C: 	fll_change |= change;
// C: 	regmap_update_bits_check(madera->regmap,
// C: 				 base + MADERA_FLL_CONTROL_6_OFFS,
// C: 				 MADERA_FLL1_REFCLK_DIV_MASK |
// C: 				 MADERA_FLL1_REFCLK_SRC_MASK,
// C: 				 cfg->refdiv << MADERA_FLL1_REFCLK_DIV_SHIFT |
// C: 				 source << MADERA_FLL1_REFCLK_SRC_SHIFT,
// C: 				 &change);
// C: 	fll_change |= change;
// C: 
// C: 	if (sync) {
// C: 		regmap_update_bits_check(madera->regmap,
// C: 					 base + MADERA_FLL_SYNCHRONISER_7_OFFS,
// C: 					 MADERA_FLL1_GAIN_MASK,
// C: 					 gain << MADERA_FLL1_GAIN_SHIFT,
// C: 					 &change);
// C: 		fll_change |= change;
// C: 	} else {
// C: 		regmap_update_bits_check(madera->regmap,
// C: 					 base + MADERA_FLL_CONTROL_7_OFFS,
// C: 					 MADERA_FLL1_GAIN_MASK,
// C: 					 gain << MADERA_FLL1_GAIN_SHIFT,
// C: 					 &change);
// C: 		fll_change |= change;
// C: 	}
// C: 
// C: 	regmap_update_bits_check(madera->regmap,
// C: 				 base + MADERA_FLL_CONTROL_2_OFFS,
// C: 				 MADERA_FLL1_CTRL_UPD | MADERA_FLL1_N_MASK,
// C: 				 MADERA_FLL1_CTRL_UPD | cfg->n, &change);
// C: 	fll_change |= change;
// C: 
// C: 	return fll_change;
// C: }
// C: 
// C: static int madera_is_enabled_fll(struct madera_fll *fll, int base)
// C: {
// C: 	struct madera *madera = fll->madera;
// C: 	unsigned int reg;
// C: 	int ret;
// C: 
// C: 	ret = regmap_read(madera->regmap,
// C: 			  base + MADERA_FLL_CONTROL_1_OFFS, &reg);
// C: 	if (ret != 0) {
// C: 		madera_fll_err(fll, "Failed to read current state: %d\n", ret);
// C: 		return ret;
// C: 	}
// C: 
// C: 	return reg & MADERA_FLL1_ENA;
// C: }
// C: 
// C: static int madera_wait_for_fll(struct madera_fll *fll, bool requested)
// C: {
// C: 	struct madera *madera = fll->madera;
// C: 	unsigned int val = 0;
// C: 	bool status;
// C: 	int i;
// C: 
// C: 	madera_fll_dbg(fll, "Waiting for FLL...\n");
// C: 
// C: 	for (i = 0; i < 30; i++) {
// C: 		regmap_read(madera->regmap, MADERA_IRQ1_RAW_STATUS_2, &val);
// C: 		status = val & (MADERA_FLL1_LOCK_STS1 << (fll->id - 1));
// C: 		if (status == requested)
// C: 			return 0;
// C: 
// C: 		switch (i) {
// C: 		case 0 ... 5:
// C: 			usleep_range(75, 125);
// C: 			break;
// C: 		case 11 ... 20:
// C: 			usleep_range(750, 1250);
// C: 			break;
// C: 		default:
// C: 			msleep(20);
// C: 			break;
// C: 		}
// C: 	}
// C: 
// C: 	madera_fll_warn(fll, "Timed out waiting for lock\n");
// C: 
// C: 	return -ETIMEDOUT;
// C: }
// C: 
// C: static bool madera_set_fll_phase_integrator(struct madera_fll *fll,
// C: 					    struct madera_fll_cfg *ref_cfg,
// C: 					    bool sync)
// C: {
// C: 	unsigned int val;
// C: 	bool reg_change;
// C: 
// C: 	if (!sync && ref_cfg->theta == 0)
// C: 		val = (1 << MADERA_FLL1_PHASE_ENA_SHIFT) |
// C: 		      (2 << MADERA_FLL1_PHASE_GAIN_SHIFT);
// C: 	else
// C: 		val = 2 << MADERA_FLL1_PHASE_GAIN_SHIFT;
// C: 
// C: 	regmap_update_bits_check(fll->madera->regmap,
// C: 				 fll->base + MADERA_FLL_EFS_2_OFFS,
// C: 				 MADERA_FLL1_PHASE_ENA_MASK |
// C: 				 MADERA_FLL1_PHASE_GAIN_MASK,
// C: 				 val, &reg_change);
// C: 
// C: 	return reg_change;
// C: }
// C: 
// C: static int madera_set_fll_clks_reg(struct madera_fll *fll, bool ena,
// C: 				   unsigned int reg, unsigned int mask,
// C: 				   unsigned int shift)
// C: {
// C: 	struct madera *madera = fll->madera;
// C: 	unsigned int src;
// C: 	struct clk *clk;
// C: 	int ret;
// C: 
// C: 	ret = regmap_read(madera->regmap, reg, &src);
// C: 	if (ret != 0) {
// C: 		madera_fll_err(fll, "Failed to read current source: %d\n",
// C: 			       ret);
// C: 		return ret;
// C: 	}
// C: 
// C: 	src = (src & mask) >> shift;
// C: 
// C: 	switch (src) {
// C: 	case MADERA_FLL_SRC_MCLK1:
// C: 		clk = madera->mclk[MADERA_MCLK1].clk;
// C: 		break;
// C: 	case MADERA_FLL_SRC_MCLK2:
// C: 		clk = madera->mclk[MADERA_MCLK2].clk;
// C: 		break;
// C: 	case MADERA_FLL_SRC_MCLK3:
// C: 		clk = madera->mclk[MADERA_MCLK3].clk;
// C: 		break;
// C: 	default:
// C: 		return 0;
// C: 	}
// C: 
// C: 	if (ena) {
// C: 		return clk_prepare_enable(clk);
// C: 	} else {
// C: 		clk_disable_unprepare(clk);
// C: 		return 0;
// C: 	}
// C: }
// C: 
// C: static inline int madera_set_fll_clks(struct madera_fll *fll, int base, bool ena)
// C: {
// C: 	return madera_set_fll_clks_reg(fll, ena,
// C: 				       base + MADERA_FLL_CONTROL_6_OFFS,
// C: 				       MADERA_FLL1_REFCLK_SRC_MASK,
// C: 				       MADERA_FLL1_REFCLK_SRC_SHIFT);
// C: }
// C: 
// C: static inline int madera_set_fllao_clks(struct madera_fll *fll, int base, bool ena)
// C: {
// C: 	return madera_set_fll_clks_reg(fll, ena,
// C: 				       base + MADERA_FLLAO_CONTROL_6_OFFS,
// C: 				       MADERA_FLL_AO_REFCLK_SRC_MASK,
// C: 				       MADERA_FLL_AO_REFCLK_SRC_SHIFT);
// C: }
// C: 
// C: static inline int madera_set_fllhj_clks(struct madera_fll *fll, int base, bool ena)
// C: {
// C: 	return madera_set_fll_clks_reg(fll, ena,
// C: 				       base + MADERA_FLL_CONTROL_1_OFFS,
// C: 				       CS47L92_FLL1_REFCLK_SRC_MASK,
// C: 				       CS47L92_FLL1_REFCLK_SRC_SHIFT);
// C: }
// C: 
// C: static void madera_disable_fll(struct madera_fll *fll)
// C: {
// C: 	struct madera *madera = fll->madera;
// C: 	unsigned int sync_base;
// C: 	bool ref_change, sync_change;
// C: 
// C: 	switch (madera->type) {
// C: 	case CS47L35:
// C: 		sync_base = fll->base + CS47L35_FLL_SYNCHRONISER_OFFS;
// C: 		break;
// C: 	default:
// C: 		sync_base = fll->base + MADERA_FLL_SYNCHRONISER_OFFS;
// C: 		break;
// C: 	}
// C: 
// C: 	madera_fll_dbg(fll, "Disabling FLL\n");
// C: 
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_1_OFFS,
// C: 			   MADERA_FLL1_FREERUN, MADERA_FLL1_FREERUN);
// C: 	regmap_update_bits_check(madera->regmap,
// C: 				 fll->base + MADERA_FLL_CONTROL_1_OFFS,
// C: 				 MADERA_FLL1_ENA, 0, &ref_change);
// C: 	regmap_update_bits_check(madera->regmap,
// C: 				 sync_base + MADERA_FLL_SYNCHRONISER_1_OFFS,
// C: 				 MADERA_FLL1_SYNC_ENA, 0, &sync_change);
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_1_OFFS,
// C: 			   MADERA_FLL1_FREERUN, 0);
// C: 
// C: 	madera_wait_for_fll(fll, false);
// C: 
// C: 	if (sync_change)
// C: 		madera_set_fll_clks(fll, sync_base, false);
// C: 
// C: 	if (ref_change) {
// C: 		madera_set_fll_clks(fll, fll->base, false);
// C: 		pm_runtime_put_autosuspend(madera->dev);
// C: 	}
// C: }
// C: 
// C: static int madera_enable_fll(struct madera_fll *fll)
// C: {
// C: 	struct madera *madera = fll->madera;
// C: 	bool have_sync = false;
// C: 	int already_enabled = madera_is_enabled_fll(fll, fll->base);
// C: 	int sync_enabled;
// C: 	struct madera_fll_cfg cfg;
// C: 	unsigned int sync_base;
// C: 	int gain, ret;
// C: 	bool fll_change = false;
// C: 
// C: 	if (already_enabled < 0)
// C: 		return already_enabled;	/* error getting current state */
// C: 
// C: 	if (fll->ref_src < 0 || fll->ref_freq == 0) {
// C: 		madera_fll_err(fll, "No REFCLK\n");
// C: 		ret = -EINVAL;
// C: 		goto err;
// C: 	}
// C: 
// C: 	madera_fll_dbg(fll, "Enabling FLL, initially %s\n",
// C: 		       str_enabled_disabled(already_enabled));
// C: 
// C: 	if (fll->fout < MADERA_FLL_MIN_FOUT ||
// C: 	    fll->fout > MADERA_FLL_MAX_FOUT) {
// C: 		madera_fll_err(fll, "invalid fout %uHz\n", fll->fout);
// C: 		ret = -EINVAL;
// C: 		goto err;
// C: 	}
// C: 
// C: 	switch (madera->type) {
// C: 	case CS47L35:
// C: 		sync_base = fll->base + CS47L35_FLL_SYNCHRONISER_OFFS;
// C: 		break;
// C: 	default:
// C: 		sync_base = fll->base + MADERA_FLL_SYNCHRONISER_OFFS;
// C: 		break;
// C: 	}
// C: 
// C: 	sync_enabled = madera_is_enabled_fll(fll, sync_base);
// C: 	if (sync_enabled < 0)
// C: 		return sync_enabled;
// C: 
// C: 	if (already_enabled) {
// C: 		/* Facilitate smooth refclk across the transition */
// C: 		regmap_update_bits(fll->madera->regmap,
// C: 				   fll->base + MADERA_FLL_CONTROL_1_OFFS,
// C: 				   MADERA_FLL1_FREERUN,
// C: 				   MADERA_FLL1_FREERUN);
// C: 		udelay(32);
// C: 		regmap_update_bits(fll->madera->regmap,
// C: 				   fll->base + MADERA_FLL_CONTROL_7_OFFS,
// C: 				   MADERA_FLL1_GAIN_MASK, 0);
// C: 
// C: 		if (sync_enabled > 0)
// C: 			madera_set_fll_clks(fll, sync_base, false);
// C: 		madera_set_fll_clks(fll, fll->base, false);
// C: 	}
// C: 
// C: 	/* Apply SYNCCLK setting */
// C: 	if (fll->sync_src >= 0) {
// C: 		ret = madera_calc_fll(fll, &cfg, fll->sync_freq, true);
// C: 		if (ret < 0)
// C: 			goto err;
// C: 
// C: 		fll_change |= madera_write_fll(madera, sync_base,
// C: 					       &cfg, fll->sync_src,
// C: 					       true, cfg.gain);
// C: 		have_sync = true;
// C: 	}
// C: 
// C: 	if (already_enabled && !!sync_enabled != have_sync)
// C: 		madera_fll_warn(fll, "Synchroniser changed on active FLL\n");
// C: 
// C: 	/* Apply REFCLK setting */
// C: 	ret = madera_calc_fll(fll, &cfg, fll->ref_freq, false);
// C: 	if (ret < 0)
// C: 		goto err;
// C: 
// C: 	/* Ref path hardcodes lambda to 65536 when sync is on */
// C: 	if (have_sync && cfg.lambda)
// C: 		cfg.theta = (cfg.theta * (1 << 16)) / cfg.lambda;
// C: 
// C: 	switch (fll->madera->type) {
// C: 	case CS47L35:
// C: 		switch (fll->madera->rev) {
// C: 		case 0:
// C: 			gain = cfg.gain;
// C: 			break;
// C: 		default:
// C: 			fll_change |=
// C: 				madera_set_fll_phase_integrator(fll, &cfg,
// C: 								have_sync);
// C: 			if (!have_sync && cfg.theta == 0)
// C: 				gain = cfg.alt_gain;
// C: 			else
// C: 				gain = cfg.gain;
// C: 			break;
// C: 		}
// C: 		break;
// C: 	case CS47L85:
// C: 	case WM1840:
// C: 		gain = cfg.gain;
// C: 		break;
// C: 	default:
// C: 		fll_change |= madera_set_fll_phase_integrator(fll, &cfg,
// C: 							      have_sync);
// C: 		if (!have_sync && cfg.theta == 0)
// C: 			gain = cfg.alt_gain;
// C: 		else
// C: 			gain = cfg.gain;
// C: 		break;
// C: 	}
// C: 
// C: 	fll_change |= madera_write_fll(madera, fll->base,
// C: 				       &cfg, fll->ref_src,
// C: 				       false, gain);
// C: 
// C: 	/*
// C: 	 * Increase the bandwidth if we're not using a low frequency
// C: 	 * sync source.
// C: 	 */
// C: 	if (have_sync && fll->sync_freq > 100000)
// C: 		regmap_update_bits(madera->regmap,
// C: 				   sync_base + MADERA_FLL_SYNCHRONISER_7_OFFS,
// C: 				   MADERA_FLL1_SYNC_DFSAT_MASK, 0);
// C: 	else
// C: 		regmap_update_bits(madera->regmap,
// C: 				   sync_base + MADERA_FLL_SYNCHRONISER_7_OFFS,
// C: 				   MADERA_FLL1_SYNC_DFSAT_MASK,
// C: 				   MADERA_FLL1_SYNC_DFSAT);
// C: 
// C: 	if (!already_enabled)
// C: 		pm_runtime_get_sync(madera->dev);
// C: 
// C: 	if (have_sync) {
// C: 		madera_set_fll_clks(fll, sync_base, true);
// C: 		regmap_update_bits(madera->regmap,
// C: 				   sync_base + MADERA_FLL_SYNCHRONISER_1_OFFS,
// C: 				   MADERA_FLL1_SYNC_ENA,
// C: 				   MADERA_FLL1_SYNC_ENA);
// C: 	}
// C: 
// C: 	madera_set_fll_clks(fll, fll->base, true);
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_1_OFFS,
// C: 			   MADERA_FLL1_ENA, MADERA_FLL1_ENA);
// C: 
// C: 	if (already_enabled)
// C: 		regmap_update_bits(madera->regmap,
// C: 				   fll->base + MADERA_FLL_CONTROL_1_OFFS,
// C: 				   MADERA_FLL1_FREERUN, 0);
// C: 
// C: 	if (fll_change || !already_enabled)
// C: 		madera_wait_for_fll(fll, true);
// C: 
// C: 	return 0;
// C: 
// C: err:
// C: 	 /* In case of error don't leave the FLL running with an old config */
// C: 	madera_disable_fll(fll);
// C: 
// C: 	return ret;
// C: }
// C: 
// C: static int madera_apply_fll(struct madera_fll *fll)
// C: {
// C: 	if (fll->fout) {
// C: 		return madera_enable_fll(fll);
// C: 	} else {
// C: 		madera_disable_fll(fll);
// C: 		return 0;
// C: 	}
// C: }
// C: 
// C: int madera_set_fll_syncclk(struct madera_fll *fll, int source,
// C: 			   unsigned int fref, unsigned int fout)
// C: {
// C: 	/*
// C: 	 * fout is ignored, since the synchronizer is an optional extra
// C: 	 * constraint on the Fout generated from REFCLK, so the Fout is
// C: 	 * set when configuring REFCLK
// C: 	 */
// C: 
// C: 	if (fll->sync_src == source && fll->sync_freq == fref)
// C: 		return 0;
// C: 
// C: 	fll->sync_src = source;
// C: 	fll->sync_freq = fref;
// C: 
// C: 	return madera_apply_fll(fll);
// C: }
// C: EXPORT_SYMBOL_GPL(madera_set_fll_syncclk);
// C: 
// C: int madera_set_fll_refclk(struct madera_fll *fll, int source,
// C: 			  unsigned int fref, unsigned int fout)
// C: {
// C: 	int ret;
// C: 
// C: 	if (fll->ref_src == source &&
// C: 	    fll->ref_freq == fref && fll->fout == fout)
// C: 		return 0;
// C: 
// C: 	/*
// C: 	 * Changes of fout on an enabled FLL aren't allowed except when
// C: 	 * setting fout==0 to disable the FLL
// C: 	 */
// C: 	if (fout && fout != fll->fout) {
// C: 		ret = madera_is_enabled_fll(fll, fll->base);
// C: 		if (ret < 0)
// C: 			return ret;
// C: 
// C: 		if (ret) {
// C: 			madera_fll_err(fll, "Can't change Fout on active FLL\n");
// C: 			return -EBUSY;
// C: 		}
// C: 	}
// C: 
// C: 	fll->ref_src = source;
// C: 	fll->ref_freq = fref;
// C: 	fll->fout = fout;
// C: 
// C: 	return madera_apply_fll(fll);
// C: }
// C: EXPORT_SYMBOL_GPL(madera_set_fll_refclk);
// C: 
// C: int madera_init_fll(struct madera *madera, int id, int base,
// C: 		    struct madera_fll *fll)
// C: {
// C: 	fll->id = id;
// C: 	fll->base = base;
// C: 	fll->madera = madera;
// C: 	fll->ref_src = MADERA_FLL_SRC_NONE;
// C: 	fll->sync_src = MADERA_FLL_SRC_NONE;
// C: 
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_1_OFFS,
// C: 			   MADERA_FLL1_FREERUN, 0);
// C: 
// C: 	return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_init_fll);
// C: 
// C: static const struct reg_sequence madera_fll_ao_32K_49M_patch[] = {
// C: 	{ MADERA_FLLAO_CONTROL_2,  0x02EE },
// C: 	{ MADERA_FLLAO_CONTROL_3,  0x0000 },
// C: 	{ MADERA_FLLAO_CONTROL_4,  0x0001 },
// C: 	{ MADERA_FLLAO_CONTROL_5,  0x0002 },
// C: 	{ MADERA_FLLAO_CONTROL_6,  0x8001 },
// C: 	{ MADERA_FLLAO_CONTROL_7,  0x0004 },
// C: 	{ MADERA_FLLAO_CONTROL_8,  0x0077 },
// C: 	{ MADERA_FLLAO_CONTROL_10, 0x06D8 },
// C: 	{ MADERA_FLLAO_CONTROL_11, 0x0085 },
// C: 	{ MADERA_FLLAO_CONTROL_2,  0x82EE },
// C: };
// C: 
// C: static const struct reg_sequence madera_fll_ao_32K_45M_patch[] = {
// C: 	{ MADERA_FLLAO_CONTROL_2,  0x02B1 },
// C: 	{ MADERA_FLLAO_CONTROL_3,  0x0001 },
// C: 	{ MADERA_FLLAO_CONTROL_4,  0x0010 },
// C: 	{ MADERA_FLLAO_CONTROL_5,  0x0002 },
// C: 	{ MADERA_FLLAO_CONTROL_6,  0x8001 },
// C: 	{ MADERA_FLLAO_CONTROL_7,  0x0004 },
// C: 	{ MADERA_FLLAO_CONTROL_8,  0x0077 },
// C: 	{ MADERA_FLLAO_CONTROL_10, 0x06D8 },
// C: 	{ MADERA_FLLAO_CONTROL_11, 0x0005 },
// C: 	{ MADERA_FLLAO_CONTROL_2,  0x82B1 },
// C: };
// C: 
// C: struct madera_fllao_patch {
// C: 	unsigned int fin;
// C: 	unsigned int fout;
// C: 	const struct reg_sequence *patch;
// C: 	unsigned int patch_size;
// C: };
// C: 
// C: static const struct madera_fllao_patch madera_fllao_settings[] = {
// C: 	{
// C: 		.fin = 32768,
// C: 		.fout = 49152000,
// C: 		.patch = madera_fll_ao_32K_49M_patch,
// C: 		.patch_size = ARRAY_SIZE(madera_fll_ao_32K_49M_patch),
// C: 
// C: 	},
// C: 	{
// C: 		.fin = 32768,
// C: 		.fout = 45158400,
// C: 		.patch = madera_fll_ao_32K_45M_patch,
// C: 		.patch_size = ARRAY_SIZE(madera_fll_ao_32K_45M_patch),
// C: 	},
// C: };
// C: 
// C: static int madera_enable_fll_ao(struct madera_fll *fll,
// C: 				const struct reg_sequence *patch,
// C: 				unsigned int patch_size)
// C: {
// C: 	struct madera *madera = fll->madera;
// C: 	int already_enabled = madera_is_enabled_fll(fll, fll->base);
// C: 	unsigned int val;
// C: 	int i;
// C: 
// C: 	if (already_enabled < 0)
// C: 		return already_enabled;
// C: 
// C: 	if (!already_enabled)
// C: 		pm_runtime_get_sync(madera->dev);
// C: 
// C: 	madera_fll_dbg(fll, "Enabling FLL_AO, initially %s\n",
// C: 		       str_enabled_disabled(already_enabled));
// C: 
// C: 	/* FLL_AO_HOLD must be set before configuring any registers */
// C: 	regmap_update_bits(fll->madera->regmap,
// C: 			   fll->base + MADERA_FLLAO_CONTROL_1_OFFS,
// C: 			   MADERA_FLL_AO_HOLD, MADERA_FLL_AO_HOLD);
// C: 
// C: 	if (already_enabled)
// C: 		madera_set_fllao_clks(fll, fll->base, false);
// C: 
// C: 	for (i = 0; i < patch_size; i++) {
// C: 		val = patch[i].def;
// C: 
// C: 		/* modify the patch to apply fll->ref_src as input clock */
// C: 		if (patch[i].reg == MADERA_FLLAO_CONTROL_6) {
// C: 			val &= ~MADERA_FLL_AO_REFCLK_SRC_MASK;
// C: 			val |= (fll->ref_src << MADERA_FLL_AO_REFCLK_SRC_SHIFT)
// C: 				& MADERA_FLL_AO_REFCLK_SRC_MASK;
// C: 		}
// C: 
// C: 		regmap_write(madera->regmap, patch[i].reg, val);
// C: 	}
// C: 
// C: 	madera_set_fllao_clks(fll, fll->base, true);
// C: 
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLLAO_CONTROL_1_OFFS,
// C: 			   MADERA_FLL_AO_ENA, MADERA_FLL_AO_ENA);
// C: 
// C: 	/* Release the hold so that fll_ao locks to external frequency */
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLLAO_CONTROL_1_OFFS,
// C: 			   MADERA_FLL_AO_HOLD, 0);
// C: 
// C: 	if (!already_enabled)
// C: 		madera_wait_for_fll(fll, true);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int madera_disable_fll_ao(struct madera_fll *fll)
// C: {
// C: 	struct madera *madera = fll->madera;
// C: 	bool change;
// C: 
// C: 	madera_fll_dbg(fll, "Disabling FLL_AO\n");
// C: 
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLLAO_CONTROL_1_OFFS,
// C: 			   MADERA_FLL_AO_HOLD, MADERA_FLL_AO_HOLD);
// C: 	regmap_update_bits_check(madera->regmap,
// C: 				 fll->base + MADERA_FLLAO_CONTROL_1_OFFS,
// C: 				 MADERA_FLL_AO_ENA, 0, &change);
// C: 
// C: 	madera_wait_for_fll(fll, false);
// C: 
// C: 	/*
// C: 	 * ctrl_up gates the writes to all fll_ao register, setting it to 0
// C: 	 * here ensures that after a runtime suspend/resume cycle when one
// C: 	 * enables the fllao then ctrl_up is the last bit that is configured
// C: 	 * by the fllao enable code rather than the cache sync operation which
// C: 	 * would have updated it much earlier before writing out all fllao
// C: 	 * registers
// C: 	 */
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLLAO_CONTROL_2_OFFS,
// C: 			   MADERA_FLL_AO_CTRL_UPD_MASK, 0);
// C: 
// C: 	if (change) {
// C: 		madera_set_fllao_clks(fll, fll->base, false);
// C: 		pm_runtime_put_autosuspend(madera->dev);
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: int madera_set_fll_ao_refclk(struct madera_fll *fll, int source,
// C: 			     unsigned int fin, unsigned int fout)
// C: {
// C: 	int ret = 0;
// C: 	const struct reg_sequence *patch = NULL;
// C: 	int patch_size = 0;
// C: 	unsigned int i;
// C: 
// C: 	if (fll->ref_src == source &&
// C: 	    fll->ref_freq == fin && fll->fout == fout)
// C: 		return 0;
// C: 
// C: 	madera_fll_dbg(fll, "Change FLL_AO refclk to fin=%u fout=%u source=%d\n",
// C: 		       fin, fout, source);
// C: 
// C: 	if (fout && (fll->ref_freq != fin || fll->fout != fout)) {
// C: 		for (i = 0; i < ARRAY_SIZE(madera_fllao_settings); i++) {
// C: 			if (madera_fllao_settings[i].fin == fin &&
// C: 			    madera_fllao_settings[i].fout == fout)
// C: 				break;
// C: 		}
// C: 
// C: 		if (i == ARRAY_SIZE(madera_fllao_settings)) {
// C: 			madera_fll_err(fll,
// C: 				       "No matching configuration for FLL_AO\n");
// C: 			return -EINVAL;
// C: 		}
// C: 
// C: 		patch = madera_fllao_settings[i].patch;
// C: 		patch_size = madera_fllao_settings[i].patch_size;
// C: 	}
// C: 
// C: 	fll->ref_src = source;
// C: 	fll->ref_freq = fin;
// C: 	fll->fout = fout;
// C: 
// C: 	if (fout)
// C: 		ret = madera_enable_fll_ao(fll, patch, patch_size);
// C: 	else
// C: 		madera_disable_fll_ao(fll);
// C: 
// C: 	return ret;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_set_fll_ao_refclk);
// C: 
// C: static int madera_fllhj_disable(struct madera_fll *fll)
// C: {
// C: 	struct madera *madera = fll->madera;
// C: 	bool change;
// C: 
// C: 	madera_fll_dbg(fll, "Disabling FLL\n");
// C: 
// C: 	/* Disable lockdet, but don't set ctrl_upd update but.  This allows the
// C: 	 * lock status bit to clear as normal, but should the FLL be enabled
// C: 	 * again due to a control clock being required, the lock won't re-assert
// C: 	 * as the FLL config registers are automatically applied when the FLL
// C: 	 * enables.
// C: 	 */
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_11_OFFS,
// C: 			   MADERA_FLL1_LOCKDET_MASK, 0);
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_1_OFFS,
// C: 			   MADERA_FLL1_HOLD_MASK, MADERA_FLL1_HOLD_MASK);
// C: 	regmap_update_bits_check(madera->regmap,
// C: 				 fll->base + MADERA_FLL_CONTROL_1_OFFS,
// C: 				 MADERA_FLL1_ENA_MASK, 0, &change);
// C: 
// C: 	madera_wait_for_fll(fll, false);
// C: 
// C: 	/* ctrl_up gates the writes to all the fll's registers, setting it to 0
// C: 	 * here ensures that after a runtime suspend/resume cycle when one
// C: 	 * enables the fll then ctrl_up is the last bit that is configured
// C: 	 * by the fll enable code rather than the cache sync operation which
// C: 	 * would have updated it much earlier before writing out all fll
// C: 	 * registers
// C: 	 */
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_2_OFFS,
// C: 			   MADERA_FLL1_CTRL_UPD_MASK, 0);
// C: 
// C: 	if (change) {
// C: 		madera_set_fllhj_clks(fll, fll->base, false);
// C: 		pm_runtime_put_autosuspend(madera->dev);
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int madera_fllhj_apply(struct madera_fll *fll, int fin)
// C: {
// C: 	struct madera *madera = fll->madera;
// C: 	int refdiv, fref, fout, lockdet_thr, fbdiv, hp, fast_clk, fllgcd;
// C: 	bool frac = false;
// C: 	unsigned int fll_n, min_n, max_n, ratio, theta, lambda;
// C: 	unsigned int gains, val, num;
// C: 
// C: 	madera_fll_dbg(fll, "fin=%d, fout=%d\n", fin, fll->fout);
// C: 
// C: 	for (refdiv = 0; refdiv < 4; refdiv++)
// C: 		if ((fin / (1 << refdiv)) <= MADERA_FLLHJ_MAX_THRESH)
// C: 			break;
// C: 
// C: 	fref = fin / (1 << refdiv);
// C: 
// C: 	/* Use simple heuristic approach to find a configuration that
// C: 	 * should work for most input clocks.
// C: 	 */
// C: 	fast_clk = 0;
// C: 	fout = fll->fout;
// C: 	frac = fout % fref;
// C: 
// C: 	if (fref < MADERA_FLLHJ_LOW_THRESH) {
// C: 		lockdet_thr = 2;
// C: 		gains = MADERA_FLLHJ_LOW_GAINS;
// C: 		if (frac)
// C: 			fbdiv = 256;
// C: 		else
// C: 			fbdiv = 4;
// C: 	} else if (fref < MADERA_FLLHJ_MID_THRESH) {
// C: 		lockdet_thr = 8;
// C: 		gains = MADERA_FLLHJ_MID_GAINS;
// C: 		fbdiv = 1;
// C: 	} else {
// C: 		lockdet_thr = 8;
// C: 		gains = MADERA_FLLHJ_HIGH_GAINS;
// C: 		fbdiv = 1;
// C: 		/* For high speed input clocks, enable 300MHz fast oscillator
// C: 		 * when we're in fractional divider mode.
// C: 		 */
// C: 		if (frac) {
// C: 			fast_clk = 0x3;
// C: 			fout = fll->fout * 6;
// C: 		}
// C: 	}
// C: 	/* Use high performance mode for fractional configurations. */
// C: 	if (frac) {
// C: 		hp = 0x3;
// C: 		min_n = MADERA_FLLHJ_FRAC_MIN_N;
// C: 		max_n = MADERA_FLLHJ_FRAC_MAX_N;
// C: 	} else {
// C: 		hp = 0x0;
// C: 		min_n = MADERA_FLLHJ_INT_MIN_N;
// C: 		max_n = MADERA_FLLHJ_INT_MAX_N;
// C: 	}
// C: 
// C: 	ratio = fout / fref;
// C: 
// C: 	madera_fll_dbg(fll, "refdiv=%d, fref=%d, frac:%d\n",
// C: 		       refdiv, fref, frac);
// C: 
// C: 	while (ratio / fbdiv < min_n) {
// C: 		fbdiv /= 2;
// C: 		if (fbdiv < 1) {
// C: 			madera_fll_err(fll, "FBDIV (%d) must be >= 1\n", fbdiv);
// C: 			return -EINVAL;
// C: 		}
// C: 	}
// C: 	while (frac && (ratio / fbdiv > max_n)) {
// C: 		fbdiv *= 2;
// C: 		if (fbdiv >= 1024) {
// C: 			madera_fll_err(fll, "FBDIV (%u) >= 1024\n", fbdiv);
// C: 			return -EINVAL;
// C: 		}
// C: 	}
// C: 
// C: 	madera_fll_dbg(fll, "lockdet=%d, hp=0x%x, fbdiv:%d\n",
// C: 		       lockdet_thr, hp, fbdiv);
// C: 
// C: 	/* Calculate N.K values */
// C: 	fllgcd = gcd(fout, fbdiv * fref);
// C: 	num = fout / fllgcd;
// C: 	lambda = (fref * fbdiv) / fllgcd;
// C: 	fll_n = num / lambda;
// C: 	theta = num % lambda;
// C: 
// C: 	madera_fll_dbg(fll, "fll_n=%d, gcd=%d, theta=%d, lambda=%d\n",
// C: 		       fll_n, fllgcd, theta, lambda);
// C: 
// C: 	/* Some sanity checks before any registers are written. */
// C: 	if (fll_n < min_n || fll_n > max_n) {
// C: 		madera_fll_err(fll, "N not in valid %s mode range %d-%d: %d\n",
// C: 			       frac ? "fractional" : "integer", min_n, max_n,
// C: 			       fll_n);
// C: 		return -EINVAL;
// C: 	}
// C: 	if (fbdiv < 1 || (frac && fbdiv >= 1024) || (!frac && fbdiv >= 256)) {
// C: 		madera_fll_err(fll, "Invalid fbdiv for %s mode (%u)\n",
// C: 			       frac ? "fractional" : "integer", fbdiv);
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	/* clear the ctrl_upd bit to guarantee we write to it later. */
// C: 	regmap_write(madera->regmap,
// C: 		     fll->base + MADERA_FLL_CONTROL_2_OFFS,
// C: 		     fll_n << MADERA_FLL1_N_SHIFT);
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_3_OFFS,
// C: 			   MADERA_FLL1_THETA_MASK,
// C: 			   theta << MADERA_FLL1_THETA_SHIFT);
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_4_OFFS,
// C: 			   MADERA_FLL1_LAMBDA_MASK,
// C: 			   lambda << MADERA_FLL1_LAMBDA_SHIFT);
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_5_OFFS,
// C: 			   MADERA_FLL1_FB_DIV_MASK,
// C: 			   fbdiv << MADERA_FLL1_FB_DIV_SHIFT);
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_6_OFFS,
// C: 			   MADERA_FLL1_REFCLK_DIV_MASK,
// C: 			   refdiv << MADERA_FLL1_REFCLK_DIV_SHIFT);
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_GAIN_OFFS,
// C: 			   0xffff,
// C: 			   gains);
// C: 	val = hp << MADERA_FLL1_HP_SHIFT;
// C: 	val |= 1 << MADERA_FLL1_PHASEDET_ENA_SHIFT;
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_10_OFFS,
// C: 			   MADERA_FLL1_HP_MASK | MADERA_FLL1_PHASEDET_ENA_MASK,
// C: 			   val);
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_11_OFFS,
// C: 			   MADERA_FLL1_LOCKDET_THR_MASK,
// C: 			   lockdet_thr << MADERA_FLL1_LOCKDET_THR_SHIFT);
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL1_DIGITAL_TEST_1_OFFS,
// C: 			   MADERA_FLL1_SYNC_EFS_ENA_MASK |
// C: 			   MADERA_FLL1_CLK_VCO_FAST_SRC_MASK,
// C: 			   fast_clk);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int madera_fllhj_enable(struct madera_fll *fll)
// C: {
// C: 	struct madera *madera = fll->madera;
// C: 	int already_enabled = madera_is_enabled_fll(fll, fll->base);
// C: 	int ret;
// C: 
// C: 	if (already_enabled < 0)
// C: 		return already_enabled;
// C: 
// C: 	if (!already_enabled)
// C: 		pm_runtime_get_sync(madera->dev);
// C: 
// C: 	madera_fll_dbg(fll, "Enabling FLL, initially %s\n",
// C: 		       str_enabled_disabled(already_enabled));
// C: 
// C: 	/* FLLn_HOLD must be set before configuring any registers */
// C: 	regmap_update_bits(fll->madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_1_OFFS,
// C: 			   MADERA_FLL1_HOLD_MASK,
// C: 			   MADERA_FLL1_HOLD_MASK);
// C: 
// C: 	if (already_enabled)
// C: 		madera_set_fllhj_clks(fll, fll->base, false);
// C: 
// C: 	/* Apply refclk */
// C: 	ret = madera_fllhj_apply(fll, fll->ref_freq);
// C: 	if (ret) {
// C: 		madera_fll_err(fll, "Failed to set FLL: %d\n", ret);
// C: 		goto out;
// C: 	}
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_1_OFFS,
// C: 			   CS47L92_FLL1_REFCLK_SRC_MASK,
// C: 			   fll->ref_src << CS47L92_FLL1_REFCLK_SRC_SHIFT);
// C: 
// C: 	madera_set_fllhj_clks(fll, fll->base, true);
// C: 
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_1_OFFS,
// C: 			   MADERA_FLL1_ENA_MASK,
// C: 			   MADERA_FLL1_ENA_MASK);
// C: 
// C: out:
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_11_OFFS,
// C: 			   MADERA_FLL1_LOCKDET_MASK,
// C: 			   MADERA_FLL1_LOCKDET_MASK);
// C: 
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_2_OFFS,
// C: 			   MADERA_FLL1_CTRL_UPD_MASK,
// C: 			   MADERA_FLL1_CTRL_UPD_MASK);
// C: 
// C: 	/* Release the hold so that flln locks to external frequency */
// C: 	regmap_update_bits(madera->regmap,
// C: 			   fll->base + MADERA_FLL_CONTROL_1_OFFS,
// C: 			   MADERA_FLL1_HOLD_MASK,
// C: 			   0);
// C: 
// C: 	if (!already_enabled)
// C: 		madera_wait_for_fll(fll, true);
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int madera_fllhj_validate(struct madera_fll *fll,
// C: 				 unsigned int ref_in,
// C: 				 unsigned int fout)
// C: {
// C: 	if (fout && !ref_in) {
// C: 		madera_fll_err(fll, "fllout set without valid input clk\n");
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	if (fll->fout && fout != fll->fout) {
// C: 		madera_fll_err(fll, "Can't change output on active FLL\n");
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	if (ref_in / MADERA_FLL_MAX_REFDIV > MADERA_FLLHJ_MAX_THRESH) {
// C: 		madera_fll_err(fll, "Can't scale %dMHz to <=13MHz\n", ref_in);
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: int madera_fllhj_set_refclk(struct madera_fll *fll, int source,
// C: 			    unsigned int fin, unsigned int fout)
// C: {
// C: 	int ret = 0;
// C: 
// C: 	/* To remain consistent with previous FLLs, we expect fout to be
// C: 	 * provided in the form of the required sysclk rate, which is
// C: 	 * 2x the calculated fll out.
// C: 	 */
// C: 	if (fout)
// C: 		fout /= 2;
// C: 
// C: 	if (fll->ref_src == source && fll->ref_freq == fin &&
// C: 	    fll->fout == fout)
// C: 		return 0;
// C: 
// C: 	if (fin && fout && madera_fllhj_validate(fll, fin, fout))
// C: 		return -EINVAL;
// C: 
// C: 	fll->ref_src = source;
// C: 	fll->ref_freq = fin;
// C: 	fll->fout = fout;
// C: 
// C: 	if (fout)
// C: 		ret = madera_fllhj_enable(fll);
// C: 	else
// C: 		madera_fllhj_disable(fll);
// C: 
// C: 	return ret;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_fllhj_set_refclk);
// C: 
// C: /**
// C:  * madera_set_output_mode - Set the mode of the specified output
// C:  *
// C:  * @component: Device to configure
// C:  * @output: Output number
// C:  * @differential: True to set the output to differential mode
// C:  *
// C:  * Some systems use external analogue switches to connect more
// C:  * analogue devices to the CODEC than are supported by the device.  In
// C:  * some systems this requires changing the switched output from single
// C:  * ended to differential mode dynamically at runtime, an operation
// C:  * supported using this function.
// C:  *
// C:  * Most systems have a single static configuration and should use
// C:  * platform data instead.
// C:  */
// C: int madera_set_output_mode(struct snd_soc_component *component, int output,
// C: 			   bool differential)
// C: {
// C: 	unsigned int reg, val;
// C: 	int ret;
// C: 
// C: 	if (output < 1 || output > MADERA_MAX_OUTPUT)
// C: 		return -EINVAL;
// C: 
// C: 	reg = MADERA_OUTPUT_PATH_CONFIG_1L + (output - 1) * 8;
// C: 
// C: 	if (differential)
// C: 		val = MADERA_OUT1_MONO;
// C: 	else
// C: 		val = 0;
// C: 
// C: 	ret = snd_soc_component_update_bits(component, reg, MADERA_OUT1_MONO,
// C: 					    val);
// C: 	if (ret < 0)
// C: 		return ret;
// C: 	else
// C: 		return 0;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_set_output_mode);
// C: 
// C: static bool madera_eq_filter_unstable(bool mode, __be16 _a, __be16 _b)
// C: {
// C: 	s16 a = be16_to_cpu(_a);
// C: 	s16 b = be16_to_cpu(_b);
// C: 
// C: 	if (!mode) {
// C: 		return abs(a) >= 4096;
// C: 	} else {
// C: 		if (abs(b) >= 4096)
// C: 			return true;
// C: 
// C: 		return (abs((a << 16) / (4096 - b)) >= 4096 << 4);
// C: 	}
// C: }
// C: 
// C: int madera_eq_coeff_put(struct snd_kcontrol *kcontrol,
// C: 			struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera *madera = priv->madera;
// C: 	struct soc_bytes *params = (void *)kcontrol->private_value;
// C: 	unsigned int val;
// C: 	__be16 *data;
// C: 	int len;
// C: 	int ret;
// C: 
// C: 	len = params->num_regs * regmap_get_val_bytes(madera->regmap);
// C: 
// C: 	data = kmemdup(ucontrol->value.bytes.data, len, GFP_KERNEL | GFP_DMA);
// C: 	if (!data)
// C: 		return -ENOMEM;
// C: 
// C: 	data[0] &= cpu_to_be16(MADERA_EQ1_B1_MODE);
// C: 
// C: 	if (madera_eq_filter_unstable(!!data[0], data[1], data[2]) ||
// C: 	    madera_eq_filter_unstable(true, data[4], data[5]) ||
// C: 	    madera_eq_filter_unstable(true, data[8], data[9]) ||
// C: 	    madera_eq_filter_unstable(true, data[12], data[13]) ||
// C: 	    madera_eq_filter_unstable(false, data[16], data[17])) {
// C: 		dev_err(madera->dev, "Rejecting unstable EQ coefficients\n");
// C: 		ret = -EINVAL;
// C: 		goto out;
// C: 	}
// C: 
// C: 	ret = regmap_read(madera->regmap, params->base, &val);
// C: 	if (ret != 0)
// C: 		goto out;
// C: 
// C: 	val &= ~MADERA_EQ1_B1_MODE;
// C: 	data[0] |= cpu_to_be16(val);
// C: 
// C: 	ret = regmap_raw_write(madera->regmap, params->base, data, len);
// C: 
// C: out:
// C: 	kfree(data);
// C: 
// C: 	return ret;
// C: }
// C: EXPORT_SYMBOL_GPL(madera_eq_coeff_put);
// C: 
// C: int madera_lhpf_coeff_put(struct snd_kcontrol *kcontrol,
// C: 			  struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
// C: 	struct madera_priv *priv = snd_soc_component_get_drvdata(component);
// C: 	struct madera *madera = priv->madera;
// C: 	__be16 *data = (__be16 *)ucontrol->value.bytes.data;
// C: 	s16 val = be16_to_cpu(*data);
// C: 
// C: 	if (abs(val) >= 4096) {
// C: 		dev_err(madera->dev, "Rejecting unstable LHPF coefficients\n");
// C: 		return -EINVAL;
// C: 	}
// C: 
// C: 	return snd_soc_bytes_put(kcontrol, ucontrol);
// C: }
// C: EXPORT_SYMBOL_GPL(madera_lhpf_coeff_put);
// C: 
// C: MODULE_SOFTDEP("pre: madera");
// C: MODULE_DESCRIPTION("ASoC Cirrus Logic Madera codec support");
// C: MODULE_AUTHOR("Charles Keepax <ckeepax@opensource.cirrus.com>");
// C: MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
// C: MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
