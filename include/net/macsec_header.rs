/* SPDX-License-Identifier: GPL-2.0+ */
/* Rust translation of net/macsec.h. */

pub const MACSEC_DEFAULT_PN_LEN: u32 = 4;
pub const MACSEC_XPN_PN_LEN: u32 = 8;
pub const MACSEC_NUM_AN: usize = 4;
pub const MACSEC_SCI_LEN: usize = 8;
pub const MACSEC_PORT_ES: u16 = 0x0001u16.to_be();
pub const MACSEC_TCI_VERSION: u8 = 0x80;
pub const MACSEC_TCI_ES: u8 = 0x40;
pub const MACSEC_TCI_SC: u8 = 0x20;
pub const MACSEC_TCI_SCB: u8 = 0x10;
pub const MACSEC_TCI_E: u8 = 0x08;
pub const MACSEC_TCI_C: u8 = 0x04;
pub const MACSEC_AN_MASK: u8 = 0x03;
pub const MACSEC_TCI_CONFID: u8 = MACSEC_TCI_E | MACSEC_TCI_C;
pub const MACSEC_DEFAULT_ICV_LEN: usize = 16;

pub type sci_t = u64;
pub type ssci_t = u32;

pub type salt_t = salt;
#[repr(C, packed)]
pub union salt {
    pub _bindgen_data_: salt__bindgen_ty_1,
    pub bytes: [u8; MACSEC_SALT_LEN],
}
#[repr(C, packed)]
pub struct salt__bindgen_ty_1 {
    pub ssci: ssci_t,
    pub pn: __be64,
}

pub type pn_t = pn;
#[repr(C)]
pub union pn {
    pub _bindgen_data_: pn__bindgen_ty_1,
    pub full64: u64,
}
#[repr(C)]
pub struct pn__bindgen_ty_1 {
    #[cfg(target_endian = "little")]
    pub lower: u32,
    #[cfg(target_endian = "little")]
    pub upper: u32,
    #[cfg(target_endian = "big")]
    pub upper: u32,
    #[cfg(target_endian = "big")]
    pub lower: u32,
}

#[repr(C)]
pub struct macsec_key {
    pub id: [u8; MACSEC_KEYID_LEN],
    pub tfm: *mut crypto_aead,
    pub salt: salt_t,
}

#[repr(C)]
pub struct macsec_rx_sc_stats { pub InOctetsValidated: __u64, pub InOctetsDecrypted: __u64, pub InPktsUnchecked: __u64, pub InPktsDelayed: __u64, pub InPktsOK: __u64, pub InPktsInvalid: __u64, pub InPktsLate: __u64, pub InPktsNotValid: __u64, pub InPktsNotUsingSA: __u64, pub InPktsUnusedSA: __u64 }
#[repr(C)]
pub struct macsec_rx_sa_stats { pub InPktsOK: __u32, pub InPktsInvalid: __u32, pub InPktsNotValid: __u32, pub InPktsNotUsingSA: __u32, pub InPktsUnusedSA: __u32 }
#[repr(C)]
pub struct macsec_tx_sa_stats { pub OutPktsProtected: __u32, pub OutPktsEncrypted: __u32 }
#[repr(C)]
pub struct macsec_tx_sc_stats { pub OutPktsProtected: __u64, pub OutPktsEncrypted: __u64, pub OutOctetsProtected: __u64, pub OutOctetsEncrypted: __u64 }
#[repr(C)]
pub struct macsec_dev_stats { pub OutPktsUntagged: __u64, pub InPktsUntagged: __u64, pub OutPktsTooLong: __u64, pub InPktsNoTag: __u64, pub InPktsBadTag: __u64, pub InPktsUnknownSCI: __u64, pub InPktsNoSCI: __u64, pub InPktsOverrun: __u64 }

#[repr(C)]
pub struct macsec_rx_sa { pub key: macsec_key, pub ssci: ssci_t, pub lock: spinlock_t, pub next_pn_halves: pn_t, pub refcnt: refcount_t, pub active: bool, pub stats: *mut macsec_rx_sa_stats, pub sc: *mut macsec_rx_sc, pub destroy_work: rcu_work }
#[repr(C)]
pub struct pcpu_rx_sc_stats { pub stats: macsec_rx_sc_stats, pub syncp: u64_stats_sync }
#[repr(C)]
pub struct pcpu_tx_sc_stats { pub stats: macsec_tx_sc_stats, pub syncp: u64_stats_sync }
#[repr(C)]
pub struct macsec_rx_sc { pub next: *mut macsec_rx_sc, pub sci: sci_t, pub active: bool, pub sa: [*mut macsec_rx_sa; MACSEC_NUM_AN], pub stats: *mut pcpu_rx_sc_stats, pub refcnt: refcount_t, pub rcu_head: rcu_head }
#[repr(C)]
pub struct macsec_tx_sa { pub key: macsec_key, pub ssci: ssci_t, pub lock: spinlock_t, pub next_pn_halves: pn_t, pub refcnt: refcount_t, pub active: bool, pub stats: *mut macsec_tx_sa_stats, pub destroy_work: rcu_work }
#[repr(C)]
pub struct macsec_tx_sc { pub active: bool, pub encoding_sa: u8, pub encrypt: bool, pub send_sci: bool, pub end_station: bool, pub scb: bool, pub sa: [*mut macsec_tx_sa; MACSEC_NUM_AN], pub stats: *mut pcpu_tx_sc_stats, pub md_dst: *mut metadata_dst }
#[repr(C)]
pub struct macsec_secy { pub netdev: *mut net_device, pub n_rx_sc: c_uint, pub sci: sci_t, pub key_len: u16, pub icv_len: u16, pub validate_frames: macsec_validation_type, pub xpn: bool, pub operational: bool, pub protect_frames: bool, pub replay_protect: bool, pub replay_window: u32, pub tx_sc: macsec_tx_sc, pub rx_sc: *mut macsec_rx_sc }

