// SPDX-License-Identifier: GPL-2.0-only
/*
 * card driver for models with WM8776/WM8766 DACs (Xonar DS/HDAV1.3 Slim)
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

/*
 * Xonar DS
 * --------
 *
 * CMI8788:
 *
 *   SPI 0 -> WM8766 (surround, center/LFE, back)
 *   SPI 1 -> WM8776 (front, input)
 *
 *   GPIO 4 <- headphone detect, 0 = plugged
 *   GPIO 6 -> route input jack to mic-in (0) or line-in (1)
 *   GPIO 7 -> enable output to front L/R speaker channels
 *   GPIO 8 -> enable output to other speaker channels and front panel headphone
 *
 * WM8776:
 *
 *   input 1 <- line
 *   input 2 <- mic
 *   input 3 <- front mic
 *   input 4 <- aux
 */

/*
 * Xonar HDAV1.3 Slim
 * ------------------
 *
 * CMI8788:
 *
 *   I2C <-> WM8776 (addr 0011010)
 *
 *   GPIO 0  -> disable HDMI output
 *   GPIO 1  -> enable HP output
 *   GPIO 6  -> firmware EEPROM I2C clock
 *   GPIO 7 <-> firmware EEPROM I2C data
 *
 *   UART <-> HDMI controller
 *
 * WM8776:
 *
 *   input 1 <- mic
 *   input 2 <- aux
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const GPIO_DS_HP_DETECT: c_uint = 0x0010;
const GPIO_DS_INPUT_ROUTE: c_uint = 0x0040;
const GPIO_DS_OUTPUT_FRONTLR: c_uint = 0x0080;
const GPIO_DS_OUTPUT_ENABLE: c_uint = 0x0100;

const GPIO_SLIM_HDMI_DISABLE: c_uint = 0x0001;
const GPIO_SLIM_OUTPUT_ENABLE: c_uint = 0x0002;
const GPIO_SLIM_FIRMWARE_CLK: c_uint = 0x0040;
const GPIO_SLIM_FIRMWARE_DATA: c_uint = 0x0080;

const I2C_DEVICE_WM8776: c_uint = 0x34; /* 001101, 0, /W=0 */

const LC_CONTROL_LIMITER: c_ulong = 0x40000000;
const LC_CONTROL_ALC: c_ulong = 0x20000000;

#[repr(C)]
pub struct xonar_wm87x6 {
    pub generic: xonar_generic,
    pub wm8776_regs: [u16; 0x17],
    pub wm8766_regs: [u16; 0x10],
    pub line_adcmux_control: *mut snd_kcontrol,
    pub mic_adcmux_control: *mut snd_kcontrol,
    pub lc_controls: [*mut snd_kcontrol; 13],
    pub hp_jack: *mut snd_jack,
    pub hdmi: xonar_hdmi,
}

unsafe fn data(chip: *mut oxygen) -> *mut xonar_wm87x6 {
    (*chip).model_data as *mut xonar_wm87x6
}

unsafe fn wm8776_write_spi(chip: *mut oxygen, reg: c_uint, value: c_uint) {
    oxygen_write_spi(
        chip,
        OXYGEN_SPI_TRIGGER
            | OXYGEN_SPI_DATA_LENGTH_2
            | OXYGEN_SPI_CLOCK_160
            | (1 << OXYGEN_SPI_CODEC_SHIFT)
            | OXYGEN_SPI_CEN_LATCH_CLOCK_LO,
        (reg << 9) | value,
    );
}

unsafe fn wm8776_write_i2c(chip: *mut oxygen, reg: c_uint, value: c_uint) {
    oxygen_write_i2c(chip, I2C_DEVICE_WM8776, (reg << 1) | (value >> 8), value);
}

unsafe fn wm8776_write(chip: *mut oxygen, reg: c_uint, mut value: c_uint) {
    let data = data(chip);

    if ((*chip).model.function_flags & OXYGEN_FUNCTION_2WIRE_SPI_MASK) == OXYGEN_FUNCTION_SPI {
        wm8776_write_spi(chip, reg, value);
    } else {
        wm8776_write_i2c(chip, reg, value);
    }
    if (reg as usize) < (*data).wm8776_regs.len() {
        /* reg >= WM8776_HPLVOL is always true */
        if reg <= WM8776_DACMASTER {
            value &= !WM8776_UPDATE;
        }
        (*data).wm8776_regs[reg as usize] = value as u16;
    }
}

unsafe fn wm8776_write_cached(chip: *mut oxygen, reg: c_uint, value: c_uint) {
    let data = data(chip);

    if (reg as usize) >= (*data).wm8776_regs.len()
        || value as u16 != (*data).wm8776_regs[reg as usize]
    {
        wm8776_write(chip, reg, value);
    }
}

unsafe fn wm8766_write(chip: *mut oxygen, reg: c_uint, mut value: c_uint) {
    let data = data(chip);

    oxygen_write_spi(
        chip,
        OXYGEN_SPI_TRIGGER
            | OXYGEN_SPI_DATA_LENGTH_2
            | OXYGEN_SPI_CLOCK_160
            | (0 << OXYGEN_SPI_CODEC_SHIFT)
            | OXYGEN_SPI_CEN_LATCH_CLOCK_LO,
        (reg << 9) | value,
    );
    if (reg as usize) < (*data).wm8766_regs.len() {
        /* reg >= WM8766_LDA1 is always true */
        if reg <= WM8766_RDA1 || (reg >= WM8766_LDA2 && reg <= WM8766_MASTDA) {
            value &= !WM8766_UPDATE;
        }
        (*data).wm8766_regs[reg as usize] = value as u16;
    }
}

unsafe fn wm8766_write_cached(chip: *mut oxygen, reg: c_uint, value: c_uint) {
    let data = data(chip);

    if (reg as usize) >= (*data).wm8766_regs.len()
        || value as u16 != (*data).wm8766_regs[reg as usize]
    {
        wm8766_write(chip, reg, value);
    }
}

unsafe fn wm8776_registers_init(chip: *mut oxygen) {
    let data = data(chip);

    wm8776_write(chip, WM8776_RESET, 0);
    wm8776_write(chip, WM8776_PHASESWAP, WM8776_PH_MASK);
    wm8776_write(
        chip,
        WM8776_DACCTRL1,
        WM8776_DZCEN | WM8776_PL_LEFT_LEFT | WM8776_PL_RIGHT_RIGHT,
    );
    wm8776_write(
        chip,
        WM8776_DACMUTE,
        if (*chip).dac_mute { WM8776_DMUTE } else { 0 },
    );
    wm8776_write(chip, WM8776_DACIFCTRL, WM8776_DACFMT_LJUST | WM8776_DACWL_24);
    wm8776_write(chip, WM8776_ADCIFCTRL, (*data).wm8776_regs[WM8776_ADCIFCTRL as usize] as c_uint);
    wm8776_write(chip, WM8776_MSTRCTRL, (*data).wm8776_regs[WM8776_MSTRCTRL as usize] as c_uint);
    wm8776_write(chip, WM8776_PWRDOWN, (*data).wm8776_regs[WM8776_PWRDOWN as usize] as c_uint);
    wm8776_write(chip, WM8776_HPLVOL, (*data).wm8776_regs[WM8776_HPLVOL as usize] as c_uint);
    wm8776_write(
        chip,
        WM8776_HPRVOL,
        (*data).wm8776_regs[WM8776_HPRVOL as usize] as c_uint | WM8776_UPDATE,
    );
    wm8776_write(chip, WM8776_ADCLVOL, (*data).wm8776_regs[WM8776_ADCLVOL as usize] as c_uint);
    wm8776_write(chip, WM8776_ADCRVOL, (*data).wm8776_regs[WM8776_ADCRVOL as usize] as c_uint);
    wm8776_write(chip, WM8776_ADCMUX, (*data).wm8776_regs[WM8776_ADCMUX as usize] as c_uint);
    wm8776_write(chip, WM8776_DACLVOL, (*chip).dac_volume[0] as c_uint);
    wm8776_write(chip, WM8776_DACRVOL, (*chip).dac_volume[1] as c_uint | WM8776_UPDATE);
}

