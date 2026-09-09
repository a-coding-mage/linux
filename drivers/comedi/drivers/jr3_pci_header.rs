/* SPDX-License-Identifier: GPL-2.0 */
/* Helper types for the 16-bit DSP memory aligned on a 32-bit PCI boundary. */

type u16 = core::primitive::u16;
type u32 = core::primitive::u32;
type s16 = core::primitive::i16;
type s32 = core::primitive::i32;

unsafe extern "C" {
    fn readl(p: *const u32) -> u32;
    fn writel(val: u32, p: *mut u32);
}

#[inline]
unsafe fn get_u16(p: *const u32) -> u16 { readl(p) as u16 }

#[inline]
unsafe fn set_u16(p: *mut u32, val: u16) { writel(val as u32, p); }

#[inline]
unsafe fn get_s16(p: *const s32) -> s16 { readl(p as *const u32) as s16 }

#[inline]
unsafe fn set_s16(p: *mut s32, val: s16) { writel(val as u32, p as *mut u32); }

#[repr(C)]
pub struct raw_channel {
    pub raw_time: u32,
    pub raw_data: s32,
    pub reserved: [s32; 2],
}

#[repr(C)]
pub struct force_array { pub fx: s32, pub fy: s32, pub fz: s32, pub mx: s32, pub my: s32, pub mz: s32, pub v1: s32, pub v2: s32 }

#[repr(C)]
pub struct six_axis_array { pub fx: s32, pub fy: s32, pub fz: s32, pub mx: s32, pub my: s32, pub mz: s32 }

/* VECT_BITS: bit pattern selecting axes used for vector calculations. */
pub const fx: u32 = 0x0001;
pub const fy: u32 = 0x0002;
pub const fz: u32 = 0x0004;
pub const mx: u32 = 0x0008;
pub const my: u32 = 0x0010;
pub const mz: u32 = 0x0020;
pub const changeV2: u32 = 0x0040;
pub const changeV1: u32 = 0x0080;

/* WARNING_BITS / XX_NEAR_SET */
pub const fx_near_sat: u32 = 0x0001;
pub const fy_near_sat: u32 = 0x0002;
pub const fz_near_sat: u32 = 0x0004;
pub const mx_near_sat: u32 = 0x0008;
pub const my_near_sat: u32 = 0x0010;
pub const mz_near_sat: u32 = 0x0020;

/* ERROR_BITS, XX_SAT, MEMORY_ERROR, SENSOR_CHANGE, SYSTEM_BUSY, CAL_CRC_BAD, WATCH_DOG */
#[repr(C)]
pub enum error_bits_t {
    fx_sat = 0x0001, fy_sat = 0x0002, fz_sat = 0x0004, mx_sat = 0x0008,
    my_sat = 0x0010, mz_sat = 0x0020, memory_error = 0x0400,
    sensor_change = 0x0800, system_busy = 0x1000, cal_crc_bad = 0x2000,
    watch_dog2 = 0x4000, watch_dog = 0x8000,
}

#[repr(C)]
pub struct thresh_struct { pub data_address: s32, pub threshold: s32, pub bit_pattern: s32 }

#[repr(C)]
pub struct le_struct {
    pub latch_bits: s32,
    pub number_of_ge_thresholds: s32,
    pub number_of_le_thresholds: s32,
    pub thresholds: [thresh_struct; 4],
    pub reserved: s32,
}

#[repr(C)]
pub enum link_types { end_x_form, tx, ty, tz, rx, ry, rz, neg }

#[repr(C)]
pub struct intern_transform {
    pub link: [intern_transform_link; 8],
}
#[repr(C)]
pub struct intern_transform_link { pub link_type: u32, pub link_amount: s32 }

#[repr(C)]
pub struct jr3_sensor {
    pub raw_channels: [raw_channel; 16],
    pub copyright: [u32; 0x0018],
    pub reserved1: [s32; 0x0008],
    pub shunts: six_axis_array,
    pub reserved2: [s32; 2],
    pub default_FS: six_axis_array,
    pub reserved3: s32,
    pub load_envelope_num: s32,
    pub min_full_scale: six_axis_array,
    pub reserved4: s32,
    pub transform_num: s32,
    pub max_full_scale: six_axis_array,
    pub reserved5: s32,
    pub peak_address: s32,
    pub full_scale: force_array,
    pub offsets: six_axis_array,
    pub offset_num: s32,
    pub vect_axes: u32,
    pub filter: [force_array; 7],
    pub rate_data: force_array,
    pub minimum_data: force_array,
    pub maximum_data: force_array,
    pub near_sat_value: s32,
    pub sat_value: s32,
    pub rate_address: s32,
    pub rate_divisor: u32,
    pub rate_count: u32,
    pub command_word2: s32,
    pub command_word1: s32,
    pub command_word0: s32,
    pub count1: u32,
    pub count2: u32,
    pub count3: u32,
    pub count4: u32,
    pub count5: u32,
    pub count6: u32,
    pub error_count: u32,
    pub count_x: u32,
    pub warnings: u32,
    pub errors: u32,
    pub threshold_bits: s32,
    pub last_CRC: s32,
    pub eeprom_ver_no: s32,
    pub software_ver_no: s32,
    pub software_day: s32,
    pub software_year: s32,
    pub serial_no: u32,
    pub model_no: u32,
    pub cal_day: s32,
    pub cal_year: s32,
    pub units: u32,
    pub bits: s32,
    pub channels: s32,
    pub thickness: s32,
    pub load_envelopes: [le_struct; 0x10],
    pub transforms: [intern_transform; 0x10],
}

#[repr(C)]
pub struct jr3_block {
    pub program_lo: [u32; 0x4000],
    pub sensor: jr3_sensor,
    pub pad2: [core::ffi::c_char; 0x30000 - 0x00c00],
    pub program_hi: [u32; 0x8000],
    pub reset: u32,
    pub pad3: [core::ffi::c_char; 0x20000 - 0x00004],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
