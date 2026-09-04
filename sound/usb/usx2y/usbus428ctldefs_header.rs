// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2003 by Karsten Wiese <annabellesgarden@yahoo.de>

#[repr(u32)]
pub enum E_IN84 {
    E_FADER_0 = 0,
    E_FADER_1 = 1,
    E_FADER_2 = 2,
    E_FADER_3 = 3,
    E_FADER_4 = 4,
    E_FADER_5 = 5,
    E_FADER_6 = 6,
    E_FADER_7 = 7,
    E_FADER_M = 8,
    E_TRANSPORT = 9,
    E_MODIFIER = 10,
    E_FILTER_SELECT = 11,
    E_SELECT = 12,
    E_MUTE = 13,
    E_SWITCH = 15,
    E_WHEEL_GAIN = 16,
    E_WHEEL_FREQ = 17,
    E_WHEEL_Q = 18,
    E_WHEEL_PAN = 19,
    E_WHEEL = 20,
}

pub const T_RECORD: u8 = 1;
pub const T_PLAY: u8 = 2;
pub const T_STOP: u8 = 4;
pub const T_F_FWD: u8 = 8;
pub const T_REW: u8 = 0x10;
pub const T_SOLO: u8 = 0x20;
pub const T_REC: u8 = 0x40;
pub const T_NULL: u8 = 0x80;

#[repr(C)]
pub struct us428_ctls {
    pub fader: [u8; 9],
    pub transport: u8,
    pub modifier: u8,
    pub filters_elect: u8,
    pub select: u8,
    pub mute: u8,
    pub unknown: u8,
    pub wswitch: u8,
    pub wheel: [u8; 5],
}

#[repr(C)]
pub struct us428_set_byte {
    pub offset: u8,
    pub value: u8,
}

pub const ELT_VOLUME: u32 = 0;
pub const ELT_LIGHT: u32 = 1;

#[repr(C)]
pub struct usx2y_volume {
    pub channel: u8,
    pub lh: u8,
    pub ll: u8,
    pub rh: u8,
    pub rl: u8,
}

#[repr(C)]
pub struct us428_lights {
    pub light: [us428_set_byte; 7],
}

#[repr(C)]
pub union us428_p4out_val {
    pub vol: usx2y_volume,
    pub lights: us428_lights,
}

#[repr(C)]
pub struct us428_p4out {
    pub r#type: i8,
    pub val: us428_p4out_val,
}

pub const N_US428_CTL_BUFS: usize = 16;
pub const N_US428_P4OUT_BUFS: usize = 16;

#[repr(C)]
pub struct us428ctls_sharedmem {
    pub ctl_snapshot: [us428_ctls; N_US428_CTL_BUFS],
    pub ctl_snapshot_differs_at: [i32; N_US428_CTL_BUFS],
    pub ctl_snapshot_last: i32,
    pub ctl_snapshot_red: i32,
    pub p4out: [us428_p4out; N_US428_P4OUT_BUFS],
    pub p4out_last: i32,
    pub p4out_sent: i32,
}

// PAGE_ALIGN(sizeof(struct us428ctls_sharedmem)) — macro from external build environment
// pub const US428_SHAREDMEM_PAGES: usize = PAGE_ALIGN(std::mem::size_of::<us428ctls_sharedmem>());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
