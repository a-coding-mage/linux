/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2022 Renesas Electronics Corporation */

/* Linux kernel dependencies are supplied by the surrounding translation. */
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;

pub const RZ_MTU3_TSTRA: u16 = 0x080;
pub const RZ_MTU3_TSTRB: u16 = 0x880;

pub const RZ_MTU3_TDDRA: u16 = 0x016;
pub const RZ_MTU3_TDDRB: u16 = 0x816;
pub const RZ_MTU3_TCDRA: u16 = 0x014;
pub const RZ_MTU3_TCDRB: u16 = 0x814;
pub const RZ_MTU3_TCBRA: u16 = 0x022;
pub const RZ_MTU3_TCBRB: u16 = 0x822;
pub const RZ_MTU3_TCNTSA: u16 = 0x020;
pub const RZ_MTU3_TCNTSB: u16 = 0x820;

pub const RZ_MTU3_TIER: u16 = 0;
pub const RZ_MTU3_NFCR: u16 = 1;
pub const RZ_MTU3_TSR: u16 = 2;
pub const RZ_MTU3_TCR: u16 = 3;
pub const RZ_MTU3_TCR2: u16 = 4;
pub const RZ_MTU3_TMDR1: u16 = 5;
pub const RZ_MTU3_TMDR1_MD: u8 = 0x0f;
pub const RZ_MTU3_TMDR1_MD_NORMAL: u8 = 0;
pub const RZ_MTU3_TMDR1_MD_PWMMODE1: u8 = 2;
pub const RZ_MTU3_TIOR: u16 = 6;
pub const RZ_MTU3_TIORH: u16 = 6;
pub const RZ_MTU3_TIORL: u16 = 7;
pub const RZ_MTU3_TBTM: u16 = 8;

pub const RZ_MTU3_TSTR: u16 = 2;
pub const RZ_MTU3_TCNTCMPCLR: u16 = 3;
pub const RZ_MTU3_TCRU: u16 = 4;
pub const RZ_MTU3_TCR2U: u16 = 5;
pub const RZ_MTU3_TIORU: u16 = 6;
pub const RZ_MTU3_TCRV: u16 = 7;
pub const RZ_MTU3_TCR2V: u16 = 8;
pub const RZ_MTU3_TIORV: u16 = 9;
pub const RZ_MTU3_TCRW: u16 = 10;
pub const RZ_MTU3_TCR2W: u16 = 11;
pub const RZ_MTU3_TIORW: u16 = 12;

pub const RZ_MTU3_TCNT: u16 = 0;
pub const RZ_MTU3_TGRA: u16 = 1;
pub const RZ_MTU3_TGRB: u16 = 2;
pub const RZ_MTU3_TGRC: u16 = 3;
pub const RZ_MTU3_TGRD: u16 = 4;
pub const RZ_MTU3_TGRE: u16 = 5;
pub const RZ_MTU3_TGRF: u16 = 6;
pub const RZ_MTU3_TADCR: u16 = 7;
pub const RZ_MTU3_TADCORA: u16 = 8;
pub const RZ_MTU3_TADCORB: u16 = 9;
pub const RZ_MTU3_TADCOBRA: u16 = 10;
pub const RZ_MTU3_TADCOBRB: u16 = 11;

pub const RZ_MTU3_TCNTU: u16 = 0;
pub const RZ_MTU3_TGRU: u16 = 1;
pub const RZ_MTU3_TCNTV: u16 = 2;
pub const RZ_MTU3_TGRV: u16 = 3;
pub const RZ_MTU3_TCNTW: u16 = 4;
pub const RZ_MTU3_TGRW: u16 = 5;

pub const RZ_MTU3_TCNTLW: u16 = 0;
pub const RZ_MTU3_TGRALW: u16 = 1;
pub const RZ_MTU3_TGRBLW: u16 = 2;
pub const RZ_MTU3_TMDR3: u16 = 0x191;

