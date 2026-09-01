// SPDX-License-Identifier: GPL-2.0-only
/*
 * card driver for the Xonar DG/DGX
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 * Copyright (c) Roman Volkov <v1ron@mail.ru>
 */

/*
 * Xonar DG/DGX
 * ------------
 *
 * CS4245 and CS4361 both will mute all outputs if any clock ratio
 * is invalid.
 *
 * CMI8788:
 *
 *   SPI 0 -> CS4245
 *
 *   Playback:
 *   I2S 1 -> CS4245
 *   I2S 2 -> CS4361 (center/LFE)
 *   I2S 3 -> CS4361 (surround)
 *   I2S 4 -> CS4361 (front)
 *   Capture:
 *   I2S ADC 1 <- CS4245
 *
 *   GPIO 3 <- ?
 *   GPIO 4 <- headphone detect
 *   GPIO 5 -> enable ADC analog circuit for the left channel
 *   GPIO 6 -> enable ADC analog circuit for the right channel
 *   GPIO 7 -> switch green rear output jack between CS4245 and the first
 *             channel of CS4361 (mechanical relay)
 *   GPIO 8 -> enable output to speakers
 *
 * CS4245:
 *
 *   input 0 <- mic
 *   input 1 <- aux
 *   input 2 <- front mic
 *   input 4 <- line
 *   DAC out -> headphones
 *   aux out -> front panel headphones
 */

use core::ffi::{c_char, c_int, c_uint, c_uchar, c_void};

