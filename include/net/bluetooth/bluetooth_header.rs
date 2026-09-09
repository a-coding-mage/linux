/* SPDX-License-Identifier: GPL-2.0 */
/* BlueZ - Bluetooth protocol stack for Linux; translated from bluetooth.h. */

// Kernel dependencies supplied by other translated units.

pub const BT_SUBSYS_VERSION: i32 = 2;
pub const BT_SUBSYS_REVISION: i32 = 22;
pub const AF_BLUETOOTH: i32 = 31;
pub const PF_BLUETOOTH: i32 = AF_BLUETOOTH;
pub const BLUETOOTH_VER_1_1: i32 = 1;
pub const BLUETOOTH_VER_1_2: i32 = 2;
pub const BLUETOOTH_VER_2_0: i32 = 3;
pub const BLUETOOTH_VER_2_1: i32 = 4;
pub const BLUETOOTH_VER_4_0: i32 = 6;
pub const BT_SKB_RESERVE: usize = 8;
pub const BTPROTO_L2CAP: i32 = 0;
pub const BTPROTO_HCI: i32 = 1;
pub const BTPROTO_SCO: i32 = 2;
pub const BTPROTO_RFCOMM: i32 = 3;
pub const BTPROTO_BNEP: i32 = 4;
pub const BTPROTO_CMTP: i32 = 5;
pub const BTPROTO_HIDP: i32 = 6;
pub const BTPROTO_AVDTP: i32 = 7;
pub const BTPROTO_ISO: i32 = 8;
pub const BTPROTO_LAST: i32 = BTPROTO_ISO;
pub const SOL_HCI: i32 = 0;
pub const SOL_L2CAP: i32 = 6;
pub const SOL_SCO: i32 = 17;
pub const SOL_RFCOMM: i32 = 18;
pub const BT_SECURITY: i32 = 4;

#[repr(C)] pub struct bt_security { pub level: u8, pub key_size: u8 }
pub const BT_SECURITY_SDP: i32 = 0; pub const BT_SECURITY_LOW: i32 = 1;
pub const BT_SECURITY_MEDIUM: i32 = 2; pub const BT_SECURITY_HIGH: i32 = 3;
pub const BT_SECURITY_FIPS: i32 = 4;
pub const BT_DEFER_SETUP: i32 = 7; pub const BT_FLUSHABLE: i32 = 8;
pub const BT_FLUSHABLE_OFF: i32 = 0; pub const BT_FLUSHABLE_ON: i32 = 1;
pub const BT_POWER: i32 = 9;
#[repr(C)] pub struct bt_power { pub force_active: u8 }
pub const BT_POWER_FORCE_ACTIVE_OFF: i32 = 0; pub const BT_POWER_FORCE_ACTIVE_ON: i32 = 1;
pub const BT_CHANNEL_POLICY: i32 = 10;
pub const BT_CHANNEL_POLICY_BREDR_ONLY: i32 = 0;
pub const BT_CHANNEL_POLICY_BREDR_PREFERRED: i32 = 1;
pub const BT_CHANNEL_POLICY_AMP_PREFERRED: i32 = 2;
pub const BT_VOICE: i32 = 11;
#[repr(C)] pub struct bt_voice { pub setting: u16 }
pub const BT_VOICE_TRANSPARENT: u16 = 0x0003; pub const BT_VOICE_CVSD_16BIT: u16 = 0x0060;
pub const BT_VOICE_TRANSPARENT_16BIT: u16 = 0x0063;
pub const BT_SNDMTU: i32 = 12; pub const BT_RCVMTU: i32 = 13; pub const BT_PHY: i32 = 14;
pub const BT_PHY_BR_1M_1SLOT:u32=1<<0; pub const BT_PHY_BR_1M_3SLOT:u32=1<<1; pub const BT_PHY_BR_1M_5SLOT:u32=1<<2;
pub const BT_PHY_EDR_2M_1SLOT:u32=1<<3; pub const BT_PHY_EDR_2M_3SLOT:u32=1<<4; pub const BT_PHY_EDR_2M_5SLOT:u32=1<<5;
pub const BT_PHY_EDR_3M_1SLOT:u32=1<<6; pub const BT_PHY_EDR_3M_3SLOT:u32=1<<7; pub const BT_PHY_EDR_3M_5SLOT:u32=1<<8;
pub const BT_PHY_LE_1M_TX:u32=1<<9; pub const BT_PHY_LE_1M_RX:u32=1<<10; pub const BT_PHY_LE_2M_TX:u32=1<<11; pub const BT_PHY_LE_2M_RX:u32=1<<12; pub const BT_PHY_LE_CODED_TX:u32=1<<13; pub const BT_PHY_LE_CODED_RX:u32=1<<14;
pub const BT_PHY_BREDR_MASK:u32=0x1ff; pub const BT_PHY_LE_MASK:u32=0x7e00;
pub const BT_MODE:i32=15; pub const BT_MODE_BASIC:u8=0; pub const BT_MODE_ERTM:u8=1; pub const BT_MODE_STREAMING:u8=2; pub const BT_MODE_LE_FLOWCTL:u8=3; pub const BT_MODE_EXT_FLOWCTL:u8=4;
pub const BT_PKT_STATUS:i32=16; pub const BT_SCM_PKT_STATUS:u8=3; pub const BT_SCM_ERROR:u8=4;
pub const BT_ISO_QOS:i32=17; pub const BT_ISO_QOS_CIG_UNSET:u8=0xff; pub const BT_ISO_QOS_CIS_UNSET:u8=0xff; pub const BT_ISO_QOS_BIG_UNSET:u8=0xff; pub const BT_ISO_QOS_BIS_UNSET:u8=0xff; pub const BT_ISO_SYNC_TIMEOUT:u16=0x07d0;

