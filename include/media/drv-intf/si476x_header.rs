/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/media/drv-intf/si476x.h -- Common definitions for si476x driver
 *
 * Copyright (C) 2012 Innovative Converged Devices(ICD)
 * Copyright (C) 2013 Andrey Smirnov
 *
 * Author: Andrey Smirnov <andrew.smirnov@gmail.com>
 */

// Dependencies supplied by the corresponding Linux headers:
// linux/types.h, linux/videodev2.h, linux/mfd/si476x-reports.h

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum si476x_ctrl_id {
    V4L2_CID_SI476X_RSSI_THRESHOLD = V4L2_CID_USER_SI476X_BASE + 1,
    V4L2_CID_SI476X_SNR_THRESHOLD = V4L2_CID_USER_SI476X_BASE + 2,
    V4L2_CID_SI476X_MAX_TUNE_ERROR = V4L2_CID_USER_SI476X_BASE + 3,
    V4L2_CID_SI476X_HARMONICS_COUNT = V4L2_CID_USER_SI476X_BASE + 4,
    V4L2_CID_SI476X_DIVERSITY_MODE = V4L2_CID_USER_SI476X_BASE + 5,
    V4L2_CID_SI476X_INTERCHIP_LINK = V4L2_CID_USER_SI476X_BASE + 6,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
