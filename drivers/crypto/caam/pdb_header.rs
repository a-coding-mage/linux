/* SPDX-License-Identifier: GPL-2.0 */
/* CAAM Protocol Data Block (PDB) definition header file */

pub const PDBHMO_ESP_DECAP_SHIFT: u32 = 28;
pub const PDBHMO_ESP_ENCAP_SHIFT: u32 = 28;
pub const PDBHMO_ESP_DECAP_DEC_TTL: u32 = 0x02 << PDBHMO_ESP_DECAP_SHIFT;
pub const PDBHMO_ESP_ENCAP_DEC_TTL: u32 = 0x02 << PDBHMO_ESP_ENCAP_SHIFT;
pub const PDBHMO_ESP_DIFFSERV: u32 = 0x01 << PDBHMO_ESP_DECAP_SHIFT;
pub const PDBHMO_ESP_DFBIT: u32 = 0x04 << PDBHMO_ESP_ENCAP_SHIFT;
pub const PDBNH_ESP_ENCAP_SHIFT: u32 = 16;
pub const PDBNH_ESP_ENCAP_MASK: u32 = 0xff << PDBNH_ESP_ENCAP_SHIFT;
pub const PDBHDRLEN_ESP_DECAP_SHIFT: u32 = 16;
pub const PDBHDRLEN_MASK: u32 = 0x0fff << PDBHDRLEN_ESP_DECAP_SHIFT;
pub const PDB_NH_OFFSET_SHIFT: u32 = 8;
pub const PDB_NH_OFFSET_MASK: u32 = 0xff << PDB_NH_OFFSET_SHIFT;

pub const PDBOPTS_ESP_ARSNONE: u8 = 0x00;
pub const PDBOPTS_ESP_ARS32: u8 = 0x40;
pub const PDBOPTS_ESP_ARS128: u8 = 0x80;
pub const PDBOPTS_ESP_ARS64: u8 = 0xc0;
pub const PDBOPTS_ESP_ARS_MASK: u8 = 0xc0;
pub const PDBOPTS_ESP_IVSRC: u8 = 0x20;
pub const PDBOPTS_ESP_ESN: u8 = 0x10;
pub const PDBOPTS_ESP_OUTFMT: u8 = 0x08;
pub const PDBOPTS_ESP_IPHDRSRC: u8 = 0x08;
pub const PDBOPTS_ESP_INCIPHDR: u8 = 0x04;
pub const PDBOPTS_ESP_IPVSN: u8 = 0x02;
pub const PDBOPTS_ESP_AOFL: u8 = 0x04;
pub const PDBOPTS_ESP_TUNNEL: u8 = 0x01;
pub const PDBOPTS_ESP_IPV6: u8 = 0x02;
pub const PDBOPTS_ESP_DIFFSERV: u8 = 0x40;
pub const PDBOPTS_ESP_UPDATE_CSUM: u8 = 0x80;
pub const PDBOPTS_ESP_VERIFY_CSUM: u8 = 0x20;

#[repr(C)] pub struct IpsecEncapCbc { pub iv: [u8; 16] }
#[repr(C)] pub struct IpsecEncapCtr { pub ctr_nonce: [u8; 4], pub ctr_initial: u32, pub iv: u64 }
#[repr(C)] pub struct IpsecEncapCcm { pub salt: [u8; 4], pub ccm_opt: u32, pub iv: u64 }
#[repr(C)] pub struct IpsecEncapGcm { pub salt: [u8; 4], pub rsvd1: u32, pub iv: u64 }

#[repr(C)] pub union IpsecEncapCipher {
    pub cbc: std::mem::ManuallyDrop<IpsecEncapCbc>,
    pub ctr: std::mem::ManuallyDrop<IpsecEncapCtr>,
    pub ccm: std::mem::ManuallyDrop<IpsecEncapCcm>,
    pub gcm: std::mem::ManuallyDrop<IpsecEncapGcm>,
}
#[repr(C)] pub struct IpsecEncapPdb {
    pub options: u32, pub seq_num_ext_hi: u32, pub seq_num: u32,
    pub cipher: IpsecEncapCipher, pub spi: u32, pub ip_hdr_len: u32,
    pub ip_hdr: [u32; 0],
}

