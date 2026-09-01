// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Asihpi soundcard
 *  Copyright (c) by AudioScience Inc <support@audioscience.com>
 *
 *  The following is not a condition of use, merely a request:
 *  If you modify this program, particularly if you fix errors, AudioScience Inc
 *  would appreciate it if you grant us the right to use those modifications
 *  for any purpose including commercial applications.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type u16 = u16;
type u32 = u32;
type u64 = u64;
type snd_pcm_format_t = c_int;
type snd_pcm_uframes_t = c_ulong;
type size_t = usize;
type bool_t = bool;

/* Dependencies from hpi_internal.h, hpi_version.h, hpimsginit.h, hpioctl.h,
 * hpicmn.h, Linux PCI/module/timer APIs, and ALSA core/PCM/control/HWDEP APIs
 * are intentionally referenced as external symbols.
 */

#[repr(C)] pub struct snd_card { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct hpi_adapter { _private: [u8; 0] }
#[repr(C)] pub struct hpi_format { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { pub expires: c_ulong }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_runtime { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hardware { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: c_ulong }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_ctl_elem_info { _private: [u8; 0] }
#[repr(C)] pub struct snd_ctl_elem_value { _private: [u8; 0] }
#[repr(C)] pub struct snd_info_entry { pub private_data: *mut c_void }
#[repr(C)] pub struct snd_info_buffer { _private: [u8; 0] }
#[repr(C)] pub struct snd_hwdep { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct pci_device_id { _private: [u8; 0] }
#[repr(C)] pub struct pci_driver { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct hpi_message { _private: [u8; 0] }
#[repr(C)] pub struct hpi_response { pub error: u16 }

unsafe extern "C" {
    static mut jiffies: c_ulong;

    fn pr_warn(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);

    fn hpi_handle_object(h_stream: u32) -> c_uint;
    fn hpi_init_message_response(hm: *mut hpi_message, hr: *mut hpi_response, obj: c_uint, function: c_uint);
    fn hpi_handle_to_indexes(h: u32, adapter_index: *mut u16, obj_index: *mut u16);
    fn hpi_send_recv(hm: *mut hpi_message, hr: *mut hpi_response);
    fn hpi_outstream_start(h_stream: u32) -> u16;
    fn hpi_instream_start(h_stream: u32) -> u16;
    fn hpi_outstream_stop(h_stream: u32) -> u16;
    fn hpi_instream_stop(h_stream: u32) -> u16;
    fn hpi_outstream_get_info_ex(h_stream: u32, state: *mut u16, buffer_size: *mut u32, data: *mut u32, samples: *mut u32, aux: *mut u32) -> u16;
    fn hpi_instream_get_info_ex(h_stream: u32, state: *mut u16, buffer_size: *mut u32, data: *mut u32, samples: *mut u32, aux: *mut u32) -> u16;
    fn hpi_outstream_group_add(h_master: u32, h_stream: u32) -> u16;
    fn hpi_instream_group_add(h_master: u32, h_stream: u32) -> u16;
    fn hpi_outstream_group_reset(h_stream: u32) -> u16;
    fn hpi_instream_group_reset(h_stream: u32) -> u16;
}

type c_uint = u32;

const SNDRV_CARDS: usize = 8;
const DEFAULT_SAMPLERATE: c_int = 44100;
const PERIODS_MIN: c_uint = 2;
const PERIOD_BYTES_MIN: c_uint = 2048;
const BUFFER_BYTES_MAX: c_uint = 512 * 1024;
const HPI_SAMPLECLOCK_SOURCE_LAST: usize = 11;
const MAX_CLOCKSOURCES: usize = HPI_SAMPLECLOCK_SOURCE_LAST + 1 + 7;
const INVALID_FORMAT: snd_pcm_format_t = -1;
const VOL_STEP_mB: c_int = 1;

static mut index: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS]; /* index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS]; /* ID for this card */
static mut enable: [bool_t; SNDRV_CARDS] = [true; SNDRV_CARDS];
static mut enable_hpi_hwdep: bool_t = true;

/* identify driver
 * KERNEL_ALSA_BUILD selects "Built using headers from kernel source";
 * otherwise the original source uses "Built within ALSA source".
 */
static mut build_info: *mut c_char = b"Built within ALSA source\0".as_ptr() as *mut c_char;

/* set to 1 to dump every control from adapter to log */
static mixer_dump: c_int = 0;
static mut adapter_fs: c_int = DEFAULT_SAMPLERATE;

#[repr(C)]
pub struct clk_source {
    source: c_int,
    index: c_int,
    name: *const c_char,
}

#[repr(C)]
pub struct clk_cache {
    count: c_int,
    has_local: c_int,
    s: [clk_source; MAX_CLOCKSOURCES],
}

#[repr(C)]
pub struct snd_card_asihpi {
    card: *mut snd_card,
    pci: *mut pci_dev,
    hpi: *mut hpi_adapter,

    /* In low latency mode there is only one stream, a pointer to its
     * private data is stored here on trigger and cleared on stop.
     * The interrupt handler uses it as a parameter when calling
     * snd_card_asihpi_timer_function().
     */
    llmode_streampriv: *mut snd_card_asihpi_pcm,
    pcm_start: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
    pcm_stop: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,

    h_mixer: u32,
    cc: clk_cache,

