/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by the Linux UAPI Rust environment:
// use of virtio_vsock types is declared by the corresponding C header.

/*
 * vsockmon is the AF_VSOCK packet capture device. Packets captured have the
 * following layout:
 *
 *   +-----------------------------------+
 *   |           vsockmon header         |
 *   |      (struct af_vsockmon_hdr)     |
 *   +-----------------------------------+
 *   |          transport header         |
 *   | (af_vsockmon_hdr->len bytes long) |
 *   +-----------------------------------+
 *   |              payload              |
 *   |       (until end of packet)       |
 *   +-----------------------------------+
 *
 * The vsockmon header is a transport-independent description of the packet.
 * It duplicates some of the information from the transport header so that
 * no transport-specific knowledge is necessary to process packets.
 *
 * The transport header is useful for low-level transport-specific packet
 * analysis. Transport type is given in af_vsockmon_hdr->transport and
 * transport header length is given in af_vsockmon_hdr->len.
 *
 * If af_vsockmon_hdr->op is AF_VSOCK_OP_PAYLOAD then the payload follows the
 * transport header. Other ops do not have a payload.
 */
#[repr(C)]
pub struct af_vsockmon_hdr {
    pub src_cid: __le64,
    pub dst_cid: __le64,
    pub src_port: __le32,
    pub dst_port: __le32,
    pub op: __le16,        /* enum af_vsockmon_op */
    pub transport: __le16, /* enum af_vsockmon_transport */
    pub len: __le16,       /* Transport header length */
    pub reserved: [__u8; 2],
}

#[repr(i32)]
pub enum af_vsockmon_op {
    AF_VSOCK_OP_UNKNOWN = 0,
    AF_VSOCK_OP_CONNECT = 1,
    AF_VSOCK_OP_DISCONNECT = 2,
    AF_VSOCK_OP_CONTROL = 3,
    AF_VSOCK_OP_PAYLOAD = 4,
}

#[repr(i32)]
pub enum af_vsockmon_transport {
    AF_VSOCK_TRANSPORT_UNKNOWN = 0,
    AF_VSOCK_TRANSPORT_NO_INFO = 1, /* No transport information */

    /* Transport header type: struct virtio_vsock_hdr */
    AF_VSOCK_TRANSPORT_VIRTIO = 2,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
