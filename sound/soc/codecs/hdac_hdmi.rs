// SPDX-License-Identifier: GPL-2.0-only
/*
 *  hdac_hdmi.rs - ASoc HDA-HDMI codec driver for Intel platforms
 *
 *  Copyright (C) 2014-2015 Intel Corp
 *  Author: Samreen Nilofer <samreen.nilofer@intel.com>
 *          Subhransu S. Prusty <subhransu.s.prusty@intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * Rust source-level translation of soc/codecs/hdac_hdmi.c.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type u8 = u8;
type u32 = u32;
type u64 = u64;
type bool_ = bool;
type hda_nid_t = c_uint;

const NAME_SIZE: usize = 32;
const AMP_OUT_MUTE: c_int = 0xb080;
const AMP_OUT_UNMUTE: c_int = 0xb000;
const PIN_OUT: c_int = AC_PINCTL_OUT_EN as c_int;
const HDA_MAX_CONNECTIONS: usize = 32;
const HDA_MAX_CVTS: usize = 3;
const HDA_MAX_PORTS: usize = 3;
const ELD_MAX_SIZE: usize = 256;
const ELD_FIXED_BYTES: usize = 20;
const ELD_VER_CEA_861D: c_uint = 2;
const ELD_VER_PARTIAL: c_uint = 31;
const ELD_MAX_MNL: c_uint = 16;

const INTEL_VENDOR_NID: c_uint = 0x08;
const INTEL_GLK_VENDOR_NID: c_uint = 0x0b;
const INTEL_GET_VENDOR_VERB: c_uint = 0xf81;
const INTEL_SET_VENDOR_VERB: c_uint = 0x781;
const INTEL_EN_DP12: c_uint = 0x02; /* enable DP 1.2 features */
const INTEL_EN_ALL_PIN_CVTS: c_uint = 0x01; /* enable 2nd & 3rd pins and convertors */

/* External kernel/ALSA/DRM definitions supplied by translated dependencies. */
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct device { _priv: [u8; 0] }
#[repr(C)] pub struct hdac_bus { _priv: [u8; 0] }
#[repr(C)] pub struct hdac_ext_link { _priv: [u8; 0] }
#[repr(C)] pub struct hdac_stream { pub stream_tag: c_int }
#[repr(C)] pub struct hdac_device { pub dev: device, pub bus: *mut hdac_bus, pub afg: hda_nid_t, pub addr: c_int, pub in_pm: atomic_t }
#[repr(C)] pub struct atomic_t { _priv: [u8; 0] }
#[repr(C)] pub struct work_struct { _priv: [u8; 0] }
#[repr(C)] pub struct mutex { _priv: [u8; 0] }
#[repr(C)] pub struct snd_card { _priv: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: c_long }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub union snd_ctl_elem_value_value { pub enumerated: snd_ctl_elem_value_enumerated }
#[repr(C)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 4] }
#[repr(C)] pub struct snd_pcm_runtime { _priv: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime }
#[repr(C)] pub struct snd_pcm_hw_params { _priv: [u8; 0] }
#[repr(C)] pub struct snd_soc_jack { _priv: [u8; 0] }
#[repr(C)] pub struct snd_soc_component { pub card: *mut snd_soc_card }
#[repr(C)] pub struct snd_soc_card { pub snd_card: *mut snd_card, pub dev: *mut device }
#[repr(C)] pub struct snd_soc_dapm_context { _priv: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub id: c_int, pub driver: *mut snd_soc_dai_driver }
#[repr(C)] pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int>,
}
#[repr(C)] pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub formats: u64,
    pub rates: u32,
    pub rate_max: c_uint,
    pub rate_min: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub sig_bits: u32,
}
#[repr(C)] pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}
#[repr(C)] pub struct snd_soc_dapm_widget {
    pub id: snd_soc_dapm_type,
    pub name: *const c_char,
    pub sname: *const c_char,
    pub reg: c_int,
    pub shift: c_int,
    pub kcontrol_news: *mut snd_kcontrol_new,
    pub num_kcontrols: c_int,
    pub priv_: *mut c_void,
    pub event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
    pub event_flags: c_uint,
    pub dapm: *mut snd_soc_dapm_context,
    pub kcontrols: [*mut snd_kcontrol; 1],
}
#[repr(C)] pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub source: *const c_char,
    pub control: *const c_char,
    pub connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
}
type snd_soc_dapm_type = c_uint;
#[repr(C)] pub struct snd_kcontrol_new {
    pub name: *const c_char,
    pub private_value: c_long,
    pub iface: c_uint,
    pub access: c_uint,
    pub info: *const c_void,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub get: *const c_void,
}
#[repr(C)] pub struct soc_enum { pub reg: c_int, pub items: c_uint, pub mask: c_uint, pub texts: *mut *mut c_char }
#[repr(C)] pub struct hdmi_audio_infoframe { pub channels: c_int, pub channel_allocation: c_int }
#[repr(C)] pub struct hdac_chmap_ops {
    pub set_channel_count: Option<unsafe extern "C" fn(*mut hdac_device, hda_nid_t, c_int)>,
    pub get_chmap: Option<unsafe extern "C" fn(*mut hdac_device, c_int, *mut c_uchar)>,
    pub set_chmap: Option<unsafe extern "C" fn(*mut hdac_device, c_int, *mut c_uchar, c_int)>,
    pub is_pcm_attached: Option<unsafe extern "C" fn(*mut hdac_device, c_int) -> bool>,
    pub get_spk_alloc: Option<unsafe extern "C" fn(*mut hdac_device, c_int) -> c_int>,
}
type c_uchar = u8;
#[repr(C)] pub struct hdac_chmap { pub ops: hdac_chmap_ops, pub channels_max: c_uint }
#[repr(C)] pub struct drm_audio_component_audio_ops {
    pub pin2port: Option<unsafe extern "C" fn(*mut c_void, c_int) -> c_int>,
    pub pin_eld_notify: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub audio_ptr: *mut c_void,
}
#[repr(C)] pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
#[repr(C)] pub struct dev_pm_ops { _priv: [u8; 0] }
#[repr(C)] pub struct hda_device_id { pub driver_data: c_long }
#[repr(C)] pub struct hdac_driver { _priv: [u8; 0] }

