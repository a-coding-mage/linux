/* SPDX-License-Identifier: GPL-2.0
 *
 * Header file for the CDX Controller
 *
 * Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
 */

// Dependency declarations supplied by the surrounding translation unit:
// #include <linux/cdx/cdx_bus.h>
// #include "mcdi_functions.h"

extern "C" {
    pub fn cdx_rpmsg_post_probe(cdx: *mut cdx_controller);

    pub fn cdx_rpmsg_pre_remove(cdx: *mut cdx_controller);

    pub fn cdx_rpmsg_send(
        cdx_mcdi: *mut cdx_mcdi,
        hdr: *const cdx_dword,
        hdr_len: usize,
        sdu: *const cdx_dword,
        sdu_len: usize,
    ) -> ::std::os::raw::c_int;

    pub fn cdx_rpmsg_read_resp(
        cdx_mcdi: *mut cdx_mcdi,
        outbuf: *mut cdx_dword,
        offset: usize,
        outlen: usize,
    );

    pub fn cdx_setup_rpmsg(pdev: *mut platform_device) -> ::std::os::raw::c_int;

    pub fn cdx_destroy_rpmsg(pdev: *mut platform_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
