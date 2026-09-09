/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Confidential Computing Platform Capability checks
 *
 * Copyright (C) 2021 Advanced Micro Devices, Inc.
 *
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 */

/**
 * enum cc_attr - Confidential computing attributes
 *
 * These attributes represent confidential computing features that are
 * currently active.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cc_attr {
    /* Memory encryption is active. */
    CC_ATTR_MEM_ENCRYPT,

    /* Host memory encryption is active. */
    CC_ATTR_HOST_MEM_ENCRYPT,

    /* Guest memory encryption is active. */
    CC_ATTR_GUEST_MEM_ENCRYPT,

    /* Guest state encryption is active. */
    CC_ATTR_GUEST_STATE_ENCRYPT,

    /* Guest string I/O is implemented with IN/OUT instructions. */
    CC_ATTR_GUEST_UNROLL_STRING_IO,

    /* Guest SNP is active. */
    CC_ATTR_GUEST_SEV_SNP,

    /* SNP Secure TSC is active. */
    CC_ATTR_GUEST_SNP_SECURE_TSC,

    /* AMD SNP enabled on the host. */
    CC_ATTR_HOST_SEV_SNP,

    /* Secure AVIC mode is active. */
    CC_ATTR_SNP_SECURE_AVIC,
}

/* CONFIG_ARCH_HAS_CC_PLATFORM is a build-time configuration condition. */
#[cfg(feature = "CONFIG_ARCH_HAS_CC_PLATFORM")]
extern "C" {
    /**
     * Checks if the specified confidential computing attribute is active.
     *
     * Returns true when the specified attribute is active and false
     * otherwise. This function may be called from any context.
     */
    pub fn cc_platform_has(attr: cc_attr) -> bool;
    pub fn cc_platform_set(attr: cc_attr);
    pub fn cc_platform_clear(attr: cc_attr);
}

/* !CONFIG_ARCH_HAS_CC_PLATFORM */
#[cfg(not(feature = "CONFIG_ARCH_HAS_CC_PLATFORM"))]
#[inline]
pub fn cc_platform_has(_attr: cc_attr) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_ARCH_HAS_CC_PLATFORM"))]
#[inline]
pub fn cc_platform_set(_attr: cc_attr) {}

#[cfg(not(feature = "CONFIG_ARCH_HAS_CC_PLATFORM"))]
#[inline]
pub fn cc_platform_clear(_attr: cc_attr) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
