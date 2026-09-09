/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * CXL IOCTLs for Memory Devices
 */

// <linux/types.h>

/**
 * DOC: UAPI
 *
 * Not all of the commands that the driver supports are available for use by
 * userspace at all times. Userspace can check the result of the QUERY command
 * to determine the live set of commands. Alternatively, it can issue the
 * command and check for failure.
 */

// These ioctl encodings depend on the platform's _IOR/_IOWR definitions and
// the corresponding C layouts; retain the source expressions for resolution
// by the enclosing UAPI environment.
pub const CXL_MEM_QUERY_COMMANDS: u32 = _IOR(0xCE, 1, cxl_mem_query_commands);
pub const CXL_MEM_SEND_COMMAND: u32 = _IOWR(0xCE, 2, cxl_send_command);

pub const CXL_MEM_COMMAND_ID_INVALID: u32 = 0;
pub const CXL_MEM_COMMAND_ID_IDENTIFY: u32 = 1;
pub const CXL_MEM_COMMAND_ID_RAW: u32 = 2;
pub const CXL_MEM_COMMAND_ID_GET_SUPPORTED_LOGS: u32 = 3;
pub const CXL_MEM_COMMAND_ID_GET_FW_INFO: u32 = 4;
pub const CXL_MEM_COMMAND_ID_GET_PARTITION_INFO: u32 = 5;
pub const CXL_MEM_COMMAND_ID_GET_LSA: u32 = 6;
pub const CXL_MEM_COMMAND_ID_GET_HEALTH_INFO: u32 = 7;
pub const CXL_MEM_COMMAND_ID_GET_LOG: u32 = 8;
pub const CXL_MEM_COMMAND_ID_SET_PARTITION_INFO: u32 = 9;
pub const CXL_MEM_COMMAND_ID_SET_LSA: u32 = 10;
pub const CXL_MEM_COMMAND_ID_GET_ALERT_CONFIG: u32 = 11;
pub const CXL_MEM_COMMAND_ID_SET_ALERT_CONFIG: u32 = 12;
pub const CXL_MEM_COMMAND_ID_GET_SHUTDOWN_STATE: u32 = 13;
pub const CXL_MEM_COMMAND_ID_SET_SHUTDOWN_STATE: u32 = 14;
pub const CXL_MEM_DEPRECATED_ID_GET_POISON: u32 = 15;
pub const CXL_MEM_DEPRECATED_ID_INJECT_POISON: u32 = 16;
pub const CXL_MEM_DEPRECATED_ID_CLEAR_POISON: u32 = 17;
pub const CXL_MEM_COMMAND_ID_GET_SCAN_MEDIA_CAPS: u32 = 18;
pub const CXL_MEM_DEPRECATED_ID_SCAN_MEDIA: u32 = 19;
pub const CXL_MEM_DEPRECATED_ID_GET_SCAN_MEDIA: u32 = 20;
pub const CXL_MEM_COMMAND_ID_GET_TIMESTAMP: u32 = 21;
pub const CXL_MEM_COMMAND_ID_GET_LOG_CAPS: u32 = 22;
pub const CXL_MEM_COMMAND_ID_CLEAR_LOG: u32 = 23;
pub const CXL_MEM_COMMAND_ID_GET_SUP_LOG_SUBLIST: u32 = 24;
pub const CXL_MEM_COMMAND_ID_MAX: u32 = 25;

pub static CXL_COMMAND_NAMES: [&'static str; 26] = [
    "Invalid Command", "Identify Command", "Raw device command",
    "Get Supported Logs", "Get FW Info", "Get Partition Information",
    "Get Label Storage Area", "Get Health Info", "Get Log",
    "Set Partition Information", "Set Label Storage Area", "Get Alert Configuration",
    "Set Alert Configuration", "Get Shutdown State", "Set Shutdown State",
    "Deprecated Get Poison List", "Deprecated Inject Poison", "Deprecated Clear Poison",
    "Get Scan Media Capabilities", "Deprecated Scan Media", "Deprecated Get Scan Media Results",
    "Get Timestamp", "Get Log Capabilities", "Clear Log",
    "Get Supported Logs Sub-List", "invalid / last command",
];

pub static CXL_DEPRECATED_COMMANDS: [u8; 26] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0,
];

pub const CXL_MEM_COMMAND_FLAG_MASK: u32 = (1 << 1) | (1 << 0);
pub const CXL_MEM_COMMAND_FLAG_ENABLED: u32 = 1 << 0;
pub const CXL_MEM_COMMAND_FLAG_EXCLUSIVE: u32 = 1 << 1;

#[repr(C)]
pub struct cxl_command_info {
    pub id: u32,
    pub flags: u32,
    pub size_in: u32,
    pub size_out: u32,
}

#[repr(C)]
pub struct cxl_mem_query_commands {
    pub n_commands: u32,
    pub rsvd: u32,
    pub commands: [cxl_command_info; 0],
}

#[repr(C)]
pub union cxl_send_command_raw {
    pub raw: cxl_send_command_raw_fields,
    pub rsvd: u32,
}

#[repr(C)]
pub struct cxl_send_command_raw_fields {
    pub opcode: u16,
    pub rsvd: u16,
}

#[repr(C)]
pub struct cxl_send_command_payload {
    pub size: u32,
    pub rsvd: u32,
    pub payload: u64,
}

#[repr(C)]
pub struct cxl_send_command {
    pub id: u32,
    pub flags: u32,
    pub _anonymous_0: cxl_send_command_raw,
    pub retval: u32,
    pub input: cxl_send_command_payload,
    pub output: cxl_send_command_payload,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
