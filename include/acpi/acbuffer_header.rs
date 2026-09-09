/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: acbuffer.h - Support for buffers returned by ACPI predefined names
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/*
 * Contains buffer structures for these predefined names:
 * _FDE, _GRT, _GTM, _PLD, _SRT
 */

/*
 * Note: C bitfields are not used for this reason:
 *
 * "Bitfields are great and easy to read, but unfortunately the C language
 * does not specify the layout of bitfields in memory, which means they are
 * essentially useless for dealing with packed data in on-disk formats or
 * binary wire protocols." (Or ACPI tables and buffers.) "If you ask me,
 * this decision was a design error in C. Ritchie could have picked an order
 * and stuck with it." Norman Ramsey.
 * See http://stackoverflow.com/a/1053662/41661
 */

/* _FDE return value */
#[repr(C)]
pub struct acpi_fde_info {
    pub floppy0: u32,
    pub floppy1: u32,
    pub floppy2: u32,
    pub floppy3: u32,
    pub tape: u32,
}

/* _GRT return value
 * _SRT input value
 */
#[repr(C)]
pub struct acpi_grt_info {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub valid: u8,
    pub milliseconds: u16,
    pub timezone: u16,
    pub daylight: u8,
    pub reserved: [u8; 3],
}

/* _GTM return value */
#[repr(C)]
pub struct acpi_gtm_info {
    pub pio_speed0: u32,
    pub dma_speed0: u32,
    pub pio_speed1: u32,
    pub dma_speed1: u32,
    pub flags: u32,
}

/* Formatted _PLD return value. The minimum size is a package containing
 * one buffer.
 * Revision 1: Buffer is 16 bytes (128 bits)
 * Revision 2: Buffer is 20 bytes (160 bits)
 *
 * Note: This structure is returned from the acpi_decode_pld_buffer
 * interface.
 */
#[repr(C)]
pub struct acpi_pld_info {
    pub revision: u8,
    pub ignore_color: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub width: u16,
    pub height: u16,
    pub user_visible: u8,
    pub dock: u8,
    pub lid: u8,
    pub panel: u8,
    pub vertical_position: u8,
    pub horizontal_position: u8,
    pub shape: u8,
    pub group_orientation: u8,
    pub group_token: u8,
    pub group_position: u8,
    pub bay: u8,
    pub ejectable: u8,
    pub ospm_eject_required: u8,
    pub cabinet_number: u8,
    pub card_cage_number: u8,
    pub reference: u8,
    pub rotation: u8,
    pub order: u8,
    pub reserved: u8,
    pub vertical_offset: u16,
    pub horizontal_offset: u16,
}

/* Macros to:
 *     1) Convert a _PLD buffer to internal struct acpi_pld_info format - ACPI_PLD_GET*
 *        (Used by acpi_decode_pld_buffer)
 *     2) Construct a _PLD buffer - ACPI_PLD_SET*
 *        (Intended for BIOS use only)
 */
pub const ACPI_PLD_REV1_BUFFER_SIZE: usize = 16;
pub const ACPI_PLD_REV2_BUFFER_SIZE: usize = 20;
pub const ACPI_PLD_BUFFER_SIZE: usize = 20;

