/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt766-sdca.h -- RT766 SDCA ALSA SoC audio driver header
 *
 * Copyright(c) 2026 Realtek Semiconductor Corp.
 */

/* Dependencies in the original C header:
 * linux/hid.h, linux/pm.h, linux/regmap.h, linux/soundwire/sdw.h,
 * linux/soundwire/sdw_type.h, sound/soc.h, linux/workqueue.h
 */

pub const RT766_VERSION_ID: u32 = 0xc404;
pub const RT766_DEV_ID1: u32 = 0xc405;
pub const RT766_DEV_ID0: u32 = 0xc406;
pub const RT766_BOND_LATCH_ID: u32 = 0xc407;

pub const RT766_HP_POWER_STATE: u32 = 0x1000004;
pub const RT766_HP_FSM_CTL2_1: u32 = 0x100000d;

/* MCU Patch address */
pub const RT766_MCU_PATCH_ADDR1_START: u32 = 0x10010000;
pub const RT766_MCU_PATCH_ADDR1_END: u32 = 0x10011fff;
pub const RT766_MCU_PATCH_ADDR2_START: u32 = 0x10020000;
pub const RT766_MCU_PATCH_ADDR2_END: u32 = 0x10023fff;

/* Buffer address for HID */
pub const RT766_BUF_ADDR_HID1: u32 = 0x44030000;
pub const RT766_BUF_ADDR_HID2: u32 = 0x44030020;

/* SDCA (Channel) */
pub const RT766_CH_1: u32 = 0x01;
pub const RT766_CH_2: u32 = 0x02;
pub const RT766_CH_3: u32 = 0x03;
pub const RT766_CH_4: u32 = 0x04;

/* RT766 SDCA Control - function number */
pub const RT766_FUNC_NUM_UAJ: u32 = 0x01;
pub const RT766_FUNC_NUM_MIC: u32 = 0x02;
pub const RT766_FUNC_NUM_HID: u32 = 0x03;
pub const RT766_FUNC_NUM_AMP: u32 = 0x04;

/* RT766 SDCA entity */
pub const RT766_SDCA_ENT_0: u32 = 0x00;
pub const RT766_SDCA_ENT_HID101: u32 = 0x01;
pub const RT766_SDCA_ENT_GE49: u32 = 0x49;
pub const RT766_SDCA_ENT_USER_FU41: u32 = 0x05;
pub const RT766_SDCA_ENT_USER_FU36: u32 = 0x0f;
pub const RT766_SDCA_ENT_USER_FU21: u32 = 0x03;
pub const RT766_SDCA_ENT_USER_FU113: u32 = 0x30;
pub const RT766_SDCA_ENT_PDE23: u32 = 0x33;
pub const RT766_SDCA_ENT_PDE47: u32 = 0x28;
pub const RT766_SDCA_ENT_PDE11: u32 = 0x2a;
pub const RT766_SDCA_ENT_PDE34: u32 = 0x29;
pub const RT766_SDCA_ENT_CS41: u32 = 0x01;
pub const RT766_SDCA_ENT_CS36: u32 = 0x11;
pub const RT766_SDCA_ENT_CS113: u32 = 0x12;
pub const RT766_SDCA_ENT_CS21: u32 = 0x21;
pub const RT766_SDCA_ENT_PLATFORM_FU33: u32 = 0x44;
pub const RT766_SDCA_ENT_PPU21: u32 = 0x04;

/* sample frequency index */
pub const RT766_SDCA_RATE_44100HZ: u32 = 0x08;
pub const RT766_SDCA_RATE_48000HZ: u32 = 0x09;
pub const RT766_SDCA_RATE_96000HZ: u32 = 0x0b;
pub const RT766_SDCA_RATE_192000HZ: u32 = 0x0d;

/* SDCA Register macros.
 * The C versions paste token suffixes, e.g. RT766_FUNC_NUM_##func.
 * These Rust translations take the resolved numeric constants directly.
 */
#[inline]
pub unsafe fn RT766_MUTE_REG(func: u32, fu: u32, ch: u32) -> u32 {
    SDW_SDCA_CTL(func, fu, SDCA_CTL_FU_MUTE, ch)
}

#[inline]
pub unsafe fn RT766_VOLUME_REG(func: u32, fu: u32, ch: u32) -> u32 {
    SDW_SDCA_CTL(func, fu, SDCA_CTL_FU_CHANNEL_VOLUME, ch)
}

#[inline]
pub unsafe fn RT766_GAIN_REG(func: u32, fu: u32, ch: u32) -> u32 {
    SDW_SDCA_CTL(func, fu, SDCA_CTL_FU_GAIN, ch)
}

#[inline]
pub unsafe fn RT766_PDE_REQ_REG(func: u32, pde: u32) -> u32 {
    SDW_SDCA_CTL(func, pde, SDCA_CTL_PDE_REQUESTED_PS, 0)
}

#[inline]
pub unsafe fn RT766_PDE_ACTUAL_REG(func: u32, pde: u32) -> u32 {
    SDW_SDCA_CTL(func, pde, SDCA_CTL_PDE_ACTUAL_PS, 0)
}

#[inline]
pub unsafe fn RT766_FUNC_STATUS_REG(func: u32) -> u32 {
    SDW_SDCA_CTL(func, RT766_SDCA_ENT_0, SDCA_CTL_ENTITY_0_FUNCTION_STATUS, 0)
}

#[inline]
pub unsafe fn RT766_SDCA_CTL(func: u32, ent: u32, ctl: u32) -> u32 {
    SDW_SDCA_CTL(func, ent, ctl, 0)
}

pub const RT766_AIF1: u32 = 0;
pub const RT766_AIF2: u32 = 1;
pub const RT766_AIF3: u32 = 2;

pub const RT766_DAI_UAJ: u32 = 0;
pub const RT766_DAI_AMP: u32 = 1;
pub const RT766_DAI_MIC: u32 = 2;

#[repr(C)]
pub struct rt766_sdca_priv {
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub slave: *mut sdw_slave,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub hs_jack: *mut snd_soc_jack,
    pub disable_irq_lock: mutex, /* SDCA irq lock protection */
    pub disable_irq: bool,
    pub jack_type: core::ffi::c_int,
    pub fu41_dapm_mute: bool,
    pub fu41_mixer_l_mute: bool,
    pub fu41_mixer_r_mute: bool,
    pub fu113_dapm_mute: bool,
    pub fu113_mixer_mute: [bool; 4],
    pub fu21_dapm_mute: bool,
    pub fu21_mixer_l_mute: bool,
    pub fu21_mixer_r_mute: bool,
    pub fu36_dapm_mute: bool,
    pub fu36_mixer_l_mute: bool,
    pub fu36_mixer_r_mute: bool,
    pub uaj_func_data: *mut sdca_function_data,
    pub sm_func_data: *mut sdca_function_data,
    pub sa_func_data: *mut sdca_function_data,
    pub hid_func_data: *mut sdca_function_data,
    pub irq_info: *mut sdca_interrupt_info,
    pub hid: *mut hid_device,
}

unsafe extern "C" {
    pub fn rt766_sdca_io_init(dev: *mut device, slave: *mut sdw_slave) -> core::ffi::c_int;
    pub fn rt766_sdca_init(
        dev: *mut device,
        regmap: *mut regmap,
        slave: *mut sdw_slave,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
