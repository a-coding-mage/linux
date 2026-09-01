// SPDX-License-Identifier: GPL-2.0-or-later
//
// Realtek ALC861-VD codec
// Based on ALC882
// In addition, an independent DAC
//

// C includes translated as external dependency intent:
// <linux/init.h>
// <linux/module.h>
// "realtek.h"

type hda_nid_t = u16;

#[repr(C)]
pub struct hda_codec {
    pub spec: *mut alc_spec,
}

#[repr(C)]
pub struct hda_device_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct alc_spec {
    pub gen: hda_gen_spec,
    pub shutup: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub gpio_mask: u32,
}

#[repr(C)]
pub struct hda_gen_spec {
    pub beep_nid: hda_nid_t,
    pub no_analog: bool,
}

#[repr(C)]
pub struct hda_fixup {
    pub type_: i32,
    pub v: hda_fixup_v,
}

#[repr(C)]
pub union hda_fixup_v {
    pub func: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_fixup, i32)>,
}

#[repr(C)]
pub struct hda_quirk {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub name: *const u8,
    pub value: usize,
}

#[repr(C)]
pub struct hda_codec_ops {
    pub probe: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_device_id) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub build_controls: Option<unsafe extern "C" fn(*mut hda_codec) -> i32>,
    pub build_pcms: Option<unsafe extern "C" fn(*mut hda_codec) -> i32>,
    pub init: Option<unsafe extern "C" fn(*mut hda_codec) -> i32>,
    pub unsol_event: Option<unsafe extern "C" fn(*mut hda_codec, u32)>,
    pub resume: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub suspend: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub check_power_status: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t) -> i32>,
    pub stream_pm: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t, bool)>,
}

#[repr(C)]
pub struct hda_codec_driver {
    pub id: *const hda_device_id,
    pub ops: *const hda_codec_ops,
}

const HDA_FIXUP_ACT_PRE_PROBE: i32 = 0;
const HDA_FIXUP_ACT_PROBE: i32 = 1;
const HDA_FIXUP_FUNC: i32 = 2;
const HDA_INPUT: i32 = 0;

unsafe extern "C" {
    fn alc_parse_auto_config(
        codec: *mut hda_codec,
        ignore: *const hda_nid_t,
        ssids: *const hda_nid_t,
    ) -> i32;
    fn snd_hda_override_pin_caps(codec: *mut hda_codec, nid: hda_nid_t, caps: u32);
    fn alc_fixup_gpio(codec: *mut hda_codec, action: i32, gpio: u32);
    fn alc_alloc_spec(codec: *mut hda_codec, nid: hda_nid_t) -> i32;
    fn has_cdefine_beep(codec: *mut hda_codec) -> bool;
    fn alc_eapd_shutup(codec: *mut hda_codec);
    fn alc_pre_init(codec: *mut hda_codec);
    fn snd_hda_pick_fixup(
        codec: *mut hda_codec,
        models: *const core::ffi::c_void,
        quirks: *const hda_quirk,
        fixups: *const hda_fixup,
    );
    fn snd_hda_apply_fixup(codec: *mut hda_codec, action: i32);
    fn set_beep_amp(spec: *mut alc_spec, nid: hda_nid_t, dir: i32, idx: i32) -> i32;
    fn snd_hda_gen_remove(codec: *mut hda_codec);
    fn alc_build_controls(codec: *mut hda_codec) -> i32;
    fn snd_hda_gen_build_pcms(codec: *mut hda_codec) -> i32;
    fn alc_init(codec: *mut hda_codec) -> i32;
    fn snd_hda_jack_unsol_event(codec: *mut hda_codec, res: u32);
    fn alc_resume(codec: *mut hda_codec);
    fn alc_suspend(codec: *mut hda_codec);
    fn snd_hda_gen_check_power_status(codec: *mut hda_codec, nid: hda_nid_t) -> i32;
    fn snd_hda_gen_stream_pm(codec: *mut hda_codec, nid: hda_nid_t, on: bool);
}

unsafe fn alc861vd_parse_auto_config(codec: *mut hda_codec) -> i32 {
    static ALC861VD_IGNORE: [hda_nid_t; 2] = [0x1d, 0];
    static ALC861VD_SSIDS: [hda_nid_t; 4] = [0x15, 0x1b, 0x14, 0];
    unsafe { alc_parse_auto_config(codec, ALC861VD_IGNORE.as_ptr(), ALC861VD_SSIDS.as_ptr()) }
}

const ALC660VD_FIX_ASUS_GPIO1: usize = 0;
const ALC861VD_FIX_DALLAS: usize = 1;

/* exclude VREF80 */
unsafe extern "C" fn alc861vd_fixup_dallas(
    codec: *mut hda_codec,
    _fix: *const hda_fixup,
    action: i32,
) {
    if action == HDA_FIXUP_ACT_PRE_PROBE {
        unsafe {
            snd_hda_override_pin_caps(codec, 0x18, 0x00000734);
            snd_hda_override_pin_caps(codec, 0x19, 0x0000073c);
        }
    }
}

