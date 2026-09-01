/* SPDX-License-Identifier: GPL-2.0 */

// Original C trace metadata:
// #undef TRACE_SYSTEM
// #define TRACE_SYSTEM snd_pcm
// #define TRACE_INCLUDE_FILE pcm_trace
// Header guard and Linux tracepoint include syntax are C-only.

use core::ffi::{c_char, c_uint};

pub const TRACE_SYSTEM: &str = "snd_pcm";
pub const TRACE_INCLUDE_FILE: &str = "pcm_trace";
pub const TRACE_INCLUDE_PATH: &str = ".";

pub type SndPcmUframesT = snd_pcm_uframes_t;

unsafe extern "C" {
    pub type snd_pcm_substream;
    pub type snd_pcm_runtime;
    pub type snd_pcm_uframes_t;

    pub static SNDRV_PCM_STREAM_PLAYBACK: c_uint;

    pub fn snd_pcm_capture_avail(runtime: *mut snd_pcm_runtime) -> snd_pcm_uframes_t;
    pub fn snd_pcm_playback_avail(runtime: *mut snd_pcm_runtime) -> snd_pcm_uframes_t;
}

// Field access for struct snd_pcm_substream and its nested ALSA structures is
// supplied by the translated dependency that defines these C-layout types.
unsafe extern "C" {
    pub fn snd_pcm_substream_pcm_card_number(substream: *mut snd_pcm_substream) -> c_uint;
    pub fn snd_pcm_substream_pcm_device(substream: *mut snd_pcm_substream) -> c_uint;
    pub fn snd_pcm_substream_number(substream: *mut snd_pcm_substream) -> c_uint;
    pub fn snd_pcm_substream_stream(substream: *mut snd_pcm_substream) -> c_uint;
    pub fn snd_pcm_substream_runtime(substream: *mut snd_pcm_substream) -> *mut snd_pcm_runtime;
    pub fn snd_pcm_runtime_period_size(runtime: *mut snd_pcm_runtime) -> snd_pcm_uframes_t;
    pub fn snd_pcm_runtime_buffer_size(runtime: *mut snd_pcm_runtime) -> snd_pcm_uframes_t;
    pub fn snd_pcm_runtime_status_hw_ptr(runtime: *mut snd_pcm_runtime) -> snd_pcm_uframes_t;
    pub fn snd_pcm_runtime_hw_ptr_base(runtime: *mut snd_pcm_runtime) -> snd_pcm_uframes_t;
}

#[repr(C)]
pub struct trace_event_raw_hwptr {
    pub in_interrupt: bool,
    pub card: c_uint,
    pub device: c_uint,
    pub number: c_uint,
    pub stream: c_uint,
    pub pos: snd_pcm_uframes_t,
    pub period_size: snd_pcm_uframes_t,
    pub buffer_size: snd_pcm_uframes_t,
    pub old_hw_ptr: snd_pcm_uframes_t,
    pub hw_ptr_base: snd_pcm_uframes_t,
}

pub const TRACE_EVENT_HWPTR_FMT: &str =
    "pcmC%dD%d%s/sub%d: %s: pos=%lu, old=%lu, base=%lu, period=%lu, buf=%lu";

pub unsafe fn trace_event_hwptr_fast_assign(
    entry: *mut trace_event_raw_hwptr,
    substream: *mut snd_pcm_substream,
    pos: snd_pcm_uframes_t,
    irq: bool,
) {
    unsafe {
        (*entry).in_interrupt = irq;
        (*entry).card = snd_pcm_substream_pcm_card_number(substream);
        (*entry).device = snd_pcm_substream_pcm_device(substream);
        (*entry).number = snd_pcm_substream_number(substream);
        (*entry).stream = snd_pcm_substream_stream(substream);
        (*entry).pos = pos;
        let runtime = snd_pcm_substream_runtime(substream);
        (*entry).period_size = snd_pcm_runtime_period_size(runtime);
        (*entry).buffer_size = snd_pcm_runtime_buffer_size(runtime);
        (*entry).old_hw_ptr = snd_pcm_runtime_status_hw_ptr(runtime);
        (*entry).hw_ptr_base = snd_pcm_runtime_hw_ptr_base(runtime);
    }
}

