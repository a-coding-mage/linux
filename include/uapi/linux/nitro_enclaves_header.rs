/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright 2020-2021 Amazon.com, Inc. or its affiliates. All Rights Reserved.
 */

//! DOC: Nitro Enclaves (NE) Kernel Driver Interface

// The `_IOR`, `_IOWR`, and `_IOW` ioctl encoding macros and the referenced
// structure types are supplied by the surrounding UAPI translation.

/// NE_CREATE_VM - Create a slot associated with an enclave VM.
pub const NE_CREATE_VM: _ = _IOR(0xAE, 0x20, u64);

/// NE_ADD_VCPU - Set a vCPU for an enclave.
pub const NE_ADD_VCPU: _ = _IOWR(0xAE, 0x21, u32);

/// NE_GET_IMAGE_LOAD_INFO - Get information needed for in-memory image loading.
pub const NE_GET_IMAGE_LOAD_INFO: _ = _IOWR(0xAE, 0x22, ne_image_load_info);

/// NE_SET_USER_MEMORY_REGION - Set a userspace-backed memory region.
pub const NE_SET_USER_MEMORY_REGION: _ = _IOW(0xAE, 0x23, ne_user_memory_region);

/// NE_START_ENCLAVE - Trigger enclave start after resources have been set.
pub const NE_START_ENCLAVE: _ = _IOWR(0xAE, 0x24, ne_enclave_start_info);

/// DOC: NE specific error codes
pub const NE_ERR_VCPU_ALREADY_USED: i32 = 256;
pub const NE_ERR_VCPU_NOT_IN_CPU_POOL: i32 = 257;
pub const NE_ERR_VCPU_INVALID_CPU_CORE: i32 = 258;
pub const NE_ERR_INVALID_MEM_REGION_SIZE: i32 = 259;
pub const NE_ERR_INVALID_MEM_REGION_ADDR: i32 = 260;
pub const NE_ERR_UNALIGNED_MEM_REGION_ADDR: i32 = 261;
pub const NE_ERR_MEM_REGION_ALREADY_USED: i32 = 262;
pub const NE_ERR_MEM_NOT_HUGE_PAGE: i32 = 263;
pub const NE_ERR_MEM_DIFFERENT_NUMA_NODE: i32 = 264;
pub const NE_ERR_MEM_MAX_REGIONS: i32 = 265;
pub const NE_ERR_NO_MEM_REGIONS_ADDED: i32 = 266;
pub const NE_ERR_NO_VCPUS_ADDED: i32 = 267;
pub const NE_ERR_ENCLAVE_MEM_MIN_SIZE: i32 = 268;
pub const NE_ERR_FULL_CORES_NOT_USED: i32 = 269;
pub const NE_ERR_NOT_IN_INIT_STATE: i32 = 270;
pub const NE_ERR_INVALID_VCPU: i32 = 271;
pub const NE_ERR_NO_CPUS_AVAIL_IN_POOL: i32 = 272;
pub const NE_ERR_INVALID_PAGE_SIZE: i32 = 273;
pub const NE_ERR_INVALID_FLAG_VALUE: i32 = 274;
pub const NE_ERR_INVALID_ENCLAVE_CID: i32 = 275;

/// DOC: Image load info flags
pub const NE_EIF_IMAGE: u64 = 0x01;
pub const NE_IMAGE_LOAD_MAX_FLAG_VAL: u64 = 0x02;

/// struct ne_image_load_info - Info necessary for in-memory enclave image loading.
#[repr(C)]
#[derive Copy, Clone]
pub struct ne_image_load_info {
    pub flags: u64,
    pub memory_offset: u64,
}

/// DOC: User memory region flags
pub const NE_DEFAULT_MEMORY_REGION: u64 = 0x00;
pub const NE_MEMORY_REGION_MAX_FLAG_VAL: u64 = 0x01;

/// struct ne_user_memory_region - Memory region to be set for an enclave.
#[repr(C)]
#[derive Copy, Clone]
pub struct ne_user_memory_region {
    pub flags: u64,
    pub memory_size: u64,
    pub userspace_addr: u64,
}

/// DOC: Enclave start info flags
pub const NE_ENCLAVE_PRODUCTION_MODE: u64 = 0x00;
pub const NE_ENCLAVE_DEBUG_MODE: u64 = 0x01;
pub const NE_ENCLAVE_START_MAX_FLAG_VAL: u64 = 0x02;

/// struct ne_enclave_start_info - Setup info necessary for enclave start.
#[repr(C)]
#[derive Copy, Clone]
pub struct ne_enclave_start_info {
    pub flags: u64,
    pub enclave_cid: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
