// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: psopinfo - AML opcode information functions and dispatch tables
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// C dependencies: acpi/acpi.h, accommon.h, acparser.h, acopcode.h, amlcode.h

static ACPI_GBL_ARGUMENT_COUNT: [u8; 12] = [0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 6];

/*******************************************************************************
 *
 * FUNCTION:    acpi_ps_get_opcode_info
 *
 * PARAMETERS:  opcode              - The AML opcode
 *
 * RETURN:      A pointer to the info about the opcode.
 *
 * DESCRIPTION: Find AML opcode description based on the opcode.
 *              NOTE: This procedure must ALWAYS return a valid pointer!
 *
 ******************************************************************************/

pub unsafe fn acpi_ps_get_opcode_info(opcode: u16) -> *const acpi_opcode_info {
    /* Detect normal 8-bit opcode or extended 16-bit opcode */
    if (opcode & 0xFF00) == 0 {
        /* Simple (8-bit) opcode: 0-255, can't index beyond table */
        return &acpi_gbl_aml_op_info[acpi_gbl_short_op_index[opcode as u8 as usize] as usize];
    }

    if (opcode & 0xFF00) == AML_EXTENDED_OPCODE
        && (opcode as u8) <= MAX_EXTENDED_OPCODE
    {
        /* Valid extended (16-bit) opcode */
        return &acpi_gbl_aml_op_info[acpi_gbl_long_op_index[opcode as u8 as usize] as usize];
    }

    #[cfg(all(ACPI_ASL_COMPILER, ACPI_DEBUG_OUTPUT))]
    {
        let opcode_name: *const core::ffi::c_char = match opcode {
            AML_RAW_DATA_BYTE => b"-Raw Data Byte-\0".as_ptr() as *const core::ffi::c_char,
            AML_RAW_DATA_WORD => b"-Raw Data Word-\0".as_ptr() as *const core::ffi::c_char,
            AML_RAW_DATA_DWORD => b"-Raw Data Dword-\0".as_ptr() as *const core::ffi::c_char,
            AML_RAW_DATA_QWORD => b"-Raw Data Qword-\0".as_ptr() as *const core::ffi::c_char,
            AML_RAW_DATA_BUFFER => b"-Raw Data Buffer-\0".as_ptr() as *const core::ffi::c_char,
            AML_RAW_DATA_CHAIN => b"-Raw Data Buffer Chain-\0".as_ptr() as *const core::ffi::c_char,
            AML_PACKAGE_LENGTH => b"-Package Length-\0".as_ptr() as *const core::ffi::c_char,
            AML_UNASSIGNED_OPCODE => b"-Unassigned Opcode-\0".as_ptr() as *const core::ffi::c_char,
            AML_DEFAULT_ARG_OP => b"-Default Arg-\0".as_ptr() as *const core::ffi::c_char,
            _ => b"Unknown AML opcode\0".as_ptr() as *const core::ffi::c_char,
        };
        // ACPI_DEBUG_PRINT((ACPI_DB_EXEC, "%s [%4.4X]\n", opcode_name, opcode));
        let _ = opcode_name;
    }

    &acpi_gbl_aml_op_info[_UNK as usize]
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ps_get_opcode_name
 *
 * PARAMETERS:  opcode              - The AML opcode
 *
 * RETURN:      A pointer to the name of the opcode (ASCII String)
 *              Note: Never returns NULL.
 *
 * DESCRIPTION: Translate an opcode into a human-readable string
 *
 ******************************************************************************/

pub unsafe fn acpi_ps_get_opcode_name(opcode: u16) -> *const core::ffi::c_char {
    #[cfg(any(ACPI_DISASSEMBLER, ACPI_DEBUG_OUTPUT))]
    {
        let op = acpi_ps_get_opcode_info(opcode);
        return (*op).name;
    }
    #[cfg(not(any(ACPI_DISASSEMBLER, ACPI_DEBUG_OUTPUT)))]
    {
        b"OpcodeName unavailable\0".as_ptr() as *const core::ffi::c_char
    }
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ps_get_argument_count
 *
 * PARAMETERS:  op_type             - Type associated with the AML opcode
 *
 * RETURN:      Argument count
 *
 * DESCRIPTION: Obtain the number of expected arguments for an AML opcode
 *
 ******************************************************************************/

pub fn acpi_ps_get_argument_count(op_type: u32) -> u8 {
    if op_type <= AML_TYPE_EXEC_6A_0T_1R {
        return ACPI_GBL_ARGUMENT_COUNT[op_type as usize];
    }
    0
}

/*
 * This table is directly indexed by the opcodes It returns
 * an index into the opcode table (acpi_gbl_aml_op_info)
 */
pub static acpi_gbl_short_op_index: [u8; 256] = [
    0x00, 0x01, _UNK, _UNK, _UNK, _UNK, 0x02, _UNK,
    0x03, _UNK, 0x04, 0x05, 0x06, 0x07, 0x6E, _UNK,
    0x08, 0x09, 0x0a, 0x6F, 0x0b, 0x81, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, 0x63, _PFX, _PFX,
    0x67, 0x66, 0x68, 0x65, 0x69, 0x64, 0x6A, 0x7D,
    0x7F, 0x80, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _ASC, _ASC, _ASC, _ASC, _ASC, _ASC, _ASC,
    _ASC, _ASC, _ASC, _ASC, _ASC, _ASC, _ASC, _ASC,
    _ASC, _ASC, _ASC, _ASC, _ASC, _ASC, _ASC, _ASC,
    _ASC, _ASC, _ASC, _UNK, _PFX, _UNK, _PFX, _ASC,
    0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13,
    0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, _UNK,
    0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22,
    0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a,
    0x2b, 0x2c, 0x2d, 0x2e, 0x70, 0x71, 0x2f, 0x30,
    0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x72,
    0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x73, 0x74,
    0x75, 0x76, _UNK, _UNK, 0x77, 0x78, 0x79, 0x7A,
    0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x60, 0x61,
    0x62, 0x82, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, 0x44, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, 0x45,
];

/*
 * This table is indexed by the second opcode of the extended opcode
 * pair. It returns an index into the opcode table (acpi_gbl_aml_op_info)
 */
pub static acpi_gbl_long_op_index: [u8; NUM_EXTENDED_OPCODE as usize] = [
    _UNK, 0x46, 0x47, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, 0x48, 0x49, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, 0x7B,
    0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51,
    0x52, 0x53, 0x54, _UNK, _UNK, _UNK, _UNK, _UNK,
    0x55, 0x56, 0x57, 0x7e, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK, _UNK,
    0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f,
    0x7C,
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
