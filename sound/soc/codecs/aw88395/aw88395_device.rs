// SPDX-License-Identifier: GPL-2.0-only
//
// aw88395_device.c --  AW88395 function for ALSA Audio Driver
//
// Copyright (c) 2022-2023 AWINIC Technology CO., LTD
//
// Author: Bruce zhao <zhaolei@awinic.com>
// Author: Ben Yi <yijiangtao@awinic.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ushort, c_uchar, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type u8 = c_uchar;
type u16 = c_ushort;
type u32 = c_uint;
type int16_t = i16;
type bool_t = bool;

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct aw_container {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aw_sec_data_desc {
    pub data: *mut u8,
    pub len: u32,
}

#[repr(C)]
pub struct aw_prof_desc {
    pub id: c_int,
    pub fw_ver: u32,
    pub sec_desc: *mut aw_sec_data_desc,
}

#[repr(C)]
pub struct aw_prof_info {
    pub prof_desc: *mut aw_prof_desc,
    pub count: c_int,
    pub prof_type: c_int,
    pub prof_name_list: *mut *mut c_char,
}

#[repr(C)]
pub struct aw_volume_desc {
    pub init_volume: u16,
    pub ctl_volume: u16,
}

#[repr(C)]
pub struct aw_cali_desc {
    pub cali_re: c_int,
    pub ra: c_int,
}

#[repr(C)]
pub struct aw_cali_delay_desc {
    pub delay: u32,
}

#[repr(C)]
pub struct aw_vmax_desc {
    pub init_vmax: u32,
}

#[repr(C)]
pub struct aw_profctrl_desc {
    pub cur_mode: c_int,
}

#[repr(C)]
pub struct aw_device {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub i2c: *mut i2c_client,
    pub dsp_lock: mutex,
    pub volume_desc: aw_volume_desc,
    pub crc_dsp_cfg: aw_sec_data_desc,
    pub cali_desc: aw_cali_desc,
    pub cali_delay_desc: aw_cali_delay_desc,
    pub profctrl_desc: aw_profctrl_desc,
    pub vmax_desc: aw_vmax_desc,
    pub prof_info: aw_prof_info,
    pub chip_id: u16,
    pub acf: *mut c_void,
    pub channel: u32,
    pub fw_status: c_int,
    pub fade_step: c_int,
    pub fade_in_time: u32,
    pub fade_out_time: u32,
    pub prof_cur: c_int,
    pub prof_index: c_int,
    pub dsp_cfg: u8,
    pub dsp_crc_st: c_int,
    pub dsp_fw_len: u32,
    pub dsp_cfg_len: u32,
    pub status: c_int,
}

unsafe extern "C" {
    static AW88395_DSPMADD_REG: u32;
    static AW88395_DSPMDAT_REG: u32;
    static AW88395_ID_REG: u32;
    static AW88395_CHIP_ID_REG: u32;
    static AW88395_DSP_16_DATA_MASK: u32;
    static AW_DSP_16_DATA: u8;
    static AW_DSP_32_DATA: u8;
    static AW88395_VOL_6DB_START: u32;
    static AW88395_VOLUME_STEP_DB: u16;
    static AW88395_DSP_FW_ADDR: u16;
    static AW88395_DATA_TYPE_DSP_FW: usize;
    static AW88395_FW_CHECK_PART: c_int;
    static AW88395_MUTE_VOL: u16;
    static AW88395_SYSCTRL2_REG: u32;
    static AW88395_VOL_START_BIT: u32;
    static AW88395_VOL_MASK: u16;
    static AW88395_DSP_CFG_ADDR: u32;
    static AW88395_DSP_REG_CFG_ADPZ_RE: u32;
    static AW88395_DSP_RE_SHIFT: u32;
    static AW88395_I2SCFG1_REG: u32;
    static AW88395_I2STXEN_MASK: u32;
    static AW88395_I2STXEN_ENABLE_VALUE: u32;
    static AW88395_I2STXEN_DISABLE_VALUE: u32;
    static AW88395_DSP_REG_CRC_ADDR: u32;
    static AW88395_HAGCCFG7_REG: u32;
    static AW88395_AGC_DSP_CTL_MASK: u32;
    static AW88395_AGC_DSP_CTL_ENABLE_VALUE: u32;
    static AW88395_AGC_DSP_CTL_DISABLE_VALUE: u32;
    static AW88395_DSP_ST_CHECK_MAX: c_int;
    static AW88395_SYSST_REG: u32;
    static AW88395_DSPS_MASK: u32;
    static AW88395_DSPS_NORMAL_VALUE: u32;
    static AW88395_SYSCTRL_REG: u32;
    static AW88395_DSPBY_MASK: u32;
    static AW88395_DSPBY_WORKING_VALUE: u32;
    static AW88395_DSPBY_BYPASS_VALUE: u32;
    static AW88395_DEV_DSP_BYPASS: u8;
    static AW88395_DEV_DSP_WORK: u8;
    static AW88395_5000_US: u32;
    static AW88395_DSP_CRC_OK: c_int;
    static AW88395_PWDN_MASK: u32;
    static AW88395_PWDN_POWER_DOWN_VALUE: u32;
    static AW88395_PWDN_WORKING_VALUE: u32;
    static AW88395_AMPPD_MASK: u32;
    static AW88395_AMPPD_POWER_DOWN_VALUE: u32;
    static AW88395_AMPPD_WORKING_VALUE: u32;
    static AW88395_HMUTE_MASK: u32;
    static AW88395_HMUTE_ENABLE_VALUE: u32;
    static AW88395_HMUTE_DISABLE_VALUE: u32;
    static AW88395_EFRM2_REG: u32;
    static AW88395_EF_ISN_GESLP_MASK: u16;
    static AW88395_EF_ISN_GESLP_SIGN_MASK: u16;
    static AW88395_EF_ISN_GESLP_SIGN_NEG: u16;
    static AW88395_EFRH_REG: u32;
    static AW88395_EF_VSENSE_GAIN_SHIFT: u32;
    static AW88395_EF_VSN_GESLP_MASK: u16;
    static AW88395_EF_VSN_GESLP_SIGN_MASK: u16;
    static AW88395_EF_VSN_GESLP_SIGN_NEG: u16;
    static AW88395_EF_DAC_GESLP_SHIFT: u32;
    static AW88395_EF_DAC_GESLP_SIGN_MASK: u16;
    static AW88395_EF_DAC_GESLP_SIGN_NEG: u16;
    static AW88395_I2SCFG3_REG: u32;
    static AW88395_VDSEL_MASK: u32;
    static AW88395_DEV_VDSEL_VSENSE: c_int;
    static AW88395_DEV_VDSEL_DAC: c_int;
    static AW88395_DSP_REG_VCALB: u16;
    static AW88395_CABL_BASE_VALUE: c_int;
    static AW88395_ICABLK_FACTOR: c_int;
    static AW88395_VCABLK_FACTOR: c_int;
    static AW88395_VCAL_FACTOR: c_int;
    static AW88395_VSCAL_FACTOR: c_int;
    static AW88395_ISCAL_FACTOR: c_int;
    static AW88395_VCABLK_FACTOR_DAC: c_int;
    static AW88395_VSCAL_FACTOR_DAC: c_int;
    static AW88395_VCALB_ADJ_FACTOR: u32;
    static AW88395_DSP_CALI_F0_DELAY: u16;
    static AW88395_SYSINT_REG: u32;
    static AW88395_BIT_PLL_CHECK: u32;
    static AW88395_DEV_SYSST_CHECK_MAX: c_int;
    static AW88395_2000_US: u32;
    static AW88395_PLLCTRL1_REG: u32;
    static AW88395_CCO_MUX_MASK: u32;
    static AW88395_CCO_MUX_DIVIDED_VALUE: u32;
    static AW88395_CCO_MUX_BYPASS_VALUE: u32;
    static AW88395_BIT_SYSST_CHECK_MASK: u32;
    static AW88395_BIT_SYSST_CHECK: u32;
    static AW88395_BIT_SYSINT_CHECK: u16;
    static AW88395_RCV_MODE_MASK: u32;
    static AW88395_RCV_MODE_RECEIVER_VALUE: u32;
    static AW88395_RCV_MODE: c_int;
    static AW88395_NOT_RCV_MODE: c_int;
    static AW88395_DEV_MEMCLK_PLL: u8;
    static AW88395_DEV_MEMCLK_OSC: u8;
    static AW88395_DBGCTRL_REG: u32;
    static AW88395_MEM_CLKSEL_MASK: u32;
    static AW88395_MEM_CLKSEL_DAP_HCLK_VALUE: u32;
    static AW88395_MEM_CLKSEL_OSC_CLK_VALUE: u32;
    static AW88395_WDT_REG: u32;
    static AW88395_WDT_CNT_MASK: u32;
    static AW88395_DSP_REG_VMAX: u16;
    static AW88395_DSP_REG_CFG_ADPZ_RA: u16;
    static AW88395_MAX_RAM_WRITE_BYTE_SIZE: u32;
    static AW88395_DSP_CRC_NA: c_int;
    static AW88395_DSP_ODD_NUM_BIT_TEST: u16;
    static AW88395_DSP_EVEN_NUM_BIT_TEST: u16;
    static AW88395_FORCE_UPDATE_OFF: bool;
    static AW88395_DEV_FW_FAILED: c_int;
    static AW88395_DATA_TYPE_REG: usize;
    static AW88395_DATA_TYPE_DSP_CFG: usize;
    static AW88395_DEV_DSP_CHECK_MAX: c_int;
    static AW88395_1000_US: u32;
    static AW88395_CALI_RE_MAX: c_int;
    static AW88395_CALI_RE_MIN: c_int;
    static AW88395_DEV_PW_ON: c_int;
    static AW88395_DEV_PW_OFF: c_int;
    static AW88395_4000_US: u32;
    static AW88395_FORCE_UPDATE_ON: bool;
    static AW88395_DSP_FW_UPDATE_ON: bool;
    static AW88395_CHIP_ID: u16;
    static AW88395_DEV_DEFAULT_CH: u32;
    static AW88395_DEV_NONE_TYPE_ID: c_int;
    static AW88395_VOL_DEFAULT_VALUE: u16;
    static GFP_KERNEL: u32;
    static EINVAL: c_int;
    static EPERM: c_int;
    static EIO: c_int;
    static ENOMEM: c_int;

    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> c_int;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> c_int;
    fn regmap_raw_write(map: *mut regmap, reg: u32, data: *const c_void, len: u32) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...) -> c_int;
    fn usleep_range(min: u32, max: u32);
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn crc32c(crc: u32, data: *const u8, len: u32) -> u32;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn devm_kfree(dev: *mut device, p: *mut c_void);
    fn swab16_array(buf: *mut u16, words: u32);
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out: *mut u32) -> c_int;
    fn aw88395_dev_cfg_load(aw_dev: *mut aw_device, aw_cfg: *mut aw_container) -> c_int;
}

