// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Nvidia Tegra HDMI codec support
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type hda_nid_t = c_uint;

const MODEL_TEGRA: c_ulong = 0;
const MODEL_TEGRA234: c_ulong = 1;

type c_ulong = usize;

/*
 * The HDA codec on NVIDIA Tegra contains two scratch registers that are
 * accessed using vendor-defined verbs. These registers can be used for
 * interoperability between the HDA and HDMI drivers.
 */

/* Audio Function Group node */
const NVIDIA_AFG_NID: c_uint = 0x01;

/*
 * The SCRATCH0 register is used to notify the HDMI codec of changes in audio
 * format. On Tegra, bit 31 is used as a trigger that causes an interrupt to
 * be raised in the HDMI codec. The remainder of the bits is arbitrary. This
 * implementation stores the HDA format (see AC_FMT_*) in bits [15:0] and an
 * additional bit (at position 30) to signal the validity of the format.
 *
 * | 31      | 30    | 29  16 | 15   0 |
 * +---------+-------+--------+--------+
 * | TRIGGER | VALID | UNUSED | FORMAT |
 * +-----------------------------------|
 *
 * Note that for the trigger bit to take effect it needs to change value
 * (i.e. it needs to be toggled). The trigger bit is not applicable from
 * TEGRA234 chip onwards, as new verb id 0xf80 will be used for interrupt
 * trigger to hdmi.
 */
const NVIDIA_SET_HOST_INTR: c_uint = 0xf80;
const NVIDIA_GET_SCRATCH0: c_uint = 0xfa6;
const NVIDIA_SET_SCRATCH0_BYTE0: c_uint = 0xfa7;
const NVIDIA_SET_SCRATCH0_BYTE1: c_uint = 0xfa8;
const NVIDIA_SET_SCRATCH0_BYTE2: c_uint = 0xfa9;
const NVIDIA_SET_SCRATCH0_BYTE3: c_uint = 0xfaa;
const NVIDIA_SCRATCH_TRIGGER: c_uint = 1 << 7;
const NVIDIA_SCRATCH_VALID: c_uint = 1 << 6;

const NVIDIA_GET_SCRATCH1: c_uint = 0xfab;
const NVIDIA_SET_SCRATCH1_BYTE0: c_uint = 0xfac;
const NVIDIA_SET_SCRATCH1_BYTE1: c_uint = 0xfad;
const NVIDIA_SET_SCRATCH1_BYTE2: c_uint = 0xfae;
const NVIDIA_SET_SCRATCH1_BYTE3: c_uint = 0xfaf;

