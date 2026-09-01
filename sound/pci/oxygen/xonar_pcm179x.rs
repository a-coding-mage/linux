// SPDX-License-Identifier: GPL-2.0-only
/*
 * card driver for models with PCM1796 DACs (Xonar D2/D2X/HDAV1.3/ST/STX)
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

/*
 * This file is a source-level Rust translation of xonar_pcm179x.c.
 * C include dependencies are intentionally left as external Rust items.
 */

const GPIO_D2X_EXT_POWER: u32 = 0x0020;
const GPIO_D2_ALT: u32 = 0x0080;
const GPIO_D2_OUTPUT_ENABLE: u32 = 0x0100;

const GPI_EXT_POWER: u32 = 0x01;
const GPIO_INPUT_ROUTE: u32 = 0x0100;

const GPIO_HDAV_OUTPUT_ENABLE: u32 = 0x0001;
const GPIO_HDAV_MAGIC: u32 = 0x00c0;

const GPIO_DB_MASK: u32 = 0x0030;
const GPIO_DB_H6: u32 = 0x0000;

const GPIO_ST_OUTPUT_ENABLE: u32 = 0x0001;
const GPIO_ST_HP_REAR: u32 = 0x0002;
const GPIO_ST_MAGIC: u32 = 0x0040;
const GPIO_ST_HP: u32 = 0x0080;

const GPIO_XENSE_OUTPUT_ENABLE: u32 = 0x0001 | 0x0010 | 0x0020;
const GPIO_XENSE_SPEAKERS: u32 = 0x0080;

#[inline]
const fn I2C_DEVICE_PCM1796(i: u32) -> u32 {
    0x98 + (i << 1) /* 10011, ii, /W=0 */
}
const I2C_DEVICE_CS2000: u32 = 0x9c; /* 100111, 0, /W=0 */

const PCM1796_REG_BASE: u8 = 16;

#[repr(C)]
pub struct xonar_pcm179x {
    pub generic: xonar_generic,
    pub dacs: u32,
    pub pcm1796_regs: [[u8; 5]; 4],
    pub current_rate: u32,
    pub h6: bool,
    pub hp_active: bool,
    pub hp_gain_offset: i8,
    pub has_cs2000: bool,
    pub cs2000_regs: [u8; 0x1f],
    pub broken_i2c: bool,
}

#[repr(C)]
pub struct xonar_hdav {
    pub pcm179x: xonar_pcm179x,
    pub hdmi: xonar_hdmi,
}

#[inline]
unsafe fn pcm1796_write_spi(chip: *mut oxygen, codec: u32, reg: u8, value: u8) {
    /* maps ALSA channel pair number to SPI output */
    static CODEC_MAP: [u8; 4] = [0, 1, 2, 4];
    unsafe {
        oxygen_write_spi(
            chip,
            OXYGEN_SPI_TRIGGER
                | OXYGEN_SPI_DATA_LENGTH_2
                | OXYGEN_SPI_CLOCK_160
                | ((CODEC_MAP[codec as usize] as u32) << OXYGEN_SPI_CODEC_SHIFT)
                | OXYGEN_SPI_CEN_LATCH_CLOCK_HI,
            ((reg as u32) << 8) | value as u32,
        );
    }
}

#[inline]
unsafe fn pcm1796_write_i2c(chip: *mut oxygen, codec: u32, reg: u8, value: u8) {
    unsafe {
        oxygen_write_i2c(chip, I2C_DEVICE_PCM1796(codec), reg, value);
    }
}

unsafe fn pcm1796_write(chip: *mut oxygen, codec: u32, reg: u8, value: u8) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;

        if ((*chip).model.function_flags & OXYGEN_FUNCTION_2WIRE_SPI_MASK) == OXYGEN_FUNCTION_SPI {
            pcm1796_write_spi(chip, codec, reg, value);
        } else {
            pcm1796_write_i2c(chip, codec, reg, value);
        }
        if (reg.wrapping_sub(PCM1796_REG_BASE) as u32) < (*data).pcm1796_regs[codec as usize].len() as u32 {
            (*data).pcm1796_regs[codec as usize][reg.wrapping_sub(PCM1796_REG_BASE) as usize] = value;
        }
    }
}

unsafe fn pcm1796_write_cached(chip: *mut oxygen, codec: u32, reg: u8, value: u8) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;

        if value != (*data).pcm1796_regs[codec as usize][reg.wrapping_sub(PCM1796_REG_BASE) as usize] {
            pcm1796_write(chip, codec, reg, value);
        }
    }
}

unsafe fn cs2000_write(chip: *mut oxygen, reg: u8, value: u8) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;

        oxygen_write_i2c(chip, I2C_DEVICE_CS2000, reg, value);
        (*data).cs2000_regs[reg as usize] = value;
    }
}

unsafe fn cs2000_write_cached(chip: *mut oxygen, reg: u8, value: u8) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;

        if value != (*data).cs2000_regs[reg as usize] {
            cs2000_write(chip, reg, value);
        }
    }
}

unsafe fn pcm1796_registers_init(chip: *mut oxygen) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;
        let mut gain_offset: i8;

        msleep(1);
        gain_offset = if (*data).hp_active { (*data).hp_gain_offset } else { 0 };
        for i in 0..(*data).dacs {
            /* set ATLD before ATL/ATR */
            pcm1796_write(chip, i, 18, (*data).pcm1796_regs[0][(18 - PCM1796_REG_BASE) as usize]);
            pcm1796_write(chip, i, 16, (*chip).dac_volume[(i * 2) as usize].wrapping_add(gain_offset as u8));
            pcm1796_write(chip, i, 17, (*chip).dac_volume[(i * 2 + 1) as usize].wrapping_add(gain_offset as u8));
            pcm1796_write(chip, i, 19, (*data).pcm1796_regs[0][(19 - PCM1796_REG_BASE) as usize]);
            pcm1796_write(chip, i, 20, (*data).pcm1796_regs[0][(20 - PCM1796_REG_BASE) as usize]);
            pcm1796_write(chip, i, 21, 0);
            gain_offset = 0;
        }
    }
}

