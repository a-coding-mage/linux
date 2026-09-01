// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm8961.c  --  WM8961 ALSA SoC Audio driver
 *
 * Copyright 2009-10 Wolfson Microelectronics, plc
 *
 * Author: Mark Brown
 *
 * Currently unimplemented features:
 *  - ALC
 */

// Rust translation of the implementation source. C include dependencies are
// expected to provide the referenced kernel, ASoC, regmap, I2C, and WM8961 items.

const WM8961_MAX_REGISTER: u32 = 0xFC;
const NULL: *mut core::ffi::c_void = core::ptr::null_mut();

#[repr(C)]
pub struct wm8961_priv {
    regmap: *mut regmap,
    sysclk: core::ffi::c_int,
}

static wm8961_reg_defaults: [reg_default; 54] = [
    reg_default { reg: 0, def: 0x009F },     /* R0   - Left Input volume */
    reg_default { reg: 1, def: 0x009F },     /* R1   - Right Input volume */
    reg_default { reg: 2, def: 0x0000 },     /* R2   - LOUT1 volume */
    reg_default { reg: 3, def: 0x0000 },     /* R3   - ROUT1 volume */
    reg_default { reg: 4, def: 0x0020 },     /* R4   - Clocking1 */
    reg_default { reg: 5, def: 0x0008 },     /* R5   - ADC & DAC Control 1 */
    reg_default { reg: 6, def: 0x0000 },     /* R6   - ADC & DAC Control 2 */
    reg_default { reg: 7, def: 0x000A },     /* R7   - Audio Interface 0 */
    reg_default { reg: 8, def: 0x01F4 },     /* R8   - Clocking2 */
    reg_default { reg: 9, def: 0x0000 },     /* R9   - Audio Interface 1 */
    reg_default { reg: 10, def: 0x00FF },    /* R10  - Left DAC volume */
    reg_default { reg: 11, def: 0x00FF },    /* R11  - Right DAC volume */
    reg_default { reg: 14, def: 0x0040 },    /* R14  - Audio Interface 2 */
    reg_default { reg: 17, def: 0x007B },    /* R17  - ALC1 */
    reg_default { reg: 18, def: 0x0000 },    /* R18  - ALC2 */
    reg_default { reg: 19, def: 0x0032 },    /* R19  - ALC3 */
    reg_default { reg: 20, def: 0x0000 },    /* R20  - Noise Gate */
    reg_default { reg: 21, def: 0x00C0 },    /* R21  - Left ADC volume */
    reg_default { reg: 22, def: 0x00C0 },    /* R22  - Right ADC volume */
    reg_default { reg: 23, def: 0x0120 },    /* R23  - Additional control(1) */
    reg_default { reg: 24, def: 0x0000 },    /* R24  - Additional control(2) */
    reg_default { reg: 25, def: 0x0000 },    /* R25  - Pwr Mgmt (1) */
    reg_default { reg: 26, def: 0x0000 },    /* R26  - Pwr Mgmt (2) */
    reg_default { reg: 27, def: 0x0000 },    /* R27  - Additional Control (3) */
    reg_default { reg: 28, def: 0x0000 },    /* R28  - Anti-pop */
    reg_default { reg: 30, def: 0x005F },    /* R30  - Clocking 3 */
    reg_default { reg: 32, def: 0x0000 },    /* R32  - ADCL signal path */
    reg_default { reg: 33, def: 0x0000 },    /* R33  - ADCR signal path */
    reg_default { reg: 40, def: 0x0000 },    /* R40  - LOUT2 volume */
    reg_default { reg: 41, def: 0x0000 },    /* R41  - ROUT2 volume */
    reg_default { reg: 47, def: 0x0000 },    /* R47  - Pwr Mgmt (3) */
    reg_default { reg: 48, def: 0x0023 },    /* R48  - Additional Control (4) */
    reg_default { reg: 49, def: 0x0000 },    /* R49  - Class D Control 1 */
    reg_default { reg: 51, def: 0x0003 },    /* R51  - Class D Control 2 */
    reg_default { reg: 56, def: 0x0106 },    /* R56  - Clocking 4 */
    reg_default { reg: 57, def: 0x0000 },    /* R57  - DSP Sidetone 0 */
    reg_default { reg: 58, def: 0x0000 },    /* R58  - DSP Sidetone 1 */
    reg_default { reg: 60, def: 0x0000 },    /* R60  - DC Servo 0 */
    reg_default { reg: 61, def: 0x0000 },    /* R61  - DC Servo 1 */
    reg_default { reg: 63, def: 0x015E },    /* R63  - DC Servo 3 */
    reg_default { reg: 65, def: 0x0010 },    /* R65  - DC Servo 5 */
    reg_default { reg: 68, def: 0x0003 },    /* R68  - Analogue PGA Bias */
    reg_default { reg: 69, def: 0x0000 },    /* R69  - Analogue HP 0 */
    reg_default { reg: 71, def: 0x01FB },    /* R71  - Analogue HP 2 */
    reg_default { reg: 72, def: 0x0000 },    /* R72  - Charge Pump 1 */
    reg_default { reg: 82, def: 0x0000 },    /* R82  - Charge Pump B */
    reg_default { reg: 87, def: 0x0000 },    /* R87  - Write Sequencer 1 */
    reg_default { reg: 88, def: 0x0000 },    /* R88  - Write Sequencer 2 */
    reg_default { reg: 89, def: 0x0000 },    /* R89  - Write Sequencer 3 */
    reg_default { reg: 90, def: 0x0000 },    /* R90  - Write Sequencer 4 */
    reg_default { reg: 91, def: 0x0000 },    /* R91  - Write Sequencer 5 */
    reg_default { reg: 92, def: 0x0000 },    /* R92  - Write Sequencer 6 */
    reg_default { reg: 93, def: 0x0000 },    /* R93  - Write Sequencer 7 */
    reg_default { reg: 252, def: 0x0001 },   /* R252 - General test 1 */
];

