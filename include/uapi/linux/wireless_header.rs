/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Rust translation of uapi/linux/wireless.h. External Linux UAPI types are
 * intentionally referenced rather than redefined here. */

pub const WIRELESS_EXT: u32 = 22;

pub const SIOCSIWCOMMIT: u32 = 0x8B00; pub const SIOCGIWNAME: u32 = 0x8B01;
pub const SIOCSIWNWID: u32 = 0x8B02; pub const SIOCGIWNWID: u32 = 0x8B03;
pub const SIOCSIWFREQ: u32 = 0x8B04; pub const SIOCGIWFREQ: u32 = 0x8B05;
pub const SIOCSIWMODE: u32 = 0x8B06; pub const SIOCGIWMODE: u32 = 0x8B07;
pub const SIOCSIWSENS: u32 = 0x8B08; pub const SIOCGIWSENS: u32 = 0x8B09;
pub const SIOCSIWRANGE: u32 = 0x8B0A; pub const SIOCGIWRANGE: u32 = 0x8B0B;
pub const SIOCSIWPRIV: u32 = 0x8B0C; pub const SIOCGIWPRIV: u32 = 0x8B0D;
pub const SIOCSIWSTATS: u32 = 0x8B0E; pub const SIOCGIWSTATS: u32 = 0x8B0F;
pub const SIOCSIWSPY: u32 = 0x8B10; pub const SIOCGIWSPY: u32 = 0x8B11;
pub const SIOCSIWTHRSPY: u32 = 0x8B12; pub const SIOCGIWTHRSPY: u32 = 0x8B13;
pub const SIOCSIWAP: u32 = 0x8B14; pub const SIOCGIWAP: u32 = 0x8B15;
pub const SIOCSIWMLME: u32 = 0x8B16; pub const SIOCGIWAPLIST: u32 = 0x8B17;
pub const SIOCSIWSCAN: u32 = 0x8B18; pub const SIOCGIWSCAN: u32 = 0x8B19;
pub const SIOCSIWESSID: u32 = 0x8B1A; pub const SIOCGIWESSID: u32 = 0x8B1B;
pub const SIOCSIWNICKN: u32 = 0x8B1C; pub const SIOCGIWNICKN: u32 = 0x8B1D;
pub const SIOCSIWRATE: u32 = 0x8B20; pub const SIOCGIWRATE: u32 = 0x8B21;
pub const SIOCSIWRTS: u32 = 0x8B22; pub const SIOCGIWRTS: u32 = 0x8B23;
pub const SIOCSIWFRAG: u32 = 0x8B24; pub const SIOCGIWFRAG: u32 = 0x8B25;
pub const SIOCSIWTXPOW: u32 = 0x8B26; pub const SIOCGIWTXPOW: u32 = 0x8B27;
pub const SIOCSIWRETRY: u32 = 0x8B28; pub const SIOCGIWRETRY: u32 = 0x8B29;
pub const SIOCSIWENCODE: u32 = 0x8B2A; pub const SIOCGIWENCODE: u32 = 0x8B2B;
pub const SIOCSIWPOWER: u32 = 0x8B2C; pub const SIOCGIWPOWER: u32 = 0x8B2D;
pub const SIOCSIWGENIE: u32 = 0x8B30; pub const SIOCGIWGENIE: u32 = 0x8B31;
pub const SIOCSIWAUTH: u32 = 0x8B32; pub const SIOCGIWAUTH: u32 = 0x8B33;
pub const SIOCSIWENCODEEXT: u32 = 0x8B34; pub const SIOCGIWENCODEEXT: u32 = 0x8B35;
pub const SIOCSIWPMKSA: u32 = 0x8B36;
pub const SIOCIWFIRSTPRIV: u32 = 0x8BE0; pub const SIOCIWLASTPRIV: u32 = 0x8BFF;
pub const SIOCIWFIRST: u32 = 0x8B00; pub const SIOCIWLAST: u32 = SIOCIWLASTPRIV;
pub const IWEVTXDROP: u32 = 0x8C00; pub const IWEVQUAL: u32 = 0x8C01;
pub const IWEVCUSTOM: u32 = 0x8C02; pub const IWEVREGISTERED: u32 = 0x8C03;
pub const IWEVEXPIRED: u32 = 0x8C04; pub const IWEVGENIE: u32 = 0x8C05;
pub const IWEVMICHAELMICFAILURE: u32 = 0x8C06; pub const IWEVASSOCREQIE: u32 = 0x8C07;
pub const IWEVASSOCRESPIE: u32 = 0x8C08; pub const IWEVPMKIDCAND: u32 = 0x8C09;
pub const IWEVFIRST: u32 = 0x8C00;

