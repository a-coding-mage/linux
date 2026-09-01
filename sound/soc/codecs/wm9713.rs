// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * wm9713.c  --  ALSA Soc WM9713 codec support
 *
 * Copyright 2006-10 Wolfson Microelectronics PLC.
 * Author: Liam Girdwood <lrg@slimlogic.co.uk>
 *
 *  Features:-
 *
 *   o Support for AC97 Codec, Voice DAC and Aux DAC
 *   o Support for DAPM
 */

// C includes translated as external dependencies:
// linux/cleanup.h, linux/init.h, linux/slab.h, linux/mfd/wm97xx.h,
// linux/module.h, linux/device.h, linux/regmap.h, sound/*, and "wm9713.h".

pub const WM9713_VENDOR_ID: u32 = 0x574d4c13;
pub const WM9713_VENDOR_ID_MASK: u32 = 0xffffffff;

#[repr(C)]
pub struct wm9713_priv {
    pub ac97: *mut snd_ac97,
    pub pll_in: u32, /* PLL input frequency */
    pub hp_mixer: [c_uint; 2],
    pub lock: mutex,
    pub mfd_pdata: *mut wm97xx_platform_data,
}

pub const HPL_MIXER: c_uint = 0;
pub const HPR_MIXER: c_uint = 1;

static wm9713_mic_mixer: [&str; 4] = ["Stereo", "Mic 1", "Mic 2", "Mute"];
static wm9713_rec_mux: [&str; 4] = ["Stereo", "Left", "Right", "Mute"];
static wm9713_rec_src: [&str; 8] = [
    "Mic 1", "Mic 2", "Line", "Mono In", "Headphone", "Speaker", "Mono Out", "Zh",
];
static wm9713_rec_gain: [&str; 2] = ["+1.5dB Steps", "+0.75dB Steps"];
static wm9713_alc_select: [&str; 4] = ["None", "Left", "Right", "Stereo"];
static wm9713_mono_pga: [&str; 4] = ["Vmid", "Zh", "Mono", "Inv"];
static wm9713_spk_pga: [&str; 8] = [
    "Vmid", "Zh", "Headphone", "Speaker", "Inv", "Headphone Vmid", "Speaker Vmid",
    "Inv Vmid",
];
static wm9713_hp_pga: [&str; 4] = ["Vmid", "Zh", "Headphone", "Headphone Vmid"];
static wm9713_out3_pga: [&str; 4] = ["Vmid", "Zh", "Inv 1", "Inv 1 Vmid"];
static wm9713_out4_pga: [&str; 4] = ["Vmid", "Zh", "Inv 2", "Inv 2 Vmid"];
static wm9713_dac_inv: [&str; 8] = [
    "Off", "Mono", "Speaker", "Left Headphone", "Right Headphone", "Headphone Mono", "NC",
    "Vmid",
];
static wm9713_bass: [&str; 2] = ["Linear Control", "Adaptive Boost"];
static wm9713_ng_type: [&str; 2] = ["Constant Gain", "Mute"];
static wm9713_mic_select: [&str; 3] = ["Mic 1", "Mic 2 A", "Mic 2 B"];
static wm9713_micb_select: [&str; 2] = ["MPB", "MPA"];

static wm9713_enum: [soc_enum; 20] = [
    SOC_ENUM_SINGLE!(AC97_LINE, 3, 4, wm9713_mic_mixer), /* record mic mixer 0 */
    SOC_ENUM_SINGLE!(AC97_VIDEO, 14, 4, wm9713_rec_mux), /* record mux hp 1 */
    SOC_ENUM_SINGLE!(AC97_VIDEO, 9, 4, wm9713_rec_mux),  /* record mux mono 2 */
    SOC_ENUM_SINGLE!(AC97_VIDEO, 3, 8, wm9713_rec_src),  /* record mux left 3 */
    SOC_ENUM_SINGLE!(AC97_VIDEO, 0, 8, wm9713_rec_src),  /* record mux right 4*/
    SOC_ENUM_DOUBLE!(AC97_CD, 14, 6, 2, wm9713_rec_gain), /* record step size 5 */
    SOC_ENUM_SINGLE!(AC97_PCI_SVID, 14, 4, wm9713_alc_select), /* alc source select 6*/
    SOC_ENUM_SINGLE!(AC97_REC_GAIN, 14, 4, wm9713_mono_pga), /* mono input select 7 */
    SOC_ENUM_SINGLE!(AC97_REC_GAIN, 11, 8, wm9713_spk_pga), /* speaker left input select 8 */
    SOC_ENUM_SINGLE!(AC97_REC_GAIN, 8, 8, wm9713_spk_pga), /* speaker right input select 9 */
    SOC_ENUM_SINGLE!(AC97_REC_GAIN, 6, 3, wm9713_hp_pga), /* headphone left input 10 */
    SOC_ENUM_SINGLE!(AC97_REC_GAIN, 4, 3, wm9713_hp_pga), /* headphone right input 11 */
    SOC_ENUM_SINGLE!(AC97_REC_GAIN, 2, 4, wm9713_out3_pga), /* out 3 source 12 */
    SOC_ENUM_SINGLE!(AC97_REC_GAIN, 0, 4, wm9713_out4_pga), /* out 4 source 13 */
    SOC_ENUM_SINGLE!(AC97_REC_GAIN_MIC, 13, 8, wm9713_dac_inv), /* dac invert 1 14 */
    SOC_ENUM_SINGLE!(AC97_REC_GAIN_MIC, 10, 8, wm9713_dac_inv), /* dac invert 2 15 */
    SOC_ENUM_SINGLE!(AC97_GENERAL_PURPOSE, 15, 2, wm9713_bass), /* bass control 16 */
    SOC_ENUM_SINGLE!(AC97_PCI_SVID, 5, 2, wm9713_ng_type), /* noise gate type 17 */
    SOC_ENUM_SINGLE!(AC97_3D_CONTROL, 12, 3, wm9713_mic_select), /* mic selection 18 */
    SOC_ENUM_SINGLE_VIRT!(2, wm9713_micb_select), /* mic selection 19 */
];

static out_tlv: TlvDbScale = DECLARE_TLV_DB_SCALE!(-4650, 150, 0);
static main_tlv: TlvDbScale = DECLARE_TLV_DB_SCALE!(-3450, 150, 0);
static misc_tlv: TlvDbScale = DECLARE_TLV_DB_SCALE!(-1500, 300, 0);
static mic_tlv: TlvDbRange = DECLARE_TLV_DB_RANGE!(
    0, 2, TLV_DB_SCALE_ITEM!(1200, 600, 0),
    3, 3, TLV_DB_SCALE_ITEM!(3000, 0, 0)
);

