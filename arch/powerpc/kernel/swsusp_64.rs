// SPDX-License-Identifier: GPL-2.0-only
/*
 * PowerPC 64-bit swsusp implementation
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 */

// Dependencies supplied by the corresponding kernel headers.
unsafe extern "C" {
    fn iommu_restore();
    fn touch_softlockup_watchdog();
    fn mb();
}

pub unsafe fn do_after_copyback() {
    iommu_restore();
    touch_softlockup_watchdog();
    mb();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
