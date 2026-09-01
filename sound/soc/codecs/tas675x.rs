// SPDX-License-Identifier: GPL-2.0
/*
 * ALSA SoC Texas Instruments TAS67524 Quad-Channel Audio Amplifier
 *
 * Copyright (C) 2026 Texas Instruments Incorporated - https://www.ti.com/
 *	Author: Sen Wang <sen@ti.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

// Dependencies from Linux, ALSA SoC, and "tas675x.h" are intentionally external.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type u8 = u8;
type u32 = u32;
type bool_ = bool;
type irqreturn_t = c_uint;

const TAS675X_FAULT_CHECK_INTERVAL_MS: c_uint = 200;
const TAS675X_DSP_PARAM_NUM: usize = 2;
const TAS675X_FAULT_REGS_NUM: usize = 9;

const TAS675X_FAULT_CRITICAL: c_uint = BIT(0);
const TAS675X_FAULT_TRACK: c_uint = BIT(1);
const TAS675X_FAULT_ACTIVE: c_uint = BIT(2);

#[repr(C)]
enum tas675x_type {
    TAS67524,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct tas675x_reg_param {
    page: u8,
    reg: u8,
    val: u32,
}

#[repr(C)]
struct tas675x_priv {
    dev: *mut device,
    regmap: *mut regmap,
    dev_type: tas675x_type,
    /* Custom regmap lock; protects writes across books */
    io_lock: mutex,

    pd_gpio: *mut gpio_desc,
    stby_gpio: *mut gpio_desc,
    supplies: [regulator_bulk_data; 2],
    vbat: *mut regulator,
    fast_boot: bool_,

    audio_slot: c_int,
    llp_slot: c_int,
    vpredict_slot: c_int,
    isense_slot: c_int,
    bclk_offset: c_int,
    slot_width: c_int,
    tx_mask: c_uint,

    gpio1_func: c_int,
    gpio2_func: c_int,

    active_playback_dais: c_ulong,
    active_capture_dais: c_ulong,
    rate: c_uint,
    saved_rtldg_en: c_uint,
    dsp_params: [tas675x_reg_param; TAS675X_DSP_PARAM_NUM],

    /* Fault monitor, disabled when Fault IRQ is used */
    fault_check_work: delayed_work,
    last_status: [c_uint; TAS675X_FAULT_REGS_NUM],
}

#[repr(C)]
struct tas675x_gpio_func {
    name: *const c_char,
    val: c_int,
}

#[repr(C)]
struct tas675x_gpio_input {
    reg: c_uint,
    mask: c_uint,
}

#[repr(C)]
struct tas675x_fault_reg {
    reg: c_uint,
    flags: c_uint,
    name: *const c_char,
}

static tas675x_supply_names: [*const c_char; 2] = [
    c"dvdd".as_ptr(), /* Digital power supply */
    c"pvdd".as_ptr(), /* Output powerstage supply */
];

/* Page 1 setup initialization defaults */
static tas675x_page1_init: [reg_sequence; 13] = [
    REG_SEQ0(TAS675X_PAGE_REG(1, 0xC8), 0x20), /* Charge pump clock */
    REG_SEQ0(TAS675X_PAGE_REG(1, 0x2F), 0x90), /* VBAT idle */
    REG_SEQ0(TAS675X_PAGE_REG(1, 0x29), 0x40), /* OC/CBC threshold */
    REG_SEQ0(TAS675X_PAGE_REG(1, 0x2E), 0x0C), /* OC/CBC config */
    REG_SEQ0(TAS675X_PAGE_REG(1, 0xC5), 0x02), /* OC/CBC config */
    REG_SEQ0(TAS675X_PAGE_REG(1, 0xC6), 0x10), /* OC/CBC config */
    REG_SEQ0(TAS675X_PAGE_REG(1, 0x1F), 0x20), /* OC/CBC config */
    REG_SEQ0(TAS675X_PAGE_REG(1, 0x16), 0x01), /* OC/CBC config */
    REG_SEQ0(TAS675X_PAGE_REG(1, 0x1E), 0x04), /* OC/CBC config */
    REG_SEQ0(TAS675X_PAGE_REG(1, 0xC1), 0x00), /* CH1 DC fault */
    REG_SEQ0(TAS675X_PAGE_REG(1, 0xC2), 0x04), /* CH2 DC fault */
    REG_SEQ0(TAS675X_PAGE_REG(1, 0xC3), 0x00), /* CH3 DC fault */
    REG_SEQ0(TAS675X_PAGE_REG(1, 0xC4), 0x00), /* CH4 DC fault */
];

unsafe fn tas675x_state_name(state: c_uint) -> *const c_char {
    match state & 0x0F {
        TAS675X_STATE_DEEPSLEEP => c"DEEPSLEEP".as_ptr(),
        TAS675X_STATE_LOAD_DIAG => c"LOAD_DIAG".as_ptr(),
        TAS675X_STATE_SLEEP => c"SLEEP".as_ptr(),
        TAS675X_STATE_HIZ => c"HIZ".as_ptr(),
        TAS675X_STATE_PLAY => c"PLAY".as_ptr(),
        TAS675X_STATE_FAULT => c"FAULT".as_ptr(),
        TAS675X_STATE_AUTOREC => c"AUTOREC".as_ptr(),
        _ => c"UNKNOWN".as_ptr(),
    }
}

unsafe fn tas675x_set_state_all(tas: *mut tas675x_priv, state: u8) -> c_int {
    let seq = [
        REG_SEQ0(TAS675X_STATE_CTRL_CH1_CH2_REG, state as c_uint),
        REG_SEQ0(TAS675X_STATE_CTRL_CH3_CH4_REG, state as c_uint),
    ];

    regmap_multi_reg_write((*tas).regmap, seq.as_ptr(), seq.len())
}

unsafe fn tas675x_select_book(regmap: *mut regmap, book: u8) -> c_int {
    let mut ret: c_int;

    /* Reset page to 0 before switching books */
    ret = regmap_write(regmap, TAS675X_PAGE_CTRL_REG, 0x00);
    if ret == 0 {
        ret = regmap_write(regmap, TAS675X_BOOK_CTRL_REG, book as c_uint);
    }

    ret
}

/* Raw I2C version of tas675x_select_book, must be called with io_lock held */
unsafe fn __tas675x_select_book(tas: *mut tas675x_priv, book: u8) -> c_int {
    let client = to_i2c_client((*tas).dev);
    let ret: c_int;

    /* Reset page to 0 before switching books */
    ret = i2c_smbus_write_byte_data(client, TAS675X_PAGE_CTRL_REG as u8, 0x00);
    if ret != 0 {
        return ret;
    }

    i2c_smbus_write_byte_data(client, TAS675X_BOOK_CTRL_REG as u8, book)
}

unsafe fn tas675x_dsp_mem_write(tas: *mut tas675x_priv, page: u8, reg: u8, val: u32) -> c_int {
    let client = to_i2c_client((*tas).dev);
    let mut buf: [u8; 4] = [0; 4];
    let mut ret: c_int;

    /* DSP registers are 32 bit big-endian */
    buf[0] = ((val >> 24) & 0xFF) as u8;
    buf[1] = ((val >> 16) & 0xFF) as u8;
    buf[2] = ((val >> 8) & 0xFF) as u8;
    buf[3] = (val & 0xFF) as u8;

    /*
     * DSP regs in a different book, therefore block
     * regmap access before completion.
     */
    mutex_lock(&mut (*tas).io_lock);

    ret = __tas675x_select_book(tas, TAS675X_BOOK_DSP as u8);
    if ret == 0 {
        ret = i2c_smbus_write_byte_data(client, TAS675X_PAGE_CTRL_REG as u8, page);
        if ret == 0 {
            ret = i2c_smbus_write_i2c_block_data(client, reg, size_of::<[u8; 4]>() as u8, buf.as_ptr());
        }
    }

    __tas675x_select_book(tas, TAS675X_BOOK_DEFAULT as u8);
    mutex_unlock(&mut (*tas).io_lock);

    ret
}

unsafe fn tas675x_dsp_mem_read(tas: *mut tas675x_priv, page: u8, reg: u8, val: *mut u32) -> c_int {
    let client = to_i2c_client((*tas).dev);
    let mut buf: [u8; 4] = [0; 4];
    let mut ret: c_int;

    /*
     * DSP regs in a different book, therefore block
     * regmap access before completion.
     */
    mutex_lock(&mut (*tas).io_lock);

    ret = __tas675x_select_book(tas, TAS675X_BOOK_DSP as u8);
    if ret == 0 {
        ret = i2c_smbus_write_byte_data(client, TAS675X_PAGE_CTRL_REG as u8, page);
        if ret == 0 {
            ret = i2c_smbus_read_i2c_block_data(client, reg, size_of::<[u8; 4]>() as u8, buf.as_mut_ptr());
            if ret == size_of::<[u8; 4]>() as c_int {
                *val = ((buf[0] as u32) << 24) | ((buf[1] as u32) << 16) | ((buf[2] as u32) << 8) | buf[3] as u32;
                ret = 0;
            } else if ret >= 0 {
                ret = -EIO;
            }
        }
    }

    __tas675x_select_book(tas, TAS675X_BOOK_DEFAULT as u8);
    mutex_unlock(&mut (*tas).io_lock);

    ret
}

static tas675x_gpio_func_map: [tas675x_gpio_func; 19] = [
    /* Output functions */
    tas675x_gpio_func { name: c"low".as_ptr(), val: TAS675X_GPIO_SEL_LOW },
    tas675x_gpio_func { name: c"auto-mute".as_ptr(), val: TAS675X_GPIO_SEL_AUTO_MUTE_ALL },
    tas675x_gpio_func { name: c"auto-mute-ch4".as_ptr(), val: TAS675X_GPIO_SEL_AUTO_MUTE_CH4 },
    tas675x_gpio_func { name: c"auto-mute-ch3".as_ptr(), val: TAS675X_GPIO_SEL_AUTO_MUTE_CH3 },
    tas675x_gpio_func { name: c"auto-mute-ch2".as_ptr(), val: TAS675X_GPIO_SEL_AUTO_MUTE_CH2 },
    tas675x_gpio_func { name: c"auto-mute-ch1".as_ptr(), val: TAS675X_GPIO_SEL_AUTO_MUTE_CH1 },
    tas675x_gpio_func { name: c"sdout2".as_ptr(), val: TAS675X_GPIO_SEL_SDOUT2 },
    tas675x_gpio_func { name: c"sdout1".as_ptr(), val: TAS675X_GPIO_SEL_SDOUT1 },
    tas675x_gpio_func { name: c"warn".as_ptr(), val: TAS675X_GPIO_SEL_WARN },
    tas675x_gpio_func { name: c"fault".as_ptr(), val: TAS675X_GPIO_SEL_FAULT },
    tas675x_gpio_func { name: c"clock-sync".as_ptr(), val: TAS675X_GPIO_SEL_CLOCK_SYNC },
    tas675x_gpio_func { name: c"invalid-clock".as_ptr(), val: TAS675X_GPIO_SEL_INVALID_CLK },
    tas675x_gpio_func { name: c"high".as_ptr(), val: TAS675X_GPIO_SEL_HIGH },
    /* Input functions */
    tas675x_gpio_func { name: c"mute".as_ptr(), val: TAS675X_GPIO_IN_MUTE },
    tas675x_gpio_func { name: c"phase-sync".as_ptr(), val: TAS675X_GPIO_IN_PHASE_SYNC },
    tas675x_gpio_func { name: c"sdin2".as_ptr(), val: TAS675X_GPIO_IN_SDIN2 },
    tas675x_gpio_func { name: c"deep-sleep".as_ptr(), val: TAS675X_GPIO_IN_DEEP_SLEEP },
    tas675x_gpio_func { name: c"hiz".as_ptr(), val: TAS675X_GPIO_IN_HIZ },
    tas675x_gpio_func { name: c"play".as_ptr(), val: TAS675X_GPIO_IN_PLAY },
    tas675x_gpio_func { name: c"sleep".as_ptr(), val: TAS675X_GPIO_IN_SLEEP },
];

unsafe fn tas675x_gpio_func_parse(dev: *mut device, propname: *const c_char) -> c_int {
    let mut str_: *const c_char = ptr::null();
    let mut i: usize;
    let ret: c_int;

    ret = device_property_read_string(dev, propname, &mut str_);
    if ret != 0 {
        return -1;
    }

    i = 0;
    while i < tas675x_gpio_func_map.len() {
        if strcmp(str_, tas675x_gpio_func_map[i].name) == 0 {
            return tas675x_gpio_func_map[i].val;
        }
        i += 1;
    }

    dev_warn(dev, c"Invalid %s value '%s'\n".as_ptr(), propname, str_);
    -1
}