static wm9713_snd_ac97_controls: &[snd_kcontrol_new] = &[
    SOC_DOUBLE_TLV!("Speaker Playback Volume", AC97_MASTER, 8, 0, 31, 1, out_tlv),
    SOC_DOUBLE!("Speaker Playback Switch", AC97_MASTER, 15, 7, 1, 1),
    SOC_DOUBLE_TLV!("Headphone Playback Volume", AC97_HEADPHONE, 8, 0, 31, 1, out_tlv),
    SOC_DOUBLE!("Headphone Playback Switch", AC97_HEADPHONE, 15, 7, 1, 1),
    SOC_DOUBLE_TLV!("Line In Volume", AC97_PC_BEEP, 8, 0, 31, 1, main_tlv),
    SOC_DOUBLE_TLV!("PCM Playback Volume", AC97_PHONE, 8, 0, 31, 1, main_tlv),
    SOC_SINGLE_TLV!("Mic 1 Volume", AC97_MIC, 8, 31, 1, main_tlv),
    SOC_SINGLE_TLV!("Mic 2 Volume", AC97_MIC, 0, 31, 1, main_tlv),
    SOC_SINGLE_TLV!("Mic 1 Preamp Volume", AC97_3D_CONTROL, 10, 3, 0, mic_tlv),
    SOC_SINGLE_TLV!("Mic 2 Preamp Volume", AC97_3D_CONTROL, 12, 3, 0, mic_tlv),
    SOC_SINGLE!("Mic Boost (+20dB) Switch", AC97_LINE, 5, 1, 0),
    SOC_SINGLE!("Mic Headphone Mixer Volume", AC97_LINE, 0, 7, 1),
    SOC_SINGLE!("Capture Switch", AC97_CD, 15, 1, 1),
    SOC_ENUM!("Capture Volume Steps", wm9713_enum[5]),
    SOC_DOUBLE!("Capture Volume", AC97_CD, 8, 0, 31, 0),
    SOC_SINGLE!("Capture ZC Switch", AC97_CD, 7, 1, 0),
    SOC_SINGLE_TLV!("Capture to Headphone Volume", AC97_VIDEO, 11, 7, 1, misc_tlv),
    SOC_SINGLE!("Capture to Mono Boost (+20dB) Switch", AC97_VIDEO, 8, 1, 0),
    SOC_SINGLE!("Capture ADC Boost (+20dB) Switch", AC97_VIDEO, 6, 1, 0),
    SOC_SINGLE!("ALC Target Volume", AC97_CODEC_CLASS_REV, 12, 15, 0),
    SOC_SINGLE!("ALC Hold Time", AC97_CODEC_CLASS_REV, 8, 15, 0),
    SOC_SINGLE!("ALC Decay Time", AC97_CODEC_CLASS_REV, 4, 15, 0),
    SOC_SINGLE!("ALC Attack Time", AC97_CODEC_CLASS_REV, 0, 15, 0),
    SOC_ENUM!("ALC Function", wm9713_enum[6]),
    SOC_SINGLE!("ALC Max Volume", AC97_PCI_SVID, 11, 7, 0),
    SOC_SINGLE!("ALC ZC Timeout", AC97_PCI_SVID, 9, 3, 0),
    SOC_SINGLE!("ALC ZC Switch", AC97_PCI_SVID, 8, 1, 0),
    SOC_SINGLE!("ALC NG Switch", AC97_PCI_SVID, 7, 1, 0),
    SOC_ENUM!("ALC NG Type", wm9713_enum[17]),
    SOC_SINGLE!("ALC NG Threshold", AC97_PCI_SVID, 0, 31, 0),
    SOC_DOUBLE!("Speaker Playback ZC Switch", AC97_MASTER, 14, 6, 1, 0),
    SOC_DOUBLE!("Headphone Playback ZC Switch", AC97_HEADPHONE, 14, 6, 1, 0),
    SOC_SINGLE!("Out4 Playback Switch", AC97_MASTER_MONO, 15, 1, 1),
    SOC_SINGLE!("Out4 Playback ZC Switch", AC97_MASTER_MONO, 14, 1, 0),
    SOC_SINGLE_TLV!("Out4 Playback Volume", AC97_MASTER_MONO, 8, 31, 1, out_tlv),
    SOC_SINGLE!("Out3 Playback Switch", AC97_MASTER_MONO, 7, 1, 1),
    SOC_SINGLE!("Out3 Playback ZC Switch", AC97_MASTER_MONO, 6, 1, 0),
    SOC_SINGLE_TLV!("Out3 Playback Volume", AC97_MASTER_MONO, 0, 31, 1, out_tlv),
    SOC_SINGLE_TLV!("Mono Capture Volume", AC97_MASTER_TONE, 8, 31, 1, main_tlv),
    SOC_SINGLE!("Mono Playback Switch", AC97_MASTER_TONE, 7, 1, 1),
    SOC_SINGLE!("Mono Playback ZC Switch", AC97_MASTER_TONE, 6, 1, 0),
    SOC_SINGLE_TLV!("Mono Playback Volume", AC97_MASTER_TONE, 0, 31, 1, out_tlv),
    SOC_SINGLE_TLV!("Headphone Mixer Beep Playback Volume", AC97_AUX, 12, 7, 1, misc_tlv),
    SOC_SINGLE_TLV!("Speaker Mixer Beep Playback Volume", AC97_AUX, 8, 7, 1, misc_tlv),
    SOC_SINGLE_TLV!("Mono Mixer Beep Playback Volume", AC97_AUX, 4, 7, 1, misc_tlv),
    SOC_SINGLE_TLV!("Voice Playback Headphone Volume", AC97_PCM, 12, 7, 1, misc_tlv),
    SOC_SINGLE!("Voice Playback Master Volume", AC97_PCM, 8, 7, 1),
    SOC_SINGLE!("Voice Playback Mono Volume", AC97_PCM, 4, 7, 1),
    SOC_SINGLE_TLV!("Headphone Mixer Aux Playback Volume", AC97_REC_SEL, 12, 7, 1, misc_tlv),
    SOC_SINGLE_TLV!("Speaker Mixer Voice Playback Volume", AC97_PCM, 8, 7, 1, misc_tlv),
    SOC_SINGLE_TLV!("Speaker Mixer Aux Playback Volume", AC97_REC_SEL, 8, 7, 1, misc_tlv),
    SOC_SINGLE_TLV!("Mono Mixer Voice Playback Volume", AC97_PCM, 4, 7, 1, misc_tlv),
    SOC_SINGLE_TLV!("Mono Mixer Aux Playback Volume", AC97_REC_SEL, 4, 7, 1, misc_tlv),
    SOC_SINGLE!("Aux Playback Headphone Volume", AC97_REC_SEL, 12, 7, 1),
    SOC_SINGLE!("Aux Playback Master Volume", AC97_REC_SEL, 8, 7, 1),
    SOC_ENUM!("Bass Control", wm9713_enum[16]),
    SOC_SINGLE!("Bass Cut-off Switch", AC97_GENERAL_PURPOSE, 12, 1, 1),
    SOC_SINGLE!("Tone Cut-off Switch", AC97_GENERAL_PURPOSE, 4, 1, 1),
    SOC_SINGLE!("Playback Attenuate (-6dB) Switch", AC97_GENERAL_PURPOSE, 6, 1, 0),
    SOC_SINGLE!("Bass Volume", AC97_GENERAL_PURPOSE, 8, 15, 1),
    SOC_SINGLE!("Tone Volume", AC97_GENERAL_PURPOSE, 0, 15, 1),
    SOC_SINGLE!("3D Upper Cut-off Switch", AC97_REC_GAIN_MIC, 5, 1, 0),
    SOC_SINGLE!("3D Lower Cut-off Switch", AC97_REC_GAIN_MIC, 4, 1, 0),
    SOC_SINGLE!("3D Depth", AC97_REC_GAIN_MIC, 0, 15, 1),
];

unsafe extern "C" fn wm9713_voice_shutdown(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    if WARN_ON(event != SND_SOC_DAPM_PRE_PMD) {
        return -EINVAL;
    }

    /* Gracefully shut down the voice interface. */
    snd_soc_component_update_bits(component, AC97_HANDSET_RATE, 0x0f00, 0x0200);
    schedule_timeout_interruptible(msecs_to_jiffies(1));
    snd_soc_component_update_bits(component, AC97_HANDSET_RATE, 0x0f00, 0x0f00);
    snd_soc_component_update_bits(component, AC97_EXTENDED_MID, 0x1000, 0x1000);

    0
}

static wm9713_mixer_mute_regs: [c_uint; 6] = [
    AC97_PC_BEEP,
    AC97_MASTER_TONE,
    AC97_PHONE,
    AC97_REC_SEL,
    AC97_PCM,
    AC97_AUX,
];

/* We have to create a fake left and right HP mixers because
 * the codec only has a single control that is shared by both channels.
 * This makes it impossible to determine the audio path using the current
 * register map, thus we add a new (virtual) register to help determine the
 * audio route within the device.
 */
unsafe extern "C" fn wm9713_hp_mixer_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let component = snd_soc_dapm_to_component(dapm);
    let wm9713 = snd_soc_component_get_drvdata(component) as *mut wm9713_priv;
    let val: c_uint = (*ucontrol).value.integer.value[0] as c_uint;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let mut update: snd_soc_dapm_update = core::mem::zeroed();

    let mixer: c_uint = (*mc).shift >> 8;
    let shift: c_uint = (*mc).shift & 0xff;
    let mask: c_uint = 1 << shift;

    mutex_lock(&mut (*wm9713).lock);
    let old = (*wm9713).hp_mixer[mixer as usize];
    if (*ucontrol).value.integer.value[0] != 0 {
        (*wm9713).hp_mixer[mixer as usize] |= mask;
    } else {
        (*wm9713).hp_mixer[mixer as usize] &= !mask;
    }

    let change = old != (*wm9713).hp_mixer[mixer as usize];
    if change {
        update.kcontrol = kcontrol;
        update.reg = wm9713_mixer_mute_regs[shift as usize];
        update.mask = 0x8000;
        if ((*wm9713).hp_mixer[0] & mask) != 0 || ((*wm9713).hp_mixer[1] & mask) != 0 {
            update.val = 0x0;
        } else {
            update.val = 0x8000;
        }

        snd_soc_dapm_mixer_update_power(dapm, kcontrol, val, &mut update);
    }
    mutex_unlock(&mut (*wm9713).lock);

    change as c_int
}

