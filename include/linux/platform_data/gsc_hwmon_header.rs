/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub enum gsc_hwmon_mode {
    mode_temperature,
    mode_voltage_24bit,
    mode_voltage_raw,
    mode_voltage_16bit,
    mode_fan,
    mode_max,
}

/**
 * struct gsc_hwmon_channel - configuration parameters
 * @reg:  I2C register offset
 * @mode: channel mode
 * @name: channel name
 * @mvoffset: voltage offset
 * @vdiv: voltage divider array (2 resistor values in milli-ohms)
 */
#[repr(C)]
pub struct gsc_hwmon_channel {
    pub reg: kernel::ffi::c_uint,
    pub mode: kernel::ffi::c_uint,
    pub name: *const kernel::ffi::c_char,
    pub mvoffset: kernel::ffi::c_uint,
    pub vdiv: [kernel::ffi::c_uint; 2],
}

/**
 * struct gsc_hwmon_platform_data - platform data for gsc_hwmon driver
 * @nchannels:\tnumber of elements in @channels array
 * @vreference: voltage reference (mV)
 * @resolution: ADC bit resolution
 * @fan_base: register base for FAN controller
 * @channels:\tarray of gsc_hwmon_channel structures describing channels
 */
#[repr(C)]
pub struct gsc_hwmon_platform_data {
    pub nchannels: kernel::ffi::c_int,
    pub resolution: kernel::ffi::c_uint,
    pub vreference: kernel::ffi::c_uint,
    pub fan_base: kernel::ffi::c_uint,
    pub channels: [gsc_hwmon_channel; 0], // __counted_by(nchannels)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
