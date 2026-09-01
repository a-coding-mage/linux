/* SPDX-License-Identifier: GPL-2.0 */

/* TRACE_SYSTEM snd_pcm */

/* Dependency intent from C header:
 * #include <linux/tracepoint.h>
 * Trace definitions are emitted by <trace/define_trace.h> with
 * TRACE_INCLUDE_PATH . and TRACE_INCLUDE_FILE pcm_param_trace.
 */

use core::ffi::{c_char, c_int, c_uint};
use core::ptr;

pub const TRACE_SYSTEM: &[u8] = b"snd_pcm\0";
pub const TRACE_INCLUDE_PATH: &[u8] = b".\0";
pub const TRACE_INCLUDE_FILE: &[u8] = b"pcm_param_trace\0";

pub type snd_pcm_hw_param_t = c_int;

unsafe extern "C" {
    pub static SNDRV_PCM_HW_PARAM_ACCESS: snd_pcm_hw_param_t;
    pub static SNDRV_PCM_HW_PARAM_FORMAT: snd_pcm_hw_param_t;
    pub static SNDRV_PCM_HW_PARAM_SUBFORMAT: snd_pcm_hw_param_t;
    pub static SNDRV_PCM_HW_PARAM_SAMPLE_BITS: snd_pcm_hw_param_t;
    pub static SNDRV_PCM_HW_PARAM_FRAME_BITS: snd_pcm_hw_param_t;
    pub static SNDRV_PCM_HW_PARAM_CHANNELS: snd_pcm_hw_param_t;
    pub static SNDRV_PCM_HW_PARAM_RATE: snd_pcm_hw_param_t;
    pub static SNDRV_PCM_HW_PARAM_PERIOD_TIME: snd_pcm_hw_param_t;
    pub static SNDRV_PCM_HW_PARAM_PERIOD_SIZE: snd_pcm_hw_param_t;
    pub static SNDRV_PCM_HW_PARAM_PERIOD_BYTES: snd_pcm_hw_param_t;
    pub static SNDRV_PCM_HW_PARAM_PERIODS: snd_pcm_hw_param_t;
    pub static SNDRV_PCM_HW_PARAM_BUFFER_TIME: snd_pcm_hw_param_t;
    pub static SNDRV_PCM_HW_PARAM_BUFFER_SIZE: snd_pcm_hw_param_t;
    pub static SNDRV_PCM_HW_PARAM_BUFFER_BYTES: snd_pcm_hw_param_t;
    pub static SNDRV_PCM_HW_PARAM_TICK_TIME: snd_pcm_hw_param_t;
}

#[repr(C)]
pub struct trace_print_flags {
    pub mask: c_uint,
    pub name: *const c_char,
}

/* C macro: #define HW_PARAM_ENTRY(param) {SNDRV_PCM_HW_PARAM_##param, #param} */
#[inline]
pub unsafe fn HW_PARAM_ENTRY(mask: snd_pcm_hw_param_t, name: *const c_char) -> trace_print_flags {
    trace_print_flags {
        mask: mask as c_uint,
        name,
    }
}

/* C macro: #define hw_param_labels ... */
pub unsafe fn hw_param_labels() -> [trace_print_flags; 15] {
    [
        HW_PARAM_ENTRY(SNDRV_PCM_HW_PARAM_ACCESS, c"ACCESS".as_ptr()),
        HW_PARAM_ENTRY(SNDRV_PCM_HW_PARAM_FORMAT, c"FORMAT".as_ptr()),
        HW_PARAM_ENTRY(SNDRV_PCM_HW_PARAM_SUBFORMAT, c"SUBFORMAT".as_ptr()),
        HW_PARAM_ENTRY(SNDRV_PCM_HW_PARAM_SAMPLE_BITS, c"SAMPLE_BITS".as_ptr()),
        HW_PARAM_ENTRY(SNDRV_PCM_HW_PARAM_FRAME_BITS, c"FRAME_BITS".as_ptr()),
        HW_PARAM_ENTRY(SNDRV_PCM_HW_PARAM_CHANNELS, c"CHANNELS".as_ptr()),
        HW_PARAM_ENTRY(SNDRV_PCM_HW_PARAM_RATE, c"RATE".as_ptr()),
        HW_PARAM_ENTRY(SNDRV_PCM_HW_PARAM_PERIOD_TIME, c"PERIOD_TIME".as_ptr()),
        HW_PARAM_ENTRY(SNDRV_PCM_HW_PARAM_PERIOD_SIZE, c"PERIOD_SIZE".as_ptr()),
        HW_PARAM_ENTRY(SNDRV_PCM_HW_PARAM_PERIOD_BYTES, c"PERIOD_BYTES".as_ptr()),
        HW_PARAM_ENTRY(SNDRV_PCM_HW_PARAM_PERIODS, c"PERIODS".as_ptr()),
        HW_PARAM_ENTRY(SNDRV_PCM_HW_PARAM_BUFFER_TIME, c"BUFFER_TIME".as_ptr()),
        HW_PARAM_ENTRY(SNDRV_PCM_HW_PARAM_BUFFER_SIZE, c"BUFFER_SIZE".as_ptr()),
        HW_PARAM_ENTRY(SNDRV_PCM_HW_PARAM_BUFFER_BYTES, c"BUFFER_BYTES".as_ptr()),
        HW_PARAM_ENTRY(SNDRV_PCM_HW_PARAM_TICK_TIME, c"TICK_TIME".as_ptr()),
    ]
}