unsafe fn wm8766_registers_init(chip: *mut oxygen) {
    let data = data(chip);

    wm8766_write(chip, WM8766_RESET, 0);
    wm8766_write(chip, WM8766_DAC_CTRL, (*data).wm8766_regs[WM8766_DAC_CTRL as usize] as c_uint);
    wm8766_write(chip, WM8766_INT_CTRL, WM8766_FMT_LJUST | WM8766_IWL_24);
    wm8766_write(
        chip,
        WM8766_DAC_CTRL2,
        WM8766_ZCD | if (*chip).dac_mute { WM8766_DMUTE_MASK } else { 0 },
    );
    wm8766_write(chip, WM8766_LDA1, (*chip).dac_volume[2] as c_uint);
    wm8766_write(chip, WM8766_RDA1, (*chip).dac_volume[3] as c_uint);
    wm8766_write(chip, WM8766_LDA2, (*chip).dac_volume[4] as c_uint);
    wm8766_write(chip, WM8766_RDA2, (*chip).dac_volume[5] as c_uint);
    wm8766_write(chip, WM8766_LDA3, (*chip).dac_volume[6] as c_uint);
    wm8766_write(chip, WM8766_RDA3, (*chip).dac_volume[7] as c_uint | WM8766_UPDATE);
}

unsafe fn wm8776_init(chip: *mut oxygen) {
    let data = data(chip);

    (*data).wm8776_regs[WM8776_HPLVOL as usize] = ((0x79 - 60) | WM8776_HPZCEN) as u16;
    (*data).wm8776_regs[WM8776_HPRVOL as usize] = ((0x79 - 60) | WM8776_HPZCEN) as u16;
    (*data).wm8776_regs[WM8776_ADCIFCTRL as usize] =
        (WM8776_ADCFMT_LJUST | WM8776_ADCWL_24 | WM8776_ADCMCLK) as u16;
    (*data).wm8776_regs[WM8776_MSTRCTRL as usize] =
        (WM8776_ADCRATE_256 | WM8776_DACRATE_256) as u16;
    (*data).wm8776_regs[WM8776_PWRDOWN as usize] = WM8776_HPPD as u16;
    (*data).wm8776_regs[WM8776_ADCLVOL as usize] = (0xa5 | WM8776_ZCA) as u16;
    (*data).wm8776_regs[WM8776_ADCRVOL as usize] = (0xa5 | WM8776_ZCA) as u16;
    (*data).wm8776_regs[WM8776_ADCMUX as usize] = 0x001;
    wm8776_registers_init(chip);
}

unsafe fn wm8766_init(chip: *mut oxygen) {
    let data = data(chip);

    (*data).wm8766_regs[WM8766_DAC_CTRL as usize] =
        (WM8766_PL_LEFT_LEFT | WM8766_PL_RIGHT_RIGHT) as u16;
    wm8766_registers_init(chip);
}

unsafe fn xonar_ds_handle_hp_jack(chip: *mut oxygen) {
    let data = data(chip);
    let hp_plugged: bool;
    let mut reg: c_uint;

    guard_mutex(&mut (*chip).mutex);
    hp_plugged = (oxygen_read16(chip, OXYGEN_GPIO_DATA) & GPIO_DS_HP_DETECT as u16) == 0;

    oxygen_write16_masked(
        chip,
        OXYGEN_GPIO_DATA,
        if hp_plugged { 0 } else { GPIO_DS_OUTPUT_FRONTLR },
        GPIO_DS_OUTPUT_FRONTLR,
    );

    reg = ((*data).wm8766_regs[WM8766_DAC_CTRL as usize] as c_uint) & !WM8766_MUTEALL;
    if hp_plugged {
        reg |= WM8766_MUTEALL;
    }
    wm8766_write_cached(chip, WM8766_DAC_CTRL, reg);

    snd_jack_report(
        (*data).hp_jack,
        if hp_plugged { SND_JACK_HEADPHONE } else { 0 },
    );
}

unsafe fn xonar_ds_init(chip: *mut oxygen) {
    let data = data(chip);

    (*data).generic.anti_pop_delay = 300;
    (*data).generic.output_enable_bit = GPIO_DS_OUTPUT_ENABLE;

    wm8776_init(chip);
    wm8766_init(chip);

    oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_DS_INPUT_ROUTE | GPIO_DS_OUTPUT_FRONTLR);
    oxygen_clear_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_DS_HP_DETECT);
    oxygen_set_bits16(chip, OXYGEN_GPIO_DATA, GPIO_DS_INPUT_ROUTE);
    oxygen_set_bits16(chip, OXYGEN_GPIO_INTERRUPT_MASK, GPIO_DS_HP_DETECT);
    (*chip).interrupt_mask |= OXYGEN_INT_GPIO;

    xonar_enable_output(chip);

    snd_jack_new(
        (*chip).card,
        c"Headphone".as_ptr(),
        SND_JACK_HEADPHONE,
        &mut (*data).hp_jack,
        false,
        false,
    );
    xonar_ds_handle_hp_jack(chip);

    snd_component_add((*chip).card, c"WM8776".as_ptr());
    snd_component_add((*chip).card, c"WM8766".as_ptr());
}

unsafe fn xonar_hdav_slim_init(chip: *mut oxygen) {
    let data = data(chip);

    (*data).generic.anti_pop_delay = 300;
    (*data).generic.output_enable_bit = GPIO_SLIM_OUTPUT_ENABLE;

    wm8776_init(chip);

    oxygen_set_bits16(
        chip,
        OXYGEN_GPIO_CONTROL,
        GPIO_SLIM_HDMI_DISABLE | GPIO_SLIM_FIRMWARE_CLK | GPIO_SLIM_FIRMWARE_DATA,
    );

    xonar_hdmi_init(chip, &mut (*data).hdmi);
    xonar_enable_output(chip);

    snd_component_add((*chip).card, c"WM8776".as_ptr());
}

unsafe fn xonar_ds_cleanup(chip: *mut oxygen) {
    xonar_disable_output(chip);
    wm8776_write(chip, WM8776_RESET, 0);
}

unsafe fn xonar_hdav_slim_cleanup(chip: *mut oxygen) {
    xonar_hdmi_cleanup(chip);
    xonar_disable_output(chip);
    wm8776_write(chip, WM8776_RESET, 0);
    msleep(2);
}

unsafe fn xonar_ds_suspend(chip: *mut oxygen) {
    xonar_ds_cleanup(chip);
}

unsafe fn xonar_hdav_slim_suspend(chip: *mut oxygen) {
    xonar_hdav_slim_cleanup(chip);
}

unsafe fn xonar_ds_resume(chip: *mut oxygen) {
    wm8776_registers_init(chip);
    wm8766_registers_init(chip);
    xonar_enable_output(chip);
    xonar_ds_handle_hp_jack(chip);
}

unsafe fn xonar_hdav_slim_resume(chip: *mut oxygen) {
    let data = data(chip);

    wm8776_registers_init(chip);
    xonar_hdmi_resume(chip, &mut (*data).hdmi);
    xonar_enable_output(chip);
}

