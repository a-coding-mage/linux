/* SPDX-License-Identifier: GPL-2.0-only */
/* IEEE802.15.4-2003 specification */

// C dependencies: linux/types.h and linux/random.h.

pub const IEEE802154_MTU: u8 = 127;
pub const IEEE802154_ACK_PSDU_LEN: u8 = 5;
pub const IEEE802154_MIN_PSDU_LEN: u8 = 9;
pub const IEEE802154_FCS_LEN: u8 = 2;
pub const IEEE802154_MAX_AUTH_TAG_LEN: u8 = 16;
pub const IEEE802154_FC_LEN: u8 = 2;
pub const IEEE802154_SEQ_LEN: u8 = 1;
pub const IEEE802154_MAX_HEADER_LEN: u8 = 2 + 1 + 20 + 14;
pub const IEEE802154_MIN_HEADER_LEN: u8 = IEEE802154_ACK_PSDU_LEN - IEEE802154_FCS_LEN;
pub const IEEE802154_PAN_ID_BROADCAST: u16 = 0xffff;
pub const IEEE802154_ADDR_SHORT_BROADCAST: u16 = 0xffff;
pub const IEEE802154_ADDR_SHORT_UNSPEC: u16 = 0xfffe;
pub const IEEE802154_EXTENDED_ADDR_LEN: u8 = 8;
pub const IEEE802154_SHORT_ADDR_LEN: u8 = 2;
pub const IEEE802154_PAN_ID_LEN: u8 = 2;
pub const IEEE802154_MAX_SCAN_DURATION: u8 = 14;
pub const IEEE802154_ACTIVE_SCAN_DURATION: u8 = 15;
pub const IEEE802154_SUPERFRAME_PERIOD: u8 = 16;
pub const IEEE802154_SLOT_PERIOD: u8 = 60;
pub const IEEE802154_LIFS_PERIOD: u8 = 40;
pub const IEEE802154_SIFS_PERIOD: u8 = 12;
pub const IEEE802154_MAX_SIFS_FRAME_SIZE: u8 = 18;
pub const IEEE802154_MAX_CHANNEL: u8 = 26;
pub const IEEE802154_MAX_PAGE: u8 = 31;

pub const IEEE802154_FC_TYPE_BEACON: u16 = 0x0;
pub const IEEE802154_FC_TYPE_DATA: u16 = 0x1;
pub const IEEE802154_FC_TYPE_ACK: u16 = 0x2;
pub const IEEE802154_FC_TYPE_MAC_CMD: u16 = 0x3;
pub const IEEE802154_FC_TYPE_SHIFT: u16 = 0;
pub const IEEE802154_FC_TYPE_MASK: u16 = (1 << 3) - 1;
pub const IEEE802154_FC_SECEN_SHIFT: u16 = 3;
pub const IEEE802154_FC_SECEN: u16 = 1 << IEEE802154_FC_SECEN_SHIFT;
pub const IEEE802154_FC_FRPEND_SHIFT: u16 = 4;
pub const IEEE802154_FC_FRPEND: u16 = 1 << IEEE802154_FC_FRPEND_SHIFT;
pub const IEEE802154_FC_ACK_REQ_SHIFT: u16 = 5;
pub const IEEE802154_FC_ACK_REQ: u16 = 1 << IEEE802154_FC_ACK_REQ_SHIFT;
pub const IEEE802154_FC_INTRA_PAN_SHIFT: u16 = 6;
pub const IEEE802154_FC_INTRA_PAN: u16 = 1 << IEEE802154_FC_INTRA_PAN_SHIFT;
pub const IEEE802154_FC_SAMODE_SHIFT: u16 = 14;
pub const IEEE802154_FC_SAMODE_MASK: u16 = 3 << IEEE802154_FC_SAMODE_SHIFT;
pub const IEEE802154_FC_DAMODE_SHIFT: u16 = 10;
pub const IEEE802154_FC_DAMODE_MASK: u16 = 3 << IEEE802154_FC_DAMODE_SHIFT;
pub const IEEE802154_FC_VERSION_SHIFT: u16 = 12;
pub const IEEE802154_FC_VERSION_MASK: u16 = 3 << IEEE802154_FC_VERSION_SHIFT;