#[repr(C)] pub struct IpsecDecapCbc { pub rsvd: [u32; 2] }
#[repr(C)] pub struct IpsecDecapCtr { pub ctr_nonce: [u8; 4], pub ctr_initial: u32 }
#[repr(C)] pub struct IpsecDecapCcm { pub salt: [u8; 4], pub ccm_opt: u32 }
#[repr(C)] pub struct IpsecDecapGcm { pub salt: [u8; 4], pub resvd: u32 }
#[repr(C)] pub union IpsecDecapCipher {
    pub cbc: std::mem::ManuallyDrop<IpsecDecapCbc>, pub ctr: std::mem::ManuallyDrop<IpsecDecapCtr>,
    pub ccm: std::mem::ManuallyDrop<IpsecDecapCcm>, pub gcm: std::mem::ManuallyDrop<IpsecDecapGcm>,
}
#[repr(C)] pub struct IpsecDecapPdb {
    pub options: u32, pub cipher: IpsecDecapCipher, pub seq_num_ext_hi: u32,
    pub seq_num: u32, pub anti_replay: [u32; 4],
}

pub const IPSEC_ENCAP_DECO_DPOVRD_USE: u8 = 0x80;
#[repr(C)] pub struct IpsecDecoDpovrd { pub ovrd_ecn: u8, pub ip_hdr_len: u8, pub nh_offset: u8, pub next_header: u8 }

pub const WIFI_PDBOPTS_FCS: u8 = 0x01;
pub const WIFI_PDBOPTS_AR: u8 = 0x40;
#[repr(C)] pub struct WifiEncapPdb { pub mac_hdr_len: u16, pub rsvd: u8, pub options: u8, pub iv_flags: u8, pub pri: u8, pub pn1: u16, pub pn2: u32, pub frm_ctrl_mask: u16, pub seq_ctrl_mask: u16, pub rsvd1: [u8; 2], pub cnst: u8, pub key_id: u8, pub ctr_flags: u8, pub rsvd2: u8, pub ctr_init: u16 }
#[repr(C)] pub struct WifiDecapPdb { pub mac_hdr_len: u16, pub rsvd: u8, pub options: u8, pub iv_flags: u8, pub pri: u8, pub pn1: u16, pub pn2: u32, pub frm_ctrl_mask: u16, pub seq_ctrl_mask: u16, pub rsvd1: [u8; 4], pub ctr_flags: u8, pub rsvd2: u8, pub ctr_init: u16 }

pub const WIMAX_PDBOPTS_FCS: u8 = 0x01;
pub const WIMAX_PDBOPTS_AR: u8 = 0x40;
#[repr(C)] pub struct WimaxEncapPdb { pub rsvd: [u8; 3], pub options: u8, pub nonce: u32, pub b0_flags: u8, pub ctr_flags: u8, pub ctr_init: u16, pub pn: u32 }
#[repr(C)] pub struct WimaxDecapPdb { pub rsvd: [u8; 3], pub options: u8, pub nonce: u32, pub iv_flags: u8, pub ctr_flags: u8, pub ctr_init: u16, pub pn: u32, pub rsvd1: [u8; 2], pub antireplay_len: u16, pub antireplay_scorecard: u64 }

pub const MACSEC_PDBOPTS_FCS: u8 = 0x01;
pub const MACSEC_PDBOPTS_AR: u8 = 0x40;
#[repr(C)] pub struct MacsecEncapPdb { pub aad_len: u16, pub rsvd: u8, pub options: u8, pub sci: u64, pub ethertype: u16, pub tci_an: u8, pub rsvd1: u8, pub pn: u32 }
#[repr(C)] pub struct MacsecDecapPdb { pub aad_len: u16, pub rsvd: u8, pub options: u8, pub sci: u64, pub rsvd1: [u8; 3], pub antireplay_len: u8, pub pn: u32, pub antireplay_scorecard: u64 }

