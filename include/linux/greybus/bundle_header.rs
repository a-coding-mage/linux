/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Greybus bundles
 *
 * Copyright 2014 Google Inc.
 * Copyright 2014 Linaro Ltd.
 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const BUNDLE_ID_NONE: u8 = u8::MAX;

/* Greybus "public" definitions */
#[repr(C)]
pub struct gb_bundle {
    pub dev: device,
    pub intf: *mut gb_interface,

    pub id: u8,
    pub class: u8,
    pub class_major: u8,
    pub class_minor: u8,

    pub num_cports: usize,
    pub cport_desc: *mut greybus_descriptor_cport,

    pub connections: list_head,
    pub state: *mut u8,

    /* interface->bundles */
    pub links: list_head,
}

/* Equivalent to container_of(d, struct gb_bundle, dev). */
#[inline]
pub unsafe fn to_gb_bundle(d: *mut device) -> *mut gb_bundle {
    crate::container_of!(d, gb_bundle, dev)
}

/* Greybus "private" definitions */
extern "C" {
    pub fn gb_bundle_create(
        intf: *mut gb_interface,
        bundle_id: u8,
        class: u8,
    ) -> *mut gb_bundle;
    pub fn gb_bundle_add(bundle: *mut gb_bundle) -> i32;
    pub fn gb_bundle_destroy(bundle: *mut gb_bundle);
}

/* Bundle Runtime PM wrappers. */
#[cfg(feature = "CONFIG_PM")]
#[inline]
pub unsafe fn gb_pm_runtime_get_sync(bundle: *mut gb_bundle) -> i32 {
    let retval = pm_runtime_get_sync(&mut (*bundle).dev);
    if retval < 0 {
        dev_err(
            &mut (*bundle).dev,
            "pm_runtime_get_sync failed: %d\n",
            retval,
        );
        pm_runtime_put_noidle(&mut (*bundle).dev);
        return retval;
    }
    0
}

#[cfg(feature = "CONFIG_PM")]
#[inline]
pub unsafe fn gb_pm_runtime_put_autosuspend(bundle: *mut gb_bundle) -> i32 {
    pm_runtime_mark_last_busy(&mut (*bundle).dev);
    pm_runtime_put_autosuspend(&mut (*bundle).dev)
}

#[cfg(feature = "CONFIG_PM")]
#[inline]
pub unsafe fn gb_pm_runtime_get_noresume(bundle: *mut gb_bundle) {
    pm_runtime_get_noresume(&mut (*bundle).dev);
}

#[cfg(feature = "CONFIG_PM")]
#[inline]
pub unsafe fn gb_pm_runtime_put_noidle(bundle: *mut gb_bundle) {
    pm_runtime_put_noidle(&mut (*bundle).dev);
}

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub unsafe fn gb_pm_runtime_get_sync(_bundle: *mut gb_bundle) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub unsafe fn gb_pm_runtime_put_autosuspend(_bundle: *mut gb_bundle) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub unsafe fn gb_pm_runtime_get_noresume(_bundle: *mut gb_bundle) {}

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub unsafe fn gb_pm_runtime_put_noidle(_bundle: *mut gb_bundle) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
