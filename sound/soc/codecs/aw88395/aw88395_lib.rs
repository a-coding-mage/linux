// SPDX-License-Identifier: GPL-2.0-only
//
// aw88395_lib.c  -- ACF bin parsing and check library file for aw88395
//
// Copyright (c) 2022-2023 AWINIC Technology CO., LTD
//
// Author: Bruce zhao <zhaolei@awinic.com>
//

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

use crate::*;

const AW88395_CRC8_POLYNOMIAL: u8 = 0x8C;

static mut aw_crc8_table: [u8; 256] = [0; 256];

static mut profile_name: [*mut c_char; 11] = [
    b"Music\0".as_ptr() as *mut c_char,
    b"Voice\0".as_ptr() as *mut c_char,
    b"Voip\0".as_ptr() as *mut c_char,
    b"Ringtone\0".as_ptr() as *mut c_char,
    b"Ringtone_hs\0".as_ptr() as *mut c_char,
    b"Lowpower\0".as_ptr() as *mut c_char,
    b"Bypass\0".as_ptr() as *mut c_char,
    b"Mmi\0".as_ptr() as *mut c_char,
    b"Fm\0".as_ptr() as *mut c_char,
    b"Notification\0".as_ptr() as *mut c_char,
    b"Receiver\0".as_ptr() as *mut c_char,
];

unsafe extern "C" {
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn devm_kcalloc(dev: *mut c_void, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut c_void, size: usize, flags: c_uint) -> *mut c_void;
    fn swab16_array(buf: *mut u16, words: c_uint);
    fn crc8(table: *const u8, pdata: *const u8, nbytes: c_uint, crc: u8) -> u8;
    fn crc8_populate_lsb(table: *mut u8, polynomial: u8);
}

unsafe fn le32_to_cpup(p: *const c_void) -> c_uint {
    u32::from_le_bytes([
        *(p as *const u8),
        *((p as *const u8).add(1)),
        *((p as *const u8).add(2)),
        *((p as *const u8).add(3)),
    ]) as c_uint
}

unsafe fn aw_parse_bin_header(aw_dev: *mut aw_device, bin: *mut aw_bin) -> c_int;

unsafe fn aw_check_sum(aw_dev: *mut aw_device, bin: *mut aw_bin, bin_num: c_int) -> c_int {
    let mut sum_data: c_uint = 0;
    let p_check_sum = (*bin).info.data.as_mut_ptr().add(
        ((*bin).header_info[bin_num as usize].valid_data_addr
            - (*bin).header_info[bin_num as usize].header_len) as usize,
    );
    let len = (*bin).header_info[bin_num as usize].bin_data_len
        + (*bin).header_info[bin_num as usize].header_len;
    let check_sum = le32_to_cpup(p_check_sum as *const c_void);

    let mut i: c_uint = 4;
    while i < len {
        sum_data = sum_data.wrapping_add(*p_check_sum.add(i as usize) as c_uint);
        i += 1;
    }

    dev_dbg!(
        (*aw_dev).dev,
        "%s -- check_sum = %p, check_sum = 0x%x, sum_data = 0x%x",
        "aw_check_sum",
        p_check_sum,
        check_sum,
        sum_data
    );
    if sum_data != check_sum {
        dev_err!(
            (*aw_dev).dev,
            "%s. CheckSum Fail.bin_num=%d, CheckSum:0x%x, SumData:0x%x",
            "aw_check_sum",
            bin_num,
            check_sum,
            sum_data
        );
        return -EINVAL;
    }

    0
}

unsafe fn aw_check_data_version(aw_dev: *mut aw_device, bin: *mut aw_bin, bin_num: c_int) -> c_int {
    if (*bin).header_info[bin_num as usize].bin_data_ver < DATA_VERSION_V1
        || (*bin).header_info[bin_num as usize].bin_data_ver > DATA_VERSION_MAX
    {
        dev_err!(
            (*aw_dev).dev,
            "aw_bin_parse Unrecognized this bin data version\n"
        );
        return -EINVAL;
    }

    0
}

unsafe fn aw_check_register_num(
    aw_dev: *mut aw_device,
    bin: *mut aw_bin,
    bin_num: c_int,
) -> c_int {
    let temp_info = (*bin).header_info[bin_num as usize];
    let p_check_sum = (*bin)
        .info
        .data
        .as_mut_ptr()
        .add(temp_info.valid_data_addr as usize);

    let parse_register_num = le32_to_cpup(p_check_sum as *const c_void);
    let check_register_num =
        ((*bin).header_info[bin_num as usize].bin_data_len - CHECK_REGISTER_NUM_OFFSET)
            / ((*bin).header_info[bin_num as usize].reg_byte_len
                + (*bin).header_info[bin_num as usize].data_byte_len);
    dev_dbg!(
        (*aw_dev).dev,
        "%s,parse_register_num = 0x%x,check_register_num = 0x%x\n",
        "aw_check_register_num",
        parse_register_num,
        check_register_num
    );
    if parse_register_num != check_register_num {
        dev_err!(
            (*aw_dev).dev,
            "%s parse_register_num = 0x%x,check_register_num = 0x%x\n",
            "aw_check_register_num",
            parse_register_num,
            check_register_num
        );
        return -EINVAL;
    }

    (*bin).header_info[bin_num as usize].reg_num = parse_register_num;
    (*bin).header_info[bin_num as usize].valid_data_len = temp_info.bin_data_len - VALID_DATA_LEN;
    (*bin).header_info[bin_num as usize].valid_data_addr =
        temp_info.valid_data_addr + VALID_DATA_ADDR;

    0
}

unsafe fn aw_check_dsp_reg_num(
    aw_dev: *mut aw_device,
    bin: *mut aw_bin,
    bin_num: c_int,
) -> c_int {
    let temp_info = (*bin).header_info[bin_num as usize];
    let p_check_sum = (*bin)
        .info
        .data
        .as_mut_ptr()
        .add(temp_info.valid_data_addr as usize);

    let parse_dsp_reg_num = le32_to_cpup(p_check_sum.add(PARSE_DSP_REG_NUM as usize) as *const c_void);
    (*bin).header_info[bin_num as usize].reg_data_byte_len =
        le32_to_cpup(p_check_sum.add(REG_DATA_BYTP_LEN as usize) as *const c_void);
    let check_dsp_reg_num =
        ((*bin).header_info[bin_num as usize].bin_data_len - CHECK_DSP_REG_NUM)
            / (*bin).header_info[bin_num as usize].reg_data_byte_len;
    dev_dbg!(
        (*aw_dev).dev,
        "%s bin_num = %d, parse_dsp_reg_num = 0x%x, check_dsp_reg_num = 0x%x",
        "aw_check_dsp_reg_num",
        bin_num,
        check_dsp_reg_num,
        check_dsp_reg_num
    );
    if parse_dsp_reg_num != check_dsp_reg_num {
        dev_err!((*aw_dev).dev, "aw_bin_parse check dsp reg num error\n");
        dev_err!(
            (*aw_dev).dev,
            "%s parse_dsp_reg_num = 0x%x, check_dsp_reg_num = 0x%x",
            "aw_check_dsp_reg_num",
            check_dsp_reg_num,
            check_dsp_reg_num
        );
        return -EINVAL;
    }

    (*bin).header_info[bin_num as usize].download_addr = le32_to_cpup(p_check_sum as *const c_void);
    (*bin).header_info[bin_num as usize].reg_num = parse_dsp_reg_num;
    (*bin).header_info[bin_num as usize].valid_data_len =
        temp_info.bin_data_len - DSP_VALID_DATA_LEN;
    (*bin).header_info[bin_num as usize].valid_data_addr =
        temp_info.valid_data_addr + DSP_VALID_DATA_ADDR;

    0
}