pub const TLS_PDBOPTS_ARS32: u8 = 0x40;
pub const TLS_PDBOPTS_ARS64: u8 = 0xc0;
pub const TLS_PDBOPTS_OUTFMT: u8 = 0x08;
pub const TLS_PDBOPTS_IV_WRTBK: u8 = 0x02;
pub const TLS_PDBOPTS_EXP_RND_IV: u8 = 0x01;
#[repr(C)] pub struct TlsBlockEncapPdb { pub type_: u8, pub version: [u8; 2], pub options: u8, pub seq_num: u64, pub iv: [u32; 4] }
#[repr(C)] pub struct TlsStreamEncapPdb { pub type_: u8, pub version: [u8; 2], pub options: u8, pub seq_num: u64, pub i: u8, pub j: u8, pub rsvd1: [u8; 2] }
#[repr(C)] pub struct DtlsBlockEncapPdb { pub type_: u8, pub version: [u8; 2], pub options: u8, pub epoch: u16, pub seq_num: [u16; 3], pub iv: [u32; 4] }
#[repr(C)] pub struct TlsBlockDecapPdb { pub rsvd: [u8; 3], pub options: u8, pub seq_num: u64, pub iv: [u32; 4] }
#[repr(C)] pub struct TlsStreamDecapPdb { pub rsvd: [u8; 3], pub options: u8, pub seq_num: u64, pub i: u8, pub j: u8, pub rsvd1: [u8; 2] }
#[repr(C)] pub struct DtlsBlockDecapPdb { pub rsvd: [u8; 3], pub options: u8, pub epoch: u16, pub seq_num: [u16; 3], pub iv: [u32; 4], pub antireplay_scorecard: u64 }

pub const SRTP_PDBOPTS_MKI: u8 = 0x08;
pub const SRTP_PDBOPTS_AR: u8 = 0x40;
#[repr(C)] pub struct SrtpEncapPdb { pub x_len: u8, pub mki_len: u8, pub n_tag: u8, pub options: u8, pub cnst0: u32, pub rsvd: [u8; 2], pub cnst1: u16, pub salt: [u16; 7], pub cnst2: u16, pub rsvd1: u32, pub roc: u32, pub opt_mki: u32 }
#[repr(C)] pub struct SrtpDecapPdb { pub x_len: u8, pub mki_len: u8, pub n_tag: u8, pub options: u8, pub cnst0: u32, pub rsvd: [u8; 2], pub cnst1: u16, pub salt: [u16; 7], pub cnst2: u16, pub rsvd1: u16, pub seq_num: u16, pub roc: u32, pub antireplay_scorecard: u64 }

pub const DSA_PDB_SGF_SHIFT: u32 = 24;
pub const DSA_PDB_SGF_MASK: u32 = 0xff << DSA_PDB_SGF_SHIFT;
pub const DSA_PDB_SGF_Q: u32 = 0x80 << DSA_PDB_SGF_SHIFT;
pub const DSA_PDB_SGF_R: u32 = 0x40 << DSA_PDB_SGF_SHIFT;
pub const DSA_PDB_SGF_G: u32 = 0x20 << DSA_PDB_SGF_SHIFT;
pub const DSA_PDB_SGF_W: u32 = 0x10 << DSA_PDB_SGF_SHIFT;
pub const DSA_PDB_SGF_S: u32 = 0x10 << DSA_PDB_SGF_SHIFT;
pub const DSA_PDB_SGF_F: u32 = 0x08 << DSA_PDB_SGF_SHIFT;
pub const DSA_PDB_SGF_C: u32 = 0x04 << DSA_PDB_SGF_SHIFT;
pub const DSA_PDB_SGF_D: u32 = 0x02 << DSA_PDB_SGF_SHIFT;
pub const DSA_PDB_SGF_AB_SIGN: u32 = 0x02 << DSA_PDB_SGF_SHIFT;
pub const DSA_PDB_SGF_AB_VERIFY: u32 = 0x01 << DSA_PDB_SGF_SHIFT;
pub const DSA_PDB_L_SHIFT: u32 = 7;
pub const DSA_PDB_L_MASK: u32 = 0x3ff << DSA_PDB_L_SHIFT;
pub const DSA_PDB_N_MASK: u8 = 0x7f;

#[repr(C)] pub struct DsaSignPdb { pub sgf_ln: u32, pub q: *mut u8, pub r: *mut u8, pub g: *mut u8, pub s: *mut u8, pub f: *mut u8, pub c: *mut u8, pub d: *mut u8, pub ab: *mut u8, pub u: *mut u8 }
#[repr(C)] pub struct DsaVerifyPdb { pub sgf_ln: u32, pub q: *mut u8, pub r: *mut u8, pub g: *mut u8, pub w: *mut u8, pub f: *mut u8, pub c: *mut u8, pub d: *mut u8, pub tmp: *mut u8, pub ab: *mut u8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