#[repr(C)] pub struct bt_iso_io_qos { pub interval:u32, pub latency:u16, pub sdu:u16, pub phys:u8, pub rtn:u8 }
#[repr(C)] pub struct bt_iso_ucast_qos { pub cig:u8,pub cis:u8,pub sca:u8,pub packing:u8,pub framing:u8,pub r#in:bt_iso_io_qos,pub out:bt_iso_io_qos }
#[repr(C)] pub struct bt_iso_bcast_qos { pub big:u8,pub bis:u8,pub sync_factor:u8,pub packing:u8,pub framing:u8,pub r#in:bt_iso_io_qos,pub out:bt_iso_io_qos,pub encryption:u8,pub bcode:[u8;16],pub options:u8,pub skip:u16,pub sync_timeout:u16,pub sync_cte_type:u8,pub mse:u8,pub timeout:u16 }
#[repr(C)] pub union bt_iso_qos_union { pub ucast: std::mem::ManuallyDrop<bt_iso_ucast_qos>, pub bcast: std::mem::ManuallyDrop<bt_iso_bcast_qos> }
#[repr(C)] pub struct bt_iso_qos { pub value: bt_iso_qos_union }
pub const BT_ISO_PHY_1M:u32=1; pub const BT_ISO_PHY_2M:u32=2; pub const BT_ISO_PHY_CODED:u32=4; pub const BT_ISO_PHY_ANY:u32=7;
pub const BT_CODEC:i32=19;
#[repr(C, packed)] pub struct bt_codec_caps { pub len:u8, pub data:[u8;0] }
#[repr(C, packed)] pub struct bt_codec { pub id:u8,pub cid:u16,pub vid:u16,pub data_path:u8,pub num_caps:u8 }
#[repr(C, packed)] pub struct bt_codecs { pub num_codecs:u8,pub codecs:[bt_codec;0] }
pub const BT_CODEC_CVSD:u8=2; pub const BT_CODEC_TRANSPARENT:u8=3; pub const BT_CODEC_MSBC:u8=5; pub const BT_ISO_BASE:i32=20; pub const BT_PKT_SEQNUM:i32=22; pub const BT_SCM_PKT_SEQNUM:u8=5;

#[repr(i32)] pub enum bt_sock_state { BT_CONNECTED=1, BT_OPEN, BT_BOUND, BT_LISTEN, BT_CONNECT, BT_CONNECT2, BT_CONFIG, BT_DISCONN, BT_CLOSED }
pub unsafe fn state_to_string(state:i32)->&'static [u8] { match state { 1=>b"BT_CONNECTED\0",2=>b"BT_OPEN\0",3=>b"BT_BOUND\0",4=>b"BT_LISTEN\0",5=>b"BT_CONNECT\0",6=>b"BT_CONNECT2\0",7=>b"BT_CONFIG\0",8=>b"BT_DISCONN\0",9=>b"BT_CLOSED\0", _=>b"invalid state\0" } }
#[repr(C, packed)] #[derive(Copy,Clone)] pub struct bdaddr_t { pub b:[u8;6] }
pub const BDADDR_BREDR:u8=0; pub const BDADDR_LE_PUBLIC:u8=1; pub const BDADDR_LE_RANDOM:u8=2;
pub const BT_SK_DEFER_SETUP:i32=0; pub const BT_SK_SUSPEND:i32=1; pub const BT_SK_PKT_STATUS:i32=2; pub const BT_SK_PKT_SEQNUM:i32=3;
extern "C" { pub fn baswap(dst:*mut bdaddr_t, src:*const bdaddr_t); pub fn bt_to_errno(code:u16)->i32; pub fn bt_status(err:i32)->u8; }

// The remaining declarations depend on kernel types supplied by other units.
extern "C" {
    pub fn bt_sock_register(proto:i32, ops:*const core::ffi::c_void)->i32;
    pub fn bt_sock_unregister(proto:i32);
    pub fn hci_sock_init()->i32; pub fn hci_sock_cleanup();
    pub fn bt_sysfs_init()->i32; pub fn bt_sysfs_cleanup();
    pub fn l2cap_init()->i32; pub fn l2cap_exit();
    pub fn mgmt_init()->i32; pub fn mgmt_exit();
}