unsafe fn pcm1796_init(chip: *mut oxygen) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;

        (*data).pcm1796_regs[0][(18 - PCM1796_REG_BASE) as usize] = PCM1796_FMT_24_I2S | PCM1796_ATLD;
        if !(*data).broken_i2c {
            (*data).pcm1796_regs[0][(18 - PCM1796_REG_BASE) as usize] |= PCM1796_MUTE;
        }
        (*data).pcm1796_regs[0][(19 - PCM1796_REG_BASE) as usize] = PCM1796_FLT_SHARP | PCM1796_ATS_1;
        (*data).pcm1796_regs[0][(20 - PCM1796_REG_BASE) as usize] =
            if (*data).h6 { PCM1796_OS_64 } else { PCM1796_OS_128 };
        pcm1796_registers_init(chip);
        (*data).current_rate = 48000;
    }
}

unsafe fn xonar_d2_init(chip: *mut oxygen) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;

        (*data).generic.anti_pop_delay = 300;
        (*data).generic.output_enable_bit = GPIO_D2_OUTPUT_ENABLE;
        (*data).dacs = 4;

        pcm1796_init(chip);

        oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_D2_ALT);
        oxygen_clear_bits16(chip, OXYGEN_GPIO_DATA, GPIO_D2_ALT);

        oxygen_ac97_set_bits(chip, 0, CM9780_JACK, CM9780_FMIC2MIC);

        xonar_init_cs53x1(chip);
        xonar_enable_output(chip);

        snd_component_add((*chip).card, c"PCM1796".as_ptr());
        snd_component_add((*chip).card, c"CS5381".as_ptr());
    }
}

unsafe fn xonar_d2x_init(chip: *mut oxygen) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;

        (*data).generic.ext_power_reg = OXYGEN_GPIO_DATA;
        (*data).generic.ext_power_int_reg = OXYGEN_GPIO_INTERRUPT_MASK;
        (*data).generic.ext_power_bit = GPIO_D2X_EXT_POWER;
        oxygen_clear_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_D2X_EXT_POWER);
        xonar_init_ext_power(chip);
        xonar_d2_init(chip);
    }
}

unsafe fn xonar_hdav_init(chip: *mut oxygen) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_hdav;

        oxygen_write16(
            chip,
            OXYGEN_2WIRE_BUS_STATUS,
            OXYGEN_2WIRE_LENGTH_8 | OXYGEN_2WIRE_INTERRUPT_MASK | OXYGEN_2WIRE_SPEED_STANDARD,
        );

        (*data).pcm179x.generic.anti_pop_delay = 100;
        (*data).pcm179x.generic.output_enable_bit = GPIO_HDAV_OUTPUT_ENABLE;
        (*data).pcm179x.generic.ext_power_reg = OXYGEN_GPI_DATA;
        (*data).pcm179x.generic.ext_power_int_reg = OXYGEN_GPI_INTERRUPT_MASK;
        (*data).pcm179x.generic.ext_power_bit = GPI_EXT_POWER;
        (*data).pcm179x.dacs = (*chip).model.dac_channels_mixer / 2;
        (*data).pcm179x.h6 = (*chip).model.dac_channels_mixer > 2;

        pcm1796_init(chip);

        oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_HDAV_MAGIC | GPIO_INPUT_ROUTE);
        oxygen_clear_bits16(chip, OXYGEN_GPIO_DATA, GPIO_INPUT_ROUTE);

        xonar_init_cs53x1(chip);
        xonar_init_ext_power(chip);
        xonar_hdmi_init(chip, &mut (*data).hdmi);
        xonar_enable_output(chip);

        snd_component_add((*chip).card, c"PCM1796".as_ptr());
        snd_component_add((*chip).card, c"CS5381".as_ptr());
    }
}

unsafe fn xonar_st_init_i2c(chip: *mut oxygen) {
    unsafe {
        oxygen_write16(
            chip,
            OXYGEN_2WIRE_BUS_STATUS,
            OXYGEN_2WIRE_LENGTH_8 | OXYGEN_2WIRE_INTERRUPT_MASK | OXYGEN_2WIRE_SPEED_STANDARD,
        );
    }
}

unsafe fn xonar_st_init_common(chip: *mut oxygen) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;

        (*data).generic.output_enable_bit = GPIO_ST_OUTPUT_ENABLE;
        (*data).dacs = (*chip).model.dac_channels_mixer / 2;
        (*data).h6 = (*chip).model.dac_channels_mixer > 2;
        (*data).hp_gain_offset = 2 * -18;

        pcm1796_init(chip);

        oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_INPUT_ROUTE | GPIO_ST_HP_REAR | GPIO_ST_MAGIC | GPIO_ST_HP);
        oxygen_clear_bits16(chip, OXYGEN_GPIO_DATA, GPIO_INPUT_ROUTE | GPIO_ST_HP_REAR | GPIO_ST_HP);

        xonar_init_cs53x1(chip);
        xonar_enable_output(chip);

        snd_component_add((*chip).card, c"PCM1792A".as_ptr());
        snd_component_add((*chip).card, c"CS5381".as_ptr());
    }
}