pub const IEEE802154_SCF_SECLEVEL_MASK: u8 = 7;
pub const IEEE802154_SCF_SECLEVEL_SHIFT: u8 = 0;
pub const IEEE802154_SCF_KEY_ID_MODE_SHIFT: u8 = 3;
pub const IEEE802154_SCF_KEY_ID_MODE_MASK: u8 = 3 << IEEE802154_SCF_KEY_ID_MODE_SHIFT;
pub const IEEE802154_SCF_KEY_IMPLICIT: u8 = 0;
pub const IEEE802154_SCF_KEY_INDEX: u8 = 1;
pub const IEEE802154_SCF_KEY_SHORT_INDEX: u8 = 2;
pub const IEEE802154_SCF_KEY_HW_INDEX: u8 = 3;
pub const IEEE802154_SCF_SECLEVEL_NONE: u8 = 0;
pub const IEEE802154_SCF_SECLEVEL_MIC32: u8 = 1;
pub const IEEE802154_SCF_SECLEVEL_MIC64: u8 = 2;
pub const IEEE802154_SCF_SECLEVEL_MIC128: u8 = 3;
pub const IEEE802154_SCF_SECLEVEL_ENC: u8 = 4;
pub const IEEE802154_SCF_SECLEVEL_ENC_MIC32: u8 = 5;
pub const IEEE802154_SCF_SECLEVEL_ENC_MIC64: u8 = 6;
pub const IEEE802154_SCF_SECLEVEL_ENC_MIC128: u8 = 7;
pub const IEEE802154_MFR_SIZE: u8 = 2;

pub const IEEE802154_CMD_ASSOCIATION_REQ: u8 = 0x01;
pub const IEEE802154_CMD_ASSOCIATION_RESP: u8 = 0x02;
pub const IEEE802154_CMD_DISASSOCIATION_NOTIFY: u8 = 0x03;
pub const IEEE802154_CMD_DATA_REQ: u8 = 0x04;
pub const IEEE802154_CMD_PANID_CONFLICT_NOTIFY: u8 = 0x05;
pub const IEEE802154_CMD_ORPHAN_NOTIFY: u8 = 0x06;
pub const IEEE802154_CMD_BEACON_REQ: u8 = 0x07;
pub const IEEE802154_CMD_COORD_REALIGN_NOTIFY: u8 = 0x08;
pub const IEEE802154_CMD_GTS_REQ: u8 = 0x09;

#[repr(u8)]
pub enum Ieee802154MacStatus {
    IEEE802154_SUCCESS = 0x0,
    IEEE802154_MAC_ERROR = 0x1,
    IEEE802154_CANCELLED = 0x2,
    IEEE802154_READY_FOR_POLL = 0x3,
    IEEE802154_COUNTER_ERROR = 0xdb,
    IEEE802154_IMPROPER_KEY_TYPE = 0xdc,
    IEEE802154_IMPROPER_SECURITY_LEVEL = 0xdd,
    IEEE802154_UNSUPPORTED_LEGACY = 0xde,
    IEEE802154_UNSUPPORTED_SECURITY = 0xdf,
    IEEE802154_BEACON_LOST = 0xe0,
    IEEE802154_CHANNEL_ACCESS_FAILURE = 0xe1,
    IEEE802154_DENIED = 0xe2,
    IEEE802154_DISABLE_TRX_FAILURE = 0xe3,
    IEEE802154_FAILED_SECURITY_CHECK = 0xe4,
    IEEE802154_FRAME_TOO_LONG = 0xe5,
    IEEE802154_INVALID_GTS = 0xe6,
    IEEE802154_INVALID_HANDLE = 0xe7,
    IEEE802154_INVALID_PARAMETER = 0xe8,
    IEEE802154_NO_ACK = 0xe9,
    IEEE802154_NO_BEACON = 0xea,
    IEEE802154_NO_DATA = 0xeb,
    IEEE802154_NO_SHORT_ADDRESS = 0xec,
    IEEE802154_OUT_OF_CAP = 0xed,
    IEEE802154_PAN_ID_CONFLICT = 0xee,
    IEEE802154_REALIGNMENT = 0xef,
    IEEE802154_TRANSACTION_EXPIRED = 0xf0,
    IEEE802154_TRANSACTION_OVERFLOW = 0xf1,
    IEEE802154_TX_ACTIVE = 0xf2,
    IEEE802154_UNAVAILABLE_KEY = 0xf3,
    IEEE802154_UNSUPPORTED_ATTRIBUTE = 0xf4,
    IEEE802154_INVALID_ADDRESS = 0xf5,
    IEEE802154_ON_TIME_TOO_LONG = 0xf6,
    IEEE802154_PAST_TIME = 0xf7,
    IEEE802154_TRACKING_OFF = 0xf8,
    IEEE802154_INVALID_INDEX = 0xf9,
    IEEE802154_LIMIT_REACHED = 0xfa,
    IEEE802154_READ_ONLY = 0xfb,
    IEEE802154_SCAN_IN_PROGRESS = 0xfc,
    IEEE802154_SUPERFRAME_OVERLAP = 0xfd,
    IEEE802154_SYSTEM_ERROR = 0xff,
}

