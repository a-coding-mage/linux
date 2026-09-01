// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2015-2016, The Linux Foundation. All rights reserved.
// Copyright (c) 2017-2018, Linaro Limited

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct wcd_clsh_ctrl {
    state: c_int,
    mode: c_int,
    flyback_users: c_int,
    buck_users: c_int,
    clsh_users: c_int,
    codec_version: c_int,
    comp: *mut snd_soc_component,
}

unsafe extern "C" {
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        value: c_uint,
    ) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

const fn WCD9335_REG(page: c_uint, reg: c_uint) -> c_uint {
    0x3000 + (page << 8) + reg
}

fn ERR_PTR(error: c_int) -> *mut wcd_clsh_ctrl {
    error as isize as *mut wcd_clsh_ctrl
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    unsafe { kzalloc(core::mem::size_of::<T>(), GFP_KERNEL) as *mut T }
}

/* Class-H registers for codecs from and above WCD9335 */
const WCD9XXX_A_CDC_RX0_RX_PATH_CFG0: c_uint = WCD9335_REG(0xB, 0x42);
const WCD9XXX_A_CDC_RX_PATH_CLSH_EN_MASK: c_uint = BIT(6);
const WCD9XXX_A_CDC_RX_PATH_CLSH_ENABLE: c_uint = BIT(6);
const WCD9XXX_A_CDC_RX_PATH_CLSH_DISABLE: c_uint = 0;
const WCD9XXX_A_CDC_RX1_RX_PATH_CFG0: c_uint = WCD9335_REG(0xB, 0x56);
const WCD9XXX_A_CDC_RX2_RX_PATH_CFG0: c_uint = WCD9335_REG(0xB, 0x6A);
const WCD9XXX_A_CDC_CLSH_K1_MSB: c_uint = WCD9335_REG(0xC, 0x08);
const WCD9XXX_A_CDC_CLSH_K1_MSB_COEF_MASK: c_uint = GENMASK(3, 0);
const WCD9XXX_A_CDC_CLSH_K1_LSB: c_uint = WCD9335_REG(0xC, 0x09);
const WCD9XXX_A_CDC_CLSH_K1_LSB_COEF_MASK: c_uint = GENMASK(7, 0);
const WCD9XXX_A_ANA_RX_SUPPLIES: c_uint = WCD9335_REG(0x6, 0x08);
const WCD9XXX_A_ANA_RX_REGULATOR_MODE_MASK: c_uint = BIT(1);
const WCD9XXX_A_ANA_RX_REGULATOR_MODE_CLS_H: c_uint = 0;
const WCD9XXX_A_ANA_RX_REGULATOR_MODE_CLS_AB: c_uint = BIT(1);
const WCD9XXX_A_ANA_RX_VNEG_PWR_LVL_MASK: c_uint = BIT(2);
const WCD9XXX_A_ANA_RX_VNEG_PWR_LVL_UHQA: c_uint = BIT(2);
const WCD9XXX_A_ANA_RX_VNEG_PWR_LVL_DEFAULT: c_uint = 0;
const WCD9XXX_A_ANA_RX_VPOS_PWR_LVL_MASK: c_uint = BIT(3);
const WCD9XXX_A_ANA_RX_VPOS_PWR_LVL_UHQA: c_uint = BIT(3);
const WCD9XXX_A_ANA_RX_VPOS_PWR_LVL_DEFAULT: c_uint = 0;
const WCD9XXX_A_ANA_RX_VNEG_EN_MASK: c_uint = BIT(6);
const WCD9XXX_A_ANA_RX_VNEG_EN_SHIFT: c_uint = 6;
const WCD9XXX_A_ANA_RX_VNEG_ENABLE: c_uint = BIT(6);
const WCD9XXX_A_ANA_RX_VNEG_DISABLE: c_uint = 0;
const WCD9XXX_A_ANA_RX_VPOS_EN_MASK: c_uint = BIT(7);
const WCD9XXX_A_ANA_RX_VPOS_EN_SHIFT: c_uint = 7;
const WCD9XXX_A_ANA_RX_VPOS_ENABLE: c_uint = BIT(7);
const WCD9XXX_A_ANA_RX_VPOS_DISABLE: c_uint = 0;
const WCD9XXX_A_ANA_HPH: c_uint = WCD9335_REG(0x6, 0x09);
const WCD9XXX_A_ANA_HPH_PWR_LEVEL_MASK: c_uint = GENMASK(3, 2);
const WCD9XXX_A_ANA_HPH_PWR_LEVEL_UHQA: c_uint = 0x08;
const WCD9XXX_A_ANA_HPH_PWR_LEVEL_LP: c_uint = 0x04;
const WCD9XXX_A_ANA_HPH_PWR_LEVEL_NORMAL: c_uint = 0x0;
const WCD9XXX_A_CDC_CLSH_CRC: c_uint = WCD9335_REG(0xC, 0x01);
const WCD9XXX_A_CDC_CLSH_CRC_CLK_EN_MASK: c_uint = BIT(0);
const WCD9XXX_A_CDC_CLSH_CRC_CLK_ENABLE: c_uint = BIT(0);
const WCD9XXX_A_CDC_CLSH_CRC_CLK_DISABLE: c_uint = 0;
const WCD9XXX_FLYBACK_EN: c_uint = WCD9335_REG(0x6, 0xA4);
const WCD9XXX_FLYBACK_EN_DELAY_SEL_MASK: c_uint = GENMASK(6, 5);
const WCD9XXX_FLYBACK_EN_DELAY_26P25_US: c_uint = 0x40;
const WCD9XXX_FLYBACK_EN_RESET_BY_EXT_MASK: c_uint = BIT(4);
const WCD9XXX_FLYBACK_EN_PWDN_WITHOUT_DELAY: c_uint = BIT(4);
const WCD9XXX_FLYBACK_EN_PWDN_WITH_DELAY: c_uint = 0;
const WCD9XXX_RX_BIAS_FLYB_BUFF: c_uint = WCD9335_REG(0x6, 0xC7);
const WCD9XXX_RX_BIAS_FLYB_VNEG_5_UA_MASK: c_uint = GENMASK(7, 4);
const WCD9XXX_RX_BIAS_FLYB_VPOS_5_UA_MASK: c_uint = GENMASK(3, 0);
const WCD9XXX_HPH_L_EN: c_uint = WCD9335_REG(0x6, 0xD3);
const WCD9XXX_HPH_CONST_SEL_L_MASK: c_uint = GENMASK(7, 3);
const WCD9XXX_HPH_CONST_SEL_BYPASS: c_uint = 0;
const WCD9XXX_HPH_CONST_SEL_LP_PATH: c_uint = 0x40;
const WCD9XXX_HPH_CONST_SEL_HQ_PATH: c_uint = 0x80;
const WCD9XXX_HPH_R_EN: c_uint = WCD9335_REG(0x6, 0xD6);
const WCD9XXX_HPH_REFBUFF_UHQA_CTL: c_uint = WCD9335_REG(0x6, 0xDD);
const WCD9XXX_HPH_REFBUFF_UHQA_GAIN_MASK: c_uint = GENMASK(2, 0);
const WCD9XXX_CLASSH_CTRL_VCL_2: c_uint = WCD9335_REG(0x6, 0x9B);
const WCD9XXX_CLASSH_CTRL_VCL_2_VREF_FILT_1_MASK: c_uint = GENMASK(5, 4);
const WCD9XXX_CLASSH_CTRL_VCL_VREF_FILT_R_50KOHM: c_uint = 0x20;
const WCD9XXX_CLASSH_CTRL_VCL_VREF_FILT_R_0KOHM: c_uint = 0x0;
const WCD9XXX_CDC_RX1_RX_PATH_CTL: c_uint = WCD9335_REG(0xB, 0x55);
const WCD9XXX_CDC_RX2_RX_PATH_CTL: c_uint = WCD9335_REG(0xB, 0x69);
const WCD9XXX_CDC_CLK_RST_CTRL_MCLK_CONTROL: c_uint = WCD9335_REG(0xD, 0x41);
const WCD9XXX_CDC_CLK_RST_CTRL_MCLK_EN_MASK: c_uint = BIT(0);
const WCD9XXX_CDC_CLK_RST_CTRL_MCLK_11P3_EN_MASK: c_uint = BIT(1);
const WCD9XXX_CLASSH_CTRL_CCL_1: c_uint = WCD9335_REG(0x6, 0x9C);
const WCD9XXX_CLASSH_CTRL_CCL_1_DELTA_IPEAK_MASK: c_uint = GENMASK(7, 4);
const WCD9XXX_CLASSH_CTRL_CCL_1_DELTA_IPEAK_50MA: c_uint = 0x50;
const WCD9XXX_CLASSH_CTRL_CCL_1_DELTA_IPEAK_30MA: c_uint = 0x30;

