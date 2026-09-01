// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 *  hdmi.rs - routines for HDMI/DisplayPort codecs
 *
 *  Copyright(c) 2008-2010 Intel Corporation
 *  Copyright (c) 2006 ATI Technologies Inc.
 *  Copyright (c) 2008 NVIDIA Corp.  All rights reserved.
 *  Copyright (c) 2008 Wei Ni <wni@nvidia.com>
 *  Copyright (c) 2013 Anssi Hannula <anssi.hannula@iki.fi>
 *
 *  Authors:
 *			Wu Fengguang <wfg@linux.intel.com>
 *
 *  Maintained by:
 *			Wu Fengguang <wfg@linux.intel.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type bool_t = bool;
type hda_nid_t = c_uint;
type u8 = u8;
type u32 = u32;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const ENODEV: c_int = 19;
const EACCES: c_int = 13;

const ELD_MAX_SIZE: usize = 256;
const HDA_MAX_CONNECTIONS: usize = 32;
const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
const SNDRV_PCM_INVALID_DEVICE: c_int = -1;
const HDA_PCM_TYPE_HDMI: c_int = 3;
const SNDRV_CTL_ELEM_TYPE_BYTES: c_int = 4;
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 1 << 0;
const SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint = 1 << 2;
const SNDRV_CTL_ELEM_ACCESS_SKIP_CHECK: c_uint = 1 << 29;
const SNDRV_CTL_ELEM_IFACE_PCM: c_int = 2;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1 << 0;
const SNDRV_CTL_EVENT_MASK_INFO: c_uint = 1 << 1;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 10;
const SND_JACK_AVOUT: c_int = 0x400;
const PM_EVENT_SUSPEND: c_uint = 2;
const RPM_SUSPENDING: c_int = 2;

const AC_WCAP_OUT_AMP: c_uint = 1 << 2;
const AC_WCAP_CONN_LIST: c_uint = 1 << 8;
const AC_WCAP_DIGITAL: c_uint = 1 << 9;
const AC_WCAP_STRIPE: c_uint = 1 << 10;
const AC_WID_AUD_OUT: c_uint = 0;
const AC_WID_PIN: c_uint = 4;
const AC_PINCAP_HDMI: c_uint = 1 << 7;
const AC_PINCAP_DP: c_uint = 1 << 24;
const AC_PINCAP_HBR: c_uint = 1 << 27;
const AC_JACK_PORT_NONE: c_uint = 0;
const AC_FMT_TYPE_NON_PCM: c_int = 1 << 15;
const AC_FMT_CHAN_MASK: c_int = 0x0f;
const AC_VERB_GET_HDMI_DIP_INDEX: c_uint = 0xf2d;
const AC_VERB_SET_HDMI_DIP_INDEX: c_uint = 0x72d;
const AC_VERB_SET_HDMI_DIP_DATA: c_uint = 0x72e;
const AC_VERB_GET_HDMI_DIP_SIZE: c_uint = 0xf2e;
const AC_VERB_SET_HDMI_DIP_XMIT: c_uint = 0x72f;
const AC_VERB_GET_HDMI_DIP_XMIT: c_uint = 0xf2f;
const AC_VERB_GET_HDMI_DIP_DATA: c_uint = 0xf2e;
const AC_VERB_SET_AMP_GAIN_MUTE: c_uint = 0x300;
const AC_VERB_SET_PIN_WIDGET_CONTROL: c_uint = 0x707;
const AC_VERB_GET_PIN_WIDGET_CONTROL: c_uint = 0xf07;
const AC_VERB_SET_CONNECT_SEL: c_uint = 0x701;
const AC_VERB_GET_DIGI_CONVERT_1: c_uint = 0xf0d;
const AC_VERB_SET_DIGI_CONVERT_3: c_uint = 0x73e;
const AC_VERB_SET_STRIPE_CONTROL: c_uint = 0x724;
const AC_VERB_SET_UNSOLICITED_ENABLE: c_uint = 0x708;
const AC_DIPXMIT_BEST: c_uint = 0xc0;
const AC_DIPXMIT_DISABLE: c_uint = 0;
const AC_DIG3_ICT: c_uint = 0xf;
const AC_PINCTL_EPT: c_int = 0x3 << 6;
const AC_PINCTL_EPT_HBR: c_int = 0x3 << 6;
const AC_PINCTL_EPT_NATIVE: c_int = 0;
const AMP_OUT_UNMUTE: c_uint = 0xb000;
const PIN_OUT: c_int = 0x40;
const AC_UNSOL_RES_TAG_SHIFT: c_uint = 26;
const AC_UNSOL_RES_SUBTAG: c_uint = 0x1f << 21;
const AC_UNSOL_RES_SUBTAG_SHIFT: c_uint = 21;
const AC_UNSOL_RES_CP_STATE: c_uint = 1 << 1;
const AC_UNSOL_RES_CP_READY: c_uint = 1 << 0;
const AC_UNSOL_RES_IA: c_uint = 1 << 8;
const AC_UNSOL_RES_PD: c_uint = 1 << 31;
const AC_UNSOL_RES_ELDV: c_uint = 1 << 30;
const AC_UNSOL_RES_DE: c_uint = 0x3f << 15;
const AC_UNSOL_RES_DE_SHIFT: c_uint = 15;
const AC_USRSP_EN: c_uint = 1 << 7;
const IEC958_AES0_NONAUDIO: c_uint = 0x02;

#[repr(C)] pub struct hda_codec { pub spec: *mut hdmi_spec, pub addr: c_int, pub core: hdac_device, pub bus: *mut hda_bus, pub card: *mut snd_card, pub dp_mst: bool, pub relaxed_resume: c_int, pub no_sticky_stream: c_int, pub eld_jack_detect: c_int, pub acomp_requested_resume: c_int, pub spdif_mutex: mutex }
#[repr(C)] pub struct hdac_device { pub afg: hda_nid_t, pub dev: device }
#[repr(C)] pub struct hda_bus { pub shutdown: bool, pub pci: *mut pci_dev, pub core: hdac_bus, pub keep_power: c_int }
#[repr(C)] pub struct hdac_bus { pub dev: *mut device }
#[repr(C)] pub struct device { pub power: dev_pm_info }
#[repr(C)] pub struct dev_pm_info { pub runtime_status: c_int, pub power_state: pm_message_t }
#[repr(C)] pub struct pm_message_t { pub event: c_uint }
#[repr(C)] pub struct pci_dev { pub bus: *mut pci_bus, pub revision: u8 }
#[repr(C)] pub struct pci_bus;
#[repr(C)] pub struct snd_card;
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct snd_kcontrol { pub private_value: isize, pub id: snd_ctl_elem_id }
#[repr(C)] pub struct snd_ctl_elem_id { pub device: c_int }
#[repr(C)] pub struct snd_ctl_elem_info { pub type_: c_int, pub count: c_uint }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_union }
#[repr(C)] pub union snd_ctl_elem_value_union { pub bytes: snd_ctl_elem_value_bytes }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_bytes { pub data: [u8; 512] }
#[repr(C)] pub struct snd_kcontrol_new { pub access: c_uint, pub iface: c_int, pub name: *const c_char, pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>, pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int> }
#[repr(C)] pub struct snd_info_entry { pub private_data: *mut c_void, pub c: snd_info_entry_c, pub mode: c_uint }
#[repr(C)] pub struct snd_info_entry_c { pub text: snd_info_entry_text }
#[repr(C)] pub struct snd_info_entry_text { pub write: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)> }
#[repr(C)] pub struct snd_info_buffer;
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime }
#[repr(C)] pub struct snd_pcm_runtime { pub hw: snd_pcm_hardware, pub channels: c_uint, pub rate: c_uint }
#[repr(C)] pub struct snd_pcm_hardware { pub channels_min: c_uint, pub channels_max: c_uint, pub formats: u64, pub rates: c_uint }
#[repr(C)] pub struct hda_pcm_stream { pub nid: hda_nid_t, pub substreams: c_uint, pub ops: hda_pcm_ops, pub channels_min: c_uint, pub channels_max: c_uint, pub rates: c_uint, pub formats: u64, pub maxbps: c_uint }
#[repr(C)] #[derive(Copy, Clone)] pub struct hda_pcm_ops { pub open: Option<unsafe extern "C" fn(*mut hda_pcm_stream, *mut hda_codec, *mut snd_pcm_substream) -> c_int>, pub close: Option<unsafe extern "C" fn(*mut hda_pcm_stream, *mut hda_codec, *mut snd_pcm_substream) -> c_int>, pub prepare: Option<unsafe extern "C" fn(*mut hda_pcm_stream, *mut hda_codec, c_uint, c_uint, *mut snd_pcm_substream) -> c_int>, pub cleanup: Option<unsafe extern "C" fn(*mut hda_pcm_stream, *mut hda_codec, *mut snd_pcm_substream) -> c_int> }
#[repr(C)] pub struct hda_pcm { pub pcm: *mut snd_pcm, pub pcm_type: c_int, pub own_chmap: bool, pub stream: [hda_pcm_stream; 2], pub device: c_int }
#[repr(C)] pub struct snd_pcm { pub streams: [snd_pcm_str; 2] }
#[repr(C)] pub struct snd_pcm_str { pub substream: *mut snd_pcm_substream }
#[repr(C)] pub struct snd_jack { pub private_data: *mut c_void, pub private_free: Option<unsafe extern "C" fn(*mut snd_jack)> }
#[repr(C)] pub struct hda_jack_callback { pub nid: hda_nid_t, pub dev_id: c_int }
#[repr(C)] pub struct hda_jack_tbl { pub jack_dirty: c_int, pub nid: hda_nid_t, pub dev_id: c_int, pub tag: c_uint }
#[repr(C)] pub struct hda_spdif_out { pub status: c_uint }
#[repr(C)] pub struct snd_pci_quirk { pub subvendor: c_uint, pub subdevice: c_uint, pub value: c_int }
#[repr(C)] pub struct hda_device_id { pub driver_data: usize }
#[repr(C)] pub struct hda_codec_ops { pub probe: Option<unsafe extern "C" fn(*mut hda_codec, *const hda_device_id) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut hda_codec)>, pub init: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>, pub build_pcms: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>, pub build_controls: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>, pub unsol_event: Option<unsafe extern "C" fn(*mut hda_codec, c_uint)>, pub suspend: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>, pub resume: Option<unsafe extern "C" fn(*mut hda_codec) -> c_int> }
#[repr(C)] pub struct hda_codec_driver { pub id: *const hda_device_id, pub ops: *const hda_codec_ops }
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct delayed_work { pub work: work_struct }

