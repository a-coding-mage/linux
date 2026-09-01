// SPDX-License-Identifier: GPL-2.0
//
// cs35l41-lib.c -- CS35L41 Common functions for HDA and ASoC Audio drivers
//
// Copyright 2017-2021 Cirrus Logic, Inc.
//
// Author: David Rhodes <david.rhodes@cirrus.com>
// Author: Lucas Tanure <lucas.tanure@cirrus.com>
//
// Rust translation of the isolated C implementation source. Linux headers:
// linux/dev_printk.h, linux/module.h, linux/regmap.h,
// linux/regulator/consumer.h, linux/slab.h, linux/firmware/cirrus/wmfw.h,
// sound/cs35l41.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

pub type bool_ = bool;
pub type u32 = u32;

pub const CS35L41_FIRMWARE_OLD_VERSION: u32 = 0x001C00; /* v0.28.0 */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
    pub delay_us: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub pad_bits: c_uint,
    pub reg_stride: c_uint,
    pub reg_format_endian: c_uint,
    pub val_format_endian: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub precious_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l41_otp_packed_element_t {
    pub reg: u32,
    pub shift: c_uint,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l41_otp_map_element_t {
    pub id: u32,
    pub map: *const cs35l41_otp_packed_element_t,
    pub num_elements: c_uint,
    pub bit_offset: c_int,
    pub word_offset: c_int,
}

#[repr(C)]
pub struct cs35l41_gpio_cfg {
    pub valid: bool,
    pub pol_inv: c_uint,
    pub out_en: c_uint,
    pub func: c_uint,
}

#[repr(C)]
pub struct cs35l41_hw_cfg {
    pub bst_type: cs35l41_boost_type,
    pub bst_ind: c_int,
    pub bst_cap: c_int,
    pub bst_ipk: c_int,
    pub gpio1: cs35l41_gpio_cfg,
    pub gpio2: cs35l41_gpio_cfg,
}

#[repr(C)]
pub struct cs_dsp_region {
    pub type_: c_uint,
    pub base: c_uint,
}

#[repr(C)]
pub struct cs_dsp {
    pub num: c_int,
    pub type_: c_uint,
    pub rev: c_uint,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub base: c_uint,
    pub base_sysinfo: c_uint,
    pub mem: *const cs_dsp_region,
    pub num_mems: c_uint,
    pub lock_regions: c_uint,
    pub running: bool,
    pub fw_id_version: u32,
}

pub type cs35l41_boost_type = c_uint;
pub type cs35l41_cspl_mbox_cmd = c_uint;
pub type cs35l41_cspl_mbox_status = c_uint;

extern "C" {
    static CS35L41_PWR_CTRL1: c_uint;
    static CS35L41_PWR_CTRL2: c_uint;
    static CS35L41_PWR_CTRL3: c_uint;
    static CS35L41_GPIO_PAD_CONTROL: c_uint;
    static CS35L41_GLOBAL_CLK_CTRL: c_uint;
    static CS35L41_TST_FS_MON0: c_uint;
    static CS35L41_BSTCVRT_PEAK_CUR: c_uint;
    static CS35L41_BSTCVRT_COEFF: c_uint;
    static CS35L41_BSTCVRT_SLOPE_LBST: c_uint;
    static CS35L41_SP_ENABLES: c_uint;
    static CS35L41_SP_RATE_CTRL: c_uint;
    static CS35L41_SP_FORMAT: c_uint;
    static CS35L41_SP_HIZ_CTRL: c_uint;
    static CS35L41_SP_FRAME_TX_SLOT: c_uint;
    static CS35L41_SP_FRAME_RX_SLOT: c_uint;
    static CS35L41_SP_TX_WL: c_uint;
    static CS35L41_SP_RX_WL: c_uint;
    static CS35L41_DAC_PCM1_SRC: c_uint;
    static CS35L41_ASP_TX1_SRC: c_uint;
    static CS35L41_ASP_TX2_SRC: c_uint;
    static CS35L41_ASP_TX3_SRC: c_uint;
    static CS35L41_ASP_TX4_SRC: c_uint;
    static CS35L41_DSP1_RX1_SRC: c_uint;
    static CS35L41_DSP1_RX2_SRC: c_uint;
    static CS35L41_DSP1_RX3_SRC: c_uint;
    static CS35L41_DSP1_RX4_SRC: c_uint;
    static CS35L41_DSP1_RX5_SRC: c_uint;
    static CS35L41_DSP1_RX6_SRC: c_uint;
    static CS35L41_DSP1_RX7_SRC: c_uint;
    static CS35L41_DSP1_RX8_SRC: c_uint;
    static CS35L41_NGATE1_SRC: c_uint;
    static CS35L41_NGATE2_SRC: c_uint;
    static CS35L41_AMP_DIG_VOL_CTRL: c_uint;
    static CS35L41_CLASSH_CFG: c_uint;
    static CS35L41_WKFET_CFG: c_uint;
    static CS35L41_NG_CFG: c_uint;
    static CS35L41_AMP_GAIN_CTRL: c_uint;
    static CS35L41_IRQ1_MASK1: c_uint;
    static CS35L41_IRQ1_MASK2: c_uint;
    static CS35L41_IRQ1_MASK3: c_uint;
    static CS35L41_IRQ1_MASK4: c_uint;
    static CS35L41_GPIO1_CTRL1: c_uint;
    static CS35L41_GPIO2_CTRL1: c_uint;
    static CS35L41_MIXER_NGATE_CFG: c_uint;
    static CS35L41_MIXER_NGATE_CH1_CFG: c_uint;
    static CS35L41_MIXER_NGATE_CH2_CFG: c_uint;
    static CS35L41_DSP1_CCM_CORE_CTRL: c_uint;
    static CS35L41_DEVID: c_uint;
    static CS35L41_REVID: c_uint;
    static CS35L41_FABID: c_uint;
    static CS35L41_RELID: c_uint;
    static CS35L41_OTPID: c_uint;
    static CS35L41_SFT_RESET: c_uint;
    static CS35L41_TEST_KEY_CTL: c_uint;
    static CS35L41_USER_KEY_CTL: c_uint;
    static CS35L41_OTP_MEM0: c_uint;
    static CS35L41_OTP_MEM31: c_uint;
    static CS35L41_DSP1_XMEM_PACK_0: c_uint;
    static CS35L41_DSP1_XMEM_PACK_3068: c_uint;
    static CS35L41_DSP1_YMEM_PACK_0: c_uint;
    static CS35L41_DSP1_YMEM_PACK_1532: c_uint;
    static CS35L41_DSP1_PMEM_0: c_uint;
    static CS35L41_DSP1_PMEM_5114: c_uint;
    static CS35L41_DSP_MBOX_1: c_uint;
    static CS35L41_DSP_MBOX_2: c_uint;
    static CS35L41_DSP_VIRT1_MBOX_1: c_uint;
    static CS35L41_DSP_VIRT2_MBOX_8: c_uint;
    static CS35L41_DSP1_CTRL_BASE: c_uint;
    static CS35L41_DSP1_SYS_ID: c_uint;
    static CS35L41_LASTREG: c_uint;
    static CS35L41_REGSTRIDE: c_uint;
    static REGMAP_ENDIAN_BIG: c_uint;
    static REGCACHE_MAPLE: c_uint;
    static CS35L41_REVID_A0: c_uint;
    static CS35L41_REVID_B0: c_uint;
    static CS35L41_REVID_B2: c_uint;
    static CS35L41_VIMON_SPKMON_RESYNC: c_uint;
    static CS35L41_VPVBST_FS_SEL: c_uint;
    static CS35L41_OTP_TRIM_30: c_uint;
    static CS35L41_BSTCVRT_DCM_CTRL: c_uint;
    static CS35L41_IRQ1_DB3: c_uint;
    static CS35L41_IRQ2_DB3: c_uint;
    static CS35L41_DSP1_YM_ACCEL_PL0_PRI: c_uint;
    static CS35L41_DSP1_XM_ACCEL_PL0_PRI: c_uint;
    static CS35L41_DSP1_RX1_RATE: c_uint;
    static CS35L41_DSP1_RX2_RATE: c_uint;
    static CS35L41_DSP1_RX3_RATE: c_uint;
    static CS35L41_DSP1_RX4_RATE: c_uint;
    static CS35L41_DSP1_RX5_RATE: c_uint;
    static CS35L41_DSP1_RX6_RATE: c_uint;
    static CS35L41_DSP1_RX7_RATE: c_uint;
    static CS35L41_DSP1_RX8_RATE: c_uint;
    static CS35L41_DSP1_TX1_RATE: c_uint;
    static CS35L41_DSP1_TX2_RATE: c_uint;
    static CS35L41_DSP1_TX3_RATE: c_uint;
    static CS35L41_DSP1_TX4_RATE: c_uint;
    static CS35L41_DSP1_TX5_RATE: c_uint;
    static CS35L41_DSP1_TX6_RATE: c_uint;
    static CS35L41_DSP1_TX7_RATE: c_uint;
    static CS35L41_DSP1_TX8_RATE: c_uint;
    static CS35L41_BST_K1_MASK: c_uint;
    static CS35L41_BST_K2_MASK: c_uint;
    static CS35L41_BST_K1_SHIFT: c_uint;
    static CS35L41_BST_K2_SHIFT: c_uint;
    static CS35L41_BST_SLOPE_MASK: c_uint;
    static CS35L41_BST_LBST_VAL_MASK: c_uint;
    static CS35L41_BST_SLOPE_SHIFT: c_uint;
    static CS35L41_BST_LBST_VAL_SHIFT: c_uint;
    static CS35L41_BST_IPK_MASK: c_uint;
    static CS35L41_BST_IPK_SHIFT: c_uint;
    static CS35L41_BST_EN_MASK: c_uint;
    static CS35L41_BST_EN_DEFAULT: c_uint;
    static CS35L41_BST_DIS_FET_OFF: c_uint;
    static CS35L41_BST_EN_SHIFT: c_uint;
    static CS35L41_MDSYNC_EN: c_uint;
    static CS35L41_BSTCVRT_VCTRL2: c_uint;
    static CS35L41_SHD_BOOST_ACTV: cs35l41_boost_type;
    static CS35L41_INT_BOOST: cs35l41_boost_type;
    static CS35L41_EXT_BOOST: cs35l41_boost_type;
    static CS35L41_EXT_BOOST_NO_VSPK_SWITCH: cs35l41_boost_type;
    static CS35L41_SHD_BOOST_PASS: cs35l41_boost_type;
    static CS35L41_PUP_DONE_MASK: c_uint;
    static CS35L41_PDN_DONE_MASK: c_uint;
    static CS35L41_IRQ1_STATUS1: c_uint;
    static CS35L41_GLOBAL_EN_MASK: c_uint;
    static CS35L41_GLOBAL_EN_SHIFT: c_uint;
    static CS35L41_SYNC_EN_MASK: c_uint;
    static CS35L41_GPIO1_MDSYNC: c_uint;
    static CS35L41_GPIO1_HIZ: c_uint;
    static CS35L41_GPIO1_CTRL_SHIFT: c_uint;
    static CS35L41_GPIO1_CTRL_MASK: c_uint;
    static CS35L41_GPIO2_CTRL_SHIFT: c_uint;
    static CS35L41_GPIO2_CTRL_MASK: c_uint;
    static CS35L41_GPIO_POL_MASK: c_uint;
    static CS35L41_GPIO_DIR_MASK: c_uint;
    static CS35L41_GPIO_POL_SHIFT: c_uint;
    static CS35L41_GPIO_DIR_SHIFT: c_uint;
    static CS35L41_GPIO2_INT_PUSH_PULL_LOW: c_uint;
    static CS35L41_GPIO2_INT_OPEN_DRAIN: c_uint;
    static CS35L41_GPIO2_INT_PUSH_PULL_HIGH: c_uint;
    static IRQF_TRIGGER_NONE: c_int;
    static IRQF_TRIGGER_LOW: c_int;
    static IRQF_TRIGGER_HIGH: c_int;
    static WMFW_HALO_PM_PACKED: c_uint;
    static WMFW_HALO_XM_PACKED: c_uint;
    static WMFW_HALO_YM_PACKED: c_uint;
    static WMFW_ADSP2_XM: c_uint;
    static WMFW_ADSP2_YM: c_uint;
    static WMFW_HALO: c_uint;
    static CSPL_MBOX_CMD_NONE: cs35l41_cspl_mbox_cmd;
    static CSPL_MBOX_CMD_UNKNOWN_CMD: cs35l41_cspl_mbox_cmd;
    static CSPL_MBOX_CMD_PAUSE: cs35l41_cspl_mbox_cmd;
    static CSPL_MBOX_CMD_OUT_OF_HIBERNATE: cs35l41_cspl_mbox_cmd;
    static CSPL_MBOX_CMD_RESUME: cs35l41_cspl_mbox_cmd;
    static CSPL_MBOX_CMD_REINIT: cs35l41_cspl_mbox_cmd;
    static CSPL_MBOX_CMD_STOP_PRE_REINIT: cs35l41_cspl_mbox_cmd;
    static CSPL_MBOX_CMD_SPK_OUT_ENABLE: cs35l41_cspl_mbox_cmd;
    static CSPL_MBOX_CMD_HIBERNATE: cs35l41_cspl_mbox_cmd;
    static CSPL_MBOX_STS_PAUSED: cs35l41_cspl_mbox_status;
    static CSPL_MBOX_STS_RUNNING: cs35l41_cspl_mbox_status;
    static CSPL_MBOX_STS_RDY_FOR_REINIT: cs35l41_cspl_mbox_status;
    static CSPL_MBOX_STS_ERROR: cs35l41_cspl_mbox_status;
    static CSPL_MBOX_STS_ERROR2: cs35l41_cspl_mbox_status;
    static CS35L41_WAKESRC_CTL: c_uint;
    static CS35L41_PWRMGT_STS: c_uint;
    static CS35L41_WR_PEND_STS_MASK: c_uint;
    static CS35L41_PWRMGT_CTL: c_uint;
}

extern "C" {
    fn regmap_multi_reg_write(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_bulk_read(map: *mut regmap, reg: c_uint, val: *mut c_void, val_count: c_int) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_register_patch(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn kmalloc_array(n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn usleep_range(min: c_uint, max: c_uint);
    fn udelay(usecs: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

extern "Rust" {
    fn regmap_read_poll_timeout(
        map: *mut regmap,
        reg: c_uint,
        val: &mut c_uint,
        cond_mask: c_uint,
        sleep_us: c_uint,
        timeout_us: c_uint,
    ) -> c_int;
}

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOMSG: c_int = 42;
const ETIMEDOUT: c_int = 110;
const CS35L41_OTP_SIZE_WORDS: usize = 32;

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    if h >= 31 && l == 0 {
        !0
    } else {
        (((1u64 << (h - l + 1)) - 1) << l) as c_uint
    }
}

const fn rs(reg: c_uint, def: c_uint) -> reg_sequence {
    reg_sequence { reg, def, delay_us: 0 }
}

const fn rsd(reg: c_uint, def: c_uint, delay_us: c_uint) -> reg_sequence {
    reg_sequence { reg, def, delay_us }
}

static cs35l41_reg: &[reg_default] = &[
    reg_default { reg: unsafe { CS35L41_PWR_CTRL1 }, def: 0x00000000 },
    reg_default { reg: unsafe { CS35L41_PWR_CTRL2 }, def: 0x00000000 },
    reg_default { reg: unsafe { CS35L41_PWR_CTRL3 }, def: 0x01000010 },
    reg_default { reg: unsafe { CS35L41_GPIO_PAD_CONTROL }, def: 0x00000000 },
    reg_default { reg: unsafe { CS35L41_GLOBAL_CLK_CTRL }, def: 0x00000003 },
    reg_default { reg: unsafe { CS35L41_TST_FS_MON0 }, def: 0x00020016 },
    reg_default { reg: unsafe { CS35L41_BSTCVRT_PEAK_CUR }, def: 0x0000004A },
    reg_default { reg: unsafe { CS35L41_BSTCVRT_COEFF }, def: 0x00002424 },
    reg_default { reg: unsafe { CS35L41_BSTCVRT_SLOPE_LBST }, def: 0x00007500 },
    reg_default { reg: unsafe { CS35L41_SP_ENABLES }, def: 0x00000000 },
    reg_default { reg: unsafe { CS35L41_SP_RATE_CTRL }, def: 0x00000028 },
    reg_default { reg: unsafe { CS35L41_SP_FORMAT }, def: 0x18180200 },
    reg_default { reg: unsafe { CS35L41_SP_HIZ_CTRL }, def: 0x00000002 },
    reg_default { reg: unsafe { CS35L41_SP_FRAME_TX_SLOT }, def: 0x03020100 },
    reg_default { reg: unsafe { CS35L41_SP_FRAME_RX_SLOT }, def: 0x00000100 },
    reg_default { reg: unsafe { CS35L41_SP_TX_WL }, def: 0x00000018 },
    reg_default { reg: unsafe { CS35L41_SP_RX_WL }, def: 0x00000018 },
    reg_default { reg: unsafe { CS35L41_DAC_PCM1_SRC }, def: 0x00000008 },
    reg_default { reg: unsafe { CS35L41_ASP_TX1_SRC }, def: 0x00000018 },
    reg_default { reg: unsafe { CS35L41_ASP_TX2_SRC }, def: 0x00000019 },
    reg_default { reg: unsafe { CS35L41_ASP_TX3_SRC }, def: 0x00000000 },
    reg_default { reg: unsafe { CS35L41_ASP_TX4_SRC }, def: 0x00000000 },
    reg_default { reg: unsafe { CS35L41_DSP1_RX1_SRC }, def: 0x00000008 },
    reg_default { reg: unsafe { CS35L41_DSP1_RX2_SRC }, def: 0x00000009 },
    reg_default { reg: unsafe { CS35L41_DSP1_RX3_SRC }, def: 0x00000018 },
    reg_default { reg: unsafe { CS35L41_DSP1_RX4_SRC }, def: 0x00000019 },
    reg_default { reg: unsafe { CS35L41_DSP1_RX5_SRC }, def: 0x00000020 },
    reg_default { reg: unsafe { CS35L41_DSP1_RX6_SRC }, def: 0x00000021 },
    reg_default { reg: unsafe { CS35L41_DSP1_RX7_SRC }, def: 0x0000003A },
    reg_default { reg: unsafe { CS35L41_DSP1_RX8_SRC }, def: 0x0000003B },
    reg_default { reg: unsafe { CS35L41_NGATE1_SRC }, def: 0x00000008 },
    reg_default { reg: unsafe { CS35L41_NGATE2_SRC }, def: 0x00000009 },
    reg_default { reg: unsafe { CS35L41_AMP_DIG_VOL_CTRL }, def: 0x00008000 },
    reg_default { reg: unsafe { CS35L41_CLASSH_CFG }, def: 0x000B0405 },
    reg_default { reg: unsafe { CS35L41_WKFET_CFG }, def: 0x00000111 },
    reg_default { reg: unsafe { CS35L41_NG_CFG }, def: 0x00000033 },
    reg_default { reg: unsafe { CS35L41_AMP_GAIN_CTRL }, def: 0x00000000 },
    reg_default { reg: unsafe { CS35L41_IRQ1_MASK1 }, def: 0xFFFFFFFF },
    reg_default { reg: unsafe { CS35L41_IRQ1_MASK2 }, def: 0xFFFFFFFF },
    reg_default { reg: unsafe { CS35L41_IRQ1_MASK3 }, def: 0xFFFF87FF },
    reg_default { reg: unsafe { CS35L41_IRQ1_MASK4 }, def: 0xFEFFFFFF },
    reg_default { reg: unsafe { CS35L41_GPIO1_CTRL1 }, def: 0x81000001 },
    reg_default { reg: unsafe { CS35L41_GPIO2_CTRL1 }, def: 0x81000001 },
    reg_default { reg: unsafe { CS35L41_MIXER_NGATE_CFG }, def: 0x00000000 },
    reg_default { reg: unsafe { CS35L41_MIXER_NGATE_CH1_CFG }, def: 0x00000303 },
    reg_default { reg: unsafe { CS35L41_MIXER_NGATE_CH2_CFG }, def: 0x00000303 },
    reg_default { reg: unsafe { CS35L41_DSP1_CCM_CORE_CTRL }, def: 0x00000101 },
];

unsafe extern "C" fn cs35l41_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg == CS35L41_DEVID || reg == CS35L41_REVID || reg == CS35L41_FABID ||
    reg == CS35L41_RELID || reg == CS35L41_OTPID || reg == CS35L41_SFT_RESET ||
    reg == CS35L41_TEST_KEY_CTL || reg == CS35L41_USER_KEY_CTL ||
    reg == CS35L41_PWR_CTRL1 || reg == CS35L41_PWR_CTRL2 || reg == CS35L41_PWR_CTRL3 ||
    reg == CS35L41_GPIO_PAD_CONTROL || reg == CS35L41_GLOBAL_CLK_CTRL ||
    reg == CS35L41_TST_FS_MON0 || reg == CS35L41_BSTCVRT_PEAK_CUR ||
    reg == CS35L41_BSTCVRT_COEFF || reg == CS35L41_BSTCVRT_SLOPE_LBST ||
    reg == CS35L41_SP_ENABLES || reg == CS35L41_SP_RATE_CTRL ||
    reg == CS35L41_SP_FORMAT || reg == CS35L41_SP_HIZ_CTRL ||
    reg == CS35L41_SP_FRAME_TX_SLOT || reg == CS35L41_SP_FRAME_RX_SLOT ||
    reg == CS35L41_SP_TX_WL || reg == CS35L41_SP_RX_WL ||
    reg == CS35L41_DAC_PCM1_SRC || reg == CS35L41_ASP_TX1_SRC ||
    reg == CS35L41_ASP_TX2_SRC || reg == CS35L41_ASP_TX3_SRC ||
    reg == CS35L41_ASP_TX4_SRC || reg == CS35L41_DSP1_RX1_SRC ||
    reg == CS35L41_DSP1_RX2_SRC || reg == CS35L41_DSP1_RX3_SRC ||
    reg == CS35L41_DSP1_RX4_SRC || reg == CS35L41_DSP1_RX5_SRC ||
    reg == CS35L41_DSP1_RX6_SRC || reg == CS35L41_DSP1_RX7_SRC ||
    reg == CS35L41_DSP1_RX8_SRC || reg == CS35L41_NGATE1_SRC ||
    reg == CS35L41_NGATE2_SRC || reg == CS35L41_AMP_DIG_VOL_CTRL ||
    reg == CS35L41_CLASSH_CFG || reg == CS35L41_WKFET_CFG ||
    reg == CS35L41_NG_CFG || reg == CS35L41_AMP_GAIN_CTRL ||
    reg == CS35L41_IRQ1_MASK1 || reg == CS35L41_IRQ1_MASK2 ||
    reg == CS35L41_IRQ1_MASK3 || reg == CS35L41_IRQ1_MASK4 ||
    reg == CS35L41_GPIO1_CTRL1 || reg == CS35L41_GPIO2_CTRL1 ||
    reg == CS35L41_MIXER_NGATE_CFG || reg == CS35L41_MIXER_NGATE_CH1_CFG ||
    reg == CS35L41_MIXER_NGATE_CH2_CFG ||
    (reg >= CS35L41_DSP_MBOX_1 && reg <= CS35L41_DSP_VIRT2_MBOX_8) ||
    (reg >= CS35L41_OTP_MEM0 && reg <= CS35L41_OTP_MEM31) ||
    (reg >= CS35L41_DSP1_XMEM_PACK_0 && reg <= CS35L41_DSP1_XMEM_PACK_3068) ||
    (reg >= CS35L41_DSP1_YMEM_PACK_0 && reg <= CS35L41_DSP1_YMEM_PACK_1532) ||
    (reg >= CS35L41_DSP1_PMEM_0 && reg <= CS35L41_DSP1_PMEM_5114)
}

unsafe extern "C" fn cs35l41_precious_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg == CS35L41_TEST_KEY_CTL || reg == CS35L41_USER_KEY_CTL ||
    reg == CS35L41_TST_FS_MON0 ||
    (reg >= CS35L41_OTP_MEM0 && reg <= CS35L41_OTP_MEM31) ||
    (reg >= CS35L41_DSP1_XMEM_PACK_0 && reg <= CS35L41_DSP1_XMEM_PACK_3068) ||
    (reg >= CS35L41_DSP1_YMEM_PACK_0 && reg <= CS35L41_DSP1_YMEM_PACK_1532) ||
    (reg >= CS35L41_DSP1_PMEM_0 && reg <= CS35L41_DSP1_PMEM_5114)
}

unsafe extern "C" fn cs35l41_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg == CS35L41_DEVID || reg == CS35L41_SFT_RESET || reg == CS35L41_FABID ||
    reg == CS35L41_REVID || reg == CS35L41_OTPID || reg == CS35L41_TEST_KEY_CTL ||
    reg == CS35L41_USER_KEY_CTL || reg == CS35L41_IRQ1_STATUS1 ||
    reg == CS35L41_DSP_MBOX_2 ||
    (reg >= CS35L41_DSP_MBOX_1 && reg <= CS35L41_DSP_VIRT2_MBOX_8) ||
    (reg >= CS35L41_DSP1_XMEM_PACK_0 && reg <= CS35L41_DSP1_XMEM_PACK_3068) ||
    (reg >= CS35L41_DSP1_YMEM_PACK_0 && reg <= CS35L41_DSP1_YMEM_PACK_1532) ||
    (reg >= CS35L41_DSP1_PMEM_0 && reg <= CS35L41_DSP1_PMEM_5114) ||
    (reg >= CS35L41_OTP_MEM0 && reg <= CS35L41_OTP_MEM31)
}

static otp_map_1: &[cs35l41_otp_packed_element_t] = &[
    cs35l41_otp_packed_element_t { reg: 0x00002030, shift: 0, size: 4 }, /*TRIM_OSC_FREQ_TRIM*/
    cs35l41_otp_packed_element_t { reg: 0x00002030, shift: 7, size: 1 }, /*TRIM_OSC_TRIM_DONE*/
    cs35l41_otp_packed_element_t { reg: 0x0000208c, shift: 24, size: 6 }, /*TST_DIGREG_VREF_TRIM*/
    cs35l41_otp_packed_element_t { reg: 0x00002090, shift: 14, size: 4 }, /*TST_REF_TRIM*/
    cs35l41_otp_packed_element_t { reg: 0x00002090, shift: 10, size: 4 }, /*TST_REF_TEMPCO_TRIM*/
    cs35l41_otp_packed_element_t { reg: 0x0000300C, shift: 11, size: 4 }, /*PLL_LDOA_TST_VREF_TRIM*/
    cs35l41_otp_packed_element_t { reg: 0x0000394C, shift: 23, size: 2 }, /*BST_ATEST_CM_VOFF*/
    cs35l41_otp_packed_element_t { reg: 0x00003950, shift: 0, size: 7 }, /*BST_ATRIM_IADC_OFFSET*/
    cs35l41_otp_packed_element_t { reg: 0x00003950, shift: 8, size: 7 }, /*BST_ATRIM_IADC_GAIN1*/
    cs35l41_otp_packed_element_t { reg: 0x00003950, shift: 16, size: 8 }, /*BST_ATRIM_IPKCOMP_OFFSET1*/
    cs35l41_otp_packed_element_t { reg: 0x00003950, shift: 24, size: 8 }, /*BST_ATRIM_IPKCOMP_GAIN1*/
    cs35l41_otp_packed_element_t { reg: 0x00003954, shift: 0, size: 7 }, /*BST_ATRIM_IADC_OFFSET2*/
    cs35l41_otp_packed_element_t { reg: 0x00003954, shift: 8, size: 7 }, /*BST_ATRIM_IADC_GAIN2*/
    cs35l41_otp_packed_element_t { reg: 0x00003954, shift: 16, size: 8 }, /*BST_ATRIM_IPKCOMP_OFFSET2*/
    cs35l41_otp_packed_element_t { reg: 0x00003954, shift: 24, size: 8 }, /*BST_ATRIM_IPKCOMP_GAIN2*/
    cs35l41_otp_packed_element_t { reg: 0x00003958, shift: 0, size: 7 }, /*BST_ATRIM_IADC_OFFSET3*/
    cs35l41_otp_packed_element_t { reg: 0x00003958, shift: 8, size: 7 }, /*BST_ATRIM_IADC_GAIN3*/
    cs35l41_otp_packed_element_t { reg: 0x00003958, shift: 16, size: 8 }, /*BST_ATRIM_IPKCOMP_OFFSET3*/
    cs35l41_otp_packed_element_t { reg: 0x00003958, shift: 24, size: 8 }, /*BST_ATRIM_IPKCOMP_GAIN3*/
    cs35l41_otp_packed_element_t { reg: 0x0000395C, shift: 0, size: 7 }, /*BST_ATRIM_IADC_OFFSET4*/
    cs35l41_otp_packed_element_t { reg: 0x0000395C, shift: 8, size: 7 }, /*BST_ATRIM_IADC_GAIN4*/
    cs35l41_otp_packed_element_t { reg: 0x0000395C, shift: 16, size: 8 }, /*BST_ATRIM_IPKCOMP_OFFSET4*/
    cs35l41_otp_packed_element_t { reg: 0x0000395C, shift: 24, size: 8 }, /*BST_ATRIM_IPKCOMP_GAIN4*/
    cs35l41_otp_packed_element_t { reg: 0x0000416C, shift: 0, size: 8 }, /*VMON_GAIN_OTP_VAL*/
    cs35l41_otp_packed_element_t { reg: 0x00004160, shift: 0, size: 7 }, /*VMON_OFFSET_OTP_VAL*/
    cs35l41_otp_packed_element_t { reg: 0x0000416C, shift: 8, size: 8 }, /*IMON_GAIN_OTP_VAL*/
    cs35l41_otp_packed_element_t { reg: 0x00004160, shift: 16, size: 10 }, /*IMON_OFFSET_OTP_VAL*/
    cs35l41_otp_packed_element_t { reg: 0x0000416C, shift: 16, size: 12 }, /*VMON_CM_GAIN_OTP_VAL*/
    cs35l41_otp_packed_element_t { reg: 0x0000416C, shift: 28, size: 1 }, /*VMON_CM_GAIN_SIGN_OTP_VAL*/
    cs35l41_otp_packed_element_t { reg: 0x00004170, shift: 0, size: 6 }, /*IMON_CAL_TEMPCO_OTP_VAL*/
    cs35l41_otp_packed_element_t { reg: 0x00004170, shift: 6, size: 1 }, /*IMON_CAL_TEMPCO_SIGN_OTP*/
    cs35l41_otp_packed_element_t { reg: 0x00004170, shift: 8, size: 6 }, /*IMON_CAL_TEMPCO2_OTP_VAL*/
    cs35l41_otp_packed_element_t { reg: 0x00004170, shift: 14, size: 1 }, /*IMON_CAL_TEMPCO2_DN_UPB_OTP_VAL*/
    cs35l41_otp_packed_element_t { reg: 0x00004170, shift: 16, size: 9 }, /*IMON_CAL_TEMPCO_TBASE_OTP_VAL*/
    cs35l41_otp_packed_element_t { reg: 0x00004360, shift: 0, size: 5 }, /*TEMP_GAIN_OTP_VAL*/
    cs35l41_otp_packed_element_t { reg: 0x00004360, shift: 6, size: 9 }, /*TEMP_OFFSET_OTP_VAL*/
    cs35l41_otp_packed_element_t { reg: 0x00004448, shift: 0, size: 8 }, /*VP_SARADC_OFFSET*/
    cs35l41_otp_packed_element_t { reg: 0x00004448, shift: 8, size: 8 }, /*VP_GAIN_INDEX*/
    cs35l41_otp_packed_element_t { reg: 0x00004448, shift: 16, size: 8 }, /*VBST_SARADC_OFFSET*/
    cs35l41_otp_packed_element_t { reg: 0x00004448, shift: 24, size: 8 }, /*VBST_GAIN_INDEX*/
    cs35l41_otp_packed_element_t { reg: 0x0000444C, shift: 0, size: 3 }, /*ANA_SELINVREF*/
    cs35l41_otp_packed_element_t { reg: 0x00006E30, shift: 0, size: 5 }, /*GAIN_ERR_COEFF_0*/
    cs35l41_otp_packed_element_t { reg: 0x00006E30, shift: 8, size: 5 }, /*GAIN_ERR_COEFF_1*/
    cs35l41_otp_packed_element_t { reg: 0x00006E30, shift: 16, size: 5 }, /*GAIN_ERR_COEFF_2*/
    cs35l41_otp_packed_element_t { reg: 0x00006E30, shift: 24, size: 5 }, /*GAIN_ERR_COEFF_3*/
    cs35l41_otp_packed_element_t { reg: 0x00006E34, shift: 0, size: 5 }, /*GAIN_ERR_COEFF_4*/
    cs35l41_otp_packed_element_t { reg: 0x00006E34, shift: 8, size: 5 }, /*GAIN_ERR_COEFF_5*/
    cs35l41_otp_packed_element_t { reg: 0x00006E34, shift: 16, size: 5 }, /*GAIN_ERR_COEFF_6*/
    cs35l41_otp_packed_element_t { reg: 0x00006E34, shift: 24, size: 5 }, /*GAIN_ERR_COEFF_7*/
    cs35l41_otp_packed_element_t { reg: 0x00006E38, shift: 0, size: 5 }, /*GAIN_ERR_COEFF_8*/
    cs35l41_otp_packed_element_t { reg: 0x00006E38, shift: 8, size: 5 }, /*GAIN_ERR_COEFF_9*/
    cs35l41_otp_packed_element_t { reg: 0x00006E38, shift: 16, size: 5 }, /*GAIN_ERR_COEFF_10*/
    cs35l41_otp_packed_element_t { reg: 0x00006E38, shift: 24, size: 5 }, /*GAIN_ERR_COEFF_11*/
    cs35l41_otp_packed_element_t { reg: 0x00006E3C, shift: 0, size: 5 }, /*GAIN_ERR_COEFF_12*/
    cs35l41_otp_packed_element_t { reg: 0x00006E3C, shift: 8, size: 5 }, /*GAIN_ERR_COEFF_13*/
    cs35l41_otp_packed_element_t { reg: 0x00006E3C, shift: 16, size: 5 }, /*GAIN_ERR_COEFF_14*/
    cs35l41_otp_packed_element_t { reg: 0x00006E3C, shift: 24, size: 5 }, /*GAIN_ERR_COEFF_15*/
    cs35l41_otp_packed_element_t { reg: 0x00006E40, shift: 0, size: 5 }, /*GAIN_ERR_COEFF_16*/
    cs35l41_otp_packed_element_t { reg: 0x00006E40, shift: 8, size: 5 }, /*GAIN_ERR_COEFF_17*/
    cs35l41_otp_packed_element_t { reg: 0x00006E40, shift: 16, size: 5 }, /*GAIN_ERR_COEFF_18*/
    cs35l41_otp_packed_element_t { reg: 0x00006E40, shift: 24, size: 5 }, /*GAIN_ERR_COEFF_19*/
    cs35l41_otp_packed_element_t { reg: 0x00006E44, shift: 0, size: 5 }, /*GAIN_ERR_COEFF_20*/
    cs35l41_otp_packed_element_t { reg: 0x00006E48, shift: 0, size: 10 }, /*VOFF_GAIN_0*/
    cs35l41_otp_packed_element_t { reg: 0x00006E48, shift: 10, size: 10 }, /*VOFF_GAIN_1*/
    cs35l41_otp_packed_element_t { reg: 0x00006E48, shift: 20, size: 10 }, /*VOFF_GAIN_2*/
    cs35l41_otp_packed_element_t { reg: 0x00006E4C, shift: 0, size: 10 }, /*VOFF_GAIN_3*/
    cs35l41_otp_packed_element_t { reg: 0x00006E4C, shift: 10, size: 10 }, /*VOFF_GAIN_4*/
    cs35l41_otp_packed_element_t { reg: 0x00006E4C, shift: 20, size: 10 }, /*VOFF_GAIN_5*/
    cs35l41_otp_packed_element_t { reg: 0x00006E50, shift: 0, size: 10 }, /*VOFF_GAIN_6*/
    cs35l41_otp_packed_element_t { reg: 0x00006E50, shift: 10, size: 10 }, /*VOFF_GAIN_7*/
    cs35l41_otp_packed_element_t { reg: 0x00006E50, shift: 20, size: 10 }, /*VOFF_GAIN_8*/
    cs35l41_otp_packed_element_t { reg: 0x00006E54, shift: 0, size: 10 }, /*VOFF_GAIN_9*/
    cs35l41_otp_packed_element_t { reg: 0x00006E54, shift: 10, size: 10 }, /*VOFF_GAIN_10*/
    cs35l41_otp_packed_element_t { reg: 0x00006E54, shift: 20, size: 10 }, /*VOFF_GAIN_11*/
    cs35l41_otp_packed_element_t { reg: 0x00006E58, shift: 0, size: 10 }, /*VOFF_GAIN_12*/
    cs35l41_otp_packed_element_t { reg: 0x00006E58, shift: 10, size: 10 }, /*VOFF_GAIN_13*/
    cs35l41_otp_packed_element_t { reg: 0x00006E58, shift: 20, size: 10 }, /*VOFF_GAIN_14*/
    cs35l41_otp_packed_element_t { reg: 0x00006E5C, shift: 0, size: 10 }, /*VOFF_GAIN_15*/
    cs35l41_otp_packed_element_t { reg: 0x00006E5C, shift: 10, size: 10 }, /*VOFF_GAIN_16*/
    cs35l41_otp_packed_element_t { reg: 0x00006E5C, shift: 20, size: 10 }, /*VOFF_GAIN_17*/
    cs35l41_otp_packed_element_t { reg: 0x00006E60, shift: 0, size: 10 }, /*VOFF_GAIN_18*/
    cs35l41_otp_packed_element_t { reg: 0x00006E60, shift: 10, size: 10 }, /*VOFF_GAIN_19*/
    cs35l41_otp_packed_element_t { reg: 0x00006E60, shift: 20, size: 10 }, /*VOFF_GAIN_20*/
    cs35l41_otp_packed_element_t { reg: 0x00006E64, shift: 0, size: 10 }, /*VOFF_INT1*/
    cs35l41_otp_packed_element_t { reg: 0x00007418, shift: 7, size: 5 }, /*DS_SPK_INT1_CAP_TRIM*/
    cs35l41_otp_packed_element_t { reg: 0x0000741C, shift: 0, size: 5 }, /*DS_SPK_INT2_CAP_TRIM*/
    cs35l41_otp_packed_element_t { reg: 0x0000741C, shift: 11, size: 4 }, /*DS_SPK_LPF_CAP_TRIM*/
    cs35l41_otp_packed_element_t { reg: 0x0000741C, shift: 19, size: 4 }, /*DS_SPK_QUAN_CAP_TRIM*/
    cs35l41_otp_packed_element_t { reg: 0x00007434, shift: 17, size: 1 }, /*FORCE_CAL*/
    cs35l41_otp_packed_element_t { reg: 0x00007434, shift: 18, size: 7 }, /*CAL_OVERRIDE*/
    cs35l41_otp_packed_element_t { reg: 0x00007068, shift: 0, size: 9 }, /*MODIX*/
    cs35l41_otp_packed_element_t { reg: 0x0000410C, shift: 7, size: 1 }, /*VIMON_DLY_NOT_COMB*/
    cs35l41_otp_packed_element_t { reg: 0x0000400C, shift: 0, size: 7 }, /*VIMON_DLY*/
    cs35l41_otp_packed_element_t { reg: 0x00000000, shift: 0, size: 1 }, /*extra bit*/
    cs35l41_otp_packed_element_t { reg: 0x00017040, shift: 0, size: 8 }, /*X_COORDINATE*/
    cs35l41_otp_packed_element_t { reg: 0x00017040, shift: 8, size: 8 }, /*Y_COORDINATE*/
    cs35l41_otp_packed_element_t { reg: 0x00017040, shift: 16, size: 8 }, /*WAFER_ID*/
    cs35l41_otp_packed_element_t { reg: 0x00017040, shift: 24, size: 8 }, /*DVS*/
    cs35l41_otp_packed_element_t { reg: 0x00017044, shift: 0, size: 24 }, /*LOT_NUMBER*/
];

static otp_map_2: &[cs35l41_otp_packed_element_t] = &[
    /* Same packing as otp_map_1 except VMON_POL replaces the extra bit. */
    cs35l41_otp_packed_element_t { reg: 0x00002030, shift: 0, size: 4 },
    cs35l41_otp_packed_element_t { reg: 0x00002030, shift: 7, size: 1 },
    cs35l41_otp_packed_element_t { reg: 0x0000208c, shift: 24, size: 6 },
    cs35l41_otp_packed_element_t { reg: 0x00002090, shift: 14, size: 4 },
    cs35l41_otp_packed_element_t { reg: 0x00002090, shift: 10, size: 4 },
    cs35l41_otp_packed_element_t { reg: 0x0000300C, shift: 11, size: 4 },
    cs35l41_otp_packed_element_t { reg: 0x0000394C, shift: 23, size: 2 },
    cs35l41_otp_packed_element_t { reg: 0x00003950, shift: 0, size: 7 },
    cs35l41_otp_packed_element_t { reg: 0x00003950, shift: 8, size: 7 },
    cs35l41_otp_packed_element_t { reg: 0x00003950, shift: 16, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x00003950, shift: 24, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x00003954, shift: 0, size: 7 },
    cs35l41_otp_packed_element_t { reg: 0x00003954, shift: 8, size: 7 },
    cs35l41_otp_packed_element_t { reg: 0x00003954, shift: 16, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x00003954, shift: 24, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x00003958, shift: 0, size: 7 },
    cs35l41_otp_packed_element_t { reg: 0x00003958, shift: 8, size: 7 },
    cs35l41_otp_packed_element_t { reg: 0x00003958, shift: 16, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x00003958, shift: 24, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x0000395C, shift: 0, size: 7 },
    cs35l41_otp_packed_element_t { reg: 0x0000395C, shift: 8, size: 7 },
    cs35l41_otp_packed_element_t { reg: 0x0000395C, shift: 16, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x0000395C, shift: 24, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x0000416C, shift: 0, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x00004160, shift: 0, size: 7 },
    cs35l41_otp_packed_element_t { reg: 0x0000416C, shift: 8, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x00004160, shift: 16, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x0000416C, shift: 16, size: 12 },
    cs35l41_otp_packed_element_t { reg: 0x0000416C, shift: 28, size: 1 },
    cs35l41_otp_packed_element_t { reg: 0x00004170, shift: 0, size: 6 },
    cs35l41_otp_packed_element_t { reg: 0x00004170, shift: 6, size: 1 },
    cs35l41_otp_packed_element_t { reg: 0x00004170, shift: 8, size: 6 },
    cs35l41_otp_packed_element_t { reg: 0x00004170, shift: 14, size: 1 },
    cs35l41_otp_packed_element_t { reg: 0x00004170, shift: 16, size: 9 },
    cs35l41_otp_packed_element_t { reg: 0x00004360, shift: 0, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00004360, shift: 6, size: 9 },
    cs35l41_otp_packed_element_t { reg: 0x00004448, shift: 0, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x00004448, shift: 8, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x00004448, shift: 16, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x00004448, shift: 24, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x0000444C, shift: 0, size: 3 },
    cs35l41_otp_packed_element_t { reg: 0x00006E30, shift: 0, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E30, shift: 8, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E30, shift: 16, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E30, shift: 24, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E34, shift: 0, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E34, shift: 8, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E34, shift: 16, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E34, shift: 24, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E38, shift: 0, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E38, shift: 8, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E38, shift: 16, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E38, shift: 24, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E3C, shift: 0, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E3C, shift: 8, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E3C, shift: 16, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E3C, shift: 24, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E40, shift: 0, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E40, shift: 8, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E40, shift: 16, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E40, shift: 24, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E44, shift: 0, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x00006E48, shift: 0, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E48, shift: 10, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E48, shift: 20, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E4C, shift: 0, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E4C, shift: 10, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E4C, shift: 20, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E50, shift: 0, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E50, shift: 10, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E50, shift: 20, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E54, shift: 0, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E54, shift: 10, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E54, shift: 20, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E58, shift: 0, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E58, shift: 10, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E58, shift: 20, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E5C, shift: 0, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E5C, shift: 10, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E5C, shift: 20, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E60, shift: 0, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E60, shift: 10, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E60, shift: 20, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00006E64, shift: 0, size: 10 },
    cs35l41_otp_packed_element_t { reg: 0x00007418, shift: 7, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x0000741C, shift: 0, size: 5 },
    cs35l41_otp_packed_element_t { reg: 0x0000741C, shift: 11, size: 4 },
    cs35l41_otp_packed_element_t { reg: 0x0000741C, shift: 19, size: 4 },
    cs35l41_otp_packed_element_t { reg: 0x00007434, shift: 17, size: 1 },
    cs35l41_otp_packed_element_t { reg: 0x00007434, shift: 18, size: 7 },
    cs35l41_otp_packed_element_t { reg: 0x00007068, shift: 0, size: 9 },
    cs35l41_otp_packed_element_t { reg: 0x0000410C, shift: 7, size: 1 },
    cs35l41_otp_packed_element_t { reg: 0x0000400C, shift: 0, size: 7 },
    cs35l41_otp_packed_element_t { reg: 0x00004000, shift: 11, size: 1 }, /*VMON_POL*/
    cs35l41_otp_packed_element_t { reg: 0x00017040, shift: 0, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x00017040, shift: 8, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x00017040, shift: 16, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x00017040, shift: 24, size: 8 },
    cs35l41_otp_packed_element_t { reg: 0x00017044, shift: 0, size: 24 },
];

static mut cs35l41_otp_map_map: [cs35l41_otp_map_element_t; 5] = [
    cs35l41_otp_map_element_t { id: 0x01, map: otp_map_1.as_ptr(), num_elements: otp_map_1.len() as c_uint, bit_offset: 16, word_offset: 2 },
    cs35l41_otp_map_element_t { id: 0x02, map: otp_map_2.as_ptr(), num_elements: otp_map_2.len() as c_uint, bit_offset: 16, word_offset: 2 },
    cs35l41_otp_map_element_t { id: 0x03, map: otp_map_2.as_ptr(), num_elements: otp_map_2.len() as c_uint, bit_offset: 16, word_offset: 2 },
    cs35l41_otp_map_element_t { id: 0x06, map: otp_map_2.as_ptr(), num_elements: otp_map_2.len() as c_uint, bit_offset: 16, word_offset: 2 },
    cs35l41_otp_map_element_t { id: 0x08, map: otp_map_1.as_ptr(), num_elements: otp_map_1.len() as c_uint, bit_offset: 16, word_offset: 2 },
];

#[no_mangle]
pub static mut cs35l41_regmap_i2c: regmap_config = regmap_config {
    reg_bits: 32, val_bits: 32, pad_bits: 0, reg_stride: unsafe { CS35L41_REGSTRIDE },
    reg_format_endian: unsafe { REGMAP_ENDIAN_BIG }, val_format_endian: unsafe { REGMAP_ENDIAN_BIG },
    max_register: unsafe { CS35L41_LASTREG }, reg_defaults: cs35l41_reg.as_ptr(),
    num_reg_defaults: cs35l41_reg.len() as c_uint, volatile_reg: Some(cs35l41_volatile_reg),
    readable_reg: Some(cs35l41_readable_reg), precious_reg: Some(cs35l41_precious_reg),
    cache_type: unsafe { REGCACHE_MAPLE },
};

#[no_mangle]
pub static mut cs35l41_regmap_spi: regmap_config = regmap_config {
    reg_bits: 32, val_bits: 32, pad_bits: 16, reg_stride: unsafe { CS35L41_REGSTRIDE },
    reg_format_endian: unsafe { REGMAP_ENDIAN_BIG }, val_format_endian: unsafe { REGMAP_ENDIAN_BIG },
    max_register: unsafe { CS35L41_LASTREG }, reg_defaults: cs35l41_reg.as_ptr(),
    num_reg_defaults: cs35l41_reg.len() as c_uint, volatile_reg: Some(cs35l41_volatile_reg),
    readable_reg: Some(cs35l41_readable_reg), precious_reg: Some(cs35l41_precious_reg),
    cache_type: unsafe { REGCACHE_MAPLE },
};

unsafe fn cs35l41_find_otp_map(otp_id: u32) -> *const cs35l41_otp_map_element_t {
    for i in 0..cs35l41_otp_map_map.len() {
        if cs35l41_otp_map_map[i].id == otp_id {
            return &cs35l41_otp_map_map[i];
        }
    }
    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_test_key_unlock(dev: *mut device, regmap: *mut regmap) -> c_int {
    let unlock = [rs(CS35L41_TEST_KEY_CTL, 0x00000055), rs(CS35L41_TEST_KEY_CTL, 0x000000AA)];
    let ret = regmap_multi_reg_write(regmap, unlock.as_ptr(), unlock.len() as c_int);
    if ret != 0 { dev_err(dev, b"Failed to unlock test key: %d\n\0".as_ptr() as *const c_char, ret); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_test_key_lock(dev: *mut device, regmap: *mut regmap) -> c_int {
    let unlock = [rs(CS35L41_TEST_KEY_CTL, 0x000000CC), rs(CS35L41_TEST_KEY_CTL, 0x00000033)];
    let ret = regmap_multi_reg_write(regmap, unlock.as_ptr(), unlock.len() as c_int);
    if ret != 0 { dev_err(dev, b"Failed to lock test key: %d\n\0".as_ptr() as *const c_char, ret); }
    ret
}

/* Must be called with the TEST_KEY unlocked */
#[no_mangle]
pub unsafe extern "C" fn cs35l41_otp_unpack(dev: *mut device, regmap: *mut regmap) -> c_int {
    let mut ret: c_int;
    let mut bit_sum: c_uint = 8;
    let mut otp_id_reg: u32 = 0;
    let otp_mem = kmalloc_array(CS35L41_OTP_SIZE_WORDS, core::mem::size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if otp_mem.is_null() { return -ENOMEM; }

    ret = regmap_read(regmap, CS35L41_OTPID, &mut otp_id_reg);
    if ret != 0 {
        dev_err(dev, b"Read OTP ID failed: %d\n\0".as_ptr() as *const c_char, ret);
        kfree(otp_mem as *mut c_void);
        return ret;
    }
    let otp_map_match = cs35l41_find_otp_map(otp_id_reg);
    if otp_map_match.is_null() {
        dev_err(dev, b"OTP Map matching ID %d not found\n\0".as_ptr() as *const c_char, otp_id_reg);
        kfree(otp_mem as *mut c_void);
        return -EINVAL;
    }
    ret = regmap_bulk_read(regmap, CS35L41_OTP_MEM0, otp_mem as *mut c_void, CS35L41_OTP_SIZE_WORDS as c_int);
    if ret != 0 {
        dev_err(dev, b"Read OTP Mem failed: %d\n\0".as_ptr() as *const c_char, ret);
        kfree(otp_mem as *mut c_void);
        return ret;
    }

    let otp_map = (*otp_map_match).map;
    let mut bit_offset = (*otp_map_match).bit_offset;
    let mut word_offset = (*otp_map_match).word_offset;
    for i in 0..(*otp_map_match).num_elements as isize {
        let elem = *otp_map.offset(i);
        dev_dbg(dev, b"bitoffset= %d, word_offset=%d, bit_sum mod 32=%d, otp_map[i].size = %u\n\0".as_ptr() as *const c_char,
                bit_offset, word_offset, bit_sum % 32, elem.size);
        let otp_val: u32;
        if bit_offset + elem.size as c_int - 1 >= 32 {
            otp_val = ((*otp_mem.offset(word_offset as isize) & GENMASK(31, bit_offset as c_uint)) >> bit_offset)
                | ((*otp_mem.offset({ word_offset += 1; word_offset } as isize) &
                    GENMASK((bit_offset + elem.size as c_int - 33) as c_uint, 0)) << (32 - bit_offset));
            bit_offset += elem.size as c_int - 32;
        } else if bit_offset + elem.size as c_int - 1 >= 0 {
            otp_val = (*otp_mem.offset(word_offset as isize) &
                GENMASK((bit_offset + elem.size as c_int - 1) as c_uint, bit_offset as c_uint)) >> bit_offset;
            bit_offset += elem.size as c_int;
        } else {
            otp_val = 0;
        }
        bit_sum += elem.size;
        if bit_offset == 32 {
            bit_offset = 0;
            word_offset += 1;
        }
        if elem.reg != 0 {
            ret = regmap_update_bits(regmap, elem.reg, GENMASK(elem.shift + elem.size - 1, elem.shift),
                                     otp_val << elem.shift);
            if ret < 0 {
                dev_err(dev, b"Write OTP val failed: %d\n\0".as_ptr() as *const c_char, ret);
                kfree(otp_mem as *mut c_void);
                return ret;
            }
        }
    }
    kfree(otp_mem as *mut c_void);
    0
}

static cs35l41_reva0_errata_patch: &[reg_sequence] = &[
    rs(0x00003854, 0x05180240), rs(unsafe { CS35L41_VIMON_SPKMON_RESYNC }, 0x00000000),
    rs(0x00004310, 0x00000000), rs(unsafe { CS35L41_VPVBST_FS_SEL }, 0x00000000),
    rs(unsafe { CS35L41_OTP_TRIM_30 }, 0x9091A1C8), rs(0x00003014, 0x0200EE0E),
    rs(unsafe { CS35L41_BSTCVRT_DCM_CTRL }, 0x00000051), rs(0x00000054, 0x00000004),
    rs(unsafe { CS35L41_IRQ1_DB3 }, 0x00000000), rs(unsafe { CS35L41_IRQ2_DB3 }, 0x00000000),
    rs(unsafe { CS35L41_DSP1_YM_ACCEL_PL0_PRI }, 0x00000000), rs(unsafe { CS35L41_DSP1_XM_ACCEL_PL0_PRI }, 0x00000000),
    rs(unsafe { CS35L41_PWR_CTRL2 }, 0x00000000), rs(unsafe { CS35L41_AMP_GAIN_CTRL }, 0x00000000),
    rs(unsafe { CS35L41_ASP_TX3_SRC }, 0x00000000), rs(unsafe { CS35L41_ASP_TX4_SRC }, 0x00000000),
];

static cs35l41_revb0_errata_patch: &[reg_sequence] = &[
    rs(unsafe { CS35L41_VIMON_SPKMON_RESYNC }, 0), rs(0x00004310, 0), rs(unsafe { CS35L41_VPVBST_FS_SEL }, 0),
    rs(unsafe { CS35L41_BSTCVRT_DCM_CTRL }, 0x51), rs(unsafe { CS35L41_DSP1_YM_ACCEL_PL0_PRI }, 0),
    rs(unsafe { CS35L41_DSP1_XM_ACCEL_PL0_PRI }, 0), rs(unsafe { CS35L41_PWR_CTRL2 }, 0),
    rs(unsafe { CS35L41_AMP_GAIN_CTRL }, 0), rs(unsafe { CS35L41_ASP_TX3_SRC }, 0), rs(unsafe { CS35L41_ASP_TX4_SRC }, 0),
];

static cs35l41_revb2_errata_patch: &[reg_sequence] = cs35l41_revb0_errata_patch;

static cs35l41_fs_errata_patch: &[reg_sequence] = &[
    rs(unsafe { CS35L41_DSP1_RX1_RATE }, 1), rs(unsafe { CS35L41_DSP1_RX2_RATE }, 1),
    rs(unsafe { CS35L41_DSP1_RX3_RATE }, 1), rs(unsafe { CS35L41_DSP1_RX4_RATE }, 1),
    rs(unsafe { CS35L41_DSP1_RX5_RATE }, 1), rs(unsafe { CS35L41_DSP1_RX6_RATE }, 1),
    rs(unsafe { CS35L41_DSP1_RX7_RATE }, 1), rs(unsafe { CS35L41_DSP1_RX8_RATE }, 1),
    rs(unsafe { CS35L41_DSP1_TX1_RATE }, 1), rs(unsafe { CS35L41_DSP1_TX2_RATE }, 1),
    rs(unsafe { CS35L41_DSP1_TX3_RATE }, 1), rs(unsafe { CS35L41_DSP1_TX4_RATE }, 1),
    rs(unsafe { CS35L41_DSP1_TX5_RATE }, 1), rs(unsafe { CS35L41_DSP1_TX6_RATE }, 1),
    rs(unsafe { CS35L41_DSP1_TX7_RATE }, 1), rs(unsafe { CS35L41_DSP1_TX8_RATE }, 1),
];

#[no_mangle]
pub unsafe extern "C" fn cs35l41_register_errata_patch(dev: *mut device, reg: *mut regmap, reg_revid: c_uint) -> c_int {
    let (mut ret, rev): (c_int, *const c_char) = if reg_revid == CS35L41_REVID_A0 {
        (regmap_register_patch(reg, cs35l41_reva0_errata_patch.as_ptr(), cs35l41_reva0_errata_patch.len() as c_int), b"A0\0".as_ptr() as *const c_char)
    } else if reg_revid == CS35L41_REVID_B0 {
        (regmap_register_patch(reg, cs35l41_revb0_errata_patch.as_ptr(), cs35l41_revb0_errata_patch.len() as c_int), b"B0\0".as_ptr() as *const c_char)
    } else if reg_revid == CS35L41_REVID_B2 {
        (regmap_register_patch(reg, cs35l41_revb2_errata_patch.as_ptr(), cs35l41_revb2_errata_patch.len() as c_int), b"B2\0".as_ptr() as *const c_char)
    } else { (-EINVAL, b"XX\0".as_ptr() as *const c_char) };
    if ret != 0 { dev_err(dev, b"Failed to apply %s errata patch: %d\n\0".as_ptr() as *const c_char, rev, ret); }
    ret = regmap_write(reg, CS35L41_DSP1_CCM_CORE_CTRL, 0);
    if ret < 0 { dev_err(dev, b"Write CCM_CORE_CTRL failed: %d\n\0".as_ptr() as *const c_char, ret); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_set_channels(dev: *mut device, reg: *mut regmap, tx_num: c_uint, tx_slot: *const c_uint, rx_num: c_uint, rx_slot: *const c_uint) -> c_int {
    if tx_num > 4 || rx_num > 2 { return -EINVAL; }
    let mut val: c_uint = 0;
    let mut mask: c_uint = 0;
    for i in 0..rx_num {
        dev_dbg(dev, b"rx slot %d position = %d\n\0".as_ptr() as *const c_char, i, *rx_slot.add(i as usize));
        val |= *rx_slot.add(i as usize) << (i * 8);
        mask |= 0x3F << (i * 8);
    }
    regmap_update_bits(reg, CS35L41_SP_FRAME_RX_SLOT, mask, val);
    val = 0; mask = 0;
    for i in 0..tx_num {
        dev_dbg(dev, b"tx slot %d position = %d\n\0".as_ptr() as *const c_char, i, *tx_slot.add(i as usize));
        val |= *tx_slot.add(i as usize) << (i * 8);
        mask |= 0x3F << (i * 8);
    }
    regmap_update_bits(reg, CS35L41_SP_FRAME_TX_SLOT, mask, val);
    0
}

static cs35l41_bst_k1_table: [[u8; 5]; 4] = [[0x24, 0x32, 0x32, 0x4F, 0x57], [0x24, 0x32, 0x32, 0x4F, 0x57], [0x40, 0x32, 0x32, 0x4F, 0x57], [0x40, 0x32, 0x32, 0x4F, 0x57]];
static cs35l41_bst_k2_table: [[u8; 5]; 4] = [[0x24, 0x49, 0x66, 0xA3, 0xEA], [0x24, 0x49, 0x66, 0xA3, 0xEA], [0x48, 0x49, 0x66, 0xA3, 0xEA], [0x48, 0x49, 0x66, 0xA3, 0xEA]];
static cs35l41_bst_slope_table: [u8; 4] = [0x75, 0x6B, 0x3B, 0x28];

unsafe fn cs35l41_boost_config(dev: *mut device, regmap: *mut regmap, boost_ind: c_int, boost_cap: c_int, boost_ipk: c_int) -> c_int {
    let bst_lbst_val: usize = match boost_ind {
        1000 => 0, 1200 => 1, 1500 => 2, 2200 => 3,
        _ => { dev_err(dev, b"Invalid boost inductor value: %d nH\n\0".as_ptr() as *const c_char, boost_ind); return -EINVAL; }
    };
    let bst_cbst_range: usize = match boost_cap {
        0..=19 => 0, 20..=50 => 1, 51..=100 => 2, 101..=200 => 3,
        _ if boost_cap < 0 => { dev_err(dev, b"Invalid boost capacitor value: %d nH\n\0".as_ptr() as *const c_char, boost_cap); return -EINVAL; },
        _ => 4,
    };
    if boost_ipk < 1600 || boost_ipk > 4500 {
        dev_err(dev, b"Invalid boost inductor peak current: %d mA\n\0".as_ptr() as *const c_char, boost_ipk);
        return -EINVAL;
    }
    let mut ret = regmap_update_bits(regmap, CS35L41_BSTCVRT_COEFF, CS35L41_BST_K1_MASK | CS35L41_BST_K2_MASK,
        ((cs35l41_bst_k1_table[bst_lbst_val][bst_cbst_range] as c_uint) << CS35L41_BST_K1_SHIFT) |
        ((cs35l41_bst_k2_table[bst_lbst_val][bst_cbst_range] as c_uint) << CS35L41_BST_K2_SHIFT));
    if ret != 0 { dev_err(dev, b"Failed to write boost coefficients: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    ret = regmap_update_bits(regmap, CS35L41_BSTCVRT_SLOPE_LBST, CS35L41_BST_SLOPE_MASK | CS35L41_BST_LBST_VAL_MASK,
        ((cs35l41_bst_slope_table[bst_lbst_val] as c_uint) << CS35L41_BST_SLOPE_SHIFT) |
        ((bst_lbst_val as c_uint) << CS35L41_BST_LBST_VAL_SHIFT));
    if ret != 0 { dev_err(dev, b"Failed to write boost slope/inductor value: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    let bst_ipk_scaled: c_uint = (((boost_ipk - 1600) / 50) + 0x10) as c_uint;
    ret = regmap_update_bits(regmap, CS35L41_BSTCVRT_PEAK_CUR, CS35L41_BST_IPK_MASK, bst_ipk_scaled << CS35L41_BST_IPK_SHIFT);
    if ret != 0 { dev_err(dev, b"Failed to write boost inductor peak current: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
    regmap_update_bits(regmap, CS35L41_PWR_CTRL2, CS35L41_BST_EN_MASK, CS35L41_BST_EN_DEFAULT << CS35L41_BST_EN_SHIFT);
    0
}

static cs35l41_safe_to_reset: &[reg_sequence] = &[rs(0x40, 0x55), rs(0x40, 0xAA), rsd(0x393C, 0xC0, 6000), rs(0x393C, 0), rs(0x7414, 0x00C82222), rs(0x742C, 0), rs(0x40, 0xCC), rs(0x40, 0x33)];
static cs35l41_active_to_safe_start: &[reg_sequence] = &[rs(0x40, 0x55), rs(0x40, 0xAA), rs(0x7438, 0x00585941), rs(unsafe { CS35L41_PWR_CTRL1 }, 0), rs(0x742C, 9)];
static cs35l41_active_to_safe_end: &[reg_sequence] = &[rs(0x7438, 0x00580941), rs(0x40, 0xCC), rs(0x40, 0x33)];
static cs35l41_safe_to_active_start: &[reg_sequence] = &[rs(0x40, 0x55), rs(0x40, 0xAA), rs(0x742C, 0x0F), rs(0x742C, 0x79), rs(0x7438, 0x00585941), rs(unsafe { CS35L41_PWR_CTRL1 }, 1)];
static cs35l41_safe_to_active_en_spk: &[reg_sequence] = &[rs(0x742C, 0xF9), rs(0x7438, 0x00580941)];
static cs35l41_reset_to_safe: &[reg_sequence] = &[rs(0x40, 0x55), rs(0x40, 0xAA), rs(0x7438, 0x00585941), rs(0x7414, 0x08C82222), rs(0x742C, 9), rs(0x40, 0xCC), rs(0x40, 0x33)];
static cs35l41_actv_seq: &[reg_sequence] = &[rs(unsafe { CS35L41_MDSYNC_EN }, 0x3000), rs(unsafe { CS35L41_BSTCVRT_VCTRL2 }, 2)];
static cs35l41_pass_seq: &[reg_sequence] = &[rs(unsafe { CS35L41_MDSYNC_EN }, 0x1000), rs(unsafe { CS35L41_PWR_CTRL2 }, 0x3300), rs(unsafe { CS35L41_BSTCVRT_VCTRL2 }, 2)];

#[no_mangle]
pub unsafe extern "C" fn cs35l41_init_boost(dev: *mut device, regmap: *mut regmap, hw_cfg: *mut cs35l41_hw_cfg) -> c_int {
    let mut ret: c_int = 0;
    if (*hw_cfg).bst_type == CS35L41_SHD_BOOST_ACTV {
        regmap_multi_reg_write(regmap, cs35l41_actv_seq.as_ptr(), cs35l41_actv_seq.len() as c_int);
        ret = cs35l41_boost_config(dev, regmap, (*hw_cfg).bst_ind, (*hw_cfg).bst_cap, (*hw_cfg).bst_ipk);
        if ret != 0 { dev_err(dev, b"Error in Boost DT config: %d\n\0".as_ptr() as *const c_char, ret); }
    } else if (*hw_cfg).bst_type == CS35L41_INT_BOOST {
        ret = cs35l41_boost_config(dev, regmap, (*hw_cfg).bst_ind, (*hw_cfg).bst_cap, (*hw_cfg).bst_ipk);
        if ret != 0 { dev_err(dev, b"Error in Boost DT config: %d\n\0".as_ptr() as *const c_char, ret); }
    } else if (*hw_cfg).bst_type == CS35L41_EXT_BOOST || (*hw_cfg).bst_type == CS35L41_EXT_BOOST_NO_VSPK_SWITCH {
        regmap_write(regmap, CS35L41_GPIO1_CTRL1, 1);
        regmap_multi_reg_write(regmap, cs35l41_reset_to_safe.as_ptr(), cs35l41_reset_to_safe.len() as c_int);
        ret = regmap_update_bits(regmap, CS35L41_PWR_CTRL2, CS35L41_BST_EN_MASK, CS35L41_BST_DIS_FET_OFF << CS35L41_BST_EN_SHIFT);
    } else if (*hw_cfg).bst_type == CS35L41_SHD_BOOST_PASS {
        ret = regmap_multi_reg_write(regmap, cs35l41_pass_seq.as_ptr(), cs35l41_pass_seq.len() as c_int);
    } else {
        dev_err(dev, b"Boost type %d not supported\n\0".as_ptr() as *const c_char, (*hw_cfg).bst_type);
        ret = -EINVAL;
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_safe_reset(regmap: *mut regmap, b_type: cs35l41_boost_type) -> bool {
    if b_type == CS35L41_EXT_BOOST_NO_VSPK_SWITCH { false }
    else if b_type == CS35L41_EXT_BOOST {
        regmap_write(regmap, CS35L41_GPIO1_CTRL1, 1);
        regmap_multi_reg_write(regmap, cs35l41_safe_to_reset.as_ptr(), cs35l41_safe_to_reset.len() as c_int);
        true
    } else { true }
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_global_enable(dev: *mut device, regmap: *mut regmap, b_type: cs35l41_boost_type, enable: c_int, dsp: *mut cs_dsp) -> c_int {
    let pup_pdn_mask = if enable != 0 { CS35L41_PUP_DONE_MASK } else { CS35L41_PDN_DONE_MASK };
    let mut pwr_ctl1_val: c_uint = 0;
    let mut ret = regmap_read(regmap, CS35L41_PWR_CTRL1, &mut pwr_ctl1_val);
    if ret != 0 { return ret; }
    if (pwr_ctl1_val & CS35L41_GLOBAL_EN_MASK) != 0 && enable != 0 {
        dev_dbg(dev, b"Cannot set Global Enable - already set.\n\0".as_ptr() as *const c_char);
        return 0;
    } else if (pwr_ctl1_val & CS35L41_GLOBAL_EN_MASK) == 0 && enable == 0 {
        dev_dbg(dev, b"Cannot unset Global Enable - not set.\n\0".as_ptr() as *const c_char);
        return 0;
    }
    let mut int_status: c_uint = 0;
    if b_type == CS35L41_SHD_BOOST_ACTV || b_type == CS35L41_SHD_BOOST_PASS {
        let mut pwr_ctrl3 = 0;
        let mut pad_control = 0;
        regmap_read(regmap, CS35L41_PWR_CTRL3, &mut pwr_ctrl3);
        regmap_read(regmap, CS35L41_GPIO_PAD_CONTROL, &mut pad_control);
        pwr_ctrl3 &= !CS35L41_SYNC_EN_MASK;
        let pwr_ctrl1 = (enable as c_uint) << CS35L41_GLOBAL_EN_SHIFT;
        let mut gpio1_func = if enable != 0 { CS35L41_GPIO1_MDSYNC } else { CS35L41_GPIO1_HIZ };
        gpio1_func <<= CS35L41_GPIO1_CTRL_SHIFT;
        pad_control &= !CS35L41_GPIO1_CTRL_MASK;
        pad_control |= gpio1_func & CS35L41_GPIO1_CTRL_MASK;
        let seq = [rs(CS35L41_PWR_CTRL3, pwr_ctrl3), rs(CS35L41_GPIO_PAD_CONTROL, pad_control), rsd(CS35L41_PWR_CTRL1, pwr_ctrl1, 3000)];
        ret = regmap_multi_reg_write(regmap, seq.as_ptr(), seq.len() as c_int);
        if ret != 0 || enable != 0 { return ret; }
        ret = regmap_read_poll_timeout(regmap, CS35L41_IRQ1_STATUS1, &mut int_status, pup_pdn_mask, 1000, 100000);
        if ret != 0 { dev_err(dev, b"Enable(%d) failed: %d\n\0".as_ptr() as *const c_char, enable, ret); }
        regmap_write(regmap, CS35L41_IRQ1_STATUS1, pup_pdn_mask);
    } else if b_type == CS35L41_INT_BOOST {
        ret = regmap_update_bits(regmap, CS35L41_PWR_CTRL1, CS35L41_GLOBAL_EN_MASK, (enable as c_uint) << CS35L41_GLOBAL_EN_SHIFT);
        if ret != 0 { dev_err(dev, b"CS35L41_PWR_CTRL1 set failed: %d\n\0".as_ptr() as *const c_char, ret); return ret; }
        ret = regmap_read_poll_timeout(regmap, CS35L41_IRQ1_STATUS1, &mut int_status, pup_pdn_mask, 1000, 100000);
        if ret != 0 { dev_err(dev, b"Enable(%d) failed: %d\n\0".as_ptr() as *const c_char, enable, ret); }
        regmap_write(regmap, CS35L41_IRQ1_STATUS1, pup_pdn_mask);
    } else if b_type == CS35L41_EXT_BOOST || b_type == CS35L41_EXT_BOOST_NO_VSPK_SWITCH {
        if enable != 0 {
            ret = regmap_multi_reg_write(regmap, cs35l41_safe_to_active_start.as_ptr(), cs35l41_safe_to_active_start.len() as c_int);
            if ret != 0 { return ret; }
            ret = regmap_read_poll_timeout(regmap, CS35L41_IRQ1_STATUS1, &mut int_status, CS35L41_PUP_DONE_MASK, 1000, 100000);
            if ret != 0 {
                dev_err(dev, b"Failed waiting for CS35L41_PUP_DONE_MASK: %d\n\0".as_ptr() as *const c_char, ret);
                cs35l41_test_key_lock(dev, regmap);
                return ret;
            }
            regmap_write(regmap, CS35L41_IRQ1_STATUS1, CS35L41_PUP_DONE_MASK);
            if (*dsp).running && (*dsp).fw_id_version > CS35L41_FIRMWARE_OLD_VERSION {
                ret = cs35l41_set_cspl_mbox_cmd(dev, regmap, CSPL_MBOX_CMD_SPK_OUT_ENABLE);
            } else {
                ret = regmap_multi_reg_write(regmap, cs35l41_safe_to_active_en_spk.as_ptr(), cs35l41_safe_to_active_en_spk.len() as c_int);
            }
            cs35l41_test_key_lock(dev, regmap);
        } else {
            ret = regmap_multi_reg_write(regmap, cs35l41_active_to_safe_start.as_ptr(), cs35l41_active_to_safe_start.len() as c_int);
            if ret != 0 { cs35l41_test_key_lock(dev, regmap); return ret; }
            ret = regmap_read_poll_timeout(regmap, CS35L41_IRQ1_STATUS1, &mut int_status, CS35L41_PDN_DONE_MASK, 1000, 100000);
            if ret != 0 {
                dev_err(dev, b"Failed waiting for CS35L41_PDN_DONE_MASK: %d\n\0".as_ptr() as *const c_char, ret);
                cs35l41_test_key_lock(dev, regmap);
                return ret;
            }
            regmap_write(regmap, CS35L41_IRQ1_STATUS1, CS35L41_PDN_DONE_MASK);
            ret = regmap_multi_reg_write(regmap, cs35l41_active_to_safe_end.as_ptr(), cs35l41_active_to_safe_end.len() as c_int);
        }
    } else {
        ret = -EINVAL;
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_mdsync_up(regmap: *mut regmap) -> c_int {
    regmap_update_bits(regmap, CS35L41_PWR_CTRL3, CS35L41_SYNC_EN_MASK, CS35L41_SYNC_EN_MASK)
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_gpio_config(regmap: *mut regmap, hw_cfg: *mut cs35l41_hw_cfg) -> c_int {
    let gpio1 = &mut (*hw_cfg).gpio1;
    let gpio2 = &mut (*hw_cfg).gpio2;
    let mut irq_pol = IRQF_TRIGGER_NONE;
    regmap_update_bits(regmap, CS35L41_GPIO1_CTRL1, CS35L41_GPIO_POL_MASK | CS35L41_GPIO_DIR_MASK,
        (gpio1.pol_inv << CS35L41_GPIO_POL_SHIFT) | ((!gpio1.out_en) << CS35L41_GPIO_DIR_SHIFT));
    regmap_update_bits(regmap, CS35L41_GPIO2_CTRL1, CS35L41_GPIO_POL_MASK | CS35L41_GPIO_DIR_MASK,
        (gpio2.pol_inv << CS35L41_GPIO_POL_SHIFT) | ((!gpio2.out_en) << CS35L41_GPIO_DIR_SHIFT));
    if gpio1.valid {
        regmap_update_bits(regmap, CS35L41_GPIO_PAD_CONTROL, CS35L41_GPIO1_CTRL_MASK, gpio1.func << CS35L41_GPIO1_CTRL_SHIFT);
    }
    if gpio2.valid {
        regmap_update_bits(regmap, CS35L41_GPIO_PAD_CONTROL, CS35L41_GPIO2_CTRL_MASK, gpio2.func << CS35L41_GPIO2_CTRL_SHIFT);
        if gpio2.func == CS35L41_GPIO2_INT_PUSH_PULL_LOW || gpio2.func == CS35L41_GPIO2_INT_OPEN_DRAIN {
            irq_pol = IRQF_TRIGGER_LOW;
        } else if gpio2.func == CS35L41_GPIO2_INT_PUSH_PULL_HIGH {
            irq_pol = IRQF_TRIGGER_HIGH;
        }
    }
    irq_pol
}

static cs35l41_dsp1_regions: &[cs_dsp_region] = &[
    cs_dsp_region { type_: unsafe { WMFW_HALO_PM_PACKED }, base: unsafe { CS35L41_DSP1_PMEM_0 } },
    cs_dsp_region { type_: unsafe { WMFW_HALO_XM_PACKED }, base: unsafe { CS35L41_DSP1_XMEM_PACK_0 } },
    cs_dsp_region { type_: unsafe { WMFW_HALO_YM_PACKED }, base: unsafe { CS35L41_DSP1_YMEM_PACK_0 } },
    cs_dsp_region { type_: unsafe { WMFW_ADSP2_XM }, base: unsafe { CS35L41_DSP1_XMEM_UNPACK24_0 } },
    cs_dsp_region { type_: unsafe { WMFW_ADSP2_YM }, base: unsafe { CS35L41_DSP1_YMEM_UNPACK24_0 } },
];

extern "C" {
    static CS35L41_DSP1_XMEM_UNPACK24_0: c_uint;
    static CS35L41_DSP1_YMEM_UNPACK24_0: c_uint;
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_configure_cs_dsp(dev: *mut device, reg: *mut regmap, dsp: *mut cs_dsp) {
    (*dsp).num = 1;
    (*dsp).type_ = WMFW_HALO;
    (*dsp).rev = 0;
    (*dsp).dev = dev;
    (*dsp).regmap = reg;
    (*dsp).base = CS35L41_DSP1_CTRL_BASE;
    (*dsp).base_sysinfo = CS35L41_DSP1_SYS_ID;
    (*dsp).mem = cs35l41_dsp1_regions.as_ptr();
    (*dsp).num_mems = cs35l41_dsp1_regions.len() as c_uint;
    (*dsp).lock_regions = 0xFFFFFFFF;
}

unsafe fn cs35l41_check_cspl_mbox_sts(cmd: cs35l41_cspl_mbox_cmd, sts: cs35l41_cspl_mbox_status) -> bool {
    if cmd == CSPL_MBOX_CMD_NONE || cmd == CSPL_MBOX_CMD_UNKNOWN_CMD { true }
    else if cmd == CSPL_MBOX_CMD_PAUSE || cmd == CSPL_MBOX_CMD_OUT_OF_HIBERNATE { sts == CSPL_MBOX_STS_PAUSED }
    else if cmd == CSPL_MBOX_CMD_RESUME { sts == CSPL_MBOX_STS_RUNNING }
    else if cmd == CSPL_MBOX_CMD_REINIT { sts == CSPL_MBOX_STS_RUNNING }
    else if cmd == CSPL_MBOX_CMD_STOP_PRE_REINIT { sts == CSPL_MBOX_STS_RDY_FOR_REINIT }
    else if cmd == CSPL_MBOX_CMD_SPK_OUT_ENABLE { sts == CSPL_MBOX_STS_RUNNING }
    else { false }
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_set_cspl_mbox_cmd(dev: *mut device, regmap: *mut regmap, cmd: cs35l41_cspl_mbox_cmd) -> c_int {
    let mut sts: c_uint = 0;
    let mut ret = regmap_write(regmap, CS35L41_DSP_VIRT1_MBOX_1, cmd);
    if ret < 0 {
        if cmd != CSPL_MBOX_CMD_OUT_OF_HIBERNATE { dev_err(dev, b"Failed to write MBOX: %d\n\0".as_ptr() as *const c_char, ret); }
        return ret;
    }
    for i in 0..5u32 {
        usleep_range(1000, 1100);
        ret = regmap_read(regmap, CS35L41_DSP_MBOX_2, &mut sts);
        if ret < 0 {
            dev_err(dev, b"Failed to read MBOX STS: %d\n\0".as_ptr() as *const c_char, ret);
            continue;
        }
        if sts == CSPL_MBOX_STS_ERROR || sts == CSPL_MBOX_STS_ERROR2 {
            dev_err(dev, b"CSPL Error Detected\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
        if !cs35l41_check_cspl_mbox_sts(cmd, sts) {
            dev_dbg(dev, b"[%u] cmd %u returned invalid sts %u\0".as_ptr() as *const c_char, i, cmd, sts);
        } else {
            return 0;
        }
    }
    if cmd != CSPL_MBOX_CMD_OUT_OF_HIBERNATE {
        dev_err(dev, b"Failed to set mailbox cmd %u (status %u)\n\0".as_ptr() as *const c_char, cmd, sts);
    }
    -ENOMSG
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_write_fs_errata(dev: *mut device, regmap: *mut regmap) -> c_int {
    let ret = regmap_multi_reg_write(regmap, cs35l41_fs_errata_patch.as_ptr(), cs35l41_fs_errata_patch.len() as c_int);
    if ret < 0 { dev_err(dev, b"Failed to write fs errata: %d\n\0".as_ptr() as *const c_char, ret); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_enter_hibernate(dev: *mut device, regmap: *mut regmap, b_type: cs35l41_boost_type) -> c_int {
    if !cs35l41_safe_reset(regmap, b_type) {
        dev_dbg(dev, b"System does not support Suspend\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    dev_dbg(dev, b"Enter hibernate\n\0".as_ptr() as *const c_char);
    regmap_write(regmap, CS35L41_WAKESRC_CTL, 0x0088);
    regmap_write(regmap, CS35L41_WAKESRC_CTL, 0x0188);
    regmap_write(regmap, CS35L41_DSP_VIRT1_MBOX_1, CSPL_MBOX_CMD_HIBERNATE);
    0
}

unsafe fn cs35l41_wait_for_pwrmgt_sts(dev: *mut device, regmap: *mut regmap) {
    let pwrmgt_retries = 10;
    let mut sts: c_uint = 0;
    for _i in 0..pwrmgt_retries {
        let ret = regmap_read(regmap, CS35L41_PWRMGT_STS, &mut sts);
        if ret != 0 {
            dev_err(dev, b"Failed to read PWRMGT_STS: %d\n\0".as_ptr() as *const c_char, ret);
        } else if (sts & CS35L41_WR_PEND_STS_MASK) == 0 {
            return;
        }
        udelay(20);
    }
    dev_err(dev, b"Timed out reading PWRMGT_STS\n\0".as_ptr() as *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn cs35l41_exit_hibernate(dev: *mut device, regmap: *mut regmap) -> c_int {
    let wake_retries = 20;
    let sleep_retries = 5;
    let mut ret: c_int = 0;
    for _i in 0..sleep_retries {
        dev_dbg(dev, b"Exit hibernate\n\0".as_ptr() as *const c_char);
        let mut j = 0;
        while j < wake_retries {
            ret = cs35l41_set_cspl_mbox_cmd(dev, regmap, CSPL_MBOX_CMD_OUT_OF_HIBERNATE);
            if ret == 0 { break; }
            usleep_range(100, 200);
            j += 1;
        }
        if j < wake_retries {
            dev_dbg(dev, b"Wake success at cycle: %d\n\0".as_ptr() as *const c_char, j);
            return 0;
        }
        dev_err(dev, b"Wake failed, re-enter hibernate: %d\n\0".as_ptr() as *const c_char, ret);
        cs35l41_wait_for_pwrmgt_sts(dev, regmap);
        regmap_write(regmap, CS35L41_WAKESRC_CTL, 0x0088);
        cs35l41_wait_for_pwrmgt_sts(dev, regmap);
        regmap_write(regmap, CS35L41_WAKESRC_CTL, 0x0188);
        cs35l41_wait_for_pwrmgt_sts(dev, regmap);
        regmap_write(regmap, CS35L41_PWRMGT_CTL, 0x3);
    }
    dev_err(dev, b"Timed out waking device\n\0".as_ptr() as *const c_char);
    -ETIMEDOUT
}

// MODULE_DESCRIPTION("CS35L41 library");
// MODULE_AUTHOR("David Rhodes, Cirrus Logic Inc, <david.rhodes@cirrus.com>");
// MODULE_AUTHOR("Lucas Tanure, Cirrus Logic Inc, <tanureal@opensource.cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