const WCD9XXX_BASE_ADDRESS: c_uint = 0x3000;
const WCD9XXX_ANA_RX_SUPPLIES: c_uint = WCD9XXX_BASE_ADDRESS + 0x008;
const WCD9XXX_ANA_HPH: c_uint = WCD9XXX_BASE_ADDRESS + 0x009;
const WCD9XXX_CLASSH_MODE_2: c_uint = WCD9XXX_BASE_ADDRESS + 0x098;
const WCD9XXX_CLASSH_MODE_3: c_uint = WCD9XXX_BASE_ADDRESS + 0x099;
const WCD9XXX_FLYBACK_VNEG_CTRL_1: c_uint = WCD9XXX_BASE_ADDRESS + 0x0A5;
const WCD9XXX_FLYBACK_VNEG_CTRL_4: c_uint = WCD9XXX_BASE_ADDRESS + 0x0A8;
const WCD9XXX_FLYBACK_VNEGDAC_CTRL_2: c_uint = WCD9XXX_BASE_ADDRESS + 0x0AF;
const WCD9XXX_RX_BIAS_HPH_LOWPOWER: c_uint = WCD9XXX_BASE_ADDRESS + 0x0BF;
const WCD9XXX_V3_RX_BIAS_FLYB_BUFF: c_uint = WCD9XXX_BASE_ADDRESS + 0x0C7;
const WCD9XXX_HPH_PA_CTL1: c_uint = WCD9XXX_BASE_ADDRESS + 0x0D1;
const WCD9XXX_HPH_NEW_INT_PA_MISC2: c_uint = WCD9XXX_BASE_ADDRESS + 0x138;

const CLSH_REQ_ENABLE: bool = true;
const CLSH_REQ_DISABLE: bool = false;
const WCD_USLEEP_RANGE: c_uint = 50;

const DAC_GAIN_0DB: c_uint = 0;
const DAC_GAIN_0P2DB: c_uint = 1;
const DAC_GAIN_0P4DB: c_uint = 2;
const DAC_GAIN_0P6DB: c_uint = 3;
const DAC_GAIN_0P8DB: c_uint = 4;
const DAC_GAIN_M0P2DB: c_uint = 5;
const DAC_GAIN_M0P4DB: c_uint = 6;
const DAC_GAIN_M0P6DB: c_uint = 7;

// From wcd-clsh-v2.h / wcd9335.h.
const CLS_H_NORMAL: c_int = 0;
const CLS_H_HIFI: c_int = 1;
const CLS_H_LP: c_int = 2;
const CLS_H_ULP: c_int = 3;
const CLS_H_LOHIFI: c_int = 4;
const CLS_AB: c_int = 5;
const CLS_AB_HIFI: c_int = 6;
const CLS_AB_LP: c_int = 7;
const CLS_AB_LOHIFI: c_int = 8;
const WCD_CLSH_STATE_IDLE: c_int = 0;
const WCD_CLSH_STATE_EAR: c_int = 1;
const WCD_CLSH_STATE_HPHL: c_int = 2;
const WCD_CLSH_STATE_HPHR: c_int = 3;
const WCD_CLSH_STATE_LO: c_int = 4;
const WCD_CLSH_STATE_AUX: c_int = 5;
const WCD_CLSH_EVENT_PRE_DAC: c_int = 0;
const WCD_CLSH_EVENT_POST_PA: c_int = 1;
const WCD937X: c_int = 0x9370;

unsafe fn wcd_enable_clsh_block(ctrl: *mut wcd_clsh_ctrl, enable: bool) {
    let comp = unsafe { (*ctrl).comp };

    if (enable
        && {
            unsafe { (*ctrl).clsh_users += 1 };
            unsafe { (*ctrl).clsh_users == 1 }
        })
        || (!enable
            && {
                unsafe { (*ctrl).clsh_users -= 1 };
                unsafe { (*ctrl).clsh_users == 0 }
            })
    {
        unsafe {
            snd_soc_component_update_bits(
                comp,
                WCD9XXX_A_CDC_CLSH_CRC,
                WCD9XXX_A_CDC_CLSH_CRC_CLK_EN_MASK,
                enable as c_uint,
            );
        }
    }
    if unsafe { (*ctrl).clsh_users < 0 } {
        unsafe { (*ctrl).clsh_users = 0 };
    }
}

