/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * STMicroelectronics sensors library driver
 *
 * Copyright 2012-2013 STMicroelectronics Inc.
 *
 * Denis Ciocca <denis.ciocca@st.com>
 */

// Dependencies supplied by the kernel translation unit are intentionally not
// redefined here.

pub const LSM9DS0_IMU_DEV_NAME: &str = "lsm9ds0";
pub const LSM303D_IMU_DEV_NAME: &str = "lsm303d";

/* Buffer size: 2 bytes per channel, 3 channels, aligned to an s64,
 * plus the 8-byte timestamp channel. */
pub const ST_SENSORS_MAX_BUFFER_SIZE: usize = 16;

pub const ST_SENSORS_ODR_LIST_MAX: usize = 10;
pub const ST_SENSORS_FULLSCALE_AVL_MAX: usize = 10;
pub const ST_SENSORS_NUMBER_ALL_CHANNELS: u32 = 4;
pub const ST_SENSORS_ENABLE_ALL_AXIS: u8 = 0x07;
pub const ST_SENSORS_SCAN_X: u32 = 0;
pub const ST_SENSORS_SCAN_Y: u32 = 1;
pub const ST_SENSORS_SCAN_Z: u32 = 2;
pub const ST_SENSORS_DEFAULT_POWER_ON_VALUE: u8 = 0x01;
pub const ST_SENSORS_DEFAULT_POWER_OFF_VALUE: u8 = 0x00;
pub const ST_SENSORS_DEFAULT_WAI_ADDRESS: u8 = 0x0f;
pub const ST_SENSORS_DEFAULT_AXIS_ADDR: u8 = 0x20;
pub const ST_SENSORS_DEFAULT_AXIS_MASK: u8 = 0x07;
pub const ST_SENSORS_DEFAULT_AXIS_N_BIT: u8 = 3;
pub const ST_SENSORS_DEFAULT_STAT_ADDR: u8 = 0x27;
pub const ST_SENSORS_MAX_NAME: usize = 17;
pub const ST_SENSORS_MAX_4WAI: usize = 8;

/* Direct equivalents of the C channel initializer macros. */
#[macro_export]
macro_rules! ST_SENSORS_LSM_CHANNELS_EXT {
    ($device_type:expr, $mask:expr, $index:expr, $mod:expr, $ch2:expr,
     $s:expr, $endian:expr, $rbits:expr, $sbits:expr, $addr:expr, $ext:expr) => {
        iio_chan_spec {
            type_: $device_type,
            modified: $mod,
            info_mask_separate: $mask,
            info_mask_shared_by_all: BIT(IIO_CHAN_INFO_SAMP_FREQ),
            scan_index: $index,
            channel2: $ch2,
            address: $addr,
            scan_type: iio_scan_type {
                sign: $s,
                realbits: $rbits,
                shift: $sbits - $rbits,
                storagebits: $sbits,
                endianness: $endian,
            },
            ext_info: $ext,
        }
    };
}

#[macro_export]
macro_rules! ST_SENSORS_LSM_CHANNELS {
    ($device_type:expr, $mask:expr, $index:expr, $mod:expr, $ch2:expr,
     $s:expr, $endian:expr, $rbits:expr, $sbits:expr, $addr:expr) => {
        ST_SENSORS_LSM_CHANNELS_EXT!($device_type, $mask, $index, $mod,
            $ch2, $s, $endian, $rbits, $sbits, $addr, core::ptr::null())
    };
}

/* These kernel attribute macros are represented as forwarding declarations. */
#[macro_export]
macro_rules! ST_SENSORS_DEV_ATTR_SAMP_FREQ_AVAIL {
    () => { IIO_DEV_ATTR_SAMP_FREQ_AVAIL!(st_sensors_sysfs_sampling_frequency_avail) };
}
#[macro_export]
macro_rules! ST_SENSORS_DEV_ATTR_SCALE_AVAIL {
    ($name:ident) => { IIO_DEVICE_ATTR!($name, S_IRUGO, st_sensors_sysfs_scale_avail, core::ptr::null(), 0) };
}

#[repr(C)]
pub struct st_sensor_odr_avl { pub hz: u32, pub value: u8 }
#[repr(C)]
pub struct st_sensor_odr { pub addr: u8, pub mask: u8, pub odr_avl: [st_sensor_odr_avl; ST_SENSORS_ODR_LIST_MAX] }
#[repr(C)]
pub struct st_sensor_power { pub addr: u8, pub mask: u8, pub value_off: u8, pub value_on: u8 }
#[repr(C)]
pub struct st_sensor_axis { pub addr: u8, pub mask: u8 }
#[repr(C)]
pub struct st_sensor_fullscale_avl { pub num: u32, pub value: u8, pub gain: u32, pub gain2: u32 }
#[repr(C)]
pub struct st_sensor_fullscale { pub addr: u8, pub mask: u8, pub fs_avl: [st_sensor_fullscale_avl; ST_SENSORS_FULLSCALE_AVL_MAX] }
#[repr(C)]
pub struct st_sensor_sim { pub addr: u8, pub value: u8 }

/** ST sensor device block data update. */
#[repr(C)]
pub struct st_sensor_bdu { pub addr: u8, pub mask: u8 }
/** ST sensor device data alignment selection. */
#[repr(C)]
pub struct st_sensor_das { pub addr: u8, pub mask: u8 }
/** ST sensor device drdy line parameters. */
#[repr(C)]
pub struct st_sensor_int_drdy { pub addr: u8, pub mask: u8, pub addr_od: u8, pub mask_od: u8 }

