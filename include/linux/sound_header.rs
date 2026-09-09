/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <uapi/linux/sound.h> declarations supplied by the included header.

/*
 *	Sound core interface functions
 */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

// Defined by the dependent file-operations header.
#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

extern "C" {
    pub fn register_sound_special(fops: *const file_operations, unit: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
    pub fn register_sound_special_device(
        fops: *const file_operations,
        unit: ::std::os::raw::c_int,
        dev: *mut device,
    ) -> ::std::os::raw::c_int;
    pub fn register_sound_mixer(
        fops: *const file_operations,
        dev: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn register_sound_dsp(
        fops: *const file_operations,
        dev: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn unregister_sound_special(unit: ::std::os::raw::c_int);
    pub fn unregister_sound_mixer(unit: ::std::os::raw::c_int);
    pub fn unregister_sound_dsp(unit: ::std::os::raw::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
