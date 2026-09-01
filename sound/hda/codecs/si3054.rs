// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Universal Interface for Intel High Definition Audio Codec
 *
 * HD audio codec driver for Silicon Labs 3054/5 modem codec
 *
 * Copyright (c) 2005 Sasha Khapyorsky <sashak@alsa-project.org>
 *                    Takashi Iwai <tiwai@suse.de>
 */

// C includes translated as external dependencies:
// <linux/init.h>, <linux/delay.h>, <linux/slab.h>, <linux/module.h>,
// <sound/core.h>, <sound/hda_codec.h>, "hda_local.h"

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u16 = u16;

/* si3054 verbs */
const SI3054_VERB_READ_NODE: c_uint = 0x900;
const SI3054_VERB_WRITE_NODE: c_uint = 0x100;

/* si3054 nodes (registers) */
const SI3054_EXTENDED_MID: c_uint = 2;
const SI3054_LINE_RATE: c_uint = 3;
const SI3054_LINE_LEVEL: c_uint = 4;
const SI3054_GPIO_CFG: c_uint = 5;
const SI3054_GPIO_POLARITY: c_uint = 6;
const SI3054_GPIO_STICKY: c_uint = 7;
const SI3054_GPIO_WAKEUP: c_uint = 8;
const SI3054_GPIO_STATUS: c_uint = 9;
const SI3054_GPIO_CONTROL: c_uint = 10;
const SI3054_MISC_AFE: c_uint = 11;
const SI3054_CHIPID: c_uint = 12;
const SI3054_LINE_CFG1: c_uint = 13;
const SI3054_LINE_STATUS: c_uint = 14;
const SI3054_DC_TERMINATION: c_uint = 15;
const SI3054_LINE_CONFIG: c_uint = 16;
const SI3054_CALLPROG_ATT: c_uint = 17;
const SI3054_SQ_CONTROL: c_uint = 18;
const SI3054_MISC_CONTROL: c_uint = 19;
const SI3054_RING_CTRL1: c_uint = 20;
const SI3054_RING_CTRL2: c_uint = 21;

/* extended MID */
const SI3054_MEI_READY: c_uint = 0xf;

/* line level */
const SI3054_ATAG_MASK: c_uint = 0x00f0;
const SI3054_DTAG_MASK: c_uint = 0xf000;

/* GPIO bits */
const SI3054_GPIO_OH: c_uint = 0x0001;
const SI3054_GPIO_CID: c_uint = 0x0002;

/* chipid and revisions */
const SI3054_CHIPID_CODEC_REV_MASK: c_uint = 0x000f;
const SI3054_CHIPID_DAA_REV_MASK: c_uint = 0x00f0;
const SI3054_CHIPID_INTERNATIONAL: c_uint = 0x0100;
const SI3054_CHIPID_DAA_ID: c_uint = 0x0f00;
const SI3054_CHIPID_CODEC_ID: c_uint = 1 << 12;

const ENOMEM: c_int = 12;
const AC_NODE_ROOT: c_uint = 0;
const AC_VERB_SET_CODEC_RESET: c_uint = 0;
const AC_VERB_SET_STREAM_FORMAT: c_uint = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 0;
const HDA_SUBDEV_NID_FLAG: c_uint = 0;
const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
const SNDRV_PCM_STREAM_CAPTURE: usize = 1;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;
const SNDRV_PCM_RATE_8000: c_uint = 0;
const SNDRV_PCM_RATE_16000: c_uint = 0;
const SNDRV_PCM_RATE_KNOT: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 0;
const HDA_PCM_TYPE_MODEM: c_uint = 0;

#[repr(C)]
struct hda_codec_core {
    mfg: c_uint,
}

#[repr(C)]
struct hda_codec {
    core: hda_codec_core,
    spec: *mut c_void,
}

#[repr(C)]
struct hda_device_id {
    vendor_id: c_uint,
    name: *const c_char,
}

