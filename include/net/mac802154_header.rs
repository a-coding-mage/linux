/* SPDX-License-Identifier: GPL-2.0-only */
/* IEEE802.15.4-2003 specification */

/* C header dependencies are supplied by other translated units. */

#[repr(u32)]
pub enum ieee802154_hw_addr_filt_flags {
    IEEE802154_AFILT_SADDR_CHANGED = BIT(0),
    IEEE802154_AFILT_IEEEADDR_CHANGED = BIT(1),
    IEEE802154_AFILT_PANID_CHANGED = BIT(2),
    IEEE802154_AFILT_PANC_CHANGED = BIT(3),
}

#[repr(C)]
pub struct ieee802154_hw_addr_filt {
    pub pan_id: __le16,
    pub short_addr: __le16,
    pub ieee_addr: __le64,
    pub pan_coord: bool,
}

#[repr(C)]
pub struct ieee802154_hw {
    /* filled by the driver */
    pub extra_tx_headroom: i32,
    pub flags: u32,
    pub parent: *mut device,
    pub priv_: *mut core::ffi::c_void,

    /* filled by mac802154 core */
    pub phy: *mut wpan_phy,
}

#[repr(u32)]
pub enum ieee802154_hw_flags {
    IEEE802154_HW_TX_OMIT_CKSUM = BIT(0),
    IEEE802154_HW_LBT = BIT(1),
    IEEE802154_HW_CSMA_PARAMS = BIT(2),
    IEEE802154_HW_FRAME_RETRIES = BIT(3),
    IEEE802154_HW_AFILT = BIT(4),
    IEEE802154_HW_PROMISCUOUS = BIT(5),
    IEEE802154_HW_RX_OMIT_CKSUM = BIT(6),
}

pub const IEEE802154_HW_OMIT_CKSUM: u32 =
    IEEE802154_HW_TX_OMIT_CKSUM as u32 | IEEE802154_HW_RX_OMIT_CKSUM as u32;

#[repr(C)]
pub struct ieee802154_ops {
    pub owner: *mut module,
    pub start: Option<unsafe extern "C" fn(*mut ieee802154_hw) -> i32>,
    pub stop: Option<unsafe extern "C" fn(*mut ieee802154_hw)>,
    pub xmit_sync: Option<unsafe extern "C" fn(*mut ieee802154_hw, *mut sk_buff) -> i32>,
    pub xmit_async: Option<unsafe extern "C" fn(*mut ieee802154_hw, *mut sk_buff) -> i32>,
    pub ed: Option<unsafe extern "C" fn(*mut ieee802154_hw, *mut u8) -> i32>,
    pub set_channel: Option<unsafe extern "C" fn(*mut ieee802154_hw, u8, u8) -> i32>,
    pub set_hw_addr_filt: Option<unsafe extern "C" fn(*mut ieee802154_hw, *mut ieee802154_hw_addr_filt, c_ulong) -> i32>,
    pub set_txpower: Option<unsafe extern "C" fn(*mut ieee802154_hw, i32) -> i32>,
    pub set_lbt: Option<unsafe extern "C" fn(*mut ieee802154_hw, bool) -> i32>,
    pub set_cca_mode: Option<unsafe extern "C" fn(*mut ieee802154_hw, *const wpan_phy_cca) -> i32>,
    pub set_cca_ed_level: Option<unsafe extern "C" fn(*mut ieee802154_hw, i32) -> i32>,
    pub set_csma_params: Option<unsafe extern "C" fn(*mut ieee802154_hw, u8, u8, u8) -> i32>,
    pub set_frame_retries: Option<unsafe extern "C" fn(*mut ieee802154_hw, i8) -> i32>,
    pub set_promiscuous_mode: Option<unsafe extern "C" fn(*mut ieee802154_hw, bool) -> i32>,
}

#[inline]
pub unsafe fn ieee802154_get_fc_from_skb(skb: *const sk_buff) -> __le16 {
    if WARN_ON(!skb_mac_header_was_set(skb) ||
        (skb_tail_pointer(skb).offset_from(skb_mac_header(skb)) as usize) < IEEE802154_FC_LEN) {
        return cpu_to_le16(0);
    }
    let mut fc: __le16 = core::mem::zeroed();
    core::ptr::copy_nonoverlapping(skb_mac_header(skb), &mut fc as *mut _ as *mut u8, IEEE802154_FC_LEN);
    fc
}