unsafe fn wm8776_adc_hardware_filter(channel: c_uint, hardware: *mut snd_pcm_hardware) {
    if channel == PCM_A {
        (*hardware).rates = SNDRV_PCM_RATE_32000
            | SNDRV_PCM_RATE_44100
            | SNDRV_PCM_RATE_48000
            | SNDRV_PCM_RATE_64000
            | SNDRV_PCM_RATE_88200
            | SNDRV_PCM_RATE_96000;
        (*hardware).rate_max = 96000;
    }
}

unsafe fn xonar_hdav_slim_hardware_filter(channel: c_uint, hardware: *mut snd_pcm_hardware) {
    wm8776_adc_hardware_filter(channel, hardware);
    xonar_hdmi_pcm_hardware_filter(channel, hardware);
}

unsafe fn set_wm87x6_dac_params(_chip: *mut oxygen, _params: *mut snd_pcm_hw_params) {}

unsafe fn set_wm8776_adc_params(chip: *mut oxygen, params: *mut snd_pcm_hw_params) {
    let mut reg: u16;

    reg = (WM8776_ADCRATE_256 | WM8776_DACRATE_256) as u16;
    if params_rate(params) > 48000 {
        reg |= WM8776_ADCOSR as u16;
    }
    wm8776_write_cached(chip, WM8776_MSTRCTRL, reg as c_uint);
}

unsafe fn set_hdav_slim_dac_params(chip: *mut oxygen, params: *mut snd_pcm_hw_params) {
    let data = data(chip);

    xonar_set_hdmi_params(chip, &mut (*data).hdmi, params);
}

unsafe fn update_wm8776_volume(chip: *mut oxygen) {
    let data = data(chip);
    let mut to_change: u8;

    if (*chip).dac_volume[0] == (*chip).dac_volume[1] {
        if (*chip).dac_volume[0] != (*data).wm8776_regs[WM8776_DACLVOL as usize] as c_uint
            || (*chip).dac_volume[1] != (*data).wm8776_regs[WM8776_DACRVOL as usize] as c_uint
        {
            wm8776_write(chip, WM8776_DACMASTER, (*chip).dac_volume[0] | WM8776_UPDATE);
            (*data).wm8776_regs[WM8776_DACLVOL as usize] = (*chip).dac_volume[0] as u16;
            (*data).wm8776_regs[WM8776_DACRVOL as usize] = (*chip).dac_volume[0] as u16;
        }
    } else {
        to_change = (((*chip).dac_volume[0]
            != (*data).wm8776_regs[WM8776_DACLVOL as usize] as c_uint) as u8)
            << 0;
        to_change |= (((*chip).dac_volume[1]
            != (*data).wm8776_regs[WM8776_DACLVOL as usize] as c_uint) as u8)
            << 1;
        if to_change & 1 != 0 {
            wm8776_write(
                chip,
                WM8776_DACLVOL,
                (*chip).dac_volume[0] | if to_change & 2 != 0 { 0 } else { WM8776_UPDATE },
            );
        }
        if to_change & 2 != 0 {
            wm8776_write(chip, WM8776_DACRVOL, (*chip).dac_volume[1] | WM8776_UPDATE);
        }
    }
}

unsafe fn update_wm87x6_volume(chip: *mut oxygen) {
    static WM8766_REGS: [u8; 6] = [
        WM8766_LDA1 as u8,
        WM8766_RDA1 as u8,
        WM8766_LDA2 as u8,
        WM8766_RDA2 as u8,
        WM8766_LDA3 as u8,
        WM8766_RDA3 as u8,
    ];
    let data = data(chip);
    let mut i: c_uint;
    let mut to_change: u8;

    update_wm8776_volume(chip);
    if (*chip).dac_volume[2] == (*chip).dac_volume[3]
        && (*chip).dac_volume[2] == (*chip).dac_volume[4]
        && (*chip).dac_volume[2] == (*chip).dac_volume[5]
        && (*chip).dac_volume[2] == (*chip).dac_volume[6]
        && (*chip).dac_volume[2] == (*chip).dac_volume[7]
    {
        to_change = 0;
        i = 0;
        while i < 6 {
            if (*chip).dac_volume[2] != (*data).wm8766_regs[WM8766_REGS[i as usize] as usize] as c_uint {
                to_change = 1;
            }
            i += 1;
        }
        if to_change != 0 {
            wm8766_write(chip, WM8766_MASTDA, (*chip).dac_volume[2] | WM8766_UPDATE);
            i = 0;
            while i < 6 {
                (*data).wm8766_regs[WM8766_REGS[i as usize] as usize] = (*chip).dac_volume[2] as u16;
                i += 1;
            }
        }
    } else {
        to_change = 0;
        i = 0;
        while i < 6 {
            to_change |= (((*chip).dac_volume[(2 + i) as usize]
                != (*data).wm8766_regs[WM8766_REGS[i as usize] as usize] as c_uint) as u8)
                << i;
            i += 1;
        }
        i = 0;
        while i < 6 {
            if to_change & (1 << i) != 0 {
                wm8766_write(
                    chip,
                    WM8766_REGS[i as usize] as c_uint,
                    (*chip).dac_volume[(2 + i) as usize]
                        | if to_change & (0x3e << i) != 0 { 0 } else { WM8766_UPDATE },
                );
            }
            i += 1;
        }
    }
}

unsafe fn update_wm8776_mute(chip: *mut oxygen) {
    wm8776_write_cached(
        chip,
        WM8776_DACMUTE,
        if (*chip).dac_mute { WM8776_DMUTE } else { 0 },
    );
}

unsafe fn update_wm87x6_mute(chip: *mut oxygen) {
    update_wm8776_mute(chip);
    wm8766_write_cached(
        chip,
        WM8766_DAC_CTRL2,
        WM8766_ZCD | if (*chip).dac_mute { WM8766_DMUTE_MASK } else { 0 },
    );
}

unsafe fn update_wm8766_center_lfe_mix(chip: *mut oxygen, mixed: bool) {
    let data = data(chip);
    let mut reg: c_uint;

    /*
     * The WM8766 can mix left and right channels, but this setting
     * applies to all three stereo pairs.
     */
    reg = ((*data).wm8766_regs[WM8766_DAC_CTRL as usize] as c_uint)
        & !(WM8766_PL_LEFT_MASK | WM8766_PL_RIGHT_MASK);
    if mixed {
        reg |= WM8766_PL_LEFT_LRMIX | WM8766_PL_RIGHT_LRMIX;
    } else {
        reg |= WM8766_PL_LEFT_LEFT | WM8766_PL_RIGHT_RIGHT;
    }
    wm8766_write_cached(chip, WM8766_DAC_CTRL, reg);
}

unsafe fn xonar_ds_gpio_changed(chip: *mut oxygen) {
    xonar_ds_handle_hp_jack(chip);
}

unsafe fn wm8776_bit_switch_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = data(chip);
    let bit = ((*ctl).private_value & 0xffff) as u16;
    let reg_index = ((*ctl).private_value >> 16) & 0xff;
    let invert = (((*ctl).private_value >> 24) & 1) != 0;

    (*value).value.integer.value[0] =
        ((((*data).wm8776_regs[reg_index as usize] & bit) != 0) ^ invert) as c_long;
    0
}

