// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Non-generic simple HDMI codec support
 */

// C dependencies: linux/slab.h, linux/module.h, hdmi_local.h, hda_jack.h

use core::ffi::{c_char, c_int, c_uint, c_void};

type hda_nid_t = c_uint;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const HDA_PCM_TYPE_HDMI: c_int = 3;
const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 10;
const SND_JACK_AVOUT: c_int = 0x0008;
const AC_VERB_SET_PIN_WIDGET_CONTROL: c_uint = 0x707;
const AC_VERB_SET_AMP_GAIN_MUTE: c_uint = 0x300;
const PIN_OUT: c_uint = 0x40;
const AC_WCAP_OUT_AMP: c_uint = 0x00040000;
const AMP_OUT_UNMUTE: c_uint = 0xb000;

const MODEL_VIA: usize = 0;

/* VIA HDMI Implementation */
const VIAHDMI_CVT_NID: hda_nid_t = 0x02; /* audio converter1 */
const VIAHDMI_PIN_NID: hda_nid_t = 0x03; /* HDMI output pin1 */

#[repr(C)]
pub struct hda_codec {
    pub spec: *mut hdmi_spec,
    pub card: *mut c_void,
}

#[repr(C)]
pub struct hdmi_spec {
    pub codec: *mut hda_codec,
    pub pins: snd_array,
    pub cvts: snd_array,
    pub multiout: hda_multi_out,
    pub num_cvts: c_int,
    pub num_pins: c_int,
    pub pcm_rec: [hdmi_pcm; 1],
    pub pcm_playback: hda_pcm_stream,
    pub hw_constraints_channels: *mut c_void,
}

