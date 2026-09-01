/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * da7219-aad.h - DA7322 ASoC AAD Driver
 *
 * Copyright (c) 2015 Dialog Semiconductor Ltd.
 *
 * Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
 */

use core::ffi::c_int;

/*
 * Registers
 */

pub const DA7219_ACCDET_STATUS_A: u32 = 0xC0;
pub const DA7219_ACCDET_STATUS_B: u32 = 0xC1;
pub const DA7219_ACCDET_IRQ_EVENT_A: u32 = 0xC2;
pub const DA7219_ACCDET_IRQ_EVENT_B: u32 = 0xC3;
pub const DA7219_ACCDET_IRQ_MASK_A: u32 = 0xC4;
pub const DA7219_ACCDET_IRQ_MASK_B: u32 = 0xC5;
pub const DA7219_ACCDET_CONFIG_1: u32 = 0xC6;
pub const DA7219_ACCDET_CONFIG_2: u32 = 0xC7;
pub const DA7219_ACCDET_CONFIG_3: u32 = 0xC8;
pub const DA7219_ACCDET_CONFIG_4: u32 = 0xC9;
pub const DA7219_ACCDET_CONFIG_5: u32 = 0xCA;
pub const DA7219_ACCDET_CONFIG_6: u32 = 0xCB;
pub const DA7219_ACCDET_CONFIG_7: u32 = 0xCC;
pub const DA7219_ACCDET_CONFIG_8: u32 = 0xCD;

/*
 * Bit Fields
 */

/* DA7219_ACCDET_STATUS_A = 0xC0 */
pub const DA7219_JACK_INSERTION_STS_SHIFT: u32 = 0;
pub const DA7219_JACK_INSERTION_STS_MASK: u32 = 0x1 << 0;
pub const DA7219_JACK_TYPE_STS_SHIFT: u32 = 1;
pub const DA7219_JACK_TYPE_STS_MASK: u32 = 0x1 << 1;
pub const DA7219_JACK_PIN_ORDER_STS_SHIFT: u32 = 2;
pub const DA7219_JACK_PIN_ORDER_STS_MASK: u32 = 0x1 << 2;
pub const DA7219_MICBIAS_UP_STS_SHIFT: u32 = 3;
pub const DA7219_MICBIAS_UP_STS_MASK: u32 = 0x1 << 3;

/* DA7219_ACCDET_STATUS_B = 0xC1 */
pub const DA7219_BUTTON_TYPE_STS_SHIFT: u32 = 0;
pub const DA7219_BUTTON_TYPE_STS_MASK: u32 = 0xFF << 0;

/* DA7219_ACCDET_IRQ_EVENT_A = 0xC2 */
pub const DA7219_E_JACK_INSERTED_SHIFT: u32 = 0;
pub const DA7219_E_JACK_INSERTED_MASK: u32 = 0x1 << 0;
pub const DA7219_E_JACK_REMOVED_SHIFT: u32 = 1;
pub const DA7219_E_JACK_REMOVED_MASK: u32 = 0x1 << 1;
pub const DA7219_E_JACK_DETECT_COMPLETE_SHIFT: u32 = 2;
pub const DA7219_E_JACK_DETECT_COMPLETE_MASK: u32 = 0x1 << 2;

/* DA7219_ACCDET_IRQ_EVENT_B = 0xC3 */
pub const DA7219_E_BUTTON_A_PRESSED_SHIFT: u32 = 0;
pub const DA7219_E_BUTTON_A_PRESSED_MASK: u32 = 0x1 << 0;
pub const DA7219_E_BUTTON_B_PRESSED_SHIFT: u32 = 1;
pub const DA7219_E_BUTTON_B_PRESSED_MASK: u32 = 0x1 << 1;
pub const DA7219_E_BUTTON_C_PRESSED_SHIFT: u32 = 2;
pub const DA7219_E_BUTTON_C_PRESSED_MASK: u32 = 0x1 << 2;
pub const DA7219_E_BUTTON_D_PRESSED_SHIFT: u32 = 3;
pub const DA7219_E_BUTTON_D_PRESSED_MASK: u32 = 0x1 << 3;
pub const DA7219_E_BUTTON_D_RELEASED_SHIFT: u32 = 4;
pub const DA7219_E_BUTTON_D_RELEASED_MASK: u32 = 0x1 << 4;
pub const DA7219_E_BUTTON_C_RELEASED_SHIFT: u32 = 5;
pub const DA7219_E_BUTTON_C_RELEASED_MASK: u32 = 0x1 << 5;
pub const DA7219_E_BUTTON_B_RELEASED_SHIFT: u32 = 6;
pub const DA7219_E_BUTTON_B_RELEASED_MASK: u32 = 0x1 << 6;
pub const DA7219_E_BUTTON_A_RELEASED_SHIFT: u32 = 7;
pub const DA7219_E_BUTTON_A_RELEASED_MASK: u32 = 0x1 << 7;