unsafe fn min_u32(a: u32, b: u32) -> u32 {
    if a < b { a } else { b }
}

unsafe fn be16_to_cpup(p: *const c_void) -> u16 {
    u16::from_be(ptr::read_unaligned(p as *const u16))
}

unsafe fn cpu_to_le16(v: u16) -> u16 {
    v.to_le()
}

unsafe fn cpu_to_le32(v: u32) -> u32 {
    v.to_le()
}

unsafe fn cpu_to_be16p(p: *const u16) -> u16 {
    ptr::read_unaligned(p).to_be()
}

unsafe fn AW88395_SHOW_RE_TO_DSP_RE(value: c_int, shift: u32) -> u32 {
    ((value as u32) << shift) as u32
}

unsafe fn AW88395_DSP_RE_TO_SHOW_RE(value: u32, shift: u32) -> c_int {
    (value >> shift) as c_int
}

unsafe fn AW88395_CALI_DELAY_CACL(value: u32) -> u32 {
    value
}

unsafe fn aw_dev_dsp_write_16bit(
    aw_dev: *mut aw_device,
    dsp_addr: c_ushort,
    dsp_data: c_uint,
) -> c_int {
    let mut ret: c_int;

    ret = regmap_write((*aw_dev).regmap, AW88395_DSPMADD_REG, dsp_addr as u32);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"%s write addr error, ret=%d".as_ptr(), c"aw_dev_dsp_write_16bit".as_ptr(), ret);
        return ret;
    }

    ret = regmap_write((*aw_dev).regmap, AW88395_DSPMDAT_REG, dsp_data as u16 as u32);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"%s write data error, ret=%d".as_ptr(), c"aw_dev_dsp_write_16bit".as_ptr(), ret);
        return ret;
    }

    0
}

unsafe fn aw_dev_dsp_write_32bit(
    aw_dev: *mut aw_device,
    dsp_addr: c_ushort,
    dsp_data: c_uint,
) -> c_int {
    let mut temp_data: u16;
    let mut ret: c_int;

    ret = regmap_write((*aw_dev).regmap, AW88395_DSPMADD_REG, dsp_addr as u32);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"%s write addr error, ret=%d".as_ptr(), c"aw_dev_dsp_write_32bit".as_ptr(), ret);
        return ret;
    }

    temp_data = (dsp_data & AW88395_DSP_16_DATA_MASK) as u16;
    ret = regmap_write((*aw_dev).regmap, AW88395_DSPMDAT_REG, temp_data as u32);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"%s write datal error, ret=%d".as_ptr(), c"aw_dev_dsp_write_32bit".as_ptr(), ret);
        return ret;
    }

    temp_data = (dsp_data >> 16) as u16;
    ret = regmap_write((*aw_dev).regmap, AW88395_DSPMDAT_REG, temp_data as u32);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"%s write datah error, ret=%d".as_ptr(), c"aw_dev_dsp_write_32bit".as_ptr(), ret);
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn aw_dev_dsp_write(
    aw_dev: *mut aw_device,
    dsp_addr: c_ushort,
    dsp_data: c_uint,
    data_type: c_uchar,
) -> c_int {
    let mut reg_value: u32 = 0;
    let ret: c_int;

    mutex_lock(&mut (*aw_dev).dsp_lock);
    match data_type {
        x if x == AW_DSP_16_DATA => {
            ret = aw_dev_dsp_write_16bit(aw_dev, dsp_addr, dsp_data);
            if ret != 0 {
                dev_err((*aw_dev).dev, c"write dsp_addr[0x%x] 16-bit dsp_data[0x%x] failed".as_ptr(), dsp_addr as u32, dsp_data);
            }
        }
        x if x == AW_DSP_32_DATA => {
            ret = aw_dev_dsp_write_32bit(aw_dev, dsp_addr, dsp_data);
            if ret != 0 {
                dev_err((*aw_dev).dev, c"write dsp_addr[0x%x] 32-bit dsp_data[0x%x] failed".as_ptr(), dsp_addr as u32, dsp_data);
            }
        }
        _ => {
            dev_err((*aw_dev).dev, c"data type[%d] unsupported".as_ptr(), data_type as c_int);
            ret = -EINVAL;
        }
    }

    /* clear dsp chip select state*/
    if regmap_read((*aw_dev).regmap, AW88395_ID_REG, &mut reg_value) != 0 {
        dev_err((*aw_dev).dev, c"%s fail to clear chip state. Err=%d\n".as_ptr(), c"aw_dev_dsp_write".as_ptr(), ret);
    }
    mutex_unlock(&mut (*aw_dev).dsp_lock);

    ret
}

unsafe fn aw_dev_dsp_read_16bit(
    aw_dev: *mut aw_device,
    dsp_addr: c_ushort,
    dsp_data: *mut c_uint,
) -> c_int {
    let mut temp_data: c_uint = 0;
    let mut ret: c_int;

    ret = regmap_write((*aw_dev).regmap, AW88395_DSPMADD_REG, dsp_addr as u32);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"%s write error, ret=%d".as_ptr(), c"aw_dev_dsp_read_16bit".as_ptr(), ret);
        return ret;
    }

    ret = regmap_read((*aw_dev).regmap, AW88395_DSPMDAT_REG, &mut temp_data);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"%s read error, ret=%d".as_ptr(), c"aw_dev_dsp_read_16bit".as_ptr(), ret);
        return ret;
    }
    *dsp_data = temp_data;

    0
}

unsafe fn aw_dev_dsp_read_32bit(
    aw_dev: *mut aw_device,
    dsp_addr: c_ushort,
    dsp_data: *mut c_uint,
) -> c_int {
    let mut temp_data: c_uint = 0;
    let mut ret: c_int;

    ret = regmap_write((*aw_dev).regmap, AW88395_DSPMADD_REG, dsp_addr as u32);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"%s write error, ret=%d".as_ptr(), c"aw_dev_dsp_read_32bit".as_ptr(), ret);
        return ret;
    }

    ret = regmap_read((*aw_dev).regmap, AW88395_DSPMDAT_REG, &mut temp_data);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"%s read error, ret=%d".as_ptr(), c"aw_dev_dsp_read_32bit".as_ptr(), ret);
        return ret;
    }
    *dsp_data = temp_data;

    ret = regmap_read((*aw_dev).regmap, AW88395_DSPMDAT_REG, &mut temp_data);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"%s read error, ret=%d".as_ptr(), c"aw_dev_dsp_read_32bit".as_ptr(), ret);
        return ret;
    }
    *dsp_data |= temp_data << 16;

    0
}

#[no_mangle]
pub unsafe extern "C" fn aw_dev_dsp_read(
    aw_dev: *mut aw_device,
    dsp_addr: c_ushort,
    dsp_data: *mut c_uint,
    data_type: c_uchar,
) -> c_int {
    let mut reg_value: u32 = 0;
    let ret: c_int;

    mutex_lock(&mut (*aw_dev).dsp_lock);
    match data_type {
        x if x == AW_DSP_16_DATA => {
            ret = aw_dev_dsp_read_16bit(aw_dev, dsp_addr, dsp_data);
            if ret != 0 {
                dev_err((*aw_dev).dev, c"read dsp_addr[0x%x] 16-bit dsp_data[0x%x] failed".as_ptr(), dsp_addr as u32, *dsp_data);
            }
        }
        x if x == AW_DSP_32_DATA => {
            ret = aw_dev_dsp_read_32bit(aw_dev, dsp_addr, dsp_data);
            if ret != 0 {
                dev_err((*aw_dev).dev, c"read dsp_addr[0x%x] 32r-bit dsp_data[0x%x] failed".as_ptr(), dsp_addr as u32, *dsp_data);
            }
        }
        _ => {
            dev_err((*aw_dev).dev, c"data type[%d] unsupported".as_ptr(), data_type as c_int);
            ret = -EINVAL;
        }
    }

    /* clear dsp chip select state*/
    if regmap_read((*aw_dev).regmap, AW88395_ID_REG, &mut reg_value) != 0 {
        dev_err((*aw_dev).dev, c"%s fail to clear chip state. Err=%d\n".as_ptr(), c"aw_dev_dsp_read".as_ptr(), ret);
    }
    mutex_unlock(&mut (*aw_dev).dsp_lock);

    ret
}

unsafe fn aw_dev_read_chipid(aw_dev: *mut aw_device, chip_id: *mut u16) -> c_int {
    let mut reg_val: c_int = 0;
    let ret: c_int;

    ret = regmap_read((*aw_dev).regmap, AW88395_CHIP_ID_REG, &mut reg_val as *mut c_int as *mut u32);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"%s read chipid error. ret = %d".as_ptr(), c"aw_dev_read_chipid".as_ptr(), ret);
        return ret;
    }

    dev_info((*aw_dev).dev, c"chip id = %x\n".as_ptr(), reg_val);
    *chip_id = reg_val as u16;

    0
}

unsafe fn reg_val_to_db(value: c_uint) -> c_uint {
    ((value >> AW88395_VOL_6DB_START) * AW88395_VOLUME_STEP_DB as u32)
        + ((value & 0x3f) % AW88395_VOLUME_STEP_DB as u32)
}

unsafe fn db_to_reg_val(value: c_ushort) -> c_ushort {
    (((value / AW88395_VOLUME_STEP_DB) << AW88395_VOL_6DB_START) + (value % AW88395_VOLUME_STEP_DB)) as u16
}