unsafe fn wcd_clsh_set_buck_mode(comp: *mut snd_soc_component, mode: c_int) {
    /* set to HIFI */
    if mode == CLS_H_HIFI {
        unsafe {
            snd_soc_component_update_bits(
                comp,
                WCD9XXX_A_ANA_RX_SUPPLIES,
                WCD9XXX_A_ANA_RX_VPOS_PWR_LVL_MASK,
                WCD9XXX_A_ANA_RX_VPOS_PWR_LVL_UHQA,
            );
        }
    } else {
        unsafe {
            snd_soc_component_update_bits(
                comp,
                WCD9XXX_A_ANA_RX_SUPPLIES,
                WCD9XXX_A_ANA_RX_VPOS_PWR_LVL_MASK,
                WCD9XXX_A_ANA_RX_VPOS_PWR_LVL_DEFAULT,
            );
        }
    }
}

unsafe fn wcd_clsh_v3_set_buck_mode(component: *mut snd_soc_component, mode: c_int) {
    if mode == CLS_H_HIFI || mode == CLS_H_LOHIFI || mode == CLS_AB_HIFI || mode == CLS_AB_LOHIFI {
        unsafe { snd_soc_component_update_bits(component, WCD9XXX_ANA_RX_SUPPLIES, 0x08, 0x08) };
    } else {
        unsafe { snd_soc_component_update_bits(component, WCD9XXX_ANA_RX_SUPPLIES, 0x08, 0x00) };
    }
}

unsafe fn wcd_clsh_set_flyback_mode(comp: *mut snd_soc_component, mode: c_int) {
    /* set to HIFI */
    if mode == CLS_H_HIFI {
        unsafe {
            snd_soc_component_update_bits(
                comp,
                WCD9XXX_A_ANA_RX_SUPPLIES,
                WCD9XXX_A_ANA_RX_VNEG_PWR_LVL_MASK,
                WCD9XXX_A_ANA_RX_VNEG_PWR_LVL_UHQA,
            );
        }
    } else {
        unsafe {
            snd_soc_component_update_bits(
                comp,
                WCD9XXX_A_ANA_RX_SUPPLIES,
                WCD9XXX_A_ANA_RX_VNEG_PWR_LVL_MASK,
                WCD9XXX_A_ANA_RX_VNEG_PWR_LVL_DEFAULT,
            );
        }
    }
}

unsafe fn wcd_clsh_buck_ctrl(ctrl: *mut wcd_clsh_ctrl, _mode: c_int, enable: bool) {
    let comp = unsafe { (*ctrl).comp };

    /* enable/disable buck */
    if (enable
        && {
            unsafe { (*ctrl).buck_users += 1 };
            unsafe { (*ctrl).buck_users == 1 }
        })
        || (!enable
            && {
                unsafe { (*ctrl).buck_users -= 1 };
                unsafe { (*ctrl).buck_users == 0 }
            })
    {
        unsafe {
            snd_soc_component_update_bits(
                comp,
                WCD9XXX_A_ANA_RX_SUPPLIES,
                WCD9XXX_A_ANA_RX_VPOS_EN_MASK,
                (enable as c_uint) << WCD9XXX_A_ANA_RX_VPOS_EN_SHIFT,
            );
        }
    }
    /*
     * 500us sleep is required after buck enable/disable
     * as per HW requirement
     */
    unsafe { usleep_range(500, 500 + WCD_USLEEP_RANGE) };
}

unsafe fn wcd_clsh_v3_buck_ctrl(
    component: *mut snd_soc_component,
    ctrl: *mut wcd_clsh_ctrl,
    mode: c_int,
    enable: bool,
) {
    /* enable/disable buck */
    if (enable
        && {
            unsafe { (*ctrl).buck_users += 1 };
            unsafe { (*ctrl).buck_users == 1 }
        })
        || (!enable
            && {
                unsafe { (*ctrl).buck_users -= 1 };
                unsafe { (*ctrl).buck_users == 0 }
            })
    {
        unsafe {
            snd_soc_component_update_bits(component, WCD9XXX_ANA_RX_SUPPLIES, 1 << 7, (enable as c_uint) << 7);
        }
        /*
         * 500us sleep is required after buck enable/disable
         * as per HW requirement
         */
        unsafe { usleep_range(500, 510) };
        if mode == CLS_H_LOHIFI || mode == CLS_H_ULP || mode == CLS_H_HIFI || mode == CLS_H_LP {
            unsafe { snd_soc_component_update_bits(component, WCD9XXX_CLASSH_MODE_3, 0x02, 0x00) };
        }

        unsafe { snd_soc_component_update_bits(component, WCD9XXX_CLASSH_MODE_2, 0xFF, 0x3A) };
        /* 500usec delay is needed as per HW requirement */
        unsafe { usleep_range(500, 500 + WCD_USLEEP_RANGE) };
    }
}

unsafe fn wcd_clsh_flyback_ctrl(ctrl: *mut wcd_clsh_ctrl, _mode: c_int, enable: bool) {
    let comp = unsafe { (*ctrl).comp };

    /* enable/disable flyback */
    if (enable
        && {
            unsafe { (*ctrl).flyback_users += 1 };
            unsafe { (*ctrl).flyback_users == 1 }
        })
        || (!enable
            && {
                unsafe { (*ctrl).flyback_users -= 1 };
                unsafe { (*ctrl).flyback_users == 0 }
            })
    {
        unsafe {
            snd_soc_component_update_bits(
                comp,
                WCD9XXX_A_ANA_RX_SUPPLIES,
                WCD9XXX_A_ANA_RX_VNEG_EN_MASK,
                (enable as c_uint) << WCD9XXX_A_ANA_RX_VNEG_EN_SHIFT,
            );
        }
        /* 100usec delay is needed as per HW requirement */
        unsafe { usleep_range(100, 110) };
    }
    /*
     * 500us sleep is required after flyback enable/disable
     * as per HW requirement
     */
    unsafe { usleep_range(500, 500 + WCD_USLEEP_RANGE) };
}