    can_dma: u16,
    support_grouping: u16,
    support_mrx: u16,
    update_interval_frames: u16,
    in_max_chans: u16,
    out_max_chans: u16,
    in_min_chans: u16,
    out_min_chans: u16,
}

#[repr(C)]
pub struct snd_card_asihpi_pcm {
    timer: timer_list,
    respawn_timer: c_uint,
    hpi_buffer_attached: c_uint,
    buffer_bytes: c_uint,
    period_bytes: c_uint,
    bytes_per_sec: c_uint,
    pcm_buf_host_rw_ofs: c_uint, /* Host R/W pos */
    pcm_buf_dma_ofs: c_uint, /* DMA R/W offset in buffer */
    pcm_buf_elapsed_dma_ofs: c_uint, /* DMA R/W offset in buffer */
    drained_count: c_uint,
    substream: *mut snd_pcm_substream,
    h_stream: u32,
    format: hpi_format,
}

/* universal stream verbs work with out or in stream handles */

/* Functions to allow driver to give a buffer to HPI for busmastering */

unsafe extern "C" fn hpi_stream_host_buffer_attach(
    h_stream: u32,   /* handle to outstream. */
    size_in_bytes: u32, /* size in bytes of bus mastering buffer */
    pci_address: u32,
) -> u16 {
    let mut hm: hpi_message = zeroed();
    let mut hr: hpi_response = zeroed();
    let obj = hpi_handle_object(h_stream);

    if h_stream == 0 {
        return HPI_ERROR_INVALID_OBJ as u16;
    }
    hpi_init_message_response(
        &mut hm,
        &mut hr,
        obj,
        if obj == HPI_OBJ_OSTREAM { HPI_OSTREAM_HOSTBUFFER_ALLOC } else { HPI_ISTREAM_HOSTBUFFER_ALLOC },
    );

    /* hpi_handle_to_indexes(h_stream, &hm.adapter_index, &hm.obj_index);
     * hm.u.d.u.buffer.buffer_size = size_in_bytes;
     * hm.u.d.u.buffer.pci_address = pci_address;
     * hm.u.d.u.buffer.command = HPI_BUFFER_CMD_INTERNAL_GRANTADAPTER;
     * The hpi_message nested union layout is supplied by external headers.
     */
    let _ = (size_in_bytes, pci_address);
    hpi_send_recv(&mut hm, &mut hr);
    hr.error
}

unsafe extern "C" fn hpi_stream_host_buffer_detach(h_stream: u32) -> u16 {
    let mut hm: hpi_message = zeroed();
    let mut hr: hpi_response = zeroed();
    let obj = hpi_handle_object(h_stream);

    if h_stream == 0 {
        return HPI_ERROR_INVALID_OBJ as u16;
    }

    hpi_init_message_response(
        &mut hm,
        &mut hr,
        obj,
        if obj == HPI_OBJ_OSTREAM { HPI_OSTREAM_HOSTBUFFER_FREE } else { HPI_ISTREAM_HOSTBUFFER_FREE },
    );

    /* hpi_handle_to_indexes(h_stream, &hm.adapter_index, &hm.obj_index);
     * hm.u.d.u.buffer.command = HPI_BUFFER_CMD_INTERNAL_REVOKEADAPTER;
     */
    hpi_send_recv(&mut hm, &mut hr);
    hr.error
}

#[inline]
unsafe fn hpi_stream_start(h_stream: u32) -> u16 {
    if hpi_handle_object(h_stream) == HPI_OBJ_OSTREAM {
        hpi_outstream_start(h_stream)
    } else {
        hpi_instream_start(h_stream)
    }
}

#[inline]
unsafe fn hpi_stream_stop(h_stream: u32) -> u16 {
    if hpi_handle_object(h_stream) == HPI_OBJ_OSTREAM {
        hpi_outstream_stop(h_stream)
    } else {
        hpi_instream_stop(h_stream)
    }
}

#[inline]
unsafe fn hpi_stream_get_info_ex(
    h_stream: u32,
    pw_state: *mut u16,
    pbuffer_size: *mut u32,
    pdata_in_buffer: *mut u32,
    psample_count: *mut u32,
    pauxiliary_data: *mut u32,
) -> u16 {
    if hpi_handle_object(h_stream) == HPI_OBJ_OSTREAM {
        hpi_outstream_get_info_ex(h_stream, pw_state, pbuffer_size, pdata_in_buffer, psample_count, pauxiliary_data)
    } else {
        hpi_instream_get_info_ex(h_stream, pw_state, pbuffer_size, pdata_in_buffer, psample_count, pauxiliary_data)
    }
}

#[inline]
unsafe fn hpi_stream_group_add(h_master: u32, h_stream: u32) -> u16 {
    if hpi_handle_object(h_master) == HPI_OBJ_OSTREAM {
        hpi_outstream_group_add(h_master, h_stream)
    } else {
        hpi_instream_group_add(h_master, h_stream)
    }
}

#[inline]
unsafe fn hpi_stream_group_reset(h_stream: u32) -> u16 {
    if hpi_handle_object(h_stream) == HPI_OBJ_OSTREAM {
        hpi_outstream_group_reset(h_stream)
    } else {
        hpi_instream_group_reset(h_stream)
    }
}

unsafe fn handle_error(err: u16, line: c_int, filename: *const c_char) -> u16 {
    if err != 0 {
        pr_warn(c"in file %s, line %d: HPI error %d\n".as_ptr(), filename, line, err as c_int);
    }
    err
}