#[repr(C)] #[derive(Copy, Clone)] pub struct hdmi_eld_info { pub spk_alloc: c_int, pub conn_type: c_int, pub sad_count: c_int }
#[repr(C)] #[derive(Copy, Clone)] pub struct hdmi_eld { pub monitor_present: bool, pub eld_valid: bool, pub eld_size: c_int, pub eld_buffer: [u8; ELD_MAX_SIZE], pub info: hdmi_eld_info }
#[repr(C)] pub struct hdmi_spec_per_cvt { pub cvt_nid: hda_nid_t, pub channels_min: c_uint, pub channels_max: c_uint, pub rates: c_uint, pub formats: u64, pub maxbps: c_uint, pub assigned: bool, pub silent_stream: bool }
#[repr(C)] pub struct hdmi_pcm { pub pcm: *mut hda_pcm, pub eld_ctl: *mut snd_kcontrol, pub jack: *mut snd_jack }
#[repr(C)] pub struct hdmi_spec_per_pin { pub codec: *mut hda_codec, pub lock: mutex, pub work: delayed_work, pub proc_entry: *mut snd_info_entry, pub pcm: *mut hdmi_pcm, pub pcm_idx: c_int, pub prev_pcm_idx: c_int, pub pin_nid: hda_nid_t, pub pin_nid_idx: c_int, pub dev_id: c_int, pub non_pcm: bool, pub sink_eld: hdmi_eld, pub cvt_nid: hda_nid_t, pub channels: c_int, pub chmap_set: bool, pub chmap: [u8; 8], pub setup: bool, pub mux_idx: c_int, pub num_mux_nids: c_int, pub mux_nids: [hda_nid_t; HDA_MAX_CONNECTIONS], pub silent_stream: bool, pub repoll_count: c_int }
#[repr(C)] pub struct hdac_chmap_ops { pub get_chmap: Option<unsafe extern "C" fn(*mut hdac_device, c_int, *mut u8)>, pub set_chmap: Option<unsafe extern "C" fn(*mut hdac_device, c_int, *mut u8, c_int)>, pub is_pcm_attached: Option<unsafe extern "C" fn(*mut hdac_device, c_int) -> bool>, pub get_spk_alloc: Option<unsafe extern "C" fn(*mut hdac_device, c_int) -> c_int>, pub set_channel_count: Option<unsafe extern "C" fn(*mut hdac_device, hda_nid_t, c_int)> }
#[repr(C)] pub struct hdac_chmap { pub channels_max: c_uint, pub ops: hdac_chmap_ops }
#[repr(C)] #[derive(Copy, Clone)] pub struct hdmi_ops { pub pin_get_eld: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t, c_int, *mut u8, *mut c_int) -> c_int>, pub pin_setup_infoframe: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t, c_int, c_int, c_int, c_int)>, pub pin_hbr_setup: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t, c_int, bool) -> c_int>, pub setup_stream: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t, hda_nid_t, c_int, u32, c_int) -> c_int>, pub pin_cvt_fixup: Option<unsafe extern "C" fn(*mut hda_codec, *mut hdmi_spec_per_pin, hda_nid_t)>, pub prepare: Option<unsafe extern "C" fn(*mut hda_codec, *mut hdmi_spec_per_pin)>, pub silent_stream: Option<unsafe extern "C" fn(*mut hda_codec, *mut hdmi_spec_per_pin, bool)> }
#[repr(C)] pub struct drm_audio_component_audio_ops { pub audio_ptr: *mut c_void, pub pin2port: *mut c_void, pub pin_eld_notify: *mut c_void, pub master_bind: *mut c_void, pub master_unbind: *mut c_void }
#[repr(C)] pub struct drm_audio_component { pub audio_ops: *mut drm_audio_component_audio_ops }
#[repr(C)] pub struct hdmi_spec { pub codec: *mut hda_codec, pub ops: hdmi_ops, pub pins: snd_array, pub cvts: snd_array, pub num_pins: c_int, pub num_nids: c_int, pub num_cvts: c_int, pub num_cvts_alloc: c_int, pub pcm_used: c_int, pub pcm_rec: [hdmi_pcm; 16], pub cvt_nids: [hda_nid_t; 16], pub pcm_lock: mutex, pub bind_lock: mutex, pub pcm_in_use: usize, pub pcm_bitmap: usize, pub chmap: hdac_chmap, pub temp_eld: hdmi_eld, pub dyn_pin_out: bool, pub nv_dp_workaround: bool, pub intel_hsw_fixup: bool, pub force_connect: bool, pub static_pcm_mapping: bool, pub dev_num: c_int, pub silent_stream_type: c_int, pub acomp_registered: bool, pub use_acomp_notifier: bool, pub drm_audio_ops: drm_audio_component_audio_ops, pub port2pin: Option<unsafe extern "C" fn(*mut hda_codec, c_int) -> hda_nid_t> }
#[repr(C)] pub struct snd_array { pub list: *mut c_void, pub used: c_uint, pub alloced: c_uint, pub elem_size: c_uint }

#[repr(C)] pub struct hdmi_audio_infoframe { pub type_: u8, pub ver: u8, pub len: u8, pub checksum: u8, pub CC02_CT47: u8, pub CA: u8, pub bytes_rest: [u8; 25] }
#[repr(C)] pub struct dp_audio_infoframe { pub type_: u8, pub len: u8, pub ver: u8, pub CC02_CT47: u8, pub CA: u8, pub bytes_rest: [u8; 26] }
#[repr(C)] pub union audio_infoframe { pub hdmi: hdmi_audio_infoframe, pub dp: dp_audio_infoframe, pub bytes: [u8; 31] }

extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut hda_codec;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_hda_ctl_add(codec: *mut hda_codec, nid: hda_nid_t, kctl: *mut snd_kcontrol) -> c_int;
    fn snd_hda_codec_read(codec: *mut hda_codec, nid: hda_nid_t, flags: c_int, verb: c_uint, parm: c_uint) -> c_int;
    fn snd_hda_codec_write(codec: *mut hda_codec, nid: hda_nid_t, flags: c_int, verb: c_uint, parm: c_uint) -> c_int;
    fn snd_hda_codec_write_cache(codec: *mut hda_codec, nid: hda_nid_t, flags: c_int, verb: c_uint, parm: c_uint) -> c_int;
    fn snd_hda_set_dev_select(codec: *mut hda_codec, nid: hda_nid_t, dev_id: c_int);
    fn snd_hdmi_get_eld(codec: *mut hda_codec, nid: hda_nid_t, buf: *mut u8, eld_size: *mut c_int) -> c_int;
    fn snd_hdmi_get_eld_size(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
    fn snd_hdmi_print_eld_info(eld: *mut hdmi_eld, buffer: *mut snd_info_buffer, pin_nid: hda_nid_t, dev_id: c_int, cvt_nid: hda_nid_t);
    fn snd_hdmi_write_eld_info(eld: *mut hdmi_eld, buffer: *mut snd_info_buffer);
    fn snd_card_proc_new(card: *mut snd_card, name: *const c_char, entry: *mut *mut snd_info_entry) -> c_int;
    fn snd_info_set_text_ops(entry: *mut snd_info_entry, data: *mut c_void, read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>);
    fn snd_info_free_entry(entry: *mut snd_info_entry);
    fn get_wcaps(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
    fn get_wcaps_channels(wcaps: c_uint) -> c_uint;
    fn get_wcaps_type(wcaps: c_uint) -> c_uint;
    fn snd_hda_query_pin_caps(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
    fn snd_hda_codec_setup_stream(codec: *mut hda_codec, nid: hda_nid_t, stream_tag: u32, channel_id: c_int, format: c_int);
    fn snd_hda_codec_cleanup_stream(codec: *mut hda_codec, nid: hda_nid_t);
    fn snd_hdac_channel_allocation(core: *mut hdac_device, spk_alloc: c_int, channels: c_int, chmap_set: bool, non_pcm: bool, chmap: *mut u8) -> c_int;
    fn snd_hdac_get_active_channels(ca: c_int) -> c_int;
    fn snd_hdac_setup_channel_mapping(chmap: *mut hdac_chmap, pin_nid: hda_nid_t, non_pcm: bool, ca: c_int, channels: c_int, map: *mut u8, set: bool);
    fn snd_hda_jack_tbl_get_from_tag(codec: *mut hda_codec, tag: c_int, dev_id: c_int) -> *mut hda_jack_tbl;
    fn snd_hda_jack_tbl_get_mst(codec: *mut hda_codec, nid: hda_nid_t, dev_id: c_int) -> *mut hda_jack_tbl;
    fn snd_hda_jack_pin_sense(codec: *mut hda_codec, nid: hda_nid_t, dev_id: c_int) -> c_int;
    fn snd_parse_eld(dev: *mut device, info: *mut hdmi_eld_info, buf: *mut u8, size: c_int) -> c_int;
    fn snd_show_eld(dev: *mut device, info: *mut hdmi_eld_info);
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn snd_jack_report(jack: *mut snd_jack, status: c_int);
    fn snd_hdac_acomp_get_eld(core: *mut hdac_device, nid: hda_nid_t, dev_id: c_int, present: *mut bool, buf: *mut u8, max: usize) -> c_int;
    fn snd_hdac_sync_audio_rate(core: *mut hdac_device, nid: hda_nid_t, dev_id: c_int, rate: c_uint);
    fn snd_hdac_get_stream_stripe_ctl(bus: *mut hdac_bus, substream: *mut snd_pcm_substream) -> c_int;
    fn snd_hda_get_sub_nodes(codec: *mut hda_codec, afg: hda_nid_t, start: *mut hda_nid_t) -> c_int;
    fn snd_hda_get_raw_connections(codec: *mut hda_codec, nid: hda_nid_t, list: *mut hda_nid_t, max: c_int) -> c_int;
    fn snd_hda_get_num_devices(codec: *mut hda_codec, nid: hda_nid_t) -> c_int;
    fn snd_hda_codec_get_pincfg(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
    fn get_defcfg_connect(config: c_uint) -> c_uint;
    fn is_jack_detectable(codec: *mut hda_codec, nid: hda_nid_t) -> bool;
    fn snd_hda_query_supported_pcm(codec: *mut hda_codec, nid: hda_nid_t, rates: *mut c_uint, formats: *mut u64, bps: *mut c_uint, maxbps: *mut c_uint) -> c_int;
    fn snd_pci_quirk_lookup(pci: *mut pci_dev, list: *const snd_pci_quirk) -> *const snd_pci_quirk;
    fn snd_hda_spdif_out_of_nid(codec: *mut hda_codec, nid: hda_nid_t) -> *mut hda_spdif_out;
    fn snd_hda_spdif_ctls_assign(codec: *mut hda_codec, idx: c_int, nid: hda_nid_t);
    fn snd_hda_spdif_ctls_unassign(codec: *mut hda_codec, idx: c_int);
    fn snd_hda_create_dig_out_ctls(codec: *mut hda_codec, a: c_int, nid: hda_nid_t, ty: c_int) -> c_int;
    fn snd_hdmi_eld_update_pcm_info(info: *mut hdmi_eld_info, hinfo: *mut hda_pcm_stream);
    fn snd_pcm_hw_constraint_step(runtime: *mut snd_pcm_runtime, a: c_int, param: c_int, step: c_int) -> c_int;
    fn snd_hda_codec_pcm_new(codec: *mut hda_codec, fmt: *const c_char, idx: c_int) -> *mut hda_pcm;
    fn snd_jack_new(card: *mut snd_card, id: *const c_char, ty: c_int, jack: *mut *mut snd_jack, initial_kctl: bool, phantom: bool) -> c_int;
    fn snd_hdac_add_chmap_ctls(pcm: *mut snd_pcm, pcm_idx: c_int, chmap: *mut hdac_chmap) -> c_int;
    fn snd_hdac_register_chmap_ops(core: *mut hdac_device, chmap: *mut hdac_chmap);
    fn snd_hda_jack_detect_enable_callback_mst(codec: *mut hda_codec, nid: hda_nid_t, dev_id: c_int, cb: Option<unsafe extern "C" fn(*mut hda_codec, *mut hda_jack_callback)>);
    fn snd_hda_codec_init(codec: *mut hda_codec);
    fn snd_hda_regmap_sync(codec: *mut hda_codec);
    fn snd_array_init(array: *mut snd_array, elem_size: usize, nums: c_int);
    fn snd_array_new(array: *mut snd_array) -> *mut c_void;
    fn snd_array_free(array: *mut snd_array);
    fn kfree(p: *mut c_void);
    fn kzalloc(size: usize) -> *mut c_void;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_uint) -> bool;
    fn msecs_to_jiffies(ms: c_uint) -> c_uint;
    fn snd_device_free(card: *mut snd_card, device: *mut c_void) -> c_int;
    fn snd_hdac_acomp_exit(core: *mut hdac_device);
    fn snd_hdac_acomp_register_notifier(core: *mut hdac_device, notifier: *mut c_void);
    fn snd_hdac_acomp_init(core: *mut hdac_device, ops: *mut drm_audio_component_audio_ops, match_master: Option<unsafe extern "C" fn(*mut device, c_int, *mut c_void) -> c_int>, extra_size: c_int) -> c_int;
    fn pm_runtime_suspended(dev: *mut device) -> bool;
    fn dev_is_pci(dev: *mut device) -> bool;
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
}

static mut static_hdmi_pcm: bool = false;
static mut enable_acomp: bool = true;
static mut enable_all_pins: bool = false;

unsafe fn get_pin(spec: *mut hdmi_spec, idx: c_int) -> *mut hdmi_spec_per_pin {
    ((*spec).pins.list as *mut hdmi_spec_per_pin).add(idx as usize)
}
unsafe fn get_cvt(spec: *mut hdmi_spec, idx: c_int) -> *mut hdmi_spec_per_cvt {
    ((*spec).cvts.list as *mut hdmi_spec_per_cvt).add(idx as usize)
}
unsafe fn get_hdmi_pcm(spec: *mut hdmi_spec, idx: c_int) -> *mut hdmi_pcm {
    (*spec).pcm_rec.as_mut_ptr().add(idx as usize)
}
unsafe fn get_pcm_rec(spec: *mut hdmi_spec, idx: c_int) -> *mut hda_pcm {
    (*get_hdmi_pcm(spec, idx)).pcm
}
unsafe fn hda_codec_dev(codec: *mut hda_codec) -> *mut device { &mut (*codec).core.dev }
unsafe fn hdac_to_hda_codec(hdac: *mut hdac_device) -> *mut hda_codec { hdac as *mut hda_codec }
unsafe fn codec_has_acomp(codec: *mut hda_codec) -> bool { !(*codec).spec.is_null() && (*(*codec).spec).use_acomp_notifier }
unsafe fn test_bit(bit: c_int, word: *const usize) -> bool { ((*word >> bit) & 1) != 0 }
unsafe fn set_bit(bit: c_int, word: *mut usize) { *word |= 1usize << bit; }
unsafe fn clear_bit(bit: c_int, word: *mut usize) { *word &= !(1usize << bit); }
unsafe fn min(a: c_int, b: c_int) -> c_int { if a < b { a } else { b } }
unsafe fn snd_BUG() {}
unsafe fn snd_BUG_ON(v: bool) -> bool { v }
unsafe fn WARN_ON(v: bool) -> bool { v }

unsafe extern "C" fn snd_hda_hdmi_pin_id_to_pin_index(codec: *mut hda_codec, pin_nid: hda_nid_t, mut dev_id: c_int) -> c_int {
    let spec = (*codec).spec;
    if dev_id == -1 { dev_id = 0; }
    for pin_idx in 0..(*spec).num_pins {
        let per_pin = get_pin(spec, pin_idx);
        if (*per_pin).pin_nid == pin_nid && (*per_pin).dev_id == dev_id {
            return pin_idx;
        }
    }
    -EINVAL
}

unsafe fn pin_id_to_pin_index(codec: *mut hda_codec, nid: hda_nid_t, dev_id: c_int) -> c_int {
    snd_hda_hdmi_pin_id_to_pin_index(codec, nid, dev_id)
}

unsafe extern "C" fn hinfo_to_pcm_index(codec: *mut hda_codec, hinfo: *mut hda_pcm_stream) -> c_int {
    let spec = (*codec).spec;
    for pcm_idx in 0..(*spec).pcm_used {
        if !get_pcm_rec(spec, pcm_idx).is_null() && (*get_pcm_rec(spec, pcm_idx)).stream.as_mut_ptr() == hinfo {
            return pcm_idx;
        }
    }
    -EINVAL
}

unsafe extern "C" fn hinfo_to_pin_index(codec: *mut hda_codec, hinfo: *mut hda_pcm_stream) -> c_int {
    let spec = (*codec).spec;
    for pin_idx in 0..(*spec).num_pins {
        let per_pin = get_pin(spec, pin_idx);
        if !(*per_pin).pcm.is_null() && !(*(*per_pin).pcm).pcm.is_null() &&
           (*(*(*per_pin).pcm).pcm).stream.as_mut_ptr() == hinfo {
            return pin_idx;
        }
    }
    -EINVAL
}

unsafe fn pcm_idx_to_pin(spec: *mut hdmi_spec, pcm_idx: c_int) -> *mut hdmi_spec_per_pin {
    for i in 0..(*spec).num_pins {
        let per_pin = get_pin(spec, i);
        if (*per_pin).pcm_idx == pcm_idx { return per_pin; }
    }
    ptr::null_mut()
}

unsafe extern "C" fn cvt_nid_to_cvt_index(codec: *mut hda_codec, cvt_nid: hda_nid_t) -> c_int {
    let spec = (*codec).spec;
    for cvt_idx in 0..(*spec).num_cvts {
        if (*get_cvt(spec, cvt_idx)).cvt_nid == cvt_nid { return cvt_idx; }
    }
    -EINVAL
}

unsafe extern "C" fn hdmi_eld_ctl_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let spec = (*codec).spec;
    let pcm_idx = (*kcontrol).private_value as c_int;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
    let per_pin = pcm_idx_to_pin(spec, pcm_idx);
    if per_pin.is_null() {
        (*uinfo).count = 0;
        return 0;
    }
    let eld = &mut (*per_pin).sink_eld;
    (*uinfo).count = if eld.eld_valid { eld.eld_size as c_uint } else { 0 };
    0
}

unsafe extern "C" fn hdmi_eld_ctl_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let spec = (*codec).spec;
    let pcm_idx = (*kcontrol).private_value as c_int;
    let per_pin = pcm_idx_to_pin(spec, pcm_idx);
    let data = (*ucontrol).value.bytes.data.as_mut_ptr();
    ptr::write_bytes(data, 0, (*ucontrol).value.bytes.data.len());
    if per_pin.is_null() { return 0; }
    let eld = &mut (*per_pin).sink_eld;
    if eld.eld_size as usize > (*ucontrol).value.bytes.data.len() || eld.eld_size as usize > ELD_MAX_SIZE {
        snd_BUG();
        return -EINVAL;
    }
    if eld.eld_valid {
        ptr::copy_nonoverlapping(eld.eld_buffer.as_ptr(), data, eld.eld_size as usize);
    }
    0
}

