/* SPDX-License-Identifier: GPL-2.0 */
/* Primary function overlay window definitions
 * and service functions used by LPDDR chips
 */

/* Dependency: declarations from <linux/mtd/qinfo.h> are supplied externally. */

/* PFOW registers addressing */
/* Address of symbol "P" */
pub const PFOW_QUERY_STRING_P: u32 = 0x0000;
/* Address of symbol "F" */
pub const PFOW_QUERY_STRING_F: u32 = 0x0002;
/* Address of symbol "O" */
pub const PFOW_QUERY_STRING_O: u32 = 0x0004;
/* Address of symbol "W" */
pub const PFOW_QUERY_STRING_W: u32 = 0x0006;
/* Identification info for LPDDR chip */
pub const PFOW_MANUFACTURER_ID: u32 = 0x0020;
pub const PFOW_DEVICE_ID: u32 = 0x0022;
/* Address in PFOW where prog buffer can be found */
pub const PFOW_PROGRAM_BUFFER_OFFSET: u32 = 0x0040;
/* Size of program buffer in words */
pub const PFOW_PROGRAM_BUFFER_SIZE: u32 = 0x0042;
/* Address command code register */
pub const PFOW_COMMAND_CODE: u32 = 0x0080;
/* command data register */
pub const PFOW_COMMAND_DATA: u32 = 0x0084;
/* command address register lower address bits */
pub const PFOW_COMMAND_ADDRESS_L: u32 = 0x0088;
/* command address register upper address bits */
pub const PFOW_COMMAND_ADDRESS_H: u32 = 0x008a;
/* number of bytes to be proggrammed lower address bits */
pub const PFOW_DATA_COUNT_L: u32 = 0x0090;
/* number of bytes to be proggrammed higher address bits */
pub const PFOW_DATA_COUNT_H: u32 = 0x0092;
/* command execution register, the only possible value is 0x01 */
pub const PFOW_COMMAND_EXECUTE: u32 = 0x00c0;
/* 0x01 should be written at this address to clear buffer */
pub const PFOW_CLEAR_PROGRAM_BUFFER: u32 = 0x00c4;
/* device program/erase suspend register */
pub const PFOW_PROGRAM_ERASE_SUSPEND: u32 = 0x00c8;
/* device status register */
pub const PFOW_DSR: u32 = 0x00cc;

/* LPDDR memory device command codes */
/* They are possible values of PFOW command code register */
pub const LPDDR_WORD_PROGRAM: u32 = 0x0041;
pub const LPDDR_BUFF_PROGRAM: u32 = 0x00E9;
pub const LPDDR_BLOCK_ERASE: u32 = 0x0020;
pub const LPDDR_LOCK_BLOCK: u32 = 0x0061;
pub const LPDDR_UNLOCK_BLOCK: u32 = 0x0062;
pub const LPDDR_READ_BLOCK_LOCK_STATUS: u32 = 0x0065;
pub const LPDDR_INFO_QUERY: u32 = 0x0098;
pub const LPDDR_READ_OTP: u32 = 0x0097;
pub const LPDDR_PROG_OTP: u32 = 0x00C0;
pub const LPDDR_RESUME: u32 = 0x00D0;

/* Defines possible value of PFOW command execution register */
pub const LPDDR_START_EXECUTION: u32 = 0x0001;
/* Defines possible value of PFOW program/erase suspend register */
pub const LPDDR_SUSPEND: u32 = 0x0001;

/* Possible values of PFOW device status register */
/* access R - read; RC read & clearable */
pub const DSR_DPS: u32 = 1 << 1;
pub const DSR_PSS: u32 = 1 << 2;
pub const DSR_VPPS: u32 = 1 << 3;
pub const DSR_PROGRAM_STATUS: u32 = 1 << 4;
pub const DSR_ERASE_STATUS: u32 = 1 << 5;
pub const DSR_ESS: u32 = 1 << 6;
pub const DSR_READY_STATUS: u32 = 1 << 7;
pub const DSR_RPS: u32 = 0x3 << 8;
pub const DSR_AOS: u32 = 1 << 12;
pub const DSR_AVAILABLE: u32 = 1 << 15;

/* The superset of all possible error bits in DSR */
pub const DSR_ERR: u32 = 0x133A;

pub unsafe fn send_pfow_command(
    map: *mut map_info,
    cmd_code: ::core::ffi::c_ulong,
    adr: ::core::ffi::c_ulong,
    len: ::core::ffi::c_ulong,
    datum: *mut map_word,
) {
    let bits_per_chip = map_bankwidth(map) * 8;

    map_write(map, CMD(cmd_code), (*map).pfow_base + PFOW_COMMAND_CODE);
    map_write(
        map,
        CMD(adr & ((1 << bits_per_chip) - 1)),
        (*map).pfow_base + PFOW_COMMAND_ADDRESS_L,
    );
    map_write(
        map,
        CMD(adr >> bits_per_chip),
        (*map).pfow_base + PFOW_COMMAND_ADDRESS_H,
    );
    if len != 0 {
        map_write(
            map,
            CMD(len & ((1 << bits_per_chip) - 1)),
            (*map).pfow_base + PFOW_DATA_COUNT_L,
        );
        map_write(
            map,
            CMD(len >> bits_per_chip),
            (*map).pfow_base + PFOW_DATA_COUNT_H,
        );
    }
    if !datum.is_null() {
        map_write(map, *datum, (*map).pfow_base + PFOW_COMMAND_DATA);
    }

    /* Command execution start */
    map_write(
        map,
        CMD(LPDDR_START_EXECUTION as ::core::ffi::c_ulong),
        (*map).pfow_base + PFOW_COMMAND_EXECUTE,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
