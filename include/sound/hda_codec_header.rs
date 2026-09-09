/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Universal Interface for Intel High Definition Audio Codec */

use core::ffi::{c_char, c_int, c_uint, c_void};

/* External types supplied by the surrounding kernel/audio implementation. */
pub enum hda_bus_core {}
pub enum hdac_bus {}
pub enum snd_card {}
pub enum pci_dev {}
pub enum mutex {}
pub enum hdac_driver {}
pub enum hda_device_id {}
pub enum hda_codec_ops {}
pub enum module {}
pub enum hdac_device {}
pub enum snd_pcm_substream {}
pub enum snd_pcm_chmap_elem {}
pub enum snd_pcm {}
pub enum list_head {}
pub enum snd_refcount {}
#[repr(C)] pub struct snd_array { pub used: c_uint }
pub enum hda_beep {}
pub enum delayed_work {}
pub enum hda_fixup {}
pub enum snd_info_buffer {}
pub enum snd_hwdep {}
pub enum snd_dma_buffer {}
pub enum dev_pm_ops {}

pub type hda_nid_t = u16;
pub type u8 = ::core::primitive::u8;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;
pub type size_t = usize;

#[repr(C)]
pub struct hda_bus {
    pub core: hdac_bus,
    pub card: *mut snd_card,
    pub pci: *mut pci_dev,
    pub modelname: *const c_char,
    pub prepare_mutex: mutex,
    pub pcm_dev_bits: [u64; 1],
    pub allow_bus_reset: c_uint,
    pub shutdown: c_uint,
    pub response_reset: c_uint,
    pub in_reset: c_uint,
    pub no_response_fallback: c_uint,
    pub bus_probing: c_uint,
    pub keep_power: c_uint,
    pub jackpoll_in_suspend: c_uint,
    pub primary_dig_out_type: c_int,
    pub mixer_assigned: c_uint,
}

pub const HDA_CODEC_ID_SKIP_PROBE: u32 = 0x00000001;
pub const HDA_CODEC_ID_GENERIC_HDMI: u32 = 0x00000101;
pub const HDA_CODEC_ID_GENERIC: u32 = 0x00000201;

#[repr(C)]
pub struct hda_codec_driver {
    pub core: hdac_driver,
    pub id: *const hda_device_id,
    pub ops: *const hda_codec_ops,
}

pub type hda_probe_fn = Option<unsafe extern "C" fn(*mut hda_codec, *const hda_device_id) -> c_int>;
pub type hda_remove_fn = Option<unsafe extern "C" fn(*mut hda_codec)>;
pub type hda_build_fn = Option<unsafe extern "C" fn(*mut hda_codec) -> c_int>;
pub type hda_unsol_fn = Option<unsafe extern "C" fn(*mut hda_codec, c_uint)>;
pub type hda_power_fn = Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t, c_uint)>;
pub type hda_status_fn = Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t) -> c_int>;
pub type hda_stream_pm_fn = Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t, bool)>;

#[repr(C)]
pub struct hda_codec_ops {
    pub probe: hda_probe_fn,
    pub remove: hda_remove_fn,
    pub build_controls: hda_build_fn,
    pub build_pcms: hda_build_fn,
    pub init: hda_build_fn,
    pub unsol_event: hda_unsol_fn,
    pub set_power_state: hda_power_fn,
    pub suspend: hda_build_fn,
    pub resume: hda_build_fn,
    pub check_power_status: hda_status_fn,
    pub stream_pm: hda_stream_pm_fn,
}

pub type pcm_open_fn = Option<unsafe extern "C" fn(*mut hda_pcm_stream, *mut hda_codec, *mut snd_pcm_substream) -> c_int>;
pub type pcm_prepare_fn = Option<unsafe extern "C" fn(*mut hda_pcm_stream, *mut hda_codec, c_uint, c_uint, *mut snd_pcm_substream) -> c_int>;
pub type pcm_delay_fn = Option<unsafe extern "C" fn(*mut hda_pcm_stream, *mut hda_codec, *mut snd_pcm_substream) -> c_uint>;
#[repr(C)]
pub struct hda_pcm_ops { pub open: pcm_open_fn, pub close: pcm_open_fn, pub prepare: pcm_prepare_fn, pub cleanup: pcm_open_fn, pub get_delay: pcm_delay_fn }

