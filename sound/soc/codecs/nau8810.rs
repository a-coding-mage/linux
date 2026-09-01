// SPDX-License-Identifier: GPL-2.0-only
/*
 * nau8810.rs  --  NAU8810 ALSA Soc Audio driver
 *
 * Copyright 2016 Nuvoton Technology Corp.
 *
 * Author: David Lin <ctlin0@nuvoton.com>
 *
 * Based on WM8974.c
 */

/* Translated from C implementation source.  The original Linux, ASoC, regmap,
 * and local "nau8810.h" dependencies are expected to provide the referenced
 * types, constants, and helper macros/functions.
 */

const NAU_PLL_FREQ_MAX: u32 = 100000000;
const NAU_PLL_FREQ_MIN: u32 = 90000000;
const NAU_PLL_REF_MAX: u32 = 33000000;
const NAU_PLL_REF_MIN: u32 = 8000000;
const NAU_PLL_OPTOP_MIN: u64 = 6;

static nau8810_mclk_scaler: [c_int; 8] = [10, 15, 20, 30, 40, 60, 80, 120];

static nau8810_reg_defaults: [reg_default; 55] = [
    reg_default { reg: NAU8810_REG_POWER1, def: 0x0000 },
    reg_default { reg: NAU8810_REG_POWER2, def: 0x0000 },
    reg_default { reg: NAU8810_REG_POWER3, def: 0x0000 },
    reg_default { reg: NAU8810_REG_IFACE, def: 0x0050 },
    reg_default { reg: NAU8810_REG_COMP, def: 0x0000 },
    reg_default { reg: NAU8810_REG_CLOCK, def: 0x0140 },
    reg_default { reg: NAU8810_REG_SMPLR, def: 0x0000 },
    reg_default { reg: NAU8810_REG_DAC, def: 0x0000 },
    reg_default { reg: NAU8810_REG_DACGAIN, def: 0x00FF },
    reg_default { reg: NAU8810_REG_ADC, def: 0x0100 },
    reg_default { reg: NAU8810_REG_ADCGAIN, def: 0x00FF },
    reg_default { reg: NAU8810_REG_EQ1, def: 0x012C },
    reg_default { reg: NAU8810_REG_EQ2, def: 0x002C },
    reg_default { reg: NAU8810_REG_EQ3, def: 0x002C },
    reg_default { reg: NAU8810_REG_EQ4, def: 0x002C },
    reg_default { reg: NAU8810_REG_EQ5, def: 0x002C },
    reg_default { reg: NAU8810_REG_DACLIM1, def: 0x0032 },
    reg_default { reg: NAU8810_REG_DACLIM2, def: 0x0000 },
    reg_default { reg: NAU8810_REG_NOTCH1, def: 0x0000 },
    reg_default { reg: NAU8810_REG_NOTCH2, def: 0x0000 },
    reg_default { reg: NAU8810_REG_NOTCH3, def: 0x0000 },
    reg_default { reg: NAU8810_REG_NOTCH4, def: 0x0000 },
    reg_default { reg: NAU8810_REG_ALC1, def: 0x0038 },
    reg_default { reg: NAU8810_REG_ALC2, def: 0x000B },
    reg_default { reg: NAU8810_REG_ALC3, def: 0x0032 },
    reg_default { reg: NAU8810_REG_NOISEGATE, def: 0x0000 },
    reg_default { reg: NAU8810_REG_PLLN, def: 0x0008 },
    reg_default { reg: NAU8810_REG_PLLK1, def: 0x000C },
    reg_default { reg: NAU8810_REG_PLLK2, def: 0x0093 },
    reg_default { reg: NAU8810_REG_PLLK3, def: 0x00E9 },
    reg_default { reg: NAU8810_REG_ATTEN, def: 0x0000 },
    reg_default { reg: NAU8810_REG_INPUT_SIGNAL, def: 0x0003 },
    reg_default { reg: NAU8810_REG_PGAGAIN, def: 0x0010 },
    reg_default { reg: NAU8810_REG_ADCBOOST, def: 0x0100 },
    reg_default { reg: NAU8810_REG_OUTPUT, def: 0x0002 },
    reg_default { reg: NAU8810_REG_SPKMIX, def: 0x0001 },
    reg_default { reg: NAU8810_REG_SPKGAIN, def: 0x0039 },
    reg_default { reg: NAU8810_REG_MONOMIX, def: 0x0001 },
    reg_default { reg: NAU8810_REG_POWER4, def: 0x0000 },
    reg_default { reg: NAU8810_REG_TSLOTCTL1, def: 0x0000 },
    reg_default { reg: NAU8810_REG_TSLOTCTL2, def: 0x0020 },
    reg_default { reg: NAU8810_REG_DEVICE_REVID, def: 0x0000 },
    reg_default { reg: NAU8810_REG_I2C_DEVICEID, def: 0x001A },
    reg_default { reg: NAU8810_REG_ADDITIONID, def: 0x00CA },
    reg_default { reg: NAU8810_REG_RESERVE, def: 0x0124 },
    reg_default { reg: NAU8810_REG_OUTCTL, def: 0x0001 },
    reg_default { reg: NAU8810_REG_ALC1ENHAN1, def: 0x0010 },
    reg_default { reg: NAU8810_REG_ALC1ENHAN2, def: 0x0000 },
    reg_default { reg: NAU8810_REG_MISCCTL, def: 0x0000 },
    reg_default { reg: NAU8810_REG_OUTTIEOFF, def: 0x0000 },
    reg_default { reg: NAU8810_REG_AGCP2POUT, def: 0x0000 },
    reg_default { reg: NAU8810_REG_AGCPOUT, def: 0x0000 },
    reg_default { reg: NAU8810_REG_AMTCTL, def: 0x0000 },
    reg_default { reg: NAU8810_REG_OUTTIEOFFMAN, def: 0x0000 },
];

