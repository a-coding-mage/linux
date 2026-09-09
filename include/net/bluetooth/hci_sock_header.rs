/* SPDX-License-Identifier: GPL-2.0 */
/*
   BlueZ - Bluetooth protocol stack for Linux
   Copyright (C) 2000-2001 Qualcomm Incorporated

   Written 2000,2001 by Maxim Krasnyansky <maxk@qualcomm.com>

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
   OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF THIRD PARTY RIGHTS.
   IN NO EVENT SHALL THE COPYRIGHT HOLDER(S) AND AUTHOR(S) BE LIABLE FOR ANY
   CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES
   WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
   ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
   OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

   ALL LIABILITY, INCLUDING LIABILITY FOR INFRINGEMENT OF ANY PATENTS,
   COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS, RELATING TO USE OF THIS
   SOFTWARE IS DISCLAIMED.
*/

/* Socket options */
pub const HCI_DATA_DIR: i32 = 1;
pub const HCI_FILTER: i32 = 2;
pub const HCI_TIME_STAMP: i32 = 3;

/* CMSG flags */
pub const HCI_CMSG_DIR: u8 = 0x01;
pub const HCI_CMSG_TSTAMP: u8 = 0x02;

#[repr(C)]
pub struct sockaddr_hci {
    pub hci_family: sa_family_t,
    pub hci_dev: u16,
    pub hci_channel: u16,
}
pub const HCI_DEV_NONE: u16 = 0xffff;

pub const HCI_CHANNEL_RAW: u16 = 0;
pub const HCI_CHANNEL_USER: u16 = 1;
pub const HCI_CHANNEL_MONITOR: u16 = 2;
pub const HCI_CHANNEL_CONTROL: u16 = 3;
pub const HCI_CHANNEL_LOGGING: u16 = 4;

#[repr(C)]
pub struct hci_filter {
    pub type_mask: c_ulong,
    pub event_mask: [c_ulong; 2],
    pub opcode: __le16,
}

#[repr(C)]
pub struct hci_ufilter {
    pub type_mask: __u32,
    pub event_mask: [__u32; 2],
    pub opcode: __le16,
}

pub const HCI_FLT_TYPE_BITS: u32 = 31;
pub const HCI_FLT_EVENT_BITS: u32 = 63;
pub const HCI_FLT_OGF_BITS: u32 = 63;
pub const HCI_FLT_OCF_BITS: u32 = 127;

/* Ioctl defines. The _IOW/_IOR encodings are architecture-dependent external macros. */
/* HCIDEVUP       = _IOW('H', 201, int); */
/* HCIDEVDOWN     = _IOW('H', 202, int); */
/* HCIDEVRESET    = _IOW('H', 203, int); */
/* HCIDEVRESTAT   = _IOW('H', 204, int); */
/* HCIGETDEVLIST  = _IOR('H', 210, int); */
/* HCIGETDEVINFO  = _IOR('H', 211, int); */
/* HCIGETCONNLIST = _IOR('H', 212, int); */
/* HCIGETCONNINFO = _IOR('H', 213, int); */
/* HCIGETAUTHINFO = _IOR('H', 215, int); */
/* HCISETRAW      = _IOW('H', 220, int); */
/* HCISETSCAN     = _IOW('H', 221, int); */
/* HCISETAUTH     = _IOW('H', 222, int); */
/* HCISETENCRYPT  = _IOW('H', 223, int); */
/* HCISETPTYPE    = _IOW('H', 224, int); */
/* HCISETLINKPOL  = _IOW('H', 225, int); */
/* HCISETLINKMODE = _IOW('H', 226, int); */
/* HCISETACLMTU   = _IOW('H', 227, int); */
/* HCISETSCOMTU   = _IOW('H', 228, int); */
/* HCIBLOCKADDR   = _IOW('H', 230, int); */
/* HCIUNBLOCKADDR = _IOW('H', 231, int); */
/* HCIINQUIRY     = _IOR('H', 240, int); */

/* Ioctl requests structures */
#[repr(C)]
pub struct hci_dev_stats {
    pub err_rx: __u32,
    pub err_tx: __u32,
    pub cmd_tx: __u32,
    pub evt_rx: __u32,
    pub acl_tx: __u32,
    pub acl_rx: __u32,
    pub sco_tx: __u32,
    pub sco_rx: __u32,
    pub byte_rx: __u32,
    pub byte_tx: __u32,
}

#[repr(C)]
pub struct hci_dev_info {
    pub dev_id: __u16,
    pub name: [c_char; 8],
    pub bdaddr: bdaddr_t,
    pub flags: __u32,
    pub type_: __u8,
    pub features: [__u8; 8],
    pub pkt_type: __u32,
    pub link_policy: __u32,
    pub link_mode: __u32,
    pub acl_mtu: __u16,
    pub acl_pkts: __u16,
    pub sco_mtu: __u16,
    pub sco_pkts: __u16,
    pub stat: hci_dev_stats,
}

#[repr(C)]
pub struct hci_conn_info {
    pub handle: __u16,
    pub bdaddr: bdaddr_t,
    pub type_: __u8,
    pub out: __u8,
    pub state: __u16,
    pub link_mode: __u32,
}

#[repr(C)]
pub struct hci_dev_req {
    pub dev_id: __u16,
    pub dev_opt: __u32,
}

#[repr(C)]
pub struct hci_dev_list_req {
    pub dev_num: __u16,
    pub dev_req: [hci_dev_req; 0], /* __counted_by(dev_num) */
}

#[repr(C)]
pub struct hci_conn_list_req {
    pub dev_id: __u16,
    pub conn_num: __u16,
    pub conn_info: [hci_conn_info; 0],
}

#[repr(C)]
pub struct hci_conn_info_req {
    pub bdaddr: bdaddr_t,
    pub type_: __u8,
    pub conn_info: [hci_conn_info; 0],
}

#[repr(C)]
pub struct hci_auth_info_req {
    pub bdaddr: bdaddr_t,
    pub type_: __u8,
}

#[repr(C)]
pub struct hci_inquiry_req {
    pub dev_id: __u16,
    pub flags: __u16,
    pub lap: [__u8; 3],
    pub length: __u8,
    pub num_rsp: __u8,
}

pub const IREQ_CACHE_FLUSH: u16 = 0x0001;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