unsafe fn wcd_clsh_set_gain_path(ctrl: *mut wcd_clsh_ctrl, mode: c_int) {
    let comp = unsafe { (*ctrl).comp };
    let mut val: c_uint = 0;

    match mode {
        CLS_H_NORMAL | CLS_AB => val = WCD9XXX_HPH_CONST_SEL_BYPASS,
        CLS_H_HIFI => val = WCD9XXX_HPH_CONST_SEL_HQ_PATH,
        CLS_H_LP => val = WCD9XXX_HPH_CONST_SEL_LP_PATH,
        _ => {}
    }

    unsafe { snd_soc_component_update_bits(comp, WCD9XXX_HPH_L_EN, WCD9XXX_HPH_CONST_SEL_L_MASK, val) };
    unsafe { snd_soc_component_update_bits(comp, WCD9XXX_HPH_R_EN, WCD9XXX_HPH_CONST_SEL_L_MASK, val) };
}

unsafe fn wcd_clsh_v2_set_hph_mode(comp: *mut snd_soc_component, mode: c_int) {
    let mut val: c_uint = 0;
    let mut gain: c_uint = 0;
    let mut ipeak: c_uint = WCD9XXX_CLASSH_CTRL_CCL_1_DELTA_IPEAK_50MA;

    let mut res_val: c_uint = WCD9XXX_CLASSH_CTRL_VCL_VREF_FILT_R_0KOHM;
    match mode {
        CLS_H_NORMAL => {
            res_val = WCD9XXX_CLASSH_CTRL_VCL_VREF_FILT_R_50KOHM;
            val = WCD9XXX_A_ANA_HPH_PWR_LEVEL_NORMAL;
            gain = DAC_GAIN_0DB;
            ipeak = WCD9XXX_CLASSH_CTRL_CCL_1_DELTA_IPEAK_50MA;
        }
        CLS_AB => {
            val = WCD9XXX_A_ANA_HPH_PWR_LEVEL_NORMAL;
            gain = DAC_GAIN_0DB;
            ipeak = WCD9XXX_CLASSH_CTRL_CCL_1_DELTA_IPEAK_50MA;
        }
        CLS_H_HIFI => {
            val = WCD9XXX_A_ANA_HPH_PWR_LEVEL_UHQA;
            gain = DAC_GAIN_M0P2DB;
            ipeak = WCD9XXX_CLASSH_CTRL_CCL_1_DELTA_IPEAK_50MA;
        }
        CLS_H_LP => {
            val = WCD9XXX_A_ANA_HPH_PWR_LEVEL_LP;
            ipeak = WCD9XXX_CLASSH_CTRL_CCL_1_DELTA_IPEAK_30MA;
        }
        _ => {}
    }

    unsafe { snd_soc_component_update_bits(comp, WCD9XXX_A_ANA_HPH, WCD9XXX_A_ANA_HPH_PWR_LEVEL_MASK, val) };
    unsafe {
        snd_soc_component_update_bits(
            comp,
            WCD9XXX_CLASSH_CTRL_VCL_2,
            WCD9XXX_CLASSH_CTRL_VCL_2_VREF_FILT_1_MASK,
            res_val,
        )
    };
    if mode != CLS_H_LP {
        unsafe {
            snd_soc_component_update_bits(comp, WCD9XXX_HPH_REFBUFF_UHQA_CTL, WCD9XXX_HPH_REFBUFF_UHQA_GAIN_MASK, gain)
        };
    }
    unsafe {
        snd_soc_component_update_bits(
            comp,
            WCD9XXX_CLASSH_CTRL_CCL_1,
            WCD9XXX_CLASSH_CTRL_CCL_1_DELTA_IPEAK_MASK,
            ipeak,
        )
    };
}

unsafe fn wcd_clsh_v3_set_hph_mode(component: *mut snd_soc_component, mode: c_int) {
    let val: c_uint;

    match mode {
        CLS_H_NORMAL => val = 0x00,
        CLS_AB | CLS_H_ULP => val = 0x0C,
        CLS_AB_HIFI | CLS_H_HIFI => val = 0x08,
        CLS_H_LP | CLS_H_LOHIFI | CLS_AB_LP | CLS_AB_LOHIFI => val = 0x04,
        _ => {
            unsafe {
                dev_err(
                    (*component).dev,
                    c"%s:Invalid mode %d\n".as_ptr(),
                    c"wcd_clsh_v3_set_hph_mode".as_ptr(),
                    mode,
                );
            }
            return;
        }
    }

    unsafe { snd_soc_component_update_bits(component, WCD9XXX_ANA_HPH, 0x0C, val) };
}

#[no_mangle]
pub unsafe extern "C" fn wcd_clsh_set_hph_mode(ctrl: *mut wcd_clsh_ctrl, mode: c_int) {
    let comp = unsafe { (*ctrl).comp };

    if unsafe { (*ctrl).codec_version >= WCD937X } {
        unsafe { wcd_clsh_v3_set_hph_mode(comp, mode) };
    } else {
        unsafe { wcd_clsh_v2_set_hph_mode(comp, mode) };
    }
}

unsafe fn wcd_clsh_set_flyback_current(comp: *mut snd_soc_component, _mode: c_int) {
    unsafe { snd_soc_component_update_bits(comp, WCD9XXX_RX_BIAS_FLYB_BUFF, WCD9XXX_RX_BIAS_FLYB_VPOS_5_UA_MASK, 0x0A) };
    unsafe { snd_soc_component_update_bits(comp, WCD9XXX_RX_BIAS_FLYB_BUFF, WCD9XXX_RX_BIAS_FLYB_VNEG_5_UA_MASK, 0x0A) };
    /* Sleep needed to avoid click and pop as per HW requirement */
    unsafe { usleep_range(100, 110) };
}

unsafe fn wcd_clsh_set_buck_regulator_mode(comp: *mut snd_soc_component, mode: c_int) {
    if mode == CLS_AB {
        unsafe {
            snd_soc_component_update_bits(
                comp,
                WCD9XXX_A_ANA_RX_SUPPLIES,
                WCD9XXX_A_ANA_RX_REGULATOR_MODE_MASK,
                WCD9XXX_A_ANA_RX_REGULATOR_MODE_CLS_AB,
            );
        }
    } else {
        unsafe {
            snd_soc_component_update_bits(
                comp,
                WCD9XXX_A_ANA_RX_SUPPLIES,
                WCD9XXX_A_ANA_RX_REGULATOR_MODE_MASK,
                WCD9XXX_A_ANA_RX_REGULATOR_MODE_CLS_H,
            );
        }
    }
}

unsafe fn wcd_clsh_v3_set_buck_regulator_mode(component: *mut snd_soc_component, _mode: c_int) {
    unsafe { snd_soc_component_update_bits(component, WCD9XXX_ANA_RX_SUPPLIES, 0x02, 0x00) };
}