static tas675x_gpio_input_table: [tas675x_gpio_input; TAS675X_GPIO_IN_NUM as usize] = {
    let mut table = [tas675x_gpio_input { reg: 0, mask: 0 }; TAS675X_GPIO_IN_NUM as usize];
    table[TAS675X_GPIO_IN_ID_MUTE as usize] = tas675x_gpio_input { reg: TAS675X_GPIO_INPUT_MUTE_REG, mask: TAS675X_GPIO_IN_MUTE_MASK };
    table[TAS675X_GPIO_IN_ID_PHASE_SYNC as usize] = tas675x_gpio_input { reg: TAS675X_GPIO_INPUT_SYNC_REG, mask: TAS675X_GPIO_IN_SYNC_MASK };
    table[TAS675X_GPIO_IN_ID_SDIN2 as usize] = tas675x_gpio_input { reg: TAS675X_GPIO_INPUT_SDIN2_REG, mask: TAS675X_GPIO_IN_SDIN2_MASK };
    table[TAS675X_GPIO_IN_ID_DEEP_SLEEP as usize] = tas675x_gpio_input { reg: TAS675X_GPIO_INPUT_SLEEP_HIZ_REG, mask: TAS675X_GPIO_IN_DEEP_SLEEP_MASK };
    table[TAS675X_GPIO_IN_ID_HIZ as usize] = tas675x_gpio_input { reg: TAS675X_GPIO_INPUT_SLEEP_HIZ_REG, mask: TAS675X_GPIO_IN_HIZ_MASK };
    table[TAS675X_GPIO_IN_ID_PLAY as usize] = tas675x_gpio_input { reg: TAS675X_GPIO_INPUT_PLAY_SLEEP_REG, mask: TAS675X_GPIO_IN_PLAY_MASK };
    table[TAS675X_GPIO_IN_ID_SLEEP as usize] = tas675x_gpio_input { reg: TAS675X_GPIO_INPUT_PLAY_SLEEP_REG, mask: TAS675X_GPIO_IN_SLEEP_MASK };
    table
};

unsafe fn tas675x_config_gpio_pin(regmap: *mut regmap, func_id: c_int, out_sel_reg: c_uint, pin_idx: c_uint, gpio_ctrl: *mut c_uint) {
    let id: c_int;

    if func_id < 0 {
        return;
    }

    if (func_id & TAS675X_GPIO_FUNC_INPUT) != 0 {
        /* 3-bit mux: 0 = disabled, 0b1 = GPIO1, 0b10 = GPIO2 */
        id = func_id & !TAS675X_GPIO_FUNC_INPUT;
        regmap_update_bits(
            regmap,
            tas675x_gpio_input_table[id as usize].reg,
            tas675x_gpio_input_table[id as usize].mask,
            (pin_idx + 1) << __ffs(tas675x_gpio_input_table[id as usize].mask),
        );
    } else {
        /* Output GPIO, update selection register and enable bit */
        regmap_write(regmap, out_sel_reg, func_id as c_uint);
        *gpio_ctrl |= if pin_idx != 0 { TAS675X_GPIO2_OUTPUT_EN } else { TAS675X_GPIO1_OUTPUT_EN };
    }
}

unsafe fn tas675x_rtldg_thresh_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    /* threshold reg ranges up to 24bit */
    (*uinfo).value.integer.max = 0x00FFFFFF;
    0
}

unsafe fn tas675x_set_rtldg_thresh(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let tas = snd_soc_component_get_drvdata(comp) as *mut tas675x_priv;
    let t = (*kcontrol).private_value as *const tas675x_reg_param;
    let val = (*ucontrol).value.integer.value[0] as u32;
    let ret: c_int;

    ret = tas675x_dsp_mem_write(tas, (*t).page, (*t).reg, val);

    /* Cache the value */
    if ret == 0 {
        let mut i = 0usize;
        while i < (*tas).dsp_params.len() {
            if (*tas).dsp_params[i].page == (*t).page && (*tas).dsp_params[i].reg == (*t).reg {
                (*tas).dsp_params[i].val = val;
                break;
            }
            i += 1;
        }
    }

    /* Return 1 to notify change, or propagate error */
    if ret != 0 { ret } else { 1 }
}

unsafe fn tas675x_get_rtldg_thresh(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let tas = snd_soc_component_get_drvdata(comp) as *mut tas675x_priv;
    let t = (*kcontrol).private_value as *const tas675x_reg_param;
    let mut val: u32 = 0;
    let ret: c_int;

    ret = tas675x_dsp_mem_read(tas, (*t).page, (*t).reg, &mut val);
    if ret == 0 {
        (*ucontrol).value.integer.value[0] = val as c_long;
    }

    ret
}

static tas675x_dsp_defaults: [tas675x_reg_param; TAS675X_DSP_PARAM_NUM] = [
    tas675x_reg_param { page: TAS675X_DSP_PAGE_RTLDG, reg: TAS675X_DSP_RTLDG_OL_THRESH_REG, val: 0 },
    tas675x_reg_param { page: TAS675X_DSP_PAGE_RTLDG, reg: TAS675X_DSP_RTLDG_SL_THRESH_REG, val: 0 },
];

const _: () = assert!(tas675x_dsp_defaults.len() == TAS675X_DSP_PARAM_NUM);

unsafe fn tas675x_set_dcldg_trigger(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let tas = snd_soc_component_get_drvdata(comp) as *mut tas675x_priv;
    let mut state: c_uint = 0;
    let mut state34: c_uint = 0;
    let mut ret: c_int;

    if (*ucontrol).value.integer.value[0] == 0 {
        return 0;
    }

    if snd_soc_component_active(comp) != 0 {
        return -EBUSY;
    }

    ret = pm_runtime_resume_and_get((*tas).dev);
    if ret < 0 {
        return ret;
    }

    /*
     * Abort automatic DC LDG retry loops (startup or init-after-fault)
     * and clear faults before manual diagnostics.
     */
    regmap_update_bits((*tas).regmap, TAS675X_DC_LDG_CTRL_REG, TAS675X_LDG_ABORT_BIT | TAS675X_LDG_BYPASS_BIT, TAS675X_LDG_ABORT_BIT | TAS675X_LDG_BYPASS_BIT);
    regmap_write((*tas).regmap, TAS675X_RESET_REG, TAS675X_FAULT_CLEAR);

    /* Wait for LOAD_DIAG to exit */
    ret = regmap_read_poll_timeout!((*tas).regmap, TAS675X_STATE_REPORT_CH1_CH2_REG, state, (state & 0x0F) != TAS675X_STATE_LOAD_DIAG && (state >> 4) != TAS675X_STATE_LOAD_DIAG, TAS675X_POLL_INTERVAL_US, TAS675X_STATE_TRANSITION_TIMEOUT_US);
    ret |= regmap_read_poll_timeout!((*tas).regmap, TAS675X_STATE_REPORT_CH3_CH4_REG, state34, (state34 & 0x0F) != TAS675X_STATE_LOAD_DIAG && (state34 >> 4) != TAS675X_STATE_LOAD_DIAG, TAS675X_POLL_INTERVAL_US, TAS675X_STATE_TRANSITION_TIMEOUT_US);
    if ret != 0 {
        dev_err((*tas).dev, c"DC LDG: abort timeout (CH1/2=0x%02x [%s/%s], CH3/4=0x%02x [%s/%s])\n".as_ptr(), state, tas675x_state_name(state), tas675x_state_name(state >> 4), state34, tas675x_state_name(state34), tas675x_state_name(state34 >> 4));
    } else {
        /* Transition to HIZ state */
        ret = tas675x_set_state_all(tas, TAS675X_STATE_HIZ_BOTH as u8);
        if ret == 0 {
            /* Set LOAD_DIAG state for manual DC LDG */
            ret = tas675x_set_state_all(tas, TAS675X_STATE_LOAD_DIAG_BOTH as u8);
        }
        if ret == 0 {
            /* Wait for device to transition to LOAD_DIAG state */
            ret = regmap_read_poll_timeout!((*tas).regmap, TAS675X_STATE_REPORT_CH1_CH2_REG, state, state == TAS675X_STATE_LOAD_DIAG_BOTH, TAS675X_POLL_INTERVAL_US, TAS675X_STATE_TRANSITION_TIMEOUT_US);
            ret |= regmap_read_poll_timeout!((*tas).regmap, TAS675X_STATE_REPORT_CH3_CH4_REG, state34, state34 == TAS675X_STATE_LOAD_DIAG_BOTH, TAS675X_POLL_INTERVAL_US, TAS675X_STATE_TRANSITION_TIMEOUT_US);
            if ret != 0 {
                dev_err((*tas).dev, c"DC LDG: LOAD_DIAG timeout (CH1/2=0x%02x [%s/%s], CH3/4=0x%02x [%s/%s])\n".as_ptr(), state, tas675x_state_name(state), tas675x_state_name(state >> 4), state34, tas675x_state_name(state34), tas675x_state_name(state34 >> 4));
            } else {
                /* Clear ABORT and BYPASS bits to enable manual DC LDG */
                ret = regmap_update_bits((*tas).regmap, TAS675X_DC_LDG_CTRL_REG, TAS675X_LDG_ABORT_BIT | TAS675X_LDG_BYPASS_BIT, 0);
                if ret == 0 {
                    dev_dbg((*tas).dev, c"DC LDG: Started\n".as_ptr());

                    /* Poll all channels for SLEEP state */
                    ret = regmap_read_poll_timeout!((*tas).regmap, TAS675X_STATE_REPORT_CH1_CH2_REG, state, state == TAS675X_STATE_SLEEP_BOTH, TAS675X_POLL_INTERVAL_US, TAS675X_DC_LDG_TIMEOUT_US);
                    ret |= regmap_read_poll_timeout!((*tas).regmap, TAS675X_STATE_REPORT_CH3_CH4_REG, state34, state34 == TAS675X_STATE_SLEEP_BOTH, TAS675X_POLL_INTERVAL_US, TAS675X_DC_LDG_TIMEOUT_US);
                    if ret != 0 {
                        dev_err((*tas).dev, c"DC LDG: SLEEP timeout (CH1/2=0x%02x [%s/%s], CH3/4=0x%02x [%s/%s])\n".as_ptr(), state, tas675x_state_name(state), tas675x_state_name(state >> 4), state34, tas675x_state_name(state34), tas675x_state_name(state34 >> 4));
                    } else {
                        dev_dbg((*tas).dev, c"DC LDG: Completed successfully (CH1/2=0x%02x, CH3/4=0x%02x)\n".as_ptr(), state, state34);
                    }
                }
            }
            tas675x_set_state_all(tas, TAS675X_STATE_HIZ_BOTH as u8);
        }
    }

    regmap_update_bits((*tas).regmap, TAS675X_DC_LDG_CTRL_REG, TAS675X_LDG_ABORT_BIT | TAS675X_LDG_BYPASS_BIT, 0);

    pm_runtime_mark_last_busy((*tas).dev);
    pm_runtime_put_autosuspend((*tas).dev);

    ret
}

unsafe fn tas675x_set_acldg_trigger(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let tas = snd_soc_component_get_drvdata(comp) as *mut tas675x_priv;
    let mut state: c_uint = 0;
    let mut state34: c_uint = 0;
    let mut ret: c_int;

    if (*ucontrol).value.integer.value[0] == 0 {
        return 0;
    }

    if snd_soc_component_active(comp) != 0 {
        return -EBUSY;
    }

    ret = pm_runtime_resume_and_get((*tas).dev);
    if ret < 0 {
        return ret;
    }

    /* AC Load Diagnostics requires SLEEP state */
    ret = tas675x_set_state_all(tas, TAS675X_STATE_SLEEP_BOTH as u8);
    if ret != 0 {
        dev_err((*tas).dev, c"AC LDG: Failed to set SLEEP state: %d\n".as_ptr(), ret);
    } else {
        /* Start AC LDG on all 4 channels (0x0F) */
        ret = regmap_write((*tas).regmap, TAS675X_AC_LDG_CTRL_REG, 0x0F);
        if ret != 0 {
            dev_err((*tas).dev, c"AC LDG: Failed to start: %d\n".as_ptr(), ret);
        } else {
            dev_dbg((*tas).dev, c"AC LDG: Started\n".as_ptr());

            /* Poll all channels for SLEEP state */
            ret = regmap_read_poll_timeout!((*tas).regmap, TAS675X_STATE_REPORT_CH1_CH2_REG, state, state == TAS675X_STATE_SLEEP_BOTH, TAS675X_POLL_INTERVAL_US, TAS675X_AC_LDG_TIMEOUT_US);
            if ret != 0 {
                dev_err((*tas).dev, c"AC LDG: CH1/CH2 timeout: %d (state=0x%02x [%s/%s])\n".as_ptr(), ret, state, tas675x_state_name(state), tas675x_state_name(state >> 4));
                regmap_write((*tas).regmap, TAS675X_AC_LDG_CTRL_REG, 0x00);
            } else {
                ret = regmap_read_poll_timeout!((*tas).regmap, TAS675X_STATE_REPORT_CH3_CH4_REG, state34, state34 == TAS675X_STATE_SLEEP_BOTH, TAS675X_POLL_INTERVAL_US, TAS675X_AC_LDG_TIMEOUT_US);
                if ret != 0 {
                    dev_err((*tas).dev, c"AC LDG: CH3/CH4 timeout: %d (state=0x%02x [%s/%s])\n".as_ptr(), ret, state34, tas675x_state_name(state34), tas675x_state_name(state34 >> 4));
                    regmap_write((*tas).regmap, TAS675X_AC_LDG_CTRL_REG, 0x00);
                } else {
                    dev_dbg((*tas).dev, c"AC LDG: Completed successfully (CH1/2=0x%02x, CH3/4=0x%02x)\n".as_ptr(), state, state34);
                    regmap_write((*tas).regmap, TAS675X_AC_LDG_CTRL_REG, 0x00);
                }
            }
        }
    }

    pm_runtime_mark_last_busy((*tas).dev);
    pm_runtime_put_autosuspend((*tas).dev);

    ret
}

unsafe fn tas675x_rtldg_impedance_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 0xFFFF;
    0
}

