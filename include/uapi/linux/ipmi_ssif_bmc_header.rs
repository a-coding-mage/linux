/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note*/
/*
 * Copyright (c) 2022, Ampere Computing LLC.
 */

// Translated from the Linux UAPI header <linux/ipmi_ssif_bmc.h>.

/// Max length of ipmi ssif message included netfn and cmd field.
pub const IPMI_SSIF_PAYLOAD_MAX: usize = 254;

#[repr(C)]
pub struct ipmi_ssif_msg {
    pub len: u32,
    pub payload: [u8; IPMI_SSIF_PAYLOAD_MAX],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
