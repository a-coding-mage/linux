/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Ultravisor API.
 *
 * Copyright 2019, IBM Corporation.
 *
 */

// Dependency: declarations from <asm/hvcall.h> are supplied externally.

/* Return codes */
pub const U_BUSY: i64 = H_BUSY;
pub const U_FUNCTION: i64 = H_FUNCTION;
pub const U_NOT_AVAILABLE: i64 = H_NOT_AVAILABLE;
pub const U_P2: i64 = H_P2;
pub const U_P3: i64 = H_P3;
pub const U_P4: i64 = H_P4;
pub const U_P5: i64 = H_P5;
pub const U_PARAMETER: i64 = H_PARAMETER;
pub const U_PERMISSION: i64 = H_PERMISSION;
pub const U_SUCCESS: i64 = H_SUCCESS;

/* opcodes */
pub const UV_WRITE_PATE: u64 = 0xF104;
pub const UV_RETURN: u64 = 0xF11C;
pub const UV_ESM: u64 = 0xF110;
pub const UV_REGISTER_MEM_SLOT: u64 = 0xF120;
pub const UV_UNREGISTER_MEM_SLOT: u64 = 0xF124;
pub const UV_PAGE_IN: u64 = 0xF128;
pub const UV_PAGE_OUT: u64 = 0xF12C;
pub const UV_SHARE_PAGE: u64 = 0xF130;
pub const UV_UNSHARE_PAGE: u64 = 0xF134;
pub const UV_UNSHARE_ALL_PAGES: u64 = 0xF140;
pub const UV_PAGE_INVAL: u64 = 0xF138;
pub const UV_SVM_TERMINATE: u64 = 0xF13C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