unsafe fn cs2000_registers_init(chip: *mut oxygen) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;

        cs2000_write(chip, CS2000_GLOBAL_CFG, CS2000_FREEZE);
        cs2000_write(chip, CS2000_DEV_CTRL, 0);
        cs2000_write(
            chip,
            CS2000_DEV_CFG_1,
            CS2000_R_MOD_SEL_1 | (0 << CS2000_R_SEL_SHIFT) | CS2000_AUX_OUT_SRC_REF_CLK | CS2000_EN_DEV_CFG_1,
        );
        cs2000_write(chip, CS2000_DEV_CFG_2, (0 << CS2000_LOCK_CLK_SHIFT) | CS2000_FRAC_N_SRC_STATIC);
        cs2000_write(chip, CS2000_RATIO_0 + 0, 0x00); /* 1.0 */
        cs2000_write(chip, CS2000_RATIO_0 + 1, 0x10);
        cs2000_write(chip, CS2000_RATIO_0 + 2, 0x00);
        cs2000_write(chip, CS2000_RATIO_0 + 3, 0x00);
        cs2000_write(chip, CS2000_FUN_CFG_1, (*data).cs2000_regs[CS2000_FUN_CFG_1 as usize]);
        cs2000_write(chip, CS2000_FUN_CFG_2, 0);
        cs2000_write(chip, CS2000_GLOBAL_CFG, CS2000_EN_DEV_CFG_2);
        msleep(3); /* PLL lock delay */
    }
}

unsafe fn xonar_st_init(chip: *mut oxygen) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;

        (*data).generic.anti_pop_delay = 100;
        (*data).h6 = (*chip).model.dac_channels_mixer > 2;
        (*data).has_cs2000 = true;
        (*data).cs2000_regs[CS2000_FUN_CFG_1 as usize] = CS2000_REF_CLK_DIV_1;
        (*data).broken_i2c = true;

        oxygen_write16(
            chip,
            OXYGEN_I2S_A_FORMAT,
            OXYGEN_RATE_48000
                | OXYGEN_I2S_FORMAT_I2S
                | OXYGEN_I2S_MCLK(if (*data).h6 { MCLK_256 } else { MCLK_512 })
                | OXYGEN_I2S_BITS_16
                | OXYGEN_I2S_MASTER
                | OXYGEN_I2S_BCLK_64,
        );

        xonar_st_init_i2c(chip);
        cs2000_registers_init(chip);
        xonar_st_init_common(chip);

        snd_component_add((*chip).card, c"CS2000".as_ptr());
    }
}

unsafe fn xonar_stx_init(chip: *mut oxygen) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;

        xonar_st_init_i2c(chip);
        (*data).generic.anti_pop_delay = 800;
        (*data).generic.ext_power_reg = OXYGEN_GPI_DATA;
        (*data).generic.ext_power_int_reg = OXYGEN_GPI_INTERRUPT_MASK;
        (*data).generic.ext_power_bit = GPI_EXT_POWER;
        xonar_init_ext_power(chip);
        xonar_st_init_common(chip);
    }
}

unsafe fn xonar_xense_init(chip: *mut oxygen) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;

        (*data).generic.ext_power_reg = OXYGEN_GPI_DATA;
        (*data).generic.ext_power_int_reg = OXYGEN_GPI_INTERRUPT_MASK;
        (*data).generic.ext_power_bit = GPI_EXT_POWER;
        xonar_init_ext_power(chip);

        (*data).generic.anti_pop_delay = 100;
        (*data).has_cs2000 = true;
        (*data).cs2000_regs[CS2000_FUN_CFG_1 as usize] = CS2000_REF_CLK_DIV_1;

        oxygen_write16(
            chip,
            OXYGEN_I2S_A_FORMAT,
            OXYGEN_RATE_48000
                | OXYGEN_I2S_FORMAT_I2S
                | OXYGEN_I2S_MCLK(MCLK_512)
                | OXYGEN_I2S_BITS_16
                | OXYGEN_I2S_MASTER
                | OXYGEN_I2S_BCLK_64,
        );

        xonar_st_init_i2c(chip);
        cs2000_registers_init(chip);

        (*data).generic.output_enable_bit = GPIO_XENSE_OUTPUT_ENABLE;
        (*data).dacs = 1;
        (*data).hp_gain_offset = 2 * -18;

        pcm1796_init(chip);

        oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_INPUT_ROUTE | GPIO_ST_HP_REAR | GPIO_ST_MAGIC | GPIO_XENSE_SPEAKERS);
        oxygen_clear_bits16(chip, OXYGEN_GPIO_DATA, GPIO_INPUT_ROUTE | GPIO_ST_HP_REAR | GPIO_XENSE_SPEAKERS);

        xonar_init_cs53x1(chip);
        xonar_enable_output(chip);

        snd_component_add((*chip).card, c"PCM1796".as_ptr());
        snd_component_add((*chip).card, c"CS5381".as_ptr());
        snd_component_add((*chip).card, c"CS2000".as_ptr());
    }
}

unsafe fn xonar_d2_cleanup(chip: *mut oxygen) {
    unsafe { xonar_disable_output(chip) }
}

unsafe fn xonar_hdav_cleanup(chip: *mut oxygen) {
    unsafe {
        xonar_hdmi_cleanup(chip);
        xonar_disable_output(chip);
        msleep(2);
    }
}

unsafe fn xonar_st_cleanup(chip: *mut oxygen) {
    unsafe { xonar_disable_output(chip) }
}

unsafe fn xonar_d2_suspend(chip: *mut oxygen) {
    unsafe { xonar_d2_cleanup(chip) }
}

unsafe fn xonar_hdav_suspend(chip: *mut oxygen) {
    unsafe { xonar_hdav_cleanup(chip) }
}

unsafe fn xonar_st_suspend(chip: *mut oxygen) {
    unsafe { xonar_st_cleanup(chip) }
}