pub const IW_MAX_FREQUENCIES: usize = 32; pub const IW_MAX_BITRATES: usize = 32;
pub const IW_MAX_TXPOWER: usize = 8; pub const IW_MAX_SPY: usize = 8;
pub const IW_MAX_AP: usize = 64; pub const IW_ESSID_MAX_SIZE: usize = 32;
pub const IW_MAX_ENCODING_SIZES: usize = 8; pub const IW_ENCODING_TOKEN_MAX: usize = 64;

pub const IW_MODE_AUTO: u32=0; pub const IW_MODE_ADHOC:u32=1; pub const IW_MODE_INFRA:u32=2;
pub const IW_MODE_MASTER:u32=3; pub const IW_MODE_REPEAT:u32=4; pub const IW_MODE_SECOND:u32=5;
pub const IW_MODE_MONITOR:u32=6; pub const IW_MODE_MESH:u32=7;
pub const IW_QUAL_QUAL_UPDATED:u8=1; pub const IW_QUAL_LEVEL_UPDATED:u8=2; pub const IW_QUAL_NOISE_UPDATED:u8=4;
pub const IW_QUAL_ALL_UPDATED:u8=7; pub const IW_QUAL_DBM:u8=8; pub const IW_QUAL_QUAL_INVALID:u8=16;
pub const IW_QUAL_LEVEL_INVALID:u8=32; pub const IW_QUAL_NOISE_INVALID:u8=64; pub const IW_QUAL_RCPI:u8=128; pub const IW_QUAL_ALL_INVALID:u8=112;
pub const IW_FREQ_AUTO:u8=0; pub const IW_FREQ_FIXED:u8=1;
pub const IW_ENCODE_INDEX:u16=0x00ff; pub const IW_ENCODE_FLAGS:u16=0xff00; pub const IW_ENCODE_MODE:u16=0xf000;
pub const IW_ENCODE_DISABLED:u16=0x8000; pub const IW_ENCODE_ENABLED:u16=0; pub const IW_ENCODE_RESTRICTED:u16=0x4000; pub const IW_ENCODE_OPEN:u16=0x2000; pub const IW_ENCODE_NOKEY:u16=0x0800; pub const IW_ENCODE_TEMP:u16=0x0400;
pub const IW_POWER_ON:u16=0; pub const IW_POWER_TYPE:u16=0xf000; pub const IW_POWER_PERIOD:u16=0x1000; pub const IW_POWER_TIMEOUT:u16=0x2000; pub const IW_POWER_MODE:u16=0x0f00; pub const IW_POWER_UNICAST_R:u16=0x0100; pub const IW_POWER_MULTICAST_R:u16=0x0200; pub const IW_POWER_ALL_R:u16=0x0300; pub const IW_POWER_FORCE_S:u16=0x0400; pub const IW_POWER_REPEATER:u16=0x0800; pub const IW_POWER_MODIFIER:u16=0x000f; pub const IW_POWER_MIN:u16=1; pub const IW_POWER_MAX:u16=2; pub const IW_POWER_RELATIVE:u16=4;
pub const IW_TXPOW_TYPE:u16=0xff; pub const IW_TXPOW_DBM:u16=0; pub const IW_TXPOW_MWATT:u16=1; pub const IW_TXPOW_RELATIVE:u16=2; pub const IW_TXPOW_RANGE:u16=0x1000;
pub const IW_RETRY_ON:u16=0; pub const IW_RETRY_TYPE:u16=0xf000; pub const IW_RETRY_LIMIT:u16=0x1000; pub const IW_RETRY_LIFETIME:u16=0x2000; pub const IW_RETRY_MODIFIER:u16=0xff; pub const IW_RETRY_MIN:u16=1; pub const IW_RETRY_MAX:u16=2; pub const IW_RETRY_RELATIVE:u16=4; pub const IW_RETRY_SHORT:u16=0x10; pub const IW_RETRY_LONG:u16=0x20;
pub const IW_SCAN_DEFAULT:u16=0; pub const IW_SCAN_ALL_ESSID:u16=1; pub const IW_SCAN_THIS_ESSID:u16=2; pub const IW_SCAN_ALL_FREQ:u16=4; pub const IW_SCAN_THIS_FREQ:u16=8; pub const IW_SCAN_ALL_MODE:u16=0x10; pub const IW_SCAN_THIS_MODE:u16=0x20; pub const IW_SCAN_ALL_RATE:u16=0x40; pub const IW_SCAN_THIS_RATE:u16=0x80; pub const IW_SCAN_TYPE_ACTIVE:u8=0; pub const IW_SCAN_TYPE_PASSIVE:u8=1; pub const IW_SCAN_MAX_DATA:usize=4096;
pub const IW_SCAN_CAPA_NONE:u8=0; pub const IW_SCAN_CAPA_ESSID:u8=1; pub const IW_SCAN_CAPA_BSSID:u8=2; pub const IW_SCAN_CAPA_CHANNEL:u8=4; pub const IW_SCAN_CAPA_MODE:u8=8; pub const IW_SCAN_CAPA_RATE:u8=0x10; pub const IW_SCAN_CAPA_TYPE:u8=0x20; pub const IW_SCAN_CAPA_TIME:u8=0x40; pub const IW_CUSTOM_MAX:usize=256; pub const IW_GENERIC_IE_MAX:usize=1024;
pub const IW_MLME_DEAUTH:u16=0; pub const IW_MLME_DISASSOC:u16=1; pub const IW_MLME_AUTH:u16=2; pub const IW_MLME_ASSOC:u16=3;
pub const IW_AUTH_INDEX:u16=0x0fff; pub const IW_AUTH_FLAGS:u16=0xf000;
pub const IW_ENCODE_SEQ_MAX_SIZE:usize=8; pub const IW_ENCODE_ALG_NONE:u16=0; pub const IW_ENCODE_ALG_WEP:u16=1; pub const IW_ENCODE_ALG_TKIP:u16=2; pub const IW_ENCODE_ALG_CCMP:u16=3; pub const IW_ENCODE_ALG_PMK:u16=4; pub const IW_ENCODE_ALG_AES_CMAC:u16=5;
pub const IW_ENCODE_EXT_TX_SEQ_VALID:u32=1; pub const IW_ENCODE_EXT_RX_SEQ_VALID:u32=2; pub const IW_ENCODE_EXT_GROUP_KEY:u32=4; pub const IW_ENCODE_EXT_SET_TX_KEY:u32=8;
pub const IW_PMKSA_ADD:u32=1; pub const IW_PMKSA_REMOVE:u32=2; pub const IW_PMKSA_FLUSH:u32=3; pub const IW_PMKID_LEN:usize=16; pub const IW_PMKID_CAND_PREAUTH:u32=1;