#[repr(C)]
pub struct snd_card {
    pub number: c_int,
}

#[repr(C)]
pub struct snd_pcm {
    pub card: *mut snd_card,
    pub device: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_constraints {
    pub rules_num: c_int,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw_constraints: snd_pcm_hw_constraints,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub pcm: *mut snd_pcm,
    pub number: c_int,
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_mask {
    pub bits: [u32; 8],
}

#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
    pub openmin: c_uint,
    pub openmax: c_uint,
    pub integer: c_uint,
    pub empty: c_uint,
}

/* TRACE_EVENT(hw_mask_param, ...) */
#[repr(C)]
pub struct trace_event_raw_hw_mask_param {
    pub card: c_int,
    pub device: c_int,
    pub subdevice: c_int,
    pub direction: c_int,
    pub type_: snd_pcm_hw_param_t,
    pub index: c_int,
    pub total: c_int,
    pub prev_bits: [u32; 8],
    pub curr_bits: [u32; 8],
}

/* TP_fast_assign body for hw_mask_param. */
pub unsafe fn trace_hw_mask_param_fast_assign(
    __entry: *mut trace_event_raw_hw_mask_param,
    substream: *mut snd_pcm_substream,
    type_: snd_pcm_hw_param_t,
    index: c_int,
    prev: *const snd_mask,
    curr: *const snd_mask,
) {
    (*__entry).card = (*(*(*substream).pcm).card).number;
    (*__entry).device = (*(*substream).pcm).device;
    (*__entry).subdevice = (*substream).number;
    (*__entry).direction = (*substream).stream;
    (*__entry).type_ = type_;
    (*__entry).index = index;
    (*__entry).total = (*(*substream).runtime).hw_constraints.rules_num;
    ptr::copy_nonoverlapping((*prev).bits.as_ptr(), (*__entry).prev_bits.as_mut_ptr(), 8);
    ptr::copy_nonoverlapping((*curr).bits.as_ptr(), (*__entry).curr_bits.as_mut_ptr(), 8);
}

pub const HW_MASK_PARAM_TP_PRINTK_FORMAT: &[u8] =
    b"pcmC%dD%d%s:%d %03d/%03d %s %08x%08x%08x%08x %08x%08x%08x%08x\0";

/* TRACE_EVENT(hw_interval_param, ...) */
#[repr(C)]
pub struct trace_event_raw_hw_interval_param {
    pub card: c_int,
    pub device: c_int,
    pub subdevice: c_int,
    pub direction: c_int,
    pub type_: snd_pcm_hw_param_t,
    pub index: c_int,
    pub total: c_int,
    pub prev_min: c_uint,
    pub prev_max: c_uint,
    pub prev_openmin: c_uint,
    pub prev_openmax: c_uint,
    pub prev_integer: c_uint,
    pub prev_empty: c_uint,
    pub curr_min: c_uint,
    pub curr_max: c_uint,
    pub curr_openmin: c_uint,
    pub curr_openmax: c_uint,
    pub curr_integer: c_uint,
    pub curr_empty: c_uint,
}

/* TP_fast_assign body for hw_interval_param. */
pub unsafe fn trace_hw_interval_param_fast_assign(
    __entry: *mut trace_event_raw_hw_interval_param,
    substream: *mut snd_pcm_substream,
    type_: snd_pcm_hw_param_t,
    index: c_int,
    prev: *const snd_interval,
    curr: *const snd_interval,
) {
    (*__entry).card = (*(*(*substream).pcm).card).number;
    (*__entry).device = (*(*substream).pcm).device;
    (*__entry).subdevice = (*substream).number;
    (*__entry).direction = (*substream).stream;
    (*__entry).type_ = type_;
    (*__entry).index = index;
    (*__entry).total = (*(*substream).runtime).hw_constraints.rules_num;
    (*__entry).prev_min = (*prev).min;
    (*__entry).prev_max = (*prev).max;
    (*__entry).prev_openmin = (*prev).openmin;
    (*__entry).prev_openmax = (*prev).openmax;
    (*__entry).prev_integer = (*prev).integer;
    (*__entry).prev_empty = (*prev).empty;
    (*__entry).curr_min = (*curr).min;
    (*__entry).curr_max = (*curr).max;
    (*__entry).curr_openmin = (*curr).openmin;
    (*__entry).curr_openmax = (*curr).openmax;
    (*__entry).curr_integer = (*curr).integer;
    (*__entry).curr_empty = (*curr).empty;
}

pub const HW_INTERVAL_PARAM_TP_PRINTK_FORMAT: &[u8] =
    b"pcmC%dD%d%s:%d %03d/%03d %s %d %d %s%u %u%s %d %d %s%u %u%s\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