unsafe fn tas675x_get_rtldg_impedance(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let tas = snd_soc_component_get_drvdata(comp) as *mut tas675x_priv;
    let msb_reg = (*kcontrol).private_value as c_uint;
    let mut buf: [u8; 2] = [0; 2];
    let ret: c_int;

    ret = regmap_bulk_read((*tas).regmap, msb_reg, buf.as_mut_ptr() as *mut c_void, 2);
    if ret != 0 {
        return ret;
    }

    (*ucontrol).value.integer.value[0] = (((buf[0] as c_uint) << 8) | buf[1] as c_uint) as c_long;
    0
}

unsafe fn tas675x_dc_resistance_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    /* 10-bit: 2-bit MSB + 8-bit LSB, 0.1 ohm/code, 0-102.3 ohm */
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1023;
    0
}

unsafe fn tas675x_get_dc_resistance(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let tas = snd_soc_component_get_drvdata(comp) as *mut tas675x_priv;
    let lsb_reg = (*kcontrol).private_value as c_uint;
    let mut msb: c_uint = 0;
    let mut lsb: c_uint = 0;
    let shift: c_uint;
    let mut ret: c_int;

    ret = regmap_read((*tas).regmap, TAS675X_DC_LDG_DCR_MSB_REG, &mut msb);
    if ret != 0 {
        return ret;
    }

    ret = regmap_read((*tas).regmap, lsb_reg, &mut lsb);
    if ret != 0 {
        return ret;
    }

    /* 2-bit MSB: CH1=[7:6], CH2=[5:4], CH3=[3:2], CH4=[1:0] */
    shift = 6 - (lsb_reg - TAS675X_CH1_DC_LDG_DCR_LSB_REG) * 2;
    msb = (msb >> shift) & 0x3;

    (*ucontrol).value.integer.value[0] = ((msb << 8) | lsb) as c_long;
    0
}

/*
 * Counterpart macros with read-only access:
 * SOC_SINGLE_RO, SOC_DC_RESIST_RO, SOC_RTLDG_IMP_RO, and SOC_DSP_THRESH_EXT
 * are preserved as Rust macro invocations below because their struct layouts
 * and helper macros are supplied by external ALSA/kernel dependencies.
 */

/*
 * DAC digital volumes. From -103 to 0 dB in 0.5 dB steps, -103.5 dB means mute.
 * DAC analog gain. From -15.5 to 0 dB in 0.5 dB steps, no mute.
 */
static tas675x_dig_vol_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(tas675x_dig_vol_tlv, -10350, 50, 1);
static tas675x_ana_gain_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(tas675x_ana_gain_tlv, -1550, 50, 0);

static tas675x_ss_texts: [*const c_char; 4] = [c"Disabled".as_ptr(), c"Triangle".as_ptr(), c"Random".as_ptr(), c"Triangle and Random".as_ptr()];
static tas675x_ss_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ss_enum, TAS675X_SS_CTRL_REG, 0, tas675x_ss_texts);
static tas675x_ss_tri_range_texts: [*const c_char; 4] = [c"6.5%".as_ptr(), c"13.5%".as_ptr(), c"5%".as_ptr(), c"10%".as_ptr()];
static tas675x_ss_tri_range_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ss_tri_range_enum, TAS675X_SS_RANGE_CTRL_REG, 0, tas675x_ss_tri_range_texts);
static tas675x_ss_rdm_range_texts: [*const c_char; 5] = [c"0.83%".as_ptr(), c"2.50%".as_ptr(), c"5.83%".as_ptr(), c"12.50%".as_ptr(), c"25.83%".as_ptr()];
static tas675x_ss_rdm_range_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ss_rdm_range_enum, TAS675X_SS_RANGE_CTRL_REG, 4, tas675x_ss_rdm_range_texts);
static tas675x_ss_rdm_dwell_texts: [*const c_char; 4] = [c"1/FSS to 2/FSS".as_ptr(), c"1/FSS to 4/FSS".as_ptr(), c"1/FSS to 8/FSS".as_ptr(), c"1/FSS to 15/FSS".as_ptr()];
static tas675x_ss_rdm_dwell_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ss_rdm_dwell_enum, TAS675X_SS_RANGE_CTRL_REG, 2, tas675x_ss_rdm_dwell_texts);
static tas675x_oc_limit_texts: [*const c_char; 4] = [c"Level 4".as_ptr(), c"Level 3".as_ptr(), c"Level 2".as_ptr(), c"Level 1".as_ptr()];
static tas675x_oc_limit_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_oc_limit_enum, TAS675X_CURRENT_LIMIT_CTRL_REG, 0, tas675x_oc_limit_texts);
static tas675x_otw_texts: [*const c_char; 8] = [c"Disabled".as_ptr(), c">95C".as_ptr(), c">110C".as_ptr(), c">125C".as_ptr(), c">135C".as_ptr(), c">145C".as_ptr(), c">155C".as_ptr(), c">165C".as_ptr()];
static tas675x_ch1_otw_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ch1_otw_enum, TAS675X_OTW_CTRL_CH1_CH2_REG, 4, tas675x_otw_texts);
static tas675x_ch2_otw_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ch2_otw_enum, TAS675X_OTW_CTRL_CH1_CH2_REG, 0, tas675x_otw_texts);
static tas675x_ch3_otw_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ch3_otw_enum, TAS675X_OTW_CTRL_CH3_CH4_REG, 4, tas675x_otw_texts);
static tas675x_ch4_otw_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ch4_otw_enum, TAS675X_OTW_CTRL_CH3_CH4_REG, 0, tas675x_otw_texts);
static tas675x_dc_ldg_sl_texts: [*const c_char; 10] = [c"0.5 Ohm".as_ptr(), c"1 Ohm".as_ptr(), c"1.5 Ohm".as_ptr(), c"2 Ohm".as_ptr(), c"2.5 Ohm".as_ptr(), c"3 Ohm".as_ptr(), c"3.5 Ohm".as_ptr(), c"4 Ohm".as_ptr(), c"4.5 Ohm".as_ptr(), c"5 Ohm".as_ptr()];
static tas675x_ch1_dc_ldg_sl_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ch1_dc_ldg_sl_enum, TAS675X_DC_LDG_SL_CH1_CH2_CTRL_REG, 4, tas675x_dc_ldg_sl_texts);
static tas675x_ch2_dc_ldg_sl_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ch2_dc_ldg_sl_enum, TAS675X_DC_LDG_SL_CH1_CH2_CTRL_REG, 0, tas675x_dc_ldg_sl_texts);
static tas675x_ch3_dc_ldg_sl_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ch3_dc_ldg_sl_enum, TAS675X_DC_LDG_SL_CH3_CH4_CTRL_REG, 4, tas675x_dc_ldg_sl_texts);
static tas675x_ch4_dc_ldg_sl_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ch4_dc_ldg_sl_enum, TAS675X_DC_LDG_SL_CH3_CH4_CTRL_REG, 0, tas675x_dc_ldg_sl_texts);
static tas675x_dc_slol_ramp_texts: [*const c_char; 4] = [c"15 ms".as_ptr(), c"30 ms".as_ptr(), c"10 ms".as_ptr(), c"20 ms".as_ptr()];
static tas675x_dc_slol_ramp_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_dc_slol_ramp_enum, TAS675X_DC_LDG_TIME_CTRL_REG, 6, tas675x_dc_slol_ramp_texts);
static tas675x_dc_slol_settling_texts: [*const c_char; 4] = [c"10 ms".as_ptr(), c"5 ms".as_ptr(), c"20 ms".as_ptr(), c"15 ms".as_ptr()];
static tas675x_dc_slol_settling_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_dc_slol_settling_enum, TAS675X_DC_LDG_TIME_CTRL_REG, 4, tas675x_dc_slol_settling_texts);
static tas675x_dc_s2pg_ramp_texts: [*const c_char; 4] = [c"5 ms".as_ptr(), c"2.5 ms".as_ptr(), c"10 ms".as_ptr(), c"15 ms".as_ptr()];
static tas675x_dc_s2pg_ramp_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_dc_s2pg_ramp_enum, TAS675X_DC_LDG_TIME_CTRL_REG, 2, tas675x_dc_s2pg_ramp_texts);
static tas675x_dc_s2pg_settling_texts: [*const c_char; 4] = [c"10 ms".as_ptr(), c"5 ms".as_ptr(), c"20 ms".as_ptr(), c"30 ms".as_ptr()];
static tas675x_dc_s2pg_settling_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_dc_s2pg_settling_enum, TAS675X_DC_LDG_TIME_CTRL_REG, 0, tas675x_dc_s2pg_settling_texts);
static tas675x_dsp_mode_texts: [*const c_char; 3] = [c"Normal".as_ptr(), c"LLP".as_ptr(), c"FFLP".as_ptr()];
static tas675x_dsp_mode_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_dsp_mode_enum, TAS675X_LL_EN_REG, 0, tas675x_dsp_mode_texts);
static tas675x_ana_ramp_texts: [*const c_char; 4] = [c"15us".as_ptr(), c"60us".as_ptr(), c"200us".as_ptr(), c"400us".as_ptr()];
static tas675x_ana_ramp_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ana_ramp_enum, TAS675X_ANALOG_GAIN_RAMP_CTRL_REG, 2, tas675x_ana_ramp_texts);
static tas675x_ramp_rate_texts: [*const c_char; 4] = [c"4 FS".as_ptr(), c"16 FS".as_ptr(), c"32 FS".as_ptr(), c"Instant".as_ptr()];
static tas675x_ramp_down_rate_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ramp_down_rate_enum, TAS675X_DIG_VOL_RAMP_CTRL_REG, 6, tas675x_ramp_rate_texts);
static tas675x_ramp_up_rate_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ramp_up_rate_enum, TAS675X_DIG_VOL_RAMP_CTRL_REG, 2, tas675x_ramp_rate_texts);
static tas675x_ramp_step_texts: [*const c_char; 4] = [c"4dB".as_ptr(), c"2dB".as_ptr(), c"1dB".as_ptr(), c"0.5dB".as_ptr()];
static tas675x_ramp_down_step_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ramp_down_step_enum, TAS675X_DIG_VOL_RAMP_CTRL_REG, 4, tas675x_ramp_step_texts);
static tas675x_ramp_up_step_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ramp_up_step_enum, TAS675X_DIG_VOL_RAMP_CTRL_REG, 0, tas675x_ramp_step_texts);
static tas675x_vol_combine_ch12_texts: [*const c_char; 3] = [c"Independent".as_ptr(), c"CH2 follows CH1".as_ptr(), c"CH1 follows CH2".as_ptr()];
static tas675x_vol_combine_ch12_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_vol_combine_ch12_enum, TAS675X_DIG_VOL_COMBINE_CTRL_REG, 0, tas675x_vol_combine_ch12_texts);
static tas675x_vol_combine_ch34_texts: [*const c_char; 3] = [c"Independent".as_ptr(), c"CH4 follows CH3".as_ptr(), c"CH3 follows CH4".as_ptr()];
static tas675x_vol_combine_ch34_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_vol_combine_ch34_enum, TAS675X_DIG_VOL_COMBINE_CTRL_REG, 2, tas675x_vol_combine_ch34_texts);
static tas675x_auto_mute_time_texts: [*const c_char; 8] = [c"11.5ms".as_ptr(), c"53ms".as_ptr(), c"106.5ms".as_ptr(), c"266.5ms".as_ptr(), c"535ms".as_ptr(), c"1065ms".as_ptr(), c"2665ms".as_ptr(), c"5330ms".as_ptr()];
static tas675x_ch1_mute_time_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ch1_mute_time_enum, TAS675X_AUTO_MUTE_TIMING_CH1_CH2_REG, 4, tas675x_auto_mute_time_texts);
static tas675x_ch2_mute_time_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ch2_mute_time_enum, TAS675X_AUTO_MUTE_TIMING_CH1_CH2_REG, 0, tas675x_auto_mute_time_texts);
static tas675x_ch3_mute_time_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ch3_mute_time_enum, TAS675X_AUTO_MUTE_TIMING_CH3_CH4_REG, 4, tas675x_auto_mute_time_texts);
static tas675x_ch4_mute_time_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(tas675x_ch4_mute_time_enum, TAS675X_AUTO_MUTE_TIMING_CH3_CH4_REG, 0, tas675x_auto_mute_time_texts);

/*
 * ALSA Mixer Controls
 *
 * For detailed documentation of each control see:
 * Documentation/sound/codecs/tas675x.rst
 */