unsafe fn wcd_clsh_v3_set_flyback_mode(component: *mut snd_soc_component, mode: c_int) {
    if mode == CLS_H_HIFI || mode == CLS_H_LOHIFI || mode == CLS_AB_HIFI || mode == CLS_AB_LOHIFI {
        unsafe { snd_soc_component_update_bits(component, WCD9XXX_ANA_RX_SUPPLIES, 0x04, 0x04) };
        unsafe { snd_soc_component_update_bits(component, WCD9XXX_FLYBACK_VNEG_CTRL_4, 0xF0, 0x80) };
    } else {
        unsafe { snd_soc_component_update_bits(component, WCD9XXX_ANA_RX_SUPPLIES, 0x04, 0x00) };
        unsafe { snd_soc_component_update_bits(component, WCD9XXX_FLYBACK_VNEG_CTRL_4, 0xF0, 0x70) };
    }
}

unsafe fn wcd_clsh_v3_force_iq_ctl(component: *mut snd_soc_component, mode: c_int, enable: bool) {
    if enable {
        unsafe { snd_soc_component_update_bits(component, WCD9XXX_FLYBACK_VNEGDAC_CTRL_2, 0xE0, 0xA0) };
        /* 100usec delay is needed as per HW requirement */
        unsafe { usleep_range(100, 110) };
        unsafe { snd_soc_component_update_bits(component, WCD9XXX_CLASSH_MODE_3, 0x02, 0x02) };
        unsafe { snd_soc_component_update_bits(component, WCD9XXX_CLASSH_MODE_2, 0xFF, 0x1C) };
        if mode == CLS_H_LOHIFI || mode == CLS_AB_LOHIFI {
            unsafe { snd_soc_component_update_bits(component, WCD9XXX_HPH_NEW_INT_PA_MISC2, 0x20, 0x20) };
            unsafe { snd_soc_component_update_bits(component, WCD9XXX_RX_BIAS_HPH_LOWPOWER, 0xF0, 0xC0) };
            unsafe { snd_soc_component_update_bits(component, WCD9XXX_HPH_PA_CTL1, 0x0E, 0x02) };
        }
    } else {
        unsafe { snd_soc_component_update_bits(component, WCD9XXX_HPH_NEW_INT_PA_MISC2, 0x20, 0x00) };
        unsafe { snd_soc_component_update_bits(component, WCD9XXX_RX_BIAS_HPH_LOWPOWER, 0xF0, 0x80) };
        unsafe { snd_soc_component_update_bits(component, WCD9XXX_HPH_PA_CTL1, 0x0E, 0x06) };
    }
}

unsafe fn wcd_clsh_v3_flyback_ctrl(
    component: *mut snd_soc_component,
    ctrl: *mut wcd_clsh_ctrl,
    _mode: c_int,
    enable: bool,
) {
    /* enable/disable flyback */
    if (enable
        && {
            unsafe { (*ctrl).flyback_users += 1 };
            unsafe { (*ctrl).flyback_users == 1 }
        })
        || (!enable
            && {
                unsafe { (*ctrl).flyback_users -= 1 };
                unsafe { (*ctrl).flyback_users == 0 }
            })
    {
        unsafe { snd_soc_component_update_bits(component, WCD9XXX_FLYBACK_VNEG_CTRL_1, 0xE0, 0xE0) };
        unsafe { snd_soc_component_update_bits(component, WCD9XXX_ANA_RX_SUPPLIES, 1 << 6, (enable as c_uint) << 6) };
        /*
         * 100us sleep is required after flyback enable/disable
         * as per HW requirement
         */
        unsafe { usleep_range(100, 110) };
        unsafe { snd_soc_component_update_bits(component, WCD9XXX_FLYBACK_VNEGDAC_CTRL_2, 0xE0, 0xE0) };
        /* 500usec delay is needed as per HW requirement */
        unsafe { usleep_range(500, 500 + WCD_USLEEP_RANGE) };
    }
}

unsafe fn wcd_clsh_v3_set_flyback_current(component: *mut snd_soc_component, _mode: c_int) {
    unsafe { snd_soc_component_update_bits(component, WCD9XXX_V3_RX_BIAS_FLYB_BUFF, 0x0F, 0x0A) };
    unsafe { snd_soc_component_update_bits(component, WCD9XXX_V3_RX_BIAS_FLYB_BUFF, 0xF0, 0xA0) };
    /* Sleep needed to avoid click and pop as per HW requirement */
    unsafe { usleep_range(100, 110) };
}

unsafe fn wcd_clsh_v3_state_aux(ctrl: *mut wcd_clsh_ctrl, _req_state: c_int, is_enable: bool, mode: c_int) {
    let component = unsafe { (*ctrl).comp };

    if is_enable {
        unsafe { wcd_clsh_v3_set_buck_mode(component, mode) };
        unsafe { wcd_clsh_v3_set_flyback_mode(component, mode) };
        unsafe { wcd_clsh_v3_flyback_ctrl(component, ctrl, mode, true) };
        unsafe { wcd_clsh_v3_set_flyback_current(component, mode) };
        unsafe { wcd_clsh_v3_buck_ctrl(component, ctrl, mode, true) };
    } else {
        unsafe { wcd_clsh_v3_buck_ctrl(component, ctrl, mode, false) };
        unsafe { wcd_clsh_v3_flyback_ctrl(component, ctrl, mode, false) };
        unsafe { wcd_clsh_v3_set_flyback_mode(component, CLS_H_NORMAL) };
        unsafe { wcd_clsh_v3_set_buck_mode(component, CLS_H_NORMAL) };
    }
}

unsafe fn wcd_clsh_state_lo(ctrl: *mut wcd_clsh_ctrl, _req_state: c_int, is_enable: bool, mode: c_int) {
    let comp = unsafe { (*ctrl).comp };

    if mode != CLS_AB {
        unsafe {
            dev_err(
                (*comp).dev,
                c"%s: LO cannot be in this mode: %d\n".as_ptr(),
                c"wcd_clsh_state_lo".as_ptr(),
                mode,
            );
        }
        return;
    }

    if is_enable {
        unsafe { wcd_clsh_set_buck_regulator_mode(comp, mode) };
        unsafe { wcd_clsh_set_buck_mode(comp, mode) };
        unsafe { wcd_clsh_set_flyback_mode(comp, mode) };
        unsafe { wcd_clsh_flyback_ctrl(ctrl, mode, true) };
        unsafe { wcd_clsh_set_flyback_current(comp, mode) };
        unsafe { wcd_clsh_buck_ctrl(ctrl, mode, true) };
    } else {
        unsafe { wcd_clsh_buck_ctrl(ctrl, mode, false) };
        unsafe { wcd_clsh_flyback_ctrl(ctrl, mode, false) };
        unsafe { wcd_clsh_set_flyback_mode(comp, CLS_H_NORMAL) };
        unsafe { wcd_clsh_set_buck_mode(comp, CLS_H_NORMAL) };
        unsafe { wcd_clsh_set_buck_regulator_mode(comp, CLS_H_NORMAL) };
    }
}