static eld_name: &[u8] = b"ELD\0";
static eld_bytes_ctl: snd_kcontrol_new = snd_kcontrol_new {
    access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE | SNDRV_CTL_ELEM_ACCESS_SKIP_CHECK,
    iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: eld_name.as_ptr() as *const c_char,
    info: Some(hdmi_eld_ctl_info),
    get: Some(hdmi_eld_ctl_get),
};

unsafe extern "C" fn hdmi_create_eld_ctl(codec: *mut hda_codec, pcm_idx: c_int, device: c_int) -> c_int {
    let kctl = snd_ctl_new1(&eld_bytes_ctl, codec as *mut c_void);
    if kctl.is_null() { return -ENOMEM; }
    (*kctl).private_value = pcm_idx as isize;
    (*kctl).id.device = device;
    let err = snd_hda_ctl_add(codec, 0, kctl);
    if err < 0 { return err; }
    (*get_hdmi_pcm((*codec).spec, pcm_idx)).eld_ctl = kctl;
    0
}

unsafe fn hdmi_set_dip_index(codec: *mut hda_codec, pin_nid: hda_nid_t, packet_index: c_int, byte_index: c_int) {
    let val = ((packet_index << 5) | (byte_index & 0x1f)) as c_uint;
    snd_hda_codec_write(codec, pin_nid, 0, AC_VERB_SET_HDMI_DIP_INDEX, val);
}

unsafe fn hdmi_write_dip_byte(codec: *mut hda_codec, pin_nid: hda_nid_t, val: u8) {
    snd_hda_codec_write(codec, pin_nid, 0, AC_VERB_SET_HDMI_DIP_DATA, val as c_uint);
}

unsafe fn hdmi_init_pin(codec: *mut hda_codec, pin_nid: hda_nid_t) {
    let spec = (*codec).spec;
    if (get_wcaps(codec, pin_nid) & AC_WCAP_OUT_AMP) != 0 {
        snd_hda_codec_write(codec, pin_nid, 0, AC_VERB_SET_AMP_GAIN_MUTE, AMP_OUT_UNMUTE);
    }
    let pin_out = if (*spec).dyn_pin_out { 0 } else { PIN_OUT };
    snd_hda_codec_write(codec, pin_nid, 0, AC_VERB_SET_PIN_WIDGET_CONTROL, pin_out as c_uint);
}

/* ELD proc files. The CONFIG_SND_PROC_FS disabled inline forms return 0 / do nothing. */
unsafe extern "C" fn print_eld_info(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let per_pin = (*entry).private_data as *mut hdmi_spec_per_pin;
    snd_hdmi_print_eld_info(&mut (*per_pin).sink_eld, buffer, (*per_pin).pin_nid, (*per_pin).dev_id, (*per_pin).cvt_nid);
}
unsafe extern "C" fn write_eld_info(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let per_pin = (*entry).private_data as *mut hdmi_spec_per_pin;
    snd_hdmi_write_eld_info(&mut (*per_pin).sink_eld, buffer);
}
unsafe fn eld_proc_new(per_pin: *mut hdmi_spec_per_pin, index: c_int) -> c_int {
    let mut entry: *mut snd_info_entry = ptr::null_mut();
    let name = [0i8; 32];
    let err = snd_card_proc_new((*(*per_pin).codec).card, name.as_ptr(), &mut entry);
    if err < 0 { return err; }
    snd_info_set_text_ops(entry, per_pin as *mut c_void, Some(print_eld_info));
    (*entry).c.text.write = Some(write_eld_info);
    (*entry).mode |= 0o200;
    (*per_pin).proc_entry = entry;
    let _ = index;
    0
}
unsafe fn eld_proc_free(per_pin: *mut hdmi_spec_per_pin) {
    if !(*(*(*per_pin).codec).bus).shutdown {
        snd_info_free_entry((*per_pin).proc_entry);
        (*per_pin).proc_entry = ptr::null_mut();
    }
}

unsafe fn hdmi_start_infoframe_trans(codec: *mut hda_codec, pin_nid: hda_nid_t) {
    hdmi_set_dip_index(codec, pin_nid, 0, 0);
    snd_hda_codec_write(codec, pin_nid, 0, AC_VERB_SET_HDMI_DIP_XMIT, AC_DIPXMIT_BEST);
}
unsafe fn hdmi_stop_infoframe_trans(codec: *mut hda_codec, pin_nid: hda_nid_t) {
    hdmi_set_dip_index(codec, pin_nid, 0, 0);
    snd_hda_codec_write(codec, pin_nid, 0, AC_VERB_SET_HDMI_DIP_XMIT, AC_DIPXMIT_DISABLE);
}
unsafe fn hdmi_debug_dip_size(codec: *mut hda_codec, pin_nid: hda_nid_t) { let _ = snd_hdmi_get_eld_size(codec, pin_nid); }
unsafe fn hdmi_clear_dip_buffers(_codec: *mut hda_codec, _pin_nid: hda_nid_t) {}