static tas675x_snd_controls: &[snd_kcontrol_new] = &snd_controls![
    /* Volume & Gain Control */
    SOC_DOUBLE_R_TLV!("Analog Playback Volume", TAS675X_ANALOG_GAIN_CH1_CH2_REG, TAS675X_ANALOG_GAIN_CH3_CH4_REG, 1, 0x1F, 1, tas675x_ana_gain_tlv),
    SOC_ENUM!("Analog Gain Ramp Step", tas675x_ana_ramp_enum),
    SOC_SINGLE_RANGE_TLV!("CH1 Digital Playback Volume", TAS675X_DIG_VOL_CH1_REG, 0, 0x30, 0xFF, 1, tas675x_dig_vol_tlv),
    SOC_SINGLE_RANGE_TLV!("CH2 Digital Playback Volume", TAS675X_DIG_VOL_CH2_REG, 0, 0x30, 0xFF, 1, tas675x_dig_vol_tlv),
    SOC_SINGLE_RANGE_TLV!("CH3 Digital Playback Volume", TAS675X_DIG_VOL_CH3_REG, 0, 0x30, 0xFF, 1, tas675x_dig_vol_tlv),
    SOC_SINGLE_RANGE_TLV!("CH4 Digital Playback Volume", TAS675X_DIG_VOL_CH4_REG, 0, 0x30, 0xFF, 1, tas675x_dig_vol_tlv),
    SOC_ENUM!("Volume Ramp Down Rate", tas675x_ramp_down_rate_enum),
    SOC_ENUM!("Volume Ramp Down Step", tas675x_ramp_down_step_enum),
    SOC_ENUM!("Volume Ramp Up Rate", tas675x_ramp_up_rate_enum),
    SOC_ENUM!("Volume Ramp Up Step", tas675x_ramp_up_step_enum),
    SOC_ENUM!("CH1/2 Volume Combine", tas675x_vol_combine_ch12_enum),
    SOC_ENUM!("CH3/4 Volume Combine", tas675x_vol_combine_ch34_enum),
    /* Auto Mute & Silence Detection */
    SOC_SINGLE!("CH1 Auto Mute Switch", TAS675X_AUTO_MUTE_EN_REG, 0, 1, 0),
    SOC_SINGLE!("CH2 Auto Mute Switch", TAS675X_AUTO_MUTE_EN_REG, 1, 1, 0),
    SOC_SINGLE!("CH3 Auto Mute Switch", TAS675X_AUTO_MUTE_EN_REG, 2, 1, 0),
    SOC_SINGLE!("CH4 Auto Mute Switch", TAS675X_AUTO_MUTE_EN_REG, 3, 1, 0),
    SOC_SINGLE!("Auto Mute Combine Switch", TAS675X_AUTO_MUTE_EN_REG, 4, 1, 0),
    SOC_ENUM!("CH1 Auto Mute Time", tas675x_ch1_mute_time_enum),
    SOC_ENUM!("CH2 Auto Mute Time", tas675x_ch2_mute_time_enum),
    SOC_ENUM!("CH3 Auto Mute Time", tas675x_ch3_mute_time_enum),
    SOC_ENUM!("CH4 Auto Mute Time", tas675x_ch4_mute_time_enum),
    /* Clock & EMI Management */
    SOC_ENUM!("Spread Spectrum Mode", tas675x_ss_enum),
    SOC_ENUM!("SS Triangle Range", tas675x_ss_tri_range_enum),
    SOC_ENUM!("SS Random Range", tas675x_ss_rdm_range_enum),
    SOC_ENUM!("SS Random Dwell Range", tas675x_ss_rdm_dwell_enum),
    SOC_SINGLE!("SS Triangle Dwell Min", TAS675X_SS_DWELL_CTRL_REG, 4, 15, 0),
    SOC_SINGLE!("SS Triangle Dwell Max", TAS675X_SS_DWELL_CTRL_REG, 0, 15, 0),
    /* Hardware Protection */
    SOC_SINGLE!("OTSD Auto Recovery Switch", TAS675X_OTSD_RECOVERY_EN_REG, 1, 1, 0),
    SOC_ENUM!("Overcurrent Limit Level", tas675x_oc_limit_enum),
    SOC_ENUM!("CH1 OTW Threshold", tas675x_ch1_otw_enum),
    SOC_ENUM!("CH2 OTW Threshold", tas675x_ch2_otw_enum),
    SOC_ENUM!("CH3 OTW Threshold", tas675x_ch3_otw_enum),
    SOC_ENUM!("CH4 OTW Threshold", tas675x_ch4_otw_enum),
    /* DSP Signal Path & Mode */
    SOC_ENUM!("DSP Signal Path Mode", tas675x_dsp_mode_enum),
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"DC LDG Trigger".as_ptr(), access: SNDRV_CTL_ELEM_ACCESS_WRITE, info: Some(snd_ctl_boolean_mono_info), put: Some(tas675x_set_dcldg_trigger), ..ZEROED },
    SOC_SINGLE!("DC LDG Auto Diagnostics Switch", TAS675X_DC_LDG_CTRL_REG, 0, 1, 1),
    SOC_SINGLE!("CH1 LO LDG Switch", TAS675X_DC_LDG_LO_CTRL_REG, 3, 1, 0),
    SOC_SINGLE!("CH2 LO LDG Switch", TAS675X_DC_LDG_LO_CTRL_REG, 2, 1, 0),
    SOC_SINGLE!("CH3 LO LDG Switch", TAS675X_DC_LDG_LO_CTRL_REG, 1, 1, 0),
    SOC_SINGLE!("CH4 LO LDG Switch", TAS675X_DC_LDG_LO_CTRL_REG, 0, 1, 0),
    SOC_ENUM!("DC LDG SLOL Ramp Time", tas675x_dc_slol_ramp_enum),
    SOC_ENUM!("DC LDG SLOL Settling Time", tas675x_dc_slol_settling_enum),
    SOC_ENUM!("DC LDG S2PG Ramp Time", tas675x_dc_s2pg_ramp_enum),
    SOC_ENUM!("DC LDG S2PG Settling Time", tas675x_dc_s2pg_settling_enum),
    SOC_ENUM!("CH1 DC LDG SL Threshold", tas675x_ch1_dc_ldg_sl_enum),
    SOC_ENUM!("CH2 DC LDG SL Threshold", tas675x_ch2_dc_ldg_sl_enum),
    SOC_ENUM!("CH3 DC LDG SL Threshold", tas675x_ch3_dc_ldg_sl_enum),
    SOC_ENUM!("CH4 DC LDG SL Threshold", tas675x_ch4_dc_ldg_sl_enum),
    SOC_SINGLE_RO!("DC LDG Result", TAS675X_DC_LDG_RESULT_REG, 0, 0xFF),
    SOC_SINGLE_RO!("CH1 DC LDG Report", TAS675X_DC_LDG_REPORT_CH1_CH2_REG, 4, 0x0F),
    SOC_SINGLE_RO!("CH2 DC LDG Report", TAS675X_DC_LDG_REPORT_CH1_CH2_REG, 0, 0x0F),
    SOC_SINGLE_RO!("CH3 DC LDG Report", TAS675X_DC_LDG_REPORT_CH3_CH4_REG, 4, 0x0F),
    SOC_SINGLE_RO!("CH4 DC LDG Report", TAS675X_DC_LDG_REPORT_CH3_CH4_REG, 0, 0x0F),
    SOC_SINGLE_RO!("CH1 LO LDG Report", TAS675X_DC_LDG_RESULT_REG, 7, 1),
    SOC_SINGLE_RO!("CH2 LO LDG Report", TAS675X_DC_LDG_RESULT_REG, 6, 1),
    SOC_SINGLE_RO!("CH3 LO LDG Report", TAS675X_DC_LDG_RESULT_REG, 5, 1),
    SOC_SINGLE_RO!("CH4 LO LDG Report", TAS675X_DC_LDG_RESULT_REG, 4, 1),
    SOC_DC_RESIST_RO!("CH1 DC Resistance", TAS675X_CH1_DC_LDG_DCR_LSB_REG),
    SOC_DC_RESIST_RO!("CH2 DC Resistance", TAS675X_CH2_DC_LDG_DCR_LSB_REG),
    SOC_DC_RESIST_RO!("CH3 DC Resistance", TAS675X_CH3_DC_LDG_DCR_LSB_REG),
    SOC_DC_RESIST_RO!("CH4 DC Resistance", TAS675X_CH4_DC_LDG_DCR_LSB_REG),
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"AC LDG Trigger".as_ptr(), access: SNDRV_CTL_ELEM_ACCESS_WRITE, info: Some(snd_ctl_boolean_mono_info), put: Some(tas675x_set_acldg_trigger), ..ZEROED },
    SOC_SINGLE!("AC LDG Gain", TAS675X_AC_LDG_CTRL_REG, 4, 1, 0),
    SOC_SINGLE!("AC LDG Test Frequency", TAS675X_AC_LDG_FREQ_CTRL_REG, 0, 0xFF, 0),
    SOC_SINGLE_RO!("CH1 AC LDG Real", TAS675X_AC_LDG_REPORT_CH1_R_REG, 0, 0xFF),
    SOC_SINGLE_RO!("CH1 AC LDG Imag", TAS675X_AC_LDG_REPORT_CH1_I_REG, 0, 0xFF),
    SOC_SINGLE_RO!("CH2 AC LDG Real", TAS675X_AC_LDG_REPORT_CH2_R_REG, 0, 0xFF),
    SOC_SINGLE_RO!("CH2 AC LDG Imag", TAS675X_AC_LDG_REPORT_CH2_I_REG, 0, 0xFF),
    SOC_SINGLE_RO!("CH3 AC LDG Real", TAS675X_AC_LDG_REPORT_CH3_R_REG, 0, 0xFF),
    SOC_SINGLE_RO!("CH3 AC LDG Imag", TAS675X_AC_LDG_REPORT_CH3_I_REG, 0, 0xFF),
    SOC_SINGLE_RO!("CH4 AC LDG Real", TAS675X_AC_LDG_REPORT_CH4_R_REG, 0, 0xFF),
    SOC_SINGLE_RO!("CH4 AC LDG Imag", TAS675X_AC_LDG_REPORT_CH4_I_REG, 0, 0xFF),
    SOC_SINGLE_RO!("PVDD Sense", TAS675X_PVDD_SENSE_REG, 0, 0xFF),
    SOC_SINGLE_RO!("Global Temperature", TAS675X_TEMP_GLOBAL_REG, 0, 0xFF),
    SOC_SINGLE_RO!("CH1 Temperature Range", TAS675X_TEMP_CH1_CH2_REG, 0, 7),
    SOC_SINGLE_RO!("CH2 Temperature Range", TAS675X_TEMP_CH1_CH2_REG, 3, 7),
    SOC_SINGLE_RO!("CH3 Temperature Range", TAS675X_TEMP_CH3_CH4_REG, 0, 7),
    SOC_SINGLE_RO!("CH4 Temperature Range", TAS675X_TEMP_CH3_CH4_REG, 3, 7),
    SOC_SINGLE!("Tweeter Detection Switch", TAS675X_TWEETER_DETECT_CTRL_REG, 0, 1, 1),
    SOC_SINGLE!("Tweeter Detect Threshold", TAS675X_TWEETER_DETECT_THRESH_REG, 0, 0xFF, 0),
    SOC_SINGLE_RO!("CH1 Tweeter Detect Report", TAS675X_TWEETER_REPORT_REG, 3, 1),
    SOC_SINGLE_RO!("CH2 Tweeter Detect Report", TAS675X_TWEETER_REPORT_REG, 2, 1),
    SOC_SINGLE_RO!("CH3 Tweeter Detect Report", TAS675X_TWEETER_REPORT_REG, 1, 1),
    SOC_SINGLE_RO!("CH4 Tweeter Detect Report", TAS675X_TWEETER_REPORT_REG, 0, 1),
    /* Unavailable in LLP, available in Normal & FFLP */
    SOC_SINGLE!("Thermal Foldback Switch", TAS675X_DSP_CTRL_REG, 0, 1, 0),
    SOC_SINGLE!("PVDD Foldback Switch", TAS675X_DSP_CTRL_REG, 4, 1, 0),
    SOC_SINGLE!("DC Blocker Bypass Switch", TAS675X_DC_BLOCK_BYP_REG, 0, 1, 0),
    SOC_SINGLE!("Clip Detect Switch", TAS675X_CLIP_DETECT_CTRL_REG, 6, 1, 0),
    SOC_SINGLE!("Audio SDOUT Switch", TAS675X_DSP_CTRL_REG, 5, 1, 0),
    /* Real-Time Load Diagnostics */
    SOC_SINGLE!("CH1 RTLDG Switch", TAS675X_RTLDG_EN_REG, 3, 1, 0),
    SOC_SINGLE!("CH2 RTLDG Switch", TAS675X_RTLDG_EN_REG, 2, 1, 0),
    SOC_SINGLE!("CH3 RTLDG Switch", TAS675X_RTLDG_EN_REG, 1, 1, 0),
    SOC_SINGLE!("CH4 RTLDG Switch", TAS675X_RTLDG_EN_REG, 0, 1, 0),
    SOC_SINGLE!("RTLDG Clip Mask Switch", TAS675X_RTLDG_EN_REG, 4, 1, 0),
    SOC_SINGLE!("ISENSE Calibration Switch", TAS675X_ISENSE_CAL_REG, 3, 1, 0),
    SOC_DSP_THRESH_EXT!("RTLDG Open Load Threshold", tas675x_dsp_defaults[TAS675X_DSP_PARAM_ID_OL_THRESH as usize]),
    SOC_DSP_THRESH_EXT!("RTLDG Short Load Threshold", tas675x_dsp_defaults[TAS675X_DSP_PARAM_ID_SL_THRESH as usize]),
    SOC_RTLDG_IMP_RO!("CH1 RTLDG Impedance", TAS675X_CH1_RTLDG_IMP_MSB_REG),
    SOC_RTLDG_IMP_RO!("CH2 RTLDG Impedance", TAS675X_CH2_RTLDG_IMP_MSB_REG),
    SOC_RTLDG_IMP_RO!("CH3 RTLDG Impedance", TAS675X_CH3_RTLDG_IMP_MSB_REG),
    SOC_RTLDG_IMP_RO!("CH4 RTLDG Impedance", TAS675X_CH4_RTLDG_IMP_MSB_REG),
];

static tas675x_audio_path_switch: snd_kcontrol_new = SOC_DAPM_SINGLE!("Switch", SND_SOC_NOPM, 0, 1, 1);
static tas675x_anc_path_switch: snd_kcontrol_new = SOC_DAPM_SINGLE!("Switch", SND_SOC_NOPM, 0, 1, 1);

