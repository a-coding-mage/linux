// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) ST-Ericsson SA 2012
 *
 * Author: Ola Lilja <ola.o.lilja@stericsson.com>,
 *         Roger Nilsson <roger.xr.nilsson@stericsson.com>
 *         for ST-Ericsson.
 */

// C dependencies: <asm/page.h>, <linux/workqueue.h>

unsafe extern "C" {
    pub fn ux500_pcm_register_platform(pdev: *mut platform_device) -> ::std::os::raw::c_int;
    pub fn ux500_pcm_unregister_platform(pdev: *mut platform_device) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