unsafe extern "C" fn wm9713_hp_mixer_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let component = snd_soc_dapm_to_component(dapm);
    let wm9713 = snd_soc_component_get_drvdata(component) as *mut wm9713_priv;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;

    let mixer: c_uint = (*mc).shift >> 8;
    let shift: c_uint = (*mc).shift & 0xff;

    (*ucontrol).value.integer.value[0] =
        (((*wm9713).hp_mixer[mixer as usize] >> shift) & 1) as c_long;

    0
}

macro_rules! WM9713_HP_MIXER_CTRL {
    ($xname:expr, $xmixer:expr, $xshift:expr) => {
        SOC_DOUBLE_EXT!($xname, SND_SOC_NOPM, $xshift, $xmixer, 1, 0,
                       wm9713_hp_mixer_get, wm9713_hp_mixer_put)
    };
}

/* Left Headphone Mixers */
static wm9713_hpl_mixer_controls: [snd_kcontrol_new; 6] = [
    WM9713_HP_MIXER_CTRL!("Beep Playback Switch", HPL_MIXER, 5),
    WM9713_HP_MIXER_CTRL!("Voice Playback Switch", HPL_MIXER, 4),
    WM9713_HP_MIXER_CTRL!("Aux Playback Switch", HPL_MIXER, 3),
    WM9713_HP_MIXER_CTRL!("PCM Playback Switch", HPL_MIXER, 2),
    WM9713_HP_MIXER_CTRL!("MonoIn Playback Switch", HPL_MIXER, 1),
    WM9713_HP_MIXER_CTRL!("Bypass Playback Switch", HPL_MIXER, 0),
];

/* Right Headphone Mixers */
static wm9713_hpr_mixer_controls: [snd_kcontrol_new; 6] = [
    WM9713_HP_MIXER_CTRL!("Beep Playback Switch", HPR_MIXER, 5),
    WM9713_HP_MIXER_CTRL!("Voice Playback Switch", HPR_MIXER, 4),
    WM9713_HP_MIXER_CTRL!("Aux Playback Switch", HPR_MIXER, 3),
    WM9713_HP_MIXER_CTRL!("PCM Playback Switch", HPR_MIXER, 2),
    WM9713_HP_MIXER_CTRL!("MonoIn Playback Switch", HPR_MIXER, 1),
    WM9713_HP_MIXER_CTRL!("Bypass Playback Switch", HPR_MIXER, 0),
];

static wm9713_hp_rec_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm9713_enum[1]);
static wm9713_hp_mic_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm9713_enum[0]);

static wm9713_speaker_mixer_controls: [snd_kcontrol_new; 6] = [
    SOC_DAPM_SINGLE!("Beep Playback Switch", AC97_AUX, 11, 1, 1),
    SOC_DAPM_SINGLE!("Voice Playback Switch", AC97_PCM, 11, 1, 1),
    SOC_DAPM_SINGLE!("Aux Playback Switch", AC97_REC_SEL, 11, 1, 1),
    SOC_DAPM_SINGLE!("PCM Playback Switch", AC97_PHONE, 14, 1, 1),
    SOC_DAPM_SINGLE!("MonoIn Playback Switch", AC97_MASTER_TONE, 14, 1, 1),
    SOC_DAPM_SINGLE!("Bypass Playback Switch", AC97_PC_BEEP, 14, 1, 1),
];

static wm9713_mono_mixer_controls: [snd_kcontrol_new; 8] = [
    SOC_DAPM_SINGLE!("Beep Playback Switch", AC97_AUX, 7, 1, 1),
    SOC_DAPM_SINGLE!("Voice Playback Switch", AC97_PCM, 7, 1, 1),
    SOC_DAPM_SINGLE!("Aux Playback Switch", AC97_REC_SEL, 7, 1, 1),
    SOC_DAPM_SINGLE!("PCM Playback Switch", AC97_PHONE, 13, 1, 1),
    SOC_DAPM_SINGLE!("MonoIn Playback Switch", AC97_MASTER_TONE, 13, 1, 1),
    SOC_DAPM_SINGLE!("Bypass Playback Switch", AC97_PC_BEEP, 13, 1, 1),
    SOC_DAPM_SINGLE!("Mic 1 Sidetone Switch", AC97_LINE, 7, 1, 1),
    SOC_DAPM_SINGLE!("Mic 2 Sidetone Switch", AC97_LINE, 6, 1, 1),
];

static wm9713_mono_mic_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm9713_enum[2]);
static wm9713_mono_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm9713_enum[7]);
static wm9713_hp_spkl_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm9713_enum[8]);
static wm9713_hp_spkr_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm9713_enum[9]);
static wm9713_hpl_out_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm9713_enum[10]);
static wm9713_hpr_out_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm9713_enum[11]);
static wm9713_out3_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm9713_enum[12]);
static wm9713_out4_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm9713_enum[13]);
static wm9713_dac_inv1_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm9713_enum[14]);
static wm9713_dac_inv2_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm9713_enum[15]);
static wm9713_rec_srcl_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm9713_enum[3]);
static wm9713_rec_srcr_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm9713_enum[4]);
static wm9713_mic_sel_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm9713_enum[18]);
static wm9713_micb_sel_mux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", wm9713_enum[19]);