static tas675x_dapm_widgets: &[snd_soc_dapm_widget] = &dapm_widgets![
    SND_SOC_DAPM_SUPPLY!("Analog Core", SND_SOC_NOPM, 0, 0, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!("SDOUT Vpredict", SND_SOC_NOPM, 0, 0, ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY!("SDOUT Isense", SND_SOC_NOPM, 0, 0, ptr::null_mut(), 0),
    SND_SOC_DAPM_DAC!("Audio DAC", "Playback", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_DAC!("ANC DAC", "ANC Playback", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_ADC!("Feedback ADC", "Feedback Capture", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_SWITCH!("Audio Path", SND_SOC_NOPM, 0, 0, &tas675x_audio_path_switch),
    SND_SOC_DAPM_SWITCH!("ANC Path", SND_SOC_NOPM, 0, 0, &tas675x_anc_path_switch),
    /*
     * Even though all channels are coupled in terms of power control,
     * use logical outputs for each channel to allow independent routing
     * and DAPM controls if needed.
     */
    SND_SOC_DAPM_OUTPUT!("OUT_CH1"),
    SND_SOC_DAPM_OUTPUT!("OUT_CH2"),
    SND_SOC_DAPM_OUTPUT!("OUT_CH3"),
    SND_SOC_DAPM_OUTPUT!("OUT_CH4"),
    SND_SOC_DAPM_INPUT!("SPEAKER_LOAD"),
];

static tas675x_dapm_routes: [snd_soc_dapm_route; 14] = [
    snd_soc_dapm_route { sink: c"Audio DAC".as_ptr(), control: ptr::null(), source: c"Analog Core".as_ptr() },
    snd_soc_dapm_route { sink: c"Audio Path".as_ptr(), control: c"Switch".as_ptr(), source: c"Audio DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT_CH1".as_ptr(), control: ptr::null(), source: c"Audio Path".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT_CH2".as_ptr(), control: ptr::null(), source: c"Audio Path".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT_CH3".as_ptr(), control: ptr::null(), source: c"Audio Path".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT_CH4".as_ptr(), control: ptr::null(), source: c"Audio Path".as_ptr() },
    snd_soc_dapm_route { sink: c"ANC DAC".as_ptr(), control: ptr::null(), source: c"Analog Core".as_ptr() },
    snd_soc_dapm_route { sink: c"ANC Path".as_ptr(), control: c"Switch".as_ptr(), source: c"ANC DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT_CH1".as_ptr(), control: ptr::null(), source: c"ANC Path".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT_CH2".as_ptr(), control: ptr::null(), source: c"ANC Path".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT_CH3".as_ptr(), control: ptr::null(), source: c"ANC Path".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT_CH4".as_ptr(), control: ptr::null(), source: c"ANC Path".as_ptr() },
    snd_soc_dapm_route { sink: c"Feedback ADC".as_ptr(), control: ptr::null(), source: c"Analog Core".as_ptr() },
    snd_soc_dapm_route { sink: c"Feedback ADC".as_ptr(), control: ptr::null(), source: c"SDOUT Vpredict".as_ptr() },
    snd_soc_dapm_route { sink: c"Feedback ADC".as_ptr(), control: ptr::null(), source: c"SDOUT Isense".as_ptr() },
    snd_soc_dapm_route { sink: c"Feedback ADC".as_ptr(), control: ptr::null(), source: c"SPEAKER_LOAD".as_ptr() },
];

unsafe fn tas675x_program_slot_offsets(tas: *mut tas675x_priv, dai_id: c_int, slot_width: c_int) {
    let mut offset: c_int = 0;

    match dai_id {
        0 => {
            /* Standard Audio on SDIN */
            if (*tas).audio_slot >= 0 {
                offset = (*tas).audio_slot * slot_width;
            } else if (*tas).tx_mask != 0 {
                offset = __ffs((*tas).tx_mask) as c_int * slot_width;
            } else {
                return;
            }
            offset += (*tas).bclk_offset;
            regmap_update_bits((*tas).regmap, TAS675X_SDIN_OFFSET_MSB_REG, TAS675X_SDIN_AUDIO_OFF_MSB_MASK, FIELD_PREP(TAS675X_SDIN_AUDIO_OFF_MSB_MASK, (offset >> 8) as c_uint));
            regmap_write((*tas).regmap, TAS675X_SDIN_AUDIO_OFFSET_REG, (offset & 0xFF) as c_uint);
        }
        1 => {
            /*
             * Low-Latency Playback on SDIN, **only** enabled in LLP mode
             * and to be mixed with main audio before output amplification
             * to achieve ANC/RNC.
             */
            if (*tas).llp_slot >= 0 {
                offset = (*tas).llp_slot * slot_width;
            } else if (*tas).tx_mask != 0 {
                offset = __ffs((*tas).tx_mask) as c_int * slot_width;
            } else {
                return;
            }
            offset += (*tas).bclk_offset;
            regmap_update_bits((*tas).regmap, TAS675X_SDIN_OFFSET_MSB_REG, TAS675X_SDIN_LL_OFF_MSB_MASK, FIELD_PREP(TAS675X_SDIN_LL_OFF_MSB_MASK, (offset >> 8) as c_uint));
            regmap_write((*tas).regmap, TAS675X_SDIN_LL_OFFSET_REG, (offset & 0xFF) as c_uint);
        }
        2 => {
            /* SDOUT Data Output (Vpredict + Isense feedback) */
            if (*tas).slot_width == 0 {
                return;
            }
            if (*tas).vpredict_slot >= 0 {
                offset = (*tas).vpredict_slot * slot_width;
                offset += (*tas).bclk_offset;
                regmap_update_bits((*tas).regmap, TAS675X_SDOUT_OFFSET_MSB_REG, TAS675X_SDOUT_VP_OFF_MSB_MASK, FIELD_PREP(TAS675X_SDOUT_VP_OFF_MSB_MASK, (offset >> 8) as c_uint));
                regmap_write((*tas).regmap, TAS675X_VPREDICT_OFFSET_REG, (offset & 0xFF) as c_uint);
            }
            if (*tas).isense_slot >= 0 {
                offset = (*tas).isense_slot * slot_width;
                offset += (*tas).bclk_offset;
                regmap_update_bits((*tas).regmap, TAS675X_SDOUT_OFFSET_MSB_REG, TAS675X_SDOUT_IS_OFF_MSB_MASK, FIELD_PREP(TAS675X_SDOUT_IS_OFF_MSB_MASK, (offset >> 8) as c_uint));
                regmap_write((*tas).regmap, TAS675X_ISENSE_OFFSET_REG, (offset & 0xFF) as c_uint);
            }
        }
        _ => {}
    }

    if offset > 511 {
        dev_warn((*tas).dev, c"DAI %d slot offset %d exceeds 511 SCLK limit\n".as_ptr(), dai_id, offset);
    }
}

unsafe fn tas675x_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let tas = snd_soc_component_get_drvdata(component) as *mut tas675x_priv;
    let rate = params_rate(params);
    let word_length: u8;

    /*
     * Single clock domain: SDIN and SDOUT share one SCLK/FSYNC pair,
     * so all active DAIs must use the same sample rate.
     */
    if (READ_ONCE((*tas).active_playback_dais) != 0 || READ_ONCE((*tas).active_capture_dais) != 0) && (*tas).rate != 0 && (*tas).rate != rate {
        dev_err((*component).dev, c"Rate %u conflicts with active rate %u\n".as_ptr(), rate, (*tas).rate);
        return -EINVAL;
    }

    match params_width(params) {
        16 => word_length = TAS675X_WL_16BIT as u8,
        20 => word_length = TAS675X_WL_20BIT as u8,
        24 => word_length = TAS675X_WL_24BIT as u8,
        32 => word_length = TAS675X_WL_32BIT as u8,
        _ => return -EINVAL,
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        /*
         * RTLDG is not supported above 96kHz. Auto-disable to
         * prevent DSP overload and restore when rate drops back.
         */
        if rate > 96000 {
            let mut val: c_uint = 0;
            regmap_read((*component).regmap, TAS675X_RTLDG_EN_REG, &mut val);
            if (val & TAS675X_RTLDG_CH_EN_MASK) != 0 {
                (*tas).saved_rtldg_en = val;
                dev_dbg((*component).dev, c"Sample rate %dHz > 96kHz: Auto-disabling RTLDG\n".as_ptr(), rate);
                regmap_update_bits((*component).regmap, TAS675X_RTLDG_EN_REG, TAS675X_RTLDG_CH_EN_MASK, 0x00);
            }
        } else if (*tas).saved_rtldg_en != 0 {
            let mut cur: c_uint = 0;
            /* Respect overrides and only restore if RTLDG is still auto-disabled */
            regmap_read((*component).regmap, TAS675X_RTLDG_EN_REG, &mut cur);
            if (cur & TAS675X_RTLDG_CH_EN_MASK) == 0 {
                dev_dbg((*component).dev, c"Restoring RTLDG config after high-rate stream\n".as_ptr());
                regmap_update_bits((*component).regmap, TAS675X_RTLDG_EN_REG, TAS675X_RTLDG_CH_EN_MASK, TAS675X_RTLDG_CH_EN_MASK & (*tas).saved_rtldg_en);
            }
            (*tas).saved_rtldg_en = 0;
        }

        /* Set SDIN word length (audio path + low-latency path) */
        regmap_update_bits((*component).regmap, TAS675X_SDIN_CTRL_REG, TAS675X_SDIN_WL_MASK, FIELD_PREP(TAS675X_SDIN_AUDIO_WL_MASK, word_length as c_uint) | FIELD_PREP(TAS675X_SDIN_LL_WL_MASK, word_length as c_uint));
    } else {
        /* Set SDOUT word length (VPREDICT + ISENSE) for capture */
        regmap_update_bits((*component).regmap, TAS675X_SDOUT_CTRL_REG, TAS675X_SDOUT_WL_MASK, FIELD_PREP(TAS675X_SDOUT_VP_WL_MASK, word_length as c_uint) | FIELD_PREP(TAS675X_SDOUT_IS_WL_MASK, word_length as c_uint));
    }

    tas675x_program_slot_offsets(tas, (*dai).id, if (*tas).slot_width != 0 { (*tas).slot_width } else { params_width(params) });
    (*tas).rate = rate;
    0
}

unsafe fn tas675x_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let tas = snd_soc_component_get_drvdata(component) as *mut tas675x_priv;
    let mut tdm_mode = false;
    let mut i2s_mode = false;

    /* Enforce Clocking Direction (Codec is strictly a consumer) */
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FC => {}
        _ => {
            dev_err((*component).dev, c"Unsupported clock provider format\n".as_ptr());
            return -EINVAL;
        }
    }

    /* SCLK polarity: NB_NF or IB_NF only (no FSYNC inversion support) */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => { regmap_update_bits((*component).regmap, TAS675X_SCLK_INV_CTRL_REG, TAS675X_SCLK_INV_MASK, 0x00); }
        SND_SOC_DAIFMT_IB_NF => { regmap_update_bits((*component).regmap, TAS675X_SCLK_INV_CTRL_REG, TAS675X_SCLK_INV_MASK, TAS675X_SCLK_INV_MASK); }
        _ => {
            dev_err((*component).dev, c"Unsupported clock inversion\n".as_ptr());
            return -EINVAL;
        }
    }

    /* Configure Audio Format and TDM Enable */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            i2s_mode = true;
            (*tas).bclk_offset = 0;
            regmap_update_bits((*component).regmap, TAS675X_AUDIO_IF_CTRL_REG, TAS675X_TDM_EN_BIT | TAS675X_SAP_FMT_MASK | TAS675X_FS_PULSE_MASK, TAS675X_SAP_FMT_I2S);
            regmap_update_bits((*component).regmap, TAS675X_SDOUT_CTRL_REG, TAS675X_SDOUT_SELECT_MASK, TAS675X_SDOUT_SELECT_NON_TDM);
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            (*tas).bclk_offset = 0;
            regmap_update_bits((*component).regmap, TAS675X_AUDIO_IF_CTRL_REG, TAS675X_TDM_EN_BIT | TAS675X_SAP_FMT_MASK | TAS675X_FS_PULSE_MASK, TAS675X_SAP_FMT_RIGHT_J);
            regmap_update_bits((*component).regmap, TAS675X_SDOUT_CTRL_REG, TAS675X_SDOUT_SELECT_MASK, TAS675X_SDOUT_SELECT_NON_TDM);
        }
        SND_SOC_DAIFMT_LEFT_J => {
            (*tas).bclk_offset = 0;
            regmap_update_bits((*component).regmap, TAS675X_AUDIO_IF_CTRL_REG, TAS675X_TDM_EN_BIT | TAS675X_SAP_FMT_MASK | TAS675X_FS_PULSE_MASK, TAS675X_SAP_FMT_LEFT_J);
            regmap_update_bits((*component).regmap, TAS675X_SDOUT_CTRL_REG, TAS675X_SDOUT_SELECT_MASK, TAS675X_SDOUT_SELECT_NON_TDM);
        }
        SND_SOC_DAIFMT_DSP_A => {
            tdm_mode = true;
            (*tas).bclk_offset = 1;
            regmap_update_bits((*component).regmap, TAS675X_AUDIO_IF_CTRL_REG, TAS675X_TDM_EN_BIT | TAS675X_SAP_FMT_MASK | TAS675X_FS_PULSE_MASK, TAS675X_TDM_EN_BIT | TAS675X_SAP_FMT_TDM | TAS675X_FS_PULSE_SHORT);
            regmap_update_bits((*component).regmap, TAS675X_SDOUT_CTRL_REG, TAS675X_SDOUT_SELECT_MASK, TAS675X_SDOUT_SELECT_TDM_SDOUT1);
        }
        SND_SOC_DAIFMT_DSP_B => {
            tdm_mode = true;
            (*tas).bclk_offset = 0;
            regmap_update_bits((*component).regmap, TAS675X_AUDIO_IF_CTRL_REG, TAS675X_TDM_EN_BIT | TAS675X_SAP_FMT_MASK | TAS675X_FS_PULSE_MASK, TAS675X_TDM_EN_BIT | TAS675X_SAP_FMT_TDM | TAS675X_FS_PULSE_SHORT);
            regmap_update_bits((*component).regmap, TAS675X_SDOUT_CTRL_REG, TAS675X_SDOUT_SELECT_MASK, TAS675X_SDOUT_SELECT_TDM_SDOUT1);
        }
        _ => {
            dev_err((*component).dev, c"Unsupported DAI format\n".as_ptr());
            return -EINVAL;
        }
    }

    /* Setup Vpredict and Isense outputs */
    if (*dai).id == 2 {
        let mut sdout_en: c_uint = 0;
        if tdm_mode {
            /* TDM: Vpredict and Isense may coexist on separate slots */
            if (*tas).vpredict_slot >= 0 { sdout_en |= TAS675X_SDOUT_EN_VPREDICT; }
            if (*tas).isense_slot >= 0 { sdout_en |= TAS675X_SDOUT_EN_ISENSE; }
            regmap_update_bits((*component).regmap, TAS675X_SDOUT_EN_REG, TAS675X_SDOUT_EN_VPREDICT | TAS675X_SDOUT_EN_ISENSE, sdout_en);
            if (*tas).vpredict_slot >= 0 && (*tas).isense_slot >= 0 && abs((*tas).vpredict_slot - (*tas).isense_slot) < 4 {
                dev_warn((*component).dev, c"ti,vpredict-slot-no and ti,isense-slot-no overlaps (each occupies 4 consecutive slots)\n".as_ptr());
            }
        } else if i2s_mode {
            /* I2S: only one source at a time; Vpredict takes priority */
            if (*tas).vpredict_slot >= 0 {
                sdout_en = TAS675X_SDOUT_NON_TDM_SEL_VPREDICT | TAS675X_SDOUT_EN_NON_TDM_ALL;
            } else if (*tas).isense_slot >= 0 {
                sdout_en = TAS675X_SDOUT_NON_TDM_SEL_ISENSE | TAS675X_SDOUT_EN_NON_TDM_ALL;
            }
            regmap_update_bits((*component).regmap, TAS675X_SDOUT_EN_REG, TAS675X_SDOUT_NON_TDM_SEL_MASK | TAS675X_SDOUT_EN_NON_TDM_ALL, sdout_en);
            if sdout_en != 0 && (*tas).gpio1_func != TAS675X_GPIO_SEL_SDOUT2 && (*tas).gpio2_func != TAS675X_GPIO_SEL_SDOUT2 {
                dev_warn((*component).dev, c"sdout enabled in I2S mode but no GPIO configured as SDOUT2; Ch3/Ch4 will be absent\n".as_ptr());
            }
        }
    }

    0
}

unsafe fn tas675x_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, _rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let tas = snd_soc_component_get_drvdata((*dai).component) as *mut tas675x_priv;

    if slots == 0 {
        (*tas).slot_width = 0;
        (*tas).tx_mask = 0;
        return 0;
    }

    /* No rx_mask as hardware does not support channel muxing for capture */
    (*tas).slot_width = slot_width;
    (*tas).tx_mask = tx_mask;
    0
}

