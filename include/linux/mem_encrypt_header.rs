/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * AMD Memory Encryption Support
 *
 * Copyright (C) 2016 Advanced Micro Devices, Inc.
 *
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 */

// C header guard: __MEM_ENCRYPT_H__
// C assembly guard: declarations below are for non-assembly builds.

// When CONFIG_ARCH_HAS_MEM_ENCRYPT is enabled, the C header includes
// <asm/mem_encrypt.h>; its declarations are supplied by the corresponding
// Rust dependency.

/*
 * The __sme_set() and __sme_clr() macros are useful for adding or removing
 * the encryption mask from a value (e.g. when dealing with pagetable
 * entries).
 */

// CONFIG_AMD_MEM_ENCRYPT selects the masked forms below.  In a C build where
// it is not selected, these macros are the identity operations.
#[cfg(feature = "CONFIG_AMD_MEM_ENCRYPT")]
#[macro_export]
macro_rules! __sme_set {
    ($x:expr) => {
        (($x) | sme_me_mask)
    };
}

#[cfg(not(feature = "CONFIG_AMD_MEM_ENCRYPT"))]
#[macro_export]
macro_rules! __sme_set {
    ($x:expr) => {
        ($x)
    };
}

#[cfg(feature = "CONFIG_AMD_MEM_ENCRYPT")]
#[macro_export]
macro_rules! __sme_clr {
    ($x:expr) => {
        (($x) & !sme_me_mask)
    };
}

#[cfg(not(feature = "CONFIG_AMD_MEM_ENCRYPT"))]
#[macro_export]
macro_rules! __sme_clr {
    ($x:expr) => {
        ($x)
    };
}

#[cfg(feature = "CONFIG_AMD_MEM_ENCRYPT")]
#[macro_export]
macro_rules! dma_addr_encrypted {
    ($x:expr) => {
        $crate::__sme_set!($x)
    };
}

#[cfg(not(feature = "CONFIG_AMD_MEM_ENCRYPT"))]
#[macro_export]
macro_rules! dma_addr_encrypted {
    ($x:expr) => {
        ($x)
    };
}

#[macro_export]
macro_rules! dma_addr_unencrypted {
    ($x:expr) => {
        ($x)
    };
}

#[cfg(feature = "CONFIG_AMD_MEM_ENCRYPT")]
#[macro_export]
macro_rules! dma_addr_canonical {
    ($x:expr) => {
        $crate::__sme_clr!($x)
    };
}

#[cfg(not(feature = "CONFIG_AMD_MEM_ENCRYPT"))]
#[macro_export]
macro_rules! dma_addr_canonical {
    ($x:expr) => {
        ($x)
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
