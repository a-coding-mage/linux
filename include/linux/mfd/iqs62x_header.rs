/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Azoteq IQS620A/621/622/624/625 Multi-Function Sensors
 *
 * Copyright (C) 2019 Jeff LaBundy <jeff@labundy.com>
 */

// C header dependency: BIT(n) is represented by the corresponding shift.

pub const IQS620_PROD_NUM: u8 = 0x41;
pub const IQS621_PROD_NUM: u8 = 0x46;
pub const IQS622_PROD_NUM: u8 = 0x42;
pub const IQS624_PROD_NUM: u8 = 0x43;
pub const IQS625_PROD_NUM: u8 = 0x4E;

pub const IQS620_HW_NUM_V0: u8 = 0x82;
pub const IQS620_HW_NUM_V1: u8 = IQS620_HW_NUM_V0;
pub const IQS620_HW_NUM_V2: u8 = IQS620_HW_NUM_V0;
pub const IQS620_HW_NUM_V3: u8 = 0x92;

pub const IQS621_ALS_FLAGS: u8 = 0x16;
pub const IQS622_ALS_FLAGS: u8 = 0x14;

pub const IQS624_HALL_UI: u8 = 0x70;
pub const IQS624_HALL_UI_WHL_EVENT: u8 = 1 << 4;
pub const IQS624_HALL_UI_INT_EVENT: u8 = 1 << 3;
pub const IQS624_HALL_UI_AUTO_CAL: u8 = 1 << 2;

pub const IQS624_INTERVAL_DIV: u8 = 0x7D;

pub const IQS620_GLBL_EVENT_MASK: u8 = 0xD7;
pub const IQS620_GLBL_EVENT_MASK_PMU: u8 = 1 << 6;

pub const IQS62X_NUM_KEYS: usize = 16;
pub const IQS62X_NUM_EVENTS: usize = IQS62X_NUM_KEYS + 6;
pub const IQS62X_EVENT_SIZE: usize = 10;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum iqs62x_ui_sel {
    IQS62X_UI_PROX,
    IQS62X_UI_SAR1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum iqs62x_event_reg {
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_SYS,
    IQS62X_EVENT_PROX,
    IQS62X_EVENT_HYST,
    IQS62X_EVENT_HALL,
    IQS62X_EVENT_ALS,
    IQS62X_EVENT_IR,
    IQS62X_EVENT_WHEEL,
    IQS62X_EVENT_INTER,
    IQS62X_EVENT_UI_LO,
    IQS62X_EVENT_UI_HI,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum iqs62x_event_flag {
    // keys
    IQS62X_EVENT_PROX_CH0_T,
    IQS62X_EVENT_PROX_CH0_P,
    IQS62X_EVENT_PROX_CH1_T,
    IQS62X_EVENT_PROX_CH1_P,
    IQS62X_EVENT_PROX_CH2_T,
    IQS62X_EVENT_PROX_CH2_P,
    IQS62X_EVENT_HYST_POS_T,
    IQS62X_EVENT_HYST_POS_P,
    IQS62X_EVENT_HYST_NEG_T,
    IQS62X_EVENT_HYST_NEG_P,
    IQS62X_EVENT_SAR1_ACT,
    IQS62X_EVENT_SAR1_QRD,
    IQS62X_EVENT_SAR1_MOVE,
    IQS62X_EVENT_SAR1_HALT,
    IQS62X_EVENT_WHEEL_UP,
    IQS62X_EVENT_WHEEL_DN,
    // switches
    IQS62X_EVENT_HALL_N_T,
    IQS62X_EVENT_HALL_N_P,
    IQS62X_EVENT_HALL_S_T,
    IQS62X_EVENT_HALL_S_P,
    // everything else
    IQS62X_EVENT_SYS_RESET,
    IQS62X_EVENT_SYS_ATI,
}

#[repr(C)]
pub struct iqs62x_event_data {
    pub ui_data: u16,
    pub als_flags: u8,
    pub ir_flags: u8,
    pub interval: u8,
}

#[repr(C)]
pub struct iqs62x_event_desc {
    pub reg: iqs62x_event_reg,
    pub mask: u8,
    pub val: u8,
}

#[repr(C)]
pub struct iqs62x_dev_desc {
    pub dev_name: *const core::ffi::c_char,
    pub sub_devs: *const mfd_cell,
    pub num_sub_devs: core::ffi::c_int,
    pub prod_num: u8,
    pub sw_num: u8,
    pub cal_regs: *const u8,
    pub num_cal_regs: core::ffi::c_int,
    pub prox_mask: u8,
    pub sar_mask: u8,
    pub hall_mask: u8,
    pub hyst_mask: u8,
    pub temp_mask: u8,
    pub als_mask: u8,
    pub ir_mask: u8,
    pub prox_settings: u8,
    pub als_flags: u8,
    pub hall_flags: u8,
    pub hyst_shift: u8,
    pub interval: u8,
    pub interval_div: u8,
    pub fw_name: *const core::ffi::c_char,
    pub event_regs: *const [iqs62x_event_reg; IQS62X_EVENT_SIZE],
}

#[repr(C)]
pub struct iqs62x_core {
    pub dev_desc: *const iqs62x_dev_desc,
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub nh: blocking_notifier_head,
    pub fw_blk_head: list_head,
    pub ati_done: completion,
    pub fw_done: completion,
    pub ui_sel: iqs62x_ui_sel,
    pub event_cache: core::ffi::c_ulong,
    pub sw_num: u8,
    pub hw_num: u8,
}

extern "C" {
    pub static iqs62x_events: [iqs62x_event_desc; IQS62X_NUM_EVENTS];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