static wm9713_dapm_widgets: &[snd_soc_dapm_widget] = &[
    SND_SOC_DAPM_MUX!("Capture Headphone Mux", SND_SOC_NOPM, 0, 0, &wm9713_hp_rec_mux_controls),
    SND_SOC_DAPM_MUX!("Sidetone Mux", SND_SOC_NOPM, 0, 0, &wm9713_hp_mic_mux_controls),
    SND_SOC_DAPM_MUX!("Capture Mono Mux", SND_SOC_NOPM, 0, 0, &wm9713_mono_mic_mux_controls),
    SND_SOC_DAPM_MUX!("Mono Out Mux", SND_SOC_NOPM, 0, 0, &wm9713_mono_mux_controls),
    SND_SOC_DAPM_MUX!("Left Speaker Out Mux", SND_SOC_NOPM, 0, 0, &wm9713_hp_spkl_mux_controls),
    SND_SOC_DAPM_MUX!("Right Speaker Out Mux", SND_SOC_NOPM, 0, 0, &wm9713_hp_spkr_mux_controls),
    SND_SOC_DAPM_MUX!("Left Headphone Out Mux", SND_SOC_NOPM, 0, 0, &wm9713_hpl_out_mux_controls),
    SND_SOC_DAPM_MUX!("Right Headphone Out Mux", SND_SOC_NOPM, 0, 0, &wm9713_hpr_out_mux_controls),
    SND_SOC_DAPM_MUX!("Out 3 Mux", SND_SOC_NOPM, 0, 0, &wm9713_out3_mux_controls),
    SND_SOC_DAPM_MUX!("Out 4 Mux", SND_SOC_NOPM, 0, 0, &wm9713_out4_mux_controls),
    SND_SOC_DAPM_MUX!("DAC Inv Mux 1", SND_SOC_NOPM, 0, 0, &wm9713_dac_inv1_mux_controls),
    SND_SOC_DAPM_MUX!("DAC Inv Mux 2", SND_SOC_NOPM, 0, 0, &wm9713_dac_inv2_mux_controls),
    SND_SOC_DAPM_MUX!("Left Capture Source", SND_SOC_NOPM, 0, 0, &wm9713_rec_srcl_mux_controls),
    SND_SOC_DAPM_MUX!("Right Capture Source", SND_SOC_NOPM, 0, 0, &wm9713_rec_srcr_mux_controls),
    SND_SOC_DAPM_MUX!("Mic A Source", SND_SOC_NOPM, 0, 0, &wm9713_mic_sel_mux_controls),
    SND_SOC_DAPM_MUX!("Mic B Source", SND_SOC_NOPM, 0, 0, &wm9713_micb_sel_mux_controls),
    SND_SOC_DAPM_MIXER!("Left HP Mixer", AC97_EXTENDED_MID, 3, 1, &wm9713_hpl_mixer_controls[0], ARRAY_SIZE!(wm9713_hpl_mixer_controls)),
    SND_SOC_DAPM_MIXER!("Right HP Mixer", AC97_EXTENDED_MID, 2, 1, &wm9713_hpr_mixer_controls[0], ARRAY_SIZE!(wm9713_hpr_mixer_controls)),
    SND_SOC_DAPM_MIXER!("Mono Mixer", AC97_EXTENDED_MID, 0, 1, &wm9713_mono_mixer_controls[0], ARRAY_SIZE!(wm9713_mono_mixer_controls)),
    SND_SOC_DAPM_MIXER!("Speaker Mixer", AC97_EXTENDED_MID, 1, 1, &wm9713_speaker_mixer_controls[0], ARRAY_SIZE!(wm9713_speaker_mixer_controls)),
    SND_SOC_DAPM_DAC!("Left DAC", "Left HiFi Playback", AC97_EXTENDED_MID, 7, 1),
    SND_SOC_DAPM_DAC!("Right DAC", "Right HiFi Playback", AC97_EXTENDED_MID, 6, 1),
    SND_SOC_DAPM_MIXER!("AC97 Mixer", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("HP Mixer", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("Line Mixer", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("Capture Mixer", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_DAC_E!("Voice DAC", "Voice Playback", AC97_EXTENDED_MID, 12, 1, wm9713_voice_shutdown, SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_DAC!("Aux DAC", "Aux Playback", AC97_EXTENDED_MID, 11, 1),
    SND_SOC_DAPM_PGA!("Left ADC", AC97_EXTENDED_MID, 5, 1, NULL, 0),
    SND_SOC_DAPM_PGA!("Right ADC", AC97_EXTENDED_MID, 4, 1, NULL, 0),
    SND_SOC_DAPM_ADC!("Left HiFi ADC", "Left HiFi Capture", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_ADC!("Right HiFi ADC", "Right HiFi Capture", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_ADC!("Left Voice ADC", "Left Voice Capture", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_ADC!("Right Voice ADC", "Right Voice Capture", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_PGA!("Left Headphone", AC97_EXTENDED_MSTATUS, 10, 1, NULL, 0),
    SND_SOC_DAPM_PGA!("Right Headphone", AC97_EXTENDED_MSTATUS, 9, 1, NULL, 0),
    SND_SOC_DAPM_PGA!("Left Speaker", AC97_EXTENDED_MSTATUS, 8, 1, NULL, 0),
    SND_SOC_DAPM_PGA!("Right Speaker", AC97_EXTENDED_MSTATUS, 7, 1, NULL, 0),
    SND_SOC_DAPM_PGA!("Out 3", AC97_EXTENDED_MSTATUS, 11, 1, NULL, 0),
    SND_SOC_DAPM_PGA!("Out 4", AC97_EXTENDED_MSTATUS, 12, 1, NULL, 0),
    SND_SOC_DAPM_PGA!("Mono Out", AC97_EXTENDED_MSTATUS, 13, 1, NULL, 0),
    SND_SOC_DAPM_PGA!("Left Line In", AC97_EXTENDED_MSTATUS, 6, 1, NULL, 0),
    SND_SOC_DAPM_PGA!("Right Line In", AC97_EXTENDED_MSTATUS, 5, 1, NULL, 0),
    SND_SOC_DAPM_PGA!("Mono In", AC97_EXTENDED_MSTATUS, 4, 1, NULL, 0),
    SND_SOC_DAPM_PGA!("Mic A PGA", AC97_EXTENDED_MSTATUS, 3, 1, NULL, 0),
    SND_SOC_DAPM_PGA!("Mic B PGA", AC97_EXTENDED_MSTATUS, 2, 1, NULL, 0),
    SND_SOC_DAPM_PGA!("Mic A Pre Amp", AC97_EXTENDED_MSTATUS, 1, 1, NULL, 0),
    SND_SOC_DAPM_PGA!("Mic B Pre Amp", AC97_EXTENDED_MSTATUS, 0, 1, NULL, 0),
    SND_SOC_DAPM_MICBIAS!("Mic Bias", AC97_EXTENDED_MSTATUS, 14, 1),
    SND_SOC_DAPM_OUTPUT!("MONO"), SND_SOC_DAPM_OUTPUT!("HPL"), SND_SOC_DAPM_OUTPUT!("HPR"),
    SND_SOC_DAPM_OUTPUT!("SPKL"), SND_SOC_DAPM_OUTPUT!("SPKR"), SND_SOC_DAPM_OUTPUT!("OUT3"),
    SND_SOC_DAPM_OUTPUT!("OUT4"), SND_SOC_DAPM_INPUT!("LINEL"), SND_SOC_DAPM_INPUT!("LINER"),
    SND_SOC_DAPM_INPUT!("MONOIN"), SND_SOC_DAPM_INPUT!("PCBEEP"), SND_SOC_DAPM_INPUT!("MIC1"),
    SND_SOC_DAPM_INPUT!("MIC2A"), SND_SOC_DAPM_INPUT!("MIC2B"), SND_SOC_DAPM_VMID!("VMID"),
];

static wm9713_audio_map: &[snd_soc_dapm_route] = &[
    /* left HP mixer */
    snd_soc_dapm_route { sink: "Left HP Mixer", control: "Beep Playback Switch", source: "PCBEEP" },
    snd_soc_dapm_route { sink: "Left HP Mixer", control: "Voice Playback Switch", source: "Voice DAC" },
    snd_soc_dapm_route { sink: "Left HP Mixer", control: "Aux Playback Switch", source: "Aux DAC" },
    snd_soc_dapm_route { sink: "Left HP Mixer", control: "Bypass Playback Switch", source: "Left Line In" },
    snd_soc_dapm_route { sink: "Left HP Mixer", control: "PCM Playback Switch", source: "Left DAC" },
    snd_soc_dapm_route { sink: "Left HP Mixer", control: "MonoIn Playback Switch", source: "Mono In" },
    snd_soc_dapm_route { sink: "Left HP Mixer", control: NULL, source: "Capture Headphone Mux" },
    /* right HP mixer */
    snd_soc_dapm_route { sink: "Right HP Mixer", control: "Beep Playback Switch", source: "PCBEEP" },
    snd_soc_dapm_route { sink: "Right HP Mixer", control: "Voice Playback Switch", source: "Voice DAC" },
    snd_soc_dapm_route { sink: "Right HP Mixer", control: "Aux Playback Switch", source: "Aux DAC" },
    snd_soc_dapm_route { sink: "Right HP Mixer", control: "Bypass Playback Switch", source: "Right Line In" },
    snd_soc_dapm_route { sink: "Right HP Mixer", control: "PCM Playback Switch", source: "Right DAC" },
    snd_soc_dapm_route { sink: "Right HP Mixer", control: "MonoIn Playback Switch", source: "Mono In" },
    snd_soc_dapm_route { sink: "Right HP Mixer", control: NULL, source: "Capture Headphone Mux" },
    /* virtual mixer - mixes left & right channels for spk and mono */
    snd_soc_dapm_route { sink: "AC97 Mixer", control: NULL, source: "Left DAC" },
    snd_soc_dapm_route { sink: "AC97 Mixer", control: NULL, source: "Right DAC" },
    snd_soc_dapm_route { sink: "Line Mixer", control: NULL, source: "Right Line In" },
    snd_soc_dapm_route { sink: "Line Mixer", control: NULL, source: "Left Line In" },
    snd_soc_dapm_route { sink: "HP Mixer", control: NULL, source: "Left HP Mixer" },
    snd_soc_dapm_route { sink: "HP Mixer", control: NULL, source: "Right HP Mixer" },
    snd_soc_dapm_route { sink: "Capture Mixer", control: NULL, source: "Left Capture Source" },
    snd_soc_dapm_route { sink: "Capture Mixer", control: NULL, source: "Right Capture Source" },
    /* speaker mixer */
    snd_soc_dapm_route { sink: "Speaker Mixer", control: "Beep Playback Switch", source: "PCBEEP" },
    snd_soc_dapm_route { sink: "Speaker Mixer", control: "Voice Playback Switch", source: "Voice DAC" },
    snd_soc_dapm_route { sink: "Speaker Mixer", control: "Aux Playback Switch", source: "Aux DAC" },
    snd_soc_dapm_route { sink: "Speaker Mixer", control: "Bypass Playback Switch", source: "Line Mixer" },
    snd_soc_dapm_route { sink: "Speaker Mixer", control: "PCM Playback Switch", source: "AC97 Mixer" },
    snd_soc_dapm_route { sink: "Speaker Mixer", control: "MonoIn Playback Switch", source: "Mono In" },
    /* mono mixer */
    snd_soc_dapm_route { sink: "Mono Mixer", control: "Beep Playback Switch", source: "PCBEEP" },
    snd_soc_dapm_route { sink: "Mono Mixer", control: "Voice Playback Switch", source: "Voice DAC" },
    snd_soc_dapm_route { sink: "Mono Mixer", control: "Aux Playback Switch", source: "Aux DAC" },
    snd_soc_dapm_route { sink: "Mono Mixer", control: "Bypass Playback Switch", source: "Line Mixer" },
    snd_soc_dapm_route { sink: "Mono Mixer", control: "PCM Playback Switch", source: "AC97 Mixer" },
    snd_soc_dapm_route { sink: "Mono Mixer", control: "Mic 1 Sidetone Switch", source: "Mic A PGA" },
    snd_soc_dapm_route { sink: "Mono Mixer", control: "Mic 2 Sidetone Switch", source: "Mic B PGA" },
    snd_soc_dapm_route { sink: "Mono Mixer", control: NULL, source: "Capture Mono Mux" },
    /* DAC inv mux 1 */
    snd_soc_dapm_route { sink: "DAC Inv Mux 1", control: "Mono", source: "Mono Mixer" },
    snd_soc_dapm_route { sink: "DAC Inv Mux 1", control: "Speaker", source: "Speaker Mixer" },
    snd_soc_dapm_route { sink: "DAC Inv Mux 1", control: "Left Headphone", source: "Left HP Mixer" },
    snd_soc_dapm_route { sink: "DAC Inv Mux 1", control: "Right Headphone", source: "Right HP Mixer" },
    snd_soc_dapm_route { sink: "DAC Inv Mux 1", control: "Headphone Mono", source: "HP Mixer" },
    /* DAC inv mux 2 */
    snd_soc_dapm_route { sink: "DAC Inv Mux 2", control: "Mono", source: "Mono Mixer" },
    snd_soc_dapm_route { sink: "DAC Inv Mux 2", control: "Speaker", source: "Speaker Mixer" },
    snd_soc_dapm_route { sink: "DAC Inv Mux 2", control: "Left Headphone", source: "Left HP Mixer" },
    snd_soc_dapm_route { sink: "DAC Inv Mux 2", control: "Right Headphone", source: "Right HP Mixer" },
    snd_soc_dapm_route { sink: "DAC Inv Mux 2", control: "Headphone Mono", source: "HP Mixer" },
    snd_soc_dapm_route { sink: "Left Headphone Out Mux", control: "Headphone", source: "Left HP Mixer" },
    snd_soc_dapm_route { sink: "Right Headphone Out Mux", control: "Headphone", source: "Right HP Mixer" },
    snd_soc_dapm_route { sink: "Left Speaker Out Mux", control: "Headphone", source: "Left HP Mixer" },
    snd_soc_dapm_route { sink: "Left Speaker Out Mux", control: "Speaker", source: "Speaker Mixer" },
    snd_soc_dapm_route { sink: "Left Speaker Out Mux", control: "Inv", source: "DAC Inv Mux 1" },
    snd_soc_dapm_route { sink: "Right Speaker Out Mux", control: "Headphone", source: "Right HP Mixer" },
    snd_soc_dapm_route { sink: "Right Speaker Out Mux", control: "Speaker", source: "Speaker Mixer" },
    snd_soc_dapm_route { sink: "Right Speaker Out Mux", control: "Inv", source: "DAC Inv Mux 2" },
    snd_soc_dapm_route { sink: "Mono Out Mux", control: "Mono", source: "Mono Mixer" },
    snd_soc_dapm_route { sink: "Mono Out Mux", control: "Inv", source: "DAC Inv Mux 1" },
    snd_soc_dapm_route { sink: "Out 3 Mux", control: "Inv 1", source: "DAC Inv Mux 1" },
    snd_soc_dapm_route { sink: "Out 4 Mux", control: "Inv 2", source: "DAC Inv Mux 2" },
    snd_soc_dapm_route { sink: "HPL", control: NULL, source: "Left Headphone" },
    snd_soc_dapm_route { sink: "Left Headphone", control: NULL, source: "Left Headphone Out Mux" },
    snd_soc_dapm_route { sink: "HPR", control: NULL, source: "Right Headphone" },
    snd_soc_dapm_route { sink: "Right Headphone", control: NULL, source: "Right Headphone Out Mux" },
    snd_soc_dapm_route { sink: "OUT3", control: NULL, source: "Out 3" },
    snd_soc_dapm_route { sink: "Out 3", control: NULL, source: "Out 3 Mux" },
    snd_soc_dapm_route { sink: "OUT4", control: NULL, source: "Out 4" },
    snd_soc_dapm_route { sink: "Out 4", control: NULL, source: "Out 4 Mux" },
    snd_soc_dapm_route { sink: "SPKL", control: NULL, source: "Left Speaker" },
    snd_soc_dapm_route { sink: "Left Speaker", control: NULL, source: "Left Speaker Out Mux" },
    snd_soc_dapm_route { sink: "SPKR", control: NULL, source: "Right Speaker" },
    snd_soc_dapm_route { sink: "Right Speaker", control: NULL, source: "Right Speaker Out Mux" },
    snd_soc_dapm_route { sink: "MONO", control: NULL, source: "Mono Out" },
    snd_soc_dapm_route { sink: "Mono Out", control: NULL, source: "Mono Out Mux" },
    snd_soc_dapm_route { sink: "Left Line In", control: NULL, source: "LINEL" },
    snd_soc_dapm_route { sink: "Right Line In", control: NULL, source: "LINER" },
    snd_soc_dapm_route { sink: "Mono In", control: NULL, source: "MONOIN" },
    snd_soc_dapm_route { sink: "Mic A PGA", control: NULL, source: "Mic A Pre Amp" },
    snd_soc_dapm_route { sink: "Mic B PGA", control: NULL, source: "Mic B Pre Amp" },
    snd_soc_dapm_route { sink: "Left Capture Source", control: "Mic 1", source: "Mic A Pre Amp" },
    snd_soc_dapm_route { sink: "Left Capture Source", control: "Mic 2", source: "Mic B Pre Amp" },
    snd_soc_dapm_route { sink: "Left Capture Source", control: "Line", source: "LINEL" },
    snd_soc_dapm_route { sink: "Left Capture Source", control: "Mono In", source: "MONOIN" },
    snd_soc_dapm_route { sink: "Left Capture Source", control: "Headphone", source: "Left HP Mixer" },
    snd_soc_dapm_route { sink: "Left Capture Source", control: "Speaker", source: "Speaker Mixer" },
    snd_soc_dapm_route { sink: "Left Capture Source", control: "Mono Out", source: "Mono Mixer" },
    snd_soc_dapm_route { sink: "Right Capture Source", control: "Mic 1", source: "Mic A Pre Amp" },
    snd_soc_dapm_route { sink: "Right Capture Source", control: "Mic 2", source: "Mic B Pre Amp" },
    snd_soc_dapm_route { sink: "Right Capture Source", control: "Line", source: "LINER" },
    snd_soc_dapm_route { sink: "Right Capture Source", control: "Mono In", source: "MONOIN" },
    snd_soc_dapm_route { sink: "Right Capture Source", control: "Headphone", source: "Right HP Mixer" },
    snd_soc_dapm_route { sink: "Right Capture Source", control: "Speaker", source: "Speaker Mixer" },
    snd_soc_dapm_route { sink: "Right Capture Source", control: "Mono Out", source: "Mono Mixer" },
    snd_soc_dapm_route { sink: "Left ADC", control: NULL, source: "Left Capture Source" },
    snd_soc_dapm_route { sink: "Left Voice ADC", control: NULL, source: "Left ADC" },
    snd_soc_dapm_route { sink: "Left HiFi ADC", control: NULL, source: "Left ADC" },
    snd_soc_dapm_route { sink: "Right ADC", control: NULL, source: "Right Capture Source" },
    snd_soc_dapm_route { sink: "Right Voice ADC", control: NULL, source: "Right ADC" },
    snd_soc_dapm_route { sink: "Right HiFi ADC", control: NULL, source: "Right ADC" },
    snd_soc_dapm_route { sink: "Mic A Pre Amp", control: NULL, source: "Mic A Source" },
    snd_soc_dapm_route { sink: "Mic A Source", control: "Mic 1", source: "MIC1" },
    snd_soc_dapm_route { sink: "Mic A Source", control: "Mic 2 A", source: "MIC2A" },
    snd_soc_dapm_route { sink: "Mic A Source", control: "Mic 2 B", source: "Mic B Source" },
    snd_soc_dapm_route { sink: "Mic B Pre Amp", control: "MPB", source: "Mic B Source" },
    snd_soc_dapm_route { sink: "Mic B Source", control: NULL, source: "MIC2B" },
    snd_soc_dapm_route { sink: "Capture Headphone Mux", control: "Stereo", source: "Capture Mixer" },
    snd_soc_dapm_route { sink: "Capture Headphone Mux", control: "Left", source: "Left Capture Source" },
    snd_soc_dapm_route { sink: "Capture Headphone Mux", control: "Right", source: "Right Capture Source" },
    snd_soc_dapm_route { sink: "Capture Mono Mux", control: "Stereo", source: "Capture Mixer" },
    snd_soc_dapm_route { sink: "Capture Mono Mux", control: "Left", source: "Left Capture Source" },
    snd_soc_dapm_route { sink: "Capture Mono Mux", control: "Right", source: "Right Capture Source" },
];

unsafe extern "C" fn wm9713_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        AC97_RESET..=AC97_PCM_SURR_DAC_RATE
        | AC97_PCM_LR_ADC_RATE
        | AC97_CENTER_LFE_MASTER
        | AC97_SPDIF..=AC97_LINE1_LEVEL
        | AC97_GPIO_CFG..=0x5c
        | AC97_CODEC_CLASS_REV..=AC97_PCI_SID
        | 0x74..=AC97_VENDOR_ID2 => true,
        _ => false,
    }
}

unsafe extern "C" fn wm9713_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    match reg {
        AC97_VENDOR_ID1 | AC97_VENDOR_ID2 => false,
        _ => wm9713_readable_reg(dev, reg),
    }
}

static wm9713_reg_defaults: [reg_default; 47] = [
    reg_default { reg: 0x02, def: 0x8080 }, /* Speaker Output Volume */
    reg_default { reg: 0x04, def: 0x8080 }, /* Headphone Output Volume */
    reg_default { reg: 0x06, def: 0x8080 }, /* Out3/OUT4 Volume */
    reg_default { reg: 0x08, def: 0xc880 }, /* Mono Volume */
    reg_default { reg: 0x0a, def: 0xe808 }, /* LINEIN Volume */
    reg_default { reg: 0x0c, def: 0xe808 }, /* DAC PGA Volume */
    reg_default { reg: 0x0e, def: 0x0808 }, /* MIC PGA Volume */
    reg_default { reg: 0x10, def: 0x00da }, /* MIC Routing Control */
    reg_default { reg: 0x12, def: 0x8000 }, /* Record PGA Volume */
    reg_default { reg: 0x14, def: 0xd600 }, /* Record Routing */
    reg_default { reg: 0x16, def: 0xaaa0 }, /* PCBEEP Volume */
    reg_default { reg: 0x18, def: 0xaaa0 }, /* VxDAC Volume */
    reg_default { reg: 0x1a, def: 0xaaa0 }, /* AUXDAC Volume */
    reg_default { reg: 0x1c, def: 0x0000 }, /* Output PGA Mux */
    reg_default { reg: 0x1e, def: 0x0000 }, /* DAC 3D control */
    reg_default { reg: 0x20, def: 0x0f0f }, /* DAC Tone Control*/
    reg_default { reg: 0x22, def: 0x0040 }, /* MIC Input Select & Bias */
    reg_default { reg: 0x24, def: 0x0000 }, /* Output Volume Mapping & Jack */
    reg_default { reg: 0x26, def: 0x7f00 }, /* Powerdown Ctrl/Stat*/
    reg_default { reg: 0x28, def: 0x0405 }, /* Extended Audio ID */
    reg_default { reg: 0x2a, def: 0x0410 }, /* Extended Audio Start/Ctrl */
    reg_default { reg: 0x2c, def: 0xbb80 }, /* Audio DACs Sample Rate */
    reg_default { reg: 0x2e, def: 0xbb80 }, /* AUXDAC Sample Rate */
    reg_default { reg: 0x32, def: 0xbb80 }, /* Audio ADCs Sample Rate */
    reg_default { reg: 0x36, def: 0x4523 }, /* PCM codec control */
    reg_default { reg: 0x3a, def: 0x2000 }, /* SPDIF control */
    reg_default { reg: 0x3c, def: 0xfdff }, /* Powerdown 1 */
    reg_default { reg: 0x3e, def: 0xffff }, /* Powerdown 2 */
    reg_default { reg: 0x40, def: 0x0000 }, /* General Purpose */
    reg_default { reg: 0x42, def: 0x0000 }, /* Fast Power-Up Control */
    reg_default { reg: 0x44, def: 0x0080 }, /* MCLK/PLL Control */
    reg_default { reg: 0x46, def: 0x0000 }, /* MCLK/PLL Control */
    reg_default { reg: 0x4c, def: 0xfffe }, /* GPIO Pin Configuration */
    reg_default { reg: 0x4e, def: 0xffff }, /* GPIO Pin Polarity / Type */
    reg_default { reg: 0x50, def: 0x0000 }, /* GPIO Pin Sticky */
    reg_default { reg: 0x52, def: 0x0000 }, /* GPIO Pin Wake-Up */
    /* GPIO Pin Status */
    reg_default { reg: 0x56, def: 0xfffe }, /* GPIO Pin Sharing */
    reg_default { reg: 0x58, def: 0x4000 }, /* GPIO PullUp/PullDown */
    reg_default { reg: 0x5a, def: 0x0000 }, /* Additional Functions 1 */
    reg_default { reg: 0x5c, def: 0x0000 }, /* Additional Functions 2 */
    reg_default { reg: 0x60, def: 0xb032 }, /* ALC Control */
    reg_default { reg: 0x62, def: 0x3e00 }, /* ALC / Noise Gate Control */
    reg_default { reg: 0x64, def: 0x0000 }, /* AUXDAC input control */
    reg_default { reg: 0x74, def: 0x0000 }, /* Digitiser Reg 1 */
    reg_default { reg: 0x76, def: 0x0006 }, /* Digitiser Reg 2 */
    reg_default { reg: 0x78, def: 0x0001 }, /* Digitiser Reg 3 */
    reg_default { reg: 0x7a, def: 0x0000 }, /* Digitiser Read Back */
];

static wm9713_regmap_config: regmap_config = regmap_config {
    reg_bits: 16,
    reg_stride: 2,
    val_bits: 16,
    max_register: 0x7e,
    cache_type: REGCACHE_MAPLE,
    reg_defaults: wm9713_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(wm9713_reg_defaults),
    volatile_reg: Some(regmap_ac97_default_volatile),
    readable_reg: Some(wm9713_readable_reg),
    writeable_reg: Some(wm9713_writeable_reg),
};

/* PLL divisors */
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _pll_div {
    pub divsel: u32,
    pub divctl: u32,
    pub lf: u32,
    pub n: u32,
    pub k: u32,
}

/* The size in bits of the PLL divide multiplied by 10
 * to allow rounding later */
pub const FIXED_PLL_SIZE: u64 = ((1u64 << 22) * 10);

unsafe fn pll_factors(component: *mut snd_soc_component, pll_div: *mut _pll_div, mut source: c_uint) {
    let mut kpart: u64;
    let mut k: c_uint;
    let ndiv: c_uint;
    let nmod: c_uint;
    let mut target: c_uint = 98304000;

    if source > 14400000 {
        source >>= 1;
        (*pll_div).divsel = 1;
        if source > 14400000 {
            source >>= 1;
            (*pll_div).divctl = 1;
        } else {
            (*pll_div).divctl = 0;
        }
    } else {
        (*pll_div).divsel = 0;
        (*pll_div).divctl = 0;
    }

    if source < 8192000 {
        (*pll_div).lf = 1;
        target >>= 2;
    } else {
        (*pll_div).lf = 0;
    }

    ndiv = target / source;
    if ndiv < 5 || ndiv > 12 {
        dev_warn((*component).dev, "WM9713 PLL N value %u out of recommended range!\n", ndiv);
    }

    (*pll_div).n = ndiv;
    nmod = target % source;
    kpart = FIXED_PLL_SIZE * nmod as u64;
    kpart /= source as u64;
    k = (kpart & 0xFFFFFFFF) as c_uint;

    if (k % 10) >= 5 {
        k += 5;
    }
    k /= 10;
    (*pll_div).k = k;
}

/*
 * Please note that changing the PLL input frequency may require
 * resynchronisation with the AC97 controller.
 */
unsafe extern "C" fn wm9713_set_pll(
    component: *mut snd_soc_component,
    _pll_id: c_int,
    freq_in: c_uint,
    _freq_out: c_uint,
) -> c_int {
    let wm9713 = snd_soc_component_get_drvdata(component) as *mut wm9713_priv;
    let mut reg: u16;
    let reg2: u16;
    let mut pll_div: _pll_div = core::mem::zeroed();

    if freq_in == 0 {
        snd_soc_component_update_bits(component, AC97_HANDSET_RATE, 0x0080, 0x0080);
        snd_soc_component_update_bits(component, AC97_EXTENDED_MID, 0x0200, 0x0200);
        (*wm9713).pll_in = 0;
        return 0;
    }

    pll_factors(component, &mut pll_div, freq_in);

    if pll_div.k == 0 {
        reg = ((pll_div.n << 12) | (pll_div.lf << 11) |
               (pll_div.divsel << 9) | (pll_div.divctl << 8)) as u16;
        snd_soc_component_write(component, AC97_LINE1_LEVEL, reg as c_uint);
    } else {
        reg2 = ((pll_div.n << 12) | (pll_div.lf << 11) | (1 << 10) |
                (pll_div.divsel << 9) | (pll_div.divctl << 8)) as u16;

        reg = (reg2 as u32 | (0x5 << 4) | (pll_div.k >> 20)) as u16;
        snd_soc_component_write(component, AC97_LINE1_LEVEL, reg as c_uint);
        reg = (reg2 as u32 | (0x4 << 4) | ((pll_div.k >> 16) & 0xf)) as u16;
        snd_soc_component_write(component, AC97_LINE1_LEVEL, reg as c_uint);
        reg = (reg2 as u32 | (0x3 << 4) | ((pll_div.k >> 12) & 0xf)) as u16;
        snd_soc_component_write(component, AC97_LINE1_LEVEL, reg as c_uint);
        reg = (reg2 as u32 | (0x2 << 4) | ((pll_div.k >> 8) & 0xf)) as u16;
        snd_soc_component_write(component, AC97_LINE1_LEVEL, reg as c_uint);
        reg = (reg2 as u32 | (0x1 << 4) | ((pll_div.k >> 4) & 0xf)) as u16;
        snd_soc_component_write(component, AC97_LINE1_LEVEL, reg as c_uint);
        reg = (reg2 as u32 | (0x0 << 4) | (pll_div.k & 0xf)) as u16; /* K [3:0] */
        snd_soc_component_write(component, AC97_LINE1_LEVEL, reg as c_uint);
    }

    snd_soc_component_update_bits(component, AC97_EXTENDED_MID, 0x0200, 0x0000);
    snd_soc_component_update_bits(component, AC97_HANDSET_RATE, 0x0080, 0x0000);
    (*wm9713).pll_in = freq_in;
    schedule_timeout_interruptible(msecs_to_jiffies(10));
    0
}

unsafe extern "C" fn wm9713_set_dai_pll(
    codec_dai: *mut snd_soc_dai,
    pll_id: c_int,
    _source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let component = (*codec_dai).component;
    wm9713_set_pll(component, pll_id, freq_in, freq_out)
}

/*
 * Tristate the PCM DAI lines, tristate can be disabled by calling
 * wm9713_set_dai_fmt()
 */
unsafe extern "C" fn wm9713_set_dai_tristate(codec_dai: *mut snd_soc_dai, tristate: c_int) -> c_int {
    let component = (*codec_dai).component;
    if tristate != 0 {
        snd_soc_component_update_bits(component, AC97_CENTER_LFE_MASTER, 0x6000, 0x0000);
    }
    0
}

unsafe extern "C" fn wm9713_set_dai_clkdiv(
    codec_dai: *mut snd_soc_dai,
    div_id: c_int,
    div: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    match div_id {
        WM9713_PCMCLK_DIV => snd_soc_component_update_bits(component, AC97_HANDSET_RATE, 0x0f00, div as c_uint),
        WM9713_CLKA_MULT => snd_soc_component_update_bits(component, AC97_HANDSET_RATE, 0x0002, div as c_uint),
        WM9713_CLKB_MULT => snd_soc_component_update_bits(component, AC97_HANDSET_RATE, 0x0004, div as c_uint),
        WM9713_HIFI_DIV => snd_soc_component_update_bits(component, AC97_HANDSET_RATE, 0x7000, div as c_uint),
        WM9713_PCMBCLK_DIV => snd_soc_component_update_bits(component, AC97_CENTER_LFE_MASTER, 0x0e00, div as c_uint),
        WM9713_PCMCLK_PLL_DIV => snd_soc_component_update_bits(component, AC97_LINE1_LEVEL, 0x007f, (div | 0x60) as c_uint),
        WM9713_HIFI_PLL_DIV => snd_soc_component_update_bits(component, AC97_LINE1_LEVEL, 0x007f, (div | 0x70) as c_uint),
        _ => return -EINVAL,
    };
    0
}

unsafe extern "C" fn wm9713_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut gpio: u16 = (snd_soc_component_read(component, AC97_GPIO_CFG) & 0xffc5) as u16;
    let mut reg: u16 = 0x8000;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => { reg |= 0x4000; gpio |= 0x0010; }
        SND_SOC_DAIFMT_CBP_CFC => { reg |= 0x6000; gpio |= 0x0018; }
        SND_SOC_DAIFMT_CBC_CFC => { reg |= 0x2000; gpio |= 0x001a; }
        SND_SOC_DAIFMT_CBC_CFP => { gpio |= 0x0012; }
        _ => {}
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_IB_IF => reg |= 0x00c0,
        SND_SOC_DAIFMT_IB_NF => reg |= 0x0080,
        SND_SOC_DAIFMT_NB_IF => reg |= 0x0040,
        _ => {}
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => reg |= 0x0002,
        SND_SOC_DAIFMT_RIGHT_J => {}
        SND_SOC_DAIFMT_LEFT_J => reg |= 0x0001,
        SND_SOC_DAIFMT_DSP_A => reg |= 0x0003,
        SND_SOC_DAIFMT_DSP_B => reg |= 0x0043,
        _ => {}
    }

    snd_soc_component_write(component, AC97_GPIO_CFG, gpio as c_uint);
    snd_soc_component_write(component, AC97_CENTER_LFE_MASTER, reg as c_uint);
    0
}

unsafe extern "C" fn wm9713_pcm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    match params_width(params) {
        16 => {}
        20 => snd_soc_component_update_bits(component, AC97_CENTER_LFE_MASTER, 0x000c, 0x0004),
        24 => snd_soc_component_update_bits(component, AC97_CENTER_LFE_MASTER, 0x000c, 0x0008),
        32 => snd_soc_component_update_bits(component, AC97_CENTER_LFE_MASTER, 0x000c, 0x000c),
        _ => {}
    }
    0
}

unsafe extern "C" fn ac97_hifi_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let runtime = (*substream).runtime;
    snd_soc_component_update_bits(component, AC97_EXTENDED_STATUS, 0x0001, 0x0001);
    let reg = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        AC97_PCM_FRONT_DAC_RATE
    } else {
        AC97_PCM_LR_ADC_RATE
    };
    snd_soc_component_write(component, reg, (*runtime).rate)
}

unsafe extern "C" fn ac97_aux_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let runtime = (*substream).runtime;
    snd_soc_component_update_bits(component, AC97_EXTENDED_STATUS, 0x0001, 0x0001);
    snd_soc_component_update_bits(component, AC97_PCI_SID, 0x8000, 0x8000);
    if (*substream).stream != SNDRV_PCM_STREAM_PLAYBACK {
        return -ENODEV;
    }
    snd_soc_component_write(component, AC97_PCM_SURR_DAC_RATE, (*runtime).rate)
}

