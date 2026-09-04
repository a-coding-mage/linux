// SPDX-License-Identifier: GPL-2.0+
//
// virtio-snd: Virtio sound device
// Copyright (C) 2021 OpenSynergy GmbH
//
// Dependencies: linux/virtio_config.h, sound/jack.h, sound/hda_verbs.h, virtio_card.h

//! DOC: Implementation Status
//!
//! At the moment jacks have a simple implementation and can only be used to
//! receive notifications about a plugged in/out device.
//!
//! VIRTIO_SND_R_JACK_REMAP
//!   is not supported

use std::ffi::CStr;
use std::os::raw::{c_int, c_char};

// External types from dependencies
extern "C" {
    pub type snd_jack;
    pub type virtio_snd;
    pub type virtio_device;
    pub type virtio_snd_config;
    pub type virtio_snd_jack_info;
    pub type virtio_snd_event;
}

/// struct virtio_jack - VirtIO jack.
/// @jack: Kernel jack control.
/// @nid: Functional group node identifier.
/// @features: Jack virtio feature bit map (1 << VIRTIO_SND_JACK_F_XXX).
/// @defconf: Pin default configuration value.
/// @caps: Pin capabilities value.
/// @connected: Current jack connection status.
/// @type: Kernel jack type (SND_JACK_XXX).
#[repr(C)]
pub struct virtio_jack {
    pub jack: *mut snd_jack,
    pub nid: u32,
    pub features: u32,
    pub defconf: u32,
    pub caps: u32,
    pub connected: bool,
    pub type_: c_int,
}

// HDA verb constants (from sound/hda_verbs.h)
const AC_DEFCFG_DEVICE: u32 = 0x0000_f000;
const AC_DEFCFG_DEVICE_SHIFT: u32 = 12;
const AC_DEFCFG_LOCATION: u32 = 0x0000_00ff;
const AC_DEFCFG_LOCATION_SHIFT: u32 = 0;

// HDA jack device types (from sound/hda_verbs.h)
const AC_JACK_LINE_OUT: u32 = 0;
const AC_JACK_SPEAKER: u32 = 1;
const AC_JACK_HP_OUT: u32 = 2;
const AC_JACK_CD: u32 = 3;
const AC_JACK_SPDIF_OUT: u32 = 4;
const AC_JACK_DIG_OTHER_OUT: u32 = 5;
const AC_JACK_LINE_IN: u32 = 0x20;
const AC_JACK_AUX: u32 = 0x21;
const AC_JACK_MIC_IN: u32 = 0x22;
const AC_JACK_SPDIF_IN: u32 = 0x23;
const AC_JACK_DIG_OTHER_IN: u32 = 0x24;

// HDA location constants (from sound/hda_verbs.h)
const AC_JACK_LOC_HDMI: u32 = 0x18;

// Sound jack types (from sound/jack.h)
const SND_JACK_LINEOUT: c_int = 2;
const SND_JACK_HEADPHONE: c_int = 4;
const SND_JACK_AVOUT: c_int = 128;
const SND_JACK_MICROPHONE: c_int = 32;
const SND_JACK_LINEIN: c_int = 8;

// Virtio sound events
const VIRTIO_SND_EVT_JACK_CONNECTED: u32 = 0x0100;
const VIRTIO_SND_EVT_JACK_DISCONNECTED: u32 = 0x0101;

// External functions from kernel/virtio libraries
extern "C" {
    fn virtio_cread_le(
        vdev: *mut virtio_device,
        config_type: *const std::ffi::c_void,
        field_offset: usize,
        ptr: *mut std::ffi::c_void,
    );

    fn devm_kcalloc(
        dev: *mut std::ffi::c_void,
        n: u32,
        size: usize,
        flags: u32,
    ) -> *mut std::ffi::c_void;

    fn kzalloc_objs(size: usize) -> *mut std::ffi::c_void;

    fn kfree(ptr: *mut std::ffi::c_void);

    fn virtsnd_ctl_query_info(
        snd: *mut virtio_snd,
        info_type: u32,
        start: u32,
        count: u32,
        size: usize,
        info: *mut std::ffi::c_void,
    ) -> c_int;

    fn snd_jack_new(
        card: *mut std::ffi::c_void,
        id: *const c_char,
        type_: c_int,
        jack: *mut *mut snd_jack,
        initial_kctl: bool,
        phantom_jack: bool,
    ) -> c_int;

    fn snd_jack_report(jack: *mut snd_jack, status: c_int);
}

