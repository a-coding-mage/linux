/* SPDX-License-Identifier: GPL-2.0 */

/* TRACE_SYSTEM napi */

/* Dependencies supplied by the surrounding kernel translation. */

pub const NO_DEV: &str = "(no_device)";

/* Equivalent payload for the napi_poll trace event. */
#[repr(C)]
pub struct NapiPollEntry {
    pub napi: *mut NapiStruct,
    pub dev_name: *const ::core::ffi::c_char,
    pub work: ::core::ffi::c_int,
    pub budget: ::core::ffi::c_int,
}

/* Equivalent payload for the dql_stall_detected trace event. */
#[repr(C)]
pub struct DqlStallDetectedEntry {
    pub thrs: u16,
    pub len: ::core::ffi::c_uint,
    pub last_reap: ::core::ffi::c_ulong,
    pub hist_head: ::core::ffi::c_ulong,
    pub now: ::core::ffi::c_ulong,
    pub hist: [::core::ffi::c_ulong; 4],
}

/* Supplied by linux/netdevice.h. */
#[repr(C)]
pub struct NapiStruct {
    _private: [u8; 0],
}

/*
 * TRACE_EVENT(napi_poll,
 *
 *     TP_PROTO(struct napi_struct *napi, int work, int budget),
 *
 *     TP_ARGS(napi, work, budget),
 *
 *     TP_fast_assign(
 *         __entry->napi = napi;
 *         __assign_str(dev_name);
 *         __entry->work = work;
 *         __entry->budget = budget;
 *     ),
 *
 *     TP_printk("napi poll on napi struct %p for device %s work %d budget %d",
 *               __entry->napi, __get_str(dev_name),
 *               __entry->work, __entry->budget)
 * );
 */

/*
 * TRACE_EVENT(dql_stall_detected,
 *
 *     TP_PROTO(unsigned short thrs, unsigned int len,
 *              unsigned long last_reap, unsigned long hist_head,
 *              unsigned long now, unsigned long *hist),
 *
 *     TP_ARGS(thrs, len, last_reap, hist_head, now, hist),
 *
 *     TP_fast_assign(
 *         __entry->thrs = thrs;
 *         __entry->len = len;
 *         __entry->last_reap = last_reap;
 *         __entry->hist_head = hist_head * BITS_PER_LONG;
 *         __entry->now = now;
 *         memcpy(__entry->hist, hist, sizeof(entry->hist));
 *     ),
 *
 *     TP_printk("thrs %u  len %u  last_reap %lu  hist_head %lu  now %lu  hist %016lx %016lx %016lx %016lx",
 *               __entry->thrs, __entry->len,
 *               __entry->last_reap, __entry->hist_head, __entry->now,
 *               __entry->hist[0], __entry->hist[1],
 *               __entry->hist[2], __entry->hist[3])
 * );
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
