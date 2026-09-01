// SPDX-License-Identifier: GPL-2.0-only
/*
 * card driver for models with CS4398/CS4362A DACs (Xonar D1/DX)
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

/*
 * Xonar D1/DX
 * -----------
 *
 * CMI8788:
 *
 *   I2C <-> CS4398 (addr 1001111) (front)
 *       <-> CS4362A (addr 0011000) (surround, center/LFE, back)
 *
 *   GPI 0 <- external power present (DX only)
 *
 *   GPIO 0 -> enable output to speakers
 *   GPIO 1 -> route output to front panel
 *   GPIO 2 -> M0 of CS5361
 *   GPIO 3 -> M1 of CS5361
 *   GPIO 6 -> ?
 *   GPIO 7 -> ?
 *   GPIO 8 -> route input jack to line-in (0) or mic-in (1)
 *
 * CM9780:
 *
 *   LINE_OUT -> input of ADC
 *
 *   AUX_IN  <- aux
 *   MIC_IN  <- mic
 *   FMIC_IN <- front mic
 *
 *   GPO 0 -> route line-in (0) or AC97 output (1) to CS5361 input
 */

// C dependencies: linux/pci.h, linux/delay.h, sound/ac97_codec.h,
// sound/control.h, sound/core.h, sound/pcm.h, sound/pcm_params.h,
// sound/tlv.h, xonar.h, cm9780.h, cs4398.h, cs4362a.h.

const GPI_EXT_POWER: u32 = 0x01;
const GPIO_D1_OUTPUT_ENABLE: u32 = 0x0001;
const GPIO_D1_FRONT_PANEL: u32 = 0x0002;
const GPIO_D1_MAGIC: u32 = 0x00c0;
const GPIO_D1_INPUT_ROUTE: u32 = 0x0100;

const I2C_DEVICE_CS4398: u8 = 0x9e; /* 10011, AD1=1, AD0=1, /W=0 */
const I2C_DEVICE_CS4362A: u8 = 0x30; /* 001100, AD0=0, /W=0 */

#[repr(C)]
struct xonar_cs43xx {
    generic: xonar_generic,
    cs4398_regs: [u8; 8],
    cs4362a_regs: [u8; 15],
}

unsafe fn cs4398_write(chip: *mut oxygen, reg: u8, value: u8) {
    let data = (*chip).model_data as *mut xonar_cs43xx;

    oxygen_write_i2c(chip, I2C_DEVICE_CS4398, reg, value);
    if (reg as usize) < (*data).cs4398_regs.len() {
        (*data).cs4398_regs[reg as usize] = value;
    }
}

unsafe fn cs4398_write_cached(chip: *mut oxygen, reg: u8, value: u8) {
    let data = (*chip).model_data as *mut xonar_cs43xx;

    if value != (*data).cs4398_regs[reg as usize] {
        cs4398_write(chip, reg, value);
    }
}

unsafe fn cs4362a_write(chip: *mut oxygen, reg: u8, value: u8) {
    let data = (*chip).model_data as *mut xonar_cs43xx;

    oxygen_write_i2c(chip, I2C_DEVICE_CS4362A, reg, value);
    if (reg as usize) < (*data).cs4362a_regs.len() {
        (*data).cs4362a_regs[reg as usize] = value;
    }
}

unsafe fn cs4362a_write_cached(chip: *mut oxygen, reg: u8, value: u8) {
    let data = (*chip).model_data as *mut xonar_cs43xx;

    if value != (*data).cs4362a_regs[reg as usize] {
        cs4362a_write(chip, reg, value);
    }
}

unsafe fn cs43xx_registers_init(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut xonar_cs43xx;
    let mut i: u32;

    /* set CPEN (control port mode) and power down */
    cs4398_write(chip, 8, CS4398_CPEN | CS4398_PDN);
    cs4362a_write(chip, 0x01, CS4362A_PDN | CS4362A_CPEN);
    /* configure */
    cs4398_write(chip, 2, (*data).cs4398_regs[2]);
    cs4398_write(chip, 3, CS4398_ATAPI_B_R | CS4398_ATAPI_A_L);
    cs4398_write(chip, 4, (*data).cs4398_regs[4]);
    cs4398_write(chip, 5, (*data).cs4398_regs[5]);
    cs4398_write(chip, 6, (*data).cs4398_regs[6]);
    cs4398_write(chip, 7, (*data).cs4398_regs[7]);
    cs4362a_write(chip, 0x02, CS4362A_DIF_LJUST);
    cs4362a_write(
        chip,
        0x03,
        CS4362A_MUTEC_6 | CS4362A_AMUTE | CS4362A_RMP_UP | CS4362A_ZERO_CROSS | CS4362A_SOFT_RAMP,
    );
    cs4362a_write(chip, 0x04, (*data).cs4362a_regs[0x04]);
    cs4362a_write(chip, 0x05, 0);
    i = 6;
    while i <= 14 {
        cs4362a_write(chip, i as u8, (*data).cs4362a_regs[i as usize]);
        i += 1;
    }
    /* clear power down */
    cs4398_write(chip, 8, CS4398_CPEN);
    cs4362a_write(chip, 0x01, CS4362A_CPEN);
}

