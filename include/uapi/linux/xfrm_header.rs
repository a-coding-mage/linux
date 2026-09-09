/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Translated from linux/xfrm.h. */

#[repr(C)]
pub union xfrm_address_t {
    pub a4: __be32,
    pub a6: [__be32; 4],
    pub in6: in6_addr,
}

#[repr(C)]
pub struct xfrm_id { pub daddr: xfrm_address_t, pub spi: __be32, pub proto: __u8 }
#[repr(C)]
pub struct xfrm_sec_ctx { pub ctx_doi: __u8, pub ctx_alg: __u8, pub ctx_len: __u16, pub ctx_sid: __u32, pub ctx_str: [c_char; 0] }

pub const XFRM_SC_DOI_RESERVED: i32 = 0;
pub const XFRM_SC_DOI_LSM: i32 = 1;
pub const XFRM_SC_ALG_RESERVED: i32 = 0;
pub const XFRM_SC_ALG_SELINUX: i32 = 1;

#[repr(C)]
pub struct xfrm_selector { pub daddr: xfrm_address_t, pub saddr: xfrm_address_t, pub dport: __be16, pub dport_mask: __be16, pub sport: __be16, pub sport_mask: __be16, pub family: __u16, pub prefixlen_d: __u8, pub prefixlen_s: __u8, pub proto: __u8, pub ifindex: c_int, pub user: __kernel_uid32_t }
pub const XFRM_INF: __u64 = !0;
#[repr(C)]
pub struct xfrm_lifetime_cfg { pub soft_byte_limit: __u64, pub hard_byte_limit: __u64, pub soft_packet_limit: __u64, pub hard_packet_limit: __u64, pub soft_add_expires_seconds: __u64, pub hard_add_expires_seconds: __u64, pub soft_use_expires_seconds: __u64, pub hard_use_expires_seconds: __u64 }
#[repr(C)]
pub struct xfrm_lifetime_cur { pub bytes: __u64, pub packets: __u64, pub add_time: __u64, pub use_time: __u64 }
#[repr(C)] pub struct xfrm_replay_state { pub oseq: __u32, pub seq: __u32, pub bitmap: __u32 }
pub const XFRMA_REPLAY_ESN_MAX: u32 = 4096;
#[repr(C)] pub struct xfrm_replay_state_esn { pub bmp_len: c_uint, pub oseq: __u32, pub seq: __u32, pub oseq_hi: __u32, pub seq_hi: __u32, pub replay_window: __u32, pub bmp: [__u32; 0] }
#[repr(C)] pub struct xfrm_algo { pub alg_name: [c_char; 64], pub alg_key_len: c_uint, pub alg_key: [c_char; 0] }
#[repr(C)] pub struct xfrm_algo_auth { pub alg_name: [c_char; 64], pub alg_key_len: c_uint, pub alg_trunc_len: c_uint, pub alg_key: [c_char; 0] }
#[repr(C)] pub struct xfrm_algo_aead { pub alg_name: [c_char; 64], pub alg_key_len: c_uint, pub alg_icv_len: c_uint, pub alg_key: [c_char; 0] }
#[repr(C)] pub struct xfrm_stats { pub replay_window: __u32, pub replay: __u32, pub integrity_failed: __u32 }

pub const XFRM_POLICY_TYPE_MAIN: i32 = 0; pub const XFRM_POLICY_TYPE_SUB: i32 = 1; pub const XFRM_POLICY_TYPE_MAX: i32 = 2; pub const XFRM_POLICY_TYPE_ANY: i32 = 255;
pub const XFRM_POLICY_IN: i32 = 0; pub const XFRM_POLICY_OUT: i32 = 1; pub const XFRM_POLICY_FWD: i32 = 2; pub const XFRM_POLICY_MASK: i32 = 3; pub const XFRM_POLICY_MAX: i32 = 3;
pub const XFRM_SA_DIR_IN: i32 = 1; pub const XFRM_SA_DIR_OUT: i32 = 2;
pub const XFRM_SHARE_ANY: i32 = 0; pub const XFRM_SHARE_SESSION: i32 = 1; pub const XFRM_SHARE_USER: i32 = 2; pub const XFRM_SHARE_UNIQUE: i32 = 3;
pub const XFRM_MODE_TRANSPORT: i32 = 0; pub const XFRM_MODE_TUNNEL: i32 = 1; pub const XFRM_MODE_ROUTEOPTIMIZATION: i32 = 2; pub const XFRM_MODE_IN_TRIGGER: i32 = 3; pub const XFRM_MODE_BEET: i32 = 4; pub const XFRM_MODE_IPTFS: i32 = 5; pub const XFRM_MODE_MAX: i32 = 6;