unsafe extern "C" fn wm8961_volatile(_dev: *mut device, reg: core::ffi::c_uint) -> bool {
    match reg {
        WM8961_SOFTWARE_RESET | WM8961_WRITE_SEQUENCER_7 | WM8961_DC_SERVO_1 => true,
        _ => false,
    }
}

unsafe extern "C" fn wm8961_readable(_dev: *mut device, reg: core::ffi::c_uint) -> bool {
    match reg {
        WM8961_LEFT_INPUT_VOLUME | WM8961_RIGHT_INPUT_VOLUME | WM8961_LOUT1_VOLUME |
        WM8961_ROUT1_VOLUME | WM8961_CLOCKING1 | WM8961_ADC_DAC_CONTROL_1 |
        WM8961_ADC_DAC_CONTROL_2 | WM8961_AUDIO_INTERFACE_0 | WM8961_CLOCKING2 |
        WM8961_AUDIO_INTERFACE_1 | WM8961_LEFT_DAC_VOLUME | WM8961_RIGHT_DAC_VOLUME |
        WM8961_AUDIO_INTERFACE_2 | WM8961_SOFTWARE_RESET | WM8961_ALC1 | WM8961_ALC2 |
        WM8961_ALC3 | WM8961_NOISE_GATE | WM8961_LEFT_ADC_VOLUME | WM8961_RIGHT_ADC_VOLUME |
        WM8961_ADDITIONAL_CONTROL_1 | WM8961_ADDITIONAL_CONTROL_2 | WM8961_PWR_MGMT_1 |
        WM8961_PWR_MGMT_2 | WM8961_ADDITIONAL_CONTROL_3 | WM8961_ANTI_POP |
        WM8961_CLOCKING_3 | WM8961_ADCL_SIGNAL_PATH | WM8961_ADCR_SIGNAL_PATH |
        WM8961_LOUT2_VOLUME | WM8961_ROUT2_VOLUME | WM8961_PWR_MGMT_3 |
        WM8961_ADDITIONAL_CONTROL_4 | WM8961_CLASS_D_CONTROL_1 | WM8961_CLASS_D_CONTROL_2 |
        WM8961_CLOCKING_4 | WM8961_DSP_SIDETONE_0 | WM8961_DSP_SIDETONE_1 |
        WM8961_DC_SERVO_0 | WM8961_DC_SERVO_1 | WM8961_DC_SERVO_3 | WM8961_DC_SERVO_5 |
        WM8961_ANALOGUE_PGA_BIAS | WM8961_ANALOGUE_HP_0 | WM8961_ANALOGUE_HP_2 |
        WM8961_CHARGE_PUMP_1 | WM8961_CHARGE_PUMP_B | WM8961_WRITE_SEQUENCER_1 |
        WM8961_WRITE_SEQUENCER_2 | WM8961_WRITE_SEQUENCER_3 | WM8961_WRITE_SEQUENCER_4 |
        WM8961_WRITE_SEQUENCER_5 | WM8961_WRITE_SEQUENCER_6 | WM8961_WRITE_SEQUENCER_7 |
        WM8961_GENERAL_TEST_1 => true,
        _ => false,
    }
}

/*
 * The headphone output supports special anti-pop sequences giving
 * silent power up and power down.
 */
