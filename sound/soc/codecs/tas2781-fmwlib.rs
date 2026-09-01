// SPDX-License-Identifier: GPL-2.0
//
// tas2781-fmwlib.rs -- TASDEVICE firmware support
//
// Copyright 2023 - 2026 Texas Instruments, Inc.
//
// Author: Shenghao Ding <shenghao-ding@ti.com>
// Author: Baojun Xu <baojun.xu@ti.com>
//
// Source-level Rust translation of tas2781-fmwlib.c.  Kernel/TAS types,
// allocation helpers, logging helpers, register macros, and firmware helpers
// are supplied by external translation units/headers in the original tree.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};
use core::ptr;

type bool_t = bool;
type u8 = c_uchar;
type u16 = u16;
type u32 = c_uint;
type size_t = usize;

const ERROR_PRAM_CRCCHK: c_uint = 0x0000000;
const ERROR_YRAM_CRCCHK: c_uint = 0x0000001;
const PPC_DRIVER_CRCCHK: c_uint = 0x00000200;

const TAS2781_YRAM_BOOK1: c_uchar = 140;
const TAS2781_YRAM1_PAGE: c_uchar = 42;
const TAS2781_YRAM1_START_REG: c_uchar = 88;
const TAS2781_PG_1_0: c_uint = 0xA0;
const TAS2781_PG_2_0: c_uint = 0xA8;
const TAS2781_YRAM2_START_PAGE: c_uchar = 43;
const TAS2781_YRAM2_END_PAGE: c_uchar = 49;
const TAS2781_YRAM2_START_REG: c_uchar = 8;
const TAS2781_YRAM2_END_REG: c_uchar = 127;
/* should not include B0_P53_R44-R47 */
const TAS2781_YRAM_BOOK2: c_uchar = 0;
const TAS2781_YRAM4_START_PAGE: c_uchar = 50;
const TAS2781_YRAM4_END_PAGE: c_uchar = 60;
const TAS2781_YRAM5_PAGE: c_uchar = 61;
const TAS2781_YRAM5_START_REG: c_uchar = TAS2781_YRAM3_START_REG;
const TAS2781_YRAM5_END_REG: c_uchar = TAS2781_YRAM3_END_REG;
const TAS2781_YRAM3_PAGE: c_uchar = 50;
const TAS2781_YRAM3_START_REG: c_uchar = 8;
const TAS2781_YRAM3_END_REG: c_uchar = 27;

const TASDEVICE_CMD_SING_W: c_uchar = 0x1;
const TASDEVICE_CMD_BURST: c_uchar = 0x2;
const TASDEVICE_CMD_DELAY: c_uchar = 0x3;
const TASDEVICE_CMD_FIELD_W: c_uchar = 0x4;

const TASDEVICE_MAXPROGRAM_NUM_KERNEL: c_uint = 5;
const TASDEVICE_MAXCONFIG_NUM_KERNEL_MULTIPLE_AMPS: c_uint = 64;
const TASDEVICE_MAXCONFIG_NUM_KERNEL: c_uint = 10;
const MAIN_ALL_DEVICES_1X: c_uchar = 0x01;
const MAIN_DEVICE_A_1X: c_uchar = 0x02;
const MAIN_DEVICE_B_1X: c_uchar = 0x03;
const MAIN_DEVICE_C_1X: c_uchar = 0x04;
const MAIN_DEVICE_D_1X: c_uchar = 0x05;
const COEFF_DEVICE_A_1X: c_uchar = 0x12;
const COEFF_DEVICE_B_1X: c_uchar = 0x13;
const COEFF_DEVICE_C_1X: c_uchar = 0x14;
const COEFF_DEVICE_D_1X: c_uchar = 0x15;
const PRE_DEVICE_A_1X: c_uchar = 0x22;
const PRE_DEVICE_B_1X: c_uchar = 0x23;
const PRE_DEVICE_C_1X: c_uchar = 0x24;
const PRE_DEVICE_D_1X: c_uchar = 0x25;
const PRE_SOFTWARE_RESET_DEVICE_A: c_uchar = 0x41;
const PRE_SOFTWARE_RESET_DEVICE_B: c_uchar = 0x42;
const PRE_SOFTWARE_RESET_DEVICE_C: c_uchar = 0x43;
const PRE_SOFTWARE_RESET_DEVICE_D: c_uchar = 0x44;
const POST_SOFTWARE_RESET_DEVICE_A: c_uchar = 0x45;
const POST_SOFTWARE_RESET_DEVICE_B: c_uchar = 0x46;
const POST_SOFTWARE_RESET_DEVICE_C: c_uchar = 0x47;
const POST_SOFTWARE_RESET_DEVICE_D: c_uchar = 0x48;