unsafe fn aw_check_soc_app_num(
    aw_dev: *mut aw_device,
    bin: *mut aw_bin,
    bin_num: c_int,
) -> c_int {
    let temp_info = (*bin).header_info[bin_num as usize];
    let p_check_sum = (*bin)
        .info
        .data
        .as_mut_ptr()
        .add(temp_info.valid_data_addr as usize);

    (*bin).header_info[bin_num as usize].app_version = le32_to_cpup(p_check_sum as *const c_void);
    let parse_soc_app_num = le32_to_cpup(p_check_sum.add(PARSE_SOC_APP_NUM as usize) as *const c_void);
    let check_soc_app_num = (*bin).header_info[bin_num as usize].bin_data_len - CHECK_SOC_APP_NUM;
    dev_dbg!(
        (*aw_dev).dev,
        "%s bin_num = %d, parse_soc_app_num=0x%x, check_soc_app_num = 0x%x\n",
        "aw_check_soc_app_num",
        bin_num,
        parse_soc_app_num,
        check_soc_app_num
    );
    if parse_soc_app_num != check_soc_app_num {
        dev_err!(
            (*aw_dev).dev,
            "%s parse_soc_app_num=0x%x, check_soc_app_num = 0x%x\n",
            "aw_check_soc_app_num",
            parse_soc_app_num,
            check_soc_app_num
        );
        return -EINVAL;
    }

    (*bin).header_info[bin_num as usize].reg_num = parse_soc_app_num;
    (*bin).header_info[bin_num as usize].download_addr =
        le32_to_cpup(p_check_sum.add(APP_DOWNLOAD_ADDR as usize) as *const c_void);
    (*bin).header_info[bin_num as usize].valid_data_len =
        temp_info.bin_data_len - APP_VALID_DATA_LEN;
    (*bin).header_info[bin_num as usize].valid_data_addr =
        temp_info.valid_data_addr + APP_VALID_DATA_ADDR;

    0
}

unsafe fn aw_get_single_bin_header(bin: *mut aw_bin) {
    memcpy(
        &mut (*bin).header_info[(*bin).all_bin_parse_num as usize] as *mut _ as *mut c_void,
        (*bin).p_addr as *const c_void,
        DATA_LEN as usize,
    );

    (*bin).header_info[(*bin).all_bin_parse_num as usize].header_len = HEADER_LEN;
    (*bin).all_bin_parse_num += 1;
}

unsafe fn aw_parse_one_of_multi_bins(
    aw_dev: *mut aw_device,
    bin_num: c_uint,
    bin_serial_num: c_int,
    bin: *mut aw_bin,
) -> c_int {
    if (*bin).info.len < size_of::<bin_header_info>() as c_uint {
        dev_err!(
            (*aw_dev).dev,
            "bin_header_info size[%d] overflow file size[%d]\n",
            size_of::<bin_header_info>() as c_int,
            (*bin).info.len
        );
        return -EINVAL;
    }

    let aw_bin_header_info = (*bin).header_info[((*bin).all_bin_parse_num - 1) as usize];
    if bin_serial_num == 0 {
        let bin_start_addr =
            le32_to_cpup((*bin).p_addr.add(START_ADDR_OFFSET as usize) as *const c_void);
        (*bin).p_addr = (*bin).p_addr.add((HEADER_LEN + bin_start_addr) as usize);
        (*bin).header_info[(*bin).all_bin_parse_num as usize].valid_data_addr =
            aw_bin_header_info.valid_data_addr
                + VALID_DATA_ADDR
                + 8 * bin_num
                + VALID_DATA_ADDR_OFFSET;
    } else {
        let valid_data_len = aw_bin_header_info.bin_data_len;
        (*bin).p_addr = (*bin).p_addr.add((HDADER_LEN + valid_data_len) as usize);
        (*bin).header_info[(*bin).all_bin_parse_num as usize].valid_data_addr =
            aw_bin_header_info.valid_data_addr
                + aw_bin_header_info.bin_data_len
                + VALID_DATA_ADDR_OFFSET;
    }

    aw_parse_bin_header(aw_dev, bin)
}

unsafe fn aw_get_multi_bin_header(aw_dev: *mut aw_device, bin: *mut aw_bin) -> c_int {
    let bin_num = le32_to_cpup((*bin).p_addr.add(VALID_DATA_ADDR_OFFSET as usize) as *const c_void);
    if (*bin).multi_bin_parse_num == 1 {
        (*bin).header_info[(*bin).all_bin_parse_num as usize].valid_data_addr =
            VALID_DATA_ADDR_OFFSET;
    }

    aw_get_single_bin_header(bin);

    let mut i: c_uint = 0;
    while i < bin_num {
        dev_dbg!((*aw_dev).dev, "aw_bin_parse enter multi bin for is %d\n", i);
        let ret = aw_parse_one_of_multi_bins(aw_dev, bin_num, i as c_int, bin);
        if ret < 0 {
            return ret;
        }
        i += 1;
    }

    0
}

unsafe fn aw_parse_bin_header(aw_dev: *mut aw_device, bin: *mut aw_bin) -> c_int {
    if (*bin).info.len < size_of::<bin_header_info>() as c_uint {
        dev_err!(
            (*aw_dev).dev,
            "bin_header_info size[%d] overflow file size[%d]\n",
            size_of::<bin_header_info>() as c_int,
            (*bin).info.len
        );
        return -EINVAL;
    }

    let bin_data_type =
        le32_to_cpup((*bin).p_addr.add(BIN_DATA_TYPE_OFFSET as usize) as *const c_void);
    dev_dbg!((*aw_dev).dev, "aw_bin_parse bin_data_type 0x%x\n", bin_data_type);
    match bin_data_type {
        DATA_TYPE_REGISTER | DATA_TYPE_DSP_REG | DATA_TYPE_SOC_APP => {
            (*bin).single_bin_parse_num += 1;
            dev_dbg!(
                (*aw_dev).dev,
                "%s bin->single_bin_parse_num is %d\n",
                "aw_parse_bin_header",
                (*bin).single_bin_parse_num
            );
            if (*bin).multi_bin_parse_num == 0 {
                (*bin).header_info[(*bin).all_bin_parse_num as usize].valid_data_addr =
                    VALID_DATA_ADDR_OFFSET;
            }
            aw_get_single_bin_header(bin);
            0
        }
        DATA_TYPE_MULTI_BINS => {
            (*bin).multi_bin_parse_num += 1;
            dev_dbg!(
                (*aw_dev).dev,
                "%s bin->multi_bin_parse_num is %d\n",
                "aw_parse_bin_header",
                (*bin).multi_bin_parse_num
            );
            aw_get_multi_bin_header(aw_dev, bin)
        }
        _ => {
            dev_dbg!((*aw_dev).dev, "%s There is no corresponding type\n", "aw_parse_bin_header");
            0
        }
    }
}

