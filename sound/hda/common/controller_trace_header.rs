/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C tracepoint metadata translated from controller_trace.h.
 *
 * Original preprocessor intent:
 *   #undef TRACE_SYSTEM
 *   #define TRACE_SYSTEM hda_controller
 *   #define TRACE_INCLUDE_FILE controller_trace
 *
 * The Linux TRACE_EVENT/DECLARE_EVENT_CLASS/DEFINE_EVENT machinery and
 * <trace/define_trace.h> integration are external dependencies in C.
 */
pub const TRACE_SYSTEM: &str = "hda_controller";
pub const TRACE_INCLUDE_FILE: &str = "controller_trace";

/*
 * Forward declarations from the C header:
 *   struct azx;
 *   struct azx_dev;
 *
 * The concrete layouts are supplied by other translated files.
 */
#[repr(C)]
pub struct azx {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct azx_dev {
    _unused: [u8; 0],
}

/*
 * TRACE_EVENT(azx_pcm_trigger,
 *
 *   TP_PROTO(struct azx *chip, struct azx_dev *dev, int cmd),
 *   TP_ARGS(chip, dev, cmd),
 *
 *   TP_STRUCT__entry(
 *     __field( int, card )
 *     __field( int, idx )
 *     __field( int, cmd )
 *   ),
 *
 *   TP_fast_assign(
 *     __entry->card = (chip)->card->number;
 *     __entry->idx = (dev)->core.index;
 *     __entry->cmd = cmd;
 *   ),
 *
 *   TP_printk("[%d:%d] cmd=%d", __entry->card, __entry->idx, __entry->cmd)
 * );
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct trace_event_raw_azx_pcm_trigger {
    pub card: ::std::os::raw::c_int,
    pub idx: ::std::os::raw::c_int,
    pub cmd: ::std::os::raw::c_int,
}

pub const AZX_PCM_TRIGGER_PRINTK_FORMAT: &str = "[%d:%d] cmd=%d";

/*
 * The C TP_fast_assign body depends on the external layouts of struct azx and
 * struct azx_dev:
 *   __entry->card = (chip)->card->number;
 *   __entry->idx = (dev)->core.index;
 *   __entry->cmd = cmd;
 */
pub unsafe fn azx_pcm_trigger_fast_assign(
    __entry: *mut trace_event_raw_azx_pcm_trigger,
    _chip: *mut azx,
    _dev: *mut azx_dev,
    cmd: ::std::os::raw::c_int,
) {
    /*
     * TODO: assign card and idx when the translated definitions for azx and
     * azx_dev expose chip->card->number and dev->core.index.
     */
    unsafe {
        (*__entry).cmd = cmd;
    }
}

/*
 * TRACE_EVENT(azx_get_position,
 *
 *   TP_PROTO(struct azx *chip, struct azx_dev *dev, unsigned int pos, unsigned int delay),
 *   TP_ARGS(chip, dev, pos, delay),
 *
 *   TP_STRUCT__entry(
 *     __field( int, card )
 *     __field( int, idx )
 *     __field( unsigned int, pos )
 *     __field( unsigned int, delay )
 *   ),
 *
 *   TP_fast_assign(
 *     __entry->card = (chip)->card->number;
 *     __entry->idx = (dev)->core.index;
 *     __entry->pos = pos;
 *     __entry->delay = delay;
 *   ),
 *
 *   TP_printk("[%d:%d] pos=%u, delay=%u", __entry->card, __entry->idx, __entry->pos, __entry->delay)
 * );
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct trace_event_raw_azx_get_position {
    pub card: ::std::os::raw::c_int,
    pub idx: ::std::os::raw::c_int,
    pub pos: ::std::os::raw::c_uint,
    pub delay: ::std::os::raw::c_uint,
}

pub const AZX_GET_POSITION_PRINTK_FORMAT: &str = "[%d:%d] pos=%u, delay=%u";

/*
 * The C TP_fast_assign body depends on the external layouts of struct azx and
 * struct azx_dev:
 *   __entry->card = (chip)->card->number;
 *   __entry->idx = (dev)->core.index;
 *   __entry->pos = pos;
 *   __entry->delay = delay;
 */
pub unsafe fn azx_get_position_fast_assign(
    __entry: *mut trace_event_raw_azx_get_position,
    _chip: *mut azx,
    _dev: *mut azx_dev,
    pos: ::std::os::raw::c_uint,
    delay: ::std::os::raw::c_uint,
) {
    /*
     * TODO: assign card and idx when the translated definitions for azx and
     * azx_dev expose chip->card->number and dev->core.index.
     */
    unsafe {
        (*__entry).pos = pos;
        (*__entry).delay = delay;
    }
}

/*
 * DECLARE_EVENT_CLASS(azx_pcm,
 *
 *   TP_PROTO(struct azx *chip, struct azx_dev *azx_dev),
 *   TP_ARGS(chip, azx_dev),
 *
 *   TP_STRUCT__entry(
 *     __field( unsigned char, stream_tag )
 *   ),
 *
 *   TP_fast_assign(
 *     __entry->stream_tag = (azx_dev)->core.stream_tag;
 *   ),
 *
 *   TP_printk("stream_tag: %d", __entry->stream_tag)
 * );
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct trace_event_raw_azx_pcm {
    pub stream_tag: ::std::os::raw::c_uchar,
}

pub const AZX_PCM_PRINTK_FORMAT: &str = "stream_tag: %d";

/*
 * The C TP_fast_assign body depends on the external layout of struct azx_dev:
 *   __entry->stream_tag = (azx_dev)->core.stream_tag;
 */
pub unsafe fn azx_pcm_fast_assign(
    _chip: *mut azx,
    __entry: *mut trace_event_raw_azx_pcm,
    _azx_dev: *mut azx_dev,
) {
    /*
     * TODO: assign stream_tag when the translated definition for azx_dev exposes
     * azx_dev->core.stream_tag.
     */
    let _ = __entry;
}

/*
 * DEFINE_EVENT(azx_pcm, azx_pcm_open,
 *   TP_PROTO(struct azx *chip, struct azx_dev *azx_dev),
 *   TP_ARGS(chip, azx_dev)
 * );
 *
 * DEFINE_EVENT(azx_pcm, azx_pcm_close,
 *   TP_PROTO(struct azx *chip, struct azx_dev *azx_dev),
 *   TP_ARGS(chip, azx_dev)
 * );
 *
 * DEFINE_EVENT(azx_pcm, azx_pcm_hw_params,
 *   TP_PROTO(struct azx *chip, struct azx_dev *azx_dev),
 *   TP_ARGS(chip, azx_dev)
 * );
 *
 * DEFINE_EVENT(azx_pcm, azx_pcm_prepare,
 *   TP_PROTO(struct azx *chip, struct azx_dev *azx_dev),
 *   TP_ARGS(chip, azx_dev)
 * );
 */
pub type azx_pcm_open = trace_event_raw_azx_pcm;
pub type azx_pcm_close = trace_event_raw_azx_pcm;
pub type azx_pcm_hw_params = trace_event_raw_azx_pcm;
pub type azx_pcm_prepare = trace_event_raw_azx_pcm;

/*
 * This part must be outside protection.
 *
 * Original preprocessor intent:
 *   #undef TRACE_INCLUDE_PATH
 *   #define TRACE_INCLUDE_PATH .
 *   #include <trace/define_trace.h>
 */
pub const TRACE_INCLUDE_PATH: &str = ".";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
