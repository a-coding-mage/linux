// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * HD-audio HDMI codec driver
 */

// C header dependencies:
// <sound/core.h>, <sound/jack.h>, <sound/hdaudio.h>, <sound/hda_i915.h>,
// <sound/hda_chmap.h>, <sound/hda_codec.h>, "hda_local.h"

#[repr(C)]
pub struct hdmi_spec_per_cvt {
    pub cvt_nid: hda_nid_t,
    pub assigned: bool,       /* the stream has been assigned */
    pub silent_stream: bool,  /* silent stream activated */
    pub channels_min: ::core::ffi::c_uint,
    pub channels_max: ::core::ffi::c_uint,
    pub rates: u32,
    pub formats: u64,
    pub maxbps: ::core::ffi::c_uint,
}

/* max. connections to a widget */
pub const HDA_MAX_CONNECTIONS: usize = 32;

#[repr(C)]
pub struct hdmi_spec_per_pin {
    pub pin_nid: hda_nid_t,
    pub dev_id: ::core::ffi::c_int,
    /* pin idx, different device entries on the same pin use the same idx */
    pub pin_nid_idx: ::core::ffi::c_int,
    pub num_mux_nids: ::core::ffi::c_int,
    pub mux_nids: [hda_nid_t; HDA_MAX_CONNECTIONS],
    pub mux_idx: ::core::ffi::c_int,
    pub cvt_nid: hda_nid_t,

    pub codec: *mut hda_codec,
    pub sink_eld: hdmi_eld,
    pub lock: mutex,
    pub work: delayed_work,
    pub pcm: *mut hdmi_pcm, /* pointer to spec->pcm_rec[n] dynamically*/
    pub pcm_idx: ::core::ffi::c_int, /* which pcm is attached. -1 means no pcm is attached */
    pub prev_pcm_idx: ::core::ffi::c_int, /* previously assigned pcm index */
    pub repoll_count: ::core::ffi::c_int,
    pub setup: bool, /* the stream has been set up by prepare callback */
    pub silent_stream: bool,
    pub channels: ::core::ffi::c_int, /* current number of channels */
    pub non_pcm: bool,
    pub chmap_set: bool,       /* channel-map override by ALSA API? */
    pub chmap: [::core::ffi::c_uchar; 8], /* ALSA API channel-map */
    // CONFIG_SND_PROC_FS:
    // pub proc_entry: *mut snd_info_entry,
}