unsafe fn xonar_d2_resume(chip: *mut oxygen) {
    unsafe {
        pcm1796_registers_init(chip);
        xonar_enable_output(chip);
    }
}

unsafe fn xonar_hdav_resume(chip: *mut oxygen) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_hdav;

        pcm1796_registers_init(chip);
        xonar_hdmi_resume(chip, &mut (*data).hdmi);
        xonar_enable_output(chip);
    }
}

unsafe fn xonar_stx_resume(chip: *mut oxygen) {
    unsafe {
        pcm1796_registers_init(chip);
        xonar_enable_output(chip);
    }
}

unsafe fn xonar_st_resume(chip: *mut oxygen) {
    unsafe {
        cs2000_registers_init(chip);
        xonar_stx_resume(chip);
    }
}

unsafe fn update_pcm1796_oversampling(chip: *mut oxygen) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;
        let reg: u8;

        if (*data).current_rate <= 48000 && !(*data).h6 {
            reg = PCM1796_OS_128;
        } else {
            reg = PCM1796_OS_64;
        }
        for i in 0..(*data).dacs {
            pcm1796_write_cached(chip, i, 20, reg);
        }
    }
}

unsafe fn update_pcm1796_deemph(chip: *mut oxygen) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;
        let mut reg: u8;

        reg = (*data).pcm1796_regs[0][(18 - PCM1796_REG_BASE) as usize] & !PCM1796_DMF_MASK;
        if (*data).current_rate == 48000 {
            reg |= PCM1796_DMF_48;
        } else if (*data).current_rate == 44100 {
            reg |= PCM1796_DMF_441;
        } else if (*data).current_rate == 32000 {
            reg |= PCM1796_DMF_32;
        }
        for i in 0..(*data).dacs {
            pcm1796_write_cached(chip, i, 18, reg);
        }
    }
}

unsafe fn set_pcm1796_params(chip: *mut oxygen, params: *mut snd_pcm_hw_params) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;

        msleep(1);
        (*data).current_rate = params_rate(params);
        update_pcm1796_oversampling(chip);
        update_pcm1796_deemph(chip);
    }
}

unsafe fn update_pcm1796_volume(chip: *mut oxygen) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;
        let mut gain_offset: i8;

        gain_offset = if (*data).hp_active { (*data).hp_gain_offset } else { 0 };
        for i in 0..(*data).dacs {
            pcm1796_write_cached(chip, i, 16, (*chip).dac_volume[(i * 2) as usize].wrapping_add(gain_offset as u8));
            pcm1796_write_cached(chip, i, 17, (*chip).dac_volume[(i * 2 + 1) as usize].wrapping_add(gain_offset as u8));
            gain_offset = 0;
        }
    }
}

unsafe fn update_pcm1796_mute(chip: *mut oxygen) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;
        let mut value: u8;

        value = (*data).pcm1796_regs[0][(18 - PCM1796_REG_BASE) as usize];
        if (*chip).dac_mute {
            value |= PCM1796_MUTE;
        } else {
            value &= !PCM1796_MUTE;
        }
        for i in 0..(*data).dacs {
            pcm1796_write_cached(chip, i, 18, value);
        }
    }
}

unsafe fn update_cs2000_rate(chip: *mut oxygen, rate: u32) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;
        let mut rate_mclk: u32;
        let reg: u8;

        match rate {
            32000 | 64000 => rate_mclk = OXYGEN_RATE_32000,
            44100 | 88200 | 176400 => rate_mclk = OXYGEN_RATE_44100,
            48000 | 96000 | 192000 | _ => rate_mclk = OXYGEN_RATE_48000,
        }

        if rate <= 96000 && (rate > 48000 || (*data).h6) {
            rate_mclk |= OXYGEN_I2S_MCLK(MCLK_256);
            reg = CS2000_REF_CLK_DIV_1;
        } else {
            rate_mclk |= OXYGEN_I2S_MCLK(MCLK_512);
            reg = CS2000_REF_CLK_DIV_2;
        }

        oxygen_write16_masked(chip, OXYGEN_I2S_A_FORMAT, rate_mclk, OXYGEN_I2S_RATE_MASK | OXYGEN_I2S_MCLK_MASK);
        cs2000_write_cached(chip, CS2000_FUN_CFG_1, reg);
        msleep(3); /* PLL lock delay */
    }
}

unsafe fn set_st_params(chip: *mut oxygen, params: *mut snd_pcm_hw_params) {
    unsafe {
        update_cs2000_rate(chip, params_rate(params));
        set_pcm1796_params(chip, params);
    }
}

unsafe fn set_hdav_params(chip: *mut oxygen, params: *mut snd_pcm_hw_params) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_hdav;

        set_pcm1796_params(chip, params);
        xonar_set_hdmi_params(chip, &mut (*data).hdmi, params);
    }
}

static alt_switch: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"Analog Loopback Switch".as_ptr(),
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(xonar_gpio_bit_switch_get),
    put: Some(xonar_gpio_bit_switch_put),
    private_value: GPIO_D2_ALT as _,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn rolloff_info(_ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> i32 {
    static NAMES: [*const core::ffi::c_char; 2] = [c"Sharp Roll-off".as_ptr(), c"Slow Roll-off".as_ptr()];
    unsafe { snd_ctl_enum_info(info, 1, 2, NAMES.as_ptr()) }
}

unsafe fn rolloff_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = (*ctl).private_data as *mut oxygen;
        let data = (*chip).model_data as *mut xonar_pcm179x;

        (*value).value.enumerated.item[0] =
            (((*data).pcm1796_regs[0][(19 - PCM1796_REG_BASE) as usize] & PCM1796_FLT_MASK) != PCM1796_FLT_SHARP) as _;
        0
    }
}