unsafe fn aw_dev_dsp_fw_check(aw_dev: *mut aw_device) -> c_int {
    let mut dsp_fw_desc: *mut aw_sec_data_desc;
    let mut set_prof_desc: *mut aw_prof_desc = ptr::null_mut();
    let base_addr: u16 = AW88395_DSP_FW_ADDR;
    let mut addr: u16 = base_addr;
    let mut dsp_val: u32 = 0;
    let mut bin_val: u16;
    let mut ret: c_int;

    ret = aw88395_dev_get_prof_data(aw_dev, (*aw_dev).prof_cur, &mut set_prof_desc);
    if ret != 0 {
        return ret;
    }

    /* update reg */
    dsp_fw_desc = (*set_prof_desc).sec_desc.add(AW88395_DATA_TYPE_DSP_FW);

    let mut i = 0;
    while i < AW88395_FW_CHECK_PART {
        ret = aw_dev_dsp_read(aw_dev, addr, &mut dsp_val, AW_DSP_16_DATA);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"dsp read failed".as_ptr());
            return ret;
        }

        bin_val = be16_to_cpup((*dsp_fw_desc).data.add((2 * (addr - base_addr)) as usize) as *const c_void);
        if dsp_val != bin_val as u32 {
            dev_err((*aw_dev).dev, c"fw check failed, addr[0x%x], read[0x%x] != bindata[0x%x]".as_ptr(), addr as u32, dsp_val, bin_val as u32);
            return -EINVAL;
        }

        addr = addr.wrapping_add((((*dsp_fw_desc).len / 2) / AW88395_FW_CHECK_PART as u32) as u16);
        if (addr - base_addr) as u32 > (*dsp_fw_desc).len {
            dev_err((*aw_dev).dev, c"fw check failed, addr[0x%x] too large".as_ptr(), addr as u32);
            return -EINVAL;
        }
        i += 1;
    }

    0
}

unsafe fn aw_dev_set_volume(aw_dev: *mut aw_device, value: c_uint) -> c_int {
    let vol_desc: *mut aw_volume_desc = &mut (*aw_dev).volume_desc;
    let mut reg_value: c_uint = 0;
    let mut real_value: u16;
    let volume: u16;
    let ret: c_int;

    volume = min_u32(value + (*vol_desc).init_volume as u32, AW88395_MUTE_VOL as u32) as u16;
    real_value = db_to_reg_val(volume);

    /* cal real value */
    let mut r = regmap_read((*aw_dev).regmap, AW88395_SYSCTRL2_REG, &mut reg_value);
    if r != 0 {
        return r;
    }

    dev_dbg((*aw_dev).dev, c"value 0x%x , reg:0x%x".as_ptr(), value, real_value as u32);

    /* [15 : 6] volume */
    real_value = ((real_value as u32) << AW88395_VOL_START_BIT | (reg_value & AW88395_VOL_MASK as u32)) as u16;

    /* write value */
    r = regmap_write((*aw_dev).regmap, AW88395_SYSCTRL2_REG, real_value as u32);
    ret = r;
    ret
}

#[no_mangle]
pub unsafe extern "C" fn aw88395_dev_set_volume(aw_dev: *mut aw_device, set_vol: c_ushort) {
    let ret = aw_dev_set_volume(aw_dev, set_vol as u32);
    if ret != 0 {
        dev_dbg((*aw_dev).dev, c"set volume failed".as_ptr());
    }
}

unsafe fn aw_dev_fade_in(aw_dev: *mut aw_device) {
    let desc: *mut aw_volume_desc = &mut (*aw_dev).volume_desc;
    let fade_in_vol: u16 = (*desc).ctl_volume;
    let fade_step: c_int = (*aw_dev).fade_step;

    if fade_step == 0 || (*aw_dev).fade_in_time == 0 {
        aw_dev_set_volume(aw_dev, fade_in_vol as u32);
        return;
    }

    let mut i: c_int = AW88395_MUTE_VOL as c_int;
    while i >= fade_in_vol as c_int {
        aw_dev_set_volume(aw_dev, i as u32);
        usleep_range((*aw_dev).fade_in_time, (*aw_dev).fade_in_time + 10);
        i -= fade_step;
    }

    if i != fade_in_vol as c_int {
        aw_dev_set_volume(aw_dev, fade_in_vol as u32);
    }
}

unsafe fn aw_dev_fade_out(aw_dev: *mut aw_device) {
    let desc: *mut aw_volume_desc = &mut (*aw_dev).volume_desc;
    let fade_step: c_int = (*aw_dev).fade_step;

    if fade_step == 0 || (*aw_dev).fade_out_time == 0 {
        aw_dev_set_volume(aw_dev, AW88395_MUTE_VOL as u32);
        return;
    }

    let mut i: c_int = (*desc).ctl_volume as c_int;
    while i <= AW88395_MUTE_VOL as c_int {
        aw_dev_set_volume(aw_dev, i as u32);
        usleep_range((*aw_dev).fade_out_time, (*aw_dev).fade_out_time + 10);
        i += fade_step;
    }

    if i != AW88395_MUTE_VOL as c_int {
        aw_dev_set_volume(aw_dev, AW88395_MUTE_VOL as u32);
        usleep_range((*aw_dev).fade_out_time, (*aw_dev).fade_out_time + 10);
    }
}

unsafe fn aw_dev_modify_dsp_cfg(
    aw_dev: *mut aw_device,
    addr: c_uint,
    dsp_data: c_uint,
    data_type: c_uchar,
) -> c_int {
    let crc_dsp_cfg: *mut aw_sec_data_desc = &mut (*aw_dev).crc_dsp_cfg;
    let addr_offset: c_uint;
    let mut data1: u16;
    let mut data2: u32;

    dev_dbg((*aw_dev).dev, c"addr:0x%x, dsp_data:0x%x".as_ptr(), addr, dsp_data);

    addr_offset = (addr - AW88395_DSP_CFG_ADDR) * 2;
    if addr_offset > (*crc_dsp_cfg).len {
        dev_err((*aw_dev).dev, c"addr_offset[%d] > crc_dsp_cfg->len[%d]".as_ptr(), addr_offset, (*crc_dsp_cfg).len);
        return -EINVAL;
    }
    match data_type {
        x if x == AW_DSP_16_DATA => {
            data1 = cpu_to_le16(dsp_data as u16);
            memcpy((*crc_dsp_cfg).data.add(addr_offset as usize) as *mut c_void, &mut data1 as *mut u16 as *mut c_void, 2);
        }
        x if x == AW_DSP_32_DATA => {
            data2 = cpu_to_le32(dsp_data);
            memcpy((*crc_dsp_cfg).data.add(addr_offset as usize) as *mut c_void, &mut data2 as *mut u32 as *mut c_void, 4);
        }
        _ => {
            dev_err((*aw_dev).dev, c"data type[%d] unsupported".as_ptr(), data_type as c_int);
            return -EINVAL;
        }
    }

    0
}

unsafe fn aw_dev_dsp_set_cali_re(aw_dev: *mut aw_device) -> c_int {
    let cali_re: u32;
    let mut ret: c_int;

    cali_re = AW88395_SHOW_RE_TO_DSP_RE((*aw_dev).cali_desc.cali_re + (*aw_dev).cali_desc.ra, AW88395_DSP_RE_SHIFT);

    /* set cali re to device */
    ret = aw_dev_dsp_write(aw_dev, AW88395_DSP_REG_CFG_ADPZ_RE as u16, cali_re, AW_DSP_32_DATA);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"set cali re error".as_ptr());
        return ret;
    }

    ret = aw_dev_modify_dsp_cfg(aw_dev, AW88395_DSP_REG_CFG_ADPZ_RE, cali_re, AW_DSP_32_DATA);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"modify dsp cfg failed".as_ptr());
    }

    ret
}

unsafe fn aw_dev_i2s_tx_enable(aw_dev: *mut aw_device, flag: bool_t) {
    let ret: c_int;

    if flag {
        ret = regmap_update_bits((*aw_dev).regmap, AW88395_I2SCFG1_REG, !AW88395_I2STXEN_MASK, AW88395_I2STXEN_ENABLE_VALUE);
    } else {
        ret = regmap_update_bits((*aw_dev).regmap, AW88395_I2SCFG1_REG, !AW88395_I2STXEN_MASK, AW88395_I2STXEN_DISABLE_VALUE);
    }

    if ret != 0 {
        dev_dbg((*aw_dev).dev, c"%s failed".as_ptr(), c"aw_dev_i2s_tx_enable".as_ptr());
    }
}

unsafe fn aw_dev_dsp_set_crc32(aw_dev: *mut aw_device) -> c_int {
    let crc_dsp_cfg: *mut aw_sec_data_desc = &mut (*aw_dev).crc_dsp_cfg;
    let crc_value: u32;
    let crc_data_len: u32;

    /* get crc data len */
    crc_data_len = (AW88395_DSP_REG_CRC_ADDR - AW88395_DSP_CFG_ADDR) * 2;
    if crc_data_len > (*crc_dsp_cfg).len {
        dev_err((*aw_dev).dev, c"crc data len :%d > cfg_data len:%d".as_ptr(), crc_data_len, (*crc_dsp_cfg).len);
        return -EINVAL;
    }

    if (crc_data_len & 0x11) != 0 {
        dev_err((*aw_dev).dev, c"The crc data len :%d unsupport".as_ptr(), crc_data_len);
        return -EINVAL;
    }

    crc_value = crc32c(0xFFFFFFFF, (*crc_dsp_cfg).data, crc_data_len) ^ 0xFFFFFFFF;

    aw_dev_dsp_write(aw_dev, AW88395_DSP_REG_CRC_ADDR as u16, crc_value, AW_DSP_32_DATA)
}