unsafe fn wm8776_bit_switch_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = data(chip);
    let bit = ((*ctl).private_value & 0xffff) as u16;
    let mut reg_value: u16;
    let reg_index = ((*ctl).private_value >> 16) & 0xff;
    let invert = (((*ctl).private_value >> 24) & 1) != 0;
    let changed: c_int;

    guard_mutex(&mut (*chip).mutex);
    reg_value = (*data).wm8776_regs[reg_index as usize] & !bit;
    if ((*value).value.integer.value[0] != 0) ^ invert {
        reg_value |= bit;
    }
    changed = (reg_value != (*data).wm8776_regs[reg_index as usize]) as c_int;
    if changed != 0 {
        wm8776_write(chip, reg_index as c_uint, reg_value as c_uint);
    }
    changed
}

unsafe fn wm8776_field_enum_info(ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    static HLD: [*const c_char; 16] = cstr_array_16([
        c"0 ms", c"2.67 ms", c"5.33 ms", c"10.6 ms", c"21.3 ms", c"42.7 ms",
        c"85.3 ms", c"171 ms", c"341 ms", c"683 ms", c"1.37 s", c"2.73 s",
        c"5.46 s", c"10.9 s", c"21.8 s", c"43.7 s",
    ]);
    static ATK_LIM: [*const c_char; 11] = cstr_array_11([
        c"0.25 ms", c"0.5 ms", c"1 ms", c"2 ms", c"4 ms", c"8 ms", c"16 ms",
        c"32 ms", c"64 ms", c"128 ms", c"256 ms",
    ]);
    static ATK_ALC: [*const c_char; 11] = cstr_array_11([
        c"8.40 ms", c"16.8 ms", c"33.6 ms", c"67.2 ms", c"134 ms", c"269 ms",
        c"538 ms", c"1.08 s", c"2.15 s", c"4.3 s", c"8.6 s",
    ]);
    static DCY_LIM: [*const c_char; 11] = cstr_array_11([
        c"1.2 ms", c"2.4 ms", c"4.8 ms", c"9.6 ms", c"19.2 ms", c"38.4 ms",
        c"76.8 ms", c"154 ms", c"307 ms", c"614 ms", c"1.23 s",
    ]);
    static DCY_ALC: [*const c_char; 11] = cstr_array_11([
        c"33.5 ms", c"67.0 ms", c"134 ms", c"268 ms", c"536 ms", c"1.07 s",
        c"2.14 s", c"4.29 s", c"8.58 s", c"17.2 s", c"34.3 s",
    ]);
    static TRANWIN: [*const c_char; 8] = cstr_array_8([
        c"0 us", c"62.5 us", c"125 us", c"250 us", c"500 us", c"1 ms", c"2 ms", c"4 ms",
    ]);
    let max = (((*ctl).private_value >> 12) & 0xf) as u8;
    let names: *const *const c_char;

    match ((*ctl).private_value >> 24) & 0x1f {
        WM8776_ALCCTRL2 => names = HLD.as_ptr(),
        WM8776_ALCCTRL3 => {
            if (((*ctl).private_value >> 20) & 0xf) == 0 {
                if (*ctl).private_value & LC_CONTROL_LIMITER != 0 {
                    names = ATK_LIM.as_ptr();
                } else {
                    names = ATK_ALC.as_ptr();
                }
            } else if (*ctl).private_value & LC_CONTROL_LIMITER != 0 {
                names = DCY_LIM.as_ptr();
            } else {
                names = DCY_ALC.as_ptr();
            }
        }
        WM8776_LIMITER => names = TRANWIN.as_ptr(),
        _ => return -ENXIO,
    }
    snd_ctl_enum_info(info, 1, max as c_uint + 1, names)
}

unsafe fn wm8776_field_volume_info(ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    (*info).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*info).count = 1;
    (*info).value.integer.min = (((*ctl).private_value >> 8) & 0xf) as c_long;
    (*info).value.integer.max = (((*ctl).private_value >> 12) & 0xf) as c_long;
    0
}

unsafe fn wm8776_field_set_from_ctl(ctl: *mut snd_kcontrol) {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = data(chip);
    let mut value: c_ulong;
    let reg_index: c_ulong;
    let mode: c_ulong;
    let min: u8;
    let max: u8;
    let shift: u8;
    let mask: u16;
    let mut reg_value: u16;
    let invert: bool;

    if ((*data).wm8776_regs[WM8776_ALCCTRL1 as usize] as c_uint & WM8776_LCSEL_MASK)
        == WM8776_LCSEL_LIMITER
    {
        mode = LC_CONTROL_LIMITER;
    } else {
        mode = LC_CONTROL_ALC;
    }
    if (*ctl).private_value & mode == 0 {
        return;
    }

    value = (*ctl).private_value & 0xf;
    min = (((*ctl).private_value >> 8) & 0xf) as u8;
    max = (((*ctl).private_value >> 12) & 0xf) as u8;
    mask = (((*ctl).private_value >> 16) & 0xf) as u16;
    shift = (((*ctl).private_value >> 20) & 0xf) as u8;
    reg_index = ((*ctl).private_value >> 24) & 0x1f;
    invert = (((*ctl).private_value >> 29) & 0x1) != 0;

    if invert {
        value = (max - (value as u8 - min)) as c_ulong;
    }
    reg_value = (*data).wm8776_regs[reg_index as usize];
    reg_value &= !(mask << shift);
    reg_value |= (value as u16) << shift;
    wm8776_write_cached(chip, reg_index as c_uint, reg_value as c_uint);
}

unsafe fn wm8776_field_set(ctl: *mut snd_kcontrol, value: c_uint) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let min = (((*ctl).private_value >> 8) & 0xf) as u8;
    let max = (((*ctl).private_value >> 12) & 0xf) as u8;
    let changed: c_int;

    if value < min as c_uint || value > max as c_uint {
        return -EINVAL;
    }
    guard_mutex(&mut (*chip).mutex);
    changed = (value as c_ulong != ((*ctl).private_value & 0xf)) as c_int;
    if changed != 0 {
        (*ctl).private_value = ((*ctl).private_value & !0xf) | value as c_ulong;
        wm8776_field_set_from_ctl(ctl);
    }
    changed
}

unsafe fn wm8776_field_enum_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    (*value).value.enumerated.item[0] = ((*ctl).private_value & 0xf) as c_uint;
    0
}

unsafe fn wm8776_field_volume_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    (*value).value.integer.value[0] = ((*ctl).private_value & 0xf) as c_long;
    0
}

unsafe fn wm8776_field_enum_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    wm8776_field_set(ctl, (*value).value.enumerated.item[0])
}

unsafe fn wm8776_field_volume_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    wm8776_field_set(ctl, (*value).value.integer.value[0] as c_uint)
}

unsafe fn wm8776_hp_vol_info(_ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    (*info).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*info).count = 2;
    (*info).value.integer.min = 0x79 - 60;
    (*info).value.integer.max = 0x7f;
    0
}

unsafe fn wm8776_hp_vol_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = data(chip);

    guard_mutex(&mut (*chip).mutex);
    (*value).value.integer.value[0] =
        ((*data).wm8776_regs[WM8776_HPLVOL as usize] as c_uint & WM8776_HPATT_MASK) as c_long;
    (*value).value.integer.value[1] =
        ((*data).wm8776_regs[WM8776_HPRVOL as usize] as c_uint & WM8776_HPATT_MASK) as c_long;
    0
}