#[repr(C)] pub struct bt_sock { pub sk: core::ffi::c_void, pub accept_q: core::ffi::c_void, pub accept_q_lock: core::ffi::c_void, pub parent:*mut core::ffi::c_void, pub flags:usize, pub skb_msg_name:Option<unsafe extern "C" fn(*mut core::ffi::c_void,*mut core::ffi::c_void,*mut i32)>, pub skb_put_cmsg:Option<unsafe extern "C" fn(*mut core::ffi::c_void,*mut core::ffi::c_void,*mut core::ffi::c_void)> }
#[repr(C)] pub struct bt_sock_list { pub head:core::ffi::c_void, pub lock:core::ffi::c_void }
#[repr(C)] pub struct l2cap_ctrl { pub flags:u8, pub reqseq:u16, pub txseq:u16, pub retries:u8, pub psm:u16, pub bdaddr:bdaddr_t, pub chan:*mut core::ffi::c_void }
#[repr(C)] pub struct hci_ctrl { pub sk:*mut core::ffi::c_void, pub opcode:u16, pub req_flags:u8, pub req_event:u8 }
#[repr(C)] pub struct mgmt_ctrl { pub hdev:*mut core::ffi::c_void, pub opcode:u16 }
#[repr(C)] pub union bt_skb_cb_union { pub l2cap:l2cap_ctrl, pub hci:hci_ctrl, pub mgmt:mgmt_ctrl, pub creds:core::ffi::c_void }
#[repr(C)] pub struct bt_skb_cb { pub pkt_type:u8, pub force_active:u8, pub expect:u16, pub pkt_seqnum:u16, pub incoming:u8, pub pkt_status:u8, pub value:bt_skb_cb_union }
pub type hci_req_complete_t=unsafe extern "C" fn(*mut core::ffi::c_void,u8,u16);
pub type hci_req_complete_skb_t=unsafe extern "C" fn(*mut core::ffi::c_void,u8,u16,*mut core::ffi::c_void);
pub const HCI_REQ_START:u8=1; pub const HCI_REQ_SKB:u8=2;
extern "C" {
    pub fn hci_req_cmd_complete(hdev:*mut core::ffi::c_void, opcode:u16, status:u8, req_complete:*mut hci_req_complete_t, req_complete_skb:*mut hci_req_complete_skb_t);
    pub fn bt_sock_link(l:*mut bt_sock_list,s:*mut core::ffi::c_void);
    pub fn bt_sock_unlink(l:*mut bt_sock_list,s:*mut core::ffi::c_void);
    pub fn bt_sock_linked(l:*mut bt_sock_list,s:*mut core::ffi::c_void)->bool;
    pub fn bt_sock_recvmsg(sock:*mut core::ffi::c_void,msg:*mut core::ffi::c_void,len:usize,flags:i32)->i32;
    pub fn bt_sock_stream_recvmsg(sock:*mut core::ffi::c_void,msg:*mut core::ffi::c_void,len:usize,flags:i32)->i32;
    pub fn bt_sock_ioctl(sock:*mut core::ffi::c_void,cmd:u32,arg:usize)->i32;
    pub fn bt_sock_wait_state(sk:*mut core::ffi::c_void,state:i32,timeo:usize)->i32;
    pub fn bt_sock_wait_ready(sk:*mut core::ffi::c_void,msg_flags:u32)->i32;
    pub fn bt_accept_enqueue(parent:*mut core::ffi::c_void,sk:*mut core::ffi::c_void,bh:bool);
    pub fn bt_accept_unlink(sk:*mut core::ffi::c_void);
    pub fn bt_accept_dequeue(parent:*mut core::ffi::c_void,newsock:*mut core::ffi::c_void)->*mut core::ffi::c_void;
    pub fn hci_sock_set_flag(sk:*mut core::ffi::c_void,nr:i32); pub fn hci_sock_clear_flag(sk:*mut core::ffi::c_void,nr:i32);
    pub fn hci_sock_test_flag(sk:*mut core::ffi::c_void,nr:i32)->i32; pub fn hci_sock_get_channel(sk:*mut core::ffi::c_void)->u16; pub fn hci_sock_get_cookie(sk:*mut core::ffi::c_void)->u32;
    pub fn bt_procfs_init(net:*mut core::ffi::c_void,name:*const i8,sk_list:*mut bt_sock_list,seq_show:*const core::ffi::c_void)->i32;
    pub fn bt_procfs_cleanup(net:*mut core::ffi::c_void,name:*const i8);
    pub fn sco_init()->i32; pub fn sco_exit(); pub fn iso_init()->i32; pub fn iso_exit()->i32; pub fn iso_inited()->bool;
    pub fn mgmt_cleanup(sk:*mut core::ffi::c_void); pub fn bt_sock_reclassify_lock(sk:*mut core::ffi::c_void,proto:i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
