/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Source-level Rust translation of hda/core/trace.h.
 *
 * The original file is a Linux tracepoint header built from TRACE_EVENT,
 * DECLARE_EVENT_CLASS, and DEFINE_EVENT macros.  Those macros expand in the
 * kernel trace infrastructure, so this file preserves the local constants,
 * forward declarations, event entry layouts, assignment behavior, and printk
 * format intent as Rust items and comments.
 */

/* Original trace system name:
 * #undef TRACE_SYSTEM
 * #define TRACE_SYSTEM hda
 */
pub const TRACE_SYSTEM: &str = "hda";

/* Original include dependencies:
 * #include <linux/tracepoint.h>
 * #include <linux/device.h>
 * #include <sound/hdaudio.h>
 */

/* Original conditional:
 * #ifndef HDAC_MSG_MAX
 * #define HDAC_MSG_MAX 500
 * #endif
 */
pub const HDAC_MSG_MAX: u32 = 500;

#[repr(C)]
pub struct hdac_bus {
    pub dev: *mut device,
}

#[repr(C)]
pub struct hdac_codec {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct hdac_stream {
    pub stream_tag: ::core::ffi::c_uchar,
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn dev_name(dev: *const device) -> *const ::core::ffi::c_char;
}

/*
 * TRACE_EVENT(hda_send_cmd,
 *     TP_PROTO(struct hdac_bus *bus, unsigned int cmd),
 *     TP_ARGS(bus, cmd),
 *     TP_STRUCT__entry(
 *         __string(name, dev_name((bus)->dev))
 *         __field(u32, cmd)
 *     ),
 *     TP_fast_assign(
 *         __assign_str(name);
 *         __entry->cmd = cmd;
 *     ),
 *     TP_printk("[%s:%d] val=0x%08x", __get_str(name), __entry->cmd >> 28, __entry->cmd)
 * );
 */
#[repr(C)]
pub struct hda_send_cmd_entry {
    pub name: *const ::core::ffi::c_char,
    pub cmd: u32,
}

pub unsafe fn hda_send_cmd_assign(bus: *mut hdac_bus, cmd: ::core::ffi::c_uint) -> hda_send_cmd_entry {
    hda_send_cmd_entry {
        name: unsafe { dev_name((*bus).dev) },
        cmd: cmd as u32,
    }
}

pub const HDA_SEND_CMD_PRINTK_FORMAT: &str = "[%s:%d] val=0x%08x";

pub unsafe fn hda_send_cmd_printk_node(entry: *const hda_send_cmd_entry) -> u32 {
    unsafe { (*entry).cmd >> 28 }
}

/*
 * TRACE_EVENT(hda_get_response,
 *     TP_PROTO(struct hdac_bus *bus, unsigned int addr, unsigned int res),
 *     TP_ARGS(bus, addr, res),
 *     TP_STRUCT__entry(
 *         __string(name, dev_name((bus)->dev))
 *         __field(u32, addr)
 *         __field(u32, res)
 *     ),
 *     TP_fast_assign(
 *         __assign_str(name);
 *         __entry->addr = addr;
 *         __entry->res = res;
 *     ),
 *     TP_printk("[%s:%d] val=0x%08x", __get_str(name), __entry->addr, __entry->res)
 * );
 */
#[repr(C)]
pub struct hda_get_response_entry {
    pub name: *const ::core::ffi::c_char,
    pub addr: u32,
    pub res: u32,
}

pub unsafe fn hda_get_response_assign(
    bus: *mut hdac_bus,
    addr: ::core::ffi::c_uint,
    res: ::core::ffi::c_uint,
) -> hda_get_response_entry {
    hda_get_response_entry {
        name: unsafe { dev_name((*bus).dev) },
        addr: addr as u32,
        res: res as u32,
    }
}

pub const HDA_GET_RESPONSE_PRINTK_FORMAT: &str = "[%s:%d] val=0x%08x";

/*
 * TRACE_EVENT(hda_unsol_event,
 *     TP_PROTO(struct hdac_bus *bus, u32 res, u32 res_ex),
 *     TP_ARGS(bus, res, res_ex),
 *     TP_STRUCT__entry(
 *         __string(name, dev_name((bus)->dev))
 *         __field(u32, res)
 *         __field(u32, res_ex)
 *     ),
 *     TP_fast_assign(
 *         __assign_str(name);
 *         __entry->res = res;
 *         __entry->res_ex = res_ex;
 *     ),
 *     TP_printk("[%s:%d] res=0x%08x, res_ex=0x%08x", __get_str(name),
 *               __entry->res_ex & 0x0f, __entry->res, __entry->res_ex)
 * );
 */
#[repr(C)]
pub struct hda_unsol_event_entry {
    pub name: *const ::core::ffi::c_char,
    pub res: u32,
    pub res_ex: u32,
}

pub unsafe fn hda_unsol_event_assign(bus: *mut hdac_bus, res: u32, res_ex: u32) -> hda_unsol_event_entry {
    hda_unsol_event_entry {
        name: unsafe { dev_name((*bus).dev) },
        res,
        res_ex,
    }
}

pub const HDA_UNSOL_EVENT_PRINTK_FORMAT: &str = "[%s:%d] res=0x%08x, res_ex=0x%08x";

pub unsafe fn hda_unsol_event_printk_addr(entry: *const hda_unsol_event_entry) -> u32 {
    unsafe { (*entry).res_ex & 0x0f }
}

/*
 * DECLARE_EVENT_CLASS(hdac_stream,
 *     TP_PROTO(struct hdac_bus *bus, struct hdac_stream *azx_dev),
 *     TP_ARGS(bus, azx_dev),
 *     TP_STRUCT__entry(
 *         __field(unsigned char, stream_tag)
 *     ),
 *     TP_fast_assign(
 *         __entry->stream_tag = (azx_dev)->stream_tag;
 *     ),
 *     TP_printk("stream_tag: %d", __entry->stream_tag)
 * );
 */
#[repr(C)]
pub struct hdac_stream_entry {
    pub stream_tag: ::core::ffi::c_uchar,
}

pub unsafe fn hdac_stream_assign(
    _bus: *mut hdac_bus,
    azx_dev: *mut hdac_stream,
) -> hdac_stream_entry {
    hdac_stream_entry {
        stream_tag: unsafe { (*azx_dev).stream_tag },
    }
}

pub const HDAC_STREAM_PRINTK_FORMAT: &str = "stream_tag: %d";

/*
 * DEFINE_EVENT(hdac_stream, snd_hdac_stream_start,
 *     TP_PROTO(struct hdac_bus *bus, struct hdac_stream *azx_dev),
 *     TP_ARGS(bus, azx_dev)
 * );
 */
pub type snd_hdac_stream_start_entry = hdac_stream_entry;

pub unsafe fn snd_hdac_stream_start_assign(
    bus: *mut hdac_bus,
    azx_dev: *mut hdac_stream,
) -> snd_hdac_stream_start_entry {
    unsafe { hdac_stream_assign(bus, azx_dev) }
}

/*
 * DEFINE_EVENT(hdac_stream, snd_hdac_stream_stop,
 *     TP_PROTO(struct hdac_bus *bus, struct hdac_stream *azx_dev),
 *     TP_ARGS(bus, azx_dev)
 * );
 */
pub type snd_hdac_stream_stop_entry = hdac_stream_entry;

pub unsafe fn snd_hdac_stream_stop_assign(
    bus: *mut hdac_bus,
    azx_dev: *mut hdac_stream,
) -> snd_hdac_stream_stop_entry {
    unsafe { hdac_stream_assign(bus, azx_dev) }
}

/* This part must be outside protection.
 *
 * Original trace include controls:
 * #undef TRACE_INCLUDE_PATH
 * #define TRACE_INCLUDE_PATH .
 *
 * #undef TRACE_INCLUDE_FILE
 * #define TRACE_INCLUDE_FILE trace
 *
 * #include <trace/define_trace.h>
 */
pub const TRACE_INCLUDE_PATH: &str = ".";
pub const TRACE_INCLUDE_FILE: &str = "trace";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