/* DA7219_ACCDET_IRQ_MASK_A = 0xC4 */
pub const DA7219_M_JACK_INSERTED_SHIFT: u32 = 0;
pub const DA7219_M_JACK_INSERTED_MASK: u32 = 0x1 << 0;
pub const DA7219_M_JACK_REMOVED_SHIFT: u32 = 1;
pub const DA7219_M_JACK_REMOVED_MASK: u32 = 0x1 << 1;
pub const DA7219_M_JACK_DETECT_COMPLETE_SHIFT: u32 = 2;
pub const DA7219_M_JACK_DETECT_COMPLETE_MASK: u32 = 0x1 << 2;

/* DA7219_ACCDET_IRQ_MASK_B = 0xC5 */
pub const DA7219_M_BUTTON_A_PRESSED_SHIFT: u32 = 0;
pub const DA7219_M_BUTTON_A_PRESSED_MASK: u32 = 0x1 << 0;
pub const DA7219_M_BUTTON_B_PRESSED_SHIFT: u32 = 1;
pub const DA7219_M_BUTTON_B_PRESSED_MASK: u32 = 0x1 << 1;
pub const DA7219_M_BUTTON_C_PRESSED_SHIFT: u32 = 2;
pub const DA7219_M_BUTTON_C_PRESSED_MASK: u32 = 0x1 << 2;
pub const DA7219_M_BUTTON_D_PRESSED_SHIFT: u32 = 3;
pub const DA7219_M_BUTTON_D_PRESSED_MASK: u32 = 0x1 << 3;
pub const DA7219_M_BUTTON_D_RELEASED_SHIFT: u32 = 4;
pub const DA7219_M_BUTTON_D_RELEASED_MASK: u32 = 0x1 << 4;
pub const DA7219_M_BUTTON_C_RELEASED_SHIFT: u32 = 5;
pub const DA7219_M_BUTTON_C_RELEASED_MASK: u32 = 0x1 << 5;
pub const DA7219_M_BUTTON_B_RELEASED_SHIFT: u32 = 6;
pub const DA7219_M_BUTTON_B_RELEASED_MASK: u32 = 0x1 << 6;
pub const DA7219_M_BUTTON_A_RELEASED_SHIFT: u32 = 7;
pub const DA7219_M_BUTTON_A_RELEASED_MASK: u32 = 0x1 << 7;

/* DA7219_ACCDET_CONFIG_1 = 0xC6 */
pub const DA7219_ACCDET_EN_SHIFT: u32 = 0;
pub const DA7219_ACCDET_EN_MASK: u32 = 0x1 << 0;
pub const DA7219_BUTTON_CONFIG_SHIFT: u32 = 1;
pub const DA7219_BUTTON_CONFIG_MASK: u32 = 0x7 << 1;
pub const DA7219_MIC_DET_THRESH_SHIFT: u32 = 4;
pub const DA7219_MIC_DET_THRESH_MASK: u32 = 0x3 << 4;
pub const DA7219_JACK_TYPE_DET_EN_SHIFT: u32 = 6;
pub const DA7219_JACK_TYPE_DET_EN_MASK: u32 = 0x1 << 6;
pub const DA7219_PIN_ORDER_DET_EN_SHIFT: u32 = 7;
pub const DA7219_PIN_ORDER_DET_EN_MASK: u32 = 0x1 << 7;

/* DA7219_ACCDET_CONFIG_2 = 0xC7 */
pub const DA7219_ACCDET_PAUSE_SHIFT: u32 = 0;
pub const DA7219_ACCDET_PAUSE_MASK: u32 = 0x1 << 0;
pub const DA7219_JACKDET_DEBOUNCE_SHIFT: u32 = 1;
pub const DA7219_JACKDET_DEBOUNCE_MASK: u32 = 0x7 << 1;
pub const DA7219_JACK_DETECT_RATE_SHIFT: u32 = 4;
pub const DA7219_JACK_DETECT_RATE_MASK: u32 = 0x3 << 4;
pub const DA7219_JACKDET_REM_DEB_SHIFT: u32 = 6;
pub const DA7219_JACKDET_REM_DEB_MASK: u32 = 0x3 << 6;

/* DA7219_ACCDET_CONFIG_3 = 0xC8 */
pub const DA7219_A_D_BUTTON_THRESH_SHIFT: u32 = 0;
pub const DA7219_A_D_BUTTON_THRESH_MASK: u32 = 0xFF << 0;

/* DA7219_ACCDET_CONFIG_4 = 0xC9 */
pub const DA7219_D_B_BUTTON_THRESH_SHIFT: u32 = 0;
pub const DA7219_D_B_BUTTON_THRESH_MASK: u32 = 0xFF << 0;

