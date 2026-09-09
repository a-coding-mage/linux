/*
 * Broadcom specific AMBA
 * GBIT MAC COMMON Core
 *
 * Licensed under the GNU/GPL. See COPYING for details.
 */

// Types and functions supplied by the BCMA and Linux dependencies are
// intentionally referenced here rather than redefined.

pub unsafe fn bcma_core_gmac_cmn_init(gc: *mut bcma_drv_gmac_cmn) {
    mutex_init(core::ptr::addr_of_mut!((*gc).phy_mutex));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