unsafe fn aw_check_bin_header_version(aw_dev: *mut aw_device, bin: *mut aw_bin) -> c_int {
    let header_version =
        le32_to_cpup((*bin).p_addr.add(HEADER_VERSION_OFFSET as usize) as *const c_void);
    dev_dbg!((*aw_dev).dev, "aw_bin_parse header_version 0x%x\n", header_version);

    match header_version {
        HEADER_VERSION_V1 => aw_parse_bin_header(aw_dev, bin),
        _ => {
            dev_err!(
                (*aw_dev).dev,
                "aw_bin_parse Unrecognized this bin header version\n"
            );
            -EINVAL
        }
    }
}

unsafe fn aw_parsing_bin_file(aw_dev: *mut aw_device, bin: *mut aw_bin) -> c_int {
    let mut ret: c_int = -EINVAL;

    if bin.is_null() {
        dev_err!((*aw_dev).dev, "aw_bin_parse bin is NULL\n");
        return ret;
    }
    (*bin).p_addr = (*bin).info.data.as_mut_ptr();
    (*bin).all_bin_parse_num = 0;
    (*bin).multi_bin_parse_num = 0;
    (*bin).single_bin_parse_num = 0;

    ret = aw_check_bin_header_version(aw_dev, bin);
    if ret < 0 {
        dev_err!((*aw_dev).dev, "aw_bin_parse check bin header version error\n");
        return ret;
    }

    let mut i: c_int = 0;
    while i < (*bin).all_bin_parse_num as c_int {
        ret = aw_check_sum(aw_dev, bin, i);
        if ret < 0 {
            dev_err!((*aw_dev).dev, "aw_bin_parse check sum data error\n");
            return ret;
        }
        ret = aw_check_data_version(aw_dev, bin, i);
        if ret < 0 {
            dev_err!((*aw_dev).dev, "aw_bin_parse check data version error\n");
            return ret;
        }
        if (*bin).header_info[i as usize].bin_data_ver == DATA_VERSION_V1 {
            ret = match (*bin).header_info[i as usize].bin_data_type {
                DATA_TYPE_REGISTER => aw_check_register_num(aw_dev, bin, i),
                DATA_TYPE_DSP_REG => aw_check_dsp_reg_num(aw_dev, bin, i),
                DATA_TYPE_SOC_APP => aw_check_soc_app_num(aw_dev, bin, i),
                _ => {
                    (*bin).header_info[i as usize].valid_data_len =
                        (*bin).header_info[i as usize].bin_data_len;
                    0
                }
            };
            if ret < 0 {
                return ret;
            }
        }
        i += 1;
    }

    0
}

unsafe fn aw_dev_parse_raw_reg(
    data: *mut u8,
    data_len: c_uint,
    prof_desc: *mut aw_prof_desc,
) -> c_int {
    (*prof_desc).sec_desc[AW88395_DATA_TYPE_REG as usize].data = data;
    (*prof_desc).sec_desc[AW88395_DATA_TYPE_REG as usize].len = data_len;
    (*prof_desc).prof_st = AW88395_PROFILE_OK;
    0
}

unsafe fn aw_dev_parse_raw_dsp_cfg(
    data: *mut u8,
    data_len: c_uint,
    prof_desc: *mut aw_prof_desc,
) -> c_int {
    if data_len & 0x01 != 0 {
        return -EINVAL;
    }
    swab16_array(data as *mut u16, data_len >> 1);
    (*prof_desc).sec_desc[AW88395_DATA_TYPE_DSP_CFG as usize].data = data;
    (*prof_desc).sec_desc[AW88395_DATA_TYPE_DSP_CFG as usize].len = data_len;
    (*prof_desc).prof_st = AW88395_PROFILE_OK;
    0
}

unsafe fn aw_dev_parse_raw_dsp_fw(
    data: *mut u8,
    data_len: c_uint,
    prof_desc: *mut aw_prof_desc,
) -> c_int {
    if data_len & 0x01 != 0 {
        return -EINVAL;
    }
    swab16_array(data as *mut u16, data_len >> 1);
    (*prof_desc).sec_desc[AW88395_DATA_TYPE_DSP_FW as usize].data = data;
    (*prof_desc).sec_desc[AW88395_DATA_TYPE_DSP_FW as usize].len = data_len;
    (*prof_desc).prof_st = AW88395_PROFILE_OK;
    0
}

unsafe fn aw_dev_prof_parse_multi_bin(
    aw_dev: *mut aw_device,
    data: *mut u8,
    data_len: c_uint,
    prof_desc: *mut aw_prof_desc,
) -> c_int {
    let aw_bin = kzalloc(data_len as usize + size_of::<aw_bin>(), GFP_KERNEL) as *mut aw_bin;
    if aw_bin.is_null() {
        return -ENOMEM;
    }

    (*aw_bin).info.len = data_len;
    memcpy((*aw_bin).info.data.as_mut_ptr() as *mut c_void, data as *const c_void, data_len as usize);

    let mut ret = aw_parsing_bin_file(aw_dev, aw_bin);
    if ret < 0 {
        dev_err!((*aw_dev).dev, "parse bin failed");
        kfree(aw_bin as *mut c_void);
        return ret;
    }

    let mut i: c_int = 0;
    while i < (*aw_bin).all_bin_parse_num as c_int {
        match (*aw_bin).header_info[i as usize].bin_data_type {
            DATA_TYPE_REGISTER => {
                (*prof_desc).sec_desc[AW88395_DATA_TYPE_REG as usize].len =
                    (*aw_bin).header_info[i as usize].valid_data_len;
                (*prof_desc).sec_desc[AW88395_DATA_TYPE_REG as usize].data =
                    data.add((*aw_bin).header_info[i as usize].valid_data_addr as usize);
            }
            DATA_TYPE_DSP_REG => {
                if (*aw_bin).header_info[i as usize].valid_data_len & 0x01 != 0 {
                    kfree(aw_bin as *mut c_void);
                    return -EINVAL;
                }
                swab16_array(
                    data.add((*aw_bin).header_info[i as usize].valid_data_addr as usize) as *mut u16,
                    (*aw_bin).header_info[i as usize].valid_data_len >> 1,
                );
                (*prof_desc).sec_desc[AW88395_DATA_TYPE_DSP_CFG as usize].len =
                    (*aw_bin).header_info[i as usize].valid_data_len;
                (*prof_desc).sec_desc[AW88395_DATA_TYPE_DSP_CFG as usize].data =
                    data.add((*aw_bin).header_info[i as usize].valid_data_addr as usize);
            }
            DATA_TYPE_DSP_FW | DATA_TYPE_SOC_APP => {
                if (*aw_bin).header_info[i as usize].valid_data_len & 0x01 != 0 {
                    kfree(aw_bin as *mut c_void);
                    return -EINVAL;
                }
                swab16_array(
                    data.add((*aw_bin).header_info[i as usize].valid_data_addr as usize) as *mut u16,
                    (*aw_bin).header_info[i as usize].valid_data_len >> 1,
                );
                (*prof_desc).fw_ver = (*aw_bin).header_info[i as usize].app_version;
                (*prof_desc).sec_desc[AW88395_DATA_TYPE_DSP_FW as usize].len =
                    (*aw_bin).header_info[i as usize].valid_data_len;
                (*prof_desc).sec_desc[AW88395_DATA_TYPE_DSP_FW as usize].data =
                    data.add((*aw_bin).header_info[i as usize].valid_data_addr as usize);
            }
            _ => dev_dbg!((*aw_dev).dev, "bin_data_type not found"),
        }
        i += 1;
    }
    (*prof_desc).prof_st = AW88395_PROFILE_OK;
    kfree(aw_bin as *mut c_void);
    0
}