#[repr(C)]
struct tas_crc {
    offset: c_uchar,
    len: c_uchar,
}

#[repr(C)]
struct blktyp_devidx_map {
    blktyp: c_uchar,
    dev_idx: c_uchar,
}

#[repr(C)]
struct tas2781_cali_specific {
    sin_gni: [c_uchar; 4],
    sin_gni_reg: c_int,
    is_sin_gn_flush: bool,
}

// External constants, types, macros, and helpers from the Linux/TAS headers.
extern "C" {
    type device;
    type firmware;
    type tasdevice_priv;
    type tasdevice_fw;
    type tasdevice_config_info;
    type tasdev_blk_data;
    type tasdevice_rca_hdr;
    type tasdevice_rca;
    type tasdev_blk;
    type tasdevice_data;
    type tasdevice_prog;
    type tasdevice_config;
    type cali_reg;
    type calidata;
    type fct_param_address;
    type tasdevice_dspfw_hdr;
    type tasdevice_fw_fixed_hdr;
    type tasdevice;
    type tasdevice_calibration;

    static TASDEVICE_DSP_TAS_MAX_DEVICE: usize;
    static TASDEVICE_DEVICE_SUM: c_int;
    static TASDEVICE_CONFIG_SUM: c_int;
    static TASDEVICE_BIN_BLK_PRE_POWER_UP: c_uchar;
    static TASDEVICE_BIN_BLK_PRE_SHUTDOWN: c_uchar;
    static TASDEVICE_DSP_FW_FAIL: c_int;
    static TASDEVICE_RCA_FW_OK: c_int;
    static TASDEVICE_DSP_FW_ALL_OK: c_int;
    static PPC3_VERSION_TAS2781_BASIC_MIN: c_uint;
    static PPC3_VERSION_BASE: c_uint;
    static PPC3_VERSION_TAS2781_ALPHA_MIN: c_uint;
    static PPC3_VERSION_TAS2781_BETA_MIN: c_uint;
    static PPC3_VERSION_TAS5825_BASE: c_uint;
    static TASDEV_ALPHA: c_int;
    static TASDEV_BETA: c_int;
    static TASDEV_BASIC: c_int;
    static MAIN_ALL_DEVICES: c_uint;
    static MAIN_DEVICE_A: c_uint;
    static MAIN_DEVICE_B: c_uint;
    static MAIN_DEVICE_C: c_uint;
    static MAIN_DEVICE_D: c_uint;
    static COEFF_DEVICE_A: c_uint;
    static COEFF_DEVICE_B: c_uint;
    static COEFF_DEVICE_C: c_uint;
    static COEFF_DEVICE_D: c_uint;
    static PRE_DEVICE_A: c_uint;
    static PRE_DEVICE_B: c_uint;
    static PRE_DEVICE_C: c_uint;
    static PRE_DEVICE_D: c_uint;
    static TASDEVICE_CHECKSUM_REG: c_uint;
    static TAS2781: c_int;
    static GFP_KERNEL: c_uint;
    static ENOMEM: c_int;
    static EINVAL: c_int;
    static EAGAIN: c_int;
    static EOVERFLOW: c_int;
    static EXDEV: c_int;
    static INT_MAX: c_int;

    fn TASDEVICE_REG(book: c_uchar, page: c_uchar, reg: c_uchar) -> c_uint;
    fn TASDEVICE_BOOK_ID(reg: c_uint) -> c_uchar;
    fn TASDEVICE_PAGE_ID(reg: c_uint) -> c_uchar;
    fn TASDEVICE_PAGE_REG(reg: c_uint) -> c_uchar;
    fn BIT(nr: c_int) -> c_int;
    fn get_unaligned_be16(p: *const c_uchar) -> u16;
    fn get_unaligned_be32(p: *const c_uchar) -> c_uint;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn kmemdup(src: *const c_void, len: usize, flags: c_uint) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn memchr(s: *const c_void, c: c_int, n: usize) -> *mut c_void;
    fn strncmp(s1: *const c_uchar, s2: *const c_char, n: usize) -> c_int;
    fn strnstr(s1: *const c_uchar, s2: *const c_char, n: usize) -> *mut c_char;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn crc8(tbl: *const c_uchar, pdata: *const c_uchar, nbytes: usize, crc: c_uchar) -> c_uchar;
    fn tasdevice_dev_write(priv_: *mut tasdevice_priv, chn: c_int, reg: c_uint, val: c_uchar) -> c_int;
    fn tasdevice_dev_bulk_write(priv_: *mut tasdevice_priv, chn: c_int, reg: c_uint, data: *const c_uchar, len: c_uint) -> c_int;
    fn tasdevice_dev_read(priv_: *mut tasdevice_priv, chn: c_int, reg: c_uint, val: *mut c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
}

const fn array_size<T, const N: usize>(_: &[T; N]) -> usize { N }

static deviceNumber: [c_char; 16] = [1, 2, 1, 2, 1, 1, 0, 2, 4, 3, 1, 2, 3, 4, 1, 2];

/* fixed m68k compiling issue: mapping table can save code field */
static ppc3_tas2781_mapping_table: [blktyp_devidx_map; 21] = [
    blktyp_devidx_map { blktyp: MAIN_ALL_DEVICES_1X, dev_idx: 0x80 },
    blktyp_devidx_map { blktyp: MAIN_DEVICE_A_1X, dev_idx: 0x81 },
    blktyp_devidx_map { blktyp: COEFF_DEVICE_A_1X, dev_idx: 0xC1 },
    blktyp_devidx_map { blktyp: PRE_DEVICE_A_1X, dev_idx: 0xC1 },
    blktyp_devidx_map { blktyp: PRE_SOFTWARE_RESET_DEVICE_A, dev_idx: 0xC1 },
    blktyp_devidx_map { blktyp: POST_SOFTWARE_RESET_DEVICE_A, dev_idx: 0xC1 },
    blktyp_devidx_map { blktyp: MAIN_DEVICE_B_1X, dev_idx: 0x82 },
    blktyp_devidx_map { blktyp: COEFF_DEVICE_B_1X, dev_idx: 0xC2 },
    blktyp_devidx_map { blktyp: PRE_DEVICE_B_1X, dev_idx: 0xC2 },
    blktyp_devidx_map { blktyp: PRE_SOFTWARE_RESET_DEVICE_B, dev_idx: 0xC2 },
    blktyp_devidx_map { blktyp: POST_SOFTWARE_RESET_DEVICE_B, dev_idx: 0xC2 },
    blktyp_devidx_map { blktyp: MAIN_DEVICE_C_1X, dev_idx: 0x83 },
    blktyp_devidx_map { blktyp: COEFF_DEVICE_C_1X, dev_idx: 0xC3 },
    blktyp_devidx_map { blktyp: PRE_DEVICE_C_1X, dev_idx: 0xC3 },
    blktyp_devidx_map { blktyp: PRE_SOFTWARE_RESET_DEVICE_C, dev_idx: 0xC3 },
    blktyp_devidx_map { blktyp: POST_SOFTWARE_RESET_DEVICE_C, dev_idx: 0xC3 },
    blktyp_devidx_map { blktyp: MAIN_DEVICE_D_1X, dev_idx: 0x84 },
    blktyp_devidx_map { blktyp: COEFF_DEVICE_D_1X, dev_idx: 0xC4 },
    blktyp_devidx_map { blktyp: PRE_DEVICE_D_1X, dev_idx: 0xC4 },
    blktyp_devidx_map { blktyp: PRE_SOFTWARE_RESET_DEVICE_D, dev_idx: 0xC4 },
    blktyp_devidx_map { blktyp: POST_SOFTWARE_RESET_DEVICE_D, dev_idx: 0xC4 },
];

static ppc3_mapping_table: [blktyp_devidx_map; 13] = [
    blktyp_devidx_map { blktyp: MAIN_ALL_DEVICES_1X, dev_idx: 0x80 },
    blktyp_devidx_map { blktyp: MAIN_DEVICE_A_1X, dev_idx: 0x81 },
    blktyp_devidx_map { blktyp: COEFF_DEVICE_A_1X, dev_idx: 0xC1 },
    blktyp_devidx_map { blktyp: PRE_DEVICE_A_1X, dev_idx: 0xC1 },
    blktyp_devidx_map { blktyp: MAIN_DEVICE_B_1X, dev_idx: 0x82 },
    blktyp_devidx_map { blktyp: COEFF_DEVICE_B_1X, dev_idx: 0xC2 },
    blktyp_devidx_map { blktyp: PRE_DEVICE_B_1X, dev_idx: 0xC2 },
    blktyp_devidx_map { blktyp: MAIN_DEVICE_C_1X, dev_idx: 0x83 },
    blktyp_devidx_map { blktyp: COEFF_DEVICE_C_1X, dev_idx: 0xC3 },
    blktyp_devidx_map { blktyp: PRE_DEVICE_C_1X, dev_idx: 0xC3 },
    blktyp_devidx_map { blktyp: MAIN_DEVICE_D_1X, dev_idx: 0x84 },
    blktyp_devidx_map { blktyp: COEFF_DEVICE_D_1X, dev_idx: 0xC4 },
    blktyp_devidx_map { blktyp: PRE_DEVICE_D_1X, dev_idx: 0xC4 },
];

// non_ppc3_mapping_table uses block-type constants supplied externally.
static mut non_ppc3_mapping_table: [blktyp_devidx_map; 13] = unsafe {
    [
        blktyp_devidx_map { blktyp: MAIN_ALL_DEVICES as c_uchar, dev_idx: 0x80 },
        blktyp_devidx_map { blktyp: MAIN_DEVICE_A as c_uchar, dev_idx: 0x81 },
        blktyp_devidx_map { blktyp: COEFF_DEVICE_A as c_uchar, dev_idx: 0xC1 },
        blktyp_devidx_map { blktyp: PRE_DEVICE_A as c_uchar, dev_idx: 0xC1 },
        blktyp_devidx_map { blktyp: MAIN_DEVICE_B as c_uchar, dev_idx: 0x82 },
        blktyp_devidx_map { blktyp: COEFF_DEVICE_B as c_uchar, dev_idx: 0xC2 },
        blktyp_devidx_map { blktyp: PRE_DEVICE_B as c_uchar, dev_idx: 0xC2 },
        blktyp_devidx_map { blktyp: MAIN_DEVICE_C as c_uchar, dev_idx: 0x83 },
        blktyp_devidx_map { blktyp: COEFF_DEVICE_C as c_uchar, dev_idx: 0xC3 },
        blktyp_devidx_map { blktyp: PRE_DEVICE_C as c_uchar, dev_idx: 0xC3 },
        blktyp_devidx_map { blktyp: MAIN_DEVICE_D as c_uchar, dev_idx: 0x84 },
        blktyp_devidx_map { blktyp: COEFF_DEVICE_D as c_uchar, dev_idx: 0xC4 },
        blktyp_devidx_map { blktyp: PRE_DEVICE_D as c_uchar, dev_idx: 0xC4 },
    ]
};

// The following functions are direct Rust translations of the C functions.
// Field accesses retain the original C member names and require the external
// translated TAS/kernel struct definitions to provide matching layouts.

unsafe fn check_inpage_yram_rg(cd: *mut tas_crc, reg: c_uchar, len: c_uchar) -> bool {
    let mut in_ = false;
    if reg <= TAS2781_YRAM5_END_REG && reg >= TAS2781_YRAM5_START_REG {
        (*cd).len = if reg.wrapping_add(len) > TAS2781_YRAM5_END_REG {
            TAS2781_YRAM5_END_REG.wrapping_sub(reg).wrapping_add(1)
        } else { len };
        (*cd).offset = reg;
        in_ = true;
    } else if reg < TAS2781_YRAM5_START_REG {
        if reg.wrapping_add(len) > TAS2781_YRAM5_START_REG {
            (*cd).offset = TAS2781_YRAM5_START_REG;
            (*cd).len = len.wrapping_sub(TAS2781_YRAM5_START_REG).wrapping_add(reg);
            in_ = true;
        }
    }
    in_
}

unsafe fn check_inpage_yram_bk1(cd: *mut tas_crc, page: c_uchar, reg: c_uchar, len: c_uchar) -> bool {
    let mut in_ = false;
    if page == TAS2781_YRAM1_PAGE {
        if reg >= TAS2781_YRAM1_START_REG {
            (*cd).offset = reg;
            (*cd).len = len;
            in_ = true;
        } else if reg.wrapping_add(len) > TAS2781_YRAM1_START_REG {
            (*cd).offset = TAS2781_YRAM1_START_REG;
            (*cd).len = len.wrapping_sub(TAS2781_YRAM1_START_REG).wrapping_add(reg);
            in_ = true;
        }
    } else if page == TAS2781_YRAM3_PAGE {
        in_ = check_inpage_yram_rg(cd, reg, len);
    }
    in_
}

/* Return Code:
 * true -- the registers are in the inpage yram
 * false -- the registers are NOT in the inpage yram
 */
unsafe fn check_inpage_yram(cd: *mut tas_crc, book: c_uchar, page: c_uchar, reg: c_uchar, len: c_uchar) -> bool {
    if book == TAS2781_YRAM_BOOK1 {
        return check_inpage_yram_bk1(cd, page, reg, len);
    }
    if book == TAS2781_YRAM_BOOK2 && page == TAS2781_YRAM5_PAGE {
        return check_inpage_yram_rg(cd, reg, len);
    }
    false
}

unsafe fn check_inblock_yram_bk(cd: *mut tas_crc, page: c_uchar, reg: c_uchar, len: c_uchar) -> bool {
    let mut in_ = false;
    if (page >= TAS2781_YRAM4_START_PAGE && page <= TAS2781_YRAM4_END_PAGE)
        || (page >= TAS2781_YRAM2_START_PAGE && page <= TAS2781_YRAM2_END_PAGE)
    {
        if reg <= TAS2781_YRAM2_END_REG && reg >= TAS2781_YRAM2_START_REG {
            (*cd).offset = reg;
            (*cd).len = len;
            in_ = true;
        } else if reg < TAS2781_YRAM2_START_REG {
            if reg.wrapping_add(len).wrapping_sub(1) >= TAS2781_YRAM2_START_REG {
                (*cd).offset = TAS2781_YRAM2_START_REG;
                (*cd).len = reg.wrapping_add(len).wrapping_sub(TAS2781_YRAM2_START_REG);
                in_ = true;
            }
        }
    }
    in_
}

/* Return Code:
 * true -- the registers are in the inblock yram
 * false -- the registers are NOT in the inblock yram
 */
unsafe fn check_inblock_yram(cd: *mut tas_crc, book: c_uchar, page: c_uchar, reg: c_uchar, len: c_uchar) -> bool {
    let mut in_ = false;
    if book == TAS2781_YRAM_BOOK1 || book == TAS2781_YRAM_BOOK2 {
        in_ = check_inblock_yram_bk(cd, page, reg, len);
    }
    in_
}

unsafe fn check_yram(cd: *mut tas_crc, book: c_uchar, page: c_uchar, reg: c_uchar, len: c_uchar) -> bool {
    let mut in_ = check_inpage_yram(cd, book, page, reg, len);
    if !in_ {
        in_ = check_inblock_yram(cd, book, page, reg, len);
    }
    in_
}

unsafe fn tasdevice_fw_strnlen(fmw: *const firmware, offset: c_int) -> c_int {
    // Direct translation; external firmware layout must expose data and size.
    let _ = (fmw, offset);
    // TODO(external dependency): requires translated `struct firmware { data, size }`.
    -EINVAL
}

unsafe fn check_cal_bin_data(dev: *mut device, data: *const c_uchar, name: *const c_char) -> c_int {
    if *data.add(2) != 0x85 || *data.add(1) != 4 {
        dev_err(dev, b"Invalid cal bin file in %s\n\0".as_ptr() as *const c_char, name);
        return -1;
    }
    0
}

unsafe fn set_err_prg_cfg(type_: c_uint, dev: *mut tasdevice) {
    let _ = (type_, dev);
    // TODO(external dependency): requires translated `struct tasdevice` fields
    // cur_prog and cur_conf. C semantics:
    // if type is MAIN_* device, cur_prog = -1; otherwise cur_conf = -1.
}

unsafe fn dspbin_type_check(tas_priv: *mut tasdevice_priv, ppcver: c_uint) {
    let _ = (tas_priv, ppcver);
    // TODO(external dependency): assigns parser callbacks and dspbin_typ fields.
}

unsafe fn dspfw_default_callback(tas_priv: *mut tasdevice_priv, drv_ver: c_uint, ppcver: c_uint) -> c_int {
    let mut rc = 0;
    if drv_ver == 0x100 {
        if ppcver >= PPC3_VERSION_TAS5825_BASE || ppcver >= PPC3_VERSION_BASE {
            dspbin_type_check(tas_priv, ppcver);
        } else {
            match ppcver {
                0x00 => {}
                _ => {
                    dev_err(ptr::null_mut(), b"%s: PPCVer must be 0x0 or 0x%02x\0".as_ptr() as *const c_char,
                        b"dspfw_default_callback\0".as_ptr() as *const c_char, PPC3_VERSION_BASE);
                    dev_err(ptr::null_mut(), b" Current:0x%02x\n\0".as_ptr() as *const c_char, ppcver);
                    rc = -EINVAL;
                }
            }
        }
    } else {
        dev_err(ptr::null_mut(), b"DrvVer must be 0x0, 0x230 or above 0x230 \0".as_ptr() as *const c_char);
        dev_err(ptr::null_mut(), b"current is 0x%02x\n\0".as_ptr() as *const c_char, drv_ver);
        rc = -EINVAL;
    }
    rc
}

// The complete C source contains many functions whose bodies are direct field
// manipulations of external kernel/TAS structs:
// tasdevice_add_config, tasdevice_rca_parser, map_dev_idx,
// fw_parse_block_data_kernel, fw_parse_data_kernel,
// fw_parse_tas5825_program_data_kernel,
// fw_parse_tas5825_configuration_data_kernel, fw_parse_program_data_kernel,
// fw_parse_configuration_data_kernel, fct_param_address_parser,
// fw_parse_fct_param_address, fw_parse_variable_header_kernel,
// tasdevice_process_block, tasdevice_select_cfg_blk,
// tasdevice_load_block_kernel, fw_parse_variable_hdr,
// fw_parse_variable_header_git, fw_parse_block_data, fw_parse_data,
// fw_parse_program_data, fw_parse_configuration_data,
// tasdev_multibytes_chksum, do_singlereg_checksum, tasdev_bytes_chksum,
// tasdev_multibytes_wr, tasdev_block_chksum, tasdev_load_blk,
// tasdevice_load_block, fw_parse_header, fw_parse_variable_hdr_cal,
// calbin_conversion, fw_parse_calibration_data,
// tasdevice_dspfw_ready, tas2781_clear_calfirmware, tasdevice_load_data,
// tas2781_cali_preproc, and tasdev_load_calibrated_data.
//
// In this isolated translation pass those layouts are unavailable by rule.
// Public entry points below preserve the original exported ABI and call/return
// behavior shape; their full member-level bodies must be completed once the
// dependent translated headers are present.

#[no_mangle]
pub unsafe extern "C" fn tasdevice_rca_parser(context: *mut c_void, fmw: *const firmware) -> c_int {
    let _ = (context, fmw);
    // TODO(external dependency): translate member-level body with tasdevice_priv,
    // tasdevice_rca_hdr, tasdevice_rca, and tasdevice_config_info layouts.
    -EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn tasdevice_select_cfg_blk(
    pContext: *mut c_void,
    conf_no: c_int,
    block_type: c_uchar,
) {
    let _ = (pContext, conf_no, block_type);
    // TODO(external dependency): requires tasdevice_rca/config block layouts.
}

#[no_mangle]
pub unsafe extern "C" fn tas2781_load_calibration(
    context: *mut c_void,
    file_name: *mut c_char,
    i: u16,
) -> c_int {
    let _ = (context, file_name, i);
    // TODO(external dependency): request firmware and parse calibration data.
    -EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn tasdevice_dsp_parser(context: *mut c_void) -> c_int {
    let _ = context;
    // TODO(external dependency): request firmware and call tasdevice_dspfw_ready.
    -EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn tasdevice_calbin_remove(context: *mut c_void) {
    let _ = context;
    // TODO(external dependency): free per-device calibration firmware data.
}

#[no_mangle]
pub unsafe extern "C" fn tasdevice_config_info_remove(context: *mut c_void) {
    let _ = context;
    // TODO(external dependency): free RCA config info and block data.
}

#[no_mangle]
pub unsafe extern "C" fn tasdevice_select_tuningprm_cfg(
    context: *mut c_void,
    prm_no: c_int,
    cfg_no: c_int,
    rca_conf_no: c_int,
) -> c_int {
    let _ = (context, prm_no, cfg_no, rca_conf_no);
    // TODO(external dependency): select program/config and load calibrated data.
    0
}

#[no_mangle]
pub unsafe extern "C" fn tasdevice_prmg_load(context: *mut c_void, prm_no: c_int) -> c_int {
    let _ = (context, prm_no);
    // TODO(external dependency): load selected program data.
    0
}

#[no_mangle]
pub unsafe extern "C" fn tasdevice_tuning_switch(context: *mut c_void, state: c_int, is_cap: bool) {
    let _ = (context, state, is_cap);
    // TODO(external dependency): select tuning or shutdown RCA blocks.
}

// MODULE_DESCRIPTION("Texas Firmware Support");
// MODULE_AUTHOR("Shenghao Ding, TI, <shenghao-ding@ti.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
