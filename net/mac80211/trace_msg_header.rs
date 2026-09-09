/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Portions of this file
 * Copyright (C) 2019 Intel Corporation
 */

/* Translation of the C header's CONFIG_MAC80211_MESSAGE_TRACING condition.
 * The condition is supplied by the surrounding build configuration. */
#[cfg(CONFIG_MAC80211_MESSAGE_TRACING)]
mod mac80211_message_tracing {
    use core::ffi::c_char;

    /* Supplied by the Linux tracing and mac80211 headers. */
    #[repr(C)]
    pub struct va_format {
        pub fmt: *const c_char,
        pub va: *mut core::ffi::c_void,
    }

    /*
     * DECLARE_EVENT_CLASS(mac80211_msg_event,
     *     TP_PROTO(struct va_format *vaf),
     *     TP_ARGS(vaf),
     *     TP_STRUCT__entry(__vstring(msg, vaf->fmt, vaf->va)),
     *     TP_fast_assign(__assign_vstr(msg, vaf->fmt, vaf->va)),
     *     TP_printk("%s", __get_str(msg))
     * );
     *
     * The event-class and tracepoint storage/printing operations are provided
     * by the external Linux tracepoint implementation.
     */
    pub type Mac80211MsgEvent = unsafe extern "C" fn(vaf: *mut va_format);

    /* DEFINE_EVENT(mac80211_msg_event, mac80211_info, ...); */
    unsafe extern "C" {
        pub fn mac80211_info(vaf: *mut va_format);
        pub fn mac80211_dbg(vaf: *mut va_format);
        pub fn mac80211_err(vaf: *mut va_format);
    }

    /* DEFINE_EVENT(mac80211_msg_event, mac80211_dbg, ...); */
    /* DEFINE_EVENT(mac80211_msg_event, mac80211_err, ...); */
}

/* TRACE_INCLUDE_PATH is . and TRACE_INCLUDE_FILE is trace_msg. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