unsafe fn wcd_clsh_v3_state_hph_r(ctrl: *mut wcd_clsh_ctrl, _req_state: c_int, is_enable: bool, mode: c_int) {
    let component = unsafe { (*ctrl).comp };

    if mode == CLS_H_NORMAL {
        unsafe {
            dev_dbg(
                (*component).dev,
                c"%s: Normal mode not applicable for hph_r\n".as_ptr(),
                c"wcd_clsh_v3_state_hph_r".as_ptr(),
            );
        }
        return;
    }

    if is_enable {
        unsafe { wcd_clsh_v3_set_buck_regulator_mode(component, mode) };
        unsafe { wcd_clsh_v3_set_flyback_mode(component, mode) };
        unsafe { wcd_clsh_v3_force_iq_ctl(component, mode, true) };
        unsafe { wcd_clsh_v3_flyback_ctrl(component, ctrl, mode, true) };
        unsafe { wcd_clsh_v3_set_flyback_current(component, mode) };
        unsafe { wcd_clsh_v3_set_buck_mode(component, mode) };
        unsafe { wcd_clsh_v3_buck_ctrl(component, ctrl, mode, true) };
        unsafe { wcd_clsh_v3_set_hph_mode(component, mode) };
    } else {
        unsafe { wcd_clsh_v3_set_hph_mode(component, CLS_H_NORMAL) };

        /* buck and flyback set to default mode and disable */
        unsafe { wcd_clsh_v3_flyback_ctrl(component, ctrl, CLS_H_NORMAL, false) };
        unsafe { wcd_clsh_v3_buck_ctrl(component, ctrl, CLS_H_NORMAL, false) };
        unsafe { wcd_clsh_v3_force_iq_ctl(component, CLS_H_NORMAL, false) };
        unsafe { wcd_clsh_v3_set_flyback_mode(component, CLS_H_NORMAL) };
        unsafe { wcd_clsh_v3_set_buck_mode(component, CLS_H_NORMAL) };
    }
}

unsafe fn wcd_clsh_state_hph_r(ctrl: *mut wcd_clsh_ctrl, _req_state: c_int, is_enable: bool, mode: c_int) {
    let comp = unsafe { (*ctrl).comp };

    if mode == CLS_H_NORMAL {
        unsafe {
            dev_err(
                (*comp).dev,
                c"%s: Normal mode not applicable for hph_r\n".as_ptr(),
                c"wcd_clsh_state_hph_r".as_ptr(),
            );
        }
        return;
    }

    if is_enable {
        if mode != CLS_AB {
            unsafe { wcd_enable_clsh_block(ctrl, true) };
            /*
             * These K1 values depend on the Headphone Impedance
             * For now it is assumed to be 16 ohm
             */
            unsafe { snd_soc_component_update_bits(comp, WCD9XXX_A_CDC_CLSH_K1_MSB, WCD9XXX_A_CDC_CLSH_K1_MSB_COEF_MASK, 0x00) };
            unsafe { snd_soc_component_update_bits(comp, WCD9XXX_A_CDC_CLSH_K1_LSB, WCD9XXX_A_CDC_CLSH_K1_LSB_COEF_MASK, 0xC0) };
            unsafe {
                snd_soc_component_update_bits(
                    comp,
                    WCD9XXX_A_CDC_RX2_RX_PATH_CFG0,
                    WCD9XXX_A_CDC_RX_PATH_CLSH_EN_MASK,
                    WCD9XXX_A_CDC_RX_PATH_CLSH_ENABLE,
                )
            };
        }
        unsafe { wcd_clsh_set_buck_regulator_mode(comp, mode) };
        unsafe { wcd_clsh_set_flyback_mode(comp, mode) };
        unsafe { wcd_clsh_flyback_ctrl(ctrl, mode, true) };
        unsafe { wcd_clsh_set_flyback_current(comp, mode) };
        unsafe { wcd_clsh_set_buck_mode(comp, mode) };
        unsafe { wcd_clsh_buck_ctrl(ctrl, mode, true) };
        unsafe { wcd_clsh_v2_set_hph_mode(comp, mode) };
        unsafe { wcd_clsh_set_gain_path(ctrl, mode) };
    } else {
        unsafe { wcd_clsh_v2_set_hph_mode(comp, CLS_H_NORMAL) };

        if mode != CLS_AB {
            unsafe {
                snd_soc_component_update_bits(
                    comp,
                    WCD9XXX_A_CDC_RX2_RX_PATH_CFG0,
                    WCD9XXX_A_CDC_RX_PATH_CLSH_EN_MASK,
                    WCD9XXX_A_CDC_RX_PATH_CLSH_DISABLE,
                )
            };
            unsafe { wcd_enable_clsh_block(ctrl, false) };
        }
        /* buck and flyback set to default mode and disable */
        unsafe { wcd_clsh_buck_ctrl(ctrl, CLS_H_NORMAL, false) };
        unsafe { wcd_clsh_flyback_ctrl(ctrl, CLS_H_NORMAL, false) };
        unsafe { wcd_clsh_set_flyback_mode(comp, CLS_H_NORMAL) };
        unsafe { wcd_clsh_set_buck_mode(comp, CLS_H_NORMAL) };
        unsafe { wcd_clsh_set_buck_regulator_mode(comp, CLS_H_NORMAL) };
    }
}

