/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __LINUX_PLATFORM_DATA_EMC2305__

pub const EMC2305_PWM_MAX: usize = 5;

/**
 * struct emc2305_platform_data - EMC2305 driver platform data
 * @max_state: maximum cooling state of the cooling device;
 * @pwm_num: number of active channels;
 * @pwm_output_mask: PWM output mask
 * @pwm_polarity_mask: PWM polarity mask
 * @pwm_separate: separate PWM settings for every channel;
 * @pwm_min: array of minimum PWM per channel;
 * @pwm_freq: array of PWM frequency per channel
 */
#[repr(C)]
pub struct emc2305_platform_data {
    pub max_state: u8,
    pub pwm_num: u8,
    pub pwm_output_mask: u8,
    pub pwm_polarity_mask: u8,
    pub pwm_separate: bool,
    pub pwm_min: [u8; EMC2305_PWM_MAX],
    pub pwm_freq: [u16; EMC2305_PWM_MAX],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
