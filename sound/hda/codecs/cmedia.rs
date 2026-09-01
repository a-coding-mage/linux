// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Universal codec driver for Intel High Definition Audio Codec
 *
 * HD audio codec driver for C-Media CMI9880
 *
 * Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::ptr;

/* Dependencies supplied by linux/init.h, linux/slab.h, linux/module.h,
 * sound/core.h, sound/hda_codec.h, hda_local.h, hda_auto_parser.h,
 * hda_jack.h, and generic.h.
 */

const ENOMEM: c_int = 12;
const AC_JACK_HP_OUT: c_uint = 0x21;
const HDA_OUTPUT: c_uint = 0;

#[repr(C)]
pub struct auto_pin_cfg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hda_codec {
    pub spec: *mut c_void,
}

#[repr(C)]
pub struct hda_device_id {
    pub vendor_id: c_uint,
    pub rev_id: c_uint,
    pub name: *const c_char,
    pub driver_data: c_ulonglong,
}

#[repr(C)]
pub struct hda_gen_spec {
    pub autocfg: auto_pin_cfg,
    pub out_vol_mask: c_ulonglong,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub index: c_uint,
    pub access: c_uint,
    pub private_value: c_ulonglong,
}

#[repr(C)]
pub struct hda_codec_ops {
    pub probe: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub build_controls: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub build_pcms: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub unsol_event: Option<unsafe extern "C" fn(*mut hda_codec, c_uint)>,
    pub check_power_status: Option<unsafe extern "C" fn(*mut hda_codec, c_uint) -> c_int>,
    pub stream_pm: Option<unsafe extern "C" fn(*mut hda_codec, c_uint, bool)>,
}

#[repr(C)]
pub struct hda_codec_driver {
    pub id: *const hda_device_id,
    pub ops: *const hda_codec_ops,
}

extern "C" {
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn snd_hda_gen_spec_init(spec: *mut hda_gen_spec);
    fn snd_hda_parse_pin_defcfg(
        codec: *mut hda_codec,
        cfg: *mut auto_pin_cfg,
        ignore_nids: *const c_void,
        cond_flags: c_uint,
    ) -> c_int;
    fn snd_hda_gen_parse_auto_config(codec: *mut hda_codec, cfg: *mut auto_pin_cfg) -> c_int;
    fn snd_hda_codec_get_pincfg(codec: *mut hda_codec, nid: c_uint) -> c_uint;
    fn get_defcfg_device(cfg: c_uint) -> c_uint;
    fn snd_hda_gen_add_kctl(
        spec: *mut hda_gen_spec,
        name: *const c_char,
        knew: *const snd_kcontrol_new,
    ) -> *mut c_void;
    fn snd_hda_gen_remove(codec: *mut hda_codec);
    fn snd_hda_gen_build_controls(codec: *mut hda_codec) -> c_int;
    fn snd_hda_gen_build_pcms(codec: *mut hda_codec) -> c_int;
    fn snd_hda_gen_init(codec: *mut hda_codec) -> c_int;
    fn snd_hda_jack_unsol_event(codec: *mut hda_codec, res: c_uint);
    fn snd_hda_gen_check_power_status(codec: *mut hda_codec, nid: c_uint) -> c_int;
    fn snd_hda_gen_stream_pm(codec: *mut hda_codec, nid: c_uint, on: bool);
}

unsafe fn kzalloc_obj_hda_gen_spec() -> *mut hda_gen_spec {
    kzalloc(core::mem::size_of::<hda_gen_spec>(), 0) as *mut hda_gen_spec
}

const fn hda_codec_volume(
    name: *const c_char,
    nid: c_uint,
    channel: c_uint,
    direction: c_uint,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: 0,
        name,
        index: 0,
        access: 0,
        private_value: ((nid as c_ulonglong) << 32)
            | ((channel as c_ulonglong) << 16)
            | direction as c_ulonglong,
    }
}

unsafe extern "C" fn cmedia_probe(
    codec: *mut hda_codec,
    id: *const hda_device_id,
) -> c_int {
    let spec: *mut hda_gen_spec;
    let cfg: *mut auto_pin_cfg;
    let is_cmi8888: bool = (*id).vendor_id == 0x13f68888;
    let mut err: c_int;

    spec = kzalloc_obj_hda_gen_spec();
    if spec.is_null() {
        return -ENOMEM;
    }

    (*codec).spec = spec as *mut c_void;
    cfg = &mut (*spec).autocfg;
    snd_hda_gen_spec_init(spec);

    if is_cmi8888 {
        /* mask NID 0x10 from the playback volume selection;
         * it's a headphone boost volume handled manually below
         */
        (*spec).out_vol_mask = 1u64 << 0x10;
    }

    err = snd_hda_parse_pin_defcfg(codec, cfg, ptr::null(), 0);
    if err < 0 {
        snd_hda_gen_remove(codec);
        return err;
    }
    err = snd_hda_gen_parse_auto_config(codec, cfg);
    if err < 0 {
        snd_hda_gen_remove(codec);
        return err;
    }

    if is_cmi8888 {
        if get_defcfg_device(snd_hda_codec_get_pincfg(codec, 0x10)) == AC_JACK_HP_OUT {
            static AMP_KCTL_NAME: &[u8] = b"Headphone Amp Playback Volume\0";
            static AMP_KCTL: snd_kcontrol_new = hda_codec_volume(
                AMP_KCTL_NAME.as_ptr() as *const c_char,
                0x10,
                0,
                HDA_OUTPUT,
            );
            if snd_hda_gen_add_kctl(spec, ptr::null(), &AMP_KCTL).is_null() {
                err = -ENOMEM;
                snd_hda_gen_remove(codec);
                return err;
            }
        }
    }

    0
}

static CMEDIA_CODEC_OPS: hda_codec_ops = hda_codec_ops {
    probe: Some(cmedia_probe),
    remove: Some(snd_hda_gen_remove),
    build_controls: Some(snd_hda_gen_build_controls),
    build_pcms: Some(snd_hda_gen_build_pcms),
    init: Some(snd_hda_gen_init),
    unsol_event: Some(snd_hda_jack_unsol_event),
    check_power_status: Some(snd_hda_gen_check_power_status),
    stream_pm: Some(snd_hda_gen_stream_pm),
};

/*
 * driver entries
 */
const fn hda_codec_id(vendor_id: c_uint, name: *const c_char) -> hda_device_id {
    hda_device_id {
        vendor_id,
        rev_id: 0,
        name,
        driver_data: 0,
    }
}

static CMI8888_NAME: &[u8] = b"CMI8888\0";
static CMI9880_NAME: &[u8] = b"CMI9880\0";

static SND_HDA_ID_CMEDIA: [hda_device_id; 4] = [
    hda_codec_id(0x13f68888, CMI8888_NAME.as_ptr() as *const c_char),
    hda_codec_id(0x13f69880, CMI9880_NAME.as_ptr() as *const c_char),
    hda_codec_id(0x434d4980, CMI9880_NAME.as_ptr() as *const c_char),
    hda_device_id {
        vendor_id: 0,
        rev_id: 0,
        name: ptr::null(),
        driver_data: 0,
    }, /* terminator */
];
/* MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_cmedia); */

/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("C-Media HD-audio codec"); */

static mut CMEDIA_DRIVER: hda_codec_driver = hda_codec_driver {
    id: SND_HDA_ID_CMEDIA.as_ptr(),
    ops: &CMEDIA_CODEC_OPS,
};

/* module_hda_codec_driver(cmedia_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