unsafe extern "C" fn nau8810_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        NAU8810_REG_RESET..=NAU8810_REG_SMPLR
        | NAU8810_REG_DAC..=NAU8810_REG_DACGAIN
        | NAU8810_REG_ADC..=NAU8810_REG_ADCGAIN
        | NAU8810_REG_EQ1..=NAU8810_REG_EQ5
        | NAU8810_REG_DACLIM1..=NAU8810_REG_DACLIM2
        | NAU8810_REG_NOTCH1..=NAU8810_REG_NOTCH4
        | NAU8810_REG_ALC1..=NAU8810_REG_ATTEN
        | NAU8810_REG_INPUT_SIGNAL..=NAU8810_REG_PGAGAIN
        | NAU8810_REG_ADCBOOST
        | NAU8810_REG_OUTPUT..=NAU8810_REG_SPKMIX
        | NAU8810_REG_SPKGAIN
        | NAU8810_REG_MONOMIX
        | NAU8810_REG_POWER4..=NAU8810_REG_TSLOTCTL2
        | NAU8810_REG_DEVICE_REVID..=NAU8810_REG_RESERVE
        | NAU8810_REG_OUTCTL..=NAU8810_REG_ALC1ENHAN2
        | NAU8810_REG_MISCCTL
        | NAU8810_REG_OUTTIEOFF..=NAU8810_REG_OUTTIEOFFMAN => true,
        _ => false,
    }
}

unsafe extern "C" fn nau8810_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        NAU8810_REG_RESET..=NAU8810_REG_SMPLR
        | NAU8810_REG_DAC..=NAU8810_REG_DACGAIN
        | NAU8810_REG_ADC..=NAU8810_REG_ADCGAIN
        | NAU8810_REG_EQ1..=NAU8810_REG_EQ5
        | NAU8810_REG_DACLIM1..=NAU8810_REG_DACLIM2
        | NAU8810_REG_NOTCH1..=NAU8810_REG_NOTCH4
        | NAU8810_REG_ALC1..=NAU8810_REG_ATTEN
        | NAU8810_REG_INPUT_SIGNAL..=NAU8810_REG_PGAGAIN
        | NAU8810_REG_ADCBOOST
        | NAU8810_REG_OUTPUT..=NAU8810_REG_SPKMIX
        | NAU8810_REG_SPKGAIN
        | NAU8810_REG_MONOMIX
        | NAU8810_REG_POWER4..=NAU8810_REG_TSLOTCTL2
        | NAU8810_REG_OUTCTL..=NAU8810_REG_ALC1ENHAN2
        | NAU8810_REG_MISCCTL
        | NAU8810_REG_OUTTIEOFF..=NAU8810_REG_OUTTIEOFFMAN => true,
        _ => false,
    }
}

unsafe extern "C" fn nau8810_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        NAU8810_REG_RESET | NAU8810_REG_DEVICE_REVID..=NAU8810_REG_RESERVE => true,
        _ => false,
    }
}

/* The EQ parameters get function is to get the 5 band equalizer control.
 * The regmap raw read can't work here because regmap doesn't provide
 * value format for value width of 9 bits. Therefore, the driver reads data
 * from cache and makes value format according to the endianness of
 * bytes type control element.
 */
unsafe extern "C" fn nau8810_eq_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let nau8810: *mut nau8810 = snd_soc_component_get_drvdata(component) as *mut nau8810;
    let params: *mut soc_bytes_ext = (*kcontrol).private_value as *mut soc_bytes_ext;
    let mut reg_val: c_int = 0;
    let val: *mut u16 = (*ucontrol).value.bytes.data.as_mut_ptr() as *mut u16;
    let reg: c_int = NAU8810_REG_EQ1 as c_int;

    let mut i: c_int = 0;
    while i < ((*params).max as usize / core::mem::size_of::<u16>()) as c_int {
        regmap_read((*nau8810).regmap, (reg + i) as c_uint, &mut reg_val);
        /* conversion of 16-bit integers between native CPU format
         * and big endian format
         */
        let tmp: __be16 = cpu_to_be16(reg_val as u16);
        memcpy(
            val.add(i as usize) as *mut c_void,
            &tmp as *const __be16 as *const c_void,
            core::mem::size_of_val(&tmp),
        );
        i += 1;
    }

    0
}

/* The EQ parameters put function is to make configuration of 5 band equalizer
 * control. These configuration includes central frequency, equalizer gain,
 * cut-off frequency, bandwidth control, and equalizer path.
 * The regmap raw write can't work here because regmap doesn't provide
 * register and value format for register with address 7 bits and value 9 bits.
 * Therefore, the driver makes value format according to the endianness of
 * bytes type control element and writes data to codec.
 */
unsafe extern "C" fn nau8810_eq_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let nau8810: *mut nau8810 = snd_soc_component_get_drvdata(component) as *mut nau8810;
    let params: *mut soc_bytes_ext = (*kcontrol).private_value as *mut soc_bytes_ext;
    let data: *mut c_void = kmemdup(
        (*ucontrol).value.bytes.data.as_mut_ptr() as *const c_void,
        (*params).max,
        GFP_KERNEL | GFP_DMA,
    );
    if data.is_null() {
        return -ENOMEM;
    }

    let val: *mut u16 = data as *mut u16;
    let reg: c_int = NAU8810_REG_EQ1 as c_int;
    let mut i: c_int = 0;
    while i < ((*params).max as usize / core::mem::size_of::<u16>()) as c_int {
        /* conversion of 16-bit integers between native CPU format
         * and big endian format
         */
        let tmp: *mut __be16 = val.add(i as usize) as *mut __be16;
        let value: u16 = be16_to_cpup(tmp);
        let ret: c_int = regmap_write((*nau8810).regmap, (reg + i) as c_uint, value as c_uint);
        if ret != 0 {
            dev_err!(
                (*component).dev,
                "EQ configuration fail, register: %x ret: %d\n",
                reg + i,
                ret
            );
            kfree(data);
            return ret;
        }
        i += 1;
    }
    kfree(data);

    0
}