unsafe fn aw_dev_parse_reg_bin_with_hdr(
    aw_dev: *mut aw_device,
    data: *mut u8,
    data_len: u32,
    prof_desc: *mut aw_prof_desc,
) -> c_int {
    let aw_bin = kzalloc(data_len as usize + size_of::<aw_bin>(), GFP_KERNEL) as *mut aw_bin;
    if aw_bin.is_null() {
        return -ENOMEM;
    }

    (*aw_bin).info.len = data_len;
    memcpy((*aw_bin).info.data.as_mut_ptr() as *mut c_void, data as *const c_void, data_len as usize);

    let ret = aw_parsing_bin_file(aw_dev, aw_bin);
    if ret < 0 {
        dev_err!((*aw_dev).dev, "parse bin failed");
        kfree(aw_bin as *mut c_void);
        return ret;
    }

    if (*aw_bin).all_bin_parse_num != 1
        || (*aw_bin).header_info[0].bin_data_type != DATA_TYPE_REGISTER
    {
        dev_err!((*aw_dev).dev, "bin num or type error");
        kfree(aw_bin as *mut c_void);
        return -EINVAL;
    }

    (*prof_desc).sec_desc[AW88395_DATA_TYPE_REG as usize].data =
        data.add((*aw_bin).header_info[0].valid_data_addr as usize);
    (*prof_desc).sec_desc[AW88395_DATA_TYPE_REG as usize].len =
        (*aw_bin).header_info[0].valid_data_len;
    (*prof_desc).prof_st = AW88395_PROFILE_OK;
    kfree(aw_bin as *mut c_void);
    0
}

unsafe fn aw_dev_parse_data_by_sec_type(
    aw_dev: *mut aw_device,
    cfg_hdr: *mut aw_cfg_hdr,
    cfg_dde: *mut aw_cfg_dde,
    scene_prof_desc: *mut aw_prof_desc,
) -> c_int {
    match (*cfg_dde).data_type {
        ACF_SEC_TYPE_REG => aw_dev_parse_raw_reg(
            (cfg_hdr as *mut u8).add((*cfg_dde).data_offset as usize),
            (*cfg_dde).data_size,
            scene_prof_desc,
        ),
        ACF_SEC_TYPE_DSP_CFG => aw_dev_parse_raw_dsp_cfg(
            (cfg_hdr as *mut u8).add((*cfg_dde).data_offset as usize),
            (*cfg_dde).data_size,
            scene_prof_desc,
        ),
        ACF_SEC_TYPE_DSP_FW => aw_dev_parse_raw_dsp_fw(
            (cfg_hdr as *mut u8).add((*cfg_dde).data_offset as usize),
            (*cfg_dde).data_size,
            scene_prof_desc,
        ),
        ACF_SEC_TYPE_MULTIPLE_BIN => aw_dev_prof_parse_multi_bin(
            aw_dev,
            (cfg_hdr as *mut u8).add((*cfg_dde).data_offset as usize),
            (*cfg_dde).data_size,
            scene_prof_desc,
        ),
        ACF_SEC_TYPE_HDR_REG => aw_dev_parse_reg_bin_with_hdr(
            aw_dev,
            (cfg_hdr as *mut u8).add((*cfg_dde).data_offset as usize),
            (*cfg_dde).data_size,
            scene_prof_desc,
        ),
        _ => {
            dev_err!(
                (*aw_dev).dev,
                "%s cfg_dde->data_type = %d\n",
                "aw_dev_parse_data_by_sec_type",
                (*cfg_dde).data_type
            );
            0
        }
    }
}

unsafe fn aw_dev_parse_dev_type(
    aw_dev: *mut aw_device,
    prof_hdr: *mut aw_cfg_hdr,
    all_prof_info: *mut aw_all_prof_info,
) -> c_int {
    let cfg_dde = (prof_hdr as *mut u8).add((*prof_hdr).hdr_offset as usize) as *mut aw_cfg_dde;
    let mut sec_num: c_int = 0;
    let mut i: c_int = 0;

    while i < (*prof_hdr).ddt_num as c_int {
        let dde = cfg_dde.add(i as usize);
        if (*(*(*aw_dev).i2c).adapter).nr == (*dde).dev_bus
            && (*(*aw_dev).i2c).addr == (*dde).dev_addr
            && (*dde).r#type == AW88395_DEV_TYPE_ID
            && (*dde).data_type != ACF_SEC_TYPE_MONITOR
        {
            if (*dde).dev_profile >= AW88395_PROFILE_MAX {
                dev_err!((*aw_dev).dev, "dev_profile [%d] overflow", (*dde).dev_profile);
                return -EINVAL;
            }
            (*aw_dev).prof_data_type = (*dde).data_type;
            let ret = aw_dev_parse_data_by_sec_type(
                aw_dev,
                prof_hdr,
                dde,
                &mut (*all_prof_info).prof_desc[(*dde).dev_profile as usize],
            );
            if ret < 0 {
                dev_err!((*aw_dev).dev, "parse failed");
                return ret;
            }
            sec_num += 1;
        }
        i += 1;
    }

    if sec_num == 0 {
        dev_dbg!((*aw_dev).dev, "get dev type num is %d, please use default", sec_num);
        return AW88395_DEV_TYPE_NONE;
    }

    AW88395_DEV_TYPE_OK
}

