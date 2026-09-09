/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * hvcserver.h
 * Copyright (C) 2004 Ryan S Arnold, IBM Corporation
 *
 * PPC64 virtual I/O console server support.
 */

/* Header guard: _PPC64_HVCSERVER_H */
/* This header's declarations are available only when __KERNEL__ is defined. */

/* Dependency: linux/list.h */

/* Converged Location Code length */
pub const HVCS_CLC_LENGTH: usize = 79;

/**
 * hvcs_partner_info - an element in a list of partner info
 * @node: list_head denoting this partner_info struct's position in the list of
 * partner info.
 * @unit_address: The partner unit address of this entry.
 * @partition_ID: The partner partition ID of this entry.
 * @location_code: The converged location code of this entry + 1 char for the
 * null-term.
 *
 * This structure outlines the format that partner info is presented to a caller
 * of the hvcs partner info fetching functions.  These are strung together into
 * a list using linux kernel lists.
 */
#[repr(C)]
pub struct hvcs_partner_info {
    pub node: crate::list_head,
    pub unit_address: u32,
    pub partition_ID: u32,
    pub location_code: [i8; HVCS_CLC_LENGTH + 1], /* CLC + 1 null-term char */
}

unsafe extern "C" {
    pub fn hvcs_free_partner_info(head: *mut crate::list_head) -> ::core::ffi::c_int;
    pub fn hvcs_get_partner_info(
        unit_address: u32,
        head: *mut crate::list_head,
        pi_buff: *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn hvcs_register_connection(
        unit_address: u32,
        p_partition_ID: u32,
        p_unit_address: u32,
    ) -> ::core::ffi::c_int;
    pub fn hvcs_free_connection(unit_address: u32) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