unsafe fn aw_dev_dsp_check_crc_enable(aw_dev: *mut aw_device, flag: bool_t) {
    let ret: c_int;

    if flag {
        ret = regmap_update_bits((*aw_dev).regmap, AW88395_HAGCCFG7_REG, !AW88395_AGC_DSP_CTL_MASK, AW88395_AGC_DSP_CTL_ENABLE_VALUE);
    } else {
        ret = regmap_update_bits((*aw_dev).regmap, AW88395_HAGCCFG7_REG, !AW88395_AGC_DSP_CTL_MASK, AW88395_AGC_DSP_CTL_DISABLE_VALUE);
    }
    if ret != 0 {
        dev_dbg((*aw_dev).dev, c"%s failed".as_ptr(), c"aw_dev_dsp_check_crc_enable".as_ptr());
    }
}

unsafe fn aw_dev_dsp_check_st(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    let mut ret: c_int = 0;
    let mut i: c_int = 0;

    while i < AW88395_DSP_ST_CHECK_MAX {
        ret = regmap_read((*aw_dev).regmap, AW88395_SYSST_REG, &mut reg_val);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"read reg0x%x failed".as_ptr(), AW88395_SYSST_REG);
            i += 1;
            continue;
        }

        if (reg_val & !AW88395_DSPS_MASK) != AW88395_DSPS_NORMAL_VALUE {
            dev_err((*aw_dev).dev, c"check dsp st fail,reg_val:0x%04x".as_ptr(), reg_val);
            ret = -EPERM;
            i += 1;
            continue;
        } else {
            dev_dbg((*aw_dev).dev, c"dsp st check ok, reg_val:0x%04x".as_ptr(), reg_val);
            return 0;
        }
    }

    ret
}

unsafe fn aw_dev_dsp_enable(aw_dev: *mut aw_device, is_enable: bool_t) {
    let ret: c_int;

    if is_enable {
        ret = regmap_update_bits((*aw_dev).regmap, AW88395_SYSCTRL_REG, !AW88395_DSPBY_MASK, AW88395_DSPBY_WORKING_VALUE);
        if ret != 0 {
            dev_dbg((*aw_dev).dev, c"enable dsp failed".as_ptr());
        }
    } else {
        ret = regmap_update_bits((*aw_dev).regmap, AW88395_SYSCTRL_REG, !AW88395_DSPBY_MASK, AW88395_DSPBY_BYPASS_VALUE);
        if ret != 0 {
            dev_dbg((*aw_dev).dev, c"disable dsp failed".as_ptr());
        }
    }
}

unsafe fn aw_dev_dsp_check_crc32(aw_dev: *mut aw_device) -> c_int {
    let mut ret: c_int;

    if (*aw_dev).dsp_cfg == AW88395_DEV_DSP_BYPASS {
        dev_info((*aw_dev).dev, c"dsp bypass".as_ptr());
        return 0;
    }

    ret = aw_dev_dsp_set_crc32(aw_dev);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"set dsp crc32 failed".as_ptr());
        return ret;
    }

    aw_dev_dsp_check_crc_enable(aw_dev, true);
    /* dsp enable */
    aw_dev_dsp_enable(aw_dev, true);
    usleep_range(AW88395_5000_US, AW88395_5000_US + 100);

    ret = aw_dev_dsp_check_st(aw_dev);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"check crc32 fail".as_ptr());
    } else {
        aw_dev_dsp_check_crc_enable(aw_dev, false);
        (*aw_dev).dsp_crc_st = AW88395_DSP_CRC_OK;
    }

    ret
}

unsafe fn aw_dev_pwd(aw_dev: *mut aw_device, pwd: bool_t) {
    let ret = if pwd {
        regmap_update_bits((*aw_dev).regmap, AW88395_SYSCTRL_REG, !AW88395_PWDN_MASK, AW88395_PWDN_POWER_DOWN_VALUE)
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88395_SYSCTRL_REG, !AW88395_PWDN_MASK, AW88395_PWDN_WORKING_VALUE)
    };
    if ret != 0 {
        dev_dbg((*aw_dev).dev, c"%s failed".as_ptr(), c"aw_dev_pwd".as_ptr());
    }
}

unsafe fn aw_dev_amppd(aw_dev: *mut aw_device, amppd: bool_t) {
    let ret = if amppd {
        regmap_update_bits((*aw_dev).regmap, AW88395_SYSCTRL_REG, !AW88395_AMPPD_MASK, AW88395_AMPPD_POWER_DOWN_VALUE)
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88395_SYSCTRL_REG, !AW88395_AMPPD_MASK, AW88395_AMPPD_WORKING_VALUE)
    };
    if ret != 0 {
        dev_dbg((*aw_dev).dev, c"%s failed".as_ptr(), c"aw_dev_amppd".as_ptr());
    }
}

#[no_mangle]
pub unsafe extern "C" fn aw88395_dev_mute(aw_dev: *mut aw_device, is_mute: bool_t) {
    let ret: c_int;

    if is_mute {
        aw_dev_fade_out(aw_dev);
        ret = regmap_update_bits((*aw_dev).regmap, AW88395_SYSCTRL_REG, !AW88395_HMUTE_MASK, AW88395_HMUTE_ENABLE_VALUE);
    } else {
        ret = regmap_update_bits((*aw_dev).regmap, AW88395_SYSCTRL_REG, !AW88395_HMUTE_MASK, AW88395_HMUTE_DISABLE_VALUE);
        aw_dev_fade_in(aw_dev);
    }

    if ret != 0 {
        dev_dbg((*aw_dev).dev, c"%s failed".as_ptr(), c"aw88395_dev_mute".as_ptr());
    }
}

unsafe fn aw_dev_get_icalk(aw_dev: *mut aw_device, icalk: *mut int16_t) -> c_int {
    let mut reg_val: c_uint = 0;
    let mut reg_icalk: u16;
    let ret = regmap_read((*aw_dev).regmap, AW88395_EFRM2_REG, &mut reg_val);
    if ret != 0 {
        return ret;
    }
    reg_icalk = (reg_val as u16) & !AW88395_EF_ISN_GESLP_MASK;
    if (reg_icalk & !AW88395_EF_ISN_GESLP_SIGN_MASK) != 0 {
        reg_icalk |= AW88395_EF_ISN_GESLP_SIGN_NEG;
    }
    *icalk = reg_icalk as int16_t;
    ret
}

unsafe fn aw_dev_get_vcalk(aw_dev: *mut aw_device, vcalk: *mut int16_t) -> c_int {
    let mut reg_val: c_uint = 0;
    let mut reg_vcalk: u16;
    let ret = regmap_read((*aw_dev).regmap, AW88395_EFRH_REG, &mut reg_val);
    if ret != 0 {
        return ret;
    }
    reg_val >>= AW88395_EF_VSENSE_GAIN_SHIFT;
    reg_vcalk = reg_val as u16 & !AW88395_EF_VSN_GESLP_MASK;
    if (reg_vcalk & !AW88395_EF_VSN_GESLP_SIGN_MASK) != 0 {
        reg_vcalk |= AW88395_EF_VSN_GESLP_SIGN_NEG;
    }
    *vcalk = reg_vcalk as int16_t;
    ret
}

unsafe fn aw_dev_get_vcalk_dac(aw_dev: *mut aw_device, vcalk: *mut int16_t) -> c_int {
    let mut reg_val: c_uint = 0;
    let mut reg_vcalk: u16;
    let ret = regmap_read((*aw_dev).regmap, AW88395_EFRM2_REG, &mut reg_val);
    if ret != 0 {
        return ret;
    }
    reg_vcalk = (reg_val >> AW88395_EF_DAC_GESLP_SHIFT) as u16;
    if (reg_vcalk & AW88395_EF_DAC_GESLP_SIGN_MASK) != 0 {
        reg_vcalk |= AW88395_EF_DAC_GESLP_SIGN_NEG;
    }
    *vcalk = reg_vcalk as int16_t;
    ret
}

unsafe fn aw_dev_vsense_select(aw_dev: *mut aw_device, vsense_select: *mut c_int) -> c_int {
    let mut vsense_reg_val: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88395_I2SCFG3_REG, &mut vsense_reg_val);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"read vsense_reg_val failed".as_ptr());
        return ret;
    }
    dev_dbg((*aw_dev).dev, c"vsense_reg = 0x%x".as_ptr(), vsense_reg_val);

    if (vsense_reg_val & !AW88395_VDSEL_MASK) != 0 {
        *vsense_select = AW88395_DEV_VDSEL_VSENSE;
        dev_dbg((*aw_dev).dev, c"vsense outside".as_ptr());
    } else {
        *vsense_select = AW88395_DEV_VDSEL_DAC;
        dev_dbg((*aw_dev).dev, c"vsense inside".as_ptr());
    }

    0
}