unsafe fn aw_dev_parse_dev_default_type(
    aw_dev: *mut aw_device,
    prof_hdr: *mut aw_cfg_hdr,
    all_prof_info: *mut aw_all_prof_info,
) -> c_int {
    let cfg_dde = (prof_hdr as *mut u8).add((*prof_hdr).hdr_offset as usize) as *mut aw_cfg_dde;
    let mut sec_num: c_int = 0;
    let mut i: c_int = 0;

    while i < (*prof_hdr).ddt_num as c_int {
        let dde = cfg_dde.add(i as usize);
        if (*aw_dev).channel == (*dde).dev_index
            && (*dde).r#type == AW88395_DEV_DEFAULT_TYPE_ID
            && (*dde).data_type != ACF_SEC_TYPE_MONITOR
        {
            if (*dde).dev_profile >= AW88395_PROFILE_MAX {
                dev_err!((*aw_dev).dev, "dev_profile [%d] overflow", (*dde).dev_profile);
                return -EINVAL;
            }
            (*aw_dev).prof_data_type = (*dde).data_type;
            let ret = aw_dev_parse_data_by_sec_type(
                aw_dev,
                prof_hdr,
                dde,
                &mut (*all_prof_info).prof_desc[(*dde).dev_profile as usize],
            );
            if ret < 0 {
                dev_err!((*aw_dev).dev, "parse failed");
                return ret;
            }
            sec_num += 1;
        }
        i += 1;
    }

    if sec_num == 0 {
        dev_err!((*aw_dev).dev, "get dev default type failed, get num[%d]", sec_num);
        return -EINVAL;
    }

    0
}

unsafe fn aw_dev_cfg_get_reg_valid_prof(
    aw_dev: *mut aw_device,
    all_prof_info: *mut aw_all_prof_info,
) -> c_int {
    let prof_desc = (*all_prof_info).prof_desc.as_mut_ptr();
    let prof_info = &mut (*aw_dev).prof_info;
    let mut num: c_int = 0;

    let mut i: c_int = 0;
    while i < AW88395_PROFILE_MAX as c_int {
        if (*prof_desc.add(i as usize)).prof_st == AW88395_PROFILE_OK {
            prof_info.count += 1;
        }
        i += 1;
    }

    dev_dbg!((*aw_dev).dev, "get valid profile:%d", (*aw_dev).prof_info.count);

    if prof_info.count == 0 {
        dev_err!((*aw_dev).dev, "no profile data");
        return -EPERM;
    }

    prof_info.prof_desc = devm_kcalloc(
        (*aw_dev).dev,
        prof_info.count as usize,
        size_of::<aw_prof_desc>(),
        GFP_KERNEL,
    ) as *mut aw_prof_desc;
    if prof_info.prof_desc.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < AW88395_PROFILE_MAX as c_int {
        if (*prof_desc.add(i as usize)).prof_st == AW88395_PROFILE_OK {
            if num >= prof_info.count as c_int {
                dev_err!((*aw_dev).dev, "overflow count[%d]", prof_info.count);
                return -EINVAL;
            }
            *prof_info.prof_desc.add(num as usize) = *prof_desc.add(i as usize);
            (*prof_info.prof_desc.add(num as usize)).id = i;
            num += 1;
        }
        i += 1;
    }

    0
}

unsafe fn aw_dev_cfg_get_multiple_valid_prof(
    aw_dev: *mut aw_device,
    all_prof_info: *mut aw_all_prof_info,
) -> c_int {
    let prof_desc = (*all_prof_info).prof_desc.as_mut_ptr();
    let prof_info = &mut (*aw_dev).prof_info;
    let mut num: c_int = 0;

    let mut i: c_int = 0;
    while i < AW88395_PROFILE_MAX as c_int {
        if (*prof_desc.add(i as usize)).prof_st == AW88395_PROFILE_OK {
            let sec_desc = (*prof_desc.add(i as usize)).sec_desc.as_mut_ptr();
            if !(*sec_desc.add(AW88395_DATA_TYPE_REG as usize)).data.is_null()
                && (*sec_desc.add(AW88395_DATA_TYPE_REG as usize)).len != 0
                && !(*sec_desc.add(AW88395_DATA_TYPE_DSP_CFG as usize)).data.is_null()
                && (*sec_desc.add(AW88395_DATA_TYPE_DSP_CFG as usize)).len != 0
                && !(*sec_desc.add(AW88395_DATA_TYPE_DSP_FW as usize)).data.is_null()
                && (*sec_desc.add(AW88395_DATA_TYPE_DSP_FW as usize)).len != 0
            {
                prof_info.count += 1;
            }
        }
        i += 1;
    }

    dev_dbg!((*aw_dev).dev, "get valid profile:%d", (*aw_dev).prof_info.count);
    if prof_info.count == 0 {
        dev_err!((*aw_dev).dev, "no profile data");
        return -EPERM;
    }

    prof_info.prof_desc = devm_kcalloc(
        (*aw_dev).dev,
        prof_info.count as usize,
        size_of::<aw_prof_desc>(),
        GFP_KERNEL,
    ) as *mut aw_prof_desc;
    if prof_info.prof_desc.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < AW88395_PROFILE_MAX as c_int {
        if (*prof_desc.add(i as usize)).prof_st == AW88395_PROFILE_OK {
            let sec_desc = (*prof_desc.add(i as usize)).sec_desc.as_mut_ptr();
            if !(*sec_desc.add(AW88395_DATA_TYPE_REG as usize)).data.is_null()
                && (*sec_desc.add(AW88395_DATA_TYPE_REG as usize)).len != 0
                && !(*sec_desc.add(AW88395_DATA_TYPE_DSP_CFG as usize)).data.is_null()
                && (*sec_desc.add(AW88395_DATA_TYPE_DSP_CFG as usize)).len != 0
                && !(*sec_desc.add(AW88395_DATA_TYPE_DSP_FW as usize)).data.is_null()
                && (*sec_desc.add(AW88395_DATA_TYPE_DSP_FW as usize)).len != 0
            {
                if num >= prof_info.count as c_int {
                    dev_err!((*aw_dev).dev, "overflow count[%d]", prof_info.count);
                    return -EINVAL;
                }
                *prof_info.prof_desc.add(num as usize) = *prof_desc.add(i as usize);
                (*prof_info.prof_desc.add(num as usize)).id = i;
                num += 1;
            }
        }
        i += 1;
    }

    0
}

unsafe fn aw_dev_load_cfg_by_hdr(aw_dev: *mut aw_device, prof_hdr: *mut aw_cfg_hdr) -> c_int {
    let all_prof_info = kzalloc(size_of::<aw_all_prof_info>(), GFP_KERNEL) as *mut aw_all_prof_info;
    if all_prof_info.is_null() {
        return -ENOMEM;
    }

    let mut ret = aw_dev_parse_dev_type(aw_dev, prof_hdr, all_prof_info);
    if ret < 0 {
        kfree(all_prof_info as *mut c_void);
        return ret;
    } else if ret == AW88395_DEV_TYPE_NONE {
        dev_dbg!((*aw_dev).dev, "get dev type num is 0, parse default dev");
        ret = aw_dev_parse_dev_default_type(aw_dev, prof_hdr, all_prof_info);
        if ret < 0 {
            kfree(all_prof_info as *mut c_void);
            return ret;
        }
    }

    ret = match (*aw_dev).prof_data_type {
        ACF_SEC_TYPE_MULTIPLE_BIN => aw_dev_cfg_get_multiple_valid_prof(aw_dev, all_prof_info),
        ACF_SEC_TYPE_HDR_REG => aw_dev_cfg_get_reg_valid_prof(aw_dev, all_prof_info),
        _ => {
            dev_err!((*aw_dev).dev, "unsupported data type\n");
            -EINVAL
        }
    };
    if ret == 0 {
        (*aw_dev).prof_info.prof_name_list = profile_name.as_mut_ptr();
    }

    kfree(all_prof_info as *mut c_void);
    ret
}