unsafe fn tas675x_mute_stream(dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int {
    let component = (*dai).component;
    let tas = snd_soc_component_get_drvdata(component) as *mut tas675x_priv;
    let mut discard: c_uint = 0;
    let ret: c_int;

    if direction == SNDRV_PCM_STREAM_CAPTURE {
        if mute != 0 { clear_bit((*dai).id as c_ulong, &mut (*tas).active_capture_dais); } else { set_bit((*dai).id as c_ulong, &mut (*tas).active_capture_dais); }
        return 0;
    }

    /*
     * Track which playback DAIs are active.
     * The TAS675x has two playback DAIs (main audio and LLP).
     * Only transition to SLEEP when ALL are muted.
     */
    if mute != 0 { clear_bit((*dai).id as c_ulong, &mut (*tas).active_playback_dais); } else { set_bit((*dai).id as c_ulong, &mut (*tas).active_playback_dais); }

    /* Last playback stream */
    if mute != 0 && READ_ONCE((*tas).active_playback_dais) == 0 {
        ret = tas675x_set_state_all(tas, TAS675X_STATE_SLEEP_BOTH as u8);
        regmap_read((*tas).regmap, TAS675X_CLK_FAULT_LATCHED_REG, &mut discard);
        return ret;
    }

    tas675x_set_state_all(tas, if READ_ONCE((*tas).active_playback_dais) != 0 { TAS675X_STATE_PLAY_BOTH as u8 } else { TAS675X_STATE_SLEEP_BOTH as u8 })
}

static tas675x_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tas675x_hw_params),
    set_fmt: Some(tas675x_set_fmt),
    set_tdm_slot: Some(tas675x_set_tdm_slot),
    mute_stream: Some(tas675x_mute_stream),
    ..ZEROED
};

static mut tas675x_dais: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver { name: c"tas675x-audio".as_ptr(), id: 0, playback: snd_soc_pcm_stream { stream_name: c"Playback".as_ptr(), channels_min: 2, channels_max: 4, rates: SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE, ..ZEROED }, ops: &tas675x_dai_ops, ..ZEROED },
    /* Only available when Low Latency Path (LLP) is enabled */
    snd_soc_dai_driver { name: c"tas675x-anc".as_ptr(), id: 1, playback: snd_soc_pcm_stream { stream_name: c"ANC Playback".as_ptr(), channels_min: 2, channels_max: 4, rates: SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE, ..ZEROED }, ops: &tas675x_dai_ops, ..ZEROED },
    snd_soc_dai_driver { name: c"tas675x-feedback".as_ptr(), id: 2, capture: snd_soc_pcm_stream { stream_name: c"Feedback Capture".as_ptr(), channels_min: 2, channels_max: 8, rates: SNDRV_PCM_RATE_48000, formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE, ..ZEROED }, ops: &tas675x_dai_ops, ..ZEROED },
];

/*
 * Enable regulators and release hardware reset GPIOs.
 * The device is not I2C-accessible until this returns.
 */
unsafe fn tas675x_hw_enable(tas: *mut tas675x_priv) -> c_int {
    let mut ret: c_int;

    ret = regulator_bulk_enable((*tas).supplies.len(), (*tas).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*tas).dev, c"Failed to enable regulators: %d\n".as_ptr(), ret);
        return ret;
    }

    if !IS_ERR((*tas).vbat as *const c_void) {
        ret = regulator_enable((*tas).vbat);
        if ret != 0 {
            dev_err((*tas).dev, c"Failed to enable vbat: %d\n".as_ptr(), ret);
            regulator_bulk_disable((*tas).supplies.len(), (*tas).supplies.as_mut_ptr());
            return ret;
        }
    }

    if !(*tas).pd_gpio.is_null() && !(*tas).stby_gpio.is_null() {
        /*
         * Independent Pin Control
         * Deassert PD first to boot digital, then STBY for analog.
         */
        /* Min 4ms digital boot wait */
        gpiod_set_value_cansleep((*tas).pd_gpio, 0);
        usleep_range(4000, 5000);
        /* ~2ms analog stabilization */
        gpiod_set_value_cansleep((*tas).stby_gpio, 0);
        usleep_range(2000, 3000);
    } else if !(*tas).pd_gpio.is_null() {
        /*
         * Simultaneous Pin Release
         * STBY tied to PD or hardwired HIGH.
         */
        /* 6ms wait for simultaneous release transition */
        gpiod_set_value_cansleep((*tas).pd_gpio, 0);
        usleep_range(6000, 7000);
    } else {
        /*
         * PD hardwired, device in DEEP_SLEEP.
         * Digital core already booted, I2C active. Deassert STBY
         * to bring up the analog output stage.
         */
        /* ~2ms analog stabilization */
        gpiod_set_value_cansleep((*tas).stby_gpio, 0);
        usleep_range(2000, 3000);
    }

    0
}

unsafe fn tas675x_hw_disable(tas: *mut tas675x_priv) {
    if !(*tas).stby_gpio.is_null() { gpiod_set_value_cansleep((*tas).stby_gpio, 1); }
    if !(*tas).pd_gpio.is_null() { gpiod_set_value_cansleep((*tas).pd_gpio, 1); }
    /*
     * Hold PD/STBY asserted for at least 10ms
     * before removing PVDD, VBAT or DVDD.
     */
    usleep_range(10000, 11000);
    if !IS_ERR((*tas).vbat as *const c_void) { regulator_disable((*tas).vbat); }
    regulator_bulk_disable((*tas).supplies.len(), (*tas).supplies.as_mut_ptr());
}

/*
 * Write device start-up defaults.
 * Must be called after tas675x_hw_enable() and after regcache is enabled.
 */
unsafe fn tas675x_init_device(tas: *mut tas675x_priv) -> c_int {
    let regmap = (*tas).regmap;
    let mut val: c_uint = 0;
    let mut ret: c_int;
    let mut i: usize;

    /* Clear POR fault flag to prevent IRQ storm */
    regmap_read(regmap, TAS675X_POWER_FAULT_LATCHED_REG, &mut val);

    /* Bypass DC Load Diagnostics for fast boot */
    if (*tas).fast_boot {
        regmap_update_bits(regmap, TAS675X_DC_LDG_CTRL_REG, TAS675X_LDG_ABORT_BIT | TAS675X_LDG_BYPASS_BIT, TAS675X_LDG_ABORT_BIT | TAS675X_LDG_BYPASS_BIT);
    }

    tas675x_select_book(regmap, TAS675X_BOOK_DEFAULT as u8);

    /* Enter setup mode */
    ret = regmap_write(regmap, TAS675X_SETUP_REG1, TAS675X_SETUP_ENTER_VAL1);
    if ret != 0 { dev_err((*tas).dev, c"Init device failed: %d\n".as_ptr(), ret); return ret; }
    ret = regmap_write(regmap, TAS675X_SETUP_REG2, TAS675X_SETUP_ENTER_VAL2);
    if ret != 0 { dev_err((*tas).dev, c"Init device failed: %d\n".as_ptr(), ret); return ret; }

    /* Set all channels to Sleep (required before Page 1 config) */
    tas675x_set_state_all(tas, TAS675X_STATE_SLEEP_BOTH as u8);
    /* Set DAC clock per TRM startup script */
    regmap_write(regmap, TAS675X_DAC_CLK_REG, 0x00);

    /*
     * Switch to Page 1 for safety-critical OC/CBC configuration,
     * while bypassing regcache. (Page 1 not accessible post setup)
     */
    regcache_cache_bypass(regmap, true);
    ret = regmap_multi_reg_write(regmap, tas675x_page1_init.as_ptr(), tas675x_page1_init.len());
    regcache_cache_bypass(regmap, false);
    if ret != 0 {
        regmap_write(regmap, TAS675X_SETUP_REG1, TAS675X_SETUP_EXIT_VAL);
        regmap_write(regmap, TAS675X_SETUP_REG2, TAS675X_SETUP_EXIT_VAL);
        dev_err((*tas).dev, c"Init device failed: %d\n".as_ptr(), ret);
        return ret;
    }

    /* Resync regmap's cached page selector */
    regmap_write(regmap, TAS675X_PAGE_CTRL_REG, 0x00);
    /* Exit setup mode */
    regmap_write(regmap, TAS675X_SETUP_REG1, TAS675X_SETUP_EXIT_VAL);
    regmap_write(regmap, TAS675X_SETUP_REG2, TAS675X_SETUP_EXIT_VAL);

    /* Write DSP parameters if cached */
    i = 0;
    while i < (*tas).dsp_params.len() {
        if (*tas).dsp_params[i].val != 0 {
            tas675x_dsp_mem_write(tas, (*tas).dsp_params[i].page, (*tas).dsp_params[i].reg, (*tas).dsp_params[i].val);
        }
        i += 1;
    }

    /*
     * Configure fault and warning event routing:
     *
     * ROUTING_1: CP fault/UVLO latch, OUTM soft short latch
     * ROUTING_2: CBC latch, OTSD latch, OTSD, power fault
     * ROUTING_3: CBC latch, OTSD latch, power latch, DC LDG,
     *            OTSD, power warnings
     * ROUTING_4: OC latch, DC latch, protection shutdown
     *            OTW latch, OTW, clip latch
     * ROUTING_5: clock latch+non-latch, RTLDG latch
     *            CBC warning, clip warning
     */
    regmap_write(regmap, TAS675X_REPORT_ROUTING_1_REG, 0x70);
    regmap_write(regmap, TAS675X_REPORT_ROUTING_2_REG, 0xA3);
    regmap_write(regmap, TAS675X_REPORT_ROUTING_3_REG, 0xBB);
    regmap_write(regmap, TAS675X_REPORT_ROUTING_4_REG, 0x7E);
    regmap_write(regmap, TAS675X_REPORT_ROUTING_5_REG, 0xF3);

    /* Configure GPIO pins if specified in DT */
    if (*tas).gpio1_func >= 0 || (*tas).gpio2_func >= 0 {
        let mut gpio_ctrl: c_uint = TAS675X_GPIO_CTRL_RSTVAL;
        tas675x_config_gpio_pin(regmap, (*tas).gpio1_func, TAS675X_GPIO1_OUTPUT_SEL_REG, 0, &mut gpio_ctrl);
        tas675x_config_gpio_pin(regmap, (*tas).gpio2_func, TAS675X_GPIO2_OUTPUT_SEL_REG, 1, &mut gpio_ctrl);
        regmap_write(regmap, TAS675X_GPIO_CTRL_REG, gpio_ctrl);
    }

    /* Clear fast boot bits */
    if (*tas).fast_boot {
        regmap_update_bits(regmap, TAS675X_DC_LDG_CTRL_REG, TAS675X_LDG_ABORT_BIT | TAS675X_LDG_BYPASS_BIT, 0);
    }

    /* Clear any stale faults from the boot sequence */
    regmap_read(regmap, TAS675X_POWER_FAULT_STATUS_1_REG, &mut val);
    regmap_read(regmap, TAS675X_POWER_FAULT_LATCHED_REG, &mut val);
    regmap_read(regmap, TAS675X_CLK_FAULT_LATCHED_REG, &mut val);
    regmap_write(regmap, TAS675X_RESET_REG, TAS675X_FAULT_CLEAR);

    0
}

