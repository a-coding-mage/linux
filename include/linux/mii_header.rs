/* SPDX-License-Identifier: GPL-2.0 */
/* linux/mii.h: definitions for MII-compatible transceivers */

// Dependencies supplied by the surrounding translation unit:
// if.h, linkmode.h, and uapi/linux/mii.h.

#[repr(C)]
pub struct mii_if_info {
    pub phy_id: ::core::ffi::c_int,
    pub advertising: ::core::ffi::c_int,
    pub phy_id_mask: ::core::ffi::c_int,
    pub reg_num_mask: ::core::ffi::c_int,
    // C bit-fields: is full duplex, is autoneg disabled, GMII registers supported.
    pub full_duplex: ::core::ffi::c_uint,
    pub force_media: ::core::ffi::c_uint,
    pub supports_gmii: ::core::ffi::c_uint,
    pub dev: *mut net_device,
    pub mdio_read: Option<unsafe extern "C" fn(*mut net_device, ::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub mdio_write: Option<unsafe extern "C" fn(*mut net_device, ::core::ffi::c_int, ::core::ffi::c_int, ::core::ffi::c_int)>,
}

pub unsafe extern "C" fn mii_link_ok(mii: *mut mii_if_info) -> ::core::ffi::c_int;
pub unsafe extern "C" fn mii_nway_restart(mii: *mut mii_if_info) -> ::core::ffi::c_int;
pub unsafe extern "C" fn mii_ethtool_gset(mii: *mut mii_if_info, ecmd: *mut ethtool_cmd);
pub unsafe extern "C" fn mii_ethtool_get_link_ksettings(mii: *mut mii_if_info, cmd: *mut ethtool_link_ksettings);
pub unsafe extern "C" fn mii_ethtool_sset(mii: *mut mii_if_info, ecmd: *mut ethtool_cmd) -> ::core::ffi::c_int;
pub unsafe extern "C" fn mii_ethtool_set_link_ksettings(mii: *mut mii_if_info, cmd: *const ethtool_link_ksettings) -> ::core::ffi::c_int;
pub unsafe extern "C" fn mii_check_gmii_support(mii: *mut mii_if_info) -> ::core::ffi::c_int;
pub unsafe extern "C" fn mii_check_link(mii: *mut mii_if_info);
pub unsafe extern "C" fn mii_check_media(mii: *mut mii_if_info, ok_to_print: ::core::ffi::c_uint, init_media: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
pub unsafe extern "C" fn generic_mii_ioctl(mii_if: *mut mii_if_info, mii_data: *mut mii_ioctl_data, cmd: ::core::ffi::c_int, duplex_changed: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;

#[inline]
pub unsafe fn if_mii(rq: *mut ifreq) -> *mut mii_ioctl_data {
    &mut (*rq).ifr_ifru as *mut _ as *mut mii_ioctl_data
}

#[inline]
pub fn mii_nway_result(negotiated: u32) -> u32 {
    if negotiated & LPA_100FULL != 0 { LPA_100FULL }
    else if negotiated & LPA_100BASE4 != 0 { LPA_100BASE4 }
    else if negotiated & LPA_100HALF != 0 { LPA_100HALF }
    else if negotiated & LPA_10FULL != 0 { LPA_10FULL }
    else { LPA_10HALF }
}

#[inline]
pub fn mii_duplex(duplex_lock: u32, negotiated: u32) -> u32 {
    if duplex_lock != 0 || mii_nway_result(negotiated) & LPA_DUPLEX != 0 { 1 } else { 0 }
}

#[inline]
pub fn ethtool_adv_to_mii_adv_t(mut ethadv: u32) -> u32 {
    let mut result = 0;
    if ethadv & ADVERTISED_10baseT_Half != 0 { result |= ADVERTISE_10HALF; }
    if ethadv & ADVERTISED_10baseT_Full != 0 { result |= ADVERTISE_10FULL; }
    if ethadv & ADVERTISED_100baseT_Half != 0 { result |= ADVERTISE_100HALF; }
    if ethadv & ADVERTISED_100baseT_Full != 0 { result |= ADVERTISE_100FULL; }
    if ethadv & ADVERTISED_Pause != 0 { result |= ADVERTISE_PAUSE_CAP; }
    if ethadv & ADVERTISED_Asym_Pause != 0 { result |= ADVERTISE_PAUSE_ASYM; }
    result
}

#[inline]
pub unsafe fn linkmode_adv_to_mii_adv_t(advertising: *const ::core::ffi::c_ulong) -> u32 {
    let mut result = 0;
    if linkmode_test_bit(ETHTOOL_LINK_MODE_10baseT_Half_BIT, advertising) != 0 { result |= ADVERTISE_10HALF; }
    if linkmode_test_bit(ETHTOOL_LINK_MODE_10baseT_Full_BIT, advertising) != 0 { result |= ADVERTISE_10FULL; }
    if linkmode_test_bit(ETHTOOL_LINK_MODE_100baseT_Half_BIT, advertising) != 0 { result |= ADVERTISE_100HALF; }
    if linkmode_test_bit(ETHTOOL_LINK_MODE_100baseT_Full_BIT, advertising) != 0 { result |= ADVERTISE_100FULL; }
    if linkmode_test_bit(ETHTOOL_LINK_MODE_Pause_BIT, advertising) != 0 { result |= ADVERTISE_PAUSE_CAP; }
    if linkmode_test_bit(ETHTOOL_LINK_MODE_Asym_Pause_BIT, advertising) != 0 { result |= ADVERTISE_PAUSE_ASYM; }
    result
}

#[inline]
pub fn mii_adv_to_ethtool_adv_t(adv: u32) -> u32 {
    let mut result = 0;
    if adv & ADVERTISE_10HALF != 0 { result |= ADVERTISED_10baseT_Half; }
    if adv & ADVERTISE_10FULL != 0 { result |= ADVERTISED_10baseT_Full; }
    if adv & ADVERTISE_100HALF != 0 { result |= ADVERTISED_100baseT_Half; }
    if adv & ADVERTISE_100FULL != 0 { result |= ADVERTISED_100baseT_Full; }
    if adv & ADVERTISE_PAUSE_CAP != 0 { result |= ADVERTISED_Pause; }
    if adv & ADVERTISE_PAUSE_ASYM != 0 { result |= ADVERTISED_Asym_Pause; }
    result
}

#[inline]
pub fn ethtool_adv_to_mii_ctrl1000_t(ethadv: u32) -> u32 {
    let mut result = 0;
    if ethadv & ADVERTISED_1000baseT_Half != 0 { result |= ADVERTISE_1000HALF; }
    if ethadv & ADVERTISED_1000baseT_Full != 0 { result |= ADVERTISE_1000FULL; }
    result
}

#[inline]
pub unsafe fn linkmode_adv_to_mii_ctrl1000_t(advertising: *const ::core::ffi::c_ulong) -> u32 {
    let mut result = 0;
    if linkmode_test_bit(ETHTOOL_LINK_MODE_1000baseT_Half_BIT, advertising) != 0 { result |= ADVERTISE_1000HALF; }
    if linkmode_test_bit(ETHTOOL_LINK_MODE_1000baseT_Full_BIT, advertising) != 0 { result |= ADVERTISE_1000FULL; }
    result
}

#[inline]
pub fn mii_ctrl1000_to_ethtool_adv_t(adv: u32) -> u32 {
    let mut result = 0;
    if adv & ADVERTISE_1000HALF != 0 { result |= ADVERTISED_1000baseT_Half; }
    if adv & ADVERTISE_1000FULL != 0 { result |= ADVERTISED_1000baseT_Full; }
    result
}

#[inline]
pub fn mii_lpa_to_ethtool_lpa_t(lpa: u32) -> u32 {
    let result = if lpa & LPA_LPACK != 0 { ADVERTISED_Autoneg } else { 0 };
    result | mii_adv_to_ethtool_adv_t(lpa)
}

#[inline]
pub fn mii_stat1000_to_ethtool_lpa_t(lpa: u32) -> u32 {
    let mut result = 0;
    if lpa & LPA_1000HALF != 0 { result |= ADVERTISED_1000baseT_Half; }
    if lpa & LPA_1000FULL != 0 { result |= ADVERTISED_1000baseT_Full; }
    result
}

#[inline]
pub unsafe fn mii_stat1000_mod_linkmode_lpa_t(advertising: *mut ::core::ffi::c_ulong, lpa: u32) {
    linkmode_mod_bit(ETHTOOL_LINK_MODE_1000baseT_Half_BIT, advertising, lpa & LPA_1000HALF);
    linkmode_mod_bit(ETHTOOL_LINK_MODE_1000baseT_Full_BIT, advertising, lpa & LPA_1000FULL);
}

#[inline]
pub fn ethtool_adv_to_mii_adv_x(ethadv: u32) -> u32 {
    let mut result = 0;
    if ethadv & ADVERTISED_1000baseT_Half != 0 { result |= ADVERTISE_1000XHALF; }
    if ethadv & ADVERTISED_1000baseT_Full != 0 { result |= ADVERTISE_1000XFULL; }
    if ethadv & ADVERTISED_Pause != 0 { result |= ADVERTISE_1000XPAUSE; }
    if ethadv & ADVERTISED_Asym_Pause != 0 { result |= ADVERTISE_1000XPSE_ASYM; }
    result
}

#[inline]
pub fn mii_adv_to_ethtool_adv_x(adv: u32) -> u32 {
    let mut result = 0;
    if adv & ADVERTISE_1000XHALF != 0 { result |= ADVERTISED_1000baseT_Half; }
    if adv & ADVERTISE_1000XFULL != 0 { result |= ADVERTISED_1000baseT_Full; }
    if adv & ADVERTISE_1000XPAUSE != 0 { result |= ADVERTISED_Pause; }
    if adv & ADVERTISE_1000XPSE_ASYM != 0 { result |= ADVERTISED_Asym_Pause; }
    result
}
#[inline]
pub unsafe fn mii_adv_mod_linkmode_adv_t(advertising: *mut ::core::ffi::c_ulong, adv: u32) {
    linkmode_mod_bit(ETHTOOL_LINK_MODE_10baseT_Half_BIT, advertising, adv & ADVERTISE_10HALF);
    linkmode_mod_bit(ETHTOOL_LINK_MODE_10baseT_Full_BIT, advertising, adv & ADVERTISE_10FULL);
    linkmode_mod_bit(ETHTOOL_LINK_MODE_100baseT_Half_BIT, advertising, adv & ADVERTISE_100HALF);
    linkmode_mod_bit(ETHTOOL_LINK_MODE_100baseT_Full_BIT, advertising, adv & ADVERTISE_100FULL);
    linkmode_mod_bit(ETHTOOL_LINK_MODE_Pause_BIT, advertising, adv & ADVERTISE_PAUSE_CAP);
    linkmode_mod_bit(ETHTOOL_LINK_MODE_Asym_Pause_BIT, advertising, adv & ADVERTISE_PAUSE_ASYM);
}

#[inline]
pub unsafe fn mii_adv_to_linkmode_adv_t(advertising: *mut ::core::ffi::c_ulong, adv: u32) {
    linkmode_zero(advertising);
    mii_adv_mod_linkmode_adv_t(advertising, adv);
}

#[inline]
pub unsafe fn mii_lpa_to_linkmode_lpa_t(lp_advertising: *mut ::core::ffi::c_ulong, lpa: u32) {
    mii_adv_to_linkmode_adv_t(lp_advertising, lpa);
    if lpa & LPA_LPACK != 0 { linkmode_set_bit(ETHTOOL_LINK_MODE_Autoneg_BIT, lp_advertising); }
}

#[inline]
pub unsafe fn mii_lpa_mod_linkmode_lpa_t(lp_advertising: *mut ::core::ffi::c_ulong, lpa: u32) {
    mii_adv_mod_linkmode_adv_t(lp_advertising, lpa);
    linkmode_mod_bit(ETHTOOL_LINK_MODE_Autoneg_BIT, lp_advertising, lpa & LPA_LPACK);
}

#[inline]
pub unsafe fn mii_ctrl1000_mod_linkmode_adv_t(advertising: *mut ::core::ffi::c_ulong, ctrl1000: u32) {
    linkmode_mod_bit(ETHTOOL_LINK_MODE_1000baseT_Half_BIT, advertising, ctrl1000 & ADVERTISE_1000HALF);
    linkmode_mod_bit(ETHTOOL_LINK_MODE_1000baseT_Full_BIT, advertising, ctrl1000 & ADVERTISE_1000FULL);
}

#[inline]
pub unsafe fn linkmode_adv_to_lcl_adv_t(advertising: *const ::core::ffi::c_ulong) -> u32 {
    let mut lcl_adv = 0;
    if linkmode_test_bit(ETHTOOL_LINK_MODE_Pause_BIT, advertising) != 0 { lcl_adv |= ADVERTISE_PAUSE_CAP; }
    if linkmode_test_bit(ETHTOOL_LINK_MODE_Asym_Pause_BIT, advertising) != 0 { lcl_adv |= ADVERTISE_PAUSE_ASYM; }
    lcl_adv
}

#[inline]
pub unsafe fn mii_lpa_mod_linkmode_x(linkmodes: *mut ::core::ffi::c_ulong, lpa: u16, fd_bit: ::core::ffi::c_int) {
    linkmode_mod_bit(ETHTOOL_LINK_MODE_Autoneg_BIT, linkmodes, (lpa as u32) & LPA_LPACK);
    linkmode_mod_bit(ETHTOOL_LINK_MODE_Pause_BIT, linkmodes, (lpa as u32) & LPA_1000XPAUSE);
    linkmode_mod_bit(ETHTOOL_LINK_MODE_Asym_Pause_BIT, linkmodes, (lpa as u32) & LPA_1000XPAUSE_ASYM);
    linkmode_mod_bit(fd_bit, linkmodes, (lpa as u32) & LPA_1000XFULL);
}

#[inline]
pub unsafe fn linkmode_adv_to_mii_adv_x(linkmodes: *const ::core::ffi::c_ulong, fd_bit: ::core::ffi::c_int) -> u16 {
    let mut adv = 0;
    if linkmode_test_bit(fd_bit, linkmodes) != 0 { adv |= ADVERTISE_1000XFULL as u16; }
    if linkmode_test_bit(ETHTOOL_LINK_MODE_Pause_BIT, linkmodes) != 0 { adv |= ADVERTISE_1000XPAUSE as u16; }
    if linkmode_test_bit(ETHTOOL_LINK_MODE_Asym_Pause_BIT, linkmodes) != 0 { adv |= ADVERTISE_1000XPSE_ASYM as u16; }
    adv
}

#[inline]
pub fn mii_advertise_flowctrl(cap: ::core::ffi::c_int) -> u16 {
    let mut adv = 0;
    if cap & FLOW_CTRL_RX != 0 { adv = (ADVERTISE_PAUSE_CAP | ADVERTISE_PAUSE_ASYM) as u16; }
    if cap & FLOW_CTRL_TX != 0 { adv ^= ADVERTISE_PAUSE_ASYM as u16; }
    adv
}

#[inline]
pub fn mii_resolve_flowctrl_fdx(lcladv: u16, rmtadv: u16) -> u8 {
    if (lcladv as u32) & (rmtadv as u32) & ADVERTISE_PAUSE_CAP != 0 { (FLOW_CTRL_TX | FLOW_CTRL_RX) as u8 }
    else if (lcladv as u32) & (rmtadv as u32) & ADVERTISE_PAUSE_ASYM != 0 {
        if (lcladv as u32) & ADVERTISE_PAUSE_CAP != 0 { FLOW_CTRL_RX as u8 }
        else if (rmtadv as u32) & ADVERTISE_PAUSE_CAP != 0 { FLOW_CTRL_TX as u8 } else { 0 }
    } else { 0 }
}

#[inline]
pub fn mii_bmcr_encode_fixed(speed: ::core::ffi::c_int, duplex: ::core::ffi::c_int) -> u16 {
    let mut bmcr = match speed {
        SPEED_2500 | SPEED_1000 => BMCR_SPEED1000,
        SPEED_100 => BMCR_SPEED100,
        SPEED_10 => BMCR_SPEED10,
        _ => BMCR_SPEED10,
    } as u16;
    if duplex == DUPLEX_FULL { bmcr |= BMCR_FULLDPLX as u16; }
    bmcr
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