pub const RZ_MTU3_TCR_CCLR: u8 = 0xe0;
pub const RZ_MTU3_TCR_CKEG: u8 = 0x18;
pub const RZ_MTU3_TCR_TPCS: u8 = 0x07;
pub const RZ_MTU3_TCR_CCLR_TGRA: u8 = 1 << 5;
pub const RZ_MTU3_TCR_CCLR_TGRC: u8 = 5 << 5;
pub const RZ_MTU3_TCR_CKEG_RISING: u8 = 0;
pub const RZ_MTU3_TIOR_IOB: u8 = 0xf0;
pub const RZ_MTU3_TIOR_IOA: u8 = 0x0f;
pub const RZ_MTU3_TIOR_OC_RETAIN: u8 = 0;
pub const RZ_MTU3_TIOR_OC_INIT_OUT_LO_HI_OUT: u8 = 2;
pub const RZ_MTU3_TIOR_OC_INIT_OUT_HI_TOGGLE_OUT: u8 = 7;
pub const RZ_MTU3_TIOR_OC_IOA_H_COMP_MATCH: u8 = 2;
pub const RZ_MTU3_TIOR_OC_IOB_TOGGLE: u8 = 7 << 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rz_mtu3_channels {
    RZ_MTU3_CHAN_0,
    RZ_MTU3_CHAN_1,
    RZ_MTU3_CHAN_2,
    RZ_MTU3_CHAN_3,
    RZ_MTU3_CHAN_4,
    RZ_MTU3_CHAN_5,
    RZ_MTU3_CHAN_6,
    RZ_MTU3_CHAN_7,
    RZ_MTU3_CHAN_8,
    RZ_MTU_NUM_CHANNELS,
}

#[repr(C)]
pub struct rz_mtu3_channel {
    pub dev: *mut device,
    pub channel_number: ::core::ffi::c_uint,
    pub lock: mutex,
    pub is_busy: bool,
}

#[repr(C)]
pub struct rz_mtu3 {
    pub clk: *mut clk,
    pub channels: [rz_mtu3_channel; 9],
    pub priv_data: *mut ::core::ffi::c_void,
}

extern "C" {
    pub fn mutex_lock(lock: *mut mutex);
    pub fn mutex_unlock(lock: *mut mutex);
}

#[inline]
pub unsafe fn rz_mtu3_request_channel(ch: *mut rz_mtu3_channel) -> bool {
    mutex_lock(&mut (*ch).lock);
    if (*ch).is_busy {
        mutex_unlock(&mut (*ch).lock);
        return false;
    }
    (*ch).is_busy = true;
    mutex_unlock(&mut (*ch).lock);
    true
}

#[inline]
pub unsafe fn rz_mtu3_release_channel(ch: *mut rz_mtu3_channel) {
    mutex_lock(&mut (*ch).lock);
    (*ch).is_busy = false;
    mutex_unlock(&mut (*ch).lock);
}

extern "C" {
    pub fn rz_mtu3_is_enabled(ch: *mut rz_mtu3_channel) -> bool;
    pub fn rz_mtu3_disable(ch: *mut rz_mtu3_channel);
    pub fn rz_mtu3_enable(ch: *mut rz_mtu3_channel) -> ::core::ffi::c_int;
    pub fn rz_mtu3_8bit_ch_read(ch: *mut rz_mtu3_channel, off: u16) -> u8;
    pub fn rz_mtu3_16bit_ch_read(ch: *mut rz_mtu3_channel, off: u16) -> u16;
    pub fn rz_mtu3_32bit_ch_read(ch: *mut rz_mtu3_channel, off: u16) -> u32;
    pub fn rz_mtu3_shared_reg_read(ch: *mut rz_mtu3_channel, off: u16) -> u16;
    pub fn rz_mtu3_8bit_ch_write(ch: *mut rz_mtu3_channel, off: u16, val: u8);
    pub fn rz_mtu3_16bit_ch_write(ch: *mut rz_mtu3_channel, off: u16, val: u16);
    pub fn rz_mtu3_32bit_ch_write(ch: *mut rz_mtu3_channel, off: u16, val: u32);
    pub fn rz_mtu3_shared_reg_write(ch: *mut rz_mtu3_channel, off: u16, val: u16);
    pub fn rz_mtu3_shared_reg_update_bit(ch: *mut rz_mtu3_channel, off: u16, pos: u16, val: u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
