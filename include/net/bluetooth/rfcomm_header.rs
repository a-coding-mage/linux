/* SPDX-License-Identifier: GPL-2.0 */
/*
   RFCOMM implementation for Linux Bluetooth stack (BlueZ)
   Copyright (C) 2002 Maxim Krasnyansky <maxk@qualcomm.com>
   Copyright (C) 2002 Marcel Holtmann <marcel@holtmann.org>

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

pub const RFCOMM_CONN_TIMEOUT: u32 = HZ * 30;
pub const RFCOMM_DISC_TIMEOUT: u32 = HZ * 20;
pub const RFCOMM_AUTH_TIMEOUT: u32 = HZ * 25;
pub const RFCOMM_IDLE_TIMEOUT: u32 = HZ * 2;
pub const RFCOMM_DEFAULT_MTU: u8 = 127;
pub const RFCOMM_DEFAULT_CREDITS: u8 = 7;
pub const RFCOMM_MAX_CREDITS: u8 = 40;
pub const RFCOMM_SKB_HEAD_RESERVE: u8 = 8;
pub const RFCOMM_SKB_TAIL_RESERVE: u8 = 2;
pub const RFCOMM_SKB_RESERVE: u8 = RFCOMM_SKB_HEAD_RESERVE + RFCOMM_SKB_TAIL_RESERVE;
pub const RFCOMM_SABM: u8 = 0x2f;
pub const RFCOMM_DISC: u8 = 0x43;
pub const RFCOMM_UA: u8 = 0x63;
pub const RFCOMM_DM: u8 = 0x0f;
pub const RFCOMM_UIH: u8 = 0xef;
pub const RFCOMM_TEST: u8 = 0x08;
pub const RFCOMM_FCON: u8 = 0x28;
pub const RFCOMM_FCOFF: u8 = 0x18;
pub const RFCOMM_MSC: u8 = 0x38;
pub const RFCOMM_RPN: u8 = 0x24;
pub const RFCOMM_RLS: u8 = 0x14;
pub const RFCOMM_PN: u8 = 0x20;
pub const RFCOMM_NSC: u8 = 0x04;
pub const RFCOMM_V24_FC: u8 = 0x02;
pub const RFCOMM_V24_RTC: u8 = 0x04;
pub const RFCOMM_V24_RTR: u8 = 0x08;
pub const RFCOMM_V24_IC: u8 = 0x40;
pub const RFCOMM_V24_DV: u8 = 0x80;

pub const RFCOMM_RPN_BR_2400: u8 = 0x0;
pub const RFCOMM_RPN_BR_4800: u8 = 0x1;
pub const RFCOMM_RPN_BR_7200: u8 = 0x2;
pub const RFCOMM_RPN_BR_9600: u8 = 0x3;
pub const RFCOMM_RPN_BR_19200: u8 = 0x4;
pub const RFCOMM_RPN_BR_38400: u8 = 0x5;
pub const RFCOMM_RPN_BR_57600: u8 = 0x6;
pub const RFCOMM_RPN_BR_115200: u8 = 0x7;
pub const RFCOMM_RPN_BR_230400: u8 = 0x8;
pub const RFCOMM_RPN_DATA_5: u8 = 0x0;
pub const RFCOMM_RPN_DATA_6: u8 = 0x1;
pub const RFCOMM_RPN_DATA_7: u8 = 0x2;
pub const RFCOMM_RPN_DATA_8: u8 = 0x3;
pub const RFCOMM_RPN_STOP_1: u8 = 0;
pub const RFCOMM_RPN_STOP_15: u8 = 1;
pub const RFCOMM_RPN_PARITY_NONE: u8 = 0x0;
pub const RFCOMM_RPN_PARITY_ODD: u8 = 0x1;
pub const RFCOMM_RPN_PARITY_EVEN: u8 = 0x3;
pub const RFCOMM_RPN_PARITY_MARK: u8 = 0x5;
pub const RFCOMM_RPN_PARITY_SPACE: u8 = 0x7;
pub const RFCOMM_RPN_FLOW_NONE: u8 = 0x00;
pub const RFCOMM_RPN_XON_CHAR: u8 = 0x11;
pub const RFCOMM_RPN_XOFF_CHAR: u8 = 0x13;
pub const RFCOMM_RPN_PM_BITRATE: u16 = 0x0001;
pub const RFCOMM_RPN_PM_DATA: u16 = 0x0002;
pub const RFCOMM_RPN_PM_STOP: u16 = 0x0004;
pub const RFCOMM_RPN_PM_PARITY: u16 = 0x0008;
pub const RFCOMM_RPN_PM_PARITY_TYPE: u16 = 0x0010;
pub const RFCOMM_RPN_PM_XON: u16 = 0x0020;
pub const RFCOMM_RPN_PM_XOFF: u16 = 0x0040;
pub const RFCOMM_RPN_PM_FLOW: u16 = 0x3F00;
pub const RFCOMM_RPN_PM_ALL: u16 = 0x3F7F;

#[repr(C, packed)] pub struct rfcomm_hdr { pub addr: u8, pub ctrl: u8, pub len: u8 }
#[repr(C, packed)] pub struct rfcomm_cmd { pub addr: u8, pub ctrl: u8, pub len: u8, pub fcs: u8 }
#[repr(C, packed)] pub struct rfcomm_mcc { pub type_: u8, pub len: u8 }
#[repr(C, packed)] pub struct rfcomm_pn { pub dlci: u8, pub flow_ctrl: u8, pub priority: u8, pub ack_timer: u8, pub mtu: __le16, pub max_retrans: u8, pub credits: u8 }
#[repr(C, packed)] pub struct rfcomm_rpn { pub dlci: u8, pub bit_rate: u8, pub line_settings: u8, pub flow_ctrl: u8, pub xon_char: u8, pub xoff_char: u8, pub param_mask: __le16 }
#[repr(C, packed)] pub struct rfcomm_rls { pub dlci: u8, pub status: u8 }
#[repr(C, packed)] pub struct rfcomm_msc { pub dlci: u8, pub v24_sig: u8 }

#[repr(C)] pub struct rfcomm_session { pub list: list_head, pub sock: *mut socket, pub timer: timer_list, pub state: c_ulong, pub flags: c_ulong, pub initiator: c_int, pub cfc: c_int, pub mtu: uint, pub dlcs: list_head }
#[repr(C)] pub struct rfcomm_dlc {
    pub list: list_head, pub session: *mut rfcomm_session, pub tx_queue: sk_buff_head, pub timer: timer_list,
    pub lock: mutex, pub state: c_ulong, pub flags: c_ulong, pub refcnt: refcount_t, pub dlci: u8, pub addr: u8,
    pub priority: u8, pub v24_sig: u8, pub remote_v24_sig: u8, pub mscex: u8, pub out: u8, pub sec_level: u8,
    pub role_switch: u8, pub defer_setup: u32, pub mtu: uint, pub cfc: uint, pub rx_credits: uint, pub tx_credits: uint,
    pub owner: *mut c_void,
    pub data_ready: Option<unsafe extern "C" fn(*mut rfcomm_dlc, *mut sk_buff)>,
    pub state_change: Option<unsafe extern "C" fn(*mut rfcomm_dlc, c_int)>,
    pub modem_status: Option<unsafe extern "C" fn(*mut rfcomm_dlc, u8)>,
}

pub const RFCOMM_RX_THROTTLED: u32 = 0; pub const RFCOMM_TX_THROTTLED: u32 = 1; pub const RFCOMM_TIMED_OUT: u32 = 2;
pub const RFCOMM_MSC_PENDING: u32 = 3; pub const RFCOMM_SEC_PENDING: u32 = 4; pub const RFCOMM_AUTH_PENDING: u32 = 5;
pub const RFCOMM_AUTH_ACCEPT: u32 = 6; pub const RFCOMM_AUTH_REJECT: u32 = 7; pub const RFCOMM_DEFER_SETUP: u32 = 8; pub const RFCOMM_ENC_DROP: u32 = 9;
pub const RFCOMM_SCHED_WAKEUP: u32 = 31;
pub const RFCOMM_MSCEX_TX: u8 = 1; pub const RFCOMM_MSCEX_RX: u8 = 2; pub const RFCOMM_MSCEX_OK: u8 = RFCOMM_MSCEX_TX + RFCOMM_MSCEX_RX;
pub const RFCOMM_CFC_UNKNOWN: i32 = -1; pub const RFCOMM_CFC_DISABLED: u8 = 0; pub const RFCOMM_CFC_ENABLED: u8 = RFCOMM_MAX_CREDITS;

extern "C" {
    pub fn rfcomm_send_rpn(s: *mut rfcomm_session, cr: c_int, dlci: u8, bit_rate: u8, data_bits: u8, stop_bits: u8, parity: u8, flow_ctrl_settings: u8, xon_char: u8, xoff_char: u8, param_mask: u16) -> c_int;
    pub fn rfcomm_dlc_send_rpn(d: *mut rfcomm_dlc, bit_rate: u8, data_bits: u8, stop_bits: u8, parity: u8, flow_ctrl_settings: u8, xon_char: u8, xoff_char: u8, param_mask: u16) -> c_int;
    pub fn rfcomm_dlc_alloc(prio: gfp_t) -> *mut rfcomm_dlc; pub fn rfcomm_dlc_free(d: *mut rfcomm_dlc); pub fn rfcomm_dlc_open(d: *mut rfcomm_dlc, src: *mut bdaddr_t, dst: *mut bdaddr_t, channel: u8) -> c_int; pub fn rfcomm_dlc_close(d: *mut rfcomm_dlc, reason: c_int) -> c_int; pub fn rfcomm_dlc_send(d: *mut rfcomm_dlc, skb: *mut sk_buff) -> c_int; pub fn rfcomm_dlc_send_noerror(d: *mut rfcomm_dlc, skb: *mut sk_buff);
    pub fn rfcomm_dlc_set_modem_status(d: *mut rfcomm_dlc, v24_sig: u8) -> c_int; pub fn rfcomm_dlc_get_modem_status(d: *mut rfcomm_dlc, v24_sig: *mut u8) -> c_int; pub fn rfcomm_dlc_accept(d: *mut rfcomm_dlc); pub fn rfcomm_dlc_exists(src: *mut bdaddr_t, dst: *mut bdaddr_t, channel: u8) -> *mut rfcomm_dlc;
    pub fn __rfcomm_dlc_throttle(d: *mut rfcomm_dlc); pub fn __rfcomm_dlc_unthrottle(d: *mut rfcomm_dlc); pub fn rfcomm_session_getaddr(s: *mut rfcomm_session, src: *mut bdaddr_t, dst: *mut bdaddr_t); pub fn rfcomm_init_sockets() -> c_int; pub fn rfcomm_cleanup_sockets(); pub fn rfcomm_connect_ind(s: *mut rfcomm_session, channel: u8, d: *mut *mut rfcomm_dlc) -> c_int;
}

#[inline] pub unsafe fn rfcomm_dlc_hold(d: *mut rfcomm_dlc) { refcount_inc(&mut (*d).refcnt); }
#[inline] pub unsafe fn rfcomm_dlc_put(d: *mut rfcomm_dlc) { if refcount_dec_and_test(&mut (*d).refcnt) { rfcomm_dlc_free(d); } }
#[inline] pub unsafe fn rfcomm_dlc_throttle(d: *mut rfcomm_dlc) { if !test_and_set_bit(RFCOMM_RX_THROTTLED, &mut (*d).flags) { __rfcomm_dlc_throttle(d); } }
#[inline] pub unsafe fn rfcomm_dlc_unthrottle(d: *mut rfcomm_dlc) { if test_and_clear_bit(RFCOMM_RX_THROTTLED, &mut (*d).flags) { __rfcomm_dlc_unthrottle(d); } }

#[repr(C)] pub struct sockaddr_rc { pub rc_family: sa_family_t, pub rc_bdaddr: bdaddr_t, pub rc_channel: u8 }
pub const RFCOMM_CONNINFO: u32 = 0x02;
#[repr(C)] pub struct rfcomm_conninfo { pub hci_handle: __u16, pub dev_class: [__u8; 3] }
pub const RFCOMM_LM: u32 = 0x03; pub const RFCOMM_LM_MASTER: u16 = 0x0001; pub const RFCOMM_LM_AUTH: u16 = 0x0002; pub const RFCOMM_LM_ENCRYPT: u16 = 0x0004; pub const RFCOMM_LM_TRUSTED: u16 = 0x0008; pub const RFCOMM_LM_RELIABLE: u16 = 0x0010; pub const RFCOMM_LM_SECURE: u16 = 0x0020; pub const RFCOMM_LM_FIPS: u16 = 0x0040;
#[repr(C)] pub struct rfcomm_pinfo { pub bt: bt_sock, pub src: bdaddr_t, pub dst: bdaddr_t, pub dlc: *mut rfcomm_dlc, pub channel: u8, pub sec_level: u8, pub role_switch: u8 }

pub const RFCOMM_MAX_DEV: u32 = 256;
// RFCOMMCREATEDEV, RFCOMMRELEASEDEV, RFCOMMGETDEVLIST, RFCOMMGETDEVINFO, and RFCOMMSTEALDLC
// retain their Linux _IOW/_IOR ioctl encodings: RFCOMMCREATEDEV = _IOW('R', 200, int),
// RFCOMMRELEASEDEV = _IOW('R', 201, int), RFCOMMGETDEVLIST = _IOR('R', 210, int),
// RFCOMMGETDEVINFO = _IOR('R', 211, int), RFCOMMSTEALDLC = _IOW('R', 220, int).
pub const RFCOMM_REUSE_DLC: u32 = 0; pub const RFCOMM_RELEASE_ONHUP: u32 = 1; pub const RFCOMM_HANGUP_NOW: u32 = 2; pub const RFCOMM_TTY_ATTACHED: u32 = 3; pub const RFCOMM_DEFUNCT_BIT4: u32 = 4;
pub const RFCOMM_DEV_RELEASED: u32 = 0; pub const RFCOMM_TTY_OWNED: u32 = 1;
#[repr(C)] pub struct rfcomm_dev_req { pub dev_id: s16, pub flags: u32, pub src: bdaddr_t, pub dst: bdaddr_t, pub channel: u8 }
#[repr(C)] pub struct rfcomm_dev_info { pub id: s16, pub flags: u32, pub state: u16, pub src: bdaddr_t, pub dst: bdaddr_t, pub channel: u8 }
#[repr(C)] pub struct rfcomm_dev_list_req { pub dev_num: u16, pub dev_info: [rfcomm_dev_info; 0] }
extern "C" { pub fn rfcomm_dev_ioctl(sk: *mut sock, cmd: c_uint, arg: *mut c_void) -> c_int; }
// Under CONFIG_BT_RFCOMM_TTY, these are external functions; otherwise the inline stubs return 0.
#[cfg(CONFIG_BT_RFCOMM_TTY)] extern "C" { pub fn rfcomm_init_ttys() -> c_int; pub fn rfcomm_cleanup_ttys(); }
#[cfg(not(CONFIG_BT_RFCOMM_TTY))] #[inline] pub fn rfcomm_init_ttys() -> c_int { 0 }
#[cfg(not(CONFIG_BT_RFCOMM_TTY))] #[inline] pub fn rfcomm_cleanup_ttys() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
