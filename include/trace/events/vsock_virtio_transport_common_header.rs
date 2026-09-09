/* SPDX-License-Identifier: GPL-2.0 */
// Translated from vsock_virtio_transport_common.h.
// The C tracepoint include and header guard are intentionally omitted.

// These constants are supplied by the virtio vsock headers.
extern "C" {
    pub static VIRTIO_VSOCK_TYPE_STREAM: u16;
    pub static VIRTIO_VSOCK_TYPE_SEQPACKET: u16;
    pub static VIRTIO_VSOCK_OP_INVALID: u16;
    pub static VIRTIO_VSOCK_OP_REQUEST: u16;
    pub static VIRTIO_VSOCK_OP_RESPONSE: u16;
    pub static VIRTIO_VSOCK_OP_RST: u16;
    pub static VIRTIO_VSOCK_OP_SHUTDOWN: u16;
    pub static VIRTIO_VSOCK_OP_RW: u16;
    pub static VIRTIO_VSOCK_OP_CREDIT_UPDATE: u16;
    pub static VIRTIO_VSOCK_OP_CREDIT_REQUEST: u16;
}

#[inline]
pub unsafe fn show_type(val: u16) -> &'static str {
    if val == VIRTIO_VSOCK_TYPE_STREAM {
        "STREAM"
    } else if val == VIRTIO_VSOCK_TYPE_SEQPACKET {
        "SEQPACKET"
    } else {
        "UNKNOWN"
    }
}

#[inline]
pub unsafe fn show_op(val: u16) -> &'static str {
    if val == VIRTIO_VSOCK_OP_INVALID { "INVALID" }
    else if val == VIRTIO_VSOCK_OP_REQUEST { "REQUEST" }
    else if val == VIRTIO_VSOCK_OP_RESPONSE { "RESPONSE" }
    else if val == VIRTIO_VSOCK_OP_RST { "RST" }
    else if val == VIRTIO_VSOCK_OP_SHUTDOWN { "SHUTDOWN" }
    else if val == VIRTIO_VSOCK_OP_RW { "RW" }
    else if val == VIRTIO_VSOCK_OP_CREDIT_UPDATE { "CREDIT_UPDATE" }
    else if val == VIRTIO_VSOCK_OP_CREDIT_REQUEST { "CREDIT_REQUEST" }
    else { "UNKNOWN" }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VirtioTransportAllocPkt {
    pub src_cid: u32,
    pub src_port: u32,
    pub dst_cid: u32,
    pub dst_port: u32,
    pub len: u32,
    pub type_: u16,
    pub op: u16,
    pub flags: u32,
    pub zcopy: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VirtioTransportRecvPkt {
    pub src_cid: u32,
    pub src_port: u32,
    pub dst_cid: u32,
    pub dst_port: u32,
    pub len: u32,
    pub type_: u16,
    pub op: u16,
    pub flags: u32,
    pub buf_alloc: u32,
    pub fwd_cnt: u32,
}

#[inline]
pub const fn virtio_transport_alloc_pkt(
    src_cid: u32, src_port: u32, dst_cid: u32, dst_port: u32,
    len: u32, type_: u16, op: u16, flags: u32, zcopy: bool,
) -> VirtioTransportAllocPkt {
    VirtioTransportAllocPkt { src_cid, src_port, dst_cid, dst_port, len, type_, op, flags, zcopy }
}

#[inline]
pub const fn virtio_transport_recv_pkt(
    src_cid: u32, src_port: u32, dst_cid: u32, dst_port: u32,
    len: u32, type_: u16, op: u16, flags: u32, buf_alloc: u32, fwd_cnt: u32,
) -> VirtioTransportRecvPkt {
    VirtioTransportRecvPkt { src_cid, src_port, dst_cid, dst_port, len, type_, op, flags, buf_alloc, fwd_cnt }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