unsafe fn aw_dev_set_vcalb(aw_dev: *mut aw_device) -> c_int {
    let mut icalk_val: int16_t = 0;
    let mut vcalk_val: int16_t = 0;
    let mut vsense_select: c_int = 0;
    let mut vcalb_adj: u32 = 0;
    let reg_val: u32;
    let mut vcalb: c_int;
    let icalk: c_int;
    let vcalk: c_int;
    let mut ret: c_int;

    ret = aw_dev_dsp_read(aw_dev, AW88395_DSP_REG_VCALB, &mut vcalb_adj, AW_DSP_16_DATA);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"read vcalb_adj failed".as_ptr());
        return ret;
    }

    ret = aw_dev_vsense_select(aw_dev, &mut vsense_select);
    if ret != 0 {
        return ret;
    }
    dev_dbg((*aw_dev).dev, c"vsense_select = %d".as_ptr(), vsense_select);

    ret = aw_dev_get_icalk(aw_dev, &mut icalk_val);
    if ret != 0 {
        return ret;
    }
    icalk = AW88395_CABL_BASE_VALUE + AW88395_ICABLK_FACTOR * icalk_val as c_int;

    if vsense_select == AW88395_DEV_VDSEL_VSENSE {
        ret = aw_dev_get_vcalk(aw_dev, &mut vcalk_val);
        if ret != 0 { return ret; }
        vcalk = AW88395_CABL_BASE_VALUE + AW88395_VCABLK_FACTOR * vcalk_val as c_int;
        vcalb = AW88395_VCAL_FACTOR * AW88395_VSCAL_FACTOR / AW88395_ISCAL_FACTOR * icalk / vcalk * vcalb_adj as c_int;
        dev_dbg((*aw_dev).dev, c"vcalk_factor=%d, vscal_factor=%d, icalk=%d, vcalk=%d".as_ptr(), AW88395_VCABLK_FACTOR, AW88395_VSCAL_FACTOR, icalk, vcalk);
    } else if vsense_select == AW88395_DEV_VDSEL_DAC {
        ret = aw_dev_get_vcalk_dac(aw_dev, &mut vcalk_val);
        if ret != 0 { return ret; }
        vcalk = AW88395_CABL_BASE_VALUE + AW88395_VCABLK_FACTOR_DAC * vcalk_val as c_int;
        vcalb = AW88395_VCAL_FACTOR * AW88395_VSCAL_FACTOR_DAC / AW88395_ISCAL_FACTOR * icalk / vcalk * vcalb_adj as c_int;
        dev_dbg((*aw_dev).dev, c"vcalk_dac_factor=%d, vscal_dac_factor=%d, icalk=%d, vcalk=%d".as_ptr(), AW88395_VCABLK_FACTOR_DAC, AW88395_VSCAL_FACTOR_DAC, icalk, vcalk);
    } else {
        dev_err((*aw_dev).dev, c"unsupported vsense status".as_ptr());
        return -EINVAL;
    }

    if vcalk == 0 || AW88395_ISCAL_FACTOR == 0 {
        dev_err((*aw_dev).dev, c"vcalk:%d or desc->iscal_factor:%d unsupported".as_ptr(), vcalk, AW88395_ISCAL_FACTOR);
        return -EINVAL;
    }

    vcalb >>= AW88395_VCALB_ADJ_FACTOR;
    reg_val = vcalb as u32;
    dev_dbg((*aw_dev).dev, c"vcalb=%d, reg_val=0x%x, vcalb_adj =0x%x".as_ptr(), vcalb, reg_val, vcalb_adj);

    ret = aw_dev_dsp_write(aw_dev, AW88395_DSP_REG_VCALB, reg_val, AW_DSP_16_DATA);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"write vcalb failed".as_ptr());
        return ret;
    }
    ret = aw_dev_modify_dsp_cfg(aw_dev, AW88395_DSP_REG_VCALB as u32, reg_val, AW_DSP_16_DATA);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"modify dsp cfg failed".as_ptr());
    }
    ret
}

unsafe fn aw_dev_get_cali_f0_delay(aw_dev: *mut aw_device) -> c_int {
    let desc: *mut aw_cali_delay_desc = &mut (*aw_dev).cali_delay_desc;
    let mut cali_delay: u32 = 0;
    let ret = aw_dev_dsp_read(aw_dev, AW88395_DSP_CALI_F0_DELAY, &mut cali_delay, AW_DSP_16_DATA);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"read cali delay failed, ret=%d".as_ptr(), ret);
    } else {
        (*desc).delay = AW88395_CALI_DELAY_CACL(cali_delay);
    }
    dev_dbg((*aw_dev).dev, c"read cali delay: %d ms".as_ptr(), (*desc).delay);
    ret
}

unsafe fn aw_dev_get_int_status(aw_dev: *mut aw_device, int_status: *mut c_ushort) {
    let mut reg_val: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88395_SYSINT_REG, &mut reg_val);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"read interrupt reg fail, ret=%d".as_ptr(), ret);
    } else {
        *int_status = reg_val as u16;
    }
    dev_dbg((*aw_dev).dev, c"read interrupt reg = 0x%04x".as_ptr(), *int_status as c_int);
}

unsafe fn aw_dev_clear_int_status(aw_dev: *mut aw_device) {
    let mut int_status: u16 = 0;
    /* read int status and clear */
    aw_dev_get_int_status(aw_dev, &mut int_status);
    /* make sure int status is clear */
    aw_dev_get_int_status(aw_dev, &mut int_status);
    if int_status != 0 {
        dev_info((*aw_dev).dev, c"int status(%d) is not cleaned.\n".as_ptr(), int_status as c_int);
    }
}

unsafe fn aw_dev_get_iis_status(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88395_SYSST_REG, &mut reg_val);
    if ret != 0 {
        return -EIO;
    }
    if (reg_val & AW88395_BIT_PLL_CHECK) != AW88395_BIT_PLL_CHECK {
        dev_err((*aw_dev).dev, c"check pll lock fail,reg_val:0x%04x".as_ptr(), reg_val);
        return -EINVAL;
    }
    0
}

unsafe fn aw_dev_check_mode1_pll(aw_dev: *mut aw_device) -> c_int {
    let mut i = 0;
    while i < AW88395_DEV_SYSST_CHECK_MAX {
        let ret = aw_dev_get_iis_status(aw_dev);
        if ret < 0 {
            dev_err((*aw_dev).dev, c"mode1 iis signal check error".as_ptr());
            usleep_range(AW88395_2000_US, AW88395_2000_US + 10);
        } else {
            return 0;
        }
        i += 1;
    }
    -EPERM
}

unsafe fn aw_dev_check_mode2_pll(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    let mut ret = regmap_read((*aw_dev).regmap, AW88395_PLLCTRL1_REG, &mut reg_val);
    if ret != 0 {
        return ret;
    }

    reg_val &= !AW88395_CCO_MUX_MASK;
    if reg_val == AW88395_CCO_MUX_DIVIDED_VALUE {
        dev_dbg((*aw_dev).dev, c"CCO_MUX is already divider".as_ptr());
        return -EPERM;
    }

    /* change mode2 */
    ret = regmap_update_bits((*aw_dev).regmap, AW88395_PLLCTRL1_REG, !AW88395_CCO_MUX_MASK, AW88395_CCO_MUX_DIVIDED_VALUE);
    if ret != 0 { return ret; }

    let mut i = 0;
    while i < AW88395_DEV_SYSST_CHECK_MAX {
        ret = aw_dev_get_iis_status(aw_dev);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"mode2 iis signal check error".as_ptr());
            usleep_range(AW88395_2000_US, AW88395_2000_US + 10);
        } else {
            break;
        }
        i += 1;
    }

    /* change mode1 */
    ret = regmap_update_bits((*aw_dev).regmap, AW88395_PLLCTRL1_REG, !AW88395_CCO_MUX_MASK, AW88395_CCO_MUX_BYPASS_VALUE);
    if ret == 0 {
        usleep_range(AW88395_2000_US, AW88395_2000_US + 10);
        i = 0;
        while i < AW88395_DEV_SYSST_CHECK_MAX {
            ret = aw_dev_check_mode1_pll(aw_dev);
            if ret < 0 {
                dev_err((*aw_dev).dev, c"mode2 switch to mode1, iis signal check error".as_ptr());
                usleep_range(AW88395_2000_US, AW88395_2000_US + 10);
            } else {
                break;
            }
            i += 1;
        }
    }
    ret
}

unsafe fn aw_dev_check_syspll(aw_dev: *mut aw_device) -> c_int {
    let mut ret = aw_dev_check_mode1_pll(aw_dev);
    if ret != 0 {
        dev_dbg((*aw_dev).dev, c"mode1 check iis failed try switch to mode2 check".as_ptr());
        ret = aw_dev_check_mode2_pll(aw_dev);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"mode2 check iis failed".as_ptr());
            return ret;
        }
    }
    ret
}

unsafe fn aw_dev_check_sysst(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    let mut i = 0;
    while i < AW88395_DEV_SYSST_CHECK_MAX {
        let ret = regmap_read((*aw_dev).regmap, AW88395_SYSST_REG, &mut reg_val);
        if ret != 0 { return ret; }
        let check_val = reg_val & !AW88395_BIT_SYSST_CHECK_MASK & AW88395_BIT_SYSST_CHECK;
        if check_val != AW88395_BIT_SYSST_CHECK {
            dev_err((*aw_dev).dev, c"check sysst fail, cnt=%d, reg_val=0x%04x, check:0x%x".as_ptr(), i, reg_val, AW88395_BIT_SYSST_CHECK);
            usleep_range(AW88395_2000_US, AW88395_2000_US + 10);
        } else {
            return 0;
        }
        i += 1;
    }
    -EPERM
}

unsafe fn aw_dev_check_sysint(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: u16 = 0;
    aw_dev_get_int_status(aw_dev, &mut reg_val);
    if (reg_val & AW88395_BIT_SYSINT_CHECK) != 0 {
        dev_err((*aw_dev).dev, c"pa stop check fail:0x%04x".as_ptr(), reg_val as c_int);
        return -EINVAL;
    }
    0
}

unsafe fn aw_dev_get_cur_mode_st(aw_dev: *mut aw_device) {
    let profctrl_desc: *mut aw_profctrl_desc = &mut (*aw_dev).profctrl_desc;
    let mut reg_val: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88395_SYSCTRL_REG, &mut reg_val);
    if ret != 0 {
        dev_dbg((*aw_dev).dev, c"%s failed".as_ptr(), c"aw_dev_get_cur_mode_st".as_ptr());
        return;
    }
    if (reg_val & !AW88395_RCV_MODE_MASK) == AW88395_RCV_MODE_RECEIVER_VALUE {
        (*profctrl_desc).cur_mode = AW88395_RCV_MODE;
    } else {
        (*profctrl_desc).cur_mode = AW88395_NOT_RCV_MODE;
    }
}

unsafe fn aw_dev_get_dsp_config(aw_dev: *mut aw_device, dsp_cfg: *mut c_uchar) {
    let mut reg_val: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88395_SYSCTRL_REG, &mut reg_val);
    if ret != 0 {
        dev_dbg((*aw_dev).dev, c"%s failed".as_ptr(), c"aw_dev_get_dsp_config".as_ptr());
        return;
    }
    if (reg_val & !AW88395_DSPBY_MASK) != 0 {
        *dsp_cfg = AW88395_DEV_DSP_BYPASS;
    } else {
        *dsp_cfg = AW88395_DEV_DSP_WORK;
    }
}

