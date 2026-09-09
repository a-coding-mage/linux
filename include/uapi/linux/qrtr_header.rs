/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* C dependencies: <linux/socket.h>, <linux/types.h> */

pub const QRTR_NODE_BCAST: u32 = 0xffff_ffffu32;
pub const QRTR_PORT_CTRL: u32 = 0xffff_fffeu32;

#[repr(C)]
pub struct sockaddr_qrtr {
    pub sq_family: __kernel_sa_family_t,
    pub sq_node: __u32,
    pub sq_port: __u32,
}

#[repr(C)]
pub enum qrtr_pkt_type {
    QRTR_TYPE_DATA = 1,
    QRTR_TYPE_HELLO = 2,
    QRTR_TYPE_BYE = 3,
    QRTR_TYPE_NEW_SERVER = 4,
    QRTR_TYPE_DEL_SERVER = 5,
    QRTR_TYPE_DEL_CLIENT = 6,
    QRTR_TYPE_RESUME_TX = 7,
    QRTR_TYPE_EXIT = 8,
    QRTR_TYPE_PING = 9,
    QRTR_TYPE_NEW_LOOKUP = 10,
    QRTR_TYPE_DEL_LOOKUP = 11,
}

#[repr(C, packed)]
pub struct qrtr_ctrl_pkt {
    pub cmd: __le32,
    pub payload: qrtr_ctrl_pkt_payload,
}

#[repr(C)]
pub union qrtr_ctrl_pkt_payload {
    pub server: qrtr_ctrl_pkt_server,
    pub client: qrtr_ctrl_pkt_client,
}

#[repr(C)]
pub struct qrtr_ctrl_pkt_server {
    pub service: __le32,
    pub instance: __le32,
    pub node: __le32,
    pub port: __le32,
}

#[repr(C)]
pub struct qrtr_ctrl_pkt_client {
    pub node: __le32,
    pub port: __le32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