unsafe fn xonar_d1_init(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut xonar_cs43xx;

    (*data).generic.anti_pop_delay = 800;
    (*data).generic.output_enable_bit = GPIO_D1_OUTPUT_ENABLE;
    (*data).cs4398_regs[2] = CS4398_FM_SINGLE | CS4398_DEM_NONE | CS4398_DIF_LJUST;
    (*data).cs4398_regs[4] = CS4398_MUTEP_LOW | CS4398_MUTE_B | CS4398_MUTE_A | CS4398_PAMUTE;
    (*data).cs4398_regs[5] = 60 * 2;
    (*data).cs4398_regs[6] = 60 * 2;
    (*data).cs4398_regs[7] = CS4398_RMP_DN | CS4398_RMP_UP | CS4398_ZERO_CROSS | CS4398_SOFT_RAMP;
    (*data).cs4362a_regs[4] = CS4362A_RMP_DN | CS4362A_DEM_NONE;
    (*data).cs4362a_regs[6] = CS4362A_FM_SINGLE | CS4362A_ATAPI_B_R | CS4362A_ATAPI_A_L;
    (*data).cs4362a_regs[7] = 60 | CS4362A_MUTE;
    (*data).cs4362a_regs[8] = 60 | CS4362A_MUTE;
    (*data).cs4362a_regs[9] = (*data).cs4362a_regs[6];
    (*data).cs4362a_regs[10] = 60 | CS4362A_MUTE;
    (*data).cs4362a_regs[11] = 60 | CS4362A_MUTE;
    (*data).cs4362a_regs[12] = (*data).cs4362a_regs[6];
    (*data).cs4362a_regs[13] = 60 | CS4362A_MUTE;
    (*data).cs4362a_regs[14] = 60 | CS4362A_MUTE;

    oxygen_write16(
        chip,
        OXYGEN_2WIRE_BUS_STATUS,
        OXYGEN_2WIRE_LENGTH_8 | OXYGEN_2WIRE_INTERRUPT_MASK | OXYGEN_2WIRE_SPEED_FAST,
    );

    cs43xx_registers_init(chip);

    oxygen_set_bits16(
        chip,
        OXYGEN_GPIO_CONTROL,
        GPIO_D1_FRONT_PANEL | GPIO_D1_MAGIC | GPIO_D1_INPUT_ROUTE,
    );
    oxygen_clear_bits16(chip, OXYGEN_GPIO_DATA, GPIO_D1_FRONT_PANEL | GPIO_D1_INPUT_ROUTE);

    xonar_init_cs53x1(chip);
    xonar_enable_output(chip);

    snd_component_add((*chip).card, c"CS4398".as_ptr());
    snd_component_add((*chip).card, c"CS4362A".as_ptr());
    snd_component_add((*chip).card, c"CS5361".as_ptr());
}

unsafe fn xonar_dx_init(chip: *mut oxygen) {
    let data = (*chip).model_data as *mut xonar_cs43xx;

    (*data).generic.ext_power_reg = OXYGEN_GPI_DATA;
    (*data).generic.ext_power_int_reg = OXYGEN_GPI_INTERRUPT_MASK;
    (*data).generic.ext_power_bit = GPI_EXT_POWER;
    xonar_init_ext_power(chip);
    xonar_d1_init(chip);
}

unsafe fn xonar_d1_cleanup(chip: *mut oxygen) {
    xonar_disable_output(chip);
    cs4362a_write(chip, 0x01, CS4362A_PDN | CS4362A_CPEN);
    oxygen_clear_bits8(chip, OXYGEN_FUNCTION, OXYGEN_FUNCTION_RESET_CODEC);
}

unsafe fn xonar_d1_suspend(chip: *mut oxygen) {
    xonar_d1_cleanup(chip);
}

unsafe fn xonar_d1_resume(chip: *mut oxygen) {
    oxygen_set_bits8(chip, OXYGEN_FUNCTION, OXYGEN_FUNCTION_RESET_CODEC);
    msleep(1);
    cs43xx_registers_init(chip);
    xonar_enable_output(chip);
}

