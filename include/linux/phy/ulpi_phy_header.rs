/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency supplied by the Linux PHY subsystem. */

/**
 * Helper that registers PHY for a ULPI device and adds a lookup for binding it
 * and it's controller, which is always the parent.
 */
#[inline]
pub unsafe fn ulpi_phy_create(
    ulpi: *mut crate::ulpi,
    ops: *const crate::phy_ops,
) -> *mut crate::phy {
    let phy: *mut crate::phy;
    let ret: core::ffi::c_int;

    phy = crate::phy_create(
        &mut (*ulpi).dev,
        core::ptr::null_mut(),
        ops,
    );
    if crate::IS_ERR(phy) {
        return phy;
    }

    ret = crate::phy_create_lookup(
        phy,
        b"usb2-phy\0".as_ptr() as *const core::ffi::c_char,
        crate::dev_name((*(*ulpi).dev.parent)),
    );
    if ret != 0 {
        crate::phy_destroy(phy);
        return crate::ERR_PTR(ret);
    }

    phy
}

/* Remove a PHY that was created with ulpi_phy_create() and it's lookup. */
#[inline]
pub unsafe fn ulpi_phy_destroy(ulpi: *mut crate::ulpi, phy: *mut crate::phy) {
    crate::phy_remove_lookup(
        phy,
        b"usb2-phy\0".as_ptr() as *const core::ffi::c_char,
        crate::dev_name((*(*ulpi).dev.parent)),
    );
    crate::phy_destroy(phy);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