unsafe fn rolloff_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = (*ctl).private_data as *mut oxygen;
        let data = (*chip).model_data as *mut xonar_pcm179x;
        let mut reg: u8;

        let _guard = guard_mutex(&mut (*chip).mutex);
        reg = (*data).pcm1796_regs[0][(19 - PCM1796_REG_BASE) as usize];
        reg &= !PCM1796_FLT_MASK;
        if (*value).value.enumerated.item[0] == 0 {
            reg |= PCM1796_FLT_SHARP;
        } else {
            reg |= PCM1796_FLT_SLOW;
        }
        let changed = (reg != (*data).pcm1796_regs[0][(19 - PCM1796_REG_BASE) as usize]) as i32;
        if changed != 0 {
            for i in 0..(*data).dacs {
                pcm1796_write(chip, i, 19, reg);
            }
        }
        changed
    }
}

static rolloff_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"DAC Filter Playback Enum".as_ptr(),
    info: Some(rolloff_info),
    get: Some(rolloff_get),
    put: Some(rolloff_put),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn deemph_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = (*ctl).private_data as *mut oxygen;
        let data = (*chip).model_data as *mut xonar_pcm179x;

        (*value).value.integer.value[0] =
            (((*data).pcm1796_regs[0][(18 - PCM1796_REG_BASE) as usize] & PCM1796_DME) != 0) as _;
        0
    }
}

unsafe fn deemph_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = (*ctl).private_data as *mut oxygen;
        let data = (*chip).model_data as *mut xonar_pcm179x;
        let mut reg: u8;

        let _guard = guard_mutex(&mut (*chip).mutex);
        reg = (*data).pcm1796_regs[0][(18 - PCM1796_REG_BASE) as usize];
        if (*value).value.integer.value[0] == 0 {
            reg &= !PCM1796_DME;
        } else {
            reg |= PCM1796_DME;
        }
        let changed = (reg != (*data).pcm1796_regs[0][(18 - PCM1796_REG_BASE) as usize]) as i32;
        if changed != 0 {
            for i in 0..(*data).dacs {
                pcm1796_write(chip, i, 18, reg);
            }
        }
        changed
    }
}

static deemph_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"De-emphasis Playback Switch".as_ptr(),
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(deemph_get),
    put: Some(deemph_put),
    ..unsafe { core::mem::zeroed() }
};

static hdav_hdmi_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"HDMI Playback Switch".as_ptr(),
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(xonar_gpio_bit_switch_get),
    put: Some(xonar_gpio_bit_switch_put),
    private_value: (GPIO_HDAV_OUTPUT_ENABLE | XONAR_GPIO_BIT_INVERT) as _,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn st_output_switch_info(_ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> i32 {
    static NAMES: [*const core::ffi::c_char; 3] = [c"Speakers".as_ptr(), c"Headphones".as_ptr(), c"FP Headphones".as_ptr()];
    unsafe { snd_ctl_enum_info(info, 1, 3, NAMES.as_ptr()) }
}

unsafe fn st_output_switch_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = (*ctl).private_data as *mut oxygen;
        let gpio = oxygen_read16(chip, OXYGEN_GPIO_DATA);

        if (gpio & GPIO_ST_HP as u16) == 0 {
            (*value).value.enumerated.item[0] = 0;
        } else if (gpio & GPIO_ST_HP_REAR as u16) != 0 {
            (*value).value.enumerated.item[0] = 1;
        } else {
            (*value).value.enumerated.item[0] = 2;
        }
        0
    }
}

unsafe fn st_output_switch_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = (*ctl).private_data as *mut oxygen;
        let data = (*chip).model_data as *mut xonar_pcm179x;

        let _guard = guard_mutex(&mut (*chip).mutex);
        let gpio_old = oxygen_read16(chip, OXYGEN_GPIO_DATA);
        let mut gpio = gpio_old;
        match (*value).value.enumerated.item[0] {
            0 => gpio &= !(GPIO_ST_HP | GPIO_ST_HP_REAR) as u16,
            1 => gpio |= (GPIO_ST_HP | GPIO_ST_HP_REAR) as u16,
            2 => gpio = (gpio | GPIO_ST_HP as u16) & !(GPIO_ST_HP_REAR as u16),
            _ => {}
        }
        oxygen_write16(chip, OXYGEN_GPIO_DATA, gpio as u32);
        (*data).hp_active = (gpio & GPIO_ST_HP as u16) != 0;
        update_pcm1796_volume(chip);
        (gpio != gpio_old) as i32
    }
}

unsafe fn st_hp_volume_offset_info(_ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> i32 {
    static NAMES: [*const core::ffi::c_char; 4] = [
        c"< 32 ohms".as_ptr(),
        c"32-64 ohms".as_ptr(),
        c"64-300 ohms".as_ptr(),
        c"300-600 ohms".as_ptr(),
    ];
    unsafe { snd_ctl_enum_info(info, 1, 4, NAMES.as_ptr()) }
}

unsafe fn st_hp_volume_offset_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = (*ctl).private_data as *mut oxygen;
        let data = (*chip).model_data as *mut xonar_pcm179x;

        let _guard = guard_mutex(&mut (*chip).mutex);
        if (*data).hp_gain_offset < 2 * -12 {
            (*value).value.enumerated.item[0] = 0;
        } else if (*data).hp_gain_offset < 2 * -6 {
            (*value).value.enumerated.item[0] = 1;
        } else if (*data).hp_gain_offset < 0 {
            (*value).value.enumerated.item[0] = 2;
        } else {
            (*value).value.enumerated.item[0] = 3;
        }
        0
    }
}