/// virtsnd_jack_get_label() - Get the name string for the jack.
/// @vjack: VirtIO jack.
///
/// Returns the jack name based on the default pin configuration value (see HDA
/// specification).
///
/// Context: Any context.
/// Return: Name string.
fn virtsnd_jack_get_label(vjack: *const virtio_jack) -> *const c_char {
    unsafe {
        let defconf = (*vjack).defconf;
        let device = (defconf & AC_DEFCFG_DEVICE) >> AC_DEFCFG_DEVICE_SHIFT;
        let location = (defconf & AC_DEFCFG_LOCATION) >> AC_DEFCFG_LOCATION_SHIFT;

        match device {
            AC_JACK_LINE_OUT => b"Line Out\0".as_ptr() as *const c_char,
            AC_JACK_SPEAKER => b"Speaker\0".as_ptr() as *const c_char,
            AC_JACK_HP_OUT => b"Headphone\0".as_ptr() as *const c_char,
            AC_JACK_CD => b"CD\0".as_ptr() as *const c_char,
            AC_JACK_SPDIF_OUT | AC_JACK_DIG_OTHER_OUT => {
                if location == AC_JACK_LOC_HDMI {
                    b"HDMI Out\0".as_ptr() as *const c_char
                } else {
                    b"SPDIF Out\0".as_ptr() as *const c_char
                }
            }
            AC_JACK_LINE_IN => b"Line\0".as_ptr() as *const c_char,
            AC_JACK_AUX => b"Aux\0".as_ptr() as *const c_char,
            AC_JACK_MIC_IN => b"Mic\0".as_ptr() as *const c_char,
            AC_JACK_SPDIF_IN => b"SPDIF In\0".as_ptr() as *const c_char,
            AC_JACK_DIG_OTHER_IN => b"Digital In\0".as_ptr() as *const c_char,
            _ => b"Misc\0".as_ptr() as *const c_char,
        }
    }
}

/// virtsnd_jack_get_type() - Get the type for the jack.
/// @vjack: VirtIO jack.
///
/// Returns the jack type based on the default pin configuration value (see HDA
/// specification).
///
/// Context: Any context.
/// Return: SND_JACK_XXX value.
fn virtsnd_jack_get_type(vjack: *const virtio_jack) -> c_int {
    unsafe {
        let defconf = (*vjack).defconf;
        let device = (defconf & AC_DEFCFG_DEVICE) >> AC_DEFCFG_DEVICE_SHIFT;

        match device {
            AC_JACK_LINE_OUT | AC_JACK_SPEAKER => SND_JACK_LINEOUT,
            AC_JACK_HP_OUT => SND_JACK_HEADPHONE,
            AC_JACK_SPDIF_OUT | AC_JACK_DIG_OTHER_OUT => SND_JACK_AVOUT,
            AC_JACK_MIC_IN => SND_JACK_MICROPHONE,
            _ => SND_JACK_LINEIN,
        }
    }
}