#[repr(C)]
pub union macsec_context_netdev { pub netdev: *mut net_device, pub phydev: *mut phy_device }
#[repr(C)]
pub union macsec_context_sa { pub rx_sa: *mut macsec_rx_sa, pub tx_sa: *mut macsec_tx_sa }
#[repr(C)]
pub struct macsec_context_sa_data { pub update_pn: bool, pub assoc_num: c_uchar, pub key: [u8; MACSEC_MAX_KEY_LEN], pub _sa: macsec_context_sa }
#[repr(C)]
pub union macsec_context_stats { pub tx_sc_stats: *mut macsec_tx_sc_stats, pub tx_sa_stats: *mut macsec_tx_sa_stats, pub rx_sc_stats: *mut macsec_rx_sc_stats, pub rx_sa_stats: *mut macsec_rx_sa_stats, pub dev_stats: *mut macsec_dev_stats }
#[repr(C)]
pub struct macsec_context { pub _device: macsec_context_netdev, pub offload: macsec_offload, pub secy: *mut macsec_secy, pub rx_sc: *mut macsec_rx_sc, pub sa: macsec_context_sa_data, pub stats: macsec_context_stats }

#[repr(C)]
pub struct macsec_ops {
    pub mdo_dev_open: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>, pub mdo_dev_stop: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>,
    pub mdo_add_secy: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>, pub mdo_upd_secy: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>, pub mdo_del_secy: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>,
    pub mdo_add_rxsc: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>, pub mdo_upd_rxsc: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>, pub mdo_del_rxsc: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>,
    pub mdo_add_rxsa: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>, pub mdo_upd_rxsa: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>, pub mdo_del_rxsa: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>,
    pub mdo_add_txsa: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>, pub mdo_upd_txsa: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>, pub mdo_del_txsa: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>,
    pub mdo_get_dev_stats: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>, pub mdo_get_tx_sc_stats: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>, pub mdo_get_tx_sa_stats: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>, pub mdo_get_rx_sc_stats: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>, pub mdo_get_rx_sa_stats: Option<unsafe extern "C" fn(*mut macsec_context) -> c_int>,
    pub mdo_insert_tx_tag: Option<unsafe extern "C" fn(*mut phy_device, *mut sk_buff) -> c_int>, pub needed_headroom: c_uint, pub needed_tailroom: c_uint, pub rx_uses_md_dst: bool,
}

extern "C" { pub fn macsec_pn_wrapped(secy: *mut macsec_secy, tx_sa: *mut macsec_tx_sa); pub fn macsec_get_real_dev(dev: *const net_device) -> *mut net_device; pub fn macsec_netdev_is_offloaded(dev: *mut net_device) -> bool; }

#[inline]
pub unsafe fn macsec_send_sci(secy: *const macsec_secy) -> bool { let tx_sc = &(*secy).tx_sc; tx_sc.send_sci || ((*secy).n_rx_sc > 1 && !tx_sc.end_station && !tx_sc.scb) }

#[inline]
pub unsafe fn macsec_netdev_priv(dev: *const net_device) -> *mut core::ffi::c_void { netdev_priv(dev) }

#[inline]
pub fn sci_to_cpu(sci: sci_t) -> u64 { u64::from_be(sci) }

// External kernel types and constants supplied by dependent headers.
extern "C" { type crypto_aead; type metadata_dst; type net_device; type phy_device; type sk_buff; type spinlock_t; type refcount_t; type rcu_work; type u64_stats_sync; type rcu_head; type macsec_validation_type; type macsec_offload; }
pub type __be64 = u64; pub type __u64 = u64; pub type __u32 = u32; pub type c_int = i32; pub type c_uint = u32; pub type c_uchar = u8;
extern "C" { fn netdev_priv(dev: *const net_device) -> *mut core::ffi::c_void; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