pub const XFRM_MSG_BASE: i32 = 0x10;
pub const XFRM_MSG_NEWSA: i32 = 0x10; pub const XFRM_MSG_DELSA: i32 = 0x11; pub const XFRM_MSG_GETSA: i32 = 0x12; pub const XFRM_MSG_NEWPOLICY: i32 = 0x13; pub const XFRM_MSG_DELPOLICY: i32 = 0x14; pub const XFRM_MSG_GETPOLICY: i32 = 0x15; pub const XFRM_MSG_ALLOCSPI: i32 = 0x16; pub const XFRM_MSG_ACQUIRE: i32 = 0x17; pub const XFRM_MSG_EXPIRE: i32 = 0x18; pub const XFRM_MSG_UPDPOLICY: i32 = 0x19; pub const XFRM_MSG_UPDSA: i32 = 0x1a; pub const XFRM_MSG_POLEXPIRE: i32 = 0x1b; pub const XFRM_MSG_FLUSHSA: i32 = 0x1c; pub const XFRM_MSG_FLUSHPOLICY: i32 = 0x1d; pub const XFRM_MSG_NEWAE: i32 = 0x1e; pub const XFRM_MSG_GETAE: i32 = 0x1f; pub const XFRM_MSG_REPORT: i32 = 0x20; pub const XFRM_MSG_MIGRATE: i32 = 0x21; pub const XFRM_MSG_NEWSADINFO: i32 = 0x22; pub const XFRM_MSG_GETSADINFO: i32 = 0x23; pub const XFRM_MSG_NEWSPDINFO: i32 = 0x24; pub const XFRM_MSG_GETSPDINFO: i32 = 0x25; pub const XFRM_MSG_MAPPING: i32 = 0x26; pub const XFRM_MSG_SETDEFAULT: i32 = 0x27; pub const XFRM_MSG_GETDEFAULT: i32 = 0x28; pub const XFRM_MSG_MIGRATE_STATE: i32 = 0x29; pub const XFRM_MSG_MAX: i32 = 0x29; pub const XFRM_NR_MSGTYPES: i32 = XFRM_MSG_MAX + 1 - XFRM_MSG_BASE;

#[repr(C)] pub struct xfrm_user_sec_ctx { pub len: __u16, pub exttype: __u16, pub ctx_alg: __u8, pub ctx_doi: __u8, pub ctx_len: __u16 }
#[repr(C)] pub struct xfrm_user_tmpl { pub id: xfrm_id, pub family: __u16, pub saddr: xfrm_address_t, pub reqid: __u32, pub mode: __u8, pub share: __u8, pub optional: __u8, pub aalgos: __u32, pub ealgos: __u32, pub calgos: __u32 }
#[repr(C)] pub struct xfrm_encap_tmpl { pub encap_type: __u16, pub encap_sport: __be16, pub encap_dport: __be16, pub encap_oa: xfrm_address_t }
pub const XFRM_AE_UNSPEC: i32 = 0; pub const XFRM_AE_RTHR: i32 = 1; pub const XFRM_AE_RVAL: i32 = 2; pub const XFRM_AE_LVAL: i32 = 4; pub const XFRM_AE_ETHR: i32 = 8; pub const XFRM_AE_CR: i32 = 16; pub const XFRM_AE_CE: i32 = 32; pub const XFRM_AE_CU: i32 = 64; pub const XFRM_AE_MAX: i32 = 64;
#[repr(C)] pub struct xfrm_userpolicy_type { pub r#type: __u8, pub reserved1: __u16, pub reserved2: __u8 }

