/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: acconfig.h - Global configuration constants
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/* Configuration options. */

/*
 * ACPI_DEBUG_OUTPUT enables all debug facilities of the ACPI subsystem.
 * ACPI_APPLICATION selects application-level use of the subsystem.
 */

/*
 * OS name, used for the _OS object. The _OS object is essentially obsolete,
 * but existing ASL/AML code may depend on this exact string.
 */
pub const ACPI_OS_NAME: &str = "Microsoft Windows NT";

/* Maximum objects in the various object caches */
pub const ACPI_MAX_STATE_CACHE_DEPTH: i32 = 96; /* State objects */
pub const ACPI_MAX_PARSE_CACHE_DEPTH: i32 = 96; /* Parse tree objects */
pub const ACPI_MAX_EXTPARSE_CACHE_DEPTH: i32 = 96; /* Parse tree objects */
pub const ACPI_MAX_OBJECT_CACHE_DEPTH: i32 = 96; /* Interpreter operand objects */
pub const ACPI_MAX_NAMESPACE_CACHE_DEPTH: i32 = 96; /* Namespace objects */
pub const ACPI_MAX_COMMENT_CACHE_DEPTH: i32 = 96; /* Comments for the -ca option */

/* Should the subsystem abort loading an ACPI table with an incorrect checksum? */
pub const ACPI_CHECKSUM_ABORT: bool = false;

/* Reduced-hardware platform support configuration. */
pub const ACPI_REDUCED_HARDWARE: bool = false;

/* Version of ACPI supported */
pub const ACPI_CA_SUPPORT_LEVEL: i32 = 5;

/* Maximum count for a semaphore object */
pub const ACPI_MAX_SEMAPHORE_COUNT: i32 = 256;

/* Maximum object reference count (detects object deletion issues) */
pub const ACPI_MAX_REFERENCE_COUNT: i32 = 0x4000;

/* Default page size for use in mapping memory for operation regions */
pub const ACPI_DEFAULT_PAGE_SIZE: i32 = 4096; /* Must be power of 2 */

/* owner_id tracking. 128 entries allows for 4095 owner_ids */
pub const ACPI_NUM_OWNERID_MASKS: i32 = 128;

/* Size of the root table array is increased by this increment */
pub const ACPI_ROOT_TABLE_SIZE_INCREMENT: i32 = 4;

/* Maximum sleep allowed via Sleep() operator */
pub const ACPI_MAX_SLEEP: i32 = 2000; /* 2000 millisec == two seconds */

/* Address Range lists are per-space_id (Memory and I/O only) */
pub const ACPI_ADDRESS_RANGE_MAX: i32 = 2;

/* Maximum time (default 30s) of While() loops before abort */
pub const ACPI_MAX_LOOP_TIMEOUT: i32 = 30;

/* ACPI Specification constants */
pub const ACPI_METHOD_NUM_LOCALS: i32 = 8;
pub const ACPI_METHOD_MAX_LOCAL: i32 = 7;
pub const ACPI_METHOD_NUM_ARGS: i32 = 7;
pub const ACPI_METHOD_MAX_ARG: i32 = 6;
pub const ACPI_OBJ_NUM_OPERANDS: i32 = 8;
pub const ACPI_OBJ_MAX_OPERAND: i32 = 7;
pub const ACPI_RESULTS_FRAME_OBJ_NUM: i32 = 8;
pub const ACPI_RESULTS_OBJ_NUM_MAX: i32 = 255;

/* Constants used in searching for the RSDP in low memory */
pub const ACPI_EBDA_PTR_LOCATION: u32 = 0x0000040E; /* Physical Address */
pub const ACPI_EBDA_PTR_LENGTH: i32 = 2;
pub const ACPI_EBDA_WINDOW_SIZE: i32 = 1024;
pub const ACPI_HI_RSDP_WINDOW_BASE: u32 = 0x000E0000; /* Physical Address */
pub const ACPI_HI_RSDP_WINDOW_SIZE: u32 = 0x00020000;
pub const ACPI_RSDP_SCAN_STEP: i32 = 16;

/* Operation regions */
pub const ACPI_USER_REGION_BEGIN: i32 = 0x80;
pub const ACPI_MAX_ADDRESS_SPACE: i32 = 255;
pub const ACPI_NUM_DEFAULT_SPACES: i32 = 4;
pub const ACPI_MAX_MATCH_OPCODE: i32 = 5;
pub const ACPI_RSDP_CHECKSUM_LENGTH: i32 = 20;
pub const ACPI_RSDP_XCHECKSUM_LENGTH: i32 = 36;

/* SMBus, GSBus and IPMI buffer sizes. */
pub const ACPI_SERIAL_HEADER_SIZE: i32 = 2; /* Common for below. Status and Length fields */
pub const ACPI_SMBUS_DATA_SIZE: i32 = 32;
pub const ACPI_SMBUS_BUFFER_SIZE: i32 = ACPI_SERIAL_HEADER_SIZE + ACPI_SMBUS_DATA_SIZE;
pub const ACPI_IPMI_DATA_SIZE: i32 = 64;
pub const ACPI_IPMI_BUFFER_SIZE: i32 = ACPI_SERIAL_HEADER_SIZE + ACPI_IPMI_DATA_SIZE;
pub const ACPI_MAX_GSBUS_DATA_SIZE: i32 = 255;
pub const ACPI_MAX_GSBUS_BUFFER_SIZE: i32 = ACPI_SERIAL_HEADER_SIZE + ACPI_MAX_GSBUS_DATA_SIZE;
pub const ACPI_PRM_INPUT_BUFFER_SIZE: i32 = 26;
pub const ACPI_FFH_INPUT_BUFFER_SIZE: i32 = 256;
pub const ACPI_NUM_sx_d_METHODS: i32 = 4;
pub const ACPI_NUM_sx_w_METHODS: i32 = 5;

/* UUID constants */
pub const UUID_BUFFER_LENGTH: i32 = 16;
pub const UUID_STRING_LENGTH: i32 = 36;
pub const UUID_HYPHEN1_OFFSET: i32 = 8;
pub const UUID_HYPHEN2_OFFSET: i32 = 13;
pub const UUID_HYPHEN3_OFFSET: i32 = 18;
pub const UUID_HYPHEN4_OFFSET: i32 = 23;

/* ACPI AML Debugger */
pub const ACPI_DEBUGGER_MAX_ARGS: i32 = ACPI_METHOD_NUM_ARGS + 4;
pub const ACPI_DB_LINE_BUFFER_SIZE: i32 = 512;
pub const ACPI_DEBUGGER_COMMAND_PROMPT: char = '-';
pub const ACPI_DEBUGGER_EXECUTE_PROMPT: char = '%';

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
