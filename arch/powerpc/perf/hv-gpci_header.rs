/* SPDX-License-Identifier: GPL-2.0 */

/*
 * counter info version => fw version/reference (spec version)
 *
 * 8 => power8 (1.07)
 * [7 is skipped by spec 1.07]
 * 6 => TLBIE (1.07)
 * 5 => v7r7m0.phyp (1.05)
 * [4 skipped]
 * 3 => v7r6m0.phyp (?)
 * [1,2 skipped]
 * 0 => v7r{2,3,4}m0.phyp (?)
 */
pub const COUNTER_INFO_VERSION_CURRENT: u32 = 0x8;

/* capability mask masks. */
pub const HV_GPCI_CM_GA: u32 = 1u32 << 7;
pub const HV_GPCI_CM_EXPANDED: u32 = 1u32 << 6;
pub const HV_GPCI_CM_LAB: u32 = 1u32 << 5;

/*
 * The C header includes req-gen/perf.h with these preprocessor settings:
 * REQUEST_FILE "../hv-gpci-requests.h"
 * NAME_LOWER hv_gpci
 * NAME_UPPER HV_GPCI
 * ENABLE_EVENTS_COUNTERINFO_V6
 *
 * The generated declarations are supplied by the surrounding translation
 * unit and are intentionally not implemented here.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
