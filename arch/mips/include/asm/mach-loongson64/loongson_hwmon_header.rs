/* SPDX-License-Identifier: GPL-2.0 */

// The Linux type and delayed-work dependencies are supplied by other files.

pub const MIN_TEMP: i32 = 0;
pub const MAX_TEMP: i32 = 255;
pub const NOT_VALID_TEMP: i32 = 999;

pub type GetTempFun = Option<unsafe extern "C" fn(i32) -> i32>;

unsafe extern "C" {
    pub fn loongson3_cpu_temp(_: i32) -> i32;
}

/* 0:Max speed, 1:Manual, 2:Auto */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum fan_control_mode {
    FAN_FULL_MODE = 0,
    FAN_MANUAL_MODE = 1,
    FAN_AUTO_MODE = 2,
    FAN_MODE_END,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct temp_range {
    pub low: u8,
    pub high: u8,
    pub level: u8,
}

pub const CONSTANT_SPEED_POLICY: i32 = 0; // at constant speed
pub const STEP_SPEED_POLICY: i32 = 1; // use up/down arrays to describe policy
pub const KERNEL_HELPER_POLICY: i32 = 2; // kernel as a helper to fan control

pub const MAX_STEP_NUM: usize = 16;
pub const MAX_FAN_LEVEL: i32 = 255;

/* loongson_fan_policy works when fan work at FAN_AUTO_MODE */
#[repr(C)]
pub struct loongson_fan_policy {
    pub type_: u8,

    /* percent only used when type is CONSTANT_SPEED_POLICY */
    pub percent: u8,

    /* period between two check. (Unit: S) */
    pub adjust_period: u8,

    /* fan adjust usually depend on a temperature input */
    pub depend_temp: GetTempFun,

    /* up_step/down_step used when type is STEP_SPEED_POLICY */
    pub up_step_num: u8,
    pub down_step_num: u8,
    pub up_step: [temp_range; MAX_STEP_NUM],
    pub down_step: [temp_range; MAX_STEP_NUM],
    pub work: delayed_work,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