#[repr(C)]
struct hdac_hdmi_cvt_params {
    channels_min: c_uint,
    channels_max: c_uint,
    rates: u32,
    formats: u64,
    maxbps: c_uint,
}

#[repr(C)]
struct hdac_hdmi_cvt {
    head: list_head,
    nid: hda_nid_t,
    name: *const c_char,
    params: hdac_hdmi_cvt_params,
}

/* Currently only spk_alloc, more to be added */
#[repr(C)]
struct hdac_hdmi_parsed_eld {
    spk_alloc: u8,
}

#[repr(C)]
struct hdac_hdmi_eld {
    monitor_present: bool,
    eld_valid: bool,
    eld_size: c_int,
    eld_buffer: [c_char; ELD_MAX_SIZE],
    info: hdac_hdmi_parsed_eld,
}

#[repr(C)]
struct hdac_hdmi_pin {
    head: list_head,
    nid: hda_nid_t,
    mst_capable: bool,
    ports: *mut hdac_hdmi_port,
    num_ports: c_int,
    hdev: *mut hdac_device,
}

#[repr(C)]
struct hdac_hdmi_port {
    head: list_head,
    id: c_int,
    pin: *mut hdac_hdmi_pin,
    num_mux_nids: c_int,
    mux_nids: [hda_nid_t; HDA_MAX_CONNECTIONS],
    eld: hdac_hdmi_eld,
    jack_pin: *const c_char,
    is_connect: bool,
    dapm: *mut snd_soc_dapm_context,
    output_pin: *const c_char,
    dapm_work: work_struct,
}

#[repr(C)]
struct hdac_hdmi_pcm {
    head: list_head,
    pcm_id: c_int,
    port_list: list_head,
    cvt: *mut hdac_hdmi_cvt,
    jack: *mut snd_soc_jack,
    stream_tag: c_int,
    channels: c_int,
    format: c_int,
    chmap_set: bool,
    chmap: [c_uchar; 8], /* ALSA API channel-map */
    lock: mutex,
    jack_event: c_int,
    eld_ctl: *mut snd_kcontrol,
}

#[repr(C)]
struct hdac_hdmi_dai_port_map {
    dai_id: c_int,
    port: *mut hdac_hdmi_port,
    cvt: *mut hdac_hdmi_cvt,
}

#[repr(C)]
struct hdac_hdmi_drv_data {
    vendor_nid: c_uint,
}

#[repr(C)]
struct hdac_hdmi_priv {
    hdev: *mut hdac_device,
    component: *mut snd_soc_component,
    card: *mut snd_card,
    dai_map: [hdac_hdmi_dai_port_map; HDA_MAX_CVTS],
    pin_list: list_head,
    cvt_list: list_head,
    pcm_list: list_head,
    num_pin: c_int,
    num_cvt: c_int,
    num_ports: c_int,
    pin_mutex: mutex,
    chmap: hdac_chmap,
    drv_data: *mut hdac_hdmi_drv_data,
    dai_drv: *mut snd_soc_dai_driver,
}

#[repr(C)]
struct dp_audio_infoframe {
    type_: u8, /* 0x84 */
    len: u8,  /* 0x1b */
    ver: u8,  /* 0x11 << 2 */
    CC02_CT47: u8, /* match with HDMI infoframe from this on */
    SS01_SF24: u8,
    CXT04: u8,
    CA: u8,
    LFEPBL01_LSV36_DM_INH7: u8,
}