#[inline] pub const fn IW_IOCTL_IDX(cmd: u32) -> usize { (cmd - SIOCIWFIRST) as usize }
#[inline] pub const fn IW_IS_SET(cmd: u32) -> bool { (cmd & 1) == 0 }
#[inline] pub const fn IW_IS_GET(cmd: u32) -> bool { (cmd & 1) != 0 }
#[inline] pub const fn IW_EVENT_IDX(cmd: u32) -> usize { (cmd - IWEVFIRST) as usize }
#[inline] pub const fn IW_EVENT_CAPA_BASE(cmd:u32)->u32 { if cmd >= SIOCIWFIRSTPRIV { cmd-SIOCIWFIRSTPRIV+0x60 } else { cmd-SIOCIWFIRST } }
#[inline] pub const fn IW_EVENT_CAPA_INDEX(cmd:u32)->usize { (IW_EVENT_CAPA_BASE(cmd)>>5) as usize }
#[inline] pub const fn IW_EVENT_CAPA_MASK(cmd:u32)->u32 { 1u32 << (IW_EVENT_CAPA_BASE(cmd)&0x1f) }
pub const IW_EVENT_CAPA_K_0:u32 = IW_EVENT_CAPA_MASK(0x8B04)|IW_EVENT_CAPA_MASK(0x8B06)|IW_EVENT_CAPA_MASK(0x8B1A);
pub const IW_EVENT_CAPA_K_1:u32 = IW_EVENT_CAPA_MASK(0x8B2A);