pub const WM9713_RATES: c_uint = SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_11025 |
    SNDRV_PCM_RATE_22050 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;
pub const WM9713_PCM_RATES: c_uint = SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_11025 |
    SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_22050 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;
pub const WM9713_PCM_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

static wm9713_dai_ops_hifi: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(ac97_hifi_prepare),
    set_clkdiv: Some(wm9713_set_dai_clkdiv),
    set_pll: Some(wm9713_set_dai_pll),
    ..unsafe { core::mem::zeroed() }
};

static wm9713_dai_ops_aux: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(ac97_aux_prepare),
    set_clkdiv: Some(wm9713_set_dai_clkdiv),
    set_pll: Some(wm9713_set_dai_pll),
    ..unsafe { core::mem::zeroed() }
};

static wm9713_dai_ops_voice: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wm9713_pcm_hw_params),
    set_clkdiv: Some(wm9713_set_dai_clkdiv),
    set_pll: Some(wm9713_set_dai_pll),
    set_fmt: Some(wm9713_set_dai_fmt),
    set_tristate: Some(wm9713_set_dai_tristate),
    ..unsafe { core::mem::zeroed() }
};

static mut wm9713_dai: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver {
        name: "wm9713-hifi",
        playback: snd_soc_pcm_stream { stream_name: "HiFi Playback", channels_min: 1, channels_max: 2, rates: WM9713_RATES, formats: SND_SOC_STD_AC97_FMTS, ..unsafe { core::mem::zeroed() } },
        capture: snd_soc_pcm_stream { stream_name: "HiFi Capture", channels_min: 1, channels_max: 2, rates: WM9713_RATES, formats: SND_SOC_STD_AC97_FMTS, ..unsafe { core::mem::zeroed() } },
        ops: &wm9713_dai_ops_hifi,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: "wm9713-aux",
        playback: snd_soc_pcm_stream { stream_name: "Aux Playback", channels_min: 1, channels_max: 1, rates: WM9713_RATES, formats: SND_SOC_STD_AC97_FMTS, ..unsafe { core::mem::zeroed() } },
        ops: &wm9713_dai_ops_aux,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: "wm9713-voice",
        playback: snd_soc_pcm_stream { stream_name: "Voice Playback", channels_min: 1, channels_max: 1, rates: WM9713_PCM_RATES, formats: WM9713_PCM_FORMATS, ..unsafe { core::mem::zeroed() } },
        capture: snd_soc_pcm_stream { stream_name: "Voice Capture", channels_min: 1, channels_max: 2, rates: WM9713_PCM_RATES, formats: WM9713_PCM_FORMATS, ..unsafe { core::mem::zeroed() } },
        ops: &wm9713_dai_ops_voice,
        symmetric_rate: 1,
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe extern "C" fn wm9713_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    match level {
        SND_SOC_BIAS_ON => snd_soc_component_update_bits(component, AC97_EXTENDED_MID, 0xe400, 0x0000),
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            snd_soc_component_update_bits(component, AC97_EXTENDED_MID, 0xc400, 0x0000);
            snd_soc_component_write(component, AC97_POWERDOWN, 0x0000);
        }
        SND_SOC_BIAS_OFF => {
            snd_soc_component_write(component, AC97_EXTENDED_MID, 0xffff);
            snd_soc_component_write(component, AC97_EXTENDED_MSTATUS, 0xffff);
            snd_soc_component_write(component, AC97_POWERDOWN, 0xffff);
        }
    }
    0
}

