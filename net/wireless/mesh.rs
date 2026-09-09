// SPDX-License-Identifier: GPL-2.0
/*
 * Portions
 * Copyright (C) 2022-2024 Intel Corporation
 */

// Dependencies supplied by the surrounding kernel translation.

/* Default values, timeouts in ms */
const MESH_TTL: i32 = 31;
const MESH_DEFAULT_ELEMENT_TTL: i32 = 31;
const MESH_MAX_RETR: i32 = 3;
const MESH_RET_T: i32 = 100;
const MESH_CONF_T: i32 = 100;
const MESH_HOLD_T: i32 = 100;

const MESH_PATH_TIMEOUT: i32 = 5000;
const MESH_RANN_INTERVAL: i32 = 5000;
const MESH_PATH_TO_ROOT_TIMEOUT: i32 = 6000;
const MESH_ROOT_INTERVAL: i32 = 5000;
const MESH_ROOT_CONFIRMATION_INTERVAL: i32 = 2000;
const MESH_DEFAULT_PLINK_TIMEOUT: i32 = 1800; /* timeout in seconds */

/* Minimum interval between two consecutive PREQs originated by the same
 * interface
 */
const MESH_PREQ_MIN_INT: i32 = 10;
const MESH_PERR_MIN_INT: i32 = 100;
const MESH_DIAM_TRAVERSAL_TIME: i32 = 50;

const MESH_RSSI_THRESHOLD: i32 = 0;

/*
 * A path will be refreshed if it is used PATH_REFRESH_TIME milliseconds
 * before timing out.  This way it will remain ACTIVE and no data frames
 * will be unnecessarily held in the pending queue.
 */
const MESH_PATH_REFRESH_TIME: i32 = 1000;
const MESH_MIN_DISCOVERY_TIMEOUT: i32 = 2 * MESH_DIAM_TRAVERSAL_TIME;

/* Default maximum number of established plinks per interface */
const MESH_MAX_ESTAB_PLINKS: i32 = 32;

const MESH_MAX_PREQ_RETRIES: i32 = 4;

const MESH_SYNC_NEIGHBOR_OFFSET_MAX: i32 = 50;

const MESH_DEFAULT_BEACON_INTERVAL: i32 = 1000; /* in 1024 us units (=TUs) */
const MESH_DEFAULT_DTIM_PERIOD: i32 = 2;
const MESH_DEFAULT_AWAKE_WINDOW: i32 = 10; /* in 1024 us units (=TUs) */

pub const default_mesh_config: mesh_config = mesh_config {
    dot11MeshRetryTimeout: MESH_RET_T,
    dot11MeshConfirmTimeout: MESH_CONF_T,
    dot11MeshHoldingTimeout: MESH_HOLD_T,
    dot11MeshMaxRetries: MESH_MAX_RETR,
    dot11MeshTTL: MESH_TTL,
    element_ttl: MESH_DEFAULT_ELEMENT_TTL,
    auto_open_plinks: true,
    dot11MeshMaxPeerLinks: MESH_MAX_ESTAB_PLINKS,
    dot11MeshNbrOffsetMaxNeighbor: MESH_SYNC_NEIGHBOR_OFFSET_MAX,
    dot11MeshHWMPactivePathTimeout: MESH_PATH_TIMEOUT,
    dot11MeshHWMPpreqMinInterval: MESH_PREQ_MIN_INT,
    dot11MeshHWMPperrMinInterval: MESH_PERR_MIN_INT,
    dot11MeshHWMPnetDiameterTraversalTime: MESH_DIAM_TRAVERSAL_TIME,
    dot11MeshHWMPmaxPREQretries: MESH_MAX_PREQ_RETRIES,
    path_refresh_time: MESH_PATH_REFRESH_TIME,
    min_discovery_timeout: MESH_MIN_DISCOVERY_TIMEOUT,
    dot11MeshHWMPRannInterval: MESH_RANN_INTERVAL,
    dot11MeshGateAnnouncementProtocol: false,
    dot11MeshForwarding: true,
    rssi_threshold: MESH_RSSI_THRESHOLD,
    ht_opmode: IEEE80211_HT_OP_MODE_PROTECTION_NONHT_MIXED,
    dot11MeshHWMPactivePathToRootTimeout: MESH_PATH_TO_ROOT_TIMEOUT,
    dot11MeshHWMProotInterval: MESH_ROOT_INTERVAL,
    dot11MeshHWMPconfirmationInterval: MESH_ROOT_CONFIRMATION_INTERVAL,
    power_mode: NL80211_MESH_POWER_ACTIVE,
    dot11MeshAwakeWindowDuration: MESH_DEFAULT_AWAKE_WINDOW,
    plink_timeout: MESH_DEFAULT_PLINK_TIMEOUT,
    dot11MeshNolearn: false,
};

pub const default_mesh_setup: mesh_setup = mesh_setup {
    /* cfg80211_join_mesh() will pick a channel if needed */
    sync_method: IEEE80211_SYNC_METHOD_NEIGHBOR_OFFSET,
    path_sel_proto: IEEE80211_PATH_PROTOCOL_HWMP,
    path_metric: IEEE80211_PATH_METRIC_AIRTIME,
    auth_id: 0, /* open */
    ie: core::ptr::null_mut(),
    ie_len: 0,
    is_secure: false,
    user_mpm: false,
    beacon_interval: MESH_DEFAULT_BEACON_INTERVAL,
    dtim_period: MESH_DEFAULT_DTIM_PERIOD,
};