unsafe fn wcd_clsh_v3_state_hph_l(ctrl: *mut wcd_clsh_ctrl, _req_state: c_int, is_enable: bool, mode: c_int) {
    let component = unsafe { (*ctrl).comp };

    if mode == CLS_H_NORMAL {
        unsafe {
            dev_dbg(
                (*component).dev,
                c"%s: Normal mode not applicable for hph_l\n".as_ptr(),
                c"wcd_clsh_v3_state_hph_l".as_ptr(),
            );
        }
        return;
    }

    if is_enable {
        unsafe { wcd_clsh_v3_set_buck_regulator_mode(component, mode) };
        unsafe { wcd_clsh_v3_set_flyback_mode(component, mode) };
        unsafe { wcd_clsh_v3_force_iq_ctl(component, mode, true) };
        unsafe { wcd_clsh_v3_flyback_ctrl(component, ctrl, mode, true) };
        unsafe { wcd_clsh_v3_set_flyback_current(component, mode) };
        unsafe { wcd_clsh_v3_set_buck_mode(component, mode) };
        unsafe { wcd_clsh_v3_buck_ctrl(component, ctrl, mode, true) };
        unsafe { wcd_clsh_v3_set_hph_mode(component, mode) };
    } else {
        unsafe { wcd_clsh_v3_set_hph_mode(component, CLS_H_NORMAL) };

        /* set buck and flyback to Default Mode */
        unsafe { wcd_clsh_v3_flyback_ctrl(component, ctrl, CLS_H_NORMAL, false) };
        unsafe { wcd_clsh_v3_buck_ctrl(component, ctrl, CLS_H_NORMAL, false) };
        unsafe { wcd_clsh_v3_force_iq_ctl(component, CLS_H_NORMAL, false) };
        unsafe { wcd_clsh_v3_set_flyback_mode(component, CLS_H_NORMAL) };
        unsafe { wcd_clsh_v3_set_buck_mode(component, CLS_H_NORMAL) };
    }
}

unsafe fn wcd_clsh_state_hph_l(ctrl: *mut wcd_clsh_ctrl, _req_state: c_int, is_enable: bool, mode: c_int) {
    let comp = unsafe { (*ctrl).comp };

    if mode == CLS_H_NORMAL {
        unsafe {
            dev_err(
                (*comp).dev,
                c"%s: Normal mode not applicable for hph_l\n".as_ptr(),
                c"wcd_clsh_state_hph_l".as_ptr(),
            );
        }
        return;
    }

    if is_enable {
        if mode != CLS_AB {
            unsafe { wcd_enable_clsh_block(ctrl, true) };
            /*
             * These K1 values depend on the Headphone Impedance
             * For now it is assumed to be 16 ohm
             */
            unsafe { snd_soc_component_update_bits(comp, WCD9XXX_A_CDC_CLSH_K1_MSB, WCD9XXX_A_CDC_CLSH_K1_MSB_COEF_MASK, 0x00) };
            unsafe { snd_soc_component_update_bits(comp, WCD9XXX_A_CDC_CLSH_K1_LSB, WCD9XXX_A_CDC_CLSH_K1_LSB_COEF_MASK, 0xC0) };
            unsafe {
                snd_soc_component_update_bits(
                    comp,
                    WCD9XXX_A_CDC_RX1_RX_PATH_CFG0,
                    WCD9XXX_A_CDC_RX_PATH_CLSH_EN_MASK,
                    WCD9XXX_A_CDC_RX_PATH_CLSH_ENABLE,
                )
            };
        }
        unsafe { wcd_clsh_set_buck_regulator_mode(comp, mode) };
        unsafe { wcd_clsh_set_flyback_mode(comp, mode) };
        unsafe { wcd_clsh_flyback_ctrl(ctrl, mode, true) };
        unsafe { wcd_clsh_set_flyback_current(comp, mode) };
        unsafe { wcd_clsh_set_buck_mode(comp, mode) };
        unsafe { wcd_clsh_buck_ctrl(ctrl, mode, true) };
        unsafe { wcd_clsh_v2_set_hph_mode(comp, mode) };
        unsafe { wcd_clsh_set_gain_path(ctrl, mode) };
    } else {
        unsafe { wcd_clsh_v2_set_hph_mode(comp, CLS_H_NORMAL) };

        if mode != CLS_AB {
            unsafe {
                snd_soc_component_update_bits(
                    comp,
                    WCD9XXX_A_CDC_RX1_RX_PATH_CFG0,
                    WCD9XXX_A_CDC_RX_PATH_CLSH_EN_MASK,
                    WCD9XXX_A_CDC_RX_PATH_CLSH_DISABLE,
                )
            };
            unsafe { wcd_enable_clsh_block(ctrl, false) };
        }
        /* set buck and flyback to Default Mode */
        unsafe { wcd_clsh_buck_ctrl(ctrl, CLS_H_NORMAL, false) };
        unsafe { wcd_clsh_flyback_ctrl(ctrl, CLS_H_NORMAL, false) };
        unsafe { wcd_clsh_set_flyback_mode(comp, CLS_H_NORMAL) };
        unsafe { wcd_clsh_set_buck_mode(comp, CLS_H_NORMAL) };
        unsafe { wcd_clsh_set_buck_regulator_mode(comp, CLS_H_NORMAL) };
    }
}

unsafe fn wcd_clsh_v3_state_ear(ctrl: *mut wcd_clsh_ctrl, _req_state: c_int, is_enable: bool, mode: c_int) {
    let component = unsafe { (*ctrl).comp };

    if is_enable {
        unsafe { wcd_clsh_v3_set_buck_regulator_mode(component, mode) };
        unsafe { wcd_clsh_v3_set_flyback_mode(component, mode) };
        unsafe { wcd_clsh_v3_force_iq_ctl(component, mode, true) };
        unsafe { wcd_clsh_v3_flyback_ctrl(component, ctrl, mode, true) };
        unsafe { wcd_clsh_v3_set_flyback_current(component, mode) };
        unsafe { wcd_clsh_v3_set_buck_mode(component, mode) };
        unsafe { wcd_clsh_v3_buck_ctrl(component, ctrl, mode, true) };
        unsafe { wcd_clsh_v3_set_hph_mode(component, mode) };
    } else {
        unsafe { wcd_clsh_v3_set_hph_mode(component, CLS_H_NORMAL) };

        /* set buck and flyback to Default Mode */
        unsafe { wcd_clsh_v3_flyback_ctrl(component, ctrl, CLS_H_NORMAL, false) };
        unsafe { wcd_clsh_v3_buck_ctrl(component, ctrl, CLS_H_NORMAL, false) };
        unsafe { wcd_clsh_v3_force_iq_ctl(component, CLS_H_NORMAL, false) };
        unsafe { wcd_clsh_v3_set_flyback_mode(component, CLS_H_NORMAL) };
        unsafe { wcd_clsh_v3_set_buck_mode(component, CLS_H_NORMAL) };
    }
}