#[repr(C)]
pub struct hda_pcm_stream {
    pub substreams: c_uint, pub channels_min: c_uint, pub channels_max: c_uint, pub nid: hda_nid_t,
    pub rates: u32, pub formats: u64, pub subformats: u32, pub maxbps: c_uint,
    pub chmap: *const snd_pcm_chmap_elem, pub ops: hda_pcm_ops,
}

pub const HDA_PCM_TYPE_AUDIO: c_uint = 0;
pub const HDA_PCM_TYPE_SPDIF: c_uint = 1;
pub const HDA_PCM_TYPE_HDMI: c_uint = 2;
pub const HDA_PCM_TYPE_MODEM: c_uint = 3;
pub const HDA_PCM_NTYPES: c_uint = 4;
pub const SNDRV_PCM_INVALID_DEVICE: c_int = -1;

#[repr(C)]
pub struct hda_pcm { pub name: *mut c_char, pub stream: [hda_pcm_stream; 2], pub pcm_type: c_uint, pub device: c_int, pub pcm: *mut snd_pcm, pub own_chmap: bool, pub codec: *mut hda_codec, pub list: list_head, pub disconnected: c_uint }

#[repr(C)]
pub struct hda_codec {
    pub core: hdac_device, pub bus: *mut hda_bus, pub card: *mut snd_card, pub addr: c_uint, pub probe_id: u32,
    pub preset: *const hda_device_id, pub modelname: *const c_char, pub pcm_list_head: list_head, pub pcm_ref: snd_refcount,
    pub spec: *mut c_void, pub beep: *mut hda_beep, pub beep_mode: c_uint, pub beep_just_power_on: bool, pub wcaps: *mut u32,
    pub mixers: snd_array, pub nids: snd_array, pub conn_list: list_head, pub spdif_mutex: mutex, pub control_mutex: mutex,
    pub spdif_out: snd_array, pub spdif_in_enable: c_uint, pub follower_dig_outs: *const hda_nid_t, pub init_pins: snd_array,
    pub driver_pins: snd_array, pub cvt_setups: snd_array, pub user_mutex: mutex,
    /* CONFIG_SND_HDA_RECONFIG fields */ pub init_verbs: snd_array, pub hints: snd_array, pub user_pins: snd_array,
    /* CONFIG_SND_HDA_HWDEP field */ pub hwdep: *mut snd_hwdep,
    pub configured: c_uint, pub in_freeing: c_uint, pub display_power_control: c_uint, pub spdif_status_reset: c_uint,
    pub pin_amp_workaround: c_uint, pub single_adc_amp: c_uint, pub no_sticky_stream: c_uint, pub pins_shutup: c_uint,
    pub no_trigger_sense: c_uint, pub no_jack_detect: c_uint, pub inv_eapd: c_uint, pub inv_jack_detect: c_uint,
    pub pcm_format_first: c_uint, pub cached_write: c_uint, pub dp_mst: c_uint, pub dump_coef: c_uint, pub power_save_node: c_uint,
    pub auto_runtime_pm: c_uint, pub force_pin_prefix: c_uint, pub link_down_at_suspend: c_uint, pub relaxed_resume: c_uint,
    pub forced_resume: c_uint, pub acomp_requested_resume: c_uint, pub no_stream_clean_at_suspend: c_uint, pub ctl_dev_id: c_uint,
    pub eld_jack_detect: c_uint, pub power_on_acct: c_ulong, pub power_off_acct: c_ulong, pub power_jiffies: c_ulong,
    pub power_filter: Option<unsafe extern "C" fn(*mut hda_codec, hda_nid_t, c_uint) -> c_uint>,
    pub proc_widget_hook: Option<unsafe extern "C" fn(*mut snd_info_buffer, *mut hda_codec, hda_nid_t)>,
    pub jacktbl: snd_array, pub jackpoll_interval: c_ulong, pub jackpoll_work: delayed_work, pub depop_delay: c_int,
    pub fixup_id: c_int, pub fixup_list: *const hda_fixup, pub fixup_name: *const c_char, pub verbs: snd_array,
}
pub type c_ulong = usize;

