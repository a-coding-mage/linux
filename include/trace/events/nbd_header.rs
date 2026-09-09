/* SPDX-License-Identifier: GPL-2.0 */

// This file is the Rust translation of trace/events/nbd.h.
// The C tracepoint include and trace-generation machinery are supplied by
// the surrounding kernel build and are intentionally not reimplemented here.

#[repr(C)]
pub struct request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nbd_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct NbdTransportEventEntry {
    pub req: *mut request,
    pub handle: u64,
}

#[repr(C)]
pub struct NbdSendRequestEntry {
    pub nbd_request: *mut nbd_request,
    pub dev_index: u64,
    pub request: *mut request,
}

// DECLARE_EVENT_CLASS(nbd_transport_event)
// TP_PROTO(struct request *req, u64 handle)
// TP_ARGS(req, handle)
// TP_fast_assign:
//     __entry->req = req;
//     __entry->handle = handle;
// TP_printk("nbd transport event: request %p, handle 0x%016llx",
//           __entry->req, __entry->handle)
pub type nbd_transport_event = NbdTransportEventEntry;

// DEFINE_EVENT(nbd_transport_event, nbd_header_sent,
//              TP_PROTO(struct request *req, u64 handle),
//              TP_ARGS(req, handle))
// DEFINE_EVENT(nbd_transport_event, nbd_payload_sent,
//              TP_PROTO(struct request *req, u64 handle),
//              TP_ARGS(req, handle))
// DEFINE_EVENT(nbd_transport_event, nbd_header_received,
//              TP_PROTO(struct request *req, u64 handle),
//              TP_ARGS(req, handle))
// DEFINE_EVENT(nbd_transport_event, nbd_payload_received,
//              TP_PROTO(struct request *req, u64 handle),
//              TP_ARGS(req, handle))

// DECLARE_EVENT_CLASS(nbd_send_request)
// TP_PROTO(struct nbd_request *nbd_request, int index, struct request *rq)
// TP_ARGS(nbd_request, index, rq)
// TP_fast_assign:
//     __entry->nbd_request = NULL;
//     __entry->dev_index = index;
//     __entry->request = rq;
// TP_printk("nbd%lld: request %p", __entry->dev_index, __entry->request)
pub type nbd_send_request = NbdSendRequestEntry;

// NBD_DEFINE_EVENT expands to DEFINE_EVENT_WRITABLE when that tracepoint
// facility is available, and otherwise expands to DEFINE_EVENT.  The size
// argument is sizeof(struct nbd_request).
// NBD_DEFINE_EVENT(nbd_send_request, nbd_send_request,
//                  TP_PROTO(struct nbd_request *nbd_request, int index,
//                           struct request *rq),
//                  TP_ARGS(nbd_request, index, rq),
//                  sizeof(struct nbd_request))

// The C header includes <trace/define_trace.h> outside its include guard.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
