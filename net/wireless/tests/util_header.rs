/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Utilities for cfg80211 unit testing
 *
 * Copyright (C) 2023 Intel Corporation
 */

/* C header guard: __CFG80211_UTILS_H */

/* C macro CHAN2G(_freq). */
macro_rules! CHAN2G {
    ($freq:expr) => {
        ieee80211_channel {
            band: NL80211_BAND_2GHZ,
            center_freq: $freq,
            hw_value: $freq,
        }
    };
}

static channels_2ghz: [ieee80211_channel; 14] = [
    CHAN2G!(2412), /* Channel 1 */
    CHAN2G!(2417), /* Channel 2 */
    CHAN2G!(2422), /* Channel 3 */
    CHAN2G!(2427), /* Channel 4 */
    CHAN2G!(2432), /* Channel 5 */
    CHAN2G!(2437), /* Channel 6 */
    CHAN2G!(2442), /* Channel 7 */
    CHAN2G!(2447), /* Channel 8 */
    CHAN2G!(2452), /* Channel 9 */
    CHAN2G!(2457), /* Channel 10 */
    CHAN2G!(2462), /* Channel 11 */
    CHAN2G!(2467), /* Channel 12 */
    CHAN2G!(2472), /* Channel 13 */
    CHAN2G!(2484), /* Channel 14 */
];

#[repr(C)]
struct t_wiphy_priv {
    test: *mut kunit,
    ops: *mut cfg80211_ops,

    ctx: *mut core::ffi::c_void,

    band_2ghz: ieee80211_supported_band,
    channels_2ghz: [ieee80211_channel; channels_2ghz.len()],
}

macro_rules! T_WIPHY {
    ($test:expr, $ctx:expr) => {{
        let __wiphy = kunit_alloc_resource(
            $test,
            t_wiphy_init,
            t_wiphy_exit,
            GFP_KERNEL,
            &mut ($ctx),
        );

        KUNIT_ASSERT_NOT_NULL!($test, __wiphy);
        __wiphy
    }};
}

macro_rules! t_wiphy_ctx {
    ($wiphy:expr) => {
        ((wiphy_priv($wiphy) as *mut t_wiphy_priv).as_ref().unwrap().ctx)
    };
}

extern "C" {
    fn t_wiphy_init(resource: *mut kunit_resource, data: *mut core::ffi::c_void) -> i32;
    fn t_wiphy_exit(resource: *mut kunit_resource);
}

macro_rules! t_skb_remove_member {
    ($skb:expr, $type:ty, $member:ident) => {{
        memmove(
            (*$skb).data.add((*$skb).len - core::mem::size_of::<$type>() + offsetof!($type, $member)),
            (*$skb).data.add((*$skb).len - core::mem::size_of::<$type>() + offsetofend!($type, $member)),
            offsetofend!($type, $member),
        );
        skb_trim($skb, (*$skb).len - sizeof_field!($type, $member));
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
