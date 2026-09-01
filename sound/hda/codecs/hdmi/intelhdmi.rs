// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Intel HDMI codec support
 */

// C includes translated as external dependencies:
// <linux/init.h>, <linux/slab.h>, <linux/module.h>, <sound/core.h>,
// <sound/hdaudio.h>, <sound/hda_i915.h>, <sound/hda_codec.h>,
// "hda_local.h", "hdmi_local.h"

use core::ffi::{c_int, c_uint, c_void};

type hda_nid_t = c_uint;
type u32 = c_uint;

extern "C" {
    static mut enable_silent_stream: bool;

    fn snd_hda_codec_read(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        direct: c_int,
        verb: c_uint,
        parm: c_uint,
    ) -> c_uint;
    fn snd_hda_codec_write(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        direct: c_int,
        verb: c_uint,
        parm: c_uint,
    );
    fn snd_hda_codec_write_cache(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        direct: c_int,
        verb: c_uint,
        parm: c_uint,
    );
    fn snd_hda_codec_write_sync(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        direct: c_int,
        verb: c_uint,
        parm: c_uint,
    );
    fn snd_hda_codec_update_widgets(codec: *mut hda_codec);
    fn snd_hdac_regmap_add_vendor_verb(core: *mut hdac_device, verb: c_uint);
    fn snd_hda_codec_set_power_to_all(codec: *mut hda_codec, fg: hda_nid_t, power_state: c_uint);
    fn snd_hdac_i915_set_bclk(core: *mut hdac_bus);
    fn snd_hda_hdmi_check_presence_and_report(codec: *mut hda_codec, pin_nid: c_int, dev_id: c_int);
    fn snd_hda_hdmi_setup_drm_audio_ops(
        codec: *mut hda_codec,
        ops: *const drm_audio_component_audio_ops,
    );
    fn snd_hdac_acomp_register_notifier(core: *mut hdac_bus, ops: *mut drm_audio_component_audio_ops);
    fn snd_hdac_sync_audio_rate(
        core: *mut hdac_device,
        pin_nid: hda_nid_t,
        dev_id: c_int,
        rate: c_int,
    );
    fn snd_hdac_stream_format(channels: c_int, bits: c_int, rate: c_int) -> c_uint;
    fn snd_hda_codec_setup_stream(
        codec: *mut hda_codec,
        cvt_nid: hda_nid_t,
        stream_tag: u32,
        channel_id: c_int,
        format: c_uint,
    );
    fn usleep_range(min: c_uint, max: c_uint);
    fn snd_hda_hdmi_setup_audio_infoframe(
        codec: *mut hda_codec,
        per_pin: *mut hdmi_spec_per_pin,
        non_pcm: bool,
    );
    fn snd_hda_power_up_pm(codec: *mut hda_codec);
    fn snd_hda_power_down_pm(codec: *mut hda_codec);
    fn snd_hda_check_power_state(codec: *mut hda_codec, nid: hda_nid_t, power_state: c_uint) -> bool;
    fn msleep(msecs: c_uint);
    fn get_pin(spec: *mut hdmi_spec, pin_idx: c_int) -> *mut hdmi_spec_per_pin;
    fn get_cvt(spec: *mut hdmi_spec, cvt_idx: c_int) -> *mut hdmi_spec_per_cvt;
    fn snd_hda_get_num_devices(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
    fn snd_hda_get_dev_select(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
    fn snd_hda_set_dev_select(codec: *mut hda_codec, nid: hda_nid_t, dev_id: c_int);
    fn pin_id_to_pin_index(codec: *mut hda_codec, pin_nid: hda_nid_t, dev_id: c_int) -> c_int;
    fn snd_hda_hdmi_setup_stream(
        codec: *mut hda_codec,
        cvt_nid: hda_nid_t,
        pin_nid: hda_nid_t,
        dev_id: c_int,
        stream_tag: u32,
        format: c_int,
    ) -> c_int;
    fn snd_hda_hdmi_generic_suspend(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_generic_resume(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_generic_alloc(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_parse_codec(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_generic_init_per_pins(codec: *mut hda_codec);
    fn snd_hda_hdmi_generic_spec_free(codec: *mut hda_codec);
    fn snd_hda_hdmi_generic_remove(codec: *mut hda_codec);
    fn snd_hda_hdmi_generic_init(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_generic_build_pcms(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_generic_build_controls(codec: *mut hda_codec) -> c_int;
    fn snd_hda_hdmi_generic_unsol_event(codec: *mut hda_codec, res: c_uint);

    fn WARN_ON(condition: bool) -> bool;
    fn codec_info(codec: *mut hda_codec, fmt: *const u8, ...);
    fn codec_dbg(codec: *mut hda_codec, fmt: *const u8, ...);
}

#[repr(C)]
pub struct hda_codec {
    pub core: hdac_device,
    pub bus: *mut hda_bus,
    pub spec: *mut hdmi_spec,
    pub relaxed_resume: c_int,
    pub dp_mst: bool,
    pub display_power_control: c_int,
    pub depop_delay: c_int,
    pub auto_runtime_pm: c_int,
    pub no_stream_clean_at_suspend: c_int,
    pub forced_resume: c_int,
    pub probe_id: c_uint,
}

#[repr(C)]
pub struct hdac_device {
    pub vendor_id: c_uint,
    pub dev: device,
}

#[repr(C)]
pub struct device {
    pub power: dev_pm_info,
}

#[repr(C)]
pub struct dev_pm_info {
    pub power_state: pm_message_t,
}

#[repr(C)]
pub struct pm_message_t {
    pub event: c_int,
}

#[repr(C)]
pub struct hda_bus {
    pub core: hdac_bus,
}

#[repr(C)]
pub struct hdac_bus {
    pub audio_component: *mut c_void,
}

#[repr(C)]
pub struct hdmi_spec {
    pub vendor_nid: hda_nid_t,
    pub use_acomp_notifier: bool,
    pub port2pin: Option<unsafe extern "C" fn(*mut hda_codec, c_int) -> c_int>,
    pub drm_audio_ops: drm_audio_component_audio_ops,
    pub port_num: c_int,
    pub port_map: *const c_int,
    pub silent_stream_type: c_int,
    pub num_cvts: c_int,
    pub cvt_nids: *mut hda_nid_t,
    pub num_pins: c_int,
    pub intel_hsw_fixup: bool,
    pub dev_num: c_int,
    pub ops: hdmi_ops,
}

#[repr(C)]
pub struct hdmi_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut hda_codec, *mut hdmi_spec_per_pin)>,
    pub setup_stream: Option<
        unsafe extern "C" fn(*mut hda_codec, hda_nid_t, hda_nid_t, c_int, u32, c_int) -> c_int,
    >,
    pub pin_cvt_fixup:
        Option<unsafe extern "C" fn(*mut hda_codec, *mut hdmi_spec_per_pin, hda_nid_t)>,
    pub silent_stream: Option<unsafe extern "C" fn(*mut hda_codec, *mut hdmi_spec_per_pin, bool)>,
}

#[repr(C)]
pub struct hdmi_spec_per_pin {
    pub pin_nid: hda_nid_t,
    pub dev_id: c_int,
    pub cvt_nid: hda_nid_t,
    pub channels: c_int,
    pub non_pcm: bool,
    pub silent_stream: bool,
    pub mux_idx: c_int,
    pub pcm: *mut c_void,
}

#[repr(C)]
pub struct hdmi_spec_per_cvt {
    pub assigned: bool,
}

#[repr(C)]
pub struct drm_audio_component_audio_ops {
    pub pin2port: Option<unsafe extern "C" fn(*mut c_void, c_int) -> c_int>,
    pub pin_eld_notify: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
}

#[repr(C)]
pub struct hda_device_id {
    pub vendor_id: c_uint,
    pub name: *const u8,
    pub driver_data: usize,
}

#[repr(C)]
pub struct hda_codec_ops {
    pub probe: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut hda_codec)>,
    pub init: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub build_pcms: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub build_controls: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub unsol_event: Option<unsafe extern "C" fn(*mut hda_codec, c_uint)>,
    pub suspend: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>,
    pub set_power_state: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t, c_uint)>,
}

#[repr(C)]
pub struct hda_codec_driver {
    pub id: *const hda_device_id,
    pub ops: *const hda_codec_ops,
}

const MODEL_HSW: c_int = 0;
const MODEL_GLK: c_int = 1;
const MODEL_ICL: c_int = 2;
const MODEL_TGL: c_int = 3;
const MODEL_ADLP: c_int = 4;
const MODEL_BYT: c_int = 5;
const MODEL_CPT: c_int = 6;

const INTEL_GET_VENDOR_VERB: c_uint = 0xf81;
const INTEL_SET_VENDOR_VERB: c_uint = 0x781;
const INTEL_EN_DP12: c_uint = 0x02; /* enable DP 1.2 features */
const INTEL_EN_ALL_PIN_CVTS: c_uint = 0x01; /* enable 2nd & 3rd pins and convertors */

const I915_SILENT_RATE: c_int = 48000;
const I915_SILENT_CHANNELS: c_int = 2;
const I915_SILENT_FORMAT_BITS: c_int = 16;
const I915_SILENT_FMT_MASK: c_uint = 0xf;

extern "C" {
    static AC_PWRST_D0: c_uint;
    static AC_VERB_SET_POWER_STATE: c_uint;
    static AC_VERB_GET_POWER_STATE: c_uint;
    static AC_PWRST_ACTUAL: c_int;
    static AC_PWRST_ACTUAL_SHIFT: c_int;
    static AC_VERB_GET_DIGI_CONVERT_1: c_uint;
    static AC_VERB_SET_DIGI_CONVERT_3: c_uint;
    static AC_VERB_GET_CONV: c_uint;
    static AC_VERB_GET_CONNECT_SEL: c_uint;
    static AC_VERB_SET_CONNECT_SEL: c_uint;
    static AC_DIG3_KAE: c_uint;
    static SILENT_STREAM_KAE: c_int;
    static SILENT_STREAM_I915: c_int;
    static PM_EVENT_SUSPEND: c_int;
    static HDA_CODEC_ID_SKIP_PROBE: c_uint;
    static ENODEV: c_int;
    static EINVAL: c_int;
}

unsafe extern "C" fn intel_haswell_enable_all_pins(codec: *mut hda_codec, update_tree: bool) {
    let mut vendor_param: c_uint;
    let spec: *mut hdmi_spec = (*codec).spec;

    vendor_param = snd_hda_codec_read(codec, (*spec).vendor_nid, 0, INTEL_GET_VENDOR_VERB, 0);
    if vendor_param == (-1i32 as c_uint) || (vendor_param & INTEL_EN_ALL_PIN_CVTS) != 0 {
        return;
    }

    vendor_param |= INTEL_EN_ALL_PIN_CVTS;
    vendor_param = snd_hda_codec_read(
        codec,
        (*spec).vendor_nid,
        0,
        INTEL_SET_VENDOR_VERB,
        vendor_param,
    );
    if vendor_param == (-1i32 as c_uint) {
        return;
    }

    if update_tree {
        snd_hda_codec_update_widgets(codec);
    }
}

unsafe extern "C" fn intel_haswell_fixup_enable_dp12(codec: *mut hda_codec) {
    let mut vendor_param: c_uint;
    let spec: *mut hdmi_spec = (*codec).spec;

    vendor_param = snd_hda_codec_read(codec, (*spec).vendor_nid, 0, INTEL_GET_VENDOR_VERB, 0);
    if vendor_param == (-1i32 as c_uint) || (vendor_param & INTEL_EN_DP12) != 0 {
        return;
    }

    /* enable DP1.2 mode */
    vendor_param |= INTEL_EN_DP12;
    snd_hdac_regmap_add_vendor_verb(&mut (*codec).core, INTEL_SET_VENDOR_VERB);
    snd_hda_codec_write_cache(
        codec,
        (*spec).vendor_nid,
        0,
        INTEL_SET_VENDOR_VERB,
        vendor_param,
    );
}

/* Haswell needs to re-issue the vendor-specific verbs before turning to D0.
 * Otherwise you may get severe h/w communication errors.
 */
unsafe extern "C" fn haswell_set_power_state(
    codec: *mut hda_codec,
    fg: hda_nid_t,
    power_state: c_uint,
) {
    /* check codec->spec: it can be called before the probe gets called */
    if !(*codec).spec.is_null() {
        if power_state == AC_PWRST_D0 {
            intel_haswell_enable_all_pins(codec, false);
            intel_haswell_fixup_enable_dp12(codec);
        }
    }

    snd_hda_codec_write_sync(codec, fg, 0, AC_VERB_SET_POWER_STATE, power_state);
    snd_hda_codec_set_power_to_all(codec, fg, power_state);
}

/* There is a fixed mapping between audio pin node and display port.
 * on SNB, IVY, HSW, BSW, SKL, BXT, KBL:
 * Pin Widget 5 - PORT B (port = 1 in i915 driver)
 * Pin Widget 6 - PORT C (port = 2 in i915 driver)
 * Pin Widget 7 - PORT D (port = 3 in i915 driver)
 *
 * on VLV, ILK:
 * Pin Widget 4 - PORT B (port = 1 in i915 driver)
 * Pin Widget 5 - PORT C (port = 2 in i915 driver)
 * Pin Widget 6 - PORT D (port = 3 in i915 driver)
 */
unsafe extern "C" fn intel_base_nid(codec: *mut hda_codec) -> c_int {
    match (*codec).core.vendor_id {
        0x80860054 | /* ILK */ 0x80862804 | /* ILK */ 0x80862882 => 4, /* VLV */
        _ => 5,
    }
}

unsafe extern "C" fn intel_pin2port(audio_ptr: *mut c_void, pin_nid: c_int) -> c_int {
    let codec: *mut hda_codec = audio_ptr as *mut hda_codec;
    let spec: *mut hdmi_spec = (*codec).spec;
    let base_nid: c_int;
    let mut i: c_int;

    if (*spec).port_num == 0 {
        base_nid = intel_base_nid(codec);
        if WARN_ON(pin_nid < base_nid || pin_nid >= base_nid + 3) {
            return -1;
        }
        return pin_nid - base_nid + 1;
    }

    /*
     * looking for the pin number in the mapping table and return
     * the index which indicate the port number
     */
    i = 0;
    while i < (*spec).port_num {
        if pin_nid == *(*spec).port_map.offset(i as isize) {
            return i;
        }
        i += 1;
    }

    codec_info(
        codec,
        c"Can't find the HDMI/DP port for pin NID 0x%x\n".as_ptr() as *const u8,
        pin_nid,
    );
    -1
}

unsafe extern "C" fn intel_port2pin(codec: *mut hda_codec, port: c_int) -> c_int {
    let spec: *mut hdmi_spec = (*codec).spec;

    if (*spec).port_num == 0 {
        /* we assume only from port-B to port-D */
        if port < 1 || port > 3 {
            return 0;
        }
        return port + intel_base_nid(codec) - 1;
    }

    if port < 0 || port >= (*spec).port_num {
        return 0;
    }
    *(*spec).port_map.offset(port as isize)
}

unsafe extern "C" fn intel_pin_eld_notify(audio_ptr: *mut c_void, port: c_int, pipe: c_int) {
    let codec: *mut hda_codec = audio_ptr as *mut hda_codec;
    let pin_nid: c_int;
    let dev_id: c_int = pipe;

    pin_nid = intel_port2pin(codec, port);
    if pin_nid == 0 {
        return;
    }
    /* skip notification during system suspend (but not in runtime PM);
     * the state will be updated at resume
     */
    if (*codec).core.dev.power.power_state.event == PM_EVENT_SUSPEND {
        return;
    }

    snd_hdac_i915_set_bclk(&mut (*(*codec).bus).core);
    snd_hda_hdmi_check_presence_and_report(codec, pin_nid, dev_id);
}

static intel_audio_ops: drm_audio_component_audio_ops = drm_audio_component_audio_ops {
    pin2port: Some(intel_pin2port),
    pin_eld_notify: Some(intel_pin_eld_notify),
};

/* register i915 component pin_eld_notify callback */
unsafe extern "C" fn register_i915_notifier(codec: *mut hda_codec) {
    let spec: *mut hdmi_spec = (*codec).spec;

    (*spec).use_acomp_notifier = true;
    (*spec).port2pin = Some(intel_port2pin);
    snd_hda_hdmi_setup_drm_audio_ops(codec, &intel_audio_ops);
    snd_hdac_acomp_register_notifier(&mut (*(*codec).bus).core, &mut (*spec).drm_audio_ops);
    /* no need for forcible resume for jack check thanks to notifier */
    (*codec).relaxed_resume = 1;
}

unsafe extern "C" fn silent_stream_enable_i915(
    codec: *mut hda_codec,
    per_pin: *mut hdmi_spec_per_pin,
) {
    let format: c_uint;

    snd_hdac_sync_audio_rate(
        &mut (*codec).core,
        (*per_pin).pin_nid,
        (*per_pin).dev_id,
        I915_SILENT_RATE,
    );

    /* trigger silent stream generation in hw */
    format = snd_hdac_stream_format(
        I915_SILENT_CHANNELS,
        I915_SILENT_FORMAT_BITS,
        I915_SILENT_RATE,
    );
    snd_hda_codec_setup_stream(
        codec,
        (*per_pin).cvt_nid,
        I915_SILENT_FMT_MASK,
        I915_SILENT_FMT_MASK as c_int,
        format,
    );
    usleep_range(100, 200);
    snd_hda_codec_setup_stream(
        codec,
        (*per_pin).cvt_nid,
        I915_SILENT_FMT_MASK,
        0,
        format,
    );

    (*per_pin).channels = I915_SILENT_CHANNELS;
    snd_hda_hdmi_setup_audio_infoframe(codec, per_pin, (*per_pin).non_pcm);
}

unsafe extern "C" fn silent_stream_set_kae(
    codec: *mut hda_codec,
    per_pin: *mut hdmi_spec_per_pin,
    enable: bool,
) {
    let mut param: c_uint;

    codec_dbg(
        codec,
        c"HDMI: KAE %d cvt-NID=0x%x\n".as_ptr() as *const u8,
        enable as c_int,
        (*per_pin).cvt_nid,
    );

    param = snd_hda_codec_read(codec, (*per_pin).cvt_nid, 0, AC_VERB_GET_DIGI_CONVERT_1, 0);
    param = (param >> 16) & 0xff;

    if enable {
        param |= AC_DIG3_KAE;
    } else {
        param &= !AC_DIG3_KAE;
    }

    snd_hda_codec_write(codec, (*per_pin).cvt_nid, 0, AC_VERB_SET_DIGI_CONVERT_3, param);
}

unsafe extern "C" fn i915_set_silent_stream(
    codec: *mut hda_codec,
    per_pin: *mut hdmi_spec_per_pin,
    enable: bool,
) {
    let spec: *mut hdmi_spec = (*codec).spec;

    match (*spec).silent_stream_type {
        x if x == SILENT_STREAM_KAE => {
            if enable {
                silent_stream_enable_i915(codec, per_pin);
                silent_stream_set_kae(codec, per_pin, true);
            } else {
                silent_stream_set_kae(codec, per_pin, false);
            }
        }
        x if x == SILENT_STREAM_I915 => {
            if enable {
                silent_stream_enable_i915(codec, per_pin);
                snd_hda_power_up_pm(codec);
            } else {
                /* release ref taken in silent_stream_enable() */
                snd_hda_power_down_pm(codec);
            }
        }
        _ => {}
    }
}

unsafe extern "C" fn haswell_verify_D0(
    codec: *mut hda_codec,
    cvt_nid: hda_nid_t,
    nid: hda_nid_t,
) {
    let mut pwr: c_int;

    /* For Haswell, the converter 1/2 may keep in D3 state after bootup,
     * thus pins could only choose converter 0 for use. Make sure the
     * converters are in correct power state
     */
    if !snd_hda_check_power_state(codec, cvt_nid, AC_PWRST_D0) {
        snd_hda_codec_write(codec, cvt_nid, 0, AC_VERB_SET_POWER_STATE, AC_PWRST_D0);
    }

    if !snd_hda_check_power_state(codec, nid, AC_PWRST_D0) {
        snd_hda_codec_write(codec, nid, 0, AC_VERB_SET_POWER_STATE, AC_PWRST_D0);
        msleep(40);
        pwr = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_POWER_STATE, 0) as c_int;
        pwr = (pwr & AC_PWRST_ACTUAL) >> AC_PWRST_ACTUAL_SHIFT;
        codec_dbg(
            codec,
            c"Haswell HDMI audio: Power for NID 0x%x is now D%d\n".as_ptr() as *const u8,
            nid,
            pwr,
        );
    }
}

/* Assure the pin select the right convetor */
unsafe extern "C" fn intel_verify_pin_cvt_connect(
    codec: *mut hda_codec,
    per_pin: *mut hdmi_spec_per_pin,
) {
    let pin_nid: hda_nid_t = (*per_pin).pin_nid;
    let mux_idx: c_int;
    let curr: c_int;

    mux_idx = (*per_pin).mux_idx;
    curr = snd_hda_codec_read(codec, pin_nid, 0, AC_VERB_GET_CONNECT_SEL, 0) as c_int;
    if curr != mux_idx {
        snd_hda_codec_write_cache(codec, pin_nid, 0, AC_VERB_SET_CONNECT_SEL, mux_idx as c_uint);
    }
}

/* get the mux index for the converter of the pins
 * converter's mux index is the same for all pins on Intel platform
 */
unsafe extern "C" fn intel_cvt_id_to_mux_idx(spec: *mut hdmi_spec, cvt_nid: hda_nid_t) -> c_int {
    let mut i: c_int;

    i = 0;
    while i < (*spec).num_cvts {
        if *(*spec).cvt_nids.offset(i as isize) == cvt_nid {
            return i;
        }
        i += 1;
    }
    -EINVAL
}

/* Intel HDMI workaround to fix audio routing issue:
 * For some Intel display codecs, pins share the same connection list.
 * So a conveter can be selected by multiple pins and playback on any of these
 * pins will generate sound on the external display, because audio flows from
 * the same converter to the display pipeline. Also muting one pin may make
 * other pins have no sound output.
 * So this function assures that an assigned converter for a pin is not selected
 * by any other pins.
 */
unsafe extern "C" fn intel_not_share_assigned_cvt(
    codec: *mut hda_codec,
    pin_nid: hda_nid_t,
    dev_id: c_int,
    mux_idx: c_int,
) {
    let spec: *mut hdmi_spec = (*codec).spec;
    let mut nid: hda_nid_t;
    let mut cvt_idx: c_int;
    let mut curr: c_int;
    let mut per_cvt: *mut hdmi_spec_per_cvt;
    let mut per_pin: *mut hdmi_spec_per_pin;
    let mut pin_idx: c_int;

    /* configure the pins connections */
    pin_idx = 0;
    while pin_idx < (*spec).num_pins {
        let dev_id_saved: c_int;
        let dev_num: c_int;

        per_pin = get_pin(spec, pin_idx);
        /*
         * pin not connected to monitor
         * no need to operate on it
         */
        if (*per_pin).pcm.is_null() {
            pin_idx += 1;
            continue;
        }

        if (*per_pin).pin_nid == pin_nid && (*per_pin).dev_id == dev_id {
            pin_idx += 1;
            continue;
        }

        /*
         * if per_pin->dev_id >= dev_num,
         * snd_hda_get_dev_select() will fail,
         * and the following operation is unpredictable.
         * So skip this situation.
         */
        dev_num = snd_hda_get_num_devices(codec, (*per_pin).pin_nid) + 1;
        if (*per_pin).dev_id >= dev_num {
            pin_idx += 1;
            continue;
        }

        nid = (*per_pin).pin_nid;

        /*
         * Calling this function should not impact
         * on the device entry selection
         * So let's save the dev id for each pin,
         * and restore it when return
         */
        dev_id_saved = snd_hda_get_dev_select(codec, nid);
        snd_hda_set_dev_select(codec, nid, (*per_pin).dev_id);
        curr = snd_hda_codec_read(codec, nid, 0, AC_VERB_GET_CONNECT_SEL, 0) as c_int;
        if curr != mux_idx {
            snd_hda_set_dev_select(codec, nid, dev_id_saved);
            pin_idx += 1;
            continue;
        }

        /* choose an unassigned converter. The conveters in the
         * connection list are in the same order as in the codec.
         */
        cvt_idx = 0;
        while cvt_idx < (*spec).num_cvts {
            per_cvt = get_cvt(spec, cvt_idx);
            if !(*per_cvt).assigned {
                codec_dbg(
                    codec,
                    c"choose cvt %d for pin NID 0x%x\n".as_ptr() as *const u8,
                    cvt_idx,
                    nid,
                );
                snd_hda_codec_write_cache(
                    codec,
                    nid,
                    0,
                    AC_VERB_SET_CONNECT_SEL,
                    cvt_idx as c_uint,
                );
                break;
            }
            cvt_idx += 1;
        }
        snd_hda_set_dev_select(codec, nid, dev_id_saved);
        pin_idx += 1;
    }
}

/* A wrapper of intel_not_share_asigned_cvt() */
unsafe extern "C" fn intel_not_share_assigned_cvt_nid(
    codec: *mut hda_codec,
    pin_nid: hda_nid_t,
    dev_id: c_int,
    cvt_nid: hda_nid_t,
) {
    let mux_idx: c_int;
    let spec: *mut hdmi_spec = (*codec).spec;

    /* On Intel platform, the mapping of converter nid to
     * mux index of the pins are always the same.
     * The pin nid may be 0, this means all pins will not
     * share the converter.
     */
    mux_idx = intel_cvt_id_to_mux_idx(spec, cvt_nid);
    if mux_idx >= 0 {
        intel_not_share_assigned_cvt(codec, pin_nid, dev_id, mux_idx);
    }
}

/*
 * prepare ops override for HSW+
 *
 * Disable keep-alive before the converter format and audio infoframe are
 * reprogrammed by the PCM prepare sequence. Changing the audio format (e.g.
 * the channel count when switching to multichannel PCM) while a keep-alive
 * stream is active is not safe, so release keep-alive here, early in the
 * sequence. It is re-enabled once the new stream has been set up, in
 * i915_hsw_setup_stream().
 */
unsafe extern "C" fn i915_hsw_prepare(codec: *mut hda_codec, per_pin: *mut hdmi_spec_per_pin) {
    let spec: *mut hdmi_spec = (*codec).spec;

    if (*spec).silent_stream_type == SILENT_STREAM_KAE && (*per_pin).silent_stream {
        silent_stream_set_kae(codec, per_pin, false);
        /* wait for pending transfers in codec to clear */
        usleep_range(100, 200);
    }
}

/* setup_stream ops override for HSW+ */
unsafe extern "C" fn i915_hsw_setup_stream(
    codec: *mut hda_codec,
    cvt_nid: hda_nid_t,
    pin_nid: hda_nid_t,
    dev_id: c_int,
    stream_tag: u32,
    format: c_int,
) -> c_int {
    let spec: *mut hdmi_spec = (*codec).spec;
    let pin_idx: c_int = pin_id_to_pin_index(codec, pin_nid, dev_id);
    let per_pin: *mut hdmi_spec_per_pin;
    let res: c_int;

    if pin_idx < 0 {
        per_pin = core::ptr::null_mut();
    } else {
        per_pin = get_pin(spec, pin_idx);
    }

    haswell_verify_D0(codec, cvt_nid, pin_nid);

    res = snd_hda_hdmi_setup_stream(codec, cvt_nid, pin_nid, dev_id, stream_tag, format);

    /*
     * Keep-alive was disabled in i915_hsw_prepare(), re-enable it now.
     * The pin lookup above resolves to the same per_pin that prepare
     * used (pin_nid comes from that per_pin), so this stays balanced; a
     * NULL per_pin only occurs on a lookup failure that also implies no
     * active keep-alive stream to restore.
     */
    if (*spec).silent_stream_type == SILENT_STREAM_KAE
        && !per_pin.is_null()
        && (*per_pin).silent_stream
    {
        usleep_range(100, 200);
        silent_stream_set_kae(codec, per_pin, true);
    }

    res
}

/* pin_cvt_fixup ops override for HSW+ and VLV+ */
unsafe extern "C" fn i915_pin_cvt_fixup(
    codec: *mut hda_codec,
    per_pin: *mut hdmi_spec_per_pin,
    cvt_nid: hda_nid_t,
) {
    if !per_pin.is_null() {
        haswell_verify_D0(codec, (*per_pin).cvt_nid, (*per_pin).pin_nid);
        snd_hda_set_dev_select(codec, (*per_pin).pin_nid, (*per_pin).dev_id);
        intel_verify_pin_cvt_connect(codec, per_pin);
        intel_not_share_assigned_cvt(codec, (*per_pin).pin_nid, (*per_pin).dev_id, (*per_pin).mux_idx);
    } else {
        intel_not_share_assigned_cvt_nid(codec, 0, 0, cvt_nid);
    }
}

unsafe extern "C" fn i915_hdmi_suspend(codec: *mut hda_codec) -> c_int {
    let spec: *mut hdmi_spec = (*codec).spec;
    let mut silent_streams: bool = false;
    let mut pin_idx: c_int;
    let res: c_int;

    res = snd_hda_hdmi_generic_suspend(codec);
    if (*spec).silent_stream_type != SILENT_STREAM_KAE {
        return res;
    }

    pin_idx = 0;
    while pin_idx < (*spec).num_pins {
        let per_pin: *mut hdmi_spec_per_pin = get_pin(spec, pin_idx);

        if (*per_pin).silent_stream {
            silent_streams = true;
            break;
        }
        pin_idx += 1;
    }

    if silent_streams {
        /*
         * stream-id should remain programmed when codec goes
         * to runtime suspend
         */
        (*codec).no_stream_clean_at_suspend = 1;

        /*
         * the system might go to S3, in which case keep-alive
         * must be reprogrammed upon resume
         */
        (*codec).forced_resume = 1;

        codec_dbg(codec, c"HDMI: KAE active at suspend\n".as_ptr() as *const u8);
    } else {
        (*codec).no_stream_clean_at_suspend = 0;
        (*codec).forced_resume = 0;
    }

    res
}

unsafe extern "C" fn i915_hdmi_resume(codec: *mut hda_codec) -> c_int {
    let spec: *mut hdmi_spec = (*codec).spec;
    let mut pin_idx: c_int;
    let res: c_int;

    res = snd_hda_hdmi_generic_resume(codec);
    if (*spec).silent_stream_type != SILENT_STREAM_KAE {
        return res;
    }

    /* KAE not programmed at suspend, nothing to do here */
    if (*codec).no_stream_clean_at_suspend == 0 {
        return res;
    }

    pin_idx = 0;
    while pin_idx < (*spec).num_pins {
        let per_pin: *mut hdmi_spec_per_pin = get_pin(spec, pin_idx);

        /*
         * If system was in suspend with monitor connected,
         * the codec setting may have been lost. Re-enable
         * keep-alive.
         */
        if (*per_pin).silent_stream {
            let mut param: c_uint;

            param = snd_hda_codec_read(codec, (*per_pin).cvt_nid, 0, AC_VERB_GET_CONV, 0);
            if param == 0 {
                codec_dbg(codec, c"HDMI: KAE: restore stream id\n".as_ptr() as *const u8);
                silent_stream_enable_i915(codec, per_pin);
            }

            param = snd_hda_codec_read(codec, (*per_pin).cvt_nid, 0, AC_VERB_GET_DIGI_CONVERT_1, 0);
            if (param & (AC_DIG3_KAE << 16)) == 0 {
                codec_dbg(codec, c"HDMI: KAE: restore DIG3_KAE\n".as_ptr() as *const u8);
                silent_stream_set_kae(codec, per_pin, true);
            }
        }
        pin_idx += 1;
    }

    res
}

/* precondition and allocation for Intel codecs */
unsafe extern "C" fn alloc_intel_hdmi(codec: *mut hda_codec) -> c_int {
    /* requires i915 binding */
    if (*(*codec).bus).core.audio_component.is_null() {
        codec_info(
            codec,
            c"No i915 binding for Intel HDMI/DP codec\n".as_ptr() as *const u8,
        );
        /* set probe_id here to prevent generic fallback binding */
        (*codec).probe_id = HDA_CODEC_ID_SKIP_PROBE;
        return -ENODEV;
    }

    snd_hda_hdmi_generic_alloc(codec)
}

/* parse and post-process for Intel codecs */
unsafe extern "C" fn parse_intel_hdmi(codec: *mut hda_codec) -> c_int {
    let mut err: c_int;
    let mut retries: c_int = 3;

    loop {
        err = snd_hda_hdmi_parse_codec(codec);
        if !(err < 0 && {
            let old = retries;
            retries -= 1;
            old != 0
        }) {
            break;
        }
    }

    if err < 0 {
        return err;
    }

    snd_hda_hdmi_generic_init_per_pins(codec);
    register_i915_notifier(codec);
    0
}

/* Intel Haswell and onwards; audio component with eld notifier */
unsafe extern "C" fn intel_hsw_common_init(
    codec: *mut hda_codec,
    vendor_nid: hda_nid_t,
    port_map: *const c_int,
    port_num: c_int,
    dev_num: c_int,
    send_silent_stream: bool,
) -> c_int {
    let spec: *mut hdmi_spec;

    spec = (*codec).spec;
    (*codec).dp_mst = true;
    (*spec).vendor_nid = vendor_nid;
    (*spec).port_map = port_map;
    (*spec).port_num = port_num;
    (*spec).intel_hsw_fixup = true;
    (*spec).dev_num = dev_num;

    intel_haswell_enable_all_pins(codec, true);
    intel_haswell_fixup_enable_dp12(codec);

    (*codec).display_power_control = 1;

    (*codec).depop_delay = 0;
    (*codec).auto_runtime_pm = 1;

    (*spec).ops.prepare = Some(i915_hsw_prepare);
    (*spec).ops.setup_stream = Some(i915_hsw_setup_stream);
    (*spec).ops.pin_cvt_fixup = Some(i915_pin_cvt_fixup);
    (*spec).ops.silent_stream = Some(i915_set_silent_stream);

    /*
     * Enable silent stream feature, if it is enabled via
     * module param or Kconfig option
     */
    if send_silent_stream {
        (*spec).silent_stream_type = SILENT_STREAM_I915;
    }

    parse_intel_hdmi(codec)
}

unsafe extern "C" fn probe_i915_hsw_hdmi(codec: *mut hda_codec) -> c_int {
    intel_hsw_common_init(codec, 0x08, core::ptr::null(), 0, 3, enable_silent_stream)
}

unsafe extern "C" fn probe_i915_glk_hdmi(codec: *mut hda_codec) -> c_int {
    /*
     * Silent stream calls audio component .get_power() from
     * .pin_eld_notify(). On GLK this will deadlock in i915 due
     * to the audio vs. CDCLK workaround.
     */
    intel_hsw_common_init(codec, 0x0b, core::ptr::null(), 0, 3, false)
}

unsafe extern "C" fn probe_i915_icl_hdmi(codec: *mut hda_codec) -> c_int {
    /*
     * pin to port mapping table where the value indicate the pin number and
     * the index indicate the port number.
     */
    static MAP: [c_int; 6] = [0x0, 0x4, 0x6, 0x8, 0xa, 0xb];

    intel_hsw_common_init(
        codec,
        0x02,
        MAP.as_ptr(),
        MAP.len() as c_int,
        3,
        enable_silent_stream,
    )
}

unsafe extern "C" fn probe_i915_tgl_hdmi(codec: *mut hda_codec) -> c_int {
    /*
     * pin to port mapping table where the value indicate the pin number and
     * the index indicate the port number.
     */
    static MAP: [c_int; 9] = [0x4, 0x6, 0x8, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf];

    intel_hsw_common_init(
        codec,
        0x02,
        MAP.as_ptr(),
        MAP.len() as c_int,
        4,
        enable_silent_stream,
    )
}

unsafe extern "C" fn probe_i915_adlp_hdmi(codec: *mut hda_codec) -> c_int {
    let spec: *mut hdmi_spec;
    let res: c_int;

    res = probe_i915_tgl_hdmi(codec);
    if res == 0 {
        spec = (*codec).spec;

        if (*spec).silent_stream_type != 0 {
            (*spec).silent_stream_type = SILENT_STREAM_KAE;
        }
    }

    res
}

/* Intel Baytrail and Braswell; with eld notifier */
unsafe extern "C" fn probe_i915_byt_hdmi(codec: *mut hda_codec) -> c_int {
    let spec: *mut hdmi_spec;

    spec = (*codec).spec;

    /* For Valleyview/Cherryview, only the display codec is in the display
     * power well and can use link_power ops to request/release the power.
     */
    (*codec).display_power_control = 1;

    (*codec).depop_delay = 0;
    (*codec).auto_runtime_pm = 1;

    (*spec).ops.pin_cvt_fixup = Some(i915_pin_cvt_fixup);

    parse_intel_hdmi(codec)
}

/* Intel IronLake, SandyBridge and IvyBridge; with eld notifier */
unsafe extern "C" fn probe_i915_cpt_hdmi(codec: *mut hda_codec) -> c_int {
    parse_intel_hdmi(codec)
}

/*
 * common driver probe
 */
unsafe extern "C" fn intelhdmi_probe(
    codec: *mut hda_codec,
    id: *const hda_device_id,
) -> c_int {
    let mut err: c_int;

    err = alloc_intel_hdmi(codec);
    if err < 0 {
        return err;
    }

    match (*id).driver_data as c_int {
        MODEL_HSW => {
            err = probe_i915_hsw_hdmi(codec);
        }
        MODEL_GLK => {
            err = probe_i915_glk_hdmi(codec);
        }
        MODEL_ICL => {
            err = probe_i915_icl_hdmi(codec);
        }
        MODEL_TGL => {
            err = probe_i915_tgl_hdmi(codec);
        }
        MODEL_ADLP => {
            err = probe_i915_adlp_hdmi(codec);
        }
        MODEL_BYT => {
            err = probe_i915_byt_hdmi(codec);
        }
        MODEL_CPT => {
            err = probe_i915_cpt_hdmi(codec);
        }
        _ => {
            err = -EINVAL;
        }
    }

    if err < 0 {
        snd_hda_hdmi_generic_spec_free(codec);
        return err;
    }

    0
}

static intelhdmi_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(intelhdmi_probe),
    remove: Some(snd_hda_hdmi_generic_remove),
    init: Some(snd_hda_hdmi_generic_init),
    build_pcms: Some(snd_hda_hdmi_generic_build_pcms),
    build_controls: Some(snd_hda_hdmi_generic_build_controls),
    unsol_event: Some(snd_hda_hdmi_generic_unsol_event),
    suspend: Some(i915_hdmi_suspend),
    resume: Some(i915_hdmi_resume),
    set_power_state: Some(haswell_set_power_state),
};

macro_rules! HDA_CODEC_ID_MODEL {
    ($id:expr, $name:expr, $model:expr) => {
        hda_device_id {
            vendor_id: $id,
            name: concat!($name, "\0").as_ptr(),
            driver_data: $model as usize,
        }
    };
}

/*
 * driver entries
 */
static snd_hda_id_intelhdmi: [hda_device_id; 33] = [
    HDA_CODEC_ID_MODEL!(0x80860054, "IbexPeak HDMI", MODEL_CPT),
    HDA_CODEC_ID_MODEL!(0x80862800, "Geminilake HDMI", MODEL_GLK),
    HDA_CODEC_ID_MODEL!(0x80862804, "IbexPeak HDMI", MODEL_CPT),
    HDA_CODEC_ID_MODEL!(0x80862805, "CougarPoint HDMI", MODEL_CPT),
    HDA_CODEC_ID_MODEL!(0x80862806, "PantherPoint HDMI", MODEL_CPT),
    HDA_CODEC_ID_MODEL!(0x80862807, "Haswell HDMI", MODEL_HSW),
    HDA_CODEC_ID_MODEL!(0x80862808, "Broadwell HDMI", MODEL_HSW),
    HDA_CODEC_ID_MODEL!(0x80862809, "Skylake HDMI", MODEL_HSW),
    HDA_CODEC_ID_MODEL!(0x8086280a, "Broxton HDMI", MODEL_HSW),
    HDA_CODEC_ID_MODEL!(0x8086280b, "Kabylake HDMI", MODEL_HSW),
    HDA_CODEC_ID_MODEL!(0x8086280c, "Cannonlake HDMI", MODEL_GLK),
    HDA_CODEC_ID_MODEL!(0x8086280d, "Geminilake HDMI", MODEL_GLK),
    HDA_CODEC_ID_MODEL!(0x8086280f, "Icelake HDMI", MODEL_ICL),
    HDA_CODEC_ID_MODEL!(0x80862812, "Tigerlake HDMI", MODEL_TGL),
    HDA_CODEC_ID_MODEL!(0x80862814, "DG1 HDMI", MODEL_TGL),
    HDA_CODEC_ID_MODEL!(0x80862815, "Alderlake HDMI", MODEL_TGL),
    HDA_CODEC_ID_MODEL!(0x80862816, "Rocketlake HDMI", MODEL_TGL),
    HDA_CODEC_ID_MODEL!(0x80862818, "Raptorlake HDMI", MODEL_TGL),
    HDA_CODEC_ID_MODEL!(0x80862819, "DG2 HDMI", MODEL_TGL),
    HDA_CODEC_ID_MODEL!(0x8086281a, "Jasperlake HDMI", MODEL_ICL),
    HDA_CODEC_ID_MODEL!(0x8086281b, "Elkhartlake HDMI", MODEL_ICL),
    HDA_CODEC_ID_MODEL!(0x8086281c, "Alderlake-P HDMI", MODEL_ADLP),
    HDA_CODEC_ID_MODEL!(0x8086281d, "Meteor Lake HDMI", MODEL_ADLP),
    HDA_CODEC_ID_MODEL!(0x8086281e, "Battlemage HDMI", MODEL_ADLP),
    HDA_CODEC_ID_MODEL!(0x8086281f, "Raptor Lake P HDMI", MODEL_ADLP),
    HDA_CODEC_ID_MODEL!(0x80862820, "Lunar Lake HDMI", MODEL_ADLP),
    HDA_CODEC_ID_MODEL!(0x80862822, "Panther Lake HDMI", MODEL_ADLP),
    HDA_CODEC_ID_MODEL!(0x80862823, "Wildcat Lake HDMI", MODEL_ADLP),
    HDA_CODEC_ID_MODEL!(0x80862824, "Nova Lake HDMI", MODEL_ADLP),
    HDA_CODEC_ID_MODEL!(0x80862882, "Valleyview2 HDMI", MODEL_BYT),
    HDA_CODEC_ID_MODEL!(0x80862883, "Braswell HDMI", MODEL_BYT),
    hda_device_id {
        vendor_id: 0,
        name: core::ptr::null(),
        driver_data: 0,
    }, /* terminator */
];
// MODULE_DEVICE_TABLE(hdaudio, snd_hda_id_intelhdmi);

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Intel HDMI HD-audio codec");
// MODULE_IMPORT_NS("SND_HDA_CODEC_HDMI");

static mut intelhdmi_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_intelhdmi.as_ptr(),
    ops: &intelhdmi_codec_ops,
};

// module_hda_codec_driver(intelhdmi_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