/// virtsnd_jack_parse_cfg() - Parse the jack configuration.
/// @snd: VirtIO sound device.
///
/// This function is called during initial device initialization.
///
/// Context: Any context that permits to sleep.
/// Return: 0 on success, -errno on failure.
pub extern "C" fn virtsnd_jack_parse_cfg(snd: *mut virtio_snd) -> c_int {
    unsafe {
        let vdev = (*snd).vdev;
        let mut njacks: u32 = 0;

        virtio_cread_le(
            vdev,
            std::ptr::null(),
            0,
            &mut njacks as *mut u32 as *mut std::ffi::c_void,
        );

        if njacks == 0 {
            return 0;
        }

        (*snd).njacks = njacks;

        let jacks = devm_kcalloc(
            &mut (*vdev).dev as *mut _ as *mut std::ffi::c_void,
            njacks,
            std::mem::size_of::<virtio_jack>(),
            0, // GFP_KERNEL - assumed to be 0 or defined elsewhere
        ) as *mut virtio_jack;

        if jacks.is_null() {
            return -12; // -ENOMEM
        }

        (*snd).jacks = jacks;

        let info = kzalloc_objs(njacks as usize * std::mem::size_of::<virtio_snd_jack_info>())
            as *mut virtio_snd_jack_info;

        if info.is_null() {
            return -12; // -ENOMEM
        }

        let rc = virtsnd_ctl_query_info(
            snd,
            1, // VIRTIO_SND_R_JACK_INFO - assumed constant
            0,
            njacks,
            std::mem::size_of::<virtio_snd_jack_info>(),
            info as *mut std::ffi::c_void,
        );

        if rc != 0 {
            kfree(info as *mut std::ffi::c_void);
            return rc;
        }

        for i in 0..njacks {
            let vjack = &mut *jacks.add(i as usize);
            let info_i = &*info.add(i as usize);

            // These field accesses assume the info structure has the right layout
            // The actual field names and offsets are defined in virtio_card.h
            vjack.nid = le32_to_cpu((*info_i).hdr.hda_fn_nid);
            vjack.features = le32_to_cpu((*info_i).features);
            vjack.defconf = le32_to_cpu((*info_i).hda_reg_defconf);
            vjack.caps = le32_to_cpu((*info_i).hda_reg_caps);
            vjack.connected = (*info_i).connected;
        }

        kfree(info as *mut std::ffi::c_void);

        rc
    }
}

/// virtsnd_jack_build_devs() - Build ALSA controls for jacks.
/// @snd: VirtIO sound device.
///
/// Context: Any context that permits to sleep.
/// Return: 0 on success, -errno on failure.
pub extern "C" fn virtsnd_jack_build_devs(snd: *mut virtio_snd) -> c_int {
    unsafe {
        let njacks = (*snd).njacks;
        let jacks = (*snd).jacks;

        for i in 0..njacks {
            let vjack = &mut *jacks.add(i as usize);

            vjack.type_ = virtsnd_jack_get_type(vjack);

            let label = virtsnd_jack_get_label(vjack);
            let mut jack_ptr: *mut snd_jack = std::ptr::null_mut();

            let rc = snd_jack_new(
                (*snd).card as *mut std::ffi::c_void,
                label,
                vjack.type_,
                &mut jack_ptr,
                true,
                true,
            );

            if rc != 0 {
                return rc;
            }

            if !jack_ptr.is_null() {
                (*jack_ptr).private_data = vjack as *mut _ as *mut std::ffi::c_void;
            }

            vjack.jack = jack_ptr;

            let status = if vjack.connected { vjack.type_ } else { 0 };
            snd_jack_report(vjack.jack, status);
        }

        0
    }
}

/// virtsnd_jack_event() - Handle the jack event notification.
/// @snd: VirtIO sound device.
/// @event: VirtIO sound event.
///
/// Context: Interrupt context.
pub extern "C" fn virtsnd_jack_event(snd: *mut virtio_snd, event: *const virtio_snd_event) {
    unsafe {
        let jack_id = le32_to_cpu((*event).data);
        let njacks = (*snd).njacks;

        if jack_id >= njacks {
            return;
        }

        let vjack = &mut *(*snd).jacks.add(jack_id as usize);

        match le32_to_cpu((*event).hdr.code) {
            VIRTIO_SND_EVT_JACK_CONNECTED => {
                vjack.connected = true;
            }
            VIRTIO_SND_EVT_JACK_DISCONNECTED => {
                vjack.connected = false;
            }
            _ => return,
        }

        let status = if vjack.connected { vjack.type_ } else { 0 };
        snd_jack_report(vjack.jack, status);
    }
}

// Helper function for converting little-endian to CPU byte order
#[inline]
fn le32_to_cpu(x: u32) -> u32 {
    u32::from_le(x)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