#[repr(C)]
struct snd_kcontrol {
    private_value: c_ulong,
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
struct snd_ctl_elem_value_integer {
    value: [c_long; 1],
}

type c_long = isize;

#[repr(C)]
struct snd_kcontrol_new {
    iface: c_uint,
    name: *const c_char,
    subdevice: c_uint,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    private_value: c_ulong,
}

#[repr(C)]
struct hda_pcm_stream {
    substreams: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
    nid: c_uint,
    rates: c_uint,
    formats: c_uint,
    maxbps: c_uint,
    ops: hda_pcm_stream_ops,
}

#[repr(C)]
struct hda_pcm_stream_ops {
    open: Option<unsafe extern "C" fn(*mut hda_pcm_stream, *mut hda_codec, *mut snd_pcm_substream) -> c_int>,
    prepare: Option<
        unsafe extern "C" fn(
            *mut hda_pcm_stream,
            *mut hda_codec,
            c_uint,
            c_uint,
            *mut snd_pcm_substream,
        ) -> c_int,
    >,
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    stream: usize,
}

#[repr(C)]
struct snd_pcm_runtime {
    rate: c_uint,
    hw: snd_pcm_hardware,
}

#[repr(C)]
struct snd_pcm_hardware {
    period_bytes_min: c_uint,
}

#[repr(C)]
struct snd_pcm_hw_constraint_list {
    count: c_uint,
    list: *const c_uint,
    mask: c_uint,
}

#[repr(C)]
struct hda_pcm {
    stream: [hda_pcm_stream; 2],
    pcm_type: c_uint,
}

#[repr(C)]
struct hda_codec_ops {
    probe: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_device_id) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut hda_codec)>,
    build_controls: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    build_pcms: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    init: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
}

#[repr(C)]
struct hda_codec_driver {
    id: *const hda_device_id,
    ops: *const hda_codec_ops,
}

#[repr(C)]
struct si3054_spec {
    international: c_uint,
}

unsafe extern "C" {
    fn snd_hda_codec_read(codec: *mut hda_codec, nid: c_uint, flags: c_uint, verb: c_uint, parm: c_uint) -> c_uint;
    fn snd_hda_codec_write(codec: *mut hda_codec, nid: c_uint, flags: c_uint, verb: c_uint, parm: c_uint) -> c_uint;
    fn snd_hda_codec_write_cache(codec: *mut hda_codec, nid: c_uint, flags: c_uint, verb: c_uint, parm: c_uint) -> c_uint;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_value) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut hda_codec;
    fn snd_hda_add_new_ctls(codec: *mut hda_codec, kcontrols: *const snd_kcontrol_new) -> c_int;
    fn snd_hda_codec_setup_stream(codec: *mut hda_codec, nid: c_uint, stream_tag: c_uint, channel_id: c_uint, format: c_uint);
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_hda_codec_pcm_new(codec: *mut hda_codec, name: *const c_char) -> *mut hda_pcm;
    fn snd_hdac_regmap_add_vendor_verb(codec: *mut hda_codec_core, verb: c_uint) -> c_int;
    fn msleep(msecs: c_uint);
    fn codec_err(codec: *mut hda_codec, fmt: *const c_char, ...);
    fn codec_dbg(codec: *mut hda_codec, fmt: *const c_char, ...);
    fn kfree(ptr: *mut c_void);
    fn kzalloc_obj_si3054_spec() -> *mut si3054_spec;
}

/* si3054 codec registers (nodes) access macros */
unsafe fn GET_REG(codec: *mut hda_codec, reg: c_uint) -> c_uint {
    unsafe { snd_hda_codec_read(codec, reg, 0, SI3054_VERB_READ_NODE, 0) }
}

unsafe fn SET_REG(codec: *mut hda_codec, reg: c_uint, val: c_uint) -> c_uint {
    unsafe { snd_hda_codec_write(codec, reg, 0, SI3054_VERB_WRITE_NODE, val) }
}

unsafe fn SET_REG_CACHE(codec: *mut hda_codec, reg: c_uint, val: c_uint) -> c_uint {
    unsafe { snd_hda_codec_write_cache(codec, reg, 0, SI3054_VERB_WRITE_NODE, val) }
}

/*
 * Modem mixer
 */

const fn PRIVATE_VALUE(reg: c_uint, mask: c_uint) -> c_ulong {
    (((reg << 16) | (mask & 0xffff)) as c_ulong)
}

const fn PRIVATE_REG(val: c_ulong) -> u16 {
    (((val >> 16) & 0xffff) as u16)
}

const fn PRIVATE_MASK(val: c_ulong) -> u16 {
    ((val & 0xffff) as u16)
}

const si3054_switch_info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int> =
    Some(snd_ctl_boolean_mono_info);

unsafe extern "C" fn si3054_switch_get(
    kcontrol: *mut snd_kcontrol,
    uvalue: *mut snd_ctl_elem_value,
) -> c_int {
    let codec: *mut hda_codec = unsafe { snd_kcontrol_chip(kcontrol) };
    let reg: u16 = PRIVATE_REG(unsafe { (*kcontrol).private_value });
    let mask: u16 = PRIVATE_MASK(unsafe { (*kcontrol).private_value });
    unsafe {
        (*uvalue).value.integer.value[0] = if (GET_REG(codec, reg as c_uint) & mask as c_uint) != 0 {
            1
        } else {
            0
        };
    }
    0
}