unsafe extern "C" fn wm9713_soc_suspend(component: *mut snd_soc_component) -> c_int {
    snd_soc_component_update_bits(component, AC97_EXTENDED_MID, 0x7fff, 0x7fff);
    snd_soc_component_write(component, AC97_EXTENDED_MSTATUS, 0xffff);
    snd_soc_component_write(component, AC97_POWERDOWN, 0x6f00);
    snd_soc_component_write(component, AC97_POWERDOWN, 0xffff);
    0
}

unsafe extern "C" fn wm9713_soc_resume(component: *mut snd_soc_component) -> c_int {
    let wm9713 = snd_soc_component_get_drvdata(component) as *mut wm9713_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let ret = snd_ac97_reset((*wm9713).ac97, true, WM9713_VENDOR_ID, WM9713_VENDOR_ID_MASK);
    if ret < 0 {
        return ret;
    }
    snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_STANDBY);
    if (*wm9713).pll_in != 0 {
        wm9713_set_pll(component, 0, (*wm9713).pll_in, 0);
    }
    if ret == 0 {
        regcache_mark_dirty((*component).regmap);
        snd_soc_component_cache_sync(component);
    }
    ret
}

unsafe extern "C" fn wm9713_soc_probe(component: *mut snd_soc_component) -> c_int {
    let wm9713 = snd_soc_component_get_drvdata(component) as *mut wm9713_priv;
    let mut regmap: *mut regmap = NULL as *mut regmap;

    if !(*wm9713).mfd_pdata.is_null() {
        (*wm9713).ac97 = (*(*wm9713).mfd_pdata).ac97;
        regmap = (*(*wm9713).mfd_pdata).regmap;
    } else if IS_ENABLED!(CONFIG_SND_SOC_AC97_BUS) {
        (*wm9713).ac97 = snd_soc_new_ac97_component(component, WM9713_VENDOR_ID, WM9713_VENDOR_ID_MASK);
        if IS_ERR((*wm9713).ac97) {
            return PTR_ERR((*wm9713).ac97);
        }
        regmap = regmap_init_ac97((*wm9713).ac97, &wm9713_regmap_config);
        if IS_ERR(regmap) {
            snd_soc_free_ac97_component((*wm9713).ac97);
            return PTR_ERR(regmap);
        }
    } else {
        return -ENXIO;
    }

    snd_soc_component_init_regmap(component, regmap);
    snd_soc_component_update_bits(component, AC97_CD, 0x7fff, 0x0000);
    0
}

