// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2011-2012, Pavel Zubarev <pavel.zubarev@gmail.com>
 * Copyright 2011-2012, Marco Porsch <marco.porsch@s2005.tu-chemnitz.de>
 * Copyright 2011-2012, cozybit Inc.
 * Copyright (C) 2021,2023 Intel Corporation
 */

// Dependencies supplied by the surrounding kernel translation.

/* This is not in the standard. It represents a tolerable tsf drift below
 * which we do no TSF adjustment.
 */
const TOFFSET_MINIMUM_ADJUSTMENT: i64 = 10;

/* This is not in the standard. It is a margin added to the
 * Toffset setpoint to mitigate TSF overcorrection
 * introduced by TSF adjustment latency.
 */
const TOFFSET_SET_MARGIN: i64 = 20;

/* This is not in the standard. It represents the maximum Toffset jump above
 * which we'll invalidate the Toffset setpoint and choose a new setpoint. This
 * could be, for instance, in case a neighbor is restarted and its TSF counter
 * reset.
 */
const TOFFSET_MAXIMUM_ADJUSTMENT: i64 = 800; // 0.8 ms

#[repr(C)]
struct sync_method {
    method: u8,
    ops: ieee80211_mesh_sync_ops,
}

/**
 * mesh_peer_tbtt_adjusting - check if an mp is currently adjusting its TBTT
 *
 * @cfg: mesh config element from the mesh peer (or %NULL)
 *
 * Returns: If the mesh peer is currently adjusting its TBTT
 */
unsafe fn mesh_peer_tbtt_adjusting(cfg: *const ieee80211_meshconf_ie) -> bool {
    !cfg.is_null()
        && ((*cfg).meshconf_cap & IEEE80211_MESHCONF_CAPAB_TBTT_ADJUSTING) != 0
}

pub unsafe fn mesh_sync_adjust_tsf(sdata: *mut ieee80211_sub_if_data) {
    let local = (*sdata).local;
    let ifmsh = &mut (*sdata).u.mesh;
    // sdata->vif.bss_conf.beacon_int in 1024us units, 0.04%
    let beacon_int_fraction: u64 = (*sdata).vif.bss_conf.beacon_int as u64 * 1024 / 2500;
    let mut tsf: u64;
    let tsfdelta: u64;

    spin_lock_bh(&mut ifmsh.sync_offset_lock);
    if ifmsh.sync_offset_clockdrift_max < beacon_int_fraction as i64 {
        msync_dbg(sdata, "TSF : max clockdrift=%lld; adjusting\n");
        tsfdelta = (-(ifmsh.sync_offset_clockdrift_max)) as u64;
        ifmsh.sync_offset_clockdrift_max = 0;
    } else {
        msync_dbg(sdata, "TSF : max clockdrift=%lld; adjusting by %llu\n");
        tsfdelta = (-(beacon_int_fraction as i64)) as u64;
        ifmsh.sync_offset_clockdrift_max -= beacon_int_fraction as i64;
    }
    spin_unlock_bh(&mut ifmsh.sync_offset_lock);

    if (*local).ops.offset_tsf.is_some() {
        drv_offset_tsf(local, sdata, tsfdelta);
    } else {
        tsf = drv_get_tsf(local, sdata);
        if tsf != u64::MAX {
            drv_set_tsf(local, sdata, tsf.wrapping_add(tsfdelta));
        }
    }
}

