/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/binder.yaml */
/* YNL-GEN uapi header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

// C header guard: _UAPI_LINUX_ANDROID_BINDER_NETLINK_H

pub const BINDER_FAMILY_NAME: &str = "binder";
pub const BINDER_FAMILY_VERSION: i32 = 1;

pub const BINDER_A_REPORT_ERROR: i32 = 1;
pub const BINDER_A_REPORT_CONTEXT: i32 = 2;
pub const BINDER_A_REPORT_FROM_PID: i32 = 3;
pub const BINDER_A_REPORT_FROM_TID: i32 = 4;
pub const BINDER_A_REPORT_TO_PID: i32 = 5;
pub const BINDER_A_REPORT_TO_TID: i32 = 6;
pub const BINDER_A_REPORT_IS_REPLY: i32 = 7;
pub const BINDER_A_REPORT_FLAGS: i32 = 8;
pub const BINDER_A_REPORT_CODE: i32 = 9;
pub const BINDER_A_REPORT_DATA_SIZE: i32 = 10;

pub const __BINDER_A_REPORT_MAX: i32 = 11;
pub const BINDER_A_REPORT_MAX: i32 = __BINDER_A_REPORT_MAX - 1;

pub const BINDER_CMD_REPORT: i32 = 1;

pub const __BINDER_CMD_MAX: i32 = 2;
pub const BINDER_CMD_MAX: i32 = __BINDER_CMD_MAX - 1;

pub const BINDER_MCGRP_REPORT: &str = "report";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