unsafe fn wm8776_hp_vol_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = data(chip);
    let mut to_update: u8;

    guard_mutex(&mut (*chip).mutex);
    to_update = (((*value).value.integer.value[0] as c_uint
        != ((*data).wm8776_regs[WM8776_HPLVOL as usize] as c_uint & WM8776_HPATT_MASK)) as u8)
        << 0;
    to_update |= (((*value).value.integer.value[1] as c_uint
        != ((*data).wm8776_regs[WM8776_HPRVOL as usize] as c_uint & WM8776_HPATT_MASK)) as u8)
        << 1;
    if (*value).value.integer.value[0] == (*value).value.integer.value[1] {
        if to_update != 0 {
            wm8776_write(
                chip,
                WM8776_HPMASTER,
                (*value).value.integer.value[0] as c_uint | WM8776_HPZCEN | WM8776_UPDATE,
            );
            (*data).wm8776_regs[WM8776_HPLVOL as usize] =
                ((*value).value.integer.value[0] as c_uint | WM8776_HPZCEN) as u16;
            (*data).wm8776_regs[WM8776_HPRVOL as usize] =
                ((*value).value.integer.value[0] as c_uint | WM8776_HPZCEN) as u16;
        }
    } else {
        if to_update & 1 != 0 {
            wm8776_write(
                chip,
                WM8776_HPLVOL,
                (*value).value.integer.value[0] as c_uint
                    | WM8776_HPZCEN
                    | if to_update & 2 != 0 { 0 } else { WM8776_UPDATE },
            );
        }
        if to_update & 2 != 0 {
            wm8776_write(
                chip,
                WM8776_HPRVOL,
                (*value).value.integer.value[1] as c_uint | WM8776_HPZCEN | WM8776_UPDATE,
            );
        }
    }
    (to_update != 0) as c_int
}

unsafe fn wm8776_input_mux_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = data(chip);
    let mux_bit = (*ctl).private_value as c_uint;

    (*value).value.integer.value[0] =
        (((*data).wm8776_regs[WM8776_ADCMUX as usize] as c_uint & mux_bit) != 0) as c_long;
    0
}

unsafe fn wm8776_input_mux_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = data(chip);
    let mut other_ctl: *mut snd_kcontrol;
    let mut mux_bit = (*ctl).private_value as c_uint;
    let mut reg: u16;
    let changed: c_int;

    guard_mutex(&mut (*chip).mutex);
    reg = (*data).wm8776_regs[WM8776_ADCMUX as usize];
    if (*value).value.integer.value[0] != 0 {
        reg |= mux_bit as u16;
        /* line-in and mic-in are exclusive */
        mux_bit ^= 3;
        if reg as c_uint & mux_bit != 0 {
            reg &= !(mux_bit as u16);
            if mux_bit == 1 {
                other_ctl = (*data).line_adcmux_control;
            } else {
                other_ctl = (*data).mic_adcmux_control;
            }
            snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*other_ctl).id);
        }
    } else {
        reg &= !(mux_bit as u16);
    }
    changed = (reg != (*data).wm8776_regs[WM8776_ADCMUX as usize]) as c_int;
    if changed != 0 {
        oxygen_write16_masked(
            chip,
            OXYGEN_GPIO_DATA,
            if reg & 1 != 0 { GPIO_DS_INPUT_ROUTE } else { 0 },
            GPIO_DS_INPUT_ROUTE,
        );
        wm8776_write(chip, WM8776_ADCMUX, reg as c_uint);
    }
    changed
}

unsafe fn wm8776_input_vol_info(_ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    (*info).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*info).count = 2;
    (*info).value.integer.min = 0xa5;
    (*info).value.integer.max = 0xff;
    0
}

unsafe fn wm8776_input_vol_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = data(chip);

    guard_mutex(&mut (*chip).mutex);
    (*value).value.integer.value[0] =
        ((*data).wm8776_regs[WM8776_ADCLVOL as usize] as c_uint & WM8776_AGMASK) as c_long;
    (*value).value.integer.value[1] =
        ((*data).wm8776_regs[WM8776_ADCRVOL as usize] as c_uint & WM8776_AGMASK) as c_long;
    0
}

unsafe fn wm8776_input_vol_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = data(chip);
    let changed: c_int;

    guard_mutex(&mut (*chip).mutex);
    changed = (((*value).value.integer.value[0] as c_uint
        != ((*data).wm8776_regs[WM8776_ADCLVOL as usize] as c_uint & WM8776_AGMASK))
        || ((*value).value.integer.value[1] as c_uint
            != ((*data).wm8776_regs[WM8776_ADCRVOL as usize] as c_uint & WM8776_AGMASK)))
        as c_int;
    wm8776_write_cached(
        chip,
        WM8776_ADCLVOL,
        (*value).value.integer.value[0] as c_uint | WM8776_ZCA,
    );
    wm8776_write_cached(
        chip,
        WM8776_ADCRVOL,
        (*value).value.integer.value[1] as c_uint | WM8776_ZCA,
    );
    changed
}

unsafe fn wm8776_level_control_info(_ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    static NAMES: [*const c_char; 3] = cstr_array_3([c"None", c"Peak Limiter", c"Automatic Level Control"]);

    snd_ctl_enum_info(info, 1, 3, NAMES.as_ptr())
}

unsafe fn wm8776_level_control_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = data(chip);

    if (*data).wm8776_regs[WM8776_ALCCTRL2 as usize] as c_uint & WM8776_LCEN == 0 {
        (*value).value.enumerated.item[0] = 0;
    } else if ((*data).wm8776_regs[WM8776_ALCCTRL1 as usize] as c_uint & WM8776_LCSEL_MASK)
        == WM8776_LCSEL_LIMITER
    {
        (*value).value.enumerated.item[0] = 1;
    } else {
        (*value).value.enumerated.item[0] = 2;
    }
    0
}

unsafe fn activate_control(chip: *mut oxygen, ctl: *mut snd_kcontrol, mode: c_ulong) {
    let access: c_uint;

    if (*ctl).private_value & mode != 0 {
        access = 0;
    } else {
        access = SNDRV_CTL_ELEM_ACCESS_INACTIVE;
    }
    if ((*ctl).vd[0].access & SNDRV_CTL_ELEM_ACCESS_INACTIVE) != access {
        (*ctl).vd[0].access ^= SNDRV_CTL_ELEM_ACCESS_INACTIVE;
        snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_INFO, &mut (*ctl).id);
    }
}

unsafe fn wm8776_level_control_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = data(chip);
    let mut mode: c_ulong = 0;
    let mut i: c_uint;
    let ctrl1: u16;
    let ctrl2: u16;
    let changed: c_int;

    if (*value).value.enumerated.item[0] >= 3 {
        return -EINVAL;
    }
    guard_mutex(&mut (*chip).mutex);
    changed = ((*value).value.enumerated.item[0] as c_ulong != (*ctl).private_value) as c_int;
    if changed != 0 {
        (*ctl).private_value = (*value).value.enumerated.item[0] as c_ulong;
        ctrl1 = (*data).wm8776_regs[WM8776_ALCCTRL1 as usize];
        ctrl2 = (*data).wm8776_regs[WM8776_ALCCTRL2 as usize];
        match (*value).value.enumerated.item[0] {
            1 => {
                wm8776_write_cached(
                    chip,
                    WM8776_ALCCTRL1,
                    (ctrl1 as c_uint & !WM8776_LCSEL_MASK) | WM8776_LCSEL_LIMITER,
                );
                wm8776_write_cached(chip, WM8776_ALCCTRL2, ctrl2 as c_uint | WM8776_LCEN);
                mode = LC_CONTROL_LIMITER;
            }
            2 => {
                wm8776_write_cached(
                    chip,
                    WM8776_ALCCTRL1,
                    (ctrl1 as c_uint & !WM8776_LCSEL_MASK) | WM8776_LCSEL_ALC_STEREO,
                );
                wm8776_write_cached(chip, WM8776_ALCCTRL2, ctrl2 as c_uint | WM8776_LCEN);
                mode = LC_CONTROL_ALC;
            }
            _ => {
                wm8776_write_cached(chip, WM8776_ALCCTRL2, ctrl2 as c_uint & !WM8776_LCEN);
            }
        }
        i = 0;
        while (i as usize) < (*data).lc_controls.len() {
            activate_control(chip, (*data).lc_controls[i as usize], mode);
            i += 1;
        }
    }
    changed
}

