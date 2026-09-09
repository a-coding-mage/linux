/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/* Documentation/netlink/specs/binder.yaml */
/* YNL-GEN kernel header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

// C dependencies: <net/netlink.h>, <net/genetlink.h>,
// <uapi/linux/android/binder_netlink.h>

pub const BINDER_NLGRP_REPORT: i32 = 0;

pub unsafe extern "C" {
    pub static mut binder_nl_family: genl_family;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