unsafe fn aw_dev_select_memclk(aw_dev: *mut aw_device, flag: c_uchar) {
    if flag == AW88395_DEV_MEMCLK_PLL {
        let ret = regmap_update_bits((*aw_dev).regmap, AW88395_DBGCTRL_REG, !AW88395_MEM_CLKSEL_MASK, AW88395_MEM_CLKSEL_DAP_HCLK_VALUE);
        if ret != 0 { dev_err((*aw_dev).dev, c"memclk select pll failed".as_ptr()); }
    } else if flag == AW88395_DEV_MEMCLK_OSC {
        let ret = regmap_update_bits((*aw_dev).regmap, AW88395_DBGCTRL_REG, !AW88395_MEM_CLKSEL_MASK, AW88395_MEM_CLKSEL_OSC_CLK_VALUE);
        if ret != 0 { dev_err((*aw_dev).dev, c"memclk select OSC failed".as_ptr()); }
    } else {
        dev_err((*aw_dev).dev, c"unknown memclk config, flag=0x%x".as_ptr(), flag as c_int);
    }
}

unsafe fn aw_dev_get_dsp_status(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    let mut ret = regmap_read((*aw_dev).regmap, AW88395_WDT_REG, &mut reg_val);
    if ret != 0 { return ret; }
    if (reg_val & !AW88395_WDT_CNT_MASK) == 0 {
        ret = -EPERM;
    }
    ret
}

unsafe fn aw_dev_get_vmax(aw_dev: *mut aw_device, vmax: *mut c_uint) -> c_int {
    aw_dev_dsp_read(aw_dev, AW88395_DSP_REG_VMAX, vmax, AW_DSP_16_DATA)
}

unsafe fn aw_dev_update_reg_container(aw_dev: *mut aw_device, data: *mut c_uchar, len: c_uint) -> c_int {
    let vol_desc: *mut aw_volume_desc = &mut (*aw_dev).volume_desc;
    let mut read_val: c_uint = 0;
    let reg_data = data as *mut int16_t;
    let data_len: c_int = (len >> 1) as c_int;
    let mut read_vol: u16;
    let mut reg_val: u16;
    let mut reg_addr: u8;
    let mut ret: c_int = 0;

    if (data_len & 0x1) != 0 {
        dev_err((*aw_dev).dev, c"data len:%d unsupported".as_ptr(), data_len);
        return -EINVAL;
    }

    let mut i = 0;
    while i < data_len {
        reg_addr = *reg_data.add(i as usize) as u8;
        reg_val = *reg_data.add((i + 1) as usize) as u16;
        if reg_addr as u32 == AW88395_SYSCTRL_REG {
            ret = regmap_read((*aw_dev).regmap, reg_addr as u32, &mut read_val);
            if ret != 0 { break; }
            read_val &= !AW88395_HMUTE_MASK;
            reg_val &= AW88395_HMUTE_MASK as u16;
            reg_val |= read_val as u16;
        }
        if reg_addr as u32 == AW88395_HAGCCFG7_REG {
            reg_val &= AW88395_AGC_DSP_CTL_MASK as u16;
        }
        if reg_addr as u32 == AW88395_I2SCFG1_REG {
            /* close tx */
            reg_val &= AW88395_I2STXEN_MASK as u16;
            reg_val |= AW88395_I2STXEN_DISABLE_VALUE as u16;
        }
        if reg_addr as u32 == AW88395_SYSCTRL2_REG {
            read_vol = ((reg_val & !AW88395_VOL_MASK) as u32 >> AW88395_VOL_START_BIT) as u16;
            (*aw_dev).volume_desc.init_volume = reg_val_to_db(read_vol as u32) as u16;
        }
        ret = regmap_write((*aw_dev).regmap, reg_addr as u32, reg_val as u32);
        if ret != 0 { break; }
        i += 2;
    }

    aw_dev_get_cur_mode_st(aw_dev);
    if (*aw_dev).prof_cur != (*aw_dev).prof_index {
        /* clear control volume when PA change profile */
        (*vol_desc).ctl_volume = 0;
    } else {
        /* keep control volume when PA start with sync mode */
        aw_dev_set_volume(aw_dev, (*vol_desc).ctl_volume as u32);
    }
    aw_dev_get_dsp_config(aw_dev, &mut (*aw_dev).dsp_cfg);
    ret
}

unsafe fn aw_dev_reg_update(aw_dev: *mut aw_device, data: *mut c_uchar, len: c_uint) -> c_int {
    if len == 0 || data.is_null() {
        dev_err((*aw_dev).dev, c"reg data is null or len is 0".as_ptr());
        return -EINVAL;
    }
    let ret = aw_dev_update_reg_container(aw_dev, data, len);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"reg update failed".as_ptr());
        return ret;
    }
    0
}

unsafe fn aw_dev_from_cali_desc(cali_desc: *mut aw_cali_desc) -> *mut aw_device {
    (cali_desc as *mut u8).sub(offset_of!(aw_device, cali_desc)) as *mut aw_device
}

unsafe fn aw_dev_get_ra(cali_desc: *mut aw_cali_desc) -> c_int {
    let aw_dev = aw_dev_from_cali_desc(cali_desc);
    let mut dsp_ra: u32 = 0;
    let ret = aw_dev_dsp_read(aw_dev, AW88395_DSP_REG_CFG_ADPZ_RA, &mut dsp_ra, AW_DSP_32_DATA);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"read ra error".as_ptr());
        return ret;
    }
    (*cali_desc).ra = AW88395_DSP_RE_TO_SHOW_RE(dsp_ra, AW88395_DSP_RE_SHIFT);
    ret
}

unsafe fn aw_dev_dsp_update_container(aw_dev: *mut aw_device, data: *mut c_uchar, len: c_uint, base: c_ushort) -> c_int {
    let mut ret: c_int;
    /* AW88395_DSP_I2C_WRITES build-time branch is preserved by using the non-raw write path translated from the #else branch. */
    let mut reg_val: u16;

    mutex_lock(&mut (*aw_dev).dsp_lock);
    /* i2c write */
    ret = regmap_write((*aw_dev).regmap, AW88395_DSPMADD_REG, base as u32);
    if ret != 0 {
        mutex_unlock(&mut (*aw_dev).dsp_lock);
        return ret;
    }
    let mut i: c_uint = 0;
    while i < len {
        reg_val = cpu_to_be16p(data.add(i as usize) as *const u16);
        ret = regmap_write((*aw_dev).regmap, AW88395_DSPMDAT_REG, reg_val as u32);
        if ret != 0 {
            mutex_unlock(&mut (*aw_dev).dsp_lock);
            return ret;
        }
        i += 2;
    }
    mutex_unlock(&mut (*aw_dev).dsp_lock);
    0
}

unsafe fn aw_dev_dsp_update_fw(aw_dev: *mut aw_device, data: *mut c_uchar, len: c_uint) -> c_int {
    dev_dbg((*aw_dev).dev, c"dsp firmware len:%d".as_ptr(), len);
    if len == 0 || data.is_null() {
        dev_err((*aw_dev).dev, c"dsp firmware data is null or len is 0".as_ptr());
        return -EINVAL;
    }
    aw_dev_dsp_update_container(aw_dev, data, len, AW88395_DSP_FW_ADDR);
    (*aw_dev).dsp_fw_len = len;
    0
}

unsafe fn aw_dev_copy_to_crc_dsp_cfg(aw_dev: *mut aw_device, data: *mut c_uchar, size: c_uint) -> c_int {
    let crc_dsp_cfg: *mut aw_sec_data_desc = &mut (*aw_dev).crc_dsp_cfg;

    if (*crc_dsp_cfg).data.is_null() {
        (*crc_dsp_cfg).data = devm_kzalloc((*aw_dev).dev, size as usize, GFP_KERNEL) as *mut u8;
        if (*crc_dsp_cfg).data.is_null() { return -ENOMEM; }
        (*crc_dsp_cfg).len = size;
    } else if (*crc_dsp_cfg).len < size {
        devm_kfree((*aw_dev).dev, (*crc_dsp_cfg).data as *mut c_void);
        (*crc_dsp_cfg).data = devm_kzalloc((*aw_dev).dev, size as usize, GFP_KERNEL) as *mut u8;
        if (*crc_dsp_cfg).data.is_null() { return -ENOMEM; }
        (*crc_dsp_cfg).len = size;
    }
    memcpy((*crc_dsp_cfg).data as *mut c_void, data as *const c_void, size as usize);
    swab16_array((*crc_dsp_cfg).data as *mut u16, size >> 1);
    0
}

unsafe fn aw_dev_dsp_update_cfg(aw_dev: *mut aw_device, data: *mut c_uchar, len: c_uint) -> c_int {
    let mut ret: c_int;
    dev_dbg((*aw_dev).dev, c"dsp config len:%d".as_ptr(), len);
    if len == 0 || data.is_null() {
        dev_err((*aw_dev).dev, c"dsp config data is null or len is 0".as_ptr());
        return -EINVAL;
    }
    aw_dev_dsp_update_container(aw_dev, data, len, AW88395_DSP_CFG_ADDR as u16);
    (*aw_dev).dsp_cfg_len = len;

    ret = aw_dev_copy_to_crc_dsp_cfg(aw_dev, data, len);
    if ret != 0 { return ret; }
    ret = aw_dev_set_vcalb(aw_dev);
    if ret != 0 { return ret; }
    ret = aw_dev_get_ra(&mut (*aw_dev).cali_desc);
    if ret != 0 { return ret; }
    ret = aw_dev_get_cali_f0_delay(aw_dev);
    if ret != 0 { return ret; }
    ret = aw_dev_get_vmax(aw_dev, &mut (*aw_dev).vmax_desc.init_vmax);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"get vmax failed".as_ptr());
        return ret;
    }
    dev_dbg((*aw_dev).dev, c"get init vmax:0x%x".as_ptr(), (*aw_dev).vmax_desc.init_vmax);
    (*aw_dev).dsp_crc_st = AW88395_DSP_CRC_NA;
    0
}

