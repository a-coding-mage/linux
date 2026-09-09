// SPDX-License-Identifier: GPL-2.0-only
/*
 * AP handling
 *
 * Partially
 * Copyright (C) 2026 Intel Corporation
 */

// Dependencies supplied by the surrounding translation unit.

unsafe fn ieee80211_send_eml_op_mode_notif(
    sdata: *mut ieee80211_sub_if_data,
    req: *mut ieee80211_mgmt,
    opt_len: i32,
) {
    let mut len: i32 = IEEE80211_MIN_ACTION_SIZE_eml_omn as i32;
    let local = (*sdata).local;
    let mut mgmt: *mut ieee80211_mgmt;
    let mut skb: *mut sk_buff;

    len += opt_len; /* optional len */
    skb = dev_alloc_skb((*local).tx_headroom + len as _);
    if skb.is_null() {
        return;
    }

    skb_reserve(skb, (*local).tx_headroom);
    mgmt = skb_put_zero(skb, len as _);
    (*mgmt).frame_control = cpu_to_le16(
        IEEE80211_FTYPE_MGMT | IEEE80211_STYPE_ACTION,
    );
    memcpy((*mgmt).da.as_mut_ptr(), (*req).sa.as_ptr(), ETH_ALEN);
    memcpy((*mgmt).sa.as_mut_ptr(), (*sdata).vif.addr.as_ptr(), ETH_ALEN);
    memcpy((*mgmt).bssid.as_mut_ptr(), (*sdata).vif.addr.as_ptr(), ETH_ALEN);

    (*mgmt).u.action.category = WLAN_CATEGORY_PROTECTED_EHT;
    (*mgmt).u.action.action_code = WLAN_PROTECTED_EHT_ACTION_EML_OP_MODE_NOTIF;
    (*mgmt).u.action.eml_omn.dialog_token =
        (*req).u.action.eml_omn.dialog_token;
    (*mgmt).u.action.eml_omn.control = (*req).u.action.eml_omn.control &
        !(IEEE80211_EML_CTRL_EMLSR_PARAM_UPDATE |
          IEEE80211_EML_CTRL_INDEV_COEX_ACT);
    /* Copy optional fields from the received notification frame */
    memcpy(
        (*mgmt).u.action.eml_omn.variable.as_mut_ptr(),
        (*req).u.action.eml_omn.variable.as_ptr(),
        opt_len as usize,
    );

    ieee80211_tx_skb(sdata, skb);
}