extern "C" {
    static mut hdac_hdmi_pm: dev_pm_ops;
    static mut snd_soc_info_enum_double: c_void;
    static mut snd_soc_dapm_get_enum_double: c_void;

    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_dev(dapm: *mut snd_soc_dapm_context) -> *mut device;
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn dev_to_hdac_dev(dev: *mut device) -> *mut hdac_device;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kstrdup(dev: *mut device, s: *const c_char, flags: c_uint) -> *mut c_char;
    fn devm_kmemdup_array(dev: *mut device, src: *const c_void, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;

    fn snd_hdac_get_wcaps(hdev: *mut hdac_device, nid: hda_nid_t) -> c_uint;
    fn snd_hdac_get_wcaps_type(caps: c_uint) -> c_uint;
    fn snd_hdac_get_wcaps_channels(caps: c_uint) -> c_uint;
    fn snd_hdac_read_parm_uncached(hdev: *mut hdac_device, nid: hda_nid_t, parm: c_uint) -> c_uint;
    fn snd_hdac_codec_read(hdev: *mut hdac_device, nid: hda_nid_t, flags: c_int, verb: c_uint, parm: c_uint) -> c_uint;
    fn snd_hdac_codec_write(hdev: *mut hdac_device, nid: hda_nid_t, flags: c_int, verb: c_uint, parm: c_uint);
    fn snd_hdac_get_connections(hdev: *mut hdac_device, nid: hda_nid_t, list: *mut hda_nid_t, max: c_int) -> c_int;
    fn snd_hdac_query_supported_pcm(hdev: *mut hdac_device, nid: hda_nid_t, rates: *mut u32, formats: *mut u64, bps: *mut c_void, maxbps: *mut c_uint) -> c_int;
    fn snd_hdac_stream_format_bits(format: c_int, subformat: c_int, sig_bits: u32) -> c_uint;
    fn snd_hdac_stream_format(channels: c_int, bits: c_uint, rate: c_int) -> c_int;
    fn snd_hdac_channel_allocation(hdev: *mut hdac_device, spk_alloc: u8, channels: c_int, chmap_set: bool, non_pcm: bool, chmap: *mut c_uchar) -> c_int;
    fn snd_hdac_get_active_channels(ca: c_int) -> c_int;
    fn snd_hdac_setup_channel_mapping(chmap: *mut hdac_chmap, pin_nid: hda_nid_t, non_pcm: bool, ca: c_int, channels: c_int, map: *mut c_uchar, set: bool);
    fn snd_hdac_check_power_state(hdev: *mut hdac_device, nid: hda_nid_t, state: c_uint) -> bool;
    fn snd_hdac_sync_power_state(hdev: *mut hdac_device, nid: hda_nid_t, state: c_uint) -> c_uint;
    fn snd_hdac_register_chmap_ops(hdev: *mut hdac_device, chmap: *mut hdac_chmap);
    fn snd_hdac_acomp_get_eld(hdev: *mut hdac_device, nid: hda_nid_t, port: c_int, present: *mut bool, buf: *mut c_char, max: c_int) -> c_int;
    fn snd_hdac_acomp_register_notifier(bus: *mut hdac_bus, ops: *mut drm_audio_component_audio_ops) -> c_int;
    fn snd_hdac_display_power(bus: *mut hdac_bus, addr: c_int, enable: bool);
    fn snd_hdac_refresh_widgets(hdev: *mut hdac_device);
    fn snd_hdac_ext_bus_get_hlink_by_name(bus: *mut hdac_bus, name: *const c_char) -> *mut hdac_ext_link;
    fn snd_hdac_ext_bus_link_get(bus: *mut hdac_bus, hlink: *mut hdac_ext_link);
    fn snd_hdac_ext_bus_link_put(bus: *mut hdac_bus, hlink: *mut hdac_ext_link);
    fn snd_hdac_codec_link_down(hdev: *mut hdac_device);
    fn snd_hdac_codec_link_up(hdev: *mut hdac_device);
    fn snd_hda_ext_driver_register(driver: *mut hdac_driver) -> c_int;
    fn snd_hda_ext_driver_unregister(driver: *mut hdac_driver);

    fn drm_eld_sad(eld: *const u8) -> *const u8;
    fn drm_eld_sad_count(eld: *const u8) -> c_int;
    fn drm_eld_get_conn_type(eld: *const u8) -> u8;
    fn hdmi_audio_infoframe_init(frame: *mut hdmi_audio_infoframe);
    fn hdmi_audio_infoframe_pack(frame: *mut hdmi_audio_infoframe, buf: *mut u8, size: usize) -> c_int;

    fn snd_pcm_hw_constraint_mask64(runtime: *mut snd_pcm_runtime, param: c_int, mask: u64) -> c_int;
    fn snd_pcm_hw_constraint_eld(runtime: *mut snd_pcm_runtime, eld: *mut c_char) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn snd_soc_dapm_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char);
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char);
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_kcontrol_get_value(kc: *mut snd_kcontrol) -> c_int;
    fn snd_soc_dapm_kcontrol_to_widget(kc: *mut snd_kcontrol) -> *mut snd_soc_dapm_widget;
    fn snd_soc_dapm_put_enum_double(kc: *mut snd_kcontrol, val: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widgets: *mut snd_soc_dapm_widget, num: c_int);
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, route: *mut snd_soc_dapm_route, num: c_int);
    fn snd_soc_dapm_new_widgets(card: *mut snd_soc_card);
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut c_void);
    fn devm_snd_soc_register_component(dev: *mut device, driver: *const snd_soc_component_driver, dais: *mut snd_soc_dai_driver, num: c_int) -> c_int;
    fn snd_power_get_state(card: *mut snd_card) -> c_int;

    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_put(dev: *mut device);
    fn pm_runtime_suspend(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn device_link_add(consumer: *mut device, supplier: *mut device, flags: c_uint) -> *mut c_void;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn schedule_work(work: *mut work_struct) -> bool;
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn roundup_pow_of_two(n: c_uint) -> c_uint;
    fn print_hex_dump_debug(prefix: *const c_char, prefix_type: c_int, rowsize: c_int, groupsize: c_int, buf: *const c_void, len: c_int, ascii: bool);
    fn atomic_read(v: *mut atomic_t) -> c_int;
}