#[repr(u32)]
pub enum ieee802154_filtering_level {
    IEEE802154_FILTERING_NONE,
    IEEE802154_FILTERING_1_FCS,
    IEEE802154_FILTERING_2_PROMISCUOUS,
    IEEE802154_FILTERING_3_SCAN,
    IEEE802154_FILTERING_4_FRAME_FIELDS,
}

pub const IEEE802154_FCTL_FTYPE: u16 = 0x0003;
pub const IEEE802154_FCTL_ACKREQ: u16 = 0x0020;
pub const IEEE802154_FCTL_SECEN: u16 = 0x0004;
pub const IEEE802154_FCTL_INTRA_PAN: u16 = 0x0040;
pub const IEEE802154_FCTL_DADDR: u16 = 0x0c00;
pub const IEEE802154_FCTL_SADDR: u16 = 0xc000;
pub const IEEE802154_FTYPE_DATA: u16 = 0x0001;
pub const IEEE802154_FCTL_ADDR_NONE: u16 = 0x0000;
pub const IEEE802154_FCTL_DADDR_SHORT: u16 = 0x0800;
pub const IEEE802154_FCTL_DADDR_EXTENDED: u16 = 0x0c00;
pub const IEEE802154_FCTL_SADDR_SHORT: u16 = 0x8000;
pub const IEEE802154_FCTL_SADDR_EXTENDED: u16 = 0xc000;

pub type __le16 = u16;
pub type __le64 = u64;

#[inline]
pub const fn ieee802154_is_data(fc: __le16) -> bool { (fc & IEEE802154_FCTL_FTYPE) == IEEE802154_FTYPE_DATA }
#[inline]
pub const fn ieee802154_is_secen(fc: __le16) -> bool { fc & IEEE802154_FCTL_SECEN != 0 }
#[inline]
pub const fn ieee802154_is_ackreq(fc: __le16) -> bool { fc & IEEE802154_FCTL_ACKREQ != 0 }
#[inline]
pub const fn ieee802154_is_intra_pan(fc: __le16) -> bool { fc & IEEE802154_FCTL_INTRA_PAN != 0 }
#[inline]
pub const fn ieee802154_daddr_mode(fc: __le16) -> __le16 { fc & IEEE802154_FCTL_DADDR }
#[inline]
pub const fn ieee802154_saddr_mode(fc: __le16) -> __le16 { fc & IEEE802154_FCTL_SADDR }
#[inline]
pub const fn ieee802154_is_valid_psdu_len(len: u8) -> bool {
    len == IEEE802154_ACK_PSDU_LEN || (len >= IEEE802154_MIN_PSDU_LEN && len <= IEEE802154_MTU)
}
#[inline]
pub const fn ieee802154_is_valid_extended_unicast_addr(addr: __le64) -> bool {
    addr != 0 && (addr & 0x0100000000000000) == 0
}
#[inline]
pub const fn ieee802154_is_broadcast_short_addr(addr: __le16) -> bool { addr == IEEE802154_ADDR_SHORT_BROADCAST }
#[inline]
pub const fn ieee802154_is_unspec_short_addr(addr: __le16) -> bool { addr == IEEE802154_ADDR_SHORT_UNSPEC }
#[inline]
pub const fn ieee802154_is_valid_src_short_addr(addr: __le16) -> bool {
    !ieee802154_is_broadcast_short_addr(addr) && !ieee802154_is_unspec_short_addr(addr)
}

// External dependency: get_random_bytes is supplied by linux/random.h.
extern "C" { pub fn get_random_bytes(buf: *mut u8, len: usize); }

#[inline]
pub unsafe fn ieee802154_random_extended_addr(addr: *mut __le64) {
    get_random_bytes(addr as *mut u8, IEEE802154_EXTENDED_ADDR_LEN as usize);
    let last = (addr as *mut u8).add(IEEE802154_EXTENDED_ADDR_LEN as usize - 1);
    *last &= !0x01;
    *last |= 0x02;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