unsafe fn aw_dev_create_prof_name_list_v1(aw_dev: *mut aw_device) -> c_int {
    let prof_info = &mut (*aw_dev).prof_info;
    let prof_desc = prof_info.prof_desc;
    let mut i: c_int = 0;

    if prof_desc.is_null() {
        dev_err!((*aw_dev).dev, "prof_desc is NULL");
        return -EINVAL;
    }

    prof_info.prof_name_list = devm_kzalloc(
        (*aw_dev).dev,
        prof_info.count as usize * PROFILE_STR_MAX as usize,
        GFP_KERNEL,
    ) as *mut *mut c_char;
    if prof_info.prof_name_list.is_null() {
        return -ENOMEM;
    }

    while i < prof_info.count as c_int {
        (*prof_desc.add(i as usize)).id = i;
        *prof_info.prof_name_list.add(i as usize) = (*prof_desc.add(i as usize)).prf_str;
        dev_dbg!((*aw_dev).dev, "prof name is %s", *prof_info.prof_name_list.add(i as usize));
        i += 1;
    }

    0
}

unsafe fn aw_get_dde_type_info(aw_dev: *mut aw_device, aw_cfg: *mut aw_container) -> c_int {
    let cfg_hdr = (*aw_cfg).data as *mut aw_cfg_hdr;
    let cfg_dde = ((*aw_cfg).data).add((*cfg_hdr).hdr_offset as usize) as *mut aw_cfg_dde_v1;
    let mut default_num: c_int = 0;
    let mut dev_num: c_int = 0;
    let mut i: c_uint = 0;

    while i < (*cfg_hdr).ddt_num {
        if (*cfg_dde.add(i as usize)).r#type == AW88395_DEV_TYPE_ID {
            dev_num += 1;
        }
        if (*cfg_dde.add(i as usize)).r#type == AW88395_DEV_DEFAULT_TYPE_ID {
            default_num += 1;
        }
        i += 1;
    }

    if dev_num != 0 {
        (*aw_dev).prof_info.prof_type = AW88395_DEV_TYPE_ID;
    } else if default_num != 0 {
        (*aw_dev).prof_info.prof_type = AW88395_DEV_DEFAULT_TYPE_ID;
    } else {
        dev_err!((*aw_dev).dev, "can't find scene");
        return -EINVAL;
    }

    0
}

unsafe fn aw_get_dev_scene_count_v1(
    aw_dev: *mut aw_device,
    aw_cfg: *mut aw_container,
    scene_num: *mut c_uint,
) -> c_int {
    let cfg_hdr = (*aw_cfg).data as *mut aw_cfg_hdr;
    let cfg_dde = ((*aw_cfg).data).add((*cfg_hdr).hdr_offset as usize) as *mut aw_cfg_dde_v1;
    let mut i: c_uint = 0;

    while i < (*cfg_hdr).ddt_num {
        let dde = cfg_dde.add(i as usize);
        if ((*dde).data_type == ACF_SEC_TYPE_REG
            || (*dde).data_type == ACF_SEC_TYPE_HDR_REG
            || (*dde).data_type == ACF_SEC_TYPE_MULTIPLE_BIN)
            && (*aw_dev).chip_id == (*dde).chip_id
            && (*(*(*aw_dev).i2c).adapter).nr == (*dde).dev_bus
            && (*(*aw_dev).i2c).addr == (*dde).dev_addr
        {
            *scene_num += 1;
        }
        i += 1;
    }

    if *scene_num == 0 {
        dev_err!((*aw_dev).dev, "failed to obtain scene, scenu_num = %d\n", *scene_num);
        return -EINVAL;
    }

    0
}

unsafe fn aw_get_default_scene_count_v1(
    aw_dev: *mut aw_device,
    aw_cfg: *mut aw_container,
    scene_num: *mut c_uint,
) -> c_int {
    let cfg_hdr = (*aw_cfg).data as *mut aw_cfg_hdr;
    let cfg_dde = ((*aw_cfg).data).add((*cfg_hdr).hdr_offset as usize) as *mut aw_cfg_dde_v1;
    let mut i: c_uint = 0;

    while i < (*cfg_hdr).ddt_num {
        let dde = cfg_dde.add(i as usize);
        if ((*dde).data_type == ACF_SEC_TYPE_MULTIPLE_BIN
            || (*dde).data_type == ACF_SEC_TYPE_REG
            || (*dde).data_type == ACF_SEC_TYPE_HDR_REG)
            && (*aw_dev).chip_id == (*dde).chip_id
            && (*aw_dev).channel == (*dde).dev_index
        {
            *scene_num += 1;
        }
        i += 1;
    }

    if *scene_num == 0 {
        dev_err!((*aw_dev).dev, "failed to obtain scene, scenu_num = %d\n", *scene_num);
        return -EINVAL;
    }

    0
}

unsafe fn aw_dev_parse_scene_count_v1(
    aw_dev: *mut aw_device,
    aw_cfg: *mut aw_container,
    count: *mut c_uint,
) -> c_int {
    let mut ret = aw_get_dde_type_info(aw_dev, aw_cfg);
    if ret < 0 {
        return ret;
    }

    ret = match (*aw_dev).prof_info.prof_type {
        AW88395_DEV_TYPE_ID => aw_get_dev_scene_count_v1(aw_dev, aw_cfg, count),
        AW88395_DEV_DEFAULT_TYPE_ID => aw_get_default_scene_count_v1(aw_dev, aw_cfg, count),
        _ => {
            dev_err!(
                (*aw_dev).dev,
                "unsupported prof_type[%x]",
                (*aw_dev).prof_info.prof_type
            );
            -EINVAL
        }
    };

    ret
}