unsafe fn set_cs43xx_params(chip: *mut oxygen, params: *mut snd_pcm_hw_params) {
    let data = (*chip).model_data as *mut xonar_cs43xx;
    let mut cs4398_fm: u8;
    let mut cs4362a_fm: u8;

    if params_rate(params) <= 50000 {
        cs4398_fm = CS4398_FM_SINGLE;
        cs4362a_fm = CS4362A_FM_SINGLE;
    } else if params_rate(params) <= 100000 {
        cs4398_fm = CS4398_FM_DOUBLE;
        cs4362a_fm = CS4362A_FM_DOUBLE;
    } else {
        cs4398_fm = CS4398_FM_QUAD;
        cs4362a_fm = CS4362A_FM_QUAD;
    }
    cs4398_fm |= CS4398_DEM_NONE | CS4398_DIF_LJUST;
    cs4398_write_cached(chip, 2, cs4398_fm);
    cs4362a_fm |= (*data).cs4362a_regs[6] & !CS4362A_FM_MASK;
    cs4362a_write_cached(chip, 6, cs4362a_fm);
    cs4362a_write_cached(chip, 12, cs4362a_fm);
    cs4362a_fm &= CS4362A_FM_MASK;
    cs4362a_fm |= (*data).cs4362a_regs[9] & !CS4362A_FM_MASK;
    cs4362a_write_cached(chip, 9, cs4362a_fm);
}

unsafe fn update_cs4362a_volumes(chip: *mut oxygen) {
    let mut i: u32;
    let mute: u8;

    mute = if (*chip).dac_mute { CS4362A_MUTE } else { 0 };
    i = 0;
    while i < 6 {
        cs4362a_write_cached(
            chip,
            (7 + i + i / 2) as u8,
            (127 - (*chip).dac_volume[(2 + i) as usize]) | mute,
        );
        i += 1;
    }
}

unsafe fn update_cs43xx_volume(chip: *mut oxygen) {
    cs4398_write_cached(chip, 5, (127 - (*chip).dac_volume[0]) * 2);
    cs4398_write_cached(chip, 6, (127 - (*chip).dac_volume[1]) * 2);
    update_cs4362a_volumes(chip);
}

unsafe fn update_cs43xx_mute(chip: *mut oxygen) {
    let mut reg: u8;

    reg = CS4398_MUTEP_LOW | CS4398_PAMUTE;
    if (*chip).dac_mute {
        reg |= CS4398_MUTE_B | CS4398_MUTE_A;
    }
    cs4398_write_cached(chip, 4, reg);
    update_cs4362a_volumes(chip);
}

unsafe fn update_cs43xx_center_lfe_mix(chip: *mut oxygen, mixed: bool) {
    let data = (*chip).model_data as *mut xonar_cs43xx;
    let mut reg: u8;

    reg = (*data).cs4362a_regs[9] & !CS4362A_ATAPI_MASK;
    if mixed {
        reg |= CS4362A_ATAPI_B_LR | CS4362A_ATAPI_A_LR;
    } else {
        reg |= CS4362A_ATAPI_B_R | CS4362A_ATAPI_A_L;
    }
    cs4362a_write_cached(chip, 9, reg);
}

static front_panel_switch: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"Front Panel Playback Switch".as_ptr(),
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(xonar_gpio_bit_switch_get),
    put: Some(xonar_gpio_bit_switch_put),
    private_value: GPIO_D1_FRONT_PANEL as _,
};

