/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Linux Security Modules (LSM) - User space API
 *
 * Copyright (C) 2022 Casey Schaufler <casey@schaufler-ca.com>
 * Copyright (C) 2022 Intel Corporation
 */

// Dependencies supplied by the surrounding UAPI environment:
// <linux/stddef.h>, <linux/types.h>, and <linux/unistd.h>

/**
 * struct lsm_ctx - LSM context information
 * @id: the LSM id number, see LSM_ID_XXX
 * @flags: LSM specific flags
 * @len: length of the lsm_ctx struct, @ctx and any other data or padding
 * @ctx_len: the size of @ctx
 * @ctx: the LSM context value
 *
 * The @len field MUST be equal to the size of the lsm_ctx struct
 * plus any additional padding and/or data placed after @ctx.
 *
 * In all cases @ctx_len MUST be equal to the length of @ctx.
 * If @ctx is a string value it should be nul terminated with
 * @ctx_len equal to `strlen(@ctx) + 1`.  Binary values are
 * supported.
 *
 * The @flags and @ctx fields SHOULD only be interpreted by the
 * LSM specified by @id; they MUST be set to zero/0 when not used.
 */
#[repr(C)]
pub struct lsm_ctx {
    pub id: u64,
    pub flags: u64,
    pub len: u64,
    pub ctx_len: u64,
    // C flexible array member: storage follows this header and is counted by ctx_len.
    pub ctx: [u8; 0],
}

/*
 * ID tokens to identify Linux Security Modules (LSMs)
 *
 * These token values are used to uniquely identify specific LSMs
 * in the kernel as well as in the kernel's LSM userspace API.
 *
 * A value of zero/0 is considered undefined and should not be used
 * outside the kernel. Values 1-99 are reserved for potential
 * future use.
 */
pub const LSM_ID_UNDEF: u64 = 0;
pub const LSM_ID_CAPABILITY: u64 = 100;
pub const LSM_ID_SELINUX: u64 = 101;
pub const LSM_ID_SMACK: u64 = 102;
pub const LSM_ID_TOMOYO: u64 = 103;
pub const LSM_ID_APPARMOR: u64 = 104;
pub const LSM_ID_YAMA: u64 = 105;
pub const LSM_ID_LOADPIN: u64 = 106;
pub const LSM_ID_SAFESETID: u64 = 107;
pub const LSM_ID_LOCKDOWN: u64 = 108;
pub const LSM_ID_BPF: u64 = 109;
pub const LSM_ID_LANDLOCK: u64 = 110;
pub const LSM_ID_IMA: u64 = 111;
pub const LSM_ID_EVM: u64 = 112;
pub const LSM_ID_IPE: u64 = 113;

/*
 * LSM_ATTR_XXX definitions identify different LSM attributes
 * which are used in the kernel's LSM userspace API. Support
 * for these attributes vary across the different LSMs. None
 * are required.
 *
 * A value of zero/0 is considered undefined and should not be used
 * outside the kernel. Values 1-99 are reserved for potential
 * future use.
 */
pub const LSM_ATTR_UNDEF: u64 = 0;
pub const LSM_ATTR_CURRENT: u64 = 100;
pub const LSM_ATTR_EXEC: u64 = 101;
pub const LSM_ATTR_FSCREATE: u64 = 102;
pub const LSM_ATTR_KEYCREATE: u64 = 103;
pub const LSM_ATTR_PREV: u64 = 104;
pub const LSM_ATTR_SOCKCREATE: u64 = 105;

/*
 * LSM_FLAG_XXX definitions identify special handling instructions
 * for the API.
 */
pub const LSM_FLAG_SINGLE: u64 = 0x0001;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