pub const XFRMA_UNSPEC: i32 = 0; pub const XFRMA_ALG_AUTH: i32 = 1; pub const XFRMA_ALG_CRYPT: i32 = 2; pub const XFRMA_ALG_COMP: i32 = 3; pub const XFRMA_ENCAP: i32 = 4; pub const XFRMA_TMPL: i32 = 5; pub const XFRMA_SA: i32 = 6; pub const XFRMA_POLICY: i32 = 7; pub const XFRMA_SEC_CTX: i32 = 8; pub const XFRMA_LTIME_VAL: i32 = 9; pub const XFRMA_REPLAY_VAL: i32 = 10; pub const XFRMA_REPLAY_THRESH: i32 = 11; pub const XFRMA_ETIMER_THRESH: i32 = 12; pub const XFRMA_SRCADDR: i32 = 13; pub const XFRMA_COADDR: i32 = 14; pub const XFRMA_LASTUSED: i32 = 15; pub const XFRMA_POLICY_TYPE: i32 = 16; pub const XFRMA_MIGRATE: i32 = 17; pub const XFRMA_ALG_AEAD: i32 = 18; pub const XFRMA_KMADDRESS: i32 = 19; pub const XFRMA_ALG_AUTH_TRUNC: i32 = 20; pub const XFRMA_MARK: i32 = 21; pub const XFRMA_TFCPAD: i32 = 22; pub const XFRMA_REPLAY_ESN_VAL: i32 = 23; pub const XFRMA_SA_EXTRA_FLAGS: i32 = 24; pub const XFRMA_PROTO: i32 = 25; pub const XFRMA_ADDRESS_FILTER: i32 = 26; pub const XFRMA_PAD: i32 = 27; pub const XFRMA_OFFLOAD_DEV: i32 = 28; pub const XFRMA_SET_MARK: i32 = 29; pub const XFRMA_SET_MARK_MASK: i32 = 30; pub const XFRMA_IF_ID: i32 = 31; pub const XFRMA_MTIMER_THRESH: i32 = 32; pub const XFRMA_SA_DIR: i32 = 33; pub const XFRMA_NAT_KEEPALIVE_INTERVAL: i32 = 34; pub const XFRMA_SA_PCPU: i32 = 35; pub const XFRMA_IPTFS_DROP_TIME: i32 = 36; pub const XFRMA_IPTFS_REORDER_WINDOW: i32 = 37; pub const XFRMA_IPTFS_DONT_FRAG: i32 = 38; pub const XFRMA_IPTFS_INIT_DELAY: i32 = 39; pub const XFRMA_IPTFS_MAX_QSIZE: i32 = 40; pub const XFRMA_IPTFS_PKT_SIZE: i32 = 41; pub const XFRMA_OUTPUT_MARK: i32 = XFRMA_SET_MARK; pub const XFRMA_MAX: i32 = 41;
pub const XFRMA_SAD_UNSPEC: i32 = 0; pub const XFRMA_SAD_CNT: i32 = 1; pub const XFRMA_SAD_HINFO: i32 = 2; pub const XFRMA_SAD_MAX: i32 = 2;
pub const XFRMA_SPD_UNSPEC: i32 = 0; pub const XFRMA_SPD_INFO: i32 = 1; pub const XFRMA_SPD_HINFO: i32 = 2; pub const XFRMA_SPD_IPV4_HTHRESH: i32 = 3; pub const XFRMA_SPD_IPV6_HTHRESH: i32 = 4; pub const XFRMA_SPD_MAX: i32 = 4;

#[repr(C)] pub struct xfrm_mark { pub v: __u32, pub m: __u32 }
#[repr(C)] pub struct xfrmu_sadhinfo { pub sadhcnt: __u32, pub sadhmcnt: __u32 }
#[repr(C)] pub struct xfrmu_spdinfo { pub incnt: __u32, pub outcnt: __u32, pub fwdcnt: __u32, pub inscnt: __u32, pub outscnt: __u32, pub fwdscnt: __u32 }
#[repr(C)] pub struct xfrmu_spdhinfo { pub spdhcnt: __u32, pub spdhmcnt: __u32 }
#[repr(C)] pub struct xfrmu_spdhthresh { pub lbits: __u8, pub rbits: __u8 }

