// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Name: acopcode.h - AML opcode information for the AML parser and interpreter
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Translated from the C header. External opcode-argument symbols and list
// constructors are intentionally referenced but not implemented here.

/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: acopcode.h - AML opcode information for the AML parser and interpreter
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/


pub const MAX_EXTENDED_OPCODE: u32 = 0x88;
pub const NUM_EXTENDED_OPCODE: u32 = (MAX_EXTENDED_OPCODE + 1);
// C source leaves MAX_INTERNAL_OPCODE empty; its build-time definition is supplied externally.
pub const NUM_INTERNAL_OPCODE: u32 = (MAX_INTERNAL_OPCODE + 1);

/* Used for non-assigned opcodes */

pub const _UNK: u32 = 0x6B;

/*
 * Reserved ASCII characters. Do not use any of these for
 * internal opcodes, since they are used to differentiate
 * name strings from AML opcodes
 */
pub const _ASC: u32 = 0x6C;
pub const _NAM: u32 = 0x6C;
pub const _PFX: u32 = 0x6D;

/*
 * All AML opcodes and the parse-time arguments for each. Used by the AML
 * parser  Each list is compressed into a 32-bit number and stored in the
 * master opcode table (in psopcode.c).
 */
macro_rules! ARGP_ACCESSFIELD_OP { () => { ARGP_LIST1!(ARGP_NAMESTRING) }; }
macro_rules! ARGP_ACQUIRE_OP { () => { ARGP_LIST2!(ARGP_SUPERNAME,  ARGP_WORDDATA) }; }
macro_rules! ARGP_ADD_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TARGET) }; }
macro_rules! ARGP_ALIAS_OP { () => { ARGP_LIST2!(ARGP_NAMESTRING, ARGP_NAME) }; }
macro_rules! ARGP_ARG0 { () => { ARG_NONE }; }
macro_rules! ARGP_ARG1 { () => { ARG_NONE }; }
macro_rules! ARGP_ARG2 { () => { ARG_NONE }; }
macro_rules! ARGP_ARG3 { () => { ARG_NONE }; }
macro_rules! ARGP_ARG4 { () => { ARG_NONE }; }
macro_rules! ARGP_ARG5 { () => { ARG_NONE }; }
macro_rules! ARGP_ARG6 { () => { ARG_NONE }; }
macro_rules! ARGP_BANK_FIELD_OP { () => { ARGP_LIST6!(ARGP_PKGLENGTH,  ARGP_NAMESTRING,    ARGP_NAMESTRING,ARGP_TERMARG,   ARGP_BYTEDATA,  ARGP_FIELDLIST) }; }
macro_rules! ARGP_BIT_AND_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TARGET) }; }
macro_rules! ARGP_BIT_NAND_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TARGET) }; }
macro_rules! ARGP_BIT_NOR_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TARGET) }; }
macro_rules! ARGP_BIT_NOT_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TARGET) }; }
macro_rules! ARGP_BIT_OR_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TARGET) }; }
macro_rules! ARGP_BIT_XOR_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TARGET) }; }
macro_rules! ARGP_BREAK_OP { () => { ARG_NONE }; }
macro_rules! ARGP_BREAK_POINT_OP { () => { ARG_NONE }; }
macro_rules! ARGP_BUFFER_OP { () => { ARGP_LIST3!(ARGP_PKGLENGTH,  ARGP_TERMARG,       ARGP_BYTELIST) }; }
macro_rules! ARGP_BYTE_OP { () => { ARGP_LIST1!(ARGP_BYTEDATA) }; }
macro_rules! ARGP_BYTELIST_OP { () => { ARGP_LIST1!(ARGP_NAMESTRING) }; }
macro_rules! ARGP_COMMENT_OP { () => { ARGP_LIST2!(ARGP_BYTEDATA,   ARGP_COMMENT) }; }
macro_rules! ARGP_CONCAT_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TARGET) }; }
macro_rules! ARGP_CONCAT_RES_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TARGET) }; }
macro_rules! ARGP_COND_REF_OF_OP { () => { ARGP_LIST2!(ARGP_SIMPLENAME, ARGP_TARGET) }; }
macro_rules! ARGP_CONNECTFIELD_OP { () => { ARGP_LIST1!(ARGP_NAMESTRING) }; }
macro_rules! ARGP_CONTINUE_OP { () => { ARG_NONE }; }
macro_rules! ARGP_COPY_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_SIMPLENAME) }; }
macro_rules! ARGP_CREATE_BIT_FIELD_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_NAME) }; }
macro_rules! ARGP_CREATE_BYTE_FIELD_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_NAME) }; }
macro_rules! ARGP_CREATE_DWORD_FIELD_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_NAME) }; }
macro_rules! ARGP_CREATE_FIELD_OP { () => { ARGP_LIST4!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TERMARG,   ARGP_NAME) }; }
macro_rules! ARGP_CREATE_QWORD_FIELD_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_NAME) }; }
macro_rules! ARGP_CREATE_WORD_FIELD_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_NAME) }; }
macro_rules! ARGP_DATA_REGION_OP { () => { ARGP_LIST4!(ARGP_NAME,       ARGP_TERMARG,       ARGP_TERMARG,   ARGP_TERMARG) }; }
macro_rules! ARGP_DEBUG_OP { () => { ARG_NONE }; }
macro_rules! ARGP_DECREMENT_OP { () => { ARGP_LIST1!(ARGP_SUPERNAME) }; }
macro_rules! ARGP_DEREF_OF_OP { () => { ARGP_LIST1!(ARGP_SUPERNAME) }; }
macro_rules! ARGP_DEVICE_OP { () => { ARGP_LIST3!(ARGP_PKGLENGTH,  ARGP_NAME,          ARGP_OBJLIST) }; }
macro_rules! ARGP_DIVIDE_OP { () => { ARGP_LIST4!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TARGET,    ARGP_TARGET) }; }
macro_rules! ARGP_DWORD_OP { () => { ARGP_LIST1!(ARGP_DWORDDATA) }; }
macro_rules! ARGP_ELSE_OP { () => { ARGP_LIST2!(ARGP_PKGLENGTH,  ARGP_TERMLIST) }; }
macro_rules! ARGP_EVENT_OP { () => { ARGP_LIST1!(ARGP_NAME) }; }
macro_rules! ARGP_EXTERNAL_OP { () => { ARGP_LIST3!(ARGP_NAME,       ARGP_BYTEDATA,      ARGP_BYTEDATA) }; }
macro_rules! ARGP_FATAL_OP { () => { ARGP_LIST3!(ARGP_BYTEDATA,   ARGP_DWORDDATA,     ARGP_TERMARG) }; }
macro_rules! ARGP_FIELD_OP { () => { ARGP_LIST4!(ARGP_PKGLENGTH,  ARGP_NAMESTRING,    ARGP_BYTEDATA,  ARGP_FIELDLIST) }; }
macro_rules! ARGP_FIND_SET_LEFT_BIT_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TARGET) }; }
macro_rules! ARGP_FIND_SET_RIGHT_BIT_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TARGET) }; }
macro_rules! ARGP_FROM_BCD_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TARGET) }; }
macro_rules! ARGP_IF_OP { () => { ARGP_LIST3!(ARGP_PKGLENGTH,  ARGP_TERMARG,       ARGP_TERMLIST) }; }
macro_rules! ARGP_INCREMENT_OP { () => { ARGP_LIST1!(ARGP_SUPERNAME) }; }
macro_rules! ARGP_INDEX_FIELD_OP { () => { ARGP_LIST5!(ARGP_PKGLENGTH,  ARGP_NAMESTRING,    ARGP_NAMESTRING,ARGP_BYTEDATA,  ARGP_FIELDLIST) }; }
macro_rules! ARGP_INDEX_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TARGET) }; }
macro_rules! ARGP_LAND_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TERMARG) }; }
macro_rules! ARGP_LEQUAL_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TERMARG) }; }
macro_rules! ARGP_LGREATER_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TERMARG) }; }
macro_rules! ARGP_LGREATEREQUAL_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TERMARG) }; }
macro_rules! ARGP_LLESS_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TERMARG) }; }
macro_rules! ARGP_LLESSEQUAL_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TERMARG) }; }
macro_rules! ARGP_LNOT_OP { () => { ARGP_LIST1!(ARGP_TERMARG) }; }
macro_rules! ARGP_LNOTEQUAL_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TERMARG) }; }
macro_rules! ARGP_LOAD_OP { () => { ARGP_LIST2!(ARGP_NAMESTRING, ARGP_SUPERNAME) }; }
macro_rules! ARGP_LOAD_TABLE_OP { () => { ARGP_LIST6!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TERMARG,   ARGP_TERMARG,  ARGP_TERMARG,   ARGP_TERMARG) }; }
macro_rules! ARGP_LOCAL0 { () => { ARG_NONE }; }
macro_rules! ARGP_LOCAL1 { () => { ARG_NONE }; }
macro_rules! ARGP_LOCAL2 { () => { ARG_NONE }; }
macro_rules! ARGP_LOCAL3 { () => { ARG_NONE }; }
macro_rules! ARGP_LOCAL4 { () => { ARG_NONE }; }
macro_rules! ARGP_LOCAL5 { () => { ARG_NONE }; }
macro_rules! ARGP_LOCAL6 { () => { ARG_NONE }; }
macro_rules! ARGP_LOCAL7 { () => { ARG_NONE }; }
macro_rules! ARGP_LOR_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TERMARG) }; }
macro_rules! ARGP_MATCH_OP { () => { ARGP_LIST6!(ARGP_TERMARG,    ARGP_BYTEDATA,      ARGP_TERMARG,   ARGP_BYTEDATA,  ARGP_TERMARG,   ARGP_TERMARG) }; }
macro_rules! ARGP_METHOD_OP { () => { ARGP_LIST4!(ARGP_PKGLENGTH,  ARGP_NAME,          ARGP_BYTEDATA,  ARGP_TERMLIST) }; }
macro_rules! ARGP_METHODCALL_OP { () => { ARGP_LIST1!(ARGP_NAMESTRING) }; }
macro_rules! ARGP_MID_OP { () => { ARGP_LIST4!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TERMARG,   ARGP_TARGET) }; }
macro_rules! ARGP_MOD_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TARGET) }; }
macro_rules! ARGP_MULTIPLY_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TARGET) }; }
macro_rules! ARGP_MUTEX_OP { () => { ARGP_LIST2!(ARGP_NAME,       ARGP_BYTEDATA) }; }
macro_rules! ARGP_NAME_OP { () => { ARGP_LIST2!(ARGP_NAME,       ARGP_DATAOBJ) }; }
macro_rules! ARGP_NAMEDFIELD_OP { () => { ARGP_LIST1!(ARGP_NAMESTRING) }; }
macro_rules! ARGP_NAMEPATH_OP { () => { ARGP_LIST1!(ARGP_NAMESTRING) }; }
macro_rules! ARGP_NOOP_OP { () => { ARG_NONE }; }
macro_rules! ARGP_NOTIFY_OP { () => { ARGP_LIST2!(ARGP_SUPERNAME,  ARGP_TERMARG) }; }
macro_rules! ARGP_OBJECT_TYPE_OP { () => { ARGP_LIST1!(ARGP_SIMPLENAME) }; }
macro_rules! ARGP_ONE_OP { () => { ARG_NONE }; }
macro_rules! ARGP_ONES_OP { () => { ARG_NONE }; }
macro_rules! ARGP_PACKAGE_OP { () => { ARGP_LIST3!(ARGP_PKGLENGTH,  ARGP_BYTEDATA,      ARGP_DATAOBJLIST) }; }
macro_rules! ARGP_POWER_RES_OP { () => { ARGP_LIST5!(ARGP_PKGLENGTH,  ARGP_NAME,          ARGP_BYTEDATA,  ARGP_WORDDATA,  ARGP_OBJLIST) }; }
macro_rules! ARGP_PROCESSOR_OP { () => { ARGP_LIST6!(ARGP_PKGLENGTH,  ARGP_NAME,          ARGP_BYTEDATA,  ARGP_DWORDDATA, ARGP_BYTEDATA,  ARGP_OBJLIST) }; }
macro_rules! ARGP_QWORD_OP { () => { ARGP_LIST1!(ARGP_QWORDDATA) }; }
macro_rules! ARGP_REF_OF_OP { () => { ARGP_LIST1!(ARGP_SIMPLENAME) }; }
macro_rules! ARGP_REGION_OP { () => { ARGP_LIST4!(ARGP_NAME,       ARGP_BYTEDATA,      ARGP_TERMARG,   ARGP_TERMARG) }; }
macro_rules! ARGP_RELEASE_OP { () => { ARGP_LIST1!(ARGP_SUPERNAME) }; }
macro_rules! ARGP_RESERVEDFIELD_OP { () => { ARGP_LIST1!(ARGP_NAMESTRING) }; }
macro_rules! ARGP_RESET_OP { () => { ARGP_LIST1!(ARGP_SUPERNAME) }; }
macro_rules! ARGP_RETURN_OP { () => { ARGP_LIST1!(ARGP_TERMARG) }; }
macro_rules! ARGP_REVISION_OP { () => { ARG_NONE }; }
macro_rules! ARGP_SCOPE_OP { () => { ARGP_LIST3!(ARGP_PKGLENGTH,  ARGP_NAME,          ARGP_TERMLIST) }; }
macro_rules! ARGP_SERIALFIELD_OP { () => { ARGP_LIST1!(ARGP_NAMESTRING) }; }
macro_rules! ARGP_SHIFT_LEFT_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TARGET) }; }
macro_rules! ARGP_SHIFT_RIGHT_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TARGET) }; }
macro_rules! ARGP_SIGNAL_OP { () => { ARGP_LIST1!(ARGP_SUPERNAME) }; }
macro_rules! ARGP_SIZE_OF_OP { () => { ARGP_LIST1!(ARGP_SUPERNAME) }; }
macro_rules! ARGP_SLEEP_OP { () => { ARGP_LIST1!(ARGP_TERMARG) }; }
macro_rules! ARGP_STALL_OP { () => { ARGP_LIST1!(ARGP_TERMARG) }; }
macro_rules! ARGP_STATICSTRING_OP { () => { ARGP_LIST1!(ARGP_NAMESTRING) }; }
macro_rules! ARGP_STORE_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_SUPERNAME) }; }
macro_rules! ARGP_STRING_OP { () => { ARGP_LIST1!(ARGP_CHARLIST) }; }
macro_rules! ARGP_SUBTRACT_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TARGET) }; }
macro_rules! ARGP_THERMAL_ZONE_OP { () => { ARGP_LIST3!(ARGP_PKGLENGTH,  ARGP_NAME,          ARGP_OBJLIST) }; }
macro_rules! ARGP_TIMER_OP { () => { ARG_NONE }; }
macro_rules! ARGP_TO_BCD_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TARGET) }; }
macro_rules! ARGP_TO_BUFFER_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TARGET) }; }
macro_rules! ARGP_TO_DEC_STR_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TARGET) }; }
macro_rules! ARGP_TO_HEX_STR_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TARGET) }; }
macro_rules! ARGP_TO_INTEGER_OP { () => { ARGP_LIST2!(ARGP_TERMARG,    ARGP_TARGET) }; }
macro_rules! ARGP_TO_STRING_OP { () => { ARGP_LIST3!(ARGP_TERMARG,    ARGP_TERMARG,       ARGP_TARGET) }; }
macro_rules! ARGP_UNLOAD_OP { () => { ARGP_LIST1!(ARGP_SUPERNAME) }; }
macro_rules! ARGP_VAR_PACKAGE_OP { () => { ARGP_LIST3!(ARGP_PKGLENGTH,  ARGP_TERMARG,       ARGP_DATAOBJLIST) }; }
macro_rules! ARGP_WAIT_OP { () => { ARGP_LIST2!(ARGP_SUPERNAME,  ARGP_TERMARG) }; }
macro_rules! ARGP_WHILE_OP { () => { ARGP_LIST3!(ARGP_PKGLENGTH,  ARGP_TERMARG,       ARGP_TERMLIST) }; }
macro_rules! ARGP_WORD_OP { () => { ARGP_LIST1!(ARGP_WORDDATA) }; }
macro_rules! ARGP_ZERO_OP { () => { ARG_NONE }; }