unsafe fn aw_dev_check_sram(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    mutex_lock(&mut (*aw_dev).dsp_lock);
    /* check the odd bits of reg 0x40 */
    regmap_write((*aw_dev).regmap, AW88395_DSPMADD_REG, AW88395_DSP_ODD_NUM_BIT_TEST as u32);
    regmap_read((*aw_dev).regmap, AW88395_DSPMADD_REG, &mut reg_val);
    if reg_val != AW88395_DSP_ODD_NUM_BIT_TEST as u32 {
        dev_err((*aw_dev).dev, c"check reg 0x40 odd bit failed, read[0x%x] != write[0x%x]".as_ptr(), reg_val, AW88395_DSP_ODD_NUM_BIT_TEST as u32);
        mutex_unlock(&mut (*aw_dev).dsp_lock);
        return -EPERM;
    }
    /* check the even bits of reg 0x40 */
    regmap_write((*aw_dev).regmap, AW88395_DSPMADD_REG, AW88395_DSP_EVEN_NUM_BIT_TEST as u32);
    regmap_read((*aw_dev).regmap, AW88395_DSPMADD_REG, &mut reg_val);
    if reg_val != AW88395_DSP_EVEN_NUM_BIT_TEST as u32 {
        dev_err((*aw_dev).dev, c"check reg 0x40 even bit failed, read[0x%x] != write[0x%x]".as_ptr(), reg_val, AW88395_DSP_EVEN_NUM_BIT_TEST as u32);
        mutex_unlock(&mut (*aw_dev).dsp_lock);
        return -EPERM;
    }
    aw_dev_dsp_write_16bit(aw_dev, AW88395_DSP_FW_ADDR, AW88395_DSP_EVEN_NUM_BIT_TEST as u32);
    aw_dev_dsp_read_16bit(aw_dev, AW88395_DSP_FW_ADDR, &mut reg_val);
    if reg_val != AW88395_DSP_EVEN_NUM_BIT_TEST as u32 {
        dev_err((*aw_dev).dev, c"check dsp fw addr failed, read[0x%x] != write[0x%x]".as_ptr(), reg_val, AW88395_DSP_EVEN_NUM_BIT_TEST as u32);
        mutex_unlock(&mut (*aw_dev).dsp_lock);
        return -EPERM;
    }
    aw_dev_dsp_write_16bit(aw_dev, AW88395_DSP_CFG_ADDR as u16, AW88395_DSP_ODD_NUM_BIT_TEST as u32);
    aw_dev_dsp_read_16bit(aw_dev, AW88395_DSP_CFG_ADDR as u16, &mut reg_val);
    if reg_val != AW88395_DSP_ODD_NUM_BIT_TEST as u32 {
        dev_err((*aw_dev).dev, c"check dsp cfg failed, read[0x%x] != write[0x%x]".as_ptr(), reg_val, AW88395_DSP_ODD_NUM_BIT_TEST as u32);
        mutex_unlock(&mut (*aw_dev).dsp_lock);
        return -EPERM;
    }
    mutex_unlock(&mut (*aw_dev).dsp_lock);
    0
}

#[no_mangle]
pub unsafe extern "C" fn aw88395_dev_fw_update(aw_dev: *mut aw_device, up_dsp_fw_en: bool_t, force_up_en: bool_t) -> c_int {
    let mut prof_index_desc: *mut aw_prof_desc = ptr::null_mut();
    let sec_desc: *mut aw_sec_data_desc;
    let mut prof_name: *mut c_char = ptr::null_mut();
    let mut ret: c_int;

    if (*aw_dev).prof_cur == (*aw_dev).prof_index && force_up_en == AW88395_FORCE_UPDATE_OFF {
        dev_dbg((*aw_dev).dev, c"scene no change, not update".as_ptr());
        return 0;
    }
    if (*aw_dev).fw_status == AW88395_DEV_FW_FAILED {
        dev_err((*aw_dev).dev, c"fw status[%d] error".as_ptr(), (*aw_dev).fw_status);
        return -EPERM;
    }
    ret = aw88395_dev_get_prof_name(aw_dev, (*aw_dev).prof_index, &mut prof_name);
    if ret != 0 { return ret; }
    dev_dbg((*aw_dev).dev, c"start update %s".as_ptr(), prof_name);
    ret = aw88395_dev_get_prof_data(aw_dev, (*aw_dev).prof_index, &mut prof_index_desc);
    if ret != 0 { return ret; }

    /* update reg */
    sec_desc = (*prof_index_desc).sec_desc;
    ret = aw_dev_reg_update(aw_dev, (*sec_desc.add(AW88395_DATA_TYPE_REG)).data, (*sec_desc.add(AW88395_DATA_TYPE_REG)).len);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"update reg failed".as_ptr());
        return ret;
    }
    aw88395_dev_mute(aw_dev, true);
    if (*aw_dev).dsp_cfg == AW88395_DEV_DSP_WORK {
        aw_dev_dsp_enable(aw_dev, false);
    }
    aw_dev_select_memclk(aw_dev, AW88395_DEV_MEMCLK_OSC);
    if up_dsp_fw_en {
        ret = aw_dev_check_sram(aw_dev);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"check sram failed".as_ptr());
            aw_dev_select_memclk(aw_dev, AW88395_DEV_MEMCLK_PLL);
            return ret;
        }
        /* update dsp firmware */
        dev_dbg((*aw_dev).dev, c"fw_ver: [%x]".as_ptr(), (*prof_index_desc).fw_ver);
        ret = aw_dev_dsp_update_fw(aw_dev, (*sec_desc.add(AW88395_DATA_TYPE_DSP_FW)).data, (*sec_desc.add(AW88395_DATA_TYPE_DSP_FW)).len);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"update dsp fw failed".as_ptr());
            aw_dev_select_memclk(aw_dev, AW88395_DEV_MEMCLK_PLL);
            return ret;
        }
    }
    ret = aw_dev_dsp_update_cfg(aw_dev, (*sec_desc.add(AW88395_DATA_TYPE_DSP_CFG)).data, (*sec_desc.add(AW88395_DATA_TYPE_DSP_CFG)).len);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"update dsp cfg failed".as_ptr());
        aw_dev_select_memclk(aw_dev, AW88395_DEV_MEMCLK_PLL);
        return ret;
    }
    aw_dev_select_memclk(aw_dev, AW88395_DEV_MEMCLK_PLL);
    (*aw_dev).prof_cur = (*aw_dev).prof_index;
    0
}

unsafe fn aw_dev_dsp_check(aw_dev: *mut aw_device) -> c_int {
    let mut ret: c_int = 0;
    if (*aw_dev).dsp_cfg == AW88395_DEV_DSP_BYPASS {
        dev_dbg((*aw_dev).dev, c"dsp bypass".as_ptr());
        ret = 0;
    } else if (*aw_dev).dsp_cfg == AW88395_DEV_DSP_WORK {
        aw_dev_dsp_enable(aw_dev, false);
        aw_dev_dsp_enable(aw_dev, true);
        usleep_range(AW88395_1000_US, AW88395_1000_US + 10);
        let mut i = 0;
        while i < AW88395_DEV_DSP_CHECK_MAX {
            ret = aw_dev_get_dsp_status(aw_dev);
            if ret != 0 {
                dev_err((*aw_dev).dev, c"dsp wdt status error=%d".as_ptr(), ret);
                usleep_range(AW88395_2000_US, AW88395_2000_US + 10);
            }
            i += 1;
        }
    } else {
        dev_err((*aw_dev).dev, c"unknown dsp cfg=%d".as_ptr(), (*aw_dev).dsp_cfg as c_int);
        ret = -EINVAL;
    }
    ret
}

unsafe fn aw_dev_update_cali_re(cali_desc: *mut aw_cali_desc) {
    let aw_dev = aw_dev_from_cali_desc(cali_desc);
    if (*aw_dev).cali_desc.cali_re < AW88395_CALI_RE_MAX && (*aw_dev).cali_desc.cali_re > AW88395_CALI_RE_MIN {
        let ret = aw_dev_dsp_set_cali_re(aw_dev);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"set cali re failed".as_ptr());
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn aw88395_dev_start(aw_dev: *mut aw_device) -> c_int {
    let mut ret: c_int;

    if (*aw_dev).status == AW88395_DEV_PW_ON {
        dev_info((*aw_dev).dev, c"already power on".as_ptr());
        return 0;
    }
    /* power on */
    aw_dev_pwd(aw_dev, false);
    usleep_range(AW88395_2000_US, AW88395_2000_US + 10);

    ret = aw_dev_check_syspll(aw_dev);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"pll check failed cannot start".as_ptr());
        aw_dev_pwd(aw_dev, true);
        (*aw_dev).status = AW88395_DEV_PW_OFF;
        return ret;
    }

    /* amppd on */
    aw_dev_amppd(aw_dev, false);
    usleep_range(AW88395_1000_US, AW88395_1000_US + 50);

    /* check i2s status */
    ret = aw_dev_check_sysst(aw_dev);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"sysst check failed".as_ptr());
        aw_dev_clear_int_status(aw_dev);
        aw_dev_amppd(aw_dev, true);
        aw_dev_pwd(aw_dev, true);
        (*aw_dev).status = AW88395_DEV_PW_OFF;
        return ret;
    }

    if (*aw_dev).dsp_cfg == AW88395_DEV_DSP_WORK {
        /* dsp bypass */
        aw_dev_dsp_enable(aw_dev, false);
        ret = aw_dev_dsp_fw_check(aw_dev);
        if ret != 0 {
            aw_dev_clear_int_status(aw_dev);
            aw_dev_amppd(aw_dev, true);
            aw_dev_pwd(aw_dev, true);
            (*aw_dev).status = AW88395_DEV_PW_OFF;
            return ret;
        }

        aw_dev_update_cali_re(&mut (*aw_dev).cali_desc);

        if (*aw_dev).dsp_crc_st != AW88395_DSP_CRC_OK {
            ret = aw_dev_dsp_check_crc32(aw_dev);
            if ret != 0 {
                dev_err((*aw_dev).dev, c"dsp crc check failed".as_ptr());
                aw_dev_dsp_enable(aw_dev, false);
                aw_dev_clear_int_status(aw_dev);
                aw_dev_amppd(aw_dev, true);
                aw_dev_pwd(aw_dev, true);
                (*aw_dev).status = AW88395_DEV_PW_OFF;
                return ret;
            }
        }

        ret = aw_dev_dsp_check(aw_dev);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"dsp status check failed".as_ptr());
            aw_dev_dsp_enable(aw_dev, false);
            aw_dev_clear_int_status(aw_dev);
            aw_dev_amppd(aw_dev, true);
            aw_dev_pwd(aw_dev, true);
            (*aw_dev).status = AW88395_DEV_PW_OFF;
            return ret;
        }
    } else {
        dev_dbg((*aw_dev).dev, c"start pa with dsp bypass".as_ptr());
    }

    /* enable tx feedback */
    aw_dev_i2s_tx_enable(aw_dev, true);
    /* close mute */
    aw88395_dev_mute(aw_dev, false);
    /* clear inturrupt */
    aw_dev_clear_int_status(aw_dev);
    (*aw_dev).status = AW88395_DEV_PW_ON;
    0
}

