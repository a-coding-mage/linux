/* SPDX-License-Identifier: GPL-2.0
 * Rust translation of linux/sound/soc.h.
 * C includes, configuration branches, and macros whose expansion depends on
 * declarations supplied by other kernel headers are retained below as intent.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Dependencies supplied by the surrounding kernel translation unit:
// linux/{device,mutex,workqueue,types,...}, sound/{pcm,control,core,...}.

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const core::ffi::c_char,
    pub formats: u64,
    pub subformats: u32,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub channels_min: u32,
    pub channels_max: u32,
    pub sig_bits: u32,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> i32>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> i32>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> i32>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> i32>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, i32) -> i32>,
}

#[repr(C)]
pub struct snd_soc_compr_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_compr_stream) -> i32>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_compr_stream)>,
    pub set_params: Option<unsafe extern "C" fn(*mut snd_compr_stream) -> i32>,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const core::ffi::c_char,
    pub of_node: *mut device_node,
    pub dai_name: *const core::ffi::c_char,
    pub dai_args: *const of_phandle_args,
    pub ext_fmt: u32,
}

#[repr(C)]
pub struct snd_soc_dai_link_ch_map { pub cpu: u32, pub codec: u32, pub ch_mask: u32 }

#[repr(C)]
pub struct soc_mixer_control {
    pub min: i32, pub max: i32, pub platform_max: i32,
    pub reg: i32, pub rreg: i32, pub shift: u32, pub rshift: u32,
    pub num_channels: u32, pub sign_bit: u32, pub invert: u32, pub autodisable: u32,
}
#[repr(C)] pub struct soc_bytes { pub base: i32, pub num_regs: i32, pub mask: u32 }
#[repr(C)] pub struct soc_mreg_control { pub min: i64, pub max: i64, pub regbase: u32, pub regcount: u32, pub nbits: u32, pub invert: u32 }
#[repr(C)] pub struct soc_enum {
    pub reg: i32, pub shift_l: u8, pub shift_r: u8, pub items: u32, pub mask: u32,
    pub texts: *const *const core::ffi::c_char, pub values: *const u32, pub autodisable: u32,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const core::ffi::c_char, pub stream_name: *const core::ffi::c_char,
    pub cpus: *mut snd_soc_dai_link_component, pub num_cpus: u32,
    pub codecs: *mut snd_soc_dai_link_component, pub num_codecs: u32,
    pub ch_maps: *mut snd_soc_dai_link_ch_map,
    pub platforms: *mut snd_soc_dai_link_component, pub num_platforms: u32,
    pub id: i32, pub c2c_params: *const snd_soc_pcm_stream, pub num_c2c_params: u32,
    pub dai_fmt: u32, pub trigger: [i32; 2],
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> i32>,
    pub exit: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime)>,
    pub be_hw_params_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> i32>,
    pub ops: *const snd_soc_ops, pub compr_ops: *const snd_soc_compr_ops,
    pub trigger_start: i32, pub trigger_stop: i32,
    pub nonatomic: u32, pub playback_only: u32, pub capture_only: u32, pub ignore_suspend: u32,
    pub symmetric_rate: u32, pub symmetric_channels: u32, pub symmetric_sample_bits: u32,
    pub no_pcm: u32, pub dynamic: u32, pub dpcm_merged_format: u32, pub dpcm_merged_chan: u32,
    pub dpcm_merged_rate: u32, pub ignore_pmdown_time: u32, pub ignore: u32,
}

#[repr(C)] pub struct snd_soc_codec_conf { pub dlc: snd_soc_dai_link_component, pub name_prefix: *const core::ffi::c_char }
#[repr(C)] pub struct snd_soc_aux_dev { pub dlc: snd_soc_dai_link_component, pub init: Option<unsafe extern "C" fn(*mut snd_soc_component) -> i32> }

#[repr(C)] pub struct snd_soc_card {
    pub name: *const core::ffi::c_char, pub long_name: *const core::ffi::c_char,
    pub driver_name: *const core::ffi::c_char, pub components: *const core::ffi::c_char,
    pub topology_shortname: *mut core::ffi::c_char, pub dev: *mut device,
    pub snd_card: *mut snd_card, pub owner: *mut module,
    pub mutex: mutex, pub dapm_mutex: mutex, pub pcm_mutex: mutex,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> i32>,
    pub late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> i32>,
    pub fixup_controls: Option<unsafe extern "C" fn(*mut snd_soc_card)>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_card) -> i32>,
    pub dai_link: *mut snd_soc_dai_link, pub num_links: i32,
    pub codec_conf: *mut snd_soc_codec_conf, pub num_configs: i32,
    pub aux_dev: *mut snd_soc_aux_dev, pub num_aux_devs: i32,
    pub controls: *const snd_kcontrol_new, pub num_controls: i32,
    pub dapm: *mut snd_soc_dapm_context, pub instantiated: u32,
    pub fully_routed: u32, pub probed: u32, pub component_chaining: u32,
    pub devres_dev: *mut device, pub drvdata: *mut core::ffi::c_void,
}

#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime }
#[repr(C)] pub struct snd_pcm_runtime { pub hw: snd_pcm_hardware }
#[repr(C)] pub struct snd_pcm_hardware;
#[repr(C)] pub struct snd_pcm_hw_params;
#[repr(C)] pub struct snd_compr_stream;
#[repr(C)] pub struct snd_soc_pcm_runtime;
#[repr(C)] pub struct snd_soc_component;
#[repr(C)] pub struct snd_soc_dai;
#[repr(C)] pub struct snd_soc_dapm_context;
#[repr(C)] pub struct snd_kcontrol_new;
#[repr(C)] pub struct snd_card;
#[repr(C)] pub struct device;
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct of_phandle_args;
#[repr(C)] pub struct module;
#[repr(C)] pub struct mutex;

pub const SND_SOC_TRIGGER_ORDER_DEFAULT: i32 = 0;
pub const SND_SOC_TRIGGER_ORDER_LDC: i32 = 1;
pub const SND_SOC_TRIGGER_ORDER_MAX: i32 = 2;

#[inline] pub unsafe fn snd_soc_card_is_instantiated(card: *const snd_soc_card) -> bool { !card.is_null() && (*card).instantiated != 0 }
#[inline] pub unsafe fn snd_soc_link_num_ch_map(link: *const snd_soc_dai_link) -> i32 { (*link).num_cpus.max((*link).num_codecs) as i32 }
#[inline] pub unsafe fn snd_soc_link_to_cpu(link: *mut snd_soc_dai_link, n: isize) -> *mut snd_soc_dai_link_component { (*link).cpus.offset(n) }
#[inline] pub unsafe fn snd_soc_link_to_codec(link: *mut snd_soc_dai_link, n: isize) -> *mut snd_soc_dai_link_component { (*link).codecs.offset(n) }
#[inline] pub unsafe fn snd_soc_link_to_platform(link: *mut snd_soc_dai_link, n: isize) -> *mut snd_soc_dai_link_component { (*link).platforms.offset(n) }
#[inline] pub unsafe fn snd_soc_volsw_is_stereo(mc: *const soc_mixer_control) -> bool { (*mc).reg != (*mc).rreg || (*mc).shift != (*mc).rshift }
#[inline] pub unsafe fn snd_soc_enum_val_to_item(e: *const soc_enum, val: u32) -> u32 { if (*e).values.is_null() { return val; } for i in 0..(*e).items { if *(*e).values.add(i as usize) == val { return i; } } 0 }
#[inline] pub unsafe fn snd_soc_enum_item_to_val(e: *const soc_enum, item: u32) -> u32 { if (*e).values.is_null() { item } else { *(*e).values.add(item as usize) } }

extern "C" {
    pub fn snd_soc_register_card(card: *mut snd_soc_card) -> i32;
    pub fn snd_soc_unregister_card(card: *mut snd_soc_card);
    pub fn snd_soc_get_pcm_runtime(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> *mut snd_soc_pcm_runtime;
    pub fn snd_soc_runtime_action(rtd: *mut snd_soc_pcm_runtime, stream: i32, action: i32);
    pub fn snd_soc_calc_frame_size(sample_size: i32, channels: i32, tdm_slots: i32) -> i32;
    pub fn snd_soc_calc_bclk(fs: i32, sample_size: i32, channels: i32, tdm_slots: i32) -> i32;
    pub fn snd_soc_util_init() -> i32;
    pub fn snd_soc_util_exit();
    pub fn snd_soc_fixup_dai_links_platform_name(card: *mut snd_soc_card, platform_name: *const core::ffi::c_char) -> i32;
}

// The source header also defines the SOC_* kcontrol builders, DAI-link
// iteration/building macros, configuration-specific declarations, and the
// remaining extern utility/control functions. Their token-level names and
// expansion intent are preserved here for dependent translation units.
#[macro_export] macro_rules! SOC_SINGLE_VALUE { ($reg:expr,$shift:expr,$min:expr,$max:expr,$invert:expr,$auto:expr) => { $crate::SOC_DOUBLE_VALUE!($reg,$shift,$shift,$min,$max,$invert,$auto) }; }
#[macro_export] macro_rules! SOC_DOUBLE_VALUE { ($reg:expr,$sl:expr,$sr:expr,$min:expr,$max:expr,$invert:expr,$auto:expr) => { unsafe { &mut $crate::soc_mixer_control { reg:$reg, rreg:$reg, shift:$sl, rshift:$sr, min:$min, max:$max, platform_max:0, num_channels:0, sign_bit:0, invert:$invert, autodisable:$auto } } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