static nau8810_companding: [&'static str; 4] = ["Off", "NC", "u-law", "A-law"];
static nau8810_companding_adc_enum: soc_enum =
    SOC_ENUM_SINGLE!(NAU8810_REG_COMP, NAU8810_ADCCM_SFT, nau8810_companding.len(), nau8810_companding);
static nau8810_companding_dac_enum: soc_enum =
    SOC_ENUM_SINGLE!(NAU8810_REG_COMP, NAU8810_DACCM_SFT, nau8810_companding.len(), nau8810_companding);

static nau8810_deemp: [&'static str; 4] = ["None", "32kHz", "44.1kHz", "48kHz"];
static nau8810_deemp_enum: soc_enum =
    SOC_ENUM_SINGLE!(NAU8810_REG_DAC, NAU8810_DEEMP_SFT, nau8810_deemp.len(), nau8810_deemp);

static nau8810_eqmode: [&'static str; 2] = ["Capture", "Playback"];
static nau8810_eqmode_enum: soc_enum =
    SOC_ENUM_SINGLE!(NAU8810_REG_EQ1, NAU8810_EQM_SFT, nau8810_eqmode.len(), nau8810_eqmode);

static nau8810_alc: [&'static str; 2] = ["Normal", "Limiter"];
static nau8810_alc_enum: soc_enum =
    SOC_ENUM_SINGLE!(NAU8810_REG_ALC3, NAU8810_ALCM_SFT, nau8810_alc.len(), nau8810_alc);

static digital_tlv: DECLARE_TLV_DB_SCALE = DECLARE_TLV_DB_SCALE!(digital_tlv, -12750, 50, 1);
static eq_tlv: DECLARE_TLV_DB_SCALE = DECLARE_TLV_DB_SCALE!(eq_tlv, -1200, 100, 0);
static inpga_tlv: DECLARE_TLV_DB_SCALE = DECLARE_TLV_DB_SCALE!(inpga_tlv, -1200, 75, 0);
static spk_tlv: DECLARE_TLV_DB_SCALE = DECLARE_TLV_DB_SCALE!(spk_tlv, -5700, 100, 0);

static nau8810_snd_controls: [snd_kcontrol_new; 39] = [
    SOC_ENUM!("ADC Companding", nau8810_companding_adc_enum),
    SOC_ENUM!("DAC Companding", nau8810_companding_dac_enum),
    SOC_ENUM!("DAC De-emphasis", nau8810_deemp_enum),
    SOC_ENUM!("EQ Function", nau8810_eqmode_enum),
    SND_SOC_BYTES_EXT!("EQ Parameters", 10, nau8810_eq_get, nau8810_eq_put),
    SOC_SINGLE!("DAC Inversion Switch", NAU8810_REG_DAC, NAU8810_DACPL_SFT, 1, 0),
    SOC_SINGLE_TLV!("Playback Volume", NAU8810_REG_DACGAIN, NAU8810_DACGAIN_SFT, 0xff, 0, digital_tlv),
    SOC_SINGLE!("High Pass Filter Switch", NAU8810_REG_ADC, NAU8810_HPFEN_SFT, 1, 0),
    SOC_SINGLE!("High Pass Cut Off", NAU8810_REG_ADC, NAU8810_HPF_SFT, 0x7, 0),
    SOC_SINGLE!("ADC Inversion Switch", NAU8810_REG_ADC, NAU8810_ADCPL_SFT, 1, 0),
    SOC_SINGLE_TLV!("Capture Volume", NAU8810_REG_ADCGAIN, NAU8810_ADCGAIN_SFT, 0xff, 0, digital_tlv),
    SOC_SINGLE_TLV!("EQ1 Volume", NAU8810_REG_EQ1, NAU8810_EQ1GC_SFT, 0x18, 1, eq_tlv),
    SOC_SINGLE_TLV!("EQ2 Volume", NAU8810_REG_EQ2, NAU8810_EQ2GC_SFT, 0x18, 1, eq_tlv),
    SOC_SINGLE_TLV!("EQ3 Volume", NAU8810_REG_EQ3, NAU8810_EQ3GC_SFT, 0x18, 1, eq_tlv),
    SOC_SINGLE_TLV!("EQ4 Volume", NAU8810_REG_EQ4, NAU8810_EQ4GC_SFT, 0x18, 1, eq_tlv),
    SOC_SINGLE_TLV!("EQ5 Volume", NAU8810_REG_EQ5, NAU8810_EQ5GC_SFT, 0x18, 1, eq_tlv),
    SOC_SINGLE!("DAC Limiter Switch", NAU8810_REG_DACLIM1, NAU8810_DACLIMEN_SFT, 1, 0),
    SOC_SINGLE!("DAC Limiter Decay", NAU8810_REG_DACLIM1, NAU8810_DACLIMDCY_SFT, 0xf, 0),
    SOC_SINGLE!("DAC Limiter Attack", NAU8810_REG_DACLIM1, NAU8810_DACLIMATK_SFT, 0xf, 0),
    SOC_SINGLE!("DAC Limiter Threshold", NAU8810_REG_DACLIM2, NAU8810_DACLIMTHL_SFT, 0x7, 0),
    SOC_SINGLE!("DAC Limiter Boost", NAU8810_REG_DACLIM2, NAU8810_DACLIMBST_SFT, 0xf, 0),
    SOC_ENUM!("ALC Mode", nau8810_alc_enum),
    SOC_SINGLE!("ALC Enable Switch", NAU8810_REG_ALC1, NAU8810_ALCEN_SFT, 1, 0),
    SOC_SINGLE!("ALC Max Volume", NAU8810_REG_ALC1, NAU8810_ALCMXGAIN_SFT, 0x7, 0),
    SOC_SINGLE!("ALC Min Volume", NAU8810_REG_ALC1, NAU8810_ALCMINGAIN_SFT, 0x7, 0),
    SOC_SINGLE!("ALC ZC Switch", NAU8810_REG_ALC2, NAU8810_ALCZC_SFT, 1, 0),
    SOC_SINGLE!("ALC Hold", NAU8810_REG_ALC2, NAU8810_ALCHT_SFT, 0xf, 0),
    SOC_SINGLE!("ALC Target", NAU8810_REG_ALC2, NAU8810_ALCSL_SFT, 0xf, 0),
    SOC_SINGLE!("ALC Decay", NAU8810_REG_ALC3, NAU8810_ALCDCY_SFT, 0xf, 0),
    SOC_SINGLE!("ALC Attack", NAU8810_REG_ALC3, NAU8810_ALCATK_SFT, 0xf, 0),
    SOC_SINGLE!("ALC Noise Gate Switch", NAU8810_REG_NOISEGATE, NAU8810_ALCNEN_SFT, 1, 0),
    SOC_SINGLE!("ALC Noise Gate Threshold", NAU8810_REG_NOISEGATE, NAU8810_ALCNTH_SFT, 0x7, 0),
    SOC_SINGLE!("PGA ZC Switch", NAU8810_REG_PGAGAIN, NAU8810_PGAZC_SFT, 1, 0),
    SOC_SINGLE_TLV!("PGA Volume", NAU8810_REG_PGAGAIN, NAU8810_PGAGAIN_SFT, 0x3f, 0, inpga_tlv),
    SOC_SINGLE!("Speaker ZC Switch", NAU8810_REG_SPKGAIN, NAU8810_SPKZC_SFT, 1, 0),
    SOC_SINGLE!("Speaker Mute Switch", NAU8810_REG_SPKGAIN, NAU8810_SPKMT_SFT, 1, 0),
    SOC_SINGLE_TLV!("Speaker Volume", NAU8810_REG_SPKGAIN, NAU8810_SPKGAIN_SFT, 0x3f, 0, spk_tlv),
    SOC_SINGLE!("Capture Boost(+20dB)", NAU8810_REG_ADCBOOST, NAU8810_PGABST_SFT, 1, 0),
    SOC_SINGLE!("Mono Mute Switch", NAU8810_REG_MONOMIX, NAU8810_MOUTMXMT_SFT, 1, 0),
    SOC_SINGLE!("DAC Oversampling Rate(128x) Switch", NAU8810_REG_DAC, NAU8810_DACOS_SFT, 1, 0),
    SOC_SINGLE!("ADC Oversampling Rate(128x) Switch", NAU8810_REG_ADC, NAU8810_ADCOS_SFT, 1, 0),
];

/* Speaker Output Mixer */
static nau8810_speaker_mixer_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!("AUX Bypass Switch", NAU8810_REG_SPKMIX, NAU8810_AUXSPK_SFT, 1, 0),
    SOC_DAPM_SINGLE!("Line Bypass Switch", NAU8810_REG_SPKMIX, NAU8810_BYPSPK_SFT, 1, 0),
    SOC_DAPM_SINGLE!("PCM Playback Switch", NAU8810_REG_SPKMIX, NAU8810_DACSPK_SFT, 1, 0),
];

/* Mono Output Mixer */
static nau8810_mono_mixer_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!("AUX Bypass Switch", NAU8810_REG_MONOMIX, NAU8810_AUXMOUT_SFT, 1, 0),
    SOC_DAPM_SINGLE!("Line Bypass Switch", NAU8810_REG_MONOMIX, NAU8810_BYPMOUT_SFT, 1, 0),
    SOC_DAPM_SINGLE!("PCM Playback Switch", NAU8810_REG_MONOMIX, NAU8810_DACMOUT_SFT, 1, 0),
];

/* PGA Mute */
static nau8810_pgaboost_mixer_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!("AUX PGA Switch", NAU8810_REG_ADCBOOST, NAU8810_AUXBSTGAIN_SFT, 0x7, 0),
    SOC_DAPM_SINGLE!("PGA Mute Switch", NAU8810_REG_PGAGAIN, NAU8810_PGAMT_SFT, 1, 1),
    SOC_DAPM_SINGLE!("PMIC PGA Switch", NAU8810_REG_ADCBOOST, NAU8810_PMICBSTGAIN_SFT, 0x7, 0),
];

/* Input PGA */
static nau8810_inpga: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!("AUX Switch", NAU8810_REG_INPUT_SIGNAL, NAU8810_AUXPGA_SFT, 1, 0),
    SOC_DAPM_SINGLE!("MicN Switch", NAU8810_REG_INPUT_SIGNAL, NAU8810_NMICPGA_SFT, 1, 0),
    SOC_DAPM_SINGLE!("MicP Switch", NAU8810_REG_INPUT_SIGNAL, NAU8810_PMICPGA_SFT, 1, 0),
];