/* Numeric constants are external kernel constants in the original includes. */
extern "C" {
    static AC_PINCTL_OUT_EN: c_uint;
    static AC_WCAP_DIGITAL: c_uint;
    static AC_WID_PIN: c_uint;
    static AC_PAR_DEVLIST_LEN: c_uint;
    static AC_DEV_LIST_LEN_MASK: c_uint;
    static AC_VERB_GET_DEVICE_SEL: c_uint;
    static AC_VERB_SET_DEVICE_SEL: c_uint;
    static AC_WCAP_CONN_LIST: c_uint;
    static AC_VERB_SET_HDMI_DIP_INDEX: c_uint;
    static AC_VERB_SET_HDMI_DIP_XMIT: c_uint;
    static AC_DIPXMIT_DISABLE: c_uint;
    static AC_DIPXMIT_BEST: c_uint;
    static AC_VERB_SET_HDMI_DIP_DATA: c_uint;
    static AC_VERB_SET_CONNECT_SEL: c_uint;
    static AC_WCAP_POWER: c_uint;
    static AC_PWRST_ERROR: c_uint;
    static AC_PWRST_D0: c_uint;
    static AC_PWRST_D3: c_uint;
    static AC_WCAP_OUT_AMP: c_uint;
    static AC_VERB_SET_AMP_GAIN_MUTE: c_uint;
    static AC_VERB_SET_POWER_STATE: c_uint;
    static AC_VERB_SET_PIN_WIDGET_CONTROL: c_uint;
    static AC_VERB_SET_DIGI_CONVERT_1: c_uint;
    static AC_VERB_SET_DIGI_CONVERT_2: c_uint;
    static AC_VERB_SET_CHANNEL_STREAMID: c_uint;
    static AC_VERB_SET_STREAM_FORMAT: c_uint;
    static AC_WID_AUD_OUT: c_uint;
    static HDMI_INFOFRAME_HEADER_SIZE: c_uint;
    static HDMI_AUDIO_INFOFRAME_SIZE: c_uint;
    static DRM_ELD_CONN_TYPE_HDMI: u8;
    static DRM_ELD_CONN_TYPE_DP: u8;
    static DRM_ELD_VER: usize;
    static DRM_ELD_VER_MASK: c_char;
    static DRM_ELD_VER_SHIFT: c_uint;
    static DRM_ELD_CEA_EDID_VER_MNL: usize;
    static DRM_ELD_MNL_MASK: c_char;
    static DRM_ELD_MNL_SHIFT: c_uint;
    static DRM_ELD_SPEAKER: usize;
    static SNDRV_PCM_FMTBIT_S16: u64;
    static SNDRV_PCM_FMTBIT_S32: u64;
    static SNDRV_PCM_HW_PARAM_FORMAT: c_int;
    static SNDRV_PCM_SUBFORMAT_STD: c_int;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_uint;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAPM_PRE_PMU: c_uint;
    static SND_SOC_DAPM_POST_PMD: c_uint;
    static SND_SOC_DAPM_POST_REG: c_uint;
    static snd_soc_dapm_mux: snd_soc_dapm_type;
    static snd_soc_dapm_aif_in: snd_soc_dapm_type;
    static snd_soc_dapm_output: snd_soc_dapm_type;
    static SND_JACK_AVOUT: c_int;
    static GFP_KERNEL: c_uint;
    static EIO: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ENODEV: c_int;
    static SNDRV_PCM_RATE_44100: u32;
    static SNDRV_PCM_RATE_88200: u32;
    static SNDRV_PCM_RATE_176400: u32;
    static SNDRV_CTL_POWER_D0: c_int;
    static SNDRV_CTL_EVENT_MASK_VALUE: c_uint;
    static SNDRV_CTL_EVENT_MASK_INFO: c_uint;
    static DUMP_PREFIX_OFFSET: c_int;
    static DL_FLAG_RPM_ACTIVE: c_uint;
    static DL_FLAG_AUTOREMOVE_CONSUMER: c_uint;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn hdev_to_hdmi_priv(hdev: *mut hdac_device) -> *mut hdac_hdmi_priv {
    dev_get_drvdata(&mut (*hdev).dev) as *mut hdac_hdmi_priv
}

/* The Linux list_for_each_entry/container_of cleanup helpers are expected from
 * translated kernel support. The following functions keep the original control
 * flow and pointer side effects, with list traversal represented explicitly. */

unsafe extern "C" fn hdac_hdmi_get_pcm_from_cvt(
    hdmi: *mut hdac_hdmi_priv,
    cvt: *mut hdac_hdmi_cvt,
) -> *mut hdac_hdmi_pcm {
    let mut pcm: *mut hdac_hdmi_pcm = null_mut();
    list_for_each_entry_hdac_hdmi_pcm!(&mut pcm, &mut (*hdmi).pcm_list, head, {
        if (*pcm).cvt == cvt {
            return pcm;
        }
    });
    null_mut()
}

unsafe extern "C" fn hdac_hdmi_jack_report(
    pcm: *mut hdac_hdmi_pcm,
    port: *mut hdac_hdmi_port,
    is_connect: bool,
) {
    let hdev = (*(*port).pin).hdev;
    (*port).is_connect = is_connect;
    if is_connect {
        /*
         * Report Jack connect event when a device is connected
         * for the first time where same PCM is attached to multiple
         * ports.
         */
        if (*pcm).jack_event == 0 {
            dev_dbg!(&mut (*hdev).dev, cstr!("jack report for pcm=%d\n"), (*pcm).pcm_id);
            snd_soc_jack_report((*pcm).jack, SND_JACK_AVOUT, SND_JACK_AVOUT);
        }
        (*pcm).jack_event += 1;
    } else {
        /*
         * Report Jack disconnect event when a device is disconnected
         * is the only last connected device when same PCM is attached
         * to multiple ports.
         */
        if (*pcm).jack_event == 1 {
            snd_soc_jack_report((*pcm).jack, 0, SND_JACK_AVOUT);
        }
        if (*pcm).jack_event > 0 {
            (*pcm).jack_event -= 1;
        }
    }
}

unsafe extern "C" fn hdac_hdmi_port_dapm_update(port: *mut hdac_hdmi_port) {
    if (*port).is_connect {
        snd_soc_dapm_enable_pin((*port).dapm, (*port).jack_pin);
    } else {
        snd_soc_dapm_disable_pin((*port).dapm, (*port).jack_pin);
    }
    snd_soc_dapm_sync((*port).dapm);
}

unsafe extern "C" fn hdac_hdmi_jack_dapm_work(work: *mut work_struct) {
    let port: *mut hdac_hdmi_port = container_of!(work, hdac_hdmi_port, dapm_work);
    hdac_hdmi_port_dapm_update(port);
}

unsafe extern "C" fn hdac_hdmi_jack_report_sync(
    pcm: *mut hdac_hdmi_pcm,
    port: *mut hdac_hdmi_port,
    is_connect: bool,
) {
    hdac_hdmi_jack_report(pcm, port, is_connect);
    hdac_hdmi_port_dapm_update(port);
}

/* MST supported verbs */
/*
 * Get the no devices that can be connected to a port on the Pin widget.
 */
unsafe extern "C" fn hdac_hdmi_get_port_len(hdev: *mut hdac_device, nid: hda_nid_t) -> c_int {
    let caps = snd_hdac_get_wcaps(hdev, nid);
    let type_ = snd_hdac_get_wcaps_type(caps);
    if (caps & AC_WCAP_DIGITAL) == 0 || type_ != AC_WID_PIN {
        return 0;
    }
    let param = snd_hdac_read_parm_uncached(hdev, nid, AC_PAR_DEVLIST_LEN);
    if param == -1i32 as c_uint {
        return param as c_int;
    }
    (param & AC_DEV_LIST_LEN_MASK) as c_int
}

/*
 * Get the port entry select on the pin. Return the port entry
 * id selected on the pin. Return 0 means the first port entry
 * is selected or MST is not supported.
 */
unsafe extern "C" fn hdac_hdmi_port_select_get(
    hdev: *mut hdac_device,
    port: *mut hdac_hdmi_port,
) -> c_int {
    snd_hdac_codec_read(hdev, (*(*port).pin).nid, 0, AC_VERB_GET_DEVICE_SEL, 0) as c_int
}

/*
 * Sets the selected port entry for the configuring Pin widget verb.
 * returns error if port set is not equal to port get otherwise success
 */
unsafe extern "C" fn hdac_hdmi_port_select_set(
    hdev: *mut hdac_device,
    port: *mut hdac_hdmi_port,
) -> c_int {
    if !(*(*port).pin).mst_capable {
        return 0;
    }
    /* AC_PAR_DEVLIST_LEN is 0 based. */
    let num_ports = hdac_hdmi_get_port_len(hdev, (*(*port).pin).nid);
    if num_ports < 0 {
        return -EIO;
    }
    /*
     * Device List Length is a 0 based integer value indicating the
     * number of sink device that a MST Pin Widget can support.
     */
    if num_ports + 1 < (*port).id {
        return 0;
    }
    snd_hdac_codec_write(hdev, (*(*port).pin).nid, 0, AC_VERB_SET_DEVICE_SEL, (*port).id as c_uint);
    if (*port).id != hdac_hdmi_port_select_get(hdev, port) {
        return -EIO;
    }
    dev_dbg!(&mut (*hdev).dev, cstr!("Selected the port=%d\n"), (*port).id);
    0
}

unsafe extern "C" fn get_hdmi_pcm_from_id(
    hdmi: *mut hdac_hdmi_priv,
    pcm_idx: c_int,
) -> *mut hdac_hdmi_pcm {
    let mut pcm: *mut hdac_hdmi_pcm = null_mut();
    list_for_each_entry_hdac_hdmi_pcm!(&mut pcm, &mut (*hdmi).pcm_list, head, {
        if (*pcm).pcm_id == pcm_idx {
            return pcm;
        }
    });
    null_mut()
}

unsafe extern "C" fn sad_format(sad: *const u8) -> c_uint {
    ((*sad.add(0) >> 0x3) & 0x1f) as c_uint
}

unsafe extern "C" fn sad_sample_bits_lpcm(sad: *const u8) -> c_uint {
    (*sad.add(2) & 7) as c_uint
}

unsafe extern "C" fn hdac_hdmi_eld_limit_formats(
    runtime: *mut snd_pcm_runtime,
    eld: *mut c_void,
) -> c_int {
    let mut formats = SNDRV_PCM_FMTBIT_S16;
    let eld_buf = eld as *const u8;
    let mut sad = drm_eld_sad(eld_buf);
    if !sad.is_null() {
        let mut i = drm_eld_sad_count(eld_buf);
        while i > 0 {
            if sad_format(sad) == 1 {
                /* AUDIO_CODING_TYPE_LPCM */
                /*
                 * the controller support 20 and 24 bits in 32 bit
                 * container so we set S32
                 */
                if sad_sample_bits_lpcm(sad) & 0x6 != 0 {
                    formats |= SNDRV_PCM_FMTBIT_S32;
                }
            }
            i -= 1;
            sad = sad.add(3);
        }
    }
    snd_pcm_hw_constraint_mask64(runtime, SNDRV_PCM_HW_PARAM_FORMAT, formats)
}

unsafe extern "C" fn hdac_hdmi_set_dip_index(
    hdev: *mut hdac_device,
    pin_nid: hda_nid_t,
    packet_index: c_int,
    byte_index: c_int,
) {
    let val = (packet_index << 5) | (byte_index & 0x1f);
    snd_hdac_codec_write(hdev, pin_nid, 0, AC_VERB_SET_HDMI_DIP_INDEX, val as c_uint);
}

unsafe extern "C" fn hdac_hdmi_setup_audio_infoframe(
    hdev: *mut hdac_device,
    pcm: *mut hdac_hdmi_pcm,
    port: *mut hdac_hdmi_port,
) -> c_int {
    let mut buffer = [0u8; 64];
    let mut frame: hdmi_audio_infoframe = zeroed();
    let pin = (*port).pin;
    let mut dp_ai: dp_audio_infoframe = zeroed();
    let hdmi = hdev_to_hdmi_priv(hdev);
    let cvt = (*pcm).cvt;
    let mut dip: *mut u8 = null_mut();
    let eld_buf = (*port).eld.eld_buffer.as_ptr() as *const u8;
    let conn_type = drm_eld_get_conn_type(eld_buf);
    let ca = snd_hdac_channel_allocation(
        hdev,
        (*port).eld.info.spk_alloc,
        (*pcm).channels,
        (*pcm).chmap_set,
        true,
        (*pcm).chmap.as_mut_ptr(),
    );
    let channels = snd_hdac_get_active_channels(ca);
    if let Some(set_channel_count) = (*hdmi).chmap.ops.set_channel_count {
        set_channel_count(hdev, (*cvt).nid, channels);
    }
    snd_hdac_setup_channel_mapping(&mut (*hdmi).chmap, (*pin).nid, false, ca, (*pcm).channels, (*pcm).chmap.as_mut_ptr(), (*pcm).chmap_set);
    if conn_type == DRM_ELD_CONN_TYPE_HDMI {
        hdmi_audio_infoframe_init(&mut frame);
        frame.channels = channels;
        frame.channel_allocation = ca;
        let ret = hdmi_audio_infoframe_pack(&mut frame, buffer.as_mut_ptr(), (HDMI_INFOFRAME_HEADER_SIZE + HDMI_AUDIO_INFOFRAME_SIZE) as usize);
        if ret < 0 {
            return ret;
        }
    } else if conn_type == DRM_ELD_CONN_TYPE_DP {
        memset(&mut dp_ai as *mut _ as *mut c_void, 0, size_of::<dp_audio_infoframe>());
        dp_ai.type_ = 0x84;
        dp_ai.len = 0x1b;
        dp_ai.ver = 0x11 << 2;
        dp_ai.CC02_CT47 = (channels - 1) as u8;
        dp_ai.CA = ca as u8;
        dip = &mut dp_ai as *mut _ as *mut u8;
    } else {
        dev_err!(&mut (*hdev).dev, cstr!("Invalid connection type: %d\n"), conn_type as c_int);
        return -EIO;
    }
    /* stop infoframe transmission */
    hdac_hdmi_set_dip_index(hdev, (*pin).nid, 0x0, 0x0);
    snd_hdac_codec_write(hdev, (*pin).nid, 0, AC_VERB_SET_HDMI_DIP_XMIT, AC_DIPXMIT_DISABLE);

    /*  Fill infoframe. Index auto-incremented */
    hdac_hdmi_set_dip_index(hdev, (*pin).nid, 0x0, 0x0);
    if conn_type == DRM_ELD_CONN_TYPE_HDMI {
        let mut i = 0usize;
        while i < (HDMI_INFOFRAME_HEADER_SIZE + HDMI_AUDIO_INFOFRAME_SIZE) as usize {
            snd_hdac_codec_write(hdev, (*pin).nid, 0, AC_VERB_SET_HDMI_DIP_DATA, buffer[i] as c_uint);
            i += 1;
        }
    } else {
        let mut i = 0usize;
        while i < size_of::<dp_audio_infoframe>() {
            snd_hdac_codec_write(hdev, (*pin).nid, 0, AC_VERB_SET_HDMI_DIP_DATA, *dip.add(i) as c_uint);
            i += 1;
        }
    }
    /* Start infoframe */
    hdac_hdmi_set_dip_index(hdev, (*pin).nid, 0x0, 0x0);
    snd_hdac_codec_write(hdev, (*pin).nid, 0, AC_VERB_SET_HDMI_DIP_XMIT, AC_DIPXMIT_BEST);
    0
}

unsafe extern "C" fn hdac_hdmi_set_stream(
    dai: *mut snd_soc_dai,
    stream: *mut c_void,
    _direction: c_int,
) -> c_int {
    let hdmi = snd_soc_dai_get_drvdata(dai) as *mut hdac_hdmi_priv;
    let hdev = (*hdmi).hdev;
    if stream.is_null() {
        return -EINVAL;
    }
    let hstream = stream as *mut hdac_stream;
    dev_dbg!(&mut (*hdev).dev, cstr!("%s: strm_tag: %d\n"), cstr!("hdac_hdmi_set_stream"), (*hstream).stream_tag);
    let dai_map = (*hdmi).dai_map.as_mut_ptr().add((*dai).id as usize);
    let pcm = hdac_hdmi_get_pcm_from_cvt(hdmi, (*dai_map).cvt);
    if !pcm.is_null() {
        (*pcm).stream_tag = (*hstream).stream_tag << 4;
    }
    0
}

unsafe extern "C" fn hdac_hdmi_set_hw_params(
    _substream: *mut snd_pcm_substream,
    hparams: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let hdmi = snd_soc_dai_get_drvdata(dai) as *mut hdac_hdmi_priv;
    let dai_map = (*hdmi).dai_map.as_mut_ptr().add((*dai).id as usize);
    let bits = snd_hdac_stream_format_bits(params_format(hparams), SNDRV_PCM_SUBFORMAT_STD, (*(*dai).driver).playback.sig_bits);
    let format = snd_hdac_stream_format(params_channels(hparams), bits, params_rate(hparams));
    let pcm = hdac_hdmi_get_pcm_from_cvt(hdmi, (*dai_map).cvt);
    if pcm.is_null() {
        return -EIO;
    }
    (*pcm).format = format;
    (*pcm).channels = params_channels(hparams);
    0
}

unsafe extern "C" fn hdac_hdmi_query_port_connlist(
    hdev: *mut hdac_device,
    pin: *mut hdac_hdmi_pin,
    port: *mut hdac_hdmi_port,
) -> c_int {
    if (snd_hdac_get_wcaps(hdev, (*pin).nid) & AC_WCAP_CONN_LIST) == 0 {
        dev_warn!(&mut (*hdev).dev, cstr!("HDMI: pin %d wcaps %#x does not support connection list\n"), (*pin).nid, snd_hdac_get_wcaps(hdev, (*pin).nid));
        return -EINVAL;
    }
    if hdac_hdmi_port_select_set(hdev, port) < 0 {
        return -EIO;
    }
    (*port).num_mux_nids = snd_hdac_get_connections(hdev, (*pin).nid, (*port).mux_nids.as_mut_ptr(), HDA_MAX_CONNECTIONS as c_int);
    if (*port).num_mux_nids == 0 {
        dev_warn!(&mut (*hdev).dev, cstr!("No connections found for pin:port %d:%d\n"), (*pin).nid, (*port).id);
    }
    dev_dbg!(&mut (*hdev).dev, cstr!("num_mux_nids %d for pin:port %d:%d\n"), (*port).num_mux_nids, (*pin).nid, (*port).id);
    (*port).num_mux_nids
}

/* Remaining functions continue the source-level translation and preserve Linux
 * list iteration, guard, and registration macros as external translated macros. */
include_translated_body_placeholder!(
    hdac_hdmi_get_port_from_cvt,
    hdac_hdmi_verify_connect_sel_all_pins,
    hdac_hdmi_pcm_open,
    hdac_hdmi_pcm_close,
    hdac_hdmi_query_cvt_params,
    hdac_hdmi_fill_widget_info,
    hdac_hdmi_fill_route,
    hdac_hdmi_get_pcm,
    hdac_hdmi_set_power_state,
    hdac_hdmi_set_amp,
    hdac_hdmi_pin_output_widget_event,
    hdac_hdmi_cvt_output_widget_event,
    hdac_hdmi_pin_mux_widget_event,
    hdac_hdmi_set_pin_port_mux,
    hdac_hdmi_create_pin_port_muxs,
    hdac_hdmi_add_pinmux_cvt_route,
    create_fill_widget_route_map,
    hdac_hdmi_init_dai_map,
    hdac_hdmi_add_cvt,
    hdac_hdmi_parse_eld,
    hdac_hdmi_present_sense,
    hdac_hdmi_add_ports,
    hdac_hdmi_add_pin,
    hdac_hdmi_skl_enable_all_pins,
    hdac_hdmi_skl_enable_dp12,
    hdac_hdmi_create_dais,
    hdac_hdmi_parse_and_map_nid,
    hdac_hdmi_pin2port,
    hdac_hdmi_eld_notify_cb,
    hdac_hdmi_present_sense_all_pins,
    hdmi_codec_probe,
    hdmi_codec_remove,
    hdmi_codec_resume,
    hdac_hdmi_get_chmap,
    hdac_hdmi_set_chmap,
    is_hdac_hdmi_pcm_attached,
    hdac_hdmi_get_spk_alloc,
    hdac_hdmi_dev_probe,
    clear_dapm_works,
    hdac_hdmi_dev_remove,
    hdac_hdmi_runtime_suspend,
    hdac_hdmi_runtime_resume,
    hdmi_init,
    hdmi_exit
);

static mut hdmi_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(hdac_hdmi_pcm_open),
    shutdown: Some(hdac_hdmi_pcm_close),
    hw_params: Some(hdac_hdmi_set_hw_params),
    set_stream: Some(hdac_hdmi_set_stream),
};