extern "C" {
    fn snd_hda_codec_read(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        flags: c_int,
        verb: c_uint,
        parm: c_uint,
    ) -> c_uint;
    fn snd_hda_codec_write(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        flags: c_int,
        verb: c_uint,
        parm: c_uint,
    ) -> c_int;
    fn snd_hda_hdmi_generic_pcm_prepare(
        hinfo: *mut hda_pcm_stream,
        codec: *mut hda_codec,
        stream_tag: c_uint,
        format: c_uint,
        substream: *mut snd_pcm_substream,
    ) -> c_int;
    fn snd_hda_hdmi_generic_pcm_cleanup(
        hinfo: *mut hda_pcm_stream,
        codec: *mut hda_codec,
        substream: *mut snd_pcm_substream,
    ) -> c_int;
    fn snd_hda_hdmi_generic_build_pcms(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_parse_codec(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_generic_spec_free(codec: *mut hda_codec);
    fn snd_hda_hdmi_generic_init_per_pins(codec: *mut hda_codec);
    fn snd_hda_hdmi_generic_alloc(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_generic_remove(codec: *mut hda_codec);
    fn snd_hda_hdmi_generic_init(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_generic_build_controls(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_generic_unsol_event(codec: *mut hda_codec, res: c_uint);
    fn snd_hda_hdmi_generic_suspend(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_generic_resume(codec: *mut hda_codec) -> c_int;
}

const HDA_PCM_TYPE_HDMI: c_int = 3;
const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
const SNDRV_CTL_TLVT_CHMAP_FIXED: c_int = 1;
const SNDRV_CTL_TLVT_CHMAP_VAR: c_int = 2;
const SNDRV_CHMAP_FL: u8 = 3;
const SNDRV_CHMAP_FR: u8 = 4;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const AC_VERB_SET_DIGI_CONVERT_1: c_uint = 0x70d;
const AC_DIG1_ENABLE: c_uint = 0x01;

#[repr(C)]
struct hda_codec {
    spec: *mut hdmi_spec,
    dp_mst: bool,
    depop_delay: c_int,
}

#[repr(C)]
struct hdmi_spec {
    num_pins: c_uint,
    num_cvts: c_int,
    cvt_nids: *mut hda_nid_t,
    chmap: hdac_chmap,
    nv_dp_workaround: bool,
    dyn_pin_out: bool,
    hdmi_intr_trig_ctrl: bool,
}

#[repr(C)]
struct hdac_chmap {
    ops: hdac_chmap_ops,
}

#[repr(C)]
struct hdac_chmap_ops {
    chmap_cea_alloc_validate_get_type: Option<
        unsafe extern "C" fn(
            chmap: *mut hdac_chmap,
            cap: *mut hdac_cea_channel_speaker_allocation,
            channels: c_int,
        ) -> c_int,
    >,
    chmap_validate: Option<
        unsafe extern "C" fn(
            chmap: *mut hdac_chmap,
            ca: c_int,
            chs: c_int,
            map: *mut u8,
        ) -> c_int,
    >,
}

#[repr(C)]
struct hdac_cea_channel_speaker_allocation {
    ca_index: c_int,
    channels: c_int,
}

#[repr(C)]
struct hda_pcm {
    pcm_type: c_int,
    stream: [hda_pcm_stream; 2],
}

#[repr(C)]
struct hda_pcm_stream {
    nid: hda_nid_t,
    ops: hda_pcm_ops,
}

#[repr(C)]
struct hda_pcm_ops {
    prepare: Option<
        unsafe extern "C" fn(
            hinfo: *mut hda_pcm_stream,
            codec: *mut hda_codec,
            stream_tag: c_uint,
            format: c_uint,
            substream: *mut snd_pcm_substream,
        ) -> c_int,
    >,
    cleanup: Option<
        unsafe extern "C" fn(
            hinfo: *mut hda_pcm_stream,
            codec: *mut hda_codec,
            substream: *mut snd_pcm_substream,
        ) -> c_int,
    >,
}

#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
struct hda_device_id {
    vendor_id: c_uint,
    rev_id: c_uint,
    api_version: c_uint,
    name: *const c_char,
    driver_data: c_ulong,
}

#[repr(C)]
struct hda_codec_ops {
    probe: Option<unsafe extern "C" fn(codec: *mut hda_codec, id: *const hda_device_id) -> c_int>,
    remove: Option<unsafe extern "C" fn(codec: *mut hda_codec)>,
    init: Option<unsafe extern "C" fn(codec: *mut hda_codec) -> c_int>,
    build_pcms: Option<unsafe extern "C" fn(codec: *mut hda_codec) -> c_int>,
    build_controls: Option<unsafe extern "C" fn(codec: *mut hda_codec) -> c_int>,
    unsol_event: Option<unsafe extern "C" fn(codec: *mut hda_codec, res: c_uint)>,
    suspend: Option<unsafe extern "C" fn(codec: *mut hda_codec) -> c_int>,
    resume: Option<unsafe extern "C" fn(codec: *mut hda_codec) -> c_int>,
}

#[repr(C)]
struct hda_codec_driver {
    id: *const hda_device_id,
    ops: *const hda_codec_ops,
}

unsafe fn get_pcm_rec(spec: *mut hdmi_spec, i: c_uint) -> *mut hda_pcm {
    /*
     * External macro/helper from hdmi_local.h. Kept as a declaration-level
     * TODO because the isolated C source relies on the dependency.
     */
    extern "C" {
        fn get_pcm_rec(spec: *mut hdmi_spec, i: c_uint) -> *mut hda_pcm;
    }

    get_pcm_rec(spec, i)
}

/*
 * The format parameter is the HDA audio format (see AC_FMT_*). If set to 0,
 * the format is invalidated so that the HDMI codec can be disabled.
 */
unsafe extern "C" fn tegra_hdmi_set_format(
    codec: *mut hda_codec,
    cvt_nid: hda_nid_t,
    format: c_uint,
) {
    let mut value: c_uint;
    let mut nid: c_uint = NVIDIA_AFG_NID;
    let spec: *mut hdmi_spec = (*codec).spec;

    /*
     * Tegra HDA codec design from TEGRA234 chip onwards support DP MST.
     * This resulted in moving scratch registers from audio function
     * group to converter widget context. So CVT NID should be used for
     * scratch register read/write for DP MST supported Tegra HDA codec.
     */
    if (*codec).dp_mst {
        nid = cvt_nid;
    }

    /* bits [31:30] contain the trigger and valid bits */
    value = snd_hda_codec_read(codec, nid, 0, NVIDIA_GET_SCRATCH0, 0);
    value = (value >> 24) & 0xff;

    /* bits [15:0] are used to store the HDA format */
    snd_hda_codec_write(codec, nid, 0, NVIDIA_SET_SCRATCH0_BYTE0, (format >> 0) & 0xff);
    snd_hda_codec_write(codec, nid, 0, NVIDIA_SET_SCRATCH0_BYTE1, (format >> 8) & 0xff);

    /* bits [16:24] are unused */
    snd_hda_codec_write(codec, nid, 0, NVIDIA_SET_SCRATCH0_BYTE2, 0);

    /*
     * Bit 30 signals that the data is valid and hence that HDMI audio can
     * be enabled.
     */
    if format == 0 {
        value &= !NVIDIA_SCRATCH_VALID;
    } else {
        value |= NVIDIA_SCRATCH_VALID;
    }

    if (*spec).hdmi_intr_trig_ctrl {
        /*
         * For Tegra HDA Codec design from TEGRA234 onwards, the
         * Interrupt to hdmi driver is triggered by writing
         * non-zero values to verb 0xF80 instead of 31st bit of
         * scratch register.
         */
        snd_hda_codec_write(codec, nid, 0, NVIDIA_SET_SCRATCH0_BYTE3, value);
        snd_hda_codec_write(codec, nid, 0, NVIDIA_SET_HOST_INTR, 0x1);
    } else {
        /*
         * Whenever the 31st trigger bit is toggled, an interrupt is raised
         * in the HDMI codec. The HDMI driver will use that as trigger
         * to update its configuration.
         */
        value ^= NVIDIA_SCRATCH_TRIGGER;

        snd_hda_codec_write(codec, nid, 0, NVIDIA_SET_SCRATCH0_BYTE3, value);
    }
}

unsafe extern "C" fn tegra_hdmi_pcm_prepare(
    hinfo: *mut hda_pcm_stream,
    codec: *mut hda_codec,
    stream_tag: c_uint,
    format: c_uint,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let err: c_int;

    err = snd_hda_hdmi_generic_pcm_prepare(hinfo, codec, stream_tag, format, substream);
    if err < 0 {
        return err;
    }

    /* notify the HDMI codec of the format change */
    tegra_hdmi_set_format(codec, (*hinfo).nid, format);

    0
}

unsafe extern "C" fn tegra_hdmi_pcm_cleanup(
    hinfo: *mut hda_pcm_stream,
    codec: *mut hda_codec,
    substream: *mut snd_pcm_substream,
) -> c_int {
    /* invalidate the format in the HDMI codec */
    tegra_hdmi_set_format(codec, (*hinfo).nid, 0);

    snd_hda_hdmi_generic_pcm_cleanup(hinfo, codec, substream)
}

unsafe extern "C" fn hda_find_pcm_by_type(codec: *mut hda_codec, type_: c_int) -> *mut hda_pcm {
    let spec: *mut hdmi_spec = (*codec).spec;
    let mut i: c_uint;

    i = 0;
    while i < (*spec).num_pins {
        let pcm: *mut hda_pcm = get_pcm_rec(spec, i);

        if (*pcm).pcm_type == type_ {
            return pcm;
        }

        i += 1;
    }

    ptr::null_mut()
}

unsafe extern "C" fn tegra_hdmi_build_pcms(codec: *mut hda_codec) -> c_int {
    let stream: *mut hda_pcm_stream;
    let pcm: *mut hda_pcm;
    let err: c_int;

    err = snd_hda_hdmi_generic_build_pcms(codec);
    if err < 0 {
        return err;
    }

    pcm = hda_find_pcm_by_type(codec, HDA_PCM_TYPE_HDMI);
    if pcm.is_null() {
        return -ENODEV;
    }

    /*
     * Override ->prepare() and ->cleanup() operations to notify the HDMI
     * codec about format changes.
     */
    stream = &mut (*pcm).stream[SNDRV_PCM_STREAM_PLAYBACK];
    (*stream).ops.prepare = Some(tegra_hdmi_pcm_prepare);
    (*stream).ops.cleanup = Some(tegra_hdmi_pcm_cleanup);

    0
}

/*
 * NVIDIA codecs ignore ASP mapping for 2ch - confirmed on:
 * - 0x10de0015
 * - 0x10de0040
 */
unsafe extern "C" fn nvhdmi_chmap_cea_alloc_validate_get_type(
    _chmap: *mut hdac_chmap,
    cap: *mut hdac_cea_channel_speaker_allocation,
    channels: c_int,
) -> c_int {
    if (*cap).ca_index == 0x00 && channels == 2 {
        return SNDRV_CTL_TLVT_CHMAP_FIXED;
    }

    /* If the speaker allocation matches the channel count, it is OK. */
    if (*cap).channels != channels {
        return -1;
    }

    /* all channels are remappable freely */
    SNDRV_CTL_TLVT_CHMAP_VAR
}

unsafe extern "C" fn nvhdmi_chmap_validate(
    _chmap: *mut hdac_chmap,
    ca: c_int,
    _chs: c_int,
    map: *mut u8,
) -> c_int {
    if ca == 0x00 && (*map.add(0) != SNDRV_CHMAP_FL || *map.add(1) != SNDRV_CHMAP_FR) {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn tegra_hdmi_init(codec: *mut hda_codec) -> c_int {
    let spec: *mut hdmi_spec = (*codec).spec;
    let mut i: c_int;
    let err: c_int;

    err = snd_hda_hdmi_parse_codec(codec);
    if err < 0 {
        snd_hda_hdmi_generic_spec_free(codec);
        return err;
    }

    i = 0;
    while i < (*spec).num_cvts {
        snd_hda_codec_write(
            codec,
            *(*spec).cvt_nids.add(i as usize),
            0,
            AC_VERB_SET_DIGI_CONVERT_1,
            AC_DIG1_ENABLE,
        );
        i += 1;
    }

    snd_hda_hdmi_generic_init_per_pins(codec);

    (*codec).depop_delay = 10;
    (*spec).chmap.ops.chmap_cea_alloc_validate_get_type =
        Some(nvhdmi_chmap_cea_alloc_validate_get_type);
    (*spec).chmap.ops.chmap_validate = Some(nvhdmi_chmap_validate);

    (*spec).chmap.ops.chmap_cea_alloc_validate_get_type =
        Some(nvhdmi_chmap_cea_alloc_validate_get_type);
    (*spec).chmap.ops.chmap_validate = Some(nvhdmi_chmap_validate);
    (*spec).nv_dp_workaround = true;

    0
}

unsafe extern "C" fn tegrahdmi_probe(codec: *mut hda_codec, id: *const hda_device_id) -> c_int {
    let spec: *mut hdmi_spec;
    let err: c_int;

    err = snd_hda_hdmi_generic_alloc(codec);
    if err < 0 {
        return err;
    }

    if (*id).driver_data == MODEL_TEGRA234 {
        (*codec).dp_mst = true;
        spec = (*codec).spec;
        (*spec).dyn_pin_out = true;
        (*spec).hdmi_intr_trig_ctrl = true;
    }

    tegra_hdmi_init(codec)
}

#[no_mangle]
static tegrahdmi_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(tegrahdmi_probe),
    remove: Some(snd_hda_hdmi_generic_remove),
    init: Some(snd_hda_hdmi_generic_init),
    build_pcms: Some(tegra_hdmi_build_pcms),
    build_controls: Some(snd_hda_hdmi_generic_build_controls),
    unsol_event: Some(snd_hda_hdmi_generic_unsol_event),
    suspend: Some(snd_hda_hdmi_generic_suspend),
    resume: Some(snd_hda_hdmi_generic_resume),
};

macro_rules! HDA_CODEC_ID_MODEL {
    ($id:expr, $name:expr, $data:expr) => {
        hda_device_id {
            vendor_id: $id,
            rev_id: 0,
            api_version: 0,
            name: concat!($name, "\0").as_ptr() as *const c_char,
            driver_data: $data,
        }
    };
}

#[no_mangle]
static snd_hda_id_tegrahdmi: [hda_device_id; 14] = [
    HDA_CODEC_ID_MODEL!(0x10de0020, "Tegra30 HDMI", MODEL_TEGRA),
    HDA_CODEC_ID_MODEL!(0x10de0022, "Tegra114 HDMI", MODEL_TEGRA),
    HDA_CODEC_ID_MODEL!(0x10de0028, "Tegra124 HDMI", MODEL_TEGRA),
    HDA_CODEC_ID_MODEL!(0x10de0029, "Tegra210 HDMI/DP", MODEL_TEGRA),
    HDA_CODEC_ID_MODEL!(0x10de002d, "Tegra186 HDMI/DP0", MODEL_TEGRA),
    HDA_CODEC_ID_MODEL!(0x10de002e, "Tegra186 HDMI/DP1", MODEL_TEGRA),
    HDA_CODEC_ID_MODEL!(0x10de002f, "Tegra194 HDMI/DP2", MODEL_TEGRA),
    HDA_CODEC_ID_MODEL!(0x10de0030, "Tegra194 HDMI/DP3", MODEL_TEGRA),
    HDA_CODEC_ID_MODEL!(0x10de0031, "Tegra234 HDMI/DP", MODEL_TEGRA234),
    HDA_CODEC_ID_MODEL!(0x10de0032, "Tegra238 HDMI/DP", MODEL_TEGRA234),
    HDA_CODEC_ID_MODEL!(0x10de0033, "SoC 33 HDMI/DP", MODEL_TEGRA234),
    HDA_CODEC_ID_MODEL!(0x10de0034, "Tegra264 HDMI/DP", MODEL_TEGRA234),
    HDA_CODEC_ID_MODEL!(0x10de0035, "SoC 35 HDMI/DP", MODEL_TEGRA234),
    hda_device_id {
        vendor_id: 0,
        rev_id: 0,
        api_version: 0,
        name: ptr::null(),
        driver_data: 0,
    }, /* terminator */
];
/* MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_tegrahdmi); */

/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("Nvidia Tegra HDMI HD-audio codec"); */
/* MODULE_IMPORT_NS("SND_HDA_CODEC_HDMI"); */

#[no_mangle]
static mut tegrahdmi_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_tegrahdmi.as_ptr(),
    ops: &tegrahdmi_codec_ops,
};

/* module_hda_codec_driver(tegrahdmi_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