/* Loopback Switch */
static nau8810_loopback: snd_kcontrol_new =
    SOC_DAPM_SINGLE!("Switch", NAU8810_REG_COMP, NAU8810_ADDAP_SFT, 1, 0);

unsafe extern "C" fn check_mclk_select_pll(
    source: *mut snd_soc_dapm_widget,
    _sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*source).dapm);
    let nau8810: *mut nau8810 = snd_soc_component_get_drvdata(component) as *mut nau8810;
    let mut value: c_uint = 0;

    regmap_read((*nau8810).regmap, NAU8810_REG_CLOCK, &mut value);
    (value & NAU8810_CLKM_MASK) as c_int
}

unsafe extern "C" fn check_mic_enabled(
    source: *mut snd_soc_dapm_widget,
    _sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*source).dapm);
    let nau8810: *mut nau8810 = snd_soc_component_get_drvdata(component) as *mut nau8810;
    let mut value: c_uint = 0;

    regmap_read((*nau8810).regmap, NAU8810_REG_INPUT_SIGNAL, &mut value);
    if (value & NAU8810_PMICPGA_EN) != 0 || (value & NAU8810_NMICPGA_EN) != 0 {
        return 1;
    }
    regmap_read((*nau8810).regmap, NAU8810_REG_ADCBOOST, &mut value);
    if (value & NAU8810_PMICBSTGAIN_MASK) != 0 {
        return 1;
    }
    0
}