#[repr(C)] pub struct iw_param { pub value: __s32, pub fixed: __u8, pub disabled: __u8, pub flags: __u16 }
#[repr(C)] pub struct iw_point { pub pointer: *mut core::ffi::c_void, pub length: __u16, pub flags: __u16 }
#[repr(C)] pub struct iw_freq { pub m: __s32, pub e: __s16, pub i: __u8, pub flags: __u8 }
#[repr(C)] pub struct iw_quality { pub qual: __u8, pub level: __u8, pub noise: __u8, pub updated: __u8 }
#[repr(C)] pub struct iw_discarded { pub nwid: __u32, pub code: __u32, pub fragment: __u32, pub retries: __u32, pub misc: __u32 }
#[repr(C)] pub struct iw_missed { pub beacon: __u32 }
#[repr(C)] pub struct iw_thrspy { pub addr: sockaddr, pub qual: iw_quality, pub low: iw_quality, pub high: iw_quality }
#[repr(C)] pub struct iw_scan_req { pub scan_type: __u8, pub essid_len: __u8, pub num_channels: __u8, pub flags: __u8, pub bssid: sockaddr, pub essid: [__u8; IW_ESSID_MAX_SIZE], pub min_channel_time: __u32, pub max_channel_time: __u32, pub channel_list: [iw_freq; IW_MAX_FREQUENCIES] }
#[repr(C)] pub struct iw_encode_ext { pub ext_flags: __u32, pub tx_seq: [__u8;8], pub rx_seq: [__u8;8], pub addr: sockaddr, pub alg: __u16, pub key_len: __u16, pub key: [__u8;0] }
#[repr(C)] pub struct iw_mlme { pub cmd: __u16, pub reason_code: __u16, pub addr: sockaddr }
#[repr(C)] pub struct iw_pmksa { pub cmd: __u32, pub bssid: sockaddr, pub pmkid: [__u8; IW_PMKID_LEN] }
#[repr(C)] pub struct iw_michaelmicfailure { pub flags: __u32, pub src_addr: sockaddr, pub tsc: [__u8;8] }
#[repr(C)] pub struct iw_pmkid_cand { pub flags: __u32, pub index: __u32, pub bssid: sockaddr }
#[repr(C)] pub struct iw_statistics { pub status: __u16, pub qual: iw_quality, pub discard: iw_discarded, pub miss: iw_missed }

