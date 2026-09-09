/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by xen/features.h and xen/xen.h remain external.

#[inline]
pub fn xen_swiotlb_detect() -> i32 {
    if !xen_domain() {
        return 0;
    }
    if xen_feature(XENFEAT_direct_mapped) {
        return 1;
    }
    /* legacy case */
    if !xen_feature(XENFEAT_not_direct_mapped) && xen_initial_domain() {
        return 1;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