unsafe fn mesh_sync_offset_rx_bcn_presp(
    sdata: *mut ieee80211_sub_if_data,
    stype: u16,
    mgmt: *mut ieee80211_mgmt,
    len: usize,
    mesh_cfg: *const ieee80211_meshconf_ie,
    rx_status: *mut ieee80211_rx_status,
) {
    let ifmsh = &mut (*sdata).u.mesh;
    let local = (*sdata).local;
    let mut sta: *mut sta_info;
    let t_t: u64;
    let t_r: u64;

    WARN_ON(ifmsh.mesh_sp_id != IEEE80211_SYNC_METHOD_NEIGHBOR_OFFSET);
    // standard mentions only beacons
    if stype != IEEE80211_STYPE_BEACON { return; }

    // Get time when timestamp field was received. If we don't have rx timestamps,
    // use current tsf as an approximation. drv_get_tsf() must precede the RCU read section.
    if ieee80211_have_rx_timestamp(rx_status) {
        t_r = ieee80211_calculate_rx_timestamp(&(*local).hw, rx_status, len + FCS_LEN, 24);
    } else {
        t_r = drv_get_tsf(local, sdata);
    }

    rcu_read_lock();
    sta = sta_info_get(sdata, (*mgmt).sa);
    if sta.is_null() { rcu_read_unlock(); return; }

    /* check offset sync conditions (13.13.2.2.1)
     * TODO also sync to dot11MeshNbrOffsetMaxNeighbor non-peer non-MBSS neighbors
     */
    if mesh_peer_tbtt_adjusting(mesh_cfg) {
        msync_dbg(sdata, "STA %pM : is adjusting TBTT\n");
        rcu_read_unlock(); return;
    }

    // Timing offset calculation (see 13.13.2.2.2)
    t_t = le64_to_cpu((*mgmt).u.beacon.timestamp);
    (*(*sta).mesh).t_offset = t_t.wrapping_sub(t_r) as i64;

    if test_sta_flag(sta, WLAN_STA_TOFFSET_KNOWN) {
        let t_clockdrift = (*(*sta).mesh).t_offset_setpoint - (*(*sta).mesh).t_offset;
        msync_dbg(sdata, "STA %pM : t_offset=%lld, t_offset_setpoint=%lld, t_clockdrift=%lld\n");
        if t_clockdrift > TOFFSET_MAXIMUM_ADJUSTMENT || t_clockdrift < -TOFFSET_MAXIMUM_ADJUSTMENT {
            msync_dbg(sdata, "STA %pM : t_clockdrift=%lld too large, setpoint reset\n");
            clear_sta_flag(sta, WLAN_STA_TOFFSET_KNOWN);
            rcu_read_unlock(); return;
        }
        spin_lock_bh(&mut ifmsh.sync_offset_lock);
        if t_clockdrift > ifmsh.sync_offset_clockdrift_max { ifmsh.sync_offset_clockdrift_max = t_clockdrift; }
        spin_unlock_bh(&mut ifmsh.sync_offset_lock);
    } else {
        (*(*sta).mesh).t_offset_setpoint = (*(*sta).mesh).t_offset - TOFFSET_SET_MARGIN;
        set_sta_flag(sta, WLAN_STA_TOFFSET_KNOWN);
        msync_dbg(sdata, "STA %pM : offset was invalid, t_offset=%lld\n");
    }
    rcu_read_unlock();
}

unsafe fn mesh_sync_offset_adjust_tsf(sdata: *mut ieee80211_sub_if_data, _beacon: *mut beacon_data) {
    let ifmsh = &mut (*sdata).u.mesh;
    WARN_ON(ifmsh.mesh_sp_id != IEEE80211_SYNC_METHOD_NEIGHBOR_OFFSET);
    WARN_ON(!rcu_read_lock_held());
    spin_lock_bh(&mut ifmsh.sync_offset_lock);
    if ifmsh.sync_offset_clockdrift_max > TOFFSET_MINIMUM_ADJUSTMENT {
        /* Adjustment is deferred because the driver TSF setter may block. */
        msync_dbg(sdata, "TSF : kicking off TSF adjustment with clockdrift_max=%lld\n");
        set_bit(MESH_WORK_DRIFT_ADJUST, &mut ifmsh.wrkq_flags);
    } else {
        msync_dbg(sdata, "TSF : max clockdrift=%lld; too small to adjust\n");
        ifmsh.sync_offset_clockdrift_max = 0;
    }
    spin_unlock_bh(&mut ifmsh.sync_offset_lock);
}

static sync_methods: [sync_method; 1] = [sync_method {
    method: IEEE80211_SYNC_METHOD_NEIGHBOR_OFFSET,
    ops: ieee80211_mesh_sync_ops {
        rx_bcn_presp: Some(mesh_sync_offset_rx_bcn_presp),
        adjust_tsf: Some(mesh_sync_offset_adjust_tsf),
    },
}];

pub unsafe fn ieee80211_mesh_sync_ops_get(method: u8) -> *const ieee80211_mesh_sync_ops {
    let mut i = 0usize;
    while i < sync_methods.len() {
        if sync_methods[i].method == method { return &sync_methods[i].ops; }
        i += 1;
    }
    core::ptr::null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
