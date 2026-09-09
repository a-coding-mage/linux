// SPDX-License-Identifier: GPL-2.0-only
/*
 * EHT handling
 *
 * Copyright(c) 2021-2026 Intel Corporation
 */

// Dependencies supplied by the surrounding translation unit.

extern "C" {
    fn ieee80211_get_eht_iftype_cap_vif(
        sband: *const ieee80211_supported_band,
        vif: *const ieee80211_vif,
    ) -> bool;
    fn ieee80211_eht_mcs_nss_size(
        he_cap_ie_elem: *const ieee80211_he_cap_elem,
        fixed: *const ieee80211_eht_cap_fixed,
        is_station: bool,
    ) -> u8;
    fn ieee80211_eht_ppe_size(ppe_hdr: u16, phy_cap_info: *const u8) -> u8;
    fn ieee80211_sta_recalc_aggregates(sta: *const ieee80211_sta);
}

pub unsafe fn ieee80211_eht_cap_ie_to_sta_eht_cap(
    sdata: *mut ieee80211_sub_if_data,
    sband: *mut ieee80211_supported_band,
    he_cap_ie: *const u8,
    he_cap_len: u8,
    eht_cap_ie_elem: *const ieee80211_eht_cap_elem,
    eht_cap_len: u8,
    link_sta: *mut link_sta_info,
) {
    let eht_cap = &mut (*(*link_sta).pub_).eht_cap;
    let he_cap_ie_elem = he_cap_ie as *const ieee80211_he_cap_elem;
    let mut eht_ppe_size: u8 = 0;
    let mcs_nss_size: u8;
    let mut eht_total_size: u8 = core::mem::size_of::<ieee80211_eht_cap_elem>() as u8;
    let mut pos = eht_cap_ie_elem as *const u8;

    core::ptr::write_bytes(
        eht_cap as *mut ieee80211_sta_eht_cap as *mut u8,
        0,
        core::mem::size_of::<ieee80211_sta_eht_cap>(),
    );

    if eht_cap_ie_elem.is_null()
        || !ieee80211_get_eht_iftype_cap_vif(sband, &(*sdata).vif)
    {
        return;
    }

    mcs_nss_size = ieee80211_eht_mcs_nss_size(
        he_cap_ie_elem,
        &(*eht_cap_ie_elem).fixed,
        (*sdata).vif.type_ == NL80211_IFTYPE_STATION,
    );

    eht_total_size = eht_total_size.wrapping_add(mcs_nss_size);

    /* Calculate the PPE thresholds length only if the header is present */
    if (*eht_cap_ie_elem).fixed.phy_cap_info[5]
        & IEEE80211_EHT_PHY_CAP5_PPE_THRESHOLD_PRESENT != 0
    {
        let eht_ppe_hdr: u16;

        if eht_cap_len < eht_total_size.wrapping_add(core::mem::size_of::<u16>() as u8) {
            return;
        }

        eht_ppe_hdr = u16::from_le_bytes([
            (*eht_cap_ie_elem).optional[mcs_nss_size as usize],
            (*eht_cap_ie_elem).optional[mcs_nss_size as usize + 1],
        ]);
        eht_ppe_size = ieee80211_eht_ppe_size(
            eht_ppe_hdr,
            (*eht_cap_ie_elem).fixed.phy_cap_info.as_ptr(),
        );
        eht_total_size = eht_total_size.wrapping_add(eht_ppe_size);

        /* we calculate as if NSS > 8 are valid, but don't handle that */
        if (eht_ppe_size as usize) > core::mem::size_of_val(&eht_cap.eht_ppe_thres) {
            return;
        }
    }

    if eht_cap_len < eht_total_size {
        return;
    }

    /* Copy the static portion of the EHT capabilities */
    core::ptr::copy_nonoverlapping(
        pos,
        &mut eht_cap.eht_cap_elem as *mut ieee80211_eht_cap_elem as *mut u8,
        core::mem::size_of::<ieee80211_eht_cap_elem>(),
    );
    pos = pos.add(core::mem::size_of::<ieee80211_eht_cap_elem>());

    /* Copy MCS/NSS which depends on the peer capabilities */
    core::ptr::write_bytes(
        &mut eht_cap.eht_mcs_nss_supp as *mut _ as *mut u8,
        0,
        core::mem::size_of_val(&eht_cap.eht_mcs_nss_supp),
    );
    core::ptr::copy_nonoverlapping(
        pos,
        &mut eht_cap.eht_mcs_nss_supp as *mut _ as *mut u8,
        mcs_nss_size as usize,
    );

    if eht_ppe_size != 0 {
        core::ptr::copy_nonoverlapping(
            (*eht_cap_ie_elem).optional.as_ptr().add(mcs_nss_size as usize),
            eht_cap.eht_ppe_thres.as_mut_ptr() as *mut u8,
            eht_ppe_size as usize,
        );
    }

    eht_cap.has_eht = true;

    /*
     * The MPDU length bits are reserved on all but 2.4 GHz and get set via
     * VHT (5 GHz) or HE (6 GHz) capabilities.
     */
    if (*sband).band != NL80211_BAND_2GHZ {
        return;
    }

    match (eht_cap.eht_cap_elem.mac_cap_info[0] & IEEE80211_EHT_MAC_CAP0_MAX_MPDU_LEN_MASK)
        >> IEEE80211_EHT_MAC_CAP0_MAX_MPDU_LEN_SHIFT
    {
        IEEE80211_EHT_MAC_CAP0_MAX_MPDU_LEN_11454 => {
            (*(*link_sta).pub_).agg.max_amsdu_len = IEEE80211_MAX_MPDU_LEN_VHT_11454;
        }
        IEEE80211_EHT_MAC_CAP0_MAX_MPDU_LEN_7991 => {
            (*(*link_sta).pub_).agg.max_amsdu_len = IEEE80211_MAX_MPDU_LEN_VHT_7991;
        }
        IEEE80211_EHT_MAC_CAP0_MAX_MPDU_LEN_3895 | _ => {
            (*(*link_sta).pub_).agg.max_amsdu_len = IEEE80211_MAX_MPDU_LEN_VHT_3895;
        }
    }

    ieee80211_sta_recalc_aggregates(&(*link_sta).sta.sta);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