#[repr(C)] pub union iwreq_data { pub name: [core::ffi::c_char; IFNAMSIZ], pub essid: iw_point, pub nwid: iw_param, pub freq: iw_freq, pub sens: iw_param, pub bitrate: iw_param, pub txpower: iw_param, pub rts: iw_param, pub frag: iw_param, pub mode: __u32, pub retry: iw_param, pub encoding: iw_point, pub power: iw_param, pub qual: iw_quality, pub ap_addr: sockaddr, pub addr: sockaddr, pub param: iw_param, pub data: iw_point }
#[repr(C)] pub union iwreq_ifrn { pub ifrn_name: [core::ffi::c_char; IFNAMSIZ] }
#[repr(C)] pub struct iwreq { pub ifr_ifrn: iwreq_ifrn, pub u: iwreq_data }
#[repr(C)] pub struct iw_range { pub throughput: __u32, pub min_nwid: __u32, pub max_nwid: __u32, pub old_num_channels: __u16, pub old_num_frequency: __u8, pub scan_capa: __u8, pub event_capa: [__u32;6], pub sensitivity: __s32, pub max_qual: iw_quality, pub avg_qual: iw_quality, pub num_bitrates: __u8, pub bitrate: [__s32;IW_MAX_BITRATES], pub min_rts: __s32, pub max_rts: __s32, pub min_frag: __s32, pub max_frag: __s32, pub min_pmp: __s32, pub max_pmp: __s32, pub min_pmt: __s32, pub max_pmt: __s32, pub pmp_flags: __u16, pub pmt_flags: __u16, pub pm_capa: __u16, pub encoding_size: [__u16;IW_MAX_ENCODING_SIZES], pub num_encoding_sizes: __u8, pub max_encoding_tokens: __u8, pub encoding_login_index: __u8, pub txpower_capa: __u16, pub num_txpower: __u8, pub txpower: [__s32;IW_MAX_TXPOWER], pub we_version_compiled: __u8, pub we_version_source: __u8, pub retry_capa: __u16, pub retry_flags: __u16, pub r_time_flags: __u16, pub min_retry: __s32, pub max_retry: __s32, pub min_r_time: __s32, pub max_r_time: __s32, pub num_channels: __u16, pub num_frequency: __u8, pub freq: [iw_freq;IW_MAX_FREQUENCIES], pub enc_capa: __u32 }
#[repr(C)] pub struct iw_priv_args { pub cmd: __u32, pub set_args: __u16, pub get_args: __u16, pub name: [core::ffi::c_char;IFNAMSIZ] }
#[repr(C)] pub struct iw_event { pub len: __u16, pub cmd: __u16, pub u: iwreq_data }