/* DA7219_ACCDET_CONFIG_5 = 0xCA */
pub const DA7219_B_C_BUTTON_THRESH_SHIFT: u32 = 0;
pub const DA7219_B_C_BUTTON_THRESH_MASK: u32 = 0xFF << 0;

/* DA7219_ACCDET_CONFIG_6 = 0xCB */
pub const DA7219_C_MIC_BUTTON_THRESH_SHIFT: u32 = 0;
pub const DA7219_C_MIC_BUTTON_THRESH_MASK: u32 = 0xFF << 0;

/* DA7219_ACCDET_CONFIG_7 = 0xCC */
pub const DA7219_BUTTON_AVERAGE_SHIFT: u32 = 0;
pub const DA7219_BUTTON_AVERAGE_MASK: u32 = 0x3 << 0;
pub const DA7219_ADC_1_BIT_REPEAT_SHIFT: u32 = 2;
pub const DA7219_ADC_1_BIT_REPEAT_MASK: u32 = 0x3 << 2;
pub const DA7219_PIN_ORDER_FORCE_SHIFT: u32 = 4;
pub const DA7219_PIN_ORDER_FORCE_MASK: u32 = 0x1 << 4;
pub const DA7219_JACK_TYPE_FORCE_SHIFT: u32 = 5;
pub const DA7219_JACK_TYPE_FORCE_MASK: u32 = 0x1 << 5;

/* DA7219_ACCDET_CONFIG_8 = 0xCD */
pub const DA7219_HPTEST_EN_SHIFT: u32 = 0;
pub const DA7219_HPTEST_EN_MASK: u32 = 0x1 << 0;
pub const DA7219_HPTEST_RES_SEL_SHIFT: u32 = 1;
pub const DA7219_HPTEST_RES_SEL_MASK: u32 = 0x3 << 1;
pub const DA7219_HPTEST_RES_SEL_1KOHMS: u32 = 0x0 << 1;
pub const DA7219_HPTEST_COMP_SHIFT: u32 = 4;
pub const DA7219_HPTEST_COMP_MASK: u32 = 0x1 << 4;

pub const DA7219_AAD_MAX_BUTTONS: u32 = 4;
pub const DA7219_AAD_REPORT_ALL_MASK: u32 = SND_JACK_MECHANICAL
    | SND_JACK_HEADSET
    | SND_JACK_LINEOUT
    | SND_JACK_BTN_0
    | SND_JACK_BTN_1
    | SND_JACK_BTN_2
    | SND_JACK_BTN_3;

pub const DA7219_AAD_MICBIAS_CHK_DELAY: u32 = 10;
pub const DA7219_AAD_MICBIAS_CHK_RETRIES: u32 = 5;

pub const DA7219_AAD_HPTEST_RAMP_FREQ: u32 = 0x28;
pub const DA7219_AAD_HPTEST_RAMP_FREQ_INT_OSC: u32 = 0x4D;
pub const DA7219_AAD_HPTEST_PERIOD: u32 = 65;
pub const DA7219_AAD_HPTEST_INT_OSC_PATH_DELAY: u32 = 20;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum da7219_aad_event_regs {
    DA7219_AAD_IRQ_REG_A = 0,
    DA7219_AAD_IRQ_REG_B = 1,
    DA7219_AAD_IRQ_REG_MAX = 2,
}

/* Private data */
#[repr(C)]
pub struct da7219_aad_priv {
    pub component: *mut snd_soc_component,
    pub irq: c_int,
    pub gnd_switch_delay: c_int,

    pub micbias_pulse_lvl: u8,
    pub micbias_pulse_time: u32,

    pub btn_cfg: u8,

    pub btn_det_work: work_struct,
    pub hptest_work: work_struct,
    pub jack_det_work: delayed_work,
    pub aad_wq: *mut workqueue_struct,

    pub jack: *mut snd_soc_jack,
    pub micbias_resume_enable: bool,
    pub jack_inserted: bool,
}

unsafe extern "C" {
    /* AAD control */
    pub fn da7219_aad_jack_det(component: *mut snd_soc_component, jack: *mut snd_soc_jack);

    /*
     * Suspend/Resume
     *
     * In C, these are declarations when CONFIG_PM is enabled. When CONFIG_PM is
     * disabled, the header provides static inline no-op functions.
     */
    pub fn da7219_aad_suspend(component: *mut snd_soc_component);
    pub fn da7219_aad_resume(component: *mut snd_soc_component);

    /* Init/Exit */
    pub fn da7219_aad_init(component: *mut snd_soc_component) -> c_int;
    pub fn da7219_aad_exit(component: *mut snd_soc_component);

    /* I2C Probe */
    pub fn da7219_aad_probe(i2c: *mut i2c_client) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
