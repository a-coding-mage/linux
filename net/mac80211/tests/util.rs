// SPDX-License-Identifier: GPL-2.0-only
/* Utilities for mac80211 unit testing */

// Linux/mac80211 dependencies are supplied by the surrounding translation.

const fn chan2g(freq: u32) -> ieee80211_channel {
    ieee80211_channel { band: NL80211_BAND_2GHZ, center_freq: freq as u16, hw_value: freq as u16, ..unsafe { core::mem::zeroed() } }
}

const fn chan5g(freq: u32) -> ieee80211_channel {
    ieee80211_channel { band: NL80211_BAND_5GHZ, center_freq: freq as u16, hw_value: freq as u16, ..unsafe { core::mem::zeroed() } }
}

static CHANNELS_2GHZ: [ieee80211_channel; 14] = [
    chan2g(2412), chan2g(2417), chan2g(2422), chan2g(2427), chan2g(2432), chan2g(2437),
    chan2g(2442), chan2g(2447), chan2g(2452), chan2g(2457), chan2g(2462), chan2g(2467),
    chan2g(2472), chan2g(2484),
];

static CHANNELS_5GHZ: [ieee80211_channel; 4] = [chan5g(5180), chan5g(5200), chan5g(5220), chan5g(5240)];

static BITRATES: [ieee80211_rate; 12] = [
    ieee80211_rate { bitrate: 10, ..unsafe { core::mem::zeroed() } },
    ieee80211_rate { bitrate: 20, flags: IEEE80211_RATE_SHORT_PREAMBLE, ..unsafe { core::mem::zeroed() } },
    ieee80211_rate { bitrate: 55, flags: IEEE80211_RATE_SHORT_PREAMBLE, ..unsafe { core::mem::zeroed() } },
    ieee80211_rate { bitrate: 110, flags: IEEE80211_RATE_SHORT_PREAMBLE, ..unsafe { core::mem::zeroed() } },
    ieee80211_rate { bitrate: 60, ..unsafe { core::mem::zeroed() } }, ieee80211_rate { bitrate: 90, ..unsafe { core::mem::zeroed() } },
    ieee80211_rate { bitrate: 120, ..unsafe { core::mem::zeroed() } }, ieee80211_rate { bitrate: 180, ..unsafe { core::mem::zeroed() } },
    ieee80211_rate { bitrate: 240, ..unsafe { core::mem::zeroed() } }, ieee80211_rate { bitrate: 360, ..unsafe { core::mem::zeroed() } },
    ieee80211_rate { bitrate: 480, ..unsafe { core::mem::zeroed() } }, ieee80211_rate { bitrate: 540, ..unsafe { core::mem::zeroed() } },
];

/* Copied from hwsim except that it only supports 4 EHT streams and STA/P2P mode. */
static SBAND_CAPA_5GHZ: [ieee80211_sband_iftype_data; 1] = [unsafe { core::mem::zeroed() }];