unsafe fn st_hp_volume_offset_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> i32 {
    static OFFSETS: [i8; 4] = [2 * -18, 2 * -12, 2 * -6, 0];
    unsafe {
        let chip = (*ctl).private_data as *mut oxygen;
        let data = (*chip).model_data as *mut xonar_pcm179x;

        if (*value).value.enumerated.item[0] > 3 {
            return -EINVAL;
        }
        let offset = OFFSETS[(*value).value.enumerated.item[0] as usize];
        let _guard = guard_mutex(&mut (*chip).mutex);
        let changed = (offset != (*data).hp_gain_offset) as i32;
        if changed != 0 {
            (*data).hp_gain_offset = offset;
            update_pcm1796_volume(chip);
        }
        changed
    }
}

static st_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Analog Output".as_ptr(),
        info: Some(st_output_switch_info),
        get: Some(st_output_switch_get),
        put: Some(st_output_switch_put),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Headphones Impedance Playback Enum".as_ptr(),
        info: Some(st_hp_volume_offset_info),
        get: Some(st_hp_volume_offset_get),
        put: Some(st_hp_volume_offset_put),
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe fn xense_output_switch_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = (*ctl).private_data as *mut oxygen;
        let gpio = oxygen_read16(chip, OXYGEN_GPIO_DATA);

        if (gpio & GPIO_XENSE_SPEAKERS as u16) != 0 {
            (*value).value.enumerated.item[0] = 0;
        } else if (gpio & GPIO_ST_HP_REAR as u16) != 0 {
            (*value).value.enumerated.item[0] = 1;
        } else {
            (*value).value.enumerated.item[0] = 2;
        }
        0
    }
}

unsafe fn xense_output_switch_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = (*ctl).private_data as *mut oxygen;
        let data = (*chip).model_data as *mut xonar_pcm179x;

        let _guard = guard_mutex(&mut (*chip).mutex);
        let gpio_old = oxygen_read16(chip, OXYGEN_GPIO_DATA);
        let mut gpio = gpio_old;
        match (*value).value.enumerated.item[0] {
            0 => gpio |= (GPIO_XENSE_SPEAKERS | GPIO_ST_HP_REAR) as u16,
            1 => gpio = (gpio | GPIO_ST_HP_REAR as u16) & !(GPIO_XENSE_SPEAKERS as u16),
            2 => gpio &= !(GPIO_XENSE_SPEAKERS | GPIO_ST_HP_REAR) as u16,
            _ => {}
        }
        oxygen_write16(chip, OXYGEN_GPIO_DATA, gpio as u32);
        (*data).hp_active = (gpio & GPIO_XENSE_SPEAKERS as u16) == 0;
        update_pcm1796_volume(chip);
        (gpio != gpio_old) as i32
    }
}

static xense_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Analog Output".as_ptr(),
        info: Some(st_output_switch_info),
        get: Some(xense_output_switch_get),
        put: Some(xense_output_switch_put),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Headphones Impedance Playback Enum".as_ptr(),
        info: Some(st_hp_volume_offset_info),
        get: Some(st_hp_volume_offset_get),
        put: Some(st_hp_volume_offset_put),
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe fn xonar_line_mic_ac97_switch(chip: *mut oxygen, reg: u32, mute: u32) {
    unsafe {
        if reg == AC97_LINE {
            let _guard = guard_spinlock_irq(&mut (*chip).reg_lock);
            oxygen_write16_masked(chip, OXYGEN_GPIO_DATA, if mute != 0 { GPIO_INPUT_ROUTE } else { 0 }, GPIO_INPUT_ROUTE);
        }
    }
}

static pcm1796_db_scale: [u32; 4] = TLV_DB_SCALE_ITEM(-6000, 50, 0);

unsafe fn xonar_d2_control_filter(template: *mut snd_kcontrol_new) -> i32 {
    unsafe {
        if strncmp((*template).name, c"CD Capture ".as_ptr(), 11) == 0 {
            /* CD in is actually connected to the video in pin */
            (*template).private_value ^= (AC97_CD ^ AC97_VIDEO) as _;
        }
        0
    }
}

unsafe fn xonar_st_h6_control_filter(template: *mut snd_kcontrol_new) -> i32 {
    unsafe {
        if strncmp((*template).name, c"Master Playback ".as_ptr(), 16) == 0 {
            /* no volume/mute, as I2C to the third DAC does not work */
            return 1;
        }
        0
    }
}

unsafe fn add_pcm1796_controls(chip: *mut oxygen) -> i32 {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;

        if !(*data).broken_i2c {
            let mut err = snd_ctl_add((*chip).card, snd_ctl_new1(&rolloff_control, chip as *mut _));
            if err < 0 {
                return err;
            }
            err = snd_ctl_add((*chip).card, snd_ctl_new1(&deemph_control, chip as *mut _));
            if err < 0 {
                return err;
            }
        }
        0
    }
}

unsafe fn xonar_d2_mixer_init(chip: *mut oxygen) -> i32 {
    unsafe {
        let mut err = snd_ctl_add((*chip).card, snd_ctl_new1(&alt_switch, chip as *mut _));
        if err < 0 {
            return err;
        }
        err = add_pcm1796_controls(chip);
        if err < 0 {
            return err;
        }
        0
    }
}

unsafe fn xonar_hdav_mixer_init(chip: *mut oxygen) -> i32 {
    unsafe {
        let mut err = snd_ctl_add((*chip).card, snd_ctl_new1(&hdav_hdmi_control, chip as *mut _));
        if err < 0 {
            return err;
        }
        err = add_pcm1796_controls(chip);
        if err < 0 {
            return err;
        }
        0
    }
}

unsafe fn xonar_st_mixer_init(chip: *mut oxygen) -> i32 {
    unsafe {
        for i in 0..st_controls.len() {
            let err = snd_ctl_add((*chip).card, snd_ctl_new1(&st_controls[i], chip as *mut _));
            if err < 0 {
                return err;
            }
        }
        let err = add_pcm1796_controls(chip);
        if err < 0 {
            return err;
        }
        0
    }
}

