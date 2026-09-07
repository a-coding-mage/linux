// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor ipc mediation function definitions.
//
// Copyright 2023 Canonical Ltd.

pub const SIGUNKNOWN: i32 = 0;
pub const MAXMAPPED_SIG: i32 = 35;

pub const MAXMAPPED_SIGNAME: i32 = MAXMAPPED_SIG + 1;
pub const SIGRT_BASE: i32 = 128;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
