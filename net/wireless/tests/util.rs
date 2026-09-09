// SPDX-License-Identifier: GPL-2.0-only
/*
 * KUnit fixture to have a (configurable) wiphy
 *
 * Copyright (C) 2023 Intel Corporation
 */

// C dependencies supplied by the surrounding wireless test code and kernel.

pub unsafe extern "C" fn t_wiphy_init(
    resource: *mut kunit_resource,
    ctx: *mut core::ffi::c_void,
) -> i32 {
    let test = kunit_get_current_test();
    let ops = kzalloc(core::mem::size_of::<cfg80211_ops>());
    KUNIT_ASSERT_NOT_NULL(test, ops);

    let wiphy = wiphy_new_nm(ops, core::mem::size_of::<t_wiphy_priv>(), b"kunit\0".as_ptr() as *const i8);
    KUNIT_ASSERT_NOT_NULL(test, wiphy);

    let priv_data = wiphy_priv(wiphy);
    (*priv_data).ctx = ctx;
    (*priv_data).ops = ops;

    /* Initialize channels, feel free to add more here channels/bands */
    core::ptr::copy_nonoverlapping(
        channels_2ghz.as_ptr(),
        (*priv_data).channels_2ghz.as_mut_ptr(),
        channels_2ghz.len(),
    );
    (*wiphy).bands[NL80211_BAND_2GHZ as usize] = &mut (*priv_data).band_2ghz;
    (*priv_data).band_2ghz.channels = (*priv_data).channels_2ghz.as_mut_ptr();
    (*priv_data).band_2ghz.n_channels = channels_2ghz.len();

    (*resource).data = wiphy as *mut core::ffi::c_void;
    (*resource).name = b"wiphy\0".as_ptr() as *const i8;

    0
}

pub unsafe extern "C" fn t_wiphy_exit(resource: *mut kunit_resource) {
    let priv_data = wiphy_priv((*resource).data as *mut wiphy);
    let ops = (*priv_data).ops;

    /* Should we ensure anything about the state here?
     * e.g. full destruction or no calls to any ops on destruction?
     */

    wiphy_free((*resource).data as *mut wiphy);
    kfree(ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