unsafe fn xonar_xense_mixer_init(chip: *mut oxygen) -> i32 {
    unsafe {
        for i in 0..xense_controls.len() {
            let err = snd_ctl_add((*chip).card, snd_ctl_new1(&xense_controls[i], chip as *mut _));
            if err < 0 {
                return err;
            }
        }
        let err = add_pcm1796_controls(chip);
        if err < 0 {
            return err;
        }
        0
    }
}

unsafe fn dump_pcm1796_registers(chip: *mut oxygen, buffer: *mut snd_info_buffer) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;

        for dac in 0..(*data).dacs {
            snd_iprintf(buffer, c"\nPCM1796 %u:".as_ptr(), dac + 1);
            for i in 0..5 {
                snd_iprintf(buffer, c" %02x".as_ptr(), (*data).pcm1796_regs[dac as usize][i]);
            }
        }
        snd_iprintf(buffer, c"\n".as_ptr());
    }
}

unsafe fn dump_cs2000_registers(chip: *mut oxygen, buffer: *mut snd_info_buffer) {
    unsafe {
        let data = (*chip).model_data as *mut xonar_pcm179x;

        if (*data).has_cs2000 {
            snd_iprintf(buffer, c"\nCS2000:\n00:   ".as_ptr());
            for i in 1..0x10 {
                snd_iprintf(buffer, c" %02x".as_ptr(), (*data).cs2000_regs[i]);
            }
            snd_iprintf(buffer, c"\n10:".as_ptr());
            for i in 0x10..0x1f {
                snd_iprintf(buffer, c" %02x".as_ptr(), (*data).cs2000_regs[i]);
            }
            snd_iprintf(buffer, c"\n".as_ptr());
        }
    }
}

unsafe fn dump_st_registers(chip: *mut oxygen, buffer: *mut snd_info_buffer) {
    unsafe {
        dump_pcm1796_registers(chip, buffer);
        dump_cs2000_registers(chip, buffer);
    }
}

static mut model_xonar_d2: oxygen_model = oxygen_model {
    longname: c"Asus Virtuoso 200".as_ptr(),
    chip: c"AV200".as_ptr(),
    init: Some(xonar_d2_init),
    control_filter: Some(xonar_d2_control_filter),
    mixer_init: Some(xonar_d2_mixer_init),
    cleanup: Some(xonar_d2_cleanup),
    suspend: Some(xonar_d2_suspend),
    resume: Some(xonar_d2_resume),
    set_dac_params: Some(set_pcm1796_params),
    set_adc_params: Some(xonar_set_cs53x1_params),
    update_dac_volume: Some(update_pcm1796_volume),
    update_dac_mute: Some(update_pcm1796_mute),
    dump_registers: Some(dump_pcm1796_registers),
    dac_tlv: pcm1796_db_scale.as_ptr(),
    model_data_size: core::mem::size_of::<xonar_pcm179x>() as _,
    device_config: PLAYBACK_0_TO_I2S | PLAYBACK_1_TO_SPDIF | CAPTURE_0_FROM_I2S_2 | CAPTURE_1_FROM_SPDIF | MIDI_OUTPUT | MIDI_INPUT | AC97_CD_INPUT,
    dac_channels_pcm: 8,
    dac_channels_mixer: 8,
    dac_volume_min: 255 - 2 * 60,
    dac_volume_max: 255,
    misc_flags: OXYGEN_MISC_MIDI,
    function_flags: OXYGEN_FUNCTION_SPI | OXYGEN_FUNCTION_ENABLE_SPI_4_5,
    dac_mclks: OXYGEN_MCLKS(512, 128, 128),
    adc_mclks: OXYGEN_MCLKS(256, 128, 128),
    dac_i2s_format: OXYGEN_I2S_FORMAT_I2S,
    adc_i2s_format: OXYGEN_I2S_FORMAT_LJUST,
    ..unsafe { core::mem::zeroed() }
};

static mut model_xonar_hdav: oxygen_model = oxygen_model {
    longname: c"Asus Virtuoso 200".as_ptr(),
    chip: c"AV200".as_ptr(),
    init: Some(xonar_hdav_init),
    mixer_init: Some(xonar_hdav_mixer_init),
    cleanup: Some(xonar_hdav_cleanup),
    suspend: Some(xonar_hdav_suspend),
    resume: Some(xonar_hdav_resume),
    pcm_hardware_filter: Some(xonar_hdmi_pcm_hardware_filter),
    set_dac_params: Some(set_hdav_params),
    set_adc_params: Some(xonar_set_cs53x1_params),
    update_dac_volume: Some(update_pcm1796_volume),
    update_dac_mute: Some(update_pcm1796_mute),
    uart_input: Some(xonar_hdmi_uart_input),
    ac97_switch: Some(xonar_line_mic_ac97_switch),
    dump_registers: Some(dump_pcm1796_registers),
    dac_tlv: pcm1796_db_scale.as_ptr(),
    model_data_size: core::mem::size_of::<xonar_hdav>() as _,
    device_config: PLAYBACK_0_TO_I2S | PLAYBACK_1_TO_SPDIF | CAPTURE_0_FROM_I2S_2 | CAPTURE_1_FROM_SPDIF,
    dac_channels_pcm: 8,
    dac_channels_mixer: 2,
    dac_volume_min: 255 - 2 * 60,
    dac_volume_max: 255,
    misc_flags: OXYGEN_MISC_MIDI,
    function_flags: OXYGEN_FUNCTION_2WIRE,
    dac_mclks: OXYGEN_MCLKS(512, 128, 128),
    adc_mclks: OXYGEN_MCLKS(256, 128, 128),
    dac_i2s_format: OXYGEN_I2S_FORMAT_I2S,
    adc_i2s_format: OXYGEN_I2S_FORMAT_LJUST,
    ..unsafe { core::mem::zeroed() }
};