unsafe fn hpf_info(_ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    static NAMES: [*const c_char; 2] = cstr_array_2([c"None", c"High-pass Filter"]);

    snd_ctl_enum_info(info, 1, 2, NAMES.as_ptr())
}

unsafe fn hpf_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = data(chip);

    (*value).value.enumerated.item[0] =
        (((*data).wm8776_regs[WM8776_ADCIFCTRL as usize] as c_uint & WM8776_ADCHPD) == 0) as c_uint;
    0
}

unsafe fn hpf_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = data(chip);
    let mut reg: c_uint;
    let changed: c_int;

    guard_mutex(&mut (*chip).mutex);
    reg = (*data).wm8776_regs[WM8776_ADCIFCTRL as usize] as c_uint & !WM8776_ADCHPD;
    if (*value).value.enumerated.item[0] == 0 {
        reg |= WM8776_ADCHPD;
    }
    changed = (reg as u16 != (*data).wm8776_regs[WM8776_ADCIFCTRL as usize]) as c_int;
    if changed != 0 {
        wm8776_write(chip, WM8776_ADCIFCTRL, reg);
    }
    changed
}

const fn wm8776_bit_switch(
    xname: *const c_char,
    reg: c_ulong,
    bit: c_ulong,
    invert: c_ulong,
    flags: c_ulong,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(wm8776_bit_switch_get),
        put: Some(wm8776_bit_switch_put),
        private_value: (reg << 16) | bit | (invert << 24) | flags,
        ..snd_kcontrol_new::zeroed()
    }
}

const fn wm8776_field_private(
    reg: c_ulong,
    shift: c_ulong,
    initval: c_ulong,
    min: c_ulong,
    max: c_ulong,
    mask: c_ulong,
    flags: c_ulong,
) -> c_ulong {
    initval | (min << 8) | (max << 12) | (mask << 16) | (shift << 20) | (reg << 24) | flags
}

const fn wm8776_field_ctl_enum(
    xname: *const c_char,
    reg: c_ulong,
    shift: c_ulong,
    init: c_ulong,
    min: c_ulong,
    max: c_ulong,
    mask: c_ulong,
    flags: c_ulong,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        private_value: wm8776_field_private(reg, shift, init, min, max, mask, flags),
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_INACTIVE,
        info: Some(wm8776_field_enum_info),
        get: Some(wm8776_field_enum_get),
        put: Some(wm8776_field_enum_put),
        ..snd_kcontrol_new::zeroed()
    }
}

const fn wm8776_field_ctl_volume(
    xname: *const c_char,
    reg: c_ulong,
    shift: c_ulong,
    init: c_ulong,
    min: c_ulong,
    max: c_ulong,
    mask: c_ulong,
    flags: c_ulong,
    tlv_p: *const c_uint,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        private_value: wm8776_field_private(reg, shift, init, min, max, mask, flags),
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE
            | SNDRV_CTL_ELEM_ACCESS_INACTIVE
            | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        info: Some(wm8776_field_volume_info),
        get: Some(wm8776_field_volume_get),
        put: Some(wm8776_field_volume_put),
        tlv: snd_kcontrol_new_tlv { p: tlv_p },
        ..snd_kcontrol_new::zeroed()
    }
}

static WM87X6_DAC_DB_SCALE: [c_uint; 4] = declare_tlv_db_scale(-6000, 50, 0);
static WM8776_ADC_DB_SCALE: [c_uint; 4] = declare_tlv_db_scale(-2100, 50, 0);
static WM8776_HP_DB_SCALE: [c_uint; 4] = declare_tlv_db_scale(-6000, 100, 0);
static WM8776_LCT_DB_SCALE: [c_uint; 4] = declare_tlv_db_scale(-1600, 100, 0);
static WM8776_MAXGAIN_DB_SCALE: [c_uint; 4] = declare_tlv_db_scale(0, 400, 0);
static WM8776_NGTH_DB_SCALE: [c_uint; 4] = declare_tlv_db_scale(-7800, 600, 0);
static WM8776_MAXATTEN_LIM_DB_SCALE: [c_uint; 4] = declare_tlv_db_scale(-1200, 100, 0);
static WM8776_MAXATTEN_ALC_DB_SCALE: [c_uint; 4] = declare_tlv_db_scale(-2100, 400, 0);

static DS_CONTROLS: [snd_kcontrol_new; 9] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Headphone Playback Volume".as_ptr(),
        info: Some(wm8776_hp_vol_info),
        get: Some(wm8776_hp_vol_get),
        put: Some(wm8776_hp_vol_put),
        tlv: snd_kcontrol_new_tlv { p: WM8776_HP_DB_SCALE.as_ptr() },
        ..snd_kcontrol_new::zeroed()
    },
    wm8776_bit_switch(c"Headphone Playback Switch".as_ptr(), WM8776_PWRDOWN as c_ulong, WM8776_HPPD as c_ulong, 1, 0),
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Input Capture Volume".as_ptr(),
        info: Some(wm8776_input_vol_info),
        get: Some(wm8776_input_vol_get),
        put: Some(wm8776_input_vol_put),
        tlv: snd_kcontrol_new_tlv { p: WM8776_ADC_DB_SCALE.as_ptr() },
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Line Capture Switch".as_ptr(),
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(wm8776_input_mux_get),
        put: Some(wm8776_input_mux_put),
        private_value: 1 << 0,
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Mic Capture Switch".as_ptr(),
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(wm8776_input_mux_get),
        put: Some(wm8776_input_mux_put),
        private_value: 1 << 1,
        ..snd_kcontrol_new::zeroed()
    },
    wm8776_bit_switch(c"Front Mic Capture Switch".as_ptr(), WM8776_ADCMUX as c_ulong, 1 << 2, 0, 0),
    wm8776_bit_switch(c"Aux Capture Switch".as_ptr(), WM8776_ADCMUX as c_ulong, 1 << 3, 0, 0),
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"ADC Filter Capture Enum".as_ptr(),
        info: Some(hpf_info),
        get: Some(hpf_get),
        put: Some(hpf_put),
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Level Control Capture Enum".as_ptr(),
        info: Some(wm8776_level_control_info),
        get: Some(wm8776_level_control_get),
        put: Some(wm8776_level_control_put),
        private_value: 0,
        ..snd_kcontrol_new::zeroed()
    },
];

static HDAV_SLIM_CONTROLS: [snd_kcontrol_new; 8] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"HDMI Playback Switch".as_ptr(),
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(xonar_gpio_bit_switch_get),
        put: Some(xonar_gpio_bit_switch_put),
        private_value: GPIO_SLIM_HDMI_DISABLE as c_ulong | XONAR_GPIO_BIT_INVERT,
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Headphone Playback Volume".as_ptr(),
        info: Some(wm8776_hp_vol_info),
        get: Some(wm8776_hp_vol_get),
        put: Some(wm8776_hp_vol_put),
        tlv: snd_kcontrol_new_tlv { p: WM8776_HP_DB_SCALE.as_ptr() },
        ..snd_kcontrol_new::zeroed()
    },
    wm8776_bit_switch(c"Headphone Playback Switch".as_ptr(), WM8776_PWRDOWN as c_ulong, WM8776_HPPD as c_ulong, 1, 0),
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Input Capture Volume".as_ptr(),
        info: Some(wm8776_input_vol_info),
        get: Some(wm8776_input_vol_get),
        put: Some(wm8776_input_vol_put),
        tlv: snd_kcontrol_new_tlv { p: WM8776_ADC_DB_SCALE.as_ptr() },
        ..snd_kcontrol_new::zeroed()
    },
    wm8776_bit_switch(c"Mic Capture Switch".as_ptr(), WM8776_ADCMUX as c_ulong, 1 << 0, 0, 0),
    wm8776_bit_switch(c"Aux Capture Switch".as_ptr(), WM8776_ADCMUX as c_ulong, 1 << 1, 0, 0),
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"ADC Filter Capture Enum".as_ptr(),
        info: Some(hpf_info),
        get: Some(hpf_get),
        put: Some(hpf_put),
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Level Control Capture Enum".as_ptr(),
        info: Some(wm8776_level_control_info),
        get: Some(wm8776_level_control_get),
        put: Some(wm8776_level_control_put),
        private_value: 0,
        ..snd_kcontrol_new::zeroed()
    },
];