/* These macros depend on the externally supplied ACPI_GET_BITS and ACPI_SET_BITS macros. */
macro_rules! ACPI_PLD_GET_REVISION { ($dword:expr) => { ACPI_GET_BITS!($dword, 0, ACPI_7BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_REVISION { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 0, ACPI_7BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_IGNORE_COLOR { ($dword:expr) => { ACPI_GET_BITS!($dword, 7, ACPI_1BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_IGNORE_COLOR { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 7, ACPI_1BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_RED { ($dword:expr) => { ACPI_GET_BITS!($dword, 8, ACPI_8BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_RED { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 8, ACPI_8BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_GREEN { ($dword:expr) => { ACPI_GET_BITS!($dword, 16, ACPI_8BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_GREEN { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 16, ACPI_8BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_BLUE { ($dword:expr) => { ACPI_GET_BITS!($dword, 24, ACPI_8BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_BLUE { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 24, ACPI_8BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_WIDTH { ($dword:expr) => { ACPI_GET_BITS!($dword, 0, ACPI_16BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_WIDTH { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 0, ACPI_16BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_HEIGHT { ($dword:expr) => { ACPI_GET_BITS!($dword, 16, ACPI_16BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_HEIGHT { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 16, ACPI_16BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_USER_VISIBLE { ($dword:expr) => { ACPI_GET_BITS!($dword, 0, ACPI_1BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_USER_VISIBLE { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 0, ACPI_1BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_DOCK { ($dword:expr) => { ACPI_GET_BITS!($dword, 1, ACPI_1BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_DOCK { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 1, ACPI_1BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_LID { ($dword:expr) => { ACPI_GET_BITS!($dword, 2, ACPI_1BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_LID { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 2, ACPI_1BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_PANEL { ($dword:expr) => { ACPI_GET_BITS!($dword, 3, ACPI_3BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_PANEL { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 3, ACPI_3BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_VERTICAL { ($dword:expr) => { ACPI_GET_BITS!($dword, 6, ACPI_2BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_VERTICAL { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 6, ACPI_2BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_HORIZONTAL { ($dword:expr) => { ACPI_GET_BITS!($dword, 8, ACPI_2BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_HORIZONTAL { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 8, ACPI_2BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_SHAPE { ($dword:expr) => { ACPI_GET_BITS!($dword, 10, ACPI_4BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_SHAPE { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 10, ACPI_4BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_ORIENTATION { ($dword:expr) => { ACPI_GET_BITS!($dword, 14, ACPI_1BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_ORIENTATION { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 14, ACPI_1BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_TOKEN { ($dword:expr) => { ACPI_GET_BITS!($dword, 15, ACPI_8BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_TOKEN { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 15, ACPI_8BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_POSITION { ($dword:expr) => { ACPI_GET_BITS!($dword, 23, ACPI_8BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_POSITION { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 23, ACPI_8BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_BAY { ($dword:expr) => { ACPI_GET_BITS!($dword, 31, ACPI_1BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_BAY { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 31, ACPI_1BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_EJECTABLE { ($dword:expr) => { ACPI_GET_BITS!($dword, 0, ACPI_1BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_EJECTABLE { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 0, ACPI_1BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_OSPM_EJECT { ($dword:expr) => { ACPI_GET_BITS!($dword, 1, ACPI_1BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_OSPM_EJECT { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 1, ACPI_1BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_CABINET { ($dword:expr) => { ACPI_GET_BITS!($dword, 2, ACPI_8BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_CABINET { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 2, ACPI_8BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_CARD_CAGE { ($dword:expr) => { ACPI_GET_BITS!($dword, 10, ACPI_8BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_CARD_CAGE { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 10, ACPI_8BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_REFERENCE { ($dword:expr) => { ACPI_GET_BITS!($dword, 18, ACPI_1BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_REFERENCE { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 18, ACPI_1BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_ROTATION { ($dword:expr) => { ACPI_GET_BITS!($dword, 19, ACPI_4BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_ROTATION { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 19, ACPI_4BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_ORDER { ($dword:expr) => { ACPI_GET_BITS!($dword, 23, ACPI_5BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_ORDER { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 23, ACPI_5BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_VERT_OFFSET { ($dword:expr) => { ACPI_GET_BITS!($dword, 0, ACPI_16BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_VERT_OFFSET { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 0, ACPI_16BIT_MASK, $value) }; }
macro_rules! ACPI_PLD_GET_HORIZ_OFFSET { ($dword:expr) => { ACPI_GET_BITS!($dword, 16, ACPI_16BIT_MASK) }; }
macro_rules! ACPI_PLD_SET_HORIZ_OFFSET { ($dword:expr, $value:expr) => { ACPI_SET_BITS!($dword, 16, ACPI_16BIT_MASK, $value) }; }

pub const ACPI_PLD_PANEL_TOP: u32 = 0;
pub const ACPI_PLD_PANEL_BOTTOM: u32 = 1;
pub const ACPI_PLD_PANEL_LEFT: u32 = 2;
pub const ACPI_PLD_PANEL_RIGHT: u32 = 3;
pub const ACPI_PLD_PANEL_FRONT: u32 = 4;
pub const ACPI_PLD_PANEL_BACK: u32 = 5;
pub const ACPI_PLD_PANEL_UNKNOWN: u32 = 6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