unsafe fn hdmi_checksum_audio_infoframe(hdmi_ai: *mut hdmi_audio_infoframe) {
    let bytes = hdmi_ai as *mut u8;
    let mut sum: u8 = 0;
    (*hdmi_ai).checksum = 0;
    for i in 0..size_of::<hdmi_audio_infoframe>() {
        sum = sum.wrapping_add(*bytes.add(i));
    }
    (*hdmi_ai).checksum = (0u8).wrapping_sub(sum);
}
unsafe fn hdmi_fill_audio_infoframe(codec: *mut hda_codec, pin_nid: hda_nid_t, dip: *mut u8, size: c_int) {
    hdmi_debug_dip_size(codec, pin_nid);
    hdmi_clear_dip_buffers(codec, pin_nid);
    hdmi_set_dip_index(codec, pin_nid, 0, 0);
    for i in 0..size { hdmi_write_dip_byte(codec, pin_nid, *dip.add(i as usize)); }
}
unsafe fn hdmi_infoframe_uptodate(codec: *mut hda_codec, pin_nid: hda_nid_t, dip: *mut u8, size: c_int) -> bool {
    hdmi_set_dip_index(codec, pin_nid, 0, 0);
    if snd_hda_codec_read(codec, pin_nid, 0, AC_VERB_GET_HDMI_DIP_XMIT, 0) as c_uint != AC_DIPXMIT_BEST { return false; }
    for i in 0..size {
        let val = snd_hda_codec_read(codec, pin_nid, 0, AC_VERB_GET_HDMI_DIP_DATA, 0) as u8;
        if val != *dip.add(i as usize) { return false; }
    }
    true
}

unsafe extern "C" fn hdmi_pin_get_eld(codec: *mut hda_codec, nid: hda_nid_t, dev_id: c_int, buf: *mut u8, eld_size: *mut c_int) -> c_int {
    snd_hda_set_dev_select(codec, nid, dev_id);
    snd_hdmi_get_eld(codec, nid, buf, eld_size)
}

unsafe extern "C" fn hdmi_pin_setup_infoframe(codec: *mut hda_codec, pin_nid: hda_nid_t, dev_id: c_int, ca: c_int, active_channels: c_int, conn_type: c_int) {
    let spec = (*codec).spec;
    let mut ai: audio_infoframe = zeroed();
    if conn_type == 0 || (conn_type == 1 && (*spec).nv_dp_workaround) {
        let hdmi_ai = &mut ai.hdmi as *mut hdmi_audio_infoframe;
        if conn_type == 0 {
            (*hdmi_ai).type_ = 0x84; (*hdmi_ai).ver = 0x01; (*hdmi_ai).len = 0x0a;
        } else {
            (*hdmi_ai).type_ = 0x84; (*hdmi_ai).ver = 0x1b; (*hdmi_ai).len = 0x11 << 2;
        }
        (*hdmi_ai).CC02_CT47 = (active_channels - 1) as u8;
        (*hdmi_ai).CA = ca as u8;
        hdmi_checksum_audio_infoframe(hdmi_ai);
    } else if conn_type == 1 {
        let dp_ai = &mut ai.dp as *mut dp_audio_infoframe;
        (*dp_ai).type_ = 0x84; (*dp_ai).len = 0x1b; (*dp_ai).ver = 0x11 << 2;
        (*dp_ai).CC02_CT47 = (active_channels - 1) as u8;
        (*dp_ai).CA = ca as u8;
    } else { return; }
    snd_hda_set_dev_select(codec, pin_nid, dev_id);
    if !hdmi_infoframe_uptodate(codec, pin_nid, ai.bytes.as_mut_ptr(), size_of::<audio_infoframe>() as c_int) {
        hdmi_stop_infoframe_trans(codec, pin_nid);
        hdmi_fill_audio_infoframe(codec, pin_nid, ai.bytes.as_mut_ptr(), size_of::<audio_infoframe>() as c_int);
        hdmi_start_infoframe_trans(codec, pin_nid);
    }
}

unsafe extern "C" fn snd_hda_hdmi_setup_audio_infoframe(codec: *mut hda_codec, per_pin: *mut hdmi_spec_per_pin, non_pcm: bool) {
    let spec = (*codec).spec;
    let channels = (*per_pin).channels;
    if channels == 0 { return; }
    snd_hda_set_dev_select(codec, (*per_pin).pin_nid, (*per_pin).dev_id);
    if (get_wcaps(codec, (*per_pin).pin_nid) & AC_WCAP_OUT_AMP) != 0 {
        snd_hda_codec_write(codec, (*per_pin).pin_nid, 0, AC_VERB_SET_AMP_GAIN_MUTE, AMP_OUT_UNMUTE);
    }
    let eld = &mut (*per_pin).sink_eld;
    let ca = snd_hdac_channel_allocation(&mut (*codec).core, eld.info.spk_alloc, channels, (*per_pin).chmap_set, non_pcm, (*per_pin).chmap.as_mut_ptr());
    let active_channels = snd_hdac_get_active_channels(ca);
    if let Some(set_count) = (*spec).chmap.ops.set_channel_count { set_count(&mut (*codec).core, (*per_pin).cvt_nid, active_channels); }
    snd_hdac_setup_channel_mapping(&mut (*spec).chmap, (*per_pin).pin_nid, non_pcm, ca, channels, (*per_pin).chmap.as_mut_ptr(), (*per_pin).chmap_set);
    if let Some(f) = (*spec).ops.pin_setup_infoframe { f(codec, (*per_pin).pin_nid, (*per_pin).dev_id, ca, active_channels, eld.info.conn_type); }
    (*per_pin).non_pcm = non_pcm;
}

unsafe fn hdmi_present_sense(per_pin: *mut hdmi_spec_per_pin, repoll: c_int);

unsafe extern "C" fn snd_hda_hdmi_check_presence_and_report(codec: *mut hda_codec, nid: hda_nid_t, dev_id: c_int) {
    let spec = (*codec).spec;
    let pin_idx = pin_id_to_pin_index(codec, nid, dev_id);
    if pin_idx < 0 { return; }
    hdmi_present_sense(get_pin(spec, pin_idx), 1);
}
unsafe extern "C" fn jack_callback(codec: *mut hda_codec, jack: *mut hda_jack_callback) {
    if codec_has_acomp(codec) { return; }
    snd_hda_hdmi_check_presence_and_report(codec, (*jack).nid, (*jack).dev_id);
}
unsafe extern "C" fn hdmi_intrinsic_event(codec: *mut hda_codec, res: c_uint, jack: *mut hda_jack_tbl) {
    (*jack).jack_dirty = 1;
    snd_hda_hdmi_check_presence_and_report(codec, (*jack).nid, (*jack).dev_id);
    let _ = res;
}
unsafe fn hdmi_non_intrinsic_event(_codec: *mut hda_codec, res: c_uint) {
    let _tag = res >> AC_UNSOL_RES_TAG_SHIFT;
    let _subtag = (res & AC_UNSOL_RES_SUBTAG) >> AC_UNSOL_RES_SUBTAG_SHIFT;
    let cp_state = (res & AC_UNSOL_RES_CP_STATE) != 0;
    let cp_ready = (res & AC_UNSOL_RES_CP_READY) != 0;
    if cp_state {}
    if cp_ready {}
}
unsafe extern "C" fn snd_hda_hdmi_generic_unsol_event(codec: *mut hda_codec, res: c_uint) {
    if codec_has_acomp(codec) { return; }
    let tag = (res >> AC_UNSOL_RES_TAG_SHIFT) as c_int;
    let subtag = ((res & AC_UNSOL_RES_SUBTAG) >> AC_UNSOL_RES_SUBTAG_SHIFT) as c_int;
    let jack = if (*codec).dp_mst {
        snd_hda_jack_tbl_get_from_tag(codec, tag, ((res & AC_UNSOL_RES_DE) >> AC_UNSOL_RES_DE_SHIFT) as c_int)
    } else {
        snd_hda_jack_tbl_get_from_tag(codec, tag, 0)
    };
    if jack.is_null() { return; }
    if subtag == 0 { hdmi_intrinsic_event(codec, res, jack); } else { hdmi_non_intrinsic_event(codec, res); }
}

fn is_hbr_format(format: c_int) -> bool { (format & AC_FMT_TYPE_NON_PCM) != 0 && (format & AC_FMT_CHAN_MASK) == 7 }

unsafe extern "C" fn hdmi_pin_hbr_setup(codec: *mut hda_codec, pin_nid: hda_nid_t, dev_id: c_int, hbr: bool) -> c_int {
    if (snd_hda_query_pin_caps(codec, pin_nid) & AC_PINCAP_HBR) != 0 {
        snd_hda_set_dev_select(codec, pin_nid, dev_id);
        let pinctl = snd_hda_codec_read(codec, pin_nid, 0, AC_VERB_GET_PIN_WIDGET_CONTROL, 0);
        if pinctl < 0 { return if hbr { -EINVAL } else { 0 }; }
        let mut new_pinctl = pinctl & !AC_PINCTL_EPT;
        if hbr { new_pinctl |= AC_PINCTL_EPT_HBR; } else { new_pinctl |= AC_PINCTL_EPT_NATIVE; }
        if pinctl != new_pinctl { snd_hda_codec_write(codec, pin_nid, 0, AC_VERB_SET_PIN_WIDGET_CONTROL, new_pinctl as c_uint); }
    } else if hbr { return -EINVAL; }
    0
}

unsafe extern "C" fn snd_hda_hdmi_setup_stream(codec: *mut hda_codec, cvt_nid: hda_nid_t, pin_nid: hda_nid_t, dev_id: c_int, stream_tag: u32, format: c_int) -> c_int {
    let spec = (*codec).spec;
    let err = (*spec).ops.pin_hbr_setup.unwrap()(codec, pin_nid, dev_id, is_hbr_format(format));
    if err != 0 { return err; }
    if (*spec).intel_hsw_fixup {
        let mut param = snd_hda_codec_read(codec, cvt_nid, 0, AC_VERB_GET_DIGI_CONVERT_1, 0) as c_uint;
        param = (param >> 16) & !AC_DIG3_ICT;
        if is_hbr_format(format) { param |= 0x1; }
        snd_hda_codec_write(codec, cvt_nid, 0, AC_VERB_SET_DIGI_CONVERT_3, param);
    }
    snd_hda_codec_setup_stream(codec, cvt_nid, stream_tag, 0, format);
    0
}

unsafe fn hdmi_choose_cvt(codec: *mut hda_codec, pin_idx: c_int, cvt_id: *mut c_int, silent: bool) -> c_int {
    let spec = (*codec).spec;
    let per_pin = if pin_idx < 0 { ptr::null_mut() } else { get_pin(spec, pin_idx) };
    if !per_pin.is_null() && (*per_pin).silent_stream {
        let cvt_idx = cvt_nid_to_cvt_index(codec, (*per_pin).cvt_nid);
        let per_cvt = get_cvt(spec, cvt_idx);
        if (*per_cvt).assigned && !silent { return -EBUSY; }
        if !cvt_id.is_null() { *cvt_id = cvt_idx; }
        return 0;
    }
    let mut cvt_idx = 0;
    let mut mux_idx = 0;
    while cvt_idx < (*spec).num_cvts {
        let per_cvt = get_cvt(spec, cvt_idx);
        if (*per_cvt).assigned || (*per_cvt).silent_stream { cvt_idx += 1; continue; }
        if per_pin.is_null() { break; }
        mux_idx = 0;
        while mux_idx < (*per_pin).num_mux_nids {
            if (*per_pin).mux_nids[mux_idx as usize] == (*per_cvt).cvt_nid { break; }
            mux_idx += 1;
        }
        if mux_idx == (*per_pin).num_mux_nids { cvt_idx += 1; continue; }
        break;
    }
    if cvt_idx == (*spec).num_cvts { return -EBUSY; }
    if !per_pin.is_null() { (*per_pin).mux_idx = mux_idx; }
    if !cvt_id.is_null() { *cvt_id = cvt_idx; }
    0
}

