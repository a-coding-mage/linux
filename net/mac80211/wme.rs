// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2004, Instant802 Networks, Inc.
 * Copyright 2013-2014  Intel Mobile Communications GmbH
 * Copyright (C) 2022 Intel Corporation
 */

// Linux/mac80211 dependencies are supplied by the surrounding translation.

/* Default mapping in classifier to work with default
 * queue setup.
 */
pub static ieee802_1d_to_ac: [i32; 8] = [
    IEEE80211_AC_BE,
    IEEE80211_AC_BK,
    IEEE80211_AC_BK,
    IEEE80211_AC_BE,
    IEEE80211_AC_VI,
    IEEE80211_AC_VI,
    IEEE80211_AC_VO,
    IEEE80211_AC_VO,
];

unsafe fn wme_downgrade_ac(skb: *mut sk_buff) -> i32 {
    match (*skb).priority {
        6 | 7 => {
            (*skb).priority = 5; /* VO -> VI */
            0
        }
        4 | 5 => {
            (*skb).priority = 3; /* VI -> BE */
            0
        }
        0 | 3 => {
            (*skb).priority = 2; /* BE -> BK */
            0
        }
        _ => -1,
    }
}

/**
 * ieee80211_fix_reserved_tid - return the TID to use if this one is reserved
 * @tid: the assumed-reserved TID
 *
 * Returns: the alternative TID to use, or 0 on error
 */
#[inline]
fn ieee80211_fix_reserved_tid(tid: u8) -> u8 {
    match tid {
        0 => 3,
        1 => 2,
        2 => 1,
        3 => 0,
        4 => 5,
        5 => 4,
        6 => 7,
        7 => 6,
        _ => 0,
    }
}

unsafe fn ieee80211_downgrade_queue(
    sdata: *mut ieee80211_sub_if_data,
    sta: *mut sta_info,
    skb: *mut sk_buff,
) -> u16 {
    let ifmgd = &mut (*sdata).u.mgd;

    /* in case we are a client verify acm is not set for this ac */
    while (*sdata).wmm_acm & BIT((*skb).priority) != 0 {
        let ac = ieee802_1d_to_ac[(*skb).priority as usize];

        if ifmgd.tx_tspec[ac as usize].admitted_time != 0
            && (*skb).priority == ifmgd.tx_tspec[ac as usize].up
        {
            return ac as u16;
        }

        if wme_downgrade_ac(skb) != 0 {
            /*
             * This should not really happen. The AP has marked all
             * lower ACs to require admission control which is not
             * a reasonable configuration. Allow the frame to be
             * transmitted using AC_BK as a workaround.
             */
            break;
        }
    }

    /* Check to see if this is a reserved TID */
    if !sta.is_null() && (*sta).reserved_tid == (*skb).priority {
        (*skb).priority = ieee80211_fix_reserved_tid((*skb).priority);
    }

    /* look up which queue to use for frames with this 1d tag */
    ieee802_1d_to_ac[(*skb).priority as usize] as u16
}

/* Indicate which queue to use for this fully formed 802.11 frame */
pub unsafe fn ieee80211_select_queue_80211(
    sdata: *mut ieee80211_sub_if_data,
    skb: *mut sk_buff,
    hdr: *mut ieee80211_hdr,
) -> u16 {
    let local = (*sdata).local;
    let info = IEEE80211_SKB_CB(skb);

    /* Ensure hash is set prior to potential SW encryption */
    skb_get_hash(skb);

    if ((*info).control.flags & IEEE80211_TX_CTRL_DONT_REORDER) != 0
        || (*local).hw.queues < IEEE80211_NUM_ACS
    {
        return 0;
    }

    if !ieee80211_is_data((*hdr).frame_control) {
        (*skb).priority = 7;
        return ieee802_1d_to_ac[(*skb).priority as usize] as u16;
    }
    if !ieee80211_is_data_qos((*hdr).frame_control) {
        (*skb).priority = 0;
        return ieee802_1d_to_ac[(*skb).priority as usize] as u16;
    }

    let p = ieee80211_get_qos_ctl(hdr);
    (*skb).priority = *p & IEEE80211_QOS_CTL_TAG1D_MASK;

    ieee80211_downgrade_queue(sdata, core::ptr::null_mut(), skb)
}