#[repr(C)] pub struct hda_verb { pub nid: hda_nid_t, pub verb: u32, pub param: u32 }
#[repr(C)] pub struct hda_pincfg { pub nid: hda_nid_t, pub ctrl: u8, pub target: u8, pub cfg: c_uint }
#[repr(C)] pub struct hda_spdif_out { pub nid: hda_nid_t, pub status: c_uint, pub ctls: u16 }
#[repr(C)] pub struct __hda_power_obj { pub codec: *mut hda_codec, pub err: c_int }

extern "C" {
    pub fn __hda_codec_driver_register(drv: *mut hda_codec_driver, name: *const c_char, owner: *mut module) -> c_int;
    pub fn hda_codec_driver_unregister(drv: *mut hda_codec_driver);
    pub fn snd_hda_codec_device_init(bus: *mut hda_bus, codec_addr: c_uint, fmt: *const c_char, ...) -> *mut hda_codec;
    pub fn snd_hda_codec_new(bus: *mut hda_bus, card: *mut snd_card, codec_addr: c_uint, codecp: *mut *mut hda_codec) -> c_int;
    pub fn snd_hda_codec_device_new(bus: *mut hda_bus, card: *mut snd_card, codec_addr: c_uint, codec: *mut hda_codec, snddev_managed: bool) -> c_int;
    pub fn snd_hda_codec_configure(codec: *mut hda_codec) -> c_int;
    pub fn snd_hda_codec_update_widgets(codec: *mut hda_codec) -> c_int;
    pub fn snd_hda_codec_register(codec: *mut hda_codec);
    pub fn snd_hda_codec_unregister(codec: *mut hda_codec);
    pub fn snd_hda_codec_cleanup_for_unbind(codec: *mut hda_codec);
    pub fn snd_hdac_codec_read(core: *mut hdac_device, nid: hda_nid_t, flags: c_int, verb: c_uint, parm: c_uint) -> c_uint;
    pub fn snd_hdac_codec_write(core: *mut hdac_device, nid: hda_nid_t, flags: c_int, verb: c_uint, parm: c_uint) -> c_int;
    pub fn snd_hdac_regmap_write(core: *mut hdac_device, nid: hda_nid_t, verb: c_uint, parm: c_uint) -> c_int;
    pub fn snd_hda_get_connections(codec: *mut hda_codec, nid: hda_nid_t, list: *mut hda_nid_t, max_conns: c_int) -> c_int;
    pub fn snd_hda_sequence_write(codec: *mut hda_codec, seq: *const hda_verb);
    pub fn snd_hda_codec_get_pincfg(codec: *mut hda_codec, nid: hda_nid_t) -> c_uint;
    pub fn snd_hda_codec_set_pincfg(codec: *mut hda_codec, nid: hda_nid_t, cfg: c_uint) -> c_int;
    pub fn snd_hda_shutup_pins(codec: *mut hda_codec);
    pub fn snd_hda_spdif_out_of_nid(codec: *mut hda_codec, nid: hda_nid_t) -> *mut hda_spdif_out;
    pub fn snd_hda_codec_build_controls(codec: *mut hda_codec) -> c_int;
    pub fn snd_hda_codec_parse_pcms(codec: *mut hda_codec) -> c_int;
    pub fn snd_hda_codec_build_pcms(codec: *mut hda_codec) -> c_int;
    pub fn snd_hda_codec_prepare(codec: *mut hda_codec, hinfo: *mut hda_pcm_stream, stream: c_uint, format: c_uint, substream: *mut snd_pcm_substream) -> c_int;
    pub fn snd_hda_codec_cleanup(codec: *mut hda_codec, hinfo: *mut hda_pcm_stream, substream: *mut snd_pcm_substream);
    pub fn snd_hda_codec_setup_stream(codec: *mut hda_codec, nid: hda_nid_t, stream_tag: u32, channel_id: c_int, format: c_int);
    pub fn __snd_hda_codec_cleanup_stream(codec: *mut hda_codec, nid: hda_nid_t, do_now: c_int);
    pub fn snd_hda_attach_pcm_stream(bus: *mut hda_bus, codec: *mut hda_codec, cpcm: *mut hda_pcm) -> c_int;
    pub fn snd_hda_get_codec_name(codec: *mut hda_codec, name: *mut c_char, namelen: c_int);
    pub fn snd_hda_codec_set_power_to_all(codec: *mut hda_codec, fg: hda_nid_t, power_state: c_uint);
    pub fn snd_hda_lock_devices(bus: *mut hda_bus) -> c_int;
    pub fn snd_hda_unlock_devices(bus: *mut hda_bus);
    pub fn snd_hda_bus_reset(bus: *mut hda_bus);
    pub fn snd_hda_bus_reset_codecs(bus: *mut hda_bus);
    pub fn snd_hda_codec_set_gpio(codec: *mut hda_codec, mask: c_uint, dir: c_uint, data: c_uint, delay: c_uint);
    pub fn snd_hda_codec_set_name(codec: *mut hda_codec, name: *const c_char) -> c_int;
    pub fn snd_hda_codec_set_power_save(codec: *mut hda_codec, delay: c_int);
    pub fn snd_hda_set_power_save(bus: *mut hda_bus, delay: c_int);
    pub fn snd_hda_update_power_acct(codec: *mut hda_codec);
    pub static hda_codec_driver_pm: dev_pm_ops;
    pub static snd_pcm_2_1_chmaps: snd_pcm_chmap_elem;
}