unsafe extern "C" fn wm8961_hp_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: core::ffi::c_int,
) -> core::ffi::c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let mut hp_reg: u16 = snd_soc_component_read(component, WM8961_ANALOGUE_HP_0) as u16;
    let cp_reg: u16 = snd_soc_component_read(component, WM8961_CHARGE_PUMP_1) as u16;
    let mut pwr_reg: u16 = snd_soc_component_read(component, WM8961_PWR_MGMT_2) as u16;
    let mut dcs_reg: u16 = snd_soc_component_read(component, WM8961_DC_SERVO_1) as u16;
    let mut timeout: core::ffi::c_int = 500;

    if event & SND_SOC_DAPM_POST_PMU != 0 {
        /* Make sure the output is shorted */
        hp_reg &= !(WM8961_HPR_RMV_SHORT | WM8961_HPL_RMV_SHORT) as u16;
        snd_soc_component_write(component, WM8961_ANALOGUE_HP_0, hp_reg as core::ffi::c_uint);

        /* Enable the charge pump */
        let mut cp_reg_mut = cp_reg | WM8961_CP_ENA as u16;
        snd_soc_component_write(component, WM8961_CHARGE_PUMP_1, cp_reg_mut as core::ffi::c_uint);
        mdelay(5);

        /* Enable the PGA */
        pwr_reg |= (WM8961_LOUT1_PGA | WM8961_ROUT1_PGA) as u16;
        snd_soc_component_write(component, WM8961_PWR_MGMT_2, pwr_reg as core::ffi::c_uint);

        /* Enable the amplifier */
        hp_reg |= (WM8961_HPR_ENA | WM8961_HPL_ENA) as u16;
        snd_soc_component_write(component, WM8961_ANALOGUE_HP_0, hp_reg as core::ffi::c_uint);

        /* Second stage enable */
        hp_reg |= (WM8961_HPR_ENA_DLY | WM8961_HPL_ENA_DLY) as u16;
        snd_soc_component_write(component, WM8961_ANALOGUE_HP_0, hp_reg as core::ffi::c_uint);

        /* Enable the DC servo & trigger startup */
        dcs_reg |= (WM8961_DCS_ENA_CHAN_HPR | WM8961_DCS_TRIG_STARTUP_HPR |
            WM8961_DCS_ENA_CHAN_HPL | WM8961_DCS_TRIG_STARTUP_HPL) as u16;
        dev_dbg!((*component).dev, "Enabling DC servo\n");

        snd_soc_component_write(component, WM8961_DC_SERVO_1, dcs_reg as core::ffi::c_uint);
        loop {
            msleep(1);
            dcs_reg = snd_soc_component_read(component, WM8961_DC_SERVO_1) as u16;
            timeout -= 1;
            if !(timeout != 0 &&
                dcs_reg & (WM8961_DCS_TRIG_STARTUP_HPR | WM8961_DCS_TRIG_STARTUP_HPL) as u16 != 0) {
                break;
            }
        }
        if dcs_reg & (WM8961_DCS_TRIG_STARTUP_HPR | WM8961_DCS_TRIG_STARTUP_HPL) as u16 != 0 {
            dev_err!((*component).dev, "DC servo timed out\n");
        } else {
            dev_dbg!((*component).dev, "DC servo startup complete\n");
        }

        /* Enable the output stage */
        hp_reg |= (WM8961_HPR_ENA_OUTP | WM8961_HPL_ENA_OUTP) as u16;
        snd_soc_component_write(component, WM8961_ANALOGUE_HP_0, hp_reg as core::ffi::c_uint);

        /* Remove the short on the output stage */
        hp_reg |= (WM8961_HPR_RMV_SHORT | WM8961_HPL_RMV_SHORT) as u16;
        snd_soc_component_write(component, WM8961_ANALOGUE_HP_0, hp_reg as core::ffi::c_uint);

        cp_reg_mut = cp_reg_mut;
    }

    if event & SND_SOC_DAPM_PRE_PMD != 0 {
        /* Short the output */
        hp_reg &= !(WM8961_HPR_RMV_SHORT | WM8961_HPL_RMV_SHORT) as u16;
        snd_soc_component_write(component, WM8961_ANALOGUE_HP_0, hp_reg as core::ffi::c_uint);

        /* Disable the output stage */
        hp_reg &= !(WM8961_HPR_ENA_OUTP | WM8961_HPL_ENA_OUTP) as u16;
        snd_soc_component_write(component, WM8961_ANALOGUE_HP_0, hp_reg as core::ffi::c_uint);

        /* Disable DC offset cancellation */
        dcs_reg &= !(WM8961_DCS_ENA_CHAN_HPR | WM8961_DCS_ENA_CHAN_HPL) as u16;
        snd_soc_component_write(component, WM8961_DC_SERVO_1, dcs_reg as core::ffi::c_uint);

        /* Finish up */
        hp_reg &= !(WM8961_HPR_ENA_DLY | WM8961_HPR_ENA |
            WM8961_HPL_ENA_DLY | WM8961_HPL_ENA) as u16;
        snd_soc_component_write(component, WM8961_ANALOGUE_HP_0, hp_reg as core::ffi::c_uint);

        /* Disable the PGA */
        pwr_reg &= !(WM8961_LOUT1_PGA | WM8961_ROUT1_PGA) as u16;
        snd_soc_component_write(component, WM8961_PWR_MGMT_2, pwr_reg as core::ffi::c_uint);

        /* Disable the charge pump */
        dev_dbg!((*component).dev, "Disabling charge pump\n");
        snd_soc_component_write(component, WM8961_CHARGE_PUMP_1,
            (cp_reg & !(WM8961_CP_ENA as u16)) as core::ffi::c_uint);
    }

    0
}

unsafe extern "C" fn wm8961_spk_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: core::ffi::c_int,
) -> core::ffi::c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let mut pwr_reg: u16 = snd_soc_component_read(component, WM8961_PWR_MGMT_2) as u16;
    let mut spk_reg: u16 = snd_soc_component_read(component, WM8961_CLASS_D_CONTROL_1) as u16;

    if event & SND_SOC_DAPM_POST_PMU != 0 {
        /* Enable the PGA */
        pwr_reg |= (WM8961_SPKL_PGA | WM8961_SPKR_PGA) as u16;
        snd_soc_component_write(component, WM8961_PWR_MGMT_2, pwr_reg as core::ffi::c_uint);

        /* Enable the amplifier */
        spk_reg |= (WM8961_SPKL_ENA | WM8961_SPKR_ENA) as u16;
        snd_soc_component_write(component, WM8961_CLASS_D_CONTROL_1, spk_reg as core::ffi::c_uint);
    }

    if event & SND_SOC_DAPM_PRE_PMD != 0 {
        /* Disable the amplifier */
        spk_reg &= !(WM8961_SPKL_ENA | WM8961_SPKR_ENA) as u16;
        snd_soc_component_write(component, WM8961_CLASS_D_CONTROL_1, spk_reg as core::ffi::c_uint);

        /* Disable the PGA */
        pwr_reg &= !(WM8961_SPKL_PGA | WM8961_SPKR_PGA) as u16;
        snd_soc_component_write(component, WM8961_PWR_MGMT_2, pwr_reg as core::ffi::c_uint);
    }

    0
}

