/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (c) 2025, Google LLC.
 * Pasha Tatashin <pasha.tatashin@soleen.com>
 */

/**
 * DOC: Live Update Orchestrator ABI
 *
 * Live Update Orchestrator uses the stable Application Binary Interface
 * defined below to pass state from a pre-update kernel to a post-update
 * kernel. The ABI is built upon the Kexec HandOver framework and registers
 * the central `struct luo_ser` via the KHO raw subtree API.
 *
 * This interface is a contract. Any modification to the structure fields,
 * compatible strings, or the layout of the `__packed` serialization
 * structures defined here constitutes a breaking change. Such changes require
 * incrementing the version number in the relevant `_COMPATIBLE` string to
 * prevent a new kernel from misinterpreting data from an old kernel.
 *
 * Changes are allowed provided the compatibility version is incremented;
 * however, backward/forward compatibility is only guaranteed for kernels
 * supporting the same ABI version.
 */

/* Dependencies supplied by the kernel ABI and liveupdate headers. */

/* The LUO state is registered under this KHO entry name. */
pub const LUO_KHO_ENTRY_NAME: &str = "LUO";
pub const LUO_ABI_COMPATIBLE: &str = "luo-v5";
pub const LUO_ABI_COMPAT_LEN: usize = 8; // ALIGN(sizeof("luo-v5"), 8)

/**
 * struct luo_ser - Centralized LUO ABI header.
 * @compatible:     Compatibility string identifying the LUO ABI version.
 * @liveupdate_num: A counter tracking the number of successful live updates.
 * @sessions_pa:    Physical address of the first session block header.
 * @flbs_pa:        Physical address of the FLB header.
 *
 * This structure is the root of all preserved LUO state.
 */
#[repr(C, packed)]
pub struct luo_ser {
    pub compatible: [i8; LUO_ABI_COMPAT_LEN],
    pub liveupdate_num: u64,
    pub sessions_pa: u64,
    pub flbs_pa: u64,
}

pub const LIVEUPDATE_HNDL_COMPAT_LENGTH: usize = 48;

/**
 * struct luo_file_ser - Represents the serialized preserves files.
 * @compatible:  File handler compatible string.
 * @data:        Private data
 * @token:       User provided token for this file
 *
 * If this structure is modified, `LUO_ABI_COMPATIBLE` must be updated.
 */
#[repr(C, packed)]
pub struct luo_file_ser {
    pub compatible: [i8; LIVEUPDATE_HNDL_COMPAT_LENGTH],
    pub data: u64,
    pub token: u64,
}

/**
 * struct luo_file_set_ser - Represents the serialized metadata for file set
 * @files:   The physical address of the first `struct kho_block_header_ser`.
 * @count:   The total number of files that were part of this session during
 *           serialization. Used for iteration and validation during
 *           restoration.
 */
#[repr(C, packed)]
pub struct luo_file_set_ser {
    pub files: u64,
    pub count: u64,
}

/**
 * struct luo_session_ser - Represents the serialized metadata for a LUO session.
 * @name:         The unique name of the session, provided by the userspace at
 *                the time of session creation.
 * @file_set_ser: Serialized files belonging to this session,
 *
 * This structure is used to package session-specific metadata for transfer
 * between kernels via Kexec Handover.
 *
 * If this structure is modified, `LUO_ABI_COMPATIBLE` must be updated.
 */
#[repr(C, packed)]
pub struct luo_session_ser {
    pub name: [i8; LIVEUPDATE_SESSION_NAME_LENGTH],
    pub file_set_ser: luo_file_set_ser,
}

/* The max size is set so it can be reliably used during in serialization. */
pub const LIVEUPDATE_FLB_COMPAT_LENGTH: usize = 48;

/**
 * struct luo_flb_header_ser - Header for the serialized FLB data block.
 * @pgcnt: The total number of pages occupied by the entire preserved memory
 *         region, including this header and the subsequent array of
 *         &struct luo_flb_ser entries.
 * @count: The number of &struct luo_flb_ser entries that follow this header.
 */
#[repr(C, packed)]
pub struct luo_flb_header_ser {
    pub pgcnt: u64,
    pub count: u64,
}

/**
 * struct luo_flb_ser - Represents the serialized state of a single FLB object.
 * @name:    The unique compatibility string of the FLB object.
 * @data:    The opaque u64 handle returned by the FLB's .preserve() operation.
 * @count:   The reference count at the time of serialization.
 *
 * If this structure is modified, `LUO_ABI_COMPATIBLE` must be updated.
 */
#[repr(C, packed)]
pub struct luo_flb_ser {
    pub name: [i8; LIVEUPDATE_FLB_COMPAT_LENGTH],
    pub data: u64,
    pub count: u64,
}

/* Kernel Live Update Test ABI; enabled in C under CONFIG_LIVEUPDATE_TEST. */
#[macro_export]
macro_rules! LIVEUPDATE_TEST_FLB_COMPATIBLE {
    ($i:literal) => {
        concat!("liveupdate-test-flb-v", stringify!($i))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
