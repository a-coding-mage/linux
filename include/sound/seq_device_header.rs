/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  ALSA sequencer device management
 *  Copyright (c) 1999 by Takashi Iwai <tiwai@suse.de>
 */

/*
 * registered device information
 */
#[repr(C)]
pub struct snd_seq_device {
    /* device info */
    pub card: *mut snd_card, /* sound card */
    pub device: ::core::ffi::c_int, /* device number */
    pub id: *const ::core::ffi::c_char, /* driver id */
    pub name: [::core::ffi::c_char; 80], /* device name */
    pub argsize: ::core::ffi::c_int, /* size of the argument */
    pub driver_data: *mut ::core::ffi::c_void, /* private data for driver */
    pub private_data: *mut ::core::ffi::c_void, /* private data for the caller */
    pub private_free: Option<unsafe extern "C" fn(device: *mut snd_seq_device)>,
    pub dev: device,
    pub args: [u8; 0], /* driver-specific argument */
}

/* Equivalent to container_of(_dev, struct snd_seq_device, dev). */
#[macro_export]
macro_rules! to_seq_dev {
    ($dev:expr) => {
        container_of!($dev, $crate::snd_seq_device, dev)
    };
}

/* sequencer driver */

/* driver operators
 * probe:
 *	Initialize the device with given parameters.
 *	Typically,
 *		1. call snd_hwdep_new
 *		2. allocate private data and initialize it
 *		3. call snd_hwdep_register
 *		4. store the instance to dev->driver_data pointer.
 *
 * remove:
 *	Release the private data.
 *	Typically, call snd_device_free(dev->card, dev->driver_data)
 */
#[repr(C)]
pub struct snd_seq_driver {
    pub probe: Option<unsafe extern "C" fn(dev: *mut snd_seq_device) -> ::core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(dev: *mut snd_seq_device)>,
    pub driver: device_driver,
    pub id: *mut ::core::ffi::c_char,
    pub argsize: ::core::ffi::c_int,
}

/* Equivalent to container_of(_drv, struct snd_seq_driver, driver). */
#[macro_export]
macro_rules! to_seq_drv {
    ($drv:expr) => {
        container_of!($drv, $crate::snd_seq_driver, driver)
    };
}

/* prototypes */
#[cfg(feature = "CONFIG_MODULES")]
unsafe extern "C" {
    pub fn snd_seq_device_load_drivers();
}

#[cfg(not(feature = "CONFIG_MODULES"))]
#[macro_export]
macro_rules! snd_seq_device_load_drivers {
    () => {};
}

unsafe extern "C" {
    pub fn snd_seq_device_new(
        card: *mut snd_card,
        device: ::core::ffi::c_int,
        id: *const ::core::ffi::c_char,
        argsize: ::core::ffi::c_int,
        result: *mut *mut snd_seq_device,
    ) -> ::core::ffi::c_int;

    pub fn __snd_seq_driver_register(
        drv: *mut snd_seq_driver,
        module: *mut module,
    ) -> ::core::ffi::c_int;

    pub fn snd_seq_driver_unregister(drv: *mut snd_seq_driver);
}

#[macro_export]
macro_rules! SNDRV_SEQ_DEVICE_ARGPTR {
    ($dev:expr) => {
        unsafe { (*$dev).args.as_mut_ptr() as *mut ::core::ffi::c_void }
    };
}

#[macro_export]
macro_rules! snd_seq_driver_register {
    ($drv:expr) => {
        __snd_seq_driver_register($drv, THIS_MODULE)
    };
}

#[macro_export]
macro_rules! module_snd_seq_driver {
    ($drv:expr) => {
        module_driver!($drv, snd_seq_driver_register, snd_seq_driver_unregister)
    };
}

/* id strings for generic devices */
pub const SNDRV_SEQ_DEV_ID_MIDISYNTH: &str = "seq-midi";
pub const SNDRV_SEQ_DEV_ID_OPL3: &str = "opl3-synth";
pub const SNDRV_SEQ_DEV_ID_UMP: &str = "seq-ump-client";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