unsafe extern "C" fn wm9713_soc_remove(component: *mut snd_soc_component) {
    let wm9713 = snd_soc_component_get_drvdata(component) as *mut wm9713_priv;
    if IS_ENABLED!(CONFIG_SND_SOC_AC97_BUS) && (*wm9713).mfd_pdata.is_null() {
        snd_soc_component_exit_regmap(component);
        snd_soc_free_ac97_component((*wm9713).ac97);
    }
}

static soc_component_dev_wm9713: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm9713_soc_probe),
    remove: Some(wm9713_soc_remove),
    suspend: Some(wm9713_soc_suspend),
    resume: Some(wm9713_soc_resume),
    set_bias_level: Some(wm9713_set_bias_level),
    controls: wm9713_snd_ac97_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(wm9713_snd_ac97_controls),
    dapm_widgets: wm9713_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(wm9713_dapm_widgets),
    dapm_routes: wm9713_audio_map.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(wm9713_audio_map),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn wm9713_probe(pdev: *mut platform_device) -> c_int {
    let wm9713 = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<wm9713_priv>(), GFP_KERNEL) as *mut wm9713_priv;
    if wm9713.is_null() {
        return -ENOMEM;
    }

    mutex_init(&mut (*wm9713).lock);
    (*wm9713).mfd_pdata = dev_get_platdata(&mut (*pdev).dev) as *mut wm97xx_platform_data;
    platform_set_drvdata(pdev, wm9713 as *mut c_void);

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_component_dev_wm9713,
        wm9713_dai.as_mut_ptr(),
        ARRAY_SIZE!(wm9713_dai),
    )
}

static mut wm9713_codec_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: "wm9713-codec",
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(wm9713_probe),
    ..unsafe { core::mem::zeroed() }
};

module_platform_driver!(wm9713_codec_driver);

MODULE_DESCRIPTION!("ASoC WM9713/WM9714 driver");
MODULE_AUTHOR!("Liam Girdwood");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