/*
 * All AML opcodes and the runtime arguments for each. Used by the AML
 * interpreter  Each list is compressed into a 32-bit number and stored
 * in the master opcode table (in psopcode.c).
 *
 * (Used by prep_operands procedure and the ASL Compiler)
 */
macro_rules! ARGI_ACCESSFIELD_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_ACQUIRE_OP { () => { ARGI_LIST2!(ARGI_MUTEX,      ARGI_INTEGER) }; }
macro_rules! ARGI_ADD_OP { () => { ARGI_LIST3!(ARGI_INTEGER,    ARGI_INTEGER,       ARGI_TARGETREF) }; }
macro_rules! ARGI_ALIAS_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_ARG0 { () => { ARG_NONE }; }
macro_rules! ARGI_ARG1 { () => { ARG_NONE }; }
macro_rules! ARGI_ARG2 { () => { ARG_NONE }; }
macro_rules! ARGI_ARG3 { () => { ARG_NONE }; }
macro_rules! ARGI_ARG4 { () => { ARG_NONE }; }
macro_rules! ARGI_ARG5 { () => { ARG_NONE }; }
macro_rules! ARGI_ARG6 { () => { ARG_NONE }; }
macro_rules! ARGI_BANK_FIELD_OP { () => { ARGI_LIST1!(ARGI_INTEGER) }; }
macro_rules! ARGI_BIT_AND_OP { () => { ARGI_LIST3!(ARGI_INTEGER,    ARGI_INTEGER,       ARGI_TARGETREF) }; }
macro_rules! ARGI_BIT_NAND_OP { () => { ARGI_LIST3!(ARGI_INTEGER,    ARGI_INTEGER,       ARGI_TARGETREF) }; }
macro_rules! ARGI_BIT_NOR_OP { () => { ARGI_LIST3!(ARGI_INTEGER,    ARGI_INTEGER,       ARGI_TARGETREF) }; }
macro_rules! ARGI_BIT_NOT_OP { () => { ARGI_LIST2!(ARGI_INTEGER,    ARGI_TARGETREF) }; }
macro_rules! ARGI_BIT_OR_OP { () => { ARGI_LIST3!(ARGI_INTEGER,    ARGI_INTEGER,       ARGI_TARGETREF) }; }
macro_rules! ARGI_BIT_XOR_OP { () => { ARGI_LIST3!(ARGI_INTEGER,    ARGI_INTEGER,       ARGI_TARGETREF) }; }
macro_rules! ARGI_BREAK_OP { () => { ARG_NONE }; }
macro_rules! ARGI_BREAK_POINT_OP { () => { ARG_NONE }; }
macro_rules! ARGI_BUFFER_OP { () => { ARGI_LIST1!(ARGI_INTEGER) }; }
macro_rules! ARGI_BYTE_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_BYTELIST_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_COMMENT_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_CONCAT_OP { () => { ARGI_LIST3!(ARGI_ANYTYPE,    ARGI_ANYTYPE,       ARGI_TARGETREF) }; }
macro_rules! ARGI_CONCAT_RES_OP { () => { ARGI_LIST3!(ARGI_BUFFER,     ARGI_BUFFER,        ARGI_TARGETREF) }; }
macro_rules! ARGI_COND_REF_OF_OP { () => { ARGI_LIST2!(ARGI_OBJECT_REF, ARGI_TARGETREF) }; }
macro_rules! ARGI_CONNECTFIELD_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_CONTINUE_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_COPY_OP { () => { ARGI_LIST2!(ARGI_ANYTYPE,    ARGI_SIMPLE_TARGET) }; }
macro_rules! ARGI_CREATE_BIT_FIELD_OP { () => { ARGI_LIST3!(ARGI_BUFFER,     ARGI_INTEGER,       ARGI_REFERENCE) }; }
macro_rules! ARGI_CREATE_BYTE_FIELD_OP { () => { ARGI_LIST3!(ARGI_BUFFER,     ARGI_INTEGER,       ARGI_REFERENCE) }; }
macro_rules! ARGI_CREATE_DWORD_FIELD_OP { () => { ARGI_LIST3!(ARGI_BUFFER,     ARGI_INTEGER,       ARGI_REFERENCE) }; }
macro_rules! ARGI_CREATE_FIELD_OP { () => { ARGI_LIST4!(ARGI_BUFFER,     ARGI_INTEGER,       ARGI_INTEGER,      ARGI_REFERENCE) }; }
macro_rules! ARGI_CREATE_QWORD_FIELD_OP { () => { ARGI_LIST3!(ARGI_BUFFER,     ARGI_INTEGER,       ARGI_REFERENCE) }; }
macro_rules! ARGI_CREATE_WORD_FIELD_OP { () => { ARGI_LIST3!(ARGI_BUFFER,     ARGI_INTEGER,       ARGI_REFERENCE) }; }
macro_rules! ARGI_DATA_REGION_OP { () => { ARGI_LIST3!(ARGI_STRING,     ARGI_STRING,        ARGI_STRING) }; }
macro_rules! ARGI_DEBUG_OP { () => { ARG_NONE }; }
macro_rules! ARGI_DECREMENT_OP { () => { ARGI_LIST1!(ARGI_TARGETREF) }; }
macro_rules! ARGI_DEREF_OF_OP { () => { ARGI_LIST1!(ARGI_REF_OR_STRING) }; }
macro_rules! ARGI_DEVICE_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_DIVIDE_OP { () => { ARGI_LIST4!(ARGI_INTEGER,    ARGI_INTEGER,       ARGI_TARGETREF,    ARGI_TARGETREF) }; }
macro_rules! ARGI_DWORD_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_ELSE_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_EVENT_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_EXTERNAL_OP { () => { ARGI_LIST3!(ARGI_STRING,     ARGI_INTEGER,       ARGI_INTEGER) }; }
macro_rules! ARGI_FATAL_OP { () => { ARGI_LIST3!(ARGI_INTEGER,    ARGI_INTEGER,       ARGI_INTEGER) }; }
macro_rules! ARGI_FIELD_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_FIND_SET_LEFT_BIT_OP { () => { ARGI_LIST2!(ARGI_INTEGER,    ARGI_TARGETREF) }; }
macro_rules! ARGI_FIND_SET_RIGHT_BIT_OP { () => { ARGI_LIST2!(ARGI_INTEGER,    ARGI_TARGETREF) }; }
macro_rules! ARGI_FROM_BCD_OP { () => { ARGI_LIST2!(ARGI_INTEGER,    ARGI_FIXED_TARGET) }; }
macro_rules! ARGI_IF_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_INCREMENT_OP { () => { ARGI_LIST1!(ARGI_TARGETREF) }; }
macro_rules! ARGI_INDEX_FIELD_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_INDEX_OP { () => { ARGI_LIST3!(ARGI_COMPLEXOBJ, ARGI_INTEGER,       ARGI_TARGETREF) }; }
macro_rules! ARGI_LAND_OP { () => { ARGI_LIST2!(ARGI_INTEGER,    ARGI_INTEGER) }; }
macro_rules! ARGI_LEQUAL_OP { () => { ARGI_LIST2!(ARGI_COMPUTEDATA,ARGI_COMPUTEDATA) }; }
macro_rules! ARGI_LGREATER_OP { () => { ARGI_LIST2!(ARGI_COMPUTEDATA,ARGI_COMPUTEDATA) }; }
macro_rules! ARGI_LGREATEREQUAL_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_LLESS_OP { () => { ARGI_LIST2!(ARGI_COMPUTEDATA,ARGI_COMPUTEDATA) }; }
macro_rules! ARGI_LLESSEQUAL_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_LNOT_OP { () => { ARGI_LIST1!(ARGI_INTEGER) }; }
macro_rules! ARGI_LNOTEQUAL_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_LOAD_OP { () => { ARGI_LIST2!(ARGI_REGION_OR_BUFFER,ARGI_TARGETREF) }; }
macro_rules! ARGI_LOAD_TABLE_OP { () => { ARGI_LIST6!(ARGI_STRING,     ARGI_STRING,        ARGI_STRING,       ARGI_STRING,    ARGI_STRING, ARGI_ANYTYPE) }; }
macro_rules! ARGI_LOCAL0 { () => { ARG_NONE }; }
macro_rules! ARGI_LOCAL1 { () => { ARG_NONE }; }
macro_rules! ARGI_LOCAL2 { () => { ARG_NONE }; }
macro_rules! ARGI_LOCAL3 { () => { ARG_NONE }; }
macro_rules! ARGI_LOCAL4 { () => { ARG_NONE }; }
macro_rules! ARGI_LOCAL5 { () => { ARG_NONE }; }
macro_rules! ARGI_LOCAL6 { () => { ARG_NONE }; }
macro_rules! ARGI_LOCAL7 { () => { ARG_NONE }; }
macro_rules! ARGI_LOR_OP { () => { ARGI_LIST2!(ARGI_INTEGER,    ARGI_INTEGER) }; }
macro_rules! ARGI_MATCH_OP { () => { ARGI_LIST6!(ARGI_PACKAGE,    ARGI_INTEGER,   ARGI_COMPUTEDATA,      ARGI_INTEGER,ARGI_COMPUTEDATA,ARGI_INTEGER) }; }
macro_rules! ARGI_METHOD_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_METHODCALL_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_MID_OP { () => { ARGI_LIST4!(ARGI_BUFFER_OR_STRING,ARGI_INTEGER,  ARGI_INTEGER,      ARGI_TARGETREF) }; }
macro_rules! ARGI_MOD_OP { () => { ARGI_LIST3!(ARGI_INTEGER,    ARGI_INTEGER,       ARGI_TARGETREF) }; }
macro_rules! ARGI_MULTIPLY_OP { () => { ARGI_LIST3!(ARGI_INTEGER,    ARGI_INTEGER,       ARGI_TARGETREF) }; }
macro_rules! ARGI_MUTEX_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_NAME_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_NAMEDFIELD_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_NAMEPATH_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_NOOP_OP { () => { ARG_NONE }; }
macro_rules! ARGI_NOTIFY_OP { () => { ARGI_LIST2!(ARGI_DEVICE_REF, ARGI_INTEGER) }; }
macro_rules! ARGI_OBJECT_TYPE_OP { () => { ARGI_LIST1!(ARGI_ANYTYPE) }; }
macro_rules! ARGI_ONE_OP { () => { ARG_NONE }; }
macro_rules! ARGI_ONES_OP { () => { ARG_NONE }; }
macro_rules! ARGI_PACKAGE_OP { () => { ARGI_LIST1!(ARGI_INTEGER) }; }
macro_rules! ARGI_POWER_RES_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_PROCESSOR_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_QWORD_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_REF_OF_OP { () => { ARGI_LIST1!(ARGI_OBJECT_REF) }; }
macro_rules! ARGI_REGION_OP { () => { ARGI_LIST2!(ARGI_INTEGER,    ARGI_INTEGER) }; }
macro_rules! ARGI_RELEASE_OP { () => { ARGI_LIST1!(ARGI_MUTEX) }; }
macro_rules! ARGI_RESERVEDFIELD_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_RESET_OP { () => { ARGI_LIST1!(ARGI_EVENT) }; }
macro_rules! ARGI_RETURN_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_REVISION_OP { () => { ARG_NONE }; }
macro_rules! ARGI_SCOPE_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_SERIALFIELD_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_SHIFT_LEFT_OP { () => { ARGI_LIST3!(ARGI_INTEGER,    ARGI_INTEGER,       ARGI_TARGETREF) }; }
macro_rules! ARGI_SHIFT_RIGHT_OP { () => { ARGI_LIST3!(ARGI_INTEGER,    ARGI_INTEGER,       ARGI_TARGETREF) }; }
macro_rules! ARGI_SIGNAL_OP { () => { ARGI_LIST1!(ARGI_EVENT) }; }
macro_rules! ARGI_SIZE_OF_OP { () => { ARGI_LIST1!(ARGI_DATAOBJECT) }; }
macro_rules! ARGI_SLEEP_OP { () => { ARGI_LIST1!(ARGI_INTEGER) }; }
macro_rules! ARGI_STALL_OP { () => { ARGI_LIST1!(ARGI_INTEGER) }; }
macro_rules! ARGI_STATICSTRING_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_STORE_OP { () => { ARGI_LIST2!(ARGI_DATAREFOBJ, ARGI_STORE_TARGET) }; }
macro_rules! ARGI_STRING_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_SUBTRACT_OP { () => { ARGI_LIST3!(ARGI_INTEGER,    ARGI_INTEGER,       ARGI_TARGETREF) }; }
macro_rules! ARGI_THERMAL_ZONE_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_TIMER_OP { () => { ARG_NONE }; }
macro_rules! ARGI_TO_BCD_OP { () => { ARGI_LIST2!(ARGI_INTEGER,    ARGI_FIXED_TARGET) }; }
macro_rules! ARGI_TO_BUFFER_OP { () => { ARGI_LIST2!(ARGI_COMPUTEDATA,ARGI_FIXED_TARGET) }; }
macro_rules! ARGI_TO_DEC_STR_OP { () => { ARGI_LIST2!(ARGI_COMPUTEDATA,ARGI_FIXED_TARGET) }; }
macro_rules! ARGI_TO_HEX_STR_OP { () => { ARGI_LIST2!(ARGI_COMPUTEDATA,ARGI_FIXED_TARGET) }; }
macro_rules! ARGI_TO_INTEGER_OP { () => { ARGI_LIST2!(ARGI_COMPUTEDATA,ARGI_FIXED_TARGET) }; }
macro_rules! ARGI_TO_STRING_OP { () => { ARGI_LIST3!(ARGI_BUFFER,     ARGI_INTEGER,       ARGI_FIXED_TARGET) }; }
macro_rules! ARGI_UNLOAD_OP { () => { ARGI_LIST1!(ARGI_DDBHANDLE) }; }
macro_rules! ARGI_VAR_PACKAGE_OP { () => { ARGI_LIST1!(ARGI_INTEGER) }; }
macro_rules! ARGI_WAIT_OP { () => { ARGI_LIST2!(ARGI_EVENT,      ARGI_INTEGER) }; }
macro_rules! ARGI_WHILE_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_WORD_OP { () => { ARGI_INVALID_OPCODE }; }
macro_rules! ARGI_ZERO_OP { () => { ARG_NONE }; }



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