/* reset GPIO1 */
unsafe extern "C" fn alc660vd_fixup_asus_gpio1(
    codec: *mut hda_codec,
    _fix: *const hda_fixup,
    action: i32,
) {
    let spec = unsafe { (*codec).spec };

    if action == HDA_FIXUP_ACT_PRE_PROBE {
        unsafe {
            (*spec).gpio_mask |= 0x02;
        }
    }
    unsafe {
        alc_fixup_gpio(codec, action, 0x01);
    }
}

static ALC861VD_FIXUPS: [hda_fixup; 2] = [
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup_v {
            func: Some(alc660vd_fixup_asus_gpio1),
        },
    },
    hda_fixup {
        type_: HDA_FIXUP_FUNC,
        v: hda_fixup_v {
            func: Some(alc861vd_fixup_dallas),
        },
    },
];

const fn snd_pci_quirk(vendor: u32, device: u32, name: *const u8, value: usize) -> hda_quirk {
    hda_quirk {
        vendor,
        device,
        subvendor: 0,
        subdevice: 0,
        name,
        value,
    }
}

static ALC861VD_FIXUP_TBL: [hda_quirk; 4] = [
    snd_pci_quirk(0x103c, 0x30bf, b"HP TX1000\0".as_ptr(), ALC861VD_FIX_DALLAS),
    snd_pci_quirk(0x1043, 0x1339, b"ASUS A7-K\0".as_ptr(), ALC660VD_FIX_ASUS_GPIO1),
    snd_pci_quirk(
        0x1179,
        0xff31,
        b"Toshiba L30-149\0".as_ptr(),
        ALC861VD_FIX_DALLAS,
    ),
    hda_quirk {
        vendor: 0,
        device: 0,
        subvendor: 0,
        subdevice: 0,
        name: core::ptr::null(),
        value: 0,
    },
];

/*
 */
unsafe extern "C" fn alc861vd_probe(codec: *mut hda_codec, _id: *const hda_device_id) -> i32 {
    let spec: *mut alc_spec;
    let mut err: i32;

    err = unsafe { alc_alloc_spec(codec, 0x0b) };
    if err < 0 {
        return err;
    }

    spec = unsafe { (*codec).spec };
    if unsafe { has_cdefine_beep(codec) } {
        unsafe {
            (*spec).gen.beep_nid = 0x23;
        }
    }

    unsafe {
        (*spec).shutup = Some(alc_eapd_shutup);
    }

    unsafe {
        alc_pre_init(codec);

        snd_hda_pick_fixup(
            codec,
            core::ptr::null(),
            ALC861VD_FIXUP_TBL.as_ptr(),
            ALC861VD_FIXUPS.as_ptr(),
        );
        snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PRE_PROBE);
    }

    /* automatic parse from the BIOS config */
    err = unsafe { alc861vd_parse_auto_config(codec) };
    if err < 0 {
        unsafe {
            snd_hda_gen_remove(codec);
        }
        return err;
    }

    if unsafe { !(*spec).gen.no_analog } {
        err = unsafe { set_beep_amp(spec, 0x0b, 0x05, HDA_INPUT) };
        if err < 0 {
            unsafe {
                snd_hda_gen_remove(codec);
            }
            return err;
        }
    }

    unsafe {
        snd_hda_apply_fixup(codec, HDA_FIXUP_ACT_PROBE);
    }

    0
}

static ALC861VD_CODEC_OPS: hda_codec_ops = hda_codec_ops {
    probe: Some(alc861vd_probe),
    remove: Some(snd_hda_gen_remove),
    build_controls: Some(alc_build_controls),
    build_pcms: Some(snd_hda_gen_build_pcms),
    init: Some(alc_init),
    unsol_event: Some(snd_hda_jack_unsol_event),
    resume: Some(alc_resume),
    suspend: Some(alc_suspend),
    check_power_status: Some(snd_hda_gen_check_power_status),
    stream_pm: Some(snd_hda_gen_stream_pm),
};

/*
 * driver entries
 */
const fn hda_codec_id(_id: u32, _name: *const u8) -> hda_device_id {
    hda_device_id { _private: [] }
}

static SND_HDA_ID_ALC861VD: [hda_device_id; 3] = [
    hda_codec_id(0x10ec0660, b"ALC660-VD\0".as_ptr()),
    hda_codec_id(0x10ec0862, b"ALC861-VD\0".as_ptr()),
    hda_device_id { _private: [] }, /* terminator */
];
// MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_alc861vd);

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Realtek ALC861-VD HD-audio codec");
// MODULE_IMPORT_NS("SND_HDA_CODEC_REALTEK");

static mut ALC861VD_DRIVER: hda_codec_driver = hda_codec_driver {
    id: SND_HDA_ID_ALC861VD.as_ptr(),
    ops: &ALC861VD_CODEC_OPS,
};

// module_hda_codec_driver(alc861vd_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
