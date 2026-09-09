// SPDX-License-Identifier: GPL-2.0-only
/*
 * Confidential Computing Platform Capability checks
 *
 * Copyright (C) 2021 Advanced Micro Devices, Inc.
 *
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 */

// External declarations provided by the corresponding Linux headers.
extern "C" {
    fn is_secure_guest() -> bool;
}

/// Confidential computing platform capability checks.
pub unsafe fn cc_platform_has(attr: cc_attr) -> bool {
    match attr {
        CC_ATTR_MEM_ENCRYPT | CC_ATTR_GUEST_MEM_ENCRYPT => is_secure_guest(),
        _ => false,
    }
}

// EXPORT_SYMBOL_GPL(cc_platform_has);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
