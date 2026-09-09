/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* PF_KEY user interface, this is defined by rfc2367 so
 * do not make arbitrary modifications or else this header
 * file will not be compliant.
 */

pub const PF_KEY_V2: u8 = 2;
pub const PFKEYV2_REVISION: i64 = 199806;

#[repr(C, packed)]
pub struct sadb_msg { pub sadb_msg_version: u8, pub sadb_msg_type: u8, pub sadb_msg_errno: u8, pub sadb_msg_satype: u8, pub sadb_msg_len: u16, pub sadb_msg_reserved: u16, pub sadb_msg_seq: u32, pub sadb_msg_pid: u32 }
#[repr(C, packed)]
pub struct sadb_ext { pub sadb_ext_len: u16, pub sadb_ext_type: u16 }
#[repr(C, packed)]
pub struct sadb_sa { pub sadb_sa_len: u16, pub sadb_sa_exttype: u16, pub sadb_sa_spi: u32, pub sadb_sa_replay: u8, pub sadb_sa_state: u8, pub sadb_sa_auth: u8, pub sadb_sa_encrypt: u8, pub sadb_sa_flags: u32 }
#[repr(C, packed)]
pub struct sadb_lifetime { pub sadb_lifetime_len: u16, pub sadb_lifetime_exttype: u16, pub sadb_lifetime_allocations: u32, pub sadb_lifetime_bytes: u64, pub sadb_lifetime_addtime: u64, pub sadb_lifetime_usetime: u64 }
#[repr(C, packed)]
pub struct sadb_address { pub sadb_address_len: u16, pub sadb_address_exttype: u16, pub sadb_address_proto: u8, pub sadb_address_prefixlen: u8, pub sadb_address_reserved: u16 }
#[repr(C, packed)]
pub struct sadb_key { pub sadb_key_len: u16, pub sadb_key_exttype: u16, pub sadb_key_bits: u16, pub sadb_key_reserved: u16 }
#[repr(C, packed)]
pub struct sadb_ident { pub sadb_ident_len: u16, pub sadb_ident_exttype: u16, pub sadb_ident_type: u16, pub sadb_ident_reserved: u16, pub sadb_ident_id: u64 }
#[repr(C, packed)]
pub struct sadb_sens { pub sadb_sens_len: u16, pub sadb_sens_exttype: u16, pub sadb_sens_dpd: u32, pub sadb_sens_sens_level: u8, pub sadb_sens_sens_len: u8, pub sadb_sens_integ_level: u8, pub sadb_sens_integ_len: u8, pub sadb_sens_reserved: u32 }
#[repr(C, packed)]
pub struct sadb_prop { pub sadb_prop_len: u16, pub sadb_prop_exttype: u16, pub sadb_prop_replay: u8, pub sadb_prop_reserved: [u8; 3] }
#[repr(C, packed)]
pub struct sadb_comb { pub sadb_comb_auth: u8, pub sadb_comb_encrypt: u8, pub sadb_comb_flags: u16, pub sadb_comb_auth_minbits: u16, pub sadb_comb_auth_maxbits: u16, pub sadb_comb_encrypt_minbits: u16, pub sadb_comb_encrypt_maxbits: u16, pub sadb_comb_reserved: u32, pub sadb_comb_soft_allocations: u32, pub sadb_comb_hard_allocations: u32, pub sadb_comb_soft_bytes: u64, pub sadb_comb_hard_bytes: u64, pub sadb_comb_soft_addtime: u64, pub sadb_comb_hard_addtime: u64, pub sadb_comb_soft_usetime: u64, pub sadb_comb_hard_usetime: u64 }
#[repr(C, packed)]
pub struct sadb_supported { pub sadb_supported_len: u16, pub sadb_supported_exttype: u16, pub sadb_supported_reserved: u32 }
#[repr(C, packed)]
pub struct sadb_alg { pub sadb_alg_id: u8, pub sadb_alg_ivlen: u8, pub sadb_alg_minbits: u16, pub sadb_alg_maxbits: u16, pub sadb_alg_reserved: u16 }
#[repr(C, packed)]
pub struct sadb_spirange { pub sadb_spirange_len: u16, pub sadb_spirange_exttype: u16, pub sadb_spirange_min: u32, pub sadb_spirange_max: u32, pub sadb_spirange_reserved: u32 }
#[repr(C, packed)]
pub struct sadb_x_kmprivate { pub sadb_x_kmprivate_len: u16, pub sadb_x_kmprivate_exttype: u16, pub sadb_x_kmprivate_reserved: u32 }
#[repr(C, packed)]
pub struct sadb_x_sa2 { pub sadb_x_sa2_len: u16, pub sadb_x_sa2_exttype: u16, pub sadb_x_sa2_mode: u8, pub sadb_x_sa2_reserved1: u8, pub sadb_x_sa2_reserved2: u16, pub sadb_x_sa2_sequence: u32, pub sadb_x_sa2_reqid: u32 }
#[repr(C, packed)]
pub struct sadb_x_policy { pub sadb_x_policy_len: u16, pub sadb_x_policy_exttype: u16, pub sadb_x_policy_type: u16, pub sadb_x_policy_dir: u8, pub sadb_x_policy_reserved: u8, pub sadb_x_policy_id: u32, pub sadb_x_policy_priority: u32 }
#[repr(C, packed)]
pub struct sadb_x_ipsecrequest { pub sadb_x_ipsecrequest_len: u16, pub sadb_x_ipsecrequest_proto: u16, pub sadb_x_ipsecrequest_mode: u8, pub sadb_x_ipsecrequest_level: u8, pub sadb_x_ipsecrequest_reserved1: u16, pub sadb_x_ipsecrequest_reqid: u32, pub sadb_x_ipsecrequest_reserved2: u32 }
#[repr(C, packed)]
pub struct sadb_x_nat_t_type { pub sadb_x_nat_t_type_len: u16, pub sadb_x_nat_t_type_exttype: u16, pub sadb_x_nat_t_type_type: u8, pub sadb_x_nat_t_type_reserved: [u8; 3] }
#[repr(C, packed)]
pub struct sadb_x_nat_t_port { pub sadb_x_nat_t_port_len: u16, pub sadb_x_nat_t_port_exttype: u16, pub sadb_x_nat_t_port_port: u16, pub sadb_x_nat_t_port_reserved: u16 }
#[repr(C, packed)]
pub struct sadb_x_sec_ctx { pub sadb_x_sec_len: u16, pub sadb_x_sec_exttype: u16, pub sadb_x_ctx_alg: u8, pub sadb_x_ctx_doi: u8, pub sadb_x_ctx_len: u16 }
#[repr(C, packed)]
pub struct sadb_x_kmaddress { pub sadb_x_kmaddress_len: u16, pub sadb_x_kmaddress_exttype: u16, pub sadb_x_kmaddress_reserved: u32 }
#[repr(C, packed)]
pub struct sadb_x_filter { pub sadb_x_filter_len: u16, pub sadb_x_filter_exttype: u16, pub sadb_x_filter_saddr: [u32; 4], pub sadb_x_filter_daddr: [u32; 4], pub sadb_x_filter_family: u16, pub sadb_x_filter_splen: u8, pub sadb_x_filter_dplen: u8 }

