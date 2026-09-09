/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the corresponding s390 KVM host definitions.

pub const PGM_OPERATION: u32 = 0x01;
pub const PGM_PRIVILEGED_OP: u32 = 0x02;
pub const PGM_EXECUTE: u32 = 0x03;
pub const PGM_PROTECTION: u32 = 0x04;
pub const PGM_ADDRESSING: u32 = 0x05;
pub const PGM_SPECIFICATION: u32 = 0x06;
pub const PGM_DATA: u32 = 0x07;
pub const PGM_FIXED_POINT_OVERFLOW: u32 = 0x08;
pub const PGM_FIXED_POINT_DIVIDE: u32 = 0x09;
pub const PGM_DECIMAL_OVERFLOW: u32 = 0x0a;
pub const PGM_DECIMAL_DIVIDE: u32 = 0x0b;
pub const PGM_HFP_EXPONENT_OVERFLOW: u32 = 0x0c;
pub const PGM_HFP_EXPONENT_UNDERFLOW: u32 = 0x0d;
pub const PGM_HFP_SIGNIFICANCE: u32 = 0x0e;
pub const PGM_HFP_DIVIDE: u32 = 0x0f;
pub const PGM_SEGMENT_TRANSLATION: u32 = 0x10;
pub const PGM_PAGE_TRANSLATION: u32 = 0x11;
pub const PGM_TRANSLATION_SPEC: u32 = 0x12;
pub const PGM_SPECIAL_OPERATION: u32 = 0x13;
pub const PGM_OPERAND: u32 = 0x15;
pub const PGM_TRACE_TABEL: u32 = 0x16;
pub const PGM_VECTOR_PROCESSING: u32 = 0x1b;
pub const PGM_SPACE_SWITCH: u32 = 0x1c;
pub const PGM_HFP_SQUARE_ROOT: u32 = 0x1d;
pub const PGM_PC_TRANSLATION_SPEC: u32 = 0x1f;
pub const PGM_AFX_TRANSLATION: u32 = 0x20;
pub const PGM_ASX_TRANSLATION: u32 = 0x21;
pub const PGM_LX_TRANSLATION: u32 = 0x22;
pub const PGM_EX_TRANSLATION: u32 = 0x23;
pub const PGM_PRIMARY_AUTHORITY: u32 = 0x24;
pub const PGM_SECONDARY_AUTHORITY: u32 = 0x25;
pub const PGM_LFX_TRANSLATION: u32 = 0x26;
pub const PGM_LSX_TRANSLATION: u32 = 0x27;
pub const PGM_ALET_SPECIFICATION: u32 = 0x28;
pub const PGM_ALEN_TRANSLATION: u32 = 0x29;
pub const PGM_ALE_SEQUENCE: u32 = 0x2a;
pub const PGM_ASTE_VALIDITY: u32 = 0x2b;
pub const PGM_ASTE_SEQUENCE: u32 = 0x2c;
pub const PGM_EXTENDED_AUTHORITY: u32 = 0x2d;
pub const PGM_LSTE_SEQUENCE: u32 = 0x2e;
pub const PGM_ASTE_INSTANCE: u32 = 0x2f;
pub const PGM_STACK_FULL: u32 = 0x30;
pub const PGM_STACK_EMPTY: u32 = 0x31;
pub const PGM_STACK_SPECIFICATION: u32 = 0x32;
pub const PGM_STACK_TYPE: u32 = 0x33;
pub const PGM_STACK_OPERATION: u32 = 0x34;
pub const PGM_ASCE_TYPE: u32 = 0x38;
pub const PGM_REGION_FIRST_TRANS: u32 = 0x39;
pub const PGM_REGION_SECOND_TRANS: u32 = 0x3a;
pub const PGM_REGION_THIRD_TRANS: u32 = 0x3b;
pub const PGM_SECURE_STORAGE_ACCESS: u32 = 0x3d;
pub const PGM_NON_SECURE_STORAGE_ACCESS: u32 = 0x3e;
pub const PGM_SECURE_STORAGE_VIOLATION: u32 = 0x3f;
pub const PGM_MONITOR: u32 = 0x40;
pub const PGM_PER: u32 = 0x80;
pub const PGM_CRYPTO_OPERATION: u32 = 0x119;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