unsafe fn tas675x_power_off(tas: *mut tas675x_priv) {
    regcache_cache_only((*tas).regmap, true);
    regcache_mark_dirty((*tas).regmap);
    tas675x_hw_disable(tas);
}

unsafe fn tas675x_power_on(tas: *mut tas675x_priv) -> c_int {
    let mut ret: c_int;

    ret = tas675x_hw_enable(tas);
    if ret != 0 { return ret; }
    regcache_cache_only((*tas).regmap, false);
    regcache_mark_dirty((*tas).regmap);
    ret = tas675x_init_device(tas);
    if ret != 0 {
        tas675x_power_off(tas);
        return ret;
    }
    ret = regcache_sync((*tas).regmap);
    if ret != 0 {
        dev_err((*tas).dev, c"Failed to sync regcache: %d\n".as_ptr(), ret);
        tas675x_power_off(tas);
        return ret;
    }
    /* Reset fault tracking */
    memset((*tas).last_status.as_mut_ptr() as *mut c_void, 0, size_of::<[c_uint; TAS675X_FAULT_REGS_NUM]>());
    0
}

unsafe fn tas675x_runtime_suspend(dev: *mut device) -> c_int {
    let tas = dev_get_drvdata(dev) as *mut tas675x_priv;
    disable_delayed_work_sync(&mut (*tas).fault_check_work);
    tas675x_set_state_all(tas, TAS675X_STATE_SLEEP_BOTH as u8);
    0
}

unsafe fn tas675x_runtime_resume(dev: *mut device) -> c_int {
    let tas = dev_get_drvdata(dev) as *mut tas675x_priv;
    tas675x_set_state_all(tas, TAS675X_STATE_SLEEP_BOTH as u8);
    if (*to_i2c_client(dev)).irq == 0 {
        enable_delayed_work(&mut (*tas).fault_check_work);
        schedule_delayed_work(&mut (*tas).fault_check_work, msecs_to_jiffies(TAS675X_FAULT_CHECK_INTERVAL_MS));
    }
    0
}

unsafe fn tas675x_system_suspend(dev: *mut device) -> c_int {
    let tas = dev_get_drvdata(dev) as *mut tas675x_priv;
    let ret = tas675x_runtime_suspend(dev);
    if ret != 0 { return ret; }
    if (*to_i2c_client(dev)).irq != 0 { disable_irq((*to_i2c_client(dev)).irq); }
    tas675x_power_off(tas);
    0
}

unsafe fn tas675x_system_resume(dev: *mut device) -> c_int {
    let tas = dev_get_drvdata(dev) as *mut tas675x_priv;
    let ret = tas675x_power_on(tas);
    if ret != 0 { return ret; }
    if (*to_i2c_client(dev)).irq != 0 { enable_irq((*to_i2c_client(dev)).irq); }
    tas675x_runtime_resume(dev)
}

static soc_codec_dev_tas675x: snd_soc_component_driver = snd_soc_component_driver {
    controls: tas675x_snd_controls.as_ptr(),
    num_controls: tas675x_snd_controls.len(),
    dapm_widgets: tas675x_dapm_widgets.as_ptr(),
    num_dapm_widgets: tas675x_dapm_widgets.len(),
    dapm_routes: tas675x_dapm_routes.as_ptr(),
    num_dapm_routes: tas675x_dapm_routes.len(),
    endianness: 1,
    ..ZEROED
};

static tas675x_fault_table: [tas675x_fault_reg; TAS675X_FAULT_REGS_NUM] = [
    /* Critical */
    tas675x_fault_reg { reg: TAS675X_OTSD_LATCHED_REG, flags: TAS675X_FAULT_CRITICAL | TAS675X_FAULT_TRACK, name: c"Overtemperature Shutdown".as_ptr() },
    tas675x_fault_reg { reg: TAS675X_OC_DC_FAULT_LATCHED_REG, flags: TAS675X_FAULT_CRITICAL | TAS675X_FAULT_TRACK, name: c"Overcurrent / DC Fault".as_ptr() },
    tas675x_fault_reg { reg: TAS675X_RTLDG_OL_SL_FAULT_LATCHED_REG, flags: TAS675X_FAULT_CRITICAL | TAS675X_FAULT_TRACK, name: c"Real-Time Load Diagnostic Fault".as_ptr() },
    tas675x_fault_reg { reg: TAS675X_CBC_FAULT_WARN_LATCHED_REG, flags: TAS675X_FAULT_CRITICAL | TAS675X_FAULT_TRACK, name: c"CBC Fault/Warning".as_ptr() },
    /* Warning */
    tas675x_fault_reg { reg: TAS675X_POWER_FAULT_STATUS_1_REG, flags: TAS675X_FAULT_TRACK, name: c"CP / OUTM Fault".as_ptr() },
    tas675x_fault_reg { reg: TAS675X_POWER_FAULT_LATCHED_REG, flags: TAS675X_FAULT_TRACK, name: c"Power Fault".as_ptr() },
    tas675x_fault_reg { reg: TAS675X_CLK_FAULT_LATCHED_REG, flags: TAS675X_FAULT_TRACK | TAS675X_FAULT_ACTIVE, name: c"Clock Fault".as_ptr() },
    tas675x_fault_reg { reg: TAS675X_OTW_LATCHED_REG, flags: TAS675X_FAULT_TRACK, name: c"Overtemperature Warning".as_ptr() },
    tas675x_fault_reg { reg: TAS675X_CLIP_WARN_LATCHED_REG, flags: TAS675X_FAULT_ACTIVE, name: c"Clip Warning".as_ptr() },
];

const _: () = assert!(tas675x_fault_table.len() == TAS675X_FAULT_REGS_NUM);

/*
 * Read and log all latched fault registers.
 * Shared by both the polled fault_check_work and IRQ handler paths
 * (which are mutually exclusive, only one is active per device).
 * Returns true if any fault register needs to be cleared.
 *
 * For deciphering fault messages, see "Fault Monitoring" in
 * Documentation/sound/codecs/tas675x.rst
 */
unsafe fn tas675x_check_faults(tas: *mut tas675x_priv) -> bool {
    let dev = (*tas).dev;
    let mut needs_clear = false;
    let mut reg: c_uint = 0;
    let mut i: usize = 0;

    while i < tas675x_fault_table.len() {
        let f = &tas675x_fault_table[i];
        let ret = regmap_read((*tas).regmap, f.reg, &mut reg);
        if ret != 0 {
            if (f.flags & TAS675X_FAULT_CRITICAL) != 0 {
                dev_err(dev, c"failed to read %s: %d\n".as_ptr(), f.name, ret);
                return needs_clear;
            }
            i += 1;
            continue;
        }

        if reg != 0 { needs_clear = true; }

        /* Skip logging stream-dependent events when no stream is active */
        if (f.flags & TAS675X_FAULT_ACTIVE) != 0 && READ_ONCE((*tas).active_playback_dais) == 0 && READ_ONCE((*tas).active_capture_dais) == 0 {
            i += 1;
            continue;
        }

        /* Log on change or on every non-zero read */
        if reg != 0 && ((f.flags & TAS675X_FAULT_TRACK) == 0 || reg != (*tas).last_status[i]) {
            if (f.flags & TAS675X_FAULT_CRITICAL) != 0 {
                dev_crit(dev, c"%s Latched: 0x%02x\n".as_ptr(), f.name, reg);
            } else {
                dev_warn(dev, c"%s Latched: 0x%02x\n".as_ptr(), f.name, reg);
            }
        }

        if (f.flags & TAS675X_FAULT_TRACK) != 0 {
            (*tas).last_status[i] = reg;
        }
        i += 1;
    }

    needs_clear
}

unsafe fn tas675x_fault_check_work(work: *mut work_struct) {
    let tas = container_of!(work, tas675x_priv, fault_check_work.work);
    if tas675x_check_faults(tas) {
        regmap_write((*tas).regmap, TAS675X_RESET_REG, TAS675X_FAULT_CLEAR);
    }
    schedule_delayed_work(&mut (*tas).fault_check_work, msecs_to_jiffies(TAS675X_FAULT_CHECK_INTERVAL_MS));
}

unsafe fn tas675x_irq_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let tas = data as *mut tas675x_priv;
    let mut ret: irqreturn_t = IRQ_NONE;

    if pm_runtime_resume_and_get((*tas).dev) < 0 {
        return IRQ_NONE;
    }

    if tas675x_check_faults(tas) {
        regmap_write((*tas).regmap, TAS675X_RESET_REG, TAS675X_FAULT_CLEAR);
        ret = IRQ_HANDLED;
    }

    pm_runtime_mark_last_busy((*tas).dev);
    pm_runtime_put_autosuspend((*tas).dev);
    ret
}

static tas675x_reg_defaults: &[reg_default] = &reg_defaults![
    { TAS675X_PAGE_CTRL_REG, 0x00 }, { TAS675X_OUTPUT_CTRL_REG, 0x00 },
    { TAS675X_STATE_CTRL_CH1_CH2_REG, TAS675X_STATE_SLEEP_BOTH },
    { TAS675X_STATE_CTRL_CH3_CH4_REG, TAS675X_STATE_SLEEP_BOTH },
    { TAS675X_ISENSE_CTRL_REG, 0x0F }, { TAS675X_DC_DETECT_CTRL_REG, 0x00 },
    { TAS675X_SCLK_INV_CTRL_REG, 0x00 }, { TAS675X_AUDIO_IF_CTRL_REG, 0x00 },
    { TAS675X_SDIN_CTRL_REG, 0x0A }, { TAS675X_SDOUT_CTRL_REG, 0x1A },
    { TAS675X_SDIN_OFFSET_MSB_REG, 0x00 }, { TAS675X_SDIN_AUDIO_OFFSET_REG, 0x00 },
    { TAS675X_SDIN_LL_OFFSET_REG, 0x60 }, { TAS675X_SDIN_CH_SWAP_REG, 0x00 },
    { TAS675X_SDOUT_OFFSET_MSB_REG, 0xCF }, { TAS675X_VPREDICT_OFFSET_REG, 0xFF },
    { TAS675X_ISENSE_OFFSET_REG, 0x00 }, { TAS675X_SDOUT_EN_REG, 0x00 },
    { TAS675X_LL_EN_REG, 0x00 }, { TAS675X_RTLDG_EN_REG, 0x10 },
    { TAS675X_DC_BLOCK_BYP_REG, 0x00 }, { TAS675X_DSP_CTRL_REG, 0x00 },
    { TAS675X_PAGE_AUTO_INC_REG, 0x00 }, { TAS675X_DIG_VOL_CH1_REG, 0x30 },
    { TAS675X_DIG_VOL_CH2_REG, 0x30 }, { TAS675X_DIG_VOL_CH3_REG, 0x30 },
    { TAS675X_DIG_VOL_CH4_REG, 0x30 }, { TAS675X_DIG_VOL_RAMP_CTRL_REG, 0x77 },
    { TAS675X_DIG_VOL_COMBINE_CTRL_REG, 0x00 }, { TAS675X_AUTO_MUTE_EN_REG, 0x00 },
    { TAS675X_AUTO_MUTE_TIMING_CH1_CH2_REG, 0x00 }, { TAS675X_AUTO_MUTE_TIMING_CH3_CH4_REG, 0x00 },
    { TAS675X_ANALOG_GAIN_CH1_CH2_REG, 0x00 }, { TAS675X_ANALOG_GAIN_CH3_CH4_REG, 0x00 },
    { TAS675X_ANALOG_GAIN_RAMP_CTRL_REG, 0x00 }, { TAS675X_PULSE_INJECTION_EN_REG, 0x03 },
    { TAS675X_CBC_CTRL_REG, 0x07 }, { TAS675X_CURRENT_LIMIT_CTRL_REG, 0x00 },
    { TAS675X_ISENSE_CAL_REG, 0x00 }, { TAS675X_PWM_PHASE_CTRL_REG, 0x00 },
    { TAS675X_SS_CTRL_REG, 0x00 }, { TAS675X_SS_RANGE_CTRL_REG, 0x00 },
    { TAS675X_SS_DWELL_CTRL_REG, 0x00 }, { TAS675X_RAMP_PHASE_CTRL_GPO_REG, 0x00 },
    { TAS675X_PWM_PHASE_M_CTRL_CH1_REG, 0x00 }, { TAS675X_PWM_PHASE_M_CTRL_CH2_REG, 0x00 },
    { TAS675X_PWM_PHASE_M_CTRL_CH3_REG, 0x00 }, { TAS675X_PWM_PHASE_M_CTRL_CH4_REG, 0x00 },
    { TAS675X_REPORT_ROUTING_1_REG, 0x00 }, { TAS675X_OTSD_RECOVERY_EN_REG, 0x00 },
    { TAS675X_REPORT_ROUTING_2_REG, 0xA2 }, { TAS675X_REPORT_ROUTING_3_REG, 0x00 },
    { TAS675X_REPORT_ROUTING_4_REG, 0x06 }, { TAS675X_CLIP_DETECT_CTRL_REG, 0x00 },
    { TAS675X_REPORT_ROUTING_5_REG, 0x00 }, { TAS675X_GPIO1_OUTPUT_SEL_REG, 0x00 },
    { TAS675X_GPIO2_OUTPUT_SEL_REG, 0x00 }, { TAS675X_GPIO_CTRL_REG, TAS675X_GPIO_CTRL_RSTVAL },
    { TAS675X_DC_LDG_CTRL_REG, 0x00 }, { TAS675X_DC_LDG_LO_CTRL_REG, 0x00 },
    { TAS675X_DC_LDG_TIME_CTRL_REG, 0x00 }, { TAS675X_DC_LDG_SL_CH1_CH2_CTRL_REG, 0x11 },
    { TAS675X_DC_LDG_SL_CH3_CH4_CTRL_REG, 0x11 }, { TAS675X_AC_LDG_CTRL_REG, 0x10 },
    { TAS675X_TWEETER_DETECT_CTRL_REG, 0x08 }, { TAS675X_TWEETER_DETECT_THRESH_REG, 0x00 },
    { TAS675X_AC_LDG_FREQ_CTRL_REG, 0xC8 }, { TAS675X_OTW_CTRL_CH1_CH2_REG, 0x11 },
    { TAS675X_OTW_CTRL_CH3_CH4_REG, 0x11 },
];