pub unsafe fn __cfg80211_join_mesh(
    rdev: *mut cfg80211_registered_device,
    dev: *mut net_device,
    setup: *mut mesh_setup,
    conf: *const mesh_config,
) -> i32 {
    let wdev = (*dev).ieee80211_ptr;
    let mut err: i32;

    BUILD_BUG_ON!(IEEE80211_MAX_SSID_LEN != IEEE80211_MAX_MESH_ID_LEN);

    lockdep_assert_wiphy((*wdev).wiphy);

    if (*dev).ieee80211_ptr.iftype != NL80211_IFTYPE_MESH_POINT { return -EOPNOTSUPP; }
    if !(*rdev).wiphy.flags.contains(WIPHY_FLAG_MESH_AUTH) && (*setup).is_secure { return -EOPNOTSUPP; }
    if (*wdev).u_.mesh.id_len != 0 { return -EALREADY; }
    if (*setup).mesh_id_len == 0 { return -EINVAL; }
    if (*rdev).ops.join_mesh.is_none() { return -EOPNOTSUPP; }
    if (*wdev).links[0].cac_started { return -EBUSY; }

    if (*setup).chandef.chan.is_null() {
        (*setup).chandef = (*wdev).u_.mesh.preset_chandef;
    }
    if (*setup).chandef.chan.is_null() {
        let mut band: u32 = 0;
        while band < NUM_NL80211_BANDS {
            let sband = (*rdev).wiphy.bands[band as usize];
            if !sband.is_null() {
                let mut i = 0;
                while i < (*sband).n_channels {
                    let chan = (*sband).channels.add(i as usize);
                    if ((*chan).flags & (IEEE80211_CHAN_NO_IR | IEEE80211_CHAN_DISABLED | IEEE80211_CHAN_RADAR)) == 0 {
                        (*setup).chandef.chan = chan;
                        break;
                    }
                    i += 1;
                }
            }
            if !(*setup).chandef.chan.is_null() { break; }
            band += 1;
        }
        if (*setup).chandef.chan.is_null() { return -EINVAL; }
        (*setup).chandef.width = NL80211_CHAN_WIDTH_20_NOHT;
        (*setup).chandef.center_freq1 = (*(*setup).chandef.chan).center_freq;
    }

    if (*setup).basic_rates == 0 {
        let sband = (*rdev).wiphy.bands[(*setup).chandef.chan.band as usize];
        if (*setup).chandef.chan.band == NL80211_BAND_2GHZ {
            for i in 0..(*sband).n_bitrates {
                if (*sband).bitrates.add(i as usize).bitrate == 10 {
                    (*setup).basic_rates = 1 << i;
                    break;
                }
            }
        } else {
            (*setup).basic_rates = ieee80211_mandatory_rates(sband);
        }
    }

    err = cfg80211_chandef_dfs_required(&(*rdev).wiphy, &(*setup).chandef, NL80211_IFTYPE_MESH_POINT);
    if err < 0 { return err; }
    if err > 0 && !(*setup).userspace_handles_dfs { return -EINVAL; }
    if !cfg80211_reg_can_beacon(&(*rdev).wiphy, &(*setup).chandef, NL80211_IFTYPE_MESH_POINT) { return -EINVAL; }

    err = rdev_join_mesh(rdev, dev, conf, setup);
    if err == 0 {
        core::ptr::copy_nonoverlapping((*setup).mesh_id, (*wdev).u_.mesh.id, (*setup).mesh_id_len as usize);
        (*wdev).u_.mesh.id_len = (*setup).mesh_id_len;
        (*wdev).u_.mesh.chandef = (*setup).chandef;
        (*wdev).u_.mesh.beacon_interval = (*setup).beacon_interval;
    }
    err
}

pub unsafe fn cfg80211_set_mesh_channel(rdev: *mut cfg80211_registered_device, wdev: *mut wireless_dev, chandef: *mut cfg80211_chan_def) -> i32 {
    let mut err: i32;
    if (*rdev).ops.libertas_set_mesh_channel.is_some() {
        if (*chandef).width != NL80211_CHAN_WIDTH_20_NOHT { return -EINVAL; }
        if !netif_running((*wdev).netdev) { return -ENETDOWN; }
        err = rdev_libertas_set_mesh_channel(rdev, (*wdev).netdev, (*chandef).chan);
        if err == 0 { (*wdev).u_.mesh.chandef = *chandef; }
        return err;
    }
    if (*wdev).u_.mesh.id_len != 0 { return -EBUSY; }
    (*wdev).u_.mesh.preset_chandef = *chandef;
    0
}

pub unsafe fn cfg80211_leave_mesh(rdev: *mut cfg80211_registered_device, dev: *mut net_device) -> i32 {
    let wdev = (*dev).ieee80211_ptr;
    lockdep_assert_wiphy((*wdev).wiphy);
    if (*dev).ieee80211_ptr.iftype != NL80211_IFTYPE_MESH_POINT { return -EOPNOTSUPP; }
    if (*rdev).ops.leave_mesh.is_none() { return -EOPNOTSUPP; }
    if (*wdev).u_.mesh.id_len == 0 { return -ENOTCONN; }
    let err = rdev_leave_mesh(rdev, dev);
    if err == 0 {
        (*wdev).conn_owner_nlportid = 0;
        (*wdev).u_.mesh.id_len = 0;
        (*wdev).u_.mesh.beacon_interval = 0;
        core::ptr::write_bytes(&mut (*wdev).u_.mesh.chandef, 0, 1);
        rdev_set_qos_map(rdev, dev, core::ptr::null_mut());
        cfg80211_sched_dfs_chan_update(rdev);
    }
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