unsafe fn aw_dev_parse_data_by_sec_type_v1(
    aw_dev: *mut aw_device,
    prof_hdr: *mut aw_cfg_hdr,
    cfg_dde: *mut aw_cfg_dde_v1,
    cur_scene_id: *mut c_int,
) -> c_int {
    let prof_info = &mut (*aw_dev).prof_info;
    let ret: c_int;

    match (*cfg_dde).data_type {
        ACF_SEC_TYPE_MULTIPLE_BIN => {
            ret = aw_dev_prof_parse_multi_bin(
                aw_dev,
                (prof_hdr as *mut u8).add((*cfg_dde).data_offset as usize),
                (*cfg_dde).data_size,
                prof_info.prof_desc.add(*cur_scene_id as usize),
            );
            if ret < 0 {
                dev_err!((*aw_dev).dev, "parse multi bin failed");
                return ret;
            }
            (*prof_info.prof_desc.add(*cur_scene_id as usize)).prf_str = (*cfg_dde).dev_profile_str;
            (*prof_info.prof_desc.add(*cur_scene_id as usize)).id = (*cfg_dde).dev_profile;
            *cur_scene_id += 1;
        }
        ACF_SEC_TYPE_HDR_REG => {
            ret = aw_dev_parse_reg_bin_with_hdr(
                aw_dev,
                (prof_hdr as *mut u8).add((*cfg_dde).data_offset as usize),
                (*cfg_dde).data_size,
                prof_info.prof_desc.add(*cur_scene_id as usize),
            );
            if ret < 0 {
                dev_err!((*aw_dev).dev, "parse reg bin with hdr failed");
                return ret;
            }
            (*prof_info.prof_desc.add(*cur_scene_id as usize)).prf_str = (*cfg_dde).dev_profile_str;
            (*prof_info.prof_desc.add(*cur_scene_id as usize)).id = (*cfg_dde).dev_profile;
            *cur_scene_id += 1;
        }
        _ => {
            dev_err!((*aw_dev).dev, "unsupported SEC_TYPE [%d]", (*cfg_dde).data_type);
            return -EINVAL;
        }
    }

    0
}

unsafe fn aw_dev_parse_dev_type_v1(aw_dev: *mut aw_device, prof_hdr: *mut aw_cfg_hdr) -> c_int {
    let cfg_dde = (prof_hdr as *mut u8).add((*prof_hdr).hdr_offset as usize) as *mut aw_cfg_dde_v1;
    let mut cur_scene_id: c_int = 0;
    let mut i: c_uint = 0;

    while i < (*prof_hdr).ddt_num {
        let dde = cfg_dde.add(i as usize);
        if (*(*(*aw_dev).i2c).adapter).nr == (*dde).dev_bus
            && (*(*aw_dev).i2c).addr == (*dde).dev_addr
            && (*aw_dev).chip_id == (*dde).chip_id
        {
            let ret = aw_dev_parse_data_by_sec_type_v1(aw_dev, prof_hdr, dde, &mut cur_scene_id);
            if ret < 0 {
                dev_err!((*aw_dev).dev, "parse failed");
                return ret;
            }
        }
        i += 1;
    }

    if cur_scene_id == 0 {
        dev_err!((*aw_dev).dev, "get dev type failed, get num [%d]", cur_scene_id);
        return -EINVAL;
    }

    0
}

unsafe fn aw_dev_parse_default_type_v1(
    aw_dev: *mut aw_device,
    prof_hdr: *mut aw_cfg_hdr,
) -> c_int {
    let cfg_dde = (prof_hdr as *mut u8).add((*prof_hdr).hdr_offset as usize) as *mut aw_cfg_dde_v1;
    let mut cur_scene_id: c_int = 0;
    let mut i: c_uint = 0;

    while i < (*prof_hdr).ddt_num {
        let dde = cfg_dde.add(i as usize);
        if (*aw_dev).channel == (*dde).dev_index && (*aw_dev).chip_id == (*dde).chip_id {
            let ret = aw_dev_parse_data_by_sec_type_v1(aw_dev, prof_hdr, dde, &mut cur_scene_id);
            if ret < 0 {
                dev_err!((*aw_dev).dev, "parse failed");
                return ret;
            }
        }
        i += 1;
    }

    if cur_scene_id == 0 {
        dev_err!(
            (*aw_dev).dev,
            "get dev default type failed, get num[%d]",
            cur_scene_id
        );
        return -EINVAL;
    }

    0
}

unsafe fn aw_dev_parse_by_hdr_v1(aw_dev: *mut aw_device, cfg_hdr: *mut aw_cfg_hdr) -> c_int {
    match (*aw_dev).prof_info.prof_type {
        AW88395_DEV_TYPE_ID => aw_dev_parse_dev_type_v1(aw_dev, cfg_hdr),
        AW88395_DEV_DEFAULT_TYPE_ID => aw_dev_parse_default_type_v1(aw_dev, cfg_hdr),
        _ => {
            dev_err!(
                (*aw_dev).dev,
                "prof type matched failed, get num[%d]",
                (*aw_dev).prof_info.prof_type
            );
            -EINVAL
        }
    }
}