unsafe extern "C" fn si3054_switch_put(
    kcontrol: *mut snd_kcontrol,
    uvalue: *mut snd_ctl_elem_value,
) -> c_int {
    let codec: *mut hda_codec = unsafe { snd_kcontrol_chip(kcontrol) };
    let reg: u16 = PRIVATE_REG(unsafe { (*kcontrol).private_value });
    let mask: u16 = PRIVATE_MASK(unsafe { (*kcontrol).private_value });
    unsafe {
        if (*uvalue).value.integer.value[0] != 0 {
            SET_REG_CACHE(codec, reg as c_uint, GET_REG(codec, reg as c_uint) | mask as c_uint);
        } else {
            SET_REG_CACHE(codec, reg as c_uint, GET_REG(codec, reg as c_uint) & !(mask as c_uint));
        }
    }
    0
}

const fn SI3054_KCONTROL(kname: *const c_char, reg: c_uint, mask: c_uint) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: kname,
        subdevice: HDA_SUBDEV_NID_FLAG | reg,
        info: si3054_switch_info,
        get: Some(si3054_switch_get),
        put: Some(si3054_switch_put),
        private_value: PRIVATE_VALUE(reg, mask),
    }
}

static si3054_modem_mixer: [snd_kcontrol_new; 3] = [
    SI3054_KCONTROL(c"Off-hook Switch".as_ptr(), SI3054_GPIO_CONTROL, SI3054_GPIO_OH),
    SI3054_KCONTROL(c"Caller ID Switch".as_ptr(), SI3054_GPIO_CONTROL, SI3054_GPIO_CID),
    snd_kcontrol_new {
        iface: 0,
        name: core::ptr::null(),
        subdevice: 0,
        info: None,
        get: None,
        put: None,
        private_value: 0,
    },
];

unsafe extern "C" fn si3054_build_controls(codec: *mut hda_codec) -> c_int {
    unsafe { snd_hda_add_new_ctls(codec, si3054_modem_mixer.as_ptr()) }
}

/*
 * PCM callbacks
 */

unsafe extern "C" fn si3054_pcm_prepare(
    hinfo: *mut hda_pcm_stream,
    codec: *mut hda_codec,
    stream_tag: c_uint,
    format: c_uint,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let mut val: u16;

    unsafe {
        SET_REG(codec, SI3054_LINE_RATE, (*(*substream).runtime).rate);
        val = GET_REG(codec, SI3054_LINE_LEVEL) as u16;
        val &= (0xffu16) << (8 * ((*substream).stream != SNDRV_PCM_STREAM_PLAYBACK) as u16);
        val |= (((stream_tag & 0xf) << 4) << (8 * ((*substream).stream == SNDRV_PCM_STREAM_PLAYBACK) as c_uint)) as u16;
        SET_REG(codec, SI3054_LINE_LEVEL, val as c_uint);

        snd_hda_codec_setup_stream(codec, (*hinfo).nid, stream_tag, 0, format);
    }
    0
}

unsafe extern "C" fn si3054_pcm_open(
    _hinfo: *mut hda_pcm_stream,
    _codec: *mut hda_codec,
    substream: *mut snd_pcm_substream,
) -> c_int {
    static rates: [c_uint; 3] = [8000, 9600, 16000];
    static hw_constraints_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
        count: rates.len() as c_uint,
        list: rates.as_ptr(),
        mask: 0,
    };
    unsafe {
        (*(*substream).runtime).hw.period_bytes_min = 80;
        snd_pcm_hw_constraint_list(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            &hw_constraints_rates,
        )
    }
}

static si3054_pcm: hda_pcm_stream = hda_pcm_stream {
    substreams: 1,
    channels_min: 1,
    channels_max: 1,
    nid: 0x1,
    rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_KNOT,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    maxbps: 16,
    ops: hda_pcm_stream_ops {
        open: Some(si3054_pcm_open),
        prepare: Some(si3054_pcm_prepare),
    },
};

unsafe extern "C" fn si3054_build_pcms(codec: *mut hda_codec) -> c_int {
    let info: *mut hda_pcm;

    unsafe {
        info = snd_hda_codec_pcm_new(codec, c"Si3054 Modem".as_ptr());
        if info.is_null() {
            return -ENOMEM;
        }
        (*info).stream[SNDRV_PCM_STREAM_PLAYBACK] = si3054_pcm;
        (*info).stream[SNDRV_PCM_STREAM_CAPTURE] = si3054_pcm;
        (*info).stream[SNDRV_PCM_STREAM_PLAYBACK].nid = (*codec).core.mfg;
        (*info).stream[SNDRV_PCM_STREAM_CAPTURE].nid = (*codec).core.mfg;
        (*info).pcm_type = HDA_PCM_TYPE_MODEM;
    }
    0
}

/*
 * Init part
 */