static LC_CONTROLS: [snd_kcontrol_new; 13] = [
    wm8776_field_ctl_volume(c"Limiter Threshold Capture Volume".as_ptr(), WM8776_ALCCTRL1 as c_ulong, 0, 11, 0, 15, 0xf, LC_CONTROL_LIMITER, WM8776_LCT_DB_SCALE.as_ptr()),
    wm8776_field_ctl_enum(c"Limiter Attack Time Capture Enum".as_ptr(), WM8776_ALCCTRL3 as c_ulong, 0, 2, 0, 10, 0xf, LC_CONTROL_LIMITER),
    wm8776_field_ctl_enum(c"Limiter Decay Time Capture Enum".as_ptr(), WM8776_ALCCTRL3 as c_ulong, 4, 3, 0, 10, 0xf, LC_CONTROL_LIMITER),
    wm8776_field_ctl_enum(c"Limiter Transient Window Capture Enum".as_ptr(), WM8776_LIMITER as c_ulong, 4, 2, 0, 7, 0x7, LC_CONTROL_LIMITER),
    wm8776_field_ctl_volume(c"Limiter Maximum Attenuation Capture Volume".as_ptr(), WM8776_LIMITER as c_ulong, 0, 6, 3, 12, 0xf, LC_CONTROL_LIMITER, WM8776_MAXATTEN_LIM_DB_SCALE.as_ptr()),
    wm8776_field_ctl_volume(c"ALC Target Level Capture Volume".as_ptr(), WM8776_ALCCTRL1 as c_ulong, 0, 11, 0, 15, 0xf, LC_CONTROL_ALC, WM8776_LCT_DB_SCALE.as_ptr()),
    wm8776_field_ctl_enum(c"ALC Attack Time Capture Enum".as_ptr(), WM8776_ALCCTRL3 as c_ulong, 0, 2, 0, 10, 0xf, LC_CONTROL_ALC),
    wm8776_field_ctl_enum(c"ALC Decay Time Capture Enum".as_ptr(), WM8776_ALCCTRL3 as c_ulong, 4, 3, 0, 10, 0xf, LC_CONTROL_ALC),
    wm8776_field_ctl_volume(c"ALC Maximum Gain Capture Volume".as_ptr(), WM8776_ALCCTRL1 as c_ulong, 4, 7, 1, 7, 0x7, LC_CONTROL_ALC, WM8776_MAXGAIN_DB_SCALE.as_ptr()),
    wm8776_field_ctl_volume(c"ALC Maximum Attenuation Capture Volume".as_ptr(), WM8776_LIMITER as c_ulong, 0, 10, 10, 15, 0xf, LC_CONTROL_ALC, WM8776_MAXATTEN_ALC_DB_SCALE.as_ptr()),
    wm8776_field_ctl_enum(c"ALC Hold Time Capture Enum".as_ptr(), WM8776_ALCCTRL2 as c_ulong, 0, 0, 0, 15, 0xf, LC_CONTROL_ALC),
    wm8776_bit_switch(c"Noise Gate Capture Switch".as_ptr(), WM8776_NOISEGATE as c_ulong, WM8776_NGAT as c_ulong, 0, LC_CONTROL_ALC),
    wm8776_field_ctl_volume(c"Noise Gate Threshold Capture Volume".as_ptr(), WM8776_NOISEGATE as c_ulong, 2, 0, 0, 7, 0x7, LC_CONTROL_ALC, WM8776_NGTH_DB_SCALE.as_ptr()),
];

unsafe fn add_lc_controls(chip: *mut oxygen) -> c_int {
    let data = data(chip);
    let mut i: c_uint;
    let mut ctl: *mut snd_kcontrol;
    let mut err: c_int;

    /* BUILD_BUG_ON(ARRAY_SIZE(lc_controls) != ARRAY_SIZE(data->lc_controls)); */
    i = 0;
    while (i as usize) < LC_CONTROLS.len() {
        ctl = snd_ctl_new1(&LC_CONTROLS[i as usize], chip as *mut c_void);
        if ctl.is_null() {
            return -ENOMEM;
        }
        err = snd_ctl_add((*chip).card, ctl);
        if err < 0 {
            return err;
        }
        (*data).lc_controls[i as usize] = ctl;
        i += 1;
    }
    0
}

unsafe fn xonar_ds_mixer_init(chip: *mut oxygen) -> c_int {
    let data = data(chip);
    let mut i: c_uint;
    let mut ctl: *mut snd_kcontrol;
    let mut err: c_int;

    i = 0;
    while (i as usize) < DS_CONTROLS.len() {
        ctl = snd_ctl_new1(&DS_CONTROLS[i as usize], chip as *mut c_void);
        if ctl.is_null() {
            return -ENOMEM;
        }
        err = snd_ctl_add((*chip).card, ctl);
        if err < 0 {
            return err;
        }
        if strcmp((*ctl).id.name.as_ptr(), c"Line Capture Switch".as_ptr()) == 0 {
            (*data).line_adcmux_control = ctl;
        } else if strcmp((*ctl).id.name.as_ptr(), c"Mic Capture Switch".as_ptr()) == 0 {
            (*data).mic_adcmux_control = ctl;
        }
        i += 1;
    }
    if (*data).line_adcmux_control.is_null() || (*data).mic_adcmux_control.is_null() {
        return -ENXIO;
    }

    add_lc_controls(chip)
}

unsafe fn xonar_hdav_slim_mixer_init(chip: *mut oxygen) -> c_int {
    let mut i: c_uint;
    let mut ctl: *mut snd_kcontrol;
    let mut err: c_int;

    i = 0;
    while (i as usize) < HDAV_SLIM_CONTROLS.len() {
        ctl = snd_ctl_new1(&HDAV_SLIM_CONTROLS[i as usize], chip as *mut c_void);
        if ctl.is_null() {
            return -ENOMEM;
        }
        err = snd_ctl_add((*chip).card, ctl);
        if err < 0 {
            return err;
        }
        i += 1;
    }

    add_lc_controls(chip)
}

unsafe fn dump_wm8776_registers(chip: *mut oxygen, buffer: *mut snd_info_buffer) {
    let data = data(chip);
    let mut i: c_uint;

    snd_iprintf(buffer, c"\nWM8776:\n00:".as_ptr());
    i = 0;
    while i < 0x10 {
        snd_iprintf(buffer, c" %03x".as_ptr(), (*data).wm8776_regs[i as usize] as c_uint);
        i += 1;
    }
    snd_iprintf(buffer, c"\n10:".as_ptr());
    i = 0x10;
    while i < 0x17 {
        snd_iprintf(buffer, c" %03x".as_ptr(), (*data).wm8776_regs[i as usize] as c_uint);
        i += 1;
    }
    snd_iprintf(buffer, c"\n".as_ptr());
}