#[repr(C)]
pub struct oxygen {
    pub model_data: *mut c_void,
    pub card: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dg {
    pub cs4245_shadow: [c_uchar; DG_CS4245_SHADOW_SIZE],
    pub output_sel: c_uint,
    pub input_sel: c_uint,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum cs4245_shadow_operation {
    CS4245_SAVE_TO_SHADOW,
    CS4245_LOAD_FROM_SHADOW,
}

unsafe extern "C" {
    fn oxygen_write_spi(chip: *mut oxygen, control: c_uint, data: c_uint) -> c_int;
    fn oxygen_read8(chip: *mut oxygen, reg: c_uint) -> c_uchar;
    fn oxygen_write16(chip: *mut oxygen, reg: c_uint, value: c_uint);
    fn oxygen_clear_bits16(chip: *mut oxygen, reg: c_uint, value: c_uint);
    fn oxygen_set_bits16(chip: *mut oxygen, reg: c_uint, value: c_uint);
    fn oxygen_write8_masked(chip: *mut oxygen, reg: c_uint, value: c_uint, mask: c_uint);
    fn snd_component_add(card: *mut c_void, component: *const c_char) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn msleep(msecs: c_uint);
}

pub unsafe extern "C" fn cs4245_write_spi(chip: *mut oxygen, reg: c_uchar) -> c_int {
    let data = unsafe { (*chip).model_data as *mut dg };
    let mut packet: c_uint;

    packet = (reg as c_uint) << 8;
    packet |= (CS4245_SPI_ADDRESS | CS4245_SPI_WRITE) << 16;
    packet |= unsafe { (*data).cs4245_shadow[reg as usize] as c_uint };

    unsafe {
        oxygen_write_spi(
            chip,
            OXYGEN_SPI_TRIGGER
                | OXYGEN_SPI_DATA_LENGTH_3
                | OXYGEN_SPI_CLOCK_1280
                | (0 << OXYGEN_SPI_CODEC_SHIFT)
                | OXYGEN_SPI_CEN_LATCH_CLOCK_HI,
            packet,
        )
    }
}

pub unsafe extern "C" fn cs4245_read_spi(chip: *mut oxygen, addr: c_uchar) -> c_int {
    let data = unsafe { (*chip).model_data as *mut dg };
    let mut ret: c_int;

    ret = unsafe {
        oxygen_write_spi(
            chip,
            OXYGEN_SPI_TRIGGER
                | OXYGEN_SPI_DATA_LENGTH_2
                | OXYGEN_SPI_CEN_LATCH_CLOCK_HI
                | OXYGEN_SPI_CLOCK_1280
                | (0 << OXYGEN_SPI_CODEC_SHIFT),
            ((CS4245_SPI_ADDRESS | CS4245_SPI_WRITE) << 8) | addr as c_uint,
        )
    };
    if ret < 0 {
        return ret;
    }

    ret = unsafe {
        oxygen_write_spi(
            chip,
            OXYGEN_SPI_TRIGGER
                | OXYGEN_SPI_DATA_LENGTH_2
                | OXYGEN_SPI_CEN_LATCH_CLOCK_HI
                | OXYGEN_SPI_CLOCK_1280
                | (0 << OXYGEN_SPI_CODEC_SHIFT),
            (CS4245_SPI_ADDRESS | CS4245_SPI_READ) << 8,
        )
    };
    if ret < 0 {
        return ret;
    }

    unsafe {
        (*data).cs4245_shadow[addr as usize] = oxygen_read8(chip, OXYGEN_SPI_DATA1);
    }

    0
}

pub unsafe extern "C" fn cs4245_shadow_control(
    chip: *mut oxygen,
    op: cs4245_shadow_operation,
) -> c_int {
    let data = unsafe { (*chip).model_data as *mut dg };
    let mut addr: c_uchar;
    let mut ret: c_int;

    addr = 1;
    while (addr as usize) < unsafe { (*data).cs4245_shadow.len() } {
        ret = if op == cs4245_shadow_operation::CS4245_SAVE_TO_SHADOW {
            unsafe { cs4245_read_spi(chip, addr) }
        } else {
            unsafe { cs4245_write_spi(chip, addr) }
        };
        if ret < 0 {
            return ret;
        }
        addr = addr.wrapping_add(1);
    }
    0
}

unsafe fn cs4245_init(chip: *mut oxygen) {
    let data = unsafe { (*chip).model_data as *mut dg };

    /* save the initial state: codec version, registers */
    unsafe {
        cs4245_shadow_control(chip, cs4245_shadow_operation::CS4245_SAVE_TO_SHADOW);
    }

    /*
     * Power up the CODEC internals, enable soft ramp & zero cross, work in
     * async. mode, enable aux output from DAC. Invert DAC output as in the
     * Windows driver.
     */
    unsafe {
        (*data).cs4245_shadow[CS4245_POWER_CTRL as usize] = 0;
        (*data).cs4245_shadow[CS4245_SIGNAL_SEL as usize] = (CS4245_A_OUT_SEL_DAC | CS4245_ASYNCH) as c_uchar;
        (*data).cs4245_shadow[CS4245_DAC_CTRL_1 as usize] =
            (CS4245_DAC_FM_SINGLE | CS4245_DAC_DIF_LJUST) as c_uchar;
        (*data).cs4245_shadow[CS4245_DAC_CTRL_2 as usize] =
            (CS4245_DAC_SOFT | CS4245_DAC_ZERO | CS4245_INVERT_DAC) as c_uchar;
        (*data).cs4245_shadow[CS4245_ADC_CTRL as usize] =
            (CS4245_ADC_FM_SINGLE | CS4245_ADC_DIF_LJUST) as c_uchar;
        (*data).cs4245_shadow[CS4245_ANALOG_IN as usize] = (CS4245_PGA_SOFT | CS4245_PGA_ZERO) as c_uchar;
        (*data).cs4245_shadow[CS4245_PGA_B_CTRL as usize] = 0;
        (*data).cs4245_shadow[CS4245_PGA_A_CTRL as usize] = 0;
        (*data).cs4245_shadow[CS4245_DAC_A_CTRL as usize] = 8;
        (*data).cs4245_shadow[CS4245_DAC_B_CTRL as usize] = 8;

        cs4245_shadow_control(chip, cs4245_shadow_operation::CS4245_LOAD_FROM_SHADOW);
        snd_component_add((*chip).card, c"CS4245".as_ptr());
    }
}

pub unsafe extern "C" fn dg_init(chip: *mut oxygen) {
    let data = unsafe { (*chip).model_data as *mut dg };

    unsafe {
        (*data).output_sel = PLAYBACK_DST_HP_FP;
        (*data).input_sel = CAPTURE_SRC_MIC;

        cs4245_init(chip);
        oxygen_write16(
            chip,
            OXYGEN_GPIO_CONTROL,
            GPIO_OUTPUT_ENABLE | GPIO_HP_REAR | GPIO_INPUT_ROUTE,
        );
        /* anti-pop delay, wait some time before enabling the output */
        msleep(2500);
        oxygen_write16(
            chip,
            OXYGEN_GPIO_DATA,
            GPIO_OUTPUT_ENABLE | GPIO_INPUT_ROUTE,
        );
    }
}

pub unsafe extern "C" fn dg_cleanup(chip: *mut oxygen) {
    unsafe {
        oxygen_clear_bits16(chip, OXYGEN_GPIO_DATA, GPIO_OUTPUT_ENABLE);
    }
}

pub unsafe extern "C" fn dg_suspend(chip: *mut oxygen) {
    unsafe {
        dg_cleanup(chip);
    }
}

pub unsafe extern "C" fn dg_resume(chip: *mut oxygen) {
    unsafe {
        cs4245_shadow_control(chip, cs4245_shadow_operation::CS4245_LOAD_FROM_SHADOW);
        msleep(2500);
        oxygen_set_bits16(chip, OXYGEN_GPIO_DATA, GPIO_OUTPUT_ENABLE);
    }
}

pub unsafe extern "C" fn set_cs4245_dac_params(
    chip: *mut oxygen,
    params: *mut snd_pcm_hw_params,
) {
    let data = unsafe { (*chip).model_data as *mut dg };
    let mut dac_ctrl: c_uchar;
    let mut mclk_freq: c_uchar;

    unsafe {
        dac_ctrl = (*data).cs4245_shadow[CS4245_DAC_CTRL_1 as usize] & !(CS4245_DAC_FM_MASK as c_uchar);
        mclk_freq = (*data).cs4245_shadow[CS4245_MCLK_FREQ as usize] & !(CS4245_MCLK1_MASK as c_uchar);
        if params_rate(params) <= 50000 {
            dac_ctrl |= CS4245_DAC_FM_SINGLE as c_uchar;
            mclk_freq |= (CS4245_MCLK_1 << CS4245_MCLK1_SHIFT) as c_uchar;
        } else if params_rate(params) <= 100000 {
            dac_ctrl |= CS4245_DAC_FM_DOUBLE as c_uchar;
            mclk_freq |= (CS4245_MCLK_1 << CS4245_MCLK1_SHIFT) as c_uchar;
        } else {
            dac_ctrl |= CS4245_DAC_FM_QUAD as c_uchar;
            mclk_freq |= (CS4245_MCLK_2 << CS4245_MCLK1_SHIFT) as c_uchar;
        }
        (*data).cs4245_shadow[CS4245_DAC_CTRL_1 as usize] = dac_ctrl;
        (*data).cs4245_shadow[CS4245_MCLK_FREQ as usize] = mclk_freq;
        cs4245_write_spi(chip, CS4245_DAC_CTRL_1 as c_uchar);
        cs4245_write_spi(chip, CS4245_MCLK_FREQ as c_uchar);
    }
}

pub unsafe extern "C" fn set_cs4245_adc_params(
    chip: *mut oxygen,
    params: *mut snd_pcm_hw_params,
) {
    let data = unsafe { (*chip).model_data as *mut dg };
    let mut adc_ctrl: c_uchar;
    let mut mclk_freq: c_uchar;

    unsafe {
        adc_ctrl = (*data).cs4245_shadow[CS4245_ADC_CTRL as usize] & !(CS4245_ADC_FM_MASK as c_uchar);
        mclk_freq = (*data).cs4245_shadow[CS4245_MCLK_FREQ as usize] & !(CS4245_MCLK2_MASK as c_uchar);
        if params_rate(params) <= 50000 {
            adc_ctrl |= CS4245_ADC_FM_SINGLE as c_uchar;
            mclk_freq |= (CS4245_MCLK_1 << CS4245_MCLK2_SHIFT) as c_uchar;
        } else if params_rate(params) <= 100000 {
            adc_ctrl |= CS4245_ADC_FM_DOUBLE as c_uchar;
            mclk_freq |= (CS4245_MCLK_1 << CS4245_MCLK2_SHIFT) as c_uchar;
        } else {
            adc_ctrl |= CS4245_ADC_FM_QUAD as c_uchar;
            mclk_freq |= (CS4245_MCLK_2 << CS4245_MCLK2_SHIFT) as c_uchar;
        }
        (*data).cs4245_shadow[CS4245_ADC_CTRL as usize] = adc_ctrl;
        (*data).cs4245_shadow[CS4245_MCLK_FREQ as usize] = mclk_freq;
        cs4245_write_spi(chip, CS4245_ADC_CTRL as c_uchar);
        cs4245_write_spi(chip, CS4245_MCLK_FREQ as c_uchar);
    }
}

#[inline]
unsafe fn shift_bits(
    value: c_uint,
    shift_from: c_uint,
    shift_to: c_uint,
    mask: c_uint,
) -> c_uint {
    if shift_from < shift_to {
        (value << (shift_to - shift_from)) & mask
    } else {
        (value >> (shift_from - shift_to)) & mask
    }
}

pub unsafe extern "C" fn adjust_dg_dac_routing(
    chip: *mut oxygen,
    play_routing: c_uint,
) -> c_uint {
    let data = unsafe { (*chip).model_data as *mut dg };

    unsafe {
        if (*data).output_sel == PLAYBACK_DST_HP || (*data).output_sel == PLAYBACK_DST_HP_FP {
            oxygen_write8_masked(
                chip,
                OXYGEN_PLAY_ROUTING,
                OXYGEN_PLAY_MUTE23 | OXYGEN_PLAY_MUTE45 | OXYGEN_PLAY_MUTE67,
                OXYGEN_PLAY_MUTE_MASK,
            );
        } else if (*data).output_sel == PLAYBACK_DST_MULTICH {
            oxygen_write8_masked(
                chip,
                OXYGEN_PLAY_ROUTING,
                OXYGEN_PLAY_MUTE01,
                OXYGEN_PLAY_MUTE_MASK,
            );
        }
    }
    (play_routing & OXYGEN_PLAY_DAC0_SOURCE_MASK)
        | unsafe {
            shift_bits(
                play_routing,
                OXYGEN_PLAY_DAC2_SOURCE_SHIFT,
                OXYGEN_PLAY_DAC1_SOURCE_SHIFT,
                OXYGEN_PLAY_DAC1_SOURCE_MASK,
            )
        }
        | unsafe {
            shift_bits(
                play_routing,
                OXYGEN_PLAY_DAC1_SOURCE_SHIFT,
                OXYGEN_PLAY_DAC2_SOURCE_SHIFT,
                OXYGEN_PLAY_DAC2_SOURCE_MASK,
            )
        }
        | unsafe {
            shift_bits(
                play_routing,
                OXYGEN_PLAY_DAC0_SOURCE_SHIFT,
                OXYGEN_PLAY_DAC3_SOURCE_SHIFT,
                OXYGEN_PLAY_DAC3_SOURCE_MASK,
            )
        }
}

pub unsafe extern "C" fn dump_cs4245_registers(
    chip: *mut oxygen,
    buffer: *mut snd_info_buffer,
) {
    let data = unsafe { (*chip).model_data as *mut dg };
    let mut addr: c_uint;

    unsafe {
        snd_iprintf(buffer, c"\nCS4245:".as_ptr());
        cs4245_read_spi(chip, CS4245_INT_STATUS as c_uchar);
        addr = 1;
        while (addr as usize) < (*data).cs4245_shadow.len() {
            snd_iprintf(
                buffer,
                c" %02x".as_ptr(),
                (*data).cs4245_shadow[addr as usize] as c_uint,
            );
            addr += 1;
        }
        snd_iprintf(buffer, c"\n".as_ptr());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