pub const SADB_RESERVED: u8=0; pub const SADB_GETSPI: u8=1; pub const SADB_UPDATE: u8=2; pub const SADB_ADD: u8=3; pub const SADB_DELETE: u8=4; pub const SADB_GET: u8=5; pub const SADB_ACQUIRE: u8=6; pub const SADB_REGISTER: u8=7; pub const SADB_EXPIRE: u8=8; pub const SADB_FLUSH: u8=9; pub const SADB_DUMP: u8=10; pub const SADB_X_PROMISC: u8=11; pub const SADB_X_PCHANGE: u8=12; pub const SADB_X_SPDUPDATE: u8=13; pub const SADB_X_SPDADD: u8=14; pub const SADB_X_SPDDELETE: u8=15; pub const SADB_X_SPDGET: u8=16; pub const SADB_X_SPDACQUIRE: u8=17; pub const SADB_X_SPDDUMP: u8=18; pub const SADB_X_SPDFLUSH: u8=19; pub const SADB_X_SPDSETIDX: u8=20; pub const SADB_X_SPDEXPIRE: u8=21; pub const SADB_X_SPDDELETE2: u8=22; pub const SADB_X_NAT_T_NEW_MAPPING: u8=23; pub const SADB_X_MIGRATE: u8=24; pub const SADB_MAX: u8=24;
pub const SADB_SAFLAGS_PFS: u32=1; pub const SADB_SAFLAGS_NOPMTUDISC: u32=0x20000000; pub const SADB_SAFLAGS_DECAP_DSCP: u32=0x40000000; pub const SADB_SAFLAGS_NOECN: u32=0x80000000;
pub const SADB_SASTATE_LARVAL: u8=0; pub const SADB_SASTATE_MATURE: u8=1; pub const SADB_SASTATE_DYING: u8=2; pub const SADB_SASTATE_DEAD: u8=3; pub const SADB_SASTATE_MAX: u8=3;
pub const SADB_SATYPE_UNSPEC: u8=0; pub const SADB_SATYPE_AH: u8=2; pub const SADB_SATYPE_ESP: u8=3; pub const SADB_SATYPE_RSVP: u8=5; pub const SADB_SATYPE_OSPFV2: u8=6; pub const SADB_SATYPE_RIPV2: u8=7; pub const SADB_SATYPE_MIP: u8=8; pub const SADB_X_SATYPE_IPCOMP: u8=9; pub const SADB_SATYPE_MAX: u8=9;
pub const SADB_AALG_NONE: u8=0; pub const SADB_AALG_MD5HMAC: u8=2; pub const SADB_AALG_SHA1HMAC: u8=3; pub const SADB_X_AALG_SHA2_256HMAC: u8=5; pub const SADB_X_AALG_SHA2_384HMAC: u8=6; pub const SADB_X_AALG_SHA2_512HMAC: u8=7; pub const SADB_X_AALG_RIPEMD160HMAC: u8=8; pub const SADB_X_AALG_AES_XCBC_MAC: u8=9; pub const SADB_X_AALG_SM3_256HMAC: u8=10; pub const SADB_X_AALG_NULL: u8=251; pub const SADB_AALG_MAX: u8=251;
pub const SADB_EALG_NONE: u8=0; pub const SADB_EALG_DESCBC: u8=2; pub const SADB_EALG_3DESCBC: u8=3; pub const SADB_X_EALG_CASTCBC: u8=6; pub const SADB_X_EALG_BLOWFISHCBC: u8=7; pub const SADB_EALG_NULL: u8=11; pub const SADB_X_EALG_AESCBC: u8=12; pub const SADB_X_EALG_AESCTR: u8=13; pub const SADB_X_EALG_AES_CCM_ICV8: u8=14; pub const SADB_X_EALG_AES_CCM_ICV12: u8=15; pub const SADB_X_EALG_AES_CCM_ICV16: u8=16; pub const SADB_X_EALG_AES_GCM_ICV8: u8=18; pub const SADB_X_EALG_AES_GCM_ICV12: u8=19; pub const SADB_X_EALG_AES_GCM_ICV16: u8=20; pub const SADB_X_EALG_CAMELLIACBC: u8=22; pub const SADB_X_EALG_NULL_AES_GMAC: u8=23; pub const SADB_X_EALG_SM4CBC: u8=24; pub const SADB_EALG_MAX: u8=253; pub const SADB_X_EALG_SERPENTCBC: u8=252; pub const SADB_X_EALG_TWOFISHCBC: u8=253;
pub const SADB_X_CALG_NONE: u8=0; pub const SADB_X_CALG_OUI: u8=1; pub const SADB_X_CALG_DEFLATE: u8=2; pub const SADB_X_CALG_LZS: u8=3; pub const SADB_X_CALG_LZJH: u8=4; pub const SADB_X_CALG_MAX: u8=4;
pub const SADB_EXT_RESERVED: u8=0; pub const SADB_EXT_SA: u8=1; pub const SADB_EXT_LIFETIME_CURRENT: u8=2; pub const SADB_EXT_LIFETIME_HARD: u8=3; pub const SADB_EXT_LIFETIME_SOFT: u8=4; pub const SADB_EXT_ADDRESS_SRC: u8=5; pub const SADB_EXT_ADDRESS_DST: u8=6; pub const SADB_EXT_ADDRESS_PROXY: u8=7; pub const SADB_EXT_KEY_AUTH: u8=8; pub const SADB_EXT_KEY_ENCRYPT: u8=9; pub const SADB_EXT_IDENTITY_SRC: u8=10; pub const SADB_EXT_IDENTITY_DST: u8=11; pub const SADB_EXT_SENSITIVITY: u8=12; pub const SADB_EXT_PROPOSAL: u8=13; pub const SADB_EXT_SUPPORTED_AUTH: u8=14; pub const SADB_EXT_SUPPORTED_ENCRYPT: u8=15; pub const SADB_EXT_SPIRANGE: u8=16; pub const SADB_X_EXT_KMPRIVATE: u8=17; pub const SADB_X_EXT_POLICY: u8=18; pub const SADB_X_EXT_SA2: u8=19; pub const SADB_X_EXT_NAT_T_TYPE: u8=20; pub const SADB_X_EXT_NAT_T_SPORT: u8=21; pub const SADB_X_EXT_NAT_T_DPORT: u8=22; pub const SADB_X_EXT_NAT_T_OA: u8=23; pub const SADB_X_EXT_SEC_CTX: u8=24; pub const SADB_X_EXT_KMADDRESS: u8=25; pub const SADB_X_EXT_FILTER: u8=26; pub const SADB_EXT_MAX: u8=26;
pub const SADB_IDENTTYPE_RESERVED: u8=0; pub const SADB_IDENTTYPE_PREFIX: u8=1; pub const SADB_IDENTTYPE_FQDN: u8=2; pub const SADB_IDENTTYPE_USERFQDN: u8=3; pub const SADB_IDENTTYPE_MAX: u8=3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