static nau8810_dapm_widgets: [snd_soc_dapm_widget; 20] = [
    SND_SOC_DAPM_MIXER!("Speaker Mixer", NAU8810_REG_POWER3, NAU8810_SPKMX_EN_SFT, 0, &nau8810_speaker_mixer_controls[0], nau8810_speaker_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("Mono Mixer", NAU8810_REG_POWER3, NAU8810_MOUTMX_EN_SFT, 0, &nau8810_mono_mixer_controls[0], nau8810_mono_mixer_controls.len()),
    SND_SOC_DAPM_DAC!("DAC", "Playback", NAU8810_REG_POWER3, NAU8810_DAC_EN_SFT, 0),
    SND_SOC_DAPM_ADC!("ADC", "Capture", NAU8810_REG_POWER2, NAU8810_ADC_EN_SFT, 0),
    SND_SOC_DAPM_PGA!("SpkN Out", NAU8810_REG_POWER3, NAU8810_NSPK_EN_SFT, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("SpkP Out", NAU8810_REG_POWER3, NAU8810_PSPK_EN_SFT, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Mono Out", NAU8810_REG_POWER3, NAU8810_MOUT_EN_SFT, 0, NULL, 0),
    SND_SOC_DAPM_MIXER!("Input PGA", NAU8810_REG_POWER2, NAU8810_PGA_EN_SFT, 0, nau8810_inpga, nau8810_inpga.len()),
    SND_SOC_DAPM_MIXER!("Input Boost Stage", NAU8810_REG_POWER2, NAU8810_BST_EN_SFT, 0, nau8810_pgaboost_mixer_controls, nau8810_pgaboost_mixer_controls.len()),
    SND_SOC_DAPM_PGA!("AUX Input", NAU8810_REG_POWER1, NAU8810_AUX_EN_SFT, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("Mic Bias", NAU8810_REG_POWER1, NAU8810_MICBIAS_EN_SFT, 0, NULL, 0),
    SND_SOC_DAPM_SUPPLY!("PLL", NAU8810_REG_POWER1, NAU8810_PLL_EN_SFT, 0, NULL, 0),
    SND_SOC_DAPM_SWITCH!("Digital Loopback", SND_SOC_NOPM, 0, 0, &nau8810_loopback),
    SND_SOC_DAPM_INPUT!("AUX"),
    SND_SOC_DAPM_INPUT!("MICN"),
    SND_SOC_DAPM_INPUT!("MICP"),
    SND_SOC_DAPM_OUTPUT!("MONOOUT"),
    SND_SOC_DAPM_OUTPUT!("SPKOUTP"),
    SND_SOC_DAPM_OUTPUT!("SPKOUTN"),
];

static nau8810_dapm_routes: [snd_soc_dapm_route; 31] = [
    snd_soc_dapm_route { sink: "DAC", control: NULL, source: "PLL", connected: Some(check_mclk_select_pll) },
    /* Mono output mixer */
    snd_soc_dapm_route { sink: "Mono Mixer", control: "AUX Bypass Switch", source: "AUX Input", connected: None },
    snd_soc_dapm_route { sink: "Mono Mixer", control: "PCM Playback Switch", source: "DAC", connected: None },
    snd_soc_dapm_route { sink: "Mono Mixer", control: "Line Bypass Switch", source: "Input Boost Stage", connected: None },
    /* Speaker output mixer */
    snd_soc_dapm_route { sink: "Speaker Mixer", control: "AUX Bypass Switch", source: "AUX Input", connected: None },
    snd_soc_dapm_route { sink: "Speaker Mixer", control: "PCM Playback Switch", source: "DAC", connected: None },
    snd_soc_dapm_route { sink: "Speaker Mixer", control: "Line Bypass Switch", source: "Input Boost Stage", connected: None },
    /* Outputs */
    snd_soc_dapm_route { sink: "Mono Out", control: NULL, source: "Mono Mixer", connected: None },
    snd_soc_dapm_route { sink: "MONOOUT", control: NULL, source: "Mono Out", connected: None },
    snd_soc_dapm_route { sink: "SpkN Out", control: NULL, source: "Speaker Mixer", connected: None },
    snd_soc_dapm_route { sink: "SpkP Out", control: NULL, source: "Speaker Mixer", connected: None },
    snd_soc_dapm_route { sink: "SPKOUTN", control: NULL, source: "SpkN Out", connected: None },
    snd_soc_dapm_route { sink: "SPKOUTP", control: NULL, source: "SpkP Out", connected: None },
    /* Input Boost Stage */
    snd_soc_dapm_route { sink: "ADC", control: NULL, source: "Input Boost Stage", connected: None },
    snd_soc_dapm_route { sink: "ADC", control: NULL, source: "PLL", connected: Some(check_mclk_select_pll) },
    snd_soc_dapm_route { sink: "Input Boost Stage", control: "AUX PGA Switch", source: "AUX Input", connected: None },
    snd_soc_dapm_route { sink: "Input Boost Stage", control: "PGA Mute Switch", source: "Input PGA", connected: None },
    snd_soc_dapm_route { sink: "Input Boost Stage", control: "PMIC PGA Switch", source: "MICP", connected: None },
    /* Input PGA */
    snd_soc_dapm_route { sink: "Input PGA", control: NULL, source: "Mic Bias", connected: Some(check_mic_enabled) },
    snd_soc_dapm_route { sink: "Input PGA", control: "AUX Switch", source: "AUX Input", connected: None },
    snd_soc_dapm_route { sink: "Input PGA", control: "MicN Switch", source: "MICN", connected: None },
    snd_soc_dapm_route { sink: "Input PGA", control: "MicP Switch", source: "MICP", connected: None },
    snd_soc_dapm_route { sink: "AUX Input", control: NULL, source: "AUX", connected: None },
    /* Digital Looptack */
    snd_soc_dapm_route { sink: "Digital Loopback", control: "Switch", source: "ADC", connected: None },
    snd_soc_dapm_route { sink: "DAC", control: NULL, source: "Digital Loopback", connected: None },
];

unsafe extern "C" fn nau8810_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let nau8810: *mut nau8810 = snd_soc_component_get_drvdata(component) as *mut nau8810;

    (*nau8810).clk_id = clk_id;
    (*nau8810).sysclk = freq;
    dev_dbg!(
        (*nau8810).dev,
        "master sysclk %dHz, source %s\n",
        freq,
        if clk_id == NAU8810_SCLK_PLL { "PLL" } else { "MCLK" }
    );

    0
}

unsafe extern "C" fn nau8810_calc_pll(
    pll_in: c_uint,
    fs: c_uint,
    pll_param: *mut nau8810_pll,
) -> c_int {
    let mut f2: u64;
    let mut f2_max: u64;
    let mut pll_ratio: u64;
    let mut i: c_int;
    let mut scal_sel: c_int;

    if pll_in > NAU_PLL_REF_MAX || pll_in < NAU_PLL_REF_MIN {
        return -EINVAL;
    }

    f2_max = 0;
    scal_sel = nau8810_mclk_scaler.len() as c_int;
    i = 0;
    while i < nau8810_mclk_scaler.len() as c_int {
        f2 = 256_u64 * fs as u64 * 4 * nau8810_mclk_scaler[i as usize] as u64;
        f2 = div_u64(f2, 10);
        if f2 > NAU_PLL_FREQ_MIN as u64 && f2 < NAU_PLL_FREQ_MAX as u64 && f2_max < f2 {
            f2_max = f2;
            scal_sel = i;
        }
        i += 1;
    }
    if nau8810_mclk_scaler.len() as c_int == scal_sel {
        return -EINVAL;
    }
    (*pll_param).mclk_scaler = scal_sel;
    f2 = f2_max;

    /* Calculate the PLL 4-bit integer input and the PLL 24-bit fractional
     * input; round up the 24+4bit.
     */
    pll_ratio = div_u64(f2 << 28, pll_in as u64);
    (*pll_param).pre_factor = 0;
    if ((pll_ratio >> 28) & 0xF) < NAU_PLL_OPTOP_MIN {
        pll_ratio <<= 1;
        (*pll_param).pre_factor = 1;
    }
    (*pll_param).pll_int = ((pll_ratio >> 28) & 0xF) as c_uint;
    (*pll_param).pll_frac = ((pll_ratio & 0xFFFFFFF) >> 4) as c_uint;

    0
}

unsafe extern "C" fn nau8810_set_pll(
    codec_dai: *mut snd_soc_dai,
    _pll_id: c_int,
    _source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let nau8810: *mut nau8810 = snd_soc_component_get_drvdata(component) as *mut nau8810;
    let map: *mut regmap = (*nau8810).regmap;
    let pll_param: *mut nau8810_pll = &mut (*nau8810).pll;
    let fs: c_int = (freq_out / 256) as c_int;
    let ret: c_int = nau8810_calc_pll(freq_in, fs as c_uint, pll_param);

    if ret < 0 {
        dev_err!((*nau8810).dev, "Unsupported input clock %d\n", freq_in);
        return ret;
    }
    dev_info!(
        (*nau8810).dev,
        "pll_int=%x pll_frac=%x mclk_scaler=%x pre_factor=%x\n",
        (*pll_param).pll_int,
        (*pll_param).pll_frac,
        (*pll_param).mclk_scaler,
        (*pll_param).pre_factor
    );

    regmap_update_bits(
        map,
        NAU8810_REG_PLLN,
        NAU8810_PLLMCLK_DIV2 | NAU8810_PLLN_MASK,
        (if (*pll_param).pre_factor != 0 { NAU8810_PLLMCLK_DIV2 } else { 0 }) | (*pll_param).pll_int,
    );
    regmap_write(
        map,
        NAU8810_REG_PLLK1,
        ((*pll_param).pll_frac >> NAU8810_PLLK1_SFT) & NAU8810_PLLK1_MASK,
    );
    regmap_write(
        map,
        NAU8810_REG_PLLK2,
        ((*pll_param).pll_frac >> NAU8810_PLLK2_SFT) & NAU8810_PLLK2_MASK,
    );
    regmap_write(map, NAU8810_REG_PLLK3, (*pll_param).pll_frac & NAU8810_PLLK3_MASK);
    regmap_update_bits(
        map,
        NAU8810_REG_CLOCK,
        NAU8810_MCLKSEL_MASK,
        ((*pll_param).mclk_scaler as c_uint) << NAU8810_MCLKSEL_SFT,
    );
    regmap_update_bits(map, NAU8810_REG_CLOCK, NAU8810_CLKM_MASK, NAU8810_CLKM_PLL);

    0
}

unsafe extern "C" fn nau8810_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let nau8810: *mut nau8810 = snd_soc_component_get_drvdata(component) as *mut nau8810;
    let mut ctrl1_val: u16 = 0;
    let mut ctrl2_val: u16 = 0;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => ctrl2_val |= NAU8810_CLKIO_MASTER as u16,
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => ctrl1_val |= NAU8810_AIFMT_I2S as u16,
        SND_SOC_DAIFMT_RIGHT_J => {}
        SND_SOC_DAIFMT_LEFT_J => ctrl1_val |= NAU8810_AIFMT_LEFT as u16,
        SND_SOC_DAIFMT_DSP_A => ctrl1_val |= NAU8810_AIFMT_PCM_A as u16,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_IF => ctrl1_val |= (NAU8810_BCLKP_IB | NAU8810_FSP_IF) as u16,
        SND_SOC_DAIFMT_IB_NF => ctrl1_val |= NAU8810_BCLKP_IB as u16,
        SND_SOC_DAIFMT_NB_IF => ctrl1_val |= NAU8810_FSP_IF as u16,
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*nau8810).regmap,
        NAU8810_REG_IFACE,
        NAU8810_AIFMT_MASK | NAU8810_FSP_IF | NAU8810_BCLKP_IB,
        ctrl1_val as c_uint,
    );
    regmap_update_bits(
        (*nau8810).regmap,
        NAU8810_REG_CLOCK,
        NAU8810_CLKIO_MASK,
        ctrl2_val as c_uint,
    );

    0
}