unsafe fn ieee80211_rx_eml_op_mode_notif(
    sdata: *mut ieee80211_sub_if_data,
    skb: *mut sk_buff,
) {
    let len: i32 = IEEE80211_MIN_ACTION_SIZE_eml_omn as i32;
    let r#type = ieee80211_vif_type_p2p(&(*sdata).vif);
    let status = IEEE80211_SKB_RXCB(skb);
    let ift_ext_capa: *const wiphy_iftype_ext_capab;
    let mgmt = (*skb).data as *mut ieee80211_mgmt;
    let local = (*sdata).local;
    let control: u8 = (*mgmt).u.action.eml_omn.control;
    let ptr = (*mgmt).u.action.eml_omn.variable.as_mut_ptr();
    let mut eml_params = ieee80211_eml_params {
        link_id: (*status).link_id,
        control,
        ..Default::default()
    };
    let sta: *mut sta_info;
    let mut opt_len: i32 = 0;

    if !ieee80211_vif_is_mld(&(*sdata).vif) { return; }
    /* eMLSR and eMLMR can't be enabled at the same time */
    if (control & IEEE80211_EML_CTRL_EMLSR_MODE) != 0 &&
       (control & IEEE80211_EML_CTRL_EMLMR_MODE) != 0 { return; }
    if (control & IEEE80211_EML_CTRL_EMLMR_MODE) != 0 &&
       (control & IEEE80211_EML_CTRL_EMLSR_PARAM_UPDATE) != 0 { return; }

    ift_ext_capa = cfg80211_get_iftype_ext_capa((*local).hw.wiphy, r#type);
    if ift_ext_capa.is_null() { return; }
    if !(*status).link_valid { return; }
    sta = sta_info_get_bss(sdata, (*mgmt).sa.as_ptr());
    if sta.is_null() { return; }

    if (control & IEEE80211_EML_CTRL_EMLSR_MODE) != 0 {
        let emlsr_param_update_len: i32;
        if ((*ift_ext_capa).eml_capabilities & IEEE80211_EML_CAP_EMLSR_SUPP) == 0 { return; }
        opt_len += core::mem::size_of::<__le16>() as i32;
        /* eMLSR param update field is not part of Notification frame
         * sent by the AP to client so account it separately. */
        emlsr_param_update_len = ((control & IEEE80211_EML_CTRL_EMLSR_PARAM_UPDATE) != 0) as i32;
        if (*skb).len < (len + opt_len + emlsr_param_update_len) as usize { return; }
        if (control & IEEE80211_EML_CTRL_EMLSR_PARAM_UPDATE) != 0 {
            let pad_delay = u8_get_bits(*ptr.add(2), IEEE80211_EML_EMLSR_PAD_DELAY);
            if pad_delay > IEEE80211_EML_CAP_EML_PADDING_DELAY_256US { return; }
            let trans_delay = u8_get_bits(*ptr.add(2), IEEE80211_EML_EMLSR_TRANS_DELAY);
            if trans_delay > IEEE80211_EML_CAP_EMLSR_TRANSITION_DELAY_256US { return; }
            (*sta).sta.eml_cap = u8_replace_bits((*sta).sta.eml_cap, pad_delay, IEEE80211_EML_CAP_EML_PADDING_DELAY);
            (*sta).sta.eml_cap = u8_replace_bits((*sta).sta.eml_cap, trans_delay, IEEE80211_EML_CAP_EML_TRANSITION_DELAY);
        }
    }

    if (control & IEEE80211_EML_CTRL_EMLMR_MODE) != 0 {
        if ((*ift_ext_capa).eml_capabilities & IEEE80211_EML_CAP_EMLMR_SUPPORT) == 0 { return; }
        opt_len += core::mem::size_of::<__le16>() as i32;
        opt_len += 1;
        if (*skb).len < (len + opt_len) as usize { return; }
        eml_params.emlmr_mcs_map_count = *ptr.add(2);
        if eml_params.emlmr_mcs_map_count > 2 { return; }
        let mcs_map_size = 3 * (1 + eml_params.emlmr_mcs_map_count as i32);
        opt_len += mcs_map_size;
        if (*skb).len < (len + opt_len) as usize { return; }
        for i in 0..mcs_map_size {
            let rx_mcs = u8_get_bits(*ptr.add(3 + i as usize), IEEE80211_EML_EMLMR_RX_MCS_MAP);
            if rx_mcs > 8 { return; }
            let tx_mcs = u8_get_bits(*ptr.add(3 + i as usize), IEEE80211_EML_EMLMR_TX_MCS_MAP);
            if tx_mcs > 8 { return; }
        }
        memcpy(eml_params.emlmr_mcs_map_bw.as_mut_ptr(), ptr.add(3), mcs_map_size as usize);
    }

    if (control & (IEEE80211_EML_CTRL_EMLSR_MODE | IEEE80211_EML_CTRL_EMLMR_MODE)) != 0 {
        eml_params.link_bitmap = get_unaligned_le16(ptr);
        if (eml_params.link_bitmap & (*sdata).vif.active_links) != eml_params.link_bitmap { return; }
    }
    if drv_set_eml_op_mode(sdata, &mut (*sta).sta, &mut eml_params) != 0 { return; }
    ieee80211_send_eml_op_mode_notif(sdata, mgmt, opt_len);
}

unsafe fn ieee80211_rx_uhr_link_reconfig_req(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) {
    let mgmt = (*skb).data as *mut ieee80211_mgmt;
    let sta = sta_info_get_bss(sdata, (*mgmt).sa.as_ptr());
    if sta.is_null() { return; }
    /* rx.c only accepts IEEE80211_UHR_LINK_RECONFIG_REQUEST_OMP_REQUEST
     * which is valid, so no need to check the frame type/format/etc. */
    let elems = ieee802_11_parse_elems((*mgmt).u.action.uhr_link_reconf_req.variable.as_ptr(),
        (*skb).len - IEEE80211_MIN_ACTION_SIZE_uhr_link_reconf_req as usize,
        IEEE80211_FTYPE_MGMT | IEEE80211_STYPE_ACTION, core::ptr::null_mut());
    /* STA will assume we processed it, not good */
    if elems.is_null() || (*elems).ml_reconf.is_null() { return; }
    let mut sub: *const element;
    for_each_mle_subelement!(sub, (*elems).ml_reconf as *const u8, (*elems).ml_reconf_len, {
        let prof = (*sub).data as *const ieee80211_mle_per_sta_profile;
        if (*sub).id != IEEE80211_MLE_SUBELEM_PER_STA_PROFILE { continue; }
        if !ieee80211_mle_reconf_sta_prof_size_ok((*sub).data, (*sub).datalen) { return; }
        let control = le16_to_cpu((*prof).control);
        let link_id = (control & IEEE80211_MLE_STA_RECONF_CONTROL_LINK_ID) as usize;
        if link_id >= IEEE80211_MLD_MAX_NUM_LINKS { return; }
        let link = sdata_dereference((*sdata).link[link_id], sdata);
        if link.is_null() { continue; }
        let chanctx_conf = sdata_dereference((*link).conf.chanctx_conf, sdata);
        if chanctx_conf.is_null() { continue; }
        let chanctx = container_of!(chanctx_conf, ieee80211_chanctx, conf);
        let link_sta = sdata_dereference((*sta).link[link_id], sdata);
        if link_sta.is_null() { continue; }
        if control & !(IEEE80211_MLE_STA_RECONF_CONTROL_LINK_ID | IEEE80211_MLE_STA_RECONF_CONTROL_OPERATION_TYPE) != 0 { continue; }
        if u16_get_bits(control, IEEE80211_MLE_STA_RECONF_CONTROL_OPERATION_TYPE) != IEEE80211_MLE_STA_RECONF_CONTROL_OPERATION_TYPE_UHR_OMP_UPD { continue; }
        let mut chg: *const element;
        for_each_element_extid!(chg, WLAN_EID_EXT_UHR_MODE_CHG,
            (*prof).variable.as_ptr().add((*prof).sta_info_len - 1),
            (*sub).datalen - core::mem::size_of::<ieee80211_mle_per_sta_profile>() - (*prof).sta_info_len + 1, {
            let mut tuple: *const ieee80211_uhr_mode_change_tuple;
            for_each_uhr_mode_change_tuple!((*chg).data.add(1), (*chg).datalen - 1, tuple, {
                let id = le16_get_bits((*tuple).control, IEEE80211_UHR_MODE_CHANGE_CONTROL_MODE_ID);
                let enabled = le16_get_bits((*tuple).control, IEEE80211_UHR_MODE_CHANGE_CONTROL_MODE_ENABLE) != 0;
                /* only handle DBE (for now?) */
                if id != IEEE80211_UHR_MODE_CHANGE_MODE_ID_DBE { continue; }
                (*link_sta).uhr_dbe_enabled = enabled;
                /* also recalculates and updates per-STA bw */
                ieee80211_recalc_chanctx_min_def((*sdata).local, chanctx);
            });
        });
    });
    /* TODO: send a response */
}

pub unsafe fn ieee80211_ap_rx_queued_frame(sdata: *mut ieee80211_sub_if_data, skb: *mut sk_buff) {
    let mgmt = (*skb).data as *mut ieee80211_mgmt;
    /* rx.c cannot queue any non-action frames to AP interfaces */
    if WARN_ON(!ieee80211_is_action((*mgmt).frame_control)) { return; }
    match (*mgmt).u.action.category {
        WLAN_CATEGORY_PROTECTED_EHT => match (*mgmt).u.action.action_code {
            WLAN_PROTECTED_EHT_ACTION_EML_OP_MODE_NOTIF => ieee80211_rx_eml_op_mode_notif(sdata, skb),
            _ => (),
        },
        WLAN_CATEGORY_PROTECTED_UHR => match (*mgmt).u.action.action_code {
            IEEE80211_PROTECTED_UHR_ACTION_LINK_RECONFIG_REQUEST => ieee80211_rx_uhr_link_reconfig_req(sdata, skb),
            _ => (),
        },
        _ => (),
    }
}

pub unsafe fn ieee80211_uhr_disable_dbe_all_stas(link: *mut ieee80211_link_data) {
    let sdata = (*link).sdata;
    let local = (*sdata).local;
    let chanctx_conf = sdata_dereference((*link).conf.chanctx_conf, sdata);
    if chanctx_conf.is_null() { return; }
    let chanctx = container_of!(chanctx_conf, ieee80211_chanctx, conf);
    let link_id = (*link).link_id as usize;
    let mut sta: *mut sta_info;
    list_for_each_entry!(sta, &mut (*local).sta_list, list, {
        if (*sta).sdata.bss != (*sdata).bss { continue; }
        let link_sta = sdata_dereference((*sta).link[link_id], sdata);
        if link_sta.is_null() { continue; }
        (*link_sta).uhr_dbe_enabled = false;
    });
    /* also recalculates and updates per-STA bw */
    ieee80211_recalc_chanctx_min_def(local, chanctx);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
