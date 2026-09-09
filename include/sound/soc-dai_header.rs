/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/sound/soc-dai.h. */

use core::ffi::{c_char, c_int, c_void};

pub const SND_SOC_DAIFMT_I2S: u32 = SND_SOC_DAI_FORMAT_I2S;
pub const SND_SOC_DAIFMT_RIGHT_J: u32 = SND_SOC_DAI_FORMAT_RIGHT_J;
pub const SND_SOC_DAIFMT_LEFT_J: u32 = SND_SOC_DAI_FORMAT_LEFT_J;
pub const SND_SOC_DAIFMT_DSP_A: u32 = SND_SOC_DAI_FORMAT_DSP_A;
pub const SND_SOC_DAIFMT_DSP_B: u32 = SND_SOC_DAI_FORMAT_DSP_B;
pub const SND_SOC_DAIFMT_AC97: u32 = SND_SOC_DAI_FORMAT_AC97;
pub const SND_SOC_DAIFMT_PDM: u32 = SND_SOC_DAI_FORMAT_PDM;
pub const SND_SOC_DAIFMT_MSB: u32 = SND_SOC_DAIFMT_LEFT_J;
pub const SND_SOC_DAIFMT_LSB: u32 = SND_SOC_DAIFMT_RIGHT_J;
pub const SND_SOC_POSSIBLE_DAIFMT_FORMAT_SHIFT: u32 = 0;
pub const SND_SOC_POSSIBLE_DAIFMT_FORMAT_MASK: u64 = 0xFFFFu64 << SND_SOC_POSSIBLE_DAIFMT_FORMAT_SHIFT;
pub const SND_SOC_POSSIBLE_DAIFMT_I2S: u64 = 1u64 << SND_SOC_DAI_FORMAT_I2S;
pub const SND_SOC_POSSIBLE_DAIFMT_RIGHT_J: u64 = 1u64 << SND_SOC_DAI_FORMAT_RIGHT_J;
pub const SND_SOC_POSSIBLE_DAIFMT_LEFT_J: u64 = 1u64 << SND_SOC_DAI_FORMAT_LEFT_J;
pub const SND_SOC_POSSIBLE_DAIFMT_DSP_A: u64 = 1u64 << SND_SOC_DAI_FORMAT_DSP_A;
pub const SND_SOC_POSSIBLE_DAIFMT_DSP_B: u64 = 1u64 << SND_SOC_DAI_FORMAT_DSP_B;
pub const SND_SOC_POSSIBLE_DAIFMT_AC97: u64 = 1u64 << SND_SOC_DAI_FORMAT_AC97;
pub const SND_SOC_POSSIBLE_DAIFMT_PDM: u64 = 1u64 << SND_SOC_DAI_FORMAT_PDM;
pub const SND_SOC_DAI_TDM_IDLE_NONE: c_int = 0;
pub const SND_SOC_DAI_TDM_IDLE_OFF: c_int = 1;
pub const SND_SOC_DAI_TDM_IDLE_ZERO: c_int = 2;
pub const SND_SOC_DAI_TDM_IDLE_PULLDOWN: c_int = 3;
pub const SND_SOC_DAI_TDM_IDLE_HIZ: c_int = 4;
pub const SND_SOC_DAI_TDM_IDLE_PULLUP: c_int = 5;
pub const SND_SOC_DAI_TDM_IDLE_DRIVE_HIGH: c_int = 6;
pub const SND_SOC_DAIFMT_CONT: u32 = 1 << 4;
pub const SND_SOC_DAIFMT_GATED: u32 = 0 << 4;
pub const SND_SOC_POSSIBLE_DAIFMT_CLOCK_SHIFT: u32 = 16;
pub const SND_SOC_POSSIBLE_DAIFMT_CLOCK_MASK: u64 = 0xFFFFu64 << 16;
pub const SND_SOC_POSSIBLE_DAIFMT_GATED: u64 = 0x1u64 << 16;
pub const SND_SOC_POSSIBLE_DAIFMT_CONT: u64 = 0x2u64 << 16;
pub const SND_SOC_DAIFMT_NB_NF: u32 = 0 << 8;
pub const SND_SOC_DAIFMT_NB_IF: u32 = 2 << 8;
pub const SND_SOC_DAIFMT_IB_NF: u32 = 3 << 8;
pub const SND_SOC_DAIFMT_IB_IF: u32 = 4 << 8;
pub const SND_SOC_POSSIBLE_DAIFMT_INV_SHIFT: u32 = 32;
pub const SND_SOC_POSSIBLE_DAIFMT_INV_MASK: u64 = 0xFFFFu64 << 32;
pub const SND_SOC_POSSIBLE_DAIFMT_NB_NF: u64 = 1u64 << 32;
pub const SND_SOC_POSSIBLE_DAIFMT_NB_IF: u64 = 2u64 << 32;
pub const SND_SOC_POSSIBLE_DAIFMT_IB_NF: u64 = 4u64 << 32;
pub const SND_SOC_POSSIBLE_DAIFMT_IB_IF: u64 = 8u64 << 32;
pub const SND_SOC_DAIFMT_CBP_CFP: u32 = 1 << 12;
pub const SND_SOC_DAIFMT_CBC_CFP: u32 = 2 << 12;
pub const SND_SOC_DAIFMT_CBP_CFC: u32 = 3 << 12;
pub const SND_SOC_DAIFMT_CBC_CFC: u32 = 4 << 12;
pub const SND_SOC_DAIFMT_BP_FP: u32 = SND_SOC_DAIFMT_CBP_CFP;
pub const SND_SOC_DAIFMT_BC_FP: u32 = SND_SOC_DAIFMT_CBC_CFP;
pub const SND_SOC_DAIFMT_BP_FC: u32 = SND_SOC_DAIFMT_CBP_CFC;
pub const SND_SOC_DAIFMT_BC_FC: u32 = SND_SOC_DAIFMT_CBC_CFC;
pub const SND_SOC_DAIFMT_FORMAT_MASK: u32 = 0x000f;
pub const SND_SOC_DAIFMT_CLOCK_MASK: u32 = 0x00f0;
pub const SND_SOC_DAIFMT_INV_MASK: u32 = 0x0f00;
pub const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: u32 = 0xf000;
pub const SND_SOC_DAIFMT_MASTER_MASK: u32 = SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK;
pub const SND_SOC_CLOCK_IN: c_int = 0;
pub const SND_SOC_CLOCK_OUT: c_int = 1;
pub const SND_SOC_STD_AC97_FMTS: u64 = SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S20_3BE | SNDRV_PCM_FMTBIT_S20_LE | SNDRV_PCM_FMTBIT_S20_BE | SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S24_3BE | SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S32_BE;
/* C preprocessor aliases retained as Rust functions where arguments are typed. */
#[inline] pub unsafe fn snd_soc_dai_get_widget_playback(dai:*mut snd_soc_dai)->*mut snd_soc_dapm_widget { snd_soc_dai_get_widget(dai,0) }
#[inline] pub unsafe fn snd_soc_dai_get_widget_capture(dai:*mut snd_soc_dai)->*mut snd_soc_dapm_widget { snd_soc_dai_get_widget(dai,1) }
#[inline] pub unsafe fn snd_soc_dai_set_widget_playback(dai:*mut snd_soc_dai,w:*mut snd_soc_dapm_widget){snd_soc_dai_set_widget(dai,0,w)}
#[inline] pub unsafe fn snd_soc_dai_set_widget_capture(dai:*mut snd_soc_dai,w:*mut snd_soc_dapm_widget){snd_soc_dai_set_widget(dai,1,w)}