pub unsafe fn ieee80211_select_queue(
    sdata: *mut ieee80211_sub_if_data,
    sta: *mut sta_info,
    skb: *mut sk_buff,
) -> u16 {
    let eth = (*skb).data as *const ethhdr;
    let qos_map: *mut mac80211_qos_map;
    let qos: bool;

    /* Ensure hash is set prior to potential SW encryption */
    skb_get_hash(skb);

    /* all mesh/ocb stations are required to support WME */
    if (((*sdata).vif.type_ == NL80211_IFTYPE_MESH_POINT
        && !is_multicast_ether_addr((*eth).h_dest.as_ptr()))
        || ((*sdata).vif.type_ == NL80211_IFTYPE_OCB && !sta.is_null()))
    {
        qos = true;
    } else if !sta.is_null() {
        qos = (*sta).sta.wme;
    } else {
        qos = false;
    }

    if !qos {
        (*skb).priority = 0; /* required for correct WPA/11i MIC */
        return IEEE80211_AC_BE as u16;
    }

    if (*skb).protocol == (*sdata).control_port_protocol {
        (*skb).priority = 7;
        return ieee80211_downgrade_queue(sdata, sta, skb);
    }

    /* use the data classifier to determine what 802.1d tag the
     * data frame has */
    qos_map = rcu_dereference((*sdata).qos_map);
    (*skb).priority = cfg80211_classify8021d(
        skb,
        if !qos_map.is_null() {
            &(*qos_map).qos_map
        } else {
            core::ptr::null()
        },
    );

    ieee80211_downgrade_queue(sdata, sta, skb)
}

/**
 * ieee80211_set_qos_hdr - Fill in the QoS header if there is one.
 *
 * @sdata: local subif
 * @skb: packet to be updated
 */
pub unsafe fn ieee80211_set_qos_hdr(
    sdata: *mut ieee80211_sub_if_data,
    skb: *mut sk_buff,
) {
    let hdr = (*skb).data as *mut ieee80211_hdr;
    let info = IEEE80211_SKB_CB(skb);
    let tid = (*skb).priority & IEEE80211_QOS_CTL_TAG1D_MASK;

    if !ieee80211_is_data_qos((*hdr).frame_control) {
        return;
    }

    let mut p = ieee80211_get_qos_ctl(hdr);

    /* don't overwrite the QoS field of injected frames */
    if ((*info).flags & IEEE80211_TX_CTL_INJECTED) != 0 {
        /* do take into account Ack policy of injected frames */
        if (*p & IEEE80211_QOS_CTL_ACK_POLICY_NOACK) != 0 {
            (*info).flags |= IEEE80211_TX_CTL_NO_ACK;
        }
        return;
    }

    /* set up the first byte */

    /*
     * preserve everything but the TID and ACK policy
     * (which we both write here)
     */
    let mut flags = *p & !(IEEE80211_QOS_CTL_TID_MASK | IEEE80211_QOS_CTL_ACK_POLICY_MASK);

    if is_multicast_ether_addr((*hdr).addr1.as_ptr())
        || ((*sdata).noack_map & BIT(tid)) != 0
    {
        flags |= IEEE80211_QOS_CTL_ACK_POLICY_NOACK;
        (*info).flags |= IEEE80211_TX_CTL_NO_ACK;
    }

    *p = flags | tid;

    /* set up the second byte */
    p = p.add(1);

    if ieee80211_vif_is_mesh(&(*sdata).vif) {
        /* preserve RSPI and Mesh PS Level bit */
        *p &= (IEEE80211_QOS_CTL_RSPI | IEEE80211_QOS_CTL_MESH_PS_LEVEL) >> 8;

        /* Nulls don't have a mesh header (frame body) */
        if !ieee80211_is_qos_nullfunc((*hdr).frame_control) {
            *p |= IEEE80211_QOS_CTL_MESH_CONTROL_PRESENT >> 8;
        }
    } else {
        *p = 0;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