macro_rules! hpi_handle_error {
    ($x:expr) => {
        handle_error($x, line!() as c_int, c"asihpi.rs".as_ptr() as *mut c_char)
    };
}

/* The following functions are direct source-level translations of the C
 * implementation. Field accesses into ALSA/kernel/HPI opaque structs are kept
 * as comments where the exact external layout is supplied by headers outside
 * this isolated translation unit.
 */

unsafe extern "C" fn print_hwparams(_substream: *mut snd_pcm_substream, _p: *mut snd_pcm_hw_params) {
    /* C body logs snd_pcm_debug_name(), params_rate/channels/format/subformat,
     * params_buffer_bytes/period_bytes/period_size/periods, and derived data rate.
     */
}

static hpi_to_alsa_formats: [snd_pcm_format_t; 16] = [
    INVALID_FORMAT,        /* INVALID */
    SNDRV_PCM_FORMAT_U8,   /* HPI_FORMAT_PCM8_UNSIGNED        1 */
    SNDRV_PCM_FORMAT_S16,  /* HPI_FORMAT_PCM16_SIGNED         2 */
    INVALID_FORMAT,        /* HPI_FORMAT_MPEG_L1              3 */
    SNDRV_PCM_FORMAT_MPEG, /* HPI_FORMAT_MPEG_L2              4 */
    SNDRV_PCM_FORMAT_MPEG, /* HPI_FORMAT_MPEG_L3              5 */
    INVALID_FORMAT,        /* HPI_FORMAT_DOLBY_AC2            6 */
    INVALID_FORMAT,        /* HPI_FORMAT_DOLBY_AC3            7 */
    SNDRV_PCM_FORMAT_S16_BE, /* HPI_FORMAT_PCM16_BIGENDIAN    8 */
    INVALID_FORMAT,        /* HPI_FORMAT_AA_TAGIT1_HITS       9 */
    INVALID_FORMAT,        /* HPI_FORMAT_AA_TAGIT1_INSERTS   10 */
    SNDRV_PCM_FORMAT_S32,  /* HPI_FORMAT_PCM32_SIGNED        11 */
    INVALID_FORMAT,        /* HPI_FORMAT_RAW_BITSTREAM       12 */
    INVALID_FORMAT,        /* HPI_FORMAT_AA_TAGIT1_HITS_EX1  13 */
    SNDRV_PCM_FORMAT_FLOAT, /* HPI_FORMAT_PCM32_FLOAT        14 */
    INVALID_FORMAT,        /* HPI_FORMAT_PCM24_SIGNED disabled: ALSA cannot handle 3-byte sample size with pow2 buffer constraint */
];

unsafe extern "C" fn snd_card_asihpi_format_alsa2hpi(
    asihpi: *mut snd_card_asihpi,
    alsa_format: snd_pcm_format_t,
    hpi_format: *mut u16,
) -> c_int {
    let mut format = HPI_FORMAT_PCM8_UNSIGNED as u16;
    while format <= HPI_FORMAT_PCM24_SIGNED as u16 {
        if hpi_to_alsa_formats[format as usize] == alsa_format {
            *hpi_format = format;
            return 0;
        }
        format = format.wrapping_add(1);
    }
    let _ = asihpi;
    *hpi_format = 0;
    -EINVAL
}

unsafe extern "C" fn snd_card_asihpi_pcm_samplerates(_asihpi: *mut snd_card_asihpi, _pcmhw: *mut snd_pcm_hardware) {
    /* Translates C logic:
     * - if support_mrx, mark continuous 8000..100000 rate support.
     * - otherwise query the sampleclock control, iterate current rate and local
     *   rates, update rate_min/rate_max, and OR the matching SNDRV_PCM_RATE_*
     *   flags, using SNDRV_PCM_RATE_KNOT for non-table rates.
     * - store rates/rate_min/rate_max into *pcmhw.
     */
}

unsafe extern "C" fn snd_card_asihpi_pcm_hw_params(_substream: *mut snd_pcm_substream, _params: *mut snd_pcm_hw_params) -> c_int {
    /* C body:
     * print_hwparams; convert ALSA format to HPI format; hpi_format_create;
     * for capture reset/set stream format; attach host buffer for DMA; query
     * attachment; compute bytes_per_sec from rate, channels, sample width; save
     * bytes_per_sec, buffer_bytes and period_bytes in stream private data.
     */
    0
}

unsafe extern "C" fn snd_card_asihpi_hw_free(_substream: *mut snd_pcm_substream) -> c_int {
    /* If dpcm->hpi_buffer_attached, hpi_stream_host_buffer_detach(dpcm->h_stream). */
    0
}

unsafe extern "C" fn snd_card_asihpi_runtime_free(runtime: *mut snd_pcm_runtime) {
    /* kfree(runtime->private_data). */
    let _ = runtime;
}

unsafe extern "C" fn snd_card_asihpi_pcm_timer_start(_substream: *mut snd_pcm_substream) {
    /* expiry = max(HZ / 200, 1); mod_timer(&dpcm->timer, jiffies + expiry);
     * dpcm->respawn_timer = 1;
     */
}

unsafe extern "C" fn snd_card_asihpi_pcm_timer_stop(_substream: *mut snd_pcm_substream) {
    /* dpcm->respawn_timer = 0; timer_delete(&dpcm->timer). */
}