unsafe fn pin_cvt_fixup(codec: *mut hda_codec, per_pin: *mut hdmi_spec_per_pin, cvt_nid: hda_nid_t) {
    let spec = (*codec).spec;
    if let Some(f) = (*spec).ops.pin_cvt_fixup { f(codec, per_pin, cvt_nid); }
}

unsafe extern "C" fn hdmi_pcm_open_no_pin(hinfo: *mut hda_pcm_stream, codec: *mut hda_codec, substream: *mut snd_pcm_substream) -> c_int {
    let spec = (*codec).spec;
    let pcm_idx = hinfo_to_pcm_index(codec, hinfo);
    if pcm_idx < 0 { return -EINVAL; }
    let mut cvt_idx = 0;
    let err = hdmi_choose_cvt(codec, -1, &mut cvt_idx, false);
    if err != 0 { return err; }
    let per_cvt = get_cvt(spec, cvt_idx);
    (*per_cvt).assigned = true;
    (*hinfo).nid = (*per_cvt).cvt_nid;
    pin_cvt_fixup(codec, ptr::null_mut(), (*per_cvt).cvt_nid);
    set_bit(pcm_idx, &mut (*spec).pcm_in_use);
    (*hinfo).channels_min = (*per_cvt).channels_min; (*hinfo).channels_max = (*per_cvt).channels_max;
    (*hinfo).rates = (*per_cvt).rates; (*hinfo).formats = (*per_cvt).formats; (*hinfo).maxbps = (*per_cvt).maxbps;
    (*(*substream).runtime).hw.channels_min = (*hinfo).channels_min;
    (*(*substream).runtime).hw.channels_max = (*hinfo).channels_max;
    (*(*substream).runtime).hw.formats = (*hinfo).formats;
    (*(*substream).runtime).hw.rates = (*hinfo).rates;
    snd_pcm_hw_constraint_step((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, 2);
    0
}

unsafe extern "C" fn hdmi_pcm_open(hinfo: *mut hda_pcm_stream, codec: *mut hda_codec, substream: *mut snd_pcm_substream) -> c_int {
    let spec = (*codec).spec;
    let pcm_idx = hinfo_to_pcm_index(codec, hinfo);
    if pcm_idx < 0 { return -EINVAL; }
    let pin_idx = hinfo_to_pin_index(codec, hinfo);
    if pin_idx < 0 { return hdmi_pcm_open_no_pin(hinfo, codec, substream); }
    let mut cvt_idx = 0;
    let err = hdmi_choose_cvt(codec, pin_idx, &mut cvt_idx, false);
    if err < 0 { return err; }
    let per_cvt = get_cvt(spec, cvt_idx);
    (*per_cvt).assigned = true;
    set_bit(pcm_idx, &mut (*spec).pcm_in_use);
    let per_pin = get_pin(spec, pin_idx);
    (*per_pin).cvt_nid = (*per_cvt).cvt_nid;
    (*hinfo).nid = (*per_cvt).cvt_nid;
    snd_hda_set_dev_select(codec, (*per_pin).pin_nid, (*per_pin).dev_id);
    snd_hda_codec_write_cache(codec, (*per_pin).pin_nid, 0, AC_VERB_SET_CONNECT_SEL, (*per_pin).mux_idx as c_uint);
    pin_cvt_fixup(codec, per_pin, 0);
    snd_hda_spdif_ctls_assign(codec, pcm_idx, (*per_cvt).cvt_nid);
    (*hinfo).channels_min = (*per_cvt).channels_min; (*hinfo).channels_max = (*per_cvt).channels_max;
    (*hinfo).rates = (*per_cvt).rates; (*hinfo).formats = (*per_cvt).formats; (*hinfo).maxbps = (*per_cvt).maxbps;
    let eld = &mut (*per_pin).sink_eld;
    if !static_hdmi_pcm && eld.eld_valid {
        snd_hdmi_eld_update_pcm_info(&mut eld.info, hinfo);
        if (*hinfo).channels_min > (*hinfo).channels_max || (*hinfo).rates == 0 || (*hinfo).formats == 0 {
            (*per_cvt).assigned = false; (*hinfo).nid = 0; snd_hda_spdif_ctls_unassign(codec, pcm_idx); return -ENODEV;
        }
    }
    (*(*substream).runtime).hw.channels_min = (*hinfo).channels_min;
    (*(*substream).runtime).hw.channels_max = (*hinfo).channels_max;
    (*(*substream).runtime).hw.formats = (*hinfo).formats;
    (*(*substream).runtime).hw.rates = (*hinfo).rates;
    snd_pcm_hw_constraint_step((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, 2);
    0
}

unsafe fn hdmi_read_pin_conn(codec: *mut hda_codec, pin_idx: c_int) -> c_int {
    let spec = (*codec).spec;
    let per_pin = get_pin(spec, pin_idx);
    let pin_nid = (*per_pin).pin_nid;
    if (get_wcaps(codec, pin_nid) & AC_WCAP_CONN_LIST) == 0 { return -EINVAL; }
    snd_hda_set_dev_select(codec, pin_nid, (*per_pin).dev_id);
    let conns = if (*spec).intel_hsw_fixup {
        for i in 0..(*spec).num_cvts { (*per_pin).mux_nids[i as usize] = (*spec).cvt_nids[i as usize]; }
        (*spec).num_cvts
    } else {
        snd_hda_get_raw_connections(codec, pin_nid, (*per_pin).mux_nids.as_mut_ptr(), HDA_MAX_CONNECTIONS as c_int)
    };
    (*per_pin).num_mux_nids = conns;
    0
}

unsafe fn hdmi_find_pcm_slot(spec: *mut hdmi_spec, _per_pin: *mut hdmi_spec_per_pin) -> c_int {
    for i in 0..(*spec).pcm_used {
        if !test_bit(i, &(*spec).pcm_bitmap) { return i; }
    }
    -EBUSY
}
unsafe fn hdmi_attach_hda_pcm(spec: *mut hdmi_spec, per_pin: *mut hdmi_spec_per_pin) {
    if !(*per_pin).pcm.is_null() { return; }
    let mut idx = (*per_pin).prev_pcm_idx;
    if idx >= 0 {
        if !test_bit(idx, &(*spec).pcm_bitmap) { } else { (*per_pin).prev_pcm_idx = -1; idx = hdmi_find_pcm_slot(spec, per_pin); }
    } else { idx = hdmi_find_pcm_slot(spec, per_pin); }
    if idx == -EBUSY { return; }
    (*per_pin).pcm_idx = idx; (*per_pin).pcm = get_hdmi_pcm(spec, idx); set_bit(idx, &mut (*spec).pcm_bitmap);
}
unsafe fn hdmi_detach_hda_pcm(spec: *mut hdmi_spec, per_pin: *mut hdmi_spec_per_pin) {
    if (*per_pin).pcm.is_null() { return; }
    let idx = (*per_pin).pcm_idx;
    (*per_pin).pcm_idx = -1; (*per_pin).prev_pcm_idx = idx; (*per_pin).pcm = ptr::null_mut();
    if idx >= 0 && idx < (*spec).pcm_used { clear_bit(idx, &mut (*spec).pcm_bitmap); }
}
unsafe fn hdmi_get_pin_cvt_mux(_spec: *mut hdmi_spec, per_pin: *mut hdmi_spec_per_pin, cvt_nid: hda_nid_t) -> c_int {
    let mut mux_idx = 0;
    while mux_idx < (*per_pin).num_mux_nids {
        if (*per_pin).mux_nids[mux_idx as usize] == cvt_nid { break; }
        mux_idx += 1;
    }
    mux_idx
}

unsafe fn check_non_pcm_per_cvt(codec: *mut hda_codec, cvt_nid: hda_nid_t) -> bool;

unsafe fn hdmi_pcm_setup_pin(spec: *mut hdmi_spec, per_pin: *mut hdmi_spec_per_pin) {
    let codec = (*per_pin).codec;
    if (*per_pin).pcm_idx < 0 || (*per_pin).pcm_idx >= (*spec).pcm_used { return; }
    let pcm = get_pcm_rec(spec, (*per_pin).pcm_idx);
    if pcm.is_null() || (*pcm).pcm.is_null() || !test_bit((*per_pin).pcm_idx, &(*spec).pcm_in_use) { return; }
    let hinfo = (*pcm).stream.as_mut_ptr().add(SNDRV_PCM_STREAM_PLAYBACK);
    let substream = (*(*pcm).pcm).streams[0].substream;
    (*per_pin).cvt_nid = (*hinfo).nid;
    let mux_idx = hdmi_get_pin_cvt_mux(spec, per_pin, (*hinfo).nid);
    if mux_idx < (*per_pin).num_mux_nids {
        snd_hda_set_dev_select(codec, (*per_pin).pin_nid, (*per_pin).dev_id);
        snd_hda_codec_write_cache(codec, (*per_pin).pin_nid, 0, AC_VERB_SET_CONNECT_SEL, mux_idx as c_uint);
    }
    snd_hda_spdif_ctls_assign(codec, (*per_pin).pcm_idx, (*hinfo).nid);
    let non_pcm = check_non_pcm_per_cvt(codec, (*hinfo).nid);
    if !(*substream).runtime.is_null() { (*per_pin).channels = (*(*substream).runtime).channels as c_int; }
    (*per_pin).setup = true; (*per_pin).mux_idx = mux_idx;
    snd_hda_hdmi_setup_audio_infoframe(codec, per_pin, non_pcm);
}
unsafe fn hdmi_pcm_reset_pin(spec: *mut hdmi_spec, per_pin: *mut hdmi_spec_per_pin) {
    if (*per_pin).pcm_idx >= 0 && (*per_pin).pcm_idx < (*spec).pcm_used { snd_hda_spdif_ctls_unassign((*per_pin).codec, (*per_pin).pcm_idx); }
    (*per_pin).chmap_set = false; ptr::write_bytes((*per_pin).chmap.as_mut_ptr(), 0, (*per_pin).chmap.len());
    (*per_pin).setup = false; (*per_pin).channels = 0;
}
unsafe fn pin_idx_to_pcm_jack(codec: *mut hda_codec, per_pin: *mut hdmi_spec_per_pin) -> *mut snd_jack {
    let spec = (*codec).spec;
    if (*per_pin).pcm_idx >= 0 { (*spec).pcm_rec[(*per_pin).pcm_idx as usize].jack } else { ptr::null_mut() }
}

unsafe fn update_eld(codec: *mut hda_codec, per_pin: *mut hdmi_spec_per_pin, eld: *mut hdmi_eld, repoll: c_int) {
    let pin_eld = &mut (*per_pin).sink_eld as *mut hdmi_eld;
    let spec = (*codec).spec;
    let old_eld_valid = (*pin_eld).eld_valid;
    if (*eld).eld_valid {
        if (*eld).eld_size <= 0 || snd_parse_eld(hda_codec_dev(codec), &mut (*eld).info, (*eld).eld_buffer.as_mut_ptr(), (*eld).eld_size) < 0 {
            (*eld).eld_valid = false;
            if repoll != 0 { schedule_delayed_work(&mut (*per_pin).work, msecs_to_jiffies(300)); return; }
        }
    }
    if !(*eld).eld_valid || (*eld).eld_size <= 0 || (*eld).info.sad_count <= 0 { (*eld).eld_valid = false; (*eld).eld_size = 0; }
    let mut pcm_idx = (*per_pin).pcm_idx;
    let mut pcm_jack = pin_idx_to_pcm_jack(codec, per_pin);
    if !(*spec).static_pcm_mapping {
        if (*eld).eld_valid { hdmi_attach_hda_pcm(spec, per_pin); hdmi_pcm_setup_pin(spec, per_pin); }
        else { hdmi_pcm_reset_pin(spec, per_pin); hdmi_detach_hda_pcm(spec, per_pin); }
    }
    if pcm_idx == -1 { pcm_idx = (*per_pin).pcm_idx; }
    if pcm_jack.is_null() { pcm_jack = pin_idx_to_pcm_jack(codec, per_pin); }
    if (*eld).eld_valid { snd_show_eld(hda_codec_dev(codec), &mut (*eld).info); }
    let mut eld_changed = (*pin_eld).eld_valid != (*eld).eld_valid || (*pin_eld).monitor_present != (*eld).monitor_present;
    if !eld_changed && (*eld).eld_valid && (*pin_eld).eld_valid {
        if (*pin_eld).eld_size != (*eld).eld_size ||
           core::slice::from_raw_parts((*pin_eld).eld_buffer.as_ptr(), (*eld).eld_size as usize) != core::slice::from_raw_parts((*eld).eld_buffer.as_ptr(), (*eld).eld_size as usize) {
            eld_changed = true;
        }
    }
    if eld_changed {
        (*pin_eld).monitor_present = (*eld).monitor_present; (*pin_eld).eld_valid = (*eld).eld_valid; (*pin_eld).eld_size = (*eld).eld_size;
        if (*eld).eld_valid { ptr::copy_nonoverlapping((*eld).eld_buffer.as_ptr(), (*pin_eld).eld_buffer.as_mut_ptr(), (*eld).eld_size as usize); }
        (*pin_eld).info = (*eld).info;
    }
    if (*eld).eld_valid && !old_eld_valid && (*per_pin).setup { pin_cvt_fixup(codec, per_pin, 0); snd_hda_hdmi_setup_audio_infoframe(codec, per_pin, (*per_pin).non_pcm); }
    if eld_changed && pcm_idx >= 0 { snd_ctl_notify((*codec).card, SNDRV_CTL_EVENT_MASK_VALUE | SNDRV_CTL_EVENT_MASK_INFO, &mut (*(*get_hdmi_pcm(spec, pcm_idx)).eld_ctl).id); }
    if eld_changed && !pcm_jack.is_null() { snd_jack_report(pcm_jack, if (*eld).monitor_present && (*eld).eld_valid { SND_JACK_AVOUT } else { 0 }); }
}

unsafe fn hdmi_present_sense_via_verbs(per_pin: *mut hdmi_spec_per_pin, repoll: c_int) {
    let codec = (*per_pin).codec;
    let spec = (*codec).spec;
    let eld = &mut (*spec).temp_eld as *mut hdmi_eld;
    let present = snd_hda_jack_pin_sense(codec, (*per_pin).pin_nid, (*per_pin).dev_id);
    (*eld).monitor_present = (present as c_uint & (1 << 31)) != 0;
    (*eld).eld_valid = if (*eld).monitor_present { (present as c_uint & (1 << 30)) != 0 } else { false };
    if (*eld).eld_valid {
        if (*spec).ops.pin_get_eld.unwrap()(codec, (*per_pin).pin_nid, (*per_pin).dev_id, (*eld).eld_buffer.as_mut_ptr(), &mut (*eld).eld_size) < 0 {
            (*eld).eld_valid = false;
        }
    }
    update_eld(codec, per_pin, eld, repoll);
}
unsafe fn silent_stream_enable(codec: *mut hda_codec, per_pin: *mut hdmi_spec_per_pin) {
    let spec = (*codec).spec;
    if (*per_pin).setup { return; }
    let pin_idx = pin_id_to_pin_index(codec, (*per_pin).pin_nid, (*per_pin).dev_id);
    let mut cvt_idx = 0;
    if hdmi_choose_cvt(codec, pin_idx, &mut cvt_idx, true) != 0 { return; }
    let per_cvt = get_cvt(spec, cvt_idx);
    (*per_cvt).silent_stream = true; (*per_pin).cvt_nid = (*per_cvt).cvt_nid; (*per_pin).silent_stream = true;
    snd_hda_set_dev_select(codec, (*per_pin).pin_nid, (*per_pin).dev_id);
    snd_hda_codec_write_cache(codec, (*per_pin).pin_nid, 0, AC_VERB_SET_CONNECT_SEL, (*per_pin).mux_idx as c_uint);
    pin_cvt_fixup(codec, per_pin, 0);
    if let Some(f) = (*spec).ops.silent_stream { f(codec, per_pin, true); }
}
unsafe fn silent_stream_disable(codec: *mut hda_codec, per_pin: *mut hdmi_spec_per_pin) {
    let spec = (*codec).spec;
    if !(*per_pin).silent_stream { return; }
    let cvt_idx = cvt_nid_to_cvt_index(codec, (*per_pin).cvt_nid);
    if cvt_idx >= 0 && cvt_idx < (*spec).num_cvts { (*get_cvt(spec, cvt_idx)).silent_stream = false; }
    if let Some(f) = (*spec).ops.silent_stream { f(codec, per_pin, false); }
    (*per_pin).cvt_nid = 0; (*per_pin).silent_stream = false;
}
unsafe fn sync_eld_via_acomp(codec: *mut hda_codec, per_pin: *mut hdmi_spec_per_pin) {
    let spec = (*codec).spec;
    let eld = &mut (*spec).temp_eld as *mut hdmi_eld;
    (*eld).monitor_present = false;
    let monitor_prev = (*per_pin).sink_eld.monitor_present;
    (*eld).eld_size = snd_hdac_acomp_get_eld(&mut (*codec).core, (*per_pin).pin_nid, (*per_pin).dev_id, &mut (*eld).monitor_present, (*eld).eld_buffer.as_mut_ptr(), ELD_MAX_SIZE);
    (*eld).eld_valid = (*eld).eld_size > 0;
    update_eld(codec, per_pin, eld, 0);
    let monitor_next = (*per_pin).sink_eld.monitor_present;
    if (*spec).silent_stream_type != 0 {
        if !monitor_prev && monitor_next { silent_stream_enable(codec, per_pin); }
        else if monitor_prev && !monitor_next { silent_stream_disable(codec, per_pin); }
    }
}
unsafe fn hdmi_present_sense(per_pin: *mut hdmi_spec_per_pin, repoll: c_int) {
    let codec = (*per_pin).codec;
    if !codec_has_acomp(codec) { hdmi_present_sense_via_verbs(per_pin, repoll); } else { sync_eld_via_acomp(codec, per_pin); }
}
unsafe extern "C" fn hdmi_repoll_eld(work: *mut work_struct) {
    let per_pin = work as *mut hdmi_spec_per_pin;
    let codec = (*per_pin).codec;
    let jack = snd_hda_jack_tbl_get_mst(codec, (*per_pin).pin_nid, (*per_pin).dev_id);
    if !jack.is_null() { (*jack).jack_dirty = 1; }
    if { let old = (*per_pin).repoll_count; (*per_pin).repoll_count += 1; old } > 6 { (*per_pin).repoll_count = 0; }
    hdmi_present_sense(per_pin, (*per_pin).repoll_count);
}

unsafe fn hdmi_add_pin(codec: *mut hda_codec, pin_nid: hda_nid_t) -> c_int {
    let spec = (*codec).spec;
    let caps = snd_hda_query_pin_caps(codec, pin_nid);
    if (caps & (AC_PINCAP_HDMI | AC_PINCAP_DP)) == 0 { return 0; }
    let config = snd_hda_codec_get_pincfg(codec, pin_nid);
    if get_defcfg_connect(config) == AC_JACK_PORT_NONE && !(*spec).force_connect { return 0; }
    let dev_num = if (*spec).intel_hsw_fixup { (*spec).dev_num } else if (*codec).dp_mst {
        let n = snd_hda_get_num_devices(codec, pin_nid) + 1;
        if (*spec).dev_num < n { (*spec).dev_num = n; }
        n
    } else { (*spec).dev_num = 1; 1 };
    for i in 0..dev_num {
        let pin_idx = (*spec).num_pins;
        let per_pin = snd_array_new(&mut (*spec).pins) as *mut hdmi_spec_per_pin;
        if per_pin.is_null() { return -ENOMEM; }
        ptr::write_bytes(per_pin, 0, 1);
        (*per_pin).pcm = ptr::null_mut(); (*per_pin).pcm_idx = -1; (*per_pin).prev_pcm_idx = -1;
        (*per_pin).pin_nid = pin_nid; (*per_pin).pin_nid_idx = (*spec).num_nids; (*per_pin).dev_id = i; (*per_pin).non_pcm = false;
        snd_hda_set_dev_select(codec, pin_nid, i);
        let err = hdmi_read_pin_conn(codec, pin_idx);
        if err < 0 { return err; }
        let _ = is_jack_detectable(codec, pin_nid);
        (*spec).num_pins += 1;
    }
    (*spec).num_nids += 1;
    0
}
unsafe fn hdmi_add_cvt(codec: *mut hda_codec, cvt_nid: hda_nid_t) -> c_int {
    let spec = (*codec).spec;
    let mut chans = get_wcaps(codec, cvt_nid);
    chans = get_wcaps_channels(chans);
    let per_cvt = snd_array_new(&mut (*spec).cvts) as *mut hdmi_spec_per_cvt;
    if per_cvt.is_null() { return -ENOMEM; }
    ptr::write_bytes(per_cvt, 0, 1);
    (*per_cvt).cvt_nid = cvt_nid; (*per_cvt).channels_min = 2;
    if chans <= 16 { (*per_cvt).channels_max = chans; if chans > (*spec).chmap.channels_max { (*spec).chmap.channels_max = chans; } }
    let err = snd_hda_query_supported_pcm(codec, cvt_nid, &mut (*per_cvt).rates, &mut (*per_cvt).formats, ptr::null_mut(), &mut (*per_cvt).maxbps);
    if err < 0 { return err; }
    if (*spec).num_cvts < (*spec).cvt_nids.len() as c_int { (*spec).cvt_nids[(*spec).num_cvts as usize] = cvt_nid; }
    (*spec).num_cvts += 1;
    0
}

static force_connect_list: [snd_pci_quirk; 17] = [
    snd_pci_quirk { subvendor: 0x103c, subdevice: 0x83e2, value: 1 },
    snd_pci_quirk { subvendor: 0x103c, subdevice: 0x83ef, value: 1 },
    snd_pci_quirk { subvendor: 0x103c, subdevice: 0x845a, value: 1 },
    snd_pci_quirk { subvendor: 0x103c, subdevice: 0x8595, value: 1 },
    snd_pci_quirk { subvendor: 0x103c, subdevice: 0x83f3, value: 1 },
    snd_pci_quirk { subvendor: 0x103c, subdevice: 0x870f, value: 1 },
    snd_pci_quirk { subvendor: 0x103c, subdevice: 0x871a, value: 1 },
    snd_pci_quirk { subvendor: 0x103c, subdevice: 0x8711, value: 1 },
    snd_pci_quirk { subvendor: 0x103c, subdevice: 0x8715, value: 1 },
    snd_pci_quirk { subvendor: 0x1043, subdevice: 0x86ae, value: 1 },
    snd_pci_quirk { subvendor: 0x1043, subdevice: 0x86c7, value: 1 },
    snd_pci_quirk { subvendor: 0x1462, subdevice: 0xec94, value: 1 },
    snd_pci_quirk { subvendor: 0x1558, subdevice: 0x14a1, value: 1 },
    snd_pci_quirk { subvendor: 0x8086, subdevice: 0x2060, value: 1 },
    snd_pci_quirk { subvendor: 0x8086, subdevice: 0x2081, value: 1 },
    snd_pci_quirk { subvendor: 0, subdevice: 0, value: 0 },
    snd_pci_quirk { subvendor: 0, subdevice: 0, value: 0 },
];

unsafe extern "C" fn snd_hda_hdmi_parse_codec(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec;
    let mut start_nid: hda_nid_t = 0;
    let nodes = snd_hda_get_sub_nodes(codec, (*codec).core.afg, &mut start_nid);
    if start_nid == 0 || nodes < 0 { return -EINVAL; }
    if enable_all_pins { (*spec).force_connect = true; }
    let q = snd_pci_quirk_lookup((*(*codec).bus).pci, force_connect_list.as_ptr());
    if !q.is_null() && (*q).value != 0 { (*spec).force_connect = true; }
    for i in 0..nodes {
        let nid = start_nid + i as hda_nid_t;
        let caps = get_wcaps(codec, nid);
        if (caps & AC_WCAP_DIGITAL) == 0 { continue; }
        if get_wcaps_type(caps) == AC_WID_AUD_OUT { hdmi_add_cvt(codec, nid); }
    }
    for i in 0..nodes {
        let nid = start_nid + i as hda_nid_t;
        let caps = get_wcaps(codec, nid);
        if (caps & AC_WCAP_DIGITAL) == 0 { continue; }
        if get_wcaps_type(caps) == AC_WID_PIN { hdmi_add_pin(codec, nid); }
    }
    0
}

unsafe fn check_non_pcm_per_cvt(codec: *mut hda_codec, cvt_nid: hda_nid_t) -> bool {
    let spdif = snd_hda_spdif_out_of_nid(codec, cvt_nid);
    if WARN_ON(spdif.is_null()) { return true; }
    ((*spdif).status & IEC958_AES0_NONAUDIO) != 0
}

unsafe extern "C" fn snd_hda_hdmi_generic_pcm_prepare(hinfo: *mut hda_pcm_stream, codec: *mut hda_codec, stream_tag: c_uint, format: c_uint, substream: *mut snd_pcm_substream) -> c_int {
    let cvt_nid = (*hinfo).nid;
    let spec = (*codec).spec;
    let pin_idx = hinfo_to_pin_index(codec, hinfo);
    if pin_idx < 0 {
        pin_cvt_fixup(codec, ptr::null_mut(), cvt_nid);
        snd_hda_codec_setup_stream(codec, cvt_nid, stream_tag, 0, format as c_int);
        return 0;
    }
    let per_pin = get_pin(spec, pin_idx);
    pin_cvt_fixup(codec, per_pin, 0);
    if codec_has_acomp(codec) { snd_hdac_sync_audio_rate(&mut (*codec).core, (*per_pin).pin_nid, (*per_pin).dev_id, (*(*substream).runtime).rate); }
    let non_pcm = check_non_pcm_per_cvt(codec, cvt_nid);
    (*per_pin).channels = (*(*substream).runtime).channels as c_int;
    (*per_pin).setup = true;
    if let Some(f) = (*spec).ops.prepare { f(codec, per_pin); }
    if (get_wcaps(codec, cvt_nid) & AC_WCAP_STRIPE) != 0 {
        let stripe = snd_hdac_get_stream_stripe_ctl(&mut (*(*codec).bus).core, substream);
        snd_hda_codec_write(codec, cvt_nid, 0, AC_VERB_SET_STRIPE_CONTROL, stripe as c_uint);
    }
    snd_hda_hdmi_setup_audio_infoframe(codec, per_pin, non_pcm);
    if (*spec).dyn_pin_out {
        snd_hda_set_dev_select(codec, (*per_pin).pin_nid, (*per_pin).dev_id);
        let pinctl = snd_hda_codec_read(codec, (*per_pin).pin_nid, 0, AC_VERB_GET_PIN_WIDGET_CONTROL, 0);
        snd_hda_codec_write(codec, (*per_pin).pin_nid, 0, AC_VERB_SET_PIN_WIDGET_CONTROL, (pinctl | PIN_OUT) as c_uint);
    }
    (*spec).ops.setup_stream.unwrap()(codec, cvt_nid, (*per_pin).pin_nid, (*per_pin).dev_id, stream_tag, format as c_int)
}
unsafe extern "C" fn snd_hda_hdmi_generic_pcm_cleanup(hinfo: *mut hda_pcm_stream, codec: *mut hda_codec, _substream: *mut snd_pcm_substream) -> c_int {
    snd_hda_codec_cleanup_stream(codec, (*hinfo).nid); 0
}
unsafe extern "C" fn hdmi_pcm_close(hinfo: *mut hda_pcm_stream, codec: *mut hda_codec, _substream: *mut snd_pcm_substream) -> c_int {
    let spec = (*codec).spec;
    if (*hinfo).nid != 0 {
        let pcm_idx = hinfo_to_pcm_index(codec, hinfo);
        if snd_BUG_ON(pcm_idx < 0) { return -EINVAL; }
        let cvt_idx = cvt_nid_to_cvt_index(codec, (*hinfo).nid);
        if snd_BUG_ON(cvt_idx < 0) { return -EINVAL; }
        (*get_cvt(spec, cvt_idx)).assigned = false; (*hinfo).nid = 0;
        snd_hda_spdif_ctls_unassign(codec, pcm_idx); clear_bit(pcm_idx, &mut (*spec).pcm_in_use);
        let pin_idx = hinfo_to_pin_index(codec, hinfo);
        if pin_idx < 0 { return 0; }
        let per_pin = get_pin(spec, pin_idx);
        if (*spec).dyn_pin_out {
            snd_hda_set_dev_select(codec, (*per_pin).pin_nid, (*per_pin).dev_id);
            let pinctl = snd_hda_codec_read(codec, (*per_pin).pin_nid, 0, AC_VERB_GET_PIN_WIDGET_CONTROL, 0);
            snd_hda_codec_write(codec, (*per_pin).pin_nid, 0, AC_VERB_SET_PIN_WIDGET_CONTROL, (pinctl & !PIN_OUT) as c_uint);
        }
        (*per_pin).chmap_set = false; ptr::write_bytes((*per_pin).chmap.as_mut_ptr(), 0, (*per_pin).chmap.len());
        (*per_pin).setup = false; (*per_pin).channels = 0;
    }
    0
}

static generic_ops: hda_pcm_ops = hda_pcm_ops { open: Some(hdmi_pcm_open), close: Some(hdmi_pcm_close), prepare: Some(snd_hda_hdmi_generic_pcm_prepare), cleanup: Some(snd_hda_hdmi_generic_pcm_cleanup) };

unsafe extern "C" fn hdmi_get_spk_alloc(hdac: *mut hdac_device, pcm_idx: c_int) -> c_int {
    let codec = hdac_to_hda_codec(hdac);
    let per_pin = pcm_idx_to_pin((*codec).spec, pcm_idx);
    if per_pin.is_null() { 0 } else { (*per_pin).sink_eld.info.spk_alloc }
}
unsafe extern "C" fn hdmi_get_chmap(hdac: *mut hdac_device, pcm_idx: c_int, chmap: *mut u8) {
    let codec = hdac_to_hda_codec(hdac);
    let per_pin = pcm_idx_to_pin((*codec).spec, pcm_idx);
    if !per_pin.is_null() { ptr::copy_nonoverlapping((*per_pin).chmap.as_ptr(), chmap, (*per_pin).chmap.len()); }
}
unsafe extern "C" fn hdmi_set_chmap(hdac: *mut hdac_device, pcm_idx: c_int, chmap: *mut u8, prepared: c_int) {
    let codec = hdac_to_hda_codec(hdac);
    let per_pin = pcm_idx_to_pin((*codec).spec, pcm_idx);
    if per_pin.is_null() { return; }
    (*per_pin).chmap_set = true; ptr::copy_nonoverlapping(chmap, (*per_pin).chmap.as_mut_ptr(), (*per_pin).chmap.len());
    if prepared != 0 { snd_hda_hdmi_setup_audio_infoframe(codec, per_pin, (*per_pin).non_pcm); }
}
unsafe extern "C" fn is_hdmi_pcm_attached(hdac: *mut hdac_device, pcm_idx: c_int) -> bool {
    let codec = hdac_to_hda_codec(hdac);
    !pcm_idx_to_pin((*codec).spec, pcm_idx).is_null()
}
unsafe extern "C" fn snd_hda_hdmi_generic_build_pcms(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec;
    let pcm_num = min((*spec).num_cvts, (*spec).num_pins);
    for idx in 0..pcm_num {
        let info = snd_hda_codec_pcm_new(codec, b"HDMI %d\0".as_ptr() as *const c_char, idx);
        if info.is_null() { return -ENOMEM; }
        (*spec).pcm_rec[idx as usize].pcm = info; (*spec).pcm_used += 1; (*info).pcm_type = HDA_PCM_TYPE_HDMI; (*info).own_chmap = true;
        let pstr = (*info).stream.as_mut_ptr().add(SNDRV_PCM_STREAM_PLAYBACK);
        (*pstr).substreams = 1; (*pstr).ops = generic_ops;
        let per_cvt = get_cvt(spec, 0);
        (*pstr).channels_min = (*per_cvt).channels_min; (*pstr).channels_max = (*per_cvt).channels_max;
        if (*spec).pcm_used >= (*spec).pcm_rec.len() as c_int { break; }
    }
    0
}

unsafe extern "C" fn free_hdmi_jack_priv(jack: *mut snd_jack) {
    let pcm = (*jack).private_data as *mut hdmi_pcm;
    (*pcm).jack = ptr::null_mut();
}
unsafe fn generic_hdmi_build_jack(codec: *mut hda_codec, pcm_idx: c_int) -> c_int {
    let spec = (*codec).spec;
    let mut jack: *mut snd_jack = ptr::null_mut();
    let err = snd_jack_new((*codec).card, b"HDMI/DP\0".as_ptr() as *const c_char, SND_JACK_AVOUT, &mut jack, true, false);
    if err < 0 { return err; }
    (*spec).pcm_rec[pcm_idx as usize].jack = jack;
    (*jack).private_data = &mut (*spec).pcm_rec[pcm_idx as usize] as *mut _ as *mut c_void;
    (*jack).private_free = Some(free_hdmi_jack_priv);
    0
}
unsafe extern "C" fn snd_hda_hdmi_generic_build_controls(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec;
    for pcm_idx in 0..(*spec).pcm_used {
        if get_pcm_rec(spec, pcm_idx).is_null() { set_bit(pcm_idx, &mut (*spec).pcm_bitmap); continue; }
        let mut err = generic_hdmi_build_jack(codec, pcm_idx); if err < 0 { return err; }
        err = snd_hda_create_dig_out_ctls(codec, 0, (*spec).cvt_nids[0], HDA_PCM_TYPE_HDMI); if err < 0 { return err; }
        snd_hda_spdif_ctls_unassign(codec, pcm_idx);
        let dev = (*get_pcm_rec(spec, pcm_idx)).device;
        if dev != SNDRV_PCM_INVALID_DEVICE { err = hdmi_create_eld_ctl(codec, pcm_idx, dev); if err < 0 { return err; } }
    }
    for pin_idx in 0..(*spec).num_pins {
        let per_pin = get_pin(spec, pin_idx);
        if (*spec).static_pcm_mapping { hdmi_attach_hda_pcm(spec, per_pin); hdmi_pcm_setup_pin(spec, per_pin); }
        (*per_pin).sink_eld.eld_valid = false; hdmi_present_sense(per_pin, 0);
    }
    for pcm_idx in 0..(*spec).pcm_used {
        let pcm = get_pcm_rec(spec, pcm_idx);
        if pcm.is_null() || (*pcm).pcm.is_null() { break; }
        let err = snd_hdac_add_chmap_ctls((*pcm).pcm, pcm_idx, &mut (*spec).chmap); if err < 0 { return err; }
    }
    0
}
unsafe extern "C" fn snd_hda_hdmi_generic_init_per_pins(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec;
    for pin_idx in 0..(*spec).num_pins {
        let per_pin = get_pin(spec, pin_idx);
        (*per_pin).codec = codec;
        eld_proc_new(per_pin, pin_idx);
    }
    0
}
unsafe extern "C" fn snd_hda_hdmi_generic_init(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec;
    for pin_idx in 0..(*spec).num_pins {
        let per_pin = get_pin(spec, pin_idx);
        snd_hda_set_dev_select(codec, (*per_pin).pin_nid, (*per_pin).dev_id);
        hdmi_init_pin(codec, (*per_pin).pin_nid);
        if codec_has_acomp(codec) { continue; }
        snd_hda_jack_detect_enable_callback_mst(codec, (*per_pin).pin_nid, (*per_pin).dev_id, Some(jack_callback));
    }
    0
}
unsafe fn hdmi_array_init(spec: *mut hdmi_spec, nums: c_int) {
    snd_array_init(&mut (*spec).pins, size_of::<hdmi_spec_per_pin>(), nums);
    snd_array_init(&mut (*spec).cvts, size_of::<hdmi_spec_per_cvt>(), nums);
}
unsafe fn hdmi_array_free(spec: *mut hdmi_spec) {
    snd_array_free(&mut (*spec).pins); snd_array_free(&mut (*spec).cvts);
}
unsafe extern "C" fn snd_hda_hdmi_generic_spec_free(codec: *mut hda_codec) {
    let spec = (*codec).spec;
    if !spec.is_null() { hdmi_array_free(spec); kfree(spec as *mut c_void); (*codec).spec = ptr::null_mut(); }
    (*codec).dp_mst = false;
}
unsafe extern "C" fn snd_hda_hdmi_generic_remove(codec: *mut hda_codec) {
    let spec = (*codec).spec;
    if (*spec).acomp_registered { snd_hdac_acomp_exit(&mut (*(*codec).bus).core as *mut _ as *mut hdac_device); }
    else if codec_has_acomp(codec) { snd_hdac_acomp_register_notifier(&mut (*(*codec).bus).core as *mut _ as *mut hdac_device, ptr::null_mut()); }
    (*codec).relaxed_resume = 0;
    for pin_idx in 0..(*spec).num_pins { let per_pin = get_pin(spec, pin_idx); cancel_delayed_work_sync(&mut (*per_pin).work); eld_proc_free(per_pin); }
    for pcm_idx in 0..(*spec).pcm_used { if !(*spec).pcm_rec[pcm_idx as usize].jack.is_null() { snd_device_free((*codec).card, (*spec).pcm_rec[pcm_idx as usize].jack as *mut c_void); } }
    snd_hda_hdmi_generic_spec_free(codec);
}
unsafe extern "C" fn snd_hda_hdmi_generic_suspend(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec;
    for pin_idx in 0..(*spec).num_pins { cancel_delayed_work_sync(&mut (*get_pin(spec, pin_idx)).work); }
    0
}
unsafe extern "C" fn snd_hda_hdmi_generic_resume(codec: *mut hda_codec) -> c_int {
    let spec = (*codec).spec;
    snd_hda_codec_init(codec); snd_hda_regmap_sync(codec);
    for pin_idx in 0..(*spec).num_pins { hdmi_present_sense(get_pin(spec, pin_idx), 1); }
    0
}

static generic_standard_hdmi_ops: hdmi_ops = hdmi_ops {
    pin_get_eld: Some(hdmi_pin_get_eld),
    pin_setup_infoframe: Some(hdmi_pin_setup_infoframe),
    pin_hbr_setup: Some(hdmi_pin_hbr_setup),
    setup_stream: Some(snd_hda_hdmi_setup_stream),
    pin_cvt_fixup: None,
    prepare: None,
    silent_stream: None,
};

unsafe extern "C" fn snd_hda_hdmi_generic_alloc(codec: *mut hda_codec) -> c_int {
    let spec = kzalloc(size_of::<hdmi_spec>()) as *mut hdmi_spec;
    if spec.is_null() { return -ENOMEM; }
    (*spec).codec = codec; (*spec).ops = generic_standard_hdmi_ops; (*spec).dev_num = 1;
    snd_hdac_register_chmap_ops(&mut (*codec).core, &mut (*spec).chmap);
    (*spec).chmap.ops.get_chmap = Some(hdmi_get_chmap);
    (*spec).chmap.ops.set_chmap = Some(hdmi_set_chmap);
    (*spec).chmap.ops.is_pcm_attached = Some(is_hdmi_pcm_attached);
    (*spec).chmap.ops.get_spk_alloc = Some(hdmi_get_spk_alloc);
    (*codec).spec = spec;
    hdmi_array_init(spec, 4);
    0
}
unsafe extern "C" fn snd_hda_hdmi_generic_probe(codec: *mut hda_codec) -> c_int {
    let mut err = snd_hda_hdmi_generic_alloc(codec);
    if err < 0 { return err; }
    err = snd_hda_hdmi_parse_codec(codec);
    if err < 0 { snd_hda_hdmi_generic_spec_free(codec); return err; }
    snd_hda_hdmi_generic_init_per_pins(codec);
    0
}

unsafe fn reprogram_jack_detect(codec: *mut hda_codec, nid: hda_nid_t, dev_id: c_int, use_acomp: bool) {
    let tbl = snd_hda_jack_tbl_get_mst(codec, nid, dev_id);
    if !tbl.is_null() {
        let val = if use_acomp { 0 } else { AC_USRSP_EN | (*tbl).tag };
        snd_hda_codec_write_cache(codec, nid, 0, AC_VERB_SET_UNSOLICITED_ENABLE, val);
    }
}
unsafe fn generic_acomp_notifier_set(acomp: *mut drm_audio_component, use_acomp: bool) {
    let spec = (*acomp).audio_ops as *mut hdmi_spec;
    (*spec).use_acomp_notifier = use_acomp;
    (*(*spec).codec).relaxed_resume = use_acomp as c_int;
    (*(*(*spec).codec).bus).keep_power = 0;
    for i in 0..(*spec).num_pins { reprogram_jack_detect((*spec).codec, (*get_pin(spec, i)).pin_nid, (*get_pin(spec, i)).dev_id, use_acomp); }
}
unsafe extern "C" fn snd_hda_hdmi_acomp_master_bind(_dev: *mut device, acomp: *mut drm_audio_component) -> c_int {
    generic_acomp_notifier_set(acomp, true); 0
}
unsafe extern "C" fn snd_hda_hdmi_acomp_master_unbind(_dev: *mut device, acomp: *mut drm_audio_component) {
    generic_acomp_notifier_set(acomp, false);
}
unsafe extern "C" fn match_bound_vga(dev: *mut device, _subtype: c_int, data: *mut c_void) -> c_int {
    let bus = data as *mut hdac_bus;
    if !dev_is_pci(dev) || !dev_is_pci((*bus).dev) { return 0; }
    let master = to_pci_dev((*bus).dev);
    let pci = to_pci_dev(dev);
    ((*master).bus == (*pci).bus) as c_int
}
unsafe extern "C" fn snd_hda_hdmi_acomp_pin_eld_notify(audio_ptr: *mut c_void, port: c_int, dev_id: c_int) {
    let codec = audio_ptr as *mut hda_codec;
    let spec = (*codec).spec;
    let pin_nid = (*spec).port2pin.unwrap()(codec, port);
    if pin_nid == 0 { return; }
    if get_wcaps_type(get_wcaps(codec, pin_nid)) != AC_WID_PIN { return; }
    if (*codec).core.dev.power.power_state.event == PM_EVENT_SUSPEND { (*codec).acomp_requested_resume = 1; return; }
    snd_hda_hdmi_check_presence_and_report(codec, pin_nid, dev_id);
}
unsafe extern "C" fn snd_hda_hdmi_setup_drm_audio_ops(codec: *mut hda_codec, ops: *const drm_audio_component_audio_ops) {
    let spec = (*codec).spec;
    (*spec).drm_audio_ops.audio_ptr = codec as *mut c_void;
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    (*spec).drm_audio_ops.pin2port = (*ops).pin2port;
    (*spec).drm_audio_ops.pin_eld_notify = (*ops).pin_eld_notify;
    (*spec).drm_audio_ops.master_bind = (*ops).master_bind;
    (*spec).drm_audio_ops.master_unbind = (*ops).master_unbind;
}
unsafe extern "C" fn snd_hda_hdmi_acomp_init(codec: *mut hda_codec, ops: *const drm_audio_component_audio_ops, port2pin: Option<unsafe extern "C" fn(*mut hda_codec, c_int) -> hda_nid_t>) {
    let spec = (*codec).spec;
    if !enable_acomp { return; }
    (*spec).port2pin = port2pin;
    snd_hda_hdmi_setup_drm_audio_ops(codec, ops);
    if snd_hdac_acomp_init(&mut (*codec).core, &mut (*spec).drm_audio_ops, Some(match_bound_vga), 0) == 0 {
        (*spec).acomp_registered = true;
    }
}

const MODEL_GENERIC: usize = 0;
const MODEL_GF: usize = 1;
const MODEL_LOONGSON: usize = 2;

unsafe extern "C" fn generichdmi_probe(codec: *mut hda_codec, id: *const hda_device_id) -> c_int {
    let err = snd_hda_hdmi_generic_probe(codec);
    if err < 0 { return err; }
    if (*id).driver_data == MODEL_GF { (*codec).no_sticky_stream = 1; }
    if (*id).driver_data == MODEL_LOONGSON {
        if !(*codec).bus.is_null() && (*(*(*codec).bus).pci).revision == 0x2 { (*codec).eld_jack_detect = 1; }
    }
    0
}

static generichdmi_codec_ops: hda_codec_ops = hda_codec_ops {
    probe: Some(generichdmi_probe),
    remove: Some(snd_hda_hdmi_generic_remove),
    init: Some(snd_hda_hdmi_generic_init),
    build_pcms: Some(snd_hda_hdmi_generic_build_pcms),
    build_controls: Some(snd_hda_hdmi_generic_build_controls),
    unsol_event: Some(snd_hda_hdmi_generic_unsol_event),
    suspend: Some(snd_hda_hdmi_generic_suspend),
    resume: Some(snd_hda_hdmi_generic_resume),
};

static snd_hda_id_generichdmi: [hda_device_id; 33] = [
    hda_device_id { driver_data: MODEL_LOONGSON },
    hda_device_id { driver_data: MODEL_GENERIC },
    hda_device_id { driver_data: MODEL_GENERIC },
    hda_device_id { driver_data: MODEL_GENERIC },
    hda_device_id { driver_data: MODEL_GENERIC },
    hda_device_id { driver_data: MODEL_GENERIC },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GENERIC },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GF },
    hda_device_id { driver_data: MODEL_GENERIC },
    hda_device_id { driver_data: MODEL_GENERIC },
    hda_device_id { driver_data: MODEL_GENERIC },
    hda_device_id { driver_data: MODEL_GENERIC },
    hda_device_id { driver_data: MODEL_GENERIC },
    hda_device_id { driver_data: MODEL_GENERIC },
    hda_device_id { driver_data: MODEL_GENERIC },
    hda_device_id { driver_data: MODEL_GENERIC },
    hda_device_id { driver_data: 0 },
];

static mut generichdmi_driver: hda_codec_driver = hda_codec_driver {
    id: snd_hda_id_generichdmi.as_ptr(),
    ops: &generichdmi_codec_ops,
};

/* MODULE_DEVICE_TABLE, MODULE_LICENSE, MODULE_DESCRIPTION, and
 * module_hda_codec_driver metadata are represented by the static driver
 * declaration above in this isolated Rust translation.
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