unsafe fn rolloff_info(_ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> i32 {
    static names: [*const core::ffi::c_char; 2] = [
        c"Fast Roll-off".as_ptr(),
        c"Slow Roll-off".as_ptr(),
    ];

    snd_ctl_enum_info(info, 1, 2, names.as_ptr())
}

unsafe fn rolloff_get(_ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> i32 {
    let chip = (*_ctl).private_data as *mut oxygen;
    let data = (*chip).model_data as *mut xonar_cs43xx;

    (*value).value.enumerated.item[0] = (((*data).cs4398_regs[7] & CS4398_FILT_SEL) != 0) as _;
    0
}

unsafe fn rolloff_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> i32 {
    let chip = (*ctl).private_data as *mut oxygen;
    let data = (*chip).model_data as *mut xonar_cs43xx;
    let changed: i32;
    let mut reg: u8;

    /* C source used guard(mutex)(&chip->mutex). */
    let _guard = guard_mutex(&mut (*chip).mutex);
    reg = (*data).cs4398_regs[7];
    if (*value).value.enumerated.item[0] != 0 {
        reg |= CS4398_FILT_SEL;
    } else {
        reg &= !CS4398_FILT_SEL;
    }
    changed = (reg != (*data).cs4398_regs[7]) as i32;
    if changed != 0 {
        cs4398_write(chip, 7, reg);
        if (reg & CS4398_FILT_SEL) != 0 {
            reg = (*data).cs4362a_regs[0x04] | CS4362A_FILT_SEL;
        } else {
            reg = (*data).cs4362a_regs[0x04] & !CS4362A_FILT_SEL;
        }
        cs4362a_write(chip, 0x04, reg);
    }
    changed
}

static rolloff_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"DAC Filter Playback Enum".as_ptr(),
    info: Some(rolloff_info),
    get: Some(rolloff_get),
    put: Some(rolloff_put),
};

unsafe fn xonar_d1_line_mic_ac97_switch(chip: *mut oxygen, reg: u32, mute: u32) {
    if reg == AC97_LINE {
        /* C source used guard(spinlock_irq)(&chip->reg_lock). */
        let _guard = guard_spinlock_irq(&mut (*chip).reg_lock);
        oxygen_write16_masked(
            chip,
            OXYGEN_GPIO_DATA,
            if mute != 0 { GPIO_D1_INPUT_ROUTE } else { 0 },
            GPIO_D1_INPUT_ROUTE,
        );
    }
}

static cs4362a_db_scale: [u32; 4] = TLV_DB_SCALE_ITEM(-6000, 100, 0);

unsafe fn xonar_d1_mixer_init(chip: *mut oxygen) -> i32 {
    let mut err: i32;

    err = snd_ctl_add((*chip).card, snd_ctl_new1(&front_panel_switch, chip as *mut _));
    if err < 0 {
        return err;
    }
    err = snd_ctl_add((*chip).card, snd_ctl_new1(&rolloff_control, chip as *mut _));
    if err < 0 {
        return err;
    }
    0
}

unsafe fn dump_cs4362a_registers(data: *mut xonar_cs43xx, buffer: *mut snd_info_buffer) {
    let mut i: u32;

    snd_iprintf(buffer, c"\nCS4362A:".as_ptr());
    i = 1;
    while i <= 14 {
        snd_iprintf(buffer, c" %02x".as_ptr(), (*data).cs4362a_regs[i as usize] as u32);
        i += 1;
    }
    snd_iprintf(buffer, c"\n".as_ptr());
}

unsafe fn dump_d1_registers(chip: *mut oxygen, buffer: *mut snd_info_buffer) {
    let data = (*chip).model_data as *mut xonar_cs43xx;
    let mut i: u32;

    snd_iprintf(buffer, c"\nCS4398: 7?".as_ptr());
    i = 2;
    while i < 8 {
        snd_iprintf(buffer, c" %02x".as_ptr(), (*data).cs4398_regs[i as usize] as u32);
        i += 1;
    }
    snd_iprintf(buffer, c"\n".as_ptr());
    dump_cs4362a_registers(data, buffer);
}

static model_xonar_d1: oxygen_model = oxygen_model {
    longname: c"Asus Virtuoso 100".as_ptr(),
    chip: c"AV200".as_ptr(),
    init: Some(xonar_d1_init),
    mixer_init: Some(xonar_d1_mixer_init),
    cleanup: Some(xonar_d1_cleanup),
    suspend: Some(xonar_d1_suspend),
    resume: Some(xonar_d1_resume),
    set_dac_params: Some(set_cs43xx_params),
    set_adc_params: Some(xonar_set_cs53x1_params),
    update_dac_volume: Some(update_cs43xx_volume),
    update_dac_mute: Some(update_cs43xx_mute),
    update_center_lfe_mix: Some(update_cs43xx_center_lfe_mix),
    ac97_switch: Some(xonar_d1_line_mic_ac97_switch),
    dump_registers: Some(dump_d1_registers),
    dac_tlv: cs4362a_db_scale.as_ptr(),
    model_data_size: core::mem::size_of::<xonar_cs43xx>(),
    device_config: PLAYBACK_0_TO_I2S
        | PLAYBACK_1_TO_SPDIF
        | CAPTURE_0_FROM_I2S_2
        | CAPTURE_1_FROM_SPDIF
        | AC97_FMIC_SWITCH,
    dac_channels_pcm: 8,
    dac_channels_mixer: 8,
    dac_volume_min: 127 - 60,
    dac_volume_max: 127,
    function_flags: OXYGEN_FUNCTION_2WIRE,
    dac_mclks: OXYGEN_MCLKS(256, 128, 128),
    adc_mclks: OXYGEN_MCLKS(256, 128, 128),
    dac_i2s_format: OXYGEN_I2S_FORMAT_LJUST,
    adc_i2s_format: OXYGEN_I2S_FORMAT_LJUST,
};

pub unsafe fn get_xonar_cs43xx_model(chip: *mut oxygen, id: *const pci_device_id) -> i32 {
    match (*id).subdevice {
        0x834f => {
            (*chip).model = model_xonar_d1;
            (*chip).model.shortname = c"Xonar D1".as_ptr();
        }
        0x8275 | 0x8327 => {
            (*chip).model = model_xonar_d1;
            (*chip).model.shortname = c"Xonar DX".as_ptr();
            (*chip).model.init = Some(xonar_dx_init);
        }
        _ => {
            return -EINVAL;
        }
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
