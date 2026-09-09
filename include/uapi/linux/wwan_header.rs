/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * Copyright (C) 2021 Intel Corporation.
 */

// Header guard: _UAPI_WWAN_H_

pub const IFLA_WWAN_UNSPEC: i32 = 0;
pub const IFLA_WWAN_LINK_ID: i32 = 1; /* u32 */

pub const __IFLA_WWAN_MAX: i32 = 2;
pub const IFLA_WWAN_MAX: i32 = __IFLA_WWAN_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