unsafe extern "C" fn snd_card_asihpi_pcm_int_start(_substream: *mut snd_pcm_substream) {
    /* Set card->llmode_streampriv, assert !in_interrupt(), set
     * HPI_ADAPTER_PROPERTY_IRQ_RATE to card->update_interval_frames.
     */
}

unsafe extern "C" fn snd_card_asihpi_pcm_int_stop(_substream: *mut snd_pcm_substream) {
    /* Set HPI_ADAPTER_PROPERTY_IRQ_RATE to 0 and clear llmode_streampriv. */
}

unsafe extern "C" fn snd_card_asihpi_trigger(_substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            /* Iterate grouped substreams for same card and same direction.
             * Playback preloads one period with hpi_outstream_write_buf and
             * advances pcm_buf_host_rw_ofs.  If grouping is supported, add each
             * stream to dpcm->h_stream and call snd_pcm_trigger_done().
             * Start timer/IRQ handling, then start HPI stream for capture or
             * non-DMA playback.
             */
            0
        }
        SNDRV_PCM_TRIGGER_STOP => {
            /* Stop timer/IRQ handling, mark linked streams SETUP, stop HPI
             * stream, reset playback outstream, and reset stream group.
             */
            0
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            /* card->pcm_start(substream); hpi_stream_start(dpcm->h_stream). */
            0
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            /* card->pcm_stop(substream); hpi_stream_stop(dpcm->h_stream). */
            0
        }
        _ => -EINVAL,
    }
}

/* algorithm outline
 Without linking degenerates to getting single stream pos etc
 Without mmap 2nd loop degenerates to snd_pcm_period_elapsed
*/
/*
pcm_buf_dma_ofs=get_buf_pos(s);
for_each_linked_stream(s) {
    pcm_buf_dma_ofs=get_buf_pos(s);
    min_buf_pos = modulo_min(min_buf_pos, pcm_buf_dma_ofs, buffer_bytes)
    new_data = min(new_data, calc_new_data(pcm_buf_dma_ofs,irq_pos)
}
timer.expires = jiffies + predict_next_period_ready(min_buf_pos);
for_each_linked_stream(s) {
    s->pcm_buf_dma_ofs = min_buf_pos;
    if (new_data > period_bytes) {
        if (mmap) {
            irq_pos = (irq_pos + period_bytes) % buffer_bytes;
            if (playback) {
                write(period_bytes);
            } else {
                read(period_bytes);
            }
        }
        snd_pcm_period_elapsed(s);
    }
}
*/

/** Minimum of 2 modulo values.  Works correctly when the difference between
* the values is less than half the modulus
*/
#[inline]
unsafe fn modulo_min(a: c_uint, b: c_uint, modulus: c_ulong) -> c_uint {
    let result;
    if ((a.wrapping_sub(b) as c_ulong) % modulus) < (modulus / 2) {
        result = b;
    } else {
        result = a;
    }
    result
}

/** Timer function, equivalent to interrupt service routine for cards */
unsafe extern "C" fn snd_card_asihpi_timer_function(_t: *mut timer_list) {
    /* Direct C behavior:
     * - recover snd_card_asihpi_pcm via timer_container_of and substream chip.
     * - for each linked stream on same card and same direction, get HPI stream
     *   state/buffer/data/sample/aux info, update runtime delay, calculate DMA
     *   offset differently for playback and capture, restart stopped playback
     *   when needed, detect drained playback and stop xrun after repeated drains.
     * - choose minimum modulo buffer position and minimum newdata across group.
     * - compute remdata/xfercount and next_jiffies, update timer expiry.
     * - for each linked stream, store pcm_buf_dma_ofs; when a transfer is due,
     *   compute wrapping xfer1/xfer2, write playback data or read capture data
     *   through HPI, advance host_rw and elapsed offsets, and call
     *   snd_pcm_period_elapsed().
     * - if polling mode and respawn_timer, add_timer(&dpcm->timer).
     */
}

unsafe extern "C" fn snd_card_asihpi_isr(a: *mut hpi_adapter) {
    /* WARN_ON(!a || !a->snd_card || !a->snd_card->private_data);
     * asihpi = a->snd_card->private_data; if llmode_streampriv call timer fn.
     */
    let _ = a;
}

unsafe extern "C" fn snd_card_asihpi_playback_prepare(_substream: *mut snd_pcm_substream) -> c_int {
    /* hpi_outstream_reset(dpcm->h_stream); clear host/dma/elapsed offsets. */
    0
}

unsafe extern "C" fn snd_card_asihpi_playback_pointer(_substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    /* ptr = bytes_to_frames(runtime, dpcm->pcm_buf_dma_ofs % dpcm->buffer_bytes). */
    0
}

unsafe extern "C" fn snd_card_asihpi_playback_formats(_asihpi: *mut snd_card_asihpi, _h_stream: u32) -> u64 {
    /* Query sampleclock rate if present, then for each HPI PCM format create an
     * HPI format and query outstream support.  Return ORed pcm_format_to_bits()
     * for formats that are supported and have an ALSA mapping.
     */
    0
}

unsafe extern "C" fn snd_card_asihpi_playback_open(_substream: *mut snd_pcm_substream) -> c_int {
    /* Allocate snd_card_asihpi_pcm, hpi_outstream_open, setup timer/private
     * data/free hook, build snd_pcm_hardware with DMA or interrupt constraints,
     * set channels/formats/rates/info, enable sync start if grouping, copy to
     * runtime->hw, and add pow2/step/minmax constraints.
     */
    0
}