pub unsafe fn t_sdata_init(resource: *mut kunit_resource, _ctx: *mut core::ffi::c_void) -> i32 {
    let test = kunit_get_current_test();
    let t_sdata = kzalloc_obj::<t_sdata>();
    KUNIT_ASSERT_NOT_NULL(test, t_sdata);
    (*resource).data = t_sdata as *mut _;
    (*resource).name = c"sdata".as_ptr();

    (*t_sdata).sdata = kzalloc_obj::<ieee80211_sub_if_data>();
    KUNIT_ASSERT_NOT_NULL(test, (*t_sdata).sdata);
    (*t_sdata).wiphy = kzalloc_obj::<wiphy>();
    KUNIT_ASSERT_NOT_NULL(test, (*t_sdata).wiphy);
    strscpy((*t_sdata).sdata.name.as_mut_ptr(), c"kunit".as_ptr());
    (*t_sdata).sdata.local = &mut (*t_sdata).local;
    (*t_sdata).sdata.local.hw.wiphy = (*t_sdata).wiphy;
    (*t_sdata).sdata.wdev.wiphy = (*t_sdata).wiphy;
    (*t_sdata).sdata.vif.type_ = NL80211_IFTYPE_STATION;
    (*t_sdata).sdata.deflink.sdata = (*t_sdata).sdata;
    (*t_sdata).sdata.deflink.link_id = 0;
    (*t_sdata).wiphy.bands[NL80211_BAND_2GHZ as usize] = &mut (*t_sdata).band_2ghz;
    (*t_sdata).wiphy.bands[NL80211_BAND_5GHZ as usize] = &mut (*t_sdata).band_5ghz;

    let mut band = NL80211_BAND_2GHZ;
    while band <= NL80211_BAND_5GHZ {
        let sband = (*t_sdata).wiphy.bands[band as usize];
        (*sband).band = band;
        (*sband).bitrates = kmemdup(BITRATES.as_ptr() as *const _, core::mem::size_of_val(&BITRATES), GFP_KERNEL);
        (*sband).n_bitrates = BITRATES.len() as _;
        match band {
            NL80211_BAND_2GHZ => {
                (*sband).channels = kmemdup(CHANNELS_2GHZ.as_ptr() as *const _, core::mem::size_of_val(&CHANNELS_2GHZ), GFP_KERNEL);
                (*sband).n_channels = CHANNELS_2GHZ.len() as _;
            },
            NL80211_BAND_5GHZ => {
                (*sband).channels = kmemdup(CHANNELS_5GHZ.as_ptr() as *const _, core::mem::size_of_val(&CHANNELS_5GHZ), GFP_KERNEL);
                (*sband).n_channels = CHANNELS_5GHZ.len() as _;
                (*sband).vht_cap.vht_supported = true;
                (*sband).vht_cap.cap = IEEE80211_VHT_CAP_MAX_MPDU_LENGTH_11454 | IEEE80211_VHT_CAP_SUPP_CHAN_WIDTH_160_80PLUS80MHZ | IEEE80211_VHT_CAP_RXLDPC | IEEE80211_VHT_CAP_SHORT_GI_80 | IEEE80211_VHT_CAP_SHORT_GI_160 | IEEE80211_VHT_CAP_TXSTBC | IEEE80211_VHT_CAP_RXSTBC_4 | IEEE80211_VHT_CAP_MAX_A_MPDU_LENGTH_EXPONENT_MASK;
                (*sband).vht_cap.vht_mcs.rx_mcs_map = cpu_to_le16(IEEE80211_VHT_MCS_SUPPORT_0_9 << 0 | IEEE80211_VHT_MCS_SUPPORT_0_9 << 2 | IEEE80211_VHT_MCS_SUPPORT_0_9 << 4 | IEEE80211_VHT_MCS_SUPPORT_0_9 << 6);
                (*sband).vht_cap.vht_mcs.tx_mcs_map = (*sband).vht_cap.vht_mcs.rx_mcs_map;
            },
            _ => { band += 1; continue; }
        }
        (*sband).ht_cap.ht_supported = band != NL80211_BAND_6GHZ;
        (*sband).ht_cap.cap = IEEE80211_HT_CAP_SUP_WIDTH_20_40 | IEEE80211_HT_CAP_GRN_FLD | IEEE80211_HT_CAP_SGI_20 | IEEE80211_HT_CAP_SGI_40 | IEEE80211_HT_CAP_DSSSCCK40;
        (*sband).ht_cap.ampdu_factor = 0x3;
        (*sband).ht_cap.ampdu_density = 0x6;
        core::ptr::write_bytes(&mut (*sband).ht_cap.mcs as *mut _, 0, 1);
        (*sband).ht_cap.mcs.rx_mask[0] = 0xff;
        (*sband).ht_cap.mcs.rx_mask[1] = 0xff;
        (*sband).ht_cap.mcs.tx_params = IEEE80211_HT_MCS_TX_DEFINED;
        band += 1;
    }
    ieee80211_set_sband_iftype_data(&mut (*t_sdata).band_5ghz, SBAND_CAPA_5GHZ.as_ptr());
    0
}

pub unsafe fn t_sdata_exit(resource: *mut kunit_resource) {
    let t_sdata = (*resource).data as *mut t_sdata;
    kfree((*t_sdata).band_2ghz.channels);
    kfree((*t_sdata).band_2ghz.bitrates);
    kfree((*t_sdata).band_5ghz.channels);
    kfree((*t_sdata).band_5ghz.bitrates);
    kfree((*t_sdata).sdata);
    kfree((*t_sdata).wiphy);
    kfree(t_sdata);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
