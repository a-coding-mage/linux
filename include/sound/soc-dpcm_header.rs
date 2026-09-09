/* SPDX-License-Identifier: GPL-2.0
 *
 * linux/sound/soc-dpcm.h -- ALSA SoC Dynamic PCM Support
 *
 * Author:              Liam Girdwood <lrg@ti.com>
 */

// Dependencies supplied by other translation units:
// linux/slab.h, linux/list.h, and sound/pcm.h.

use core::ffi::c_int;

#[repr(C)]
pub struct snd_soc_pcm_runtime;
#[repr(C)]
pub struct snd_soc_card;
#[repr(C)]
pub struct snd_soc_dapm_widget_list;
#[repr(C)]
pub struct snd_soc_dapm_widget;
#[repr(C)]
pub struct list_head;
#[repr(C)]
pub struct dentry;
#[repr(C)]
pub struct snd_pcm_substream;
#[repr(C)]
pub struct snd_pcm_hw_params;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_dpcm_update {
    SND_SOC_DPCM_UPDATE_NO = 0,
    SND_SOC_DPCM_UPDATE_BE,
    SND_SOC_DPCM_UPDATE_FE,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_dpcm_link_state {
    SND_SOC_DPCM_LINK_STATE_NEW = 0, // newly created link
    SND_SOC_DPCM_LINK_STATE_FREE,     // link to be dismantled
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_dpcm_state {
    SND_SOC_DPCM_STATE_NEW = 0,
    SND_SOC_DPCM_STATE_OPEN,
    SND_SOC_DPCM_STATE_HW_PARAMS,
    SND_SOC_DPCM_STATE_PREPARE,
    SND_SOC_DPCM_STATE_START,
    SND_SOC_DPCM_STATE_STOP,
    SND_SOC_DPCM_STATE_PAUSED,
    SND_SOC_DPCM_STATE_SUSPEND,
    SND_SOC_DPCM_STATE_HW_FREE,
    SND_SOC_DPCM_STATE_CLOSE,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_dpcm_trigger {
    SND_SOC_DPCM_TRIGGER_PRE = 0,
    SND_SOC_DPCM_TRIGGER_POST,
}

#[repr(C)]
pub struct snd_soc_dpcm {
    pub be: *mut snd_soc_pcm_runtime,
    pub fe: *mut snd_soc_pcm_runtime,
    pub state: snd_soc_dpcm_link_state,
    pub list_be: list_head,
    pub list_fe: list_head,
    #[cfg(CONFIG_DEBUG_FS)]
    pub debugfs_state: *mut dentry,
}

#[repr(C)]
pub struct snd_soc_dpcm_runtime {
    pub be_clients: list_head,
    pub fe_clients: list_head,
    pub users: c_int,
    pub hw_params: snd_pcm_hw_params,
    pub runtime_update: snd_soc_dpcm_update,
    pub state: snd_soc_dpcm_state,
    pub trigger_pending: c_int,
    pub be_start: c_int,
    pub be_pause: c_int,
    pub fe_pause: bool,
}

// Dynamic PCM Frontend -> Backend link management and traversal macros.
#[macro_export]
macro_rules! for_each_dpcm_fe {
    ($be:expr, $stream:expr, $_dpcm:ident) => {
        list_for_each_entry!($_dpcm, &mut (*$be).dpcm[$stream].fe_clients, list_fe)
    };
}
#[macro_export]
macro_rules! for_each_dpcm_be {
    ($fe:expr, $stream:expr, $_dpcm:ident) => {
        list_for_each_entry!($_dpcm, &mut (*$fe).dpcm[$stream].be_clients, list_be)
    };
}
#[macro_export]
macro_rules! for_each_dpcm_be_safe {
    ($fe:expr, $stream:expr, $_dpcm:ident, $__dpcm:ident) => {
        list_for_each_entry_safe!($_dpcm, $__dpcm, &mut (*$fe).dpcm[$stream].be_clients, list_be)
    };
}
#[macro_export]
macro_rules! for_each_dpcm_be_rollback {
    ($fe:expr, $stream:expr, $_dpcm:ident) => {
        list_for_each_entry_continue_reverse!($_dpcm, &mut (*$fe).dpcm[$stream].be_clients, list_be)
    };
}

extern "C" {
    pub fn snd_soc_dpcm_get_substream(be: *mut snd_soc_pcm_runtime, stream: c_int) -> *mut snd_pcm_substream;
    pub fn snd_soc_dpcm_runtime_update(card: *mut snd_soc_card) -> c_int;
    #[cfg(CONFIG_DEBUG_FS)]
    pub fn soc_dpcm_debugfs_add(rtd: *mut snd_soc_pcm_runtime);
    pub fn dpcm_path_get(fe: *mut snd_soc_pcm_runtime, stream: c_int, list_: *mut *mut snd_soc_dapm_widget_list) -> c_int;
    pub fn dpcm_path_put(list: *mut *mut snd_soc_dapm_widget_list);
    pub fn dpcm_add_paths(fe: *mut snd_soc_pcm_runtime, stream: c_int, list_: *mut *mut snd_soc_dapm_widget_list) -> c_int;
    pub fn dpcm_be_dai_startup(fe: *mut snd_soc_pcm_runtime, stream: c_int) -> c_int;
    pub fn dpcm_be_dai_stop(fe: *mut snd_soc_pcm_runtime, stream: c_int, do_hw_free: c_int, last: *mut snd_soc_dpcm);
    pub fn dpcm_be_disconnect(fe: *mut snd_soc_pcm_runtime, stream: c_int);
    pub fn dpcm_clear_pending_state(fe: *mut snd_soc_pcm_runtime, stream: c_int);
    pub fn dpcm_be_dai_hw_free(fe: *mut snd_soc_pcm_runtime, stream: c_int);
    pub fn dpcm_be_dai_hw_params(fe: *mut snd_soc_pcm_runtime, tream: c_int) -> c_int;
    pub fn dpcm_be_dai_trigger(fe: *mut snd_soc_pcm_runtime, stream: c_int, cmd: c_int) -> c_int;
    pub fn dpcm_be_dai_prepare(fe: *mut snd_soc_pcm_runtime, stream: c_int) -> c_int;
    pub fn dpcm_dapm_stream_event(fe: *mut snd_soc_pcm_runtime, dir: c_int, event: c_int);
    pub fn dpcm_end_walk_at_be(widget: *mut snd_soc_dapm_widget, dir: c_int) -> bool;
    pub fn widget_in_list(list: *mut snd_soc_dapm_widget_list, widget: *mut snd_soc_dapm_widget) -> c_int;
}

#[macro_export]
macro_rules! dpcm_be_dai_startup_rollback { ($fe:expr, $stream:expr, $last:expr) => { dpcm_be_dai_stop!($fe, $stream, 0, $last) }; }
#[macro_export]
macro_rules! dpcm_be_dai_startup_unwind { ($fe:expr, $stream:expr) => { dpcm_be_dai_stop!($fe, $stream, 0, core::ptr::null_mut()) }; }
#[macro_export]
macro_rules! dpcm_be_dai_shutdown { ($fe:expr, $stream:expr) => { dpcm_be_dai_stop!($fe, $stream, 1, core::ptr::null_mut()) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