#[repr(C)] pub struct snd_pcm_substream { _opaque: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { _opaque: [u8; 0] }
#[repr(C)] pub struct snd_compr_stream { _opaque: [u8; 0] }
#[repr(C)] pub struct clk { _opaque: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_driver;
#[repr(C)] pub struct snd_soc_dai;
#[repr(C)] pub struct snd_ac97_bus_ops;
#[repr(C)] pub struct snd_soc_pcm_runtime;
#[repr(C)] pub struct snd_pcm_hw_params;
#[repr(C)] pub struct snd_compr_params;
#[repr(C)] pub struct snd_codec;
#[repr(C)] pub struct snd_compr_tstamp64;
#[repr(C)] pub struct snd_compr_metadata;
#[repr(C)] pub struct snd_soc_dobj;
#[repr(C)] pub struct of_phandle_args;
#[repr(C)] pub struct snd_soc_pcm_stream;
#[repr(C)] pub struct device;
#[repr(C)] pub struct snd_soc_component;
#[repr(C)] pub struct list_head;
pub type snd_pcm_sframes_t = i64;

extern "C" {
    pub fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: u32, dir: c_int) -> c_int;
    pub fn snd_soc_dai_set_clkdiv(dai: *mut snd_soc_dai, div_id: c_int, div: c_int) -> c_int;
    pub fn snd_soc_dai_set_pll(dai: *mut snd_soc_dai, pll_id: c_int, source: c_int, freq_in: u32, freq_out: u32) -> c_int;
    pub fn snd_soc_dai_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: u32) -> c_int;
    pub fn snd_soc_dai_set_bclk_clk(dai: *mut snd_soc_dai, bclk: *mut clk);
    pub fn snd_soc_dai_auto_select_format(rtd: *const snd_soc_pcm_runtime) -> u32;
    pub fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: u32) -> c_int;
    pub fn snd_soc_dai_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: u32, rx_mask: u32, slots: c_int, slot_width: c_int) -> c_int;
    pub fn snd_soc_dai_set_tdm_idle(dai: *mut snd_soc_dai, tx_mask: u32, rx_mask: u32, tx_mode: c_int, rx_mode: c_int) -> c_int;
    pub fn snd_soc_dai_set_channel_map(dai: *mut snd_soc_dai, tx_num: u32, tx_slot: *const u32, rx_num: u32, rx_slot: *const u32) -> c_int;
    pub fn snd_soc_dai_set_tristate(dai: *mut snd_soc_dai, tristate: c_int) -> c_int;
    pub fn snd_soc_dai_prepare(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> c_int;
    pub fn snd_soc_dai_digital_mute(dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int;
    pub fn snd_soc_dai_mute_is_ctrled_at_trigger(dai: *mut snd_soc_dai) -> c_int;
    pub fn snd_soc_dai_get_channel_map(dai: *const snd_soc_dai, tx_num: *mut u32, tx_slot: *mut u32, rx_num: *mut u32, rx_slot: *mut u32) -> c_int;
    pub fn snd_soc_dai_is_dummy(dai: *const snd_soc_dai) -> c_int;
    pub fn snd_soc_dai_hw_params(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int;
    pub fn snd_soc_dai_hw_free(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, rollback: c_int);
    pub fn snd_soc_dai_startup(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> c_int;
    pub fn snd_soc_dai_shutdown(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, rollback: c_int);
    pub fn snd_soc_dai_suspend(dai: *mut snd_soc_dai);
    pub fn snd_soc_dai_resume(dai: *mut snd_soc_dai);
    pub fn snd_soc_dai_compress_new(dai: *mut snd_soc_dai, rtd: *mut snd_soc_pcm_runtime) -> c_int;
    pub fn snd_soc_dai_stream_valid(dai: *const snd_soc_dai, stream: c_int) -> bool;
    pub fn snd_soc_dai_action(dai: *mut snd_soc_dai, stream: c_int, action: c_int);
    pub fn snd_soc_dai_active(dai: *const snd_soc_dai) -> c_int;
    pub fn snd_soc_pcm_dai_probe(rtd: *mut snd_soc_pcm_runtime, order: c_int) -> c_int;
    pub fn snd_soc_pcm_dai_remove(rtd: *mut snd_soc_pcm_runtime, order: c_int) -> c_int;
    pub fn snd_soc_pcm_dai_new(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    pub fn snd_soc_pcm_dai_prepare(substream: *mut snd_pcm_substream) -> c_int;
    pub fn snd_soc_pcm_dai_trigger(substream: *mut snd_pcm_substream, cmd: c_int, rollback: c_int) -> c_int;
    pub fn snd_soc_pcm_dai_delay(substream: *mut snd_pcm_substream, cpu_delay: *mut snd_pcm_sframes_t, codec_delay: *mut snd_pcm_sframes_t);
    pub fn snd_soc_dai_compr_startup(dai: *mut snd_soc_dai, cstream: *mut snd_compr_stream) -> c_int;
    pub fn snd_soc_dai_compr_shutdown(dai: *mut snd_soc_dai, cstream: *mut snd_compr_stream, rollback: c_int);
    pub fn snd_soc_dai_compr_trigger(dai: *mut snd_soc_dai, cstream: *mut snd_compr_stream, cmd: c_int) -> c_int;
    pub fn snd_soc_dai_compr_set_params(dai: *mut snd_soc_dai, cstream: *mut snd_compr_stream, params: *mut snd_compr_params) -> c_int;
    pub fn snd_soc_dai_compr_get_params(dai: *mut snd_soc_dai, cstream: *mut snd_compr_stream, params: *mut snd_codec) -> c_int;
    pub fn snd_soc_dai_compr_ack(dai: *mut snd_soc_dai, cstream: *mut snd_compr_stream, bytes: usize) -> c_int;
    pub fn snd_soc_dai_compr_pointer(dai: *mut snd_soc_dai, cstream: *mut snd_compr_stream, tstamp: *mut snd_compr_tstamp64) -> c_int;
    pub fn snd_soc_dai_compr_set_metadata(dai: *mut snd_soc_dai, cstream: *mut snd_compr_stream, metadata: *mut snd_compr_metadata) -> c_int;
    pub fn snd_soc_dai_compr_get_metadata(dai: *mut snd_soc_dai, cstream: *mut snd_compr_stream, metadata: *mut snd_compr_metadata) -> c_int;
    pub fn snd_soc_dai_name_get(dai: *const snd_soc_dai) -> *const c_char;
}

#[repr(C)] pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub compress_new: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai,c_int,u32,c_int)->c_int>, pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_dai,c_int,c_int,u32,u32)->c_int>, pub set_clkdiv: Option<unsafe extern "C" fn(*mut snd_soc_dai,c_int,c_int)->c_int>, pub set_bclk_ratio: Option<unsafe extern "C" fn(*mut snd_soc_dai,u32)->c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai,u32)->c_int>, pub xlate_tdm_slot_mask: Option<unsafe extern "C" fn(u32,*mut u32,*mut u32)->c_int>, pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai,u32,u32,c_int,c_int)->c_int>, pub set_tdm_idle: Option<unsafe extern "C" fn(*mut snd_soc_dai,u32,u32,c_int,c_int)->c_int>, pub set_channel_map: Option<unsafe extern "C" fn(*mut snd_soc_dai,u32,*const u32,u32,*const u32)->c_int>, pub get_channel_map: Option<unsafe extern "C" fn(*const snd_soc_dai,*mut u32,*mut u32,*mut u32,*mut u32)->c_int>, pub set_tristate: Option<unsafe extern "C" fn(*mut snd_soc_dai,c_int)->c_int>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai,*mut c_void,c_int)->c_int>, pub get_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai,c_int)->*mut c_void>, pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai,c_int,c_int)->c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream,*mut snd_soc_dai)->c_int>, pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream,*mut snd_soc_dai)>, pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream,*mut snd_pcm_hw_params,*mut snd_soc_dai)->c_int>, pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream,*mut snd_soc_dai)>, pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream,*mut snd_soc_dai)->c_int>, pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream,c_int,*mut snd_soc_dai)->c_int>, pub delay: Option<unsafe extern "C" fn(*mut snd_pcm_substream,*mut snd_soc_dai)->snd_pcm_sframes_t>,
    pub auto_selectable_formats: *const u64, pub num_auto_selectable_formats: c_int, pub probe_order: c_int, pub remove_order: c_int, pub no_capture_mute: u32, pub mute_unmute_on_trigger: u32,
}
#[repr(C)] pub struct snd_soc_cdai_ops { pub startup: Option<unsafe extern "C" fn(*mut snd_compr_stream,*mut snd_soc_dai)->c_int>, pub shutdown: Option<unsafe extern "C" fn(*mut snd_compr_stream,*mut snd_soc_dai)>, pub set_params: Option<unsafe extern "C" fn(*mut snd_compr_stream,*mut snd_compr_params,*mut snd_soc_dai)->c_int>, pub get_params: Option<unsafe extern "C" fn(*mut snd_compr_stream,*mut snd_codec,*mut snd_soc_dai)->c_int>, pub set_metadata: Option<unsafe extern "C" fn(*mut snd_compr_stream,*mut snd_compr_metadata,*mut snd_soc_dai)->c_int>, pub get_metadata: Option<unsafe extern "C" fn(*mut snd_compr_stream,*mut snd_compr_metadata,*mut snd_soc_dai)->c_int>, pub trigger: Option<unsafe extern "C" fn(*mut snd_compr_stream,c_int,*mut snd_soc_dai)->c_int>, pub pointer: Option<unsafe extern "C" fn(*mut snd_compr_stream,*mut snd_compr_tstamp64,*mut snd_soc_dai)->c_int>, pub ack: Option<unsafe extern "C" fn(*mut snd_compr_stream,usize,*mut snd_soc_dai)->c_int> }
#[repr(C)] pub struct snd_soc_dai_stream { pub widget: *mut snd_soc_dapm_widget, pub active: u32, pub tdm_mask: u32, pub dma_data: *mut c_void }
#[repr(C)] pub struct snd_soc_dai_driver { pub name:*const c_char, pub id:u32, pub base:u32, pub dobj:snd_soc_dobj, pub dai_args:*const of_phandle_args, pub ops:*const snd_soc_dai_ops, pub cops:*const snd_soc_cdai_ops, pub capture:snd_soc_pcm_stream, pub playback:snd_soc_pcm_stream, pub symmetric_rate:u32, pub symmetric_channels:u32, pub symmetric_sample_bits:u32 }
#[repr(C)] pub struct snd_soc_dai { pub name:*const c_char, pub id:c_int, pub dev:*mut device, pub driver:*mut snd_soc_dai_driver, pub stream:[snd_soc_dai_stream; 2], pub symmetric_rate:u32, pub symmetric_channels:u32, pub symmetric_sample_bits:u32, pub bclk:*mut clk, pub bclk_ratio:u32, pub component:*mut snd_soc_component, pub list:list_head, pub mark_startup:*mut snd_pcm_substream, pub mark_hw_params:*mut snd_pcm_substream, pub mark_trigger:*mut snd_pcm_substream, pub mark_compr_startup:*mut snd_compr_stream, pub probed:u32, pub priv_:*mut c_void }

extern "C" { pub fn snd_soc_dai_action(dai:*mut snd_soc_dai, stream:c_int, action:c_int); }
#[inline] pub unsafe fn snd_soc_dai_activate(dai:*mut snd_soc_dai, stream:c_int) { snd_soc_dai_action(dai,stream,1) }
#[inline] pub unsafe fn snd_soc_dai_deactivate(dai:*mut snd_soc_dai, stream:c_int) { snd_soc_dai_action(dai,stream,-1) }
#[inline] pub unsafe fn snd_soc_dai_get_pcm_stream(dai:*const snd_soc_dai, stream:c_int)->*const snd_soc_pcm_stream { if stream==SNDRV_PCM_STREAM_PLAYBACK { &(*(*dai).driver).playback } else { &(*(*dai).driver).capture } }
#[inline] pub unsafe fn snd_soc_dai_get_widget(dai:*mut snd_soc_dai, stream:usize)->*mut snd_soc_dapm_widget { (*dai).stream[stream].widget }
#[inline] pub unsafe fn snd_soc_dai_set_widget(dai:*mut snd_soc_dai, stream:usize, widget:*mut snd_soc_dapm_widget) { (*dai).stream[stream].widget=widget }
#[inline] pub unsafe fn snd_soc_dai_dma_data_get(dai:*const snd_soc_dai, stream:usize)->*mut c_void { (*dai).stream[stream].dma_data }
#[inline] pub unsafe fn snd_soc_dai_dma_data_set(dai:*mut snd_soc_dai, stream:usize, data:*mut c_void) { (*dai).stream[stream].dma_data=data }
#[inline] pub unsafe fn snd_soc_dai_init_dma_data(dai:*mut snd_soc_dai, playback:*mut c_void, capture:*mut c_void) { snd_soc_dai_dma_data_set(dai,0,playback); snd_soc_dai_dma_data_set(dai,1,capture) }
#[inline] pub unsafe fn snd_soc_dai_tdm_mask_get(dai:*const snd_soc_dai, stream:usize)->u32 { (*dai).stream[stream].tdm_mask }
#[inline] pub unsafe fn snd_soc_dai_tdm_mask_set(dai:*mut snd_soc_dai, stream:usize, mask:u32) { (*dai).stream[stream].tdm_mask=mask }
#[inline] pub unsafe fn snd_soc_dai_stream_active(dai:*const snd_soc_dai, stream:usize)->u32 { (*dai).stream[stream].active }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
