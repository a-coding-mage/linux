// SPDX-License-Identifier: GPL-2.0-only
//
// aw88399-lib.rs -- Rust translation of aw88399-lib.c
//
// Original C dependencies: linux/firmware.h, linux/gpio/consumer.h,
// linux/i2c.h, linux/minmax.h, linux/regmap.h, sound/soc.h,
// sound/aw88399.h, aw88395/aw88395_device.h.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_variables, improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_uchar, c_ushort, c_void};
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type int16_t = i16;
type uint16_t = u16;
type int32_t = i32;
type uint32_t = u32;

macro_rules! dev_dbg { ($($arg:tt)*) => {{ }} }
macro_rules! dev_err { ($($arg:tt)*) => {{ }} }

// Types, constants, and helper functions below are supplied by translated headers/other files.
#[repr(C)] pub struct regmap_config { pub val_bits: c_uint, pub reg_bits: c_uint, pub max_register: c_uint, pub reg_format_endian: c_uint, pub val_format_endian: c_uint }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct i2c_client { pub dev: device, pub addr: c_uint }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { pub work: work_struct }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct firmware { pub size: usize, pub data: *const u8 }
#[repr(C)] pub struct aw_cali_desc { pub cali_re: u32, pub ra: u32 }
#[repr(C)] pub struct aw_volume_desc { pub init_volume: u16, pub ctl_volume: u16 }
#[repr(C)] pub struct aw_profctrl_desc { pub cur_mode: c_int }
#[repr(C)] pub struct aw_sec_data_desc { pub data: *mut c_uchar, pub len: c_uint }
#[repr(C)] pub struct aw_prof_desc { pub id: c_int, pub fw_ver: c_uint, pub sec_desc: *mut aw_sec_data_desc }
#[repr(C)] pub struct aw_prof_info { pub prof_desc: *mut aw_prof_desc, pub count: c_int, pub prof_type: c_int, pub prof_name_list: *mut *mut c_char }
#[repr(C)] pub struct aw_container { pub len: c_int, pub data: [u8; 0] }
#[repr(C)] pub struct aw_device { pub i2c: *mut i2c_client, pub dev: *mut device, pub regmap: *mut regmap, pub dsp_lock: mutex, pub chip_id: c_uint, pub acf: *mut c_void, pub prof_info: aw_prof_info, pub channel: c_uint, pub fw_status: c_int, pub status: c_int, pub dsp_cfg: c_int, pub dsp_fw_len: c_uint, pub dsp_cfg_len: c_uint, pub fade_step: c_int, pub fade_in_time: c_uint, pub fade_out_time: c_uint, pub prof_cur: c_int, pub prof_index: c_int, pub cali_desc: aw_cali_desc, pub volume_desc: aw_volume_desc, pub profctrl_desc: aw_profctrl_desc }
#[repr(C)] pub struct aw88399 { pub aw_pa: *mut aw_device, pub aw_cfg: *mut aw_container, pub reset_gpio: *mut gpio_desc, pub lock: mutex, pub start_work: delayed_work, pub bsts_unreliable: bool, pub check_val: c_int, pub vcalb_init_val: u16, pub crc_init_val: u16, pub dither_st: u16, pub fw_needs_reload: bool }

unsafe extern "C" {
    static mut system_dfl_wq: *mut workqueue_struct;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_raw_write(map: *mut regmap, reg: c_uint, val: *const c_void, val_len: usize) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn aw_dev_dsp_read(aw_dev: *mut aw_device, dsp_addr: c_uint, dsp_data: *mut c_uint, data_type: c_uint) -> c_int;
    fn aw_dev_dsp_write(aw_dev: *mut aw_device, dsp_addr: c_uint, dsp_data: c_uint, data_type: c_uint) -> c_int;
    fn aw88395_dev_cfg_load(aw_dev: *mut aw_device, aw_cfg: *mut aw_container) -> c_int;
    fn aw88395_dev_load_acf_check(aw_dev: *mut aw_device, aw_cfg: *mut aw_container) -> c_int;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn queue_delayed_work(wq: *mut workqueue_struct, work: *mut delayed_work, delay: c_uint) -> bool;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn aw_device_from_cali_desc(cali_desc: *mut aw_cali_desc) -> *mut aw_device;
    fn aw88399_from_start_work(work: *mut work_struct) -> *mut aw88399;
}

pub const aw88399_remap_config: regmap_config = regmap_config { val_bits: 16, reg_bits: 8, max_register: AW88399_REG_MAX, reg_format_endian: REGMAP_ENDIAN_LITTLE, val_format_endian: REGMAP_ENDIAN_BIG };

unsafe fn aw_dev_pwd(aw_dev: *mut aw_device, pwd: bool) {
    let ret: c_int = if pwd {
        regmap_update_bits((*aw_dev).regmap, AW88399_SYSCTRL_REG, !AW88399_PWDN_MASK, AW88399_PWDN_POWER_DOWN_VALUE)
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88399_SYSCTRL_REG, !AW88399_PWDN_MASK, AW88399_PWDN_WORKING_VALUE)
    };
    if ret != 0 { dev_dbg!((*aw_dev).dev, "%s failed", "aw_dev_pwd"); }
}

unsafe fn aw_dev_get_int_status(aw_dev: *mut aw_device, int_status: *mut c_ushort) {
    let mut reg_val: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88399_SYSINT_REG, &mut reg_val);
    if ret != 0 {
        dev_err!((*aw_dev).dev, "read interrupt reg fail, ret=%d", ret);
    } else {
        *int_status = reg_val as c_ushort;
    }
    dev_dbg!((*aw_dev).dev, "read interrupt reg=0x%04x", *int_status);
}

unsafe fn aw_dev_clear_int_status(aw_dev: *mut aw_device) {
    let mut int_status: u16 = 0;
    /* read int status and clear */
    aw_dev_get_int_status(aw_dev, &mut int_status);
    /* make sure int status is clear */
    aw_dev_get_int_status(aw_dev, &mut int_status);
    if int_status != 0 { dev_dbg!((*aw_dev).dev, "int status(%d) is not cleaned.\n", int_status); }
}

unsafe fn aw_dev_get_iis_status(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88399_SYSST_REG, &mut reg_val);
    if ret != 0 { return ret; }
    if (reg_val & AW88399_BIT_PLL_CHECK) != AW88399_BIT_PLL_CHECK {
        dev_err!((*aw_dev).dev, "check pll lock fail, reg_val:0x%04x", reg_val);
        return -EINVAL;
    }
    0
}

unsafe fn aw_dev_check_mode1_pll(aw_dev: *mut aw_device) -> c_int {
    let mut i = 0;
    while i < AW88399_DEV_SYSST_CHECK_MAX {
        let ret = aw_dev_get_iis_status(aw_dev);
        if ret != 0 {
            dev_err!((*aw_dev).dev, "mode1 iis signal check error");
            usleep_range(AW88399_2000_US, AW88399_2000_US + 10);
        } else {
            return 0;
        }
        i += 1;
    }
    -EPERM
}