unsafe fn tas675x_is_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TAS675X_RESET_REG => false,
        _ => true,
    }
}

unsafe fn tas675x_is_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TAS675X_RESET_REG | TAS675X_BOOK_CTRL_REG | TAS675X_AUTO_MUTE_STATUS_REG |
        TAS675X_STATE_REPORT_CH1_CH2_REG | TAS675X_STATE_REPORT_CH3_CH4_REG |
        TAS675X_PVDD_SENSE_REG | TAS675X_TEMP_GLOBAL_REG | TAS675X_TEMP_CH1_CH2_REG |
        TAS675X_TEMP_CH3_CH4_REG | TAS675X_FS_MON_REG | TAS675X_SCLK_MON_REG |
        TAS675X_POWER_FAULT_STATUS_1_REG | TAS675X_POWER_FAULT_STATUS_2_REG |
        TAS675X_OT_FAULT_REG | TAS675X_OTW_STATUS_REG | TAS675X_CLIP_WARN_STATUS_REG |
        TAS675X_CBC_WARNING_STATUS_REG | TAS675X_POWER_FAULT_LATCHED_REG |
        TAS675X_OTSD_LATCHED_REG | TAS675X_OTW_LATCHED_REG |
        TAS675X_CLIP_WARN_LATCHED_REG | TAS675X_CLK_FAULT_LATCHED_REG |
        TAS675X_RTLDG_OL_SL_FAULT_LATCHED_REG | TAS675X_CBC_FAULT_WARN_LATCHED_REG |
        TAS675X_OC_DC_FAULT_LATCHED_REG | TAS675X_WARN_OT_MAX_FLAG_REG => true,
        _ if reg >= TAS675X_DC_LDG_REPORT_CH1_CH2_REG && reg <= TAS675X_TWEETER_REPORT_REG => true,
        _ if reg >= TAS675X_CH1_RTLDG_IMP_MSB_REG && reg <= TAS675X_CH4_DC_LDG_DCR_LSB_REG => true,
        _ => false,
    }
}

static tas675x_ranges: [regmap_range_cfg; 1] = [
    regmap_range_cfg {
        name: c"Pages".as_ptr(),
        range_min: 0,
        range_max: TAS675X_PAGE_SIZE * TAS675X_PAGE_SIZE - 1,
        selector_reg: TAS675X_PAGE_CTRL_REG,
        selector_mask: 0xff,
        selector_shift: 0,
        window_start: 0,
        window_len: TAS675X_PAGE_SIZE,
    },
];

unsafe fn tas675x_regmap_lock(lock_arg: *mut c_void) {
    let tas = lock_arg as *mut tas675x_priv;
    mutex_lock(&mut (*tas).io_lock);
}

unsafe fn tas675x_regmap_unlock(lock_arg: *mut c_void) {
    let tas = lock_arg as *mut tas675x_priv;
    mutex_unlock(&mut (*tas).io_lock);
}

static tas675x_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: TAS675X_PAGE_SIZE * TAS675X_PAGE_SIZE - 1,
    ranges: tas675x_ranges.as_ptr(),
    num_ranges: tas675x_ranges.len(),
    cache_type: REGCACHE_MAPLE,
    reg_defaults: tas675x_reg_defaults.as_ptr(),
    num_reg_defaults: tas675x_reg_defaults.len(),
    readable_reg: Some(tas675x_is_readable_register),
    volatile_reg: Some(tas675x_is_volatile_register),
    ..ZEROED
};

unsafe fn tas675x_i2c_probe(client: *mut i2c_client) -> c_int {
    let mut cfg = tas675x_regmap_config;
    let tas: *mut tas675x_priv;
    let mut val: u32 = 0;
    let mut i: usize;
    let mut ret: c_int;

    tas = devm_kzalloc(&mut (*client).dev, size_of::<tas675x_priv>(), GFP_KERNEL) as *mut tas675x_priv;
    if tas.is_null() {
        return -ENOMEM;
    }

    (*tas).dev = &mut (*client).dev;
    i2c_set_clientdata(client, tas as *mut c_void);

    mutex_init(&mut (*tas).io_lock);
    cfg.lock = Some(tas675x_regmap_lock);
    cfg.unlock = Some(tas675x_regmap_unlock);
    cfg.lock_arg = tas as *mut c_void;

    memcpy((*tas).dsp_params.as_mut_ptr() as *mut c_void, tas675x_dsp_defaults.as_ptr() as *const c_void, size_of::<[tas675x_reg_param; TAS675X_DSP_PARAM_NUM]>());
    INIT_DELAYED_WORK(&mut (*tas).fault_check_work, Some(tas675x_fault_check_work));

    (*tas).regmap = devm_regmap_init_i2c(client, &cfg);
    if IS_ERR((*tas).regmap as *const c_void) {
        return PTR_ERR((*tas).regmap as *const c_void);
    }

    /* Keep regmap cache-only until hardware is powered on */
    regcache_cache_only((*tas).regmap, true);

    (*tas).dev_type = core::mem::transmute::<c_ulong, tas675x_type>(device_get_match_data((*tas).dev) as c_ulong);
    (*tas).fast_boot = device_property_read_bool((*tas).dev, c"ti,fast-boot".as_ptr());

    (*tas).audio_slot = -1;
    (*tas).llp_slot = -1;
    (*tas).vpredict_slot = -1;
    (*tas).isense_slot = -1;
    if device_property_read_u32((*tas).dev, c"ti,audio-slot-no".as_ptr(), &mut val) == 0 { (*tas).audio_slot = val as c_int; }
    if device_property_read_u32((*tas).dev, c"ti,llp-slot-no".as_ptr(), &mut val) == 0 { (*tas).llp_slot = val as c_int; }
    if device_property_read_u32((*tas).dev, c"ti,vpredict-slot-no".as_ptr(), &mut val) == 0 { (*tas).vpredict_slot = val as c_int; }
    if device_property_read_u32((*tas).dev, c"ti,isense-slot-no".as_ptr(), &mut val) == 0 { (*tas).isense_slot = val as c_int; }

    (*tas).gpio1_func = tas675x_gpio_func_parse((*tas).dev, c"ti,gpio1-function".as_ptr());
    (*tas).gpio2_func = tas675x_gpio_func_parse((*tas).dev, c"ti,gpio2-function".as_ptr());

    i = 0;
    while i < tas675x_supply_names.len() {
        (*tas).supplies[i].supply = tas675x_supply_names[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get((*tas).dev, (*tas).supplies.len(), (*tas).supplies.as_mut_ptr());
    if ret != 0 { return dev_err_probe((*tas).dev, ret, c"Failed to request supplies\n".as_ptr()); }

    (*tas).vbat = devm_regulator_get_optional((*tas).dev, c"vbat".as_ptr());
    if IS_ERR((*tas).vbat as *const c_void) && PTR_ERR((*tas).vbat as *const c_void) != -ENODEV {
        return dev_err_probe((*tas).dev, PTR_ERR((*tas).vbat as *const c_void), c"Failed to get vbat supply\n".as_ptr());
    }

    (*tas).pd_gpio = devm_gpiod_get_optional((*tas).dev, c"powerdown".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*tas).pd_gpio as *const c_void) {
        return dev_err_probe((*tas).dev, PTR_ERR((*tas).pd_gpio as *const c_void), c"Failed powerdown-gpios\n".as_ptr());
    }

    (*tas).stby_gpio = devm_gpiod_get_optional((*tas).dev, c"standby".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*tas).stby_gpio as *const c_void) {
        return dev_err_probe((*tas).dev, PTR_ERR((*tas).stby_gpio as *const c_void), c"Failed standby-gpios\n".as_ptr());
    }

    if (*tas).pd_gpio.is_null() && (*tas).stby_gpio.is_null() {
        return dev_err_probe((*tas).dev, -EINVAL, c"At least one of powerdown-gpios or standby-gpios is required\n".as_ptr());
    }

    ret = tas675x_power_on(tas);
    if ret != 0 { return ret; }

    if (*client).irq != 0 {
        ret = devm_request_threaded_irq((*tas).dev, (*client).irq, None, Some(tas675x_irq_handler), IRQF_ONESHOT | IRQF_TRIGGER_FALLING, c"tas675x-fault".as_ptr(), tas as *mut c_void);
        if ret != 0 {
            tas675x_power_off(tas);
            return dev_err_probe((*tas).dev, ret, c"Failed to request IRQ\n".as_ptr());
        }
    } else {
        /* Schedule delayed work for fault checking at probe and runtime resume */
        schedule_delayed_work(&mut (*tas).fault_check_work, msecs_to_jiffies(TAS675X_FAULT_CHECK_INTERVAL_MS));
    }

    /* Enable runtime PM with 2s autosuspend */
    pm_runtime_set_autosuspend_delay((*tas).dev, 2000);
    pm_runtime_use_autosuspend((*tas).dev);
    pm_runtime_set_active((*tas).dev);
    pm_runtime_mark_last_busy((*tas).dev);
    pm_runtime_enable((*tas).dev);

    ret = devm_snd_soc_register_component((*tas).dev, &soc_codec_dev_tas675x, tas675x_dais.as_mut_ptr(), tas675x_dais.len());
    if ret != 0 {
        pm_runtime_force_suspend((*tas).dev);
        pm_runtime_disable((*tas).dev);
        tas675x_power_off(tas);
        return ret;
    }

    0
}

unsafe fn tas675x_i2c_remove(client: *mut i2c_client) {
    let tas = dev_get_drvdata(&mut (*client).dev) as *mut tas675x_priv;

    disable_delayed_work_sync(&mut (*tas).fault_check_work);
    if (*client).irq != 0 { disable_irq((*client).irq); }

    pm_runtime_force_suspend(&mut (*client).dev);
    pm_runtime_disable(&mut (*client).dev);
    tas675x_power_off(tas);
}

static tas675x_pm_ops: dev_pm_ops = dev_pm_ops {
    system_sleep: SYSTEM_SLEEP_PM_OPS!(tas675x_system_suspend, tas675x_system_resume),
    runtime: RUNTIME_PM_OPS!(tas675x_runtime_suspend, tas675x_runtime_resume, None),
    ..ZEROED
};

static tas675x_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"ti,tas67524".as_ptr(), data: tas675x_type::TAS67524 as c_ulong as *mut c_void, ..ZEROED },
    of_device_id { ..ZEROED },
];
MODULE_DEVICE_TABLE!(of, tas675x_of_match);

static tas675x_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: c"tas67524".as_ptr(), driver_data: tas675x_type::TAS67524 as c_ulong, ..ZEROED },
    i2c_device_id { ..ZEROED },
];
MODULE_DEVICE_TABLE!(i2c, tas675x_i2c_id);

static mut tas675x_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"tas675x".as_ptr(),
        of_match_table: tas675x_of_match.as_ptr(),
        pm: pm_ptr(&tas675x_pm_ops),
        ..ZEROED
    },
    probe: Some(tas675x_i2c_probe),
    remove: Some(tas675x_i2c_remove),
    id_table: tas675x_i2c_id.as_ptr(),
    ..ZEROED
};

module_i2c_driver!(tas675x_i2c_driver);

MODULE_AUTHOR!("Sen Wang <sen@ti.com>");
MODULE_DESCRIPTION!("ASoC TAS675x Audio Amplifier Driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