unsafe extern "C" fn nau8810_mclk_clkdiv(nau8810: *mut nau8810, rate: c_int) -> c_int {
    let mut div: c_int = 0;
    let imclk: c_int = rate * 256;

    if (*nau8810).sysclk == 0 {
        dev_err!(
            (*nau8810).dev,
            "Make mclk div configuration fail because of invalid system clock\n"
        );
        return -EINVAL;
    }

    /* Configure the master clock prescaler div to make system
     * clock to approximate the internal master clock (IMCLK);
     * and large or equal to IMCLK.
     */
    let mut i: c_int = 1;
    while i < nau8810_mclk_scaler.len() as c_int {
        let sclk: c_int = ((*nau8810).sysclk as c_int * 10) / nau8810_mclk_scaler[i as usize];
        if sclk < imclk {
            break;
        }
        div = i;
        i += 1;
    }
    dev_dbg!((*nau8810).dev, "master clock prescaler %x for fs %d\n", div, rate);

    /* master clock from MCLK and disable PLL */
    regmap_update_bits(
        (*nau8810).regmap,
        NAU8810_REG_CLOCK,
        NAU8810_MCLKSEL_MASK,
        (div as c_uint) << NAU8810_MCLKSEL_SFT,
    );
    regmap_update_bits(
        (*nau8810).regmap,
        NAU8810_REG_CLOCK,
        NAU8810_CLKM_MASK,
        NAU8810_CLKM_MCLK,
    );

    0
}

