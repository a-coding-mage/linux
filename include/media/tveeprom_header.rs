/* SPDX-License-Identifier: GPL-2.0 */

/*
 * tveeprom - Contains structures and functions to work with Hauppauge
 *            eeproms.
 */

/**
 * enum tveeprom_audio_processor - Specifies the type of audio processor
 *                                  used on a Hauppauge device.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tveeprom_audio_processor {
    TVEEPROM_AUDPROC_NONE,
    TVEEPROM_AUDPROC_INTERNAL,
    TVEEPROM_AUDPROC_MSP,
    TVEEPROM_AUDPROC_OTHER,
}

/**
 * struct tveeprom - Contains the fields parsed from Hauppauge eeproms
 *
 * The field meanings are preserved from the C declaration and its
 * documentation in the source header.
 */
#[repr(C)]
pub struct tveeprom {
    pub has_radio: u32,
    pub has_ir: u32,
    pub has_MAC_address: u32,

    pub tuner_type: u32,
    pub tuner_formats: u32,
    pub tuner_hauppauge_model: u32,

    pub tuner2_type: u32,
    pub tuner2_formats: u32,
    pub tuner2_hauppauge_model: u32,

    pub audio_processor: u32,
    pub decoder_processor: u32,

    pub model: u32,
    pub revision: u32,
    pub serial_number: u32,
    pub rev_str: [std::os::raw::c_char; 5],
    pub MAC_address: [u8; 6], // ETH_ALEN from <uapi/linux/if_ether.h>
}

// External type supplied by the I2C subsystem.
#[repr(C)]
pub struct i2c_client {
    _private: [u8; 0],
}

extern "C" {
    /** Fill struct tveeprom using the contents of the previously read EEPROM. */
    pub fn tveeprom_hauppauge_analog(
        tvee: *mut tveeprom,
        eeprom_data: *mut std::os::raw::c_uchar,
    );

    /** Read the contents of the EEPROM found at Hauppauge devices. */
    pub fn tveeprom_read(
        c: *mut i2c_client,
        eedata: *mut std::os::raw::c_uchar,
        len: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
