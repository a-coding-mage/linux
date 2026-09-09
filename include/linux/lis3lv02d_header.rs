/* SPDX-License-Identifier: GPL-2.0 */

/**
 * struct lis3lv02d_platform_data - lis3 chip family platform data
 *
 * Platform data is used to setup the sensor chip. Meaning of the different
 * chip features can be found from the data sheet. It is publicly available at
 * www.st.com web pages. Currently the platform data is used only for the 8 bit
 * device. The 8 bit device has two wake up / free fall detection units and
 * click detection unit. There are plenty of ways to configure the chip which
 * makes is quite hard to explain deeper meaning of the fields here. Behaviour
 * of the detection blocks varies heavily depending on the configuration. For
 * example, interrupt detection block can use high pass filtered data which
 * makes it react to the changes in the acceleration. Irq_flags can be used to
 * enable interrupt detection on the both edges. With proper chip
 * configuration this produces interrupt when some trigger starts and when it
 * goes away.
 */
#[repr(C)]
pub struct lis3lv02d_platform_data {
    /* please note: the 'click' feature is only supported for
     * LIS[32]02DL variants of the chip and will be ignored for
     * others */
    pub click_flags: u8,
    pub click_thresh_x: u8,
    pub click_thresh_y: u8,
    pub click_thresh_z: u8,
    pub click_time_limit: u8,
    pub click_latency: u8,
    pub click_window: u8,
    pub irq_cfg: u8,
    pub irq_flags1: u8, /* Additional irq edge / level flags */
    pub irq_flags2: u8, /* Additional irq edge / level flags */
    pub duration1: u8,
    pub duration2: u8,
    pub wakeup_flags: u8,
    pub wakeup_thresh: u8,
    pub wakeup_flags2: u8,
    pub wakeup_thresh2: u8,
    pub hipass_ctrl: u8,
    pub axis_x: i8,
    pub axis_y: i8,
    pub axis_z: i8,
    pub driver_features: u16,
    pub default_rate: i32,
    pub setup_resources: Option<unsafe extern "C" fn() -> i32>,
    pub release_resources: Option<unsafe extern "C" fn() -> i32>,
    /* Limits for selftest are specified in chip data sheet */
    pub st_min_limits: [i16; 3], /* min pass limit x, y, z */
    pub st_max_limits: [i16; 3], /* max pass limit x, y, z */
    pub irq2: i32,
}

pub const LIS3_CLICK_SINGLE_X: i32 = 1 << 0;
pub const LIS3_CLICK_DOUBLE_X: i32 = 1 << 1;
pub const LIS3_CLICK_SINGLE_Y: i32 = 1 << 2;
pub const LIS3_CLICK_DOUBLE_Y: i32 = 1 << 3;
pub const LIS3_CLICK_SINGLE_Z: i32 = 1 << 4;
pub const LIS3_CLICK_DOUBLE_Z: i32 = 1 << 5;

pub const LIS3_IRQ1_DISABLE: i32 = 0 << 0;
pub const LIS3_IRQ1_FF_WU_1: i32 = 1 << 0;
pub const LIS3_IRQ1_FF_WU_2: i32 = 2 << 0;
pub const LIS3_IRQ1_FF_WU_12: i32 = 3 << 0;
pub const LIS3_IRQ1_DATA_READY: i32 = 4 << 0;
pub const LIS3_IRQ1_CLICK: i32 = 7 << 0;
pub const LIS3_IRQ1_MASK: i32 = 7 << 0;
pub const LIS3_IRQ2_DISABLE: i32 = 0 << 3;
pub const LIS3_IRQ2_FF_WU_1: i32 = 1 << 3;
pub const LIS3_IRQ2_FF_WU_2: i32 = 2 << 3;
pub const LIS3_IRQ2_FF_WU_12: i32 = 3 << 3;
pub const LIS3_IRQ2_DATA_READY: i32 = 4 << 3;
pub const LIS3_IRQ2_CLICK: i32 = 7 << 3;
pub const LIS3_IRQ2_MASK: i32 = 7 << 3;
pub const LIS3_IRQ_OPEN_DRAIN: i32 = 1 << 6;
pub const LIS3_IRQ_ACTIVE_LOW: i32 = 1 << 7;

pub const LIS3_WAKEUP_X_LO: i32 = 1 << 0;
pub const LIS3_WAKEUP_X_HI: i32 = 1 << 1;
pub const LIS3_WAKEUP_Y_LO: i32 = 1 << 2;
pub const LIS3_WAKEUP_Y_HI: i32 = 1 << 3;
pub const LIS3_WAKEUP_Z_LO: i32 = 1 << 4;
pub const LIS3_WAKEUP_Z_HI: i32 = 1 << 5;

pub const LIS3_HIPASS_CUTFF_8HZ: i32 = 0;
pub const LIS3_HIPASS_CUTFF_4HZ: i32 = 1;
pub const LIS3_HIPASS_CUTFF_2HZ: i32 = 2;
pub const LIS3_HIPASS_CUTFF_1HZ: i32 = 3;
pub const LIS3_HIPASS1_DISABLE: i32 = 1 << 2;
pub const LIS3_HIPASS2_DISABLE: i32 = 1 << 3;

pub const LIS3_NO_MAP: i8 = 0;
pub const LIS3_DEV_X: i8 = 1;
pub const LIS3_DEV_Y: i8 = 2;
pub const LIS3_DEV_Z: i8 = 3;
pub const LIS3_INV_DEV_X: i8 = -1;
pub const LIS3_INV_DEV_Y: i8 = -2;
pub const LIS3_INV_DEV_Z: i8 = -3;
pub const LIS3_USE_BLOCK_READ: u16 = 0x02;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