#[inline]
pub unsafe fn ieee802154_skb_dst_pan(fc: __le16, skb: *const sk_buff) -> *mut u8 {
    match ieee802154_daddr_mode(fc) {
        x if x == cpu_to_le16(IEEE802154_FCTL_ADDR_NONE) => core::ptr::null_mut(),
        x if x == cpu_to_le16(IEEE802154_FCTL_DADDR_SHORT) || x == cpu_to_le16(IEEE802154_FCTL_DADDR_EXTENDED) =>
            skb_mac_header(skb).add(IEEE802154_FC_LEN + IEEE802154_SEQ_LEN),
        _ => { WARN_ONCE(true, "invalid addr mode detected"); core::ptr::null_mut() }
    }
}

#[inline]
pub unsafe fn ieee802154_skb_src_pan(fc: __le16, skb: *const sk_buff) -> *mut u8 {
    match ieee802154_saddr_mode(fc) {
        x if x == cpu_to_le16(IEEE802154_FCTL_ADDR_NONE) => core::ptr::null_mut(),
        x if x == cpu_to_le16(IEEE802154_FCTL_SADDR_SHORT) || x == cpu_to_le16(IEEE802154_FCTL_SADDR_EXTENDED) => {
            if ieee802154_is_intra_pan(fc) { return ieee802154_skb_dst_pan(fc, skb); }
            match ieee802154_daddr_mode(fc) {
                x if x == cpu_to_le16(IEEE802154_FCTL_ADDR_NONE) => skb_mac_header(skb).add(IEEE802154_FC_LEN + IEEE802154_SEQ_LEN),
                x if x == cpu_to_le16(IEEE802154_FCTL_DADDR_SHORT) => skb_mac_header(skb).add(IEEE802154_FC_LEN + IEEE802154_SEQ_LEN + IEEE802154_PAN_ID_LEN + IEEE802154_SHORT_ADDR_LEN),
                x if x == cpu_to_le16(IEEE802154_FCTL_DADDR_EXTENDED) => skb_mac_header(skb).add(IEEE802154_FC_LEN + IEEE802154_SEQ_LEN + IEEE802154_PAN_ID_LEN + IEEE802154_EXTENDED_ADDR_LEN),
                _ => { WARN_ONCE(true, "invalid addr mode detected"); core::ptr::null_mut() }
            }
        }
        _ => { WARN_ONCE(true, "invalid addr mode detected"); core::ptr::null_mut() }
    }
}

#[inline]
pub unsafe fn ieee802154_skb_is_intra_pan_addressing(fc: __le16, skb: *const sk_buff) -> bool {
    let dst_pan = ieee802154_skb_dst_pan(fc, skb);
    let src_pan = ieee802154_skb_src_pan(fc, skb);
    if dst_pan.is_null() || src_pan.is_null() { return false; }
    libc_memcmp(dst_pan, src_pan, IEEE802154_PAN_ID_LEN) == 0
}

#[inline] pub unsafe fn ieee802154_be64_to_le64(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) { put_unaligned_le64(get_unaligned_be64(src), dst); }
#[inline] pub unsafe fn ieee802154_le64_to_be64(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) { put_unaligned_be64(get_unaligned_le64(src), dst); }
#[inline] pub unsafe fn ieee802154_le16_to_be16(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) { put_unaligned_be16(get_unaligned_le16(src), dst); }
#[inline] pub unsafe fn ieee802154_be16_to_le16(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) { put_unaligned_le16(get_unaligned_be16(src), dst); }

extern "C" {
    pub fn ieee802154_alloc_hw(priv_data_len: usize, ops: *const ieee802154_ops) -> *mut ieee802154_hw;
    pub fn ieee802154_free_hw(hw: *mut ieee802154_hw);
    pub fn ieee802154_register_hw(hw: *mut ieee802154_hw) -> i32;
    pub fn ieee802154_unregister_hw(hw: *mut ieee802154_hw);
    pub fn ieee802154_rx_irqsafe(hw: *mut ieee802154_hw, skb: *mut sk_buff, lqi: u8);
    pub fn ieee802154_xmit_complete(hw: *mut ieee802154_hw, skb: *mut sk_buff, ifs_handling: bool);
    pub fn ieee802154_xmit_error(hw: *mut ieee802154_hw, skb: *mut sk_buff, reason: i32);
    pub fn ieee802154_xmit_hw_error(hw: *mut ieee802154_hw, skb: *mut sk_buff);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