unsafe extern "C" fn nau8810_pcm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let nau8810: *mut nau8810 = snd_soc_component_get_drvdata(component) as *mut nau8810;
    let mut val_len: c_int = 0;
    let mut val_rate: c_int = 0;
    let mut ret: c_int = 0;
    let mut ctrl_val: c_uint = 0;

    /* Select BCLK configuration if the codec as master. */
    regmap_read((*nau8810).regmap, NAU8810_REG_CLOCK, &mut ctrl_val);
    if (ctrl_val & NAU8810_CLKIO_MASTER) != 0 {
        /* get the bclk and fs ratio */
        let bclk_fs: c_uint = snd_soc_params_to_bclk(params) / params_rate(params);
        let bclk_div: c_uint;
        if bclk_fs <= 32 {
            bclk_div = NAU8810_BCLKDIV_8;
        } else if bclk_fs <= 64 {
            bclk_div = NAU8810_BCLKDIV_4;
        } else if bclk_fs <= 128 {
            bclk_div = NAU8810_BCLKDIV_2;
        } else {
            return -EINVAL;
        }
        regmap_update_bits((*nau8810).regmap, NAU8810_REG_CLOCK, NAU8810_BCLKSEL_MASK, bclk_div);
    }

    match params_width(params) {
        16 => {}
        20 => val_len |= NAU8810_WLEN_20 as c_int,
        24 => val_len |= NAU8810_WLEN_24 as c_int,
        32 => val_len |= NAU8810_WLEN_32 as c_int,
        _ => {}
    }

    match params_rate(params) {
        8000 => val_rate |= NAU8810_SMPLR_8K as c_int,
        11025 => val_rate |= NAU8810_SMPLR_12K as c_int,
        16000 => val_rate |= NAU8810_SMPLR_16K as c_int,
        22050 => val_rate |= NAU8810_SMPLR_24K as c_int,
        32000 => val_rate |= NAU8810_SMPLR_32K as c_int,
        44100 | 48000 => {}
        _ => {}
    }

    regmap_update_bits((*nau8810).regmap, NAU8810_REG_IFACE, NAU8810_WLEN_MASK, val_len as c_uint);
    regmap_update_bits((*nau8810).regmap, NAU8810_REG_SMPLR, NAU8810_SMPLR_MASK, val_rate as c_uint);

    /* If the master clock is from MCLK, provide the runtime FS for driver
     * to get the master clock prescaler configuration.
     */
    if (*nau8810).clk_id == NAU8810_SCLK_MCLK {
        ret = nau8810_mclk_clkdiv(nau8810, params_rate(params) as c_int);
        if ret < 0 {
            dev_err!((*nau8810).dev, "MCLK div configuration fail\n");
        }
    }

    ret
}