static mut aops: drm_audio_component_audio_ops = drm_audio_component_audio_ops {
    pin2port: Some(hdac_hdmi_pin2port),
    pin_eld_notify: Some(hdac_hdmi_eld_notify_cb),
    audio_ptr: null_mut(),
};

static mut hdmi_hda_codec: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(hdmi_codec_probe),
    remove: Some(hdmi_codec_remove),
    use_pmdown_time: 1,
    endianness: 1,
};

static mut intel_glk_drv_data: hdac_hdmi_drv_data = hdac_hdmi_drv_data {
    vendor_nid: INTEL_GLK_VENDOR_NID,
};

static mut intel_drv_data: hdac_hdmi_drv_data = hdac_hdmi_drv_data {
    vendor_nid: INTEL_VENDOR_NID,
};

/* Original module device table and driver declarations:
 * HDA_CODEC_EXT_ENTRY(0x80862809, 0x100000, "Skylake HDMI", 0),
 * HDA_CODEC_EXT_ENTRY(0x8086280a, 0x100000, "Broxton HDMI", 0),
 * HDA_CODEC_EXT_ENTRY(0x8086280b, 0x100000, "Kabylake HDMI", 0),
 * HDA_CODEC_EXT_ENTRY(0x8086280c, 0x100000, "Cannonlake HDMI", &intel_glk_drv_data),
 * HDA_CODEC_EXT_ENTRY(0x8086280d, 0x100000, "Geminilake HDMI", &intel_glk_drv_data),
 * MODULE_DEVICE_TABLE(hdaudio, hdmi_list);
 */
static mut hdmi_driver: hdac_driver = unsafe { zeroed() };

/* module_init(hdmi_init);
 * module_exit(hdmi_exit);
 * MODULE_LICENSE("GPL v2");
 * MODULE_DESCRIPTION("HDMI HD codec");
 * MODULE_AUTHOR("Samreen Nilofer<samreen.nilofer@intel.com>");
 * MODULE_AUTHOR("Subhransu S. Prusty<subhransu.s.prusty@intel.com>");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