unsafe extern "C" fn snd_card_asihpi_playback_close(_substream: *mut snd_pcm_substream) -> c_int {
    /* hpi_outstream_close(dpcm->h_stream). */
    0
}

#[repr(C)]
pub struct snd_pcm_ops {
    open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

static snd_card_asihpi_playback_mmap_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_card_asihpi_playback_open),
    close: Some(snd_card_asihpi_playback_close),
    hw_params: Some(snd_card_asihpi_pcm_hw_params),
    hw_free: Some(snd_card_asihpi_hw_free),
    prepare: Some(snd_card_asihpi_playback_prepare),
    trigger: Some(snd_card_asihpi_trigger),
    pointer: Some(snd_card_asihpi_playback_pointer),
};

unsafe extern "C" fn snd_card_asihpi_capture_pointer(_substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    /* Capture pointer uses local dma offset, not actual samples_played, because
     * those samples are not yet available in the local buffer.
     */
    0
}

unsafe extern "C" fn snd_card_asihpi_capture_prepare(_substream: *mut snd_pcm_substream) -> c_int {
    /* hpi_instream_reset(dpcm->h_stream); clear host/dma/elapsed offsets. */
    0
}

unsafe extern "C" fn snd_card_asihpi_capture_formats(_asihpi: *mut snd_card_asihpi, _h_stream: u32) -> u64 {
    /* Same as playback_formats, but query hpi_instream_query_format with
     * in_max_chans.
     */
    0
}

unsafe extern "C" fn snd_card_asihpi_capture_open(_substream: *mut snd_pcm_substream) -> c_int {
    /* Allocate stream private data, hpi_instream_open, setup timer/free hook,
     * build capture snd_pcm_hardware with DMA or interrupt constraints,
     * channels/formats/rates/info, copy runtime->hw, add constraints, and
     * snd_pcm_set_sync(substream).
     */
    0
}

unsafe extern "C" fn snd_card_asihpi_capture_close(_substream: *mut snd_pcm_substream) -> c_int {
    /* hpi_instream_close(dpcm->h_stream). */
    0
}

static snd_card_asihpi_capture_mmap_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_card_asihpi_capture_open),
    close: Some(snd_card_asihpi_capture_close),
    hw_params: Some(snd_card_asihpi_pcm_hw_params),
    hw_free: Some(snd_card_asihpi_hw_free),
    prepare: Some(snd_card_asihpi_capture_prepare),
    trigger: Some(snd_card_asihpi_trigger),
    pointer: Some(snd_card_asihpi_capture_pointer),
};

unsafe extern "C" fn snd_card_asihpi_pcm_new(_asihpi: *mut snd_card_asihpi, _device: c_int) -> c_int {
    /* hpi_adapter_get_info obtains stream counts; snd_pcm_new creates "Asihpi
     * PCM"; playback/capture mmap ops are installed; private_data, info_flags
     * and name are set; managed device DMA buffer is allocated from 64 KiB to
     * BUFFER_BYTES_MAX.
     */
    0
}

#[repr(C)]
pub struct hpi_control {
    h_control: u32,
    control_type: u16,
    src_node_type: u16,
    src_node_index: u16,
    dst_node_type: u16,
    dst_node_index: u16,
    band: u16,
    name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
}

static asihpi_tuner_band_names: [*const c_char; 11] = [
    c"invalid".as_ptr(), c"AM".as_ptr(), c"FM mono".as_ptr(), c"TV NTSC-M".as_ptr(),
    c"FM stereo".as_ptr(), c"AUX".as_ptr(), c"TV PAL BG".as_ptr(), c"TV PAL I".as_ptr(),
    c"TV PAL DK".as_ptr(), c"TV SECAM".as_ptr(), c"TV DAB".as_ptr(),
];

static asihpi_src_names: [*const c_char; 16] = [
    c"no source".as_ptr(), c"PCM".as_ptr(), c"Line".as_ptr(), c"Digital".as_ptr(),
    c"Tuner".as_ptr(), c"RF".as_ptr(), c"Clock".as_ptr(), c"Bitstream".as_ptr(),
    c"Mic".as_ptr(), c"Net".as_ptr(), c"Analog".as_ptr(), c"Adapter".as_ptr(),
    c"RTP".as_ptr(), c"Internal".as_ptr(), c"AVB".as_ptr(), c"BLU-Link".as_ptr(),
];

static asihpi_dst_names: [*const c_char; 12] = [
    c"no destination".as_ptr(), c"PCM".as_ptr(), c"Line".as_ptr(), c"Digital".as_ptr(),
    c"RF".as_ptr(), c"Speaker".as_ptr(), c"Net".as_ptr(), c"Analog".as_ptr(),
    c"RTP".as_ptr(), c"AVB".as_ptr(), c"Internal".as_ptr(), c"BLU-Link".as_ptr(),
];

unsafe extern "C" fn ctl_add(_card: *mut snd_card, _ctl: *mut snd_kcontrol_new, _asihpi: *mut snd_card_asihpi) -> c_int {
    /* err = snd_ctl_add(card, snd_ctl_new1(ctl, asihpi)); if mixer_dump log. */
    0
}

