/* SPDX-License-Identifier: GPL-2.0 */

/* C header intent:
 * #undef TRACE_SYSTEM
 * #define TRACE_SYSTEM snd_ctl
 *
 * Includes from the original header:
 * <linux/tracepoint.h>
 * <uapi/sound/asound.h>
 */

/* Original header guard:
 * #if !defined(_TRACE_SND_CTL_H) || defined(TRACE_HEADER_MULTI_READ)
 * #define _TRACE_SND_CTL_H
 */

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub numid: c_uint,
    pub iface: c_int,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: [c_char; 44],
    pub index: c_uint,
}

#[repr(C)]
pub struct trace_event_raw_snd_ctl_put {
    pub numid: c_uint,
    pub iname: *const c_char,
    pub kname: *const c_char,
    pub index: c_uint,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub card: c_uint,
    pub expected: c_int,
    pub actual: c_int,
}

/* TRACE_EVENT(snd_ctl_put,
 *
 *     TP_PROTO(struct snd_ctl_elem_id *id, const char *iname, unsigned int card,
 *              int expected, int actual),
 *
 *     TP_ARGS(id, iname, card, expected, actual),
 *
 *     TP_STRUCT__entry(
 *         __field(unsigned int, numid)
 *         __string(iname,       iname)
 *         __string(kname,       id->name)
 *         __field(unsigned int, index)
 *         __field(unsigned int, device)
 *         __field(unsigned int, subdevice)
 *         __field(unsigned int, card)
 *         __field(int,          expected)
 *         __field(int,          actual)
 *     ),
 *
 *     TP_fast_assign(...)
 *
 *     TP_printk("%s: expected=%d, actual=%d for ctl numid=%d, iface=%s, name='%s', index=%d, device=%d, subdevice=%d, card=%d\n",
 *               __entry->expected == __entry->actual ? "success" : "fail",
 *               ...)
 * );
 */
pub unsafe fn trace_snd_ctl_put_assign(
    __entry: *mut trace_event_raw_snd_ctl_put,
    id: *mut snd_ctl_elem_id,
    iname: *const c_char,
    card: c_uint,
    expected: c_int,
    actual: c_int,
) {
    unsafe {
        (*__entry).numid = (*id).numid;
        (*__entry).iname = iname;
        (*__entry).kname = (*id).name.as_ptr();
        (*__entry).index = (*id).index;
        (*__entry).device = (*id).device;
        (*__entry).subdevice = (*id).subdevice;
        (*__entry).card = card;
        (*__entry).expected = expected;
        (*__entry).actual = actual;
    }
}

pub const TRACE_SND_CTL_PUT_PRINTK_FORMAT: &[u8] = b"%s: expected=%d, actual=%d for ctl numid=%d, iface=%s, name='%s', index=%d, device=%d, subdevice=%d, card=%d\n\0";

pub unsafe fn trace_snd_ctl_put_result(entry: *const trace_event_raw_snd_ctl_put) -> *const c_char {
    unsafe {
        if (*entry).expected == (*entry).actual {
            c"success".as_ptr()
        } else {
            c"fail".as_ptr()
        }
    }
}

/* #endif _TRACE_SND_CTL_H */

/* This part must be outside protection */
/* Original trace include selection:
 * #undef TRACE_INCLUDE_PATH
 * #define TRACE_INCLUDE_PATH .
 * #define TRACE_INCLUDE_FILE control_trace
 * #include <trace/define_trace.h>
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