static adc_hpf_text: [&'static [u8]; 4] = [b"Hi-fi\0", b"Voice 1\0", b"Voice 2\0", b"Voice 3\0"];
SOC_ENUM_SINGLE_DECL!(adc_hpf, WM8961_ADC_DAC_CONTROL_2, 7, adc_hpf_text);

static dac_deemph_text: [&'static [u8]; 4] = [b"None\0", b"32kHz\0", b"44.1kHz\0", b"48kHz\0"];
SOC_ENUM_SINGLE_DECL!(dac_deemph, WM8961_ADC_DAC_CONTROL_1, 1, dac_deemph_text);

DECLARE_TLV_DB_SCALE!(out_tlv, -12100, 100, 1);
DECLARE_TLV_DB_SCALE!(hp_sec_tlv, -700, 100, 0);
DECLARE_TLV_DB_SCALE!(adc_tlv, -7200, 75, 1);
DECLARE_TLV_DB_SCALE!(sidetone_tlv, -3600, 300, 0);
DECLARE_TLV_DB_RANGE!(boost_tlv,
    0, 0, TLV_DB_SCALE_ITEM!(0, 0, 0),
    1, 1, TLV_DB_SCALE_ITEM!(13, 0, 0),
    2, 2, TLV_DB_SCALE_ITEM!(20, 0, 0),
    3, 3, TLV_DB_SCALE_ITEM!(29, 0, 0)
);
DECLARE_TLV_DB_SCALE!(pga_tlv, -2325, 75, 0);

static wm8961_snd_controls: [snd_kcontrol_new; 18] = [
    SOC_DOUBLE_R_TLV!("Headphone Volume", WM8961_LOUT1_VOLUME, WM8961_ROUT1_VOLUME, 0, 127, 0, out_tlv),
    SOC_DOUBLE_TLV!("Headphone Secondary Volume", WM8961_ANALOGUE_HP_2, 6, 3, 7, 0, hp_sec_tlv),
    SOC_DOUBLE_R!("Headphone ZC Switch", WM8961_LOUT1_VOLUME, WM8961_ROUT1_VOLUME, 7, 1, 0),
    SOC_DOUBLE_R_TLV!("Speaker Volume", WM8961_LOUT2_VOLUME, WM8961_ROUT2_VOLUME, 0, 127, 0, out_tlv),
    SOC_DOUBLE_R!("Speaker ZC Switch", WM8961_LOUT2_VOLUME, WM8961_ROUT2_VOLUME, 7, 1, 0),
    SOC_SINGLE!("Speaker AC Gain", WM8961_CLASS_D_CONTROL_2, 0, 7, 0),
    SOC_SINGLE!("DAC x128 OSR Switch", WM8961_ADC_DAC_CONTROL_2, 0, 1, 0),
    SOC_ENUM!("DAC Deemphasis", dac_deemph),
    SOC_SINGLE!("DAC Soft Mute Switch", WM8961_ADC_DAC_CONTROL_2, 3, 1, 0),
    SOC_DOUBLE_R_TLV!("Sidetone Volume", WM8961_DSP_SIDETONE_0, WM8961_DSP_SIDETONE_1, 4, 12, 0, sidetone_tlv),
    SOC_SINGLE!("ADC High Pass Filter Switch", WM8961_ADC_DAC_CONTROL_1, 0, 1, 0),
    SOC_ENUM!("ADC High Pass Filter Mode", adc_hpf),
    SOC_DOUBLE_R_TLV!("Capture Volume", WM8961_LEFT_ADC_VOLUME, WM8961_RIGHT_ADC_VOLUME, 1, 119, 0, adc_tlv),
    SOC_DOUBLE_R_TLV!("Capture Boost Volume", WM8961_ADCL_SIGNAL_PATH, WM8961_ADCR_SIGNAL_PATH, 4, 3, 0, boost_tlv),
    SOC_DOUBLE_R_TLV!("Capture PGA Volume", WM8961_LEFT_INPUT_VOLUME, WM8961_RIGHT_INPUT_VOLUME, 0, 62, 0, pga_tlv),
    SOC_DOUBLE_R!("Capture PGA ZC Switch", WM8961_LEFT_INPUT_VOLUME, WM8961_RIGHT_INPUT_VOLUME, 6, 1, 1),
    SOC_DOUBLE_R!("Capture PGA Switch", WM8961_LEFT_INPUT_VOLUME, WM8961_RIGHT_INPUT_VOLUME, 7, 1, 1),
];

static sidetone_text: [&'static [u8]; 3] = [b"None\0", b"Left\0", b"Right\0"];
SOC_ENUM_SINGLE_DECL!(dacl_sidetone, WM8961_DSP_SIDETONE_0, 2, sidetone_text);
SOC_ENUM_SINGLE_DECL!(dacr_sidetone, WM8961_DSP_SIDETONE_1, 2, sidetone_text);

static dacl_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("DACL Sidetone", dacl_sidetone);
static dacr_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("DACR Sidetone", dacr_sidetone);

static wm8961_dapm_widgets: [snd_soc_dapm_widget; 20] = [
    SND_SOC_DAPM_INPUT!("LINPUT"),
    SND_SOC_DAPM_INPUT!("RINPUT"),
    SND_SOC_DAPM_SUPPLY!("CLK_DSP", WM8961_CLOCKING2, 4, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Left Input", WM8961_PWR_MGMT_1, 5, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Right Input", WM8961_PWR_MGMT_1, 4, 0, NULL, 0),
    SND_SOC_DAPM_ADC!("ADCL", "HiFi Capture", WM8961_PWR_MGMT_1, 3, 0),
    SND_SOC_DAPM_ADC!("ADCR", "HiFi Capture", WM8961_PWR_MGMT_1, 2, 0),
    SND_SOC_DAPM_SUPPLY!("MICBIAS", WM8961_PWR_MGMT_1, 1, 0, NULL, 0),
    SND_SOC_DAPM_MUX!("DACL Sidetone", SND_SOC_NOPM, 0, 0, &dacl_mux),
    SND_SOC_DAPM_MUX!("DACR Sidetone", SND_SOC_NOPM, 0, 0, &dacr_mux),
    SND_SOC_DAPM_DAC!("DACL", "HiFi Playback", WM8961_PWR_MGMT_2, 8, 0),
    SND_SOC_DAPM_DAC!("DACR", "HiFi Playback", WM8961_PWR_MGMT_2, 7, 0),
    /* Handle as a mono path for DCS */
    SND_SOC_DAPM_PGA_E!("Headphone Output", SND_SOC_NOPM, 4, 0, NULL, 0, wm8961_hp_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_PGA_E!("Speaker Output", SND_SOC_NOPM, 4, 0, NULL, 0, wm8961_spk_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_OUTPUT!("HP_L"),
    SND_SOC_DAPM_OUTPUT!("HP_R"),
    SND_SOC_DAPM_OUTPUT!("SPK_LN"),
    SND_SOC_DAPM_OUTPUT!("SPK_LP"),
    SND_SOC_DAPM_OUTPUT!("SPK_RN"),
    SND_SOC_DAPM_OUTPUT!("SPK_RP"),
];

static audio_paths: [snd_soc_dapm_route; 29] = [
    snd_soc_dapm_route { sink: b"DACL\0".as_ptr() as _, control: core::ptr::null(), source: b"CLK_DSP\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"DACL\0".as_ptr() as _, control: core::ptr::null(), source: b"DACL Sidetone\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"DACR\0".as_ptr() as _, control: core::ptr::null(), source: b"CLK_DSP\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"DACR\0".as_ptr() as _, control: core::ptr::null(), source: b"DACR Sidetone\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"DACL Sidetone\0".as_ptr() as _, control: b"Left\0".as_ptr() as _, source: b"ADCL\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"DACL Sidetone\0".as_ptr() as _, control: b"Right\0".as_ptr() as _, source: b"ADCR\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"DACR Sidetone\0".as_ptr() as _, control: b"Left\0".as_ptr() as _, source: b"ADCL\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"DACR Sidetone\0".as_ptr() as _, control: b"Right\0".as_ptr() as _, source: b"ADCR\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"HP_L\0".as_ptr() as _, control: core::ptr::null(), source: b"Headphone Output\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"HP_R\0".as_ptr() as _, control: core::ptr::null(), source: b"Headphone Output\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"Headphone Output\0".as_ptr() as _, control: core::ptr::null(), source: b"DACL\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"Headphone Output\0".as_ptr() as _, control: core::ptr::null(), source: b"DACR\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"SPK_LN\0".as_ptr() as _, control: core::ptr::null(), source: b"Speaker Output\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"SPK_LP\0".as_ptr() as _, control: core::ptr::null(), source: b"Speaker Output\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"SPK_RN\0".as_ptr() as _, control: core::ptr::null(), source: b"Speaker Output\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"SPK_RP\0".as_ptr() as _, control: core::ptr::null(), source: b"Speaker Output\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"Speaker Output\0".as_ptr() as _, control: core::ptr::null(), source: b"DACL\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"Speaker Output\0".as_ptr() as _, control: core::ptr::null(), source: b"DACR\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"ADCL\0".as_ptr() as _, control: core::ptr::null(), source: b"Left Input\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"ADCL\0".as_ptr() as _, control: core::ptr::null(), source: b"CLK_DSP\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"ADCR\0".as_ptr() as _, control: core::ptr::null(), source: b"Right Input\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"ADCR\0".as_ptr() as _, control: core::ptr::null(), source: b"CLK_DSP\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"Left Input\0".as_ptr() as _, control: core::ptr::null(), source: b"LINPUT\0".as_ptr() as _ },
    snd_soc_dapm_route { sink: b"Right Input\0".as_ptr() as _, control: core::ptr::null(), source: b"RINPUT\0".as_ptr() as _ },
];

#[repr(C)]
struct wm8961_ratio { ratio: core::ffi::c_int, val: u16 }

/* Values for CLK_SYS_RATE */
static mut wm8961_clk_sys_ratio: [wm8961_ratio; 10] = [
    wm8961_ratio { ratio: 64, val: 0 },
    wm8961_ratio { ratio: 128, val: 1 },
    wm8961_ratio { ratio: 192, val: 2 },
    wm8961_ratio { ratio: 256, val: 3 },
    wm8961_ratio { ratio: 384, val: 4 },
    wm8961_ratio { ratio: 512, val: 5 },
    wm8961_ratio { ratio: 768, val: 6 },
    wm8961_ratio { ratio: 1024, val: 7 },
    wm8961_ratio { ratio: 1408, val: 8 },
    wm8961_ratio { ratio: 1536, val: 9 },
];

#[repr(C)]
struct wm8961_rate { rate: core::ffi::c_int, val: u16 }

/* Values for SAMPLE_RATE */
static mut wm8961_srate: [wm8961_rate; 9] = [
    wm8961_rate { rate: 48000, val: 0 },
    wm8961_rate { rate: 44100, val: 0 },
    wm8961_rate { rate: 32000, val: 1 },
    wm8961_rate { rate: 22050, val: 2 },
    wm8961_rate { rate: 24000, val: 2 },
    wm8961_rate { rate: 16000, val: 3 },
    wm8961_rate { rate: 11250, val: 4 },
    wm8961_rate { rate: 12000, val: 4 },
    wm8961_rate { rate: 8000, val: 5 },
];

unsafe extern "C" fn wm8961_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let component = (*dai).component;
    let wm8961 = snd_soc_component_get_drvdata(component) as *mut wm8961_priv;
    let mut i: usize;
    let mut best: usize;
    let target: core::ffi::c_int;
    let fs: core::ffi::c_int = params_rate(params);
    let mut reg: u16;

    if (*wm8961).sysclk == 0 {
        dev_err!((*component).dev, "MCLK has not been specified\n");
        return -EINVAL;
    }

    /* Find the closest sample rate for the filters */
    best = 0;
    i = 0;
    while i < wm8961_srate.len() {
        if (wm8961_srate[i].rate - fs).abs() < (wm8961_srate[best].rate - fs).abs() {
            best = i;
        }
        i += 1;
    }
    reg = snd_soc_component_read(component, WM8961_ADDITIONAL_CONTROL_3) as u16;
    reg &= !(WM8961_SAMPLE_RATE_MASK as u16);
    reg |= wm8961_srate[best].val;
    snd_soc_component_write(component, WM8961_ADDITIONAL_CONTROL_3, reg as core::ffi::c_uint);
    dev_dbg!((*component).dev, "Selected SRATE %dHz for %dHz\n", wm8961_srate[best].rate, fs);

    /* Select a CLK_SYS/fs ratio equal to or higher than required */
    target = (*wm8961).sysclk / fs;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK && target < 64 {
        dev_err!((*component).dev, "SYSCLK must be at least 64*fs for DAC\n");
        return -EINVAL;
    }
    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE && target < 256 {
        dev_err!((*component).dev, "SYSCLK must be at least 256*fs for ADC\n");
        return -EINVAL;
    }

    i = 0;
    while i < wm8961_clk_sys_ratio.len() {
        if wm8961_clk_sys_ratio[i].ratio >= target {
            break;
        }
        i += 1;
    }
    if i == wm8961_clk_sys_ratio.len() {
        dev_err!((*component).dev, "Unable to generate CLK_SYS_RATE\n");
        return -EINVAL;
    }
    dev_dbg!((*component).dev, "Selected CLK_SYS_RATE of %d for %d/%d=%d\n",
        wm8961_clk_sys_ratio[i].ratio, (*wm8961).sysclk, fs, (*wm8961).sysclk / fs);

    reg = snd_soc_component_read(component, WM8961_CLOCKING_4) as u16;
    reg &= !(WM8961_CLK_SYS_RATE_MASK as u16);
    reg |= (wm8961_clk_sys_ratio[i].val as u32 << WM8961_CLK_SYS_RATE_SHIFT) as u16;
    snd_soc_component_write(component, WM8961_CLOCKING_4, reg as core::ffi::c_uint);

    reg = snd_soc_component_read(component, WM8961_AUDIO_INTERFACE_0) as u16;
    reg &= !(WM8961_WL_MASK as u16);
    match params_width(params) {
        16 => {}
        20 => reg |= (1 << WM8961_WL_SHIFT) as u16,
        24 => reg |= (2 << WM8961_WL_SHIFT) as u16,
        32 => reg |= (3 << WM8961_WL_SHIFT) as u16,
        _ => return -EINVAL,
    }
    snd_soc_component_write(component, WM8961_AUDIO_INTERFACE_0, reg as core::ffi::c_uint);

    /* Sloping stop-band filter is recommended for <= 24kHz */
    reg = snd_soc_component_read(component, WM8961_ADC_DAC_CONTROL_2) as u16;
    if fs <= 24000 {
        reg |= WM8961_DACSLOPE as u16;
    } else {
        reg &= !(WM8961_DACSLOPE as u16);
    }
    snd_soc_component_write(component, WM8961_ADC_DAC_CONTROL_2, reg as core::ffi::c_uint);

    0
}

unsafe extern "C" fn wm8961_set_sysclk(
    dai: *mut snd_soc_dai,
    _clk_id: core::ffi::c_int,
    mut freq: core::ffi::c_uint,
    _dir: core::ffi::c_int,
) -> core::ffi::c_int {
    let component = (*dai).component;
    let wm8961 = snd_soc_component_get_drvdata(component) as *mut wm8961_priv;
    let mut reg: u16 = snd_soc_component_read(component, WM8961_CLOCKING1) as u16;

    if freq > 33000000 {
        dev_err!((*component).dev, "MCLK must be <33MHz\n");
        return -EINVAL;
    }

    if freq > 16500000 {
        dev_dbg!((*component).dev, "Using MCLK/2 for %dHz MCLK\n", freq);
        reg |= WM8961_MCLKDIV as u16;
        freq /= 2;
    } else {
        dev_dbg!((*component).dev, "Using MCLK/1 for %dHz MCLK\n", freq);
        reg &= !(WM8961_MCLKDIV as u16);
    }

    snd_soc_component_write(component, WM8961_CLOCKING1, reg as core::ffi::c_uint);
    (*wm8961).sysclk = freq as core::ffi::c_int;
    0
}

unsafe extern "C" fn wm8961_set_fmt(dai: *mut snd_soc_dai, fmt: core::ffi::c_uint) -> core::ffi::c_int {
    let component = (*dai).component;
    let mut aif: u16 = snd_soc_component_read(component, WM8961_AUDIO_INTERFACE_0) as u16;

    aif &= !((WM8961_BCLKINV | WM8961_LRP | WM8961_MS | WM8961_FORMAT_MASK) as u16);

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => aif |= WM8961_MS as u16,
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_RIGHT_J => {}
        SND_SOC_DAIFMT_LEFT_J => aif |= 1,
        SND_SOC_DAIFMT_I2S => aif |= 2,
        SND_SOC_DAIFMT_DSP_B => {
            aif |= WM8961_LRP as u16;
            aif |= 3;
            match fmt & SND_SOC_DAIFMT_INV_MASK {
                SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_IB_NF => {}
                _ => return -EINVAL,
            }
        }
        SND_SOC_DAIFMT_DSP_A => {
            aif |= 3;
            match fmt & SND_SOC_DAIFMT_INV_MASK {
                SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_IB_NF => {}
                _ => return -EINVAL,
            }
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_NB_IF => aif |= WM8961_LRP as u16,
        SND_SOC_DAIFMT_IB_NF => aif |= WM8961_BCLKINV as u16,
        SND_SOC_DAIFMT_IB_IF => aif |= (WM8961_BCLKINV | WM8961_LRP) as u16,
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, WM8961_AUDIO_INTERFACE_0, aif as core::ffi::c_uint)
}

unsafe extern "C" fn wm8961_set_tristate(dai: *mut snd_soc_dai, tristate: core::ffi::c_int) -> core::ffi::c_int {
    let component = (*dai).component;
    let mut reg: u16 = snd_soc_component_read(component, WM8961_ADDITIONAL_CONTROL_2) as u16;

    if tristate != 0 {
        reg |= WM8961_TRIS as u16;
    } else {
        reg &= !(WM8961_TRIS as u16);
    }

    snd_soc_component_write(component, WM8961_ADDITIONAL_CONTROL_2, reg as core::ffi::c_uint)
}

unsafe extern "C" fn wm8961_mute(
    dai: *mut snd_soc_dai,
    mute: core::ffi::c_int,
    _direction: core::ffi::c_int,
) -> core::ffi::c_int {
    let component = (*dai).component;
    let mut reg: u16 = snd_soc_component_read(component, WM8961_ADC_DAC_CONTROL_1) as u16;

    if mute != 0 {
        reg |= WM8961_DACMU as u16;
    } else {
        reg &= !(WM8961_DACMU as u16);
    }

    msleep(17);
    snd_soc_component_write(component, WM8961_ADC_DAC_CONTROL_1, reg as core::ffi::c_uint)
}

unsafe extern "C" fn wm8961_set_clkdiv(
    dai: *mut snd_soc_dai,
    div_id: core::ffi::c_int,
    div: core::ffi::c_int,
) -> core::ffi::c_int {
    let component = (*dai).component;
    let mut reg: u16;

    match div_id {
        WM8961_BCLK => {
            reg = snd_soc_component_read(component, WM8961_CLOCKING2) as u16;
            reg &= !(WM8961_BCLKDIV_MASK as u16);
            reg |= div as u16;
            snd_soc_component_write(component, WM8961_CLOCKING2, reg as core::ffi::c_uint);
        }
        WM8961_LRCLK => {
            reg = snd_soc_component_read(component, WM8961_AUDIO_INTERFACE_2) as u16;
            reg &= !(WM8961_LRCLK_RATE_MASK as u16);
            reg |= div as u16;
            snd_soc_component_write(component, WM8961_AUDIO_INTERFACE_2, reg as core::ffi::c_uint);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn wm8961_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> core::ffi::c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let mut reg: u16;

    /* This is all slightly unusual since we have no bypass paths
     * and the output amplifier structure means we can just slam
     * the biases straight up rather than having to ramp them
     * slowly.
     */
    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_STANDBY {
                /* Enable bias generation */
                reg = snd_soc_component_read(component, WM8961_ANTI_POP) as u16;
                reg |= (WM8961_BUFIOEN | WM8961_BUFDCOPEN) as u16;
                snd_soc_component_write(component, WM8961_ANTI_POP, reg as core::ffi::c_uint);

                /* VMID=2*50k, VREF */
                reg = snd_soc_component_read(component, WM8961_PWR_MGMT_1) as u16;
                reg &= !(WM8961_VMIDSEL_MASK as u16);
                reg |= ((1 << WM8961_VMIDSEL_SHIFT) | WM8961_VREF) as u16;
                snd_soc_component_write(component, WM8961_PWR_MGMT_1, reg as core::ffi::c_uint);
            }
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_PREPARE {
                /* VREF off */
                reg = snd_soc_component_read(component, WM8961_PWR_MGMT_1) as u16;
                reg &= !(WM8961_VREF as u16);
                snd_soc_component_write(component, WM8961_PWR_MGMT_1, reg as core::ffi::c_uint);

                /* Bias generation off */
                reg = snd_soc_component_read(component, WM8961_ANTI_POP) as u16;
                reg &= !((WM8961_BUFIOEN | WM8961_BUFDCOPEN) as u16);
                snd_soc_component_write(component, WM8961_ANTI_POP, reg as core::ffi::c_uint);

                /* VMID off */
                reg = snd_soc_component_read(component, WM8961_PWR_MGMT_1) as u16;
                reg &= !(WM8961_VMIDSEL_MASK as u16);
                snd_soc_component_write(component, WM8961_PWR_MGMT_1, reg as core::ffi::c_uint);
            }
        }
        SND_SOC_BIAS_OFF => {}
    }

    0
}

const WM8961_RATES: u32 = SNDRV_PCM_RATE_8000_48000;
const WM8961_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

static wm8961_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wm8961_hw_params),
    set_sysclk: Some(wm8961_set_sysclk),
    set_fmt: Some(wm8961_set_fmt),
    mute_stream: Some(wm8961_mute),
    set_tristate: Some(wm8961_set_tristate),
    set_clkdiv: Some(wm8961_set_clkdiv),
    no_capture_mute: 1,
    ..unsafe { core::mem::zeroed() }
};

static mut wm8961_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"wm8961-hifi\0".as_ptr() as _,
    playback: snd_soc_pcm_stream {
        stream_name: b"HiFi Playback\0".as_ptr() as _,
        channels_min: 1,
        channels_max: 2,
        rates: WM8961_RATES,
        formats: WM8961_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"HiFi Capture\0".as_ptr() as _,
        channels_min: 1,
        channels_max: 2,
        rates: WM8961_RATES,
        formats: WM8961_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &wm8961_dai_ops,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn wm8961_probe(component: *mut snd_soc_component) -> core::ffi::c_int {
    let mut reg: u16;

    /* Enable class W */
    reg = snd_soc_component_read(component, WM8961_CHARGE_PUMP_B) as u16;
    reg |= WM8961_CP_DYN_PWR_MASK as u16;
    snd_soc_component_write(component, WM8961_CHARGE_PUMP_B, reg as core::ffi::c_uint);

    /* Latch volume update bits (right channel only, we always
     * write both out) and default ZC on. */
    reg = snd_soc_component_read(component, WM8961_ROUT1_VOLUME) as u16;
    snd_soc_component_write(component, WM8961_ROUT1_VOLUME, (reg | (WM8961_LO1ZC | WM8961_OUT1VU) as u16) as core::ffi::c_uint);
    snd_soc_component_write(component, WM8961_LOUT1_VOLUME, (reg | WM8961_LO1ZC as u16) as core::ffi::c_uint);
    reg = snd_soc_component_read(component, WM8961_ROUT2_VOLUME) as u16;
    snd_soc_component_write(component, WM8961_ROUT2_VOLUME, (reg | (WM8961_SPKRZC | WM8961_SPKVU) as u16) as core::ffi::c_uint);
    snd_soc_component_write(component, WM8961_LOUT2_VOLUME, (reg | WM8961_SPKLZC as u16) as core::ffi::c_uint);

    reg = snd_soc_component_read(component, WM8961_RIGHT_ADC_VOLUME) as u16;
    snd_soc_component_write(component, WM8961_RIGHT_ADC_VOLUME, (reg | WM8961_ADCVU as u16) as core::ffi::c_uint);
    reg = snd_soc_component_read(component, WM8961_RIGHT_INPUT_VOLUME) as u16;
    snd_soc_component_write(component, WM8961_RIGHT_INPUT_VOLUME, (reg | WM8961_IPVU as u16) as core::ffi::c_uint);

    /* Use soft mute by default */
    reg = snd_soc_component_read(component, WM8961_ADC_DAC_CONTROL_2) as u16;
    reg |= WM8961_DACSMM as u16;
    snd_soc_component_write(component, WM8961_ADC_DAC_CONTROL_2, reg as core::ffi::c_uint);

    /* Use automatic clocking mode by default; for now this is all
     * we support.
     */
    reg = snd_soc_component_read(component, WM8961_CLOCKING_3) as u16;
    reg &= !(WM8961_MANUAL_MODE as u16);
    snd_soc_component_write(component, WM8961_CLOCKING_3, reg as core::ffi::c_uint);

    0
}

// CONFIG_PM conditional in C: wm8961_resume is present only with power management.
#[cfg(CONFIG_PM)]
unsafe extern "C" fn wm8961_resume(component: *mut snd_soc_component) -> core::ffi::c_int {
    snd_soc_component_cache_sync(component);
    0
}

#[cfg(CONFIG_PM)]
const WM8961_RESUME: Option<unsafe extern "C" fn(*mut snd_soc_component) -> core::ffi::c_int> = Some(wm8961_resume);
#[cfg(not(CONFIG_PM))]
const WM8961_RESUME: Option<unsafe extern "C" fn(*mut snd_soc_component) -> core::ffi::c_int> = None;

static soc_component_dev_wm8961: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm8961_probe),
    resume: WM8961_RESUME,
    set_bias_level: Some(wm8961_set_bias_level),
    controls: wm8961_snd_controls.as_ptr(),
    num_controls: wm8961_snd_controls.len() as core::ffi::c_uint,
    dapm_widgets: wm8961_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm8961_dapm_widgets.len() as core::ffi::c_uint,
    dapm_routes: audio_paths.as_ptr(),
    num_dapm_routes: audio_paths.len() as core::ffi::c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

static wm8961_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 16,
    max_register: WM8961_MAX_REGISTER,
    reg_defaults: wm8961_reg_defaults.as_ptr(),
    num_reg_defaults: wm8961_reg_defaults.len() as core::ffi::c_uint,
    cache_type: REGCACHE_MAPLE,
    volatile_reg: Some(wm8961_volatile),
    readable_reg: Some(wm8961_readable),
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn wm8961_i2c_probe(i2c: *mut i2c_client) -> core::ffi::c_int {
    let wm8961: *mut wm8961_priv;
    let mut val: core::ffi::c_uint = 0;
    let mut ret: core::ffi::c_int;

    wm8961 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<wm8961_priv>(), GFP_KERNEL) as *mut wm8961_priv;
    if wm8961.is_null() {
        return -ENOMEM;
    }

    (*wm8961).regmap = devm_regmap_init_i2c(i2c, &wm8961_regmap);
    if IS_ERR((*wm8961).regmap as _) {
        return PTR_ERR((*wm8961).regmap as _) as core::ffi::c_int;
    }

    ret = regmap_read((*wm8961).regmap, WM8961_SOFTWARE_RESET, &mut val);
    if ret != 0 {
        dev_err!(&mut (*i2c).dev, "Failed to read chip ID: %d\n", ret);
        return ret;
    }

    if val != 0x1801 {
        dev_err!(&mut (*i2c).dev, "Device is not a WM8961: ID=0x%x\n", val);
        return -EINVAL;
    }

    /* This isn't volatile - readback doesn't correspond to write */
    regcache_cache_bypass((*wm8961).regmap, true);
    ret = regmap_read((*wm8961).regmap, WM8961_RIGHT_INPUT_VOLUME, &mut val);
    regcache_cache_bypass((*wm8961).regmap, false);

    if ret != 0 {
        dev_err!(&mut (*i2c).dev, "Failed to read chip revision: %d\n", ret);
        return ret;
    }

    dev_info!(&mut (*i2c).dev, "WM8961 family %d revision %c\n",
        (val & WM8961_DEVICE_ID_MASK) >> WM8961_DEVICE_ID_SHIFT,
        ((val & WM8961_CHIP_REV_MASK) >> WM8961_CHIP_REV_SHIFT) + b'A' as core::ffi::c_uint);

    ret = regmap_write((*wm8961).regmap, WM8961_SOFTWARE_RESET, 0x1801);
    if ret != 0 {
        dev_err!(&mut (*i2c).dev, "Failed to issue reset: %d\n", ret);
        return ret;
    }

    i2c_set_clientdata(i2c, wm8961 as *mut core::ffi::c_void);

    ret = devm_snd_soc_register_component(&mut (*i2c).dev,
        &soc_component_dev_wm8961, &mut wm8961_dai, 1);

    ret
}

static wm8961_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: *b"wm8961\0", ..unsafe { core::mem::zeroed() } },
    i2c_device_id { ..unsafe { core::mem::zeroed() } },
];
MODULE_DEVICE_TABLE!(i2c, wm8961_i2c_id);

static wm8961_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"wlf,wm8961\0".as_ptr() as _, ..unsafe { core::mem::zeroed() } },
    of_device_id { ..unsafe { core::mem::zeroed() } },
];
MODULE_DEVICE_TABLE!(of, wm8961_of_match);

static mut wm8961_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"wm8961\0".as_ptr() as _,
        of_match_table: of_match_ptr(wm8961_of_match.as_ptr()),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(wm8961_i2c_probe),
    id_table: wm8961_i2c_id.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

module_i2c_driver!(wm8961_i2c_driver);

MODULE_DESCRIPTION!("ASoC WM8961 driver");
MODULE_AUTHOR!("Mark Brown <broonie@opensource.wolfsonmicro.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