static mut model_xonar_st: oxygen_model = oxygen_model {
    longname: c"Asus Virtuoso 100".as_ptr(),
    chip: c"AV200".as_ptr(),
    init: Some(xonar_st_init),
    mixer_init: Some(xonar_st_mixer_init),
    cleanup: Some(xonar_st_cleanup),
    suspend: Some(xonar_st_suspend),
    resume: Some(xonar_st_resume),
    set_dac_params: Some(set_st_params),
    set_adc_params: Some(xonar_set_cs53x1_params),
    update_dac_volume: Some(update_pcm1796_volume),
    update_dac_mute: Some(update_pcm1796_mute),
    ac97_switch: Some(xonar_line_mic_ac97_switch),
    dump_registers: Some(dump_st_registers),
    dac_tlv: pcm1796_db_scale.as_ptr(),
    model_data_size: core::mem::size_of::<xonar_pcm179x>() as _,
    device_config: PLAYBACK_0_TO_I2S | PLAYBACK_1_TO_SPDIF | CAPTURE_0_FROM_I2S_2 | CAPTURE_1_FROM_SPDIF | AC97_FMIC_SWITCH,
    dac_channels_pcm: 2,
    dac_channels_mixer: 2,
    dac_volume_min: 255 - 2 * 60,
    dac_volume_max: 255,
    function_flags: OXYGEN_FUNCTION_2WIRE,
    dac_mclks: OXYGEN_MCLKS(512, 128, 128),
    adc_mclks: OXYGEN_MCLKS(256, 128, 128),
    dac_i2s_format: OXYGEN_I2S_FORMAT_I2S,
    adc_i2s_format: OXYGEN_I2S_FORMAT_LJUST,
    ..unsafe { core::mem::zeroed() }
};

#[no_mangle]
pub unsafe extern "C" fn get_xonar_pcm179x_model(chip: *mut oxygen, id: *const pci_device_id) -> i32 {
    unsafe {
        match (*id).subdevice {
            0x8269 => {
                (*chip).model = model_xonar_d2;
                (*chip).model.shortname = c"Xonar D2".as_ptr();
            }
            0x82b7 => {
                (*chip).model = model_xonar_d2;
                (*chip).model.shortname = c"Xonar D2X".as_ptr();
                (*chip).model.init = Some(xonar_d2x_init);
            }
            0x8314 => {
                (*chip).model = model_xonar_hdav;
                oxygen_clear_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_DB_MASK);
                match oxygen_read16(chip, OXYGEN_GPIO_DATA) as u32 & GPIO_DB_MASK {
                    GPIO_DB_H6 => {
                        (*chip).model.shortname = c"Xonar HDAV1.3+H6".as_ptr();
                        (*chip).model.dac_channels_mixer = 8;
                        (*chip).model.dac_mclks = OXYGEN_MCLKS(256, 128, 128);
                    }
                    _ => {
                        (*chip).model.shortname = c"Xonar HDAV1.3".as_ptr();
                    }
                }
            }
            0x835d => {
                (*chip).model = model_xonar_st;
                oxygen_clear_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_DB_MASK);
                match oxygen_read16(chip, OXYGEN_GPIO_DATA) as u32 & GPIO_DB_MASK {
                    GPIO_DB_H6 => {
                        (*chip).model.shortname = c"Xonar ST+H6".as_ptr();
                        (*chip).model.control_filter = Some(xonar_st_h6_control_filter);
                        (*chip).model.dac_channels_pcm = 8;
                        (*chip).model.dac_channels_mixer = 8;
                        (*chip).model.dac_volume_min = 255;
                        (*chip).model.dac_mclks = OXYGEN_MCLKS(256, 128, 128);
                    }
                    _ => {
                        (*chip).model.shortname = c"Xonar ST".as_ptr();
                    }
                }
            }
            0x835c => {
                (*chip).model = model_xonar_st;
                (*chip).model.shortname = c"Xonar STX".as_ptr();
                (*chip).model.init = Some(xonar_stx_init);
                (*chip).model.resume = Some(xonar_stx_resume);
                (*chip).model.set_dac_params = Some(set_pcm1796_params);
            }
            0x85f4 => {
                (*chip).model = model_xonar_st;
                oxygen_clear_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_DB_MASK);
                match oxygen_read16(chip, OXYGEN_GPIO_DATA) as u32 & GPIO_DB_MASK {
                    GPIO_DB_H6 => {
                        (*chip).model.shortname = c"Xonar STX II+H6".as_ptr();
                        (*chip).model.dac_channels_pcm = 8;
                        (*chip).model.dac_channels_mixer = 8;
                        (*chip).model.dac_mclks = OXYGEN_MCLKS(256, 128, 128);
                    }
                    _ => {
                        (*chip).model.shortname = c"Xonar STX II".as_ptr();
                    }
                }
                (*chip).model.init = Some(xonar_stx_init);
                (*chip).model.resume = Some(xonar_stx_resume);
                (*chip).model.set_dac_params = Some(set_pcm1796_params);
            }
            0x8428 => {
                (*chip).model = model_xonar_st;
                (*chip).model.shortname = c"Xonar Xense".as_ptr();
                (*chip).model.chip = c"AV100".as_ptr();
                (*chip).model.init = Some(xonar_xense_init);
                (*chip).model.mixer_init = Some(xonar_xense_mixer_init);
            }
            _ => return -EINVAL,
        }
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
