/* SPDX-License-Identifier: GPL-2.0-only */

/**
 * DOC: Kexec Metadata ABI
 *
 * The "kexec-metadata" subtree stores optional metadata about the kexec chain.
 * It is registered via kho_add_subtree(), keeping it independent from the core
 * KHO ABI. This allows the metadata format to evolve without affecting other
 * KHO consumers.
 *
 * The metadata is stored as a plain C struct rather than FDT format for
 * simplicity and direct field access.
 *
 * Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2026 Breno Leitao <leitao@debian.org>
 */

// Dependency intent: `u32` corresponds to the kernel's `u32`; `__NEW_UTS_LEN`
// is supplied by the kernel UTS ABI.

pub const KHO_KEXEC_METADATA_VERSION: u32 = 1;

/**
 * struct kho_kexec_metadata - Kexec metadata passed between kernels
 * @version: ABI version of this struct (must be first field)
 * @previous_release: Kernel version string that initiated the kexec
 * @kexec_count: Number of kexec boots since last cold boot
 *
 * This structure is preserved across kexec and allows the new kernel to
 * identify which kernel it was booted from and how many kexec reboots
 * have occurred.
 *
 * __NEW_UTS_LEN is part of uABI, so it safe to use it in here.
 */
#[repr(C, packed)]
pub struct kho_kexec_metadata {
    pub version: u32,
    pub previous_release: [core::ffi::c_char; __NEW_UTS_LEN + 1],
    pub kexec_count: u32,
}

pub const KHO_METADATA_NODE_NAME: &str = "kexec-metadata";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