#[repr(C)]
pub struct st_sensor_data_ready_irq {
    pub int1: st_sensor_int_drdy,
    pub int2: st_sensor_int_drdy,
    pub addr_ihl: u8,
    pub mask_ihl: u8,
    pub stat_drdy: st_sensor_reg_pair,
    pub ig1: st_sensor_ig1,
}
#[repr(C)] pub struct st_sensor_reg_pair { pub addr: u8, pub mask: u8 }
#[repr(C)] pub struct st_sensor_ig1 { pub en_addr: u8, pub en_mask: u8 }

#[repr(C)]
pub struct st_sensor_settings {
    pub wai: u8, pub wai_addr: u8,
    pub sensors_supported: [[core::ffi::c_char; ST_SENSORS_MAX_NAME]; ST_SENSORS_MAX_4WAI],
    pub ch: *mut iio_chan_spec, pub num_ch: i32,
    pub odr: st_sensor_odr, pub pw: st_sensor_power, pub enable_axis: st_sensor_axis,
    pub fs: st_sensor_fullscale, pub bdu: st_sensor_bdu, pub das: st_sensor_das,
    pub drdy_irq: st_sensor_data_ready_irq, pub sim: st_sensor_sim,
    pub multi_read_bit: bool, pub bootime: u32,
}

#[repr(C)]
pub struct st_sensor_data {
    pub trig: *mut iio_trigger,
    pub mount_matrix: iio_mount_matrix,
    pub sensor_settings: *mut st_sensor_settings,
    pub current_fullscale: *mut st_sensor_fullscale_avl,
    pub regmap: *mut regmap,
    pub enabled: bool,
    pub odr: u32, pub num_data_channels: u32,
    pub drdy_int_pin: u8, pub int_pin_open_drain: bool, pub irq: i32,
    pub edge_irq: bool, pub hw_irq_trigger: bool, pub hw_timestamp: i64,
    pub odr_lock: mutex,
    pub buffer_data: [core::ffi::c_char; ST_SENSORS_MAX_BUFFER_SIZE],
}

#[cfg(CONFIG_IIO_BUFFER)]
extern "C" { pub fn st_sensors_trigger_handler(irq: i32, p: *mut core::ffi::c_void) -> irqreturn_t; }

#[cfg(CONFIG_IIO_TRIGGER)]
extern "C" {
    pub fn st_sensors_allocate_trigger(indio_dev: *mut iio_dev, trigger_ops: *const iio_trigger_ops) -> i32;
    pub fn st_sensors_validate_device(trig: *mut iio_trigger, indio_dev: *mut iio_dev) -> i32;
}
#[cfg(not(CONFIG_IIO_TRIGGER))]
pub unsafe fn st_sensors_allocate_trigger(_: *mut iio_dev, _: *const iio_trigger_ops) -> i32 { 0 }
#[cfg(not(CONFIG_IIO_TRIGGER))]
pub const st_sensors_validate_device: Option<unsafe extern "C" fn(*mut iio_trigger, *mut iio_dev) -> i32> = None;

extern "C" {
    pub fn st_sensors_init_sensor(indio_dev: *mut iio_dev, pdata: *mut st_sensors_platform_data) -> i32;
    pub fn st_sensors_set_enable(indio_dev: *mut iio_dev, enable: bool) -> i32;
    pub fn st_sensors_set_axis_enable(indio_dev: *mut iio_dev, axis_enable: u8) -> i32;
    pub fn st_sensors_power_enable(indio_dev: *mut iio_dev) -> i32;
    pub fn st_sensors_debugfs_reg_access(indio_dev: *mut iio_dev, reg: u32, writeval: u32, readval: *mut u32) -> i32;
    pub fn st_sensors_set_odr(indio_dev: *mut iio_dev, odr: u32) -> i32;
    pub fn st_sensors_set_dataready_irq(indio_dev: *mut iio_dev, enable: bool) -> i32;
    pub fn st_sensors_set_fullscale_by_gain(indio_dev: *mut iio_dev, scale: i32) -> i32;
    pub fn st_sensors_read_info_raw(indio_dev: *mut iio_dev, ch: *const iio_chan_spec, val: *mut i32) -> i32;
    pub fn st_sensors_get_settings_index(name: *const core::ffi::c_char, list: *const st_sensor_settings, list_length: i32) -> i32;
    pub fn st_sensors_verify_id(indio_dev: *mut iio_dev) -> i32;
    pub fn st_sensors_sysfs_sampling_frequency_avail(dev: *mut device, attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> isize;
    pub fn st_sensors_sysfs_scale_avail(dev: *mut device, attr: *mut device_attribute, buf: *mut core::ffi::c_char) -> isize;
    pub fn st_sensors_dev_name_probe(dev: *mut device, name: *mut core::ffi::c_char, len: i32);
    pub fn st_accel_get_settings(name: *const core::ffi::c_char) -> *const st_sensor_settings;
    pub fn st_accel_common_probe(indio_dev: *mut iio_dev) -> i32;
    pub fn st_gyro_get_settings(name: *const core::ffi::c_char) -> *const st_sensor_settings;
    pub fn st_gyro_common_probe(indio_dev: *mut iio_dev) -> i32;
    pub fn st_magn_get_settings(name: *const core::ffi::c_char) -> *const st_sensor_settings;
    pub fn st_magn_common_probe(indio_dev: *mut iio_dev) -> i32;
    pub fn st_press_get_settings(name: *const core::ffi::c_char) -> *const st_sensor_settings;
    pub fn st_press_common_probe(indio_dev: *mut iio_dev) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