unsafe fn aw_dev_load_cfg_by_hdr_v1(
    aw_dev: *mut aw_device,
    aw_cfg: *mut aw_container,
) -> c_int {
    let cfg_hdr = (*aw_cfg).data as *mut aw_cfg_hdr;
    let prof_info = &mut (*aw_dev).prof_info;

    let mut ret = aw_dev_parse_scene_count_v1(aw_dev, aw_cfg, &mut prof_info.count);
    if ret < 0 {
        dev_err!((*aw_dev).dev, "get scene count failed");
        return ret;
    }

    prof_info.prof_desc = devm_kcalloc(
        (*aw_dev).dev,
        prof_info.count as usize,
        size_of::<aw_prof_desc>(),
        GFP_KERNEL,
    ) as *mut aw_prof_desc;
    if prof_info.prof_desc.is_null() {
        return -ENOMEM;
    }

    ret = aw_dev_parse_by_hdr_v1(aw_dev, cfg_hdr);
    if ret < 0 {
        dev_err!((*aw_dev).dev, "parse hdr failed");
        return ret;
    }

    ret = aw_dev_create_prof_name_list_v1(aw_dev);
    if ret < 0 {
        dev_err!((*aw_dev).dev, "create prof name list failed");
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn aw88395_dev_cfg_load(
    aw_dev: *mut aw_device,
    aw_cfg: *mut aw_container,
) -> c_int {
    let cfg_hdr = (*aw_cfg).data as *mut aw_cfg_hdr;

    match (*cfg_hdr).hdr_version {
        AW88395_CFG_HDR_VER => {
            let ret = aw_dev_load_cfg_by_hdr(aw_dev, cfg_hdr);
            if ret < 0 {
                dev_err!(
                    (*aw_dev).dev,
                    "hdr_version[0x%x] parse failed",
                    (*cfg_hdr).hdr_version
                );
                return ret;
            }
        }
        AW88395_CFG_HDR_VER_V1 => {
            let ret = aw_dev_load_cfg_by_hdr_v1(aw_dev, aw_cfg);
            if ret < 0 {
                dev_err!(
                    (*aw_dev).dev,
                    "hdr_version[0x%x] parse failed",
                    (*cfg_hdr).hdr_version
                );
                return ret;
            }
        }
        _ => {
            dev_err!(
                (*aw_dev).dev,
                "unsupported hdr_version [0x%x]",
                (*cfg_hdr).hdr_version
            );
            return -EINVAL;
        }
    }
    (*aw_dev).fw_status = AW88395_DEV_FW_OK;

    0
}

unsafe fn aw_dev_check_cfg_by_hdr(
    aw_dev: *mut aw_device,
    aw_cfg: *mut aw_container,
) -> c_int {
    let cfg_hdr = (*aw_cfg).data as *mut aw_cfg_hdr;
    let mut act_data: c_uint = 0;

    /* check file type id is awinic acf file */
    if (*cfg_hdr).id != ACF_FILE_ID {
        dev_err!((*aw_dev).dev, "not acf type file");
        return -EINVAL;
    }

    let hdr_ddt_len = (*cfg_hdr).hdr_offset + (*cfg_hdr).ddt_size;
    if hdr_ddt_len > (*aw_cfg).len {
        dev_err!(
            (*aw_dev).dev,
            "hdr_len with ddt_len [%d] overflow file size[%d]",
            (*cfg_hdr).hdr_offset,
            (*aw_cfg).len
        );
        return -EINVAL;
    }

    /* check data size */
    let cfg_dde = ((*aw_cfg).data).add((*cfg_hdr).hdr_offset as usize) as *mut aw_cfg_dde;
    act_data += hdr_ddt_len;
    let mut i: c_uint = 0;
    while i < (*cfg_hdr).ddt_num {
        act_data += (*cfg_dde.add(i as usize)).data_size;
        i += 1;
    }

    if act_data != (*aw_cfg).len {
        dev_err!(
            (*aw_dev).dev,
            "act_data[%d] not equal to file size[%d]!",
            act_data,
            (*aw_cfg).len
        );
        return -EINVAL;
    }

    i = 0;
    while i < (*cfg_hdr).ddt_num {
        /* data check */
        let end_data_offset = (*cfg_dde.add(i as usize)).data_offset + (*cfg_dde.add(i as usize)).data_size;
        if end_data_offset > (*aw_cfg).len {
            dev_err!(
                (*aw_dev).dev,
                "ddt_num[%d] end_data_offset[%d] overflow size[%d]",
                i,
                end_data_offset,
                (*aw_cfg).len
            );
            return -EINVAL;
        }

        /* crc check */
        let act_crc8 = crc8(
            aw_crc8_table.as_ptr(),
            ((*aw_cfg).data).add((*cfg_dde.add(i as usize)).data_offset as usize),
            (*cfg_dde.add(i as usize)).data_size,
            0,
        );
        if act_crc8 != (*cfg_dde.add(i as usize)).data_crc {
            dev_err!(
                (*aw_dev).dev,
                "ddt_num[%d] act_crc8:0x%x != data_crc:0x%x",
                i,
                act_crc8 as u32,
                (*cfg_dde.add(i as usize)).data_crc
            );
            return -EINVAL;
        }
        i += 1;
    }

    0
}

unsafe fn aw_dev_check_acf_by_hdr_v1(
    aw_dev: *mut aw_device,
    aw_cfg: *mut aw_container,
) -> c_int {
    let cfg_hdr = (*aw_cfg).data as *mut aw_cfg_hdr;
    let mut act_data: c_uint = 0;

    /* check file type id is awinic acf file */
    if (*cfg_hdr).id != ACF_FILE_ID {
        dev_err!((*aw_dev).dev, "not acf type file");
        return -EINVAL;
    }

    let hdr_ddt_len = (*cfg_hdr).hdr_offset + (*cfg_hdr).ddt_size;
    if hdr_ddt_len > (*aw_cfg).len {
        dev_err!(
            (*aw_dev).dev,
            "hdrlen with ddt_len [%d] overflow file size[%d]",
            (*cfg_hdr).hdr_offset,
            (*aw_cfg).len
        );
        return -EINVAL;
    }

    /* check data size */
    let cfg_dde = ((*aw_cfg).data).add((*cfg_hdr).hdr_offset as usize) as *mut aw_cfg_dde_v1;
    act_data += hdr_ddt_len;
    let mut i: c_int = 0;
    while i < (*cfg_hdr).ddt_num as c_int {
        act_data += (*cfg_dde.add(i as usize)).data_size;
        i += 1;
    }

    if act_data != (*aw_cfg).len {
        dev_err!(
            (*aw_dev).dev,
            "act_data[%d] not equal to file size[%d]!",
            act_data,
            (*aw_cfg).len
        );
        return -EINVAL;
    }

    i = 0;
    while i < (*cfg_hdr).ddt_num as c_int {
        /* data check */
        let end_data_offset = (*cfg_dde.add(i as usize)).data_offset + (*cfg_dde.add(i as usize)).data_size;
        if end_data_offset > (*aw_cfg).len {
            dev_err!(
                (*aw_dev).dev,
                "ddt_num[%d] end_data_offset[%d] overflow size[%d]",
                i,
                end_data_offset,
                (*aw_cfg).len
            );
            return -EINVAL;
        }

        /* crc check */
        let act_crc8 = crc8(
            aw_crc8_table.as_ptr(),
            ((*aw_cfg).data).add((*cfg_dde.add(i as usize)).data_offset as usize),
            (*cfg_dde.add(i as usize)).data_size,
            0,
        );
        if act_crc8 != (*cfg_dde.add(i as usize)).data_crc {
            dev_err!(
                (*aw_dev).dev,
                "ddt_num[%d] act_crc8:0x%x != data_crc 0x%x",
                i,
                act_crc8 as u32,
                (*cfg_dde.add(i as usize)).data_crc
            );
            return -EINVAL;
        }
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn aw88395_dev_load_acf_check(
    aw_dev: *mut aw_device,
    aw_cfg: *mut aw_container,
) -> c_int {
    if aw_cfg.is_null() {
        dev_err!((*aw_dev).dev, "aw_prof is NULL");
        return -EINVAL;
    }

    if (*aw_cfg).len < size_of::<aw_cfg_hdr>() as c_uint {
        dev_err!(
            (*aw_dev).dev,
            "cfg hdr size[%d] overflow file size[%d]",
            (*aw_cfg).len,
            size_of::<aw_cfg_hdr>() as c_int
        );
        return -EINVAL;
    }

    crc8_populate_lsb(aw_crc8_table.as_mut_ptr(), AW88395_CRC8_POLYNOMIAL);

    let cfg_hdr = (*aw_cfg).data as *mut aw_cfg_hdr;
    match (*cfg_hdr).hdr_version {
        AW88395_CFG_HDR_VER => aw_dev_check_cfg_by_hdr(aw_dev, aw_cfg),
        AW88395_CFG_HDR_VER_V1 => aw_dev_check_acf_by_hdr_v1(aw_dev, aw_cfg),
        _ => {
            dev_err!(
                (*aw_dev).dev,
                "unsupported hdr_version [0x%x]",
                (*cfg_hdr).hdr_version
            );
            -EINVAL
        }
    }
}

// MODULE_DESCRIPTION("AW88395 ACF File Parsing Lib");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
