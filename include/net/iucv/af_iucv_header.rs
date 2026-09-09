/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2006 IBM Corporation
 * IUCV protocol stack for Linux on zSeries
 * Version 1.0
 * Author(s): Jennifer Hunt <jenhunt@us.ibm.com>
 */

/* Dependencies supplied by the surrounding translation unit. */

pub const AF_IUCV: i32 = 32;
pub const PF_IUCV: i32 = AF_IUCV;

/* Connection and socket states */
pub const IUCV_CONNECTED: i32 = 1;
pub const IUCV_OPEN: i32 = 2;
pub const IUCV_BOUND: i32 = 3;
pub const IUCV_LISTEN: i32 = 4;
pub const IUCV_DISCONN: i32 = 5;
pub const IUCV_CLOSING: i32 = 6;
pub const IUCV_CLOSED: i32 = 7;

pub const IUCV_QUEUELEN_DEFAULT: u32 = 65535;
pub const IUCV_HIPER_MSGLIM_DEFAULT: u32 = 128;
pub const IUCV_CONN_TIMEOUT: u32 = HZ * 40;
pub const IUCV_DISCONN_TIMEOUT: u32 = HZ * 2;
pub const IUCV_CONN_IDLE_TIMEOUT: u32 = HZ * 60;
pub const IUCV_BUFSIZE_DEFAULT: u32 = 32768;

#[repr(C)]
pub struct sockaddr_iucv {
    pub siucv_family: sa_family_t,
    pub siucv_port: u16, // Reserved
    pub siucv_addr: u32, // Reserved
    pub siucv_nodeid: [i8; 8], // Reserved
    pub siucv_user_id: [i8; 8], // Guest User Id
    pub siucv_name: [i8; 8], // Application Name
}

#[repr(C)]
pub struct sock_msg_q {
    pub path: *mut iucv_path,
    pub msg: iucv_message,
    pub list: list_head,
    pub lock: spinlock_t,
}

pub const AF_IUCV_FLAG_ACK: u32 = 0x1;
pub const AF_IUCV_FLAG_SYN: u32 = 0x2;
pub const AF_IUCV_FLAG_FIN: u32 = 0x4;
pub const AF_IUCV_FLAG_WIN: u32 = 0x8;
pub const AF_IUCV_FLAG_SHT: u32 = 0x10;

#[repr(C, packed)]
pub struct af_iucv_trans_hdr {
    pub magic: u16,
    pub version: u8,
    pub flags: u8,
    pub window: u16,
    pub destNodeID: [i8; 8],
    pub destUserID: [i8; 8],
    pub destAppName: [i8; 16],
    pub srcNodeID: [i8; 8],
    pub srcUserID: [i8; 8],
    pub srcAppName: [i8; 16], // => 70 bytes
    pub iucv_hdr: iucv_message, // => 33 bytes
    pub pad: u8, // total 104 bytes
}

#[inline]
pub unsafe fn iucv_trans_hdr(skb: *mut sk_buff) -> *mut af_iucv_trans_hdr {
    skb_network_header(skb) as *mut af_iucv_trans_hdr
}

#[repr(C)]
pub enum iucv_tx_notify {
    TX_NOTIFY_OK = 0,
    TX_NOTIFY_UNREACHABLE = 1,
    TX_NOTIFY_TPQFULL = 2,
    TX_NOTIFY_GENERALERROR = 3,
    TX_NOTIFY_PENDING = 4,
    TX_NOTIFY_DELAYED_OK = 5,
    TX_NOTIFY_DELAYED_UNREACHABLE = 6,
    TX_NOTIFY_DELAYED_GENERALERROR = 7,
}

#[inline]
pub unsafe fn iucv_sk(__sk: *mut sock) -> *mut iucv_sock {
    __sk as *mut iucv_sock
}

pub const AF_IUCV_TRANS_IUCV: i32 = 0;
pub const AF_IUCV_TRANS_HIPER: i32 = 1;

#[repr(C)]
pub struct iucv_sock {
    pub sk: sock,
    pub src_user_id: [i8; 8],
    pub src_name: [i8; 8],
    pub dst_user_id: [i8; 8],
    pub dst_name: [i8; 8],
    pub accept_q: list_head,
    pub accept_q_lock: spinlock_t,
    pub parent: *mut sock,
    pub path: *mut iucv_path,
    pub hs_dev: *mut net_device,
    pub send_skb_q: sk_buff_head,
    pub backlog_skb_q: sk_buff_head,
    pub message_q: sock_msg_q,
    pub send_tag: u32,
    pub flags: u8,
    pub msglimit: u16,
    pub msglimit_peer: u16,
    pub skbs_in_xmit: atomic_t,
    pub msg_sent: atomic_t,
    pub msg_recv: atomic_t,
    pub pendings: atomic_t,
    pub transport: i32,
    pub sk_txnotify: Option<unsafe extern "C" fn(*mut sock, iucv_tx_notify)>,
}

#[repr(C)]
pub struct iucv_skb_cb {
    pub class: u32, // target class of message
    pub tag: u32, // tag associated with message
    pub offset: u32, // offset for skb receival
}

#[inline]
pub unsafe fn IUCV_SKB_CB(__skb: *mut sk_buff) -> *mut iucv_skb_cb {
    (*__skb).cb.as_mut_ptr() as *mut iucv_skb_cb
}

pub const SO_IPRMDATA_MSG: u32 = 0x0080; // send/recv IPRM_DATA msgs
pub const SO_MSGLIMIT: u32 = 0x1000; // get/set IUCV MSGLIMIT
pub const SO_MSGSIZE: u32 = 0x0800; // get maximum msgsize

pub const SCM_IUCV_TRGCLS: u32 = 0x0001; // target class control message

#[repr(C)]
pub struct iucv_sock_list {
    pub head: hlist_head,
    pub lock: rwlock_t,
    pub autobind_name: atomic_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