#[repr(C)] pub struct xfrm_usersa_info { pub sel: xfrm_selector, pub id: xfrm_id, pub saddr: xfrm_address_t, pub lft: xfrm_lifetime_cfg, pub curlft: xfrm_lifetime_cur, pub stats: xfrm_stats, pub seq: __u32, pub reqid: __u32, pub family: __u16, pub mode: __u8, pub replay_window: __u8, pub flags: __u8 }
pub const XFRM_STATE_NOECN: u8 = 1; pub const XFRM_STATE_DECAP_DSCP: u8 = 2; pub const XFRM_STATE_NOPMTUDISC: u8 = 4; pub const XFRM_STATE_WILDRECV: u8 = 8; pub const XFRM_STATE_ICMP: u8 = 16; pub const XFRM_STATE_AF_UNSPEC: u8 = 32; pub const XFRM_STATE_ALIGN4: u8 = 64; pub const XFRM_STATE_ESN: u8 = 128;
pub const XFRM_SA_XFLAG_DONT_ENCAP_DSCP: i32 = 1; pub const XFRM_SA_XFLAG_OSEQ_MAY_WRAP: i32 = 2;
#[repr(C)] pub struct xfrm_usersa_id { pub daddr: xfrm_address_t, pub spi: __be32, pub family: __u16, pub proto: __u8 }
#[repr(C)] pub struct xfrm_aevent_id { pub sa_id: xfrm_usersa_id, pub saddr: xfrm_address_t, pub flags: __u32, pub reqid: __u32 }
#[repr(C)] pub struct xfrm_userspi_info { pub info: xfrm_usersa_info, pub min: __u32, pub max: __u32 }
#[repr(C)] pub struct xfrm_userpolicy_info { pub sel: xfrm_selector, pub lft: xfrm_lifetime_cfg, pub curlft: xfrm_lifetime_cur, pub priority: __u32, pub index: __u32, pub dir: __u8, pub action: __u8, pub flags: __u8, pub share: __u8 }
pub const XFRM_POLICY_ALLOW: u8 = 0; pub const XFRM_POLICY_BLOCK: u8 = 1; pub const XFRM_POLICY_LOCALOK: u8 = 1; pub const XFRM_POLICY_ICMP: u8 = 2; pub const XFRM_POLICY_CPU_ACQUIRE: u8 = 4;
#[repr(C)] pub struct xfrm_userpolicy_id { pub sel: xfrm_selector, pub index: __u32, pub dir: __u8 }
#[repr(C)] pub struct xfrm_user_acquire { pub id: xfrm_id, pub saddr: xfrm_address_t, pub sel: xfrm_selector, pub policy: xfrm_userpolicy_info, pub aalgos: __u32, pub ealgos: __u32, pub calgos: __u32, pub seq: __u32 }
#[repr(C)] pub struct xfrm_user_expire { pub state: xfrm_usersa_info, pub hard: __u8 }
#[repr(C)] pub struct xfrm_user_polexpire { pub pol: xfrm_userpolicy_info, pub hard: __u8 }
#[repr(C)] pub struct xfrm_usersa_flush { pub proto: __u8 }
#[repr(C)] pub struct xfrm_user_report { pub proto: __u8, pub sel: xfrm_selector }
#[repr(C)] pub struct xfrm_user_kmaddress { pub local: xfrm_address_t, pub remote: xfrm_address_t, pub reserved: __u32, pub family: __u16 }
#[repr(C)] pub struct xfrm_user_migrate { pub old_daddr: xfrm_address_t, pub old_saddr: xfrm_address_t, pub new_daddr: xfrm_address_t, pub new_saddr: xfrm_address_t, pub proto: __u8, pub mode: __u8, pub reserved: __u16, pub reqid: __u32, pub old_family: __u16, pub new_family: __u16 }
#[repr(C)] pub struct xfrm_user_migrate_state { pub id: xfrm_usersa_id, pub new_daddr: xfrm_address_t, pub new_saddr: xfrm_address_t, pub old_mark: xfrm_mark, pub new_sel: xfrm_selector, pub new_reqid: __u32, pub flags: __u32, pub new_family: __u16, pub reserved: __u16 }
pub const XFRM_MIGRATE_STATE_CLEAR_OFFLOAD: i32 = 1; pub const XFRM_MIGRATE_STATE_UPDATE_H2H_SEL: i32 = 2; pub const XFRM_MIGRATE_STATE_KNOWN_FLAGS: i32 = 3;
#[repr(C)] pub struct xfrm_user_mapping { pub id: xfrm_usersa_id, pub reqid: __u32, pub old_saddr: xfrm_address_t, pub new_saddr: xfrm_address_t, pub old_sport: __be16, pub new_sport: __be16 }
#[repr(C)] pub struct xfrm_address_filter { pub saddr: xfrm_address_t, pub daddr: xfrm_address_t, pub family: __u16, pub splen: __u8, pub dplen: __u8 }
#[repr(C)] pub struct xfrm_user_offload { pub ifindex: c_int, pub flags: __u8 }
pub const XFRM_OFFLOAD_IPV6: u8 = 1; pub const XFRM_OFFLOAD_INBOUND: u8 = 2; pub const XFRM_OFFLOAD_PACKET: u8 = 4;
#[repr(C)] pub struct xfrm_userpolicy_default { pub r#in: __u8, pub fwd: __u8, pub out: __u8 }
pub const XFRM_USERPOLICY_UNSPEC: u8 = 0; pub const XFRM_USERPOLICY_BLOCK: u8 = 1; pub const XFRM_USERPOLICY_ACCEPT: u8 = 2;
pub const XFRMGRP_ACQUIRE: i32 = 1; pub const XFRMGRP_EXPIRE: i32 = 2; pub const XFRMGRP_SA: i32 = 4; pub const XFRMGRP_POLICY: i32 = 8; pub const XFRMGRP_REPORT: i32 = 0x20;
pub const XFRMNLGRP_NONE: i32 = 0; pub const XFRMNLGRP_ACQUIRE: i32 = 1; pub const XFRMNLGRP_EXPIRE: i32 = 2; pub const XFRMNLGRP_SA: i32 = 3; pub const XFRMNLGRP_POLICY: i32 = 4; pub const XFRMNLGRP_AEVENTS: i32 = 5; pub const XFRMNLGRP_REPORT: i32 = 6; pub const XFRMNLGRP_MIGRATE: i32 = 7; pub const XFRMNLGRP_MAPPING: i32 = 8; pub const XFRMNLGRP_MAX: i32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
