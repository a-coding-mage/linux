/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Headers for EFI variable service via StandAloneMM, EDK2 application running
 * in OP-TEE. Most of the structs and defines resemble the EDK2 naming.
 *
 * Copyright (c) 2017, Intel Corporation. All rights reserved.
 * Copyright (C) 2020 Linaro Ltd.
 */

/*
 * Interface to the pseudo Trusted Application (TA), which provides a
 * communication channel with the Standalone MM (Management Mode)
 * Secure Partition running at Secure-EL0
 */

pub const PTA_STMM_CMD_COMMUNICATE: i32 = 0;

/*
 * Defined in OP-TEE, this UUID is used to identify the pseudo-TA.
 * OP-TEE is using big endian GUIDs while UEFI uses little endian ones.
 * This is the byte representation of UUID_INIT(0xed32d533, 0x99e6,
 * 0x4209, 0x9c, 0xc0, 0x2d, 0x72, 0xcd, 0xd9, 0x98, 0xa7).
 */
pub const PTA_STMM_UUID: [u8; 16] = [
    0xed, 0x32, 0xd5, 0x33, 0x99, 0xe6, 0x42, 0x09,
    0x9c, 0xc0, 0x2d, 0x72, 0xcd, 0xd9, 0x98, 0xa7,
];

pub const EFI_MM_VARIABLE_GUID: [u8; 16] = PTA_STMM_UUID;

/**
 * Header used for SMM variable communication
 *
 * Defined in the PI spec as EFI_MM_COMMUNICATE_HEADER.
 * To avoid confusion in interpreting frames, the communication buffer should
 * always begin with efi_mm_communicate_header.
 */
#[repr(C, packed)]
pub struct efi_mm_communicate_header {
    pub header_guid: efi_guid_t,
    pub message_len: usize,
    pub data: [u8; 0],
}

pub const MM_COMMUNICATE_HEADER_SIZE: usize = core::mem::size_of::<efi_mm_communicate_header>();

/* SPM return error codes */
pub const ARM_SVC_SPM_RET_SUCCESS: i32 = 0;
pub const ARM_SVC_SPM_RET_NOT_SUPPORTED: i32 = -1;
pub const ARM_SVC_SPM_RET_INVALID_PARAMS: i32 = -2;
pub const ARM_SVC_SPM_RET_DENIED: i32 = -3;
pub const ARM_SVC_SPM_RET_NO_MEMORY: i32 = -5;

pub const SMM_VARIABLE_FUNCTION_GET_VARIABLE: usize = 1;
pub const SMM_VARIABLE_FUNCTION_GET_NEXT_VARIABLE_NAME: usize = 2;
pub const SMM_VARIABLE_FUNCTION_SET_VARIABLE: usize = 3;
pub const SMM_VARIABLE_FUNCTION_QUERY_VARIABLE_INFO: usize = 4;
pub const SMM_VARIABLE_FUNCTION_READY_TO_BOOT: usize = 5;
pub const SMM_VARIABLE_FUNCTION_EXIT_BOOT_SERVICE: usize = 6;
pub const SMM_VARIABLE_FUNCTION_GET_STATISTICS: usize = 7;
pub const SMM_VARIABLE_FUNCTION_LOCK_VARIABLE: usize = 8;
pub const SMM_VARIABLE_FUNCTION_VAR_CHECK_VARIABLE_PROPERTY_SET: usize = 9;
pub const SMM_VARIABLE_FUNCTION_VAR_CHECK_VARIABLE_PROPERTY_GET: usize = 10;
pub const SMM_VARIABLE_FUNCTION_GET_PAYLOAD_SIZE: usize = 11;
pub const SMM_VARIABLE_FUNCTION_INIT_RUNTIME_VARIABLE_CACHE_CONTEXT: usize = 12;
pub const SMM_VARIABLE_FUNCTION_SYNC_RUNTIME_CACHE: usize = 13;
pub const SMM_VARIABLE_FUNCTION_GET_RUNTIME_CACHE_INFO: usize = 14;

/** Used for SMM variable communication. */
#[repr(C)]
pub struct smm_variable_communicate_header {
    pub function: usize,
    pub ret_status: efi_status_t,
    pub data: [u8; 0],
}

pub const MM_VARIABLE_COMMUNICATE_SIZE: usize = core::mem::size_of::<smm_variable_communicate_header>();

/** Used to communicate with StMM by SetVariable and GetVariable. */
#[repr(C)]
pub struct smm_variable_access {
    pub guid: efi_guid_t,
    pub data_size: usize,
    pub name_size: usize,
    pub attr: u32,
    pub name: [u16; 0],
}

pub const MM_VARIABLE_ACCESS_HEADER_SIZE: usize = core::mem::size_of::<smm_variable_access>();

/** Used to get the max allowed payload used in StMM. */
#[repr(C)]
pub struct smm_variable_payload_size {
    pub size: usize,
}

/** Used to communicate with StMM for GetNextVariableName. */
#[repr(C)]
pub struct smm_variable_getnext {
    pub guid: efi_guid_t,
    pub name_size: usize,
    pub name: [u16; 0],
}

pub const MM_VARIABLE_GET_NEXT_HEADER_SIZE: usize = core::mem::size_of::<smm_variable_getnext>();

/** Used to communicate with StMM for QueryVariableInfo. */
#[repr(C)]
pub struct smm_variable_query_info {
    pub max_variable_storage: u64,
    pub remaining_variable_storage: u64,
    pub max_variable_size: u64,
    pub attr: u32,
}

pub const VAR_CHECK_VARIABLE_PROPERTY_REVISION: u16 = 0x0001;
pub const VAR_CHECK_VARIABLE_PROPERTY_READ_ONLY: u32 = 1 << 0;

/** Used to store variable properties in StMM. */
#[repr(C)]
pub struct var_check_property {
    pub revision: u16,
    pub property: u16,
    pub attributes: u32,
    pub minsize: usize,
    pub maxsize: usize,
}

/** Used to communicate variable properties with StMM. */
#[repr(C)]
pub struct smm_variable_var_check_property {
    pub guid: efi_guid_t,
    pub name_size: usize,
    pub property: var_check_property,
    pub name: [u16; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