#[no_mangle]
pub unsafe extern "C" fn aw88395_dev_stop(aw_dev: *mut aw_device) -> c_int {
    let dsp_cfg: *mut aw_sec_data_desc = (*(*aw_dev).prof_info.prof_desc.add((*aw_dev).prof_cur as usize)).sec_desc.add(AW88395_DATA_TYPE_DSP_CFG);
    let dsp_fw: *mut aw_sec_data_desc = (*(*aw_dev).prof_info.prof_desc.add((*aw_dev).prof_cur as usize)).sec_desc.add(AW88395_DATA_TYPE_DSP_FW);
    let int_st: c_int;
    let mut ret: c_int;

    if (*aw_dev).status == AW88395_DEV_PW_OFF {
        dev_info((*aw_dev).dev, c"already power off".as_ptr());
        return 0;
    }

    (*aw_dev).status = AW88395_DEV_PW_OFF;
    /* set mute */
    aw88395_dev_mute(aw_dev, true);
    usleep_range(AW88395_4000_US, AW88395_4000_US + 100);
    /* close tx feedback */
    aw_dev_i2s_tx_enable(aw_dev, false);
    usleep_range(AW88395_1000_US, AW88395_1000_US + 100);
    /* check sysint state */
    int_st = aw_dev_check_sysint(aw_dev);
    /* close dsp */
    aw_dev_dsp_enable(aw_dev, false);
    /* enable amppd */
    aw_dev_amppd(aw_dev, true);

    if int_st < 0 {
        /* system status anomaly */
        aw_dev_select_memclk(aw_dev, AW88395_DEV_MEMCLK_OSC);
        ret = aw_dev_dsp_update_fw(aw_dev, (*dsp_fw).data, (*dsp_fw).len);
        if ret != 0 { dev_err((*aw_dev).dev, c"update dsp fw failed".as_ptr()); }
        ret = aw_dev_dsp_update_cfg(aw_dev, (*dsp_cfg).data, (*dsp_cfg).len);
        if ret != 0 { dev_err((*aw_dev).dev, c"update dsp cfg failed".as_ptr()); }
        aw_dev_select_memclk(aw_dev, AW88395_DEV_MEMCLK_PLL);
    }
    /* set power down */
    aw_dev_pwd(aw_dev, true);
    0
}

#[no_mangle]
pub unsafe extern "C" fn aw88395_dev_init(aw_dev: *mut aw_device, aw_cfg: *mut aw_container) -> c_int {
    let mut ret: c_int;

    if aw_dev.is_null() || aw_cfg.is_null() {
        pr_err(c"aw_dev is NULL or aw_cfg is NULL".as_ptr());
        return -ENOMEM;
    }
    ret = aw88395_dev_cfg_load(aw_dev, aw_cfg);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"aw_dev acf parse failed".as_ptr());
        return -EINVAL;
    }
    (*aw_dev).fade_in_time = AW88395_1000_US / 10;
    (*aw_dev).fade_out_time = AW88395_1000_US >> 1;
    (*aw_dev).prof_cur = (*(*aw_dev).prof_info.prof_desc.add(0)).id;
    (*aw_dev).prof_index = (*(*aw_dev).prof_info.prof_desc.add(0)).id;

    ret = aw88395_dev_fw_update(aw_dev, AW88395_FORCE_UPDATE_ON, AW88395_DSP_FW_UPDATE_ON);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"fw update failed ret = %d\n".as_ptr(), ret);
        return ret;
    }
    /* set mute */
    aw88395_dev_mute(aw_dev, true);
    usleep_range(AW88395_4000_US, AW88395_4000_US + 100);
    /* close tx feedback */
    aw_dev_i2s_tx_enable(aw_dev, false);
    usleep_range(AW88395_1000_US, AW88395_1000_US + 100);
    /* close dsp */
    aw_dev_dsp_enable(aw_dev, false);
    /* enable amppd */
    aw_dev_amppd(aw_dev, true);
    /* set power down */
    aw_dev_pwd(aw_dev, true);
    0
}

unsafe fn aw88395_parse_channel_dt(aw_dev: *mut aw_device) {
    let np: *mut device_node = (*(*aw_dev).dev).of_node;
    let mut channel_value: u32 = 0;
    let ret = of_property_read_u32(np, c"awinic,audio-channel".as_ptr(), &mut channel_value);
    if ret != 0 {
        dev_dbg((*aw_dev).dev, c"read audio-channel failed,use default 0".as_ptr());
        (*aw_dev).channel = AW88395_DEV_DEFAULT_CH;
        return;
    }
    dev_dbg((*aw_dev).dev, c"read audio-channel value is: %d".as_ptr(), channel_value);
    (*aw_dev).channel = channel_value;
}

unsafe fn aw_dev_init(aw_dev: *mut aw_device) -> c_int {
    (*aw_dev).chip_id = AW88395_CHIP_ID;
    /* call aw device init func */
    (*aw_dev).acf = ptr::null_mut();
    (*aw_dev).prof_info.prof_desc = ptr::null_mut();
    (*aw_dev).prof_info.count = 0;
    (*aw_dev).prof_info.prof_type = AW88395_DEV_NONE_TYPE_ID;
    (*aw_dev).channel = 0;
    (*aw_dev).fw_status = AW88395_DEV_FW_FAILED;
    (*aw_dev).fade_step = AW88395_VOLUME_STEP_DB as c_int;
    (*aw_dev).volume_desc.ctl_volume = AW88395_VOL_DEFAULT_VALUE;
    aw88395_parse_channel_dt(aw_dev);
    0
}

#[no_mangle]
pub unsafe extern "C" fn aw88395_dev_get_profile_count(aw_dev: *mut aw_device) -> c_int {
    (*aw_dev).prof_info.count
}

#[no_mangle]
pub unsafe extern "C" fn aw88395_dev_get_profile_index(aw_dev: *mut aw_device) -> c_int {
    (*aw_dev).prof_index
}

#[no_mangle]
pub unsafe extern "C" fn aw88395_dev_set_profile_index(aw_dev: *mut aw_device, index: c_int) -> c_int {
    /* check the index whether is valid */
    if index >= (*aw_dev).prof_info.count || index < 0 {
        return -EINVAL;
    }
    /* check the index whether change */
    if (*aw_dev).prof_index == index {
        return -EINVAL;
    }
    (*aw_dev).prof_index = index;
    dev_dbg((*aw_dev).dev, c"set prof[%s]".as_ptr(), *(*aw_dev).prof_info.prof_name_list.add((*(*aw_dev).prof_info.prof_desc.add(index as usize)).id as usize));
    0
}

#[no_mangle]
pub unsafe extern "C" fn aw88395_dev_get_prof_name(aw_dev: *mut aw_device, index: c_int, prof_name: *mut *mut c_char) -> c_int {
    let prof_info: *mut aw_prof_info = &mut (*aw_dev).prof_info;
    let prof_desc: *mut aw_prof_desc;
    if index >= (*aw_dev).prof_info.count || index < 0 {
        dev_err((*aw_dev).dev, c"index[%d] overflow count[%d]".as_ptr(), index, (*aw_dev).prof_info.count);
        return -EINVAL;
    }
    prof_desc = (*aw_dev).prof_info.prof_desc.add(index as usize);
    *prof_name = *(*prof_info).prof_name_list.add((*prof_desc).id as usize);
    0
}

#[no_mangle]
pub unsafe extern "C" fn aw88395_dev_get_prof_data(aw_dev: *mut aw_device, index: c_int, prof_desc: *mut *mut aw_prof_desc) -> c_int {
    if index >= (*aw_dev).prof_info.count || index < 0 {
        dev_err((*aw_dev).dev, c"%s: index[%d] overflow count[%d]\n".as_ptr(), c"aw88395_dev_get_prof_data".as_ptr(), index, (*aw_dev).prof_info.count);
        return -EINVAL;
    }
    *prof_desc = (*aw_dev).prof_info.prof_desc.add(index as usize);
    0
}

#[no_mangle]
pub unsafe extern "C" fn aw88395_init(aw_dev: *mut *mut aw_device, i2c: *mut i2c_client, regmap: *mut regmap) -> c_int {
    let mut chip_id: u16 = 0;
    let mut ret: c_int;

    if !(*aw_dev).is_null() {
        dev_info(&mut (*i2c).dev, c"it should be initialized here.\n".as_ptr());
    } else {
        *aw_dev = devm_kzalloc(&mut (*i2c).dev, size_of::<aw_device>(), GFP_KERNEL) as *mut aw_device;
        if (*aw_dev).is_null() {
            return -ENOMEM;
        }
    }
    (**aw_dev).i2c = i2c;
    (**aw_dev).dev = &mut (*i2c).dev;
    (**aw_dev).regmap = regmap;
    mutex_init(&mut (**aw_dev).dsp_lock);

    /* read chip id */
    ret = aw_dev_read_chipid(*aw_dev, &mut chip_id);
    if ret != 0 {
        dev_err(&mut (*i2c).dev, c"dev_read_chipid failed ret=%d".as_ptr(), ret);
        return ret;
    }

    if chip_id == AW88395_CHIP_ID {
        ret = aw_dev_init(*aw_dev);
    } else {
        ret = -EINVAL;
        dev_err((**aw_dev).dev, c"unsupported device".as_ptr());
    }
    ret
}

/* MODULE_DESCRIPTION("AW88395 device lib"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