unsafe fn wcd_clsh_state_ear(ctrl: *mut wcd_clsh_ctrl, _req_state: c_int, is_enable: bool, mode: c_int) {
    let comp = unsafe { (*ctrl).comp };

    if mode != CLS_H_NORMAL {
        unsafe {
            dev_err(
                (*comp).dev,
                c"%s: mode: %d cannot be used for EAR\n".as_ptr(),
                c"wcd_clsh_state_ear".as_ptr(),
                mode,
            );
        }
        return;
    }

    if is_enable {
        unsafe { wcd_enable_clsh_block(ctrl, true) };
        unsafe {
            snd_soc_component_update_bits(
                comp,
                WCD9XXX_A_CDC_RX0_RX_PATH_CFG0,
                WCD9XXX_A_CDC_RX_PATH_CLSH_EN_MASK,
                WCD9XXX_A_CDC_RX_PATH_CLSH_ENABLE,
            )
        };
        unsafe { wcd_clsh_set_buck_mode(comp, mode) };
        unsafe { wcd_clsh_set_flyback_mode(comp, mode) };
        unsafe { wcd_clsh_flyback_ctrl(ctrl, mode, true) };
        unsafe { wcd_clsh_set_flyback_current(comp, mode) };
        unsafe { wcd_clsh_buck_ctrl(ctrl, mode, true) };
    } else {
        unsafe {
            snd_soc_component_update_bits(
                comp,
                WCD9XXX_A_CDC_RX0_RX_PATH_CFG0,
                WCD9XXX_A_CDC_RX_PATH_CLSH_EN_MASK,
                WCD9XXX_A_CDC_RX_PATH_CLSH_DISABLE,
            )
        };
        unsafe { wcd_enable_clsh_block(ctrl, false) };
        unsafe { wcd_clsh_buck_ctrl(ctrl, mode, false) };
        unsafe { wcd_clsh_flyback_ctrl(ctrl, mode, false) };
        unsafe { wcd_clsh_set_flyback_mode(comp, CLS_H_NORMAL) };
        unsafe { wcd_clsh_set_buck_mode(comp, CLS_H_NORMAL) };
    }
}

unsafe fn _wcd_clsh_ctrl_set_state(
    ctrl: *mut wcd_clsh_ctrl,
    req_state: c_int,
    is_enable: bool,
    mode: c_int,
) -> c_int {
    match req_state {
        WCD_CLSH_STATE_EAR => {
            if unsafe { (*ctrl).codec_version >= WCD937X } {
                unsafe { wcd_clsh_v3_state_ear(ctrl, req_state, is_enable, mode) };
            } else {
                unsafe { wcd_clsh_state_ear(ctrl, req_state, is_enable, mode) };
            }
        }
        WCD_CLSH_STATE_HPHL => {
            if unsafe { (*ctrl).codec_version >= WCD937X } {
                unsafe { wcd_clsh_v3_state_hph_l(ctrl, req_state, is_enable, mode) };
            } else {
                unsafe { wcd_clsh_state_hph_l(ctrl, req_state, is_enable, mode) };
            }
        }
        WCD_CLSH_STATE_HPHR => {
            if unsafe { (*ctrl).codec_version >= WCD937X } {
                unsafe { wcd_clsh_v3_state_hph_r(ctrl, req_state, is_enable, mode) };
            } else {
                unsafe { wcd_clsh_state_hph_r(ctrl, req_state, is_enable, mode) };
            }
        }
        WCD_CLSH_STATE_LO => {
            if unsafe { (*ctrl).codec_version < WCD937X } {
                unsafe { wcd_clsh_state_lo(ctrl, req_state, is_enable, mode) };
            }
        }
        WCD_CLSH_STATE_AUX => {
            if unsafe { (*ctrl).codec_version >= WCD937X } {
                unsafe { wcd_clsh_v3_state_aux(ctrl, req_state, is_enable, mode) };
            }
        }
        _ => {}
    }

    0
}

/*
 * Function: wcd_clsh_is_state_valid
 * Params: state
 * Description:
 * Provides information on valid states of Class H configuration
 */
fn wcd_clsh_is_state_valid(state: c_int) -> bool {
    match state {
        WCD_CLSH_STATE_IDLE
        | WCD_CLSH_STATE_EAR
        | WCD_CLSH_STATE_HPHL
        | WCD_CLSH_STATE_HPHR
        | WCD_CLSH_STATE_LO
        | WCD_CLSH_STATE_AUX => true,
        _ => false,
    }
}

/*
 * Function: wcd_clsh_fsm
 * Params: ctrl, req_state, req_type, clsh_event
 * Description:
 * This function handles PRE DAC and POST DAC conditions of different devices
 * and updates class H configuration of different combination of devices
 * based on validity of their states. ctrl will contain current
 * class h state information
 */
#[no_mangle]
pub unsafe extern "C" fn wcd_clsh_ctrl_set_state(
    ctrl: *mut wcd_clsh_ctrl,
    clsh_event: c_int,
    nstate: c_int,
    mode: c_int,
) -> c_int {
    let comp = unsafe { (*ctrl).comp };

    if !wcd_clsh_is_state_valid(nstate) {
        unsafe { dev_err((*comp).dev, c"Class-H not a valid new state:\n".as_ptr()) };
        return -EINVAL;
    }

    match clsh_event {
        WCD_CLSH_EVENT_PRE_DAC => {
            unsafe { _wcd_clsh_ctrl_set_state(ctrl, nstate, CLSH_REQ_ENABLE, mode) };
        }
        WCD_CLSH_EVENT_POST_PA => {
            unsafe { _wcd_clsh_ctrl_set_state(ctrl, nstate, CLSH_REQ_DISABLE, mode) };
        }
        _ => {}
    }

    unsafe {
        (*ctrl).state = nstate;
        (*ctrl).mode = mode;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn wcd_clsh_ctrl_get_state(ctrl: *mut wcd_clsh_ctrl) -> c_int {
    unsafe { (*ctrl).state }
}

#[no_mangle]
pub unsafe extern "C" fn wcd_clsh_ctrl_alloc(
    comp: *mut snd_soc_component,
    version: c_int,
) -> *mut wcd_clsh_ctrl {
    let ctrl: *mut wcd_clsh_ctrl;

    ctrl = unsafe { kzalloc_obj::<wcd_clsh_ctrl>() };
    if ctrl.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    unsafe {
        (*ctrl).state = WCD_CLSH_STATE_IDLE;
        (*ctrl).comp = comp;
        (*ctrl).codec_version = version;
    }

    ctrl
}

#[no_mangle]
pub unsafe extern "C" fn wcd_clsh_ctrl_free(ctrl: *mut wcd_clsh_ctrl) {
    unsafe { kfree(ctrl as *mut c_void) };
}

// MODULE_DESCRIPTION("WCD93XX Class-H driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