unsafe extern "C" fn si3054_init(codec: *mut hda_codec) -> c_int {
    let spec: *mut si3054_spec = unsafe { (*codec).spec as *mut si3054_spec };
    let mut wait_count: c_uint;
    let mut val: u16;

    unsafe {
        if snd_hdac_regmap_add_vendor_verb(&mut (*codec).core, SI3054_VERB_WRITE_NODE) != 0 {
            return -ENOMEM;
        }

        snd_hda_codec_write(codec, AC_NODE_ROOT, 0, AC_VERB_SET_CODEC_RESET, 0);
        snd_hda_codec_write(codec, (*codec).core.mfg, 0, AC_VERB_SET_STREAM_FORMAT, 0);
        SET_REG(codec, SI3054_LINE_RATE, 9600);
        SET_REG(codec, SI3054_LINE_LEVEL, SI3054_DTAG_MASK | SI3054_ATAG_MASK);
        SET_REG(codec, SI3054_EXTENDED_MID, 0);

        wait_count = 10;
        loop {
            msleep(2);
            val = GET_REG(codec, SI3054_EXTENDED_MID) as u16;
            let continue_loop =
                (val as c_uint & SI3054_MEI_READY) != SI3054_MEI_READY && {
                    let old = wait_count;
                    wait_count = wait_count.wrapping_sub(1);
                    old != 0
                };
            if !continue_loop {
                break;
            }
        }

        if (val as c_uint & SI3054_MEI_READY) != SI3054_MEI_READY {
            codec_err(codec, c"si3054: cannot initialize. EXT MID = %04x\n".as_ptr(), val as c_uint);
            /* let's pray that this is no fatal error */
            /* return -EACCES; */
        }

        SET_REG(codec, SI3054_GPIO_POLARITY, 0xffff);
        SET_REG(codec, SI3054_GPIO_CFG, 0x0);
        SET_REG(codec, SI3054_MISC_AFE, 0);
        SET_REG(codec, SI3054_LINE_CFG1, 0x200);

        if (GET_REG(codec, SI3054_LINE_STATUS) & (1 << 6)) == 0 {
            codec_dbg(
                codec,
                c"Link Frame Detect(FDT) is not ready (line status: %04x)\n".as_ptr(),
                GET_REG(codec, SI3054_LINE_STATUS),
            );
        }

        (*spec).international = GET_REG(codec, SI3054_CHIPID) & SI3054_CHIPID_INTERNATIONAL;
    }

    0
}

unsafe extern "C" fn si3054_remove(codec: *mut hda_codec) {
    unsafe {
        kfree((*codec).spec);
    }
}

/*
 */

unsafe extern "C" fn si3054_probe(codec: *mut hda_codec, _id: *const hda_device_id) -> c_int {
    unsafe {
        (*codec).spec = kzalloc_obj_si3054_spec() as *mut c_void;
        if (*codec).spec.is_null() {
            return -ENOMEM;
        }
    }
    0
}

static si3054_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(si3054_probe),
    remove: Some(si3054_remove),
    build_controls: Some(si3054_build_controls),
    build_pcms: Some(si3054_build_pcms),
    init: Some(si3054_init),
};

/*
 * driver entries
 */
const fn HDA_CODEC_ID(id: c_uint, name: *const c_char) -> hda_device_id {
    hda_device_id {
        vendor_id: id,
        name,
    }
}

static snd_hda_id_si3054: [hda_device_id; 12] = [
    HDA_CODEC_ID(0x163c3055, c"Si3054".as_ptr()),
    HDA_CODEC_ID(0x163c3155, c"Si3054".as_ptr()),
    HDA_CODEC_ID(0x11c13026, c"Si3054".as_ptr()),
    HDA_CODEC_ID(0x11c13055, c"Si3054".as_ptr()),
    HDA_CODEC_ID(0x11c13155, c"Si3054".as_ptr()),
    HDA_CODEC_ID(0x10573055, c"Si3054".as_ptr()),
    HDA_CODEC_ID(0x10573057, c"Si3054".as_ptr()),
    HDA_CODEC_ID(0x10573155, c"Si3054".as_ptr()),
    /* VIA HDA on Clevo m540 */
    HDA_CODEC_ID(0x11063288, c"Si3054".as_ptr()),
    /* Asus A8J Modem (SM56) */
    HDA_CODEC_ID(0x15433155, c"Si3054".as_ptr()),
    /* LG LW20 modem */
    HDA_CODEC_ID(0x18540018, c"Si3054".as_ptr()),
    hda_device_id {
        vendor_id: 0,
        name: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_si3054);

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Si3054 HD-audio modem codec");

static mut si3054_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_si3054.as_ptr(),
    ops: &si3054_codec_ops,
};

// module_hda_codec_driver(si3054_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