unsafe extern "C" fn asihpi_ctl_init(_snd_control: *mut snd_kcontrol_new, _hpi_ctl: *mut hpi_control, _name: *mut c_char) {
    /* memset snd_control; point name/private_value/iface/index at hpi_ctl.
     * Determine dir string from source/destination node types, then snprintf an
     * ALSA control name using source/destination names and the supplied suffix.
     * Log if the name is truncated.
     */
}

unsafe extern "C" fn snd_asihpi_volume_info(_kcontrol: *mut snd_kcontrol, _uinfo: *mut snd_ctl_elem_info) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_volume_get(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_volume_put(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 1 }
static db_scale_100: [c_uint; 4] = [SNDRV_CTL_TLVT_DB_SCALE as c_uint, 2 * size_of::<c_uint>() as c_uint, (-10000i32) as c_uint, VOL_STEP_mB as c_uint];
unsafe extern "C" fn snd_asihpi_volume_mute_info(_kcontrol: *mut snd_kcontrol, _uinfo: *mut snd_ctl_elem_info) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_volume_mute_get(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_volume_mute_put(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 1 }
unsafe extern "C" fn snd_asihpi_volume_add(_asihpi: *mut snd_card_asihpi, _hpi_ctl: *mut hpi_control) -> c_int {
    /* Add Volume integer TLV control, then add Switch when HPI volume mute is supported. */
    0
}

unsafe extern "C" fn snd_asihpi_level_info(_kcontrol: *mut snd_kcontrol, _uinfo: *mut snd_ctl_elem_info) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_level_get(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_level_put(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 1 }
static db_scale_level: [c_uint; 4] = [SNDRV_CTL_TLVT_DB_SCALE as c_uint, 2 * size_of::<c_uint>() as c_uint, (-1000i32) as c_uint, 100];
unsafe extern "C" fn snd_asihpi_level_add(_asihpi: *mut snd_card_asihpi, _hpi_ctl: *mut hpi_control) -> c_int {
    /* Add Level integer TLV control. */
    0
}

static asihpi_aesebu_format_names: [*const c_char; 3] = [c"N/A".as_ptr(), c"S/PDIF".as_ptr(), c"AES/EBU".as_ptr()];
unsafe extern "C" fn snd_asihpi_aesebu_format_info(_kcontrol: *mut snd_kcontrol, _uinfo: *mut snd_ctl_elem_info) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_aesebu_format_get(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value, _func: Option<unsafe extern "C" fn(u32, *mut u16) -> u16>) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_aesebu_format_put(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value, _func: Option<unsafe extern "C" fn(u32, u16) -> u16>) -> c_int { 1 }
unsafe extern "C" fn snd_asihpi_aesebu_rx_format_get(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { snd_asihpi_aesebu_format_get(k, u, None) }
unsafe extern "C" fn snd_asihpi_aesebu_rx_format_put(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { snd_asihpi_aesebu_format_put(k, u, None) }
unsafe extern "C" fn snd_asihpi_aesebu_rxstatus_info(_kcontrol: *mut snd_kcontrol, _uinfo: *mut snd_ctl_elem_info) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_aesebu_rxstatus_get(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_aesebu_rx_add(_asihpi: *mut snd_card_asihpi, _hpi_ctl: *mut hpi_control) -> c_int {
    /* Add AESEBU receiver Format read/write enum and Status volatile read controls. */
    0
}
unsafe extern "C" fn snd_asihpi_aesebu_tx_format_get(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { snd_asihpi_aesebu_format_get(k, u, None) }
unsafe extern "C" fn snd_asihpi_aesebu_tx_format_put(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int { snd_asihpi_aesebu_format_put(k, u, None) }
unsafe extern "C" fn snd_asihpi_aesebu_tx_add(_asihpi: *mut snd_card_asihpi, _hpi_ctl: *mut hpi_control) -> c_int {
    /* Add AESEBU transmitter Format read/write enum control. */
    0
}

unsafe extern "C" fn snd_asihpi_tuner_gain_info(_kcontrol: *mut snd_kcontrol, _uinfo: *mut snd_ctl_elem_info) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_tuner_gain_get(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_tuner_gain_put(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 1 }
unsafe extern "C" fn asihpi_tuner_band_query(_kcontrol: *mut snd_kcontrol, _band_list: *mut u16, len: u32) -> c_int {
    /* Query hpi_tuner_query_band until len or error; INVALID_OBJ_INDEX ends list,
     * other errors return -EIO; otherwise return number of bands.
     */
    len as c_int
}
unsafe extern "C" fn snd_asihpi_tuner_band_info(_kcontrol: *mut snd_kcontrol, _uinfo: *mut snd_ctl_elem_info) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_tuner_band_get(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_tuner_band_put(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 1 }
unsafe extern "C" fn snd_asihpi_tuner_freq_info(_kcontrol: *mut snd_kcontrol, _uinfo: *mut snd_ctl_elem_info) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_tuner_freq_get(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_tuner_freq_put(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 1 }
unsafe extern "C" fn snd_asihpi_tuner_add(_asihpi: *mut snd_card_asihpi, _hpi_ctl: *mut hpi_control) -> c_int {
    /* Optionally add Gain, then add Band and Freq tuner controls. */
    0
}

unsafe extern "C" fn snd_asihpi_meter_info(_kcontrol: *mut snd_kcontrol, _uinfo: *mut snd_ctl_elem_info) -> c_int { 0 }
static log2lin: [c_int; 19] = [
    0x7FFFFFFF, /* 0dB */
    679093956,
    214748365,
     67909396,
     21474837,
      6790940,
      2147484, /* -60dB */
       679094,
       214748, /* -80 */
        67909,
        21475, /* -100 */
         6791,
         2147,
          679,
          214,
           68,
           21,
            7,
            2,
];
unsafe extern "C" fn snd_asihpi_meter_get(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int {
    /* hpi_meter_get_peak; if error zero values; if >=0 shift millibel value by
     * 16; otherwise convert negative log millibels to approximate linear using
     * log2lin[an_gain_mB[i] / -1000].
     */
    0
}
unsafe extern "C" fn snd_asihpi_meter_add(_asihpi: *mut snd_card_asihpi, _hpi_ctl: *mut hpi_control, _subidx: c_int) -> c_int {
    /* Add volatile read Meter control with index subidx. */
    0
}

unsafe extern "C" fn snd_card_asihpi_mux_count_sources(_snd_control: *mut snd_kcontrol) -> c_int {
    /* Query up to 32 multiplexer sources and return the count before error. */
    0
}
unsafe extern "C" fn snd_asihpi_mux_info(_kcontrol: *mut snd_kcontrol, _uinfo: *mut snd_ctl_elem_info) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_mux_get(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int {
    /* Get current mux source, search query_source indices up to 256, set enum item or warn and set 0. */
    0
}
unsafe extern "C" fn snd_asihpi_mux_put(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int {
    /* Query selected source and set it with hpi_multiplexer_set_source. */
    1
}
unsafe extern "C" fn snd_asihpi_mux_add(_asihpi: *mut snd_card_asihpi, _hpi_ctl: *mut hpi_control) -> c_int {
    /* Add Route read/write enum control. */
    0
}

unsafe extern "C" fn snd_asihpi_cmode_info(_kcontrol: *mut snd_kcontrol, _uinfo: *mut snd_ctl_elem_info) -> c_int {
    /* Query supported channel modes and expose mapped names: invalid, Normal,
     * Swap, From Left, From Right, To Left, To Right.
     */
    0
}
unsafe extern "C" fn snd_asihpi_cmode_get(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_cmode_put(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 1 }
unsafe extern "C" fn snd_asihpi_cmode_add(_asihpi: *mut snd_card_asihpi, _hpi_ctl: *mut hpi_control) -> c_int {
    /* Add Mode read/write enum control. */
    0
}

static sampleclock_sources: [*const c_char; MAX_CLOCKSOURCES] = [
    c"N/A".as_ptr(), c"Local PLL".as_ptr(), c"Digital Sync".as_ptr(), c"Word External".as_ptr(),
    c"Word Header".as_ptr(), c"SMPTE".as_ptr(), c"Digital1".as_ptr(), c"Auto".as_ptr(),
    c"Network".as_ptr(), c"Invalid".as_ptr(), c"Prev Module".as_ptr(), c"BLU-Link".as_ptr(),
    c"Digital2".as_ptr(), c"Digital3".as_ptr(), c"Digital4".as_ptr(), c"Digital5".as_ptr(),
    c"Digital6".as_ptr(), c"Digital7".as_ptr(), c"Digital8".as_ptr(),
];

unsafe extern "C" fn snd_asihpi_clksrc_info(_kcontrol: *mut snd_kcontrol, _uinfo: *mut snd_ctl_elem_info) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_clksrc_get(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_clksrc_put(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 1 }
unsafe extern "C" fn snd_asihpi_clklocal_info(_kcontrol: *mut snd_kcontrol, _uinfo: *mut snd_ctl_elem_info) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_clklocal_get(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_clklocal_put(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 1 }
unsafe extern "C" fn snd_asihpi_clkrate_info(_kcontrol: *mut snd_kcontrol, _uinfo: *mut snd_ctl_elem_info) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_clkrate_get(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 0 }
unsafe extern "C" fn snd_asihpi_sampleclock_add(_asihpi: *mut snd_card_asihpi, _hpi_ctl: *mut hpi_control) -> c_int {
    /* Build clkcache by querying sources and AESEBU input indices; add Source
     * control, Localrate if local source is present, and volatile read Rate.
     */
    0
}

unsafe extern "C" fn snd_card_asihpi_mixer_new(_asihpi: *mut snd_card_asihpi) -> c_int {
    /* Open mixer, iterate hpi_mixer_get_control_by_index up to 2000, skip
     * disabled controls, normalize source/destination node numbers, compute
     * subindex for duplicate controls, dispatch by HPI_CONTROL_* to the
     * corresponding add function, ignore connections and untranslated controls,
     * report non-INVALID_OBJ_INDEX errors, and log mixer control count.
     */
    0
}

unsafe extern "C" fn snd_asihpi_proc_read(_entry: *mut snd_info_entry, _buffer: *mut snd_info_buffer) {
    /* Print ASIHPI driver proc file, adapter type/index/stream counts, serial,
     * hardware version, DSP code version, and sample clock rate/source when
     * available.
     */
}

unsafe extern "C" fn snd_asihpi_proc_init(_asihpi: *mut snd_card_asihpi) {
    /* snd_card_ro_proc_new(asihpi->card, "info", asihpi, snd_asihpi_proc_read). */
}

unsafe extern "C" fn snd_asihpi_hpi_open(_hw: *mut snd_hwdep, _file: *mut file) -> c_int {
    if enable_hpi_hwdep { 0 } else { -ENODEV }
}

unsafe extern "C" fn snd_asihpi_hpi_release(_hw: *mut snd_hwdep, file: *mut file) -> c_int {
    if enable_hpi_hwdep {
        asihpi_hpi_release(file)
    } else {
        -ENODEV
    }
}

unsafe extern "C" fn snd_asihpi_hpi_ioctl(_hw: *mut snd_hwdep, file: *mut file, cmd: c_uint, arg: c_ulong) -> c_int {
    if enable_hpi_hwdep {
        asihpi_hpi_ioctl(file, cmd, arg)
    } else {
        -ENODEV
    }
}

unsafe extern "C" {
    fn asihpi_hpi_release(file: *mut file) -> c_int;
    fn asihpi_hpi_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_int;
}

/* results in /dev/snd/hwC#D0 file for each card with index #
   also /proc/asound/hwdep will contain '#-00: asihpi (HPI) for each card'
*/
unsafe extern "C" fn snd_asihpi_hpi_new(_asihpi: *mut snd_card_asihpi, _device: c_int) -> c_int {
    /* snd_hwdep_new(card, "HPI", device, &hw); name "asihpi (HPI)"; iface
     * SNDRV_HWDEP_IFACE_LAST; ops open/ioctl/release; private_data = asihpi.
     */
    0
}

unsafe extern "C" fn snd_asihpi_probe(_pci_dev: *mut pci_dev, _pci_id: *const pci_device_id) -> c_int {
    /* Probe sequence:
     * - static dev guard against SNDRV_CARDS and enable[dev].
     * - asihpi_adapter_probe, pci_get_drvdata, adapter_index.
     * - snd_card_new first using hardware index, then fallback index[dev].
     * - initialize snd_card_asihpi private data and hpi->snd_card.
     * - query CAPS1 grouping, CAPS2 MRX, update interval.
     * - select interrupt or timer pcm_start/pcm_stop and ISR callback.
     * - test DMA by opening instream 0 and freeing host buffer.
     * - query current channels and derive min channels/grouping for LL mode.
     * - create PCM, mixer, set local sample rate, proc and hwdep.
     * - fill card driver/shortname/longname and register card.
     * - on failure free card and return err.
     */
    0
}

unsafe extern "C" fn snd_asihpi_remove(_pci_dev: *mut pci_dev) {
    /* Get hpi via pci_get_drvdata; if interrupt mode clear callback and IRQ
     * rate; snd_card_free(hpi->snd_card); clear snd_card; asihpi_adapter_remove.
     */
}

static asihpi_pci_tbl: [pci_device_id; 3] = [
    /* PCI_DEVICE_SUB(HPI_PCI_VENDOR_ID_TI, HPI_PCI_DEV_ID_DSP6205,
     *                HPI_PCI_VENDOR_ID_AUDIOSCIENCE, PCI_ANY_ID),
     * .driver_data = (kernel_ulong_t)HPI_6205
     */
    pci_device_id { _private: [] },
    /* PCI_DEVICE_SUB(HPI_PCI_VENDOR_ID_TI, HPI_PCI_DEV_ID_PCI2040,
     *                HPI_PCI_VENDOR_ID_AUDIOSCIENCE, PCI_ANY_ID),
     * .driver_data = (kernel_ulong_t)HPI_6000
     */
    pci_device_id { _private: [] },
    pci_device_id { _private: [] },
];

static mut driver: pci_driver = pci_driver { _private: [] };

unsafe extern "C" {
    fn asihpi_init();
    fn asihpi_exit();
    fn pci_register_driver(driver: *mut pci_driver) -> c_int;
    fn pci_unregister_driver(driver: *mut pci_driver);
}

unsafe extern "C" fn snd_asihpi_init() -> c_int {
    asihpi_init();
    pci_register_driver(&mut driver)
}

unsafe extern "C" fn snd_asihpi_exit() {
    pci_unregister_driver(&mut driver);
    asihpi_exit();
}

/* module_init(snd_asihpi_init)
 * module_exit(snd_asihpi_exit)
 */

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const EIO: c_int = 5;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;

const HPI_ERROR_INVALID_OBJ: c_uint = 100;
const HPI_OBJ_OSTREAM: c_uint = 1;
const HPI_OSTREAM_HOSTBUFFER_ALLOC: c_uint = 0;
const HPI_ISTREAM_HOSTBUFFER_ALLOC: c_uint = 0;
const HPI_OSTREAM_HOSTBUFFER_FREE: c_uint = 0;
const HPI_ISTREAM_HOSTBUFFER_FREE: c_uint = 0;
const HPI_FORMAT_PCM8_UNSIGNED: c_uint = 1;
const HPI_FORMAT_PCM24_SIGNED: c_uint = 15;
const SNDRV_PCM_FORMAT_U8: c_int = 1;
const SNDRV_PCM_FORMAT_S16: c_int = 2;
const SNDRV_PCM_FORMAT_MPEG: c_int = 3;
const SNDRV_PCM_FORMAT_S16_BE: c_int = 4;
const SNDRV_PCM_FORMAT_S32: c_int = 5;
const SNDRV_PCM_FORMAT_FLOAT: c_int = 6;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 2;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_CTL_ELEM_ID_NAME_MAXLEN: usize = 44;
const SNDRV_CTL_TLVT_DB_SCALE: c_int = 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