#[repr(C)]
pub struct snd_array {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hda_multi_out {
    pub num_dacs: c_int,
    pub max_channels: c_int,
    pub dig_out_nid: hda_nid_t,
}

#[repr(C)]
pub struct hdmi_pcm {
    pub pcm: *mut hda_pcm,
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct hda_pcm {
    pub pcm_type: c_int,
    pub stream: [hda_pcm_stream; 2],
    pub device: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_pcm_stream {
    pub substreams: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub nid: hda_nid_t,
    pub ops: hda_pcm_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_pcm_ops {
    pub open: Option<
        unsafe extern "C" fn(
            *mut hda_pcm_stream,
            *mut hda_codec,
            *mut snd_pcm_substream,
        ) -> c_int,
    >,
    pub close: Option<
        unsafe extern "C" fn(
            *mut hda_pcm_stream,
            *mut hda_codec,
            *mut snd_pcm_substream,
        ) -> c_int,
    >,
    pub prepare: Option<
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
pub struct hdmi_spec_per_cvt {
    pub cvt_nid: hda_nid_t,
}

#[repr(C)]
pub struct hdmi_spec_per_pin {
    pub pin_nid: hda_nid_t,
    pub dev_id: c_int,
}

#[repr(C)]
pub struct snd_jack {
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_jack)>,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut c_void,
}

#[repr(C)]
pub struct hda_device_id {
    pub driver_data: usize,
}

#[repr(C)]
pub struct hda_codec_ops {
    pub probe: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub build_controls: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub build_pcms: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub unsol_event: Option<unsafe extern "C" fn(*mut hda_codec, c_uint)>,
}

#[repr(C)]
pub struct hda_codec_driver {
    pub id: *const hda_device_id,
    pub ops: *const hda_codec_ops,
}

unsafe extern "C" {
    fn get_cvt(spec: *mut hdmi_spec, idx: c_int) -> *mut hdmi_spec_per_cvt;
    fn get_pin(spec: *mut hdmi_spec, idx: c_int) -> *mut hdmi_spec_per_pin;
    fn get_hdmi_pcm(spec: *mut hdmi_spec, idx: c_int) -> *mut hdmi_pcm;
    fn get_wcaps(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
    fn get_wcaps_channels(wcaps: c_uint) -> c_uint;
    fn snd_hda_codec_pcm_new(codec: *mut hda_codec, name: *const c_char) -> *mut hda_pcm;
    fn snd_hda_jack_set_dirty_all(codec: *mut hda_codec);
    fn snd_hda_jack_report_sync(codec: *mut hda_codec);
    fn strlen(s: *const c_char) -> usize;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snd_jack_new(
        card: *mut c_void,
        id: *const c_char,
        type_: c_int,
        jack: *mut *mut snd_jack,
        initial_kctl: bool,
        phantom_jack: bool,
    ) -> c_int;
    fn snd_hda_create_dig_out_ctls(
        codec: *mut hda_codec,
        associated_nid: hda_nid_t,
        cvt_nid: hda_nid_t,
        type_: c_int,
    ) -> c_int;
    fn snd_hda_codec_write(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        flags: c_int,
        verb: c_uint,
        parm: c_uint,
    );
    fn snd_hda_jack_detect_enable(codec: *mut hda_codec, nid: hda_nid_t, dev_id: c_int);
    fn snd_array_free(array: *mut snd_array);
    fn kfree(ptr: *mut c_void);
    fn snd_pcm_hw_constraint_list(
        runtime: *mut c_void,
        cond: c_uint,
        var: c_int,
        l: *mut c_void,
    ) -> c_int;
    fn snd_pcm_hw_constraint_step(
        runtime: *mut c_void,
        cond: c_uint,
        var: c_int,
        step: c_uint,
    ) -> c_int;
    fn snd_hda_multi_out_dig_open(codec: *mut hda_codec, mout: *mut hda_multi_out) -> c_int;
    fn snd_hda_multi_out_dig_close(codec: *mut hda_codec, mout: *mut hda_multi_out) -> c_int;
    fn snd_hda_multi_out_dig_prepare(
        codec: *mut hda_codec,
        mout: *mut hda_multi_out,
        stream_tag: c_uint,
        format: c_uint,
        substream: *mut snd_pcm_substream,
    ) -> c_int;
    fn kzalloc_obj_hdmi_spec() -> *mut hdmi_spec;
    fn snd_array_init(array: *mut snd_array, elem_size: usize, alloc_align: c_uint);
    fn snd_array_new(array: *mut snd_array) -> *mut c_void;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_hdmi_simple_build_pcms(codec: *mut hda_codec) -> c_int {
    let spec = unsafe { (*codec).spec };
    let info: *mut hda_pcm;
    let mut chans: c_uint;
    let pstr: *mut hda_pcm_stream;
    let per_cvt: *mut hdmi_spec_per_cvt;

    per_cvt = unsafe { get_cvt(spec, 0) };
    chans = unsafe { get_wcaps(codec, (*per_cvt).cvt_nid) };
    chans = unsafe { get_wcaps_channels(chans) };

    info = unsafe { snd_hda_codec_pcm_new(codec, c"HDMI 0".as_ptr()) };
    if info.is_null() {
        return -ENOMEM;
    }
    unsafe {
        (*spec).pcm_rec[0].pcm = info;
        (*info).pcm_type = HDA_PCM_TYPE_HDMI;
    }
    pstr = unsafe { &mut (*info).stream[SNDRV_PCM_STREAM_PLAYBACK] };
    unsafe {
        *pstr = (*spec).pcm_playback;
        (*pstr).nid = (*per_cvt).cvt_nid;
        if (*pstr).channels_max <= 2 && chans != 0 && chans <= 16 {
            (*pstr).channels_max = chans;
        }
    }

    0
}
// EXPORT_SYMBOL_NS_GPL(snd_hda_hdmi_simple_build_pcms, "SND_HDA_CODEC_HDMI");

/* unsolicited event for jack sensing */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_hdmi_simple_unsol_event(
    codec: *mut hda_codec,
    _res: c_uint,
) {
    unsafe {
        snd_hda_jack_set_dirty_all(codec);
        snd_hda_jack_report_sync(codec);
    }
}
// EXPORT_SYMBOL_NS_GPL(snd_hda_hdmi_simple_unsol_event, "SND_HDA_CODEC_HDMI");

unsafe extern "C" fn free_hdmi_jack_priv(jack: *mut snd_jack) {
    let pcm = unsafe { (*jack).private_data as *mut hdmi_pcm };

    unsafe {
        (*pcm).jack = core::ptr::null_mut();
    }
}

unsafe extern "C" fn simple_hdmi_build_jack(codec: *mut hda_codec) -> c_int {
    let mut hdmi_str = [0 as c_char; 32];
    hdmi_str[0] = b'H' as c_char;
    hdmi_str[1] = b'D' as c_char;
    hdmi_str[2] = b'M' as c_char;
    hdmi_str[3] = b'I' as c_char;
    hdmi_str[4] = b'/' as c_char;
    hdmi_str[5] = b'D' as c_char;
    hdmi_str[6] = b'P' as c_char;
    let spec = unsafe { (*codec).spec };
    let mut jack: *mut snd_jack = core::ptr::null_mut();
    let pcmp = unsafe { get_hdmi_pcm(spec, 0) };
    let pcmdev = unsafe { (*(*pcmp).pcm).device };
    let err: c_int;

    if pcmdev > 0 {
        unsafe {
            sprintf(
                hdmi_str.as_mut_ptr().add(strlen(hdmi_str.as_ptr())),
                c",pcm=%d".as_ptr(),
                pcmdev,
            );
        }
    }

    err = unsafe {
        snd_jack_new(
            (*codec).card,
            hdmi_str.as_ptr(),
            SND_JACK_AVOUT,
            &mut jack,
            true,
            false,
        )
    };
    if err < 0 {
        return err;
    }

    unsafe {
        (*pcmp).jack = jack;
        (*jack).private_data = pcmp as *mut c_void;
        (*jack).private_free = Some(free_hdmi_jack_priv);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_hdmi_simple_build_controls(codec: *mut hda_codec) -> c_int {
    let spec = unsafe { (*codec).spec };
    let per_cvt: *mut hdmi_spec_per_cvt;
    let err: c_int;

    per_cvt = unsafe { get_cvt(spec, 0) };
    err = unsafe {
        snd_hda_create_dig_out_ctls(
            codec,
            (*per_cvt).cvt_nid,
            (*per_cvt).cvt_nid,
            HDA_PCM_TYPE_HDMI,
        )
    };
    if err < 0 {
        return err;
    }
    unsafe { simple_hdmi_build_jack(codec) }
}
// EXPORT_SYMBOL_NS_GPL(snd_hda_hdmi_simple_build_controls, "SND_HDA_CODEC_HDMI");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_hdmi_simple_init(codec: *mut hda_codec) -> c_int {
    let spec = unsafe { (*codec).spec };
    let per_pin = unsafe { get_pin(spec, 0) };
    let pin: hda_nid_t = unsafe { (*per_pin).pin_nid };

    unsafe {
        snd_hda_codec_write(codec, pin, 0, AC_VERB_SET_PIN_WIDGET_CONTROL, PIN_OUT);
    }
    /* some codecs require to unmute the pin */
    if unsafe { get_wcaps(codec, pin) & AC_WCAP_OUT_AMP } != 0 {
        unsafe {
            snd_hda_codec_write(
                codec,
                pin,
                0,
                AC_VERB_SET_AMP_GAIN_MUTE,
                AMP_OUT_UNMUTE,
            );
        }
    }
    unsafe {
        snd_hda_jack_detect_enable(codec, pin, (*per_pin).dev_id);
    }
    0
}
// EXPORT_SYMBOL_NS_GPL(snd_hda_hdmi_simple_init, "SND_HDA_CODEC_HDMI");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_hdmi_simple_remove(codec: *mut hda_codec) {
    let spec = unsafe { (*codec).spec };

    unsafe {
        snd_array_free(&mut (*spec).pins);
        snd_array_free(&mut (*spec).cvts);
        kfree(spec as *mut c_void);
    }
}
// EXPORT_SYMBOL_NS_GPL(snd_hda_hdmi_simple_remove, "SND_HDA_CODEC_HDMI");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_hdmi_simple_pcm_open(
    _hinfo: *mut hda_pcm_stream,
    codec: *mut hda_codec,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let spec = unsafe { (*codec).spec };

    unsafe {
        if !(*spec).hw_constraints_channels.is_null() {
            snd_pcm_hw_constraint_list(
                (*substream).runtime,
                0,
                SNDRV_PCM_HW_PARAM_CHANNELS,
                (*spec).hw_constraints_channels,
            );
        } else {
            snd_pcm_hw_constraint_step((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, 2);
        }

        snd_hda_multi_out_dig_open(codec, &mut (*spec).multiout)
    }
}
// EXPORT_SYMBOL_NS_GPL(snd_hda_hdmi_simple_pcm_open, "SND_HDA_CODEC_HDMI");

unsafe extern "C" fn simple_playback_pcm_close(
    _hinfo: *mut hda_pcm_stream,
    codec: *mut hda_codec,
    _substream: *mut snd_pcm_substream,
) -> c_int {
    let spec = unsafe { (*codec).spec };

    unsafe { snd_hda_multi_out_dig_close(codec, &mut (*spec).multiout) }
}

unsafe extern "C" fn simple_playback_pcm_prepare(
    _hinfo: *mut hda_pcm_stream,
    codec: *mut hda_codec,
    stream_tag: c_uint,
    format: c_uint,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let spec = unsafe { (*codec).spec };

    unsafe {
        snd_hda_multi_out_dig_prepare(
            codec,
            &mut (*spec).multiout,
            stream_tag,
            format,
            substream,
        )
    }
}

static SIMPLE_PCM_PLAYBACK: hda_pcm_stream = hda_pcm_stream {
    substreams: 1,
    channels_min: 2,
    channels_max: 2,
    nid: 0,
    ops: hda_pcm_ops {
        open: Some(snd_hda_hdmi_simple_pcm_open),
        close: Some(simple_playback_pcm_close),
        prepare: Some(simple_playback_pcm_prepare),
    },
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_hdmi_simple_probe(
    codec: *mut hda_codec,
    cvt_nid: hda_nid_t,
    pin_nid: hda_nid_t,
) -> c_int {
    let spec: *mut hdmi_spec;
    let per_cvt: *mut hdmi_spec_per_cvt;
    let per_pin: *mut hdmi_spec_per_pin;

    spec = unsafe { kzalloc_obj_hdmi_spec() };
    if spec.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*spec).codec = codec;
        (*codec).spec = spec;
        snd_array_init(
            &mut (*spec).pins,
            core::mem::size_of::<hdmi_spec_per_pin>(),
            1,
        );
        snd_array_init(
            &mut (*spec).cvts,
            core::mem::size_of::<hdmi_spec_per_cvt>(),
            1,
        );

        (*spec).multiout.num_dacs = 0; /* no analog */
        (*spec).multiout.max_channels = 2;
        (*spec).multiout.dig_out_nid = cvt_nid;
        (*spec).num_cvts = 1;
        (*spec).num_pins = 1;
        per_pin = snd_array_new(&mut (*spec).pins) as *mut hdmi_spec_per_pin;
        per_cvt = snd_array_new(&mut (*spec).cvts) as *mut hdmi_spec_per_cvt;
        if per_pin.is_null() || per_cvt.is_null() {
            snd_hda_hdmi_simple_remove(codec);
            return -ENOMEM;
        }
        (*per_cvt).cvt_nid = cvt_nid;
        (*per_pin).pin_nid = pin_nid;
        (*spec).pcm_playback = SIMPLE_PCM_PLAYBACK;
    }

    0
}
// EXPORT_SYMBOL_NS_GPL(snd_hda_hdmi_simple_probe, "SND_HDA_CODEC_HDMI");

/*
 * driver entries
 */

unsafe extern "C" fn simplehdmi_probe(
    codec: *mut hda_codec,
    id: *const hda_device_id,
) -> c_int {
    match unsafe { (*id).driver_data } {
        MODEL_VIA => unsafe { snd_hda_hdmi_simple_probe(codec, VIAHDMI_CVT_NID, VIAHDMI_PIN_NID) },
        _ => -EINVAL,
    }
}

static SIMPLEHDMI_CODEC_OPS: hda_codec_ops = hda_codec_ops {
    probe: Some(simplehdmi_probe),
    remove: Some(snd_hda_hdmi_simple_remove),
    build_controls: Some(snd_hda_hdmi_simple_build_controls),
    build_pcms: Some(snd_hda_hdmi_simple_build_pcms),
    init: Some(snd_hda_hdmi_simple_init),
    unsol_event: Some(snd_hda_hdmi_simple_unsol_event),
};

// HDA_CODEC_ID_MODEL(0x11069f80, "VX900 HDMI/DP", MODEL_VIA)
// HDA_CODEC_ID_MODEL(0x11069f81, "VX900 HDMI/DP", MODEL_VIA)
static SND_HDA_ID_SIMPLEHDMI: [hda_device_id; 3] = [
    hda_device_id {
        driver_data: MODEL_VIA,
    },
    hda_device_id {
        driver_data: MODEL_VIA,
    },
    hda_device_id { driver_data: 0 },
]; /* terminator */

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Simple HDMI HD-audio codec support");

static mut SIMPLEHDMI_DRIVER: hda_codec_driver = hda_codec_driver {
    id: SND_HDA_ID_SIMPLEHDMI.as_ptr(),
    ops: &SIMPLEHDMI_CODEC_OPS,
};

// module_hda_codec_driver(simplehdmi_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