unsafe extern "C" fn nau8810_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let nau8810: *mut nau8810 = snd_soc_component_get_drvdata(component) as *mut nau8810;
    let map: *mut regmap = (*nau8810).regmap;

    match level {
        SND_SOC_BIAS_ON | SND_SOC_BIAS_PREPARE => {
            regmap_update_bits(map, NAU8810_REG_POWER1, NAU8810_REFIMP_MASK, NAU8810_REFIMP_80K);
        }
        SND_SOC_BIAS_STANDBY => {
            regmap_update_bits(
                map,
                NAU8810_REG_POWER1,
                NAU8810_IOBUF_EN | NAU8810_ABIAS_EN,
                NAU8810_IOBUF_EN | NAU8810_ABIAS_EN,
            );

            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                regcache_sync(map);
                regmap_update_bits(map, NAU8810_REG_POWER1, NAU8810_REFIMP_MASK, NAU8810_REFIMP_3K);
                mdelay(100);
            }
            regmap_update_bits(map, NAU8810_REG_POWER1, NAU8810_REFIMP_MASK, NAU8810_REFIMP_300K);
        }
        SND_SOC_BIAS_OFF => {
            regmap_write(map, NAU8810_REG_POWER1, 0);
            regmap_write(map, NAU8810_REG_POWER2, 0);
            regmap_write(map, NAU8810_REG_POWER3, 0);
        }
    }

    0
}

const NAU8810_RATES: c_uint = SNDRV_PCM_RATE_8000_48000;
const NAU8810_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static nau8810_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(nau8810_pcm_hw_params),
    set_fmt: Some(nau8810_set_dai_fmt),
    set_sysclk: Some(nau8810_set_sysclk),
    set_pll: Some(nau8810_set_pll),
};

static mut nau8810_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: "nau8810-hifi",
    playback: snd_soc_pcm_stream {
        stream_name: "Playback",
        channels_min: 1,
        channels_max: 2, /* Only 1 channel of data */
        rates: NAU8810_RATES,
        formats: NAU8810_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: "Capture",
        channels_min: 1,
        channels_max: 2, /* Only 1 channel of data */
        rates: NAU8810_RATES,
        formats: NAU8810_FORMATS,
    },
    ops: &nau8810_ops,
    symmetric_rate: 1,
};

static nau8810_regmap_config: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 9,
    max_register: NAU8810_REG_MAX,
    readable_reg: Some(nau8810_readable_reg),
    writeable_reg: Some(nau8810_writeable_reg),
    volatile_reg: Some(nau8810_volatile_reg),
    cache_type: REGCACHE_RBTREE,
    reg_defaults: nau8810_reg_defaults.as_ptr(),
    num_reg_defaults: nau8810_reg_defaults.len(),
};

static nau8810_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(nau8810_set_bias_level),
    controls: nau8810_snd_controls.as_ptr(),
    num_controls: nau8810_snd_controls.len(),
    dapm_widgets: nau8810_dapm_widgets.as_ptr(),
    num_dapm_widgets: nau8810_dapm_widgets.len(),
    dapm_routes: nau8810_dapm_routes.as_ptr(),
    num_dapm_routes: nau8810_dapm_routes.len(),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn nau8810_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let dev: *mut device = &mut (*i2c).dev;
    let mut nau8810: *mut nau8810 = dev_get_platdata(dev) as *mut nau8810;

    if nau8810.is_null() {
        nau8810 = devm_kzalloc(dev, core::mem::size_of::<nau8810>(), GFP_KERNEL) as *mut nau8810;
        if nau8810.is_null() {
            return -ENOMEM;
        }
    }
    i2c_set_clientdata(i2c, nau8810 as *mut c_void);

    (*nau8810).regmap = devm_regmap_init_i2c(i2c, &nau8810_regmap_config);
    if IS_ERR((*nau8810).regmap as *const c_void) {
        return PTR_ERR((*nau8810).regmap as *const c_void);
    }
    (*nau8810).dev = dev;

    regmap_write((*nau8810).regmap, NAU8810_REG_RESET, 0x00);

    devm_snd_soc_register_component(dev, &nau8810_component_driver, &mut nau8810_dai, 1)
}

static nau8810_i2c_id: [i2c_device_id; 3] = [
    i2c_device_id { name: "nau8810" },
    i2c_device_id { name: "nau8812" },
    i2c_device_id { name: "nau8814" },
];
MODULE_DEVICE_TABLE!(i2c, nau8810_i2c_id);

/* CONFIG_OF conditional device table from the C source. */
#[cfg(CONFIG_OF)]
static nau8810_of_match: [of_device_id; 3] = [
    of_device_id { compatible: "nuvoton,nau8810" },
    of_device_id { compatible: "nuvoton,nau8812" },
    of_device_id { compatible: "nuvoton,nau8814" },
];
#[cfg(CONFIG_OF)]
MODULE_DEVICE_TABLE!(of, nau8810_of_match);

static mut nau8810_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: "nau8810",
        of_match_table: of_match_ptr!(nau8810_of_match),
    },
    probe: Some(nau8810_i2c_probe),
    id_table: nau8810_i2c_id.as_ptr(),
};

module_i2c_driver!(nau8810_i2c_driver);

MODULE_DESCRIPTION!("ASoC NAU8810 driver");
MODULE_AUTHOR!("David Lin <ctlin0@nuvoton.com>");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
