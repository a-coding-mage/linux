// SPDX-License-Identifier: GPL-2.0
/*
 * ARM MHUv2 Mailbox Message
 *
 * Copyright (C) 2020 Arm Ltd.
 * Copyright (C) 2020 Linaro Ltd.
 */

use core::ffi::c_void;

/* Data structure for data-transfer protocol */
#[repr(C)]
pub struct arm_mhuv2_mbox_msg {
    pub data: *mut c_void,
    pub len: usize,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