unsafe fn dump_wm87x6_registers(chip: *mut oxygen, buffer: *mut snd_info_buffer) {
    let data = data(chip);
    let mut i: c_uint;

    dump_wm8776_registers(chip, buffer);
    snd_iprintf(buffer, c"\nWM8766:\n00:".as_ptr());
    i = 0;
    while i < 0x10 {
        snd_iprintf(buffer, c" %03x".as_ptr(), (*data).wm8766_regs[i as usize] as c_uint);
        i += 1;
    }
    snd_iprintf(buffer, c"\n".as_ptr());
}

static MODEL_XONAR_DS: oxygen_model = oxygen_model {
    longname: c"Asus Virtuoso 66".as_ptr(),
    chip: c"AV200".as_ptr(),
    init: Some(xonar_ds_init),
    mixer_init: Some(xonar_ds_mixer_init),
    cleanup: Some(xonar_ds_cleanup),
    suspend: Some(xonar_ds_suspend),
    resume: Some(xonar_ds_resume),
    pcm_hardware_filter: Some(wm8776_adc_hardware_filter),
    set_dac_params: Some(set_wm87x6_dac_params),
    set_adc_params: Some(set_wm8776_adc_params),
    update_dac_volume: Some(update_wm87x6_volume),
    update_dac_mute: Some(update_wm87x6_mute),
    update_center_lfe_mix: Some(update_wm8766_center_lfe_mix),
    gpio_changed: Some(xonar_ds_gpio_changed),
    dump_registers: Some(dump_wm87x6_registers),
    dac_tlv: WM87X6_DAC_DB_SCALE.as_ptr(),
    model_data_size: size_of::<xonar_wm87x6>(),
    device_config: PLAYBACK_0_TO_I2S | PLAYBACK_1_TO_SPDIF | CAPTURE_0_FROM_I2S_1 | CAPTURE_1_FROM_SPDIF,
    dac_channels_pcm: 8,
    dac_channels_mixer: 8,
    dac_volume_min: 255 - 2 * 60,
    dac_volume_max: 255,
    function_flags: OXYGEN_FUNCTION_SPI,
    dac_mclks: OXYGEN_MCLKS(256, 256, 128),
    adc_mclks: OXYGEN_MCLKS(256, 256, 128),
    dac_i2s_format: OXYGEN_I2S_FORMAT_LJUST,
    adc_i2s_format: OXYGEN_I2S_FORMAT_LJUST,
    ..oxygen_model::zeroed()
};

static MODEL_XONAR_HDAV_SLIM: oxygen_model = oxygen_model {
    shortname: c"Xonar HDAV1.3 Slim".as_ptr(),
    longname: c"Asus Virtuoso 200".as_ptr(),
    chip: c"AV200".as_ptr(),
    init: Some(xonar_hdav_slim_init),
    mixer_init: Some(xonar_hdav_slim_mixer_init),
    cleanup: Some(xonar_hdav_slim_cleanup),
    suspend: Some(xonar_hdav_slim_suspend),
    resume: Some(xonar_hdav_slim_resume),
    pcm_hardware_filter: Some(xonar_hdav_slim_hardware_filter),
    set_dac_params: Some(set_hdav_slim_dac_params),
    set_adc_params: Some(set_wm8776_adc_params),
    update_dac_volume: Some(update_wm8776_volume),
    update_dac_mute: Some(update_wm8776_mute),
    uart_input: Some(xonar_hdmi_uart_input),
    dump_registers: Some(dump_wm8776_registers),
    dac_tlv: WM87X6_DAC_DB_SCALE.as_ptr(),
    model_data_size: size_of::<xonar_wm87x6>(),
    device_config: PLAYBACK_0_TO_I2S | PLAYBACK_1_TO_SPDIF | CAPTURE_0_FROM_I2S_1 | CAPTURE_1_FROM_SPDIF,
    dac_channels_pcm: 8,
    dac_channels_mixer: 2,
    dac_volume_min: 255 - 2 * 60,
    dac_volume_max: 255,
    function_flags: OXYGEN_FUNCTION_2WIRE,
    dac_mclks: OXYGEN_MCLKS(256, 256, 128),
    adc_mclks: OXYGEN_MCLKS(256, 256, 128),
    dac_i2s_format: OXYGEN_I2S_FORMAT_LJUST,
    adc_i2s_format: OXYGEN_I2S_FORMAT_LJUST,
    ..oxygen_model::zeroed()
};

#[no_mangle]
pub unsafe extern "C" fn get_xonar_wm87x6_model(
    chip: *mut oxygen,
    id: *const pci_device_id,
) -> c_int {
    match (*id).subdevice {
        0x838e => {
            (*chip).model = MODEL_XONAR_DS;
            (*chip).model.shortname = c"Xonar DS".as_ptr();
        }
        0x8522 => {
            (*chip).model = MODEL_XONAR_DS;
            (*chip).model.shortname = c"Xonar DSX".as_ptr();
        }
        0x835e => {
            (*chip).model = MODEL_XONAR_HDAV_SLIM;
        }
        _ => return -EINVAL,
    }
    0
}

/* External declarations and C dependency surface from included kernel/ALSA headers. */
type c_long = isize;
type snd_ctl_info_fn = unsafe fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int;
type snd_ctl_get_fn = unsafe fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int;
type snd_ctl_put_fn = unsafe fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int;

extern "C" {
    fn oxygen_write_spi(chip: *mut oxygen, control: c_uint, data: c_uint);
    fn oxygen_write_i2c(chip: *mut oxygen, device: c_uint, map: c_uint, data: c_uint);
    fn oxygen_read16(chip: *mut oxygen, reg: c_uint) -> u16;
    fn oxygen_write16_masked(chip: *mut oxygen, reg: c_uint, value: c_uint, mask: c_uint);
    fn oxygen_set_bits16(chip: *mut oxygen, reg: c_uint, value: c_uint);
    fn oxygen_clear_bits16(chip: *mut oxygen, reg: c_uint, value: c_uint);
    fn snd_jack_report(jack: *mut snd_jack, status: c_int);
    fn snd_jack_new(card: *mut snd_card, id: *const c_char, type_: c_int, jack: *mut *mut snd_jack, initial_kctl: bool, phantom_jack: bool) -> c_int;
    fn snd_component_add(card: *mut snd_card, component: *const c_char) -> c_int;
    fn xonar_enable_output(chip: *mut oxygen);
    fn xonar_disable_output(chip: *mut oxygen);
    fn xonar_hdmi_init(chip: *mut oxygen, hdmi: *mut xonar_hdmi);
    fn xonar_hdmi_cleanup(chip: *mut oxygen);
    fn xonar_hdmi_resume(chip: *mut oxygen, hdmi: *mut xonar_hdmi);
    fn xonar_hdmi_pcm_hardware_filter(channel: c_uint, hardware: *mut snd_pcm_hardware);
    fn xonar_set_hdmi_params(chip: *mut oxygen, hdmi: *mut xonar_hdmi, params: *mut snd_pcm_hw_params);
    fn xonar_hdmi_uart_input(chip: *mut oxygen);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_ctl_enum_info(info: *mut snd_ctl_elem_info, channels: c_uint, items: c_uint, names: *const *const c_char) -> c_int;
    fn snd_ctl_boolean_mono_info(ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn msleep(msecs: c_uint);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn guard_mutex(mutex: *mut mutex);
}

/* Constants, structs, helpers, and macros below are provided by translated dependencies. */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