#[repr(C)]
pub struct trace_event_raw_xrun {
    pub card: c_uint,
    pub device: c_uint,
    pub number: c_uint,
    pub stream: c_uint,
    pub period_size: snd_pcm_uframes_t,
    pub buffer_size: snd_pcm_uframes_t,
    pub old_hw_ptr: snd_pcm_uframes_t,
    pub hw_ptr_base: snd_pcm_uframes_t,
}

pub const TRACE_EVENT_XRUN_FMT: &str =
    "pcmC%dD%d%s/sub%d: XRUN: old=%lu, base=%lu, period=%lu, buf=%lu";

pub unsafe fn trace_event_xrun_fast_assign(
    entry: *mut trace_event_raw_xrun,
    substream: *mut snd_pcm_substream,
) {
    unsafe {
        (*entry).card = snd_pcm_substream_pcm_card_number(substream);
        (*entry).device = snd_pcm_substream_pcm_device(substream);
        (*entry).number = snd_pcm_substream_number(substream);
        (*entry).stream = snd_pcm_substream_stream(substream);
        let runtime = snd_pcm_substream_runtime(substream);
        (*entry).period_size = snd_pcm_runtime_period_size(runtime);
        (*entry).buffer_size = snd_pcm_runtime_buffer_size(runtime);
        (*entry).old_hw_ptr = snd_pcm_runtime_status_hw_ptr(runtime);
        (*entry).hw_ptr_base = snd_pcm_runtime_hw_ptr_base(runtime);
    }
}

#[repr(C)]
pub struct trace_event_raw_hw_ptr_error {
    pub card: c_uint,
    pub device: c_uint,
    pub number: c_uint,
    pub stream: c_uint,
    pub reason: *const c_char,
}

pub const TRACE_EVENT_HW_PTR_ERROR_FMT: &str = "pcmC%dD%d%s/sub%d: ERROR: %s";

pub unsafe fn trace_event_hw_ptr_error_fast_assign(
    entry: *mut trace_event_raw_hw_ptr_error,
    substream: *mut snd_pcm_substream,
    why: *const c_char,
) {
    unsafe {
        (*entry).card = snd_pcm_substream_pcm_card_number(substream);
        (*entry).device = snd_pcm_substream_pcm_device(substream);
        (*entry).number = snd_pcm_substream_number(substream);
        (*entry).stream = snd_pcm_substream_stream(substream);
        (*entry).reason = why;
    }
}

#[repr(C)]
pub struct trace_event_raw_applptr {
    pub card: c_uint,
    pub device: c_uint,
    pub number: c_uint,
    pub stream: c_uint,
    pub prev: snd_pcm_uframes_t,
    pub curr: snd_pcm_uframes_t,
    pub avail: snd_pcm_uframes_t,
    pub period_size: snd_pcm_uframes_t,
    pub buffer_size: snd_pcm_uframes_t,
}

pub const TRACE_EVENT_APPLPTR_FMT: &str =
    "pcmC%dD%d%s/sub%d: prev=%lu, curr=%lu, avail=%lu, period=%lu, buf=%lu";

pub unsafe fn trace_event_applptr_fast_assign(
    entry: *mut trace_event_raw_applptr,
    substream: *mut snd_pcm_substream,
    prev: snd_pcm_uframes_t,
    curr: snd_pcm_uframes_t,
) {
    unsafe {
        (*entry).card = snd_pcm_substream_pcm_card_number(substream);
        (*entry).device = snd_pcm_substream_pcm_device(substream);
        (*entry).number = snd_pcm_substream_number(substream);
        (*entry).stream = snd_pcm_substream_stream(substream);
        (*entry).prev = prev;
        (*entry).curr = curr;
        let runtime = snd_pcm_substream_runtime(substream);
        (*entry).avail = if (*entry).stream != 0 {
            snd_pcm_capture_avail(runtime)
        } else {
            snd_pcm_playback_avail(runtime)
        };
        (*entry).period_size = snd_pcm_runtime_period_size(runtime);
        (*entry).buffer_size = snd_pcm_runtime_buffer_size(runtime);
    }
}

// Original C footer:
// #undef TRACE_INCLUDE_PATH
// #define TRACE_INCLUDE_PATH .
// #include <trace/define_trace.h>

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