unsafe fn aw_dev_check_mode2_pll(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    let mut ret = regmap_read((*aw_dev).regmap, AW88399_PLLCTRL2_REG, &mut reg_val);
    if ret != 0 { return ret; }
    reg_val &= !AW88399_CCO_MUX_MASK;
    if reg_val == AW88399_CCO_MUX_DIVIDED_VALUE {
        dev_dbg!((*aw_dev).dev, "CCO_MUX is already divider");
        return -EPERM;
    }
    /* change mode2 */
    ret = regmap_update_bits((*aw_dev).regmap, AW88399_PLLCTRL2_REG, !AW88399_CCO_MUX_MASK, AW88399_CCO_MUX_DIVIDED_VALUE);
    if ret != 0 { return ret; }
    let mut i = 0;
    while i < AW88399_DEV_SYSST_CHECK_MAX {
        ret = aw_dev_get_iis_status(aw_dev);
        if ret != 0 {
            dev_err!((*aw_dev).dev, "mode2 iis signal check error");
            usleep_range(AW88399_2000_US, AW88399_2000_US + 10);
        } else { break; }
        i += 1;
    }
    /* change mode1 */
    regmap_update_bits((*aw_dev).regmap, AW88399_PLLCTRL2_REG, !AW88399_CCO_MUX_MASK, AW88399_CCO_MUX_BYPASS_VALUE);
    if ret == 0 {
        usleep_range(AW88399_2000_US, AW88399_2000_US + 10);
        i = 0;
        while i < AW88399_DEV_SYSST_CHECK_MAX {
            ret = aw_dev_get_iis_status(aw_dev);
            if ret != 0 {
                dev_err!((*aw_dev).dev, "mode2 switch to mode1, iis signal check error");
                usleep_range(AW88399_2000_US, AW88399_2000_US + 10);
            } else { break; }
            i += 1;
        }
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn aw_dev_check_syspll(aw_dev: *mut aw_device) -> c_int {
    let mut ret = aw_dev_check_mode1_pll(aw_dev);
    if ret != 0 {
        dev_dbg!((*aw_dev).dev, "mode1 check iis failed try switch to mode2 check");
        ret = aw_dev_check_mode2_pll(aw_dev);
        if ret != 0 {
            dev_err!((*aw_dev).dev, "mode2 check iis failed");
            return ret;
        }
    }
    0
}

unsafe fn aw_dev_check_sysst(aw88399: *mut aw88399) -> c_int {
    let aw_dev = (*aw88399).aw_pa;
    let mut reg_val: c_uint = 0;
    let mut ret = regmap_read((*aw_dev).regmap, AW88399_PWMCTRL3_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let mut check_val = if (reg_val & !AW88399_NOISE_GATE_EN_MASK) != 0 { AW88399_BIT_SYSST_NOSWS_CHECK } else { AW88399_BIT_SYSST_SWS_CHECK };
    /*
     * On some hardware the BSTS (boost-finished) status bit does not
     * reliably assert even when audio output is working normally.
     * Allow per-instance bypass when flagged by the side-codec driver.
     */
    if (*aw88399).bsts_unreliable { check_val &= !AW88399_BSTS_FINISHED_VALUE; }
    let mut i = 0;
    while i < AW88399_DEV_SYSST_CHECK_MAX {
        ret = regmap_read((*aw_dev).regmap, AW88399_SYSST_REG, &mut reg_val);
        if ret != 0 { return ret; }
        if (reg_val & !AW88399_BIT_SYSST_CHECK_MASK & check_val) != check_val {
            dev_err!((*aw_dev).dev, "check sysst fail, cnt=%d, reg_val=0x%04x, check:0x%x", i, reg_val, AW88399_BIT_SYSST_NOSWS_CHECK);
            usleep_range(AW88399_2000_US, AW88399_2000_US + 10);
        } else { return 0; }
        i += 1;
    }
    -EPERM
}

unsafe fn aw_dev_amppd(aw_dev: *mut aw_device, amppd: bool) {
    let ret = if amppd {
        regmap_update_bits((*aw_dev).regmap, AW88399_SYSCTRL_REG, !AW88399_AMPPD_MASK, AW88399_AMPPD_POWER_DOWN_VALUE)
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88399_SYSCTRL_REG, !AW88399_AMPPD_MASK, AW88399_AMPPD_WORKING_VALUE)
    };
    if ret != 0 { dev_dbg!((*aw_dev).dev, "%s failed", "aw_dev_amppd"); }
}

#[no_mangle]
pub unsafe extern "C" fn aw_dev_dsp_enable(aw_dev: *mut aw_device, is_enable: bool) {
    let ret = if is_enable {
        regmap_update_bits((*aw_dev).regmap, AW88399_SYSCTRL_REG, !AW88399_DSPBY_MASK, AW88399_DSPBY_WORKING_VALUE)
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88399_SYSCTRL_REG, !AW88399_DSPBY_MASK, AW88399_DSPBY_BYPASS_VALUE)
    };
    if ret != 0 { dev_dbg!((*aw_dev).dev, "%s failed\n", "aw_dev_dsp_enable"); }
}

unsafe fn aw88399_dev_get_icalk(aw88399: *mut aw88399, icalk: *mut int16_t) -> c_int {
    let aw_dev = (*aw88399).aw_pa;
    let mut reg_val: c_uint = 0;
    let mut ret = regmap_read((*aw_dev).regmap, AW88399_EFRH4_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let icalkh_val: uint16_t = (reg_val & !AW88399_EF_ISN_GESLP_H_MASK) as uint16_t;
    ret = regmap_read((*aw_dev).regmap, AW88399_EFRL4_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let icalkl_val: uint16_t = (reg_val & !AW88399_EF_ISN_GESLP_L_MASK) as uint16_t;
    let mut icalk_val = if (*aw88399).check_val == AW_EF_AND_CHECK { icalkh_val & icalkl_val } else { icalkh_val | icalkl_val };
    if (icalk_val & !AW88399_EF_ISN_GESLP_SIGN_MASK as u16) != 0 { icalk_val |= AW88399_EF_ISN_GESLP_SIGN_NEG as u16; }
    *icalk = icalk_val as int16_t;
    0
}

unsafe fn aw88399_dev_get_vcalk(aw88399: *mut aw88399, vcalk: *mut int16_t) -> c_int {
    let aw_dev = (*aw88399).aw_pa;
    let mut reg_val: c_uint = 0;
    let mut ret = regmap_read((*aw_dev).regmap, AW88399_EFRH3_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let vcalkh_val: uint16_t = (reg_val & !AW88399_EF_VSN_GESLP_H_MASK) as uint16_t;
    ret = regmap_read((*aw_dev).regmap, AW88399_EFRL3_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let vcalkl_val: uint16_t = (reg_val & !AW88399_EF_VSN_GESLP_L_MASK) as uint16_t;
    let mut vcalk_val = if (*aw88399).check_val == AW_EF_AND_CHECK { vcalkh_val & vcalkl_val } else { vcalkh_val | vcalkl_val };
    if (vcalk_val & AW88399_EF_VSN_GESLP_SIGN_MASK as u16) != 0 { vcalk_val |= AW88399_EF_VSN_GESLP_SIGN_NEG as u16; }
    *vcalk = vcalk_val as int16_t;
    0
}

unsafe fn aw88399_dev_get_internal_vcalk(aw88399: *mut aw88399, vcalk: *mut int16_t) -> c_int {
    let aw_dev = (*aw88399).aw_pa;
    let mut reg_val: c_uint = 0;
    let mut ret = regmap_read((*aw_dev).regmap, AW88399_EFRH2_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let vcalkh_val: uint16_t = (reg_val & !AW88399_INTERNAL_VSN_TRIM_H_MASK) as uint16_t;
    ret = regmap_read((*aw_dev).regmap, AW88399_EFRL2_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let vcalkl_val: uint16_t = (reg_val & !AW88399_INTERNAL_VSN_TRIM_L_MASK) as uint16_t;
    let mut vcalk_val = if (*aw88399).check_val == AW_EF_AND_CHECK {
        (vcalkh_val >> AW88399_INTERNAL_VSN_TRIM_H_START_BIT) & (vcalkl_val >> AW88399_INTERNAL_VSN_TRIM_L_START_BIT)
    } else {
        (vcalkh_val >> AW88399_INTERNAL_VSN_TRIM_H_START_BIT) | (vcalkl_val >> AW88399_INTERNAL_VSN_TRIM_L_START_BIT)
    };
    if (vcalk_val & !AW88399_TEM4_SIGN_MASK as u16) != 0 { vcalk_val |= AW88399_TEM4_SIGN_NEG as u16; }
    *vcalk = vcalk_val as int16_t;
    0
}

unsafe fn aw_dev_set_vcalb(aw88399: *mut aw88399) -> c_int {
    let aw_dev = (*aw88399).aw_pa;
    let mut vsense_value: c_uint = 0;
    let mut ret = regmap_read((*aw_dev).regmap, AW88399_VSNCTRL1_REG, &mut vsense_value);
    if ret != 0 { return ret; }
    let vsense_select = vsense_value & !AW88399_VDSEL_MASK;
    let mut icalk: int16_t = 0;
    let mut vcalk: int16_t = 0;
    ret = aw88399_dev_get_icalk(aw88399, &mut icalk);
    if ret != 0 {
        dev_err!((*aw_dev).dev, "get icalk failed\n");
        return ret;
    }
    let ical_k: int32_t = icalk as int32_t * AW88399_ICABLK_FACTOR + AW88399_CABL_BASE_VALUE;
    let mut vcalb: int32_t = 0;
    if vsense_select == AW88399_DEV_VDSEL_VSENSE {
        ret = aw88399_dev_get_vcalk(aw88399, &mut vcalk);
        let vcal_k = vcalk as int32_t * AW88399_VCABLK_FACTOR + AW88399_CABL_BASE_VALUE;
        vcalb = AW88399_VCALB_ACCURACY * AW88399_VSCAL_FACTOR / AW88399_ISCAL_FACTOR * ical_k / vcal_k * (*aw88399).vcalb_init_val as int32_t;
    } else if vsense_select == AW88399_DEV_VDSEL_DAC {
        ret = aw88399_dev_get_internal_vcalk(aw88399, &mut vcalk);
        let vcal_k = vcalk as int32_t * AW88399_VCABLK_DAC_FACTOR + AW88399_CABL_BASE_VALUE;
        vcalb = AW88399_VCALB_ACCURACY * AW88399_VSCAL_DAC_FACTOR / AW88399_ISCAL_DAC_FACTOR * ical_k / vcal_k * (*aw88399).vcalb_init_val as int32_t;
    } else {
        dev_err!((*aw_dev).dev, "%s: unsupported vsense\n", "aw_dev_set_vcalb");
        ret = -EINVAL;
    }
    if ret != 0 { return ret; }
    vcalb >>= AW88399_VCALB_ADJ_FACTOR;
    regmap_write((*aw_dev).regmap, AW88399_DSPVCALB_REG, vcalb as uint32_t);
    0
}

#[no_mangle]
pub unsafe extern "C" fn aw_dev_update_cali_re(cali_desc: *mut aw_cali_desc) -> c_int {
    let aw_dev = aw_device_from_cali_desc(cali_desc);
    if (*aw_dev).cali_desc.cali_re >= AW88399_CALI_RE_MAX || (*aw_dev).cali_desc.cali_re <= AW88399_CALI_RE_MIN {
        return -EINVAL;
    }
    let cali_re: u32 = AW88399_SHOW_RE_TO_DSP_RE((*aw_dev).cali_desc.cali_re + (*aw_dev).cali_desc.ra, AW88399_DSP_RE_SHIFT);
    let re_hbits: u16 = ((cali_re & !AW88399_CALI_RE_HBITS_MASK) >> AW88399_CALI_RE_HBITS_SHIFT) as u16;
    let re_lbits: u16 = ((cali_re & !AW88399_CALI_RE_LBITS_MASK) >> AW88399_CALI_RE_LBITS_SHIFT) as u16;
    let mut ret = regmap_write((*aw_dev).regmap, AW88399_ACR1_REG, re_hbits as c_uint);
    if ret != 0 {
        dev_err!((*aw_dev).dev, "set cali re error");
        return ret;
    }
    ret = regmap_write((*aw_dev).regmap, AW88399_ACR2_REG, re_lbits as c_uint);
    if ret != 0 { dev_err!((*aw_dev).dev, "set cali re error"); }
    ret
}

unsafe fn aw_dev_fw_crc_check(aw_dev: *mut aw_device) -> c_int {
    let fw_len_val: uint16_t = (((*aw_dev).dsp_fw_len / AW_FW_ADDR_LEN) - 1 + AW88399_CRC_FW_BASE_ADDR) as uint16_t;
    let mut ret = regmap_update_bits((*aw_dev).regmap, AW88399_CRCCTRL_REG, !AW88399_CRC_END_ADDR_MASK, fw_len_val as c_uint);
    if ret != 0 { return ret; }
    ret = regmap_update_bits((*aw_dev).regmap, AW88399_CRCCTRL_REG, !AW88399_CRC_CODE_EN_MASK, AW88399_CRC_CODE_EN_ENABLE_VALUE);
    usleep_range(AW88399_2000_US, AW88399_2000_US + 10);
    let mut reg_val: c_uint = 0;
    regmap_read((*aw_dev).regmap, AW88399_HAGCST_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let check_val: uint16_t = ((reg_val & !AW88399_CRC_CHECK_BITS_MASK) >> AW88399_CRC_CHECK_START_BIT) as uint16_t;
    ret = regmap_update_bits((*aw_dev).regmap, AW88399_CRCCTRL_REG, !AW88399_CRC_CODE_EN_MASK, AW88399_CRC_CODE_EN_DISABLE_VALUE);
    if ret != 0 { return ret; }
    if check_val != AW88399_CRC_CHECK_PASS_VAL as uint16_t {
        dev_err!((*aw_dev).dev, "%s failed, check_val 0x%x != 0x%x", "aw_dev_fw_crc_check", check_val, AW88399_CRC_CHECK_PASS_VAL);
        ret = -EINVAL;
    }
    ret
}

unsafe fn aw_dev_cfg_crc_check(aw_dev: *mut aw_device) -> c_int {
    let cfg_len_val: uint16_t = (((*aw_dev).dsp_cfg_len / AW_FW_ADDR_LEN) - 1 + AW88399_CRC_CFG_BASE_ADDR) as uint16_t;
    let mut ret = regmap_update_bits((*aw_dev).regmap, AW88399_CRCCTRL_REG, !AW88399_CRC_END_ADDR_MASK, cfg_len_val as c_uint);
    if ret != 0 { return ret; }
    ret = regmap_update_bits((*aw_dev).regmap, AW88399_CRCCTRL_REG, !AW88399_CRC_CFG_EN_MASK, AW88399_CRC_CFG_EN_ENABLE_VALUE);
    if ret != 0 { return ret; }
    usleep_range(AW88399_1000_US, AW88399_1000_US + 10);
    let mut reg_val: c_uint = 0;
    ret = regmap_read((*aw_dev).regmap, AW88399_HAGCST_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let check_val: uint16_t = ((reg_val & !AW88399_CRC_CHECK_BITS_MASK) >> AW88399_CRC_CHECK_START_BIT) as uint16_t;
    ret = regmap_update_bits((*aw_dev).regmap, AW88399_CRCCTRL_REG, !AW88399_CRC_CFG_EN_MASK, AW88399_CRC_CFG_EN_DISABLE_VALUE);
    if ret != 0 { return ret; }
    if check_val != AW88399_CRC_CHECK_PASS_VAL as uint16_t {
        dev_err!((*aw_dev).dev, "crc_check failed, check val 0x%x != 0x%x", check_val, AW88399_CRC_CHECK_PASS_VAL);
        ret = -EINVAL;
    }
    ret
}

unsafe fn aw_dev_hw_crc_check(aw88399: *mut aw88399) -> c_int {
    let aw_dev = (*aw88399).aw_pa;
    let mut ret = regmap_update_bits((*aw_dev).regmap, AW88399_I2SCFG1_REG, !AW88399_RAM_CG_BYP_MASK, AW88399_RAM_CG_BYP_BYPASS_VALUE);
    if ret != 0 { return ret; }
    ret = aw_dev_fw_crc_check(aw_dev);
    if ret != 0 {
        dev_err!((*aw_dev).dev, "fw_crc_check failed\n");
        regmap_update_bits((*aw_dev).regmap, AW88399_I2SCFG1_REG, !AW88399_RAM_CG_BYP_MASK, AW88399_RAM_CG_BYP_WORK_VALUE);
        return ret;
    }
    ret = aw_dev_cfg_crc_check(aw_dev);
    if ret != 0 {
        dev_err!((*aw_dev).dev, "cfg_crc_check failed\n");
        regmap_update_bits((*aw_dev).regmap, AW88399_I2SCFG1_REG, !AW88399_RAM_CG_BYP_MASK, AW88399_RAM_CG_BYP_WORK_VALUE);
        return ret;
    }
    ret = regmap_write((*aw_dev).regmap, AW88399_CRCCTRL_REG, (*aw88399).crc_init_val as c_uint);
    if ret != 0 { return ret; }
    regmap_update_bits((*aw_dev).regmap, AW88399_I2SCFG1_REG, !AW88399_RAM_CG_BYP_MASK, AW88399_RAM_CG_BYP_WORK_VALUE)
}

unsafe fn aw_dev_i2s_tx_enable(aw_dev: *mut aw_device, flag: bool) {
    let ret = if flag {
        regmap_update_bits((*aw_dev).regmap, AW88399_I2SCTRL3_REG, !AW88399_I2STXEN_MASK, AW88399_I2STXEN_ENABLE_VALUE)
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88399_I2SCFG1_REG, !AW88399_I2STXEN_MASK, AW88399_I2STXEN_DISABLE_VALUE)
    };
    if ret != 0 { dev_dbg!((*aw_dev).dev, "%s failed", "aw_dev_i2s_tx_enable"); }
}

#[no_mangle]
pub unsafe extern "C" fn aw_dev_get_dsp_status(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88399_WDT_REG, &mut reg_val);
    if ret != 0 { return ret; }
    if (reg_val & !AW88399_WDT_CNT_MASK) == 0 { return -EPERM; }
    0
}

unsafe fn aw_dev_dsp_check(aw_dev: *mut aw_device) -> c_int {
    let mut ret: c_int;
    if (*aw_dev).dsp_cfg == AW88399_DEV_DSP_BYPASS {
        dev_dbg!((*aw_dev).dev, "dsp bypass");
        ret = 0;
    } else if (*aw_dev).dsp_cfg == AW88399_DEV_DSP_WORK {
        aw_dev_dsp_enable(aw_dev, false);
        aw_dev_dsp_enable(aw_dev, true);
        usleep_range(AW88399_1000_US, AW88399_1000_US + 10);
        ret = 0;
        let mut i = 0;
        while i < AW88399_DEV_DSP_CHECK_MAX {
            ret = aw_dev_get_dsp_status(aw_dev);
            if ret != 0 {
                dev_err!((*aw_dev).dev, "dsp wdt status error=%d", ret);
                usleep_range(AW88399_2000_US, AW88399_2000_US + 10);
            }
            i += 1;
        }
    } else {
        dev_err!((*aw_dev).dev, "unknown dsp cfg=%d", (*aw_dev).dsp_cfg);
        ret = -EINVAL;
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn aw_dev_set_volume(aw_dev: *mut aw_device, value: c_uint) -> c_int {
    let vol_desc = &mut (*aw_dev).volume_desc as *mut aw_volume_desc;
    let mut real_value: u16 = core::cmp::min(value + (*vol_desc).init_volume as c_uint, AW88399_MUTE_VOL as c_uint) as u16;
    let mut reg_value: c_uint = 0;
    let mut ret = regmap_read((*aw_dev).regmap, AW88399_SYSCTRL2_REG, &mut reg_value);
    if ret != 0 { return ret; }
    dev_dbg!((*aw_dev).dev, "value 0x%x , reg:0x%x", value, real_value);
    real_value = ((real_value as c_uint) << AW88399_VOL_START_BIT | (reg_value & AW88399_VOL_MASK)) as u16;
    ret = regmap_write((*aw_dev).regmap, AW88399_SYSCTRL2_REG, real_value as c_uint);
    ret
}

unsafe fn aw_dev_fade_in(aw_dev: *mut aw_device) {
    let fade_in_vol = (*aw_dev).volume_desc.ctl_volume as c_int;
    let fade_step = (*aw_dev).fade_step;
    if fade_step == 0 || (*aw_dev).fade_in_time == 0 {
        aw_dev_set_volume(aw_dev, fade_in_vol as c_uint);
        return;
    }
    let mut i = AW88399_MUTE_VOL as c_int;
    while i >= fade_in_vol {
        aw_dev_set_volume(aw_dev, i as c_uint);
        usleep_range((*aw_dev).fade_in_time, (*aw_dev).fade_in_time + 10);
        i -= fade_step;
    }
    if i != fade_in_vol { aw_dev_set_volume(aw_dev, fade_in_vol as c_uint); }
}

unsafe fn aw_dev_fade_out(aw_dev: *mut aw_device) {
    let fade_step = (*aw_dev).fade_step;
    if fade_step == 0 || (*aw_dev).fade_out_time == 0 {
        aw_dev_set_volume(aw_dev, AW88399_MUTE_VOL as c_uint);
        return;
    }
    let mut i = (*aw_dev).volume_desc.ctl_volume as c_int;
    while i <= AW88399_MUTE_VOL as c_int {
        aw_dev_set_volume(aw_dev, i as c_uint);
        usleep_range((*aw_dev).fade_out_time, (*aw_dev).fade_out_time + 10);
        i += fade_step;
    }
    if i != AW88399_MUTE_VOL as c_int {
        aw_dev_set_volume(aw_dev, AW88399_MUTE_VOL as c_uint);
        usleep_range((*aw_dev).fade_out_time, (*aw_dev).fade_out_time + 10);
    }
}

#[no_mangle]
pub unsafe extern "C" fn aw88399_dev_mute(aw_dev: *mut aw_device, is_mute: bool) {
    if is_mute {
        aw_dev_fade_out(aw_dev);
        regmap_update_bits((*aw_dev).regmap, AW88399_SYSCTRL_REG, !AW88399_HMUTE_MASK, AW88399_HMUTE_ENABLE_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88399_SYSCTRL_REG, !AW88399_HMUTE_MASK, AW88399_HMUTE_DISABLE_VALUE);
        aw_dev_fade_in(aw_dev);
    }
}

unsafe fn aw88399_dev_set_dither(aw88399: *mut aw88399, dither: bool) {
    let aw_dev = (*aw88399).aw_pa;
    if dither {
        regmap_update_bits((*aw_dev).regmap, AW88399_DBGCTRL_REG, !AW88399_DITHER_EN_MASK, AW88399_DITHER_EN_ENABLE_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88399_DBGCTRL_REG, !AW88399_DITHER_EN_MASK, AW88399_DITHER_EN_DISABLE_VALUE);
    }
}

unsafe fn aw88399_dev_start(aw88399: *mut aw88399) -> c_int {
    let aw_dev = (*aw88399).aw_pa;
    if (*aw_dev).status == AW88399_DEV_PW_ON {
        dev_dbg!((*aw_dev).dev, "already power on");
        return 0;
    }
    aw88399_dev_set_dither(aw88399, false);
    /* power on */
    aw_dev_pwd(aw_dev, false);
    usleep_range(AW88399_2000_US, AW88399_2000_US + 10);
    let mut ret = aw_dev_check_syspll(aw_dev);
    if ret != 0 {
        dev_err!((*aw_dev).dev, "pll check failed cannot start");
        aw_dev_pwd(aw_dev, true);
        (*aw_dev).status = AW88399_DEV_PW_OFF;
        return ret;
    }
    /* amppd on */
    aw_dev_amppd(aw_dev, false);
    usleep_range(AW88399_1000_US, AW88399_1000_US + 50);
    /* check i2s status */
    ret = aw_dev_check_sysst(aw88399);
    if ret != 0 {
        dev_err!((*aw_dev).dev, "sysst check failed");
        aw_dev_clear_int_status(aw_dev);
        aw_dev_amppd(aw_dev, true);
        aw_dev_pwd(aw_dev, true);
        (*aw_dev).status = AW88399_DEV_PW_OFF;
        return ret;
    }
    if (*aw_dev).dsp_cfg == AW88399_DEV_DSP_WORK {
        ret = aw_dev_hw_crc_check(aw88399);
        if ret != 0 {
            dev_err!((*aw_dev).dev, "dsp crc check failed");
            aw_dev_dsp_enable(aw_dev, false);
            aw_dev_clear_int_status(aw_dev);
            aw_dev_amppd(aw_dev, true);
            aw_dev_pwd(aw_dev, true);
            (*aw_dev).status = AW88399_DEV_PW_OFF;
            return ret;
        }
        aw_dev_dsp_enable(aw_dev, false);
        aw_dev_set_vcalb(aw88399);
        aw_dev_update_cali_re(&mut (*aw_dev).cali_desc);
        ret = aw_dev_dsp_check(aw_dev);
        if ret != 0 {
            dev_err!((*aw_dev).dev, "dsp status check failed");
            aw_dev_dsp_enable(aw_dev, false);
            aw_dev_clear_int_status(aw_dev);
            aw_dev_amppd(aw_dev, true);
            aw_dev_pwd(aw_dev, true);
            (*aw_dev).status = AW88399_DEV_PW_OFF;
            return ret;
        }
    } else {
        dev_dbg!((*aw_dev).dev, "start pa with dsp bypass");
    }
    /* enable tx feedback */
    aw_dev_i2s_tx_enable(aw_dev, true);
    if (*aw88399).dither_st == AW88399_DITHER_EN_ENABLE_VALUE as u16 {
        aw88399_dev_set_dither(aw88399, true);
    }
    /* close mute */
    aw88399_dev_mute(aw_dev, false);
    /* clear inturrupt */
    aw_dev_clear_int_status(aw_dev);
    (*aw_dev).status = AW88399_DEV_PW_ON;
    0
}

unsafe fn aw_dev_dsp_update_container(aw_dev: *mut aw_device, data: *mut c_uchar, len: c_uint, base: c_ushort) -> c_int {
    let mut ret = regmap_write((*aw_dev).regmap, AW88399_DSPMADD_REG, base as c_uint);
    if ret != 0 { return ret; }
    let mut i: c_uint = 0;
    while i < len {
        let tmp_len = core::cmp::min(len - i, AW88399_MAX_RAM_WRITE_BYTE_SIZE);
        ret = regmap_raw_write((*aw_dev).regmap, AW88399_DSPMDAT_REG, data.add(i as usize) as *const c_void, tmp_len as usize);
        if ret != 0 { return ret; }
        i += AW88399_MAX_RAM_WRITE_BYTE_SIZE;
    }
    0
}

unsafe fn aw_dev_get_ra(cali_desc: *mut aw_cali_desc) -> c_int {
    let aw_dev = aw_device_from_cali_desc(cali_desc);
    let mut dsp_ra: u32 = 0;
    let ret = aw_dev_dsp_read(aw_dev, AW88399_DSP_REG_CFG_ADPZ_RA, &mut dsp_ra, AW_DSP_32_DATA);
    if ret != 0 {
        dev_err!((*aw_dev).dev, "read ra error");
        return ret;
    }
    (*cali_desc).ra = AW88399_DSP_RE_TO_SHOW_RE(dsp_ra, AW88399_DSP_RE_SHIFT);
    0
}

unsafe fn aw_dev_dsp_update_cfg(aw_dev: *mut aw_device, data: *mut c_uchar, len: c_uint) -> c_int {
    dev_dbg!((*aw_dev).dev, "dsp config len:%d", len);
    if len == 0 || data.is_null() {
        dev_err!((*aw_dev).dev, "dsp config data is null or len is 0");
        return -EINVAL;
    }
    let ret = aw_dev_dsp_update_container(aw_dev, data, len, AW88399_DSP_CFG_ADDR);
    if ret != 0 { return ret; }
    (*aw_dev).dsp_cfg_len = len;
    aw_dev_get_ra(&mut (*aw_dev).cali_desc)
}

unsafe fn aw_dev_dsp_update_fw(aw_dev: *mut aw_device, data: *mut c_uchar, len: c_uint) -> c_int {
    dev_dbg!((*aw_dev).dev, "dsp firmware len:%d", len);
    if len == 0 || data.is_null() {
        dev_err!((*aw_dev).dev, "dsp firmware data is null or len is 0");
        return -EINVAL;
    }
    (*aw_dev).dsp_fw_len = len;
    aw_dev_dsp_update_container(aw_dev, data, len, AW88399_DSP_FW_ADDR)
}

unsafe fn aw_dev_check_sram(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    /* read dsp_rom_check_reg */
    aw_dev_dsp_read(aw_dev, AW88399_DSP_ROM_CHECK_ADDR, &mut reg_val, AW_DSP_16_DATA);
    if reg_val != AW88399_DSP_ROM_CHECK_DATA {
        dev_err!((*aw_dev).dev, "check dsp rom failed, read[0x%x] != check[0x%x]", reg_val, AW88399_DSP_ROM_CHECK_DATA);
        return -EPERM;
    }
    /* check dsp_cfg_base_addr */
    aw_dev_dsp_write(aw_dev, AW88399_DSP_CFG_ADDR, AW88399_DSP_ODD_NUM_BIT_TEST, AW_DSP_16_DATA);
    aw_dev_dsp_read(aw_dev, AW88399_DSP_CFG_ADDR, &mut reg_val, AW_DSP_16_DATA);
    if reg_val != AW88399_DSP_ODD_NUM_BIT_TEST {
        dev_err!((*aw_dev).dev, "check dsp cfg failed, read[0x%x] != write[0x%x]", reg_val, AW88399_DSP_ODD_NUM_BIT_TEST);
        return -EPERM;
    }
    0
}

unsafe fn aw_dev_select_memclk(aw_dev: *mut aw_device, flag: c_uchar) {
    let ret;
    if flag as c_uint == AW88399_DEV_MEMCLK_PLL {
        ret = regmap_update_bits((*aw_dev).regmap, AW88399_DBGCTRL_REG, !AW88399_MEM_CLKSEL_MASK, AW88399_MEM_CLKSEL_DAPHCLK_VALUE);
        if ret != 0 { dev_err!((*aw_dev).dev, "memclk select pll failed"); }
    } else if flag as c_uint == AW88399_DEV_MEMCLK_OSC {
        ret = regmap_update_bits((*aw_dev).regmap, AW88399_DBGCTRL_REG, !AW88399_MEM_CLKSEL_MASK, AW88399_MEM_CLKSEL_OSCCLK_VALUE);
        if ret != 0 { dev_err!((*aw_dev).dev, "memclk select OSC failed"); }
    } else {
        dev_err!((*aw_dev).dev, "unknown memclk config, flag=0x%x", flag);
    }
}

unsafe fn aw_dev_get_cur_mode_st(aw_dev: *mut aw_device) {
    let profctrl_desc = &mut (*aw_dev).profctrl_desc as *mut aw_profctrl_desc;
    let mut reg_val: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88399_SYSCTRL_REG, &mut reg_val);
    if ret != 0 {
        dev_dbg!((*aw_dev).dev, "%s failed", "aw_dev_get_cur_mode_st");
        return;
    }
    if (reg_val & !AW88399_RCV_MODE_MASK) == AW88399_RCV_MODE_RECEIVER_VALUE {
        (*profctrl_desc).cur_mode = AW88399_RCV_MODE;
    } else {
        (*profctrl_desc).cur_mode = AW88399_NOT_RCV_MODE;
    }
}

unsafe fn aw_dev_update_reg_container(aw88399: *mut aw88399, data: *mut c_uchar, len: c_uint) -> c_int {
    let aw_dev = (*aw88399).aw_pa;
    let vol_desc = &mut (*aw_dev).volume_desc as *mut aw_volume_desc;
    let reg_data = data as *mut int16_t;
    let data_len: c_int = (len >> 1) as c_int;
    if (data_len & 0x1) != 0 {
        dev_err!((*aw_dev).dev, "data len:%d unsupported", data_len);
        return -EINVAL;
    }
    let mut i: c_int = 0;
    while i < data_len {
        let reg_addr: u8 = *reg_data.offset(i as isize) as u8;
        let mut reg_val: u16 = *reg_data.offset((i + 1) as isize) as u16;
        if reg_addr as c_uint == AW88399_DSPVCALB_REG {
            (*aw88399).vcalb_init_val = reg_val;
            i += 2;
            continue;
        }
        if reg_addr as c_uint == AW88399_SYSCTRL_REG {
            if (reg_val as c_uint & !AW88399_DSPBY_MASK) != 0 {
                (*aw_dev).dsp_cfg = AW88399_DEV_DSP_BYPASS;
            } else {
                (*aw_dev).dsp_cfg = AW88399_DEV_DSP_WORK;
            }
            reg_val &= (AW88399_HMUTE_MASK | AW88399_PWDN_MASK | AW88399_DSPBY_MASK) as u16;
            reg_val |= (AW88399_HMUTE_ENABLE_VALUE | AW88399_PWDN_POWER_DOWN_VALUE | AW88399_DSPBY_BYPASS_VALUE) as u16;
        }
        if reg_addr as c_uint == AW88399_I2SCTRL3_REG {
            reg_val &= AW88399_I2STXEN_MASK as u16;
            reg_val |= AW88399_I2STXEN_DISABLE_VALUE as u16;
        }
        if reg_addr as c_uint == AW88399_SYSCTRL2_REG {
            let read_vol = ((reg_val as c_uint & !AW88399_VOL_MASK) >> AW88399_VOL_START_BIT) as u16;
            (*aw_dev).volume_desc.init_volume = read_vol;
        }
        if reg_addr as c_uint == AW88399_DBGCTRL_REG {
            if (reg_val as c_uint & !AW88399_EF_DBMD_MASK) == AW88399_EF_DBMD_OR_VALUE {
                (*aw88399).check_val = AW_EF_OR_CHECK;
            } else {
                (*aw88399).check_val = AW_EF_AND_CHECK;
            }
            (*aw88399).dither_st = (reg_val as c_uint & !AW88399_DITHER_EN_MASK) as u16;
        }
        if reg_addr as c_uint == AW88399_CRCCTRL_REG {
            (*aw88399).crc_init_val = reg_val;
        }
        let ret = regmap_write((*aw_dev).regmap, reg_addr as c_uint, reg_val as c_uint);
        if ret != 0 { return ret; }
        i += 2;
    }
    aw_dev_pwd(aw_dev, false);
    usleep_range(AW88399_1000_US, AW88399_1000_US + 10);
    aw_dev_get_cur_mode_st(aw_dev);
    if (*aw_dev).prof_cur != (*aw_dev).prof_index {
        (*vol_desc).ctl_volume = 0;
    } else {
        aw_dev_set_volume(aw_dev, (*vol_desc).ctl_volume as c_uint);
    }
    0
}

unsafe fn aw_dev_reg_update(aw88399: *mut aw88399, data: *mut c_uchar, len: c_uint) -> c_int {
    if len == 0 || data.is_null() {
        dev_err!((*(*aw88399).aw_pa).dev, "reg data is null or len is 0");
        return -EINVAL;
    }
    let ret = aw_dev_update_reg_container(aw88399, data, len);
    if ret != 0 { dev_err!((*(*aw88399).aw_pa).dev, "reg update failed"); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn aw88399_dev_get_prof_name(aw_dev: *mut aw_device, index: c_int, prof_name: *mut *mut c_char) -> c_int {
    let prof_info = &mut (*aw_dev).prof_info as *mut aw_prof_info;
    if index >= (*aw_dev).prof_info.count || index < 0 {
        dev_err!((*aw_dev).dev, "index[%d] overflow count[%d]", index, (*aw_dev).prof_info.count);
        return -EINVAL;
    }
    let prof_desc = (*aw_dev).prof_info.prof_desc.offset(index as isize);
    *prof_name = *(*prof_info).prof_name_list.offset((*prof_desc).id as isize);
    0
}

unsafe fn aw88399_dev_get_prof_data(aw_dev: *mut aw_device, index: c_int, prof_desc: *mut *mut aw_prof_desc) -> c_int {
    if index >= (*aw_dev).prof_info.count || index < 0 {
        dev_err!((*aw_dev).dev, "%s: index[%d] overflow count[%d]\n", "aw88399_dev_get_prof_data", index, (*aw_dev).prof_info.count);
        return -EINVAL;
    }
    *prof_desc = (*aw_dev).prof_info.prof_desc.offset(index as isize);
    0
}

unsafe fn aw88399_dev_fw_update(aw88399: *mut aw88399, up_dsp_fw_en: bool, force_up_en: bool) -> c_int {
    let aw_dev = (*aw88399).aw_pa;
    if (*aw_dev).prof_cur == (*aw_dev).prof_index && force_up_en == AW88399_FORCE_UPDATE_OFF {
        dev_dbg!((*aw_dev).dev, "scene no change, not update");
        return 0;
    }
    if (*aw_dev).fw_status == AW88399_DEV_FW_FAILED {
        dev_err!((*aw_dev).dev, "fw status[%d] error", (*aw_dev).fw_status);
        return -EPERM;
    }
    let mut prof_name: *mut c_char = ptr::null_mut();
    let mut ret = aw88399_dev_get_prof_name(aw_dev, (*aw_dev).prof_index, &mut prof_name);
    if ret != 0 { return ret; }
    dev_dbg!((*aw_dev).dev, "start update %s", prof_name);
    let mut prof_index_desc: *mut aw_prof_desc = ptr::null_mut();
    ret = aw88399_dev_get_prof_data(aw_dev, (*aw_dev).prof_index, &mut prof_index_desc);
    if ret != 0 { return ret; }
    /* update reg */
    let sec_desc = (*prof_index_desc).sec_desc;
    ret = aw_dev_reg_update(aw88399, (*sec_desc.add(AW88395_DATA_TYPE_REG as usize)).data, (*sec_desc.add(AW88395_DATA_TYPE_REG as usize)).len);
    if ret != 0 {
        dev_err!((*aw_dev).dev, "update reg failed");
        return ret;
    }
    aw88399_dev_mute(aw_dev, true);
    if (*aw_dev).dsp_cfg == AW88399_DEV_DSP_WORK { aw_dev_dsp_enable(aw_dev, false); }
    aw_dev_select_memclk(aw_dev, AW88399_DEV_MEMCLK_OSC as c_uchar);
    ret = aw_dev_check_sram(aw_dev);
    if ret != 0 {
        dev_err!((*aw_dev).dev, "check sram failed");
        aw_dev_select_memclk(aw_dev, AW88399_DEV_MEMCLK_PLL as c_uchar);
        return ret;
    }
    if up_dsp_fw_en {
        dev_dbg!((*aw_dev).dev, "fw_ver: [%x]", (*prof_index_desc).fw_ver);
        ret = aw_dev_dsp_update_fw(aw_dev, (*sec_desc.add(AW88395_DATA_TYPE_DSP_FW as usize)).data, (*sec_desc.add(AW88395_DATA_TYPE_DSP_FW as usize)).len);
        if ret != 0 {
            dev_err!((*aw_dev).dev, "update dsp fw failed");
            aw_dev_select_memclk(aw_dev, AW88399_DEV_MEMCLK_PLL as c_uchar);
            return ret;
        }
    }
    /* update dsp config */
    ret = aw_dev_dsp_update_cfg(aw_dev, (*sec_desc.add(AW88395_DATA_TYPE_DSP_CFG as usize)).data, (*sec_desc.add(AW88395_DATA_TYPE_DSP_CFG as usize)).len);
    if ret != 0 {
        dev_err!((*aw_dev).dev, "update dsp cfg failed");
        aw_dev_select_memclk(aw_dev, AW88399_DEV_MEMCLK_PLL as c_uchar);
        return ret;
    }
    aw_dev_select_memclk(aw_dev, AW88399_DEV_MEMCLK_PLL as c_uchar);
    (*aw_dev).prof_cur = (*aw_dev).prof_index;
    0
}

unsafe fn aw88399_start_pa(aw88399: *mut aw88399) {
    let mut i = 0;
    while i < AW88399_START_RETRIES {
        let mut ret = aw88399_dev_start(aw88399);
        if ret != 0 {
            dev_err!((*(*aw88399).aw_pa).dev, "aw88399 device start failed. retry = %d", i);
            ret = aw88399_dev_fw_update(aw88399, AW88399_DSP_FW_UPDATE_ON, true);
            if ret != 0 {
                dev_err!((*(*aw88399).aw_pa).dev, "fw update failed");
                i += 1;
                continue;
            }
        } else {
            dev_dbg!((*(*aw88399).aw_pa).dev, "start success\n");
            break;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn aw88399_startup_work(work: *mut work_struct) {
    let aw88399 = aw88399_from_start_work(work);
    mutex_lock(&mut (*aw88399).lock);
    aw88399_start_pa(aw88399);
    mutex_unlock(&mut (*aw88399).lock);
}

#[no_mangle]
pub unsafe extern "C" fn aw88399_start(aw88399: *mut aw88399, sync_start: bool) {
    if (*(*aw88399).aw_pa).fw_status != AW88399_DEV_FW_OK { return; }
    if (*(*aw88399).aw_pa).status == AW88399_DEV_PW_ON { return; }
    let ret = aw88399_dev_fw_update(aw88399, if (*aw88399).fw_needs_reload { AW88399_DSP_FW_UPDATE_ON } else { AW88399_DSP_FW_UPDATE_OFF }, true);
    if ret != 0 {
        dev_err!((*(*aw88399).aw_pa).dev, "fw update failed.");
        return;
    }
    (*aw88399).fw_needs_reload = false;
    if sync_start == AW88399_SYNC_START {
        aw88399_start_pa(aw88399);
    } else {
        queue_delayed_work(system_dfl_wq, &mut (*aw88399).start_work, AW88399_START_WORK_DELAY_MS);
    }
}

unsafe fn aw_dev_check_sysint(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: u16 = 0;
    aw_dev_get_int_status(aw_dev, &mut reg_val);
    if (reg_val as c_uint & AW88399_BIT_SYSINT_CHECK) != 0 {
        dev_err!((*aw_dev).dev, "pa stop check fail:0x%04x", reg_val);
        return -EINVAL;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn aw88399_stop(aw_dev: *mut aw_device) -> c_int {
    let dsp_cfg = &mut *(*(*aw_dev).prof_info.prof_desc.offset((*aw_dev).prof_cur as isize)).sec_desc.add(AW88395_DATA_TYPE_DSP_CFG as usize);
    let dsp_fw = &mut *(*(*aw_dev).prof_info.prof_desc.offset((*aw_dev).prof_cur as isize)).sec_desc.add(AW88395_DATA_TYPE_DSP_FW as usize);
    if (*aw_dev).status == AW88399_DEV_PW_OFF {
        dev_dbg!((*aw_dev).dev, "already power off");
        return 0;
    }
    (*aw_dev).status = AW88399_DEV_PW_OFF;
    aw88399_dev_mute(aw_dev, true);
    usleep_range(AW88399_4000_US, AW88399_4000_US + 100);
    aw_dev_i2s_tx_enable(aw_dev, false);
    usleep_range(AW88399_1000_US, AW88399_1000_US + 100);
    let int_st = aw_dev_check_sysint(aw_dev);
    aw_dev_dsp_enable(aw_dev, false);
    aw_dev_amppd(aw_dev, true);
    if int_st != 0 {
        aw_dev_select_memclk(aw_dev, AW88399_DEV_MEMCLK_OSC as c_uchar);
        aw_dev_dsp_update_fw(aw_dev, dsp_fw.data, dsp_fw.len);
        aw_dev_dsp_update_cfg(aw_dev, dsp_cfg.data, dsp_cfg.len);
        aw_dev_select_memclk(aw_dev, AW88399_DEV_MEMCLK_PLL as c_uchar);
    }
    aw_dev_pwd(aw_dev, true);
    0
}

unsafe fn aw88399_dev_init(aw88399: *mut aw88399, aw_cfg: *mut aw_container) -> c_int {
    let aw_dev = (*aw88399).aw_pa;
    let mut ret = aw88395_dev_cfg_load(aw_dev, aw_cfg);
    if ret != 0 {
        dev_err!((*aw_dev).dev, "aw_dev acf parse failed");
        return -EINVAL;
    }
    (*aw_dev).fade_in_time = AW88399_1000_US / 10;
    (*aw_dev).fade_out_time = AW88399_1000_US >> 1;
    (*aw_dev).prof_cur = (*(*aw_dev).prof_info.prof_desc.add(0)).id;
    (*aw_dev).prof_index = (*(*aw_dev).prof_info.prof_desc.add(0)).id;
    ret = aw88399_dev_fw_update(aw88399, AW88399_FORCE_UPDATE_ON, AW88399_DSP_FW_UPDATE_ON);
    if ret != 0 {
        dev_err!((*aw_dev).dev, "fw update failed ret = %d\n", ret);
        return ret;
    }
    aw88399_dev_mute(aw_dev, true);
    /* close tx feedback */
    aw_dev_i2s_tx_enable(aw_dev, false);
    usleep_range(AW88399_1000_US, AW88399_1000_US + 100);
    /* enable amppd */
    aw_dev_amppd(aw_dev, true);
    /* close dsp */
    aw_dev_dsp_enable(aw_dev, false);
    /* set power down */
    aw_dev_pwd(aw_dev, true);
    0
}

#[no_mangle]
pub unsafe extern "C" fn aw88399_request_firmware_file(aw88399: *mut aw88399) -> c_int {
    let mut cont: *const firmware = ptr::null();
    (*(*aw88399).aw_pa).fw_status = AW88399_DEV_FW_FAILED;
    let mut ret = request_firmware(&mut cont, AW88399_ACF_FILE, (*(*aw88399).aw_pa).dev);
    if ret != 0 {
        dev_err!((*(*aw88399).aw_pa).dev, "request [%s] failed!", AW88399_ACF_FILE);
        return ret;
    }
    dev_dbg!((*(*aw88399).aw_pa).dev, "loaded %s - size: %zu\n", AW88399_ACF_FILE, if !cont.is_null() { (*cont).size } else { 0 });
    let alloc_size = core::mem::size_of::<aw_container>() + (*cont).size;
    (*aw88399).aw_cfg = devm_kzalloc((*(*aw88399).aw_pa).dev, alloc_size, GFP_KERNEL) as *mut aw_container;
    if (*aw88399).aw_cfg.is_null() { return -ENOMEM; }
    (*(*aw88399).aw_cfg).len = (*cont).size as c_int;
    memcpy((*(*aw88399).aw_cfg).data.as_mut_ptr() as *mut c_void, (*cont).data as *const c_void, (*cont).size);
    ret = aw88395_dev_load_acf_check((*aw88399).aw_pa, (*aw88399).aw_cfg);
    if ret != 0 {
        dev_err!((*(*aw88399).aw_pa).dev, "load [%s] failed!", AW88399_ACF_FILE);
        return ret;
    }
    mutex_lock(&mut (*aw88399).lock);
    /* aw device init */
    ret = aw88399_dev_init(aw88399, (*aw88399).aw_cfg);
    if ret != 0 { dev_err!((*(*aw88399).aw_pa).dev, "dev init failed"); }
    mutex_unlock(&mut (*aw88399).lock);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn aw88399_hw_reset(aw88399: *mut aw88399) {
    if !(*aw88399).reset_gpio.is_null() {
        gpiod_set_value_cansleep((*aw88399).reset_gpio, 1);
        usleep_range(AW88399_1000_US, AW88399_1000_US + 10);
        gpiod_set_value_cansleep((*aw88399).reset_gpio, 0);
        usleep_range(AW88399_1000_US, AW88399_1000_US + 10);
        gpiod_set_value_cansleep((*aw88399).reset_gpio, 1);
        usleep_range(AW88399_1000_US, AW88399_1000_US + 10);
    }
}

unsafe fn aw88399_parse_channel_dt(aw_dev: *mut aw_device) {
    let np = (*(*aw_dev).dev).of_node;
    let mut channel_value: u32 = 0;
    let ret = of_property_read_u32(np, b"awinic,audio-channel\0".as_ptr() as *const c_char, &mut channel_value);
    if ret != 0 {
        /*
         * On ACPI systems, DT properties don't exist. Derive channel
         * from I2C address: 0x34 -> channel 0 (left), 0x35 -> channel 1 (right)
         */
        (*aw_dev).channel = (*(*aw_dev).i2c).addr - 0x34;
        dev_dbg!((*aw_dev).dev, "DT channel property not found, using I2C address-based channel %d (addr 0x%02x)\n", (*aw_dev).channel, (*(*aw_dev).i2c).addr);
        return;
    }
    (*aw_dev).channel = channel_value;
}

#[no_mangle]
pub unsafe extern "C" fn aw88399_init(aw88399: *mut aw88399, i2c: *mut i2c_client, regmap: *mut regmap) -> c_int {
    let mut chip_id: c_uint = 0;
    let mut ret = regmap_read(regmap, AW88399_ID_REG, &mut chip_id);
    if ret != 0 {
        dev_err!(&mut (*i2c).dev, "%s read chipid error. ret = %d", "aw88399_init", ret);
        return ret;
    }
    if chip_id != AW88399_CHIP_ID {
        dev_err!(&mut (*i2c).dev, "unsupported device");
        return -ENXIO;
    }
    dev_dbg!(&mut (*i2c).dev, "chip id = %x\n", chip_id);
    let aw_dev = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<aw_device>(), GFP_KERNEL) as *mut aw_device;
    if aw_dev.is_null() { return -ENOMEM; }
    (*aw88399).aw_pa = aw_dev;
    (*aw_dev).i2c = i2c;
    (*aw_dev).dev = &mut (*i2c).dev;
    (*aw_dev).regmap = regmap;
    mutex_init(&mut (*aw_dev).dsp_lock);
    (*aw_dev).chip_id = chip_id;
    (*aw_dev).acf = ptr::null_mut();
    (*aw_dev).prof_info.prof_desc = ptr::null_mut();
    (*aw_dev).prof_info.count = 0;
    (*aw_dev).prof_info.prof_type = AW88395_DEV_NONE_TYPE_ID;
    (*aw_dev).channel = AW88399_DEV_DEFAULT_CH;
    (*aw_dev).fw_status = AW88399_DEV_FW_FAILED;
    (*aw_dev).fade_step = AW88399_VOLUME_STEP_DB;
    (*aw_dev).volume_desc.ctl_volume = AW88399_VOL_DEFAULT_VALUE;
    aw88399_parse_channel_dt(aw_dev);
    0
}

#[no_mangle]
pub unsafe extern "C" fn aw88399_dev_set_channel(aw88399: *mut aw88399, channel: c_int) {
    (*(*aw88399).aw_pa).channel = channel as c_uint;
}

// MODULE_DESCRIPTION("AW88399 common device library");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
