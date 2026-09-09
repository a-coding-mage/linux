/*
 * include/linux/platform_data/media/si4713.h
 *
 * Board related data definitions for Si4713 i2c device driver.
 *
 * Copyright (c) 2009 Nokia Corporation
 * Contact: Eduardo Valentin <eduardo.valentin@nokia.com>
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2. This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 *
 */

/* The SI4713 I2C sensor chip has a fixed slave address of 0xc6 or 0x22. */
pub const SI4713_I2C_ADDR_BUSEN_HIGH: u32 = 0x63;
pub const SI4713_I2C_ADDR_BUSEN_LOW: u32 = 0x11;

/*
 * Platform dependent definition
 */
#[repr(C)]
pub struct si4713_platform_data {
    pub is_platform_device: bool,
}

/*
 * Structure to query for Received Noise Level (RNL).
 */
#[repr(C)]
pub struct si4713_rnl {
    pub index: __u32,       /* modulator index */
    pub frequency: __u32,   /* frequency to perform rnl measurement */
    pub rnl: __s32,         /* result of measurement in dBuV */
    pub reserved: [__u32; 4], /* drivers and apps must init this to 0 */
}

/*
 * This is the ioctl number to query for rnl. Users must pass a
 * struct si4713_rnl pointer specifying desired frequency in 'frequency' field
 * following driver capabilities (i.e V4L2_TUNER_CAP_LOW).
 * Driver must return measured value in the same structure, filling 'rnl' field.
 */
/* Build-time ioctl encoding supplied by the surrounding kernel bindings. */
pub const SI4713_IOC_MEASURE_RNL: _ = _IOWR!('V', BASE_VIDIOC_PRIVATE + 0, si4713_rnl);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