/* operations used by generic code that can be overridden by codec drivers */
#[repr(C)]
pub struct hdmi_ops {
    pub pin_get_eld: Option<
        unsafe extern "C" fn(
            codec: *mut hda_codec,
            pin_nid: hda_nid_t,
            dev_id: ::core::ffi::c_int,
            buf: *mut ::core::ffi::c_uchar,
            eld_size: *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,

    pub pin_setup_infoframe: Option<
        unsafe extern "C" fn(
            codec: *mut hda_codec,
            pin_nid: hda_nid_t,
            dev_id: ::core::ffi::c_int,
            ca: ::core::ffi::c_int,
            active_channels: ::core::ffi::c_int,
            conn_type: ::core::ffi::c_int,
        ),
    >,

    /* enable/disable HBR (HD passthrough) */
    pub pin_hbr_setup: Option<
        unsafe extern "C" fn(
            codec: *mut hda_codec,
            pin_nid: hda_nid_t,
            dev_id: ::core::ffi::c_int,
            hbr: bool,
        ) -> ::core::ffi::c_int,
    >,

    pub setup_stream: Option<
        unsafe extern "C" fn(
            codec: *mut hda_codec,
            cvt_nid: hda_nid_t,
            pin_nid: hda_nid_t,
            dev_id: ::core::ffi::c_int,
            stream_tag: u32,
            format: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,

    /*
     * Optional hook invoked at the beginning of the PCM prepare
     * sequence, before the audio infoframe and stream format are
     * (re)programmed. Used to disable keep-alive / silent stream so
     * that the format change is not done while keep-alive is active.
     */
    pub prepare: Option<
        unsafe extern "C" fn(codec: *mut hda_codec, per_pin: *mut hdmi_spec_per_pin),
    >,

    pub pin_cvt_fixup: Option<
        unsafe extern "C" fn(
            codec: *mut hda_codec,
            per_pin: *mut hdmi_spec_per_pin,
            cvt_nid: hda_nid_t,
        ),
    >,

    pub silent_stream: Option<
        unsafe extern "C" fn(
            codec: *mut hda_codec,
            per_pin: *mut hdmi_spec_per_pin,
            enable: bool,
        ),
    >,
}

#[repr(C)]
pub struct hdmi_pcm {
    pub pcm: *mut hda_pcm,
    pub jack: *mut snd_jack,
    pub eld_ctl: *mut snd_kcontrol,
}

pub const SILENT_STREAM_OFF: ::core::ffi::c_int = 0;
pub const SILENT_STREAM_KAE: ::core::ffi::c_int = 1; /* use standard HDA Keep-Alive */
pub const SILENT_STREAM_I915: ::core::ffi::c_int = 2; /* Intel i915 extension */

#[repr(C)]
pub struct hdmi_spec {
    pub codec: *mut hda_codec,
    pub num_cvts: ::core::ffi::c_int,
    pub cvts: snd_array, /* struct hdmi_spec_per_cvt */
    pub cvt_nids: [hda_nid_t; 4], /* only for haswell fix */

    /*
     * num_pins is the number of virtual pins
     * for example, there are 3 pins, and each pin
     * has 4 device entries, then the num_pins is 12
     */
    pub num_pins: ::core::ffi::c_int,
    /*
     * num_nids is the number of real pins
     * In the above example, num_nids is 3
     */
    pub num_nids: ::core::ffi::c_int,
    /*
     * dev_num is the number of device entries
     * on each pin.
     * In the above example, dev_num is 4
     */
    pub dev_num: ::core::ffi::c_int,
    pub pins: snd_array, /* struct hdmi_spec_per_pin */
    pub pcm_rec: [hdmi_pcm; 8],
    pub pcm_lock: mutex,
    pub bind_lock: mutex, /* for audio component binding */
    /* pcm_bitmap means which pcms have been assigned to pins*/
    pub pcm_bitmap: ::core::ffi::c_ulong,
    pub pcm_used: ::core::ffi::c_int, /* counter of pcm_rec[] */
    /* bitmap shows whether the pcm is opened in user space
     * bit 0 means the first playback PCM (PCM3);
     * bit 1 means the second playback PCM, and so on.
     */
    pub pcm_in_use: ::core::ffi::c_ulong,

    pub temp_eld: hdmi_eld,
    pub ops: hdmi_ops,

    pub dyn_pin_out: bool,
    pub static_pcm_mapping: bool,
    /* hdmi interrupt trigger control flag for Nvidia codec */
    pub hdmi_intr_trig_ctrl: bool,
    pub nv_dp_workaround: bool, /* workaround DP audio infoframe for Nvidia */

    pub intel_hsw_fixup: bool, /* apply Intel platform-specific fixups */
    /*
     * Non-generic VIA/NVIDIA specific
     */
    pub multiout: hda_multi_out,
    pub pcm_playback: hda_pcm_stream,

    pub use_acomp_notifier: bool, /* use eld_notify callback for hotplug */
    pub acomp_registered: bool, /* audio component registered in this driver */
    pub force_connect: bool, /* force connectivity */
    pub drm_audio_ops: drm_audio_component_audio_ops,
    pub port2pin: Option<
        unsafe extern "C" fn(codec: *mut hda_codec, port: ::core::ffi::c_int) -> ::core::ffi::c_int,
    >, /* reverse port/pin mapping */

    pub chmap: hdac_chmap,
    pub vendor_nid: hda_nid_t,
    pub port_map: *const ::core::ffi::c_int,
    pub port_num: ::core::ffi::c_int,
    pub silent_stream_type: ::core::ffi::c_int,

    pub hw_constraints_channels: *const snd_pcm_hw_constraint_list,
}

// CONFIG_SND_HDA_COMPONENT:
#[inline]
pub unsafe fn codec_has_acomp(codec: *mut hda_codec) -> bool {
    let spec = (*codec).spec as *mut hdmi_spec;

    (*spec).use_acomp_notifier
}

#[repr(C)]
pub struct hdmi_audio_infoframe {
    pub type_: u8, /* 0x84 */
    pub ver: u8,  /* 0x01 */
    pub len: u8,  /* 0x0a */

    pub checksum: u8,

    pub CC02_CT47: u8, /* CC in bits 0:2, CT in 4:7 */
    pub SS01_SF24: u8,
    pub CXT04: u8,
    pub CA: u8,
    pub LFEPBL01_LSV36_DM_INH7: u8,
}

#[repr(C)]
pub struct dp_audio_infoframe {
    pub type_: u8, /* 0x84 */
    pub len: u8,  /* 0x1b */
    pub ver: u8,  /* 0x11 << 2 */

    pub CC02_CT47: u8, /* match with HDMI infoframe from this on */
    pub SS01_SF24: u8,
    pub CXT04: u8,
    pub CA: u8,
    pub LFEPBL01_LSV36_DM_INH7: u8,
}

#[repr(C)]
pub union audio_infoframe {
    pub hdmi: ::core::mem::ManuallyDrop<hdmi_audio_infoframe>,
    pub dp: ::core::mem::ManuallyDrop<dp_audio_infoframe>,
    pub bytes: [u8; 0],
}

// LIMITED_RATE_FMT_SUPPORT:
/* support only the safe format and rate */
pub const SUPPORTED_RATES_LIMITED: u32 = SNDRV_PCM_RATE_48000;
pub const SUPPORTED_MAXBPS_LIMITED: ::core::ffi::c_int = 16;
pub const SUPPORTED_FORMATS_LIMITED: u64 = SNDRV_PCM_FMTBIT_S16_LE;

// !LIMITED_RATE_FMT_SUPPORT:
/* support all rates and formats */
pub const SUPPORTED_RATES: u32 = SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;
pub const SUPPORTED_MAXBPS: ::core::ffi::c_int = 24;
pub const SUPPORTED_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE;

/*
 * HDMI routines
 */

pub unsafe fn get_pin(spec: *mut hdmi_spec, idx: ::core::ffi::c_int) -> *mut hdmi_spec_per_pin {
    snd_array_elem(&mut (*spec).pins as *mut snd_array, idx) as *mut hdmi_spec_per_pin
}

pub unsafe fn get_cvt(spec: *mut hdmi_spec, idx: ::core::ffi::c_int) -> *mut hdmi_spec_per_cvt {
    snd_array_elem(&mut (*spec).cvts as *mut snd_array, idx) as *mut hdmi_spec_per_cvt
}

/* obtain hdmi_pcm object assigned to idx */
pub unsafe fn get_hdmi_pcm(spec: *mut hdmi_spec, idx: usize) -> *mut hdmi_pcm {
    &mut (*spec).pcm_rec[idx] as *mut hdmi_pcm
}

/* obtain hda_pcm object assigned to idx */
pub unsafe fn get_pcm_rec(spec: *mut hdmi_spec, idx: usize) -> *mut hda_pcm {
    (*get_hdmi_pcm(spec, idx)).pcm
}

unsafe extern "C" {
    /* Generic HDMI codec support */
    pub fn snd_hda_hdmi_generic_alloc(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn snd_hda_hdmi_parse_codec(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn snd_hda_hdmi_generic_probe(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn snd_hda_hdmi_generic_remove(codec: *mut hda_codec);

    pub fn snd_hda_hdmi_generic_build_pcms(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn snd_hda_hdmi_generic_build_controls(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn snd_hda_hdmi_generic_init(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn snd_hda_hdmi_generic_suspend(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn snd_hda_hdmi_generic_resume(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn snd_hda_hdmi_generic_unsol_event(codec: *mut hda_codec, res: ::core::ffi::c_uint);

    pub fn snd_hda_hdmi_pin_id_to_pin_index(
        codec: *mut hda_codec,
        pin_nid: hda_nid_t,
        dev_id: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

pub unsafe fn pin_id_to_pin_index(
    codec: *mut hda_codec,
    pin: hda_nid_t,
    dev: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    snd_hda_hdmi_pin_id_to_pin_index(codec, pin, dev)
}

unsafe extern "C" {
    pub fn snd_hda_hdmi_generic_init_per_pins(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn snd_hda_hdmi_generic_spec_free(codec: *mut hda_codec);
    pub fn snd_hda_hdmi_setup_stream(
        codec: *mut hda_codec,
        cvt_nid: hda_nid_t,
        pin_nid: hda_nid_t,
        dev_id: ::core::ffi::c_int,
        stream_tag: u32,
        format: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn snd_hda_hdmi_generic_pcm_prepare(
        hinfo: *mut hda_pcm_stream,
        codec: *mut hda_codec,
        stream_tag: ::core::ffi::c_uint,
        format: ::core::ffi::c_uint,
        substream: *mut snd_pcm_substream,
    ) -> ::core::ffi::c_int;
    pub fn snd_hda_hdmi_generic_pcm_cleanup(
        hinfo: *mut hda_pcm_stream,
        codec: *mut hda_codec,
        substream: *mut snd_pcm_substream,
    ) -> ::core::ffi::c_int;

    pub fn snd_hda_hdmi_check_presence_and_report(
        codec: *mut hda_codec,
        nid: hda_nid_t,
        dev_id: ::core::ffi::c_int,
    );
    pub fn snd_hda_hdmi_setup_audio_infoframe(
        codec: *mut hda_codec,
        per_pin: *mut hdmi_spec_per_pin,
        non_pcm: bool,
    );

    /* Audio component support */
    pub fn snd_hda_hdmi_setup_drm_audio_ops(
        codec: *mut hda_codec,
        ops: *const drm_audio_component_audio_ops,
    );
    pub fn snd_hda_hdmi_acomp_init(
        codec: *mut hda_codec,
        ops: *const drm_audio_component_audio_ops,
        port2pin: Option<
            unsafe extern "C" fn(
                codec: *mut hda_codec,
                port: ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
    );
    pub fn snd_hda_hdmi_acomp_pin_eld_notify(
        audio_ptr: *mut ::core::ffi::c_void,
        port: ::core::ffi::c_int,
        dev_id: ::core::ffi::c_int,
    );
    pub fn snd_hda_hdmi_acomp_master_bind(
        dev: *mut device,
        acomp: *mut drm_audio_component,
    ) -> ::core::ffi::c_int;
    pub fn snd_hda_hdmi_acomp_master_unbind(dev: *mut device, acomp: *mut drm_audio_component);

    /* Simple / legacy HDMI codec support */
    pub fn snd_hda_hdmi_simple_probe(
        codec: *mut hda_codec,
        cvt_nid: hda_nid_t,
        pin_nid: hda_nid_t,
    ) -> ::core::ffi::c_int;
    pub fn snd_hda_hdmi_simple_remove(codec: *mut hda_codec);

    pub fn snd_hda_hdmi_simple_build_pcms(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn snd_hda_hdmi_simple_build_controls(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn snd_hda_hdmi_simple_init(codec: *mut hda_codec) -> ::core::ffi::c_int;
    pub fn snd_hda_hdmi_simple_unsol_event(codec: *mut hda_codec, res: ::core::ffi::c_uint);
    pub fn snd_hda_hdmi_simple_pcm_open(
        hinfo: *mut hda_pcm_stream,
        codec: *mut hda_codec,
        substream: *mut snd_pcm_substream,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