pub const IW_EV_LCP_PK_LEN:usize=4;
pub const IW_PRIV_TYPE_MASK:u16=0x7000; pub const IW_PRIV_TYPE_NONE:u16=0; pub const IW_PRIV_TYPE_BYTE:u16=0x1000; pub const IW_PRIV_TYPE_CHAR:u16=0x2000; pub const IW_PRIV_TYPE_INT:u16=0x4000; pub const IW_PRIV_TYPE_FLOAT:u16=0x5000; pub const IW_PRIV_TYPE_ADDR:u16=0x6000; pub const IW_PRIV_SIZE_FIXED:u16=0x0800; pub const IW_PRIV_SIZE_MASK:u16=0x07ff;
pub const IW_AUTH_WPA_VERSION:u32=0; pub const IW_AUTH_CIPHER_PAIRWISE:u32=1; pub const IW_AUTH_CIPHER_GROUP:u32=2; pub const IW_AUTH_KEY_MGMT:u32=3; pub const IW_AUTH_TKIP_COUNTERMEASURES:u32=4; pub const IW_AUTH_DROP_UNENCRYPTED:u32=5; pub const IW_AUTH_80211_AUTH_ALG:u32=6; pub const IW_AUTH_WPA_ENABLED:u32=7; pub const IW_AUTH_RX_UNENCRYPTED_EAPOL:u32=8; pub const IW_AUTH_ROAMING_CONTROL:u32=9; pub const IW_AUTH_PRIVACY_INVOKED:u32=10; pub const IW_AUTH_CIPHER_GROUP_MGMT:u32=11; pub const IW_AUTH_MFP:u32=12;
pub const IW_AUTH_WPA_VERSION_DISABLED:u32=1; pub const IW_AUTH_WPA_VERSION_WPA:u32=2; pub const IW_AUTH_WPA_VERSION_WPA2:u32=4; pub const IW_AUTH_CIPHER_NONE:u32=1; pub const IW_AUTH_CIPHER_WEP40:u32=2; pub const IW_AUTH_CIPHER_TKIP:u32=4; pub const IW_AUTH_CIPHER_CCMP:u32=8; pub const IW_AUTH_CIPHER_WEP104:u32=0x10; pub const IW_AUTH_CIPHER_AES_CMAC:u32=0x20; pub const IW_AUTH_KEY_MGMT_802_1X:u32=1; pub const IW_AUTH_KEY_MGMT_PSK:u32=2; pub const IW_AUTH_ALG_OPEN_SYSTEM:u32=1; pub const IW_AUTH_ALG_SHARED_KEY:u32=2; pub const IW_AUTH_ALG_LEAP:u32=4; pub const IW_AUTH_ROAMING_ENABLE:u32=0; pub const IW_AUTH_ROAMING_DISABLE:u32=1; pub const IW_AUTH_MFP_DISABLED:u32=0; pub const IW_AUTH_MFP_OPTIONAL:u32=1; pub const IW_AUTH_MFP_REQUIRED:u32=2;
pub const IW_MICFAILURE_KEY_ID:u32=3; pub const IW_MICFAILURE_GROUP:u32=4; pub const IW_MICFAILURE_PAIRWISE:u32=8; pub const IW_MICFAILURE_STAKEY:u32=0x10; pub const IW_MICFAILURE_COUNT:u32=0x60; pub const IW_ENC_CAPA_WPA:u32=1; pub const IW_ENC_CAPA_WPA2:u32=2; pub const IW_ENC_CAPA_CIPHER_TKIP:u32=4; pub const IW_ENC_CAPA_CIPHER_CCMP:u32=8; pub const IW_ENC_CAPA_4WAY_HANDSHAKE:u32=0x10;
pub const IW_EV_LCP_LEN:usize=0; pub const IW_EV_CHAR_LEN:usize=IW_EV_LCP_LEN+IFNAMSIZ; pub const IW_EV_UINT_LEN:usize=IW_EV_LCP_LEN+4; pub const IW_EV_FREQ_LEN:usize=IW_EV_LCP_LEN+core::mem::size_of::<iw_freq>(); pub const IW_EV_PARAM_LEN:usize=IW_EV_LCP_LEN+core::mem::size_of::<iw_param>(); pub const IW_EV_ADDR_LEN:usize=IW_EV_LCP_LEN+core::mem::size_of::<sockaddr>(); pub const IW_EV_QUAL_LEN:usize=IW_EV_LCP_LEN+core::mem::size_of::<iw_quality>();
pub const IW_EV_CHAR_PK_LEN:usize=IW_EV_LCP_PK_LEN+IFNAMSIZ; pub const IW_EV_UINT_PK_LEN:usize=IW_EV_LCP_PK_LEN+4; pub const IW_EV_FREQ_PK_LEN:usize=IW_EV_LCP_PK_LEN+core::mem::size_of::<iw_freq>(); pub const IW_EV_PARAM_PK_LEN:usize=IW_EV_LCP_PK_LEN+core::mem::size_of::<iw_param>(); pub const IW_EV_ADDR_PK_LEN:usize=IW_EV_LCP_PK_LEN+core::mem::size_of::<sockaddr>(); pub const IW_EV_QUAL_PK_LEN:usize=IW_EV_LCP_PK_LEN+core::mem::size_of::<iw_quality>(); pub const IW_EV_POINT_PK_LEN:usize=IW_EV_LCP_PK_LEN+4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