pub const HDA_RW_NO_RESPONSE_FALLBACK: c_int = 1 << 0;

#[inline] pub unsafe fn snd_hda_codec_read(c: *mut hda_codec, n: hda_nid_t, f: c_int, v: c_uint, p: c_uint) -> c_uint { snd_hdac_codec_read(&mut (*c).core, n, f, v, p) }
#[inline] pub unsafe fn snd_hda_codec_write(c: *mut hda_codec, n: hda_nid_t, f: c_int, v: c_uint, p: c_uint) -> c_int { snd_hdac_codec_write(&mut (*c).core, n, f, v, p) }
#[inline] pub unsafe fn snd_hda_codec_write_sync(c: *mut hda_codec, n: hda_nid_t, f: c_int, v: c_uint, p: c_uint) -> c_int { snd_hdac_codec_read(&mut (*c).core, n, f, v, p) as c_int }
#[inline] pub unsafe fn snd_hda_get_num_conns(c: *mut hda_codec, n: hda_nid_t) -> c_int { snd_hda_get_connections(c, n, core::ptr::null_mut(), 0) }
#[inline] pub unsafe fn snd_hda_codec_write_cache(c: *mut hda_codec, n: hda_nid_t, _f: c_int, v: c_uint, p: c_uint) -> c_int { snd_hdac_regmap_write(&mut (*c).core, n, v, p) }
#[inline] pub unsafe fn hda_codec_need_resume(c: *mut hda_codec) -> bool { (*c).relaxed_resume == 0 && (*c).jacktbl.used != 0 }

/* Configuration-gated patch loader and DSP loader declarations are preserved as external interfaces. */
extern "C" {
    pub fn snd_hda_load_patch(bus: *mut hda_bus, size: size_t, buf: *const c_void) -> c_int;
    pub fn snd_hda_codec_load_dsp_prepare(codec: *mut hda_codec, format: c_uint, size: c_uint, bufp: *mut snd_dma_buffer) -> c_int;
    pub fn snd_hda_codec_load_dsp_trigger(codec: *mut hda_codec, start: bool);
    pub fn snd_hda_codec_load_dsp_cleanup(codec: *mut hda_codec, dmab: *mut snd_dma_buffer);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
